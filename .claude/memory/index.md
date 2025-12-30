# Système de Mémoire Léon

## Règles de Gestion

### Principe : Mémoire Compacte et Utile
La mémoire doit rester **légère et actionable**. Pas de prose, pas de redondance.

### Fichiers et Rétention

| Fichier | Contenu | Rétention | Limite |
|---------|---------|-----------|--------|
| `context.md` | État actuel, WIP | Écrasé à chaque session | 50 lignes max |
| `decisions.md` | Choix d'architecture | Permanent, append-only | 30 entrées max |

### Format Obligatoire

**context.md** - Format clé:valeur
```
WIP: [tâche en cours]
BLOCKER: [problème actuel si existe]
LAST_FILES: [3 derniers fichiers modifiés]
NEXT: [prochaine action prévue]
```

**decisions.md** - Format compact
```
[YYYY-MM-DD] SUJET: décision (raison courte)
```

### Règles Anti-Pollution

1. **Pas de doublons** - Vérifier avant d'ajouter
2. **Pas d'évidences** - Ne pas écrire ce qui est dans le code
3. **Pas d'historique détaillé** - Seul le dernier état compte
4. **Pas de TODO lists** - Utiliser les issues GitHub
5. **Supprimer l'obsolète** - Si c'est fait, effacer

### Nettoyage Automatique

À chaque `/checkpoint` :
1. context.md → Écraser avec état actuel
2. decisions.md → Garder 30 dernières, archiver le reste
3. Supprimer entrées > 90 jours sans référence

### Quand Écrire

| Événement | Action |
|-----------|--------|
| Nouvelle décision d'archi | → decisions.md |
| Changement de tâche | → context.md (écraser) |
| Bug résolu | → Rien (c'est dans Git) |
| Feature terminée | → Rien (c'est dans Git) |
| Pattern découvert | → decisions.md si réutilisable |

### Quand NE PAS Écrire

- Infos déjà dans CLAUDE.md
- Infos déjà dans le code (types, commentaires)
- États temporaires (< 1 jour)
- Logs de debug
- Messages de commit (c'est dans Git)
