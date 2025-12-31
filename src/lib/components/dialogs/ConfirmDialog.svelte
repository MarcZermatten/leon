<script lang="ts">
	import { confirmDialog } from '$lib/stores/dialogs';
	import { AlertTriangle, Info, AlertCircle } from 'lucide-svelte';

	function handleConfirm() {
		const current = $confirmDialog;
		if (current.resolve) {
			current.resolve(true);
		}
		confirmDialog.set({ ...current, show: false, resolve: null });
	}

	function handleCancel() {
		const current = $confirmDialog;
		if (current.resolve) {
			current.resolve(false);
		}
		confirmDialog.set({ ...current, show: false, resolve: null });
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			handleCancel();
		} else if (e.key === 'Enter') {
			handleConfirm();
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if $confirmDialog.show}
<div class="dialog-overlay" onclick={handleCancel}>
	<div class="dialog-content" onclick={(e) => e.stopPropagation()}>
		<div class="dialog-icon" class:danger={$confirmDialog.variant === 'danger'} class:warning={$confirmDialog.variant === 'warning'}>
			{#if $confirmDialog.variant === 'danger'}
				<AlertCircle size={24} />
			{:else if $confirmDialog.variant === 'warning'}
				<AlertTriangle size={24} />
			{:else}
				<Info size={24} />
			{/if}
		</div>
		<h2>{$confirmDialog.title}</h2>
		<p>{$confirmDialog.message}</p>
		<div class="dialog-actions">
			<button class="btn-secondary" onclick={handleCancel}>{$confirmDialog.cancelText}</button>
			<button
				class="btn-primary"
				class:danger={$confirmDialog.variant === 'danger'}
				onclick={handleConfirm}
			>
				{$confirmDialog.confirmText}
			</button>
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

	.dialog-icon.danger {
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
	}

	.dialog-actions {
		display: flex;
		gap: 0.75rem;
		justify-content: center;
	}

	.btn-secondary, .btn-primary {
		padding: 0.5rem 1.25rem;
		border-radius: 6px;
		font-size: 0.9rem;
		cursor: pointer;
		border: none;
	}

	.btn-secondary {
		background: var(--color-bg-hover);
		color: var(--color-text-secondary);
	}

	.btn-secondary:hover {
		background: var(--color-bg-tertiary);
	}

	.btn-primary {
		background: var(--color-lion-600);
		color: white;
	}

	.btn-primary:hover {
		background: var(--color-lion-500);
	}

	.btn-primary.danger {
		background: var(--color-error, #ff6b6b);
	}

	.btn-primary.danger:hover {
		background: #ff5252;
	}
</style>
