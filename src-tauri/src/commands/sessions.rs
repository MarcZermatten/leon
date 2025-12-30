//! Module pour lister et gérer les sessions Claude Code
//! Les sessions sont stockées dans ~/.claude/projects/

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub id: String,
    pub project_path: String,
    pub project_name: String,
    pub last_modified: u64,
    pub message_count: i32,
}

/// Obtenir le chemin du dossier Claude
fn get_claude_dir() -> Option<PathBuf> {
    if cfg!(windows) {
        std::env::var("USERPROFILE")
            .ok()
            .map(|home| PathBuf::from(home).join(".claude"))
    } else {
        std::env::var("HOME")
            .ok()
            .map(|home| PathBuf::from(home).join(".claude"))
    }
}

/// Lister les sessions disponibles pour un projet
#[command]
pub async fn list_project_sessions(project_path: String) -> Result<Vec<SessionInfo>, String> {
    let claude_dir = get_claude_dir().ok_or("Impossible de trouver le dossier Claude")?;

    // Le chemin vers les sessions du projet est basé sur un hash du chemin
    let projects_dir = claude_dir.join("projects");

    if !projects_dir.exists() {
        return Ok(vec![]);
    }

    let mut sessions = Vec::new();

    // Parcourir les sous-dossiers de projects/
    if let Ok(entries) = fs::read_dir(&projects_dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_dir() {
                // Chercher les fichiers .jsonl (sessions)
                if let Ok(files) = fs::read_dir(&path) {
                    for file in files.filter_map(|f| f.ok()) {
                        let file_path = file.path();
                        if file_path.extension().map_or(false, |e| e == "jsonl") {
                            if let Some(session) = parse_session_file(&file_path, &project_path) {
                                sessions.push(session);
                            }
                        }
                    }
                }
            }
        }
    }

    // Trier par date de modification décroissante
    sessions.sort_by(|a, b| b.last_modified.cmp(&a.last_modified));

    // Limiter à 10 sessions
    sessions.truncate(10);

    Ok(sessions)
}

/// Parser un fichier de session
fn parse_session_file(path: &PathBuf, project_path: &str) -> Option<SessionInfo> {
    let file_name = path.file_stem()?.to_string_lossy().to_string();

    // Obtenir les métadonnées du fichier
    let metadata = fs::metadata(path).ok()?;
    let modified = metadata
        .modified()
        .ok()?
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_secs();

    // Compter les lignes (approximation du nombre de messages)
    let content = fs::read_to_string(path).ok()?;
    let message_count = content.lines().count() as i32;

    // Extraire le nom du projet depuis le chemin
    let project_name = PathBuf::from(project_path)
        .file_name()?
        .to_string_lossy()
        .to_string();

    Some(SessionInfo {
        id: file_name,
        project_path: project_path.to_string(),
        project_name,
        last_modified: modified,
        message_count,
    })
}

/// Obtenir les sessions récentes globales
#[command]
pub async fn get_recent_sessions() -> Result<Vec<SessionInfo>, String> {
    let claude_dir = get_claude_dir().ok_or("Impossible de trouver le dossier Claude")?;
    let projects_dir = claude_dir.join("projects");

    if !projects_dir.exists() {
        return Ok(vec![]);
    }

    let mut all_sessions = Vec::new();

    // Parcourir tous les projets
    if let Ok(project_entries) = fs::read_dir(&projects_dir) {
        for project_entry in project_entries.filter_map(|e| e.ok()) {
            let project_path = project_entry.path();
            if project_path.is_dir() {
                // Chercher les fichiers .jsonl
                if let Ok(files) = fs::read_dir(&project_path) {
                    for file in files.filter_map(|f| f.ok()) {
                        let file_path = file.path();
                        if file_path.extension().map_or(false, |e| e == "jsonl") {
                            // Essayer de lire le projet path depuis le premier message
                            let project_name = project_path
                                .file_name()
                                .map(|n| n.to_string_lossy().to_string())
                                .unwrap_or_default();

                            if let Some(mut session) =
                                parse_session_file(&file_path, &project_name)
                            {
                                // Mettre à jour le project_path avec le vrai chemin si possible
                                session.project_path = project_path.to_string_lossy().to_string();
                                all_sessions.push(session);
                            }
                        }
                    }
                }
            }
        }
    }

    // Trier par date décroissante
    all_sessions.sort_by(|a, b| b.last_modified.cmp(&a.last_modified));

    // Limiter à 20 sessions
    all_sessions.truncate(20);

    Ok(all_sessions)
}
