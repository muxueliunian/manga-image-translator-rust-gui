mod detector;
mod dict;
mod inpainter;
mod mask_refinement;
mod ocr;
mod textline_merge;
mod translator;
mod upscaler;

use std::{path::PathBuf, ptr, sync::Arc, time::Instant};

use export::Export;
use image::DynamicImage;
use interface_image::{CpuImageProcessor, ImageOp, RawImage};

use crate::{
    debug::{bbox::render_bboxes, save_img, save_json, save_mask, textblocks::render_textblocks},
    diagnostics,
    perf::{format_duration, sample_nvidia_gpu_memory, JobLogger, StageReport, StageTimer},
    settings::Settings,
    setup::Models,
};

pub type ImageProcessor = Arc<dyn ImageOp + Sync + Send>;

pub type StageCallback<'a> = Option<&'a mut (dyn FnMut(&'static str) + Send)>;

impl Models {
    pub async fn execute(
        &mut self,
        img: DynamicImage,
        config: &Settings,
        debug_path: Option<PathBuf>,
    ) -> anyhow::Result<Option<Export>> {
        self.execute_with_progress(img, config, debug_path, None)
            .await
    }

    pub async fn execute_with_progress(
        &mut self,
        img: DynamicImage,
        config: &Settings,
        debug_path: Option<PathBuf>,
        on_stage: StageCallback<'_>,
    ) -> anyhow::Result<Option<Export>> {
        self.execute_inner(img, config, debug_path, on_stage, None)
            .await
    }

    pub async fn execute_with_progress_and_logger(
        &mut self,
        img: DynamicImage,
        config: &Settings,
        debug_path: Option<PathBuf>,
        on_stage: StageCallback<'_>,
        logger: Option<&JobLogger>,
    ) -> anyhow::Result<Option<Export>> {
        self.execute_inner(img, config, debug_path, on_stage, logger)
            .await
    }

    async fn execute_inner(
        &mut self,
        img: DynamicImage,
        config: &Settings,
        debug_path: Option<PathBuf>,
        mut on_stage: StageCallback<'_>,
        logger: Option<&JobLogger>,
    ) -> anyhow::Result<Option<Export>> {
        let total_timer = Instant::now();
        let mut report = StageReport::new();
        log_job(logger, "image pipeline started");
        log_gpu_memory(logger, "pipeline-start");

        emit_stage(&mut on_stage, "图片预处理");
        let timer = StageTimer::start("preprocess", logger);
        let ip = Arc::new(CpuImageProcessor::default()) as ImageProcessor;
        let (img, alpha) = RawImage::rgba(img);
        report.record_timer(timer);

        emit_stage(&mut on_stage, "图像放大");
        let timer = StageTimer::start("upscaler", logger);
        let (img, alpha) = self.run_upscaler(img, alpha, config.upscaler, &ip).await?;
        report.record_timer(timer);

        if let Some(debug_path) = &debug_path {
            save_json(config, &debug_path.join("0_config.json"))?;
            save_img(&img, &debug_path.join("0_input.png"))?;
        }

        emit_stage(&mut on_stage, "文字检测");
        let timer = StageTimer::start("detector", logger);
        let (areas, mask) = self.run_detector(&img, &config.detector, &ip).await?;
        report.record_timer(timer);
        log_gpu_memory(logger, "after-detector");
        if let Some(debug_path) = &debug_path {
            save_mask(&mask, &debug_path.join("1_mask_raw.png"))?;
            save_json(&areas, &debug_path.join("1_quadrilateral.json"))?;
            render_bboxes(&img, &areas, debug_path)?;
        }
        if areas.is_empty() {
            return Ok(None);
        }

        let areas = areas.into_iter().map(to_mutex).collect::<Vec<_>>();
        let upscaled_img = img;

        emit_stage(&mut on_stage, "OCR 识别");
        let timer = StageTimer::start("ocr", logger);
        let textlines = self
            .run_ocr(&upscaled_img, &areas, &config.ocr, &debug_path, &ip)
            .await?;
        report.record_timer(timer);
        log_gpu_memory(logger, "after-ocr");
        diagnostics::record_ocr(
            logger,
            debug_path.as_deref(),
            &format!("{:?}", config.ocr.ocr),
            &textlines,
        )?;

        if textlines.is_empty() {
            return Ok(None);
        }

        if let Some(debug_path) = &debug_path {
            save_json(&textlines, &debug_path.join("2_quadrilateral.json"))?;
        }

        emit_stage(&mut on_stage, "文本行合并");
        let timer = StageTimer::start("textline-merge", logger);
        let textblocks = self.run_textline_merge(
            &textlines,
            upscaled_img.width,
            upscaled_img.height,
            &config.ocr,
            &config.translator,
        )?;
        report.record_timer(timer);
        diagnostics::record_textblocks(logger, debug_path.as_deref(), &textblocks)?;
        if textblocks.is_empty() {
            return Ok(None);
        }

        if let Some(debug_path) = &debug_path {
            save_json(&textblocks, &debug_path.join("3_textblock.json"))?;
            render_textblocks(&upscaled_img, &textblocks, debug_path)?;
        }

        emit_stage(&mut on_stage, "翻译文本");
        let timer = StageTimer::start("pre-dict", logger);
        let textblocks = self.run_pre_dict(textblocks, &config.translator)?;
        report.record_timer(timer);
        if let Some(debug_path) = &debug_path {
            if config.translator.pre_dict.is_some() {
                save_json(
                    &textlines,
                    &debug_path.join("3_textblock_predict_applied.json"),
                )?;
            }
        }

        let timer = StageTimer::start("translator", logger);
        let textblocks = self
            .run_translators(
                textblocks,
                &config.translator,
                debug_path.as_deref(),
                logger,
            )
            .await?;
        report.record_timer(timer);

        if let Some(debug_path) = &debug_path {
            save_json(
                &textblocks,
                &debug_path.join("4_textblocks_translated.json"),
            )?;
        }

        let timer = StageTimer::start("post-dict", logger);
        let textblocks = self.run_post_dict(textblocks, &config.translator)?;
        report.record_timer(timer);

        emit_stage(&mut on_stage, "修补文字区域");
        let timer = StageTimer::start("mask-refinement", logger);
        let mask_refined = Models::run_mask_refinement(
            &upscaled_img,
            &mask,
            &textblocks,
            &config.mask_refinement,
            &ip,
        )?;
        report.record_timer(timer);

        if let Some(debug_path) = &debug_path {
            save_mask(&mask_refined, &debug_path.join("4_mask_refined.png"))?;
        }

        let upscaled_img = Arc::new(upscaled_img);

        emit_stage(&mut on_stage, "图像修补");
        let timer = StageTimer::start("inpainter", logger);
        let (inpainted, mask) = self
            .run_inpainter(&upscaled_img, mask, mask_refined, &config.inpainter, &ip)
            .await?;
        report.record_timer(timer);
        log_gpu_memory(logger, "after-inpainter");

        let timer = StageTimer::start("compose", logger);
        let inpainted = inpainted.add_a(mask.data);
        if let Some(debug_path) = &debug_path {
            let mut img = upscaled_img.as_ref().clone();
            img.apply_filter(&inpainted, |a, b| unsafe {
                if *b.get_unchecked(3) > 128 {
                    ptr::copy_nonoverlapping(b.as_ptr(), a.as_mut_ptr(), 3);
                }
            });
            save_img(&img, &debug_path.join("5_inpainted.png"))?;
        }
        report.record_timer(timer);

        let export = Export::new(
            match alpha {
                Some(a) => upscaled_img.as_ref().clone().add_a(a),
                None => upscaled_img.as_ref().clone(),
            }
            .to_image()?,
            inpainted.to_image()?,
            textblocks,
            None,
        );
        if let Some(logger) = logger {
            for line in report.summary_lines() {
                logger.log("info", line);
            }
        }
        log_job(
            logger,
            format!(
                "image pipeline finished in {} (measured stages {})",
                format_duration(total_timer.elapsed()),
                format_duration(report.measured_total())
            ),
        );
        Ok(Some(export))
    }
}

fn emit_stage(on_stage: &mut StageCallback<'_>, stage: &'static str) {
    if let Some(callback) = on_stage.as_deref_mut() {
        callback(stage);
    }
}

fn to_mutex<T>(areas: T) -> Arc<parking_lot::Mutex<T>> {
    Arc::new(parking_lot::Mutex::new(areas))
}

fn log_job(logger: Option<&JobLogger>, message: impl AsRef<str>) {
    if let Some(logger) = logger {
        logger.log("info", message);
    }
}

fn log_gpu_memory(logger: Option<&JobLogger>, label: &str) {
    let Some(logger) = logger else {
        return;
    };
    if let Some(samples) = sample_nvidia_gpu_memory() {
        for sample in samples {
            logger.log(
                "info",
                format!(
                    "gpu-memory {label}: {} {} MiB / {} MiB",
                    sample.name, sample.used_mb, sample.total_mb
                ),
            );
        }
    }
}
