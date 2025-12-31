<script lang="ts">
	import { Settings, FileCode, Bot, Shield, X, Check, Loader2 } from 'lucide-svelte';

	interface ProjectConfigStatus {
		has_claude_md: boolean;
		has_claude_dir: boolean;
		has_settings: boolean;
		has_agents: boolean;
		is_fully_configured: boolean;
	}

	let {
		show = false,
		projectPath = '',
		projectName = '',
		configStatus = null,
		onInit = () => {},
		onSkip = () => {},
		onClose = () => {}
	} = $props<{
		show: boolean;
		projectPath: string;
		projectName: string;
		configStatus: ProjectConfigStatus | null;
		onInit: () => void;
		onSkip: () => void;
		onClose: () => void;
	}>();

	let isInitializing = $state(false);

	async function handleInit() {
		isInitializing = true;
		try {
			await onInit();
		} finally {
			isInitializing = false;
		}
	}
</script>

{#if show}
	<div class="dialog-overlay" onclick={onClose}>
		<div class="dialog" onclick={(e) => e.stopPropagation()}>
			<button class="close-btn" onclick={onClose}>
				<X size={18} />
			</button>

			<div class="dialog-header">
				<Settings size={24} />
				<h2>Configuration Claude Code</h2>
			</div>

			<div class="dialog-body">
				<p class="project-name">{projectName}</p>
				<p class="project-path">{projectPath}</p>

				<div class="config-status">
					<h3>Etat de la configuration</h3>
					<ul class="status-list">
						<li class:has={configStatus?.has_claude_md}>
							<span class="icon">{configStatus?.has_claude_md ? '✓' : '✗'}</span>
							<FileCode size={14} />
							<span>CLAUDE.md</span>
						</li>
						<li class:has={configStatus?.has_settings}>
							<span class="icon">{configStatus?.has_settings ? '✓' : '✗'}</span>
							<Shield size={14} />
							<span>Permissions (.claude/settings.json)</span>
						</li>
						<li class:has={configStatus?.has_agents}>
							<span class="icon">{configStatus?.has_agents ? '✓' : '✗'}</span>
							<Bot size={14} />
							<span>Agents (.claude/agents/)</span>
						</li>
					</ul>
				</div>

				{#if !configStatus?.is_fully_configured}
					<div class="info-box">
						<p>
							Ce projet n'a pas de configuration Claude Code complete.
							Sans configuration, Claude Code fonctionnera avec les parametres par defaut.
						</p>
						<p class="highlight">
							Voulez-vous initialiser ce projet avec la configuration Leon?
						</p>
					</div>
				{/if}
			</div>

			<div class="dialog-actions">
				{#if !configStatus?.is_fully_configured}
					<button class="btn-secondary" onclick={onSkip}>
						Ignorer
					</button>
					<button class="btn-primary" onclick={handleInit} disabled={isInitializing}>
						{#if isInitializing}
							<Loader2 size={16} class="spin" />
							Initialisation...
						{:else}
							<Check size={16} />
							Initialiser
						{/if}
					</button>
				{:else}
					<button class="btn-primary" onclick={onClose}>
						<Check size={16} />
						Fermer
					</button>
				{/if}
			</div>
		</div>
	</div>
{/if}

<style>
	.dialog-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.6);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
	}

	.dialog {
		position: relative;
		background: var(--color-bg-secondary);
		border: 1px solid var(--color-border);
		border-radius: 12px;
		width: 90%;
		max-width: 480px;
		padding: 1.5rem;
		box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
	}

	.close-btn {
		position: absolute;
		top: 0.75rem;
		right: 0.75rem;
		background: none;
		border: none;
		color: var(--color-text-muted);
		cursor: pointer;
		padding: 0.25rem;
		border-radius: 4px;
	}

	.close-btn:hover {
		background: var(--color-bg-hover);
		color: var(--color-text-primary);
	}

	.dialog-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 1rem;
		color: var(--color-lion-400);
	}

	.dialog-header h2 {
		margin: 0;
		font-size: 1.1rem;
		font-weight: 600;
	}

	.dialog-body {
		margin-bottom: 1.5rem;
	}

	.project-name {
		font-weight: 600;
		color: var(--color-text-primary);
		margin: 0 0 0.25rem;
	}

	.project-path {
		font-size: 0.75rem;
		color: var(--color-text-muted);
		font-family: 'JetBrains Mono', monospace;
		margin: 0 0 1rem;
		word-break: break-all;
	}

	.config-status h3 {
		font-size: 0.85rem;
		font-weight: 500;
		color: var(--color-text-secondary);
		margin: 0 0 0.5rem;
	}

	.status-list {
		list-style: none;
		padding: 0;
		margin: 0 0 1rem;
	}

	.status-list li {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.375rem 0;
		font-size: 0.85rem;
		color: var(--color-text-muted);
	}

	.status-list li.has {
		color: var(--color-success);
	}

	.status-list li .icon {
		width: 1rem;
		text-align: center;
		font-weight: bold;
	}

	.status-list li:not(.has) .icon {
		color: var(--color-error);
	}

	.info-box {
		background: var(--color-bg-tertiary);
		border: 1px solid var(--color-border);
		border-radius: 8px;
		padding: 0.75rem;
		font-size: 0.85rem;
		color: var(--color-text-secondary);
	}

	.info-box p {
		margin: 0 0 0.5rem;
	}

	.info-box p:last-child {
		margin-bottom: 0;
	}

	.info-box .highlight {
		color: var(--color-lion-400);
		font-weight: 500;
	}

	.dialog-actions {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
	}

	.btn-primary,
	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 1rem;
		border-radius: 6px;
		font-size: 0.85rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.btn-primary {
		background: var(--color-lion-600);
		border: 1px solid var(--color-lion-500);
		color: white;
	}

	.btn-primary:hover:not(:disabled) {
		background: var(--color-lion-500);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		background: transparent;
		border: 1px solid var(--color-border);
		color: var(--color-text-secondary);
	}

	.btn-secondary:hover {
		background: var(--color-bg-hover);
		color: var(--color-text-primary);
	}

	:global(.spin) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}
</style>
