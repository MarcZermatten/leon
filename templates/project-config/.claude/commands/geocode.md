# Skill: geocode

Geocoder des adresses suisses via l'API swisstopo.

## Arguments
$ARGUMENTS = adresse OU fichier CSV avec colonne adresse

## Instructions

### Adresse unique
1. Appeler api3.geo.admin.ch/rest/services/api/SearchServer
2. Parser le resultat JSON
3. Retourner: coordonnees MN95, commune, NPA, score

### Fichier CSV
1. Lire le CSV
2. Identifier colonne adresse (adresse, address, rue, strasse)
3. Geocoder chaque ligne avec rate limiting (10 req/sec)
4. Ajouter colonnes: X, Y, score, commune_geo
5. Sauvegarder CSV enrichi
6. Rapport: taux de succes, adresses non trouvees

## Code type
```python
import requests
import time

def geocode_address(address):
    url = "https://api3.geo.admin.ch/rest/services/api/SearchServer"
    params = {
        'searchText': address,
        'type': 'locations',
        'limit': 1
    }
    response = requests.get(url, params=params)
    if response.status_code == 200:
        results = response.json().get('results', [])
        if results:
            attrs = results[0]['attrs']
            return {
                'x': attrs.get('x'),  # MN95 Est
                'y': attrs.get('y'),  # MN95 Nord
                'label': attrs.get('label'),
                'score': results[0].get('weight', 0)
            }
    return None

# Rate limiting
time.sleep(0.1)  # 100ms entre requetes
```

## API alternatives
- Nominatim (OSM) - mondial mais moins precis CH
- Google Geocoding - payant
- Here - payant
