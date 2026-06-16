use ort::{
    execution_providers::{CUDAExecutionProvider, ExecutionProvider},
    session::Session,
};
use reqwest::Client;
use semver::Version;
use serde::Deserialize;

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    html_url: String,
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

pub async fn check_crate_version(repo: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", repo);

    let client = Client::new();
    let release: Release = client
        .get(&url)
        .header("User-Agent", "reqwest")
        .send()
        .await?
        .json()
        .await?;

    let latest_version = Version::parse(release.tag_name.trim_start_matches('v'))?;
    let current_version = Version::parse(env!("CARGO_PKG_VERSION"))?;

    if current_version < latest_version {
        eprintln!(
            "⚠️  Version is outdated (current: {}, latest: {}).\n👉 See: {}",
            current_version, latest_version, release.html_url
        );
        Ok(false)
    } else {
        Ok(true)
    }
}
