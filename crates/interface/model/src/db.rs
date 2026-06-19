use std::{
    fs::{self, read_dir, read_to_string, File, OpenOptions},
    io::{BufReader, Read, Seek as _, Write as _},
    path::{Component, Path, PathBuf},
    sync::RwLock,
};

use anyhow::anyhow;
use base_util::project::root_path;
use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, info};
use sha2::{Digest, Sha256};
use tar::Archive;

/// How the on-disk model root is resolved. Defaults to the legacy
/// `root_path()/models` so the egui UI / CLI / tests keep working; the portable
/// WebView sets it explicitly (a configured external folder, or "require" so an
/// unset path errors instead of silently downloading into the wiped dist dir).
#[derive(Clone, Debug)]
pub enum ModelRootMode {
    Default,
    Configured(PathBuf),
    RequireConfigured,
}

static MODEL_ROOT: RwLock<ModelRootMode> = RwLock::new(ModelRootMode::Default);

pub fn set_model_root(mode: ModelRootMode) {
    if let Ok(mut guard) = MODEL_ROOT.write() {
        *guard = mode;
    }
}

/// The directory that holds `<kind>/<name>/...` model files.
pub fn model_base_dir() -> anyhow::Result<PathBuf> {
    let mode = MODEL_ROOT
        .read()
        .map_err(|_| anyhow!("model root lock poisoned"))?
        .clone();
    match mode {
        ModelRootMode::Default => Ok(root_path().join("models")),
        ModelRootMode::Configured(path) => Ok(path),
        ModelRootMode::RequireConfigured => Err(anyhow!(
            "模型目录未设置：请在「模型」页选择一个外部文件夹后再下载 / 翻译。"
        )),
    }
}

/// Resolve `<root>/<kind>/<name>/<file>` plus the directory used as the hash
/// base, mirroring [`ModelDb::get`]'s path handling (folder downloads collapse to
/// the parent dir). Shared by the readiness check and the download helper.
fn resolve_file_path(
    kind: &str,
    name: &str,
    file: &str,
) -> anyhow::Result<(PathBuf, PathBuf, bool)> {
    let base_path = model_base_dir()?.join(kind).join(name);
    let mut file_path = base_path.join(file);
    let folder = file.contains('/');
    if folder {
        file_path = file_path
            .parent()
            .expect("joined file always has a parent")
            .to_path_buf();
    }
    Ok((base_path, file_path, folder))
}

/// Whether a single model file is present and (when a real hash is given) valid,
/// **without** triggering any download. Used by the model-management UI to render
/// downloaded/missing status. Errors only when the model root itself is
/// unresolved (e.g. unset in the portable WebView).
pub fn model_file_ready(kind: &str, name: &str, file: &str, hash: &str) -> anyhow::Result<bool> {
    let (base_path, file_path, _) = resolve_file_path(kind, name, file)?;
    Ok(!failure(Some(&base_path), &file_path, hash))
}

/// Download one model file into the configured root, returning its final path.
/// Same retry/hash behaviour as inference-time loads ([`ModelDb::get`]) but
/// returns an error instead of panicking on repeated failure, so the UI can
/// report it per file and keep going.
pub fn download_model_file(
    kind: &str,
    name: &str,
    file: &str,
    url: &str,
    hash: &str,
    progress: &mut dyn FnMut(u64, u64),
) -> anyhow::Result<PathBuf> {
    let (base_path, file_path, folder) = resolve_file_path(kind, name, file)?;
    let ret_file_path = base_path.join(file);
    std::fs::create_dir_all(
        ret_file_path
            .parent()
            .expect("joined file always has a parent"),
    )?;
    let mut attempts = 0u8;
    while failure(Some(&base_path), &file_path, hash) {
        if attempts >= 3 {
            return Err(anyhow!("下载失败（已重试 {attempts} 次）: {url}"));
        }
        if attempts > 0 {
            let _ = std::fs::remove_file(&file_path);
        }
        download_and_extract(url, &file_path, folder, Some(&mut *progress))?;
        attempts += 1;
    }
    Ok(ret_file_path)
}

pub struct ModelDb {}

