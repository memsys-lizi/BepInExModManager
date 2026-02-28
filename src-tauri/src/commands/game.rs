use std::path::Path;
use serde::Serialize;

#[derive(Serialize)]
pub struct GameValidation {
    pub valid: bool,
    pub has_data_folder: bool,   // 有 *_Data 文件夹 → Unity 游戏
    pub has_exe: bool,
    pub exe_name: Option<String>,
    pub message: String,
}

/// 验证路径是否为合法的 Unity 游戏目录
#[tauri::command]
pub fn validate_game_dir(path: String) -> GameValidation {
    let dir = Path::new(&path);

    if !dir.exists() || !dir.is_dir() {
        return GameValidation {
            valid: false,
            has_data_folder: false,
            has_exe: false,
            exe_name: None,
            message: "目录不存在".into(),
        };
    }

    // 查找 exe
    let exe = std::fs::read_dir(dir)
        .ok()
        .and_then(|entries| {
            entries
                .filter_map(|e| e.ok())
                .find(|e| {
                    e.path().extension().map(|x| x == "exe").unwrap_or(false)
                })
                .map(|e| e.file_name().to_string_lossy().into_owned())
        });

    // 查找 *_Data 文件夹（Unity 特征）
    let has_data = std::fs::read_dir(dir)
        .ok()
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .any(|e| {
                    e.path().is_dir()
                        && e.file_name()
                            .to_string_lossy()
                            .ends_with("_Data")
                })
        })
        .unwrap_or(false);

    let has_exe = exe.is_some();
    let valid = has_exe || has_data;

    GameValidation {
        valid,
        has_data_folder: has_data,
        has_exe,
        exe_name: exe,
        message: if valid {
            "有效的 Unity 游戏目录".into()
        } else {
            "未找到 .exe 文件或 Unity _Data 文件夹，可能不是 Unity 游戏".into()
        },
    }
}
