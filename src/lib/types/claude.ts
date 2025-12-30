// Types pour les messages Claude Code CLI
// Correspond aux types Rust dans claude_types.rs

export type ClaudeEventType =
  | 'session_started'
  | 'assistant_text'
  | 'tool_start'
  | 'tool_end'
  | 'session_ended'
  | 'error';

export interface UsageInfo {
  input_tokens: number;
  output_tokens: number;
  cache_creation_input_tokens?: number;
  cache_read_input_tokens?: number;
}

export interface ClaudeEvent {
  event_type: ClaudeEventType;
  session_id?: string;
  content?: string;
  partial?: boolean;
  name?: string;
  input?: unknown;
  is_error?: boolean;
  usage?: UsageInfo;
  message?: string;
}

// Types pour l'affichage dans le chat
export type MessageRole = 'user' | 'assistant' | 'tool' | 'system';

export interface ChatMessage {
  id: string;
  role: MessageRole;
  content: string;
  timestamp: Date;
  isStreaming?: boolean;
  toolName?: string;
  toolInput?: unknown;
  isError?: boolean;
}

export interface SessionState {
  id: string | null;
  isRunning: boolean;
  totalInputTokens: number;
  totalOutputTokens: number;
  workingDirectory: string | null;
}
