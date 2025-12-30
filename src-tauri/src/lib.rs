mod commands;

use commands::{
    capture_window, check_claude_available, get_claude_stats, get_claude_version,
    get_session_id, get_session_stats, is_session_active, kill_pty, list_windows, resize_pty,
    send_claude_message, start_claude_session, start_pty, stop_claude_session, write_pty,
    ClaudeRunner, PtyManager,
};
use std::sync::{Arc, Mutex};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(Arc::new(Mutex::new(ClaudeRunner::default())))
        .manage(Arc::new(Mutex::new(PtyManager::default())))
        .invoke_handler(tauri::generate_handler![
            // Window capture
            list_windows,
            capture_window,
            // Claude legacy (peut être supprimé plus tard)
            start_claude_session,
            stop_claude_session,
            send_claude_message,
            is_session_active,
            get_session_id,
            check_claude_available,
            get_claude_version,
            // PTY (nouveau - terminal natif)
            start_pty,
            write_pty,
            resize_pty,
            kill_pty,
            // Stats
            get_claude_stats,
            get_session_stats
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
