<script lang="ts">
	import {
		GitBranch,
		GitCommit,
		Plus,
		Minus,
		RotateCcw,
		Upload,
		Download,
		ChevronDown,
		ChevronRight,
		FileText,
		X,
		Check
	} from 'lucide-svelte';
	import {
		getGitStatus,
		getGitCommits,
		getGitBranches,
		getGitDiff,
		gitStageFile,
		gitUnstageFile,
		gitDiscardFile,
		gitCommit,
		gitPush,
		gitPull,
		formatFileStatus,
		getStatusColor,
		type GitStatus,
		type GitCommit as GitCommitType,
		type GitBranch as GitBranchType,
		type GitDiff
	} from '$lib/services/git';

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

	let status = $state<GitStatus | null>(null);
	let commits = $state<GitCommitType[]>([]);
	let branches = $state<GitBranchType[]>([]);
	let selectedDiff = $state<GitDiff[] | null>(null);
	let selectedFile = $state<string | null>(null);
	let commitMessage = $state('');
	let loading = $state(false);

	// Sections collapsibles
	let showStaged = $state(true);
	let showUnstaged = $state(true);
	let showUntracked = $state(true);
	let showCommits = $state(false);
	let showBranches = $state(false);

	// Charger les données Git quand le projet change
	$effect(() => {
		if (projectPath && isVisible) {
			refreshAll();
		}
	});

	async function refreshAll() {
		loading = true;
		try {
			await Promise.all([refreshStatus(), refreshCommits(), refreshBranches()]);
		} finally {
			loading = false;
		}
	}

	async function refreshStatus() {
		if (!projectPath) return;
		status = await getGitStatus(projectPath);
	}

	async function refreshCommits() {
		if (!projectPath) return;
		commits = await getGitCommits(projectPath, 10);
	}

	async function refreshBranches() {
		if (!projectPath) return;
		branches = await getGitBranches(projectPath);
	}

	async function handleStageFile(path: string) {
		if (!projectPath) return;
		await gitStageFile(projectPath, path);
		await refreshStatus();
	}

	async function handleUnstageFile(path: string) {
		if (!projectPath) return;
		await gitUnstageFile(projectPath, path);
		await refreshStatus();
	}

	async function handleDiscardFile(path: string) {
		if (!projectPath || !confirm(`Discard changes to ${path}?`)) return;
		await gitDiscardFile(projectPath, path);
		await refreshStatus();
	}

	async function handleStageAll() {
		if (!projectPath || !status) return;
		for (const file of status.unstaged) {
			await gitStageFile(projectPath, file.path);
		}
		for (const file of status.untracked) {
			await gitStageFile(projectPath, file);
		}
		await refreshStatus();
	}

	async function handleCommit() {
		if (!projectPath || !commitMessage.trim()) return;
		const success = await gitCommit(projectPath, commitMessage);
		if (success) {
			commitMessage = '';
			await refreshAll();
		}
	}

	async function handlePush() {
		if (!projectPath) return;
		loading = true;
		await gitPush(projectPath);
		await refreshStatus();
		loading = false;
	}

	async function handlePull() {
		if (!projectPath) return;
		loading = true;
		await gitPull(projectPath);
		await refreshAll();
		loading = false;
	}

	async function showFileDiff(path: string, staged: boolean) {
		if (!projectPath) return;
		selectedFile = path;
		selectedDiff = await getGitDiff(projectPath, path, staged);
	}

	function closeDiff() {
		selectedFile = null;
		selectedDiff = null;
	}

	// Computed
	let totalChanges = $derived(
		(status?.staged.length || 0) + (status?.unstaged.length || 0) + (status?.untracked.length || 0)
	);

	let canCommit = $derived((status?.staged.length || 0) > 0 && commitMessage.trim().length > 0);
</script>

