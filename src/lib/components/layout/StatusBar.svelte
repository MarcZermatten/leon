<script lang="ts">
	import { Circle, Cpu, Hash, Zap, Database, Calendar } from 'lucide-svelte';

	let {
		project = 'Aucun projet',
		model = 'Claude Code',
		sessionId = null,
		tokensUsed = { input: 0, output: 0 },
		status = 'idle',
		contextRemaining = null,
		sessionUsage = null,
		weeklyUsage = null
	} = $props<{
		project: string;
		model: string;
		sessionId: string | null;
		tokensUsed: { input: number; output: number };
		status: 'idle' | 'thinking' | 'executing' | 'error';
		contextRemaining: number | null;
		sessionUsage: { used: number; limit: number } | null;
		weeklyUsage: { used: number; limit: number } | null;
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

	function formatPercentage(used: number, limit: number): string {
		if (limit === 0) return '0%';
		return Math.round((used / limit) * 100) + '%';
	}

	function getUsageColor(used: number, limit: number): string {
		const pct = limit > 0 ? (used / limit) * 100 : 0;
		if (pct >= 90) return 'var(--color-error)';
		if (pct >= 70) return 'var(--color-warning, #ffd43b)';
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
		{#if contextRemaining !== null}
			<div class="status-item context" title="Contexte restant avant auto-compactage">
				<Database size={14} />
				<span style="color: {getUsageColor(100 - contextRemaining, 100)}">{contextRemaining}%</span>
			</div>
		{/if}

		{#if sessionUsage}
			<div class="status-item usage" title="Utilisation session">
				<Zap size={14} />
				<span style="color: {getUsageColor(sessionUsage.used, sessionUsage.limit)}">
					{formatPercentage(sessionUsage.used, sessionUsage.limit)}
				</span>
			</div>
		{/if}

		{#if weeklyUsage}
			<div class="status-item weekly" title="Utilisation semaine">
				<Calendar size={14} />
				<span style="color: {getUsageColor(weeklyUsage.used, weeklyUsage.limit)}">
					{formatPercentage(weeklyUsage.used, weeklyUsage.limit)}
				</span>
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
	.weekly span {
		font-family: 'JetBrains Mono', 'Cascadia Code', monospace;
		font-weight: 500;
	}

	.session span {
		font-family: 'JetBrains Mono', 'Cascadia Code', monospace;
		opacity: 0.7;
	}
</style>
