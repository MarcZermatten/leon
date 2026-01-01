---
paths: **/*
---

# Auto-déclenchement des Skills SIT

Quand l'utilisateur pose une question ou demande une action, utiliser automatiquement le skill approprié sans attendre qu'il tape la commande slash.

## Détection automatique

### Données & ETL
| Pattern utilisateur | Skill à invoquer |
|---------------------|------------------|
| "importe ce CSV", "charge ce fichier CSV" | `/import-csv` |
| "exporte la table", "extrais les données" | `/export` |
| "convertis en gpkg/shp/geojson/dxf" | `/convert` |
| "valide ce XTF/ITF", "vérifie l'INTERLIS" | `/validate-xtf` |
| "géocode", "trouve les coordonnées de", "où se trouve" | `/geocode` |

### PostGIS
| Pattern utilisateur | Skill à invoquer |
|---------------------|------------------|
| "requête SQL", "SELECT", "montre les parcelles qui..." | `/sql` |
| "décris la table", "structure de", "colonnes de" | `/describe` |
| "vérifie les géométries", "qualité spatiale" | `/spatial-check` |
| "sauvegarde la table", "backup" | `/backup` |

### QGIS
| Pattern utilisateur | Skill à invoquer |
|---------------------|------------------|
| "nouveau projet QGIS", "crée un projet" | `/new-qgis` |
| "applique un style", "colore par", "étiquettes" | `/apply-style` |

### Rapports
| Pattern utilisateur | Skill à invoquer |
|---------------------|------------------|
| "info parcelle", "rapport parcelle", "parcelle N°" | `/parcelle` |
| "statistiques", "combien de", "évolution" | `/stats` |

### Utilitaires
| Pattern utilisateur | Skill à invoquer |
|---------------------|------------------|
| "convertis ces coordonnées", "MN95 vers WGS84" | `/coords` |
| "zone tampon", "buffer de X mètres" | `/buffer` |
| "distance entre", "à quelle distance" | `/distance` |

### Projet
| Pattern utilisateur | Skill à invoquer |
|---------------------|------------------|
| "déploie en prod/test" | `/deploy` |
| "lance le workbench FME", "exécute le .fmw" | `/fme-run` |

## Comportement intelligent

1. **Détection du contexte** : Analyser la demande pour identifier le skill approprié
2. **Confirmation implicite** : Exécuter directement si l'intention est claire
3. **Clarification si ambigu** : Demander précision si plusieurs skills possibles
4. **Chaînage** : Enchaîner plusieurs skills si nécessaire (ex: geocode → buffer → sql)

## Exemples de traitement automatique

### Exemple 1: Question simple
```
Utilisateur: "Où se trouve la rue du Simplon 10 à Bussigny?"
→ Invoquer automatiquement /geocode "Rue du Simplon 10, Bussigny"
→ Afficher coordonnées + lien map.geo.admin.ch
```

### Exemple 2: Requête données
```
Utilisateur: "Montre-moi toutes les parcelles de plus de 2000m² en zone villa"
→ Invoquer automatiquement /sql
→ Générer et exécuter: SELECT * FROM geo.parcelles p JOIN geo.zones z ON ST_Intersects(...)
   WHERE ST_Area(p.geometry) > 2000 AND z.type = 'Zone villa'
```

### Exemple 3: Chaînage
```
Utilisateur: "Quelles sont les hydrantes à moins de 100m de la parcelle 1234?"
→ /sql pour récupérer géométrie parcelle 1234
→ /buffer 100m autour
→ /sql pour trouver hydrantes intersectant le buffer
```

### Exemple 4: Rapport
```
Utilisateur: "Dis-moi tout sur la parcelle 567"
→ Invoquer automatiquement /parcelle 567
→ Rapport complet (cadastre, zone, bâtiments, réseaux, RDPPF)
```

## Priorité
Cette règle a priorité sur le comportement par défaut. Ne pas attendre que l'utilisateur tape explicitement /skill - anticiper et exécuter.
