<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import Sidebar from '$lib/components/layout/Sidebar.svelte';
	import Terminal from '$lib/components/terminal/Terminal.svelte';
	import PreviewPanel from '$lib/components/preview/PreviewPanel.svelte';
	import StatusBar from '$lib/components/layout/StatusBar.svelte';
	import type { PreviewState, PreviewMode } from '$lib/types/preview';
	import { defaultPreviewState } from '$lib/types/preview';
	import { checkClaudeAvailable, getClaudeVersion } from '$lib/services/claude';
	import { FolderOpen } from 'lucide-svelte';

	// State
	let sessionsList = $state<Array<{ id: string; name: string; project: string; timestamp: Date }>>([]);
	let activeSession = $state<string | null>(null);
	let claudeAvailable = $state(false);
	let claudeVersion = $state<string | null>(null);
	let workingDir = $state<string | null>(null);
	let terminalReady = $state(false);
	let terminalComponent: Terminal;

	// Preview state
	let previewState = $state<PreviewState>(defaultPreviewState);
	let showPreview = $state(true);

	// Panel widths
	let sidebarWidth = $state(260);
	let previewWidth = $state(450);

	onMount(async () => {
		// Vérifier si Claude CLI est disponible
		claudeAvailable = await checkClaudeAvailable();
		if (claudeAvailable) {
			claudeVersion = await getClaudeVersion();
		}
	});

	// Handlers
	async function handleOpenFolder() {
		try {
			const { open } = await import('@tauri-apps/plugin-dialog');
			const selected = await open({
				directory: true,
				multiple: false,
				title: 'Sélectionner un dossier de projet'
			});
			if (selected && typeof selected === 'string') {
				workingDir = selected;
				// Ajouter à la liste des sessions
				const id = crypto.randomUUID();
				const projectName = selected.split(/[/\\]/).pop() || 'Projet';
				const newSession = {
					id,
					name: projectName,
					project: selected,
					timestamp: new Date()
				};
				sessionsList = [newSession, ...sessionsList];
				activeSession = id;
			}
		} catch (e) {
			console.error('Erreur ouverture dossier:', e);
		}
	}

	function handleNewChat() {
		// Reset le terminal avec un nouveau projet
		handleOpenFolder();
	}

	function handleSelectSession(id: string) {
		activeSession = id;
		const session = sessionsList.find(s => s.id === id);
		if (session) {
			workingDir = session.project;
		}
	}

	function handleTerminalReady() {
		terminalReady = true;
		terminalComponent?.focus();
	}

	function closePreview() {
		showPreview = false;
	}

	function handlePreviewModeChange(mode: PreviewMode) {
		previewState = { ...previewState, mode };
	}

	function handlePreviewStateChange(newState: PreviewState) {
		previewState = newState;
	}

	// Resize panel logic
	let isResizing = $state<'sidebar' | 'preview' | null>(null);
	let startX = 0;
	let startWidth = 0;

	function startResize(panel: 'sidebar' | 'preview', e: MouseEvent) {
		isResizing = panel;
		startX = e.clientX;
		startWidth = panel === 'sidebar' ? sidebarWidth : previewWidth;
		document.body.style.cursor = 'col-resize';
		document.body.style.userSelect = 'none';
	}

	function handleMouseMove(e: MouseEvent) {
		if (!isResizing) return;

		const delta = e.clientX - startX;

		if (isResizing === 'sidebar') {
			const newWidth = Math.max(200, Math.min(400, startWidth + delta));
			sidebarWidth = newWidth;
		} else if (isResizing === 'preview') {
			const newWidth = Math.max(350, Math.min(800, startWidth - delta));
			previewWidth = newWidth;
		}
	}

	function stopResize() {
		if (isResizing) {
			isResizing = null;
			document.body.style.cursor = '';
			document.body.style.userSelect = '';
		}
	}
</script>

<svelte:window onmousemove={handleMouseMove} onmouseup={stopResize} />

