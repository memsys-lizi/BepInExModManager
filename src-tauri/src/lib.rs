mod commands;

use commands::{bepinex, mods, config, game};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![
            // game
            game::validate_game_dir,
            game::extract_exe_icon,
            // bepinex
            bepinex::check_bepinex_status,
            bepinex::fetch_bepinex_releases,
            bepinex::install_bepinex,
            bepinex::uninstall_bepinex,
            // mods
            mods::scan_mods,
            mods::enable_mod,
            mods::disable_mod,
            mods::delete_mod,
            mods::open_plugins_dir,
            mods::install_mod,
            mods::install_mod_from_url,
            mods::launch_game,
            // config
            config::list_config_files,
            config::read_config_file,
            config::write_config_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
