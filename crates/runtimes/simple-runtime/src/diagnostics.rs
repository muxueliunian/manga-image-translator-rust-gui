use std::{
    fs::{create_dir_all, read_dir, File, OpenOptions},
    io::Write as _,
    path::{Path, PathBuf},
    time::Duration,
};

use interface_ocr::QuadrilateralInfo;
use serde::Serialize;
use textline_merge::TextBlock;

use crate::perf::{format_duration, JobLogger};

#[derive(Serialize)]
struct OcrDebugEntry {
    index: usize,
    model: String,
    text: String,
    char_count: usize,
    prob: f64,
    bbox_xywh: [i64; 4],
    bbox_xyxy: [i64; 4],
    detector_score: f64,
    vertical: bool,
    points: Vec<[i64; 2]>,
    patch_path: Option<String>,
}

#[derive(Serialize)]
struct TextBlockDebugEntry<'a> {
    index: usize,
    text: &'a str,
    char_count: usize,
    line_count: usize,
    font_size: u64,
    angle: f64,
    skip_translate: bool,
    language: Option<String>,
    lines: Vec<Vec<[i64; 2]>>,
    translations: &'a std::collections::HashMap<String, String>,
}

#[derive(Serialize)]
struct TranslatorDebugEntry<'a> {
    phase: &'static str,
    step: usize,
    translator: &'a str,
    from: &'a str,
    to: &'a str,
    line_count: usize,
    char_count: usize,
    elapsed_ms: Option<u128>,
    output_language: Option<&'a str>,
    input_texts: &'a [String],
    output_texts: Option<&'a [String]>,
    error: Option<&'a str>,
}

pub fn record_ocr(
    logger: Option<&JobLogger>,
    debug_path: Option<&Path>,
    model: &str,
    textlines: &[QuadrilateralInfo],
) -> anyhow::Result<()> {
    if let Some(logger) = logger {
        logger.log(
            "info",
            format!(
                "ocr diagnostics: model={model}, detected_lines={}",
                textlines.len()
            ),
        );
        for (index, item) in textlines.iter().enumerate() {
            let quad = item.pos.lock();
            let bbox = quad.aabb();
            logger.log(
                "info",
                format!(
                    "ocr line {}: prob={:.3}, bbox=[{}, {}, {}, {}], vertical={}, text=\"{}\"",
                    index + 1,
                    item.prob,
                    bbox.x,
                    bbox.y,
                    bbox.w,
                    bbox.h,
                    quad.vertical(),
                    preview_text(&item.text, 120)
                ),
            );
        }
    }

    let Some(debug_path) = debug_path else {
        return Ok(());
    };
    create_dir_all(debug_path)?;
    let patch_files = collect_patch_files(&debug_path.join("ocr_patches"));
    let out_path = debug_path.join("ocr_debug.jsonl");
    let mut file = File::create(&out_path)?;
    for (index, item) in textlines.iter().enumerate() {
        let quad = item.pos.lock();
        let bbox = quad.aabb();
        let entry = OcrDebugEntry {
            index,
            model: model.to_owned(),
            text: item.text.clone(),
            char_count: item.text.chars().count(),
            prob: item.prob,
            bbox_xywh: [bbox.x, bbox.y, bbox.w, bbox.h],
            bbox_xyxy: [bbox.x, bbox.y, bbox.x + bbox.w, bbox.y + bbox.h],
            detector_score: quad.score(),
            vertical: quad.vertical(),
            points: quad.pts().iter().map(|point| [point.x, point.y]).collect(),
            patch_path: patch_files
                .get(index)
                .map(|path| path.display().to_string()),
        };
        serde_json::to_writer(&mut file, &entry)?;
        file.write_all(b"\n")?;
    }
    if let Some(logger) = logger {
        logger.log(
            "info",
            format!("ocr diagnostics file={}", out_path.display()),
        );
    }
    Ok(())
}

pub fn record_textblocks(
    logger: Option<&JobLogger>,
    debug_path: Option<&Path>,
    textblocks: &[TextBlock],
) -> anyhow::Result<()> {
    if let Some(logger) = logger {
        let skipped = textblocks
            .iter()
            .filter(|block| block.skip_translate)
            .count();
        logger.log(
            "info",
            format!(
                "textblock diagnostics: blocks={}, skipped_translate={skipped}",
                textblocks.len()
            ),
        );
        for (index, block) in textblocks.iter().enumerate() {
            logger.log(
                "info",
                format!(
                    "textblock {}: lines={}, skip_translate={}, chars={}, text=\"{}\"",
                    index + 1,
                    block.lines.len(),
                    block.skip_translate,
                    block.text.chars().count(),
                    preview_text(&block.text, 120)
                ),
            );
        }
    }

    let Some(debug_path) = debug_path else {
        return Ok(());
    };
    create_dir_all(debug_path)?;
    let out_path = debug_path.join("textblock_debug.jsonl");
    let mut file = File::create(&out_path)?;
    for (index, block) in textblocks.iter().enumerate() {
        let entry = TextBlockDebugEntry {
            index,
            text: &block.text,
            char_count: block.text.chars().count(),
            line_count: block.lines.len(),
            font_size: block.font_size,
            angle: block.angle,
            skip_translate: block.skip_translate,
            language: block
                .language
                .and_then(|language| serde_json::to_value(language).ok())
                .and_then(|value| value.as_str().map(str::to_owned)),
            lines: block
                .lines
                .iter()
                .map(|line| line.iter().map(|point| [point.x, point.y]).collect())
                .collect(),
            translations: &block.translations,
        };
        serde_json::to_writer(&mut file, &entry)?;
        file.write_all(b"\n")?;
    }
    if let Some(logger) = logger {
        logger.log(
            "info",
            format!("textblock diagnostics file={}", out_path.display()),
        );
    }
    Ok(())
}

