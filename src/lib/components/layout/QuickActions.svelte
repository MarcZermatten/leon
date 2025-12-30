<script lang="ts">
	import {
		Play,
		Bug,
		TestTube,
		FileCode,
		Wand2,
		MessageSquarePlus,
		Eraser,
		RotateCcw,
		Zap
	} from 'lucide-svelte';

	let {
		projectPath = null,
		hasGitChanges = false,
		contextUsedPercent = 0,
		onSendCommand = (cmd: string) => {}
	} = $props<{
		projectPath: string | null;
		hasGitChanges: boolean;
		contextUsedPercent: number;
		onSendCommand: (cmd: string) => void;
	}>();

	interface QuickAction {
		id: string;
		icon: any;
		label: string;
		command: string;
		color: string;
		condition?: () => boolean;
	}

	const actions: QuickAction[] = [
		{
			id: 'explain',
			icon: MessageSquarePlus,
			label: 'Expliquer',
			command: 'Explique-moi le code actuel et son fonctionnement\n',
			color: 'var(--color-info)'
		},
		{
			id: 'fix',
			icon: Bug,
			label: 'Fix Bug',
			command: 'Trouve et corrige les bugs dans le code actuel\n',
			color: 'var(--color-error)'
		},
		{
			id: 'test',
			icon: TestTube,
			label: 'Tests',
			command: 'Crée des tests unitaires pour le code actuel\n',
			color: 'var(--color-success)'
		},
		{
			id: 'refactor',
			icon: Wand2,
			label: 'Refactor',
			command: 'Refactorise le code pour améliorer la lisibilité et les performances\n',
			color: 'var(--color-warning)'
		},
		{
			id: 'doc',
			icon: FileCode,
			label: 'Documenter',
			command: 'Ajoute de la documentation et des commentaires au code\n',
			color: 'var(--color-lion-400)'
		},
		{
			id: 'run',
			icon: Play,
			label: 'Run',
			command: 'Lance le projet en mode développement\n',
			color: 'var(--color-success)'
		},
		{
			id: 'compact',
			icon: Eraser,
			label: 'Compact',
			command: '/compact\n',
			color: 'var(--color-text-muted)',
			condition: () => contextUsedPercent > 70
		},
		{
			id: 'clear',
			icon: RotateCcw,
			label: 'Clear',
			command: '/clear\n',
			color: 'var(--color-text-muted)'
		}
	];

	// Actions visibles basées sur les conditions
	let visibleActions = $derived(actions.filter((a) => !a.condition || a.condition()));

	function handleAction(action: QuickAction) {
		onSendCommand(action.command);
	}
</script>

{#if projectPath}
	<div class="quick-actions">
		<div class="actions-header">
			<Zap size={12} />
			<span>Quick Actions</span>
		</div>
		<div class="actions-grid">
			{#each visibleActions as action (action.id)}
				<button
					class="action-btn"
					onclick={() => handleAction(action)}
					title={action.label}
					style="--action-color: {action.color}"
				>
					<svelte:component this={action.icon} size={14} />
					<span>{action.label}</span>
				</button>
			{/each}
		</div>
	</div>
{/if}

<style>
	.quick-actions {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		padding: 0.5rem;
		background: var(--color-bg-tertiary);
		border-top: 1px solid var(--color-border);
	}

	.actions-header {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.65rem;
		font-weight: 600;
		color: var(--color-text-muted);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.actions-grid {
		display: flex;
		flex-wrap: wrap;
		gap: 0.375rem;
	}

	.action-btn {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.25rem 0.5rem;
		background: var(--color-bg-secondary);
		border: 1px solid var(--color-border);
		border-radius: 4px;
		color: var(--color-text-secondary);
		font-size: 0.7rem;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.action-btn:hover {
		background: var(--color-bg-hover);
		border-color: var(--action-color);
		color: var(--action-color);
	}

	.action-btn:active {
		transform: scale(0.98);
	}
</style>
