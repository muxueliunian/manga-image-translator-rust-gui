#![allow(
    clippy::manual_div_ceil,
    clippy::needless_return,
    clippy::new_without_default,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]

use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
    time::Instant,
};

use clap::Parser as _;
use config::Config;
use export::Export;
use html::HtmlRenderer;
use image::{ExtendedColorType, ImageEncoder};
use log::{error, info, warn};
use png::{PngRenderConfig, PngRenderer, TextDirectionMode};
use tracing_subscriber::EnvFilter;
use walkdir::WalkDir;

use crate::{
    perf::{ensure_cuda_policy, format_duration, JobLogger, RuntimeDiagnostics, StageTimer},
    settings::{Renderer, Settings, TextDirection},
    setup::Models,
    update::{check_crate_version, check_cuda_error},
};

pub mod cli;
mod debug;
mod diagnostics;
mod dict;
mod execute;
pub mod gpu_runtime;
mod perf;
mod proxy;
pub mod settings;
pub mod setup;
mod update;
mod webview_ui;

pub fn render_export_to_png_bytes(exp: Export) -> anyhow::Result<Vec<u8>> {
    render_export_to_png_bytes_with_direction(exp, TextDirection::Auto)
}

pub fn render_export_to_png_bytes_with_direction(
    exp: Export,
    text_direction: TextDirection,
) -> anyhow::Result<Vec<u8>> {
    let mut renderer = PngRenderer::default();
    let img = renderer.render(
        exp,
        PngRenderConfig {
            text_direction: match text_direction {
                TextDirection::Auto => TextDirectionMode::Auto,
                TextDirection::Horizontal => TextDirectionMode::Horizontal,
                TextDirection::Vertical => TextDirectionMode::Vertical,
            },
            ..PngRenderConfig::default()
        },
    );
    let mut data = Vec::new();
    let color = match img.channels {
        4 => ExtendedColorType::Rgba8,
        3 => ExtendedColorType::Rgb8,
        1 => ExtendedColorType::L8,
        ch => anyhow::bail!("Unsupported PNG channel count: {ch}"),
    };
    image::codecs::png::PngEncoder::new(&mut data).write_image(
        &img.data,
        img.width as u32,
        img.height as u32,
        color,
    )?;
    Ok(data)
}

pub fn render_export_bytes(exp: Export, renderer: &Renderer) -> anyhow::Result<Vec<u8>> {
    match renderer {
        Renderer::Png => render_export_to_png_bytes(exp),
        Renderer::Html => Ok(HtmlRenderer::render(vec![exp], None, false).0),
        Renderer::Raw => Ok(exp.export()),
    }
}

pub fn render_export_bytes_with_settings(
    exp: Export,
    settings: &Settings,
) -> anyhow::Result<Vec<u8>> {
    match settings.render.renderer {
        Renderer::Png => {
            render_export_to_png_bytes_with_direction(exp, settings.render.text_direction)
        }
        Renderer::Html => Ok(HtmlRenderer::render(vec![exp], None, false).0),
        Renderer::Raw => Ok(exp.export()),
    }
}

pub fn prepare_renderer_assets(path: &std::path::Path, renderer: &Renderer) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
        if renderer == &Renderer::Html {
            html::copy_files(parent)?;
        }
    }
    Ok(())
}

fn logs_dir() -> &'static Path {
    Path::new("logs")
}

fn main() {
    proxy::apply_system_proxy();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to build Tokio runtime")
        .block_on(async_main());
}

