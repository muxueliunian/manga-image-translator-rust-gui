use std::{
    fs::{copy, create_dir_all, read_to_string, File},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use anyhow::{anyhow, Result};
use interface_model::{set_model_root, ModelRootMode};
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use tao::{
    dpi::{LogicalSize, PhysicalPosition, PhysicalSize},
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::{Window, WindowBuilder},
};
use tokio::sync::{Mutex as AsyncMutex, Semaphore};
use wry::http::{header::CONTENT_TYPE, Response};
use wry::WebViewBuilder;

use crate::{
    gpu_runtime,
    perf::{
        ensure_cuda_policy, format_duration, sample_nvidia_gpu_memory, JobLogger,
        RuntimeDiagnostics, StageTimer,
    },
    prepare_renderer_assets, render_export_bytes_with_settings,
    settings::{Renderer, Settings},
    setup::{self, Models},
    update::{self, check_cuda_error},
};

const INDEX_HTML: &str = include_str!("../webview/index.html");
const STYLES_CSS: &str = include_str!("../webview/styles.css");
const APP_JS: &str = include_str!("../webview/app.js");
/// Extension of the editable Export sidecar written next to each rendered result
/// (P5). Holds the raw `Export` blob (original + inpainted background + text blocks).
const EDITABLE_EXT: &str = "mit";
// A single shared `Models` instance serves all concurrent images. The pipeline runs with
// `&self`, and `AsyncSessionPool` handles concurrent ONNX submissions internally, so there
// is no need to duplicate `Models` (which would double the resident VRAM). GPU concurrency
// is bounded by `gpu_semaphore`.
type ModelPool = Arc<AsyncMutex<Option<Arc<Models>>>>;

#[derive(Debug)]
enum UserEvent {
    Ipc(String),
    IpcResponse(IpcResponse<serde_json::Value>),
    Progress(ProgressEvent),
    Log(LogEvent),
    Restart,
    ExitForUpdate,
}

#[derive(Debug, Deserialize)]
struct IpcRequest {
    id: String,
    kind: IpcKind,
    #[serde(default)]
    payload: serde_json::Value,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
enum IpcKind {
    AppReady,
    PickImages,
    PickFolder,
    PickOutputDir,
    Defaults,
    LoadConfig,
    SaveConfig,
    StartTranslation,
    PreviewResult,
    ExportResults,
    ListDir,
    ReadImage,
    LoadEditable,
    RerenderExport,
    RevealInExplorer,
    GetModelsConfig,
    SetModelsDir,
    SetAutoDownload,
    GetModelsStatus,
    DownloadModels,
    GetGpuRuntimeStatus,
    DownloadCudaRuntime,
    RestartApp,
    CheckAppUpdate,
    DownloadAppUpdate,
    InstallAppUpdate,
    OpenExternal,
}

#[derive(Debug, Serialize)]
struct IpcResponse<T: Serialize> {
    id: String,
    ok: bool,
    data: Option<T>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct PickedPaths {
    paths: Vec<String>,
}

#[derive(Debug, Serialize)]
struct AppReadyData {
    version: &'static str,
    platform: &'static str,
    backend: &'static str,
    diagnostics: RuntimeDiagnostics,
}

#[derive(Debug, Serialize)]
struct StartTranslationResult {
    status: String,
    message: String,
    outputs: Vec<TranslationOutput>,
}

#[derive(Debug, Serialize)]
struct TranslationOutput {
    input: String,
    output: Option<String>,
    file_name: Option<String>,
    status: String,
    message: String,
    /// Path to the editable Export sidecar (P5), when one was written for this result.
    editable: Option<String>,
}

#[derive(Debug, Serialize)]
struct ProgressEvent {
    current: usize,
    total: usize,
    percent: u8,
    message: String,
}

#[derive(Debug, Serialize)]
struct LogEvent {
    level: String,
    message: String,
}

#[derive(Deserialize)]
struct StartTranslationPayload {
    input_paths: Vec<PathBuf>,
    settings: Settings,
    output_format: String,
    #[serde(default)]
    device_mode: base_util::onnx::DeviceMode,
    #[serde(default = "default_max_parallel_images")]
    max_parallel_images: usize,
    #[serde(default = "default_max_parallel_gpu_jobs")]
    max_parallel_gpu_jobs: usize,
    /// When true, write the heavy per-image diagnostics dump (intermediate
    /// images, masks, OCR patch crops, JSON). Off by default to keep runs fast
    /// and stage timings clean; the lightweight job log is always written.
    #[serde(default)]
    debug: bool,
}

#[derive(Deserialize)]
struct SaveConfigPayload {
    settings: Settings,
}

#[derive(Deserialize)]
struct PreviewResultPayload {
    path: PathBuf,
}

#[derive(Deserialize)]
struct ExportResultsPayload {
    outputs: Vec<PathBuf>,
    export_dir: PathBuf,
}

#[derive(Serialize)]
struct ExportResultsData {
    exported: Vec<String>,
}

#[derive(Deserialize)]
struct ListDirPayload {
    path: PathBuf,
}

#[derive(Deserialize)]
struct ReadImagePayload {
    path: PathBuf,
}

#[derive(Deserialize)]
struct RevealInExplorerPayload {
    path: PathBuf,
}

/// P5: load the editable Export sidecar for a rendered result. `path` is the
/// result image path; the sidecar is resolved by swapping its extension.
#[derive(Deserialize)]
struct LoadEditablePayload {
    path: PathBuf,
}

/// P5.2: apply manual edits (text and/or position offset) to a result's blocks and
/// re-render. `path` is the result image path (its sidecar holds the Export).
#[derive(Deserialize)]
struct RerenderExportPayload {
    path: PathBuf,
    #[serde(default)]
    edits: Vec<BlockEdit>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct BlockEdit {
    index: usize,
    /// Replacement translated text for this block (None = leave text unchanged).
    #[serde(default)]
    text: Option<String>,
    /// Image-space pixel offset to shift the whole block by (applied to every point).
    #[serde(default)]
    dx: i64,
    #[serde(default)]
    dy: i64,
}

#[derive(Deserialize)]
struct OpenExternalPayload {
    url: String,
}

/// Persisted model-management config (`config/models.json`). Kept separate from
/// the translation `app.json` since the model root is an environment concern.
#[derive(Debug, Default, Serialize, Deserialize)]
struct ModelsConfig {
    #[serde(default)]
    model_dir: Option<String>,
    #[serde(default)]
    auto_download: bool,
}

#[derive(Deserialize)]
struct SetAutoDownloadPayload {
    value: bool,
}

/// Which model groups to download. Empty = every group with missing files.
/// Group ids come from [`setup::model_catalog`]'s `id` field.
#[derive(Deserialize, Default)]
struct DownloadModelsPayload {
    #[serde(default)]
    targets: Vec<String>,
}

#[derive(Serialize)]
struct DirEntryInfo {
    name: String,
    path: String,
    is_dir: bool,
    is_image: bool,
}

#[derive(Serialize)]
struct ListDirData {
    entries: Vec<DirEntryInfo>,
}

fn default_max_parallel_images() -> usize {
    2
}

fn default_max_parallel_gpu_jobs() -> usize {
    1
}

pub fn run() -> Result<()> {
    // Resolve the model root from config before anything can trigger a download.
    apply_models_config(&load_models_config());
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();
    let ipc_proxy = proxy.clone();
    let models: ModelPool = Arc::new(AsyncMutex::new(None));
    let saved_window = load_window_state();
    let mut window_builder = WindowBuilder::new()
        .with_title("漫画图片翻译器")
        .with_min_inner_size(LogicalSize::new(960.0, 640.0));
    match saved_window
        .as_ref()
        .filter(|state| state.width >= 320 && state.height >= 240)
    {
        Some(state) => {
            window_builder =
                window_builder.with_inner_size(PhysicalSize::new(state.width, state.height));
            if let (Some(x), Some(y)) = (state.x, state.y) {
                window_builder = window_builder.with_position(PhysicalPosition::new(x, y));
            }
        }
        None => {
            window_builder = window_builder.with_inner_size(LogicalSize::new(1180.0, 780.0));
        }
    }
    let window = window_builder.build(&event_loop)?;
    if saved_window.map(|state| state.maximized).unwrap_or(false) {
        window.set_maximized(true);
    }

    let webview = WebViewBuilder::new()
        .with_custom_protocol("mit".into(), move |_webview_id, _request| {
            // Only the page itself is served here. Local image bytes can't be loaded
            // as `<img>` subresources from a `mit://` page (WebView2 blocks `file://`,
            // and custom-scheme subresource fetches are dropped), so previews/thumbnails
            // go through the `ReadImage` IPC and render as `data:` URLs instead.
            Response::builder()
                .header(CONTENT_TYPE, "text/html; charset=utf-8")
                .body(build_html().into_bytes())
                .unwrap()
                .map(Into::into)
        })
        .with_url("mit://localhost/")
        .with_ipc_handler(move |request| {
            let _ = ipc_proxy.send_event(UserEvent::Ipc(request.body().to_string()));
        })
        .build(&window)?;

    // Startup auto-download (opt-in): front-load the *currently selected* pipeline
    // models so the first translation doesn't stall on a download. Scoped to
    // detector/OCR/inpainter only — the upscaler is opt-in and variant-heavy, so it
    // stays lazy. Events queue until the event loop below drains them.
    let startup_models_config = load_models_config();
    if startup_models_config.auto_download
        && startup_models_config
            .model_dir
            .as_deref()
            .map(|dir| !dir.trim().is_empty())
            .unwrap_or(false)
    {
        let auto_proxy = proxy.clone();
        thread::spawn(move || {
            let settings = load_saved_settings().unwrap_or_default();
            send_log(&auto_proxy, "info", "启动自动下载：检查当前流水线所需模型…");
            let jobs = setup::selected_core_download_jobs(&settings);
            run_download_jobs(jobs, &auto_proxy);
        });
    }

    event_loop.run(move |event, _target, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                send_event(
                    &webview,
                    "log",
                    serde_json::json!({
                        "level": "info",
                        "message": "WebView UI started. Backend bridge is ready."
                    }),
                );
            }
            Event::UserEvent(UserEvent::Ipc(message)) => {
                handle_ipc_message(&webview, &proxy, models.clone(), &message);
            }
            Event::UserEvent(UserEvent::IpcResponse(response)) => {
                resolve_request(&webview, response);
            }
            Event::UserEvent(UserEvent::Progress(progress)) => {
                send_event(&webview, "progress", progress);
            }
            Event::UserEvent(UserEvent::Log(log)) => {
                send_event(&webview, "log", log);
            }
            Event::UserEvent(UserEvent::Restart) => {
                save_window_state_from(&window);
                if let Ok(exe) = std::env::current_exe() {
                    let _ = std::process::Command::new(exe).spawn();
                }
                *control_flow = ControlFlow::Exit;
            }
            Event::UserEvent(UserEvent::ExitForUpdate) => {
                save_window_state_from(&window);
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                save_window_state_from(&window);
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}

fn build_html() -> String {
    INDEX_HTML
        .replace("<!-- MIT_WEBVIEW_STYLES -->", STYLES_CSS)
        .replace("/* MIT_WEBVIEW_APP */", APP_JS)
}

/// Read a local image and return it as a `data:` URL for the frontend to drop into
/// an `<img src>`. WebView2 won't fetch `file://` or custom-scheme subresources from
/// a `mit://` page, so the bytes are base64-inlined instead.
fn read_image_data_url(path: &Path) -> Result<serde_json::Value> {
    let bytes = std::fs::read(path).map_err(|err| anyhow!("Failed to read image: {err}"))?;
    let data_url = format!("data:{};base64,{}", mime_for(path), base64_encode(&bytes));
    Ok(serde_json::json!({ "data_url": data_url }))
}

/// Translation-map key used to force a manually edited string through the renderer.
/// The renderer resolves text via `translations["last_trans"] -> translations[that]`,
/// so a manual edit points `last_trans` here and stores the text under this key.
const MANUAL_TRANS_KEY: &str = "__manual_edit__";

/// Resolve the sidecar path for a rendered result and return it as a string if the
/// editable Export blob exists on disk.
fn editable_sidecar(output: &Path) -> Option<String> {
    let sidecar = output.with_extension(EDITABLE_EXT);
    sidecar.exists().then(|| sidecar.display().to_string())
}

/// Resolve the text a block currently renders, matching the PNG renderer's lookup
/// (`last_trans` -> keyed translation -> any translation -> source text).
fn resolved_block_text(block: &textline_merge::TextBlock) -> String {
    block
        .translations
        .get("last_trans")
        .and_then(|key| block.translations.get(key))
        .or_else(|| block.translations.values().next())
        .cloned()
        .unwrap_or_else(|| block.text.clone())
}

/// Load the editable Export for a result: returns the text-free inpainted background
/// as a data URL plus a lightweight per-block region list (axis-aligned bbox in the
/// render image's coordinate space, current text, colors). The frontend scales these
/// boxes by `naturalWidth/Height` to overlay them on the canvas (P5.1/P5.2).
fn load_editable(result_path: &Path) -> Result<serde_json::Value> {
    let sidecar = result_path.with_extension(EDITABLE_EXT);
    let bytes =
        std::fs::read(&sidecar).map_err(|err| anyhow!("Failed to read editable sidecar: {err}"))?;
    let exp = export::Export::load(bytes)
        .ok_or_else(|| anyhow!("Failed to parse editable export (corrupt sidecar?)"))?;

    let bg = png::background_image(&exp);
    let (width, height) = (bg.width, bg.height);
    let bg_png = crate::raw_image_to_png_bytes(&bg)?;
    let background = format!("data:image/png;base64,{}", base64_encode(&bg_png));

    let regions = exp
        .blocks
        .iter()
        .enumerate()
        .map(|(index, block)| {
            let mut min_x = i64::MAX;
            let mut min_y = i64::MAX;
            let mut max_x = i64::MIN;
            let mut max_y = i64::MIN;
            for line in &block.lines {
                for p in line {
                    min_x = min_x.min(p.x);
                    min_y = min_y.min(p.y);
                    max_x = max_x.max(p.x);
                    max_y = max_y.max(p.y);
                }
            }
            // Empty `lines` would leave the sentinels; clamp to a zero-size box.
            if min_x > max_x {
                min_x = 0;
                max_x = 0;
                min_y = 0;
                max_y = 0;
            }
            serde_json::json!({
                "index": index,
                "x": min_x,
                "y": min_y,
                "w": (max_x - min_x).max(0),
                "h": (max_y - min_y).max(0),
                "angle": block.angle,
                "text": resolved_block_text(block),
                "fg": block.fg_color.map(|(r, g, b)| [r, g, b]),
                "bg": block.bg_color.map(|(r, g, b)| [r, g, b]),
                "fontSize": block.font_size,
            })
        })
        .collect::<Vec<_>>();

    Ok(serde_json::json!({
        "width": width,
        "height": height,
        "background": background,
        "regions": regions,
    }))
}

/// Apply manual edits (text and/or position offset) to a result's blocks, re-render
/// the PNG (renderer-only, no models), overwrite both the result image and its
/// editable sidecar, and return the fresh image as a data URL (P5.2).
fn rerender_export(result_path: &Path, edits: &[BlockEdit]) -> Result<serde_json::Value> {
    let sidecar = result_path.with_extension(EDITABLE_EXT);
    let bytes =
        std::fs::read(&sidecar).map_err(|err| anyhow!("Failed to read editable sidecar: {err}"))?;
    let mut exp = export::Export::load(bytes)
        .ok_or_else(|| anyhow!("Failed to parse editable export (corrupt sidecar?)"))?;

    for edit in edits {
        let Some(block) = exp.blocks.get_mut(edit.index) else {
            continue;
        };
        if let Some(text) = &edit.text {
            block
                .translations
                .insert("last_trans".to_owned(), MANUAL_TRANS_KEY.to_owned());
            block
                .translations
                .insert(MANUAL_TRANS_KEY.to_owned(), text.clone());
        }
        if edit.dx != 0 || edit.dy != 0 {
            for line in &mut block.lines {
                for p in line.iter_mut() {
                    p.x += edit.dx;
                    p.y += edit.dy;
                }
            }
        }
    }

    // Re-serialize the mutated Export back to the sidecar so further edits accumulate.
    let raw = exp.export();
    if let Err(err) = File::create(&sidecar).and_then(|mut f| f.write_all(&raw)) {
        return Err(anyhow!("Failed to update editable sidecar: {err}"));
    }
    let exp = export::Export::load(raw).ok_or_else(|| anyhow!("Failed to reload edited export"))?;

    // Editing always targets the PNG output (the GUI only enables it for png results).
    let settings = load_saved_settings().unwrap_or_default();
    let data =
        crate::render_export_to_png_bytes_with_direction(exp, settings.render.text_direction)?;
    File::create(result_path)
        .and_then(|mut f| f.write_all(&data))
        .map_err(|err| anyhow!("Failed to write re-rendered image: {err}"))?;

    Ok(serde_json::json!({
        "data_url": format!("data:image/png;base64,{}", base64_encode(&data)),
    }))
}

/// Minimal standard base64 encoder (with padding); avoids pulling in a crate.
fn base64_encode(input: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity((input.len() + 2) / 3 * 4);
    for chunk in input.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = *chunk.get(1).unwrap_or(&0) as u32;
        let b2 = *chunk.get(2).unwrap_or(&0) as u32;
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(ALPHABET[(n >> 18) as usize & 0x3f] as char);
        out.push(ALPHABET[(n >> 12) as usize & 0x3f] as char);
        out.push(if chunk.len() > 1 {
            ALPHABET[(n >> 6) as usize & 0x3f] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            ALPHABET[n as usize & 0x3f] as char
        } else {
            '='
        });
    }
    out
}

fn mime_for(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase())
        .as_deref()
    {
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("webp") => "image/webp",
        Some("gif") => "image/gif",
        Some("bmp") => "image/bmp",
        Some("tif") | Some("tiff") => "image/tiff",
        Some("avif") => "image/avif",
        _ => "application/octet-stream",
    }
}

fn handle_ipc_message(
    webview: &wry::WebView,
    proxy: &tao::event_loop::EventLoopProxy<UserEvent>,
    models: ModelPool,
    message: &str,
) {
    let request = match serde_json::from_str::<IpcRequest>(message) {
        Ok(request) => request,
        Err(err) => {
            send_event(
                webview,
                "log",
                serde_json::json!({
                    "level": "error",
                    "message": format!("Invalid IPC payload: {err}")
                }),
            );
            return;
        }
    };

    if matches!(request.kind, IpcKind::StartTranslation) {
        handle_start_translation(request, proxy.clone(), models);
        return;
    }

    if matches!(request.kind, IpcKind::DownloadModels) {
        handle_download_models(request, proxy.clone());
        return;
    }

    if matches!(request.kind, IpcKind::DownloadCudaRuntime) {
        handle_download_cuda_runtime(request, proxy.clone());
        return;
    }

    if matches!(request.kind, IpcKind::CheckAppUpdate) {
        handle_check_app_update(request, proxy.clone());
        return;
    }

    if matches!(request.kind, IpcKind::DownloadAppUpdate) {
        handle_download_app_update(request, proxy.clone());
        return;
    }

    if matches!(request.kind, IpcKind::InstallAppUpdate) {
        handle_install_app_update(request, proxy.clone());
        return;
    }

    if matches!(request.kind, IpcKind::RestartApp) {
        resolve_request(
            webview,
            IpcResponse::<serde_json::Value> {
                id: request.id,
                ok: true,
                data: None,
                error: None,
            },
        );
        let _ = proxy.send_event(UserEvent::Restart);
        return;
    }

    let response = match handle_ipc_request(&request) {
        Ok(value) => IpcResponse {
            id: request.id,
            ok: true,
            data: Some(value),
            error: None,
        },
        Err(err) => IpcResponse::<serde_json::Value> {
            id: request.id,
            ok: false,
            data: None,
            error: Some(err.to_string()),
        },
    };

    resolve_request(webview, response);
}

fn handle_ipc_request(request: &IpcRequest) -> Result<serde_json::Value> {
    match request.kind {
        IpcKind::AppReady => {
            let cuda_error = check_cuda_error();
            to_value(AppReadyData {
                version: env!("CARGO_PKG_VERSION"),
                platform: std::env::consts::OS,
                backend: "wry/webview2",
                diagnostics: RuntimeDiagnostics::collect_with_error(
                    cuda_error.is_none(),
                    cuda_error,
                ),
            })
        }
        IpcKind::PickImages => {
            let files = FileDialog::new()
                .add_filter("Images", &["png", "jpg", "jpeg", "webp"])
                .set_title("选择要翻译的图片")
                .pick_files()
                .unwrap_or_default();
            to_value(paths_payload(files))
        }
        IpcKind::PickFolder => {
            let folders = FileDialog::new()
                .set_title("选择图片文件夹")
                .pick_folders()
                .unwrap_or_default();
            to_value(paths_payload(folders))
        }
        IpcKind::PickOutputDir => {
            let folder = FileDialog::new().set_title("选择输出目录").pick_folder();
            to_value(paths_payload(folder.into_iter().collect()))
        }
        IpcKind::Defaults => to_value(Settings::default()),
        IpcKind::LoadConfig => to_value(load_saved_settings().unwrap_or_default()),
        IpcKind::SaveConfig => {
            let payload = serde_json::from_value::<SaveConfigPayload>(request.payload.clone())
                .map_err(|err| anyhow!("Invalid config payload: {err}"))?;
            save_settings(&payload.settings)?;
            to_value(serde_json::json!({
                "path": config_path().display().to_string(),
            }))
        }
        IpcKind::PreviewResult => {
            let payload = serde_json::from_value::<PreviewResultPayload>(request.payload.clone())
                .map_err(|err| anyhow!("Invalid preview payload: {err}"))?;
            open_path(&payload.path)?;
            to_value(serde_json::json!({
                "path": payload.path.display().to_string(),
            }))
        }
        IpcKind::ExportResults => {
            let payload = serde_json::from_value::<ExportResultsPayload>(request.payload.clone())
                .map_err(|err| anyhow!("Invalid export payload: {err}"))?;
            export_results(&payload.outputs, &payload.export_dir).and_then(to_value)
        }
        IpcKind::ListDir => {
            let payload = serde_json::from_value::<ListDirPayload>(request.payload.clone())
                .map_err(|err| anyhow!("Invalid listDir payload: {err}"))?;
            to_value(list_dir(&payload.path)?)
        }
        IpcKind::ReadImage => {
            let payload = serde_json::from_value::<ReadImagePayload>(request.payload.clone())
                .map_err(|err| anyhow!("Invalid readImage payload: {err}"))?;
            read_image_data_url(&payload.path).and_then(to_value)
        }
        IpcKind::LoadEditable => {
            let payload = serde_json::from_value::<LoadEditablePayload>(request.payload.clone())
                .map_err(|err| anyhow!("Invalid loadEditable payload: {err}"))?;
            load_editable(&payload.path)
        }
        IpcKind::RerenderExport => {
            let payload = serde_json::from_value::<RerenderExportPayload>(request.payload.clone())
                .map_err(|err| anyhow!("Invalid rerenderExport payload: {err}"))?;
            rerender_export(&payload.path, &payload.edits)
        }
        IpcKind::RevealInExplorer => {
            let payload =
                serde_json::from_value::<RevealInExplorerPayload>(request.payload.clone())
                    .map_err(|err| anyhow!("Invalid revealInExplorer payload: {err}"))?;
            reveal_in_explorer(&payload.path)?;
            to_value(serde_json::json!({
                "path": payload.path.display().to_string(),
            }))
        }
        IpcKind::GetModelsConfig => to_value(load_models_config()),
        IpcKind::SetModelsDir => {
            let mut config = load_models_config();
            if let Some(dir) = FileDialog::new().set_title("选择模型目录").pick_folder() {
                config.model_dir = Some(dir.display().to_string());
                save_models_config(&config)?;
                apply_models_config(&config);
            }
            to_value(config)
        }
        IpcKind::SetAutoDownload => {
            let payload = serde_json::from_value::<SetAutoDownloadPayload>(request.payload.clone())
                .map_err(|err| anyhow!("Invalid setAutoDownload payload: {err}"))?;
            let mut config = load_models_config();
            config.auto_download = payload.value;
            save_models_config(&config)?;
            to_value(config)
        }
        IpcKind::GetModelsStatus => Ok(build_models_status()),
        IpcKind::GetGpuRuntimeStatus => to_value(gpu_runtime::gpu_runtime_status()),
        IpcKind::OpenExternal => {
            let payload = serde_json::from_value::<OpenExternalPayload>(request.payload.clone())
                .map_err(|err| anyhow!("Invalid openExternal payload: {err}"))?;
            open_external_url(&payload.url)?;
            to_value(serde_json::json!({ "url": payload.url }))
        }
        IpcKind::StartTranslation
        | IpcKind::DownloadModels
        | IpcKind::DownloadCudaRuntime
        | IpcKind::RestartApp
        | IpcKind::CheckAppUpdate
        | IpcKind::DownloadAppUpdate
        | IpcKind::InstallAppUpdate => unreachable!("handled asynchronously"),
    }
}

/// Snapshot for the model-management table: configured dir + per-model
/// downloaded/missing status. Readiness is checked without downloading.
fn build_models_status() -> serde_json::Value {
    let dir = load_models_config().model_dir.unwrap_or_default();
    let set = !dir.trim().is_empty();
    serde_json::json!({
        "modelDir": dir,
        "modelDirSet": set,
        "groups": setup::model_catalog(),
    })
}

/// Download the requested model groups (or all missing) on a worker thread,
/// streaming per-file `progress`/`log` events, then resolve with a fresh status
/// snapshot. Mirrors [`handle_start_translation`]'s threading.
fn handle_download_models(request: IpcRequest, proxy: tao::event_loop::EventLoopProxy<UserEvent>) {
    thread::spawn(move || {
        let result = (|| -> Result<serde_json::Value> {
            let payload = serde_json::from_value::<DownloadModelsPayload>(request.payload.clone())
                .unwrap_or_default();
            let configured = load_models_config()
                .model_dir
                .as_deref()
                .map(|dir| !dir.trim().is_empty())
                .unwrap_or(false);
            if !configured {
                return Err(anyhow!(
                    "模型目录未设置：请先在「模型」页选择一个外部文件夹。"
                ));
            }
            let jobs = setup::download_jobs(&payload.targets);
            run_download_jobs(jobs, &proxy);
            Ok(build_models_status())
        })();

        let response = match result {
            Ok(value) => IpcResponse {
                id: request.id,
                ok: true,
                data: Some(value),
                error: None,
            },
            Err(err) => IpcResponse::<serde_json::Value> {
                id: request.id,
                ok: false,
                data: None,
                error: Some(err.to_string()),
            },
        };
        let _ = proxy.send_event(UserEvent::IpcResponse(response));
    });
}

fn handle_download_cuda_runtime(
    request: IpcRequest,
    proxy: tao::event_loop::EventLoopProxy<UserEvent>,
) {
    thread::spawn(move || {
        let result = (|| -> Result<serde_json::Value> {
            let started = Instant::now();
            let mut last_emit: Option<Instant> = None;
            gpu_runtime::download_cuda_runtime(
                |event| {
                    let now = Instant::now();
                    let done = event.total_bytes > 0 && event.downloaded >= event.total_bytes;
                    if !done {
                        if let Some(prev) = last_emit {
                            if now.duration_since(prev) < Duration::from_millis(150) {
                                return;
                            }
                        }
                    }
                    last_emit = Some(now);
                    let percent = if event.total_bytes > 0 {
                        ((event.downloaded as f64 / event.total_bytes as f64) * 100.0)
                            .round()
                            .min(100.0) as u8
                    } else {
                        0
                    };
                    let elapsed = started.elapsed().as_secs_f64().max(0.001);
                    let speed = event.downloaded as f64 / elapsed;
                    let message = if event.total_bytes > 0 {
                        format!(
                            "下载 CUDA runtime {} {} · {} / {} · {}/s",
                            event.package,
                            event.version,
                            fmt_bytes(event.downloaded),
                            fmt_bytes(event.total_bytes),
                            fmt_bytes(speed as u64),
                        )
                    } else {
                        format!(
                            "下载 CUDA runtime {} {} · {} · {}/s",
                            event.package,
                            event.version,
                            fmt_bytes(event.downloaded),
                            fmt_bytes(speed as u64),
                        )
                    };
                    let _ = proxy.send_event(UserEvent::Progress(ProgressEvent {
                        current: event.current,
                        total: event.total,
                        percent,
                        message,
                    }));
                },
                |level, message| send_log(&proxy, level, message),
            )?;
            send_progress(
                &proxy,
                1,
                1,
                format!(
                    "CUDA runtime DLL 下载完成，安装目录：{}",
                    gpu_runtime::cuda_runtime_dir().display()
                ),
            );
            Ok(to_value(gpu_runtime::gpu_runtime_status())?)
        })();

        let response = match result {
            Ok(value) => IpcResponse {
                id: request.id,
                ok: true,
                data: Some(value),
                error: None,
            },
            Err(err) => IpcResponse::<serde_json::Value> {
                id: request.id,
                ok: false,
                data: None,
                error: Some(format!("{err:#}")),
            },
        };
        let _ = proxy.send_event(UserEvent::IpcResponse(response));
    });
}

fn handle_check_app_update(request: IpcRequest, proxy: tao::event_loop::EventLoopProxy<UserEvent>) {
    thread::spawn(move || {
        let result = update::check_app_update().and_then(to_value);
        let response = match result {
            Ok(value) => IpcResponse {
                id: request.id,
                ok: true,
                data: Some(value),
                error: None,
            },
            Err(err) => IpcResponse::<serde_json::Value> {
                id: request.id,
                ok: false,
                data: None,
                error: Some(format!("{err:#}")),
            },
        };
        let _ = proxy.send_event(UserEvent::IpcResponse(response));
    });
}

fn handle_download_app_update(
    request: IpcRequest,
    proxy: tao::event_loop::EventLoopProxy<UserEvent>,
) {
    thread::spawn(move || {
        let started = Instant::now();
        let mut last_emit: Option<Instant> = None;
        let result = update::download_app_update(|event| {
            let now = Instant::now();
            let done = event.total_bytes > 0 && event.downloaded >= event.total_bytes;
            if !done {
                if let Some(prev) = last_emit {
                    if now.duration_since(prev) < Duration::from_millis(150) {
                        return;
                    }
                }
            }
            last_emit = Some(now);
            let percent = if event.total_bytes > 0 {
                ((event.downloaded as f64 / event.total_bytes as f64) * 100.0)
                    .round()
                    .min(100.0) as u8
            } else {
                0
            };
            let elapsed = started.elapsed().as_secs_f64().max(0.001);
            let speed = event.downloaded as f64 / elapsed;
            let message = if event.total_bytes > 0 {
                format!(
                    "下载应用更新 {} · {} / {} · {}/s",
                    event.asset_name,
                    fmt_bytes(event.downloaded),
                    fmt_bytes(event.total_bytes),
                    fmt_bytes(speed as u64),
                )
            } else {
                format!(
                    "下载应用更新 {} · {} · {}/s",
                    event.asset_name,
                    fmt_bytes(event.downloaded),
                    fmt_bytes(speed as u64),
                )
            };
            let _ = proxy.send_event(UserEvent::Progress(ProgressEvent {
                current: 1,
                total: 1,
                percent,
                message,
            }));
        })
        .and_then(to_value);

        let response = match result {
            Ok(value) => IpcResponse {
                id: request.id,
                ok: true,
                data: Some(value),
                error: None,
            },
            Err(err) => IpcResponse::<serde_json::Value> {
                id: request.id,
                ok: false,
                data: None,
                error: Some(format!("{err:#}")),
            },
        };
        let _ = proxy.send_event(UserEvent::IpcResponse(response));
    });
}

fn handle_install_app_update(
    request: IpcRequest,
    proxy: tao::event_loop::EventLoopProxy<UserEvent>,
) {
    thread::spawn(move || {
        let result = update::install_app_update().and_then(to_value);
        let ok = result.is_ok();
        let response = match result {
            Ok(value) => IpcResponse {
                id: request.id,
                ok: true,
                data: Some(value),
                error: None,
            },
            Err(err) => IpcResponse::<serde_json::Value> {
                id: request.id,
                ok: false,
                data: None,
                error: Some(format!("{err:#}")),
            },
        };
        let _ = proxy.send_event(UserEvent::IpcResponse(response));
        if ok {
            let _ = proxy.send_event(UserEvent::ExitForUpdate);
        }
    });
}

/// Download each job in turn, emitting progress/log events. Returns
/// `(succeeded, failed)`. Per-file failures are logged and skipped so one bad
/// download doesn't abort the rest.
fn run_download_jobs(
    jobs: Vec<setup::DownloadJob>,
    proxy: &tao::event_loop::EventLoopProxy<UserEvent>,
) -> (usize, usize) {
    let total = jobs.len();
    if total == 0 {
        send_log(proxy, "info", "没有需要下载的模型（均已就绪）。");
        return (0, 0);
    }
    let mut ok = 0;
    let mut failed = 0;
    let mut skipped = 0;
    for (index, job) in jobs.iter().enumerate() {
        let file_no = index + 1;
        let label = job.label;
        let file = job.file.as_str();
        let started = Instant::now();
        let mut last_emit: Option<Instant> = None;
        // Throttled byte-level progress: speed/ETA/percent for the current file.
        // `current`/`total`/`percent` carry the file count + byte % (the frontend
        // appends them as ` · n/total · p%`), so the message text omits them.
        let mut on_progress = |downloaded: u64, size: u64| {
            let now = Instant::now();
            let done = size > 0 && downloaded >= size;
            if !done {
                if let Some(prev) = last_emit {
                    if now.duration_since(prev) < Duration::from_millis(150) {
                        return;
                    }
                }
            }
            last_emit = Some(now);
            let elapsed = started.elapsed().as_secs_f64().max(0.001);
            let speed = downloaded as f64 / elapsed;
            let percent = if size > 0 {
                ((downloaded as f64 / size as f64) * 100.0)
                    .round()
                    .min(100.0) as u8
            } else {
                0
            };
            let message = if size > 0 {
                let eta = if speed > 1.0 {
                    (size.saturating_sub(downloaded) as f64 / speed) as u64
                } else {
                    0
                };
                format!(
                    "下载 {label} / {file} · {} / {} · {}/s · 剩 {}",
                    fmt_bytes(downloaded),
                    fmt_bytes(size),
                    fmt_bytes(speed as u64),
                    fmt_eta(eta),
                )
            } else {
                format!(
                    "下载 {label} / {file} · {} · {}/s",
                    fmt_bytes(downloaded),
                    fmt_bytes(speed as u64),
                )
            };
            let _ = proxy.send_event(UserEvent::Progress(ProgressEvent {
                current: file_no,
                total,
                percent,
                message,
            }));
        };
        match interface_model::download_model_file(
            job.kind,
            job.name,
            &job.file,
            job.url,
            job.hash,
            &mut on_progress,
        ) {
            Ok(path) => {
                ok += 1;
                send_log(
                    proxy,
                    "success",
                    format!("已下载 {} / {} → {}", job.label, job.file, path.display()),
                );
            }
            Err(err) => {
                let msg = err.to_string();
                // Some catalog variants are listed but never published in the
                // upstream release (404). Treat those as skipped, not failures,
                // so "download all missing" stays clean.
                if msg.contains("404") {
                    skipped += 1;
                    send_log(
                        proxy,
                        "warn",
                        format!("⊘ {} / {} 上游未发布(404)，跳过", job.label, job.file),
                    );
                } else {
                    failed += 1;
                    send_log(
                        proxy,
                        "error",
                        format!("下载失败 {} / {}: {msg}", job.label, job.file),
                    );
                }
            }
        }
    }
    let tail = if skipped > 0 {
        format!("，跳过 {skipped}")
    } else {
        String::new()
    };
    send_progress(
        proxy,
        total,
        total,
        format!("模型下载完成（成功 {ok}，失败 {failed}{tail}）"),
    );
    (ok, failed)
}

/// Human-readable byte size (B/KB/MB/GB) for download progress strings.
fn fmt_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    let b = bytes as f64;
    if b >= GB {
        format!("{:.1} GB", b / GB)
    } else if b >= MB {
        format!("{:.1} MB", b / MB)
    } else if b >= KB {
        format!("{:.0} KB", b / KB)
    } else {
        format!("{bytes} B")
    }
}

/// Compact ETA string (`—` when unknown).
fn fmt_eta(secs: u64) -> String {
    if secs == 0 {
        "—".to_string()
    } else if secs >= 60 {
        format!("{}分{}秒", secs / 60, secs % 60)
    } else {
        format!("{secs}秒")
    }
}

/// List the immediate children of `path` for the left-panel file tree:
/// directories (kept for further browsing) plus supported image files only.
/// Directories first, then files, each sorted case-insensitively by name.
fn list_dir(path: &Path) -> Result<ListDirData> {
    let mut dirs = Vec::new();
    let mut files = Vec::new();
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            dirs.push(DirEntryInfo {
                name,
                path: entry_path.display().to_string(),
                is_dir: true,
                is_image: false,
            });
        } else if file_type.is_file() && is_supported_image(&entry_path) {
            files.push(DirEntryInfo {
                name,
                path: entry_path.display().to_string(),
                is_dir: false,
                is_image: true,
            });
        }
    }
    let by_name =
        |a: &DirEntryInfo, b: &DirEntryInfo| a.name.to_lowercase().cmp(&b.name.to_lowercase());
    dirs.sort_by(by_name);
    files.sort_by(by_name);
    dirs.extend(files);
    Ok(ListDirData { entries: dirs })
}

