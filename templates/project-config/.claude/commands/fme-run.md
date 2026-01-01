# Skill: fme-run

Executer un workbench FME avec parametres.

## Arguments
$ARGUMENTS = workbench.fmw [parametres]
- workbench: nom du fichier .fmw ou chemin complet
- parametres: PARAM=valeur PARAM2=valeur2

## Instructions

1. Localiser le workbench:
   - Chemin absolu fourni → utiliser tel quel
   - Nom seul → chercher dans scripts/fme/
2. Valider les parametres requis
3. Executer avec FME Command Line
4. Capturer les logs
5. Afficher resume execution

## Workbenches disponibles Bussigny

| Workbench | Description | Parametres |
|-----------|-------------|------------|
| import_rf.fmw | Import registre foncier | DATE_EXTRACTION |
| export_cadastre.fmw | Export DXF cadastre | COMMUNES, FORMAT |
| sync_reseaux.fmw | Synchro reseaux eau | SOURCE_DATE |
| generate_stats.fmw | Stats mensuelles | MOIS, ANNEE |
| validate_data.fmw | Validation donnees | TABLES, STRICT |

## Execution locale
```bash
# FME Desktop
"C:\Program Files\FME\fme.exe" scripts/fme/import_rf.fmw ^
    --DATE_EXTRACTION "2024-01-15" ^
    --LOG_FILE "logs/import_rf.log"
```

## Execution FME Server
```python
import requests

def run_fme_server(workspace, params):
    url = "http://fme-server/fmerest/v3/transformations/submit/Bussigny"

    payload = {
        "workspace": workspace,
        "publishedParameters": [
            {"name": k, "value": v} for k, v in params.items()
        ]
    }

    headers = {
        "Authorization": "fmetoken token=xxx",
        "Content-Type": "application/json"
    }

    response = requests.post(url, json=payload, headers=headers)
    job_id = response.json()['id']

    # Attendre completion
    while True:
        status = requests.get(f"{url}/jobs/{job_id}", headers=headers)
        if status.json()['status'] in ['SUCCESS', 'FAILURE']:
            break
        time.sleep(5)

    return status.json()
```

## Code type execution locale
```python
import subprocess
import os

def run_fme_workbench(workbench, params=None):
    fme_exe = r"C:\Program Files\FME\fme.exe"

    # Construire commande
    cmd = [fme_exe, workbench]

    if params:
        for key, value in params.items():
            cmd.extend([f"--{key}", str(value)])

    # Log file
    log_file = workbench.replace('.fmw', '.log')
    cmd.extend(["--LOG_FILE", log_file])

    # Executer
    result = subprocess.run(cmd, capture_output=True, text=True)

    return {
        'returncode': result.returncode,
        'stdout': result.stdout,
        'stderr': result.stderr,
        'log_file': log_file
    }

# Usage
result = run_fme_workbench(
    "scripts/fme/import_rf.fmw",
    {"DATE_EXTRACTION": "2024-01-15"}
)
```

## Sortie type
```
FME EXECUTION
=============
Workbench: import_rf.fmw
Parametres:
  - DATE_EXTRACTION: 2024-01-15

EXECUTION:
[14:30:00] Starting...
[14:30:02] Reading source: RF_20240115.xml
[14:30:15] Transforming 4,532 features
[14:30:45] Writing to PostGIS: geo.parcelles_new
[14:31:02] Completed

RESULTAT: SUCCES
----------------
Features lus: 4,532
Features ecrits: 4,530
Rejetes: 2
Temps: 62 secondes
Log: logs/import_rf_20240115.log
```

## Gestion erreurs
- Rejets → exports vers fichier _rejected.csv
- Erreurs → detail dans le log
- Timeout → configurable (defaut 30 min)
