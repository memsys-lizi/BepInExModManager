use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

// ── 数据结构 ──────────────────────────────────────────────────

/// 各特征点检查结果，前端可用于展示详情
/// 参考 BepInEx 5.x 初始安装包的真实文件结构
#[derive(Serialize, Clone)]
pub struct BepInExIntegrity {
    pub winhttp_dll: bool,        // winhttp.dll —— doorstop 注入器（必须）
    pub doorstop_cfg: bool,       // doorstop_config.ini —— doorstop 配置（必须）
    pub doorstop_version: bool,   // .doorstop_version —— 版本标记（必须）
    pub core_dir: bool,           // BepInEx/core/ 目录存在（必须）
    pub core_bepinex_dll: bool,   // core/BepInEx.dll 或同类核心 dll（必须）
    pub changelog: bool,          // changelog.txt —— 可选，但通常存在
    pub score: u8,                // 必须项满分 5，>=4 视为已安装（changelog 可缺）
}

#[derive(Serialize, Clone)]
pub struct BepInExStatus {
    pub installed: bool,
    pub version: Option<String>,
    pub path: Option<String>,
    pub integrity: BepInExIntegrity,
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
    prerelease: bool,
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
// 判断依据：BepInEx 5.x 初始安装包的真实文件结构
//   游戏目录/
//   ├── winhttp.dll            ← doorstop 注入器（必须）
//   ├── doorstop_config.ini    ← doorstop 配置（必须）
//   ├── .doorstop_version      ← 版本标记（必须）
//   ├── changelog.txt          ← 可选
//   └── BepInEx/core/
//       └── BepInEx.dll        ← 核心（必须；6.x 为 BepInEx.Unity.*.dll）

#[tauri::command]
pub fn check_bepinex_status(game_path: String) -> BepInExStatus {
    let game_dir = PathBuf::from(&game_path);
    let bep_dir  = game_dir.join("BepInEx");
    let core_dir = bep_dir.join("core");

    // ── 逐项检查 ──────────────────────────────────────────────
    let winhttp_dll      = game_dir.join("winhttp.dll").is_file();
    let doorstop_cfg     = game_dir.join("doorstop_config.ini").is_file();
    let doorstop_version = game_dir.join(".doorstop_version").is_file();
    let core_dir_ok      = core_dir.is_dir();
    let changelog        = game_dir.join("changelog.txt").is_file();

    // core/ 下必须有 BepInEx.dll（5.x）或 BepInEx.Unity.*.dll（6.x）等核心 dll
    let core_bepinex_dll = core_dir_ok && dir_contains_bepinex_dll(&core_dir);

    // ── 完整性评分（必须项各 1 分，满分 5；changelog 可选不计分）─
    // 阈值 >=4：允许 .doorstop_version 或 core_dll 其中一项缺失仍视为安装
    let score: u8 = (winhttp_dll as u8)
        + (doorstop_cfg as u8)
        + (doorstop_version as u8)
        + (core_dir_ok as u8)
        + (core_bepinex_dll as u8);

    let integrity = BepInExIntegrity {
        winhttp_dll,
        doorstop_cfg,
        doorstop_version,
        core_dir: core_dir_ok,
        core_bepinex_dll,
        changelog,
        score,
    };

    if score < 4 {
        return BepInExStatus { installed: false, version: None, path: None, integrity };
    }

    let version = read_bepinex_version(&game_dir);

    BepInExStatus {
        installed: true,
        version,
        path: Some(bep_dir.to_string_lossy().into_owned()),
        integrity,
    }
}

/// core/ 下是否存在以 "BepInEx" 开头的 .dll（兼容 5.x / 6.x 核心命名）
fn dir_contains_bepinex_dll(dir: &Path) -> bool {
    std::fs::read_dir(dir)
        .ok()
        .map(|rd| {
            rd.flatten().any(|e| {
                let name = e.file_name();
                let name = name.to_string_lossy();
                name.starts_with("BepInEx") && name.ends_with(".dll")
            })
        })
        .unwrap_or(false)
}

/// 读取版本号，优先解析 changelog.txt 首行
fn read_bepinex_version(game_dir: &Path) -> Option<String> {
    let changelog = game_dir.join("changelog.txt");
    if let Ok(text) = std::fs::read_to_string(&changelog) {
        // changelog.txt 首行通常形如 "## BepInEx v5.4.23.2" 或 "BepInEx 5.4.23.2"
        for line in text.lines() {
            let trimmed = line.trim().trim_start_matches('#').trim();
            if trimmed.is_empty() {
                continue;
            }
            // 提取版本号：找到 v\d 或纯数字开头的 token
            if let Some(ver) = extract_version_token(trimmed) {
                return Some(ver);
            }
            // 找不到版本号但行非空，直接返回原行（至少有信息）
            return Some(trimmed.to_string());
        }
    }
    None
}

/// 从字符串中提取形如 "v5.4.23" 或 "5.4.23" 的版本号
fn extract_version_token(s: &str) -> Option<String> {
    for token in s.split_whitespace() {
        let t = token.trim_start_matches('v');
        // 版本号：至少两段数字（如 5.4）
        let parts: Vec<&str> = t.split('.').collect();
        if parts.len() >= 2 && parts.iter().all(|p| p.parse::<u32>().is_ok()) {
            return Some(format!("v{}", t));
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

    for release in releases.iter().take(20) {
        // 跳过预发布版（6.0.0-pre.x 等）
        if release.prerelease {
            continue;
        }

        for asset in &release.assets {
            let name = &asset.name;

            // 只处理 zip 包
            if !name.ends_with(".zip") {
                continue;
            }

            // 5.x 命名：BepInEx_win_x64_5.4.23.5.zip
            // 5.x 旧命名：BepInEx_x64_5.4.22.0.zip（无平台前缀）
            // 必须是 Windows 包，linux/macos/unix 全部排除
            if name.contains("linux") || name.contains("unix") || name.contains("macos") {
                continue;
            }
            // Patcher 包跳过（不是完整安装包）
            if name.contains("Patcher") {
                continue;
            }

            // 架构：优先匹配 win_x64 / win_x86，兼容旧的 _x64 / _x86
            let arch = if name.contains("x64") {
                "x64"
            } else if name.contains("x86") {
                "x86"
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
