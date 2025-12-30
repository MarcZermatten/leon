<script lang="ts">
	import { RefreshCw, Camera, Search, Play, Pause, Settings, AlertCircle } from 'lucide-svelte';
	import { invoke } from '@tauri-apps/api/core';
	import type { AppPreviewData } from '$lib/types/preview';

	let { data, onUpdateWindow = (title: string) => {} } = $props<{
		data: AppPreviewData | null;
		onUpdateWindow?: (title: string) => void;
	}>();

	let windowSearch = $state(data?.windowTitle || '');
	let screenshotSrc = $state<string | null>(null);
	let isCapturing = $state(false);
	let autoCapture = $state(data?.autoRefresh || false);
	let captureInterval = $state(data?.refreshInterval || 2000);
	let captureTimer: ReturnType<typeof setInterval> | null = null;
	let availableWindows = $state<string[]>([]);
	let showWindowList = $state(false);
	let error = $state<string | null>(null);
	let showSettings = $state(false);

	$effect(() => {
		if (data?.windowTitle && data.windowTitle !== windowSearch) {
			windowSearch = data.windowTitle;
		}
	});

	$effect(() => {
		// Auto-capture handling
		if (captureTimer) {
			clearInterval(captureTimer);
			captureTimer = null;
		}

		if (autoCapture && windowSearch && captureInterval > 0) {
			captureScreenshot();
			captureTimer = setInterval(() => {
				captureScreenshot();
			}, captureInterval);
		}

		return () => {
			if (captureTimer) {
				clearInterval(captureTimer);
			}
		};
	});

	async function captureScreenshot() {
		if (!windowSearch || isCapturing) return;

		isCapturing = true;
		error = null;

		try {
			// Appel à la commande Tauri pour capturer la fenêtre
			const base64Image = await invoke<string>('capture_window', {
				windowTitle: windowSearch
			});

			if (base64Image) {
				screenshotSrc = `data:image/png;base64,${base64Image}`;
			}
		} catch (e) {
			console.error('Capture error:', e);
			error = `Impossible de capturer "${windowSearch}". Vérifiez que la fenêtre est ouverte.`;
			screenshotSrc = null;
		} finally {
			isCapturing = false;
		}
	}

	async function listWindows() {
		showWindowList = true;
		try {
			const windows = await invoke<string[]>('list_windows');
			availableWindows = windows;
		} catch (e) {
			console.error('List windows error:', e);
			availableWindows = [];
		}
	}

	function selectWindow(title: string) {
		windowSearch = title;
		showWindowList = false;
		onUpdateWindow(title);
		captureScreenshot();
	}

	function handleSearchSubmit(e: Event) {
		e.preventDefault();
		onUpdateWindow(windowSearch);
		captureScreenshot();
	}

	function toggleAutoCapture() {
		autoCapture = !autoCapture;
		if (!autoCapture && captureTimer) {
			clearInterval(captureTimer);
			captureTimer = null;
		}
	}
</script>