fn handle_start_translation(
    request: IpcRequest,
    proxy: tao::event_loop::EventLoopProxy<UserEvent>,
    models: ModelPool,
) {
    thread::spawn(move || {
        let result = (|| -> Result<serde_json::Value> {
            let payload =
                serde_json::from_value::<StartTranslationPayload>(request.payload.clone())
                    .map_err(|err| anyhow!("Invalid translation payload: {err}"))?;
            validate_translation_payload(&payload)?;
            let runtime = tokio::runtime::Runtime::new()?;
            let result = runtime.block_on(run_translation_job(payload, models, proxy.clone()))?;
            to_value(result)
        })();

        let response = match result {
            Ok(value) => IpcResponse {
                id: request.id,
                ok: true,
                data: Some(value),
                error: None,
            },
            Err(err) => IpcResponse::<serde_json::Value> {
                id: request.id,
                ok: false,
                data: None,
                error: Some(err.to_string()),
            },
        };

        let _ = proxy.send_event(UserEvent::IpcResponse(response));
    });
}

fn config_path() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("config")
        .join("app.json")
}

fn load_saved_settings() -> Result<Settings> {
    let path = config_path();
    let content = read_to_string(&path)?;
    serde_json::from_str(&content).map_err(Into::into)
}

fn models_config_path() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("config")
        .join("models.json")
}

