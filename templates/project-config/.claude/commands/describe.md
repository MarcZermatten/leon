# Skill: describe

Decrire la structure d'une table ou schema PostGIS.

## Arguments
$ARGUMENTS = nom_table OU schema.table OU schema seul

## Instructions

### Table specifique
1. Recuperer structure:
   - Colonnes (nom, type, nullable, default)
   - Colonne geometrique (type, SRID)
   - Contraintes (PK, FK, UNIQUE, CHECK)
   - Index (dont spatial GIST)
2. Statistiques:
   - Nombre de lignes
   - Taille table
   - Emprise spatiale (bbox)
3. Echantillon de donnees (5 premieres lignes)

### Schema entier
1. Lister toutes les tables du schema
2. Pour chaque table: nom, nb colonnes, type geometrie, nb lignes

## Code type
```sql
-- Structure colonnes
SELECT column_name, data_type, is_nullable, column_default
FROM information_schema.columns
WHERE table_schema = 'geo' AND table_name = 'parcelles';

-- Info geometrie
SELECT f_geometry_column, type, srid
FROM geometry_columns
WHERE f_table_schema = 'geo' AND f_table_name = 'parcelles';

-- Index
SELECT indexname, indexdef
FROM pg_indexes
WHERE schemaname = 'geo' AND tablename = 'parcelles';

-- Stats
SELECT COUNT(*) as nb_rows,
       pg_size_pretty(pg_total_relation_size('geo.parcelles')) as size,
       ST_Extent(geometry) as bbox
FROM geo.parcelles;
```

## Sortie type
```
TABLE: geo.parcelles
================================================================================
Colonnes (8):
  - id          : integer (PK, NOT NULL)
  - numero      : varchar(20) (NOT NULL)
  - commune     : varchar(100)
  - surface     : numeric(12,2)
  - geometry    : geometry(MultiPolygon, 2056)
  ...

Index:
  - idx_parcelles_pkey (btree) sur id
  - idx_parcelles_geom (gist) sur geometry

Stats:
  - Lignes: 4,532
  - Taille: 12 MB
  - Emprise: BOX(2532000 1152000, 2536000 1156000)
```

## Connexion PostGIS
postgresql://postgres:$POSTGIS_PASSWORD@srv-fme:5432/Prod
