use std::fs;
use std::path::Path;
use tauri::command;
use std::io;

/// Check if a project has Claude Code configuration
#[command]
pub fn check_project_config(project_path: String) -> Result<ProjectConfigStatus, String> {
    let path = Path::new(&project_path);

    if !path.exists() {
        return Err(format!("Project path does not exist: {}", project_path));
    }

    let claude_md = path.join("CLAUDE.md");
    let claude_dir = path.join(".claude");
    let settings_json = claude_dir.join("settings.json");
    let agents_dir = claude_dir.join("agents");

    Ok(ProjectConfigStatus {
        has_claude_md: claude_md.exists(),
        has_claude_dir: claude_dir.exists(),
        has_settings: settings_json.exists(),
        has_agents: agents_dir.exists() && agents_dir.is_dir(),
        is_fully_configured: claude_md.exists() && claude_dir.exists() && settings_json.exists(),
    })
}

/// Initialize a project with Claude Code configuration
#[command]
pub fn init_project_config(
    project_path: String,
    project_name: String,
    template_path: String,
) -> Result<InitResult, String> {
    let project = Path::new(&project_path);
    let template = Path::new(&template_path);

    if !project.exists() {
        return Err(format!("Project path does not exist: {}", project_path));
    }

    if !template.exists() {
        return Err(format!("Template path does not exist: {}", template_path));
    }

    let mut files_created: Vec<String> = Vec::new();
    let mut files_skipped: Vec<String> = Vec::new();

    // Create .claude directory
    let claude_dir = project.join(".claude");
    if !claude_dir.exists() {
        fs::create_dir_all(&claude_dir).map_err(|e| e.to_string())?;
    }

    // Create agents directory
    let agents_dir = claude_dir.join("agents");
    if !agents_dir.exists() {
        fs::create_dir_all(&agents_dir).map_err(|e| e.to_string())?;
    }

    // Copy and process CLAUDE.md
    let template_claude_md = template.join("CLAUDE.md");
    let target_claude_md = project.join("CLAUDE.md");
    if template_claude_md.exists() && !target_claude_md.exists() {
        let content = fs::read_to_string(&template_claude_md).map_err(|e| e.to_string())?;
        let processed = process_template(&content, &project_name, &project_path);
        fs::write(&target_claude_md, processed).map_err(|e| e.to_string())?;
        files_created.push("CLAUDE.md".to_string());
    } else if target_claude_md.exists() {
        files_skipped.push("CLAUDE.md (already exists)".to_string());
    }

    // Copy and process settings.json
    let template_settings = template.join(".claude").join("settings.json");
    let target_settings = claude_dir.join("settings.json");
    if template_settings.exists() && !target_settings.exists() {
        let content = fs::read_to_string(&template_settings).map_err(|e| e.to_string())?;
        let processed = process_template(&content, &project_name, &project_path);
        fs::write(&target_settings, processed).map_err(|e| e.to_string())?;
        files_created.push(".claude/settings.json".to_string());
    } else if target_settings.exists() {
        files_skipped.push(".claude/settings.json (already exists)".to_string());
    }

    // Copy agent files
    let template_agents = template.join(".claude").join("agents");
    if template_agents.exists() {
        for entry in fs::read_dir(&template_agents).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let file_name = entry.file_name();
            let target_file = agents_dir.join(&file_name);

            if !target_file.exists() {
                let content = fs::read_to_string(entry.path()).map_err(|e| e.to_string())?;
                fs::write(&target_file, content).map_err(|e| e.to_string())?;
                files_created.push(format!(".claude/agents/{}", file_name.to_string_lossy()));
            } else {
                files_skipped.push(format!(".claude/agents/{} (already exists)", file_name.to_string_lossy()));
            }
        }
    }

    Ok(InitResult {
        success: true,
        files_created,
        files_skipped,
    })
}

/// Process template content by replacing placeholders
fn process_template(content: &str, project_name: &str, project_path: &str) -> String {
    // Escape backslashes for JSON compatibility
    let escaped_path = project_path.replace("\\", "\\\\");

    content
        .replace("{{PROJECT_NAME}}", project_name)
        .replace("{{PROJECT_PATH}}", &escaped_path)
}

#[derive(serde::Serialize)]
pub struct ProjectConfigStatus {
    pub has_claude_md: bool,
    pub has_claude_dir: bool,
    pub has_settings: bool,
    pub has_agents: bool,
    pub is_fully_configured: bool,
}

#[derive(serde::Serialize)]
pub struct InitResult {
    pub success: bool,
    pub files_created: Vec<String>,
    pub files_skipped: Vec<String>,
}

/// Copy a project to the Leon projects directory
#[command]
pub fn copy_project(
    source_path: String,
    dest_path: String,
    move_instead_of_copy: bool,
) -> Result<CopyResult, String> {
    let source = Path::new(&source_path);
    let dest = Path::new(&dest_path);

    if !source.exists() {
        return Err(format!("Source path does not exist: {}", source_path));
    }

    if dest.exists() {
        return Err(format!("Destination already exists: {}", dest_path));
    }

    // Create destination parent if needed
    if let Some(parent) = dest.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
    }

    // Copy the directory
    let files_copied = copy_dir_recursive(source, dest)?;

    // If move, delete source after successful copy
    if move_instead_of_copy {
        fs::remove_dir_all(source).map_err(|e| format!("Copy succeeded but failed to remove source: {}", e))?;
    }

    Ok(CopyResult {
        success: true,
        files_copied,
        moved: move_instead_of_copy,
        new_path: dest_path,
    })
}

/// Recursively copy a directory
fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<usize, String> {
    let mut count = 0;

    fs::create_dir_all(dst).map_err(|e| e.to_string())?;

    for entry in fs::read_dir(src).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            count += copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path).map_err(|e| e.to_string())?;
            count += 1;
        }
    }

    Ok(count)
}

#[derive(serde::Serialize)]
pub struct CopyResult {
    pub success: bool,
    pub files_copied: usize,
    pub moved: bool,
    pub new_path: String,
}
