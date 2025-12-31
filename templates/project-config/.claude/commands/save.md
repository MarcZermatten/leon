# /save - Sauvegarder sur GitHub

Commit et push les modifications du projet.

## Instructions

1. Executer `git status` pour voir les changements
2. Executer `git diff --stat` pour resumer
3. Ajouter tous les fichiers modifies: `git add -A`
4. Creer un commit avec message descriptif:
   - Format: `type: description courte`
   - Types: feat, fix, refactor, docs, style, chore
5. Push sur origin: `git push`
6. Confirmer le succes a l'utilisateur

## Exemple

```bash
git add -A && git commit -m "feat: add new feature" && git push
```
