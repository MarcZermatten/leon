# Skill: deploy

Deployer des donnees ou configurations vers les environnements Bussigny.

## Arguments
$ARGUMENTS = source destination [options]
- source: fichier, table, ou projet
- destination: prod, test, qgis-server, fme-server
- options: --force, --backup, --dry-run

## Instructions

1. Valider la source existe
2. Creer backup de la destination (si --backup)
3. Deployer selon type:
   - Table PostGIS → copie vers autre schema/serveur
   - Fichier GeoPackage → upload vers serveur
   - Projet QGIS → publication sur QGIS Server
   - Script Python → copie vers serveur de taches
4. Verifier le deploiement
5. Rapport de deploiement

## Destinations disponibles

| Destination | Type | Usage |
|-------------|------|-------|
| prod | PostGIS | Base de production |
| test | PostGIS | Base de test |
| qgis-server | QGIS Server | Publication web |
| fme-server | FME Server | Workbenches automatises |

## Deploiement table PostGIS
```python
import subprocess

def deploy_table(schema, table, target):
    # Backup
    dump_cmd = f"pg_dump -h srv-fme -U postgres -d Prod -t {schema}.{table} -f backup.sql"
    subprocess.run(dump_cmd, shell=True)

    # Copy vers target
    if target == 'test':
        restore_cmd = f"psql -h srv-fme -U postgres -d Test -f backup.sql"
    elif target == 'prod':
        # Confirmation requise
        confirm = input("Deployer en PRODUCTION? (oui/non): ")
        if confirm != 'oui':
            return
        restore_cmd = f"psql -h srv-fme -U postgres -d Prod -f backup.sql"

    subprocess.run(restore_cmd, shell=True)
```

## Deploiement QGIS Server
```bash
# Copier projet vers repertoire QGIS Server
scp projet.qgz user@qgis-server:/var/www/qgis/projets/

# Recharger cache
curl -X POST http://qgis-server/admin/reload?project=projet
```

## Deploiement FME Server
```bash
# Upload workbench via API REST
curl -X POST "http://fme-server/fmerest/v3/repositories/Bussigny/items" \
  -H "Authorization: fmetoken token=xxx" \
  -F "file=@workbench.fmw"
```

## Sortie type
```
DEPLOIEMENT
===========
Source: geo.parcelles (4,532 entites)
Destination: test
Date: 2024-01-15 14:30

ETAPES:
[✓] Backup destination: backup_20240115_143000.sql
[✓] Export source: OK
[✓] Import destination: OK
[✓] Verification: 4,532 entites

RESULTAT: SUCCES
Temps: 45 secondes
```

## Securite
- Deploiement PROD necessite confirmation explicite
- Logs de tous les deploiements dans /logs/deploy.log
- Backup automatique avant ecrasement