fn load_models_config() -> ModelsConfig {
    read_to_string(models_config_path())
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_default()
}

fn save_models_config(config: &ModelsConfig) -> Result<()> {
    let path = models_config_path();
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(config)?;
    File::create(path)?.write_all(content.as_bytes())?;
    Ok(())
}

/// Push the configured model dir into the global root. A set, non-empty dir is
/// used as-is; otherwise the WebView requires an explicit choice (errors on
/// download/inference) rather than falling back to the wiped portable dir.
fn apply_models_config(config: &ModelsConfig) {
    match config.model_dir.as_deref().map(str::trim) {
        Some(dir) if !dir.is_empty() => {
            set_model_root(ModelRootMode::Configured(PathBuf::from(dir)));
        }
        _ => set_model_root(ModelRootMode::RequireConfigured),
    }
}

fn save_settings(settings: &Settings) -> Result<()> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(settings)?;
    File::create(path)?.write_all(content.as_bytes())?;
    Ok(())
}

/// Persisted native window geometry so the app reopens at the user's last
/// size, position and maximized state instead of the hardcoded default.
#[derive(Debug, Serialize, Deserialize)]
struct WindowState {
    width: u32,
    height: u32,
    #[serde(default)]
    x: Option<i32>,
    #[serde(default)]
    y: Option<i32>,
    #[serde(default)]
    maximized: bool,
}