<div class="app-preview">
	<div class="toolbar">
		<form class="search-form" onsubmit={handleSearchSubmit}>
			<input
				type="text"
				bind:value={windowSearch}
				placeholder="Nom de la fenêtre (ex: MyApp.exe)"
				class="search-input"
			/>
			<button type="button" class="search-btn" onclick={listWindows} title="Lister les fenêtres">
				<Search size={16} />
			</button>
		</form>

		<div class="toolbar-actions">
			<button
				class="action-btn"
				onclick={toggleAutoCapture}
				title={autoCapture ? 'Arrêter auto-capture' : 'Démarrer auto-capture'}
				class:active={autoCapture}
			>
				{#if autoCapture}
					<Pause size={16} />
				{:else}
					<Play size={16} />
				{/if}
			</button>
			<button
				class="action-btn"
				onclick={captureScreenshot}
				title="Capturer"
				disabled={isCapturing || !windowSearch}
				class:spinning={isCapturing}
			>
				<Camera size={16} />
			</button>
			<button class="action-btn" onclick={() => showSettings = !showSettings} title="Paramètres">
				<Settings size={16} />
			</button>
		</div>
	</div>

	{#if showSettings}
		<div class="settings-panel">
			<label class="setting-item">
				<span>Intervalle (ms):</span>
				<input
					type="number"
					bind:value={captureInterval}
					min="500"
					max="30000"
					step="500"
					class="number-input"
				/>
			</label>
		</div>
	{/if}

	{#if showWindowList}
		<div class="window-list">
			<div class="window-list-header">
				<span>Fenêtres disponibles</span>
				<button onclick={() => showWindowList = false}>&times;</button>
			</div>
			<div class="window-list-content">
				{#if availableWindows.length === 0}
					<p class="no-windows">Aucune fenêtre trouvée ou fonctionnalité non disponible</p>
				{:else}
					{#each availableWindows as win}
						<button class="window-item" onclick={() => selectWindow(win)}>
							{win}
						</button>
					{/each}
				{/if}
			</div>
		</div>
	{/if}

	<div class="preview-container">
		{#if error}
			<div class="error-state">
				<AlertCircle size={48} />
				<p>{error}</p>
				<button class="retry-btn" onclick={listWindows}>Lister les fenêtres</button>
			</div>
		{:else if screenshotSrc}
			<div class="screenshot-wrapper">
				<img src={screenshotSrc} alt="App Screenshot" class="screenshot" />
				{#if autoCapture}
					<div class="live-indicator">
						<span class="live-dot"></span>
						LIVE
					</div>
				{/if}
			</div>
		{:else if !windowSearch}
			<div class="empty-state">
				<Camera size={48} strokeWidth={1} />
				<p>Entrez le nom d'une fenêtre ou sélectionnez-en une</p>
				<button class="browse-btn" onclick={listWindows}>
					<Search size={16} />
					Parcourir les fenêtres
				</button>
			</div>
		{:else}
			<div class="empty-state">
				<RefreshCw size={48} strokeWidth={1} class={isCapturing ? 'spinner' : ''} />
				<p>Cliquez sur Capturer ou activez l'auto-capture</p>
			</div>
		{/if}
	</div>
</div>

<style>
	.app-preview {
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

	.search-form {
		flex: 1;
		display: flex;
		gap: 0.25rem;
	}

	.search-input {
		flex: 1;
		padding: 0.5rem 0.75rem;
		background-color: var(--color-bg-tertiary);
		border: 1px solid var(--color-border);
		border-radius: 6px;
		color: var(--color-text-primary);
		font-size: 0.8125rem;
	}

	.search-input:focus {
		outline: none;
		border-color: var(--color-lion-500);
	}

	.search-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		background-color: var(--color-bg-tertiary);
		border: 1px solid var(--color-border);
		border-radius: 6px;
		color: var(--color-text-secondary);
		cursor: pointer;
	}

	.search-btn:hover {
		background-color: var(--color-bg-hover);
		color: var(--color-text-primary);
	}

	.toolbar-actions {
		display: flex;
		gap: 0.25rem;
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

	.action-btn:hover:not(:disabled) {
		background-color: var(--color-bg-hover);
		color: var(--color-text-primary);
	}

	.action-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.action-btn.active {
		color: var(--color-lion-500);
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

	.number-input {
		width: 80px;
		padding: 0.25rem 0.5rem;
		background-color: var(--color-bg-secondary);
		border: 1px solid var(--color-border);
		border-radius: 4px;
		color: var(--color-text-primary);
		font-size: 0.8125rem;
	}

	.window-list {
		position: absolute;
		top: 50px;
		left: 0.5rem;
		right: 0.5rem;
		max-height: 300px;
		background-color: var(--color-bg-secondary);
		border: 1px solid var(--color-border);
		border-radius: 8px;
		z-index: 100;
		overflow: hidden;
		box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
	}

	.window-list-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.5rem 0.75rem;
		background-color: var(--color-bg-tertiary);
		border-bottom: 1px solid var(--color-border);
		font-size: 0.8125rem;
		color: var(--color-text-secondary);
	}

	.window-list-header button {
		background: none;
		border: none;
		color: var(--color-text-secondary);
		font-size: 1.25rem;
		cursor: pointer;
	}

	.window-list-content {
		max-height: 250px;
		overflow-y: auto;
	}

	.window-item {
		display: block;
		width: 100%;
		padding: 0.625rem 0.75rem;
		background: none;
		border: none;
		border-bottom: 1px solid var(--color-border);
		color: var(--color-text-primary);
		text-align: left;
		font-size: 0.8125rem;
		cursor: pointer;
	}

	.window-item:hover {
		background-color: var(--color-bg-hover);
	}

	.window-item:last-child {
		border-bottom: none;
	}

	.no-windows {
		padding: 1rem;
		text-align: center;
		color: var(--color-text-muted);
		font-size: 0.8125rem;
	}

	.preview-container {
		flex: 1;
		position: relative;
		overflow: auto;
		display: flex;
		align-items: center;
		justify-content: center;
		background-color: #1a1a1a;
		background-image:
			linear-gradient(45deg, #222 25%, transparent 25%),
			linear-gradient(-45deg, #222 25%, transparent 25%),
			linear-gradient(45deg, transparent 75%, #222 75%),
			linear-gradient(-45deg, transparent 75%, #222 75%);
		background-size: 20px 20px;
		background-position: 0 0, 0 10px, 10px -10px, -10px 0px;
	}

	.screenshot-wrapper {
		position: relative;
		max-width: 100%;
		max-height: 100%;
	}

	.screenshot {
		max-width: 100%;
		max-height: 100%;
		object-fit: contain;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
		border-radius: 4px;
	}

	.live-indicator {
		position: absolute;
		top: 0.5rem;
		right: 0.5rem;
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.25rem 0.625rem;
		background-color: rgba(239, 68, 68, 0.9);
		color: white;
		font-size: 0.6875rem;
		font-weight: 600;
		border-radius: 4px;
	}

	.live-dot {
		width: 6px;
		height: 6px;
		background-color: white;
		border-radius: 50%;
		animation: pulse 1s infinite;
	}

	@keyframes pulse {
		0%, 100% { opacity: 1; }
		50% { opacity: 0.5; }
	}

	.empty-state,
	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
		color: var(--color-text-muted);
		text-align: center;
		padding: 2rem;
	}

	.error-state {
		color: var(--color-error);
	}

	.empty-state p,
	.error-state p {
		margin: 0;
		font-size: 0.875rem;
		max-width: 300px;
	}

	.browse-btn,
	.retry-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background-color: var(--color-lion-600);
		color: white;
		border: none;
		border-radius: 6px;
		font-size: 0.8125rem;
		cursor: pointer;
	}

	.browse-btn:hover,
	.retry-btn:hover {
		background-color: var(--color-lion-500);
	}

	:global(.spinner) {
		animation: spin 1s linear infinite;
	}
</style>
