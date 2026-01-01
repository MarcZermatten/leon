# /checkpoint - Sauvegarder État Mémoire

Met à jour la mémoire projet et commit.

## Instructions

1. Mettre à jour `.claude/memory/context.md`:
   - WIP: tâche actuelle
   - BLOCKER: problème si existe
   - LAST_FILES: 3 derniers fichiers modifiés
   - NEXT: prochaine action

2. Ajouter à `.claude/memory/decisions.md` si nouvelle décision d'archi:
   - Format: `[YYYY-MM-DD] SUJET: décision (raison)`

3. Vérifier decisions.md ne dépasse pas 30 entrées

4. Commit: `git add -A && git commit -m "checkpoint: [résumé]" && git push`

## Règles

- Ne PAS ajouter d'infos redondantes
- Ne PAS détailler ce qui est dans le code
- Garder context.md < 50 lignes
