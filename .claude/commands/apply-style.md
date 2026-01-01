# Skill: apply-style

Appliquer un style QML a une couche QGIS ou generer un nouveau style.

## Arguments
$ARGUMENTS = couche [style.qml | description_style]
- couche: nom de la couche dans QGIS ou table PostGIS
- style.qml: fichier style existant
- description_style: description en langage naturel

## Instructions

### Appliquer style existant
1. Charger le fichier .qml
2. Appliquer a la couche specifiee
3. Rafraichir l'affichage

### Generer nouveau style
1. Analyser la couche (attributs, type geometrie)
2. Interpreter la description:
   - "colorer par type" → categorise
   - "gradient selon valeur" → gradue
   - "etiquettes nom" → labeling
3. Generer le QML correspondant
4. Sauvegarder dans scripts/qgis/styles/
5. Appliquer a la couche

## Styles disponibles Bussigny

| Couche | Style | Description |
|--------|-------|-------------|
| parcelles | parcelles.qml | Contour noir, transparent |
| batiments | batiments.qml | Gris fonce, ombre |
| zones | zones_affectation.qml | Couleurs par zone |
| conduites_eau | conduites_eau.qml | Bleu, epaisseur par diametre |
| arbres | arbres.qml | Vert, taille par circonference |

## Code PyQGIS
```python
from qgis.core import QgsProject, QgsVectorLayer

# Charger style existant
layer = QgsProject.instance().mapLayersByName("Parcelles")[0]
layer.loadNamedStyle("scripts/qgis/styles/parcelles.qml")
layer.triggerRepaint()

# Style categorise (par attribut)
from qgis.core import (
    QgsCategorizedSymbolRenderer,
    QgsRendererCategory,
    QgsSymbol
)

categories = []
for value, color in [("Zone habitat", "#FFD700"), ("Zone activite", "#87CEEB")]:
    symbol = QgsSymbol.defaultSymbol(layer.geometryType())
    symbol.setColor(QColor(color))
    categories.append(QgsRendererCategory(value, symbol, value))

renderer = QgsCategorizedSymbolRenderer("type_zone", categories)
layer.setRenderer(renderer)

# Style gradue (par valeur numerique)
from qgis.core import QgsGraduatedSymbolRenderer, QgsGradientColorRamp

renderer = QgsGraduatedSymbolRenderer()
renderer.setClassAttribute("surface")
renderer.setSourceColorRamp(QgsGradientColorRamp(QColor("#FFFFCC"), QColor("#006837")))
renderer.updateClasses(layer, QgsGraduatedSymbolRenderer.EqualInterval, 5)
layer.setRenderer(renderer)

# Etiquettes
from qgis.core import QgsPalLayerSettings, QgsVectorLayerSimpleLabeling

settings = QgsPalLayerSettings()
settings.fieldName = "nom"
settings.enabled = True
labeling = QgsVectorLayerSimpleLabeling(settings)
layer.setLabeling(labeling)
layer.setLabelsEnabled(True)
```

## Types de rendu
| Type | Usage | Parametre cle |
|------|-------|---------------|
| Simple | Couleur unique | color, width |
| Categorise | Par valeur attribut | field, categories |
| Gradue | Par plage valeurs | field, classes, ramp |
| Regle | Conditions complexes | rules, expressions |

## Emplacement styles
scripts/qgis/styles/
