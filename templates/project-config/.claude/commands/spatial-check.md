# Skill: spatial-check

Verifier la qualite des geometries d'une table PostGIS.

## Arguments
$ARGUMENTS = schema.table [--fix]
- --fix: Corriger automatiquement les geometries invalides

## Instructions

1. Analyser la table:
   - Type de geometrie attendu
   - SRID (doit etre 2056)
   - Nombre total d'entites

2. Verifier chaque critere:
   - Geometries NULL
   - Geometries invalides (ST_IsValid = false)
   - Geometries vides (ST_IsEmpty = true)
   - SRID incorrect (!= 2056)
   - Geometries en double (meme coordonnees)
   - Auto-intersections (polygones)

3. Rapport de qualite:
   - Score global (% valides)
   - Detail par type d'erreur
   - Exemples d'entites problematiques (IDs)

4. Si --fix:
   - ST_MakeValid() pour invalides
   - ST_Transform() pour mauvais SRID
   - Rapport des corrections

## Code type
```sql
-- Geometries invalides
SELECT id, ST_IsValidReason(geometry) as raison
FROM geo.parcelles
WHERE NOT ST_IsValid(geometry);

-- SRID incorrect
SELECT id, ST_SRID(geometry) as srid_actuel
FROM geo.parcelles
WHERE ST_SRID(geometry) != 2056;

-- Geometries vides
SELECT COUNT(*) FROM geo.parcelles WHERE ST_IsEmpty(geometry);

-- Correction automatique
UPDATE geo.parcelles
SET geometry = ST_MakeValid(geometry)
WHERE NOT ST_IsValid(geometry);

-- Transformation SRID
UPDATE geo.parcelles
SET geometry = ST_Transform(geometry, 2056)
WHERE ST_SRID(geometry) != 2056;
```

## Rapport type
```
RAPPORT QUALITE SPATIALE: geo.parcelles
========================================
Total entites: 4,532
Score qualite: 98.5%

VERIFICATION          | OK    | ERREUR | %
----------------------|-------|--------|-------
Geometries valides    | 4,465 | 67     | 98.5%
SRID = 2056          | 4,532 | 0      | 100%
Non vides            | 4,530 | 2      | 99.9%
Sans doublons        | 4,520 | 12     | 99.7%

DETAILS ERREURS:
- Invalides (67): Self-intersection aux IDs: 123, 456, 789...
- Vides (2): IDs 1001, 1002
- Doublons (12): Paires (10,15), (234,235)...

RECOMMANDATIONS:
1. Executer ST_MakeValid() sur 67 geometries
2. Supprimer ou corriger les 2 geometries vides
3. Verifier les 12 doublons potentiels
```

## Connexion PostGIS
postgresql://postgres:$POSTGIS_PASSWORD@srv-fme:5432/Prod
