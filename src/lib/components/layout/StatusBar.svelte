<script lang="ts">
	import { Circle, Cpu, Hash, Zap, Database, Calendar } from 'lucide-svelte';

	let {
		project = 'Aucun projet',
		model = 'Claude Code',
		sessionId = null,
		tokensUsed = { input: 0, output: 0 },
		status = 'idle',
		contextUsedPercent = null,
		sessionMessages = null,
		todayMessages = null,
		weeklyMessages = null
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

	function getContextColor(percent: number): string {
		if (percent >= 90) return 'var(--color-error)';
		if (percent >= 70) return 'var(--color-warning, #ffd43b)';
		return 'var(--color-success, #69db7c)';
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
	</div>

	<div class="status-center">
		<div class="status-item project">
			<span class="project-name">{project}</span>
		</div>
	</div>

	<div class="status-right">
		{#if contextUsedPercent !== null}
			<div class="status-item context" title="Contexte utilisé ({Math.round(contextUsedPercent)}% de 200K tokens)">
				<Database size={14} />
				<span style="color: {getContextColor(contextUsedPercent)}">{Math.round(contextUsedPercent)}%</span>
			</div>
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
</style>
