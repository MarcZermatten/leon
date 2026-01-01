# Skill: new-qgis

Creer un nouveau projet QGIS avec les couches de base Bussigny.

## Arguments
$ARGUMENTS = nom_projet [theme]
- theme: cadastre, reseaux, urbanisme, environnement (optionnel)

## Instructions

1. Creer fichier .qgz dans scripts/qgis/
2. Configurer le projet:
   - SRID: 2056 (MN95)
   - Emprise par defaut: Bussigny
   - Fond de plan: Swisstopo
3. Ajouter couches de base selon theme:
   - cadastre: parcelles, batiments, adresses
   - reseaux: conduites, regards, vannes
   - urbanisme: zones, PGA, servitudes
   - environnement: biotopes, arbres, zones_protection
4. Appliquer styles standards (.qml)
5. Configurer mise en page si demande

## Couches de base disponibles

### Fond de plan
- Swisstopo WMTS (plan, ortho, cadastre)
- OpenStreetMap

### Cadastre (theme: cadastre)
```
geo.parcelles
geo.batiments
geo.points_adresses
geo.limites_communes
```

### Reseaux (theme: reseaux)
```
geo.conduites_eau
geo.vannes
geo.hydrantes
geo.conduites_eaux_usees
geo.regards
```

### Urbanisme (theme: urbanisme)
```
geo.zones_affectation
geo.servitudes
geo.plans_quartier
```

## Code PyQGIS
```python
from qgis.core import (
    QgsProject, QgsVectorLayer, QgsRasterLayer,
    QgsCoordinateReferenceSystem
)

# Nouveau projet
project = QgsProject.instance()
project.setCrs(QgsCoordinateReferenceSystem("EPSG:2056"))

# Connexion PostGIS
uri = "dbname='Prod' host=srv-fme port=5432 user='postgres' password='dsg#6hY95!' sslmode=disable"

# Ajouter couche
layer_uri = f"{uri} key='id' table=\"geo\".\"parcelles\" (geometry)"
layer = QgsVectorLayer(layer_uri, "Parcelles", "postgres")
project.addMapLayer(layer)

# Swisstopo WMTS
wmts_url = "contextualWMSLegend=0&crs=EPSG:2056&dpiMode=7&featureCount=10&format=image/png&layers=ch.swisstopo.pixelkarte-farbe&styles&tileMatrixSet=2056&url=https://wmts.geo.admin.ch/EPSG/2056/1.0.0/WMTSCapabilities.xml"
basemap = QgsRasterLayer(wmts_url, "Swisstopo", "wms")
project.addMapLayer(basemap)

# Sauvegarder
project.write(f"scripts/qgis/{nom_projet}.qgz")
```

## Structure projet standard
```
scripts/qgis/
├── styles/           # Fichiers .qml
│   ├── parcelles.qml
│   ├── batiments.qml
│   └── ...
├── templates/        # Mises en page
│   ├── A4_portrait.qpt
│   └── A3_paysage.qpt
└── projets/          # Fichiers .qgz
    └── {nom_projet}.qgz
```
