<script lang="ts">
	import { X, FolderOpen, Monitor, Palette } from 'lucide-svelte';

	let { isOpen = false, onClose = () => {} } = $props<{
		isOpen: boolean;
		onClose: () => void;
	}>();

	// Settings state
	let defaultProjectDir = $state('C:\\Users\\Marc\\projets');
	let theme = $state<'dark' | 'light'>('dark');
	let fontSize = $state(14);

	function handleSave() {
		// TODO: Persist settings
		localStorage.setItem('leon_settings', JSON.stringify({
			defaultProjectDir,
			theme,
			fontSize
		}));
		onClose();
	}

	// Load settings on mount
	$effect(() => {
		const saved = localStorage.getItem('leon_settings');
		if (saved) {
			try {
				const settings = JSON.parse(saved);
				defaultProjectDir = settings.defaultProjectDir || defaultProjectDir;
				theme = settings.theme || theme;
				fontSize = settings.fontSize || fontSize;
			} catch (e) {
				console.error('Error loading settings:', e);
			}
		}
	});
</script>

{#if isOpen}
	<div class="modal-overlay" onclick={onClose} role="dialog" aria-modal="true">
		<div class="modal-content" onclick={(e) => e.stopPropagation()}>
			<div class="modal-header">
				<h2>Paramètres</h2>
				<button class="close-btn" onclick={onClose}>
					<X size={20} />
				</button>
			</div>

			<div class="modal-body">
				<section class="settings-section">
					<h3><FolderOpen size={16} /> Projets</h3>
					<div class="setting-item">
						<label for="defaultDir">Dossier par défaut</label>
						<input
							id="defaultDir"
							type="text"
							bind:value={defaultProjectDir}
							placeholder="C:\Users\Marc\projets"
						/>
					</div>
				</section>

				<section class="settings-section">
					<h3><Monitor size={16} /> Terminal</h3>
					<div class="setting-item">
						<label for="fontSize">Taille de police</label>
						<input
							id="fontSize"
							type="number"
							bind:value={fontSize}
							min="10"
							max="24"
						/>
					</div>
				</section>

				<section class="settings-section">
					<h3><Palette size={16} /> Apparence</h3>
					<div class="setting-item">
						<label>Thème</label>
						<div class="theme-buttons">
							<button
								class="theme-btn"
								class:active={theme === 'dark'}
								onclick={() => theme = 'dark'}
							>
								Sombre
							</button>
							<button
								class="theme-btn"
								class:active={theme === 'light'}
								onclick={() => theme = 'light'}
								disabled
							>
								Clair (bientôt)
							</button>
						</div>
					</div>
				</section>
			</div>

			<div class="modal-footer">
				<button class="btn-secondary" onclick={onClose}>Annuler</button>
				<button class="btn-primary" onclick={handleSave}>Enregistrer</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.modal-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(0, 0, 0, 0.7);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
	}

	.modal-content {
		background: var(--color-bg-secondary);
		border-radius: 12px;
		width: 90%;
		max-width: 500px;
		max-height: 80vh;
		overflow: hidden;
		display: flex;
		flex-direction: column;
		border: 1px solid var(--color-border);
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem 1.5rem;
		border-bottom: 1px solid var(--color-border);
	}

	.modal-header h2 {
		margin: 0;
		font-size: 1.25rem;
		font-weight: 600;
	}

	.close-btn {
		background: none;
		border: none;
		color: var(--color-text-secondary);
		cursor: pointer;
		padding: 0.25rem;
		border-radius: 4px;
	}

	.close-btn:hover {
		background: var(--color-bg-hover);
		color: var(--color-text-primary);
	}

	.modal-body {
		padding: 1.5rem;
		overflow-y: auto;
		flex: 1;
	}

	.settings-section {
		margin-bottom: 1.5rem;
	}

	.settings-section:last-child {
		margin-bottom: 0;
	}

	.settings-section h3 {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--color-text-secondary);
		margin: 0 0 0.75rem 0;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.setting-item {
		margin-bottom: 1rem;
	}

	.setting-item:last-child {
		margin-bottom: 0;
	}

	.setting-item label {
		display: block;
		font-size: 0.875rem;
		color: var(--color-text-primary);
		margin-bottom: 0.5rem;
	}

	.setting-item input[type="text"],
	.setting-item input[type="number"] {
		width: 100%;
		padding: 0.5rem 0.75rem;
		background: var(--color-bg-primary);
		border: 1px solid var(--color-border);
		border-radius: 6px;
		color: var(--color-text-primary);
		font-size: 0.875rem;
	}

	.setting-item input:focus {
		outline: none;
		border-color: var(--color-lion-500);
	}

	.theme-buttons {
		display: flex;
		gap: 0.5rem;
	}

	.theme-btn {
		flex: 1;
		padding: 0.5rem 1rem;
		background: var(--color-bg-primary);
		border: 1px solid var(--color-border);
		border-radius: 6px;
		color: var(--color-text-secondary);
		cursor: pointer;
		font-size: 0.875rem;
	}

	.theme-btn:hover:not(:disabled) {
		background: var(--color-bg-hover);
	}

	.theme-btn.active {
		background: var(--color-lion-900);
		border-color: var(--color-lion-500);
		color: var(--color-lion-300);
	}

	.theme-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1rem 1.5rem;
		border-top: 1px solid var(--color-border);
	}

	.btn-secondary,
	.btn-primary {
		padding: 0.5rem 1rem;
		border-radius: 6px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
	}

	.btn-secondary {
		background: var(--color-bg-primary);
		border: 1px solid var(--color-border);
		color: var(--color-text-secondary);
	}

	.btn-secondary:hover {
		background: var(--color-bg-hover);
	}

	.btn-primary {
		background: var(--color-lion-600);
		border: none;
		color: var(--color-text-primary);
	}

	.btn-primary:hover {
		background: var(--color-lion-500);
	}
</style>
