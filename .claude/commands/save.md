# /save - Sauvegarder sur GitHub

Commit et push les modifications du projet Léon.

## Instructions

1. Exécuter `git status` pour voir les changements
2. Exécuter `git diff --stat` pour résumer
3. Ajouter tous les fichiers modifiés: `git add -A`
4. Créer un commit avec message descriptif:
   - Format: `type: description courte`
   - Types: feat, fix, refactor, docs, style, chore
5. Push sur origin: `git push`
6. Confirmer le succès à l'utilisateur

## Exemple

```bash
git add -A && git commit -m "feat: add preview auto-update on file detection" && git push
```
