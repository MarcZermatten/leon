// Service pour l'exploration de fichiers
import { invoke } from '@tauri-apps/api/core';

export interface FileEntry {
	name: string;
	path: string;
	is_dir: boolean;
	is_hidden: boolean;
	extension: string | null;
	size: number | null;
	children: FileEntry[] | null;
}

export interface FileInfo {
	name: string;
	path: string;
	is_dir: boolean;
	size: number;
	extension: string | null;
	modified: number | null;
}

/**
 * Lister les fichiers d'un répertoire
 */
export async function listDirectory(
	dirPath: string,
	showHidden: boolean = false,
	depth: number = 2
): Promise<FileEntry[]> {
	try {
		return await invoke<FileEntry[]>('list_directory', { dirPath, showHidden, depth });
	} catch (e) {
		console.error('[Files] Error listing directory:', e);
		return [];
	}
}

/**
 * Lire le contenu d'un fichier
 */
export async function readFileContent(filePath: string): Promise<string | null> {
	try {
		return await invoke<string>('read_file_content', { filePath });
	} catch (e) {
		console.error('[Files] Error reading file:', e);
		return null;
	}
}

/**
 * Obtenir les informations d'un fichier
 */
export async function getFileInfo(filePath: string): Promise<FileInfo | null> {
	try {
		return await invoke<FileInfo>('get_file_info', { filePath });
	} catch (e) {
		console.error('[Files] Error getting file info:', e);
		return null;
	}
}

/**
 * Obtenir l'icône pour un type de fichier
 */
export function getFileIcon(entry: FileEntry): string {
	if (entry.is_dir) {
		return 'folder';
	}

	const ext = entry.extension?.toLowerCase();
	const iconMap: Record<string, string> = {
		// Code
		ts: 'typescript',
		tsx: 'react',
		js: 'javascript',
		jsx: 'react',
		svelte: 'svelte',
		vue: 'vue',
		rs: 'rust',
		py: 'python',
		go: 'go',
		java: 'java',
		cpp: 'cpp',
		c: 'c',
		h: 'header',
		cs: 'csharp',
		rb: 'ruby',
		php: 'php',
		swift: 'swift',
		kt: 'kotlin',
		dart: 'dart',
		// Data
		json: 'json',
		yaml: 'yaml',
		yml: 'yaml',
		xml: 'xml',
		toml: 'toml',
		sql: 'database',
		// Style
		css: 'css',
		scss: 'sass',
		sass: 'sass',
		less: 'less',
		// Web
		html: 'html',
		// Config
		md: 'markdown',
		txt: 'text',
		env: 'env',
		gitignore: 'git',
		// Images
		png: 'image',
		jpg: 'image',
		jpeg: 'image',
		gif: 'image',
		svg: 'svg',
		ico: 'image',
		// Other
		pdf: 'pdf',
		zip: 'archive',
		tar: 'archive',
		gz: 'archive'
	};

	return iconMap[ext || ''] || 'file';
}

/**
 * Formater la taille d'un fichier
 */
export function formatFileSize(bytes: number | null): string {
	if (bytes === null) return '';

	if (bytes < 1024) return `${bytes} B`;
	if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
	if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
	return `${(bytes / (1024 * 1024 * 1024)).toFixed(1)} GB`;
}

/**
 * Obtenir le langage pour l'éditeur
 */
export function getLanguageFromExtension(ext: string | null): string {
	if (!ext) return 'plaintext';

	const langMap: Record<string, string> = {
		ts: 'typescript',
		tsx: 'typescript',
		js: 'javascript',
		jsx: 'javascript',
		svelte: 'html',
		vue: 'html',
		html: 'html',
		css: 'css',
		scss: 'scss',
		json: 'json',
		md: 'markdown',
		py: 'python',
		rs: 'rust',
		go: 'go',
		sql: 'sql',
		yaml: 'yaml',
		yml: 'yaml',
		toml: 'toml',
		dart: 'dart',
		java: 'java',
		kt: 'kotlin',
		swift: 'swift',
		rb: 'ruby',
		php: 'php',
		c: 'c',
		cpp: 'cpp',
		h: 'c',
		cs: 'csharp'
	};

	return langMap[ext.toLowerCase()] || 'plaintext';
}