impl ModelDb {
    pub fn get(
        &self,
        kind: &str,
        name: &str,
        file: &str,
        url: &str,
        hash: &str,
    ) -> anyhow::Result<PathBuf> {
        let base_path = model_base_dir()?.join(kind).join(name);
        let mut file_path = base_path.join(file);

        std::fs::create_dir_all(file_path.parent().expect("set above"))?;
        let mut folder = false;
        // allow:clone[pathbuf]
        let ret_file_path = file_path.clone();
        if file.contains("/") {
            file_path = file_path.parent().expect("set above").to_path_buf();

            folder = true;
        }
        if failure(Some(&base_path), &file_path, hash) {
            download_and_extract(url, &file_path, folder, None)?;
            if failure(Some(&base_path), &file_path, hash) {
                let _ = std::fs::remove_file(&file_path);
                download_and_extract(url, &file_path, folder, None)?;
                if failure(Some(&base_path), &file_path, hash) {
                    let _ = std::fs::remove_file(&file_path);
                    download_and_extract(url, &file_path, folder, None)?;
                    if failure(Some(&base_path), &file_path, hash) {
                        panic!()
                    }
                }
            }
        }
        Ok(ret_file_path)
    }
}

fn get_all_files_recursively<P: AsRef<Path>>(dir: P) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(get_all_files_recursively(&path));
            } else if path.is_file() {
                files.push(path);
            }
        }
    }

    files
}

/// Whether `path` holds real downloaded bytes: a non-empty file, or a directory
/// containing at least one non-empty, non-hidden file. Used to reject empty or
/// truncated-to-zero downloads for models that ship without a real hash ("###").
fn has_nonempty_content(path: &Path) -> bool {
    match fs::metadata(path) {
        Ok(meta) if meta.is_file() => meta.len() > 0,
        Ok(meta) if meta.is_dir() => get_all_files_recursively(path).iter().any(|p| {
            let visible = p
                .file_name()
                .and_then(|n| n.to_str())
                .map(|n| !n.starts_with('.'))
                .unwrap_or(false);
            visible && fs::metadata(p).map(|m| m.len() > 0).unwrap_or(false)
        }),
        _ => false,
    }
}

fn failure<P: AsRef<Path>>(base_path: Option<P>, file_path: P, expected_hash: &str) -> bool {
    if !file_path.as_ref().exists() {
        return true;
    }
    if file_path.as_ref().is_dir() {
        let files = read_dir(file_path.as_ref())
            .unwrap()
            .filter_map(|v| v.ok())
            .filter(|v| !v.file_name().to_str().unwrap_or(".").starts_with("."))
            .count();
        if files == 0 {
            return true;
        }
    }
    if expected_hash == "###" {
        // No real hash to verify against; at least make sure the path actually
        // holds bytes, so an empty / truncated-to-zero download isn't treated as
        // ready (it gets re-fetched instead of silently used).
        return !has_nonempty_content(file_path.as_ref());
    }

    if let Some(base_path) = &base_path {
        let base_path = base_path.as_ref();
        let info_path = base_path.join("hashes");
        let p = file_path
            .as_ref()
            .strip_prefix(base_path)
            .unwrap_or(file_path.as_ref());
        let content = read_to_string(&info_path).unwrap_or_default();
        let hash_cache = content
            .lines()
            .filter_map(|v| v.trim().rsplit_once(" "))
            .find(|v| Path::new(v.0) == p)
            .map(|v| v.1);
        if let Some(hash) = hash_cache {
            if hash != expected_hash {
                let content = content
                    .lines()
                    .filter_map(|v| v.trim().rsplit_once(" "))
                    .filter(|v| Path::new(v.0) != p)
                    .map(|v| format!("{} {}", v.0, v.1))
                    .collect::<Vec<_>>()
                    .join("\n");
                let mut file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(&info_path)
                    .unwrap();

                file.write_all(content.as_bytes()).unwrap();
            }
            return hash != expected_hash;
        }
    }

    match file_path.as_ref().is_dir() {
        true => {
            let mut entries = Vec::new();

            if let Ok(walk) = fs::read_dir(file_path.as_ref()) {
                for entry in walk.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        entries.extend(get_all_files_recursively(&path));
                    } else if path.is_file() {
                        entries.push(path);
                    }
                }
            }

            entries.sort();

            let mut hasher = Sha256::new();

            for path in entries {
                if let Ok(rel_path) = path.strip_prefix(file_path.as_ref()) {
                    hasher.update(rel_path.to_string_lossy().as_bytes());
                }

                let mut file = match File::open(&path) {
                    Ok(f) => f,
                    Err(_) => return true,
                };

                let mut buffer = Vec::new();
                if file.read_to_end(&mut buffer).is_err() {
                    return true;
                }
                hasher.update(&buffer);
            }

            let result = hasher.finalize();
            let dir_hash = format!("{:x}", result);
            debug!("Dir hash: {}", dir_hash);
            if let Some(base_path) = base_path {
                if dir_hash == expected_hash {
                    let mut file = OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(base_path.as_ref().join("hashes"))
                        .unwrap();

                    writeln!(
                        file,
                        "{} {}",
                        file_path
                            .as_ref()
                            .strip_prefix(base_path.as_ref())
                            .unwrap_or(file_path.as_ref())
                            .to_string_lossy(),
                        expected_hash
                    )
                    .unwrap();
                }
            }
            dir_hash != expected_hash
        }
        false => {
            let mut file = match std::fs::File::open(&file_path) {
                Ok(f) => f,
                Err(_) => return true,
            };

            let mut hasher = Sha256::new();
            let mut buffer = Vec::new();
            if file.read_to_end(&mut buffer).is_err() {
                return true;
            }

            hasher.update(&buffer);
            let result = hasher.finalize();
            let file_hash = format!("{:x}", result);
            debug!("File hash: {}", file_hash);
            if let Some(base_path) = base_path {
                if file_hash == expected_hash {
                    let mut file = OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(base_path.as_ref().join("hashes"))
                        .unwrap();

                    writeln!(
                        file,
                        "{} {}",
                        file_path
                            .as_ref()
                            .strip_prefix(base_path.as_ref())
                            .unwrap_or(file_path.as_ref())
                            .to_string_lossy(),
                        expected_hash
                    )
                    .unwrap();
                }
            }

            file_hash != expected_hash
        }
    }
}

