<div align="center">

[简体中文](README.md) | [繁體中文](README_TC.md) | [English](README_EN.md) | [日本語](README_JA.md) | [한국어](README_KO.md) | [Русский](README_RU.md) | [Français](README_FR.md) | [Deutsch](README_DE.md) | [Español](README_ES.md) | [Italiano](README_IT.md) | [Português](README_PT.md)

# 🎵 Ter-Music-Rust - Lecteur de musique en terminal 🎵

</div>

Un lecteur de musique en terminal, simple et pratique, développé en Rust. Il prend en charge la recherche/téléchargement local et en ligne, le téléchargement et l'affichage automatiques des paroles, l'affichage des commentaires, ainsi que le changement de langue et de thème, sur Windows, Linux et MacOS.

![preview1](preview1.png)

![preview2](preview2.png)

![preview3](preview3.png)

![preview4](preview4.png)

![preview5](preview5.png)

![preview6](preview6.png)

## ✨ Fonctionnalités

### 🎵 Lecture audio
- **10 formats audio pris en charge** : MP3, WAV, FLAC, OGG, OGA, Opus, M4A, AAC, AIFF, APE
- **Contrôles de lecture** : lecture/pause/arrêt, piste précédente/suivante
- **Avance rapide** : saut rapide de 5s / 10s
- **Recherche par barre de progression** : cliquez sur la barre de progression pour un saut précis
- **Contrôle du volume** : ajustement en temps réel de 0 à 100, cliquez sur la barre de volume pour définir

### 🔄 5 Modes de lecture
| Touche | Mode | Description |
|------|------|------|
| `1` | Lecture unique | Arrêt après la fin de la piste en cours |
| `2` | Boucle unique | Répéter la piste en cours |
| `3` | Lecture séquentielle | Lecture dans l'ordre, arrêt à la fin |
| `4` | Boucle de liste | Répéter toute la liste de lecture |
| `5` | Lecture aléatoire | Sélection aléatoire des pistes |

### 📜 Système de paroles
- **Chargement local des paroles** : recherche automatique des fichiers `.lrc` correspondants
- **Détection de l'encodage des paroles** : détection automatique UTF-8 / GBK
- **Téléchargement automatique en ligne** : téléchargement asynchrone en arrière-plan lorsque les paroles locales sont manquantes
- **Surbrillance défilante** : la ligne en cours est mise en surbrillance avec `►`, défilement automatique centré
- **Saut par position de paroles** : glissez la zone des paroles ou utilisez la molette de la souris pour sauter à l'horodatage des paroles

### 🔍 Recherche
- **Recherche locale** : appuyez sur `s` pour rechercher des chansons dans le répertoire de musique actuel
- **Recherche en ligne** : appuyez sur `n` pour rechercher des chansons en ligne par mot-clé
- **Recherche Juhe** : appuyez sur `j` pour entrer. Recherche de chansons Juhe basée sur la correspondance de mots-clés.
- **Recherche de listes de lecture** : appuyez sur `p` pour entrer. Recherche de listes de lecture en ligne basée sur la correspondance de mots-clés.
- **Pagination** : `PgUp` / `PgDn` pour plus de résultats
- **Téléchargement en ligne** : appuyez sur `Entrée` sur le résultat en ligne sélectionné pour télécharger dans le répertoire de musique actuel (avec affichage de la progression)

### 🤖 Informations sur la chanson
- **Requête intelligente** : appuyez sur `i` pour interroger les informations détaillées de la chanson, prend en charge toute API compatible OpenAI
- **Sortie en flux** : les résultats s'affichent caractère par caractère, pas besoin d'attendre la génération complète
- **Informations riches** : couvre 13 catégories, notamment les détails de l'artiste, l'écriture, la liste des pistes de l'album, le contexte créatif, le sens des paroles, le style musical, des anecdotes, et plus encore
- **Prise en charge multilingue** : la langue de la réponse suit le paramètre de langue de l'interface (SC/TC/EN/JP/KR)
- **API personnalisée** : appuyez sur `k` pour configurer l'URL de base de l'API, la clé API et le nom du modèle en 3 étapes — prend en charge DeepSeek, OpenRouter, AIHubMix et tout point de terminaison compatible OpenAI
- **Repli gratuit** : utilise automatiquement le modèle gratuit d'OpenRouter (minimax/minimax-m2.5:free) lorsqu'aucune clé API n'est configurée

### ⭐ Favoris
- **Ajouter/supprimer des favoris** : appuyez sur `f` pour basculer l'état favori de la piste en cours
- **Liste des favoris** : appuyez sur `v` pour voir les favoris (avec le marqueur `★`)
- **Lecture inter-répertoires** : changement automatique de répertoire lorsqu'un favori se trouve en dehors du répertoire actuel
- **Supprimer un favori** : appuyez sur `d` dans la liste des favoris

### 💬 Commentaires
- **Commentaires de chanson** : appuyez sur `c` pour voir les commentaires de la chanson en cours
- **Détails des commentaires** : appuyez sur `Entrée` pour basculer entre la vue liste/détail (texte complet en détail)
- **Affichage des réponses** : montre le texte du commentaire original, le pseudo et l'heure
- **Pagination des commentaires** : `PgUp` / `PgDn`, 20 commentaires par page
- **Chargement en arrière-plan** : les commentaires sont récupérés dans des threads d'arrière-plan sans bloquer l'interface

### 📂 Gestion des répertoires
- **Choisir le répertoire de musique** : appuyez sur `o` pour ouvrir la boîte de dialogue de sélection de dossier (la lecture démarre automatiquement après la première ouverture réussie)
- **Historique des répertoires** : appuyez sur `m` pour voir et changer rapidement de répertoire
- **Marqueur du répertoire actuel** : `▶` indique le répertoire actif actuel
- **Supprimer un élément de l'historique** : appuyez sur `d` dans la vue de l'historique

