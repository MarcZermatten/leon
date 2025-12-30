use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::command;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DailyActivity {
    pub date: String,
    #[serde(rename = "messageCount")]
    pub message_count: u64,
    #[serde(rename = "sessionCount")]
    pub session_count: u64,
    #[serde(rename = "toolCallCount")]
    pub tool_call_count: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelUsage {
    #[serde(rename = "inputTokens")]
    pub input_tokens: u64,
    #[serde(rename = "outputTokens")]
    pub output_tokens: u64,
    #[serde(rename = "cacheReadInputTokens")]
    pub cache_read_input_tokens: u64,
    #[serde(rename = "cacheCreationInputTokens")]
    pub cache_creation_input_tokens: u64,
    #[serde(rename = "costUSD")]
    pub cost_usd: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StatsCache {
    pub version: u32,
    #[serde(rename = "lastComputedDate")]
    pub last_computed_date: String,
    #[serde(rename = "dailyActivity")]
    pub daily_activity: Vec<DailyActivity>,
    #[serde(rename = "modelUsage")]
    pub model_usage: std::collections::HashMap<String, ModelUsage>,
    #[serde(rename = "totalSessions")]
    pub total_sessions: u64,
    #[serde(rename = "totalMessages")]
    pub total_messages: u64,
}

#[derive(Debug, Serialize, Clone)]
pub struct ClaudeStats {
    // Stats globales
    pub total_sessions: u64,
    pub total_messages: u64,
    pub total_input_tokens: u64,
    pub total_output_tokens: u64,
    // Stats de la semaine
    pub weekly_messages: u64,
    pub weekly_sessions: u64,
    pub weekly_tool_calls: u64,
    // Stats du jour
    pub today_messages: u64,
    pub today_sessions: u64,
    pub today_tool_calls: u64,
}

fn get_claude_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".claude"))
}

#[command]
pub async fn get_claude_stats() -> Result<ClaudeStats, String> {
    let claude_dir = get_claude_dir().ok_or("Could not find home directory")?;
    let stats_file = claude_dir.join("stats-cache.json");

    if !stats_file.exists() {
        return Ok(ClaudeStats {
            total_sessions: 0,
            total_messages: 0,
            total_input_tokens: 0,
            total_output_tokens: 0,
            weekly_messages: 0,
            weekly_sessions: 0,
            weekly_tool_calls: 0,
            today_messages: 0,
            today_sessions: 0,
            today_tool_calls: 0,
        });
    }

    let content = fs::read_to_string(&stats_file)
        .map_err(|e| format!("Failed to read stats file: {}", e))?;

    let stats: StatsCache = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse stats: {}", e))?;

    // Calculer les totaux de tokens
    let (total_input, total_output) = stats.model_usage.values()
        .fold((0u64, 0u64), |(inp, out), usage| {
            (inp + usage.input_tokens + usage.cache_read_input_tokens + usage.cache_creation_input_tokens,
             out + usage.output_tokens)
        });

    // Obtenir la date d'aujourd'hui
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    // Calculer les stats du jour
    let today_stats = stats.daily_activity.iter()
        .find(|d| d.date == today)
        .map(|d| (d.message_count, d.session_count, d.tool_call_count))
        .unwrap_or((0, 0, 0));

    // Calculer les stats de la semaine (7 derniers jours)
    let week_ago = chrono::Local::now() - chrono::Duration::days(7);
    let week_ago_str = week_ago.format("%Y-%m-%d").to_string();

    let weekly_stats = stats.daily_activity.iter()
        .filter(|d| d.date >= week_ago_str)
        .fold((0u64, 0u64, 0u64), |(msgs, sess, tools), d| {
            (msgs + d.message_count, sess + d.session_count, tools + d.tool_call_count)
        });

    Ok(ClaudeStats {
        total_sessions: stats.total_sessions,
        total_messages: stats.total_messages,
        total_input_tokens: total_input,
        total_output_tokens: total_output,
        weekly_messages: weekly_stats.0,
        weekly_sessions: weekly_stats.1,
        weekly_tool_calls: weekly_stats.2,
        today_messages: today_stats.0,
        today_sessions: today_stats.1,
        today_tool_calls: today_stats.2,
    })
}