pub fn record_translator_request(
    logger: Option<&JobLogger>,
    debug_path: Option<&Path>,
    step: usize,
    translator: &str,
    from: &str,
    to: &str,
    input_texts: &[String],
) -> anyhow::Result<()> {
    if let Some(logger) = logger {
        logger.log(
            "info",
            format!(
                "translator request {}: provider={}, from={}, to={}, lines={}, chars={}, input=\"{}\"",
                step,
                translator,
                from,
                to,
                input_texts.len(),
                char_count(input_texts),
                preview_texts(input_texts, 180)
            ),
        );
    }
    append_translator_entry(
        debug_path,
        &TranslatorDebugEntry {
            phase: "request",
            step,
            translator,
            from,
            to,
            line_count: input_texts.len(),
            char_count: char_count(input_texts),
            elapsed_ms: None,
            output_language: None,
            input_texts,
            output_texts: None,
            error: None,
        },
    )
}

pub fn record_translator_response(
    logger: Option<&JobLogger>,
    debug_path: Option<&Path>,
    step: usize,
    translator: &str,
    from: &str,
    to: &str,
    input_texts: &[String],
    output_texts: &[String],
    output_language: Option<&str>,
    elapsed: Duration,
) -> anyhow::Result<()> {
    if let Some(logger) = logger {
        logger.log(
            "info",
            format!(
                "translator response {}: provider={}, elapsed={}, output_lang={}, lines={}, output=\"{}\"",
                step,
                translator,
                format_duration(elapsed),
                output_language.unwrap_or("unknown"),
                output_texts.len(),
                preview_texts(output_texts, 180)
            ),
        );
    }
    append_translator_entry(
        debug_path,
        &TranslatorDebugEntry {
            phase: "response",
            step,
            translator,
            from,
            to,
            line_count: input_texts.len(),
            char_count: char_count(input_texts),
            elapsed_ms: Some(elapsed.as_millis()),
            output_language,
            input_texts,
            output_texts: Some(output_texts),
            error: None,
        },
    )
}

pub fn record_translator_error(
    logger: Option<&JobLogger>,
    debug_path: Option<&Path>,
    step: usize,
    translator: &str,
    from: &str,
    to: &str,
    input_texts: &[String],
    error: &str,
    elapsed: Duration,
) -> anyhow::Result<()> {
    if let Some(logger) = logger {
        logger.log(
            "error",
            format!(
                "translator error {}: provider={}, elapsed={}, error={}",
                step,
                translator,
                format_duration(elapsed),
                error
            ),
        );
    }
    append_translator_entry(
        debug_path,
        &TranslatorDebugEntry {
            phase: "error",
            step,
            translator,
            from,
            to,
            line_count: input_texts.len(),
            char_count: char_count(input_texts),
            elapsed_ms: Some(elapsed.as_millis()),
            output_language: None,
            input_texts,
            output_texts: None,
            error: Some(error),
        },
    )
}

pub fn preview_text(value: &str, limit: usize) -> String {
    let mut out = value
        .chars()
        .map(|ch| match ch {
            '\r' | '\n' | '\t' => ' ',
            ch => ch,
        })
        .take(limit)
        .collect::<String>();
    if value.chars().count() > limit {
        out.push_str("...");
    }
    out
}

fn preview_texts(texts: &[String], limit: usize) -> String {
    preview_text(&texts.join(" | "), limit)
}

fn char_count(texts: &[String]) -> usize {
    texts.iter().map(|text| text.chars().count()).sum()
}

fn append_translator_entry(
    debug_path: Option<&Path>,
    entry: &TranslatorDebugEntry<'_>,
) -> anyhow::Result<()> {
    let Some(debug_path) = debug_path else {
        return Ok(());
    };
    create_dir_all(debug_path)?;
    let out_path = debug_path.join("translator_debug.jsonl");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(out_path)?;
    serde_json::to_writer(&mut file, entry)?;
    file.write_all(b"\n")?;
    Ok(())
}

fn collect_patch_files(path: &Path) -> Vec<PathBuf> {
    let mut files = read_dir(path)
        .ok()
        .into_iter()
        .flat_map(|entries| entries.filter_map(Result::ok))
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .collect::<Vec<_>>();
    files.sort_by_key(|path| patch_sort_key(path));
    files
}

fn patch_sort_key(path: &Path) -> Vec<u32> {
    path.file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .split('_')
        .filter_map(|part| part.parse::<u32>().ok())
        .collect()
}