async fn async_main() {
    let cli = cli::Cli::parse();
    let (level, ort_level) = match cli.verbose {
        3 | 2 => ("debug", "ort=debug"),
        1 => ("info", "ort=warn"),
        _ => ("warn", "ort=error"),
    };

    let base_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level));
    let filter = base_filter.add_directive(ort_level.parse().unwrap());

    tracing_subscriber::fmt()
        .with_level(true)
        .with_target(true)
        .with_env_filter(filter)
        .init();

    let cuda_error = check_cuda_error();
    let cuda = cuda_error.is_none();
    let diagnostics = RuntimeDiagnostics::collect_with_error(cuda, cuda_error.clone());
    info!(
        "Runtime diagnostics: cuda_feature={}, cuda_available={}, require_cuda={}, provider_status={}, cuda_error={}",
        diagnostics.cuda_feature,
        diagnostics.cuda_available,
        diagnostics.require_cuda,
        diagnostics.provider_status,
        diagnostics.cuda_error.as_deref().unwrap_or("")
    );
    if let Err(err) = ensure_cuda_policy(cuda) {
        error!("{err}");
        return;
    }
    if !cuda && cfg!(all(target_arch = "x86_64", not(target_os = "macos"))) {
        warn!("CUDA is not available")
    }

    if matches!(cli.command, cli::Commands::UiWebview) {
        webview_ui::run().expect("Failed to run WebView UI");
        return;
    }

    let _ = check_crate_version("muxueliunian/manga-image-translator-rust-gui").await;

    let models = Models::new(
        cli.max_batch_size_upscaler,
        cli.max_batch_size_ocr,
        true,
        cuda,
    )
    .await;
    match cli.command {
        cli::Commands::Cli {
            input,
            output,
            config,
            overwrite,
        } => {
            let job_timer = Instant::now();
            let job_logger = JobLogger::create(logs_dir()).ok();
            if let Some(logger) = &job_logger {
                logger.log(
                    "info",
                    format!(
                        "cli job started: cuda_feature={}, cuda_available={}, require_cuda={}, provider_status={}",
                        diagnostics.cuda_feature,
                        diagnostics.cuda_available,
                        diagnostics.require_cuda,
                        diagnostics.provider_status
                    ),
                );
                info!("Writing performance log to {}", logger.path().display());
            }
            let mut input_list = WalkDir::new(&input)
                .into_iter()
                .filter_map(|v| v.ok())
                .map(|v| v.path().to_path_buf())
                .filter(|v| v.is_file())
                .filter(|v| !v.to_string_lossy().starts_with("."))
                .map(|v| v.strip_prefix(&input).map(|v| v.to_path_buf()).unwrap_or(v))
                //TODO: add other extensions
                .filter(|v| {
                    ["png", "jpg", "jpeg", "webp"].contains(
                        &v.extension()
                            .map(|v| v.to_string_lossy())
                            .unwrap_or_default()
                            .to_lowercase()
                            .as_str(),
                    )
                })
                .collect::<Vec<_>>();
            let mut settings = Config::builder();
            if let Some(config) = config {
                if !config.exists() {
                    panic!("Config file does not exist")
                }
                settings = settings.add_source(config::File::from(config));
            }
            let settings = settings.build().expect("Failed to build settings");
            let settings = settings.try_deserialize::<Settings>().unwrap_or_default();
            let out_ext = settings.render.renderer.extension();
            if !overwrite {
                input_list = input_list
                    .into_iter()
                    .filter(|v| {
                        let mut path = output.join(v);
                        path.set_extension(out_ext);
                        !path.exists()
                    })
                    .collect::<Vec<_>>();
            }

            for path in input_list {
                info!("Processing {}", path.display());
                let mut output = output.join(&path);
                let path = input.join(path);
                if !path.exists() || !path.is_file() {
                    warn!("File {} cant be found", path.display());
                    continue;
                }
                let img = match image::open(&path) {
                    Ok(img) => img,
                    Err(err) => {
                        error!("Failed to open image {}: {}", path.display(), err);
                        continue;
                    }
                };
                let debug_path = if cli.verbose > 2 {
                    let id = uuid::Uuid::new_v4();
                    let p = PathBuf::from(format!("debug/{id}"));
                    create_dir_all(&p).expect("Failed to create debug directory");
                    Some(p)
                } else {
                    None
                };
                let exp = models
                    .execute_with_progress_and_logger(
                        img,
                        &settings,
                        debug_path,
                        None,
                        job_logger.as_ref(),
                    )
                    .await
                    .unwrap();
                let exp = match exp {
                    Some(v) => v,
                    None => {
                        info!("Failed to detect any translatable content");
                        continue;
                    }
                };
                output.set_extension(out_ext);
                prepare_renderer_assets(&output, &settings.render.renderer)
                    .expect("Failed to prepare render output");
                let render_timer = StageTimer::start("render", job_logger.as_ref());
                let data = render_export_bytes_with_settings(exp, &settings)
                    .expect("Failed to render output");
                render_timer.finish();
                let write_timer = StageTimer::start("write", job_logger.as_ref());
                File::create(output).unwrap().write_all(&data).unwrap();
                write_timer.finish();
            }
            if let Some(logger) = &job_logger {
                logger.log(
                    "info",
                    format!(
                        "cli job finished in {}",
                        format_duration(job_timer.elapsed())
                    ),
                );
            }
        }
        cli::Commands::UiWebview => unreachable!("ui-webview exits before model initialization"),
    }
}
