//! Module pour l'exécution de Claude Code CLI en mode INTERACTIF
//! Bidirectionnel : stdin stream-json pour envoyer, stdout stream-json pour recevoir

use crate::commands::claude_types::{ClaudeEvent, ClaudeMessage, ContentBlock};
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::{Arc, Mutex};
use tauri::{command, AppHandle, Emitter, Manager};

#[cfg(windows)]
use std::os::windows::process::CommandExt;

/// Flag Windows pour créer un processus sans fenêtre console
#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

/// État global du runner Claude - garde le process ET stdin pour envoyer des messages
pub struct ClaudeRunner {
    pub process: Option<Child>,
    pub stdin: Option<ChildStdin>,
    pub session_id: Option<String>,
}

impl Default for ClaudeRunner {
    fn default() -> Self {
        Self {
            process: None,
            stdin: None,
            session_id: None,
        }
    }
}

/// Démarre une session Claude interactive
#[command]
pub async fn start_claude_session(
    app: AppHandle,
    prompt: String,
    working_dir: Option<String>,
    continue_session: Option<String>,
) -> Result<String, String> {
    // Vérifier qu'il n'y a pas déjà une session active
    {
        let state = app.state::<Arc<Mutex<ClaudeRunner>>>();
        let runner = state.lock().map_err(|e| format!("Lock error: {}", e))?;
        if runner.process.is_some() {
            return Err("Une session Claude est déjà en cours".to_string());
        }
    }

    let claude_path = get_claude_path();
    log::info!("Starting interactive Claude session from: {}", claude_path);

    let mut cmd = Command::new(&claude_path);

    // Masquer la fenêtre console sur Windows
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);

    // Mode interactif bidirectionnel (PAS de -p !)
    cmd.arg("--input-format").arg("stream-json");
    cmd.arg("--output-format").arg("stream-json");
    cmd.arg("--verbose");
    cmd.arg("--model").arg("opus");

    // Continuer une session existante
    if let Some(ref session_id) = continue_session {
        cmd.arg("--resume").arg(session_id);
    }

    // Répertoire de travail
    if let Some(ref dir) = working_dir {
        cmd.current_dir(dir);
    }

    // Configurer les pipes bidirectionnels
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    // Démarrer le process
    let mut child = cmd.spawn().map_err(|e| format!("Erreur démarrage Claude: {}", e))?;

    // Récupérer stdin et stdout
    let stdin = child.stdin.take().ok_or("Impossible de capturer stdin")?;
    let stdout = child.stdout.take().ok_or("Impossible de capturer stdout")?;

    // Stocker le process et stdin dans l'état
    {
        let state = app.state::<Arc<Mutex<ClaudeRunner>>>();
        let mut runner = state.lock().map_err(|e| format!("Lock error: {}", e))?;
        runner.process = Some(child);
        runner.stdin = Some(stdin);
    }

    // Émettre l'event de démarrage
    let _ = app.emit("claude_event", ClaudeEvent::SessionStarted { session_id: None });

    // Spawner un thread pour lire le stream de sortie
    let app_handle = app.clone();
    std::thread::spawn(move || {
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            match line {
                Ok(json_line) if !json_line.trim().is_empty() => {
                    log::debug!("Claude NDJSON: {}", json_line);

                    match serde_json::from_str::<ClaudeMessage>(&json_line) {
                        Ok(msg) => {
                            process_claude_message(&app_handle, msg);
                        }
                        Err(e) => {
                            log::warn!("Erreur parsing NDJSON: {} - Line: {}", e, json_line);
                        }
                    }
                }
                Err(e) => {
                    log::error!("Erreur lecture stdout: {}", e);
                    break;
                }
                _ => {}
            }
        }

        // Session terminée
        log::info!("Claude session ended");
        let _ = app_handle.emit("claude_event", ClaudeEvent::SessionEnded { usage: None });

        // Nettoyer l'état
        if let Some(state) = app_handle.try_state::<Arc<Mutex<ClaudeRunner>>>() {
            if let Ok(mut runner) = state.lock() {
                runner.process = None;
                runner.stdin = None;
                runner.session_id = None;
            }
        }
    });

    // Envoyer le premier message
    send_message_internal(&app, &prompt)?;

    Ok("Session interactive démarrée".to_string())
}

/// Envoie un message à la session Claude active
#[command]
pub async fn send_claude_message(
    app: AppHandle,
    message: String,
) -> Result<(), String> {
    send_message_internal(&app, &message)
}

/// Fonction interne pour envoyer un message via stdin
fn send_message_internal(app: &AppHandle, message: &str) -> Result<(), String> {
    let state = app.state::<Arc<Mutex<ClaudeRunner>>>();
    let mut runner = state.lock().map_err(|e| format!("Lock error: {}", e))?;

    if let Some(ref mut stdin) = runner.stdin {
        // Format stream-json pour les messages utilisateur
        let json_message = serde_json::json!({
            "type": "user",
            "message": {
                "role": "user",
                "content": message
            }
        });

        let mut line = serde_json::to_string(&json_message)
            .map_err(|e| format!("Erreur sérialisation: {}", e))?;
        line.push('\n');

        stdin.write_all(line.as_bytes())
            .map_err(|e| format!("Erreur écriture stdin: {}", e))?;
        stdin.flush()
            .map_err(|e| format!("Erreur flush stdin: {}", e))?;

        log::info!("Message envoyé à Claude: {}", message);
        Ok(())
    } else {
        Err("Aucune session Claude active".to_string())
    }
}

