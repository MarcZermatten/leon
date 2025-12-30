<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { Terminal } from '@xterm/xterm';
	import { FitAddon } from '@xterm/addon-fit';
	import { WebLinksAddon } from '@xterm/addon-web-links';
	import '@xterm/xterm/css/xterm.css';

	let { workingDir = null, onReady = () => {} } = $props<{
		workingDir: string | null;
		onReady: () => void;
	}>();

	let terminalContainer: HTMLDivElement;
	let terminal: Terminal;
	let fitAddon: FitAddon;
	let ptyId: string | null = null;
	let unlistenData: (() => void) | null = null;
	let unlistenExit: (() => void) | null = null;

	onMount(async () => {
		// Créer le terminal xterm.js
		terminal = new Terminal({
			theme: {
				background: '#1a1a1a',
				foreground: '#e0e0e0',
				cursor: '#d4a574',
				cursorAccent: '#1a1a1a',
				selectionBackground: '#d4a57444',
				black: '#1a1a1a',
				red: '#ff6b6b',
				green: '#69db7c',
				yellow: '#ffd43b',
				blue: '#74c0fc',
				magenta: '#da77f2',
				cyan: '#66d9e8',
				white: '#e0e0e0',
				brightBlack: '#495057',
				brightRed: '#ff8787',
				brightGreen: '#8ce99a',
				brightYellow: '#ffe066',
				brightBlue: '#a5d8ff',
				brightMagenta: '#e599f7',
				brightCyan: '#99e9f2',
				brightWhite: '#f8f9fa'
			},
			fontFamily: '"Cascadia Code", "Fira Code", Consolas, monospace',
			fontSize: 14,
			lineHeight: 1.2,
			cursorBlink: true,
			cursorStyle: 'bar',
			scrollback: 10000,
			allowProposedApi: true
		});

		fitAddon = new FitAddon();
		terminal.loadAddon(fitAddon);
		terminal.loadAddon(new WebLinksAddon());

		terminal.open(terminalContainer);
		fitAddon.fit();

		// Observer le resize
		const resizeObserver = new ResizeObserver(() => {
			fitAddon.fit();
			if (ptyId) {
				resizePty();
			}
		});
		resizeObserver.observe(terminalContainer);

		// Démarrer le PTY
		await startPty();

		// Écouter les entrées utilisateur
		terminal.onData((data) => {
			sendToPty(data);
		});

		onReady();

		return () => {
			resizeObserver.disconnect();
		};
	});

	onDestroy(async () => {
		if (unlistenData) unlistenData();
		if (unlistenExit) unlistenExit();
		if (ptyId) {
			try {
				const { invoke } = await import('@tauri-apps/api/core');
				await invoke('kill_pty', { ptyId });
			} catch (e) {
				console.error('Error killing PTY:', e);
			}
		}
		terminal?.dispose();
	});

	async function startPty() {
		try {
			const { invoke } = await import('@tauri-apps/api/core');
			const { listen } = await import('@tauri-apps/api/event');

			// Démarrer le PTY avec Claude Code
			const result = await invoke<{ pty_id: string }>('start_pty', {
				workingDir: workingDir,
				cols: terminal.cols,
				rows: terminal.rows
			});

			ptyId = result.pty_id;
			console.log('[Terminal] PTY started:', ptyId);

			// Écouter les données du PTY
			unlistenData = await listen<{ pty_id: string; data: number[] }>('pty_data', (event) => {
				if (event.payload.pty_id === ptyId) {
					const bytes = new Uint8Array(event.payload.data);
					terminal.write(bytes);
				}
			});

			// Écouter la fin du PTY
			unlistenExit = await listen<{ pty_id: string; code: number }>('pty_exit', (event) => {
				if (event.payload.pty_id === ptyId) {
					terminal.writeln(`\r\n[Process exited with code ${event.payload.code}]`);
					ptyId = null;
				}
			});

		} catch (e) {
			console.error('[Terminal] Error starting PTY:', e);
			terminal.writeln(`Error: ${e}`);
			terminal.writeln('Make sure Claude Code CLI is installed.');
		}
	}

	async function sendToPty(data: string) {
		if (!ptyId) return;
		try {
			const { invoke } = await import('@tauri-apps/api/core');
			await invoke('write_pty', { ptyId, data });
		} catch (e) {
			console.error('[Terminal] Error writing to PTY:', e);
		}
	}

	async function resizePty() {
		if (!ptyId) return;
		try {
			const { invoke } = await import('@tauri-apps/api/core');
			await invoke('resize_pty', {
				ptyId,
				cols: terminal.cols,
				rows: terminal.rows
			});
		} catch (e) {
			console.error('[Terminal] Error resizing PTY:', e);
		}
	}

	// Méthode publique pour focus le terminal
	export function focus() {
		terminal?.focus();
	}
</script>

<div class="terminal-wrapper" bind:this={terminalContainer}></div>

<style>
	.terminal-wrapper {
		width: 100%;
		height: 100%;
		background-color: #1a1a1a;
		padding: 8px;
		box-sizing: border-box;
	}

	.terminal-wrapper :global(.xterm) {
		height: 100%;
	}

	.terminal-wrapper :global(.xterm-viewport) {
		overflow-y: auto !important;
	}

	.terminal-wrapper :global(.xterm-viewport::-webkit-scrollbar) {
		width: 8px;
	}

	.terminal-wrapper :global(.xterm-viewport::-webkit-scrollbar-track) {
		background: #2a2a2a;
	}

	.terminal-wrapper :global(.xterm-viewport::-webkit-scrollbar-thumb) {
		background: #4a4a4a;
		border-radius: 4px;
	}

	.terminal-wrapper :global(.xterm-viewport::-webkit-scrollbar-thumb:hover) {
		background: #5a5a5a;
	}
</style>
