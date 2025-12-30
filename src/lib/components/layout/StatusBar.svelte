<script lang="ts">
	import { Circle, Cpu, Hash, Clock, Zap } from 'lucide-svelte';

	let {
		project = 'Aucun projet',
		model = 'claude-3.5-sonnet',
		sessionId = null,
		tokensUsed = { input: 0, output: 0 },
		status = 'idle'
	} = $props<{
		project: string;
		model: string;
		sessionId: string | null;
		tokensUsed: { input: number; output: number };
		status: 'idle' | 'thinking' | 'executing' | 'error';
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
	</div>

	<div class="status-center">
		<div class="status-item project">
			<span class="project-name">{project}</span>
		</div>
	</div>

	<div class="status-right">
		<div class="status-item tokens">
			<Zap size={14} />
			<span>{formatTokens(tokensUsed.input)} / {formatTokens(tokensUsed.output)}</span>
		</div>
		{#if sessionId}
			<div class="status-item session">
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
	}

	.project-name {
		font-weight: 500;
		color: var(--color-lion-400);
	}

	.tokens span {
		font-family: 'JetBrains Mono', monospace;
	}

	.session span {
		font-family: 'JetBrains Mono', monospace;
		opacity: 0.7;
	}
</style>
