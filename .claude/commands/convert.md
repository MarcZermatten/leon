# Skill: convert

Convertir un fichier entre formats geospatiaux.

## Arguments
$ARGUMENTS = fichier_source [format_cible] [srid_cible]
- format_cible: gpkg, shp, geojson, dxf, kml (detecte depuis extension si absent)
- srid_cible: 2056 par defaut

## Instructions

1. Detecter format source depuis extension
2. Lire avec le driver approprie:
   - .shp, .gpkg, .geojson → geopandas
   - .dxf → geopandas ou FME CLI
   - .kml → geopandas (driver KML)
   - .xtf/.itf → ili2gpkg
3. Transformer SRID si different de cible
4. Valider geometries (make_valid si necessaire)
5. Ecrire dans format cible
6. Afficher resume conversion

## Conversions speciales

### DXF vers GeoPackage
```bash
ogr2ogr -f "GPKG" output.gpkg input.dxf -t_srs EPSG:2056
```

### INTERLIS vers GeoPackage
```bash
java -jar ili2gpkg.jar --import --dbfile output.gpkg input.xtf
```

### GeoPackage vers DXF
```bash
ogr2ogr -f "DXF" output.dxf input.gpkg layer_name
```

## Code type
```python
import geopandas as gpd

gdf = gpd.read_file(input_path)
if gdf.crs.to_epsg() != target_srid:
    gdf = gdf.to_crs(epsg=target_srid)
gdf['geometry'] = gdf['geometry'].make_valid()
gdf.to_file(output_path, driver=target_driver)
```
