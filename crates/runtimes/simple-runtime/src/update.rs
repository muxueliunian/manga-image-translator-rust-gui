use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::Command,
    sync::{Mutex, OnceLock},
};

use anyhow::{anyhow, Context, Result};
use ort::{
    execution_providers::{CUDAExecutionProvider, ExecutionProvider},
    session::Session,
};
use semver::Version;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const DEFAULT_UPDATE_REPO: &str = "muxueliunian/manga-image-translator-rust-gui";
const PORTABLE_ZIP: &str = "manga-image-translator-rust-portable.zip";
const USER_AGENT: &str = "manga-image-translator-rust-simple-runtime";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppUpdateInfo {
    pub current_version: String,
    pub latest_version: String,
    pub update_available: bool,
    pub tag_name: String,
    pub release_name: String,
    pub html_url: String,
    pub body: String,
    pub published_at: String,
    pub asset_name: Option<String>,
    pub asset_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StagedAppUpdate {
    pub current_version: String,
    pub latest_version: String,
    pub tag_name: String,
    pub asset_name: String,
    pub asset_size: u64,
    pub archive_path: String,
    pub sha256: Option<String>,
}

#[derive(Debug, Clone)]
struct DownloadAsset {
    name: String,
    browser_download_url: String,
    size: u64,
    digest: Option<String>,
}

#[derive(Debug, Clone)]
struct ReleaseSelection {
    info: AppUpdateInfo,
    asset: Option<DownloadAsset>,
    sha256_asset: Option<DownloadAsset>,
}

#[derive(Debug, Clone)]
struct StagedState {
    data: StagedAppUpdate,
    archive_path: PathBuf,
}

#[derive(Deserialize)]
struct GithubRelease {
    tag_name: String,
    #[serde(default)]
    name: String,
    html_url: String,
    #[serde(default)]
    body: String,
    #[serde(default)]
    published_at: String,
    #[serde(default)]
    assets: Vec<GithubAsset>,
}

#[derive(Debug, Clone, Deserialize)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
    #[serde(default)]
    size: u64,
    #[serde(default)]
    digest: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AppUpdateDownloadProgress {
    pub asset_name: String,
    pub downloaded: u64,
    pub total_bytes: u64,
}

pub fn check_cuda_error() -> Option<String> {
    let cuda = CUDAExecutionProvider::default();
    if !cuda.is_available().unwrap_or_default() {
        return Some("CUDA execution provider is not reported as available.".to_owned());
    }

    let provider = cuda.with_device_id(0).build().error_on_failure();
    Session::builder()
        .and_then(|builder| builder.with_execution_providers(vec![provider]))
        .err()
        .map(|err| err.to_string())
}

pub async fn check_crate_version(repo: &str) -> Result<bool> {
    let release = select_release(repo)?;
    if release.info.update_available {
        eprintln!(
            "Version is outdated (current: {}, latest: {}). See: {}",
            release.info.current_version, release.info.latest_version, release.info.html_url
        );
        Ok(false)
    } else {
        Ok(true)
    }
}

pub fn check_app_update() -> Result<AppUpdateInfo> {
    Ok(select_release(DEFAULT_UPDATE_REPO)?.info)
}

pub fn download_app_update<P>(mut progress: P) -> Result<StagedAppUpdate>
where
    P: FnMut(AppUpdateDownloadProgress),
{
    let release = select_release(DEFAULT_UPDATE_REPO)?;
    if !release.info.update_available {
        return Err(anyhow!(
            "当前已是最新版本: {}",
            release.info.current_version
        ));
    }
    let asset = release.asset.ok_or_else(|| {
        anyhow!(
            "GitHub Release 未找到与当前构建兼容的便携包 asset: {PORTABLE_ZIP} 或匹配当前 CPU/CUDA 构建的 portable .zip"
        )
    })?;

    let stage_dir = temp_update_dir()?;
    let archive_path = stage_dir.join(sanitize_file_name(&asset.name));
    let actual_sha256 = download_asset(&asset, &archive_path, &mut progress)
        .with_context(|| format!("下载更新包失败: {}", asset.name))?;
    let expected_sha256 = expected_sha256(&asset, release.sha256_asset.as_ref())?;
    if let Some(expected) = expected_sha256.as_deref() {
        if !actual_sha256.eq_ignore_ascii_case(expected) {
            let _ = fs::remove_file(&archive_path);
            return Err(anyhow!(
                "更新包 SHA256 不匹配: {} expected {} got {}",
                asset.name,
                expected,
                actual_sha256
            ));
        }
    }

    let staged = StagedAppUpdate {
        current_version: release.info.current_version,
        latest_version: release.info.latest_version,
        tag_name: release.info.tag_name,
        asset_name: asset.name,
        asset_size: asset.size,
        archive_path: archive_path.display().to_string(),
        sha256: Some(actual_sha256),
    };
    *staged_state().lock().expect("staged update mutex poisoned") = Some(StagedState {
        data: staged.clone(),
        archive_path,
    });
    Ok(staged)
}

pub fn install_app_update() -> Result<StagedAppUpdate> {
    let staged = staged_state()
        .lock()
        .expect("staged update mutex poisoned")
        .clone()
        .ok_or_else(|| anyhow!("没有已下载的更新包，请先下载更新。"))?;
    if !staged.archive_path.exists() {
        return Err(anyhow!(
            "更新包不存在或已被清理: {}",
            staged.archive_path.display()
        ));
    }
    launch_external_updater(&staged.archive_path)?;
    Ok(staged.data)
}

fn select_release(repo: &str) -> Result<ReleaseSelection> {
    let url = format!("https://api.github.com/repos/{repo}/releases/latest");
    let release: GithubRelease = get_json(&url)?;
    let current = Version::parse(env!("CARGO_PKG_VERSION")).context("解析当前应用版本失败")?;
    let latest = Version::parse(release.tag_name.trim_start_matches('v'))
        .with_context(|| format!("解析 GitHub Release tag 失败: {}", release.tag_name))?;
    let asset = select_portable_asset(&release.assets);
    let sha256_asset = asset
        .as_ref()
        .and_then(|asset| select_sha256_asset(&release.assets, &asset.name));

    Ok(ReleaseSelection {
        info: AppUpdateInfo {
            current_version: current.to_string(),
            latest_version: latest.to_string(),
            update_available: current < latest,
            tag_name: release.tag_name.clone(),
            release_name: if release.name.trim().is_empty() {
                release.tag_name
            } else {
                release.name
            },
            html_url: release.html_url,
            body: release.body,
            published_at: release.published_at,
            asset_name: asset.as_ref().map(|asset| asset.name.clone()),
            asset_size: asset.as_ref().map(|asset| asset.size),
        },
        asset,
        sha256_asset,
    })
}

fn get_json<T: for<'de> Deserialize<'de>>(url: &str) -> Result<T> {
    let mut response = ureq::get(url)
        .header("User-Agent", USER_AGENT)
        .header("Accept", "application/vnd.github+json")
        .call()
        .with_context(|| format!("请求 GitHub Release 失败: {url}"))?;
    let mut body = String::new();
    response
        .body_mut()
        .as_reader()
        .read_to_string(&mut body)
        .with_context(|| format!("读取 GitHub Release 响应失败: {url}"))?;
    serde_json::from_str(&body).with_context(|| format!("解析 GitHub Release 响应失败: {url}"))
}

fn select_portable_asset(assets: &[GithubAsset]) -> Option<DownloadAsset> {
    let exact = assets
        .iter()
        .find(|asset| asset.name.eq_ignore_ascii_case(PORTABLE_ZIP));
    let compatible = assets
        .iter()
        .find(|asset| compatible_portable_zip(&asset.name));
    exact.or(compatible).map(download_asset_from_github)
}

fn compatible_portable_zip(name: &str) -> bool {
    let name = name.to_ascii_lowercase();
    if !name.ends_with(".zip") || !name.contains("portable") {
        return false;
    }
    if cfg!(feature = "cuda") {
        name.contains("cuda") || !name.contains("cpu")
    } else {
        name.contains("cpu") || !name.contains("cuda")
    }
}

fn select_sha256_asset(assets: &[GithubAsset], archive_name: &str) -> Option<DownloadAsset> {
    let archive_name = archive_name.to_ascii_lowercase();
    assets
        .iter()
        .find(|asset| {
            let name = asset.name.to_ascii_lowercase();
            name == format!("{archive_name}.sha256")
                || name == archive_name.replace(".zip", ".sha256")
                || (name.ends_with(".sha256") && name.contains("portable"))
        })
        .map(download_asset_from_github)
}

fn download_asset_from_github(asset: &GithubAsset) -> DownloadAsset {
    DownloadAsset {
        name: asset.name.clone(),
        browser_download_url: asset.browser_download_url.clone(),
        size: asset.size,
        digest: asset.digest.clone(),
    }
}

fn download_asset<P>(asset: &DownloadAsset, path: &Path, progress: &mut P) -> Result<String>
where
    P: FnMut(AppUpdateDownloadProgress),
{
    let mut response = ureq::get(&asset.browser_download_url)
        .header("User-Agent", USER_AGENT)
        .call()
        .with_context(|| format!("请求下载失败: {}", asset.browser_download_url))?;
    let total_bytes = response
        .headers()
        .get("Content-Length")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(asset.size);
    let content_encoded = response
        .headers()
        .get("Content-Encoding")
        .and_then(|value| value.to_str().ok())
        .map(|value| {
            let value = value.trim();
            !value.is_empty() && !value.eq_ignore_ascii_case("identity")
        })
        .unwrap_or(false);

    let mut file =
        File::create(path).with_context(|| format!("创建更新包文件失败: {}", path.display()))?;
    let mut hasher = Sha256::new();
    let mut downloaded = 0u64;
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
            progress(AppUpdateDownloadProgress {
                asset_name: asset.name.clone(),
                downloaded,
                total_bytes,
            });
        }
    }
    file.flush()?;

    if total_bytes > 0 && !content_encoded && downloaded < total_bytes {
        let _ = fs::remove_file(path);
        return Err(anyhow!(
            "下载不完整：仅收到 {downloaded}/{total_bytes} 字节 ({})",
            asset.name
        ));
    }
    progress(AppUpdateDownloadProgress {
        asset_name: asset.name.clone(),
        downloaded,
        total_bytes,
    });
    Ok(format!("{:x}", hasher.finalize()))
}

