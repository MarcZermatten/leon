<script lang="ts">
	import { History, ChevronDown, MessageSquare, Clock } from 'lucide-svelte';
	import { listProjectSessions, formatRelativeTime, type SessionInfo } from '$lib/services/sessions';

	let {
		projectPath = null,
		onSelectSession = (sessionId: string) => {},
		onNewSession = () => {}
	} = $props<{
		projectPath: string | null;
		onSelectSession: (sessionId: string) => void;
		onNewSession: () => void;
	}>();

	let isOpen = $state(false);
	let sessions = $state<SessionInfo[]>([]);
	let loading = $state(false);

	// Charger les sessions quand le projet change
	$effect(() => {
		if (projectPath) {
			loadSessions();
		} else {
			sessions = [];
		}
	});

	async function loadSessions() {
		if (!projectPath) return;
		loading = true;
		try {
			sessions = await listProjectSessions(projectPath);
		} catch (e) {
			console.error('[SessionSelector] Error loading sessions:', e);
		} finally {
			loading = false;
		}
	}

	function toggleDropdown() {
		isOpen = !isOpen;
		if (isOpen) {
			loadSessions();
		}
	}

	function selectSession(session: SessionInfo) {
		onSelectSession(session.id);
		isOpen = false;
	}

	function handleNewSession() {
		onNewSession();
		isOpen = false;
	}

	function handleClickOutside(e: MouseEvent) {
		const target = e.target as HTMLElement;
		if (!target.closest('.session-selector')) {
			isOpen = false;
		}
	}
</script>

<svelte:window on:click={handleClickOutside} />

<div class="session-selector">
	<button class="selector-btn" onclick={toggleDropdown} title="Sessions précédentes">
		<History size={14} />
		<span>Sessions</span>
		<ChevronDown size={12} class={isOpen ? 'rotated' : ''} />
	</button>

	{#if isOpen}
		<div class="dropdown">
			<div class="dropdown-header">
				<span>Sessions précédentes</span>
				<button class="new-session-btn" onclick={handleNewSession}>
					+ Nouvelle
				</button>
			</div>

			<div class="sessions-list">
				{#if loading}
					<div class="loading">Chargement...</div>
				{:else if sessions.length === 0}
					<div class="empty">Aucune session précédente</div>
				{:else}
					{#each sessions as session (session.id)}
						<button class="session-item" onclick={() => selectSession(session)}>
							<div class="session-info">
								<span class="session-id">{session.id.slice(0, 8)}...</span>
								<span class="session-meta">
									<MessageSquare size={12} />
									{session.message_count} msg
								</span>
							</div>
							<div class="session-time">
								<Clock size={12} />
								{formatRelativeTime(session.last_modified)}
							</div>
						</button>
					{/each}
				{/if}
			</div>

			<div class="dropdown-footer">
				<span class="hint">Cliquer pour reprendre avec --resume</span>
			</div>
		</div>
	{/if}
</div>

<style>
	.session-selector {
		position: relative;
	}

	.selector-btn {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.25rem 0.5rem;
		background: var(--color-bg-hover);
		border: 1px solid var(--color-border);
		border-radius: 4px;
		color: var(--color-text-secondary);
		font-size: 0.7rem;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.selector-btn:hover {
		background: var(--color-lion-900);
		border-color: var(--color-lion-700);
		color: var(--color-lion-300);
	}

	.selector-btn :global(.rotated) {
		transform: rotate(180deg);
	}

	.dropdown {
		position: absolute;
		top: 100%;
		right: 0;
		margin-top: 4px;
		width: 280px;
		background: var(--color-bg-secondary);
		border: 1px solid var(--color-border);
		border-radius: 8px;
		box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
		z-index: 100;
		overflow: hidden;
	}

	.dropdown-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.625rem 0.75rem;
		border-bottom: 1px solid var(--color-border);
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--color-text-secondary);
	}

	.new-session-btn {
		padding: 0.25rem 0.5rem;
		background: var(--color-lion-900);
		border: 1px solid var(--color-lion-700);
		border-radius: 4px;
		color: var(--color-lion-300);
		font-size: 0.65rem;
		cursor: pointer;
	}

	.new-session-btn:hover {
		background: var(--color-lion-800);
	}

	.sessions-list {
		max-height: 240px;
		overflow-y: auto;
	}

	.loading,
	.empty {
		padding: 1.5rem;
		text-align: center;
		font-size: 0.8rem;
		color: var(--color-text-muted);
	}

	.session-item {
		display: flex;
		justify-content: space-between;
		align-items: center;
		width: 100%;
		padding: 0.5rem 0.75rem;
		background: transparent;
		border: none;
		border-bottom: 1px solid var(--color-border);
		cursor: pointer;
		transition: background 0.1s ease;
		text-align: left;
	}

	.session-item:last-child {
		border-bottom: none;
	}

	.session-item:hover {
		background: var(--color-bg-hover);
	}

	.session-info {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.session-id {
		font-size: 0.8rem;
		font-family: 'JetBrains Mono', monospace;
		color: var(--color-text-primary);
	}

	.session-meta {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.7rem;
		color: var(--color-text-muted);
	}

	.session-time {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.65rem;
		color: var(--color-text-muted);
	}

	.dropdown-footer {
		padding: 0.5rem 0.75rem;
		border-top: 1px solid var(--color-border);
		background: var(--color-bg-tertiary);
	}

	.hint {
		font-size: 0.65rem;
		color: var(--color-text-muted);
		font-style: italic;
	}
</style>
