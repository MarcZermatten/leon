<script lang="ts">
	import { RefreshCw, ExternalLink, Smartphone, Monitor, Tablet, Settings } from 'lucide-svelte';
	import type { WebPreviewData } from '$lib/types/preview';

	let { data, onUpdateUrl = (url: string) => {} } = $props<{
		data: WebPreviewData | null;
		onUpdateUrl?: (url: string) => void;
	}>();

	let iframeRef: HTMLIFrameElement;
	let urlInput = $state(data?.url || 'http://localhost:5173');
	let isLoading = $state(true);
	let viewport = $state<'desktop' | 'tablet' | 'mobile'>('desktop');
	let showSettings = $state(false);
	let autoRefresh = $state(data?.autoRefresh || false);
	let refreshInterval = $state(data?.refreshInterval || 5000);
	let refreshTimer: ReturnType<typeof setInterval> | null = null;

	const viewportSizes = {
		desktop: { width: '100%', height: '100%' },
		tablet: { width: '768px', height: '1024px' },
		mobile: { width: '375px', height: '667px' }
	};

	$effect(() => {
		if (data?.url && data.url !== urlInput) {
			urlInput = data.url;
		}
	});

	$effect(() => {
		// Auto-refresh handling
		if (refreshTimer) {
			clearInterval(refreshTimer);
			refreshTimer = null;
		}

		if (autoRefresh && refreshInterval > 0) {
			refreshTimer = setInterval(() => {
				refresh();
			}, refreshInterval);
		}

		return () => {
			if (refreshTimer) {
				clearInterval(refreshTimer);
			}
		};
	});

	function handleUrlSubmit(e: Event) {
		e.preventDefault();
		let url = urlInput.trim();
		if (!url.startsWith('http://') && !url.startsWith('https://')) {
			url = 'http://' + url;
			urlInput = url;
		}
		onUpdateUrl(url);
		refresh();
	}

	function refresh() {
		if (iframeRef) {
			isLoading = true;
			iframeRef.src = iframeRef.src;
		}
	}

	function handleIframeLoad() {
		isLoading = false;
	}

	function openExternal() {
		window.open(urlInput, '_blank');
	}
</script>

