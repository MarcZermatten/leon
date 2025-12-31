<script lang="ts">
	import { Plus, FolderOpen, MessageSquare, Settings, ChevronRight, ChevronLeft, Save, Rocket, GitBranch, Files, Zap, PanelRightClose, PanelRightOpen } from 'lucide-svelte';

	interface Session {
		id: string;
		name: string;
		project: string;
		timestamp: Date;
	}

	let {
		sessions = [],
		activeSession = null,
		onNewProject = () => {},
		onOpenProject = () => {},
		onSelectSession = (id: string) => {},
		onOpenSettings = () => {},
		onSave = () => {},
		onRelease = () => {},
		onToggleGit = () => {},
		onToggleFiles = () => {},
		onToggleSnippets = () => {},
		onTogglePreview = () => {},
		onToggleSidebar = () => {},
		hasActiveProject = false,
		gitChanges = 0,
		isPreviewOpen = false
	} = $props<{
		sessions: Session[];
		activeSession: string | null;
		onNewProject: () => void;
		onOpenProject: () => void;
		onSelectSession: (id: string) => void;
		onOpenSettings: () => void;
		onSave: () => void;
		onRelease: () => void;
		onToggleGit: () => void;
		onToggleFiles: () => void;
		onToggleSnippets: () => void;
		onTogglePreview: () => void;
		onToggleSidebar: () => void;
		hasActiveProject: boolean;
		gitChanges: number;
		isPreviewOpen: boolean;
	}>();
</script>