fn window_state_path() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("config")
        .join("window.json")
}

fn load_window_state() -> Option<WindowState> {
    let content = read_to_string(window_state_path()).ok()?;
    serde_json::from_str::<WindowState>(&content).ok()
}

fn save_window_state(state: &WindowState) -> Result<()> {
    let path = window_state_path();
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(state)?;
    File::create(path)?.write_all(content.as_bytes())?;
    Ok(())
}

fn save_window_state_from(window: &Window) {
    let size = window.inner_size();
    let position = window.outer_position().ok();
    let state = WindowState {
        width: size.width,
        height: size.height,
        x: position.map(|p| p.x),
        y: position.map(|p| p.y),
        maximized: window.is_maximized(),
    };
    if let Err(err) = save_window_state(&state) {
        eprintln!("Failed to persist window state: {err}");
    }
}

fn validate_translation_payload(payload: &StartTranslationPayload) -> Result<()> {
    if payload.input_paths.is_empty() {
        return Err(anyhow!(
            "Please choose at least one image or a folder first."
        ));
    }

    match payload.output_format.as_str() {
        "html" | "raw" | "png" => {}
        value => return Err(anyhow!("Unsupported output format: {value}")),
    }

    serde_json::to_value(&payload.settings)?;
    Ok(())
}

