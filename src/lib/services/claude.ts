// Service de communication avec Claude Code CLI via Tauri
// Mode INTERACTIF avec session persistante
import type { ClaudeEvent } from '$lib/types/claude';
import {
  handleClaudeEvent,
  addUserMessage,
  addSystemMessage,
  updateSession,
  isLoading
} from '$lib/stores/chat';

let eventUnlisten: (() => void) | null = null;
let tauriAvailable: boolean | null = null;
let sessionActive = false;

// Initialise l'écoute des events Claude
export async function initClaudeListener(): Promise<void> {
  try {
    const { listen } = await import('@tauri-apps/api/event');

    if (eventUnlisten) {
      eventUnlisten();
    }

    eventUnlisten = await listen<ClaudeEvent>('claude_event', (event) => {
      console.log('[Claude] Event reçu:', event.payload);
      handleClaudeEvent(event.payload);

      // Tracker l'état de la session
      if (event.payload.event_type === 'session_started') {
        sessionActive = true;
        updateSession({ isRunning: true });
      } else if (event.payload.event_type === 'session_ended') {
        sessionActive = false;
        updateSession({ isRunning: false });
        isLoading.set(false);
      }
    });

    tauriAvailable = true;
    console.log('[Claude] Listener initialisé');

    // Vérifier si une session est déjà active
    sessionActive = await checkSessionActive();
  } catch (error) {
    console.warn('[Claude] Tauri non disponible:', error);
    tauriAvailable = false;
  }
}

// Nettoie l'écoute
export function cleanupClaudeListener(): void {
  if (eventUnlisten) {
    eventUnlisten();
    eventUnlisten = null;
  }
}

// Envoie un message à Claude (démarre une session si nécessaire)
export async function sendPrompt(
  prompt: string,
  workingDir?: string,
  continueSession?: string
): Promise<void> {
  // Ajouter le message utilisateur au chat
  addUserMessage(prompt);
  isLoading.set(true);

  if (tauriAvailable === false) {
    addSystemMessage('Léon doit être lancé dans Tauri, pas dans le navigateur', true);
    isLoading.set(false);
    return;
  }

  console.log('[Claude] Envoi message:', prompt, '| Session active:', sessionActive);

  try {
    const { invoke } = await import('@tauri-apps/api/core');

    if (sessionActive) {
      // Session déjà active -> envoyer un message de suivi
      await invoke('send_claude_message', { message: prompt });
      console.log('[Claude] Message envoyé à la session active');
    } else {
      // Pas de session -> en démarrer une nouvelle
      const result = await invoke('start_claude_session', {
        prompt,
        workingDir: workingDir ?? null,
        continueSession: continueSession ?? null
      });
      console.log('[Claude] Nouvelle session démarrée:', result);
    }
  } catch (error) {
    console.error('[Claude] Erreur:', error);
    handleClaudeEvent({
      event_type: 'error',
      message: `Erreur: ${error}`
    });
    isLoading.set(false);
  }
}

// Arrête la session en cours
export async function stopSession(): Promise<void> {
  if (tauriAvailable === false) return;

  try {
    const { invoke } = await import('@tauri-apps/api/core');
    await invoke('stop_claude_session');
    sessionActive = false;
    updateSession({ isRunning: false });
    isLoading.set(false);
    console.log('[Claude] Session arrêtée');
  } catch (error) {
    console.error('Erreur arrêt session:', error);
  }
}

// Vérifie si une session est active (côté backend)
export async function checkSessionActive(): Promise<boolean> {
  if (tauriAvailable === false) return false;

  try {
    const { invoke } = await import('@tauri-apps/api/core');
    const result = await invoke<boolean>('is_session_active');
    sessionActive = result;
    return result;
  } catch {
    return false;
  }
}

// Récupère l'ID de session actuel
export async function getSessionId(): Promise<string | null> {
  if (tauriAvailable === false) return null;

  try {
    const { invoke } = await import('@tauri-apps/api/core');
    return await invoke<string | null>('get_session_id');
  } catch {
    return null;
  }
}

// Vérifie si Claude CLI est disponible
export async function checkClaudeAvailable(): Promise<boolean> {
  if (tauriAvailable === false) return false;

  try {
    const { invoke } = await import('@tauri-apps/api/core');
    const result = await invoke<boolean>('check_claude_available');
    console.log('[Claude] CLI disponible:', result);
    return result;
  } catch (error) {
    console.error('[Claude] Erreur check CLI:', error);
    return false;
  }
}

// Récupère la version de Claude CLI
export async function getClaudeVersion(): Promise<string | null> {
  if (tauriAvailable === false) return null;

  try {
    const { invoke } = await import('@tauri-apps/api/core');
    return await invoke<string>('get_claude_version');
  } catch {
    return null;
  }
}

// Expose l'état de la session pour le frontend
export function isSessionActive(): boolean {
  return sessionActive;
}
