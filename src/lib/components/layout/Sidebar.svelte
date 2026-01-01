<script lang="ts">
	import { Plus, FolderOpen, MessageSquare, Settings, ChevronRight, ChevronLeft, Save, Rocket, GitBranch, Files, Zap, PanelRightClose, PanelRightOpen, Download, ChevronUp, ChevronDown } from 'lucide-svelte';

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
		onImportProject = () => {},
		onSelectSession = (id: string) => {},
		onSessionContextMenu = (e: MouseEvent, session: Session) => {},
		onReorderSessions = (sessions: Session[]) => {},
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
		onImportProject: () => void;
		onSelectSession: (id: string) => void;
		onSessionContextMenu: (e: MouseEvent, session: Session) => void;
		onReorderSessions: (sessions: Session[]) => void;
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

	function moveUp(index: number) {
		if (index <= 0) return;
		const newSessions = [...sessions];
		[newSessions[index - 1], newSessions[index]] = [newSessions[index], newSessions[index - 1]];
		onReorderSessions(newSessions);
	}

	function moveDown(index: number) {
		if (index >= sessions.length - 1) return;
		const newSessions = [...sessions];
		[newSessions[index], newSessions[index + 1]] = [newSessions[index + 1], newSessions[index]];
		onReorderSessions(newSessions);
	}
</script>

