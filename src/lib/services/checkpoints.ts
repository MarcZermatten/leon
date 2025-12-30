// Service de gestion des checkpoints pour undo/restore
import { invoke } from '@tauri-apps/api/core';

export interface Checkpoint {
	id: string;
	timestamp: number;
	description: string;
	file_count: number;
}

export interface UndoResult {
	description: string;
	restored_files: string[];
}

/**
 * Créer un checkpoint avant modification de fichiers
 */
export async function createCheckpoint(
	description: string,
	files: string[]
): Promise<string | null> {
	try {
		return await invoke<string | null>('create_checkpoint', { description, files });
	} catch (e) {
		console.error('[Checkpoints] Error creating checkpoint:', e);
		return null;
	}
}

/**
 * Annuler la dernière modification (undo)
 */
export async function undoLastChange(): Promise<UndoResult | null> {
	try {
		return await invoke<UndoResult>('undo_last_change');
	} catch (e) {
		console.error('[Checkpoints] Error undoing:', e);
		return null;
	}
}

/**
 * Restaurer un checkpoint spécifique
 */
export async function restoreCheckpoint(checkpointId: string): Promise<string[]> {
	try {
		return await invoke<string[]>('restore_checkpoint', { checkpointId });
	} catch (e) {
		console.error('[Checkpoints] Error restoring checkpoint:', e);
		return [];
	}
}

/**
 * Lister tous les checkpoints disponibles
 */
export async function listCheckpoints(): Promise<Checkpoint[]> {
	try {
		return await invoke<Checkpoint[]>('list_checkpoints');
	} catch (e) {
		console.error('[Checkpoints] Error listing checkpoints:', e);
		return [];
	}
}

/**
 * Obtenir le nombre de checkpoints disponibles
 */
export async function getCheckpointCount(): Promise<number> {
	try {
		return await invoke<number>('get_checkpoint_count');
	} catch (e) {
		console.error('[Checkpoints] Error getting count:', e);
		return 0;
	}
}

/**
 * Définir le projet actif (reset les checkpoints)
 */
export async function setCheckpointProject(projectPath: string): Promise<void> {
	try {
		await invoke('set_checkpoint_project', { projectPath });
	} catch (e) {
		console.error('[Checkpoints] Error setting project:', e);
	}
}

/**
 * Parser l'output terminal pour détecter les fichiers modifiés
 * Retourne les chemins de fichiers détectés
 */
export function detectModifiedFiles(terminalOutput: string): string[] {
	const files: Set<string> = new Set();

	// Patterns pour détecter les fichiers modifiés par Claude
	const patterns = [
		// Tool use patterns
		/(?:Read|Edit|Write|Created?|Modified?|Deleted?|Updated?)[:\s]+["']?([A-Za-z]:\\[^\s"'\n\r\]]+)/gi,
		/(?:Read|Edit|Write|Created?|Modified?|Deleted?|Updated?)[:\s]+["']?(\/[^\s"'\n\r]+)/gi,
		// File path mentions
		/(?:file|fichier|path)[:\s]+["']?([A-Za-z]:\\[^\s"'\n\r\]]+)/gi,
		/(?:file|fichier|path)[:\s]+["']?(\/[^\s"'\n\r]+)/gi,
		// Direct paths in tool output
		/"file_path":\s*"([^"]+)"/gi,
		// Svelte/TS/RS files explicitly mentioned
		/([A-Za-z]:\\[^\s"'\n\r]+\.(?:svelte|ts|tsx|js|jsx|rs|json|md|css|html))/gi,
		/(\/[^\s"'\n\r]+\.(?:svelte|ts|tsx|js|jsx|rs|json|md|css|html))/gi
	];

	for (const pattern of patterns) {
		const matches = terminalOutput.matchAll(pattern);
		for (const match of matches) {
			const filePath = match[1];
			// Nettoyer le chemin
			const cleanPath = filePath.replace(/["']/g, '').trim();
			if (cleanPath && cleanPath.length > 3) {
				files.add(cleanPath);
			}
		}
	}

	return Array.from(files);
}

/**
 * Déterminer si une action nécessite un checkpoint
 */
export function shouldCreateCheckpoint(terminalOutput: string): boolean {
	const destructivePatterns = [
		/\bEdit\b/i,
		/\bWrite\b/i,
		/\bDelete\b/i,
		/\bRemove\b/i,
		/\brm\s+-/i,
		/\bdel\s+/i,
		/git\s+reset/i,
		/git\s+checkout\s+--/i
	];

	return destructivePatterns.some((pattern) => pattern.test(terminalOutput));
}
