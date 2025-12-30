<script lang="ts">
	import { Plus, FolderOpen, MessageSquare, Settings, ChevronRight } from 'lucide-svelte';

	interface Session {
		id: string;
		name: string;
		project: string;
		timestamp: Date;
	}

	let { sessions = [], activeSession = null, onNewChat = () => {}, onSelectSession = (id: string) => {} } = $props<{
		sessions: Session[];
		activeSession: string | null;
		onNewChat: () => void;
		onSelectSession: (id: string) => void;
	}>();
</script>

<aside class="sidebar">
	<div class="sidebar-header">
		<img src="/images/logo.png" alt="Léon" class="logo" />
	</div>

	<button class="new-chat-btn" onclick={onNewChat}>
		<Plus size={18} />
		<span>Nouvelle conversation</span>
	</button>

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
		<button class="settings-btn">
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

	.new-chat-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin: 1rem;
		padding: 0.75rem 1rem;
		background-color: var(--color-lion-600);
		color: var(--color-text-primary);
		border: none;
		border-radius: 8px;
		cursor: pointer;
		font-weight: 500;
	}

	.new-chat-btn:hover {
		background-color: var(--color-lion-500);
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
</style>
