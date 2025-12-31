<script lang="ts">
	import { Command, Search, Terminal, Settings, FolderOpen, Save, Play, Undo2, Trash2, RefreshCw } from 'lucide-svelte';

	interface CommandItem {
		id: string;
		label: string;
		description: string;
		shortcut?: string;
		icon: typeof Command;
		action: () => void;
		category: 'navigation' | 'terminal' | 'project' | 'settings';
	}

	let {
		isOpen = false,
		onClose = () => {},
		onOpenFolder = () => {},
		onNewProject = () => {},
		onSave = () => {},
		onUndo = () => {},
		onSettings = () => {},
		onCompact = () => {},
		onClearTerminal = () => {},
		onSendCommand = (cmd: string) => {}
	} = $props<{
		isOpen: boolean;
		onClose: () => void;
		onOpenFolder: () => void;
		onNewProject: () => void;
		onSave: () => void;
		onUndo: () => void;
		onSettings: () => void;
		onCompact: () => void;
		onClearTerminal: () => void;
		onSendCommand: (cmd: string) => void;
	}>();

	let searchQuery = $state('');
	let selectedIndex = $state(0);
	let inputRef = $state<HTMLInputElement | null>(null);

	const commands: CommandItem[] = [
		{
			id: 'new-project',
			label: 'Nouveau projet',
			description: 'Créer un nouveau projet de développement',
			shortcut: 'Ctrl+N',
			icon: Terminal,
			action: onNewProject,
			category: 'project'
		},
		{
			id: 'open-folder',
			label: 'Ouvrir un projet',
			description: 'Sélectionner un projet existant',
			shortcut: 'Ctrl+O',
			icon: FolderOpen,
			action: onOpenFolder,
			category: 'project'
		},
		{
			id: 'save',
			label: 'Sauvegarder (Git)',
			description: 'Commit et push les modifications',
			shortcut: 'Ctrl+S',
			icon: Save,
			action: onSave,
			category: 'project'
		},
		{
			id: 'undo',
			label: 'Annuler',
			description: 'Restaurer le dernier checkpoint',
			shortcut: 'Ctrl+Z',
			icon: Undo2,
			action: onUndo,
			category: 'terminal'
		},
		{
			id: 'compact',
			label: '/compact',
			description: 'Compacter le contexte Claude',
			icon: RefreshCw,
			action: onCompact,
			category: 'terminal'
		},
		{
			id: 'clear',
			label: '/clear',
			description: 'Effacer le terminal',
			icon: Trash2,
			action: onClearTerminal,
			category: 'terminal'
		},
		{
			id: 'help',
			label: '/help',
			description: 'Afficher l\'aide Claude Code',
			icon: Command,
			action: () => onSendCommand('/help\n'),
			category: 'terminal'
		},
		{
			id: 'status',
			label: '/status',
			description: 'Voir le statut de la session',
			icon: Command,
			action: () => onSendCommand('/status\n'),
			category: 'terminal'
		},
		{
			id: 'settings',
			label: 'Paramètres',
			description: 'Ouvrir les paramètres de Léon',
			shortcut: 'Ctrl+,',
			icon: Settings,
			action: onSettings,
			category: 'settings'
		}
	];

	let filteredCommands = $derived(
		searchQuery.trim() === ''
			? commands
			: commands.filter(cmd =>
				cmd.label.toLowerCase().includes(searchQuery.toLowerCase()) ||
				cmd.description.toLowerCase().includes(searchQuery.toLowerCase())
			)
	);

	// Reset selection when filtered results change
	$effect(() => {
		if (filteredCommands.length > 0 && selectedIndex >= filteredCommands.length) {
			selectedIndex = 0;
		}
	});

	// Focus input when opened
	$effect(() => {
		if (isOpen && inputRef) {
			searchQuery = '';
			selectedIndex = 0;
			setTimeout(() => inputRef?.focus(), 50);
		}
	});

	function handleKeydown(e: KeyboardEvent) {
		switch (e.key) {
			case 'ArrowDown':
				e.preventDefault();
				selectedIndex = Math.min(selectedIndex + 1, filteredCommands.length - 1);
				break;
			case 'ArrowUp':
				e.preventDefault();
				selectedIndex = Math.max(selectedIndex - 1, 0);
				break;
			case 'Enter':
				e.preventDefault();
				if (filteredCommands[selectedIndex]) {
					executeCommand(filteredCommands[selectedIndex]);
				}
				break;
			case 'Escape':
				e.preventDefault();
				onClose();
				break;
		}
	}

	function executeCommand(cmd: CommandItem) {
		cmd.action();
		onClose();
	}

	function handleBackdropClick(e: MouseEvent) {
		if (e.target === e.currentTarget) {
			onClose();
		}
	}
