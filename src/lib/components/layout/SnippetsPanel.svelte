<script lang="ts">
	import {
		Play,
		GitBranch,
		MessageSquare,
		Package,
		TestTube,
		CheckCircle,
		Download,
		List,
		Eye,
		Wand2,
		FileText,
		Bug,
		Zap,
		X,
		Search,
		Plus,
		Trash2
	} from 'lucide-svelte';
	import {
		loadSnippets,
		addSnippet,
		deleteSnippet,
		getSnippetsByCategory,
		searchSnippets,
		CATEGORY_NAMES,
		type Snippet
	} from '$lib/services/snippets';
	import { showConfirm } from '$lib/stores/dialogs';

	let {
		isVisible = true,
		onClose = () => {},
		onExecute = (command: string) => {}
	} = $props<{
		isVisible: boolean;
		onClose: () => void;
		onExecute: (command: string) => void;
	}>();

	let snippets = $state<Snippet[]>([]);
	let searchQuery = $state('');
	let showAddForm = $state(false);
	let newSnippet = $state({ name: '', command: '', category: 'custom', description: '' });

	// Charger les snippets au montage
	$effect(() => {
		if (isVisible) {
			snippets = loadSnippets();
		}
	});

	// Snippets filtrés
	let filteredSnippets = $derived(
		searchQuery ? searchSnippets(snippets, searchQuery) : snippets
	);

	// Snippets par catégorie
	let snippetsByCategory = $derived(getSnippetsByCategory(filteredSnippets));

	// Icons mapping
	const iconMap: Record<string, any> = {
		play: Play,
		package: Package,
		'test-tube': TestTube,
		'check-circle': CheckCircle,
		'git-branch': GitBranch,
		download: Download,
		list: List,
		'message-square': MessageSquare,
		eye: Eye,
		wand: Wand2,
		'file-text': FileText,
		bug: Bug,
		zap: Zap
	};

	function getIcon(iconName?: string) {
		return iconMap[iconName || 'play'] || Play;
	}

	function handleExecute(snippet: Snippet) {
		onExecute(snippet.command + '\n');
	}

	function handleAddSnippet() {
		if (!newSnippet.name.trim() || !newSnippet.command.trim()) return;

		addSnippet({
			name: newSnippet.name.trim(),
			command: newSnippet.command.trim(),
			category: newSnippet.category,
			description: newSnippet.description.trim() || undefined
		});

		// Recharger et reset
		snippets = loadSnippets();
		newSnippet = { name: '', command: '', category: 'custom', description: '' };
		showAddForm = false;
	}

	async function handleDelete(id: string) {
		const confirmed = await showConfirm({
			title: 'Supprimer le snippet',
			message: 'Voulez-vous vraiment supprimer ce snippet ?',
			confirmText: 'Supprimer',
			variant: 'danger'
		});
		if (confirmed) {
			if (deleteSnippet(id)) {
				snippets = loadSnippets();
			}
		}
	}
</script>

