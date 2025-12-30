<script lang="ts">
	import { User, Bot, Terminal, FileText, Check } from 'lucide-svelte';

	interface Message {
		id: string;
		type: 'user' | 'assistant' | 'tool_use' | 'tool_result' | 'system';
		content: string;
		timestamp: Date;
		toolName?: string;
		toolInput?: Record<string, any>;
	}

	let { message } = $props<{ message: Message }>();

	const toolIcons: Record<string, any> = {
		Read: FileText,
		Edit: FileText,
		Write: FileText,
		Bash: Terminal,
		default: Terminal
	};

	function getToolIcon(toolName: string | undefined) {
		return toolIcons[toolName || 'default'] || toolIcons.default;
	}

	function formatTime(date: Date): string {
		return date.toLocaleTimeString('fr-CH', { hour: '2-digit', minute: '2-digit' });
	}
</script>

<div class="message" class:user={message.type === 'user'} class:assistant={message.type === 'assistant'} class:tool={message.type === 'tool_use' || message.type === 'tool_result'}>
	{#if message.type === 'user'}
		<div class="avatar user-avatar">
			<User size={18} />
		</div>
		<div class="content">
			<div class="bubble user-bubble">
				{message.content}
			</div>
			<span class="timestamp">{formatTime(message.timestamp)}</span>
		</div>
	{:else if message.type === 'assistant'}
		<div class="avatar assistant-avatar">
			<Bot size={18} />
		</div>
		<div class="content">
			<div class="bubble assistant-bubble">
				{message.content}
			</div>
			<span class="timestamp">{formatTime(message.timestamp)}</span>
		</div>
	{:else if message.type === 'tool_use'}
		<div class="tool-card">
			<div class="tool-header">
				<svelte:component this={getToolIcon(message.toolName)} size={16} />
				<span class="tool-name">{message.toolName}</span>
				<span class="tool-status running">En cours...</span>
			</div>
			{#if message.toolInput}
				<pre class="tool-input">{JSON.stringify(message.toolInput, null, 2)}</pre>
			{/if}
		</div>
	{:else if message.type === 'tool_result'}
		<div class="tool-card completed">
			<div class="tool-header">
				<Check size={16} />
				<span class="tool-name">{message.toolName}</span>
				<span class="tool-status completed">Terminé</span>
			</div>
			<div class="tool-result">
				{message.content.slice(0, 200)}{message.content.length > 200 ? '...' : ''}
			</div>
		</div>
	{/if}
</div>

<style>
	.message {
		display: flex;
		gap: 0.75rem;
		max-width: 85%;
	}

	.message.user {
		align-self: flex-end;
		flex-direction: row-reverse;
	}

	.message.assistant {
		align-self: flex-start;
	}

	.message.tool {
		align-self: flex-start;
		max-width: 100%;
	}

	.avatar {
		flex-shrink: 0;
		width: 32px;
		height: 32px;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.user-avatar {
		background-color: var(--color-lion-600);
		color: white;
	}

	.assistant-avatar {
		background-color: var(--color-bg-tertiary);
		color: var(--color-lion-400);
		border: 1px solid var(--color-border);
	}

	.content {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.bubble {
		padding: 0.75rem 1rem;
		border-radius: 16px;
		font-size: 0.9375rem;
		line-height: 1.5;
		white-space: pre-wrap;
		word-break: break-word;
	}

	.user-bubble {
		background-color: var(--color-lion-600);
		color: white;
		border-bottom-right-radius: 4px;
	}

	.assistant-bubble {
		background-color: var(--color-bg-tertiary);
		color: var(--color-text-primary);
		border: 1px solid var(--color-border);
		border-bottom-left-radius: 4px;
	}

	.timestamp {
		font-size: 0.75rem;
		color: var(--color-text-muted);
		padding: 0 0.5rem;
	}

	.message.user .timestamp {
		text-align: right;
	}

	/* Tool cards */
	.tool-card {
		background-color: var(--color-bg-tertiary);
		border: 1px solid var(--color-border);
		border-radius: 8px;
		overflow: hidden;
		width: 100%;
		max-width: 600px;
	}

	.tool-card.completed {
		border-color: var(--color-success);
	}

	.tool-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 0.875rem;
		background-color: var(--color-bg-secondary);
		border-bottom: 1px solid var(--color-border);
		font-size: 0.875rem;
	}

	.tool-name {
		font-weight: 500;
		color: var(--color-text-primary);
	}

	.tool-status {
		margin-left: auto;
		font-size: 0.75rem;
		padding: 0.125rem 0.5rem;
		border-radius: 4px;
	}

	.tool-status.running {
		background-color: var(--color-lion-900);
		color: var(--color-lion-300);
	}

	.tool-status.completed {
		background-color: rgba(34, 197, 94, 0.2);
		color: var(--color-success);
	}

	.tool-input,
	.tool-result {
		padding: 0.75rem;
		font-family: 'JetBrains Mono', 'Fira Code', monospace;
		font-size: 0.8125rem;
		line-height: 1.5;
		margin: 0;
		overflow-x: auto;
		color: var(--color-text-secondary);
	}

	.tool-input {
		background-color: var(--color-bg-primary);
	}
</style>
