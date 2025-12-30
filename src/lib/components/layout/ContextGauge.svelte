<script lang="ts">
	import { Database, AlertTriangle } from 'lucide-svelte';

	let {
		percent = 0,
		showAlert = true,
		onCompact = () => {}
	} = $props<{
		percent: number;
		showAlert?: boolean;
		onCompact?: () => void;
	}>();

	// Couleurs basées sur le pourcentage
	function getColor(p: number): string {
		if (p >= 90) return 'var(--color-error, #ff6b6b)';
		if (p >= 70) return 'var(--color-warning, #ffd43b)';
		return 'var(--color-success, #69db7c)';
	}

	// Niveau d'alerte
	let alertLevel = $derived(
		percent >= 90 ? 'critical' : percent >= 70 ? 'warning' : 'ok'
	);

	let showCompactTip = $derived(percent >= 80 && showAlert);
</script>

<div class="context-gauge" class:warning={alertLevel === 'warning'} class:critical={alertLevel === 'critical'}>
	<div class="gauge-icon" title="Utilisation du contexte">
		{#if alertLevel === 'critical'}
			<AlertTriangle size={14} />
		{:else}
			<Database size={14} />
		{/if}
	</div>

	<div class="gauge-bar-container">
		<div
			class="gauge-bar"
			style="width: {Math.min(percent, 100)}%; background-color: {getColor(percent)}"
		></div>
	</div>

	<span class="gauge-value" style="color: {getColor(percent)}">{Math.round(percent)}%</span>

	{#if showCompactTip}
		<button class="compact-btn" onclick={onCompact} title="Compacter le contexte avec /compact">
			/compact
		</button>
	{/if}
</div>

<style>
	.context-gauge {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.125rem 0.5rem;
		border-radius: 4px;
		transition: all 0.2s ease;
	}

	.context-gauge.warning {
		background: rgba(255, 212, 59, 0.1);
	}

	.context-gauge.critical {
		background: rgba(255, 107, 107, 0.15);
		animation: pulse 2s ease-in-out infinite;
	}

	@keyframes pulse {
		0%, 100% { opacity: 1; }
		50% { opacity: 0.7; }
	}

	.gauge-icon {
		display: flex;
		align-items: center;
		color: var(--color-text-muted);
	}

	.critical .gauge-icon {
		color: var(--color-error, #ff6b6b);
	}

	.gauge-bar-container {
		width: 60px;
		height: 6px;
		background: var(--color-bg-secondary);
		border-radius: 3px;
		overflow: hidden;
	}

	.gauge-bar {
		height: 100%;
		border-radius: 3px;
		transition: width 0.3s ease, background-color 0.3s ease;
	}

	.gauge-value {
		font-family: 'JetBrains Mono', 'Cascadia Code', monospace;
		font-size: 0.7rem;
		font-weight: 600;
		min-width: 32px;
		text-align: right;
	}

	.compact-btn {
		padding: 0.125rem 0.375rem;
		font-size: 0.65rem;
		font-family: 'JetBrains Mono', monospace;
		background: var(--color-lion-900);
		border: 1px solid var(--color-lion-700);
		border-radius: 3px;
		color: var(--color-lion-300);
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.compact-btn:hover {
		background: var(--color-lion-800);
		border-color: var(--color-lion-500);
		color: var(--color-lion-200);
	}
</style>
