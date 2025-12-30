// Service pour récupérer les statistiques Claude Code
import { invoke } from '@tauri-apps/api/core';

export interface ClaudeStats {
	total_sessions: number;
	total_messages: number;
	total_input_tokens: number;
	total_output_tokens: number;
	weekly_messages: number;
	weekly_sessions: number;
	weekly_tool_calls: number;
	today_messages: number;
	today_sessions: number;
	today_tool_calls: number;
}

export interface SessionStats {
	session_id: string;
	input_tokens: number;
	output_tokens: number;
	cache_read_tokens: number;
	cache_creation_tokens: number;
	message_count: number;
	context_used_percent: number;
}

export async function getClaudeStats(): Promise<ClaudeStats> {
	try {
		return await invoke<ClaudeStats>('get_claude_stats');
	} catch (e) {
		console.error('Error getting Claude stats:', e);
		return {
			total_sessions: 0,
			total_messages: 0,
			total_input_tokens: 0,
			total_output_tokens: 0,
			weekly_messages: 0,
			weekly_sessions: 0,
			weekly_tool_calls: 0,
			today_messages: 0,
			today_sessions: 0,
			today_tool_calls: 0
		};
	}
}

export async function getSessionStats(projectPath: string): Promise<SessionStats> {
	try {
		return await invoke<SessionStats>('get_session_stats', { projectPath });
	} catch (e) {
		console.error('Error getting session stats:', e);
		return {
			session_id: '',
			input_tokens: 0,
			output_tokens: 0,
			cache_read_tokens: 0,
			cache_creation_tokens: 0,
			message_count: 0,
			context_used_percent: 0
		};
	}
}

// Formater les nombres de tokens
export function formatTokens(n: number): string {
	if (n >= 1_000_000) return (n / 1_000_000).toFixed(1) + 'M';
	if (n >= 1_000) return (n / 1_000).toFixed(1) + 'K';
	return n.toString();
}
