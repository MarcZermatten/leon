// Store pour le chat avec Claude
// Utilise writable store Svelte classique pour compatibilité

import { writable, derived, get } from 'svelte/store';
import type { ChatMessage, SessionState, ClaudeEvent, UsageInfo } from '$lib/types/claude';

// Stores de base
export const messages = writable<ChatMessage[]>([]);
export const inputValue = writable('');
export const isLoading = writable(false);
export const currentStreamingId = writable<string | null>(null);

export const session = writable<SessionState>({
  id: null,
  isRunning: false,
  totalInputTokens: 0,
  totalOutputTokens: 0,
  workingDirectory: null
});

// Génère un ID unique
function generateId(): string {
  return `msg-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
}

// Ajoute un message utilisateur
export function addUserMessage(content: string): void {
  const message: ChatMessage = {
    id: generateId(),
    role: 'user',
    content,
    timestamp: new Date()
  };
  messages.update(msgs => [...msgs, message]);
}

// Ajoute ou met à jour un message assistant (streaming)
export function updateAssistantMessage(content: string, partial: boolean): void {
  const streamingId = get(currentStreamingId);

  if (partial && streamingId) {
    // Mettre à jour le message existant
    messages.update(msgs => {
      const idx = msgs.findIndex(m => m.id === streamingId);
      if (idx >= 0) {
        msgs[idx] = { ...msgs[idx], content: msgs[idx].content + content };
      }
      return [...msgs];
    });
  } else if (partial) {
    // Nouveau message streaming
    const message: ChatMessage = {
      id: generateId(),
      role: 'assistant',
      content,
      timestamp: new Date(),
      isStreaming: true
    };
    messages.update(msgs => [...msgs, message]);
    currentStreamingId.set(message.id);
  } else {
    // Message final
    if (streamingId) {
      messages.update(msgs => {
        const idx = msgs.findIndex(m => m.id === streamingId);
        if (idx >= 0) {
          msgs[idx] = {
            ...msgs[idx],
            content: msgs[idx].content + content,
            isStreaming: false
          };
        }
        return [...msgs];
      });
      currentStreamingId.set(null);
    } else {
      const message: ChatMessage = {
        id: generateId(),
        role: 'assistant',
        content,
        timestamp: new Date()
      };
      messages.update(msgs => [...msgs, message]);
    }
  }
}

// Ajoute un message d'utilisation d'outil
export function addToolMessage(name: string, input: unknown, isStart: boolean): void {
  const message: ChatMessage = {
    id: generateId(),
    role: 'tool',
    content: isStart ? `Utilisation de ${name}...` : '',
    timestamp: new Date(),
    toolName: name,
    toolInput: input,
    isStreaming: isStart
  };
  messages.update(msgs => [...msgs, message]);

  if (isStart) {
    currentStreamingId.set(message.id);
  }
}

// Met à jour le résultat d'un outil
export function updateToolResult(content: string, isError: boolean): void {
  const streamingId = get(currentStreamingId);
  if (streamingId) {
    messages.update(msgs => {
      const idx = msgs.findIndex(m => m.id === streamingId);
      if (idx >= 0) {
        msgs[idx] = {
          ...msgs[idx],
          content,
          isStreaming: false,
          isError
        };
      }
      return [...msgs];
    });
    currentStreamingId.set(null);
  }
}

// Ajoute un message système/erreur
export function addSystemMessage(content: string, isError = false): void {
  const message: ChatMessage = {
    id: generateId(),
    role: 'system',
    content,
    timestamp: new Date(),
    isError
  };
  messages.update(msgs => [...msgs, message]);
}

// Met à jour l'état de la session
export function updateSession(updates: Partial<SessionState>): void {
  session.update(s => ({ ...s, ...updates }));
}

// Met à jour les tokens
export function updateUsage(usage: UsageInfo): void {
  session.update(s => ({
    ...s,
    totalInputTokens: s.totalInputTokens + usage.input_tokens,
    totalOutputTokens: s.totalOutputTokens + usage.output_tokens
  }));
}

// Efface tous les messages
export function clearMessages(): void {
  messages.set([]);
  currentStreamingId.set(null);
}

// Reset complet
export function resetChat(): void {
  messages.set([]);
  inputValue.set('');
  isLoading.set(false);
  currentStreamingId.set(null);
  session.set({
    id: null,
    isRunning: false,
    totalInputTokens: 0,
    totalOutputTokens: 0,
    workingDirectory: null
  });
}

// Traite un event Claude
export function handleClaudeEvent(event: ClaudeEvent): void {
  switch (event.event_type) {
    case 'session_started':
      updateSession({
        id: event.session_id ?? null,
        isRunning: true
      });
      break;

    case 'assistant_text':
      updateAssistantMessage(event.content ?? '', event.partial ?? false);
      break;

    case 'tool_start':
      addToolMessage(event.name ?? 'Unknown', event.input, true);
      break;

    case 'tool_end':
      updateToolResult(event.content ?? '', event.is_error ?? false);
      break;

    case 'session_ended':
      updateSession({ isRunning: false });
      if (event.usage) {
        updateUsage(event.usage);
      }
      isLoading.set(false);
      break;

    case 'error':
      addSystemMessage(event.message ?? 'Erreur inconnue', true);
      isLoading.set(false);
      break;
  }
}