/// Traite un message Claude et émet les events appropriés
fn process_claude_message(app: &AppHandle, msg: ClaudeMessage) {
    match msg {
        ClaudeMessage::System(sys) => {
            if let Some(ref sid) = sys.session_id {
                // Stocker le session_id
                if let Some(state) = app.try_state::<Arc<Mutex<ClaudeRunner>>>() {
                    if let Ok(mut runner) = state.lock() {
                        runner.session_id = Some(sid.clone());
                    }
                }
            }
            let _ = app.emit("claude_event", ClaudeEvent::SessionStarted {
                session_id: sys.session_id
            });
        }

        ClaudeMessage::Assistant(wrapper) => {
            let mut text_content = String::new();

            for block in &wrapper.message.content {
                match block {
                    ContentBlock::Text { text } => {
                        text_content.push_str(text);
                    }
                    ContentBlock::ToolUse { id: _, name, input } => {
                        let _ = app.emit("claude_event", ClaudeEvent::ToolStart {
                            name: name.clone(),
                            input: input.clone()
                        });
                    }
                }
            }

            if !text_content.is_empty() {
                let _ = app.emit("claude_event", ClaudeEvent::AssistantText {
                    content: text_content,
                    partial: wrapper.message.stop_reason.is_none()
                });
            }
        }

        ClaudeMessage::User(user_msg) => {
            // Les messages User contiennent les résultats des outils
            for block in &user_msg.message.content {
                let _ = app.emit("claude_event", ClaudeEvent::ToolEnd {
                    content: block.content.clone(),
                    is_error: false // TODO: détecter les erreurs
                });
            }
        }

        ClaudeMessage::Result(res) => {
            if let Some(ref sid) = res.session_id {
                if let Some(state) = app.try_state::<Arc<Mutex<ClaudeRunner>>>() {
                    if let Ok(mut runner) = state.lock() {
                        runner.session_id = Some(sid.clone());
                    }
                }
            }
            // Note: on n'émet pas SessionEnded ici car la session reste ouverte
            // SessionEnded est émis quand le process se termine
        }
    }
}

/// Arrête la session Claude en cours
#[command]
pub async fn stop_claude_session(app: AppHandle) -> Result<(), String> {
    let state = app.state::<Arc<Mutex<ClaudeRunner>>>();
    let mut runner = state.lock().map_err(|e| format!("Lock error: {}", e))?;

    // Fermer stdin proprement d'abord (drop le handle)
    runner.stdin = None;

    // Puis kill le process si présent
    if let Some(ref mut child) = runner.process {
        let _ = child.kill();
    }

    // Nettoyer l'état
    runner.process = None;
    runner.session_id = None;

    let _ = app.emit("claude_event", ClaudeEvent::SessionEnded { usage: None });
    log::info!("Claude session stopped by user");

    Ok(())
}

/// Retourne le chemin vers Claude CLI
fn get_claude_path() -> String {
    if cfg!(windows) {
        if let Ok(appdata) = std::env::var("APPDATA") {
            let npm_path = format!("{}\\npm\\claude.cmd", appdata);
            if std::path::Path::new(&npm_path).exists() {
                return npm_path;
            }
        }
    }
    "claude".to_string()
}

/// Vérifie si Claude CLI est disponible
#[command]
pub fn check_claude_available() -> Result<bool, String> {
    let claude_path = get_claude_path();
    log::info!("Checking Claude at: {}", claude_path);

    let mut cmd = Command::new(&claude_path);
    cmd.arg("--version");

    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);

    match cmd.output() {
        Ok(output) => {
            log::info!("Claude check result: {}", output.status.success());
            Ok(output.status.success())
        },
        Err(e) => {
            log::error!("Claude check error: {}", e);
            Ok(false)
        },
    }
}

/// Récupère la version de Claude CLI
#[command]
pub fn get_claude_version() -> Result<String, String> {
    let claude_path = get_claude_path();

    let mut cmd = Command::new(&claude_path);
    cmd.arg("--version");

    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);

    let output = cmd.output()
        .map_err(|e| format!("Claude CLI non trouvé: {}", e))?;

    String::from_utf8(output.stdout)
        .map(|s| s.trim().to_string())
        .map_err(|e| format!("Erreur décodage: {}", e))
}

/// Vérifie si une session est active
#[command]
pub fn is_session_active(app: AppHandle) -> Result<bool, String> {
    let state = app.state::<Arc<Mutex<ClaudeRunner>>>();
    let runner = state.lock().map_err(|e| format!("Lock error: {}", e))?;
    Ok(runner.process.is_some())
}

/// Récupère l'ID de session actuel
#[command]
pub fn get_session_id(app: AppHandle) -> Result<Option<String>, String> {
    let state = app.state::<Arc<Mutex<ClaudeRunner>>>();
    let runner = state.lock().map_err(|e| format!("Lock error: {}", e))?;
    Ok(runner.session_id.clone())
}
