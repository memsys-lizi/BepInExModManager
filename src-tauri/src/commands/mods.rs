use std::path::{Path, PathBuf};
use serde::Serialize;

// ── 数据结构 ──────────────────────────────────────────────────

#[derive(Serialize, Clone)]
pub struct DllEntry {
    pub name: String,
    pub file_name: String,
    pub file_path: String,
    pub status: String,
}

#[derive(Serialize, Clone)]
pub struct ModEntry {
    pub id: String,
    pub name: String,
    pub mod_path: String,
    pub is_folder: bool,
    pub status: String,
    pub dlls: Vec<DllEntry>,
}

// ── 扫描 ──────────────────────────────────────────────────────

#[tauri::command]
pub fn scan_mods(game_path: String) -> Vec<ModEntry> {
    let plugins_dir = PathBuf::from(&game_path)
        .join("BepInEx")
        .join("plugins");

    if !plugins_dir.exists() {
        return vec![];
    }

    let Ok(entries) = std::fs::read_dir(&plugins_dir) else {
        return vec![];
    };

    let mut mods = Vec::new();

    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_dir() {
            if let Some(m) = scan_folder_mod(&path) {
                mods.push(m);
            }
        } else if let Some(dll) = parse_dll_entry(&path) {
            let id = hash_str(&dll.file_path);
            let status = dll.status.clone();
            mods.push(ModEntry {
                id,
                name: dll.name.clone(),
                mod_path: dll.file_path.clone(),
                is_folder: false,
                status,
                dlls: vec![dll],
            });
        }
    }

    mods
}

/// 扫描文件夹 Mod，**递归**收集所有子目录的 dll
fn scan_folder_mod(dir: &Path) -> Option<ModEntry> {
    let folder_name = dir.file_name()?.to_string_lossy().into_owned();
    let dlls = collect_dlls_recursive(dir);

    let status = if !dlls.is_empty() && dlls.iter().all(|d| d.status == "disabled") {
        "disabled"
    } else {
        "enabled"
    };

    Some(ModEntry {
        id: hash_str(&dir.to_string_lossy()),
        name: folder_name,
        mod_path: dir.to_string_lossy().into_owned(),
        is_folder: true,
        status: status.into(),
        dlls,
    })
}

/// 递归收集目录（包含子目录）中的所有 dll / dll.disabled 文件
fn collect_dlls_recursive(dir: &Path) -> Vec<DllEntry> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return vec![];
    };
    let mut dlls = Vec::new();
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_dir() {
            dlls.extend(collect_dlls_recursive(&path));
        } else if let Some(dll) = parse_dll_entry(&path) {
            dlls.push(dll);
        }
    }
    dlls
}

fn parse_dll_entry(path: &Path) -> Option<DllEntry> {
    let file_name = path.file_name()?.to_string_lossy().into_owned();
    let path_str = path.to_string_lossy().into_owned();

    if file_name.ends_with(".dll.disabled") {
        let name = file_name
            .trim_end_matches(".disabled")
            .trim_end_matches(".dll")
            .to_string();
        return Some(DllEntry { name, file_name, file_path: path_str, status: "disabled".into() });
    }
    if file_name.ends_with(".dll") {
        let name = file_name.trim_end_matches(".dll").to_string();
        return Some(DllEntry { name, file_name, file_path: path_str, status: "enabled".into() });
    }
    None
}

// ── 启用 / 禁用 ───────────────────────────────────────────────

#[tauri::command]
pub fn enable_mod(mod_path: String, is_folder: bool) -> Result<String, String> {
    let path = PathBuf::from(&mod_path);
    if is_folder {
        toggle_dlls_recursive(&path, true)?;
        Ok(mod_path)
    } else {
        let new_path = PathBuf::from(mod_path.trim_end_matches(".disabled"));
        std::fs::rename(&path, &new_path).map_err(|e| e.to_string())?;
        Ok(new_path.to_string_lossy().into_owned())
    }
}

