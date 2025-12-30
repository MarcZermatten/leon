//! Types pour les messages Claude Code CLI
//! Basé sur le format NDJSON réel de --output-format stream-json --verbose

use serde::{Deserialize, Serialize};

/// Message entrant du stream Claude (format réel)
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClaudeMessage {
    /// Message système (init, etc.)
    System(SystemMessage),
    /// Message de l'assistant
    Assistant(AssistantMessageWrapper),
    /// Message utilisateur (tool results)
    User(UserMessage),
    /// Résultat final
    Result(ResultMessage),
}

#[derive(Debug, Clone, Deserialize)]
pub struct SystemMessage {
    pub subtype: Option<String>,
    pub session_id: Option<String>,
    pub cwd: Option<String>,
    pub model: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AssistantMessageWrapper {
    pub message: AssistantMessageInner,
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AssistantMessageInner {
    pub content: Vec<ContentBlock>,
    pub stop_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    Text { text: String },
    ToolUse { id: String, name: String, input: serde_json::Value },
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserMessage {
    pub message: UserMessageInner,
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserMessageInner {
    pub content: Vec<ToolResultBlock>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ToolResultBlock {
    pub tool_use_id: String,
    #[serde(rename = "type")]
    pub block_type: String,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResultMessage {
    pub subtype: Option<String>,
    pub session_id: Option<String>,
    pub cost_usd: Option<f64>,
    pub duration_ms: Option<u64>,
    pub duration_api_ms: Option<u64>,
    pub total_cost_usd: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageInfo {
    pub input_tokens: u32,
    pub output_tokens: u32,
    #[serde(default)]
    pub cache_creation_input_tokens: Option<u32>,
    #[serde(default)]
    pub cache_read_input_tokens: Option<u32>,
}

/// État de la session Claude
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClaudeSession {
    pub id: Option<String>,
    pub is_running: bool,
    pub total_input_tokens: u32,
    pub total_output_tokens: u32,
}

/// Event envoyé au frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum ClaudeEvent {
    /// Session démarrée
    SessionStarted { session_id: Option<String> },
    /// Texte de l'assistant (streaming)
    AssistantText { content: String, partial: bool },
    /// Début d'utilisation d'un outil
    ToolStart { name: String, input: serde_json::Value },
    /// Résultat d'un outil
    ToolEnd { content: String, is_error: bool },
    /// Session terminée
    SessionEnded { usage: Option<UsageInfo> },
    /// Erreur
    Error { message: String },
}