<aside class="sidebar">
	<div class="sidebar-header">
		<img src="/images/logo.png" alt="Léon" class="logo" />
	</div>

	<div class="sidebar-actions">
		<button class="action-primary" onclick={onNewProject} title="Créer un nouveau projet">
			<Plus size={16} />
			<span>Nouveau</span>
		</button>
		<button class="action-secondary" onclick={onOpenProject} title="Ouvrir un dossier existant">
			<FolderOpen size={14} />
			<span>Ouvrir</span>
		</button>
		<button class="action-secondary" onclick={onImportProject} title="Importer un projet existant">
			<Download size={14} />
			<span>Importer</span>
		</button>
	</div>

	<nav class="sidebar-nav">
		<div class="nav-section">
			<div class="nav-section-header">
				<FolderOpen size={14} />
				<span>Projets</span>
			</div>
			<ul class="session-list">
				{#if sessions.length === 0}
					<li class="empty-state">Aucun projet</li>
				{:else}
					{#each sessions as session, index (session.id)}
						<li>
							<div class="session-row">
								<div class="reorder-buttons">
									<button
										class="reorder-btn"
										onclick={() => moveUp(index)}
										disabled={index === 0}
										title="Monter"
									>
										<ChevronUp size={12} />
									</button>
									<button
										class="reorder-btn"
										onclick={() => moveDown(index)}
										disabled={index === sessions.length - 1}
										title="Descendre"
									>
										<ChevronDown size={12} />
									</button>
								</div>
								<button
									class="session-item"
									class:active={activeSession === session.id}
									onclick={() => onSelectSession(session.id)}
									oncontextmenu={(e) => onSessionContextMenu(e, session)}
								>
									<MessageSquare size={14} />
									<span class="session-name">{session.name}</span>
									<ChevronRight size={14} class="chevron" />
								</button>
							</div>
						</li>
					{/each}
				{/if}
			</ul>
		</div>
	</nav>

	<div class="sidebar-footer">
		{#if hasActiveProject}
			<div class="toolbar">
				<button class="tool-btn" onclick={onToggleFiles} title="Fichiers">
					<Files size={16} />
				</button>
				<button class="tool-btn" onclick={onToggleGit} title="Git">
					<GitBranch size={16} />
					{#if gitChanges > 0}
						<span class="badge">{gitChanges}</span>
					{/if}
				</button>
				<button class="tool-btn" onclick={onToggleSnippets} title="Snippets">
					<Zap size={16} />
				</button>
				<span class="toolbar-divider"></span>
				<button class="tool-btn save" onclick={onSave} title="Sauver">
					<Save size={16} />
				</button>
				<button class="tool-btn release" onclick={onRelease} title="Release">
					<Rocket size={16} />
				</button>
			</div>
		{/if}
		<div class="toolbar bottom">
			{#if hasActiveProject}
				<button
					class="tool-btn"
					class:active={isPreviewOpen}
					onclick={onTogglePreview}
					title="Preview (Ctrl+P)"
				>
					{#if isPreviewOpen}
						<PanelRightClose size={16} />
					{:else}
						<PanelRightOpen size={16} />
					{/if}
				</button>
			{/if}
			<button class="tool-btn" onclick={onOpenSettings} title="Paramètres">
				<Settings size={16} />
			</button>
			<button
				class="tool-btn collapse"
				onclick={onToggleSidebar}
				title="Masquer (Ctrl+B)"
			>
				<ChevronLeft size={16} />
			</button>
		</div>
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
		flex-wrap: wrap;
		gap: 0.375rem;
		padding: 0.5rem 0.75rem;
	}

	.action-primary {
		flex: 1 1 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		padding: 0.5rem 0.65rem;
		background: var(--color-lion-600);
		color: var(--color-text-primary);
		border: none;
		border-radius: 6px;
		cursor: pointer;
		font-size: 0.8rem;
		font-weight: 500;
		transition: background 0.15s;
	}

	.action-primary:hover {
		background: var(--color-lion-500);
	}

	.action-secondary {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.3rem;
		padding: 0.4rem 0.5rem;
		background: transparent;
		color: var(--color-text-secondary);
		border: 1px solid var(--color-border);
		border-radius: 5px;
		cursor: pointer;
		font-size: 0.7rem;
		white-space: nowrap;
		overflow: hidden;
		transition: all 0.15s;
	}

	.action-secondary span {
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.action-secondary:hover {
		background: var(--color-bg-hover);
		color: var(--color-text-primary);
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

	.session-row {
		display: flex;
		align-items: center;
		gap: 0;
	}

	.reorder-buttons {
		display: flex;
		flex-direction: column;
		opacity: 0;
		transition: opacity 0.15s;
	}

	.session-row:hover .reorder-buttons {
		opacity: 1;
	}

	.reorder-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0.125rem;
		background: none;
		border: none;
		color: var(--color-text-muted);
		cursor: pointer;
		border-radius: 3px;
	}

	.reorder-btn:hover:not(:disabled) {
		background: var(--color-bg-hover);
		color: var(--color-text-primary);
	}

	.reorder-btn:disabled {
		opacity: 0.2;
		cursor: default;
	}

	.session-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		flex: 1;
		padding: 0.625rem 0.75rem 0.625rem 0.5rem;
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

	.toolbar {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.5rem;
		border-bottom: 1px solid var(--color-border);
	}

	.toolbar.bottom {
		border-bottom: none;
		border-top: 1px solid var(--color-border);
	}

	.toolbar-divider {
		width: 1px;
		height: 16px;
		background: var(--color-border);
		margin: 0 0.25rem;
	}

	.tool-btn {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0.4rem;
		background: none;
		border: none;
		border-radius: 5px;
		color: var(--color-text-muted);
		cursor: pointer;
		transition: all 0.15s;
	}

	.tool-btn:hover {
		background: var(--color-bg-hover);
		color: var(--color-text-primary);
	}

	.tool-btn.active {
		background: var(--color-lion-900);
		color: var(--color-lion-400);
	}

	.tool-btn.save:hover {
		color: var(--color-success, #69db7c);
	}

	.tool-btn.release:hover {
		color: var(--color-lion-400);
	}

	.tool-btn.collapse {
		margin-left: auto;
	}

	.badge {
		position: absolute;
		top: -2px;
		right: -2px;
		min-width: 14px;
		height: 14px;
		padding: 0 3px;
		background: var(--color-warning, #ffa94d);
		border-radius: 7px;
		font-size: 0.55rem;
		font-weight: 600;
		color: black;
		display: flex;
		align-items: center;
		justify-content: center;
	}

</style>
