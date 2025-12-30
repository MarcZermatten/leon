//! Système de checkpoints pour undo/restore des fichiers
//! Sauvegarde automatique avant chaque modification destructive

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{command, AppHandle, Manager};

const MAX_CHECKPOINTS: usize = 50;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSnapshot {
    pub path: String,
    pub content: String,
    pub existed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub id: String,
    pub timestamp: u64,
    pub description: String,
    pub files: Vec<FileSnapshot>,
}

#[derive(Debug, Default)]
pub struct CheckpointManager {
    pub checkpoints: VecDeque<Checkpoint>,
    pub project_path: Option<String>,
}

impl CheckpointManager {
    pub fn new() -> Self {
        Self {
            checkpoints: VecDeque::with_capacity(MAX_CHECKPOINTS),
            project_path: None,
        }
    }

    pub fn set_project(&mut self, path: String) {
        self.project_path = Some(path);
        self.checkpoints.clear();
    }

    pub fn create_checkpoint(&mut self, description: String, files: Vec<String>) -> Option<String> {
        let mut snapshots = Vec::new();

        for file_path in files {
            let path = PathBuf::from(&file_path);
            let snapshot = if path.exists() {
                match fs::read_to_string(&path) {
                    Ok(content) => FileSnapshot {
                        path: file_path,
                        content,
                        existed: true,
                    },
                    Err(_) => continue,
                }
            } else {
                FileSnapshot {
                    path: file_path,
                    content: String::new(),
                    existed: false,
                }
            };
            snapshots.push(snapshot);
        }

        if snapshots.is_empty() {
            return None;
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let id = format!("cp_{}", timestamp);

        let checkpoint = Checkpoint {
            id: id.clone(),
            timestamp,
            description,
            files: snapshots,
        };

        // Garder seulement MAX_CHECKPOINTS
        if self.checkpoints.len() >= MAX_CHECKPOINTS {
            self.checkpoints.pop_front();
        }

        self.checkpoints.push_back(checkpoint);
        Some(id)
    }

    pub fn restore_checkpoint(&mut self, checkpoint_id: &str) -> Result<Vec<String>, String> {
        let checkpoint = self
            .checkpoints
            .iter()
            .find(|cp| cp.id == checkpoint_id)
            .cloned()
            .ok_or_else(|| "Checkpoint non trouvé".to_string())?;

        let mut restored_files = Vec::new();

        for snapshot in &checkpoint.files {
            let path = PathBuf::from(&snapshot.path);

            if snapshot.existed {
                // Restaurer le contenu
                if let Some(parent) = path.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                fs::write(&path, &snapshot.content)
                    .map_err(|e| format!("Erreur restauration {}: {}", snapshot.path, e))?;
            } else {
                // Le fichier n'existait pas, le supprimer s'il existe maintenant
                if path.exists() {
                    let _ = fs::remove_file(&path);
                }
            }
            restored_files.push(snapshot.path.clone());
        }

        Ok(restored_files)
    }

    pub fn undo_last(&mut self) -> Result<(String, Vec<String>), String> {
        let checkpoint = self
            .checkpoints
            .pop_back()
            .ok_or_else(|| "Aucun checkpoint disponible".to_string())?;

        let mut restored_files = Vec::new();

        for snapshot in &checkpoint.files {
            let path = PathBuf::from(&snapshot.path);

            if snapshot.existed {
                if let Some(parent) = path.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                fs::write(&path, &snapshot.content)
                    .map_err(|e| format!("Erreur restauration {}: {}", snapshot.path, e))?;
            } else {
                if path.exists() {
                    let _ = fs::remove_file(&path);
                }
            }
            restored_files.push(snapshot.path.clone());
        }

        Ok((checkpoint.description, restored_files))
    }
}

/// Créer un checkpoint avant modification
#[command]
pub async fn create_checkpoint(
    app: AppHandle,
    description: String,
    files: Vec<String>,
) -> Result<Option<String>, String> {
    let state = app.state::<Arc<Mutex<CheckpointManager>>>();
    let mut manager = state.lock().map_err(|e| e.to_string())?;
    Ok(manager.create_checkpoint(description, files))
}

/// Annuler la dernière modification
#[command]
pub async fn undo_last_change(app: AppHandle) -> Result<serde_json::Value, String> {
    let state = app.state::<Arc<Mutex<CheckpointManager>>>();
    let mut manager = state.lock().map_err(|e| e.to_string())?;

    let (description, files) = manager.undo_last()?;

    Ok(serde_json::json!({
        "description": description,
        "restored_files": files
    }))
}

/// Restaurer un checkpoint spécifique
#[command]
pub async fn restore_checkpoint(
    app: AppHandle,
    checkpoint_id: String,
) -> Result<Vec<String>, String> {
    let state = app.state::<Arc<Mutex<CheckpointManager>>>();
    let mut manager = state.lock().map_err(|e| e.to_string())?;
    manager.restore_checkpoint(&checkpoint_id)
}

/// Lister tous les checkpoints
#[command]
pub async fn list_checkpoints(app: AppHandle) -> Result<Vec<serde_json::Value>, String> {
    let state = app.state::<Arc<Mutex<CheckpointManager>>>();
    let manager = state.lock().map_err(|e| e.to_string())?;

    let checkpoints: Vec<serde_json::Value> = manager
        .checkpoints
        .iter()
        .rev()
        .map(|cp| {
            serde_json::json!({
                "id": cp.id,
                "timestamp": cp.timestamp,
                "description": cp.description,
                "file_count": cp.files.len()
            })
        })
        .collect();

    Ok(checkpoints)
}

/// Obtenir le nombre de checkpoints disponibles
#[command]
pub async fn get_checkpoint_count(app: AppHandle) -> Result<usize, String> {
    let state = app.state::<Arc<Mutex<CheckpointManager>>>();
    let manager = state.lock().map_err(|e| e.to_string())?;
    Ok(manager.checkpoints.len())
}

/// Définir le projet actif (clear les checkpoints)
#[command]
pub async fn set_checkpoint_project(app: AppHandle, project_path: String) -> Result<(), String> {
    let state = app.state::<Arc<Mutex<CheckpointManager>>>();
    let mut manager = state.lock().map_err(|e| e.to_string())?;
    manager.set_project(project_path);
    Ok(())
}