### 🌐 Interface multilingue
Prend en charge 11 langues d'interface (cycle avec `l`) :

| Langue | Valeur de configuration |
|------|--------|
| Chinois simplifié | `sc` |
| Chinois traditionnel | `tc` |
| Anglais | `en` |
| Japonais | `ja` |
| Coréen | `ko` |
| Russe | `ru` |
| Français | `fr` |
| Allemand | `de` |
| Espagnol | `es` |
| Italien | `it` |
| Portugais | `pt` |

### 🎨 Interface multi-thème
Prend en charge 4 thèmes (cycle avec `t`) :

| Thème | Style |
|------|------|
| Neon | Tons néon |
| Sunset | Or crépusculaire chaleureux |
| Ocean | Bleu océan profond |
| GrayWhite | Niveaux de gris style console |

### 🖱️ Interaction souris
- **Clic sur la liste de lecture** : cliquez pour lire directement la chanson
- **Clic sur la barre de progression** : sauter à une position précise
- **Clic sur la barre de volume** : ajuster le volume
- **Glissement des paroles** : glissement à gauche pour sauter à l'horodatage des paroles
- **Molette sur les paroles** : défilement haut/bas pour sauter à la ligne de paroles précédente/suivante
- **Clic sur un résultat de recherche** : recherche locale - clic pour lire, recherche en ligne - clic pour télécharger
- **Clic sur un commentaire** : cliquez pour ouvrir les détails

### 📊 Visualisation de la forme d'onde
- Barres de forme d'onde dynamiques basées sur le volume RMS réel pendant la lecture
- Lissage EMA pour un rendu plus doux
- La forme d'onde se fige en pause

### ⚙️ Configuration persistante
La configuration est stockée dans `USERPROFILE/ter-music-rust/config.json` dans le répertoire du programme et est automatiquement sauvegardée/restaurée :

