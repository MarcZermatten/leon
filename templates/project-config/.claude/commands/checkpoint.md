# /checkpoint - Sauvegarder Etat Memoire

Met a jour la memoire projet et commit.

## Instructions

1. Mettre a jour `.claude/memory/context.md`:
   - WIP: tache actuelle
   - BLOCKER: probleme si existe
   - LAST_FILES: 3 derniers fichiers modifies
   - NEXT: prochaine action

2. Ajouter a `.claude/memory/decisions.md` si nouvelle decision d'archi:
   - Format: `[YYYY-MM-DD] SUJET: decision (raison)`

3. Commit: `git add -A && git commit -m "checkpoint: [resume]" && git push`

## Regles

- Ne PAS ajouter d'infos redondantes
- Ne PAS detailler ce qui est dans le code
- Garder context.md < 50 lignes