{#if isVisible}
	<div class="snippets-panel">
		<div class="panel-header">
			<div class="header-title">
				<Zap size={16} />
				<span>Snippets</span>
			</div>
			<div class="header-actions">
				<button class="icon-btn" onclick={() => (showAddForm = !showAddForm)} title="Ajouter">
					<Plus size={14} />
				</button>
				<button class="icon-btn" onclick={onClose} title="Fermer">
					<X size={14} />
				</button>
			</div>
		</div>

		<div class="search-box">
			<Search size={14} />
			<input
				type="text"
				placeholder="Rechercher..."
				bind:value={searchQuery}
			/>
		</div>

		{#if showAddForm}
			<div class="add-form">
				<input
					type="text"
					placeholder="Nom du snippet"
					bind:value={newSnippet.name}
				/>
				<textarea
					placeholder="Commande ou prompt"
					bind:value={newSnippet.command}
				></textarea>
				<input
					type="text"
					placeholder="Description (optionnel)"
					bind:value={newSnippet.description}
				/>
				<div class="form-actions">
					<button class="cancel-btn" onclick={() => (showAddForm = false)}>
						Annuler
					</button>
					<button class="save-btn" onclick={handleAddSnippet}>
						Ajouter
					</button>
				</div>
			</div>
		{/if}

		<div class="panel-content">
			{#each Object.entries(snippetsByCategory) as [category, categorySnippets]}
				<div class="category">
					<div class="category-header">
						{CATEGORY_NAMES[category] || category}
					</div>
					<div class="snippets-list">
						{#each categorySnippets as snippet (snippet.id)}
							<div class="snippet-item">
								<button
									class="snippet-btn"
									onclick={() => handleExecute(snippet)}
									title={snippet.description || snippet.command}
								>
									<svelte:component this={getIcon(snippet.icon)} size={14} />
									<span class="snippet-name">{snippet.name}</span>
								</button>
								{#if snippet.id.startsWith('custom-')}
									<button
										class="delete-btn"
										onclick={() => handleDelete(snippet.id)}
										title="Supprimer"
									>
										<Trash2 size={12} />
									</button>
								{/if}
							</div>
						{/each}
					</div>
				</div>
			{/each}

			{#if filteredSnippets.length === 0}
				<div class="empty">
					{searchQuery ? 'Aucun résultat' : 'Aucun snippet'}
				</div>
			{/if}
		</div>
	</div>
{/if}

<style>
	.snippets-panel {
		display: flex;
		flex-direction: column;
		height: 100%;
		background: var(--color-bg-secondary);
		border-left: 1px solid var(--color-border);
	}

	.panel-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.5rem 0.75rem;
		border-bottom: 1px solid var(--color-border);
		background: var(--color-bg-tertiary);
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.8rem;
		font-weight: 600;
		color: var(--color-text-primary);
	}

	.header-actions {
		display: flex;
		gap: 0.25rem;
	}

	.icon-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		background: transparent;
		border: none;
		border-radius: 4px;
		color: var(--color-text-secondary);
		cursor: pointer;
	}

	.icon-btn:hover {
		background: var(--color-bg-hover);
		color: var(--color-text-primary);
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		border-bottom: 1px solid var(--color-border);
		color: var(--color-text-muted);
	}

	.search-box input {
		flex: 1;
		background: transparent;
		border: none;
		color: var(--color-text-primary);
		font-size: 0.8rem;
		outline: none;
	}

	.search-box input::placeholder {
		color: var(--color-text-muted);
	}

	.add-form {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		padding: 0.75rem;
		border-bottom: 1px solid var(--color-border);
		background: var(--color-bg-tertiary);
	}

	.add-form input,
	.add-form textarea {
		padding: 0.5rem;
		background: var(--color-bg-primary);
		border: 1px solid var(--color-border);
		border-radius: 4px;
		color: var(--color-text-primary);
		font-size: 0.8rem;
	}

	.add-form textarea {
		min-height: 60px;
		resize: vertical;
		font-family: 'JetBrains Mono', monospace;
	}

	.add-form input:focus,
	.add-form textarea:focus {
		outline: none;
		border-color: var(--color-lion-500);
	}

	.form-actions {
		display: flex;
		justify-content: flex-end;
		gap: 0.5rem;
	}

	.cancel-btn,
	.save-btn {
		padding: 0.375rem 0.75rem;
		border-radius: 4px;
		font-size: 0.75rem;
		cursor: pointer;
	}

	.cancel-btn {
		background: transparent;
		border: 1px solid var(--color-border);
		color: var(--color-text-secondary);
	}

	.cancel-btn:hover {
		background: var(--color-bg-hover);
	}

	.save-btn {
		background: var(--color-lion-600);
		border: none;
		color: white;
	}

	.save-btn:hover {
		background: var(--color-lion-500);
	}

	.panel-content {
		flex: 1;
		overflow-y: auto;
		padding: 0.5rem;
	}

	.category {
		margin-bottom: 0.75rem;
	}

	.category-header {
		padding: 0.375rem 0.5rem;
		font-size: 0.65rem;
		font-weight: 600;
		color: var(--color-text-muted);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.snippets-list {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.snippet-item {
		display: flex;
		align-items: center;
	}

	.snippet-btn {
		flex: 1;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		background: transparent;
		border: none;
		border-radius: 4px;
		color: var(--color-text-secondary);
		font-size: 0.8rem;
		cursor: pointer;
		text-align: left;
	}

	.snippet-btn:hover {
		background: var(--color-bg-hover);
		color: var(--color-text-primary);
	}

	.snippet-name {
		flex: 1;
	}

	.delete-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		background: transparent;
		border: none;
		border-radius: 4px;
		color: var(--color-text-muted);
		cursor: pointer;
		opacity: 0;
	}

	.snippet-item:hover .delete-btn {
		opacity: 1;
	}

	.delete-btn:hover {
		background: rgba(255, 107, 107, 0.2);
		color: var(--color-error);
	}

	.empty {
		padding: 2rem;
		text-align: center;
		font-size: 0.8rem;
		color: var(--color-text-muted);
	}
</style>