| Élément de configuration | Description |
|--------|------|
| `music_directory` | Dernier répertoire de musique ouvert |
| `play_mode` | Mode de lecture |
| `current_index` | Index de la dernière chanson jouée (reprise de la lecture) |
| `volume` | Volume (0-100) |
| `favorites` | Liste des favoris |
| `dir_history` | Historique des répertoires |
| `api_key` | Clé API (pour la requête d'informations sur les chansons, compatible avec `deepseek_api_key`) |
| `api_base_url` | URL de base de l'API (par défaut : `https://api.deepseek.com/`) |
| `api_model` | Nom du modèle AI (par défaut : `deepseek-v4-flash`) |
| `github_token` | Jeton GitHub (utilisé pour soumettre des discussions sur les chansons ; laisser vide pour utiliser le jeton par défaut) |
| `theme` | Nom du thème |
| `language` | Langue de l'interface (`sc` / `tc` / `en` / `ja` / `ko` / `ru` / `fr` / `de` / `es` / `it` / `pt`) |

**Déclencheurs de sauvegarde automatique** : changement de piste, changement de thème, changement de langue, modification des favoris, toutes les 30 secondes, et à la sortie (y compris Ctrl+C)

---

## 🚀 Démarrage rapide

### 1. Installer Rust

```powershell
# Méthode 1 : winget (recommandé)
winget install Rustlang.Rustup

# Méthode 2 : installateur officiel
# Visitez https://rustup.rs/ et installez
```

Vérifier l'installation :

```powershell
rustc --version
cargo --version
```

### 2. Compiler le projet

```powershell
cd <répertoire-du-projet>

# Méthode 1 : script de compilation (recommandé)
build-win.bat

# Méthode 2 : Cargo
cargo build --release
```

### 3. Exécuter

```powershell
# Méthode 1 : cargo run
cargo run --release

# Méthode 2 : exécuter directement
.\target\release\ter-music-rust.exe

# Méthode 3 : spécifier le répertoire de musique
.\target\release\ter-music-rust.exe -o d:\Music
cargo run --release -- -o d:\Music
```

**Priorité de chargement des répertoires** : ligne de commande `-o` > fichier de configuration > boîte de dialogue de sélection de dossier

---

## 🎮 Raccourcis clavier

### Vue principale

| Touche | Action |
|------|------|
| `↑/↓` | Sélectionner une chanson |
| `Entrée` | Lire la chanson sélectionnée |
| `Espace` | Lecture/Pause |
| `Échap` | Arrêter la lecture (dans la vue commentaires : retour aux paroles) |
| `←/→` | Chanson précédente/suivante |
| `[` | Reculer de 5s |
| `]` | Avancer de 5s |
| `,` | Reculer de 10s |
| `.` | Avancer de 10s |
| `+/-` | Augmenter/diminuer le volume (pas de 5) |
| `1-5` | Changer de mode de lecture |
| `o` | Ouvrir le répertoire de musique |
| `s` | Rechercher des chansons locales |
| `n` | Rechercher des chansons en ligne |
| `j` | Recherche Juhe |
| `p` | Rechercher des listes de lecture en ligne |
| `i` | Requête d'informations sur la chanson |
| `f` | Ajouter/Retirer des favoris |
| `v` | Voir les favoris |
| `m` | Voir l'historique des répertoires |
| `h` | Afficher l'aide |
| `c` | Voir les commentaires de la chanson |
| `l` | Changer la langue de l'interface |
| `t` | Changer de thème |
| `k` | Configurer le point de terminaison API |
| `g` | Configurer le jeton GitHub |
| `q` | Quitter |

### Vue de recherche

| Touche | Action |
|------|------|
| Saisie de caractères | Entrer le mot-clé de recherche |
| `Retour arrière` | Supprimer un caractère |
| `Entrée` | Rechercher/Lire/Télécharger |
| `↑/↓` | Sélectionner un résultat |
| `PgUp/PgDn` | Page haut/bas (recherche en ligne) |
| `s/n/j` | Basculer recherche locale/en ligne/Juhe |

| `Échap` | Quitter la recherche |

### Vue des favoris

| Touche | Action |
|------|------|
| `↑/↓` | Sélectionner une chanson |
| `Entrée` | Lire la chanson sélectionnée |
| `d` | Supprimer le favori |
| `Échap` | Retour à la liste de lecture |

### Vue de l'historique des répertoires

| Touche | Action |
|------|------|
| `↑/↓` | Sélectionner un répertoire |
| `Entrée` | Basculer vers le répertoire sélectionné |
| `d` | Supprimer l'enregistrement |
| `Échap` | Retour à la liste de lecture |

### Vue des commentaires

| Touche | Action |
|------|------|
| `↑/↓` | Sélectionner un commentaire |
| `Entrée` | Basculer vue liste/détail |
| `PgUp/PgDn` | Page haut/bas |
| `Échap` | Retour à la vue des paroles |

### Vue des informations sur la chanson

| Touche | Action |
|------|------|
| `↑/↓` | Faire défiler les informations |
| `i` | Re-interroger les informations |
| `Échap` | Retour à la vue des paroles |

### Vue de recherche de listes de lecture

| Touche | Action |
|------|------|
| Saisie de caractères | Entrer le mot-clé de la liste de lecture |
| `Retour arrière` | Supprimer un caractère |
| `Entrée` | Rechercher/Entrer dans la liste/Lire et télécharger |
| `↑/↓` | Sélectionner une liste ou une chanson |
| `PgUp/PgDn` | Page haut/bas |
| `Échap` | Retour au niveau précédent / Quitter la recherche |

### Vue d'aide


| Touche | Action |
|------|------|
| `↑/↓` | Faire défiler le contenu de l'aide |
| `Échap` | Retour à la vue des paroles |

---

## 📦 Installation et compilation

### Configuration système requise

- **Système d'exploitation** : Windows 10/11
- **Rust** : 1.70+
- **Terminal** : Windows Terminal (recommandé) / CMD / PowerShell
- **Taille de la fenêtre** : 80×25 ou plus recommandée

### Option 1 : Chaîne d'outils MSVC (meilleure compatibilité, taille plus importante)

```powershell
# 1. Installer Rust
winget install Rustlang.Rustup

# 2. Installer les outils de compilation
winget install Microsoft.VisualStudio.2022.BuildTools
# Exécuter l'installateur -> sélectionner « Développement Desktop en C++ » -> installer

# 3. Redémarrer le terminal et compiler
cargo build --release
```

### Option 2 : Chaîne d'outils GNU (recommandée, légère ~300 Mo)

```powershell
# 1. Installer Rust
winget install Rustlang.Rustup

# 2. Installer MSYS2
winget install MSYS2.MSYS2
# Dans le terminal MSYS2, exécuter :
pacman -Syu
pacman -S mingw-w64-x86_64-toolchain

# 3. Ajouter au PATH (PowerShell en tant qu'administrateur)
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\msys64\mingw64\bin", "Machine")

# 4. Changer de chaîne d'outils et compiler
rustup default stable-x86_64-pc-windows-gnu
cargo build --release
```

> Les programmes compilés avec la chaîne d'outils GNU peuvent nécessiter ces DLL dans le répertoire de l'exécutable :
> `libgcc_s_seh-1.dll`, `libstdc++-6.dll`, `libwinpthread-1.dll`

### Option 3 : Compilation croisée Linux sur Windows

Utilisez `cargo-zigbuild` + `zig` comme éditeur de liens. Aucune machine virtuelle/installation Linux requise.

```powershell
# 1. Installer zig (choisir une option)
# A : via pip (recommandé)
pip install ziglang

# B : via MSYS2
pacman -S mingw-w64-x86_64-zig

# C : téléchargement manuel
# Visitez https://ziglang.org/download/, décompressez et ajoutez au PATH

# 2. Installer cargo-zigbuild
cargo install cargo-zigbuild

# 3. Ajouter la cible Linux
rustup target add x86_64-unknown-linux-gnu

# 4. Préparer le sysroot Linux (en-têtes/bibliothèques ALSA)
# Le projet inclut déjà linux-sysroot/
# Pour une préparation manuelle, copiez depuis Debian/Ubuntu :
#   /usr/include/alsa/ -> linux-sysroot/usr/include/alsa/
#   /usr/lib/x86_64-linux-gnu/libasound.so* -> linux-sysroot/usr/lib/x86_64-linux-gnu/

# 5. Compiler
build-linux.bat

# Ou exécuter manuellement :
cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.34
```

**Résultat** : `target/x86_64-unknown-linux-gnu/release/ter-music-rust`

**Déploiement sur Linux** :

```bash
# 1. Copier sur l'hôte Linux
scp ter-music-rust user@linux-host:~/

# 2. Rendre exécutable
chmod +x ter-music-rust

# 3. Installer le runtime ALSA
sudo apt install libasound2

# 4. Exécuter
./ter-music-rust -o /path/to/music
```

> `build-linux.bat` configure automatiquement `PKG_CONFIG_PATH`, `PKG_CONFIG_ALLOW_CROSS`, `RUSTFLAGS`, etc.
> Dans la cible `x86_64-unknown-linux-gnu.2.34`, `.2.34` indique la version minimale de glibc pour une meilleure compatibilité avec les anciens systèmes Linux.

### Empaquetage Linux (DEB / RPM)

Si vous compilez/empaquetez sur Linux, utilisez :

```bash
# 1) RPM
./build-rpm.sh

# Générer un RPM debuginfo (optionnel)
./build-rpm.sh --with-debuginfo

# 2) DEB
./build-deb.sh

# Générer un DEB avec symboles de débogage (optionnel)
./build-deb.sh --with-debuginfo

# Générer un paquet source conforme à dpkg-source (.dsc/.orig.tar/.debian.tar)
./build-deb.sh --with-source

# Générer debuginfo + paquet source
./build-deb.sh --with-debuginfo --with-source
```

Répertoires de sortie par défaut :
- `dist/rpm/` : RPM / SRPM
- `dist/deb/` : DEB / paquets source

> Les scripts lisent `name` et `version` depuis `Cargo.toml` pour nommer automatiquement les fichiers de paquets.

### Option 4 : Compilation croisée MacOS sur Windows

Utilisez `cargo-zigbuild` + `zig` + SDK MacOS. L'audio sur MacOS utilise CoreAudio et nécessite les en-têtes du SDK.

**Prérequis :**

```powershell
# 1. Installer zig (comme pour la compilation croisée Linux)
pip install ziglang

# 2. Installer cargo-zigbuild
cargo install cargo-zigbuild

# 3. Installer LLVM/Clang (fournit libclang.dll pour bindgen)
# A : via MSYS2
pacman -S mingw-w64-x86_64-clang

# B : LLVM officiel
winget install LLVM.LLVM

# 4. Ajouter les cibles MacOS
rustup target add x86_64-apple-darwin aarch64-apple-darwin
```

**Préparer le SDK MacOS :**

Décompressez `MacOSX13.3.sdk.tar.xz` dans `macos-sysroot`.
Le projet inclut déjà `macos-sysroot/` (téléchargé depuis [macosx-sdks](https://github.com/joseluisq/macosx-sdks)).

Pour le récupérer à nouveau :

```powershell
# A : Télécharger le SDK préemballé depuis GitHub (recommandé, ~56 Mo)
# Miroir : https://ghfast.top/https://github.com/joseluisq/macosx-sdks/releases/download/13.3/MacOSX13.3.sdk.tar.xz
curl -L -o MacOSX13.3.sdk.tar.xz https://github.com/joseluisq/macosx-sdks/releases/download/13.3/MacOSX13.3.sdk.tar.xz
mkdir macos-sysroot
tar -xf MacOSX13.3.sdk.tar.xz -C macos-sysroot --strip-components=1
del MacOSX13.3.sdk.tar.xz

# B : Copier depuis un système MacOS
scp -r mac:/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk ./macos-sysroot
```

> Source du SDK : https://github.com/joseluisq/macosx-sdks
> Inclut les en-têtes pour CoreAudio, AudioToolbox, AudioUnit, CoreMIDI, OpenAL, IOKit, etc.

**Compilation :**

```powershell
# Utiliser le script de compilation (définit automatiquement toutes les variables d'environnement)
build-mac.bat

# Ou manuellement :
$env:LIBCLANG_PATH = "C:\msys64\mingw64\bin"      # Répertoire contenant libclang.dll
$env:COREAUDIO_SDK_PATH = "./macos-sysroot"         # Chemin du SDK MacOS (barres obliques)
$env:SDKROOT = "./macos-sysroot"                    # Nécessaire à l'éditeur de liens zig pour localiser les bibliothèques système
$FW = "./macos-sysroot/System/Library/Frameworks"
$env:BINDGEN_EXTRA_CLANG_ARGS = "--target=x86_64-apple-darwin -isysroot ./macos-sysroot -F $FW -iframework $FW -I ./macos-sysroot/usr/include"
cargo zigbuild --release --target x86_64-apple-darwin   # Mac Intel
# Pour Apple Silicon, remplacez x86_64 par aarch64
cargo zigbuild --release --target aarch64-apple-darwin  # Apple Silicon
```

**Résultats :**
- `target/x86_64-apple-darwin/release/ter-music-rust` — Mac Intel
- `target/aarch64-apple-darwin/release/ter-music-rust` — Apple Silicon (M1/M2/M3/M4)

**Déploiement sur MacOS** :

```bash
# 1. Copier sur l'hôte MacOS
scp ter-music-rust user@mac-host:~/

# 2. Rendre exécutable
chmod +x ter-music-rust

# 3. Autoriser l'exécution d'un binaire de source inconnue
xattr -cr ter-music-rust

# 4. Exécuter (aucune bibliothèque audio supplémentaire requise)
./ter-music-rust -o /path/to/music
```

> Remarque : la compilation croisée MacOS nécessite les en-têtes du SDK MacOS ; ce projet inclut déjà `macos-sysroot/`.
> Elle nécessite également `libclang.dll` (installez via MSYS2 ou LLVM).

### Changer de chaîne d'outils

```powershell
# Afficher la chaîne d'outils actuelle
rustup show

# Passer à MSVC
rustup default stable-x86_64-pc-windows-msvc

# Passer à GNU
rustup default stable-x86_64-pc-windows-gnu
```

### Miroir Cargo en Chine (téléchargements plus rapides)

Créez ou éditez `~/.cargo/config` :

```toml
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
```

---

## 🛠️ Structure du projet

```text
src/
├── main.rs       # Point d'entrée (analyse des arguments, initialisation, restauration/sauvegarde de la configuration)
├── defs.rs       # Définitions partagées (énumérations PlayMode/PlayState, structures MusicFile/Playlist)
├── audio.rs      # Contrôle audio (encapsulage rodio, lecture/pause/saut/volume/progression)
├── analyzer.rs   # Analyseur audio (volume RMS en temps réel, lissage EMA, rendu de la forme d'onde)
├── playlist.rs   # Gestion de la liste de lecture (scan de répertoire, chargement parallèle des durées, sélection de dossier)
├── lyrics.rs     # Analyse des paroles (LRC, recherche locale, détection d'encodage, téléchargement en arrière-plan)
├── search.rs     # Recherche/téléchargement en ligne (recherche Kuwo + Kugou + NetEase, téléchargement, récupération des commentaires, requête en flux des informations sur les chansons)
├── config.rs     # Gestion de la configuration (sérialisation JSON, 8 éléments persistants)
└── ui.rs         # Interface utilisateur (rendu terminal, gestion des événements, mode multi-vues, système de thèmes/langues)
```

### Pile technologique

| Dépendance | Version | Usage |
|------|------|------|
| [rodio](https://github.com/RustAudio/rodio) | 0.19 | Décodage et lecture audio (Rust pur) |
| [crossterm](https://github.com/crossterm-rs/crossterm) | 0.28 | Contrôle de l'interface terminale |
| [reqwest](https://github.com/seanmonstar/reqwest) | 0.12 | Requêtes HTTP |
| [serde](https://github.com/serde-rs/serde) + serde_json | 1.0 | Sérialisation JSON |
| [rayon](https://github.com/rayon-rs/rayon) | 1.10 | Chargement parallèle des durées audio |
| [encoding_rs](https://github.com/hsivonen/encoding_rs) | 0.8 | Décodage des paroles GBK |
| [walkdir](https://github.com/BurntSushi/walkdir) | 2.5 | Scan récursif des répertoires |
| [rand](https://github.com/rust-random/rand) | 0.8 | Mode de lecture aléatoire |
| [unicode-width](https://github.com/unicode-rs/unicode-width) | 0.2 | Calcul de la largeur d'affichage CJK |
| [chrono](https://github.com/chronotope/chrono) | 0.4 | Formatage de l'heure des commentaires |
| [ctrlc](https://github.com/Detegr/rust-ctrlc) | 3.4 | Gestion du signal Ctrl+C |
| [md5](https://github.com/johannhof/md5) | 0.7 | Signature MD5 de l'API Kugou Music |
| [winapi](https://github.com/retep998/winapi-rs) | 0.3 | Prise en charge UTF-8 de la console Windows |

### Optimisation de la compilation Release

```toml
[profile.release]
opt-level = 3       # niveau d'optimisation le plus élevé
lto = true          # optimisation à l'édition de liens
codegen-units = 1   # unité de génération de code unique pour une meilleure optimisation
strip = true        # suppression des symboles de débogage
```

---

## Comparaison entre Rust et la version C

| Caractéristique | Version Rust | Version C |
|------|-----------|--------|
| Taille d'installation | ~200 Mo (Rust) / ~300 Mo (GNU) | ~7 Go (Visual Studio) |
| Temps d'installation | ~5 minutes | ~1 heure |
| Vitesse de compilation | ⚡ Rapide | 🐢 Plus lente |
| Gestion des dépendances | ✅ Automatique via Cargo | ❌ Configuration manuelle |
| Sécurité mémoire | ✅ Garanties à la compilation | ⚠️ Gestion manuelle nécessaire |
| Multiplateforme | ✅ Entièrement multiplateforme | ⚠️ Nécessite des modifications du code |
| Taille de l'exécutable | ~2 Mo | ~500 Ko |
| Utilisation mémoire | ~15-20 Mo | ~10 Mo |
| Utilisation CPU | < 1% | < 1% |

---

## 📊 Performances

| Métrique | Valeur |
|------|------|
| Intervalle de rafraîchissement de l'interface | 50ms |
| Réponse aux touches | < 50ms |
| Téléchargement des paroles | En arrière-plan, sans blocage |
| Scan de répertoire | Chargement parallèle des durées, accélération 2-4x |
| Temps de démarrage | < 100ms |
| Utilisation mémoire | ~15-20 Mo |

---

## 🐛 Dépannage

### Erreurs de compilation

```powershell
# Mettre à jour Rust
rustup update

# Nettoyer et recompiler
cargo clean
cargo build --release
```

### `link.exe introuvable`

Installez Visual Studio Build Tools (voir Option 1 ci-dessus).

### `dlltool.exe introuvable`

Installez la chaîne d'outils MinGW-w64 complète (voir Option 2 ci-dessus).

### DLL runtime manquantes (chaîne d'outils GNU)

```powershell
Copy-Item "C:\msys64\mingw64\bin\libgcc_s_seh-1.dll" -Destination ".\target\release\"
Copy-Item "C:\msys64\mingw64\bin\libstdc++-6.dll" -Destination ".\target\release\"
Copy-Item "C:\msys64\mingw64\bin\libwinpthread-1.dll" -Destination ".\target\release\"
```

### Aucun périphérique audio trouvé

1. Vérifiez que le périphérique audio du système fonctionne
2. Vérifiez les paramètres de volume Windows
3. Essayez de lire un son de test système

### Problèmes de rendu de l'interface

- Assurez-vous que la taille de la fenêtre du terminal est d'au moins 80×25
- Utilisez Windows Terminal pour une meilleure expérience
- Dans CMD, assurez-vous que la police sélectionnée prend en charge les caractères CJK si nécessaire

### La recherche en ligne / le téléchargement des paroles échoue

- Vérifiez votre connexion réseau
- Certaines chansons peuvent nécessiter un accès VIP ou avoir été supprimées
- Le fichier de paroles doit être au format LRC standard valide

### La requête d'informations sur la chanson échoue

- Lorsqu'aucune clé API n'est configurée, le modèle gratuit d'OpenRouter est utilisé automatiquement — aucune configuration manuelle nécessaire
- Pour utiliser un point de terminaison personnalisé, appuyez sur `k` et entrez l'URL de base de l'API, la clé API et le nom du modèle successivement
- Prend en charge toute API compatible OpenAI (DeepSeek, OpenRouter, AIHubMix, etc.)
- Vérifiez la connectivité réseau avec le service API correspondant

### Première compilation lente

La première compilation télécharge et compile toutes les dépendances ; c'est normal. Les compilations suivantes sont beaucoup plus rapides.

### Télécharger les versions
[ter-music-rust-win.zip](https://storage.deepin.org/thread/202605041058546980_ter-music-rust-win.zip "附件(Attached)") 
[ter-music-rust-mac.zip](https://storage.deepin.org/thread/202605041059025049_ter-music-rust-mac.zip "附件(Attached)") 
[ter-music-rust-linux.zip](https://storage.deepin.org/thread/202605041059164016_ter-music-rust-linux.zip "附件(Attached)") 
[ter-music-rust_deb.zip](https://storage.deepin.org/thread/202605041059236181_ter-music-rust_deb.zip "附件(Attached)")

---

## Version 1.6.0 (2026-05-04)

### 🎉 Nouvelles fonctionnalités

#### Extension multilingue et refactoring d'internationalisation
- ✨ **6 nouvelles langues d'interface ajoutées** : russe (Русский), français (Français), allemand (Deutsch), espagnol (Español), italien (Italiano), portugais (Português) — 11 langues désormais supportées
- ✨ **Internationalisation complète des modules** : tous les textes orientés utilisateur (interface UI, aide CLI, messages d'erreur, titres de boîtes de dialogue) ont été internationalisés, y compris `ui.rs`, `main.rs`, `search.rs`, `audio.rs`, `config.rs`, `playlist.rs`
- ✨ **Gestion centralisée du pack de langues** : ajout du module `langs.rs` pour centraliser tous les textes de traduction dans un seul fichier, incluant la structure `LangTexts` et 11 instances statiques de langues
- ✨ **Accesseur de langue global** : fournit la fonction `langs::global_texts()` pour que les modules non-UI (search.rs / audio.rs / config.rs / playlist.rs) puissent récupérer de manière thread-safe les textes de traduction actuels
- ✨ **Prompts AI multilingues** : les prompts de requête d'informations sur les chansons pour chaque langue sont générés dans la langue correspondante, garantissant que la langue de réponse correspond à la langue de l'interface

### 🔧 Améliorations

- 🌐 **Internationalisation de l'aide CLI** : l'information d'aide `-h` en ligne de commande suit désormais le paramètre de langue de l'interface
- 🌐 **Internationalisation des messages d'erreur** : les erreurs audio, recherche, configuration, répertoire, etc. suivent désormais la langue de l'interface
- 🌐 **Internationalisation des titres de boîtes de dialogue** : les titres des boîtes de dialogue de sélection de dossier macOS / Linux suivent la langue de l'interface
- ♻️ **Découplage du code** : les modules ne contiennent plus de chaînes de texte en dur ; tous les textes sont lus via `self.t()` ou `langs::global_texts()`

### 🐞 Corrections de bugs

- 🛠️ **Correction du focus clavier en mode commentaires** : correction d'un problème où en mode recherche en ligne/Juhe/playlist, après avoir appuyé sur `c` pour voir les commentaires, les touches haut/bas contrôlaient la liste des chansons au lieu de la liste des commentaires
- 🛠️ **Correction du dialogue de sélection de dossier Linux** : correction d'un problème où appuyer sur `o` sous Linux n'affichait pas le dialogue graphique de sélection de dossier ; gestion correcte du conflit entre le mode raw et le dialogue graphique
- 🛠️ **Correction de sécurité du découpage UTF-8 des logs** : correction d'un crash possible du programme dû au découpage par octets des chaînes UTF-8 multi-octets ; passage à une troncature sécurisée par caractères
- 🛠️ **Correction du formatage du fichier de configuration** : correction d'un problème de double remplacement `replace("{}")` dans les messages d'erreur de configuration, empêchant le second placeholder d'être correctement remplacé
---

## 📝 Journal des modifications

## Version 1.5.0 (2026-04-30)

### 🎉 Nouvelles fonctionnalités

#### Recherche de listes de lecture en ligne
- ✨ **Entrée de recherche de listes de lecture** : appuyez sur `p` pour rechercher directement des listes de lecture en ligne
- ✨ **Navigation dans le contenu de la liste** : après être entré dans une liste, vous pouvez parcourir les chansons et les lire rapidement
- ✨ **Lecture depuis le cache** : lors de la recherche en ligne/Juhe/de listes de lecture, si la chanson existe déjà localement ou est en cache, le téléchargement est ignoré et la lecture est directe
- ✨ **Déduplication du téléchargement des paroles** : lors de la recherche en ligne/Juhe/de listes de lecture, si la chanson existe déjà localement ou est en cache, les fichiers de paroles ne sont pas téléchargés à nouveau

### 🔧 Améliorations

- 🎵 **Optimisation de la stratégie des paroles** : pendant la lecture, les paroles utilisent désormais « Juhe en priorité, repli classique » pour améliorer la précision de correspondance
- 🎯 **Optimisation du focus de recherche** : appuyer sur `s/n/j/p` focalise désormais le champ de recherche par défaut pour une saisie immédiate
- 🎯 **Optimisation de l'interaction recherche-vers-liste** : après avoir appuyé sur Entrée ou cliqué sur une chanson, le focus passe à la liste pour que les raccourcis clavier n'entrent plus dans le champ de recherche
- 🎯 **Cohérence du style des listes en ligne** : dans les vues de recherche en ligne/Juhe/listes de lecture, le curseur de sélection et le marqueur de lecture sont séparés et l'espacement est aligné avec le style de la liste locale
- 🎲 **Optimisation de la cohérence de la lecture aléatoire en ligne** : en mode aléatoire, les résultats de recherche en ligne et Juhe prennent en charge le comportement de passage automatique aléatoire cohérent avec la lecture de liste
- 🛡️ **Protection contre le passage automatique en ligne** : ajout d'une limitation de débit pour le saut automatique en ligne ; si 5 sauts automatiques consécutifs se produisent en 3 secondes, la lecture s'arrête automatiquement pour éviter un saut incontrôlé sur des pistes illisibles

### 🐞 Corrections de bugs

- 🛠️ **Correction de la priorité des paroles** : correction de l'ordre de priorité de téléchargement des paroles dans les flux de recherche en ligne/Juhe/listes de lecture
- 🛠️ **Correction de l'index de lecture automatique en ligne** : correction d'un problème où le déplacement du curseur pendant la lecture pouvait faire continuer le passage automatique depuis la position du curseur au lieu de la chanson en cours
- 🛠️ **Correction de l'entrée espace dans la recherche** : correction d'un problème où la barre d'espace était écrite dans le champ de recherche en état de focus liste et modifiait/effaçait les résultats de manière inattendue
- 🛠️ **Correction du focus initial de la recherche réseau** : correction de l'absence de focus initial lors de l'entrée dans la recherche réseau avec `n`
- 🛠️ **Correction du comportement manquant de la lecture aléatoire en ligne** : correction d'un problème où le mode aléatoire ne fonctionnait pas dans les listes de résultats de recherche en ligne/Juhe
- 🛠️ **Correction de l'arrêt prématuré de la lecture automatique en ligne** : correction d'un problème où la lecture pouvait s'arrêter trop tôt lorsque la première piste en ligne était illisible en ne comptant que les tentatives réelles de passage automatique et en réinitialisant la fenêtre après une lecture réussie

---

## Version 1.4.0 (2026-04-28)


### 🎉 Nouvelles fonctionnalités

#### Recherche Juhe comme solution de secours
- ✨ **Recherche Juhe de chansons** : lorsque la recherche en ligne échoue, vous pouvez utiliser la recherche Juhe pour chercher des chansons par titre/artiste et les télécharger
- ✨ **Recherche Juhe de paroles** : s'il n'y a pas de paroles locales et que la recherche en ligne échoue, le système recherchera automatiquement des paroles par titre/artiste via la recherche Juhe et les téléchargera
- ✨ **Expérience fluide** : la recherche et le téléchargement se font en arrière-plan sans bloquer l'interface

#### Configuration du jeton GitHub
- ✨ **Jeton GitHub personnalisé** : appuyez sur `g` pour saisir votre propre jeton GitHub, sauvegardé dans le fichier de configuration
- ✨ **Repli par défaut** : utilise automatiquement un jeton par défaut lorsqu'il n'est pas configuré
- ✨ **Reconnaissance d'identité** : lors de la soumission d'informations sur les chansons pour discussion avec votre propre jeton, votre identité GitHub sera affichée

### 🔧 Améliorations

- 🔍 **Nouvel élément de configuration** : `github_token` (Jeton GitHub, laisser vide pour utiliser la valeur par défaut)

---

## Version 1.3.0 (2026-04-26)

### 🎉 Nouvelles fonctionnalités

#### Point de terminaison API AI personnalisé
- ✨ **API compatible OpenAI** : prend en charge toute API compatible OpenAI pour les requêtes d'informations sur les chansons (DeepSeek, OpenRouter, OpenAI, etc.)
- ✨ **Configuration en 3 étapes** : appuyez sur `k` pour saisir successivement l'URL de base de l'API → la clé API → le nom du modèle
- ✨ **Repli gratuit** : utilise automatiquement le modèle gratuit d'OpenRouter (minimax/minimax-m2.5:free) lorsqu'aucune clé API n'est définie
- ✨ **Requête directe** : appuyez sur `i` pour interroger directement les informations sur la chanson — aucune pré-configuration de clé API requise

### 🔧 Améliorations

- 🔍 **Optimisation du prompt** : renommé « Signification de la chanson » → « Signification des paroles », « Anecdotes » → « Anecdotes »
- 🔍 **Champ de configuration renommé** : `deepseek_api_key` → `api_key` (compatible avec les fichiers de configuration existants)
- 🔍 **Nouveaux éléments de configuration** : `api_base_url` (point de terminaison API, DeepSeek par défaut), `api_model` (nom du modèle, deepseek-v4-flash par défaut)

---

## Version 1.2.0 (2026-04-24)

### 🎉 Nouvelles fonctionnalités

#### Requête d'informations sur la chanson
- ✨ **Requête DeepSeek** : appuyez sur `i` pour interroger en flux les informations détaillées de la chanson via DeepSeek
- ✨ **Sortie en flux** : les résultats s'affichent caractère par caractère, pas besoin d'attendre la génération complète
- ✨ **13 catégories d'informations** : interprètes, détails de l'artiste, écriture et production, date de sortie, album (avec liste des pistes), contexte créatif, signification de la chanson, style musical, performances commerciales, récompenses, impact et critiques, reprises et utilisations, anecdotes
- ✨ **Réponse multilingue** : la langue de la réponse suit la langue de l'interface (SC/TC/EN/JP/KR)
- ✨ **Gestion de la clé API** : appuyez sur `k` pour saisir la clé API DeepSeek, ou définissez-la via la variable d'environnement `DEEPSEEK_API_KEY`

#### Source Kugou Music
- ✨ **Kugou Music** : ajout de Kugou comme troisième plateforme de recherche/téléchargement
- ✨ **Recherche sur 3 plateformes** : ordre de priorité Kuwo → Kugou → NetEase
- ✨ **Réduction des restrictions VIP** : Kugou fournit plus de ressources de téléchargement gratuites
- ✨ **Authentification par signature MD5** : les liens de téléchargement Kugou utilisent une signature MD5 pour un meilleur taux de réussite

### 🔧 Améliorations

#### Optimisation du prompt d'informations sur la chanson
- 🔍 **Pas de préambule** : les réponses n'incluent plus de salutations ou de présentations
- 🔍 **Pas de listes numérotées** : le contenu de sortie n'utilise plus le format de liste numérotée
- 🔍 **Détails de l'artiste** : nouvelle catégorie avec des informations détaillées sur l'artiste (nationalité, lieu de naissance, date de naissance, etc.)
- 🔍 **Liste des pistes de l'album** : la section album inclut désormais la liste complète des pistes

### 💻 Détails techniques

#### Mises à jour des dépendances
- ➕ Ajout de la dépendance `md5` (signature de l'API Kugou Music)

#### Structures de données
- ♻️ Ajout du champ `hash` à `OnlineSong` (Kugou utilise le hash pour identifier les chansons)
- ♻️ Ajout de la variante d'énumération `MusicSource::Kugou`
- ♻️ Ajout des structures d'analyse JSON Kugou

---

## Version 1.1.0 (2026-04-17)

### 🎉 Nouvelles fonctionnalités

#### Système d'affichage des paroles
- ✨ **Disposition à deux panneaux** : liste de chansons à gauche, paroles à droite
- ✨ **Téléchargement automatique des paroles** : téléchargement depuis le réseau lorsque les paroles sont manquantes
- ✨ **Correspondance intelligente** : recherche automatique des noms de fichiers de paroles marqués
- ✨ **Prise en charge multi-encodage** : prend en charge les fichiers de paroles UTF-8 et GBK
- ✨ **Défilement des paroles** : défilement automatique avec la progression de la lecture
- ✨ **Surbrillance** : la ligne de paroles en cours est mise en surbrillance en jaune
- ✨ **Affichage du titre de la chanson** : le titre des paroles affiche le nom de la chanson en cours

#### Expérience utilisateur
- ✨ **Correspondance/téléchargement automatique des paroles** pendant la lecture
- ✨ **Style unifié** : la liste de lecture et la zone des paroles utilisent un style jaune cohérent
- ✨ **Titre dynamique** : le titre des paroles se met à jour avec la chanson en cours
- ✨ Prise en charge du **changement de langue**
- ✨ Prise en charge du **changement de thème**

### 🚀 Optimisation des performances

#### Rendu de l'interface
- ⚡ **Mises à jour plus fluides de la barre de progression**
- ⚡ **Réduction des redessins** en optimisant la boucle d'événements
- ⚡ **Optimisation des verrous** pour améliorer la réactivité

#### Chargement des paroles
- ⚡ **Cache intelligent** après chargement pour éviter l'analyse répétée
- ⚡ **Chargement paresseux** uniquement lorsque c'est nécessaire
- ⚡ **Prise en charge du renommage par lot** pour nettoyer les marqueurs de noms de fichiers de paroles

### 🎨 Améliorations de l'interface

#### Mises à jour visuelles
- 🎨 **Palette de couleurs unifiée** dans la liste de lecture et la zone des paroles
- 🎨 **Disposition séparée** pour une meilleure utilisation de l'espace
- 🎨 **Ligne de séparation centrale** pour une structure visuelle plus claire

#### Affichage des informations
- 📊 **Affichage de la plage visible** de la liste de lecture
- 📊 **Nom de la chanson dans le titre des paroles**
- 📊 **Mises à jour plus fréquentes de la barre de progression**

### 🔧 Améliorations fonctionnelles

#### Gestion des paroles
- 🔍 **Recherche intelligente** de plusieurs modèles de noms de fichiers de paroles
- 🔍 **Mappage des fichiers** garantissant une correspondance bijective chanson-paroles

#### Gestion des erreurs
- 🛡️ **Messages conviviaux** en cas d'échec de téléchargement
- 🛡️ **Détection automatique de l'encodage** des fichiers de paroles
- 🛡️ **Délai d'attente réseau de 10 secondes** pour éviter les attentes bloquantes prolongées

### 🐛 Corrections de bugs

- 🐛 Correction de l'inadéquation des paroles causée par les marqueurs de noms de fichiers
- 🐛 Correction des problèmes d'encodage dans le téléchargement des paroles
- 🐛 Correction du scintillement de l'interface lors du redessin
- 🐛 Correction du retard des mises à jour de la barre de progression

### 💻 Détails techniques

#### Mises à jour des dépendances
- ➕ Ajout du client HTTP `reqwest`
- ➕ Ajout de la prise en charge `urlencoding`
- ➕ Ajout de la prise en charge de transcodage `encoding_rs`

#### Refactorisation
- ♻️ Optimisation de la logique de la boucle d'événements
- ♻️ Amélioration du flux de chargement des paroles
- ♻️ Définitions unifiées des constantes de couleur

---

## Version 1.0.0 (2026-04-09)

### Fonctionnalités principales
- 🎵 Lecture audio (multi-format)
- 📋 Gestion de la liste de lecture
- 🎹 Contrôles de lecture
- 🔊 Contrôle du volume
- 🎲 Changement de mode de lecture
- 📂 Navigation dans les dossiers

---

## 📄 Assistance IA

GLM, Codex

## 📄 Licence

MIT License

## 🤝 Contribution

Les Issues et Pull Requests sont les bienvenues !