<aside class="sidebar">
	<div class="sidebar-header">
		<img src="/images/logo.png" alt="Léon" class="logo" />
	</div>

	<div class="sidebar-actions">
		<button class="new-project-btn" onclick={onNewProject}>
			<Plus size={18} />
			<span>Nouveau projet</span>
		</button>
		<button class="open-project-btn" onclick={onOpenProject}>
			<FolderOpen size={18} />
			<span>Ouvrir un projet</span>
		</button>
	</div>

	<nav class="sidebar-nav">
		<div class="nav-section">
			<div class="nav-section-header">
				<FolderOpen size={14} />
				<span>Projets récents</span>
			</div>
			<ul class="session-list">
				{#if sessions.length === 0}
					<li class="empty-state">Aucune session</li>
				{:else}
					{#each sessions as session (session.id)}
						<li>
							<button
								class="session-item"
								class:active={activeSession === session.id}
								onclick={() => onSelectSession(session.id)}
							>
								<MessageSquare size={14} />
								<span class="session-name">{session.name}</span>
								<ChevronRight size={14} class="chevron" />
							</button>
						</li>
					{/each}
				{/if}
			</ul>
		</div>
	</nav>

	<div class="sidebar-footer">
		{#if hasActiveProject}
			<div class="action-buttons">
				<button class="action-btn files-btn" onclick={onToggleFiles} title="Afficher l'explorateur de fichiers">
					<Files size={18} />
					<span>Files</span>
				</button>
				<button class="action-btn git-btn" onclick={onToggleGit} title="Afficher le panneau Git">
					<GitBranch size={18} />
					<span>Git</span>
					{#if gitChanges > 0}
						<span class="git-badge">{gitChanges}</span>
					{/if}
				</button>
				<button class="action-btn snippets-btn" onclick={onToggleSnippets} title="Afficher les snippets (Ctrl+Shift+S)">
					<Zap size={18} />
					<span>Snippets</span>
				</button>
			</div>
			<div class="action-buttons">
				<button class="action-btn save-btn" onclick={onSave} title="Sauvegarder et pousser sur GitHub">
					<Save size={18} />
					<span>Sauver</span>
				</button>
				<button class="action-btn release-btn" onclick={onRelease} title="Créer et pousser une release">
					<Rocket size={18} />
					<span>Release</span>
				</button>
			</div>
		{/if}
		<div class="toggle-buttons">
			<button
				class="toggle-btn"
				class:active={isPreviewOpen}
				onclick={onTogglePreview}
				title="Afficher/masquer le preview (Ctrl+P)"
			>
				{#if isPreviewOpen}
					<PanelRightClose size={18} />
				{:else}
					<PanelRightOpen size={18} />
				{/if}
				<span>Preview</span>
			</button>
			<button
				class="toggle-btn collapse-btn"
				onclick={onToggleSidebar}
				title="Masquer la sidebar (Ctrl+B)"
			>
				<ChevronLeft size={18} />
			</button>
		</div>
		<button class="settings-btn" onclick={onOpenSettings}>
			<Settings size={18} />
			<span>Paramètres</span>
		</button>
	</div>
</aside>

<style>
	.sidebar {
		display: flex;
		flex-direction: column;
		height: 100%;
		background-color: var(--color-bg-secondary);
		border-right: 1px solid var(--color-border);
	}

	.sidebar-header {
		padding: 1rem;
		border-bottom: 1px solid var(--color-border);
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.logo {
		height: 80px;
		width: auto;
	}

	.sidebar-actions {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		padding: 1rem;
	}

	.new-project-btn,
	.open-project-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		color: var(--color-text-primary);
		border: none;
		border-radius: 8px;
		cursor: pointer;
		font-weight: 500;
		font-size: 0.875rem;
	}

	.new-project-btn {
		background-color: var(--color-lion-600);
	}

	.new-project-btn:hover {
		background-color: var(--color-lion-500);
	}

	.open-project-btn {
		background-color: var(--color-bg-hover);
		border: 1px solid var(--color-border);
	}

	.open-project-btn:hover {
		background-color: var(--color-lion-900);
		border-color: var(--color-lion-600);
	}

	.sidebar-nav {
		flex: 1;
		overflow-y: auto;
		padding: 0.5rem;
	}

	.nav-section-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem;
		color: var(--color-text-secondary);
		font-size: 0.75rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.session-list {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.empty-state {
		padding: 1rem;
		color: var(--color-text-muted);
		font-size: 0.875rem;
		text-align: center;
	}

	.session-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.625rem 0.75rem;
		background: none;
		border: none;
		border-radius: 6px;
		color: var(--color-text-secondary);
		cursor: pointer;
		text-align: left;
	}

	.session-item:hover {
		background-color: var(--color-bg-hover);
		color: var(--color-text-primary);
	}

	.session-item.active {
		background-color: var(--color-lion-900);
		color: var(--color-lion-300);
	}

	.session-name {
		flex: 1;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		font-size: 0.875rem;
	}

	.chevron {
		opacity: 0;
		transition: opacity 0.15s;
	}

	.session-item:hover .chevron,
	.session-item.active .chevron {
		opacity: 1;
	}

	.sidebar-footer {
		padding: 0.5rem;
		border-top: 1px solid var(--color-border);
	}

	.settings-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.75rem;
		background: none;
		border: none;
		border-radius: 6px;
		color: var(--color-text-secondary);
		cursor: pointer;
	}

	.settings-btn:hover {
		background-color: var(--color-bg-hover);
		color: var(--color-text-primary);
	}

	.action-buttons {
		display: flex;
		gap: 0.5rem;
		padding: 0.5rem;
		border-bottom: 1px solid var(--color-border);
	}

	.action-btn {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		padding: 0.5rem;
		background: none;
		border: 1px solid var(--color-border);
		border-radius: 6px;
		color: var(--color-text-secondary);
		cursor: pointer;
		font-size: 0.75rem;
		transition: all 0.15s;
	}

	.action-btn:hover {
		background-color: var(--color-bg-hover);
		color: var(--color-text-primary);
	}

	.save-btn:hover {
		border-color: var(--color-success, #69db7c);
		color: var(--color-success, #69db7c);
	}

	.release-btn:hover {
		border-color: var(--color-lion-500);
		color: var(--color-lion-400);
	}

	.files-btn:hover {
		border-color: var(--color-lion-500);
		color: var(--color-lion-400);
	}

	.snippets-btn:hover {
		border-color: var(--color-warning, #ffa94d);
		color: var(--color-warning, #ffa94d);
	}

	.git-btn {
		position: relative;
	}

	.git-btn:hover {
		border-color: var(--color-info, #74c0fc);
		color: var(--color-info, #74c0fc);
	}

	.git-badge {
		position: absolute;
		top: -4px;
		right: -4px;
		min-width: 16px;
		height: 16px;
		padding: 0 4px;
		background: var(--color-warning, #ffa94d);
		border-radius: 8px;
		font-size: 0.6rem;
		font-weight: 600;
		color: black;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.toggle-buttons {
		display: flex;
		gap: 0.5rem;
		padding: 0.5rem;
		border-bottom: 1px solid var(--color-border);
	}

	.toggle-btn {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		padding: 0.5rem;
		background: none;
		border: 1px solid var(--color-border);
		border-radius: 6px;
		color: var(--color-text-secondary);
		cursor: pointer;
		font-size: 0.75rem;
		transition: all 0.15s;
	}

	.toggle-btn:hover {
		background-color: var(--color-bg-hover);
		color: var(--color-text-primary);
	}

	.toggle-btn.active {
		background-color: var(--color-lion-900);
		border-color: var(--color-lion-600);
		color: var(--color-lion-300);
	}

	.collapse-btn {
		flex: 0;
		padding: 0.5rem 0.75rem;
	}

	.collapse-btn:hover {
		border-color: var(--color-lion-500);
		color: var(--color-lion-400);
	}
</style>