<div class="app-container">
	<div class="main-layout">
		<!-- Sidebar -->
		<div class="sidebar-container" style="width: {sidebarWidth}px">
			<Sidebar
				sessions={sessionsList}
				{activeSession}
				onNewChat={handleNewChat}
				onSelectSession={handleSelectSession}
			/>
		</div>

		<!-- Resize handle sidebar -->
		<div
			class="resize-handle sidebar-resize"
			onmousedown={(e) => startResize('sidebar', e)}
			role="separator"
			aria-orientation="vertical"
		></div>

		<!-- Terminal -->
		<div class="terminal-container">
			{#if workingDir}
				<div class="terminal-header">
					<span class="project-path">{workingDir}</span>
				</div>
				{#key workingDir}
					<Terminal
						bind:this={terminalComponent}
						{workingDir}
						onReady={handleTerminalReady}
					/>
				{/key}
			{:else}
				<div class="welcome-screen">
					<img src="/images/logo.png" alt="Léon" class="welcome-logo" />
					<h1>Bienvenue dans Léon</h1>
					<p>Interface graphique pour Claude Code</p>

					{#if claudeAvailable}
						<p class="claude-status success">
							Claude Code CLI détecté {claudeVersion ? `(${claudeVersion})` : ''}
						</p>
					{:else}
						<p class="claude-status error">
							Claude Code CLI non détecté
						</p>
						<p class="install-hint">
							Installez-le avec: <code>npm install -g @anthropic-ai/claude-code</code>
						</p>
					{/if}

					<button class="open-folder-btn" onclick={handleOpenFolder} disabled={!claudeAvailable}>
						<FolderOpen size={20} />
						<span>Ouvrir un projet</span>
					</button>
				</div>
			{/if}
		</div>

		<!-- Preview (conditional) -->
		{#if showPreview && workingDir}
			<!-- Resize handle preview -->
			<div
				class="resize-handle preview-resize"
				onmousedown={(e) => startResize('preview', e)}
				role="separator"
				aria-orientation="vertical"
			></div>

			<div class="preview-container" style="width: {previewWidth}px">
				<PreviewPanel
					state={previewState}
					onClose={closePreview}
					onModeChange={handlePreviewModeChange}
					onStateChange={handlePreviewStateChange}
				/>
			</div>
		{/if}
	</div>

	<!-- Status Bar -->
	<StatusBar
		project={workingDir?.split(/[/\\]/).pop() || 'Aucun projet'}
		model={claudeVersion || 'Claude Code'}
		sessionId={activeSession}
		tokensUsed={{ input: 0, output: 0 }}
		status={terminalReady ? 'idle' : 'thinking'}
	/>
</div>

<style>
	.app-container {
		display: flex;
		flex-direction: column;
		height: 100vh;
		overflow: hidden;
	}

	.main-layout {
		display: flex;
		flex: 1;
		min-height: 0;
	}

	.sidebar-container {
		flex-shrink: 0;
		min-width: 200px;
		max-width: 400px;
	}

	.terminal-container {
		flex: 1;
		min-width: 400px;
		display: flex;
		flex-direction: column;
		background-color: #1a1a1a;
	}

	.terminal-header {
		padding: 8px 12px;
		background-color: #252525;
		border-bottom: 1px solid var(--color-border);
		font-size: 0.75rem;
		color: var(--color-text-secondary);
	}

	.project-path {
		font-family: monospace;
	}

	.preview-container {
		flex-shrink: 0;
		min-width: 350px;
		max-width: 800px;
	}

	.resize-handle {
		width: 4px;
		cursor: col-resize;
		background-color: transparent;
		transition: background-color 0.15s;
	}

	.resize-handle:hover {
		background-color: var(--color-lion-500);
	}

	.sidebar-resize {
		border-right: 1px solid var(--color-border);
	}

	.preview-resize {
		border-left: 1px solid var(--color-border);
	}

	/* Welcome screen */
	.welcome-screen {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		padding: 2rem;
		text-align: center;
	}

	.welcome-logo {
		width: 120px;
		height: auto;
		margin-bottom: 1rem;
	}

	.welcome-screen h1 {
		font-size: 1.75rem;
		font-weight: 600;
		color: var(--color-text-primary);
		margin: 0;
	}

	.welcome-screen p {
		color: var(--color-text-secondary);
		margin: 0;
	}

	.claude-status {
		padding: 0.5rem 1rem;
		border-radius: 6px;
		font-size: 0.875rem;
	}

	.claude-status.success {
		background-color: rgba(105, 219, 124, 0.1);
		color: #69db7c;
	}

	.claude-status.error {
		background-color: rgba(255, 107, 107, 0.1);
		color: #ff6b6b;
	}

	.install-hint {
		font-size: 0.875rem;
	}

	.install-hint code {
		background-color: #2a2a2a;
		padding: 0.25rem 0.5rem;
		border-radius: 4px;
		font-family: monospace;
	}

	.open-folder-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background-color: var(--color-lion-600);
		color: var(--color-text-primary);
		border: none;
		border-radius: 8px;
		font-size: 1rem;
		font-weight: 500;
		cursor: pointer;
		margin-top: 1rem;
		transition: background-color 0.15s;
	}

	.open-folder-btn:hover:not(:disabled) {
		background-color: var(--color-lion-500);
	}

	.open-folder-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}
</style>
