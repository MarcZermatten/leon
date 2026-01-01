# Skill: buffer

Creer une zone tampon autour d'une geometrie ou d'un point.

## Arguments
$ARGUMENTS = source distance [unite]
- source: coordonnees (x,y), adresse, ou table.colonne=valeur
- distance: valeur numerique
- unite: m (defaut), km

## Instructions

1. Identifier la source:
   - Coordonnees → creer point
   - Adresse → geocoder puis point
   - table.colonne=valeur → requete PostGIS
2. Creer buffer avec ST_Buffer
3. Exporter resultat (GeoJSON, WKT)
4. Optionnel: lister objets dans le buffer

## Exemples d'utilisation

### Autour d'un point
```
/buffer 2534567,1152345 100m
```

### Autour d'une adresse
```
/buffer "Rue du Simplon 1, Bussigny" 50m
```

### Autour d'un objet existant
```
/buffer geo.parcelles.numero=1234 10m
```

## Code type
```sql
-- Buffer autour d'un point
SELECT ST_Buffer(
    ST_SetSRID(ST_MakePoint(2534567, 1152345), 2056),
    100  -- 100 metres
) as buffer_geom;

-- Buffer autour d'une parcelle
SELECT ST_Buffer(p.geometry, 10) as buffer_geom
FROM geo.parcelles p
WHERE p.numero = '1234';

-- Objets dans le buffer
SELECT b.*
FROM geo.batiments b,
     (SELECT ST_Buffer(geometry, 50) as geom
      FROM geo.parcelles WHERE numero = '1234') buffer
WHERE ST_Intersects(b.geometry, buffer.geom);
```

## Code Python
```python
import geopandas as gpd
from shapely.geometry import Point
from shapely.ops import transform
import pyproj

# Buffer autour d'un point
point = Point(2534567, 1152345)
buffer = point.buffer(100)  # 100m

# Exporter en GeoJSON
gdf = gpd.GeoDataFrame({'geometry': [buffer]}, crs="EPSG:2056")
gdf.to_file("buffer.geojson", driver="GeoJSON")
```

## Sortie type
```
BUFFER CREE
===========
Centre: 2534567, 1152345
Rayon: 100 m
Surface: 31'415.93 m² (3.14 ha)

GEOMETRIE (WKT):
POLYGON((2534667 1152345, 2534666.9 1152355.2, ...))

OBJETS DANS LE BUFFER:
- 3 parcelles
- 5 batiments
- 2 hydrantes

Export: buffer_100m.geojson
```

## Options avancees
- `--dissolve`: fusionner si plusieurs sources
- `--segments=32`: nombre de segments pour arcs
- `--cap=round|flat|square`: style des extremites
- `--list=table`: lister objets intersectant
