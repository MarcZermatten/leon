<script lang="ts">
	import {
		Folder,
		FolderOpen,
		File,
		FileCode,
		FileText,
		FileJson,
		Image,
		ChevronRight,
		ChevronDown,
		RefreshCw,
		Eye,
		EyeOff,
		X
	} from 'lucide-svelte';
	import {
		listDirectory,
		formatFileSize,
		type FileEntry
	} from '$lib/services/files';

	let {
		projectPath = null,
		isVisible = true,
		onClose = () => {},
		onFileSelect = (path: string) => {}
	} = $props<{
		projectPath: string | null;
		isVisible: boolean;
		onClose: () => void;
		onFileSelect: (path: string) => void;
	}>();

	let files = $state<FileEntry[]>([]);
	let expandedDirs = $state<Set<string>>(new Set());
	let selectedFile = $state<string | null>(null);
	let showHidden = $state(false);
	let loading = $state(false);

	// Charger les fichiers quand le projet change
	$effect(() => {
		if (projectPath && isVisible) {
			loadFiles();
		}
	});

	async function loadFiles() {
		if (!projectPath) return;
		loading = true;
		try {
			files = await listDirectory(projectPath, showHidden, 1);
		} finally {
			loading = false;
		}
	}

	async function toggleDir(entry: FileEntry) {
		if (!entry.is_dir) return;

		if (expandedDirs.has(entry.path)) {
			expandedDirs.delete(entry.path);
			expandedDirs = new Set(expandedDirs);
		} else {
			// Charger les enfants si nécessaire
			if (!entry.children || entry.children.length === 0) {
				const children = await listDirectory(entry.path, showHidden, 1);
				entry.children = children;
				files = [...files]; // Trigger reactivity
			}
			expandedDirs.add(entry.path);
			expandedDirs = new Set(expandedDirs);
		}
	}

	function handleFileClick(entry: FileEntry) {
		if (entry.is_dir) {
			toggleDir(entry);
		} else {
			selectedFile = entry.path;
			onFileSelect(entry.path);
		}
	}

	function toggleHidden() {
		showHidden = !showHidden;
		loadFiles();
	}

	function getFileIcon(entry: FileEntry) {
		if (entry.is_dir) {
			return expandedDirs.has(entry.path) ? FolderOpen : Folder;
		}

		const ext = entry.extension?.toLowerCase();
		switch (ext) {
			case 'ts':
			case 'tsx':
			case 'js':
			case 'jsx':
			case 'svelte':
			case 'vue':
			case 'rs':
			case 'py':
			case 'go':
				return FileCode;
			case 'json':
				return FileJson;
			case 'png':
			case 'jpg':
			case 'jpeg':
			case 'gif':
			case 'svg':
				return Image;
			case 'md':
			case 'txt':
				return FileText;
			default:
				return File;
		}
	}

	function getFileColor(entry: FileEntry): string {
		if (entry.is_dir) return 'var(--color-lion-400)';

		const ext = entry.extension?.toLowerCase();
		const colorMap: Record<string, string> = {
			ts: '#3178c6',
			tsx: '#3178c6',
			js: '#f7df1e',
			jsx: '#f7df1e',
			svelte: '#ff3e00',
			vue: '#42b883',
			rs: '#dea584',
			py: '#3776ab',
			go: '#00add8',
			json: '#cbcb41',
			md: '#519aba',
			css: '#563d7c',
			scss: '#c6538c',
			html: '#e34f26'
		};

		return colorMap[ext || ''] || 'var(--color-text-secondary)';
	}
</script>

