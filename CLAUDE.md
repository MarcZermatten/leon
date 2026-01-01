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

### SIT / Géospatial (haiku/sonnet)
| Agent | Déclenchement automatique |
|-------|---------------------------|
| **python-geo** | Scripts Python géospatiaux (GeoPandas, Shapely, Fiona, Rasterio) |
| **qgis-expert** | QGIS, PyQGIS, expressions, plugins, projets .qgz, styles QML |
| **fme-etl** | FME Workbench, transformers, pipelines ETL, fichiers .fmw |
| **postgis-sql** (sonnet) | PostgreSQL/PostGIS, SQL spatial, optimisation, SRID 2056 |
| **interlis-expert** | INTERLIS 1/2, modèles MD.*, validation XTF/ITF, MOpublic |
| **cartographie** | Sémiologie graphique, styles SLD/QML, publication WMS/WFS |

### Données & Intégration (haiku)
| Agent | Déclenchement automatique |
|-------|---------------------------|
| **excel-data** | Fichiers Excel/CSV, import/export, tableaux croisés |
| **api-integrator** | APIs REST, services web cantonaux/fédéraux, WMS/WFS |
| **pdf-reports** | Génération rapports PDF, extraction données, templates |

### Domaines métier (haiku/sonnet)
| Agent | Déclenchement automatique |
|-------|---------------------------|
| **3d-bim** | CityGML, IFC, LiDAR, MNT/MNS, maquettes 3D |
| **network-infra** | Canalisations, réseaux, éclairage, SIA 405 |
| **urbanisme** (sonnet) | PGA, PPA, zones, indices IUS/IOS, procédures |
| **environnement** (sonnet) | Zones protection, biodiversité, bruit, sols pollués |
| **legal-ch** (sonnet) | Droit suisse (LAT, OAT, RF, CC), cadastre, servitudes |
| **stats-analyst** | Statistiques, démographie, indicateurs territoriaux |

Les agents sont définis dans `.claude/agents/` et se lancent automatiquement selon le contexte de la tâche.

## MCP Servers Disponibles

| Serveur | Usage |
|---------|-------|
| `postgres-bussigny` | Connexion PostGIS srv-fme |
| `filesystem` | Accès fichiers projets |
| `memory` | Persistance mémoire sessions |
| `github` | Repos, issues, PRs |
| `fetch` | Requêtes HTTP/APIs |
| `sequential-thinking` | Raisonnement complexe |
| `playwright` | Automatisation web |
| `time` | Dates, délais, planification |
| `gdrive` | Documents Google Drive |

## Commandes Disponibles
- `/save` - Commit + push les changements
- `/build` - Compiler l'application
- `/dev` - Lancer en mode développement
- `/checkpoint` - Sauvegarder l'état mémoire

## Skills SIT Disponibles

Les skills sont des commandes spécialisées pour les tâches géospatiales courantes. Ils sont définis dans `.claude/commands/`.

### Données & ETL
| Skill | Description | Exemple |
|-------|-------------|---------|
| `/import-csv` | Importer CSV dans PostGIS avec détection géo | `/import-csv adresses.csv` |
| `/export` | Exporter table PostGIS vers fichier | `/export geo.parcelles gpkg` |
| `/convert` | Convertir entre formats (shp, gpkg, dxf...) | `/convert data.shp geojson` |
| `/validate-xtf` | Valider fichier INTERLIS | `/validate-xtf cadastre.xtf` |
| `/geocode` | Géocoder adresses suisses (swisstopo) | `/geocode "Rue du Simplon 1, Bussigny"` |

### PostGIS
| Skill | Description | Exemple |
|-------|-------------|---------|
| `/sql` | Exécuter requête SQL ou langage naturel | `/sql parcelles > 1000m2 zone habitat` |
| `/describe` | Décrire structure table/schema | `/describe geo.parcelles` |
| `/spatial-check` | Vérifier qualité géométries | `/spatial-check geo.batiments --fix` |
| `/backup` | Sauvegarder table/schema | `/backup geo.parcelles` |

### QGIS
| Skill | Description | Exemple |
|-------|-------------|---------|
| `/new-qgis` | Créer projet QGIS avec couches Bussigny | `/new-qgis cadastre2024 cadastre` |
| `/apply-style` | Appliquer/générer style QML | `/apply-style parcelles gradient par surface` |

### Rapports
| Skill | Description | Exemple |
|-------|-------------|---------|
| `/parcelle` | Rapport complet sur une parcelle | `/parcelle 1234` |
| `/stats` | Statistiques géodonnées communales | `/stats batiments 2020-2024` |

### Utilitaires
| Skill | Description | Exemple |
|-------|-------------|---------|
| `/coords` | Convertir coordonnées (MN95↔WGS84) | `/coords 2534567 1152345` |
| `/buffer` | Créer zone tampon | `/buffer 2534567,1152345 100m` |
| `/distance` | Calculer distance entre points | `/distance adresse1 adresse2` |

### Projet
| Skill | Description | Exemple |
|-------|-------------|---------|
| `/deploy` | Déployer données vers prod/test | `/deploy geo.parcelles prod` |
| `/fme-run` | Exécuter workbench FME | `/fme-run import_rf.fmw DATE=2024-01-15` |
