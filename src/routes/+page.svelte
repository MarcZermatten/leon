<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import Sidebar from '$lib/components/layout/Sidebar.svelte';
	import ChatPanel from '$lib/components/chat/ChatPanel.svelte';
	import PreviewPanel from '$lib/components/preview/PreviewPanel.svelte';
	import StatusBar from '$lib/components/layout/StatusBar.svelte';
	import type { PreviewState, PreviewMode } from '$lib/types/preview';
	import { defaultPreviewState } from '$lib/types/preview';
	import {
		initClaudeListener,
		cleanupClaudeListener,
		sendPrompt,
		stopSession,
		checkClaudeAvailable,
		getClaudeVersion
	} from '$lib/services/claude';
	import {
		messages,
		session,
		isLoading,
		addUserMessage,
		addSystemMessage,
		resetChat
	} from '$lib/stores/chat';
	import type { ChatMessage } from '$lib/types/claude';

	// State
	let sessionsList = $state<Array<{ id: string; name: string; project: string; timestamp: Date }>>([]);
	let activeSession = $state<string | null>(null);
	let claudeAvailable = $state(false);
	let claudeVersion = $state<string | null>(null);

	// Preview state
	let previewState = $state<PreviewState>(defaultPreviewState);
	let showPreview = $state(true);

	// Panel widths
	let sidebarWidth = $state(260);
	let previewWidth = $state(450);

	// Convertir les messages du store au format attendu par ChatPanel
	function convertMessages(msgs: ChatMessage[]) {
		return msgs.map((m: ChatMessage) => ({
			id: m.id,
			type: m.role === 'assistant' ? 'assistant' as const :
			      m.role === 'user' ? 'user' as const :
			      m.role === 'tool' ? 'tool_use' as const :
			      'system' as const,
			content: m.content,
			timestamp: m.timestamp,
			toolName: m.toolName,
			toolInput: m.toolInput as Record<string, any> | undefined
		}));
	}

	onMount(async () => {
		// Vérifier si Claude CLI est disponible
		claudeAvailable = await checkClaudeAvailable();
		if (claudeAvailable) {
			claudeVersion = await getClaudeVersion();
		}

		// Initialiser l'écoute des events Claude
		await initClaudeListener();
	});

	onDestroy(() => {
		cleanupClaudeListener();
	});

	// Handlers
	function handleNewChat() {
		const id = crypto.randomUUID();
		const newSession = {
			id,
			name: `Conversation ${sessionsList.length + 1}`,
			project: 'Nouveau projet',
			timestamp: new Date()
		};
		sessionsList = [newSession, ...sessionsList];
		activeSession = id;
		resetChat();
	}

	function handleSelectSession(id: string) {
		activeSession = id;
		// TODO: Load session messages from storage
	}

	async function handleSendMessage(content: string) {
		if (!claudeAvailable) {
			// Mode simulation si Claude CLI n'est pas disponible
			addUserMessage(content);
			addSystemMessage('Claude CLI non disponible. Installez-le avec: npm install -g @anthropic-ai/claude-code', true);
			return;
		}

		// Envoyer via le service Claude
		await sendPrompt(content);
	}

	async function handleStopGeneration() {
		await stopSession();
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

		<!-- Chat -->
		<div class="chat-container">
			<ChatPanel
				messages={convertMessages($messages)}
				isLoading={$isLoading}
				onSendMessage={handleSendMessage}
				onStopGeneration={handleStopGeneration}
			/>
		</div>

		<!-- Preview (conditional) -->
		{#if showPreview}
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
		project={activeSession ? sessionsList.find(s => s.id === activeSession)?.project || 'Projet' : 'Aucun projet'}
		model="claude-3.5-sonnet"
		sessionId={activeSession}
		tokensUsed={{ input: $session.totalInputTokens, output: $session.totalOutputTokens }}
		status={$session.isRunning ? 'thinking' : 'idle'}
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

	.chat-container {
		flex: 1;
		min-width: 400px;
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
</style>
