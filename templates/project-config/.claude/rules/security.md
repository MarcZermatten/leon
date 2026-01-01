---
paths: **/*
---

# Règles de Sécurité Léon

## Dossiers INTERDITS (Lecture/Écriture)

JAMAIS toucher ces chemins, même si demandé :

```
C:\Windows\**
C:\Program Files\**
C:\Program Files (x86)\**
C:\ProgramData\**
C:\Users\*\AppData\Local\Microsoft\**
C:\Users\*\AppData\Roaming\Microsoft\**
C:\Users\*\NTUSER.DAT
C:\System Volume Information\**
C:\$Recycle.Bin\**
```

## Dossiers SENSIBLES (Demander Confirmation)

Toujours demander avant de modifier :

```
C:\Users\Marc\.ssh\**
C:\Users\Marc\.gnupg\**
C:\Users\Marc\.aws\**
C:\Users\Marc\.azure\**
C:\Users\Marc\.config\**
C:\Users\Marc\.claude\settings*.json
*.env
*credentials*
*secret*
*password*
*.pem
*.key
*.p12
```

## Dossiers AUTORISÉS (Travail Libre)

```
C:\Users\Marc\projets\leon\**
C:\Users\Marc\projets\**
C:\Users\Marc\GeoBrain\**
C:\Users\Marc\Documents\**
C:\Users\Marc\Desktop\**
```

## Commandes INTERDITES

Ne JAMAIS exécuter :

```bash
# Destruction
rm -rf /
del /f /s /q C:\
format
diskpart

# Système
reg delete
bcdedit
sfc
dism

# Réseau dangereux
netsh advfirewall set
route delete

# Permissions
takeown /f C:\Windows
icacls C:\Windows /grant

# Processus système
taskkill /f /im winlogon.exe
taskkill /f /im csrss.exe
taskkill /f /im services.exe
```

## Commandes SENSIBLES (Demander Confirmation)

```bash
# Git dangereux
git push --force
git reset --hard
git clean -fd

# Suppression récursive
rm -rf
del /s /q
Remove-Item -Recurse -Force

# Installation globale
npm install -g
pip install --user
cargo install
```

## Actions AVANT Modification

1. **Fichiers de config** → Créer backup `.bak` avant
2. **Plus de 5 fichiers** → Lister et demander confirmation
3. **Fichiers > 100KB** → Avertir avant suppression
4. **Hors projet Léon** → Demander confirmation explicite

## Réflexe de Sécurité

Avant toute commande destructive, se poser :
1. Est-ce réversible ?
2. Y a-t-il un backup ?
3. Est-ce dans un dossier autorisé ?
4. L'utilisateur a-t-il explicitement demandé ?

Si doute → DEMANDER
