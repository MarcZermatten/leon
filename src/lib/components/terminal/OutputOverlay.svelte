<script lang="ts">
	import { AlertCircle, AlertTriangle, FileCode, ExternalLink, X, ChevronDown, ChevronRight } from 'lucide-svelte';
	import {
		analyzeOutput,
		formatFilePath,
		getFileName,
		type FileReference,
		type OutputStats
	} from '$lib/services/outputParser';

	let {
		output = '',
		isVisible = true,
		onFileClick = (path: string, line?: number) => {},
		onUrlClick = (url: string) => {},
		onClose = () => {}
	} = $props<{
		output: string;
		isVisible: boolean;
		onFileClick: (path: string, line?: number) => void;
		onUrlClick: (url: string) => void;
		onClose: () => void;
	}>();

	let stats = $state<OutputStats | null>(null);
	let expanded = $state(true);

	// Analyser la sortie quand elle change
	$effect(() => {
		if (output) {
			stats = analyzeOutput(output);
		}
	});

	function handleFileClick(ref: FileReference) {
		onFileClick(ref.path, ref.line);
	}

	function handleUrlClick(url: string) {
		onUrlClick(url);
	}

	// Ne rien afficher s'il n'y a pas d'erreurs/fichiers
	let hasContent = $derived(
		stats && (stats.errors > 0 || stats.warnings > 0 || stats.files.length > 0)
	);
</script>

{#if isVisible && hasContent && stats}
	<div class="output-overlay" class:collapsed={!expanded}>
		<div class="header">
			<button class="header-toggle" onclick={() => (expanded = !expanded)}>
				{#if expanded}
					<ChevronDown size={14} />
				{:else}
					<ChevronRight size={14} />
				{/if}
				<span class="title">Output Analysis</span>
			</button>
			<div class="stats">
				{#if stats.errors > 0}
					<span class="stat error">
						<AlertCircle size={12} />
						{stats.errors}
					</span>
				{/if}
				{#if stats.warnings > 0}
					<span class="stat warning">
						<AlertTriangle size={12} />
						{stats.warnings}
					</span>
				{/if}
				{#if stats.files.length > 0}
					<span class="stat files">
						<FileCode size={12} />
						{stats.files.length}
					</span>
				{/if}
			</div>
			<button class="close-btn" onclick={onClose}>
				<X size={12} />
			</button>
		</div>

		{#if expanded}
			<div class="content">
				{#if stats.files.length > 0}
					<div class="section">
						<div class="section-title">Fichiers référencés</div>
						<div class="file-list">
							{#each stats.files.slice(0, 10) as ref}
								<button class="file-item" onclick={() => handleFileClick(ref)}>
									<FileCode size={12} />
									<span class="file-name">{getFileName(ref.path)}</span>
									{#if ref.line}
										<span class="file-location">:{ref.line}</span>
									{/if}
								</button>
							{/each}
							{#if stats.files.length > 10}
								<span class="more">+{stats.files.length - 10} autres</span>
							{/if}
						</div>
					</div>
				{/if}

				{#if stats.urls.length > 0}
					<div class="section">
						<div class="section-title">URLs</div>
						<div class="url-list">
							{#each stats.urls.slice(0, 5) as url}
								<button class="url-item" onclick={() => handleUrlClick(url)}>
									<ExternalLink size={12} />
									<span class="url-text">{url}</span>
								</button>
							{/each}
						</div>
					</div>
				{/if}
			</div>
		{/if}
	</div>
{/if}

<style>
	.output-overlay {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		background: var(--color-bg-secondary);
		border-top: 1px solid var(--color-border);
		z-index: 10;
		max-height: 200px;
		overflow: hidden;
	}

	.output-overlay.collapsed {
		max-height: 32px;
	}

	.header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.375rem 0.75rem;
		background: var(--color-bg-tertiary);
		color: var(--color-text-secondary);
	}

	.header-toggle {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		background: transparent;
		border: none;
		color: inherit;
		cursor: pointer;
		padding: 0;
	}

	.header-toggle:hover {
		color: var(--color-text-primary);
	}

	.title {
		font-size: 0.7rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.stats {
		display: flex;
		gap: 0.5rem;
	}

	.stat {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.125rem 0.375rem;
		border-radius: 4px;
		font-size: 0.7rem;
		font-weight: 500;
	}

	.stat.error {
		background: rgba(255, 107, 107, 0.2);
		color: var(--color-error);
	}

	.stat.warning {
		background: rgba(255, 169, 77, 0.2);
		color: var(--color-warning);
	}

	.stat.files {
		background: rgba(116, 192, 252, 0.2);
		color: var(--color-info);
	}

	.close-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 20px;
		height: 20px;
		background: transparent;
		border: none;
		border-radius: 4px;
		color: var(--color-text-muted);
		cursor: pointer;
	}

	.close-btn:hover {
		background: var(--color-bg-hover);
		color: var(--color-text-primary);
	}

	.content {
		padding: 0.5rem 0.75rem;
		overflow-y: auto;
		max-height: 160px;
	}

	.section {
		margin-bottom: 0.5rem;
	}

	.section:last-child {
		margin-bottom: 0;
	}

	.section-title {
		font-size: 0.65rem;
		font-weight: 600;
		color: var(--color-text-muted);
		text-transform: uppercase;
		margin-bottom: 0.25rem;
	}

	.file-list,
	.url-list {
		display: flex;
		flex-wrap: wrap;
		gap: 0.25rem;
	}

	.file-item,
	.url-item {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.25rem 0.5rem;
		background: var(--color-bg-primary);
		border: 1px solid var(--color-border);
		border-radius: 4px;
		color: var(--color-text-secondary);
		font-size: 0.7rem;
		cursor: pointer;
	}

	.file-item:hover,
	.url-item:hover {
		background: var(--color-bg-hover);
		border-color: var(--color-lion-500);
		color: var(--color-lion-400);
	}

	.file-name {
		font-family: 'JetBrains Mono', monospace;
	}

	.file-location {
		color: var(--color-text-muted);
	}

	.url-text {
		max-width: 200px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.more {
		font-size: 0.65rem;
		color: var(--color-text-muted);
		padding: 0.25rem;
	}
</style>
