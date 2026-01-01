# Skill: import-csv

Importer un fichier CSV dans PostGIS avec detection automatique des colonnes geometriques.

## Arguments
$ARGUMENTS = chemin du fichier CSV et nom de la table cible (optionnel)

## Instructions

1. Lire le fichier CSV avec pandas
2. Detecter les colonnes geometriques:
   - Colonnes X/Y, Lon/Lat, E/N, Est/Nord
   - Colonne WKT (geometry en texte)
   - Colonne GeoJSON
3. Determiner le SRID:
   - Si coordonnees > 2000000 → MN95 (2056)
   - Si coordonnees > 400000 → MN03 (21781)
   - Sinon → WGS84 (4326), transformer vers 2056
4. Creer GeoDataFrame avec geometrie Point ou depuis WKT
5. Charger dans PostGIS:
   - Schema: geo (par defaut)
   - Table: nom du fichier sans extension ou argument fourni
   - Si table existe: demander confirmation pour remplacer
6. Creer index spatial GIST
7. Afficher resume: nb lignes, colonnes, SRID, table creee

## Code type
```python
import pandas as pd
import geopandas as gpd
from sqlalchemy import create_engine

df = pd.read_csv(fichier, sep=';', encoding='utf-8')
# Detection et creation geometrie...
gdf = gpd.GeoDataFrame(df, geometry=geometry, crs=f"EPSG:{srid}")
gdf.to_postgis(table_name, engine, schema='geo', if_exists='replace', index=True)
```

## Connexion PostGIS
postgresql://postgres:dsg#6hY95!@srv-fme:5432/Prod
