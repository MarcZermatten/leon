// Service pour l'intégration Git visuelle
import { invoke } from '@tauri-apps/api/core';

export interface GitStatus {
	branch: string;
	ahead: number;
	behind: number;
	staged: FileChange[];
	unstaged: FileChange[];
	untracked: string[];
}

export interface FileChange {
	path: string;
	status: string; // M, A, D, R, C, U
}

export interface GitCommit {
	hash: string;
	short_hash: string;
	message: string;
	author: string;
	date: string;
	relative_date: string;
}

export interface GitBranch {
	name: string;
	is_current: boolean;
	is_remote: boolean;
	tracking: string | null;
}

export interface GitDiff {
	file_path: string;
	hunks: DiffHunk[];
}

export interface DiffHunk {
	header: string;
	lines: DiffLine[];
}

export interface DiffLine {
	content: string;
	line_type: 'add' | 'remove' | 'context';
	old_line: number | null;
	new_line: number | null;
}

/**
 * Obtenir le status Git du projet
 */
export async function getGitStatus(projectPath: string): Promise<GitStatus | null> {
	try {
		return await invoke<GitStatus>('get_git_status', { projectPath });
	} catch (e) {
		console.error('[Git] Error getting status:', e);
		return null;
	}
}

/**
 * Obtenir les commits récents
 */
export async function getGitCommits(projectPath: string, limit: number = 10): Promise<GitCommit[]> {
	try {
		return await invoke<GitCommit[]>('get_git_commits', { projectPath, limit });
	} catch (e) {
		console.error('[Git] Error getting commits:', e);
		return [];
	}
}

/**
 * Obtenir les branches
 */
export async function getGitBranches(projectPath: string): Promise<GitBranch[]> {
	try {
		return await invoke<GitBranch[]>('get_git_branches', { projectPath });
	} catch (e) {
		console.error('[Git] Error getting branches:', e);
		return [];
	}
}

/**
 * Obtenir le diff
 */
export async function getGitDiff(
	projectPath: string,
	filePath?: string,
	staged: boolean = false
): Promise<GitDiff[]> {
	try {
		return await invoke<GitDiff[]>('get_git_diff', { projectPath, filePath, staged });
	} catch (e) {
		console.error('[Git] Error getting diff:', e);
		return [];
	}
}

/**
 * Stage un fichier
 */
export async function gitStageFile(projectPath: string, filePath: string): Promise<boolean> {
	try {
		await invoke('git_stage_file', { projectPath, filePath });
		return true;
	} catch (e) {
		console.error('[Git] Error staging file:', e);
		return false;
	}
}

/**
 * Unstage un fichier
 */
export async function gitUnstageFile(projectPath: string, filePath: string): Promise<boolean> {
	try {
		await invoke('git_unstage_file', { projectPath, filePath });
		return true;
	} catch (e) {
		console.error('[Git] Error unstaging file:', e);
		return false;
	}
}

/**
 * Discard les changements d'un fichier
 */
export async function gitDiscardFile(projectPath: string, filePath: string): Promise<boolean> {
	try {
		await invoke('git_discard_file', { projectPath, filePath });
		return true;
	} catch (e) {
		console.error('[Git] Error discarding file:', e);
		return false;
	}
}

/**
 * Créer un commit
 */
export async function gitCommit(projectPath: string, message: string): Promise<boolean> {
	try {
		await invoke('git_commit', { projectPath, message });
		return true;
	} catch (e) {
		console.error('[Git] Error committing:', e);
		return false;
	}
}

/**
 * Push les changements
 */
export async function gitPush(projectPath: string): Promise<boolean> {
	try {
		await invoke('git_push', { projectPath });
		return true;
	} catch (e) {
		console.error('[Git] Error pushing:', e);
		return false;
	}
}

/**
 * Pull les changements
 */
export async function gitPull(projectPath: string): Promise<boolean> {
	try {
		await invoke('git_pull', { projectPath });
		return true;
	} catch (e) {
		console.error('[Git] Error pulling:', e);
		return false;
	}
}

/**
 * Changer de branche
 */
export async function gitCheckoutBranch(projectPath: string, branchName: string): Promise<boolean> {
	try {
		await invoke('git_checkout_branch', { projectPath, branchName });
		return true;
	} catch (e) {
		console.error('[Git] Error checking out branch:', e);
		return false;
	}
}

/**
 * Formater le status d'un fichier
 */
export function formatFileStatus(status: string): string {
	const statusMap: Record<string, string> = {
		M: 'Modified',
		A: 'Added',
		D: 'Deleted',
		R: 'Renamed',
		C: 'Copied',
		U: 'Unmerged'
	};
	return statusMap[status] || status;
}

/**
 * Obtenir la couleur du status
 */
export function getStatusColor(status: string): string {
	const colorMap: Record<string, string> = {
		M: 'var(--color-warning)',
		A: 'var(--color-success)',
		D: 'var(--color-error)',
		R: 'var(--color-info)',
		C: 'var(--color-info)',
		U: 'var(--color-error)'
	};
	return colorMap[status] || 'var(--color-text-secondary)';
}