</script>

{#if isOpen}
	<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
	<div class="palette-backdrop" onclick={handleBackdropClick}>
		<div class="palette-container">
			<div class="palette-header">
				<Search size={18} />
				<input
					bind:this={inputRef}
					bind:value={searchQuery}
					type="text"
					placeholder="Rechercher une commande..."
					class="palette-input"
					onkeydown={handleKeydown}
				/>
				<kbd class="shortcut-badge">Esc</kbd>
			</div>

			<div class="palette-results">
				{#each filteredCommands as cmd, i}
					<!-- svelte-ignore a11y_click_events_have_key_events -->
					<div
						class="palette-item"
						class:selected={i === selectedIndex}
						onclick={() => executeCommand(cmd)}
						role="button"
						tabindex="-1"
					>
						<div class="item-icon">
							<cmd.icon size={16} />
						</div>
						<div class="item-content">
							<span class="item-label">{cmd.label}</span>
							<span class="item-description">{cmd.description}</span>
						</div>
						{#if cmd.shortcut}
							<kbd class="item-shortcut">{cmd.shortcut}</kbd>
						{/if}
					</div>
				{:else}
					<div class="palette-empty">
						Aucune commande trouvée
					</div>
				{/each}
			</div>

			<div class="palette-footer">
				<span><kbd>↑↓</kbd> naviguer</span>
				<span><kbd>Enter</kbd> exécuter</span>
				<span><kbd>Esc</kbd> fermer</span>
			</div>
		</div>
	</div>
{/if}

<style>
	.palette-backdrop {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		justify-content: center;
		padding-top: 15vh;
		z-index: 1000;
		backdrop-filter: blur(2px);
	}

	.palette-container {
		width: 100%;
		max-width: 550px;
		max-height: 400px;
		background: var(--color-bg-primary);
		border: 1px solid var(--color-border);
		border-radius: 12px;
		box-shadow: 0 20px 40px rgba(0, 0, 0, 0.4);
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.palette-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.875rem 1rem;
		border-bottom: 1px solid var(--color-border);
		color: var(--color-text-muted);
	}

	.palette-input {
		flex: 1;
		background: transparent;
		border: none;
		outline: none;
		font-size: 1rem;
		color: var(--color-text-primary);
	}

	.palette-input::placeholder {
		color: var(--color-text-muted);
	}

	.shortcut-badge {
		padding: 0.125rem 0.375rem;
		background: var(--color-bg-tertiary);
		border: 1px solid var(--color-border);
		border-radius: 4px;
		font-size: 0.7rem;
		color: var(--color-text-muted);
	}

	.palette-results {
		flex: 1;
		overflow-y: auto;
		padding: 0.5rem;
	}

	.palette-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.625rem 0.75rem;
		border-radius: 6px;
		cursor: pointer;
		transition: background 0.1s ease;
	}

	.palette-item:hover,
	.palette-item.selected {
		background: var(--color-bg-hover);
	}

	.palette-item.selected {
		background: var(--color-lion-900);
	}

	.item-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		background: var(--color-bg-tertiary);
		border-radius: 6px;
		color: var(--color-text-secondary);
	}

	.palette-item.selected .item-icon {
		background: var(--color-lion-800);
		color: var(--color-lion-300);
	}

	.item-content {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.item-label {
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--color-text-primary);
	}

	.item-description {
		font-size: 0.75rem;
		color: var(--color-text-muted);
	}

	.item-shortcut {
		padding: 0.125rem 0.5rem;
		background: var(--color-bg-tertiary);
		border: 1px solid var(--color-border);
		border-radius: 4px;
		font-size: 0.65rem;
		color: var(--color-text-muted);
		font-family: inherit;
	}

	.palette-empty {
		padding: 2rem;
		text-align: center;
		color: var(--color-text-muted);
		font-size: 0.875rem;
	}

	.palette-footer {
		display: flex;
		gap: 1.5rem;
		padding: 0.5rem 1rem;
		border-top: 1px solid var(--color-border);
		font-size: 0.7rem;
		color: var(--color-text-muted);
	}

	.palette-footer kbd {
		padding: 0.125rem 0.25rem;
		background: var(--color-bg-tertiary);
		border: 1px solid var(--color-border);
		border-radius: 3px;
		font-size: 0.65rem;
		margin-right: 0.25rem;
	}
</style>
