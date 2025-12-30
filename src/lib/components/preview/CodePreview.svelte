<script lang="ts">
	import { onMount } from 'svelte';
	import hljs from 'highlight.js';
	import 'highlight.js/styles/github-dark.css';
	import type { CodePreviewData } from '$lib/types/preview';

	let { data } = $props<{ data: CodePreviewData | null }>();

	let codeElement: HTMLElement;
	let highlightedCode = $state('');

	$effect(() => {
		if (data?.content) {
			try {
				const result = data.language && data.language !== 'plaintext'
					? hljs.highlight(data.content, { language: data.language })
					: hljs.highlightAuto(data.content);
				highlightedCode = result.value;
			} catch {
				highlightedCode = escapeHtml(data.content);
			}
		}
	});

	function escapeHtml(text: string): string {
		return text
			.replace(/&/g, '&amp;')
			.replace(/</g, '&lt;')
			.replace(/>/g, '&gt;')
			.replace(/"/g, '&quot;')
			.replace(/'/g, '&#039;');
	}

	function getLineNumbers(content: string): number[] {
		return content.split('\n').map((_, i) => i + 1);
	}
</script>

<div class="code-preview">
	{#if data}
		<div class="code-container">
			{#if data.lineNumbers !== false}
				<div class="line-numbers">
					{#each getLineNumbers(data.content) as lineNum}
						<span
							class="line-number"
							class:highlighted={data.highlightLines?.includes(lineNum)}
						>{lineNum}</span>
					{/each}
				</div>
			{/if}
			<pre class="code-content"><code bind:this={codeElement} class="hljs">{@html highlightedCode}</code></pre>
		</div>
	{:else}
		<div class="empty-state">
			<p>Aucun fichier sélectionné</p>
		</div>
	{/if}
</div>

<style>
	.code-preview {
		height: 100%;
		overflow: auto;
		background-color: var(--color-bg-primary);
	}

	.code-container {
		display: flex;
		min-height: 100%;
	}

	.line-numbers {
		display: flex;
		flex-direction: column;
		padding: 1rem 0.75rem;
		background-color: var(--color-bg-secondary);
		border-right: 1px solid var(--color-border);
		text-align: right;
		user-select: none;
		position: sticky;
		left: 0;
	}

	.line-number {
		font-family: 'JetBrains Mono', 'Fira Code', monospace;
		font-size: 0.8125rem;
		line-height: 1.6;
		color: var(--color-text-muted);
	}

	.line-number.highlighted {
		color: var(--color-lion-400);
		background-color: var(--color-lion-900);
		padding: 0 0.25rem;
		border-radius: 2px;
	}

	.code-content {
		flex: 1;
		margin: 0;
		padding: 1rem;
		overflow-x: auto;
	}

	.code-content code {
		font-family: 'JetBrains Mono', 'Fira Code', monospace;
		font-size: 0.8125rem;
		line-height: 1.6;
		white-space: pre;
	}

	.empty-state {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100%;
		color: var(--color-text-muted);
	}

	/* Override highlight.js background */
	:global(.hljs) {
		background: transparent !important;
	}
</style>
