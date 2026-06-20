use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use zip::ZipArchive;

use crate::update;

// Bundling CUDA 12.4 wheels; strict 12.4 wants ~551 but 527.41 is
// CUDA-12.0 floor soft gate - revisit.
pub const MIN_DRIVER_VERSION: &str = "527.41";

const REQUIRED_DLLS: [&str; 5] = [
    "cublasLt64_12.dll",
    "cublas64_12.dll",
    "cufft64_11.dll",
    "cudart64_12.dll",
    "cudnn64_9.dll",
];

const WHEEL_PACKAGES: [(&str, &str); 4] = [
    ("nvidia-cuda-runtime-cu12", "12.4.127"),
    ("nvidia-cublas-cu12", "12.4.5.8"),
    ("nvidia-cufft-cu12", "11.2.1.3"),
    ("nvidia-cudnn-cu12", "9.11.0.98"),
];

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NvidiaGpu {
    pub name: String,
    pub driver_version: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DllStatus {
    pub name: &'static str,
    pub present: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GpuRuntimeStatus {
    pub cuda_feature: bool,
    pub gpu_detected: bool,
    pub gpu_name: Option<String>,
    pub driver_version: Option<String>,
    pub driver_ok: bool,
    pub min_driver: String,
    pub dlls: Vec<DllStatus>,
    pub dll_all_present: bool,
    pub ep_ok: bool,
    pub recommendation: &'static str,
}

#[derive(Debug, Clone)]
pub struct WheelSpec {
    pub package: &'static str,
    pub version: &'static str,
    pub filename: String,
    pub url: String,
    pub sha256: String,
}

#[derive(Debug, Clone)]
pub struct CudaDownloadProgress {
    pub package: &'static str,
    pub version: &'static str,
    pub current: usize,
    pub total: usize,
    pub downloaded: u64,
    pub total_bytes: u64,
}

#[derive(Deserialize)]
struct PypiRelease {
    urls: Vec<PypiUrl>,
}

#[derive(Deserialize)]
struct PypiUrl {
    filename: String,
    url: String,
    digests: PypiDigests,
}

#[derive(Deserialize)]
struct PypiDigests {
    sha256: String,
}

pub fn detect_nvidia_gpu() -> Option<NvidiaGpu> {
    let output = Command::new("nvidia-smi")
        .args([
            "--query-gpu=name,driver_version",
            "--format=csv,noheader,nounits",
        ])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.lines().find_map(|line| {
        let mut parts = line.split(',').map(str::trim);
        let name = parts.next()?.to_owned();
        let driver_version = parts.next()?.to_owned();
        if name.is_empty() || driver_version.is_empty() {
            return None;
        }
        Some(NvidiaGpu {
            name,
            driver_version,
        })
    })
}

pub fn cuda_runtime_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(Path::to_path_buf))
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
}

pub fn check_runtime_dlls() -> Vec<DllStatus> {
    let runtime_dir = cuda_runtime_dir();
    REQUIRED_DLLS
        .iter()
        .copied()
        .map(|name| DllStatus {
            name,
            present: runtime_dir.join(name).is_file(),
        })
        .collect()
}

pub fn gpu_runtime_status() -> GpuRuntimeStatus {
    let gpu = detect_nvidia_gpu();
    let driver_ok = gpu
        .as_ref()
        .map(|gpu| driver_version_at_least(&gpu.driver_version, MIN_DRIVER_VERSION))
        .unwrap_or(false);
    let dlls = check_runtime_dlls();
    let dll_all_present = dlls.iter().all(|dll| dll.present);
    let cuda_feature = cfg!(feature = "cuda");
    let ep_ok = update::check_cuda_error().is_none();
    let recommendation = if !cuda_feature {
        "cpu_only_build"
    } else if gpu.is_none() {
        "no_gpu"
    } else if !driver_ok {
        "need_driver_update"
    } else if !dll_all_present {
        "need_download_dll"
    } else {
        "ready"
    };

    GpuRuntimeStatus {
        cuda_feature,
        gpu_detected: gpu.is_some(),
        gpu_name: gpu.as_ref().map(|gpu| gpu.name.clone()),
        driver_version: gpu.map(|gpu| gpu.driver_version),
        driver_ok,
        min_driver: MIN_DRIVER_VERSION.to_owned(),
        dlls,
        dll_all_present,
        ep_ok,
        recommendation,
    }
}

pub fn wheel_download_plan() -> Vec<WheelSpec> {
    try_wheel_download_plan().unwrap_or_default()
}

pub fn download_cuda_runtime<P, L>(mut progress: P, mut log: L) -> Result<()>
where
    P: FnMut(CudaDownloadProgress),
    L: FnMut(&str, String),
{
    log("info", "解析 CUDA 运行时 wheel 下载地址…".to_owned());
    let plan = try_wheel_download_plan()?;
    if plan.is_empty() {
        return Err(anyhow!("未找到可下载的 Windows x64 CUDA runtime wheel"));
    }

    fs::create_dir_all(cuda_runtime_dir()).context("创建 CUDA runtime 目录失败")?;
    for (index, wheel) in plan.iter().enumerate() {
        let current = index + 1;
        let total = plan.len();
        log(
            "info",
            format!(
                "下载 CUDA runtime wheel {current}/{total}: {} {}",
                wheel.package, wheel.version
            ),
        );
        let temp_path = download_wheel(wheel, current, total, &mut progress)?;
        let result = (|| -> Result<usize> {
            verify_sha256(&temp_path, &wheel.sha256)
                .with_context(|| format!("SHA256 校验失败: {}", wheel.filename))?;
            extract_dlls(&temp_path).with_context(|| format!("解压失败: {}", wheel.filename))
        })();
        let _ = fs::remove_file(&temp_path);
        let extracted = result?;
        log(
            "success",
            format!(
                "已安装 CUDA runtime DLL: {} {}，提取 {extracted} 个 DLL",
                wheel.package, wheel.version
            ),
        );
    }
    Ok(())
}

