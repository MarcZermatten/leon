<script lang="ts">
	import { html } from 'diff2html';
	import 'diff2html/bundles/css/diff2html.min.css';
	import type { DiffPreviewData } from '$lib/types/preview';

	let { data } = $props<{ data: DiffPreviewData | null }>();

	let diffHtml = $state('');
	let viewMode = $state<'split' | 'unified'>('split');

	$effect(() => {
		if (data) {
			const unifiedDiff = createUnifiedDiff(data.oldContent, data.newContent, data.filePath);
			diffHtml = html(unifiedDiff, {
				outputFormat: viewMode === 'split' ? 'side-by-side' : 'line-by-line',
				drawFileList: false,
				matching: 'lines',
				diffStyle: 'word'
			});
		}
	});

	function createUnifiedDiff(oldContent: string, newContent: string, filePath: string): string {
		const oldLines = oldContent.split('\n');
		const newLines = newContent.split('\n');

		let diff = `--- a/${filePath}\n+++ b/${filePath}\n`;

		// Simple diff algorithm (pour une vraie app, utiliser un algo plus robuste)
		const maxLines = Math.max(oldLines.length, newLines.length);
		let hunkStart = -1;
		let hunkLines: string[] = [];

		for (let i = 0; i < maxLines; i++) {
			const oldLine = oldLines[i];
			const newLine = newLines[i];

			if (oldLine !== newLine) {
				if (hunkStart === -1) {
					hunkStart = i;
					// Add context before
					const contextStart = Math.max(0, i - 3);
					for (let j = contextStart; j < i; j++) {
						if (oldLines[j] !== undefined) {
							hunkLines.push(` ${oldLines[j]}`);
						}
					}
				}

				if (oldLine !== undefined && newLine === undefined) {
					hunkLines.push(`-${oldLine}`);
				} else if (oldLine === undefined && newLine !== undefined) {
					hunkLines.push(`+${newLine}`);
				} else if (oldLine !== newLine) {
					if (oldLine !== undefined) hunkLines.push(`-${oldLine}`);
					if (newLine !== undefined) hunkLines.push(`+${newLine}`);
				}
			} else if (hunkStart !== -1) {
				// Add context after
				hunkLines.push(` ${oldLine || ''}`);
				if (hunkLines.filter(l => l.startsWith(' ')).length >= 6) {
					// Flush hunk
					diff += `@@ -${hunkStart + 1},${oldLines.length - hunkStart} +${hunkStart + 1},${newLines.length - hunkStart} @@\n`;
					diff += hunkLines.join('\n') + '\n';
					hunkStart = -1;
					hunkLines = [];
				}
			}
		}

		// Flush remaining hunk
		if (hunkLines.length > 0) {
			diff += `@@ -${hunkStart + 1},${oldLines.length - hunkStart} +${hunkStart + 1},${newLines.length - hunkStart} @@\n`;
			diff += hunkLines.join('\n') + '\n';
		}

		return diff;
	}
</script>

<div class="diff-preview">
	{#if data}
		<div class="diff-toolbar">
			<span class="file-path">{data.filePath}</span>
			<div class="view-toggle">
				<button
					class:active={viewMode === 'split'}
					onclick={() => viewMode = 'split'}
				>
					Split
				</button>
				<button
					class:active={viewMode === 'unified'}
					onclick={() => viewMode = 'unified'}
				>
					Unified
				</button>
			</div>
		</div>
		<div class="diff-content">
			{@html diffHtml}
		</div>
	{:else}
		<div class="empty-state">
			<p>Aucune modification à afficher</p>
		</div>
	{/if}
</div>

<style>
	.diff-preview {
		height: 100%;
		display: flex;
		flex-direction: column;
		background-color: var(--color-bg-primary);
	}

	.diff-toolbar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.5rem 1rem;
		background-color: var(--color-bg-secondary);
		border-bottom: 1px solid var(--color-border);
	}

	.file-path {
		font-family: 'JetBrains Mono', monospace;
		font-size: 0.8125rem;
		color: var(--color-text-secondary);
	}

	.view-toggle {
		display: flex;
		gap: 0.25rem;
	}

	.view-toggle button {
		padding: 0.25rem 0.75rem;
		background-color: var(--color-bg-tertiary);
		border: 1px solid var(--color-border);
		border-radius: 4px;
		color: var(--color-text-secondary);
		font-size: 0.75rem;
		cursor: pointer;
	}

	.view-toggle button:hover {
		background-color: var(--color-bg-hover);
	}

	.view-toggle button.active {
		background-color: var(--color-lion-600);
		border-color: var(--color-lion-600);
		color: white;
	}

	.diff-content {
		flex: 1;
		overflow: auto;
		padding: 1rem;
	}

	.empty-state {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100%;
		color: var(--color-text-muted);
	}

	/* Override diff2html styles for dark theme */
	:global(.d2h-wrapper) {
		background-color: var(--color-bg-primary) !important;
	}

	:global(.d2h-file-header) {
		display: none;
	}

	:global(.d2h-code-linenumber),
	:global(.d2h-code-line) {
		font-family: 'JetBrains Mono', monospace !important;
		font-size: 0.8125rem !important;
	}

	:global(.d2h-del) {
		background-color: rgba(239, 68, 68, 0.2) !important;
	}

	:global(.d2h-ins) {
		background-color: rgba(34, 197, 94, 0.2) !important;
	}

	:global(.d2h-code-side-linenumber) {
		background-color: var(--color-bg-secondary) !important;
		color: var(--color-text-muted) !important;
	}
</style>
