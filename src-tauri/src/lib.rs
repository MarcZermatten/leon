mod commands;

use commands::{
    capture_window, check_claude_available, get_claude_version, get_session_id,
    is_session_active, list_windows, send_claude_message, start_claude_session,
    stop_claude_session, ClaudeRunner,
};
use std::sync::{Arc, Mutex};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .manage(Arc::new(Mutex::new(ClaudeRunner::default())))
        .invoke_handler(tauri::generate_handler![
            list_windows,
            capture_window,
            start_claude_session,
            stop_claude_session,
            send_claude_message,
            is_session_active,
            get_session_id,
            check_claude_available,
            get_claude_version
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
