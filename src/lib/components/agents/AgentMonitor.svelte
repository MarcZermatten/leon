<script lang="ts">
	import { Bot, Loader2, CheckCircle, XCircle, Clock, Zap, ChevronDown, ChevronRight } from 'lucide-svelte';

	interface Agent {
		id: string;
		name: string;
		type: string;
		status: 'running' | 'completed' | 'failed' | 'queued';
		startTime: number;
		endTime?: number;
		model: 'haiku' | 'sonnet' | 'opus';
		description?: string;
	}

	let {
		agents = [],
		contextUsage = 0,
		maxContext = 200000,
		onKillAgent = (id: string) => {}
	} = $props<{
		agents: Agent[];
		contextUsage: number;
		maxContext: number;
		onKillAgent: (id: string) => void;
	}>();

	let isExpanded = $state(true);

	const runningAgents = $derived(agents.filter((a: Agent) => a.status === 'running'));
	const completedAgents = $derived(agents.filter((a: Agent) => a.status === 'completed'));
	const failedAgents = $derived(agents.filter((a: Agent) => a.status === 'failed'));

	const contextPercent = $derived(Math.round((contextUsage / maxContext) * 100));
	const contextColor = $derived(
		contextPercent > 80 ? 'var(--color-error)' :
		contextPercent > 60 ? 'var(--color-warning)' :
		'var(--color-success)'
	);

	function formatDuration(start: number, end?: number): string {
		const ms = (end || Date.now()) - start;
		if (ms < 1000) return `${ms}ms`;
		return `${(ms / 1000).toFixed(1)}s`;
	}

	function getModelColor(model: string): string {
		switch (model) {
			case 'opus': return '#a855f7';
			case 'sonnet': return '#3b82f6';
			case 'haiku': return '#22c55e';
			default: return '#6b7280';
		}
	}

	function getStatusIcon(status: string) {
		switch (status) {
			case 'running': return Loader2;
			case 'completed': return CheckCircle;
			case 'failed': return XCircle;
			case 'queued': return Clock;
			default: return Bot;
		}
	}
</script>

<div class="agent-monitor">
	<button class="monitor-header" onclick={() => isExpanded = !isExpanded}>
		<div class="header-left">
			{#if isExpanded}
				<ChevronDown size={14} />
			{:else}
				<ChevronRight size={14} />
			{/if}
			<Bot size={16} />
			<span class="title">Agents</span>
			{#if runningAgents.length > 0}
				<span class="running-badge">
					<Loader2 size={12} class="spin" />
					{runningAgents.length}
				</span>
			{/if}
		</div>
		<div class="header-right">
			<div class="context-indicator" title="Utilisation contexte: {contextPercent}%">
				<div class="context-bar">
					<div
						class="context-fill"
						style="width: {contextPercent}%; background-color: {contextColor}"
					></div>
				</div>
				<span class="context-text">{contextPercent}%</span>
			</div>
		</div>
	</button>

	{#if isExpanded}
		<div class="monitor-content">
			{#if agents.length === 0}
				<div class="empty-state">
					<Zap size={20} />
					<span>Aucun agent actif</span>
				</div>
			{:else}
				<div class="agents-list">
					{#each agents as agent (agent.id)}
						<div class="agent-item" class:running={agent.status === 'running'} class:failed={agent.status === 'failed'}>
							<div class="agent-icon" class:spin={agent.status === 'running'}>
								<svelte:component this={getStatusIcon(agent.status)} size={14} />
							</div>
							<div class="agent-info">
								<div class="agent-name">{agent.name}</div>
								{#if agent.description}
									<div class="agent-desc">{agent.description}</div>
								{/if}
							</div>
							<div class="agent-meta">
								<span class="model-badge" style="background-color: {getModelColor(agent.model)}">
									{agent.model}
								</span>
								<span class="duration">
									{formatDuration(agent.startTime, agent.endTime)}
								</span>
							</div>
							{#if agent.status === 'running'}
								<button class="kill-btn" onclick={() => onKillAgent(agent.id)} title="Arrêter">
									<XCircle size={12} />
								</button>
							{/if}
						</div>
					{/each}
				</div>

				<div class="stats-row">
					<span class="stat">
						<CheckCircle size={12} />
						{completedAgents.length} terminés
					</span>
					{#if failedAgents.length > 0}
						<span class="stat error">
							<XCircle size={12} />
							{failedAgents.length} échecs
						</span>
					{/if}
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.agent-monitor {
		background-color: var(--color-bg-secondary);
		border: 1px solid var(--color-border);
		border-radius: 8px;
		overflow: hidden;
	}

	.monitor-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		width: 100%;
		padding: 0.5rem 0.75rem;
		background: none;
		border: none;
		cursor: pointer;
		color: var(--color-text-primary);
	}

	.monitor-header:hover {
		background-color: var(--color-bg-hover);
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.title {
		font-size: 0.8rem;
		font-weight: 500;
	}

	.running-badge {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.125rem 0.375rem;
		background-color: var(--color-lion-900);
		border-radius: 10px;
		font-size: 0.7rem;
		color: var(--color-lion-300);
	}

	.header-right {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.context-indicator {
		display: flex;
		align-items: center;
		gap: 0.375rem;
	}

	.context-bar {
		width: 60px;
		height: 6px;
		background-color: var(--color-bg-tertiary);
		border-radius: 3px;
		overflow: hidden;
	}

	.context-fill {
		height: 100%;
		border-radius: 3px;
		transition: width 0.3s, background-color 0.3s;
	}

	.context-text {
		font-size: 0.65rem;
		color: var(--color-text-muted);
		min-width: 28px;
	}

	.monitor-content {
		border-top: 1px solid var(--color-border);
		padding: 0.5rem;
	}

	.empty-state {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 1rem;
		color: var(--color-text-muted);
		font-size: 0.8rem;
	}

	.agents-list {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	.agent-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.375rem 0.5rem;
		background-color: var(--color-bg-tertiary);
		border-radius: 6px;
		font-size: 0.75rem;
	}

	.agent-item.running {
		border-left: 2px solid var(--color-lion-500);
	}

	.agent-item.failed {
		border-left: 2px solid var(--color-error);
	}

	.agent-icon {
		color: var(--color-text-muted);
	}

	.agent-icon.spin :global(svg) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	.agent-info {
		flex: 1;
		min-width: 0;
	}

	.agent-name {
		font-weight: 500;
		color: var(--color-text-primary);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.agent-desc {
		font-size: 0.65rem;
		color: var(--color-text-muted);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.agent-meta {
		display: flex;
		align-items: center;
		gap: 0.375rem;
	}

	.model-badge {
		padding: 0.0625rem 0.25rem;
		border-radius: 4px;
		font-size: 0.6rem;
		font-weight: 500;
		color: white;
		text-transform: uppercase;
	}

	.duration {
		color: var(--color-text-muted);
		font-family: 'JetBrains Mono', monospace;
		font-size: 0.65rem;
	}

	.kill-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0.25rem;
		background: none;
		border: none;
		border-radius: 4px;
		color: var(--color-text-muted);
		cursor: pointer;
	}

	.kill-btn:hover {
		background-color: var(--color-error);
		color: white;
	}

	.stats-row {
		display: flex;
		gap: 0.75rem;
		margin-top: 0.5rem;
		padding-top: 0.5rem;
		border-top: 1px solid var(--color-border);
	}

	.stat {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.7rem;
		color: var(--color-text-muted);
	}

	.stat.error {
		color: var(--color-error);
	}

	:global(.spin) {
		animation: spin 1s linear infinite;
	}
</style>