fn expected_sha256(
    asset: &DownloadAsset,
    sha256_asset: Option<&DownloadAsset>,
) -> Result<Option<String>> {
    if let Some(digest) = asset.digest.as_deref() {
        if let Some(hash) = digest.strip_prefix("sha256:").and_then(first_sha256_hex) {
            return Ok(Some(hash));
        }
    }
    let Some(sha256_asset) = sha256_asset else {
        return Ok(None);
    };
    let mut response = ureq::get(&sha256_asset.browser_download_url)
        .header("User-Agent", USER_AGENT)
        .call()
        .with_context(|| format!("请求 SHA256 文件失败: {}", sha256_asset.name))?;
    let mut body = String::new();
    response
        .body_mut()
        .as_reader()
        .read_to_string(&mut body)
        .with_context(|| format!("读取 SHA256 文件失败: {}", sha256_asset.name))?;
    first_sha256_hex(&body)
        .map(Some)
        .ok_or_else(|| anyhow!("SHA256 文件格式无效: {}", sha256_asset.name))
}

fn first_sha256_hex(value: &str) -> Option<String> {
    let mut run = String::new();
    for ch in value.chars() {
        if ch.is_ascii_hexdigit() {
            run.push(ch);
            if run.len() == 64 {
                return Some(run);
            }
        } else {
            run.clear();
        }
    }
    None
}

