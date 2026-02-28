use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

// ── 数据结构 ──────────────────────────────────────────────────

#[derive(Serialize, Clone)]
pub struct BepInExStatus {
    pub installed: bool,
    pub version: Option<String>,
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ReleaseAsset {
    pub version: String,
    pub tag_name: String,
    pub download_url: String,
    pub arch: String,
    pub published_at: String,
}

#[derive(Deserialize)]
struct GhRelease {
    tag_name: String,
    published_at: String,
    assets: Vec<GhAsset>,
}

#[derive(Deserialize)]
struct GhAsset {
    name: String,
    browser_download_url: String,
}

// ── 辅助：构建带可选代理的 HTTP Client ────────────────────────

fn build_client(proxy: Option<&str>) -> Result<reqwest::Client, String> {
    let mut builder = reqwest::Client::builder()
        .user_agent("BepInExModManager/0.1");

    if let Some(proxy_url) = proxy {
        if !proxy_url.is_empty() {
            let proxy = reqwest::Proxy::all(proxy_url)
                .map_err(|e| format!("代理配置无效: {}", e))?;
            builder = builder.proxy(proxy);
        }
    }

    builder.build().map_err(|e| e.to_string())
}

// ── 命令：检测安装状态 ────────────────────────────────────────

#[tauri::command]
pub fn check_bepinex_status(game_path: String) -> BepInExStatus {
    let bep_dir = PathBuf::from(&game_path).join("BepInEx");
    let core_dir = bep_dir.join("core");
    let chainloader = core_dir.join("BepInEx.Core.dll");

    if !chainloader.exists() {
        return BepInExStatus { installed: false, version: None, path: None };
    }

    let version = read_bepinex_version(&core_dir);

    BepInExStatus {
        installed: true,
        version,
        path: Some(bep_dir.to_string_lossy().into_owned()),
    }
}

fn read_bepinex_version(core_dir: &Path) -> Option<String> {
    let changelog = core_dir.parent()?.parent()?.join("changelog.txt");
    if let Ok(text) = std::fs::read_to_string(&changelog) {
        if let Some(line) = text.lines().next() {
            return Some(line.trim().to_string());
        }
    }
    None
}

// ── 命令：获取 GitHub 发布列表 ────────────────────────────────

#[tauri::command]
pub async fn fetch_bepinex_releases(proxy: Option<String>) -> Result<Vec<ReleaseAsset>, String> {
    let client = build_client(proxy.as_deref())?;

    let releases: Vec<GhRelease> = client
        .get("https://api.github.com/repos/BepInEx/BepInEx/releases")
        .send()
        .await
        .map_err(|e| format!("网络请求失败: {}", e))?
        .json()
        .await
        .map_err(|e| format!("解析响应失败: {}", e))?;

    let mut assets: Vec<ReleaseAsset> = Vec::new();

    for release in releases.iter().take(10) {
        for asset in &release.assets {
            let name = &asset.name;
            if !name.ends_with(".zip") {
                continue;
            }
            let arch = if name.contains("x64") {
                "x64"
            } else if name.contains("x86") {
                "x86"
            } else if name.contains("unix") || name.contains("linux") {
                "unix"
            } else {
                continue;
            };

            let version = release.tag_name.trim_start_matches('v').to_string();

            assets.push(ReleaseAsset {
                version,
                tag_name: release.tag_name.clone(),
                download_url: asset.browser_download_url.clone(),
                arch: arch.into(),
                published_at: release.published_at[..10].to_string(),
            });
        }
    }

    Ok(assets)
}

// ── 命令：下载并安装 BepInEx ─────────────────────────────────

#[tauri::command]
pub async fn install_bepinex(
    app: AppHandle,
    game_path: String,
    download_url: String,
    version: String,
    proxy: Option<String>,
) -> Result<(), String> {
    emit_progress(&app, "downloading", 0, "正在连接...");

    let client = build_client(proxy.as_deref())?;

    let resp = client
        .get(&download_url)
        .send()
        .await
        .map_err(|e| format!("下载失败: {}", e))?;

    let total = resp.content_length().unwrap_or(0);
    let mut downloaded: u64 = 0;
    let mut bytes = Vec::new();

    use futures_util::StreamExt;
    let mut stream = resp.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("读取数据失败: {}", e))?;
        bytes.extend_from_slice(&chunk);
        downloaded += chunk.len() as u64;
        if total > 0 {
            let pct = (downloaded * 60 / total) as u8;
            emit_progress(&app, "downloading", pct, "正在下载...");
        }
    }

    emit_progress(&app, "extracting", 61, "正在解压...");

    let cursor = std::io::Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(cursor)
        .map_err(|e| format!("解压失败: {}", e))?;

    let total_files = archive.len();
    let game_dir = PathBuf::from(&game_path);

    for i in 0..total_files {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let out_path = game_dir.join(file.name());

        if file.name().ends_with('/') {
            std::fs::create_dir_all(&out_path).map_err(|e| e.to_string())?;
        } else {
            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            let mut out_file = std::fs::File::create(&out_path)
                .map_err(|e| e.to_string())?;
            std::io::copy(&mut file, &mut out_file).map_err(|e| e.to_string())?;
        }

        let pct = 61 + (i * 35 / total_files.max(1)) as u8;
        emit_progress(&app, "extracting", pct, "正在解压...");
    }

    std::fs::create_dir_all(game_dir.join("BepInEx").join("plugins"))
        .map_err(|e| e.to_string())?;

    emit_progress(&app, "done", 100, &format!("安装完成 v{}", version));
    Ok(())
}

// ── 命令：卸载 BepInEx ────────────────────────────────────────

#[tauri::command]
pub fn uninstall_bepinex(game_path: String) -> Result<(), String> {
    let bep_dir = PathBuf::from(&game_path).join("BepInEx");

    if bep_dir.exists() {
        for sub in &["core", "patchers", "cache", "LogOutput.log"] {
            let p = bep_dir.join(sub);
            if p.is_dir() {
                std::fs::remove_dir_all(&p).map_err(|e| e.to_string())?;
            } else if p.is_file() {
                std::fs::remove_file(&p).map_err(|e| e.to_string())?;
            }
        }
    }

    let game_dir = PathBuf::from(&game_path);
    for file in &[
        "winhttp.dll",
        "doorstop_config.ini",
        ".doorstop_version",
        "changelog.txt",
    ] {
        let p = game_dir.join(file);
        if p.exists() {
            std::fs::remove_file(&p).ok();
        }
    }

    Ok(())
}

// ── 辅助 ─────────────────────────────────────────────────────

#[derive(Clone, Serialize)]
struct ProgressPayload {
    stage: String,
    percent: u8,
    message: String,
}

fn emit_progress(app: &AppHandle, stage: &str, percent: u8, message: &str) {
    let _ = app.emit("bepinex://progress", ProgressPayload {
        stage: stage.into(),
        percent,
        message: message.into(),
    });
}
