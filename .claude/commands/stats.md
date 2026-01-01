# Skill: stats

Generer des statistiques sur les geodonnees communales.

## Arguments
$ARGUMENTS = theme [periode] [format]
- theme: parcelles, batiments, population, reseaux, permis, arbres
- periode: annee ou plage (2023, 2020-2024)
- format: table, chart, csv (defaut: table)

## Instructions

1. Identifier les tables concernees
2. Calculer les statistiques demandees
3. Comparer avec periodes precedentes si dispo
4. Generer visualisation ou export

## Themes disponibles

### Parcelles
- Nombre par zone d'affectation
- Surface totale par zone
- Evolution du parcellaire
- Taux de construction

### Batiments
- Nombre et surface par type
- Repartition par epoque de construction
- Nouveaux batiments par annee
- Emprise batie totale

### Population
- Habitants par quartier
- Densite par zone
- Evolution demographique

### Reseaux
- Lineaire par type (eau, EU, EP)
- Age moyen du reseau
- Interventions par annee

### Permis de construire
- Nombre par annee/type
- Surface autorisee
- Delais moyens

### Arbres
- Inventaire par essence
- Repartition par quartier
- Etat sanitaire

## Code type
```sql
-- Stats parcelles par zone
SELECT
    z.type_zone,
    COUNT(*) as nb_parcelles,
    SUM(ST_Area(p.geometry)) as surface_totale,
    AVG(ST_Area(p.geometry)) as surface_moyenne
FROM geo.parcelles p
JOIN geo.zones_affectation z ON ST_Intersects(p.geometry, z.geometry)
GROUP BY z.type_zone
ORDER BY nb_parcelles DESC;

-- Stats batiments par epoque
SELECT
    CASE
        WHEN annee_construction < 1900 THEN 'Avant 1900'
        WHEN annee_construction < 1950 THEN '1900-1949'
        WHEN annee_construction < 1980 THEN '1950-1979'
        WHEN annee_construction < 2000 THEN '1980-1999'
        ELSE '2000+'
    END as epoque,
    COUNT(*) as nb_batiments,
    SUM(ST_Area(geometry)) as emprise_totale
FROM geo.batiments
GROUP BY 1
ORDER BY 1;

-- Evolution annuelle
SELECT
    EXTRACT(YEAR FROM date_creation) as annee,
    COUNT(*) as nb_nouveaux,
    SUM(ST_Area(geometry)) as surface_nouvelle
FROM geo.batiments
WHERE date_creation >= '2020-01-01'
GROUP BY 1
ORDER BY 1;
```

## Format sortie (table)
```
STATISTIQUES PARCELLES PAR ZONE
===============================
Commune: Bussigny
Date: 2024-01-15

Zone                          | Nb     | Surface (m²) | Moyenne
------------------------------|--------|--------------|--------
Zone habitat individuel       | 1,234  | 2,456,789    | 1,991
Zone habitat collectif        | 456    | 1,234,567    | 2,707
Zone d'activite              | 89     | 567,890      | 6,381
Zone agricole                 | 234    | 3,456,789    | 14,773
Zone foret                    | 45     | 890,123      | 19,780
------------------------------|--------|--------------|--------
TOTAL                         | 2,058  | 8,606,158    | 4,181
```

## Visualisation (si chart)
Genere un graphique avec matplotlib:
- Camembert pour repartitions
- Barres pour comparaisons
- Lignes pour evolutions

## Export CSV
```csv
zone;nb_parcelles;surface_totale;surface_moyenne
Zone habitat individuel;1234;2456789;1991
Zone habitat collectif;456;1234567;2707
...
```
