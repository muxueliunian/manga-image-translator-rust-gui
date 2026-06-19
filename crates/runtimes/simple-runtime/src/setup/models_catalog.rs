//! Model-management catalog (M1b).
//!
//! Builds a lightweight, no-load view of every downloadable model so the WebView
//! can show a downloaded/missing table and pre-fetch weights. Each concrete
//! module type already implements [`interface_model::Model`] (which carries
//! `kind`/`name`/`models`/`files`), so we just construct one of each — that only
//! allocates the empty model wrapper, no ONNX session is loaded — and read its
//! metadata.
//!
//! Note on `name()` collisions: `waifu2x`, `esrgan` and `anime4k` all report
//! `("upscaler", "waifu2x")` and share that on-disk dir, but their `files()` sets
//! are disjoint, so a stable per-entry `id` (not `kind/name`) is the group key.

use std::sync::Arc;

use base_util::onnx::{all_providers, Providers};
use interface_model::{model_file_ready, Model};
use serde::Serialize;

use crate::settings::{Detector, Inpainter, Settings, OCR};

/// Per-file readiness inside a model group.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelFileStatus {
    pub file: String,
    pub ready: bool,
}

/// One downloadable model and its on-disk status, as shown in the model table.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelGroupStatus {
    /// Stable, unique group key (used as the download target id).
    pub id: &'static str,
    /// Module category: `detector` | `ocr` | `inpainter` | `upscaler`.
    pub kind: &'static str,
    /// Friendly display label.
    pub label: &'static str,
    /// True when every file is present (group has at least one file).
    pub ready: bool,
    pub files: Vec<ModelFileStatus>,
}

/// A single file to fetch, resolved with its source URL/hash for downloading.
pub struct DownloadJob {
    pub kind: &'static str,
    pub name: &'static str,
    pub file: String,
    pub url: &'static str,
    pub hash: &'static str,
    pub label: &'static str,
}

type Entry = (&'static str, &'static str, Box<dyn Model + Send + Sync>);

/// Construct one instance of every downloadable module. Construction is cheap
/// (no model load); the batch sizes are irrelevant to metadata and unused here.
fn registry() -> Vec<Entry> {
    let p: Arc<Vec<Providers>> = Arc::new(all_providers());
    vec![
        // allow:clone[arc] — providers are shared read-only across constructors.
        (
            "dbnet",
            "DBNet",
            Box::new(dbnet::DbNetDetector::new(p.clone(), false)),
        ),
        (
            "paddle",
            "Paddle",
            Box::new(paddle::PaddleDetector::new(p.clone())),
        ),
        ("ctd", "CTD", Box::new(ctd::CtdDetector::new(p.clone()))),
        (
            "ocr-48px",
            "48px",
            Box::new(ocr_48px::Ocr48px::new(p.clone(), 256, 16)),
        ),
        (
            "ctc-48px",
            "CTC 48px",
            Box::new(ctc_48px::Ctc48pxOcr::new(p.clone(), 16)),
        ),
        (
            "manga-ocr",
            "Manga-OCR",
            Box::new(manga_ocr::MangaOCR::new(p.clone(), 256)),
        ),
        (
            "lama-aot",
            "LaMa AOT",
            Box::new(lama_aot::LamaLargeInpainter::new(p.clone())),
        ),
        (
            "lama-large",
            "LaMa Large",
            Box::new(lama_large::LamaLargeInpainter::new(p.clone())),
        ),
        (
            "lama-mpe",
            "LaMa MPE",
            Box::new(lama_mpe::LamaLargeInpainter::new(p.clone())),
        ),
        (
            "waifu2x",
            "Waifu2x",
            Box::new(waifu2x::Waifu2xUpscaler::new(
                waifu2x::Waifu2xModels::CuNetArt { noise: None },
                16,
                p.clone(),
            )),
        ),
        (
            "esrgan",
            "Real-ESRGAN",
            Box::new(esrgan::EsrGan::new(
                esrgan::EsrGanModel::X2Plus { f32: false },
                16,
                p.clone(),
            )),
        ),
        (
            "anime4k",
            "Anime4K",
            Box::new(anime4k::Anime4KUpscaler::new(
                anime4k::Anime4KModel::X2S,
                p.clone(),
            )),
        ),
    ]
}

