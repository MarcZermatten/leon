# Skill: coords

Convertir des coordonnees entre systemes de reference (SRID).

## Arguments
$ARGUMENTS = x y [srid_source] [srid_cible]
- srid_source: detecte automatiquement ou specifie (2056, 21781, 4326)
- srid_cible: 2056 par defaut (MN95)

## Instructions

1. Parser les coordonnees (formats varies acceptes)
2. Detecter le SRID source si non specifie:
   - X > 2000000 → MN95 (2056)
   - X > 400000 et X < 900000 → MN03 (21781)
   - X < 180 et Y < 90 → WGS84 (4326)
3. Transformer vers SRID cible
4. Afficher tous les formats utiles

## Systemes suisses

| SRID | Nom | X typique | Y typique |
|------|-----|-----------|-----------|
| 2056 | MN95/LV95 | 2'530'000 | 1'150'000 |
| 21781 | MN03/LV03 | 530'000 | 150'000 |
| 4326 | WGS84 | 6.6° | 46.5° |

## Formats d'entree acceptes
- `2534567 1152345` (espace)
- `2534567, 1152345` (virgule)
- `2'534'567 1'152'345` (apostrophes suisses)
- `6.634521 46.543210` (WGS84 decimal)
- `6°38'04.3" 46°32'35.6"` (DMS)

## Code type
```python
from pyproj import Transformer, CRS

def detect_srid(x, y):
    if x > 2000000:
        return 2056  # MN95
    elif 400000 < x < 900000:
        return 21781  # MN03
    elif -180 <= x <= 180 and -90 <= y <= 90:
        return 4326  # WGS84
    return None

def transform_coords(x, y, source_srid, target_srid):
    transformer = Transformer.from_crs(
        CRS.from_epsg(source_srid),
        CRS.from_epsg(target_srid),
        always_xy=True
    )
    return transformer.transform(x, y)

# MN03 vers MN95
x_mn03, y_mn03 = 534567, 152345
x_mn95, y_mn95 = transform_coords(x_mn03, y_mn03, 21781, 2056)

# WGS84 vers MN95
lon, lat = 6.634521, 46.543210
x, y = transform_coords(lon, lat, 4326, 2056)
```

## SQL PostGIS
```sql
-- Transformation simple
SELECT ST_Transform(
    ST_SetSRID(ST_MakePoint(534567, 152345), 21781),
    2056
) as point_mn95;

-- Extraire coordonnees
SELECT ST_X(geom) as x, ST_Y(geom) as y
FROM ST_Transform(ST_SetSRID(ST_MakePoint(6.634521, 46.543210), 4326), 2056) as geom;
```

## Sortie type
```
CONVERSION COORDONNEES
======================
Entree: 534567, 152345
SRID detecte: 21781 (MN03/LV03)

RESULTATS:
----------
MN95 (2056):   2'534'566.87  1'152'344.62
MN03 (21781):  534'567.00    152'345.00
WGS84 (4326):  6.63452°      46.54321°
WGS84 DMS:     6°38'04.3"E   46°32'35.6"N

Lien map.geo.admin.ch:
https://map.geo.admin.ch/?E=2534567&N=1152345&zoom=10
```