async fn run_translation_job(
    mut payload: StartTranslationPayload,
    models: ModelPool,
    proxy: tao::event_loop::EventLoopProxy<UserEvent>,
) -> Result<StartTranslationResult> {
    let job_timer = Instant::now();
    let logger = JobLogger::create(logs_dir())?;
    send_log(
        &proxy,
        "info",
        format!("日志文件: {}", logger.path().display()),
    );
    logger.log("info", format!("log_file={}", logger.path().display()));

    if base_util::onnx::set_device_mode(payload.device_mode) {
        let mut guard = models.lock().await;
        *guard = None;
        logger.log("info", "device changed -> reloading models");
        send_log(&proxy, "info", "device changed -> reloading models");
    }

    let cuda_error = check_cuda_error();
    let cuda = cuda_error.is_none();
    let diagnostics = RuntimeDiagnostics::collect_with_error(cuda, cuda_error.clone());
    let device_mode = base_util::onnx::device_mode();
    let require_cuda = device_mode == base_util::onnx::DeviceMode::Cuda;
    let effective_provider_status = match device_mode {
        base_util::onnx::DeviceMode::Cpu => "CPU forced".to_owned(),
        base_util::onnx::DeviceMode::Cuda if cuda => "CUDA available".to_owned(),
        base_util::onnx::DeviceMode::Cuda => "CUDA unavailable".to_owned(),
        base_util::onnx::DeviceMode::Auto => diagnostics.provider_status.clone(),
    };
    logger.log(
        "info",
        format!(
            "runtime cuda_feature={}, cuda_available={}, device_mode={:?}, require_cuda={}, provider_status={}, cuda_error={}",
            diagnostics.cuda_feature,
            diagnostics.cuda_available,
            device_mode,
            require_cuda,
            effective_provider_status,
            diagnostics.cuda_error.as_deref().unwrap_or("")
        ),
    );
    match device_mode {
        base_util::onnx::DeviceMode::Cpu => {
            logger.log(
                "info",
                "CPU inference is forced; GPU providers will be skipped.",
            );
        }
        base_util::onnx::DeviceMode::Auto if cuda => {
            logger.log(
                "info",
                "CUDA is available and will be preferred automatically. Force CUDA only disables CPU fallback.",
            );
        }
        _ => {}
    }
    send_log(
        &proxy,
        match device_mode {
            base_util::onnx::DeviceMode::Cpu => "info",
            _ if cuda => "success",
            _ => "warn",
        },
        format!("推理状态: {effective_provider_status}"),
    );
    if device_mode == base_util::onnx::DeviceMode::Cuda && !cuda {
        anyhow::bail!(
            "CUDA was requested, but CUDA is not available. {}",
            diagnostics
                .cuda_error
                .as_deref()
                .unwrap_or("Check NVIDIA driver, CUDA/cuDNN, and ONNX Runtime CUDA provider DLLs.")
        );
    }
    ensure_cuda_policy(cuda)?;

    let renderer = renderer_from_web_value(&payload.output_format)?;
    payload.settings.render.renderer = renderer;
    payload.max_parallel_images = payload.max_parallel_images.clamp(1, 8);
    payload.max_parallel_gpu_jobs = payload.max_parallel_gpu_jobs.clamp(1, 4);
    logger.log(
        "info",
        format!(
            "concurrency max_parallel_images={}, max_parallel_gpu_jobs={}",
            payload.max_parallel_images, payload.max_parallel_gpu_jobs
        ),
    );
    logger.log(
        "info",
        format!(
            "debug diagnostics dump={} (timing log always on)",
            payload.debug
        ),
    );

    // Translation always writes to an internal temp dir (preview reads from
    // there). Saving to a user folder is the job of "Export selected".
    let output_dir = internal_results_dir();
    create_dir_all(&output_dir)?;

    let inputs = expand_input_paths(&payload.input_paths)?;
    if inputs.is_empty() {
        return Err(anyhow!("No supported image files were found."));
    }
    let total = inputs.len();

    send_progress(&proxy, 0, total, "正在准备模型");
    let timer = StageTimer::start("model-prepare", Some(&logger));
    ensure_models(&models, &logger).await?;
    timer.finish();
    send_log(&proxy, "info", format!("模型已就绪，开始处理 {total} 张"));

    let output_slots: Arc<AsyncMutex<Vec<Option<TranslationOutput>>>> =
        Arc::new(AsyncMutex::new((0..total).map(|_| None).collect()));
    let image_semaphore = Arc::new(Semaphore::new(payload.max_parallel_images));
    let gpu_semaphore = Arc::new(Semaphore::new(payload.max_parallel_gpu_jobs));
    let mut handles = Vec::with_capacity(total);

    for (index, input) in inputs.into_iter().enumerate() {
        send_progress(
            &proxy,
            index,
            total,
            format!(
                "正在处理 {}",
                input
                    .file_name()
                    .and_then(|v| v.to_str())
                    .unwrap_or("image")
            ),
        );
        logger.log(
            "info",
            format!(
                "image {} / {} started: {}",
                index + 1,
                total,
                input.display()
            ),
        );

        let output_dir = output_dir.clone();
        let settings = payload.settings.clone();
        let models = models.clone();
        let proxy = proxy.clone();
        let logger = logger.clone();
        let output_slots = output_slots.clone();
        let image_semaphore = image_semaphore.clone();
        let gpu_semaphore = gpu_semaphore.clone();
        let debug = payload.debug;
        handles.push(tokio::spawn(async move {
            let _image_permit = image_semaphore.acquire_owned().await?;
            let result = process_one(
                &input,
                &output_dir,
                &settings,
                &models,
                &proxy,
                &logger,
                &gpu_semaphore,
                index,
                total,
                debug,
            )
            .await;
            let output = match result {
                Ok(Some(output)) => {
                    send_progress(&proxy, index + 1, total, "已完成");
                    logger.log("info", format!("image {} finished", index + 1));
                    TranslationOutput {
                        input: input.display().to_string(),
                        file_name: output
                            .file_name()
                            .and_then(|value| value.to_str())
                            .map(str::to_owned),
                        output: Some(output.display().to_string()),
                        status: "done".to_owned(),
                        message: "完成".to_owned(),
                        editable: editable_sidecar(&output),
                    }
                }
                Ok(None) => {
                    send_progress(&proxy, index + 1, total, "未检测到文本");
                    logger.log("warn", format!("image {} skipped: no text", index + 1));
                    TranslationOutput {
                        input: input.display().to_string(),
                        output: None,
                        file_name: None,
                        status: "skipped".to_owned(),
                        message: "未检测到可翻译文本".to_owned(),
                        editable: None,
                    }
                }
                Err(err) => {
                    logger.log("error", format!("image {} failed: {err}", index + 1));
                    send_log(&proxy, "error", format!("处理失败: {err}"));
                    send_progress(&proxy, index + 1, total, "处理失败");
                    TranslationOutput {
                        input: input.display().to_string(),
                        output: None,
                        file_name: None,
                        status: "failed".to_owned(),
                        message: err.to_string(),
                        editable: None,
                    }
                }
            };
            output_slots.lock().await[index] = Some(output);
            anyhow::Ok(())
        }));
    }

    for handle in handles {
        handle.await??;
    }

    let outputs = Arc::try_unwrap(output_slots)
        .map_err(|_| anyhow!("Failed to unwrap output slots"))?
        .into_inner()
        .into_iter()
        .map(|item| item.ok_or_else(|| anyhow!("Missing translation output")))
        .collect::<Result<Vec<_>>>()?;

    let failed = outputs
        .iter()
        .filter(|item| item.status == "failed")
        .count();
    let done = outputs.iter().filter(|item| item.status == "done").count();
    if let Some(samples) = sample_nvidia_gpu_memory() {
        for sample in samples {
            logger.log(
                "info",
                format!(
                    "gpu-memory job-end: {} {} MiB / {} MiB",
                    sample.name, sample.used_mb, sample.total_mb
                ),
            );
        }
    }
    let elapsed = format_duration(job_timer.elapsed());
    logger.log("info", format!("job finished in {elapsed}"));
    send_log(&proxy, "success", format!("任务耗时: {elapsed}"));
    Ok(StartTranslationResult {
        status: if failed == 0 { "done" } else { "partial" }.to_owned(),
        message: format!(
            "已完成 {done} 张，失败 {failed} 张。日志：{}",
            logger.path().display()
        ),
        outputs,
    })
}