/// `progress`, when given, is called with `(bytes_downloaded, total_bytes)` as
/// the body streams in (`total` is 0 if the server omits Content-Length). It runs
/// on the calling thread; callers should throttle their own UI updates.
fn download_and_extract(
    url: &str,
    file_path: &Path,
    folder: bool,
    mut progress: Option<&mut dyn FnMut(u64, u64)>,
) -> anyhow::Result<()> {
    info!("Downloading from: {}", url);

    let mut response = ureq::get(url).call()?;
    let total_size = response
        .headers()
        .get("Content-Length")
        .and_then(|val| val.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(0);
    // Content-Length describes the *encoded* body; if the server applied a
    // transfer compression the bytes we read won't match it, so only treat the
    // length as an integrity target for identity (uncompressed) responses.
    let content_encoded = response
        .headers()
        .get("Content-Encoding")
        .and_then(|val| val.to_str().ok())
        .map(|v| {
            let v = v.trim();
            !v.is_empty() && !v.eq_ignore_ascii_case("identity")
        })
        .unwrap_or(false);

    let pb = if total_size > 0 {
        let pb = ProgressBar::new(total_size);
        if let Ok(style) = ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        {
            pb.set_style(style.progress_chars("#>-"));
        }
        pb.set_message("Downloading");
        Some(pb)
    } else {
        None
    };

    let mut temp_file = tempfile::tempfile()?;
    let mut downloaded = 0u64;
    {
        let body = response.body_mut();
        let mut reader = body.as_reader();
        let mut buf = [0u8; 65536];
        loop {
            let n = reader.read(&mut buf)?;
            if n == 0 {
                break;
            }
            temp_file.write_all(&buf[..n])?;
            downloaded += n as u64;
            if let Some(pb) = &pb {
                pb.set_position(downloaded);
            }
            if let Some(cb) = progress.as_deref_mut() {
                cb(downloaded, total_size);
            }
        }
    }

    if let Some(pb) = pb {
        pb.finish_with_message("Download complete");
    }

    // Catch interrupted downloads: an identity response that delivered fewer
    // bytes than Content-Length is truncated, so fail instead of saving / using
    // the partial file (the caller's retry loop will re-fetch).
    if total_size > 0 && !content_encoded && downloaded < total_size {
        return Err(anyhow!(
            "下载不完整：仅收到 {downloaded}/{total_size} 字节，可能网络中断（{url}）"
        ));
    }

    let url = url.split_once("?").map(|v| v.0).unwrap_or(url);
    if url.ends_with(".tar.gz") {
        debug!("Extracting archive...");

        temp_file.rewind()?;

        let buf_reader = BufReader::new(temp_file);
        let decoder = GzDecoder::new(buf_reader);
        let archive = Archive::new(decoder);

        let extract_dir = if folder {
            std::fs::create_dir_all(file_path)?;
            file_path
        } else {
            file_path.parent().expect("file_path must have parent")
        };

        unpack_without_top_dir(archive, extract_dir)?;
        debug!("Extraction complete.");
    } else {
        debug!("Downloaded file is not a .tar.gz archive, saving as normal file.");
        temp_file.rewind()?;
        let mut output = File::create(file_path)?;
        std::io::copy(&mut temp_file, &mut output)?;
    }

    Ok(())
}

fn normalize_join(target_dir: &Path, relative_path: &Path) -> PathBuf {
    let cleaned = relative_path.strip_prefix(".").unwrap_or(relative_path);

    let mut rel_components = cleaned.components();
    let mut tar_comp = target_dir.components();
    let first = tar_comp.next_back();
    let second = rel_components.next();

    if let (Some(Component::Normal(first)), Some(Component::Normal(second))) = (first, second) {
        if first == second {
            return target_dir.join(rel_components.collect::<PathBuf>());
        }
    }

    target_dir.join(cleaned)
}
fn unpack_without_top_dir<R: std::io::Read>(
    mut archive: Archive<R>,
    target_dir: &Path,
) -> std::io::Result<()> {
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        let components = path.components();

        let relative_path: PathBuf = components.collect();
        let out_path = normalize_join(target_dir, &relative_path);
        if let Some(name) = out_path.file_name().and_then(|v| v.to_str()) {
            if name.starts_with(".") {
                continue;
            }
        }

        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        entry.unpack(out_path)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;
    //TODO: test successfull download

    #[test]
    fn hashing() {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Debug)
            .try_init();
        assert_eq!(
            failure(
                None,
                root_path().join("models/detector/paddle/det.onnx"),
                ""
            ),
            true
        );
    }

    #[test]
    fn test_failure_returns_true_for_nonexistent_file() {
        let path = PathBuf::from("nonexistent.file");
        assert!(failure(None, path, "abc"));
    }

    #[test]
    fn test_failure_returns_false_for_correct_hash() {
        let dir = tempdir().expect("couldnt create tempdir");
        let path = dir.path().join("test.txt");
        fs::write(&path, "correct content").expect("couldnt write temp file");

        let correct_hash = format!("{:x}", Sha256::digest(b"correct content"));
        assert!(!failure(None, &path, &correct_hash));
    }

    #[test]
    #[should_panic]
    fn test_get_panics_on_double_hash_failure() {
        let db = ModelDb {};
        let _ = db.get(
            "invalid",
            "invalid",
            "bad.txt",
            "https://example.com/404.tar.gz",
            "invalidhash",
        );
    }

    #[test]
    fn test_placeholder_hash_rejects_empty_file() {
        // A "###" model has no real hash; an empty / truncated-to-zero file must
        // still be reported as not-ready so it gets re-downloaded.
        let dir = tempdir().expect("couldnt create tempdir");
        let path = dir.path().join("model.onnx");
        fs::write(&path, b"").expect("couldnt write empty file");
        assert!(failure(Some(dir.path()), path.as_path(), "###"));
    }

    #[test]
    fn test_placeholder_hash_accepts_nonempty_file() {
        let dir = tempdir().expect("couldnt create tempdir");
        let path = dir.path().join("model.onnx");
        fs::write(&path, b"real weights").expect("couldnt write file");
        assert!(!failure(Some(dir.path()), path.as_path(), "###"));
    }

    #[test]
    fn test_placeholder_hash_rejects_dir_of_empty_files() {
        let dir = tempdir().expect("couldnt create tempdir");
        let model_dir = dir.path().join("bundle");
        fs::create_dir_all(&model_dir).expect("couldnt create model dir");
        fs::write(model_dir.join("a.bin"), b"").expect("couldnt write file");
        assert!(failure(Some(dir.path()), model_dir.as_path(), "###"));
    }

    #[test]
    fn test_placeholder_hash_accepts_dir_with_nonempty_file() {
        let dir = tempdir().expect("couldnt create tempdir");
        let model_dir = dir.path().join("bundle");
        fs::create_dir_all(&model_dir).expect("couldnt create model dir");
        fs::write(model_dir.join("a.bin"), b"data").expect("couldnt write file");
        assert!(!failure(Some(dir.path()), model_dir.as_path(), "###"));
    }

    #[test]
    fn hashing2() {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Debug)
            .try_init();
        assert_eq!(
            failure(
                None,
                root_path().join("models/invalid/invalid/spm.nopretok"),
                ""
            ),
            true
        );
    }

    // Optional: test download_and_extract with a fake URL
    // (would need a test server or mock ureq)
}
