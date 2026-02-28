use std::path::{Path, PathBuf};
use serde::Serialize;

// ── 数据结构 ──────────────────────────────────────────────────

/// plugins 目录下每个 dll 文件的信息
#[derive(Serialize, Clone)]
pub struct DllEntry {
    pub name: String,       // 文件名（不含 .disabled）
    pub file_name: String,  // 实际文件名
    pub file_path: String,  // 完整路径
    pub status: String,     // "enabled" | "disabled"
}

/// 一个 Mod 条目（对应一个文件夹，或根目录散装 dll）
#[derive(Serialize, Clone)]
pub struct ModEntry {
    pub id: String,
    pub name: String,
    pub mod_path: String,       // 文件夹路径（散装 dll 时为文件路径）
    pub is_folder: bool,        // true=文件夹 Mod，false=散装 dll
    pub status: String,         // "enabled" | "disabled"（文件夹：全部 dll 都禁用才算禁用）
    pub dlls: Vec<DllEntry>,    // 包含的 dll 列表
}

// ── 扫描 ──────────────────────────────────────────────────────

/// 扫描 BepInEx/plugins，返回 Mod 列表（文件夹优先，兼容散装 dll）
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
            // 文件夹 = 一个 Mod
            if let Some(m) = scan_folder_mod(&path) {
                mods.push(m);
            }
        } else {
            // 根目录的散装 dll
            if let Some(dll) = parse_dll_entry(&path) {
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
    }

    mods
}

/// 扫描一个文件夹，生成文件夹级 Mod
fn scan_folder_mod(dir: &Path) -> Option<ModEntry> {
    let folder_name = dir.file_name()?.to_string_lossy().into_owned();

    let Ok(entries) = std::fs::read_dir(dir) else {
        return None;
    };

    let mut dlls: Vec<DllEntry> = Vec::new();
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(dll) = parse_dll_entry(&path) {
                dlls.push(dll);
            }
        }
    }

    // Mod 状态：所有 dll 都禁用 → disabled，否则 → enabled
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

/// 解析单个 dll 或 dll.disabled 文件
fn parse_dll_entry(path: &Path) -> Option<DllEntry> {
    let file_name = path.file_name()?.to_string_lossy().into_owned();
    let path_str = path.to_string_lossy().into_owned();

    if file_name.ends_with(".dll.disabled") {
        let name = file_name
            .trim_end_matches(".disabled")
            .trim_end_matches(".dll")
            .to_string();
        return Some(DllEntry {
            name,
            file_name,
            file_path: path_str,
            status: "disabled".into(),
        });
    }

    if file_name.ends_with(".dll") {
        let name = file_name.trim_end_matches(".dll").to_string();
        return Some(DllEntry {
            name,
            file_name,
            file_path: path_str,
            status: "enabled".into(),
        });
    }

    None
}

// ── 启用 / 禁用（文件夹粒度） ─────────────────────────────────

/// 启用 Mod：
///   - 文件夹型：对文件夹内所有 .dll.disabled 重命名为 .dll，文件夹本身不动
///   - 散装 dll：将 .dll.disabled → .dll
#[tauri::command]
pub fn enable_mod(mod_path: String, is_folder: bool) -> Result<String, String> {
    let path = PathBuf::from(&mod_path);

    if is_folder {
        toggle_dlls_in_dir(&path, true)?;
        Ok(mod_path)
    } else {
        let new_path = PathBuf::from(mod_path.trim_end_matches(".disabled"));
        std::fs::rename(&path, &new_path).map_err(|e| e.to_string())?;
        Ok(new_path.to_string_lossy().into_owned())
    }
}

/// 禁用 Mod：
///   - 文件夹型：对文件夹内所有 .dll 重命名为 .dll.disabled，文件夹本身不动
///   - 散装 dll：将 .dll → .dll.disabled
#[tauri::command]
pub fn disable_mod(mod_path: String, is_folder: bool) -> Result<String, String> {
    let path = PathBuf::from(&mod_path);

    if is_folder {
        toggle_dlls_in_dir(&path, false)?;
        Ok(mod_path)
    } else {
        let new_path = PathBuf::from(format!("{}.disabled", mod_path));
        std::fs::rename(&path, &new_path).map_err(|e| e.to_string())?;
        Ok(new_path.to_string_lossy().into_owned())
    }
}

/// 删除 Mod：文件夹 → remove_dir_all；散装 dll → remove_file
#[tauri::command]
pub fn delete_mod(mod_path: String, is_folder: bool) -> Result<(), String> {
    let path = PathBuf::from(&mod_path);
    if is_folder {
        std::fs::remove_dir_all(&path).map_err(|e| e.to_string())
    } else {
        std::fs::remove_file(&path).map_err(|e| e.to_string())
    }
}

/// 打开 plugins 文件夹
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

// ── 辅助 ─────────────────────────────────────────────────────

/// enable=true：把文件夹内 .dll.disabled → .dll
/// enable=false：把文件夹内 .dll → .dll.disabled
fn toggle_dlls_in_dir(dir: &Path, enable: bool) -> Result<(), String> {
    let Ok(entries) = std::fs::read_dir(dir) else { return Ok(()); };
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
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

fn hash_str(s: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut h = DefaultHasher::new();
    s.hash(&mut h);
    format!("{:x}", h.finish())
}