#[derive(Debug, Serialize, Clone)]
pub struct SessionStats {
    pub session_id: String,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_read_tokens: u64,
    pub cache_creation_tokens: u64,
    pub message_count: u64,
    // Estimation du contexte utilisé (approximatif)
    pub context_used_percent: f64,
}

#[derive(Debug, Deserialize)]
struct SessionMessage {
    #[serde(rename = "type")]
    msg_type: Option<String>,
    message: Option<MessageContent>,
    #[serde(rename = "sessionId")]
    session_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MessageContent {
    usage: Option<UsageInfo>,
}

#[derive(Debug, Deserialize)]
struct UsageInfo {
    input_tokens: Option<u64>,
    output_tokens: Option<u64>,
    cache_read_input_tokens: Option<u64>,
    cache_creation_input_tokens: Option<u64>,
}

#[command]
pub async fn get_session_stats(project_path: String) -> Result<SessionStats, String> {
    let claude_dir = get_claude_dir().ok_or("Could not find home directory")?;
    let projects_dir = claude_dir.join("projects");

    // Convertir le chemin du projet en nom de dossier Claude
    // Claude encode les chemins: C:\Users\Marc\projets\leon -> C--Users-Marc-projets-leon
    let project_folder = project_path
        .replace(":", "")
        .replace("\\", "-")
        .replace("/", "-");

    let project_sessions_dir = projects_dir.join(&project_folder);

    if !project_sessions_dir.exists() {
        return Ok(SessionStats {
            session_id: String::new(),
            input_tokens: 0,
            output_tokens: 0,
            cache_read_tokens: 0,
            cache_creation_tokens: 0,
            message_count: 0,
            context_used_percent: 0.0,
        });
    }

    // Trouver le fichier de session le plus récent
    let mut sessions: Vec<_> = fs::read_dir(&project_sessions_dir)
        .map_err(|e| format!("Failed to read sessions dir: {}", e))?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|ext| ext == "jsonl").unwrap_or(false))
        .filter(|e| !e.file_name().to_string_lossy().starts_with("agent-"))
        .collect();

    sessions.sort_by_key(|e| {
        e.metadata()
            .and_then(|m| m.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
    });

    let latest_session = sessions.last()
        .ok_or("No session files found")?;

    let session_path = latest_session.path();
    let session_id = session_path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();

    // Lire et parser le fichier JSONL
    let content = fs::read_to_string(&session_path)
        .map_err(|e| format!("Failed to read session file: {}", e))?;

    let mut total_input = 0u64;
    let mut total_output = 0u64;
    let mut total_cache_read = 0u64;
    let mut total_cache_creation = 0u64;
    let mut message_count = 0u64;

    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }

        if let Ok(msg) = serde_json::from_str::<SessionMessage>(line) {
            if msg.msg_type.as_deref() == Some("assistant") {
                message_count += 1;

                if let Some(message) = msg.message {
                    if let Some(usage) = message.usage {
                        total_input += usage.input_tokens.unwrap_or(0);
                        total_output += usage.output_tokens.unwrap_or(0);
                        total_cache_read += usage.cache_read_input_tokens.unwrap_or(0);
                        total_cache_creation += usage.cache_creation_input_tokens.unwrap_or(0);
                    }
                }
            }
        }
    }

    // Estimation du contexte utilisé
    // Claude Opus 4.5 a une fenêtre de 200K tokens
    // On estime le contexte actuel basé sur les tokens accumulés
    let context_window = 200_000u64;
    let estimated_context = total_input + total_output + total_cache_read;
    let context_percent = ((estimated_context as f64 / context_window as f64) * 100.0).min(100.0);

    Ok(SessionStats {
        session_id,
        input_tokens: total_input,
        output_tokens: total_output,
        cache_read_tokens: total_cache_read,
        cache_creation_tokens: total_cache_creation,
        message_count,
        context_used_percent: context_percent,
    })
}