fn send_progress(
    proxy: &tao::event_loop::EventLoopProxy<UserEvent>,
    current: usize,
    total: usize,
    message: impl Into<String>,
) {
    let percent = if total == 0 {
        0
    } else {
        ((current as f32 / total as f32) * 100.0).round() as u8
    }
    .min(100);
    let _ = proxy.send_event(UserEvent::Progress(ProgressEvent {
        current,
        total,
        percent,
        message: message.into(),
    }));
}

fn send_log(
    proxy: &tao::event_loop::EventLoopProxy<UserEvent>,
    level: impl Into<String>,
    message: impl Into<String>,
) {
    let _ = proxy.send_event(UserEvent::Log(LogEvent {
        level: level.into(),
        message: message.into(),
    }));
}

fn internal_results_dir() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("results")
        .join("webview")
        .join(format!("job_{}", uuid::Uuid::new_v4()))
}

fn logs_dir() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("logs")
}

async fn ensure_models(models: &ModelPool, logger: &JobLogger) -> Result<Arc<Models>> {
    let mut guard = models.lock().await;
    if let Some(existing) = guard.as_ref() {
        logger.log("info", "shared model registry already initialized");
        return Ok(existing.clone());
    }

    let cuda = check_cuda_error().is_none();
    logger.log(
        "info",
        format!(
            "initializing shared model registry: cuda_feature={}, cuda_available={cuda}",
            cfg!(feature = "cuda")
        ),
    );
    let instance = Arc::new(Models::new(2, 16, true, cuda).await);
    *guard = Some(instance.clone());
    Ok(instance)
}