#[tauri::command]
pub fn disable_mod(mod_path: String, is_folder: bool) -> Result<String, String> {
    let path = PathBuf::from(&mod_path);
    if is_folder {
        toggle_dlls_recursive(&path, false)?;
        Ok(mod_path)
    } else {
        let new_path = PathBuf::from(format!("{}.disabled", mod_path));
        std::fs::rename(&path, &new_path).map_err(|e| e.to_string())?;
        Ok(new_path.to_string_lossy().into_owned())
    }
}

/// 递归切换目录内所有 dll 的启用/禁用状态
fn toggle_dlls_recursive(dir: &Path, enable: bool) -> Result<(), String> {
    let Ok(entries) = std::fs::read_dir(dir) else { return Ok(()); };
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_dir() {
            toggle_dlls_recursive(&path, enable)?;
            continue;
        }
        if !path.is_file() { continue; }
        let name = path.file_name().unwrap_or_default().to_string_lossy().into_owned();
        if enable && name.ends_with(".dll.disabled") {
            let new_path = PathBuf::from(
                path.to_string_lossy().trim_end_matches(".disabled").to_string()
            );
            std::fs::rename(&path, &new_path).map_err(|e| e.to_string())?;
        } else if !enable && name.ends_with(".dll") {
            let new_path = PathBuf::from(format!("{}.disabled", path.to_string_lossy()));
            std::fs::rename(&path, &new_path).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
pub fn delete_mod(mod_path: String, is_folder: bool) -> Result<(), String> {
    let path = PathBuf::from(&mod_path);
    if is_folder {
        std::fs::remove_dir_all(&path).map_err(|e| e.to_string())
    } else {
        std::fs::remove_file(&path).map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub fn open_plugins_dir(game_path: String) -> Result<(), String> {
    let dir = PathBuf::from(&game_path).join("BepInEx").join("plugins");
    if !dir.exists() {
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "windows")]
    std::process::Command::new("explorer")
        .arg(&dir)
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

// ── 安装 Mod ─────────────────────────────────────────────────

#[derive(Serialize)]
pub struct InstallModResult {
    pub mod_name: String,
    pub mod_path: String,
    pub is_folder: bool,
    /// 是否存在同名目录冲突（此时文件尚未安装）
    pub conflict: bool,
    /// 冲突时的已存在路径
    pub conflict_path: Option<String>,
}

/// 冲突处理策略（前端传入）
#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ConflictStrategy {
    /// 自动重命名（加 _1 _2 …）
    Rename,
    /// 覆盖（先删除旧目录再安装）
    Overwrite,
    /// 取消安装
    Cancel,
}

/// 安装 Mod（本地文件）
/// conflict_strategy：仅在目标已存在时生效
#[tauri::command]
pub fn install_mod(
    game_path: String,
    source_path: String,
    conflict_strategy: Option<ConflictStrategy>,
) -> Result<InstallModResult, String> {
    let src = PathBuf::from(&source_path);
    let plugins_dir = PathBuf::from(&game_path).join("BepInEx").join("plugins");
    if !plugins_dir.exists() {
        std::fs::create_dir_all(&plugins_dir).map_err(|e| e.to_string())?;
    }

    let ext = src.extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    match ext.as_str() {
        "zip" => install_from_zip(&src, &plugins_dir, conflict_strategy),
        "dll" => install_from_dll(&src, &plugins_dir, conflict_strategy),
        _ => Err(format!("不支持的文件类型 .{}，请使用 .zip 或 .dll", ext)),
    }
}

/// 通过 URL 下载后安装
#[tauri::command]
pub async fn install_mod_from_url(
    game_path: String,
    url: String,
    conflict_strategy: Option<ConflictStrategy>,
) -> Result<InstallModResult, String> {
    let plugins_dir = PathBuf::from(&game_path).join("BepInEx").join("plugins");
    if !plugins_dir.exists() {
        std::fs::create_dir_all(&plugins_dir).map_err(|e| e.to_string())?;
    }

    // 判断扩展名（从 URL 路径推断）
    let url_path = url.split('?').next().unwrap_or(&url);
    let ext = url_path.rsplit('.').next()
        .map(|e| e.to_lowercase())
        .unwrap_or_default();

    if ext != "zip" && ext != "dll" {
        return Err(format!("无法识别 URL 中的文件类型（.{}），仅支持 .zip 和 .dll", ext));
    }

    // 下载到临时文件
    let client = reqwest::Client::builder()
        .user_agent("BepInExModManager/0.1")
        .build()
        .map_err(|e| e.to_string())?;

    let bytes = client.get(&url)
        .send()
        .await
        .map_err(|e| format!("下载失败: {}", e))?
        .bytes()
        .await
        .map_err(|e| format!("读取响应失败: {}", e))?;

    // 写入临时文件
    let tmp_path = std::env::temp_dir().join(format!("bmm_mod_download.{}", ext));
    std::fs::write(&tmp_path, &bytes).map_err(|e| e.to_string())?;

    let result = match ext.as_str() {
        "zip" => install_from_zip(&tmp_path, &plugins_dir, conflict_strategy),
        "dll" => install_from_dll(&tmp_path, &plugins_dir, conflict_strategy),
        _ => unreachable!(),
    };

    let _ = std::fs::remove_file(&tmp_path);
    result
}

// ── 安装辅助 ─────────────────────────────────────────────────

fn install_from_zip(
    zip_path: &Path,
    plugins_dir: &Path,
    strategy: Option<ConflictStrategy>,
) -> Result<InstallModResult, String> {
    let file = std::fs::File::open(zip_path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| format!("解压失败: {}", e))?;

    // 确定 Mod 文件夹名
    let base_name = zip_top_folder(&mut archive).unwrap_or_else(|| {
        zip_path.file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "UnknownMod".into())
    });

    let dest_dir = resolve_dest_dir(plugins_dir, &base_name, strategy)?;
    // resolve_dest_dir 返回 None 表示用户取消
    let dest_dir = match dest_dir {
        Some(d) => d,
        None => {
            return Ok(InstallModResult {
                mod_name: base_name.clone(),
                mod_path: plugins_dir.join(&base_name).to_string_lossy().into_owned(),
                is_folder: true,
                conflict: true,
                conflict_path: None,
            });
        }
    };

    // 如果目录已存在且是覆盖策略，先清空
    if dest_dir.exists() {
        std::fs::remove_dir_all(&dest_dir).map_err(|e| e.to_string())?;
    }
    std::fs::create_dir_all(&dest_dir).map_err(|e| e.to_string())?;

    // 重新打开压缩包（第一次用于探测已被消耗）
    let file2 = std::fs::File::open(zip_path).map_err(|e| e.to_string())?;
    let mut archive2 = zip::ZipArchive::new(file2).map_err(|e| e.to_string())?;

    for i in 0..archive2.len() {
        let mut zip_file = archive2.by_index(i).map_err(|e| e.to_string())?;
        let raw_name = zip_file.name().to_string();

        // 去掉顶层文件夹前缀（如果有）
        let relative = strip_top_prefix(&raw_name, &base_name);
        if relative.is_empty() { continue; }
        if raw_name.ends_with('/') {
            // 目录条目，创建即可
            std::fs::create_dir_all(dest_dir.join(&relative))
                .map_err(|e| e.to_string())?;
            continue;
        }

        let out_path = dest_dir.join(&relative);
        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let mut out = std::fs::File::create(&out_path).map_err(|e| e.to_string())?;
        std::io::copy(&mut zip_file, &mut out).map_err(|e| e.to_string())?;
    }

    let mod_name = dest_dir.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or(base_name);

    Ok(InstallModResult {
        mod_name,
        mod_path: dest_dir.to_string_lossy().into_owned(),
        is_folder: true,
        conflict: false,
        conflict_path: None,
    })
}

fn install_from_dll(
    dll_path: &Path,
    plugins_dir: &Path,
    strategy: Option<ConflictStrategy>,
) -> Result<InstallModResult, String> {
    let dll_name = dll_path.file_name()
        .ok_or("无效文件名")?
        .to_string_lossy()
        .into_owned();
    let base_name = dll_path.file_stem()
        .ok_or("无效文件名")?
        .to_string_lossy()
        .into_owned();

    let dest_folder = resolve_dest_dir(plugins_dir, &base_name, strategy)?;
    let dest_folder = match dest_folder {
        Some(d) => d,
        None => {
            return Ok(InstallModResult {
                mod_name: base_name.clone(),
                mod_path: plugins_dir.join(&base_name).to_string_lossy().into_owned(),
                is_folder: true,
                conflict: true,
                conflict_path: None,
            });
        }
    };

    if dest_folder.exists() {
        std::fs::remove_dir_all(&dest_folder).map_err(|e| e.to_string())?;
    }
    std::fs::create_dir_all(&dest_folder).map_err(|e| e.to_string())?;

    let dest_file = dest_folder.join(&dll_name);
    std::fs::copy(dll_path, &dest_file).map_err(|e| e.to_string())?;

    let mod_name = dest_folder.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or(base_name);

    Ok(InstallModResult {
        mod_name,
        mod_path: dest_folder.to_string_lossy().into_owned(),
        is_folder: true,
        conflict: false,
        conflict_path: None,
    })
}

/// 根据策略决定实际目标目录。
/// - 不存在 → 直接返回原始路径
/// - 存在 + 无策略 → 返回冲突信息（`Ok(None)` 表示需要前端干预，但此处直接返回冲突）
/// - 存在 + Rename → 自动追加 _1 _2 …
/// - 存在 + Overwrite → 返回原始路径（调用方会覆盖）
/// - 存在 + Cancel → 返回 None（调用方跳过）
fn resolve_dest_dir(
    plugins_dir: &Path,
    base_name: &str,
    strategy: Option<ConflictStrategy>,
) -> Result<Option<PathBuf>, String> {
    let candidate = plugins_dir.join(base_name);

    if !candidate.exists() {
        return Ok(Some(candidate));
    }

    // 已存在，根据策略处理
    match strategy {
        None => {
            // 没有策略：返回 None，前端会收到 conflict=true 并弹窗选择
            Ok(None)
        }
        Some(ConflictStrategy::Cancel) => Ok(None),
        Some(ConflictStrategy::Overwrite) => Ok(Some(candidate)),
        Some(ConflictStrategy::Rename) => {
            // 自动追加 _1 _2 … 直到找到空位
            for i in 1..=99 {
                let new_name = format!("{}_{}", base_name, i);
                let new_path = plugins_dir.join(&new_name);
                if !new_path.exists() {
                    return Ok(Some(new_path));
                }
            }
            Err(format!("无法找到空闲的文件夹名（已尝试 {}_1 ~ {}_99）", base_name, base_name))
        }
    }
}

// ── 启动游戏 ──────────────────────────────────────────────────

#[tauri::command]
pub fn launch_game(game_path: String, exe_name: String) -> Result<(), String> {
    let exe = PathBuf::from(&game_path).join(&exe_name);
    if !exe.exists() {
        return Err(format!("找不到可执行文件：{}", exe.display()));
    }
    std::process::Command::new(&exe)
        .current_dir(&game_path)
        .spawn()
        .map_err(|e| format!("启动失败: {}", e))?;
    Ok(())
}

// ── 辅助 ─────────────────────────────────────────────────────

fn zip_top_folder(archive: &mut zip::ZipArchive<std::fs::File>) -> Option<String> {
    let mut top: std::collections::HashSet<String> = std::collections::HashSet::new();
    for i in 0..archive.len() {
        if let Ok(f) = archive.by_index(i) {
            if let Some(first) = f.name().split('/').next() {
                if !first.is_empty() {
                    top.insert(first.to_string());
                }
            }
        }
    }
    if top.len() == 1 { top.into_iter().next() } else { None }
}

fn strip_top_prefix(path: &str, folder: &str) -> String {
    let prefix = format!("{}/", folder);
    if path.starts_with(&prefix) {
        path[prefix.len()..].to_string()
    } else {
        path.to_string()
    }
}

fn hash_str(s: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut h = DefaultHasher::new();
    s.hash(&mut h);
    format!("{:x}", h.finish())
}