{#if isVisible}
	<div class="git-panel">
		<div class="panel-header">
			<div class="header-title">
				<GitBranch size={16} />
				<span>Git</span>
				{#if status}
					<span class="branch-name">{status.branch}</span>
					{#if status.ahead > 0}
						<span class="badge ahead" title="Commits ahead">+{status.ahead}</span>
					{/if}
					{#if status.behind > 0}
						<span class="badge behind" title="Commits behind">-{status.behind}</span>
					{/if}
				{/if}
			</div>
			<div class="header-actions">
				<button class="icon-btn" onclick={handlePull} title="Pull" disabled={loading}>
					<Download size={14} />
				</button>
				<button class="icon-btn" onclick={handlePush} title="Push" disabled={loading}>
					<Upload size={14} />
				</button>
				<button class="icon-btn" onclick={refreshAll} title="Refresh" disabled={loading}>
					<RotateCcw size={14} class={loading ? 'spinning' : ''} />
				</button>
				<button class="icon-btn" onclick={onClose} title="Close">
					<X size={14} />
				</button>
			</div>
		</div>

		{#if selectedDiff}
			<!-- Diff Viewer -->
			<div class="diff-viewer">
				<div class="diff-header">
					<span class="diff-file">{selectedFile}</span>
					<button class="icon-btn" onclick={closeDiff}>
						<X size={14} />
					</button>
				</div>
				<div class="diff-content">
					{#each selectedDiff as diff}
						{#each diff.hunks as hunk}
							<div class="hunk-header">{hunk.header}</div>
							{#each hunk.lines as line}
								<div class="diff-line {line.line_type}">
									<span class="line-number old">{line.old_line ?? ''}</span>
									<span class="line-number new">{line.new_line ?? ''}</span>
									<span class="line-content">{line.content}</span>
								</div>
							{/each}
						{/each}
					{/each}
				</div>
			</div>
		{:else}
			<div class="panel-content">
				<!-- Staged Changes -->
				{#if status && status.staged.length > 0}
					<div class="section">
						<button class="section-header" onclick={() => (showStaged = !showStaged)}>
							{#if showStaged}
								<ChevronDown size={14} />
							{:else}
								<ChevronRight size={14} />
							{/if}
							<span>Staged Changes</span>
							<span class="count">{status.staged.length}</span>
						</button>
						{#if showStaged}
							<div class="file-list">
								{#each status.staged as file}
									<div class="file-item staged">
										<button
											class="file-name"
											onclick={() => showFileDiff(file.path, true)}
										>
											<span class="status" style="color: {getStatusColor(file.status)}"
												>{file.status}</span
											>
											<span>{file.path}</span>
										</button>
										<div class="file-actions">
											<button
												class="icon-btn tiny"
												onclick={() => handleUnstageFile(file.path)}
												title="Unstage"
											>
												<Minus size={12} />
											</button>
										</div>
									</div>
								{/each}
							</div>
						{/if}
					</div>
				{/if}

				<!-- Unstaged Changes -->
				{#if status && status.unstaged.length > 0}
					<div class="section">
						<button class="section-header" onclick={() => (showUnstaged = !showUnstaged)}>
							{#if showUnstaged}
								<ChevronDown size={14} />
							{:else}
								<ChevronRight size={14} />
							{/if}
							<span>Changes</span>
							<span class="count">{status.unstaged.length}</span>
						</button>
						{#if showUnstaged}
							<div class="file-list">
								{#each status.unstaged as file}
									<div class="file-item">
										<button
											class="file-name"
											onclick={() => showFileDiff(file.path, false)}
										>
											<span class="status" style="color: {getStatusColor(file.status)}"
												>{file.status}</span
											>
											<span>{file.path}</span>
										</button>
										<div class="file-actions">
											<button
												class="icon-btn tiny"
												onclick={() => handleStageFile(file.path)}
												title="Stage"
											>
												<Plus size={12} />
											</button>
											<button
												class="icon-btn tiny danger"
												onclick={() => handleDiscardFile(file.path)}
												title="Discard"
											>
												<X size={12} />
											</button>
										</div>
									</div>
								{/each}
							</div>
						{/if}
					</div>
				{/if}

				<!-- Untracked Files -->
				{#if status && status.untracked.length > 0}
					<div class="section">
						<button class="section-header" onclick={() => (showUntracked = !showUntracked)}>
							{#if showUntracked}
								<ChevronDown size={14} />
							{:else}
								<ChevronRight size={14} />
							{/if}
							<span>Untracked</span>
							<span class="count">{status.untracked.length}</span>
						</button>
						{#if showUntracked}
							<div class="file-list">
								{#each status.untracked as file}
									<div class="file-item untracked">
										<button class="file-name" onclick={() => onFileSelect(file)}>
											<span class="status" style="color: var(--color-success)">?</span>
											<span>{file}</span>
										</button>
										<div class="file-actions">
											<button
												class="icon-btn tiny"
												onclick={() => handleStageFile(file)}
												title="Stage"
											>
												<Plus size={12} />
											</button>
										</div>
									</div>
								{/each}
							</div>
						{/if}
					</div>
				{/if}

				<!-- Commit Box -->
				{#if status && (status.staged.length > 0 || status.unstaged.length > 0 || status.untracked.length > 0)}
					<div class="commit-box">
						<div class="commit-actions-row">
							<button class="stage-all-btn" onclick={handleStageAll}>
								<Plus size={12} />
								Stage All
							</button>
						</div>
						<input
							type="text"
							class="commit-input"
							placeholder="Commit message..."
							bind:value={commitMessage}
							onkeydown={(e) => e.key === 'Enter' && canCommit && handleCommit()}
						/>
						<button class="commit-btn" onclick={handleCommit} disabled={!canCommit}>
							<Check size={14} />
							Commit
						</button>
					</div>
				{/if}

				<!-- Recent Commits -->
				<div class="section">
					<button class="section-header" onclick={() => (showCommits = !showCommits)}>
						{#if showCommits}
							<ChevronDown size={14} />
						{:else}
							<ChevronRight size={14} />
						{/if}
						<GitCommit size={14} />
						<span>Recent Commits</span>
					</button>
					{#if showCommits}
						<div class="commits-list">
							{#each commits as commit}
								<div class="commit-item">
									<span class="commit-hash">{commit.short_hash}</span>
									<span class="commit-message">{commit.message}</span>
									<span class="commit-date">{commit.relative_date}</span>
								</div>
							{/each}
						</div>
					{/if}
				</div>

				<!-- Branches -->
				<div class="section">
					<button class="section-header" onclick={() => (showBranches = !showBranches)}>
						{#if showBranches}
							<ChevronDown size={14} />
						{:else}
							<ChevronRight size={14} />
						{/if}
						<GitBranch size={14} />
						<span>Branches</span>
					</button>
					{#if showBranches}
						<div class="branches-list">
							{#each branches.filter((b) => !b.is_remote) as branch}
								<div class="branch-item" class:current={branch.is_current}>
									{#if branch.is_current}
										<Check size={12} />
									{/if}
									<span>{branch.name}</span>
								</div>
							{/each}
						</div>
					{/if}
				</div>

				<!-- Empty State -->
				{#if !status || totalChanges === 0}
					<div class="empty-state">
						<Check size={24} />
						<span>Working tree clean</span>
					</div>
				{/if}
			</div>
		{/if}
	</div>
{/if}

<style>
	.git-panel {
		display: flex;
		flex-direction: column;
		height: 100%;
		background: var(--color-bg-secondary);
		border-left: 1px solid var(--color-border);
	}

	.panel-header {
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

	.branch-name {
		font-weight: 400;
		color: var(--color-lion-400);
	}

	.badge {
		padding: 0.125rem 0.375rem;
		border-radius: 4px;
		font-size: 0.65rem;
		font-weight: 600;
	}

	.badge.ahead {
		background: var(--color-success);
		color: white;
	}

	.badge.behind {
		background: var(--color-warning);
		color: black;
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

	.icon-btn.tiny {
		width: 20px;
		height: 20px;
	}

	.icon-btn.danger:hover {
		background: rgba(255, 107, 107, 0.2);
		color: var(--color-error);
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

	.panel-content {
		flex: 1;
		overflow-y: auto;
		padding: 0.5rem;
	}

	.section {
		margin-bottom: 0.5rem;
	}

	.section-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.375rem 0.5rem;
		background: transparent;
		border: none;
		border-radius: 4px;
		color: var(--color-text-secondary);
		font-size: 0.75rem;
		font-weight: 600;
		cursor: pointer;
		text-align: left;
	}

	.section-header:hover {
		background: var(--color-bg-hover);
	}

	.count {
		margin-left: auto;
		padding: 0.125rem 0.375rem;
		background: var(--color-bg-hover);
		border-radius: 4px;
		font-size: 0.65rem;
	}

	.file-list {
		display: flex;
		flex-direction: column;
		gap: 2px;
		padding-left: 1rem;
	}

	.file-item {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.25rem 0.5rem;
		border-radius: 4px;
	}

	.file-item:hover {
		background: var(--color-bg-hover);
	}

	.file-name {
		flex: 1;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		background: transparent;
		border: none;
		color: var(--color-text-primary);
		font-size: 0.75rem;
		font-family: 'JetBrains Mono', monospace;
		cursor: pointer;
		text-align: left;
	}

	.file-name:hover {
		color: var(--color-lion-400);
	}

	.status {
		font-weight: 600;
		min-width: 1rem;
	}

	.file-actions {
		display: flex;
		gap: 2px;
		opacity: 0;
	}

	.file-item:hover .file-actions {
		opacity: 1;
	}

	.commit-box {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		padding: 0.75rem;
		margin-top: 0.5rem;
		background: var(--color-bg-tertiary);
		border-radius: 6px;
	}

	.commit-actions-row {
		display: flex;
		justify-content: flex-end;
	}

	.stage-all-btn {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.25rem 0.5rem;
		background: transparent;
		border: 1px solid var(--color-border);
		border-radius: 4px;
		color: var(--color-text-secondary);
		font-size: 0.7rem;
		cursor: pointer;
	}

	.stage-all-btn:hover {
		background: var(--color-bg-hover);
		color: var(--color-text-primary);
	}

	.commit-input {
		padding: 0.5rem;
		background: var(--color-bg-primary);
		border: 1px solid var(--color-border);
		border-radius: 4px;
		color: var(--color-text-primary);
		font-size: 0.8rem;
	}

	.commit-input:focus {
		outline: none;
		border-color: var(--color-lion-500);
	}

	.commit-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.5rem;
		background: var(--color-lion-600);
		border: none;
		border-radius: 4px;
		color: white;
		font-size: 0.8rem;
		font-weight: 500;
		cursor: pointer;
	}

	.commit-btn:hover:not(:disabled) {
		background: var(--color-lion-500);
	}

	.commit-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.commits-list,
	.branches-list {
		display: flex;
		flex-direction: column;
		gap: 2px;
		padding-left: 1rem;
	}

	.commit-item {
		display: grid;
		grid-template-columns: auto 1fr auto;
		gap: 0.5rem;
		padding: 0.375rem 0.5rem;
		border-radius: 4px;
		font-size: 0.75rem;
	}

	.commit-item:hover {
		background: var(--color-bg-hover);
	}

	.commit-hash {
		font-family: 'JetBrains Mono', monospace;
		color: var(--color-lion-400);
	}

	.commit-message {
		color: var(--color-text-primary);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.commit-date {
		color: var(--color-text-muted);
		font-size: 0.65rem;
	}

	.branch-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.375rem 0.5rem;
		border-radius: 4px;
		font-size: 0.75rem;
		color: var(--color-text-secondary);
	}

	.branch-item:hover {
		background: var(--color-bg-hover);
	}

	.branch-item.current {
		color: var(--color-lion-400);
		font-weight: 500;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
		padding: 2rem;
		color: var(--color-text-muted);
		font-size: 0.8rem;
	}

	/* Diff Viewer */
	.diff-viewer {
		flex: 1;
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.diff-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.5rem 0.75rem;
		border-bottom: 1px solid var(--color-border);
		background: var(--color-bg-tertiary);
	}

	.diff-file {
		font-family: 'JetBrains Mono', monospace;
		font-size: 0.75rem;
		color: var(--color-text-primary);
	}

	.diff-content {
		flex: 1;
		overflow-y: auto;
		font-family: 'JetBrains Mono', monospace;
		font-size: 0.75rem;
	}

	.hunk-header {
		padding: 0.25rem 0.5rem;
		background: var(--color-bg-tertiary);
		color: var(--color-text-muted);
		border-bottom: 1px solid var(--color-border);
	}

	.diff-line {
		display: flex;
		padding: 0 0.5rem;
		min-height: 1.4em;
	}

	.diff-line.add {
		background: rgba(105, 219, 124, 0.15);
	}

	.diff-line.remove {
		background: rgba(255, 107, 107, 0.15);
	}

	.line-number {
		min-width: 3rem;
		padding-right: 0.5rem;
		color: var(--color-text-muted);
		text-align: right;
		user-select: none;
	}

	.line-content {
		flex: 1;
		white-space: pre;
	}

	.diff-line.add .line-content {
		color: var(--color-success);
	}

	.diff-line.remove .line-content {
		color: var(--color-error);
	}
</style>
