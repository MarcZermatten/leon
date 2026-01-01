<script lang="ts">
	import { Pencil, Trash2, FolderOpen, Copy } from 'lucide-svelte';

	interface MenuItem {
		label: string;
		icon?: typeof Pencil;
		action: () => void;
		variant?: 'danger' | 'default';
		separator?: boolean;
	}

	let {
		x = 0,
		y = 0,
		show = false,
		items = [],
		onClose = () => {}
	} = $props<{
		x: number;
		y: number;
		show: boolean;
		items: MenuItem[];
		onClose: () => void;
	}>();

	function handleItemClick(item: MenuItem) {
		item.action();
		onClose();
	}

	function handleBackdropClick() {
		onClose();
	}

	// Fermer avec Escape
	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			onClose();
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if show}
	<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
	<div class="context-backdrop" onclick={handleBackdropClick}></div>
	<div class="context-menu" style="left: {x}px; top: {y}px;">
		{#each items as item}
			{#if item.separator}
				<div class="separator"></div>
			{/if}
			<button
				class="menu-item"
				class:danger={item.variant === 'danger'}
				onclick={() => handleItemClick(item)}
			>
				{#if item.icon}
					<svelte:component this={item.icon} size={14} />
				{/if}
				<span>{item.label}</span>
			</button>
		{/each}
	</div>
{/if}

<style>
	.context-backdrop {
		position: fixed;
		inset: 0;
		z-index: 999;
	}

	.context-menu {
		position: fixed;
		z-index: 1000;
		background: var(--color-bg-secondary);
		border: 1px solid var(--color-border);
		border-radius: 8px;
		padding: 0.25rem;
		min-width: 160px;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
	}

	.menu-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.5rem 0.75rem;
		background: transparent;
		border: none;
		border-radius: 6px;
		color: var(--color-text-primary);
		font-size: 0.85rem;
		cursor: pointer;
		text-align: left;
		transition: background 0.1s;
	}

	.menu-item:hover {
		background: var(--color-bg-hover);
	}

	.menu-item.danger {
		color: #ff6b6b;
	}

	.menu-item.danger:hover {
		background: rgba(255, 107, 107, 0.1);
	}

	.separator {
		height: 1px;
		background: var(--color-border);
		margin: 0.25rem 0;
	}
</style>
