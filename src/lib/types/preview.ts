// Types pour le système de Preview multi-mode

export type PreviewMode = 'code' | 'diff' | 'web' | 'app';

export interface CodePreviewData {
	filePath: string;
	content: string;
	language: string;
	lineNumbers?: boolean;
	highlightLines?: number[];
}

export interface DiffPreviewData {
	filePath: string;
	oldContent: string;
	newContent: string;
	language: string;
}

export interface WebPreviewData {
	url: string;
	title?: string;
	autoRefresh?: boolean;
	refreshInterval?: number; // en ms
}

export interface AppPreviewData {
	windowTitle: string;
	processName?: string;
	autoRefresh: boolean;
	refreshInterval: number; // en ms
	lastScreenshot?: string; // base64 ou path
}

export interface PreviewState {
	mode: PreviewMode;
	isVisible: boolean;
	code: CodePreviewData | null;
	diff: DiffPreviewData | null;
	web: WebPreviewData | null;
	app: AppPreviewData | null;
}

// Configuration par défaut
export const defaultPreviewState: PreviewState = {
	mode: 'code',
	isVisible: true,
	code: null,
	diff: null,
	web: {
		url: 'http://localhost:5173',
		title: 'Dev Server',
		autoRefresh: false,
		refreshInterval: 5000
	},
	app: {
		windowTitle: '',
		autoRefresh: true,
		refreshInterval: 2000,
		lastScreenshot: undefined
	}
};

// Helper pour détecter le langage depuis l'extension
export function detectLanguage(filePath: string): string {
	const ext = filePath.split('.').pop()?.toLowerCase() || '';
	const languageMap: Record<string, string> = {
		ts: 'typescript',
		tsx: 'typescript',
		js: 'javascript',
		jsx: 'javascript',
		svelte: 'html',
		vue: 'html',
		html: 'html',
		css: 'css',
		scss: 'scss',
		less: 'less',
		json: 'json',
		md: 'markdown',
		py: 'python',
		rs: 'rust',
		go: 'go',
		java: 'java',
		kt: 'kotlin',
		swift: 'swift',
		sql: 'sql',
		sh: 'bash',
		bash: 'bash',
		yaml: 'yaml',
		yml: 'yaml',
		toml: 'toml',
		xml: 'xml',
		dart: 'dart',
		c: 'c',
		cpp: 'cpp',
		h: 'c',
		hpp: 'cpp',
		cs: 'csharp',
		rb: 'ruby',
		php: 'php',
		r: 'r'
	};
	return languageMap[ext] || 'plaintext';
}
