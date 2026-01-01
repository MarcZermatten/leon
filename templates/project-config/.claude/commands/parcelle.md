# Skill: parcelle

Generer un rapport complet sur une parcelle cadastrale.

## Arguments
$ARGUMENTS = numero_parcelle [commune]
- commune: Bussigny par defaut

## Instructions

1. Rechercher la parcelle dans PostGIS
2. Collecter toutes les informations:
   - Donnees cadastrales (RF)
   - Zones d'affectation
   - Servitudes
   - Batiments sur la parcelle
   - Reseaux traversants
   - Restrictions (RDPPF)
3. Generer rapport structure

## Donnees collectees

### Cadastre (RF)
- Numero parcelle, commune, lieu-dit
- Surface officielle
- Proprietaire(s) - si accessible
- Date derniere mutation

### Urbanisme
- Zone d'affectation (PGA)
- Indice d'utilisation du sol (IUS)
- Indice de masse (IM)
- Distance aux limites

### RDPPF
- Plans d'affectation
- Zones de protection
- Servitudes publiques
- Restrictions forestieres/agricoles

### Infrastructures
- Batiments (emprise, type, date)
- Acces routier
- Desserte en eau/electricite/gaz
- Raccordement eaux usees

## Code type
```sql
-- Info parcelle
SELECT p.numero, p.commune, p.surface,
       ST_Area(p.geometry) as surface_calc
FROM geo.parcelles p
WHERE p.numero = '1234' AND p.commune = 'Bussigny';

-- Zone affectation
SELECT z.type_zone, z.ius, z.im
FROM geo.zones_affectation z, geo.parcelles p
WHERE ST_Intersects(z.geometry, p.geometry)
AND p.numero = '1234';

-- Batiments
SELECT b.type, b.egid, ST_Area(b.geometry) as emprise
FROM geo.batiments b, geo.parcelles p
WHERE ST_Within(b.geometry, p.geometry)
AND p.numero = '1234';

-- Conduites traversantes
SELECT c.type, c.diametre, c.materiau,
       ST_Length(ST_Intersection(c.geometry, p.geometry)) as longueur
FROM geo.conduites_eau c, geo.parcelles p
WHERE ST_Intersects(c.geometry, p.geometry)
AND p.numero = '1234';
```

## Format rapport
```
RAPPORT PARCELLE
================

IDENTIFICATION
--------------
Numero: 1234
Commune: Bussigny
Lieu-dit: Les Pres
Surface RF: 1'250 m²
Surface calculee: 1'248.5 m²

URBANISME
---------
Zone: Zone d'habitation collective de moyenne densite (ZHC2)
IUS: 0.6
IM: 2.0
Hauteur max: 12 m

BATIMENTS (2)
-------------
1. Habitation (EGID 1234567) - 180 m²
2. Garage (EGID 1234568) - 25 m²
Surface batie totale: 205 m²
Taux d'occupation: 16.4%

RESEAUX
-------
- Eau: Conduite DN100 fonte (12.5 m)
- EU: Collecteur DN300 beton (8.2 m)
- Electricite: Desserte confirmee

RESTRICTIONS RDPPF
------------------
- Zone de protection des eaux S3
- Aucune servitude publique

Genere le: 2024-01-15 14:30
Source: SIT Bussigny
```
