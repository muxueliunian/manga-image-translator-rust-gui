use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    sync::{Arc, Mutex},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
pub struct RuntimeDiagnostics {
    pub cuda_feature: bool,
    pub cuda_available: bool,
    pub require_cuda: bool,
    pub provider_status: String,
    pub cuda_error: Option<String>,
}

impl RuntimeDiagnostics {
    pub fn collect_with_error(cuda_available: bool, cuda_error: Option<String>) -> Self {
        let require_cuda = require_cuda();
        Self {
            cuda_feature: cfg!(feature = "cuda"),
            cuda_available,
            require_cuda,
            provider_status: provider_status(cuda_available, require_cuda),
            cuda_error,
        }
    }
}

#[derive(Clone)]
pub struct JobLogger {
    inner: Arc<JobLoggerInner>,
}

struct JobLoggerInner {
    path: PathBuf,
    file: Mutex<File>,
}

impl JobLogger {
    pub fn create(log_dir: impl AsRef<Path>) -> Result<Self> {
        create_dir_all(log_dir.as_ref())?;
        let path = log_dir
            .as_ref()
            .join(format!("job_{}.log", timestamp_millis()));
        let file = File::create(&path)?;
        Ok(Self {
            inner: Arc::new(JobLoggerInner {
                path,
                file: Mutex::new(file),
            }),
        })
    }

    pub fn path(&self) -> &Path {
        &self.inner.path
    }

    pub fn log(&self, level: &str, message: impl AsRef<str>) {
        let line = format!(
            "[{}][{}] {}\n",
            timestamp_millis(),
            level.to_ascii_uppercase(),
            message.as_ref()
        );
        if let Ok(mut file) = self.inner.file.lock() {
            let _ = file.write_all(line.as_bytes());
            let _ = file.flush();
        }
    }
}

pub struct StageTimer {
    name: &'static str,
    start: Instant,
    logger: Option<JobLogger>,
}

impl StageTimer {
    pub fn start(name: &'static str, logger: Option<&JobLogger>) -> Self {
        if let Some(logger) = logger {
            logger.log("info", format!("{name} started"));
        }
        Self {
            name,
            start: Instant::now(),
            logger: logger.cloned(),
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn finish(self) -> Duration {
        let elapsed = self.start.elapsed();
        if let Some(logger) = &self.logger {
            logger.log(
                "info",
                format!("{} finished in {}", self.name, format_duration(elapsed)),
            );
        }
        elapsed
    }
}

/// Collects per-stage timings so the pipeline can emit a single summary table
/// (sorted by cost, with each stage's share of the measured total) into the
/// job log. This is the primary signal for "where does the time go" analysis.
#[derive(Default)]
pub struct StageReport {
    stages: Vec<(&'static str, Duration)>,
}

impl StageReport {
    pub fn new() -> Self {
        Self::default()
    }

    /// Finish a stage timer (which also logs its individual line) and record it.
    pub fn record_timer(&mut self, timer: StageTimer) {
        let name = timer.name();
        let elapsed = timer.finish();
        self.stages.push((name, elapsed));
    }

    /// Sum of all recorded stage durations. Stages run sequentially, so this is
    /// close to wall-clock pipeline time minus untimed gaps (e.g. debug saves).
    pub fn measured_total(&self) -> Duration {
        self.stages.iter().map(|(_, d)| *d).sum()
    }

    /// Lines for a human-readable summary table, biggest stage first.
    pub fn summary_lines(&self) -> Vec<String> {
        let total = self.measured_total();
        let total_secs = total.as_secs_f64();
        let name_width = self
            .stages
            .iter()
            .map(|(name, _)| name.len())
            .max()
            .unwrap_or(5)
            .max(5);

        let mut sorted = self.stages.clone();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));

        let mut lines = Vec::with_capacity(sorted.len() + 1);
        lines.push(format!(
            "stage timing summary (measured total {}):",
            format_duration(total)
        ));
        for (name, elapsed) in sorted {
            let pct = if total_secs > 0.0 {
                elapsed.as_secs_f64() / total_secs * 100.0
            } else {
                0.0
            };
            lines.push(format!(
                "  {name:<name_width$}  {:>9}  {pct:>5.1}%",
                format_duration(elapsed)
            ));
        }
        lines
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GpuMemorySample {
    pub name: String,
    pub used_mb: u64,
    pub total_mb: u64,
}

pub fn sample_nvidia_gpu_memory() -> Option<Vec<GpuMemorySample>> {
    let output = Command::new("nvidia-smi")
        .args([
            "--query-gpu=name,memory.used,memory.total",
            "--format=csv,noheader,nounits",
        ])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let samples = stdout
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(',').map(str::trim);
            let name = parts.next()?.to_owned();
            let used_mb = parts.next()?.parse().ok()?;
            let total_mb = parts.next()?.parse().ok()?;
            Some(GpuMemorySample {
                name,
                used_mb,
                total_mb,
            })
        })
        .collect::<Vec<_>>();
    if samples.is_empty() {
        None
    } else {
        Some(samples)
    }
}

pub fn require_cuda() -> bool {
    matches!(
        base_util::onnx::device_mode(),
        base_util::onnx::DeviceMode::Cuda
    )
}

pub fn ensure_cuda_policy(cuda_available: bool) -> Result<()> {
    if require_cuda() && !cuda_available {
        anyhow::bail!(
            "MIT_REQUIRE_CUDA=1 but CUDA is not available. Check NVIDIA driver, CUDA/cuDNN, and ONNX Runtime CUDA provider DLLs."
        );
    }
    Ok(())
}

pub fn format_duration(duration: Duration) -> String {
    let millis = duration.as_millis();
    if millis >= 1000 {
        format!("{:.2}s", duration.as_secs_f64())
    } else {
        format!("{millis}ms")
    }
}

fn provider_status(cuda_available: bool, require_cuda: bool) -> String {
    if cuda_available {
        "CUDA available".to_owned()
    } else if require_cuda {
        "CUDA unavailable".to_owned()
    } else {
        "CPU/DirectML fallback".to_owned()
    }
}

fn timestamp_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default()
}
