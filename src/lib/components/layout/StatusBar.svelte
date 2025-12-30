<script lang="ts">
	import { Circle, Cpu, Hash, Zap, Calendar, Undo2, History } from 'lucide-svelte';
	import ContextGauge from './ContextGauge.svelte';

	let {
		project = 'Aucun projet',
		model = 'Claude Code',
		sessionId = null,
		tokensUsed = { input: 0, output: 0 },
		status = 'idle',
		contextUsedPercent = null,
		sessionMessages = null,
		todayMessages = null,
		weeklyMessages = null,
		checkpointCount = 0,
		onUndo = () => {},
		onShowCheckpoints = () => {},
		onCompact = () => {}
	} = $props<{
		project: string;
		model: string;
		sessionId: string | null;
		tokensUsed: { input: number; output: number };
		status: 'idle' | 'thinking' | 'executing' | 'error';
		contextUsedPercent: number | null;
		sessionMessages: number | null;
		todayMessages: number | null;
		weeklyMessages: number | null;
		checkpointCount: number;
		onUndo: () => void;
		onShowCheckpoints: () => void;
		onCompact: () => void;
	}>();

	const statusConfig: Record<string, { color: string; label: string }> = {
		idle: { color: 'var(--color-text-muted)', label: 'Prêt' },
		thinking: { color: 'var(--color-lion-500)', label: 'Réflexion...' },
		executing: { color: 'var(--color-success)', label: 'Exécution...' },
		error: { color: 'var(--color-error)', label: 'Erreur' }
	};

	function formatTokens(n: number): string {
		if (n >= 1000000) return (n / 1000000).toFixed(1) + 'M';
		if (n >= 1000) return (n / 1000).toFixed(1) + 'K';
		return n.toString();
	}
</script>

<footer class="status-bar">
	<div class="status-left">
		<div class="status-item">
			<Circle size={8} fill={statusConfig[status].color} color={statusConfig[status].color} />
			<span>{statusConfig[status].label}</span>
		</div>
		<div class="status-item">
			<Cpu size={14} />
			<span>{model}</span>
		</div>
		{#if checkpointCount > 0}
			<button
				class="status-btn undo"
				title="Annuler dernière modification (Ctrl+Z)"
				onclick={onUndo}
			>
				<Undo2 size={14} />
				<span>Undo</span>
			</button>
			<button
				class="status-btn history"
				title="Historique des checkpoints ({checkpointCount})"
				onclick={onShowCheckpoints}
			>
				<History size={14} />
				<span>{checkpointCount}</span>
			</button>
		{/if}
	</div>

	<div class="status-center">
		<div class="status-item project">
			<span class="project-name">{project}</span>
		</div>
	</div>

	<div class="status-right">
		{#if contextUsedPercent !== null}
			<ContextGauge percent={contextUsedPercent} {onCompact} />
		{/if}

		{#if sessionMessages !== null}
			<div class="status-item usage" title="Messages dans cette session">
				<Zap size={14} />
				<span>{sessionMessages} msg</span>
			</div>
		{/if}

		{#if todayMessages !== null}
			<div class="status-item today" title="Messages aujourd'hui">
				<span class="today-label">Auj:</span>
				<span>{todayMessages}</span>
			</div>
		{/if}

		{#if weeklyMessages !== null}
			<div class="status-item weekly" title="Messages cette semaine">
				<Calendar size={14} />
				<span>{weeklyMessages}</span>
			</div>
		{/if}

		{#if sessionId}
			<div class="status-item session" title="ID Session">
				<Hash size={14} />
				<span>{sessionId.slice(0, 8)}</span>
			</div>
		{/if}
	</div>
</footer>

<style>
	.status-bar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		height: 28px;
		padding: 0 0.75rem;
		background-color: var(--color-bg-tertiary);
		border-top: 1px solid var(--color-border);
		font-size: 0.75rem;
		color: var(--color-text-secondary);
	}

	.status-left,
	.status-center,
	.status-right {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.status-left {
		flex: 1;
	}

	.status-right {
		flex: 1;
		justify-content: flex-end;
	}

	.status-item {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		cursor: default;
	}

	.project-name {
		font-weight: 500;
		color: var(--color-lion-400);
	}

	.context span,
	.usage span,
	.today span,
	.weekly span {
		font-family: 'JetBrains Mono', 'Cascadia Code', monospace;
		font-weight: 500;
	}

	.today-label {
		color: var(--color-text-muted);
		font-weight: 400;
	}

	.session span {
		font-family: 'JetBrains Mono', 'Cascadia Code', monospace;
		opacity: 0.7;
	}

	.status-btn {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.125rem 0.5rem;
		background: transparent;
		border: 1px solid var(--color-border);
		border-radius: 4px;
		color: var(--color-text-secondary);
		font-size: 0.7rem;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.status-btn:hover {
		background: var(--color-bg-secondary);
		border-color: var(--color-lion-500);
		color: var(--color-lion-400);
	}

	.status-btn.undo:hover {
		border-color: var(--color-warning, #ffd43b);
		color: var(--color-warning, #ffd43b);
	}

	.status-btn.history span {
		font-family: 'JetBrains Mono', 'Cascadia Code', monospace;
		font-weight: 600;
	}
</style>
