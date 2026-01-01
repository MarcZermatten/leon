# Skill: distance

Calculer la distance entre deux points ou objets geographiques.

## Arguments
$ARGUMENTS = point1 point2
- point: coordonnees (x,y), adresse, ou table.colonne=valeur

## Instructions

1. Identifier les deux points:
   - Coordonnees → point direct
   - Adresse → geocoder
   - Reference table → centroide de l'objet
2. Calculer distance euclidienne (2D)
3. Optionnel: distance reseau (routing)

## Exemples

### Entre deux coordonnees
```
/distance 2534567,1152345 2535000,1153000
```

### Entre deux adresses
```
/distance "Rue du Simplon 1, Bussigny" "Place de la Gare, Lausanne"
```

### Entre un point et un objet
```
/distance 2534567,1152345 geo.hydrantes.numero=H123
```

### Plus proche voisin
```
/distance geo.parcelles.numero=1234 geo.hydrantes --nearest
```

## Code SQL
```sql
-- Distance entre deux points
SELECT ST_Distance(
    ST_SetSRID(ST_MakePoint(2534567, 1152345), 2056),
    ST_SetSRID(ST_MakePoint(2535000, 1153000), 2056)
) as distance_m;

-- Distance entre deux objets
SELECT ST_Distance(p.geometry, h.geometry) as distance_m
FROM geo.parcelles p, geo.hydrantes h
WHERE p.numero = '1234' AND h.numero = 'H123';

-- Plus proche hydrante d'une parcelle
SELECT h.numero, ST_Distance(p.geometry, h.geometry) as distance_m
FROM geo.parcelles p, geo.hydrantes h
WHERE p.numero = '1234'
ORDER BY p.geometry <-> h.geometry
LIMIT 1;

-- Toutes les hydrantes a moins de 100m
SELECT h.numero, ST_Distance(p.geometry, h.geometry) as distance_m
FROM geo.parcelles p, geo.hydrantes h
WHERE p.numero = '1234'
AND ST_DWithin(p.geometry, h.geometry, 100)
ORDER BY distance_m;
```

## Code Python
```python
from shapely.geometry import Point
import math

def distance_2d(x1, y1, x2, y2):
    return math.sqrt((x2-x1)**2 + (y2-y1)**2)

# Ou avec Shapely
p1 = Point(2534567, 1152345)
p2 = Point(2535000, 1153000)
dist = p1.distance(p2)  # en metres (si SRID 2056)
```

## Sortie type
```
CALCUL DISTANCE
===============
Point 1: 2534567, 1152345 (Rue du Simplon 1, Bussigny)
Point 2: 2535000, 1153000 (Rue de Lausanne 25, Bussigny)

DISTANCE A VOL D'OISEAU:
------------------------
785.24 metres
0.79 km

PLUS PROCHES HYDRANTES:
-----------------------
1. H045 - 23.5 m (Rue du Simplon)
2. H046 - 67.8 m (Rue de la Poste)
3. H044 - 89.2 m (Chemin des Vignes)
```

## Options
- `--3d`: inclure altitude (Z)
- `--nearest`: trouver plus proche
- `--radius=100`: tous les objets dans rayon
- `--route`: distance par le reseau routier (si disponible)
