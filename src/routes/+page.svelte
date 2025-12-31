<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import Sidebar from '$lib/components/layout/Sidebar.svelte';
	import TerminalTabs from '$lib/components/terminal/TerminalTabs.svelte';
	import PreviewPanel from '$lib/components/preview/PreviewPanel.svelte';
	import StatusBar from '$lib/components/layout/StatusBar.svelte';
	import SettingsModal from '$lib/components/layout/SettingsModal.svelte';
	import CommandPalette from '$lib/components/layout/CommandPalette.svelte';
	import PlanPanel from '$lib/components/plan/PlanPanel.svelte';
	import SessionSelector from '$lib/components/layout/SessionSelector.svelte';
	import GitPanel from '$lib/components/git/GitPanel.svelte';
	import FileExplorer from '$lib/components/files/FileExplorer.svelte';
	import QuickActions from '$lib/components/layout/QuickActions.svelte';
	import OutputOverlay from '$lib/components/terminal/OutputOverlay.svelte';
	import SnippetsPanel from '$lib/components/layout/SnippetsPanel.svelte';
	import ProjectInitDialog from '$lib/components/layout/ProjectInitDialog.svelte';
	import type { PreviewState, PreviewMode } from '$lib/types/preview';
	import { defaultPreviewState } from '$lib/types/preview';
	import { checkClaudeAvailable, getClaudeVersion } from '$lib/services/claude';
	import { getClaudeStats, getSessionStats, type ClaudeStats, type SessionStats } from '$lib/services/stats';
	import {
		createCheckpoint,
		undoLastChange,
		getCheckpointCount,
		setCheckpointProject,
		detectModifiedFiles,
		shouldCreateCheckpoint
	} from '$lib/services/checkpoints';
	import { getGitStatus, type GitStatus } from '$lib/services/git';
	import { FolderOpen, RotateCcw, Plus, ChevronRight } from 'lucide-svelte';

	interface ProjectSession {
		id: string;
		name: string;
		project: string;
		timestamp: Date;
	}

	interface ProjectConfigStatus {
		has_claude_md: boolean;
		has_claude_dir: boolean;
		has_settings: boolean;
		has_agents: boolean;
		is_fully_configured: boolean;
	}

	// State
	let sessionsList = $state<ProjectSession[]>([]);
	let activeSession = $state<string | null>(null);
	let claudeAvailable = $state(false);
	let claudeVersion = $state<string | null>(null);
	let workingDir = $state<string | null>(null);
	let terminalReady = $state(false);
	let terminalComponent = $state<TerminalTabs | null>(null);
	let showSettings = $state(false);
	let showCommandPalette = $state(false);

	// Usage stats (connecté aux vraies données Claude Code)
	let claudeStats = $state<ClaudeStats | null>(null);
	let sessionStats = $state<SessionStats | null>(null);
	let statsRefreshInterval: ReturnType<typeof setInterval> | null = null;

	// Checkpoints
	let checkpointCount = $state(0);
	let pendingFiles: string[] = [];

	// Session Claude active (pour --resume)
	let activeClaudeSessionId = $state<string | null>(null);

	// Computed stats for StatusBar
	let contextUsedPercent = $derived(sessionStats ? sessionStats.context_used_percent : null);
	let sessionMessages = $derived(sessionStats ? sessionStats.message_count : null);
	let todayMessages = $derived(claudeStats ? claudeStats.today_messages : null);
	let weeklyMessages = $derived(claudeStats ? claudeStats.weekly_messages : null);

	// Preview state
	let previewState = $state<PreviewState>(defaultPreviewState);
	let showPreview = $state(true);

	// Plan panel state
	let showPlanPanel = $state(false);
	let planPanelComponent = $state<PlanPanel | null>(null);

	// Git panel state
	let showGitPanel = $state(false);
	let gitStatus = $state<GitStatus | null>(null);

	// File explorer state
	let showFileExplorer = $state(false);

	// Output overlay state
	let terminalOutput = $state('');
	let showOutputOverlay = $state(true);

	// Snippets panel state
	let showSnippetsPanel = $state(false);

	// Project init dialog state
	let showProjectInitDialog = $state(false);
	let pendingProjectPath = $state<string | null>(null);
	let pendingProjectName = $state<string>('');
	let pendingProjectConfigStatus = $state<ProjectConfigStatus | null>(null);

	// Computed Git changes count
	let gitChangesCount = $derived(
		(gitStatus?.staged.length || 0) +
		(gitStatus?.unstaged.length || 0) +
		(gitStatus?.untracked.length || 0)
	);

	// Panel widths and visibility
	let sidebarWidth = $state(260);
	let previewWidth = $state(450);
	let showSidebar = $state(true);

	// LocalStorage key
	const PROJECTS_KEY = 'leon_recent_projects';

	// Dossier par défaut pour les projets Léon
	const LEON_PROJECTS_DIR = 'C:\\Users\\Marc\\Leon\\projets';

	// Raccourcis clavier globaux
	function handleGlobalKeydown(e: KeyboardEvent) {
		// Ctrl+K → Command Palette
		if (e.ctrlKey && e.key === 'k') {
			e.preventDefault();
			showCommandPalette = true;
		}
		// Ctrl+N → New project
		if (e.ctrlKey && e.key === 'n') {
			e.preventDefault();
			handleNewProject();
		}
		// Ctrl+O → Open project
		if (e.ctrlKey && e.key === 'o') {
			e.preventDefault();
			handleOpenFolder();
		}
		// Ctrl+, → Settings
		if (e.ctrlKey && e.key === ',') {
			e.preventDefault();
			showSettings = true;
		}
		// Ctrl+G → Git panel
		if (e.ctrlKey && e.key === 'g') {
			e.preventDefault();
			handleToggleGit();
		}
		// Ctrl+E → File explorer
		if (e.ctrlKey && e.key === 'e') {
			e.preventDefault();
			handleToggleFiles();
		}
		// Ctrl+Shift+S → Snippets
		if (e.ctrlKey && e.shiftKey && e.key === 'S') {
			e.preventDefault();
			handleToggleSnippets();
		}
		// Ctrl+B → Toggle sidebar
		if (e.ctrlKey && e.key === 'b') {
			e.preventDefault();
			showSidebar = !showSidebar;
		}
		// Ctrl+P → Toggle preview (quand pas dans input)
		if (e.ctrlKey && e.key === 'p' && !e.shiftKey) {
			e.preventDefault();
			handleTogglePreview();
		}
	}

	// Toggle preview panel
	function handleTogglePreview() {
		if (showPreview || showGitPanel || showFileExplorer || showSnippetsPanel || showPlanPanel) {
			// Fermer tous les panels de droite
			showPreview = false;
			showGitPanel = false;
			showFileExplorer = false;
			showSnippetsPanel = false;
			showPlanPanel = false;
		} else {
			// Ouvrir le preview par défaut
			showPreview = true;
		}
	}

	onMount(() => {
		// Async init
		(async () => {
			// Charger les projets récents
			loadRecentProjects();

			// Vérifier si Claude CLI est disponible
			claudeAvailable = await checkClaudeAvailable();
			if (claudeAvailable) {
				claudeVersion = await getClaudeVersion();
			}

			// Charger les stats initiales
			await refreshStats();

			// Rafraîchir les stats toutes les 30 secondes
			statsRefreshInterval = setInterval(refreshStats, 30000);
		})();

		window.addEventListener('keydown', handleGlobalKeydown);

		return () => {
			window.removeEventListener('keydown', handleGlobalKeydown);
		};
	});

	onDestroy(() => {
		if (statsRefreshInterval) {
			clearInterval(statsRefreshInterval);
		}
	});

	async function refreshStats() {
		try {
			claudeStats = await getClaudeStats();
			if (workingDir) {
				sessionStats = await getSessionStats(workingDir);
				// Rafraîchir aussi le status Git
				await refreshGitStatus();
			}
			// Mettre à jour le compteur de checkpoints
			checkpointCount = await getCheckpointCount();
		} catch (e) {
			console.error('Error refreshing stats:', e);
		}
	}

	async function refreshGitStatus() {
		if (!workingDir) {
			gitStatus = null;
			return;
		}
		try {
			gitStatus = await getGitStatus(workingDir);
		} catch (e) {
			console.error('Error refreshing git status:', e);
		}
	}

	async function handleUndo() {
		const result = await undoLastChange();
		if (result) {
			console.log('[Undo] Restored:', result.description, result.restored_files);
			checkpointCount = await getCheckpointCount();
			// Rafraîchir le preview si un fichier restauré est affiché
			if (previewState.code?.filePath && result.restored_files.includes(previewState.code.filePath)) {
				updatePreviewForFile(previewState.code.filePath);
			}
		}
	}

	function handleShowCheckpoints() {
		// TODO: Afficher un modal avec la liste des checkpoints
		console.log('[Checkpoints] Show history');
	}

	function handleCompact() {
		// Envoyer /compact au terminal
		if (terminalComponent && workingDir) {
			terminalComponent.sendText('/compact\n');
		}
	}

	function handleSave() {
		// Envoyer commande git save au terminal
		if (terminalComponent && workingDir) {
			terminalComponent.sendText('git add -A && git commit -m "save" && git push\n');
		}
	}

	function handleClearTerminal() {
		if (terminalComponent) {
			terminalComponent.sendText('/clear\n');
		}
	}

	function handleSendCommand(cmd: string) {
		if (terminalComponent && workingDir) {
			terminalComponent.sendText(cmd);
		}
	}

	function loadRecentProjects() {
		try {
			const saved = localStorage.getItem(PROJECTS_KEY);
			if (saved) {
				const projects = JSON.parse(saved);
				sessionsList = projects.map((p: any) => ({
					...p,
					timestamp: new Date(p.timestamp)
				}));
			}
		} catch (e) {
			console.error('Error loading recent projects:', e);
		}
	}

	function saveRecentProjects() {
		try {
			localStorage.setItem(PROJECTS_KEY, JSON.stringify(sessionsList));
		} catch (e) {
			console.error('Error saving recent projects:', e);
		}
	}

	// Handlers
	async function handleOpenFolder() {
		try {
			const { open } = await import('@tauri-apps/plugin-dialog');
			const selected = await open({
				directory: true,
				multiple: false,
				title: 'Sélectionner un dossier de projet'
			});
			if (selected && typeof selected === 'string') {
				openProject(selected);
			}
		} catch (e) {
			console.error('Erreur ouverture dossier:', e);
		}
	}

	async function openProject(path: string, customName?: string, skipConfigCheck = false) {
		// Vérifier si le projet existe déjà dans la liste
		let existing = sessionsList.find(s => s.project === path);
		let projectName = customName || existing?.name || path.split(/[/\\]/).pop() || 'Projet';
		let projectId: string;

		// Vérifier la configuration Claude Code du projet (sauf si skipConfigCheck)
		if (!skipConfigCheck) {
			try {
				const { invoke } = await import('@tauri-apps/api/core');
				const configStatus = await invoke<ProjectConfigStatus>('check_project_config', {
					projectPath: path
				});

				// Si le projet n'est pas configuré, afficher le dialogue
				if (!configStatus.is_fully_configured) {
					pendingProjectPath = path;
					pendingProjectName = projectName;
					pendingProjectConfigStatus = configStatus;
					showProjectInitDialog = true;
					return; // Attendre la décision de l'utilisateur
				}
			} catch (e) {
				console.error('[Project] Error checking config:', e);
				// Continuer quand même si la vérification échoue
			}
		}

		// Continuer avec l'ouverture du projet
		await doOpenProject(path, projectName);
	}

	async function doOpenProject(path: string, projectName: string) {
		let existing = sessionsList.find(s => s.project === path);
		let projectId: string;

		if (existing) {
			projectId = existing.id;
			activeSession = existing.id;
		} else {
			// Ajouter le nouveau projet à la liste
			projectId = crypto.randomUUID();
			const newSession: ProjectSession = {
				id: projectId,
				name: projectName,
				project: path,
				timestamp: new Date()
			};
			sessionsList = [newSession, ...sessionsList.slice(0, 9)]; // Garder max 10
			activeSession = projectId;
			saveRecentProjects();
		}

		// Ouvrir dans un onglet terminal (créé ou existant)
		if (terminalComponent) {
			terminalComponent.openProject(path, projectName, projectId);
		}

		// Initialiser les checkpoints pour ce projet
		await setCheckpointProject(path);
		checkpointCount = 0;

		// Rafraîchir les stats pour ce projet
		refreshStats();
	}

	// Callback quand l'onglet actif change dans TerminalTabs
	function handleActiveProjectChange(path: string | null) {
		workingDir = path;
		if (path) {
			const session = sessionsList.find(s => s.project === path);
			if (session) {
				activeSession = session.id;
			}
		}
	}

	async function handleNewProject() {
		try {
			// Demander le nom du projet
			const projectName = prompt('Nom du nouveau projet:');
			if (!projectName || !projectName.trim()) return;

			// Créer le dossier Léon s'il n'existe pas
			const { mkdir, writeTextFile, exists } = await import('@tauri-apps/plugin-fs');

			// Vérifier et créer le dossier parent
			try {
				const dirExists = await exists(LEON_PROJECTS_DIR);
				if (!dirExists) {
					await mkdir(LEON_PROJECTS_DIR, { recursive: true });
					console.log('[Leon] Created projects directory:', LEON_PROJECTS_DIR);
				}
			} catch (e) {
				console.log('[Leon] Creating projects directory...');
				await mkdir(LEON_PROJECTS_DIR, { recursive: true });
			}

			// Créer le chemin complet
			const safeName = projectName.trim().replace(/[<>:"/\\|?*]/g, '_');
			const projectPath = `${LEON_PROJECTS_DIR}\\${safeName}`;

			// Créer le dossier du projet
			await mkdir(projectPath, { recursive: true });

			// Créer README.md
			await writeTextFile(`${projectPath}/README.md`, `# ${projectName.trim()}\n\nProjet créé avec Léon.\n`);

			// Créer .gitignore basique
			const gitignore = `# Dependencies
node_modules/
.pnp/
.pnp.js

# Build
dist/
build/
*.egg-info/
__pycache__/

# Environment
.env
.env.local
.env.*.local

# IDE
.idea/
.vscode/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db

# Logs
*.log
npm-debug.log*
`;
			await writeTextFile(`${projectPath}/.gitignore`, gitignore);

			// Initialiser la configuration Claude Code pour le nouveau projet
			try {
				const { invoke } = await import('@tauri-apps/api/core');
				const templatePath = await getTemplatePath();
				await invoke('init_project_config', {
					projectPath: projectPath,
					projectName: projectName.trim(),
					templatePath: templatePath
				});
				console.log('[NewProject] Config initialized');
			} catch (e) {
				console.error('[NewProject] Error initializing config:', e);
			}

			// Ouvrir le projet (skip la vérification car on vient de l'initialiser)
			await openProject(projectPath, projectName.trim(), true);

			// Attendre que le terminal soit prêt, puis initialiser git
			setTimeout(() => {
				if (terminalComponent) {
					terminalComponent.sendText(`git init && git add -A && git commit -m "Initial commit - Created with Léon"\n`);
				}
			}, 1000);

		} catch (e) {
			console.error('Erreur création projet:', e);
		}
	}

	function handleSelectSession(id: string) {
		const session = sessionsList.find(s => s.id === id);
		if (session) {
			// Ouvrir le projet (crée un onglet ou active l'existant)
			openProject(session.project, session.name);
		}
	}

	function handleRenameProject(id: string, newName: string) {
		sessionsList = sessionsList.map(s =>
			s.id === id ? { ...s, name: newName } : s
		);
		saveRecentProjects();
	}

	function handleTerminalReady() {
		terminalReady = true;
		terminalComponent?.focus();
	}

	async function handleTerminalOutput(text: string) {
		// Détecter les fichiers modifiés
		const detectedFiles = detectModifiedFiles(text);

		// Collecter les fichiers pour checkpoint
		for (const file of detectedFiles) {
			if (!pendingFiles.includes(file)) {
				pendingFiles.push(file);
			}
		}

		// Si une action destructive est détectée, créer un checkpoint
		if (shouldCreateCheckpoint(text) && pendingFiles.length > 0) {
			const cpId = await createCheckpoint(
				`Auto: ${pendingFiles.length} fichier(s)`,
				pendingFiles
			);
			if (cpId) {
				console.log('[Checkpoint] Created:', cpId);
				checkpointCount = await getCheckpointCount();
			}
			pendingFiles = [];
		}

		// Détecter si Claude affiche un plan
		if (planPanelComponent && planPanelComponent.parsePlanFromText(text)) {
			showPlanPanel = true;
			console.log('[Plan] Detected plan in output');
		}

		// Mettre à jour le preview pour le dernier fichier détecté
		if (detectedFiles.length > 0) {
			const lastFile = detectedFiles[detectedFiles.length - 1];
			console.log('[Preview] Detected file:', lastFile);
			updatePreviewForFile(lastFile);
		}

		// Collecter l'output pour l'analyse (garder les 5000 derniers caractères)
		terminalOutput = (terminalOutput + text).slice(-5000);
	}

	async function updatePreviewForFile(filePath: string) {
		try {
			const { readTextFile } = await import('@tauri-apps/plugin-fs');
			const content = await readTextFile(filePath);

			// Détecter le langage depuis l'extension
			const ext = filePath.split('.').pop()?.toLowerCase() || '';
			const languageMap: Record<string, string> = {
				ts: 'typescript', tsx: 'typescript', js: 'javascript', jsx: 'javascript',
				svelte: 'html', vue: 'html', html: 'html', css: 'css', scss: 'scss',
				json: 'json', md: 'markdown', py: 'python', rs: 'rust', go: 'go',
				sql: 'sql', yaml: 'yaml', yml: 'yaml', toml: 'toml', dart: 'dart'
			};

			const language = languageMap[ext] || 'plaintext';

			previewState = {
				...previewState,
				mode: 'code',
				code: {
					filePath,
					content,
					language,
					lineNumbers: true
				}
			};
			showPreview = true;
		} catch (e) {
			console.error('[Preview] Error reading file:', e);
		}
	}

	function handleOpenSettings() {
		showSettings = true;
	}

	function closePreview() {
		showPreview = false;
	}

	function handlePreviewModeChange(mode: PreviewMode) {
		previewState = { ...previewState, mode };
	}

	function handlePreviewStateChange(newState: PreviewState) {
		previewState = newState;
	}

	// Resize panel logic
	let isResizing = $state<'sidebar' | 'preview' | null>(null);
	let startX = 0;
	let startWidth = 0;

	function startResize(panel: 'sidebar' | 'preview', e: MouseEvent) {
		isResizing = panel;
		startX = e.clientX;
		startWidth = panel === 'sidebar' ? sidebarWidth : previewWidth;
		document.body.style.cursor = 'col-resize';
		document.body.style.userSelect = 'none';
	}

	function handleMouseMove(e: MouseEvent) {
		if (!isResizing) return;

		const delta = e.clientX - startX;

		if (isResizing === 'sidebar') {
			const newWidth = Math.max(200, Math.min(400, startWidth + delta));
			sidebarWidth = newWidth;
		} else if (isResizing === 'preview') {
			// Permettre jusqu'à 75% de la largeur de la fenêtre
			const maxWidth = Math.floor(window.innerWidth * 0.75);
			const newWidth = Math.max(350, Math.min(maxWidth, startWidth - delta));
			previewWidth = newWidth;
		}
	}

	function stopResize() {
		if (isResizing) {
			isResizing = null;
			document.body.style.cursor = '';
			document.body.style.userSelect = '';
		}
	}

	// Double-click pour renommer
	function handleDoubleClickSession(session: ProjectSession) {
		const newName = prompt('Renommer le projet:', session.name);
		if (newName && newName.trim()) {
			handleRenameProject(session.id, newName.trim());
		}
	}

	// Créer et pousser une release
	function handleRelease() {
		if (terminalComponent && workingDir) {
			const releaseCommand = 'Crée une nouvelle release Git avec un tag de version approprié et pousse-la sur GitHub.\n';
			terminalComponent.sendText(releaseCommand);
		}
	}

	// Session Resume - Reprendre une session Claude Code précédente
	function handleResumeClaudeSession(sessionId: string) {
		activeClaudeSessionId = sessionId;
		console.log('[Session] Resuming session:', sessionId);

		// Envoyer commande pour reprendre la session
		if (terminalComponent && workingDir) {
			// Tuer la session existante et en démarrer une nouvelle avec --resume
			terminalComponent.sendText('\x03'); // Ctrl+C pour arrêter
			setTimeout(() => {
				terminalComponent?.sendText(`claude --resume ${sessionId}\n`);
			}, 500);
		}
	}

	// Nouvelle session Claude (sans resume)
	function handleNewClaudeSession() {
		activeClaudeSessionId = null;
		console.log('[Session] Starting new session');

		if (terminalComponent && workingDir) {
			// Tuer la session existante et en démarrer une nouvelle
			terminalComponent.sendText('\x03'); // Ctrl+C
			setTimeout(() => {
				terminalComponent?.sendText('claude\n');
			}, 500);
		}
	}

	// Toggle Git Panel
	function handleToggleGit() {
		showGitPanel = !showGitPanel;
		if (showGitPanel) {
			// Fermer les autres panels
			showPreview = false;
			showPlanPanel = false;
			showSnippetsPanel = false;
			showFileExplorer = false;
			refreshGitStatus();
		}
	}

	function handleCloseGitPanel() {
		showGitPanel = false;
	}

	function handleGitFileSelect(filePath: string) {
		// Afficher le fichier sélectionné dans le preview
		if (workingDir) {
			const fullPath = filePath.startsWith(workingDir) ? filePath : `${workingDir}/${filePath}`;
			updatePreviewForFile(fullPath);
		}
	}

	// Toggle File Explorer
	function handleToggleFiles() {
		showFileExplorer = !showFileExplorer;
		if (showFileExplorer) {
			// Fermer les autres panels
			showPreview = false;
			showPlanPanel = false;
			showGitPanel = false;
			showSnippetsPanel = false;
		}
	}

	function handleCloseFileExplorer() {
		showFileExplorer = false;
	}

	function handleFileExplorerSelect(filePath: string) {
		updatePreviewForFile(filePath);
		showFileExplorer = false;
		showPreview = true;
	}

	// Output overlay handlers
	function handleOutputFileClick(path: string, line?: number) {
		// Construire le chemin complet si relatif
		const fullPath = path.startsWith('/') || path.includes(':')
			? path
			: `${workingDir}/${path}`;
		updatePreviewForFile(fullPath);
		showFileExplorer = false;
		showGitPanel = false;
		showPreview = true;
	}

	function handleOutputUrlClick(url: string) {
		// Ouvrir dans le navigateur par défaut
		window.open(url, '_blank');
	}

	function handleCloseOutputOverlay() {
		showOutputOverlay = false;
	}

	// Toggle Snippets Panel
	function handleToggleSnippets() {
		showSnippetsPanel = !showSnippetsPanel;
		if (showSnippetsPanel) {
			// Fermer les autres panels
			showPreview = false;
			showPlanPanel = false;
			showGitPanel = false;
			showFileExplorer = false;
		}
	}

	function handleCloseSnippetsPanel() {
		showSnippetsPanel = false;
	}

	function handleExecuteSnippet(command: string) {
		handleSendCommand(command);
	}

	// Obtenir le chemin du template
	async function getTemplatePath(): Promise<string> {
		try {
			const { resourceDir } = await import('@tauri-apps/api/path');
			const resDir = await resourceDir();
			return resDir + 'templates/project-config';
		} catch {
			// En dev, utiliser le chemin absolu
			return 'C:\\Users\\Marc\\projets\\leon\\templates\\project-config';
		}
	}

	// Project init dialog handlers
	async function handleInitProject() {
		if (!pendingProjectPath) return;

		try {
			const { invoke } = await import('@tauri-apps/api/core');
			const templatePath = await getTemplatePath();

			const result = await invoke<{
				success: boolean;
				files_created: string[];
				files_skipped: string[];
			}>('init_project_config', {
				projectPath: pendingProjectPath,
				projectName: pendingProjectName,
				templatePath: templatePath
			});

			console.log('[Project] Init result:', result);

			// Fermer le dialogue et ouvrir le projet
			showProjectInitDialog = false;
			await doOpenProject(pendingProjectPath, pendingProjectName);

			// Réinitialiser les états
			pendingProjectPath = null;
			pendingProjectName = '';
			pendingProjectConfigStatus = null;

		} catch (e) {
			console.error('[Project] Error initializing:', e);
			// Ouvrir quand même le projet
			showProjectInitDialog = false;
			if (pendingProjectPath) {
				await doOpenProject(pendingProjectPath, pendingProjectName);
			}
		}
	}

	function handleSkipProjectInit() {
		showProjectInitDialog = false;
		if (pendingProjectPath) {
			doOpenProject(pendingProjectPath, pendingProjectName);
		}
		pendingProjectPath = null;
		pendingProjectName = '';
		pendingProjectConfigStatus = null;
	}

	function handleCloseProjectInitDialog() {
		showProjectInitDialog = false;
		pendingProjectPath = null;
		pendingProjectName = '';
		pendingProjectConfigStatus = null;
	}

	// Import an existing project to Leon folder
	async function handleImportProject() {
		try {
			const { open } = await import('@tauri-apps/plugin-dialog');
			const { invoke } = await import('@tauri-apps/api/core');
			const { mkdir, exists } = await import('@tauri-apps/plugin-fs');

			// Sélectionner le projet à importer
			const sourcePath = await open({
				directory: true,
				multiple: false,
				title: 'Sélectionner le projet à importer'
			});

			if (!sourcePath || typeof sourcePath !== 'string') return;

			// Extraire le nom du projet
			const projectName = sourcePath.split(/[/\\]/).pop() || 'projet';

			// Demander copie ou déplacement
			const moveProject = confirm(
				`Importer "${projectName}" vers Leon\\projets\n\n` +
				`Cliquez OK pour DÉPLACER (supprime l'original)\n` +
				`Cliquez Annuler pour COPIER (garde l'original)`
			);

			// S'assurer que le dossier Leon existe
			try {
				const dirExists = await exists(LEON_PROJECTS_DIR);
				if (!dirExists) {
					await mkdir(LEON_PROJECTS_DIR, { recursive: true });
				}
			} catch {
				await mkdir(LEON_PROJECTS_DIR, { recursive: true });
			}

			// Chemin destination
			const destPath = `${LEON_PROJECTS_DIR}\\${projectName}`;

			// Copier/déplacer le projet
			const result = await invoke<{
				success: boolean;
				files_copied: number;
				moved: boolean;
				new_path: string;
			}>('copy_project', {
				sourcePath: sourcePath,
				destPath: destPath,
				moveInsteadOfCopy: moveProject
			});

			console.log('[Import] Result:', result);

			// Initialiser la config Claude Code
			const templatePath = await getTemplatePath();
			await invoke('init_project_config', {
				projectPath: destPath,
				projectName: projectName,
				templatePath: templatePath
			});

			// Ouvrir le projet importé
			await openProject(destPath, projectName, true);

			alert(
				`Projet importé avec succès !\n\n` +
				`${result.files_copied} fichiers ${result.moved ? 'déplacés' : 'copiés'}\n` +
				`Nouveau chemin: ${destPath}`
			);

		} catch (e) {
			console.error('[Import] Error:', e);
			alert(`Erreur lors de l'importation: ${e}`);
		}
	}
</script>

<svelte:window onmousemove={handleMouseMove} onmouseup={stopResize} />

<div class="app-container">
	<div class="main-layout">
		<!-- Sidebar -->
		{#if showSidebar}
			<div class="sidebar-container" style="width: {sidebarWidth}px">
				<Sidebar
					sessions={sessionsList}
					{activeSession}
					onNewProject={handleNewProject}
					onOpenProject={handleOpenFolder}
					onImportProject={handleImportProject}
					onSelectSession={handleSelectSession}
					onOpenSettings={handleOpenSettings}
					onSave={handleSave}
					onRelease={handleRelease}
					onToggleGit={handleToggleGit}
					onToggleFiles={handleToggleFiles}
					onToggleSnippets={handleToggleSnippets}
					onTogglePreview={handleTogglePreview}
					onToggleSidebar={() => showSidebar = false}
					hasActiveProject={!!workingDir}
					gitChanges={gitChangesCount}
					isPreviewOpen={showPreview || showGitPanel || showFileExplorer || showSnippetsPanel || showPlanPanel}
				/>
			</div>

			<!-- Resize handle sidebar -->
			<div
				class="resize-handle sidebar-resize"
				onmousedown={(e) => startResize('sidebar', e)}
				role="separator"
				aria-orientation="vertical"
				tabindex="-1"
			></div>
		{:else}
			<!-- Sidebar collapsed - show expand button -->
			<button class="sidebar-expand-btn" onclick={() => showSidebar = true} title="Afficher la sidebar (Ctrl+B)">
				<ChevronRight size={16} />
			</button>
		{/if}

		<!-- Terminal -->
		<div class="terminal-container">
			{#if workingDir}
				<div class="terminal-header">
					<div class="header-left">
						<span class="project-path">{workingDir}</span>
						<button class="rename-btn" onclick={() => {
							const session = sessionsList.find(s => s.project === workingDir);
							if (session) handleDoubleClickSession(session);
						}}>
							Renommer
						</button>
					</div>
					<div class="header-right">
						{#if activeClaudeSessionId}
							<span class="session-badge" title="Session active">
								<RotateCcw size={10} />
								{activeClaudeSessionId.slice(0, 8)}...
							</span>
						{/if}
						<SessionSelector
							projectPath={workingDir}
							onSelectSession={handleResumeClaudeSession}
							onNewSession={handleNewClaudeSession}
						/>
					</div>
				</div>
				<QuickActions
					projectPath={workingDir}
					hasGitChanges={gitChangesCount > 0}
					contextUsedPercent={contextUsedPercent || 0}
					onSendCommand={handleSendCommand}
				/>
				<div class="terminal-wrapper">
					<TerminalTabs
						bind:this={terminalComponent}
						onReady={handleTerminalReady}
						onOutput={handleTerminalOutput}
						onActiveProjectChange={handleActiveProjectChange}
					/>
					<OutputOverlay
						output={terminalOutput}
						isVisible={showOutputOverlay}
						onFileClick={handleOutputFileClick}
						onUrlClick={handleOutputUrlClick}
						onClose={handleCloseOutputOverlay}
					/>
				</div>
			{:else}
				<div class="welcome-screen">
					<img src="/images/logo.png" alt="Léon" class="welcome-logo" />
					<h1>Bienvenue dans Léon</h1>
					<p>Interface graphique pour Claude Code</p>

					{#if claudeAvailable}
						<p class="claude-status success">
							Claude Code CLI détecté {claudeVersion ? `(${claudeVersion})` : ''}
						</p>
					{:else}
						<p class="claude-status error">
							Claude Code CLI non détecté
						</p>
						<p class="install-hint">
							Installez-le avec: <code>npm install -g @anthropic-ai/claude-code</code>
						</p>
					{/if}

					<div class="welcome-actions">
						<button class="new-project-btn" onclick={handleNewProject} disabled={!claudeAvailable}>
							<Plus size={20} />
							<span>Nouveau projet</span>
						</button>
						<button class="open-project-btn" onclick={handleOpenFolder} disabled={!claudeAvailable}>
							<FolderOpen size={20} />
							<span>Ouvrir un projet</span>
						</button>
					</div>

					{#if sessionsList.length > 0}
						<div class="recent-projects">
							<h3>Projets récents</h3>
							<ul>
								{#each sessionsList.slice(0, 5) as session (session.id)}
									<li>
										<button onclick={() => handleSelectSession(session.id)}>
											<span class="session-name">{session.name}</span>
											<span class="session-path">{session.project}</span>
										</button>
									</li>
								{/each}
							</ul>
						</div>
					{/if}
				</div>
			{/if}
		</div>

		<!-- Snippets Panel (conditional) -->
		{#if showSnippetsPanel && workingDir}
			<div
				class="resize-handle preview-resize"
				onmousedown={(e) => startResize('preview', e)}
				role="separator"
				aria-orientation="vertical"
				tabindex="-1"
			></div>

			<div class="preview-container" style="width: {previewWidth}px">
				<SnippetsPanel
					isVisible={showSnippetsPanel}
					onClose={handleCloseSnippetsPanel}
					onExecute={handleExecuteSnippet}
				/>
			</div>
		<!-- File Explorer (conditional) -->
		{:else if showFileExplorer && workingDir}
			<div
				class="resize-handle preview-resize"
				onmousedown={(e) => startResize('preview', e)}
				role="separator"
				aria-orientation="vertical"
				tabindex="-1"
			></div>

			<div class="preview-container" style="width: {previewWidth}px">
				<FileExplorer
					projectPath={workingDir}
					isVisible={showFileExplorer}
					onClose={handleCloseFileExplorer}
					onFileSelect={handleFileExplorerSelect}
				/>
			</div>
		<!-- Git Panel (conditional) -->
		{:else if showGitPanel && workingDir}
			<div
				class="resize-handle preview-resize"
				onmousedown={(e) => startResize('preview', e)}
				role="separator"
				aria-orientation="vertical"
				tabindex="-1"
			></div>

			<div class="preview-container" style="width: {previewWidth}px">
				<GitPanel
					projectPath={workingDir}
					isVisible={showGitPanel}
					onClose={handleCloseGitPanel}
					onFileSelect={handleGitFileSelect}
				/>
			</div>
		<!-- Plan Panel (conditional) -->
		{:else if showPlanPanel && workingDir}
			<div
				class="resize-handle preview-resize"
				onmousedown={(e) => startResize('preview', e)}
				role="separator"
				aria-orientation="vertical"
				tabindex="-1"
			></div>

			<div class="preview-container" style="width: {previewWidth}px">
				<PlanPanel
					bind:this={planPanelComponent}
					isVisible={showPlanPanel}
					onClose={() => showPlanPanel = false}
				/>
			</div>
		<!-- Preview (conditional) -->
		{:else if showPreview && workingDir}
			<!-- Resize handle preview -->
			<div
				class="resize-handle preview-resize"
				onmousedown={(e) => startResize('preview', e)}
				role="separator"
				aria-orientation="vertical"
				tabindex="-1"
			></div>

			<div class="preview-container" style="width: {previewWidth}px">
				<PreviewPanel
					state={previewState}
					onClose={closePreview}
					onModeChange={handlePreviewModeChange}
					onStateChange={handlePreviewStateChange}
				/>
			</div>
		{/if}
	</div>

	<!-- Status Bar -->
	<StatusBar
		project={workingDir?.split(/[/\\]/).pop() || 'Aucun projet'}
		model={claudeVersion || 'Claude Code'}
		sessionId={activeSession}
		tokensUsed={{ input: 0, output: 0 }}
		status={terminalReady ? 'idle' : 'thinking'}
		{contextUsedPercent}
		{sessionMessages}
		{todayMessages}
		{weeklyMessages}
		{checkpointCount}
		onUndo={handleUndo}
		onShowCheckpoints={handleShowCheckpoints}
		onCompact={handleCompact}
	/>
</div>

<!-- Settings Modal -->
<SettingsModal isOpen={showSettings} onClose={() => showSettings = false} />

<!-- Command Palette -->
<CommandPalette
	isOpen={showCommandPalette}
	onClose={() => showCommandPalette = false}
	onOpenFolder={handleOpenFolder}
	onNewProject={handleNewProject}
	onSave={handleSave}
	onUndo={handleUndo}
	onSettings={() => showSettings = true}
	onCompact={handleCompact}
	onClearTerminal={handleClearTerminal}
	onSendCommand={handleSendCommand}
/>

<!-- Project Init Dialog -->
<ProjectInitDialog
	show={showProjectInitDialog}
	projectPath={pendingProjectPath || ''}
	projectName={pendingProjectName}
	configStatus={pendingProjectConfigStatus}
	onInit={handleInitProject}
	onSkip={handleSkipProjectInit}
	onClose={handleCloseProjectInitDialog}
/>

<style>
	.app-container {
		display: flex;
		flex-direction: column;
		height: 100vh;
		overflow: hidden;
	}

	.main-layout {
		display: flex;
		flex: 1;
		min-height: 0;
	}

	.sidebar-container {
		flex-shrink: 0;
		min-width: 200px;
		max-width: 400px;
	}

	.terminal-container {
		flex: 1;
		min-width: 400px;
		display: flex;
		flex-direction: column;
		background-color: #1a1a1a;
	}

	.terminal-wrapper {
		flex: 1;
		position: relative;
		display: flex;
		flex-direction: column;
		min-height: 0;
	}

	.terminal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 8px 12px;
		background-color: #252525;
		border-bottom: 1px solid var(--color-border);
		font-size: 0.75rem;
		color: var(--color-text-secondary);
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.header-right {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.project-path {
		font-family: monospace;
	}

	.session-badge {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.125rem 0.375rem;
		background: var(--color-lion-900);
		border: 1px solid var(--color-lion-700);
		border-radius: 4px;
		font-size: 0.65rem;
		font-family: 'JetBrains Mono', monospace;
		color: var(--color-lion-300);
	}

	.rename-btn {
		padding: 2px 8px;
		font-size: 0.7rem;
		background: var(--color-bg-hover);
		border: 1px solid var(--color-border);
		border-radius: 4px;
		color: var(--color-text-secondary);
		cursor: pointer;
	}

	.rename-btn:hover {
		background: var(--color-lion-900);
		color: var(--color-lion-300);
	}

	.preview-container {
		flex-shrink: 0;
		min-width: 350px;
		max-width: 75vw; /* Permet jusqu'à 75% de la largeur de la fenêtre */
	}

	.sidebar-expand-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 100%;
		min-height: 100%;
		background-color: var(--color-bg-secondary);
		border: none;
		border-right: 1px solid var(--color-border);
		color: var(--color-text-muted);
		cursor: pointer;
		transition: all 0.15s;
	}

	.sidebar-expand-btn:hover {
		background-color: var(--color-bg-hover);
		color: var(--color-lion-400);
	}

	.resize-handle {
		width: 4px;
		cursor: col-resize;
		background-color: transparent;
		transition: background-color 0.15s;
	}

	.resize-handle:hover {
		background-color: var(--color-lion-500);
	}

	.sidebar-resize {
		border-right: 1px solid var(--color-border);
	}

	.preview-resize {
		border-left: 1px solid var(--color-border);
	}

	/* Welcome screen */
	.welcome-screen {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		padding: 2rem;
		text-align: center;
	}

	.welcome-logo {
		width: 120px;
		height: auto;
		margin-bottom: 1rem;
	}

	.welcome-screen h1 {
		font-size: 1.75rem;
		font-weight: 600;
		color: var(--color-text-primary);
		margin: 0;
	}

	.welcome-screen p {
		color: var(--color-text-secondary);
		margin: 0;
	}

	.claude-status {
		padding: 0.5rem 1rem;
		border-radius: 6px;
		font-size: 0.875rem;
	}

	.claude-status.success {
		background-color: rgba(105, 219, 124, 0.1);
		color: #69db7c;
	}

	.claude-status.error {
		background-color: rgba(255, 107, 107, 0.1);
		color: #ff6b6b;
	}

	.install-hint {
		font-size: 0.875rem;
	}

	.install-hint code {
		background-color: #2a2a2a;
		padding: 0.25rem 0.5rem;
		border-radius: 4px;
		font-family: monospace;
	}

	.welcome-actions {
		display: flex;
		gap: 1rem;
		margin-top: 1.5rem;
	}

	.welcome-actions .new-project-btn,
	.welcome-actions .open-project-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		color: var(--color-text-primary);
		border: none;
		border-radius: 8px;
		font-size: 1rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
	}

	.welcome-actions .new-project-btn {
		background-color: var(--color-lion-600);
	}

	.welcome-actions .new-project-btn:hover:not(:disabled) {
		background-color: var(--color-lion-500);
	}

	.welcome-actions .open-project-btn {
		background-color: var(--color-bg-secondary);
		border: 1px solid var(--color-border);
	}

	.welcome-actions .open-project-btn:hover:not(:disabled) {
		background-color: var(--color-bg-hover);
		border-color: var(--color-lion-500);
	}

	.welcome-actions button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Recent projects */
	.recent-projects {
		margin-top: 2rem;
		text-align: left;
		width: 100%;
		max-width: 400px;
	}

	.recent-projects h3 {
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--color-text-secondary);
		margin: 0 0 0.75rem 0;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.recent-projects ul {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.recent-projects li button {
		display: flex;
		flex-direction: column;
		width: 100%;
		padding: 0.75rem;
		background: var(--color-bg-secondary);
		border: 1px solid var(--color-border);
		border-radius: 6px;
		cursor: pointer;
		text-align: left;
		margin-bottom: 0.5rem;
		transition: all 0.15s;
	}

	.recent-projects li button:hover {
		background: var(--color-bg-hover);
		border-color: var(--color-lion-500);
	}

	.session-name {
		font-weight: 500;
		color: var(--color-text-primary);
	}

	.session-path {
		font-size: 0.75rem;
		color: var(--color-text-muted);
		font-family: monospace;
		margin-top: 0.25rem;
	}
</style>
