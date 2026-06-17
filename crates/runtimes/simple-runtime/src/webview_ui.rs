use std::{
    fs::{copy, create_dir_all, read_to_string, File},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    sync::Arc,
    thread,
    time::Instant,
};

use anyhow::{anyhow, Result};
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use tao::{
    dpi::LogicalSize,
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
};
use tokio::sync::{Mutex as AsyncMutex, Semaphore};
use wry::http::{header::CONTENT_TYPE, Response};
use wry::WebViewBuilder;

use crate::{
    perf::{
        ensure_cuda_policy, format_duration, sample_nvidia_gpu_memory, JobLogger,
        RuntimeDiagnostics, StageTimer,
    },
    prepare_renderer_assets, render_export_bytes_with_settings,
    settings::{Renderer, Settings},
    setup::Models,
    update::check_cuda_error,
};

const INDEX_HTML: &str = include_str!("../webview/index.html");
const STYLES_CSS: &str = include_str!("../webview/styles.css");
const APP_JS: &str = include_str!("../webview/app.js");
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
    require_cuda: bool,
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

fn default_max_parallel_images() -> usize {
    2
}

fn default_max_parallel_gpu_jobs() -> usize {
    1
}

pub fn run() -> Result<()> {
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();
    let ipc_proxy = proxy.clone();
    let models: ModelPool = Arc::new(AsyncMutex::new(None));
    let window = WindowBuilder::new()
        .with_title("漫画图片翻译器")
        .with_inner_size(LogicalSize::new(1180.0, 780.0))
        .with_min_inner_size(LogicalSize::new(960.0, 640.0))
        .build(&event_loop)?;

    let webview = WebViewBuilder::new()
        .with_custom_protocol("mit".into(), move |_webview_id, _request| {
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
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
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
        IpcKind::StartTranslation => unreachable!("handled asynchronously"),
    }
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

fn save_settings(settings: &Settings) -> Result<()> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(settings)?;
    File::create(path)?.write_all(content.as_bytes())?;
    Ok(())
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

    let cuda_error = check_cuda_error();
    let cuda = cuda_error.is_none();
    base_util::onnx::set_require_cuda_override(payload.require_cuda);
    let diagnostics = RuntimeDiagnostics::collect_with_error(cuda, cuda_error.clone());
    let effective_require_cuda = diagnostics.require_cuda || payload.require_cuda;
    let effective_provider_status = if cuda {
        "CUDA available".to_owned()
    } else if effective_require_cuda {
        "CUDA unavailable".to_owned()
    } else {
        diagnostics.provider_status.clone()
    };
    logger.log(
        "info",
        format!(
            "runtime cuda_feature={}, cuda_available={}, require_cuda={}, gui_require_cuda={}, provider_status={}, cuda_error={}",
            diagnostics.cuda_feature,
            diagnostics.cuda_available,
            diagnostics.require_cuda,
            payload.require_cuda,
            effective_provider_status,
            diagnostics.cuda_error.as_deref().unwrap_or("")
        ),
    );
    if cuda && !payload.require_cuda && !diagnostics.require_cuda {
        logger.log(
            "info",
            "CUDA is available and will be preferred automatically. Force CUDA only disables CPU fallback.",
        );
    }
    send_log(
        &proxy,
        if diagnostics.cuda_available {
            "success"
        } else {
            "warn"
        },
        format!("推理状态: {effective_provider_status}"),
    );
    if payload.require_cuda && !cuda {
        anyhow::bail!(
            "GUI requested CUDA, but CUDA is not available. {}",
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
    let img = image::open(input)?;
    let debug_path = if debug {
        Some(diagnostics_path(logger, index, input)?)
    } else {
        None
    };
    let exp = {
        let _gpu_permit = gpu_semaphore.acquire().await?;
        let model_state = shared_models(models, logger).await?;
        let file_name = input
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or("image")
            .to_owned();
        let mut stage_sender = |stage: &'static str| {
            send_progress(proxy, index, total, format!("{}：{}", file_name, stage));
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
    let Some(exp) = exp else {
        return Ok(None);
    };

    send_progress(
        proxy,
        index,
        total,
        format!(
            "{}：渲染嵌字",
            input
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or("image")
        ),
    );
    let mut output = output_dir.join(
        input
            .file_name()
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("output")),
    );
    output.set_extension(settings.render.renderer.extension());
    prepare_renderer_assets(&output, &settings.render.renderer)?;
    let render_timer = StageTimer::start("render", Some(logger));
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
