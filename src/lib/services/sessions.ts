// Service pour gérer les sessions Claude Code
import { invoke } from '@tauri-apps/api/core';

export interface SessionInfo {
	id: string;
	project_path: string;
	project_name: string;
	last_modified: number;
	message_count: number;
}

/**
 * Lister les sessions disponibles pour un projet
 */
export async function listProjectSessions(projectPath: string): Promise<SessionInfo[]> {
	try {
		return await invoke<SessionInfo[]>('list_project_sessions', { projectPath });
	} catch (e) {
		console.error('[Sessions] Error listing project sessions:', e);
		return [];
	}
}

/**
 * Obtenir les sessions récentes globales
 */
export async function getRecentSessions(): Promise<SessionInfo[]> {
	try {
		return await invoke<SessionInfo[]>('get_recent_sessions');
	} catch (e) {
		console.error('[Sessions] Error getting recent sessions:', e);
		return [];
	}
}

/**
 * Formater la date relative
 */
export function formatRelativeTime(timestamp: number): string {
	const now = Date.now() / 1000;
	const diff = now - timestamp;

	if (diff < 60) return "À l'instant";
	if (diff < 3600) return `Il y a ${Math.floor(diff / 60)} min`;
	if (diff < 86400) return `Il y a ${Math.floor(diff / 3600)} h`;
	if (diff < 604800) return `Il y a ${Math.floor(diff / 86400)} j`;

	const date = new Date(timestamp * 1000);
	return date.toLocaleDateString('fr-FR', { day: 'numeric', month: 'short' });
}