fn try_wheel_download_plan() -> Result<Vec<WheelSpec>> {
    WHEEL_PACKAGES
        .iter()
        .map(|(package, version)| resolve_wheel(package, version))
        .collect()
}

fn resolve_wheel(package: &'static str, version: &'static str) -> Result<WheelSpec> {
    let url = format!("https://pypi.org/pypi/{package}/{version}/json");
    let mut response = ureq::get(&url)
        .call()
        .with_context(|| format!("请求 PyPI 失败: {url}"))?;
    let mut body = String::new();
    response
        .body_mut()
        .as_reader()
        .read_to_string(&mut body)
        .with_context(|| format!("读取 PyPI 响应失败: {url}"))?;
    let release: PypiRelease =
        serde_json::from_str(&body).with_context(|| format!("解析 PyPI 响应失败: {url}"))?;
    let file = release
        .urls
        .into_iter()
        .find(|entry| entry.filename.contains("win_amd64") && entry.filename.ends_with(".whl"))
        .ok_or_else(|| anyhow!("PyPI 未提供 Windows x64 wheel: {package}=={version}"))?;

    Ok(WheelSpec {
        package,
        version,
        filename: file.filename,
        url: file.url,
        sha256: file.digests.sha256,
    })
}

fn download_wheel<P>(
    wheel: &WheelSpec,
    current: usize,
    total: usize,
    progress: &mut P,
) -> Result<PathBuf>
where
    P: FnMut(CudaDownloadProgress),
{
    let mut response = ureq::get(&wheel.url)
        .call()
        .with_context(|| format!("下载失败: {}", wheel.url))?;
    let total_bytes = response
        .headers()
        .get("Content-Length")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(0);
    let content_encoded = response
        .headers()
        .get("Content-Encoding")
        .and_then(|value| value.to_str().ok())
        .map(|value| {
            let value = value.trim();
            !value.is_empty() && !value.eq_ignore_ascii_case("identity")
        })
        .unwrap_or(false);

    let temp_path = temp_wheel_path(&wheel.filename);
    let mut file = File::create(&temp_path)
        .with_context(|| format!("创建临时 wheel 文件失败: {}", temp_path.display()))?;
    let mut downloaded = 0u64;
    let mut hasher = Sha256::new();
    {
        let mut reader = response.body_mut().as_reader();
        let mut buf = [0u8; 65536];
        loop {
            let n = reader.read(&mut buf)?;
            if n == 0 {
                break;
            }
            file.write_all(&buf[..n])?;
            hasher.update(&buf[..n]);
            downloaded += n as u64;
            progress(CudaDownloadProgress {
                package: wheel.package,
                version: wheel.version,
                current,
                total,
                downloaded,
                total_bytes,
            });
        }
    }
    file.flush()?;

    if total_bytes > 0 && !content_encoded && downloaded < total_bytes {
        let _ = fs::remove_file(&temp_path);
        return Err(anyhow!(
            "下载不完整：仅收到 {downloaded}/{total_bytes} 字节 ({})",
            wheel.filename
        ));
    }

    let actual = format!("{:x}", hasher.finalize());
    if !actual.eq_ignore_ascii_case(&wheel.sha256) {
        let _ = fs::remove_file(&temp_path);
        return Err(anyhow!(
            "SHA256 不匹配: {} expected {} got {}",
            wheel.filename,
            wheel.sha256,
            actual
        ));
    }

    progress(CudaDownloadProgress {
        package: wheel.package,
        version: wheel.version,
        current,
        total,
        downloaded,
        total_bytes,
    });
    Ok(temp_path)
}

fn verify_sha256(path: &Path, expected: &str) -> Result<()> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 65536];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    let actual = format!("{:x}", hasher.finalize());
    if actual.eq_ignore_ascii_case(expected) {
        Ok(())
    } else {
        Err(anyhow!("expected {expected} got {actual}"))
    }
}

fn extract_dlls(wheel_path: &Path) -> Result<usize> {
    let file = File::open(wheel_path)?;
    let mut archive = ZipArchive::new(file)?;
    let runtime_dir = cuda_runtime_dir();
    let mut extracted = 0usize;

    for index in 0..archive.len() {
        let mut entry = archive.by_index(index)?;
        if !entry.is_file() {
            continue;
        }
        let Some(path) = entry.enclosed_name() else {
            continue;
        };
        let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if !file_name.to_ascii_lowercase().ends_with(".dll") {
            continue;
        }
        let out_path = runtime_dir.join(file_name);
        let mut out_file = File::create(&out_path)
            .with_context(|| format!("写入 DLL 失败: {}", out_path.display()))?;
        std::io::copy(&mut entry, &mut out_file)?;
        extracted += 1;
    }

    Ok(extracted)
}

fn temp_wheel_path(filename: &str) -> PathBuf {
    let safe_name = filename
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '.' | '-' | '_') {
                ch
            } else {
                '_'
            }
        })
        .collect::<String>();
    std::env::temp_dir().join(format!("mit_cuda_{}_{}", uuid::Uuid::new_v4(), safe_name))
}

fn driver_version_at_least(actual: &str, minimum: &str) -> bool {
    let Some(actual) = parse_driver_version(actual) else {
        return false;
    };
    let Some(minimum) = parse_driver_version(minimum) else {
        return false;
    };
    actual >= minimum
}

fn parse_driver_version(value: &str) -> Option<(u32, u32)> {
    let mut parts = value.trim().split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next().unwrap_or("0").parse().ok()?;
    Some((major, minor))
}
