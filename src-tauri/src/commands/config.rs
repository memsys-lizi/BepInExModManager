use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigEntry {
    pub key: String,
    pub value: String,
    pub description: Option<String>,
    pub entry_type: String, // "string" | "number" | "boolean" | "enum"
    pub options: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigSection {
    pub name: String,
    pub entries: Vec<ConfigEntry>,
}

/// 列出某游戏 BepInEx/config 目录下所有 .cfg 文件
#[tauri::command]
pub fn list_config_files(game_path: String) -> Vec<String> {
    let config_dir = PathBuf::from(&game_path).join("BepInEx").join("config");
    if !config_dir.exists() {
        return vec![];
    }

    std::fs::read_dir(&config_dir)
        .ok()
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path()
                        .extension()
                        .map(|x| x == "cfg")
                        .unwrap_or(false)
                })
                .map(|e| e.path().to_string_lossy().into_owned())
                .collect()
        })
        .unwrap_or_default()
}

/// 解析 BepInEx .cfg 文件（INI 格式）
#[tauri::command]
pub fn read_config_file(file_path: String) -> Result<Vec<ConfigSection>, String> {
    let content = std::fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
    Ok(parse_cfg(&content))
}

/// 将修改后的 sections 写回 .cfg 文件
#[tauri::command]
pub fn write_config_file(
    file_path: String,
    sections: Vec<ConfigSection>,
) -> Result<(), String> {
    let mut out = String::new();

    for section in &sections {
        out.push_str(&format!("[{}]\n", section.name));
        for entry in &section.entries {
            if let Some(desc) = &entry.description {
                for line in desc.lines() {
                    out.push_str(&format!("## {}\n", line));
                }
            }
            out.push_str(&format!("{} = {}\n", entry.key, entry.value));
        }
        out.push('\n');
    }

    std::fs::write(&file_path, out).map_err(|e| e.to_string())
}

/// 解析 INI/CFG 格式文本
fn parse_cfg(content: &str) -> Vec<ConfigSection> {
    let mut sections: Vec<ConfigSection> = Vec::new();
    let mut current_section: Option<String> = None;
    let mut current_entries: Vec<ConfigEntry> = Vec::new();
    let mut pending_desc: Vec<String> = Vec::new();

    for raw_line in content.lines() {
        let line = raw_line.trim();

        // 空行
        if line.is_empty() {
            pending_desc.clear();
            continue;
        }

        // Section header [SectionName]
        if line.starts_with('[') && line.ends_with(']') {
            // 保存上一个 section
            if let Some(name) = current_section.take() {
                sections.push(ConfigSection {
                    name,
                    entries: std::mem::take(&mut current_entries),
                });
            }
            current_section = Some(line[1..line.len() - 1].to_string());
            pending_desc.clear();
            continue;
        }

        // 注释行 ## 或 # (BepInEx 用 ## 描述)
        if line.starts_with("##") {
            pending_desc.push(line.trim_start_matches('#').trim().to_string());
            continue;
        }
        if line.starts_with('#') || line.starts_with(';') {
            pending_desc.clear();
            continue;
        }

        // Key = Value
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim().to_string();
            let value = value.trim().to_string();
            let description = if pending_desc.is_empty() {
                None
            } else {
                Some(pending_desc.join("\n"))
            };
            pending_desc.clear();

            // 尝试推断类型
            let entry_type = infer_type(&value);

            current_entries.push(ConfigEntry {
                key,
                value,
                description,
                entry_type,
                options: None,
            });
        }
    }

    // 最后一个 section
    if let Some(name) = current_section {
        sections.push(ConfigSection {
            name,
            entries: current_entries,
        });
    }

    sections
}

fn infer_type(value: &str) -> String {
    let v = value.to_lowercase();
    if v == "true" || v == "false" {
        "boolean".into()
    } else if value.parse::<f64>().is_ok() {
        "number".into()
    } else {
        "string".into()
    }
}
