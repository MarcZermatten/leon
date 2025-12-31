---
paths: **/*
---

# Regles de Securite

## Dossiers INTERDITS (Lecture/Ecriture)

JAMAIS toucher ces chemins, meme si demande :

```
C:\Windows\**
C:\Program Files\**
C:\Program Files (x86)\**
C:\ProgramData\**
C:\Users\*\AppData\Local\Microsoft\**
```

## Dossiers SENSIBLES (Demander Confirmation)

Toujours demander avant de modifier :

```
C:\Users\*\.ssh\**
C:\Users\*\.aws\**
*.env
*credentials*
*secret*
*password*
*.pem
*.key
```

## Dossiers AUTORISES (Travail Libre)

```
{{PROJECT_PATH}}\**
```

## Commandes INTERDITES

Ne JAMAIS executer :

```bash
# Destruction
rm -rf /
del /f /s /q C:\
format
diskpart

# Systeme
reg delete
bcdedit
```

## Commandes SENSIBLES (Demander Confirmation)

```bash
# Git dangereux
git push --force
git reset --hard
git clean -fd

# Suppression recursive
rm -rf
del /s /q
Remove-Item -Recurse -Force
```

## Reflexe de Securite

Avant toute commande destructive, se poser :
1. Est-ce reversible ?
2. Y a-t-il un backup ?
3. Est-ce dans un dossier autorise ?
4. L'utilisateur a-t-il explicitement demande ?

Si doute -> DEMANDER
