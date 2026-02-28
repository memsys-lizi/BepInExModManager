use std::path::{Path, PathBuf};
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct ModEntry {
    pub id: String,
    pub name: String,
    pub file_name: String,
    pub file_path: String,
    pub status: String, // "enabled" | "disabled"
}

/// 扫描游戏 BepInEx/plugins 目录，返回所有 Mod
#[tauri::command]
pub fn scan_mods(game_path: String) -> Vec<ModEntry> {
    let plugins_dir = PathBuf::from(&game_path)
        .join("BepInEx")
        .join("plugins");

    if !plugins_dir.exists() {
        return vec![];
    }

    collect_mods(&plugins_dir)
}

fn collect_mods(dir: &Path) -> Vec<ModEntry> {
    let mut result = Vec::new();

    let Ok(entries) = std::fs::read_dir(dir) else {
        return result;
    };

    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_dir() {
            // 递归子目录
            result.extend(collect_mods(&path));
            continue;
        }

        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        // .dll → 启用；.dll.disabled → 禁用
        let (status, real_name) = if ext == "dll" {
            ("enabled", path.file_stem().unwrap_or_default().to_string_lossy().into_owned())
        } else if path.to_string_lossy().ends_with(".dll.disabled") {
            let stem = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .trim_end_matches(".disabled")
                .trim_end_matches(".dll")
                .to_owned();
            ("disabled", stem)
        } else {
            continue;
        };

        let file_path = path.to_string_lossy().into_owned();
        let file_name = path.file_name().unwrap_or_default().to_string_lossy().into_owned();
        let id = format!("{:x}", md5_str(&file_path));

        result.push(ModEntry {
            id,
            name: real_name,
            file_name,
            file_path,
            status: status.into(),
        });
    }

    result
}

/// 启用 Mod：将 .dll.disabled → .dll
#[tauri::command]
pub fn enable_mod(file_path: String) -> Result<String, String> {
    let path = PathBuf::from(&file_path);
    if !path.exists() {
        return Err("文件不存在".into());
    }

    let new_path = PathBuf::from(file_path.trim_end_matches(".disabled"));
    std::fs::rename(&path, &new_path).map_err(|e| e.to_string())?;
    Ok(new_path.to_string_lossy().into_owned())
}

/// 禁用 Mod：将 .dll → .dll.disabled
#[tauri::command]
pub fn disable_mod(file_path: String) -> Result<String, String> {
    let path = PathBuf::from(&file_path);
    if !path.exists() {
        return Err("文件不存在".into());
    }

    let new_path = PathBuf::from(format!("{}.disabled", file_path));
    std::fs::rename(&path, &new_path).map_err(|e| e.to_string())?;
    Ok(new_path.to_string_lossy().into_owned())
}

/// 删除 Mod 文件
#[tauri::command]
pub fn delete_mod(file_path: String) -> Result<(), String> {
    std::fs::remove_file(&file_path).map_err(|e| e.to_string())
}

/// 打开 plugins 文件夹（通过 shell）
#[tauri::command]
pub fn open_plugins_dir(game_path: String) -> Result<(), String> {
    let dir = PathBuf::from(&game_path).join("BepInEx").join("plugins");
    if !dir.exists() {
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "windows")]
    std::process::Command::new("explorer")
        .arg(dir)
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

// 简单的字符串哈希，不依赖 md5 crate
fn md5_str(s: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut h = DefaultHasher::new();
    s.hash(&mut h);
    h.finish()
}