async fn process_one(
    input: &Path,
    output_dir: &Path,
    settings: &Settings,
    models: &ModelPool,
    proxy: &tao::event_loop::EventLoopProxy<UserEvent>,
    logger: &JobLogger,
    gpu_semaphore: &Arc<Semaphore>,
    index: usize,
    total: usize,
    debug: bool,
) -> Result<Option<PathBuf>> {
    let image_timer = Instant::now();
    let file_name = input
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("image")
        .to_owned();
    send_log(
        proxy,
        "info",
        format!("▶ [{}/{}] {}", index + 1, total, file_name),
    );
    let img = image::open(input)?;
    let debug_path = if debug {
        Some(diagnostics_path(logger, index, input)?)
    } else {
        None
    };
    let exp = {
        let _gpu_permit = gpu_semaphore.acquire().await?;
        let model_state = shared_models(models, logger).await?;
        // Stream pipeline stages to the terminal. The status bar always shows the
        // current stage; in debug mode each line also reports the *previous*
        // stage's elapsed time (timed between callbacks), nested under the ▶ line.
        let mut last_stage: Option<(&'static str, Instant)> = None;
        let result = {
            let mut stage_sender = |stage: &'static str| {
                send_progress(proxy, index, total, format!("{}：{}", file_name, stage));
                if debug {
                    let now = Instant::now();
                    if let Some((prev, started)) = last_stage.take() {
                        send_log(
                            proxy,
                            "info",
                            format!(
                                "    {} · {} {}",
                                file_name,
                                prev,
                                format_duration(started.elapsed())
                            ),
                        );
                    }
                    last_stage = Some((stage, now));
                }
            };
            model_state
                .execute_with_progress_and_logger(
                    img,
                    settings,
                    debug_path,
                    Some(&mut stage_sender),
                    Some(logger),
                )
                .await?
        };
        if debug {
            if let Some((prev, started)) = last_stage {
                send_log(
                    proxy,
                    "info",
                    format!(
                        "    {} · {} {}",
                        file_name,
                        prev,
                        format_duration(started.elapsed())
                    ),
                );
            }
        }
        result
    };
    let Some(exp) = exp else {
        send_log(proxy, "warn", format!("⊘ {} · 未检测到文本", file_name));
        return Ok(None);
    };

    send_progress(proxy, index, total, format!("{}：渲染嵌字", file_name));
    let mut output = output_dir.join(
        input
            .file_name()
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("output")),
    );
    output.set_extension(settings.render.renderer.extension());
    prepare_renderer_assets(&output, &settings.render.renderer)?;
    let render_timer = StageTimer::start("render", Some(logger));
    // P5.0: persist the editable Export as a sidecar next to the output, then reload
    // it to render the flattened image. The raw blob carries the original image, the
    // inpainted background and every text block, so the GUI can later edit text or
    // position and re-render without rerunning any model. Only the PNG renderer is
    // editable (re-rendering produces a PNG), so skip the sidecar for html/raw.
    let exp = if matches!(settings.render.renderer, Renderer::Png) {
        let editable_bytes = exp.export();
        let sidecar = output.with_extension(EDITABLE_EXT);
        if let Err(err) = File::create(&sidecar).and_then(|mut f| f.write_all(&editable_bytes)) {
            logger.log("warn", format!("failed to write editable sidecar: {err}"));
        }
        export::Export::load(editable_bytes)
            .ok_or_else(|| anyhow!("failed to reload editable export for {file_name}"))?
    } else {
        exp
    };
    let data = render_export_bytes_with_settings(exp, settings)?;
    render_timer.finish();
    let write_timer = StageTimer::start("write", Some(logger));
    File::create(&output)?.write_all(&data)?;
    write_timer.finish();
    logger.log(
        "info",
        format!(
            "image output written: {} ({})",
            output.display(),
            format_duration(image_timer.elapsed())
        ),
    );
    send_log(
        proxy,
        "success",
        format!(
            "✓ {} · {}",
            file_name,
            format_duration(image_timer.elapsed())
        ),
    );
    Ok(Some(output))
}

