<script lang="ts">
	import { Send, Square, Paperclip } from 'lucide-svelte';
	import MessageBubble from './MessageBubble.svelte';

	interface Message {
		id: string;
		type: 'user' | 'assistant' | 'tool_use' | 'tool_result' | 'system';
		content: string;
		timestamp: Date;
		toolName?: string;
		toolInput?: Record<string, any>;
	}

	let {
		messages = [],
		isLoading = false,
		onSendMessage = (msg: string) => {},
		onStopGeneration = () => {}
	} = $props<{
		messages: Message[];
		isLoading: boolean;
		onSendMessage: (msg: string) => void;
		onStopGeneration: () => void;
	}>();

	let inputValue = $state('');
	let messagesContainer: HTMLDivElement;

	function handleSubmit(e: Event) {
		e.preventDefault();
		if (inputValue.trim() && !isLoading) {
			onSendMessage(inputValue.trim());
			inputValue = '';
		}
	}

	function handleKeyDown(e: KeyboardEvent) {
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			handleSubmit(e);
		}
	}

	$effect(() => {
		// Auto-scroll to bottom on new messages
		if (messagesContainer && messages.length) {
			messagesContainer.scrollTop = messagesContainer.scrollHeight;
		}
	});
</script>

<div class="chat-panel">
	<div class="messages-container" bind:this={messagesContainer}>
		{#if messages.length === 0}
			<div class="empty-chat">
				<img src="/images/logo.png" alt="Léon" class="empty-logo" />
				<h2>Bienvenue dans Léon</h2>
				<p>Votre assistant Claude Code avec une interface améliorée.</p>
				<p class="hint">Tapez votre message pour commencer...</p>
			</div>
		{:else}
			{#each messages as message (message.id)}
				<MessageBubble {message} />
			{/each}
		{/if}

		{#if isLoading}
			<div class="loading-indicator">
				<div class="typing-dots">
					<span></span>
					<span></span>
					<span></span>
				</div>
			</div>
		{/if}
	</div>

	<form class="input-area" onsubmit={handleSubmit}>
		<div class="input-wrapper">
			<button type="button" class="attach-btn" title="Joindre un fichier">
				<Paperclip size={18} />
			</button>
			<textarea
				bind:value={inputValue}
				onkeydown={handleKeyDown}
				placeholder="Envoyer un message à Claude..."
				rows="1"
				disabled={isLoading}
			></textarea>
			{#if isLoading}
				<button type="button" class="stop-btn" onclick={onStopGeneration} title="Arrêter">
					<Square size={18} />
				</button>
			{:else}
				<button type="submit" class="send-btn" disabled={!inputValue.trim()} title="Envoyer">
					<Send size={18} />
				</button>
			{/if}
		</div>
	</form>
</div>

<style>
	.chat-panel {
		display: flex;
		flex-direction: column;
		height: 100%;
		background-color: var(--color-bg-primary);
	}

	.messages-container {
		flex: 1;
		overflow-y: auto;
		padding: 1rem;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.empty-chat {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		text-align: center;
		color: var(--color-text-secondary);
		gap: 0.5rem;
	}

	.empty-logo {
		width: 360px;
		height: auto;
		opacity: 0.9;
		margin-bottom: 2rem;
	}

	.empty-chat h2 {
		color: var(--color-text-primary);
		font-size: 1.5rem;
		font-weight: 600;
		margin: 0;
	}

	.empty-chat p {
		margin: 0;
		font-size: 0.875rem;
	}

	.empty-chat .hint {
		color: var(--color-text-muted);
		margin-top: 1rem;
	}

	.loading-indicator {
		display: flex;
		justify-content: flex-start;
		padding: 0.5rem 1rem;
	}

	.typing-dots {
		display: flex;
		gap: 4px;
	}

	.typing-dots span {
		width: 8px;
		height: 8px;
		background-color: var(--color-lion-500);
		border-radius: 50%;
		animation: typing 1.4s infinite ease-in-out both;
	}

	.typing-dots span:nth-child(1) { animation-delay: -0.32s; }
	.typing-dots span:nth-child(2) { animation-delay: -0.16s; }

	@keyframes typing {
		0%, 80%, 100% { transform: scale(0.6); opacity: 0.6; }
		40% { transform: scale(1); opacity: 1; }
	}

	.input-area {
		padding: 1rem;
		border-top: 1px solid var(--color-border);
		background-color: var(--color-bg-secondary);
	}

	.input-wrapper {
		display: flex;
		align-items: flex-end;
		gap: 0.5rem;
		background-color: var(--color-bg-tertiary);
		border: 1px solid var(--color-border);
		border-radius: 12px;
		padding: 0.5rem;
	}

	.input-wrapper:focus-within {
		border-color: var(--color-lion-500);
	}

	textarea {
		flex: 1;
		background: none;
		border: none;
		color: var(--color-text-primary);
		font-size: 0.9375rem;
		line-height: 1.5;
		resize: none;
		min-height: 24px;
		max-height: 200px;
		padding: 0.25rem 0.5rem;
	}

	textarea::placeholder {
		color: var(--color-text-muted);
	}

	textarea:focus {
		outline: none;
	}

	.attach-btn,
	.send-btn,
	.stop-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.15s;
	}

	.attach-btn {
		background: none;
		color: var(--color-text-secondary);
	}

	.attach-btn:hover {
		color: var(--color-text-primary);
		background-color: var(--color-bg-hover);
	}

	.send-btn {
		background-color: var(--color-lion-600);
		color: white;
	}

	.send-btn:hover:not(:disabled) {
		background-color: var(--color-lion-500);
	}

	.send-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.stop-btn {
		background-color: var(--color-error);
		color: white;
	}

	.stop-btn:hover {
		background-color: #dc2626;
	}
</style>