<div class="web-preview">
	<div class="toolbar">
		<form class="url-form" onsubmit={handleUrlSubmit}>
			<input
				type="text"
				bind:value={urlInput}
				placeholder="http://localhost:5173"
				class="url-input"
			/>
		</form>

		<div class="toolbar-actions">
			<div class="viewport-toggle">
				<button
					class:active={viewport === 'desktop'}
					onclick={() => viewport = 'desktop'}
					title="Desktop"
				>
					<Monitor size={16} />
				</button>
				<button
					class:active={viewport === 'tablet'}
					onclick={() => viewport = 'tablet'}
					title="Tablet"
				>
					<Tablet size={16} />
				</button>
				<button
					class:active={viewport === 'mobile'}
					onclick={() => viewport = 'mobile'}
					title="Mobile"
				>
					<Smartphone size={16} />
				</button>
			</div>

			<button class="action-btn" onclick={refresh} title="Rafraîchir" class:spinning={isLoading}>
				<RefreshCw size={16} />
			</button>
			<button class="action-btn" onclick={openExternal} title="Ouvrir dans le navigateur">
				<ExternalLink size={16} />
			</button>
			<button class="action-btn" onclick={() => showSettings = !showSettings} title="Paramètres">
				<Settings size={16} />
			</button>
		</div>
	</div>

	{#if showSettings}
		<div class="settings-panel">
			<label class="setting-item">
				<input type="checkbox" bind:checked={autoRefresh} />
				<span>Auto-refresh</span>
			</label>
			{#if autoRefresh}
				<label class="setting-item">
					<span>Intervalle (ms):</span>
					<input
						type="number"
						bind:value={refreshInterval}
						min="1000"
						max="60000"
						step="1000"
						class="number-input"
					/>
				</label>
			{/if}
		</div>
	{/if}

	<div class="iframe-container" class:centered={viewport !== 'desktop'}>
		{#if isLoading}
			<div class="loading-overlay">
				<RefreshCw size={32} class="spinner" />
				<span>Chargement...</span>
			</div>
		{/if}

		<div
			class="iframe-wrapper"
			style="width: {viewportSizes[viewport].width}; height: {viewportSizes[viewport].height}"
		>
			<iframe
				bind:this={iframeRef}
				src={urlInput}
				title={data?.title || 'Web Preview'}
				onload={handleIframeLoad}
				sandbox="allow-forms allow-modals allow-pointer-lock allow-popups allow-same-origin allow-scripts allow-top-navigation"
			></iframe>
		</div>
	</div>
</div>

<style>
	.web-preview {
		height: 100%;
		display: flex;
		flex-direction: column;
		background-color: var(--color-bg-primary);
	}

	.toolbar {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem;
		background-color: var(--color-bg-secondary);
		border-bottom: 1px solid var(--color-border);
	}

	.url-form {
		flex: 1;
	}

	.url-input {
		width: 100%;
		padding: 0.5rem 0.75rem;
		background-color: var(--color-bg-tertiary);
		border: 1px solid var(--color-border);
		border-radius: 6px;
		color: var(--color-text-primary);
		font-family: 'JetBrains Mono', monospace;
		font-size: 0.8125rem;
	}

	.url-input:focus {
		outline: none;
		border-color: var(--color-lion-500);
	}

	.toolbar-actions {
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	.viewport-toggle {
		display: flex;
		background-color: var(--color-bg-tertiary);
		border-radius: 6px;
		padding: 2px;
		margin-right: 0.5rem;
	}

	.viewport-toggle button {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		background: none;
		border: none;
		border-radius: 4px;
		color: var(--color-text-secondary);
		cursor: pointer;
	}

	.viewport-toggle button:hover {
		color: var(--color-text-primary);
	}

	.viewport-toggle button.active {
		background-color: var(--color-lion-600);
		color: white;
	}

	.action-btn {
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

	.action-btn:hover {
		background-color: var(--color-bg-hover);
		color: var(--color-text-primary);
	}

	.action-btn.spinning :global(svg) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	.settings-panel {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 0.5rem 1rem;
		background-color: var(--color-bg-tertiary);
		border-bottom: 1px solid var(--color-border);
		font-size: 0.8125rem;
	}

	.setting-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: var(--color-text-secondary);
	}

	.setting-item input[type="checkbox"] {
		accent-color: var(--color-lion-500);
	}

	.number-input {
		width: 80px;
		padding: 0.25rem 0.5rem;
		background-color: var(--color-bg-secondary);
		border: 1px solid var(--color-border);
		border-radius: 4px;
		color: var(--color-text-primary);
		font-size: 0.8125rem;
	}

	.iframe-container {
		flex: 1;
		position: relative;
		overflow: auto;
		background-color: #1a1a1a;
		background-image:
			linear-gradient(45deg, #222 25%, transparent 25%),
			linear-gradient(-45deg, #222 25%, transparent 25%),
			linear-gradient(45deg, transparent 75%, #222 75%),
			linear-gradient(-45deg, transparent 75%, #222 75%);
		background-size: 20px 20px;
		background-position: 0 0, 0 10px, 10px -10px, -10px 0px;
	}

	.iframe-container.centered {
		display: flex;
		align-items: flex-start;
		justify-content: center;
		padding: 1rem;
	}

	.iframe-wrapper {
		background-color: white;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
		border-radius: 4px;
		overflow: hidden;
	}

	iframe {
		width: 100%;
		height: 100%;
		border: none;
		display: block;
	}

	.loading-overlay {
		position: absolute;
		inset: 0;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		background-color: rgba(0, 0, 0, 0.7);
		color: var(--color-text-secondary);
		z-index: 10;
	}

	.loading-overlay :global(.spinner) {
		animation: spin 1s linear infinite;
		color: var(--color-lion-500);
	}
</style>
