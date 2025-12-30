# Léon - Claude Code Desktop UI

## SÉCURITÉ - À LIRE EN PREMIER

### Zones Protégées (INTERDIT)
```
C:\Windows\**           C:\Program Files\**
C:\ProgramData\**       C:\Users\*\AppData\Local\Microsoft\**
```

### Zones Sensibles (DEMANDER CONFIRMATION)
```
.ssh/   .aws/   .env   *credentials*   *secret*   *.pem   *.key
```

### Zone de Travail Autorisée
```
C:\Users\Marc\projets\leon\**  (ce projet uniquement)
```

### Commandes Interdites
`rm -rf /`, `del /f /s /q C:\`, `git push --force main`, `format`, `diskpart`

**Règle d'or** : En cas de doute → DEMANDER avant d'exécuter.

Voir `.claude/rules/security.md` pour les règles complètes.

---

## Identité du Projet
**Léon** est une interface graphique desktop pour Claude Code CLI, construite avec Tauri 2 + SvelteKit 5. L'objectif est de créer l'outil de développement AI le plus efficace possible.

## Stack Technique
| Couche | Technologie |
|--------|-------------|
| Desktop | Tauri 2.9.5 (Rust) |
| Frontend | SvelteKit 5 + Svelte 5 |
| Terminal | xterm.js 6 + ConPTY |
| Styling | Tailwind CSS 4 |
| Build | Vite 7 |

## Architecture Critique

### Frontend (`src/`)
```
src/
├── routes/+page.svelte       # Page principale (layout 3 panneaux)
├── lib/
│   ├── components/
│   │   ├── terminal/         # Terminal PTY natif
│   │   ├── preview/          # Preview Code/Diff/Web/App
│   │   └── layout/           # Sidebar, StatusBar, Settings
│   ├── services/             # claude.ts, stats.ts
│   ├── stores/               # État Svelte (chat.ts)
│   └── types/                # TypeScript definitions
```

### Backend Rust (`src-tauri/src/`)
```
src-tauri/src/
├── lib.rs                    # Entry point + state management
├── commands/
│   ├── mod.rs               # Export tous les commands
│   ├── claude.rs            # Session Claude interactive
│   ├── pty.rs               # Terminal PTY (ConPTY)
│   ├── stats.rs             # Lecture ~/.claude/stats
│   └── windows_capture.rs   # Capture fenêtres Windows
```

## Conventions de Code

### Svelte 5
- Utiliser `$state()`, `$derived()`, `$effect()` (pas les anciens stores)
- Props avec `let { prop } = $props<Type>()`
- Pas de `export let` (Svelte 4 deprecated)

### Rust/Tauri
- Commands async avec `#[command]`
- État partagé via `app.state::<Arc<Mutex<T>>>()`
- Logs avec `log::info!()`, `log::error!()`
- Émission events: `app.emit("event_name", payload)`

### TypeScript
- Types stricts, pas de `any`
- Imports Tauri: `import { invoke } from '@tauri-apps/api/core'`
- Events Tauri: `import { listen } from '@tauri-apps/api/event'`

## Patterns Importants

### Communication Frontend ↔ Backend
```typescript
// Frontend → Backend
const result = await invoke<ReturnType>('command_name', { param });

// Backend → Frontend (events)
app.emit("event_name", serde_json::json!({ "data": value }));

// Frontend écoute
const unlisten = await listen<PayloadType>('event_name', (event) => {
    console.log(event.payload);
});
```

### Gestion PTY
```rust
// Créer PTY → retourne pty_id
// Écrire: write_pty(pty_id, data)
// Resize: resize_pty(pty_id, cols, rows)
// Kill: kill_pty(pty_id)
// Events: pty_data, pty_exit
```

## Règles Strictes

1. **Pas de fichiers temporaires** - Tout en mémoire ou localStorage
2. **Pas de dépendances inutiles** - Vérifier avant d'ajouter
3. **Pas de console.log en prod** - Utiliser un flag DEBUG
4. **Erreurs explicites** - Pas de silent fail, toujours afficher à l'utilisateur
5. **Performance** - Debounce les events fréquents (resize, scroll, output)

## Fichiers à Ne Jamais Modifier
- `src-tauri/tauri.conf.json` (sauf pour les capabilities)
- `package-lock.json` (auto-généré)
- `Cargo.lock` (auto-généré)

## Mémoire Projet
Consulter `.claude/memory/` pour :
- `context.md` - État technique actuel, décisions récentes
- `decisions.md` - Choix d'architecture avec justifications
- `index.md` - Index et règles de gestion mémoire

## Agents Spécialisés

Claude Code utilise automatiquement ces agents pour optimiser le contexte. Chaque agent a des instructions spécialisées et utilise le modèle approprié (haiku pour rapidité, sonnet pour analyse approfondie).

### Frontend (haiku)
| Agent | Déclenchement automatique |
|-------|---------------------------|
| **svelte-expert** | Questions Svelte 5, runes, SvelteKit, composants .svelte |
| **css-stylist** | Styles CSS, layout flexbox/grid, responsive, animations |
| **component-builder** | Création de composants, refactoring, design d'API |
| **accessibility-checker** | Vérification a11y, ARIA, navigation clavier |

### Backend (haiku)
| Agent | Déclenchement automatique |
|-------|---------------------------|
| **tauri-expert** | Tauri 2.x config, invoke, plugins, capabilities |
| **rust-expert** | Code Rust, ownership, Serde, async/tokio |

### Qualité (haiku/sonnet)
| Agent | Déclenchement automatique |
|-------|---------------------------|
| **typescript-fixer** | Erreurs TypeScript à corriger rapidement |
| **error-resolver** | Messages d'erreur, stack traces, erreurs build |
| **code-reviewer** | Revue de code après écriture significative |
| **test-writer** | Écriture de tests Vitest/Jest/Playwright |
| **bug-hunter** (sonnet) | Investigation bugs complexes, memory leaks |
| **performance-optimizer** (sonnet) | Optimisation performance, bundle size |
| **refactoring-assistant** | Refactoring, extraction, simplification |

### DevOps (haiku)
| Agent | Déclenchement automatique |
|-------|---------------------------|
| **git-assistant** | Conflits merge, rebase, cherry-pick, bisect |
| **dependency-analyzer** | Audit sécurité, mise à jour packages, bundle size |

### Documentation (haiku)
| Agent | Déclenchement automatique |
|-------|---------------------------|
| **doc-writer** | Documentation technique, JSDoc, README, Changelog |

Les agents sont définis dans `.claude/agents/` et se lancent automatiquement selon le contexte de la tâche.

## Commandes Disponibles
- `/save` - Commit + push les changements
- `/build` - Compiler l'application
- `/dev` - Lancer en mode développement
- `/checkpoint` - Sauvegarder l'état mémoire
