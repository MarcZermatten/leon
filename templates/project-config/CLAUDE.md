# {{PROJECT_NAME}}

## Description
Projet initialisé avec Léon - Claude Code Desktop UI.

## Stack Technique
*(À compléter selon le projet)*

## Structure
```
{{PROJECT_NAME}}/
├── CLAUDE.md          # Ce fichier
├── README.md          # Documentation
├── .claude/
│   ├── settings.json  # Configuration Claude Code
│   ├── agents/        # Agents spécialisés
│   ├── commands/      # Skills (slash commands)
│   └── rules/         # Règles automatiques
└── ...
```

## Agents Disponibles

### SIT / Géospatial
| Agent | Usage |
|-------|-------|
| **python-geo** | Scripts Python géospatiaux |
| **qgis-expert** | QGIS, PyQGIS, projets .qgz |
| **fme-etl** | FME Workbench, pipelines ETL |
| **postgis-sql** | PostgreSQL/PostGIS, SQL spatial |
| **interlis-expert** | INTERLIS, validation XTF/ITF |
| **cartographie** | Styles, sémiologie, WMS/WFS |

### Données & Intégration
| Agent | Usage |
|-------|-------|
| **excel-data** | Fichiers Excel/CSV |
| **api-integrator** | APIs REST, services web |
| **pdf-reports** | Génération rapports PDF |

### Domaines métier
| Agent | Usage |
|-------|-------|
| **legal-ch** | Droit suisse, cadastre |
| **urbanisme** | PGA, zones, procédures |
| **environnement** | Zones protection, biodiversité |
| **network-infra** | Réseaux, SIA 405 |

## Skills Disponibles

### Données & ETL
- `/import-csv` - Importer CSV dans PostGIS
- `/export` - Exporter table vers fichier
- `/convert` - Convertir entre formats
- `/validate-xtf` - Valider INTERLIS
- `/geocode` - Géocoder adresses suisses

### PostGIS
- `/sql` - Exécuter requête SQL
- `/describe` - Décrire table/schema
- `/spatial-check` - Vérifier géométries
- `/backup` - Sauvegarder table

### QGIS
- `/new-qgis` - Créer projet QGIS
- `/apply-style` - Appliquer style QML

### Rapports
- `/parcelle` - Rapport complet parcelle
- `/stats` - Statistiques géodonnées

### Utilitaires
- `/coords` - Convertir coordonnées
- `/buffer` - Zone tampon
- `/distance` - Calculer distance

### Projet
- `/deploy` - Déployer vers prod/test
- `/fme-run` - Exécuter workbench FME

## Configuration PostGIS
Configurer la connexion dans `.claude/settings.json` ou via variables d'environnement.
