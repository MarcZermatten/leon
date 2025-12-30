//! Module pour l'exploration de fichiers
//! Fournit des commandes pour lister l'arborescence du projet

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub is_hidden: bool,
    pub extension: Option<String>,
    pub size: Option<u64>,
    pub children: Option<Vec<FileEntry>>,
}

/// Lister les fichiers d'un répertoire
#[command]
pub async fn list_directory(
    dir_path: String,
    show_hidden: bool,
    depth: i32,
) -> Result<Vec<FileEntry>, String> {
    let path = PathBuf::from(&dir_path);

    if !path.exists() {
        return Err(format!("Directory not found: {}", dir_path));
    }

    if !path.is_dir() {
        return Err(format!("Not a directory: {}", dir_path));
    }

    list_dir_recursive(&path, show_hidden, depth, 0)
}

fn list_dir_recursive(
    dir: &PathBuf,
    show_hidden: bool,
    max_depth: i32,
    current_depth: i32,
) -> Result<Vec<FileEntry>, String> {
    let mut entries = Vec::new();

    let read_dir = fs::read_dir(dir).map_err(|e| e.to_string())?;

    for entry in read_dir.filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        // Ignorer les fichiers cachés sauf si demandé
        let is_hidden = name.starts_with('.');
        if is_hidden && !show_hidden {
            continue;
        }

        // Ignorer certains dossiers
        if name == "node_modules"
            || name == ".git"
            || name == "target"
            || name == "__pycache__"
            || name == ".svelte-kit"
            || name == "dist"
            || name == "build"
        {
            continue;
        }

        let is_dir = path.is_dir();
        let extension = if is_dir {
            None
        } else {
            path.extension().map(|e| e.to_string_lossy().to_string())
        };

        let size = if is_dir {
            None
        } else {
            fs::metadata(&path).ok().map(|m| m.len())
        };

        let children = if is_dir && current_depth < max_depth {
            list_dir_recursive(&path, show_hidden, max_depth, current_depth + 1).ok()
        } else {
            None
        };

        entries.push(FileEntry {
            name,
            path: path.to_string_lossy().to_string(),
            is_dir,
            is_hidden,
            extension,
            size,
            children,
        });
    }

    // Trier: dossiers d'abord, puis par nom
    entries.sort_by(|a, b| {
        if a.is_dir == b.is_dir {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        } else if a.is_dir {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });

    Ok(entries)
}

/// Lire le contenu d'un fichier
#[command]
pub async fn read_file_content(file_path: String) -> Result<String, String> {
    let path = PathBuf::from(&file_path);

    if !path.exists() {
        return Err(format!("File not found: {}", file_path));
    }

    if !path.is_file() {
        return Err(format!("Not a file: {}", file_path));
    }

    // Vérifier la taille
    let metadata = fs::metadata(&path).map_err(|e| e.to_string())?;
    if metadata.len() > 1_000_000 {
        return Err("File too large (> 1MB)".to_string());
    }

    fs::read_to_string(&path).map_err(|e| format!("Failed to read file: {}", e))
}

/// Obtenir les informations d'un fichier
#[command]
pub async fn get_file_info(file_path: String) -> Result<FileInfo, String> {
    let path = PathBuf::from(&file_path);

    if !path.exists() {
        return Err(format!("File not found: {}", file_path));
    }

    let metadata = fs::metadata(&path).map_err(|e| e.to_string())?;
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    let extension = path.extension().map(|e| e.to_string_lossy().to_string());

    let modified = metadata
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs());

    Ok(FileInfo {
        name,
        path: file_path,
        is_dir: metadata.is_dir(),
        size: metadata.len(),
        extension,
        modified,
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
    pub extension: Option<String>,
    pub modified: Option<u64>,
}
