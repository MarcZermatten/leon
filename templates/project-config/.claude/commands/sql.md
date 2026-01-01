# Skill: sql

Executer une requete SQL sur PostGIS et afficher les resultats.

## Arguments
$ARGUMENTS = requete SQL ou description en langage naturel

## Instructions

### Requete SQL directe
1. Si $ARGUMENTS commence par SELECT, INSERT, UPDATE, DELETE, CREATE, ALTER, DROP:
   - Executer directement sur la base
   - Afficher resultats en tableau formaté
   - Montrer nombre de lignes affectees

### Langage naturel
1. Si $ARGUMENTS est une description (ex: "parcelles de plus de 1000m2"):
   - Identifier la/les tables concernees
   - Generer la requete SQL appropriee
   - Montrer la requete generee avant execution
   - Executer et afficher resultats

## Code type
```python
import psycopg2
import pandas as pd
from tabulate import tabulate

conn = psycopg2.connect(
    host="srv-fme",
    database="Prod",
    user="postgres",
    password=os.environ.get("POSTGIS_PASSWORD", "")
)

df = pd.read_sql(query, conn)
print(tabulate(df, headers='keys', tablefmt='psql'))
conn.close()
```

## Requetes frequentes

### Statistiques table
```sql
SELECT COUNT(*) as nb,
       ST_GeometryType(geometry) as type_geom,
       ST_SRID(geometry) as srid
FROM schema.table
GROUP BY ST_GeometryType(geometry), ST_SRID(geometry);
```

### Recherche par attribut
```sql
SELECT * FROM geo.parcelles
WHERE commune = 'Bussigny' AND surface > 1000;
```

### Recherche spatiale
```sql
SELECT a.* FROM geo.batiments a, geo.parcelles b
WHERE ST_Intersects(a.geometry, b.geometry)
AND b.numero = '1234';
```

## Connexion PostGIS
postgresql://postgres:$POSTGIS_PASSWORD@srv-fme:5432/Prod
