# Skill: backup

Sauvegarder une table ou schema PostGIS.

## Arguments
$ARGUMENTS = schema[.table] [chemin_sortie]
- Si table specifiee: backup table seule
- Si schema seul: backup schema complet
- chemin_sortie: optionnel, sinon ./backups/YYYYMMDD_HHMMSS/

## Instructions

### Backup table
1. Creer repertoire de sortie
2. Exporter structure (CREATE TABLE, INDEX, CONSTRAINTS)
3. Exporter donnees (pg_dump ou COPY)
4. Generer fichier info (date, nb lignes, taille)

### Backup schema
1. Lister toutes les tables du schema
2. Pour chaque table: structure + donnees
3. Exporter sequences et fonctions
4. Generer manifest.json avec inventaire

## Formats de sortie
- `.sql` : Script SQL complet (pg_dump)
- `.gpkg` : GeoPackage (donnees spatiales)
- `.csv` : Donnees tabulaires (sans geometrie)

## Code type
```bash
# Backup table avec pg_dump
pg_dump -h srv-fme -U postgres -d Prod \
  -t geo.parcelles \
  --no-owner --no-privileges \
  -f backup_parcelles.sql

# Backup schema complet
pg_dump -h srv-fme -U postgres -d Prod \
  -n geo \
  --no-owner --no-privileges \
  -f backup_geo_schema.sql
```

```python
import subprocess
from datetime import datetime
import os

def backup_table(schema, table, output_dir=None):
    if output_dir is None:
        output_dir = f"./backups/{datetime.now().strftime('%Y%m%d_%H%M%S')}"
    os.makedirs(output_dir, exist_ok=True)

    output_file = f"{output_dir}/{schema}_{table}.sql"

    cmd = [
        'pg_dump',
        '-h', 'srv-fme',
        '-U', 'postgres',
        '-d', 'Prod',
        '-t', f'{schema}.{table}',
        '--no-owner',
        '-f', output_file
    ]

    env = os.environ.copy()
    env['PGPASSWORD'] = '$POSTGIS_PASSWORD'

    subprocess.run(cmd, env=env, check=True)
    return output_file
```

## Sortie type
```
BACKUP COMPLETE
===============
Source: geo.parcelles
Date: 2024-01-15 14:30:00
Fichiers:
  - backups/20240115_143000/geo_parcelles.sql (2.3 MB)
  - backups/20240115_143000/geo_parcelles.gpkg (1.8 MB)
  - backups/20240115_143000/manifest.json

Contenu:
  - 4,532 entites
  - Structure: OK
  - Index: 2
  - Contraintes: 3
```

## Restauration
```bash
# Restaurer depuis SQL
psql -h srv-fme -U postgres -d Prod -f backup_parcelles.sql

# Restaurer depuis GeoPackage (via ogr2ogr)
ogr2ogr -f "PostgreSQL" PG:"host=srv-fme dbname=Prod user=postgres" \
  backup_parcelles.gpkg -nln geo.parcelles_restored
```

## Connexion PostGIS
postgresql://postgres:$POSTGIS_PASSWORD@srv-fme:5432/Prod