async fn shared_models(models: &ModelPool, logger: &JobLogger) -> Result<Arc<Models>> {
    if let Some(model) = models.lock().await.as_ref() {
        return Ok(model.clone());
    }
    // Should not happen: ensure_models runs before any image. Initialize defensively.
    ensure_models(models, logger).await
}

fn diagnostics_path(logger: &JobLogger, index: usize, input: &Path) -> Result<PathBuf> {
    let log_stem = logger
        .path()
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("job");
    let root = logger
        .path()
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(format!("{log_stem}_diagnostics"));
    let image_stem = input
        .file_stem()
        .and_then(|value| value.to_str())
        .map(sanitize_path_component)
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "image".to_owned());
    let path = root.join(format!("{:03}_{image_stem}", index + 1));
    create_dir_all(&path)?;
    logger.log(
        "info",
        format!("image {} diagnostics_dir={}", index + 1, path.display()),
    );
    Ok(path)
}

fn sanitize_path_component(value: &str) -> String {
    value
        .chars()
        .map(|ch| match ch {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
            ch if ch.is_control() => '_',
            ch => ch,
        })
        .take(80)
        .collect()
}

fn open_path(path: &Path) -> Result<()> {
    if !path.exists() {
        return Err(anyhow!("Preview file does not exist: {}", path.display()));
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("explorer.exe").arg(path).spawn()?;
        return Ok(());
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg(path).spawn()?;
        return Ok(());
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        Command::new("xdg-open").arg(path).spawn()?;
        return Ok(());
    }
}

/// Reveal a file/folder in the OS file manager, selecting it where supported.
fn reveal_in_explorer(path: &Path) -> Result<()> {
    if !path.exists() {
        return Err(anyhow!("Path does not exist: {}", path.display()));
    }

    #[cfg(target_os = "windows")]
    {
        // `explorer /select,<path>` opens the parent and highlights the entry.
        // explorer.exe needs the *path* quoted (not the whole `/select,...` token),
        // so build the command line verbatim with `raw_arg` — letting std quote the
        // argument wraps `/select,...` itself and explorer silently opens Documents.
        use std::os::windows::process::CommandExt;
        Command::new("explorer.exe")
            .raw_arg(format!("/select,\"{}\"", path.display()))
            .spawn()?;
        return Ok(());
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg("-R").arg(path).spawn()?;
        return Ok(());
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        // No portable "select" on Linux; open the containing directory.
        let target = if path.is_dir() {
            path
        } else {
            path.parent().unwrap_or(path)
        };
        Command::new("xdg-open").arg(target).spawn()?;
        return Ok(());
    }
}

fn open_external_url(url: &str) -> Result<()> {
    let trimmed = url.trim();
    if !(trimmed.starts_with("https://") || trimmed.starts_with("http://")) {
        return Err(anyhow!("Only http(s) URLs can be opened: {trimmed}"));
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("rundll32.exe")
            .args(["url.dll,FileProtocolHandler", trimmed])
            .spawn()?;
        return Ok(());
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg(trimmed).spawn()?;
        return Ok(());
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        Command::new("xdg-open").arg(trimmed).spawn()?;
        return Ok(());
    }
}

fn export_results(outputs: &[PathBuf], export_dir: &Path) -> Result<ExportResultsData> {
    if outputs.is_empty() {
        return Err(anyhow!("Please select at least one finished result."));
    }
    create_dir_all(export_dir)?;

    let mut exported = Vec::with_capacity(outputs.len());
    for output in outputs {
        if !output.is_file() {
            return Err(anyhow!("Result file does not exist: {}", output.display()));
        }
        let file_name = output
            .file_name()
            .ok_or_else(|| anyhow!("Invalid result file path: {}", output.display()))?;
        let target = unique_target_path(export_dir, Path::new(file_name));
        copy(output, &target)?;
        exported.push(target.display().to_string());
    }

    Ok(ExportResultsData { exported })
}

fn unique_target_path(dir: &Path, file_name: &Path) -> PathBuf {
    let candidate = dir.join(file_name);
    if !candidate.exists() {
        return candidate;
    }

    let stem = file_name
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("result");
    let ext = file_name.extension().and_then(|value| value.to_str());
    for index in 1.. {
        let name = match ext {
            Some(ext) if !ext.is_empty() => format!("{stem}_{index}.{ext}"),
            _ => format!("{stem}_{index}"),
        };
        let candidate = dir.join(name);
        if !candidate.exists() {
            return candidate;
        }
    }
    unreachable!()
}

fn renderer_from_web_value(value: &str) -> Result<Renderer> {
    match value {
        "png" => Ok(Renderer::Png),
        "html" => Ok(Renderer::Html),
        "raw" | "mit" | "mit.bin" => Ok(Renderer::Raw),
        value => Err(anyhow!("Unsupported output format: {value}")),
    }
}

fn expand_input_paths(paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut result = Vec::new();
    for path in paths {
        if path.is_file() {
            if is_supported_image(path) {
                result.push(path.clone());
            }
        } else if path.is_dir() {
            for entry in walkdir::WalkDir::new(path)
                .into_iter()
                .filter_map(|entry| entry.ok())
            {
                let path = entry.path();
                if path.is_file() && is_supported_image(path) {
                    result.push(path.to_path_buf());
                }
            }
        }
    }
    result.sort();
    result.dedup();
    Ok(result)
}

fn is_supported_image(path: &Path) -> bool {
    path.extension()
        .and_then(|value| value.to_str())
        .map(|ext| {
            matches!(
                ext.to_ascii_lowercase().as_str(),
                "png" | "jpg" | "jpeg" | "webp"
            )
        })
        .unwrap_or(false)
}

fn paths_payload(paths: Vec<PathBuf>) -> PickedPaths {
    PickedPaths {
        paths: paths
            .into_iter()
            .map(|path| path.display().to_string())
            .collect(),
    }
}

fn to_value<T: Serialize>(value: T) -> Result<serde_json::Value> {
    serde_json::to_value(value).map_err(Into::into)
}

fn resolve_request<T: Serialize>(webview: &wry::WebView, response: IpcResponse<T>) {
    let script = match serde_json::to_string(&response) {
        Ok(json) => format!("window.MIT_BRIDGE && window.MIT_BRIDGE.resolve({json});"),
        Err(err) => format!(
            "window.MIT_BRIDGE && window.MIT_BRIDGE.emit('log', {{ level: 'error', message: {} }});",
            js_string(&format!("Failed to serialize IPC response: {err}"))
        ),
    };

    if let Err(err) = webview.evaluate_script(&script) {
        eprintln!("Failed to evaluate IPC response script: {err}");
    }
}

fn send_event<T: Serialize>(webview: &wry::WebView, name: &str, payload: T) {
    let Ok(payload) = serde_json::to_string(&payload) else {
        return;
    };
    let script = format!(
        "window.MIT_BRIDGE && window.MIT_BRIDGE.emit({}, {});",
        js_string(name),
        payload
    );

    if let Err(err) = webview.evaluate_script(&script) {
        eprintln!("Failed to evaluate event script: {err}");
    }
}

fn js_string(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "\"\"".to_string())
}
