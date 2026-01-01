# Skill: export

Exporter une table PostGIS vers un fichier (GeoPackage, Shapefile, GeoJSON).

## Arguments
$ARGUMENTS = nom_table [format] [chemin_sortie]
- format: gpkg (defaut), shp, geojson, csv
- chemin_sortie: optionnel, sinon ./exports/

## Instructions

1. Connecter a PostGIS
2. Lire la table avec GeoDataFrame
3. Verifier le SRID (doit etre 2056 pour export)
4. Exporter selon format:
   - gpkg: gdf.to_file(path, driver='GPKG')
   - shp: gdf.to_file(path, driver='ESRI Shapefile', encoding='utf-8')
   - geojson: gdf.to_file(path, driver='GeoJSON')
   - csv: gdf.to_csv(path, sep=';', index=False)
5. Afficher: nb entites, chemin fichier, taille

## Code type
```python
import geopandas as gpd
from sqlalchemy import create_engine

engine = create_engine('postgresql://postgres:dsg#6hY95!@srv-fme:5432/Prod')
gdf = gpd.read_postgis(f"SELECT * FROM {schema}.{table}", engine, geom_col='geometry')
gdf.to_file(output_path, driver=driver)
```

## Formats supportes
| Format | Extension | Driver |
|--------|-----------|--------|
| GeoPackage | .gpkg | GPKG |
| Shapefile | .shp | ESRI Shapefile |
| GeoJSON | .geojson | GeoJSON |
| CSV | .csv | (pandas) |
