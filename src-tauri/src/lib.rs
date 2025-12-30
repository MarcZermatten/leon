mod commands;

use commands::{
    capture_window, check_claude_available, create_checkpoint, get_checkpoint_count,
    get_claude_stats, get_claude_version, get_file_info, get_git_branches, get_git_commits,
    get_git_diff, get_git_status, get_recent_sessions, get_session_id, get_session_stats,
    git_checkout_branch, git_commit, git_discard_file, git_pull, git_push, git_stage_file,
    git_unstage_file, is_session_active, kill_pty, list_checkpoints, list_directory,
    list_project_sessions, list_windows, read_file_content, resize_pty, restore_checkpoint,
    send_claude_message, set_checkpoint_project, start_claude_session, start_pty,
    stop_claude_session, undo_last_change, write_pty, CheckpointManager, ClaudeRunner, PtyManager,
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
        .manage(Arc::new(Mutex::new(CheckpointManager::new())))
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
            get_session_stats,
            // Checkpoints
            create_checkpoint,
            undo_last_change,
            restore_checkpoint,
            list_checkpoints,
            get_checkpoint_count,
            set_checkpoint_project,
            // Sessions
            list_project_sessions,
            get_recent_sessions,
            // Git
            get_git_status,
            get_git_commits,
            get_git_branches,
            get_git_diff,
            git_stage_file,
            git_unstage_file,
            git_discard_file,
            git_commit,
            git_push,
            git_pull,
            git_checkout_branch,
            // Files
            list_directory,
            read_file_content,
            get_file_info
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
