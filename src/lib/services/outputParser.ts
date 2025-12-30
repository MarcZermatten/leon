// Service pour parser la sortie du terminal et détecter les patterns importants
// Permet de rendre les erreurs cliquables et d'extraire des informations

export interface ParsedOutput {
	type: 'error' | 'warning' | 'info' | 'file' | 'url' | 'text';
	content: string;
	file?: string;
	line?: number;
	column?: number;
	url?: string;
}

export interface FileReference {
	path: string;
	line?: number;
	column?: number;
	match: string;
}

// Patterns pour détecter les fichiers et erreurs
const FILE_PATTERNS = [
	// TypeScript/JavaScript errors: src/file.ts(10,5): error
	/([a-zA-Z]:[\\\/][^\s:]+|[.\/][^\s:]+)\((\d+),(\d+)\):/g,
	// Rust errors: --> src/main.rs:10:5
	/-->\s*([^\s:]+):(\d+):(\d+)/g,
	// Python errors: File "script.py", line 10
	/File "([^"]+)", line (\d+)/g,
	// Generic: path/to/file.ext:10:5
	/([a-zA-Z]:[\\\/][^\s:]+|[.\/][^\s:]+):(\d+):(\d+)/g,
	// Simple: path/to/file.ext:10
	/([a-zA-Z]:[\\\/][^\s:]+|[.\/][^\s:]+):(\d+)(?!\d|:)/g,
	// Jest/Vitest: at file.ts:10:5
	/at\s+([^\s:]+):(\d+):(\d+)/g,
	// Svelte: src/Component.svelte:10
	/([^\s]+\.svelte):(\d+)/g
];

const URL_PATTERN = /https?:\/\/[^\s<>"\]]+/g;

const ERROR_PATTERNS = [
	/error\[E\d+\]:/i,
	/Error:/i,
	/error TS\d+:/i,
	/FAIL/i,
	/failed/i,
	/exception/i,
	/panic!/i,
	/✕|×|❌/
];

const WARNING_PATTERNS = [/warning:/i, /warn:/i, /WARNING/i, /⚠️|⚠/];

const SUCCESS_PATTERNS = [/PASS/i, /success/i, /✓|✔️|✅/, /passed/i];

/**
 * Extraire les références de fichiers d'un texte
 */
export function extractFileReferences(text: string): FileReference[] {
	const references: FileReference[] = [];
	const seen = new Set<string>();

	for (const pattern of FILE_PATTERNS) {
		// Reset lastIndex for global patterns
		pattern.lastIndex = 0;
		let match;

		while ((match = pattern.exec(text)) !== null) {
			const path = match[1];
			const line = match[2] ? parseInt(match[2], 10) : undefined;
			const column = match[3] ? parseInt(match[3], 10) : undefined;

			// Éviter les doublons
			const key = `${path}:${line || 0}:${column || 0}`;
			if (!seen.has(key)) {
				seen.add(key);
				references.push({
					path,
					line,
					column,
					match: match[0]
				});
			}
		}
	}

	return references;
}

/**
 * Extraire les URLs d'un texte
 */
export function extractUrls(text: string): string[] {
	const matches = text.match(URL_PATTERN);
	return matches ? [...new Set(matches)] : [];
}

/**
 * Détecter le type de message
 */
export function detectMessageType(text: string): 'error' | 'warning' | 'success' | 'info' {
	if (ERROR_PATTERNS.some((p) => p.test(text))) {
		return 'error';
	}
	if (WARNING_PATTERNS.some((p) => p.test(text))) {
		return 'warning';
	}
	if (SUCCESS_PATTERNS.some((p) => p.test(text))) {
		return 'success';
	}
	return 'info';
}

/**
 * Parser une ligne de sortie
 */
export function parseLine(line: string): ParsedOutput[] {
	const results: ParsedOutput[] = [];
	const type = detectMessageType(line);

	// Extraire les références de fichiers
	const fileRefs = extractFileReferences(line);
	for (const ref of fileRefs) {
		results.push({
			type: 'file',
			content: ref.match,
			file: ref.path,
			line: ref.line,
			column: ref.column
		});
	}

	// Extraire les URLs
	const urls = extractUrls(line);
	for (const url of urls) {
		results.push({
			type: 'url',
			content: url,
			url
		});
	}

	// Ajouter le type de message global
	if (results.length === 0) {
		results.push({
			type: type === 'success' ? 'info' : type,
			content: line
		});
	}

	return results;
}

/**
 * Statistiques de sortie
 */
export interface OutputStats {
	errors: number;
	warnings: number;
	files: FileReference[];
	urls: string[];
}

/**
 * Analyser un bloc de texte complet
 */
export function analyzeOutput(text: string): OutputStats {
	const lines = text.split('\n');
	let errors = 0;
	let warnings = 0;

	for (const line of lines) {
		const type = detectMessageType(line);
		if (type === 'error') errors++;
		if (type === 'warning') warnings++;
	}

	return {
		errors,
		warnings,
		files: extractFileReferences(text),
		urls: extractUrls(text)
	};
}

/**
 * Formater un chemin de fichier pour l'affichage
 */
export function formatFilePath(path: string, line?: number, column?: number): string {
	let result = path;
	if (line !== undefined) {
		result += `:${line}`;
		if (column !== undefined) {
			result += `:${column}`;
		}
	}
	return result;
}

/**
 * Extraire le nom de fichier d'un chemin
 */
export function getFileName(path: string): string {
	const parts = path.split(/[/\\]/);
	return parts[parts.length - 1] || path;
}
