<script lang="ts">
	import { X, Code, GitCompare, Globe, Monitor } from 'lucide-svelte';
	import CodePreview from './CodePreview.svelte';
	import DiffPreview from './DiffPreview.svelte';
	import WebPreview from './WebPreview.svelte';
	import AppPreview from './AppPreview.svelte';
	import type { PreviewMode, PreviewState, CodePreviewData, DiffPreviewData, WebPreviewData, AppPreviewData } from '$lib/types/preview';
	import { defaultPreviewState } from '$lib/types/preview';

	let {
		state = defaultPreviewState,
		onClose = () => {},
		onModeChange = (mode: PreviewMode) => {},
		onStateChange = (state: PreviewState) => {}
	} = $props<{
		state: PreviewState;
		onClose: () => void;
		onModeChange: (mode: PreviewMode) => void;
		onStateChange: (state: PreviewState) => void;
	}>();

	const modes: { id: PreviewMode; label: string; icon: typeof Code }[] = [
		{ id: 'code', label: 'Code', icon: Code },
		{ id: 'diff', label: 'Diff', icon: GitCompare },
		{ id: 'web', label: 'Web', icon: Globe },
		{ id: 'app', label: 'App', icon: Monitor }
	];

	function setMode(mode: PreviewMode) {
		onModeChange(mode);
		onStateChange({ ...state, mode });
	}

	function handleWebUrlUpdate(url: string) {
		onStateChange({
			...state,
			web: { ...state.web!, url }
		});
	}

	function handleAppWindowUpdate(windowTitle: string) {
		onStateChange({
			...state,
			app: { ...state.app!, windowTitle }
		});
	}
</script>

<div class="preview-panel">
	<header class="preview-header">
		<nav class="mode-tabs">
			{#each modes as mode}
				<button
					class="mode-tab"
					class:active={state.mode === mode.id}
					onclick={() => setMode(mode.id)}
					title={mode.label}
				>
					<svelte:component this={mode.icon} size={16} />
					<span class="tab-label">{mode.label}</span>
				</button>
			{/each}
		</nav>

		<button class="close-btn" onclick={onClose} title="Fermer le preview">
			<X size={18} />
		</button>
	</header>

	<div class="preview-content">
		{#if state.mode === 'code'}
			<CodePreview data={state.code} />
		{:else if state.mode === 'diff'}
			<DiffPreview data={state.diff} />
		{:else if state.mode === 'web'}
			<WebPreview
				data={state.web}
				onUpdateUrl={handleWebUrlUpdate}
			/>
		{:else if state.mode === 'app'}
			<AppPreview
				data={state.app}
				onUpdateWindow={handleAppWindowUpdate}
			/>
		{/if}
	</div>
</div>

<style>
	.preview-panel {
		height: 100%;
		display: flex;
		flex-direction: column;
		background-color: var(--color-bg-secondary);
		border-left: 1px solid var(--color-border);
	}

	.preview-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.375rem;
		background-color: var(--color-bg-tertiary);
		border-bottom: 1px solid var(--color-border);
	}

	.mode-tabs {
		display: flex;
		gap: 0.125rem;
	}

	.mode-tab {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.75rem;
		background: none;
		border: none;
		border-radius: 6px;
		color: var(--color-text-secondary);
		font-size: 0.8125rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.mode-tab:hover {
		background-color: var(--color-bg-hover);
		color: var(--color-text-primary);
	}

	.mode-tab.active {
		background-color: var(--color-lion-600);
		color: white;
	}

	.tab-label {
		display: none;
	}

	/* Show labels on wider panels */
	@media (min-width: 500px) {
		.tab-label {
			display: inline;
		}
	}

	.close-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: none;
		border: none;
		border-radius: 6px;
		color: var(--color-text-secondary);
		cursor: pointer;
	}

	.close-btn:hover {
		background-color: var(--color-error);
		color: white;
	}

	.preview-content {
		flex: 1;
		min-height: 0;
		overflow: hidden;
		position: relative;
	}
</style>
