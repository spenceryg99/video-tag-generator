mod commands;
mod dashscope;

use tokio::sync::Mutex;

pub struct AppState {
    pub api_key: Mutex<Option<String>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            api_key: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            commands::video::analyze_video,
            commands::video::set_api_key,
            commands::video::get_api_key_masked,
            commands::video::validate_api_key,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
