<script lang="ts">
	import { Plus, X, Terminal as TerminalIcon, FolderOpen } from 'lucide-svelte';
	import Terminal from './Terminal.svelte';

	interface Tab {
		id: string;
		name: string;
		workingDir: string;
		projectId?: string; // Pour identifier le projet associé
	}

	let {
		onReady = () => {},
		onOutput = (text: string) => {},
		onActiveProjectChange = (path: string | null) => {}
	} = $props<{
		onReady: () => void;
		onOutput: (text: string) => void;
		onActiveProjectChange: (path: string | null) => void;
	}>();

	let tabs = $state<Tab[]>([]);
	let activeTabId = $state<string | null>(null);
	let terminals = $state<(Terminal | null)[]>([]);

	// Notifier le parent quand l'onglet actif change
	$effect(() => {
		if (activeTabId) {
			const activeTab = tabs.find(t => t.id === activeTabId);
			onActiveProjectChange(activeTab?.workingDir || null);
		} else {
			onActiveProjectChange(null);
		}
	});

	/**
	 * Ouvre un projet dans un onglet (existant ou nouveau)
	 */
	export function openProject(projectPath: string, projectName: string, projectId?: string): void {
		// Vérifier si un onglet existe déjà pour ce projet
		const existingTab = tabs.find(t => t.workingDir === projectPath);

		if (existingTab) {
			// Activer l'onglet existant
			activeTabId = existingTab.id;
			const index = tabs.findIndex(t => t.id === existingTab.id);
			setTimeout(() => terminals[index]?.focus(), 50);
			return;
		}

		// Créer un nouvel onglet
		const id = crypto.randomUUID();
		const newTab: Tab = {
			id,
			name: projectName,
			workingDir: projectPath,
			projectId
		};

		tabs = [...tabs, newTab];
		activeTabId = id;
	}

	function addTab() {
		// Ajouter un tab pour le même projet que l'onglet actif
		const activeTab = tabs.find(t => t.id === activeTabId);
		if (!activeTab) return;

		const id = crypto.randomUUID();
		const sameProjectTabs = tabs.filter(t => t.workingDir === activeTab.workingDir);
		const tabNumber = sameProjectTabs.length + 1;

		const newTab: Tab = {
			id,
			name: `${activeTab.name} (${tabNumber})`,
			workingDir: activeTab.workingDir,
			projectId: activeTab.projectId
		};

		tabs = [...tabs, newTab];
		activeTabId = id;
	}

	function closeTab(tabId: string) {
		const index = tabs.findIndex(t => t.id === tabId);
		if (index === -1) return;

		// Ne pas fermer le dernier tab
		if (tabs.length === 1) return;

		tabs = tabs.filter(t => t.id !== tabId);
		terminals = terminals.filter((_, i) => i !== index);

		// Sélectionner un autre tab si on ferme le tab actif
		if (activeTabId === tabId) {
			const newIndex = Math.max(0, index - 1);
			activeTabId = tabs[newIndex]?.id || null;
		}
	}

	function selectTab(tabId: string) {
		activeTabId = tabId;
		// Focus le terminal du tab actif
		const index = tabs.findIndex(t => t.id === tabId);
		setTimeout(() => {
			terminals[index]?.focus();
		}, 50);
	}

	function handleTerminalReady(tabId: string) {
		if (tabId === tabs[0]?.id) {
			onReady();
		}
	}

	// Méthode publique pour focus le terminal actif
	export function focus() {
		if (activeTabId) {
			const index = tabs.findIndex(t => t.id === activeTabId);
			terminals[index]?.focus();
		}
	}

	// Méthode publique pour envoyer du texte au terminal actif
	export function sendText(text: string) {
		if (activeTabId) {
			const index = tabs.findIndex(t => t.id === activeTabId);
			terminals[index]?.sendText(text);
		}
	}

	// Méthode pour récupérer le workingDir actif
	export function getActiveWorkingDir(): string | null {
		if (activeTabId) {
			const activeTab = tabs.find(t => t.id === activeTabId);
			return activeTab?.workingDir || null;
		}
		return null;
	}

	// Méthode pour vérifier si un projet est ouvert
	export function hasProject(projectPath: string): boolean {
		return tabs.some(t => t.workingDir === projectPath);
	}

	// Méthode pour récupérer le nombre d'onglets
	export function getTabCount(): number {
		return tabs.length;
	}
</script>

<div class="terminal-tabs-container">
	{#if tabs.length > 0}
		<div class="tabs-bar">
			<div class="tabs-list">
				{#each tabs as tab (tab.id)}
					<button
						class="tab"
						class:active={tab.id === activeTabId}
						onclick={() => selectTab(tab.id)}
						title={tab.workingDir}
					>
						{#if tab.projectId}
							<FolderOpen size={12} />
						{:else}
							<TerminalIcon size={12} />
						{/if}
						<span class="tab-name">{tab.name}</span>
						{#if tabs.length > 1}
							<button
								class="tab-close"
								onclick={(e) => { e.stopPropagation(); closeTab(tab.id); }}
								title="Fermer"
							>
								<X size={12} />
							</button>
						{/if}
					</button>
				{/each}
			</div>
			<button class="add-tab-btn" onclick={addTab} title="Nouveau terminal">
				<Plus size={14} />
			</button>
		</div>

		<div class="terminals-container">
			{#each tabs as tab, index (tab.id)}
				<div class="terminal-wrapper" class:active={tab.id === activeTabId}>
					<Terminal
						workingDir={tab.workingDir}
						onReady={() => handleTerminalReady(tab.id)}
						{onOutput}
						bind:this={terminals[index]}
					/>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.terminal-tabs-container {
		display: flex;
		flex-direction: column;
		height: 100%;
		background-color: #1a1a1a;
	}

	.tabs-bar {
		display: flex;
		align-items: center;
		background-color: #252525;
		border-bottom: 1px solid var(--color-border);
		min-height: 32px;
	}

	.tabs-list {
		display: flex;
		flex: 1;
		overflow-x: auto;
		scrollbar-width: none;
	}

	.tabs-list::-webkit-scrollbar {
		display: none;
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.75rem;
		background: transparent;
		border: none;
		border-right: 1px solid var(--color-border);
		color: var(--color-text-muted);
		font-size: 0.75rem;
		cursor: pointer;
		transition: all 0.15s ease;
		white-space: nowrap;
	}

	.tab:hover {
		background: rgba(255, 255, 255, 0.05);
		color: var(--color-text-secondary);
	}

	.tab.active {
		background: #1a1a1a;
		color: var(--color-lion-400);
		border-bottom: 2px solid var(--color-lion-500);
		margin-bottom: -1px;
	}

	.tab-name {
		max-width: 120px;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.tab-close {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0.125rem;
		background: transparent;
		border: none;
		border-radius: 3px;
		color: var(--color-text-muted);
		cursor: pointer;
		opacity: 0;
		transition: all 0.1s ease;
	}

	.tab:hover .tab-close {
		opacity: 1;
	}

	.tab-close:hover {
		background: rgba(255, 255, 255, 0.1);
		color: var(--color-text-primary);
	}

	.add-tab-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0.375rem 0.5rem;
		background: transparent;
		border: none;
		color: var(--color-text-muted);
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.add-tab-btn:hover {
		background: rgba(255, 255, 255, 0.05);
		color: var(--color-lion-400);
	}

	.terminals-container {
		flex: 1;
		position: relative;
		min-height: 0;
	}

	.terminal-wrapper {
		position: absolute;
		inset: 0;
		display: none;
	}

	.terminal-wrapper.active {
		display: block;
	}
</style>