{#if isVisible}
	<div class="file-explorer">
		<div class="explorer-header">
			<div class="header-title">
				<Folder size={16} />
				<span>Explorer</span>
			</div>
			<div class="header-actions">
				<button
					class="icon-btn"
					onclick={toggleHidden}
					title={showHidden ? 'Cacher fichiers cachés' : 'Afficher fichiers cachés'}
				>
					{#if showHidden}
						<Eye size={14} />
					{:else}
						<EyeOff size={14} />
					{/if}
				</button>
				<button class="icon-btn" onclick={loadFiles} title="Rafraîchir" disabled={loading}>
					<RefreshCw size={14} class={loading ? 'spinning' : ''} />
				</button>
				<button class="icon-btn" onclick={onClose} title="Fermer">
					<X size={14} />
				</button>
			</div>
		</div>

		<div class="explorer-content">
			{#if loading && files.length === 0}
				<div class="loading">Chargement...</div>
			{:else if files.length === 0}
				<div class="empty">Aucun fichier</div>
			{:else}
				<div class="file-tree">
					{#each files as entry (entry.path)}
						{@render fileItem(entry, 0)}
					{/each}
				</div>
			{/if}
		</div>
	</div>
{/if}

{#snippet fileItem(entry: FileEntry, depth: number)}
	<div class="tree-item" style="padding-left: {depth * 12 + 8}px">
		<button
			class="item-row"
			class:selected={selectedFile === entry.path}
			class:directory={entry.is_dir}
			onclick={() => handleFileClick(entry)}
		>
			{#if entry.is_dir}
				<span class="expand-icon">
					{#if expandedDirs.has(entry.path)}
						<ChevronDown size={12} />
					{:else}
						<ChevronRight size={12} />
					{/if}
				</span>
			{:else}
				<span class="expand-icon"></span>
			{/if}

			<span class="file-icon" style="color: {getFileColor(entry)}">
				<svelte:component this={getFileIcon(entry)} size={14} />
			</span>

			<span class="file-name" class:hidden={entry.is_hidden}>
				{entry.name}
			</span>

			{#if !entry.is_dir && entry.size}
				<span class="file-size">{formatFileSize(entry.size)}</span>
			{/if}
		</button>

		{#if entry.is_dir && expandedDirs.has(entry.path) && entry.children}
			{#each entry.children as child (child.path)}
				{@render fileItem(child, depth + 1)}
			{/each}
		{/if}
	</div>
{/snippet}

<style>
	.file-explorer {
		display: flex;
		flex-direction: column;
		height: 100%;
		background: var(--color-bg-secondary);
		border-left: 1px solid var(--color-border);
	}

	.explorer-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.5rem 0.75rem;
		border-bottom: 1px solid var(--color-border);
		background: var(--color-bg-tertiary);
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.8rem;
		font-weight: 600;
		color: var(--color-text-primary);
	}

	.header-actions {
		display: flex;
		gap: 0.25rem;
	}

	.icon-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		background: transparent;
		border: none;
		border-radius: 4px;
		color: var(--color-text-secondary);
		cursor: pointer;
	}

	.icon-btn:hover {
		background: var(--color-bg-hover);
		color: var(--color-text-primary);
	}

	.icon-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.icon-btn :global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	.explorer-content {
		flex: 1;
		overflow-y: auto;
	}

	.loading,
	.empty {
		padding: 2rem;
		text-align: center;
		font-size: 0.8rem;
		color: var(--color-text-muted);
	}

	.file-tree {
		padding: 0.25rem 0;
	}

	.tree-item {
		display: flex;
		flex-direction: column;
	}

	.item-row {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		width: 100%;
		padding: 0.25rem 0.5rem;
		background: transparent;
		border: none;
		border-radius: 0;
		color: var(--color-text-primary);
		font-size: 0.75rem;
		cursor: pointer;
		text-align: left;
	}

	.item-row:hover {
		background: var(--color-bg-hover);
	}

	.item-row.selected {
		background: var(--color-lion-900);
		color: var(--color-lion-300);
	}

	.expand-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 12px;
		color: var(--color-text-muted);
	}

	.file-icon {
		display: flex;
		align-items: center;
		flex-shrink: 0;
	}

	.file-name {
		flex: 1;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.file-name.hidden {
		opacity: 0.6;
	}

	.file-size {
		flex-shrink: 0;
		font-size: 0.65rem;
		color: var(--color-text-muted);
		font-family: 'JetBrains Mono', monospace;
	}
</style>
