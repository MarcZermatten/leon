<script lang="ts">
	import { alertDialog } from '$lib/stores/dialogs';
	import { CheckCircle, AlertCircle, AlertTriangle, Info } from 'lucide-svelte';

	function handleClose() {
		const current = $alertDialog;
		if (current.resolve) {
			current.resolve();
		}
		alertDialog.set({ ...current, show: false, resolve: null });
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape' || e.key === 'Enter') {
			handleClose();
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if $alertDialog.show}
<div class="dialog-overlay" onclick={handleClose}>
	<div class="dialog-content" onclick={(e) => e.stopPropagation()}>
		<div class="dialog-icon" class:success={$alertDialog.variant === 'success'} class:error={$alertDialog.variant === 'error'} class:warning={$alertDialog.variant === 'warning'}>
			{#if $alertDialog.variant === 'success'}
				<CheckCircle size={24} />
			{:else if $alertDialog.variant === 'error'}
				<AlertCircle size={24} />
			{:else if $alertDialog.variant === 'warning'}
				<AlertTriangle size={24} />
			{:else}
				<Info size={24} />
			{/if}
		</div>
		<h2>{$alertDialog.title}</h2>
		<p>{$alertDialog.message}</p>
		<div class="dialog-actions">
			<button class="btn-primary" onclick={handleClose} autofocus>OK</button>
		</div>
	</div>
</div>
{/if}

<style>
	.dialog-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
	}

	.dialog-content {
		background: var(--color-bg-secondary);
		border: 1px solid var(--color-border);
		border-radius: 12px;
		padding: 1.5rem;
		max-width: 400px;
		width: 90%;
		text-align: center;
	}

	.dialog-icon {
		display: flex;
		justify-content: center;
		margin-bottom: 1rem;
		color: var(--color-info, #74c0fc);
	}

	.dialog-icon.success {
		color: var(--color-success, #69db7c);
	}

	.dialog-icon.error {
		color: var(--color-error, #ff6b6b);
	}

	.dialog-icon.warning {
		color: var(--color-warning, #ffa94d);
	}

	h2 {
		margin: 0 0 0.5rem 0;
		color: var(--color-text-primary);
		font-size: 1.25rem;
	}

	p {
		margin: 0 0 1.5rem 0;
		color: var(--color-text-secondary);
		font-size: 0.9rem;
		white-space: pre-wrap;
	}

	.dialog-actions {
		display: flex;
		justify-content: center;
	}

	.btn-primary {
		padding: 0.5rem 2rem;
		border-radius: 6px;
		font-size: 0.9rem;
		cursor: pointer;
		border: none;
		background: var(--color-lion-600);
		color: white;
	}

	.btn-primary:hover {
		background: var(--color-lion-500);
	}
</style>