fn temp_update_dir() -> Result<PathBuf> {
    let dir = std::env::temp_dir().join(format!("mit_app_update_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&dir).with_context(|| format!("创建更新临时目录失败: {}", dir.display()))?;
    Ok(dir)
}

fn sanitize_file_name(value: &str) -> String {
    let cleaned: String = value
        .chars()
        .map(|ch| match ch {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
            ch if ch.is_control() => '_',
            ch => ch,
        })
        .collect();
    if cleaned.trim().is_empty() {
        PORTABLE_ZIP.to_owned()
    } else {
        cleaned
    }
}

fn staged_state() -> &'static Mutex<Option<StagedState>> {
    static STAGED: OnceLock<Mutex<Option<StagedState>>> = OnceLock::new();
    STAGED.get_or_init(|| Mutex::new(None))
}

#[cfg(windows)]
fn launch_external_updater(archive_path: &Path) -> Result<()> {
    let exe = std::env::current_exe().context("获取当前 exe 路径失败")?;
    let target_dir = exe
        .parent()
        .ok_or_else(|| anyhow!("无法确定安装目录: {}", exe.display()))?;
    let script_path = archive_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("mit-self-update.ps1");
    let script = build_windows_updater_script();
    fs::write(&script_path, script)
        .with_context(|| format!("写入 updater 脚本失败: {}", script_path.display()))?;

    Command::new("powershell.exe")
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            &script_path.display().to_string(),
            "-PidToWait",
            &std::process::id().to_string(),
            "-Archive",
            &archive_path.display().to_string(),
            "-Target",
            &target_dir.display().to_string(),
            "-Exe",
            &exe.display().to_string(),
        ])
        .spawn()
        .context("启动外部 updater 失败")?;
    Ok(())
}

