use std::sync::atomic::{AtomicU8, Ordering};

use log::{info, warn, Level};
use ndarray::{Array, Array2, IxDyn};
use ort::{
    execution_providers::{
        ArenaExtendStrategy, CUDAExecutionProvider, CoreMLExecutionProvider,
        DirectMLExecutionProvider, ROCmExecutionProvider, TensorRTExecutionProvider,
    },
    session::{
        builder::{GraphOptimizationLevel, SessionBuilder},
        Session,
    },
};

#[derive(Clone, Debug)]
pub enum Providers {
    TensorRT,
    CUDA,
    DirectML,
    CoreML,
    RocM,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeviceMode {
    Auto,
    Cuda,
    Cpu,
}

impl Default for DeviceMode {
    fn default() -> Self {
        Self::Auto
    }
}

static DEVICE_MODE: AtomicU8 = AtomicU8::new(device_mode_to_u8(DeviceMode::Auto));

const fn device_mode_to_u8(mode: DeviceMode) -> u8 {
    match mode {
        DeviceMode::Auto => 0,
        DeviceMode::Cuda => 1,
        DeviceMode::Cpu => 2,
    }
}

fn device_mode_from_u8(value: u8) -> DeviceMode {
    match value {
        1 => DeviceMode::Cuda,
        2 => DeviceMode::Cpu,
        _ => DeviceMode::Auto,
    }
}

pub fn set_device_mode(mode: DeviceMode) -> bool {
    let previous = DEVICE_MODE.swap(device_mode_to_u8(mode), Ordering::Relaxed);
    previous != device_mode_to_u8(mode)
}

pub fn device_mode() -> DeviceMode {
    let mode = device_mode_from_u8(DEVICE_MODE.load(Ordering::Relaxed));
    if mode == DeviceMode::Auto && env_require_cuda() {
        DeviceMode::Cuda
    } else {
        mode
    }
}

pub fn set_require_cuda_override(require_cuda: bool) {
    let mode = if require_cuda {
        DeviceMode::Cuda
    } else {
        DeviceMode::Auto
    };
    set_device_mode(mode);
}

pub fn all_providers() -> Vec<Providers> {
    vec![
        Providers::CUDA,
        Providers::RocM,
        Providers::TensorRT,
        #[cfg(target_os = "windows")]
        Providers::DirectML,
        #[cfg(target_os = "macos")]
        Providers::CoreML,
    ]
}

pub fn gpu_providers() -> Vec<Providers> {
    vec![
        Providers::CUDA,
        Providers::RocM,
        Providers::TensorRT,
        #[cfg(target_os = "windows")]
        Providers::DirectML,
        Providers::RocM,
    ]
}

pub fn new_session(providers: &[Providers]) -> anyhow::Result<SessionBuilder> {
    new_session_(Session::builder()?, providers)
}

pub fn new_session_(
    session_builder: SessionBuilder,
    providers: &[Providers],
) -> anyhow::Result<SessionBuilder> {
    let mode = device_mode();
    info!("Creating ONNX Runtime session: device_mode={mode:?}, provider_order={providers:?}");

    let session_builder = session_builder
        .with_logger(Box::new(
            |level: ort::logging::LogLevel,
             category: &str,
             id: &str,
             code_location: &str,
             message: &str| {
                let log_level = match level {
                    ort::logging::LogLevel::Verbose => Level::Trace,
                    ort::logging::LogLevel::Info => Level::Info,
                    ort::logging::LogLevel::Warning => Level::Warn,
                    ort::logging::LogLevel::Error => Level::Error,
                    ort::logging::LogLevel::Fatal => Level::Error,
                };

                log::log!(
                    log_level,
                    "[ORT][{category}][{id}] {message} (at {code_location})"
                );
            },
        ))?
        .with_optimization_level(GraphOptimizationLevel::Level3)?
        .with_parallel_execution(true)?
        .with_intra_threads(4)?
        .with_inter_threads(2)?;

    if mode == DeviceMode::Cpu {
        info!("Execution provider CPU in use");
        return Ok(session_builder);
    }

    if mode == DeviceMode::Cuda {
        let provider_ = Providers::CUDA;
        let provider = build_provider(&provider_).error_on_failure();
        return match session_builder
            .clone()
            .with_execution_providers(vec![provider])
        {
            Ok(session_builder) => {
                info!("Execution provider {:?} in use", provider_);
                Ok(session_builder)
            }
            Err(err) => {
                warn!("Execution provider {:?} unavailable: {err}", provider_);
                anyhow::bail!(
                    "CUDA device mode requested, but the CUDA execution provider is unavailable. {} Original error: {err}",
                    cuda_error_hint(&err.to_string())
                );
            }
        };
    }

    for provider_ in providers {
        let provider = build_provider(provider_).error_on_failure();
        match session_builder
            .clone()
            .with_execution_providers(vec![provider])
        {
            Ok(session_builder) => {
                info!("Execution provider {:?} in use", provider_);
                return Ok(session_builder);
            }
            Err(err) => {
                warn!("Execution provider {:?} unavailable: {err}", provider_);
            }
        }
    }
    info!("Execution provider CPU in use");
    Ok(session_builder)
}

fn build_provider(provider: &Providers) -> ort::execution_providers::ExecutionProviderDispatch {
    match provider {
        Providers::TensorRT => TensorRTExecutionProvider::default()
            .with_device_id(0)
            .build(),
        Providers::CUDA => CUDAExecutionProvider::default()
            .with_device_id(0)
            .with_arena_extend_strategy(ArenaExtendStrategy::SameAsRequested)
            .build(),
        Providers::DirectML => DirectMLExecutionProvider::default()
            .with_device_id(0)
            .build(),
        Providers::CoreML => CoreMLExecutionProvider::default()
            .with_model_cache_dir("models/cache")
            .with_compute_units(ort::execution_providers::coreml::CoreMLComputeUnits::All)
            .build(),
        Providers::RocM => ROCmExecutionProvider::default().with_device_id(0).build(),
    }
}

fn env_require_cuda() -> bool {
    std::env::var("MIT_REQUIRE_CUDA")
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
}

fn cuda_error_hint(error: &str) -> String {
    let lower = error.to_ascii_lowercase();
    let known_dlls = [
        "cublaslt64_12.dll",
        "cublas64_12.dll",
        "cufft64_11.dll",
        "cudart64_12.dll",
        "cudnn64_9.dll",
    ];
    let missing = known_dlls.iter().copied().find(|dll| lower.contains(dll));

    match missing {
        Some(dll) => format!(
            "Missing CUDA runtime dependency: {dll}. Install NVIDIA CUDA Toolkit 12.x or rebuild the portable package with -CudaRuntimeDir pointing at the CUDA bin directory."
        ),
        None => "Check NVIDIA driver, CUDA/cuDNN runtime DLLs, and ONNX Runtime CUDA provider compatibility.".to_owned(),
    }
}

pub fn dyn_to_2d(arr: Array<f32, IxDyn>) -> Option<Array2<f32>> {
    if arr.ndim() == 2 {
        let shape = arr.shape();
        let (rows, cols) = (shape[0], shape[1]);

        arr.into_shape_with_order((rows, cols)).ok()
    } else {
        None
    }
}
