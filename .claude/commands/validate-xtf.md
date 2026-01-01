# Skill: validate-xtf

Valider un fichier INTERLIS (XTF/ITF) avec ilivalidator.

## Arguments
$ARGUMENTS = chemin du fichier XTF ou ITF

## Instructions

1. Verifier que le fichier existe
2. Detecter le modele INTERLIS depuis le fichier
3. Lancer ilivalidator:
   ```bash
   java -jar ilivalidator.jar --log result.log fichier.xtf
   ```
4. Parser le log de resultat
5. Afficher resume:
   - Nombre d'erreurs
   - Nombre d'avertissements
   - Details des erreurs par type
6. Si erreurs, proposer corrections courantes

## Erreurs frequentes et corrections

| Erreur | Cause | Correction |
|--------|-------|------------|
| MANDATORY constraint | Attribut obligatoire NULL | Completer la donnee |
| UNIQUE constraint | Doublon TID | Regenerer les TID |
| Geometry invalid | Geometrie invalide | ST_MakeValid dans PostGIS |
| Reference error | FK vers objet inexistant | Verifier coherence donnees |

## Modeles frequents Bussigny
- MOpublic_VD (cadastre)
- RDPPF (restrictions)
- SIA405_Eau, SIA405_Assainissement
- Zones_protection_eaux

## Emplacement ilivalidator
Si pas installe: telecharger depuis https://github.com/claeis/ilivalidator/releases