#[cfg(not(windows))]
fn launch_external_updater(_archive_path: &Path) -> Result<()> {
    Err(anyhow!("当前自动安装更新仅支持 Windows 便携包。"))
}

#[cfg(windows)]
fn build_windows_updater_script() -> &'static str {
    r#"
param(
  [Parameter(Mandatory=$true)][int]$PidToWait,
  [Parameter(Mandatory=$true)][string]$Archive,
  [Parameter(Mandatory=$true)][string]$Target,
  [Parameter(Mandatory=$true)][string]$Exe
)
$ErrorActionPreference = 'Stop'
Wait-Process -Id $PidToWait -ErrorAction SilentlyContinue
Start-Sleep -Milliseconds 500

$extract = Join-Path ([System.IO.Path]::GetTempPath()) ("mit_app_update_extract_" + [guid]::NewGuid().ToString("N"))
New-Item -ItemType Directory -Force -Path $extract | Out-Null
Expand-Archive -LiteralPath $Archive -DestinationPath $extract -Force

$dirs = @(Get-ChildItem -LiteralPath $extract -Directory -Force)
$source = $extract
if ($dirs.Count -eq 1 -and (Test-Path -LiteralPath (Join-Path $dirs[0].FullName "simple-runtime.exe"))) {
  $source = $dirs[0].FullName
} elseif (Test-Path -LiteralPath (Join-Path $extract "manga-image-translator-rust-portable")) {
  $source = Join-Path $extract "manga-image-translator-rust-portable"
}

$preserveDirs = @("config", "models", "uploads", "results", "logs")
$preserveDllPatterns = @("cudart64_*.dll", "cublas64_*.dll", "cublasLt64_*.dll", "cufft64_*.dll", "cudnn64_*.dll", "cudnn_*.dll")

foreach ($item in Get-ChildItem -LiteralPath $source -Force) {
  if ($item.PSIsContainer -and ($preserveDirs -contains $item.Name)) {
    continue
  }
  if (-not $item.PSIsContainer) {
    $skip = $false
    foreach ($pattern in $preserveDllPatterns) {
      if ($item.Name -like $pattern) {
        $skip = $true
        break
      }
    }
    if ($skip) {
      continue
    }
  }
  Copy-Item -LiteralPath $item.FullName -Destination $Target -Recurse -Force
}

Remove-Item -LiteralPath $extract -Recurse -Force -ErrorAction SilentlyContinue
Start-Process -FilePath $Exe -ArgumentList "ui-webview" -WorkingDirectory $Target
"#
}
