//! Module pour l'intégration Git visuelle
//! Fournit des commandes pour status, diff, branches, commits

use serde::{Deserialize, Serialize};
use std::process::Command;
use tauri::command;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

/// Flag Windows pour créer un processus sans fenêtre console
#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitStatus {
    pub branch: String,
    pub ahead: i32,
    pub behind: i32,
    pub staged: Vec<FileChange>,
    pub unstaged: Vec<FileChange>,
    pub untracked: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    pub path: String,
    pub status: String, // M, A, D, R, C, U
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitCommit {
    pub hash: String,
    pub short_hash: String,
    pub message: String,
    pub author: String,
    pub date: String,
    pub relative_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitBranch {
    pub name: String,
    pub is_current: bool,
    pub is_remote: bool,
    pub tracking: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitDiff {
    pub file_path: String,
    pub hunks: Vec<DiffHunk>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffHunk {
    pub header: String,
    pub lines: Vec<DiffLine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffLine {
    pub content: String,
    pub line_type: String, // "add", "remove", "context"
    pub old_line: Option<i32>,
    pub new_line: Option<i32>,
}

/// Exécuter une commande Git
fn run_git_command(project_path: &str, args: &[&str]) -> Result<String, String> {
    let mut cmd = Command::new("git");
    cmd.args(args).current_dir(project_path);

    // Masquer la fenêtre console sur Windows
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);

    let output = cmd.output()
        .map_err(|e| format!("Failed to execute git: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Obtenir le status Git du projet
#[command]
pub async fn get_git_status(project_path: String) -> Result<GitStatus, String> {
    // Obtenir la branche courante
    let branch = run_git_command(&project_path, &["branch", "--show-current"])?
        .trim()
        .to_string();

    // Obtenir ahead/behind
    let (ahead, behind) = get_ahead_behind(&project_path).unwrap_or((0, 0));

    // Obtenir le status porcelain
    let status_output = run_git_command(&project_path, &["status", "--porcelain=v1"])?;

    let mut staged = Vec::new();
    let mut unstaged = Vec::new();
    let mut untracked = Vec::new();

    for line in status_output.lines() {
        if line.len() < 3 {
            continue;
        }

        let index_status = line.chars().nth(0).unwrap_or(' ');
        let worktree_status = line.chars().nth(1).unwrap_or(' ');
        let file_path = line[3..].to_string();

        // Fichiers non trackés
        if index_status == '?' {
            untracked.push(file_path);
            continue;
        }

        // Fichiers staged
        if index_status != ' ' && index_status != '?' {
            staged.push(FileChange {
                path: file_path.clone(),
                status: index_status.to_string(),
            });
        }

        // Fichiers unstaged
        if worktree_status != ' ' && worktree_status != '?' {
            unstaged.push(FileChange {
                path: file_path,
                status: worktree_status.to_string(),
            });
        }
    }

    Ok(GitStatus {
        branch,
        ahead,
        behind,
        staged,
        unstaged,
        untracked,
    })
}

fn get_ahead_behind(project_path: &str) -> Option<(i32, i32)> {
    let output = run_git_command(
        project_path,
        &["rev-list", "--left-right", "--count", "HEAD...@{upstream}"],
    )
    .ok()?;

    let parts: Vec<&str> = output.trim().split('\t').collect();
    if parts.len() == 2 {
        let ahead = parts[0].parse().unwrap_or(0);
        let behind = parts[1].parse().unwrap_or(0);
        Some((ahead, behind))
    } else {
        None
    }
}

/// Obtenir les commits récents
#[command]
pub async fn get_git_commits(project_path: String, limit: i32) -> Result<Vec<GitCommit>, String> {
    let format = "%H|%h|%s|%an|%ai|%ar";
    let output = run_git_command(
        &project_path,
        &[
            "log",
            &format!("--pretty=format:{}", format),
            &format!("-{}", limit),
        ],
    )?;

    let commits: Vec<GitCommit> = output
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 6 {
                Some(GitCommit {
                    hash: parts[0].to_string(),
                    short_hash: parts[1].to_string(),
                    message: parts[2].to_string(),
                    author: parts[3].to_string(),
                    date: parts[4].to_string(),
                    relative_date: parts[5].to_string(),
                })
            } else {
                None
            }
        })
        .collect();

    Ok(commits)
}

/// Obtenir les branches
#[command]
pub async fn get_git_branches(project_path: String) -> Result<Vec<GitBranch>, String> {
    let output = run_git_command(&project_path, &["branch", "-a", "-v"])?;

    let branches: Vec<GitBranch> = output
        .lines()
        .filter_map(|line| {
            let is_current = line.starts_with('*');
            let line = line.trim_start_matches('*').trim();

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                return None;
            }

            let name = parts[0].to_string();
            let is_remote = name.starts_with("remotes/");

            Some(GitBranch {
                name: name.trim_start_matches("remotes/").to_string(),
                is_current,
                is_remote,
                tracking: None,
            })
        })
        .collect();

    Ok(branches)
}

/// Obtenir le diff d'un fichier
#[command]
pub async fn get_git_diff(
    project_path: String,
    file_path: Option<String>,
    staged: bool,
) -> Result<Vec<GitDiff>, String> {
    let mut args = vec!["diff"];

    if staged {
        args.push("--cached");
    }

    if let Some(ref fp) = file_path {
        args.push("--");
        args.push(fp);
    }

    let output = run_git_command(&project_path, &args)?;

    let mut diffs = Vec::new();
    let mut current_file: Option<String> = None;
    let mut current_hunks: Vec<DiffHunk> = Vec::new();
    let mut current_hunk: Option<DiffHunk> = None;
    let mut old_line = 0;
    let mut new_line = 0;

    for line in output.lines() {
        if line.starts_with("diff --git") {
            // Sauvegarder le diff précédent
            if let Some(hunk) = current_hunk.take() {
                current_hunks.push(hunk);
            }
            if let Some(file) = current_file.take() {
                diffs.push(GitDiff {
                    file_path: file,
                    hunks: std::mem::take(&mut current_hunks),
                });
            }

            // Extraire le nom du fichier
            let parts: Vec<&str> = line.split(' ').collect();
            if parts.len() >= 4 {
                current_file = Some(parts[3].trim_start_matches("b/").to_string());
            }
        } else if line.starts_with("@@") {
            // Nouveau hunk
            if let Some(hunk) = current_hunk.take() {
                current_hunks.push(hunk);
            }

            // Parser les numéros de ligne
            if let Some(header_end) = line[2..].find("@@") {
                let header_content = &line[2..2 + header_end].trim();
                let parts: Vec<&str> = header_content.split(' ').collect();

                if parts.len() >= 2 {
                    // Format: -old_start,old_count +new_start,new_count
                    if let Some(old_start) = parts[0].trim_start_matches('-').split(',').next() {
                        old_line = old_start.parse().unwrap_or(1);
                    }
                    if let Some(new_start) = parts[1].trim_start_matches('+').split(',').next() {
                        new_line = new_start.parse().unwrap_or(1);
                    }
                }
            }

            current_hunk = Some(DiffHunk {
                header: line.to_string(),
                lines: Vec::new(),
            });
        } else if let Some(ref mut hunk) = current_hunk {
            let (line_type, old_ln, new_ln) = if line.starts_with('+') {
                let ln = new_line;
                new_line += 1;
                ("add", None, Some(ln))
            } else if line.starts_with('-') {
                let ln = old_line;
                old_line += 1;
                ("remove", Some(ln), None)
            } else {
                let oln = old_line;
                let nln = new_line;
                old_line += 1;
                new_line += 1;
                ("context", Some(oln), Some(nln))
            };

            hunk.lines.push(DiffLine {
                content: line.to_string(),
                line_type: line_type.to_string(),
                old_line: old_ln,
                new_line: new_ln,
            });
        }
    }

    // Sauvegarder le dernier diff
    if let Some(hunk) = current_hunk {
        current_hunks.push(hunk);
    }
    if let Some(file) = current_file {
        diffs.push(GitDiff {
            file_path: file,
            hunks: current_hunks,
        });
    }

    Ok(diffs)
}

/// Stage un fichier
#[command]
pub async fn git_stage_file(project_path: String, file_path: String) -> Result<(), String> {
    run_git_command(&project_path, &["add", &file_path])?;
    Ok(())
}

/// Unstage un fichier
#[command]
pub async fn git_unstage_file(project_path: String, file_path: String) -> Result<(), String> {
    run_git_command(&project_path, &["reset", "HEAD", &file_path])?;
    Ok(())
}

/// Discard les changements d'un fichier
#[command]
pub async fn git_discard_file(project_path: String, file_path: String) -> Result<(), String> {
    run_git_command(&project_path, &["checkout", "--", &file_path])?;
    Ok(())
}

/// Créer un commit
#[command]
pub async fn git_commit(project_path: String, message: String) -> Result<String, String> {
    let output = run_git_command(&project_path, &["commit", "-m", &message])?;
    Ok(output)
}

/// Push les changements
#[command]
pub async fn git_push(project_path: String) -> Result<String, String> {
    let output = run_git_command(&project_path, &["push"])?;
    Ok(output)
}

/// Pull les changements
#[command]
pub async fn git_pull(project_path: String) -> Result<String, String> {
    let output = run_git_command(&project_path, &["pull"])?;
    Ok(output)
}

/// Changer de branche
#[command]
pub async fn git_checkout_branch(project_path: String, branch_name: String) -> Result<(), String> {
    run_git_command(&project_path, &["checkout", &branch_name])?;
    Ok(())
}
