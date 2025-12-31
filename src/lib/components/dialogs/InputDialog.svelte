<script lang="ts">
	import { inputDialog } from '$lib/stores/dialogs';

	let inputValue = $state('');

	$effect(() => {
		if ($inputDialog.show) {
			inputValue = $inputDialog.defaultValue;
		}
	});

	function handleConfirm() {
		const current = $inputDialog;
		if (current.resolve) {
			current.resolve(inputValue);
		}
		inputDialog.set({ ...current, show: false, resolve: null });
	}

	function handleCancel() {
		const current = $inputDialog;
		if (current.resolve) {
			current.resolve(null);
		}
		inputDialog.set({ ...current, show: false, resolve: null });
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			handleCancel();
		} else if (e.key === 'Enter') {
			handleConfirm();
		}
	}
</script>

{#if $inputDialog.show}
<div class="dialog-overlay" onclick={handleCancel}>
	<div class="dialog-content" onclick={(e) => e.stopPropagation()}>
		<h2>{$inputDialog.title}</h2>
		<p>{$inputDialog.message}</p>
		<input
			type="text"
			bind:value={inputValue}
			placeholder={$inputDialog.placeholder}
			autofocus
			onkeydown={handleKeydown}
		/>
		<div class="dialog-actions">
			<button class="btn-secondary" onclick={handleCancel}>{$inputDialog.cancelText}</button>
			<button class="btn-primary" onclick={handleConfirm} disabled={!inputValue.trim()}>
				{$inputDialog.confirmText}
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
	}

	h2 {
		margin: 0 0 0.5rem 0;
		color: var(--color-text-primary);
		font-size: 1.25rem;
	}

	p {
		margin: 0 0 1rem 0;
		color: var(--color-text-secondary);
		font-size: 0.9rem;
	}

	input {
		width: 100%;
		padding: 0.75rem;
		background: var(--color-bg-primary);
		border: 1px solid var(--color-border);
		border-radius: 8px;
		color: var(--color-text-primary);
		font-size: 1rem;
		margin-bottom: 1rem;
	}

	input:focus {
		outline: none;
		border-color: var(--color-lion-500);
	}

	.dialog-actions {
		display: flex;
		gap: 0.75rem;
		justify-content: flex-end;
	}

	.btn-secondary, .btn-primary {
		padding: 0.5rem 1rem;
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

	.btn-primary:hover:not(:disabled) {
		background: var(--color-lion-500);
	}

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}
</style>
