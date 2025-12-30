//! Module PTY pour exécuter Claude Code dans un vrai terminal
//! Utilise ConPTY sur Windows pour support complet des couleurs et du curseur

use portable_pty::{native_pty_system, CommandBuilder, PtyPair, PtySize};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::{command, AppHandle, Emitter, Manager};
use uuid::Uuid;

/// Structure pour gérer un PTY actif
pub struct PtyInstance {
    pub pair: PtyPair,
    pub writer: Box<dyn Write + Send>,
}

/// État global des PTY
pub struct PtyManager {
    pub instances: HashMap<String, Arc<Mutex<PtyInstance>>>,
}

impl Default for PtyManager {
    fn default() -> Self {
        Self {
            instances: HashMap::new(),
        }
    }
}

/// Démarre un nouveau PTY avec Claude Code
#[command]
pub async fn start_pty(
    app: AppHandle,
    working_dir: Option<String>,
    cols: u16,
    rows: u16,
) -> Result<serde_json::Value, String> {
    let pty_system = native_pty_system();

    // Créer le PTY avec la taille spécifiée
    let pair = pty_system
        .openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|e| format!("Erreur création PTY: {}", e))?;

    // Construire la commande Claude
    let claude_path = get_claude_path();
    let mut cmd = CommandBuilder::new(&claude_path);

    // Répertoire de travail
    if let Some(ref dir) = working_dir {
        cmd.cwd(dir);
    }

    // Spawner le processus
    let mut child = pair
        .slave
        .spawn_command(cmd)
        .map_err(|e| format!("Erreur spawn Claude: {}", e))?;

    // Générer un ID unique
    let pty_id = Uuid::new_v4().to_string();

    // Récupérer le reader et writer
    let mut reader = pair
        .master
        .try_clone_reader()
        .map_err(|e| format!("Erreur clone reader: {}", e))?;

    let writer = pair
        .master
        .take_writer()
        .map_err(|e| format!("Erreur take writer: {}", e))?;

    // Stocker l'instance
    let instance = Arc::new(Mutex::new(PtyInstance { pair, writer }));
    {
        let state = app.state::<Arc<Mutex<PtyManager>>>();
        let mut manager = state.lock().map_err(|e| format!("Lock error: {}", e))?;
        manager.instances.insert(pty_id.clone(), instance.clone());
    }

    // Thread pour lire la sortie du PTY et l'envoyer au frontend
    let app_handle = app.clone();
    let pty_id_clone = pty_id.clone();
    thread::spawn(move || {
        let mut buf = [0u8; 8192];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    let data: Vec<u8> = buf[..n].to_vec();
                    let _ = app_handle.emit(
                        "pty_data",
                        serde_json::json!({
                            "pty_id": pty_id_clone,
                            "data": data
                        }),
                    );
                }
                Err(e) => {
                    log::error!("Erreur lecture PTY: {}", e);
                    break;
                }
            }
        }

        // Attendre la fin du processus
        let exit_code = child
            .wait()
            .map(|status| status.exit_code())
            .unwrap_or(1);

        let _ = app_handle.emit(
            "pty_exit",
            serde_json::json!({
                "pty_id": pty_id_clone,
                "code": exit_code
            }),
        );

        // Nettoyer l'instance
        if let Some(state) = app_handle.try_state::<Arc<Mutex<PtyManager>>>() {
            if let Ok(mut manager) = state.lock() {
                manager.instances.remove(&pty_id_clone);
            }
        }

        log::info!("PTY {} terminé avec code {}", pty_id_clone, exit_code);
    });

    log::info!("PTY {} démarré pour Claude Code", pty_id);

    Ok(serde_json::json!({
        "pty_id": pty_id
    }))
}

/// Écrit des données dans un PTY
#[command]
pub async fn write_pty(app: AppHandle, pty_id: String, data: String) -> Result<(), String> {
    let state = app.state::<Arc<Mutex<PtyManager>>>();
    let manager = state.lock().map_err(|e| format!("Lock error: {}", e))?;

    if let Some(instance) = manager.instances.get(&pty_id) {
        let mut inst = instance.lock().map_err(|e| format!("Lock error: {}", e))?;
        inst.writer
            .write_all(data.as_bytes())
            .map_err(|e| format!("Erreur écriture PTY: {}", e))?;
        inst.writer
            .flush()
            .map_err(|e| format!("Erreur flush PTY: {}", e))?;
        Ok(())
    } else {
        Err("PTY non trouvé".to_string())
    }
}

/// Redimensionne un PTY
#[command]
pub async fn resize_pty(
    app: AppHandle,
    pty_id: String,
    cols: u16,
    rows: u16,
) -> Result<(), String> {
    let state = app.state::<Arc<Mutex<PtyManager>>>();
    let manager = state.lock().map_err(|e| format!("Lock error: {}", e))?;

    if let Some(instance) = manager.instances.get(&pty_id) {
        let inst = instance.lock().map_err(|e| format!("Lock error: {}", e))?;
        inst.pair
            .master
            .resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| format!("Erreur resize PTY: {}", e))?;
        Ok(())
    } else {
        Err("PTY non trouvé".to_string())
    }
}

/// Termine un PTY
#[command]
pub async fn kill_pty(app: AppHandle, pty_id: String) -> Result<(), String> {
    let state = app.state::<Arc<Mutex<PtyManager>>>();
    let mut manager = state.lock().map_err(|e| format!("Lock error: {}", e))?;

    if manager.instances.remove(&pty_id).is_some() {
        log::info!("PTY {} tué", pty_id);
        Ok(())
    } else {
        Err("PTY non trouvé".to_string())
    }
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