/// Per-file readiness for one model. Returns `None` when the model has no
/// downloadable files (e.g. system OCR), so it is omitted from the table.
fn group_status(
    id: &'static str,
    label: &'static str,
    m: &(dyn Model + Send + Sync),
) -> Option<ModelGroupStatus> {
    let sources = m.models();
    let files: Vec<ModelFileStatus> = m
        .files()
        .into_iter()
        .map(|(key, file)| {
            let hash = sources.get(key).map(|s| s.hash).unwrap_or("###");
            let ready = model_file_ready(m.kind(), m.name(), &file, hash).unwrap_or(false);
            ModelFileStatus { file, ready }
        })
        .collect();
    if files.is_empty() {
        return None;
    }
    let ready = files.iter().all(|f| f.ready);
    Some(ModelGroupStatus {
        id,
        kind: m.kind(),
        label,
        ready,
        files,
    })
}

/// Full downloadable-model table, in display order (detector → ocr → inpainter
/// → upscaler). Readiness is checked against the configured model root without
/// downloading anything.
pub fn model_catalog() -> Vec<ModelGroupStatus> {
    registry()
        .iter()
        .filter_map(|(id, label, m)| group_status(id, label, m.as_ref()))
        .collect()
}

/// Resolve the missing files to download for the given group `ids` (empty = all
/// groups). Already-present files are skipped.
pub fn download_jobs(ids: &[String]) -> Vec<DownloadJob> {
    let mut jobs = Vec::new();
    for (id, label, m) in registry() {
        if !ids.is_empty() && !ids.iter().any(|want| want == id) {
            continue;
        }
        let sources = m.models();
        for (key, file) in m.files() {
            let Some(src) = sources.get(key) else {
                continue;
            };
            if model_file_ready(m.kind(), m.name(), &file, src.hash).unwrap_or(false) {
                continue;
            }
            jobs.push(DownloadJob {
                kind: m.kind(),
                name: m.name(),
                file,
                url: src.url,
                hash: src.hash,
                label,
            });
        }
    }
    jobs
}

/// Missing files of the currently-selected detector / OCR / inpainter, for the
/// startup auto-download. The upscaler is intentionally excluded: it is opt-in
/// and its `files()` covers every variant, so it stays lazy (downloads on first
/// use) instead of pulling tens of variants up front.
pub fn selected_core_download_jobs(settings: &Settings) -> Vec<DownloadJob> {
    let p: Arc<Vec<Providers>> = Arc::new(all_providers());
    let mut entries: Vec<(&'static str, Box<dyn Model + Send + Sync>)> = Vec::new();

    // allow:clone[arc]
    entries.push(match settings.detector.detector {
        Detector::DBNet => (
            "DBNet",
            Box::new(dbnet::DbNetDetector::new(p.clone(), false)),
        ),
        Detector::Paddle => ("Paddle", Box::new(paddle::PaddleDetector::new(p.clone()))),
        Detector::Ctd => ("CTD", Box::new(ctd::CtdDetector::new(p.clone()))),
    });
    // allow:clone[arc]
    entries.push(match settings.ocr.ocr {
        OCR::Ocr48px => ("48px", Box::new(ocr_48px::Ocr48px::new(p.clone(), 256, 16))),
        OCR::Ctc48px => (
            "CTC 48px",
            Box::new(ctc_48px::Ctc48pxOcr::new(p.clone(), 16)),
        ),
        OCR::MangaOcr => (
            "Manga-OCR",
            Box::new(manga_ocr::MangaOCR::new(p.clone(), 256)),
        ),
        OCR::Native => ("Native", Box::new(native::NativeOCR::default())),
        OCR::Tesseract => ("Tesseract", Box::new(tesseract::TesseractOCR::default())),
    });
    // allow:clone[arc]
    entries.push(match settings.inpainter.inpainter {
        Inpainter::LamaAot => (
            "LaMa AOT",
            Box::new(lama_aot::LamaLargeInpainter::new(p.clone())),
        ),
        Inpainter::LamaLarge => (
            "LaMa Large",
            Box::new(lama_large::LamaLargeInpainter::new(p.clone())),
        ),
        Inpainter::LamaMpe => (
            "LaMa MPE",
            Box::new(lama_mpe::LamaLargeInpainter::new(p.clone())),
        ),
    });

    let mut jobs = Vec::new();
    for (label, m) in entries {
        let sources = m.models();
        for (key, file) in m.files() {
            let Some(src) = sources.get(key) else {
                continue;
            };
            if model_file_ready(m.kind(), m.name(), &file, src.hash).unwrap_or(false) {
                continue;
            }
            jobs.push(DownloadJob {
                kind: m.kind(),
                name: m.name(),
                file,
                url: src.url,
                hash: src.hash,
                label,
            });
        }
    }
    jobs
}
