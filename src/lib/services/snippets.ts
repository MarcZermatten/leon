// Service pour gérer les snippets et templates de commandes fréquentes

export interface Snippet {
	id: string;
	name: string;
	command: string;
	category: string;
	icon?: string;
	shortcut?: string;
	description?: string;
}

// Clé localStorage
const SNIPPETS_KEY = 'leon_snippets';

// Snippets par défaut
const DEFAULT_SNIPPETS: Snippet[] = [
	// Development
	{
		id: 'dev-start',
		name: 'Start Dev',
		command: 'npm run dev',
		category: 'dev',
		icon: 'play',
		description: 'Lancer le serveur de développement'
	},
	{
		id: 'dev-build',
		name: 'Build',
		command: 'npm run build',
		category: 'dev',
		icon: 'package',
		description: 'Construire le projet pour la production'
	},
	{
		id: 'dev-test',
		name: 'Tests',
		command: 'npm test',
		category: 'dev',
		icon: 'test-tube',
		description: 'Lancer les tests'
	},
	{
		id: 'dev-lint',
		name: 'Lint',
		command: 'npm run lint',
		category: 'dev',
		icon: 'check-circle',
		description: 'Vérifier le code'
	},
	// Git
	{
		id: 'git-status',
		name: 'Git Status',
		command: 'git status',
		category: 'git',
		icon: 'git-branch',
		description: 'Voir l\'état du repo'
	},
	{
		id: 'git-pull',
		name: 'Git Pull',
		command: 'git pull',
		category: 'git',
		icon: 'download',
		description: 'Récupérer les changements'
	},
	{
		id: 'git-log',
		name: 'Git Log',
		command: 'git log --oneline -10',
		category: 'git',
		icon: 'list',
		description: 'Voir les 10 derniers commits'
	},
	// Claude prompts
	{
		id: 'claude-explain',
		name: 'Explain Code',
		command: 'Explique-moi ce code en détail, son fonctionnement et ses patterns utilisés.',
		category: 'claude',
		icon: 'message-square',
		description: 'Demander une explication du code'
	},
	{
		id: 'claude-review',
		name: 'Code Review',
		command: 'Fais une revue de code complète: bugs potentiels, améliorations, bonnes pratiques.',
		category: 'claude',
		icon: 'eye',
		description: 'Demander une revue de code'
	},
	{
		id: 'claude-refactor',
		name: 'Refactor',
		command: 'Refactorise ce code pour améliorer sa lisibilité et sa maintenabilité.',
		category: 'claude',
		icon: 'wand',
		description: 'Demander un refactoring'
	},
	{
		id: 'claude-tests',
		name: 'Write Tests',
		command: 'Écris des tests unitaires complets pour ce code avec une bonne couverture.',
		category: 'claude',
		icon: 'test-tube',
		description: 'Demander d\'écrire des tests'
	},
	{
		id: 'claude-docs',
		name: 'Add Docs',
		command: 'Ajoute de la documentation et des commentaires JSDoc/TSDoc au code.',
		category: 'claude',
		icon: 'file-text',
		description: 'Demander d\'ajouter de la documentation'
	},
	{
		id: 'claude-fix',
		name: 'Fix Bug',
		command: 'Il y a un bug: [décris le bug]. Trouve la cause et corrige-le.',
		category: 'claude',
		icon: 'bug',
		description: 'Demander de corriger un bug'
	},
	{
		id: 'claude-optimize',
		name: 'Optimize',
		command: 'Optimise ce code pour de meilleures performances.',
		category: 'claude',
		icon: 'zap',
		description: 'Demander une optimisation'
	}
];

/**
 * Charger tous les snippets (défauts + personnalisés)
 */
export function loadSnippets(): Snippet[] {
	try {
		const saved = localStorage.getItem(SNIPPETS_KEY);
		if (saved) {
			const custom = JSON.parse(saved) as Snippet[];
			// Fusionner avec les défauts, les personnalisés override les défauts
			const defaultIds = new Set(DEFAULT_SNIPPETS.map((s) => s.id));
			const customNonDefault = custom.filter((s) => !defaultIds.has(s.id));
			return [...DEFAULT_SNIPPETS, ...customNonDefault];
		}
	} catch (e) {
		console.error('[Snippets] Error loading:', e);
	}
	return DEFAULT_SNIPPETS;
}

/**
 * Sauvegarder les snippets personnalisés
 */
export function saveSnippets(snippets: Snippet[]): void {
	try {
		// Ne sauvegarder que les non-defaults ou modifiés
		const defaultIds = new Set(DEFAULT_SNIPPETS.map((s) => s.id));
		const toSave = snippets.filter((s) => !defaultIds.has(s.id));
		localStorage.setItem(SNIPPETS_KEY, JSON.stringify(toSave));
	} catch (e) {
		console.error('[Snippets] Error saving:', e);
	}
}

/**
 * Ajouter un nouveau snippet
 */
export function addSnippet(snippet: Omit<Snippet, 'id'>): Snippet {
	const newSnippet: Snippet = {
		...snippet,
		id: `custom-${Date.now()}`
	};

	const current = loadSnippets();
	const updated = [...current, newSnippet];
	saveSnippets(updated);

	return newSnippet;
}

/**
 * Supprimer un snippet (seulement les personnalisés)
 */
export function deleteSnippet(id: string): boolean {
	// Ne pas supprimer les défauts
	if (DEFAULT_SNIPPETS.find((s) => s.id === id)) {
		return false;
	}

	const current = loadSnippets();
	const updated = current.filter((s) => s.id !== id);
	saveSnippets(updated);

	return true;
}

/**
 * Obtenir les snippets par catégorie
 */
export function getSnippetsByCategory(snippets: Snippet[]): Record<string, Snippet[]> {
	return snippets.reduce(
		(acc, snippet) => {
			if (!acc[snippet.category]) {
				acc[snippet.category] = [];
			}
			acc[snippet.category].push(snippet);
			return acc;
		},
		{} as Record<string, Snippet[]>
	);
}

/**
 * Noms des catégories
 */
export const CATEGORY_NAMES: Record<string, string> = {
	dev: 'Développement',
	git: 'Git',
	claude: 'Claude Prompts',
	custom: 'Personnalisés'
};

/**
 * Chercher des snippets
 */
export function searchSnippets(snippets: Snippet[], query: string): Snippet[] {
	const lowerQuery = query.toLowerCase();
	return snippets.filter(
		(s) =>
			s.name.toLowerCase().includes(lowerQuery) ||
			s.command.toLowerCase().includes(lowerQuery) ||
			s.description?.toLowerCase().includes(lowerQuery)
	);
}
