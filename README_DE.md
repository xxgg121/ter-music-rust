<div align="center">

[简体中文](README.md) | [繁體中文](README_TC.md) | [English](README_EN.md) | [日本語](README_JA.md) | [한국어](README_KO.md) | [Русский](README_RU.md) | [Français](README_FR.md) | [Deutsch](README_DE.md) | [Español](README_ES.md) | [Italiano](README_IT.md) | [Português](README_PT.md)

# 🎵 Ter-Music-Rust - Terminal-Musikplayer 🎵

</div>

Ein schlanker und praktischer Terminal-Musikplayer in Rust mit lokaler/online Suche und Download von Songs, automatischem Herunterladen und Anzeigen von Liedtexten, Kommentaransicht sowie Sprach- und Theme-Umschaltung. Unterstützt Windows, Linux und MacOS.

![preview1](preview1.png)

![preview2](preview2.png)

![preview3](preview3.png)

![preview4](preview4.png)

![preview5](preview5.png)

![preview6](preview6.png)

## ✨ Funktionen

### 🎵 Audiowiedergabe
- **10 Audioformate unterstützt**: MP3, WAV, FLAC, OGG, OGA, Opus, M4A, AAC, AIFF, APE
- **Wiedergabesteuerung**: Wiedergabe/Pause/Stopp, vorheriger/nächster Titel
- **Vorspulen**: Schnelles Vorspulen um 5s / 10s
- **Fortschrittsbalken-Vorspulen**: Klick auf den Fortschrittsbalken für präzises Springen
- **Lautstärkeregelung**: Echtzeit-Einstellung von 0-100, Klick auf die Lautstärkeleiste zum Setzen

### 🔄 5 Wiedergabemodi
| Taste | Modus | Beschreibung |
|------|------|------|
| `1` | Einzeltitelwiedergabe | Nach Abschluss des aktuellen Titels stoppen |
| `2` | Einzeltitelwiederholung | Aktuellen Titel wiederholen |
| `3` | Fortlaufende Wiedergabe | In Reihenfolge abspielen, am Ende stoppen |
| `4` | Listenwiederholung | Gesamte Wiedergabeliste wiederholen |
| `5` | Zufallswiedergabe | Zufällige Titelauswahl |

### 📜 Liedtext-System
- **Lokales Laden von Liedtexten**: Automatisches Finden passender `.lrc`-Dateien
- **Liedtext-Kodierungserkennung**: Automatische Erkennung von UTF-8 / GBK
- **Automatischer Online-Download**: Asynchroner Hintergrund-Download bei fehlenden lokalen Liedtexten
- **Scrollende Hervorhebung**: Aktuelle Zeile wird mit `>` hervorgehoben, automatisches zentriertes Scrollen
- **Liedtext-Positionssprung**: Ziehen des Liedtextbereichs oder Mausrad zum Springen nach Liedtext-Zeitstempel

### 🔍 Suche
- **Lokale Suche**: `s` drücken, um Songs im aktuellen Musikverzeichnis zu suchen
- **Online-Suche**: `n` drücken, um Online-Songs nach Schlüsselwort zu suchen
- **Juhe-Suche**: `j` drücken zum Eingang. Suche nach Juhe-Songs basierend auf Schlüsselwortübereinstimmung.
- **Wiedergabelisten-Suche**: `p` drücken zum Eingang. Suche nach Online-Wiedergabelisten basierend auf Schlüsselwortübereinstimmung.
- **Seitenwechsel**: `PgUp` / `PgDn` für weitere Ergebnisse
- **Online-Download**: `Enter` auf ausgewähltem Online-Ergebnis drücken, um in das aktuelle Musikverzeichnis herunterzuladen (mit Fortschrittsanzeige)

### 🤖 Song-Informationen
- **Intelligente Abfrage**: `i` drücken, um detaillierte Song-Informationen abzufragen, unterstützt jede OpenAI-kompatible API
- **Streaming-Ausgabe**: Ergebnisse werden Zeichen für Zeichen angezeigt, kein Warten auf vollständige Generierung erforderlich
- **Umfangreiche Informationen**: Abdeckt 13 Kategorien einschließlich Künstlerdetails, Songwriting, Album-Titelliste, kreativer Hintergrund, Liedtextbedeutung, Musikstil, Anekdoten und mehr
- **Mehrsprachige Unterstützung**: Antwortsprache folgt der Einstellung der Benutzeroberflächensprache (SC/TC/EN/JP/KR)
- **Benutzerdefinierte API**: `k` drücken, um API-Basis-URL, API-Schlüssel und Modellnamen in 3 Schritten zu konfigurieren — unterstützt DeepSeek, OpenRouter, AIHubMix und jeden OpenAI-kompatiblen Endpunkt
- **Kostenloser Fallback**: Verwendet automatisch das kostenlose Modell von OpenRouter (minimax/minimax-m2.5:free), wenn kein API-Schlüssel konfiguriert ist

### ⭐ Favoriten
- **Favoriten hinzufügen/entfernen**: `f` drücken, um den Favoritenstatus des aktuellen Titels umzuschalten
- **Favoritenliste**: `v` drücken, um Favoriten anzuzeigen (mit `*`-Markierung)
- **Verzeichnisübergreifende Wiedergabe**: Automatischer Verzeichniswechsel, wenn sich ein Favorit außerhalb des aktuellen Verzeichnisses befindet
- **Favorit löschen**: `d` in der Favoritenliste drücken

### 💬 Kommentare
- **Song-Kommentare**: `c` drücken, um Kommentare zum aktuellen Song anzuzeigen
- **Kommentardetails**: `Enter` drücken, um zwischen Listen-/Detailansicht zu wechseln (Volltext in Detailansicht)
- **Antwortanzeige**: Zeigt den Originaltext des beantworteten Kommentars, den Spitznamen und die Zeit an
- **Kommentar-Seitenwechsel**: `PgUp` / `PgDn`, 20 Kommentare pro Seite
- **Hintergrundladen**: Kommentare werden in Hintergrund-Threads abgerufen, ohne die Benutzeroberfläche zu blockieren

### 📂 Verzeichnisverwaltung
- **Musikverzeichnis wählen**: `o` drücken, um den Ordnerauswahl-Dialog zu öffnen (Wiedergabe startet automatisch nach erster erfolgreicher Öffnung)
- **Verzeichnisverlauf öffnen**: `m` drücken, um Verzeichnisse anzuzeigen und schnell zu wechseln
- **Aktuelles Verzeichnis-Marker**: `>>` zeigt das aktuell aktive Verzeichnis an
- **Verlaufseintrag löschen**: `d` in der Verlaufsansicht drücken

### 🌐 Mehrsprachige Benutzeroberfläche
Unterstützt 11 UI-Sprachen (Wechsel mit `l`):

| Sprache | Konfigurationswert |
|------|--------|
| Vereinfachtes Chinesisch | `sc` |
| Traditionelles Chinesisch | `tc` |
| Englisch | `en` |
| Japanisch | `ja` |
| Koreanisch | `ko` |
| Russisch | `ru` |
| Französisch | `fr` |
| Deutsch | `de` |
| Spanisch | `es` |
| Italienisch | `it` |
| Portugiesisch | `pt` |

### 🎨 Mehrthema-Benutzeroberfläche
Unterstützt 4 Themen (Wechsel mit `t`):

| Thema | Stil |
|------|------|
| Neon | Neontöne |
| Sunset | Warmes Sonnenuntergangsgold |
| Ocean | Tiefes Ozeanblau |
| GrayWhite | Konsolenartige Graustufen |

### 🖱️ Maus-Interaktion
- **Wiedergabelisten-Klick**: Klick zum direkten Abspielen eines Songs
- **Fortschrittsbalken-Klick**: Zu einer bestimmten Position springen
- **Lautstärkeleisten-Klick**: Lautstärke anpassen
- **Liedtext-Ziehen**: Linksziehen zum Springen nach Liedtext-Zeitstempel
- **Liedtext-Mausrad**: Hoch/Runter scrollen zur vorherigen/nächsten Liedtextzeile springen
- **Suchergebnis-Klick**: Lokale Suche — Klick zum Abspielen, Online-Suche — Klick zum Herunterladen
- **Kommentar-Klick**: Klick zum Öffnen der Details

### 📊 Wellenform-Visualisierung
- Dynamische Wellenformbalken basierend auf dem tatsächlichen RMS-Volumen während der Wiedergabe
- EMA-Glättung für weichere Darstellung
- Wellenform friert bei Pause ein

### ⚙️ Persistente Konfiguration
Die Konfiguration wird in `USERPROFILE/ter-music-rust/config.json` im Programmverzeichnis gespeichert und automatisch gespeichert/wiederhergestellt:

| Konfigurationselement | Beschreibung |
|--------|------|
| `music_directory` | Zuletzt geöffnetes Musikverzeichnis |
| `play_mode` | Wiedergabemodus |
| `current_index` | Index des zuletzt abgespielten Songs (Wiedergabe fortsetzen) |
| `volume` | Lautstärke (0-100) |
| `favorites` | Favoritenliste |
| `dir_history` | Verzeichnisverlauf |
| `api_key` | API-Schlüssel (für Song-Info-Abfrage, abwärtskompatibel mit `deepseek_api_key`) |
| `api_base_url` | API-Basis-URL (Standard: `https://api.deepseek.com/`) |
| `api_model` | AI-Modellname (Standard: `deepseek-v4-flash`) |
| `github_token` | GitHub-Token (verwendet für Song-Info-Diskussionen; leer lassen für Standard-Token) |
| `theme` | Themenname |
| `language` | UI-Sprache (`sc` / `tc` / `en` / `ja` / `ko` / `ru` / `fr` / `de` / `es` / `it` / `pt`) |

**Auto-Speichern-Auslöser**: Titelwechsel, Themawechsel, Sprachwechsel, Favoritenänderung, alle 30 Sekunden und beim Beenden (einschließlich Strg+C)

---

## 🚀 Schnellstart

### 1. Rust installieren

```powershell
# Methode 1: winget (empfohlen)
winget install Rustlang.Rustup

# Methode 2: Offizieller Installer
# https://rustup.rs/ besuchen und installieren
```

Installation überprüfen:

```powershell
rustc --version
cargo --version
```

### 2. Projekt erstellen

```powershell
cd <Projektverzeichnis>

# Methode 1: Build-Skript (empfohlen)
build-win.bat

# Methode 2: Cargo
cargo build --release
```

### 3. Ausführen

```powershell
# Methode 1: cargo run
cargo run --release

# Methode 2: Ausführbare Datei direkt starten
.\target\release\ter-music-rust.exe

# Methode 3: Musikverzeichnis angeben
.\target\release\ter-music-rust.exe -o d:\Music
cargo run --release -- -o d:\Music
```

**Verzeichnis-Ladepriorität**: Kommandozeile `-o` > Konfigurationsdatei > Ordnerauswahl-Dialog

---

## 🎮 Tastenkombinationen

### Hauptansicht

| Taste | Aktion |
|------|------|
| `↑/↓` | Song auswählen |
| `Enter` | Ausgewählten Song abspielen |
| `Leertaste` | Wiedergabe/Pause |
| `Esc` | Wiedergabe stoppen (in Kommentaransicht: zurück zu Liedtexten) |
| `←/→` | Vorheriger/Nächster Song |
| `[` | 5s zurückspulen |
| `]` | 5s vorspulen |
| `,` | 10s zurückspulen |
| `.` | 10s vorspulen |
| `+/-` | Lautstärke hoch/runter (Schritt 5) |
| `1-5` | Wiedergabemodus wechseln |
| `o` | Musikverzeichnis öffnen |
| `s` | Lokale Songs suchen |
| `n` | Online-Songs suchen |
| `j` | Juhe-Songs suchen |
| `p` | Online-Wiedergabelisten suchen |
| `i` | Song-Info abfragen |
| `f` | Favorit hinzufügen/entfernen |
| `v` | Favoriten anzeigen |
| `m` | Verzeichnisverlauf anzeigen |
| `h` | Hilfe anzeigen |
| `c` | Song-Kommentare anzeigen |
| `l` | UI-Sprache wechseln |
| `t` | Thema wechseln |
| `k` | API-Endpunkt konfigurieren |
| `g` | GitHub-Token konfigurieren |
| `q` | Beenden |

### Suchansicht

| Taste | Aktion |
|------|------|
| Zeicheneingabe | Suchbegriff eingeben |
| `Rücktaste` | Zeichen löschen |
| `Enter` | Suchen/Abspielen/Herunterladen |
| `↑/↓` | Ergebnis auswählen |
| `PgUp/PgDn` | Seite hoch/runter (Online-Suche) |
| `s/n/j` | Lokale/Online/Juhe-Suche wechseln |

| `Esc` | Suche beenden |

### Favoritenansicht

| Taste | Aktion |
|------|------|
| `↑/↓` | Song auswählen |
| `Enter` | Ausgewählten Song abspielen |
| `d` | Favorit löschen |
| `Esc` | Zurück zur Wiedergabeliste |

### Verzeichnisverlaufsansicht

| Taste | Aktion |
|------|------|
| `↑/↓` | Verzeichnis auswählen |
| `Enter` | Zum ausgewählten Verzeichnis wechseln |
| `d` | Eintrag löschen |
| `Esc` | Zurück zur Wiedergabeliste |

### Kommentaransicht

| Taste | Aktion |
|------|------|
| `↑/↓` | Kommentar auswählen |
| `Enter` | Listen-/Detailansicht wechseln |
| `PgUp/PgDn` | Seite hoch/runter |
| `Esc` | Zurück zur Liedtextansicht |

### Song-Info-Ansicht

| Taste | Aktion |
|------|------|
| `↑/↓` | Song-Info scrollen |
| `i` | Song-Info erneut abfragen |
| `Esc` | Zurück zur Liedtextansicht |

### Wiedergabelisten-Suchansicht

| Taste | Aktion |
|------|------|
| Zeicheneingabe | Wiedergabelisten-Schlüsselwort eingeben |
| `Rücktaste` | Zeichen löschen |
| `Enter` | Suchen/Wiedergabeliste betreten/Abspielen & Herunterladen |
| `↑/↓` | Wiedergabeliste oder Song auswählen |
| `PgUp/PgDn` | Seite hoch/runter |
| `Esc` | Zurück zur vorherigen Ebene / Suche beenden |

### Hilfeansicht


| Taste | Aktion |
|------|------|
| `↑/↓` | Hilfeinhalt scrollen |
| `Esc` | Zurück zur Liedtextansicht |

---

## 📦 Installation & Build

### Systemanforderungen

- **Betriebssystem**: Windows 10/11
- **Rust**: 1.70+
- **Terminal**: Windows Terminal (empfohlen) / CMD / PowerShell
- **Fenstergröße**: 80×25 oder größer empfohlen

### Option 1: MSVC-Toolchain (beste Kompatibilität, größere Dateigröße)

```powershell
# 1. Rust installieren
winget install Rustlang.Rustup

# 2. Build-Tools installieren
winget install Microsoft.VisualStudio.2022.BuildTools
# Installer ausführen -> „Desktop-Entwicklung mit C++" auswählen -> installieren

# 3. Terminal neu starten und erstellen
cargo build --release
```

### Option 2: GNU-Toolchain (empfohlen, leichtgewichtig ~300 MB)

```powershell
# 1. Rust installieren
winget install Rustlang.Rustup

# 2. MSYS2 installieren
winget install MSYS2.MSYS2
# Im MSYS2-Terminal ausführen:
pacman -Syu
pacman -S mingw-w64-x86_64-toolchain

# 3. PATH hinzufügen (PowerShell als Administrator)
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\msys64\mingw64\bin", "Machine")

# 4. Toolchain wechseln und erstellen
rustup default stable-x86_64-pc-windows-gnu
cargo build --release
```

> Programme, die mit der GNU-Toolchain erstellt wurden, benötigen möglicherweise diese DLLs im Verzeichnis der ausführbaren Datei:
> `libgcc_s_seh-1.dll`, `libstdc++-6.dll`, `libwinpthread-1.dll`

### Option 3: Cross-Compiling für Linux auf Windows

Verwenden Sie `cargo-zigbuild` + `zig` als Linker. Keine Linux-VM/Installation erforderlich.

```powershell
# 1. Zig installieren (eine Option wählen)
# A: via pip (empfohlen)
pip install ziglang

# B: via MSYS2
pacman -S mingw-w64-x86_64-zig

# C: Manueller Download
# https://ziglang.org/download/ besuchen, entpacken und zum PATH hinzufügen

# 2. cargo-zigbuild installieren
cargo install cargo-zigbuild

# 3. Linux-Target hinzufügen
rustup target add x86_64-unknown-linux-gnu

# 4. Linux-Sysroot vorbereiten (ALSA-Header/Bibliotheken)
# Das Projekt enthält bereits linux-sysroot/
# Für manuelle Vorbereitung von Debian/Ubuntu kopieren:
#   /usr/include/alsa/ -> linux-sysroot/usr/include/alsa/
#   /usr/lib/x86_64-linux-gnu/libasound.so* -> linux-sysroot/usr/lib/x86_64-linux-gnu/

# 5. Erstellen
build-linux.bat

# Oder manuell ausführen:
cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.34
```

**Ausgabe**: `target/x86_64-unknown-linux-gnu/release/ter-music-rust`

**Auf Linux bereitstellen**:

```bash
# 1. Auf Linux-Host kopieren
scp ter-music-rust user@linux-host:~/

# 2. Ausführbar machen
chmod +x ter-music-rust

# 3. ALSA-Runtime installieren
sudo apt install libasound2

# 4. Ausführen
./ter-music-rust -o /path/to/music
```

> `build-linux.bat` konfiguriert automatisch `PKG_CONFIG_PATH`, `PKG_CONFIG_ALLOW_CROSS`, `RUSTFLAGS` usw.
> Im Target `x86_64-unknown-linux-gnu.2.34` gibt `.2.34` die minimale glibc-Version für bessere Kompatibilität mit älteren Linux-Systemen an.

### Linux-Verpackung (DEB / RPM)

Wenn Sie auf Linux erstellen/verpacken, verwenden Sie:

```bash
# 1) RPM
./build-rpm.sh

# Debuginfo-RPM generieren (optional)
./build-rpm.sh --with-debuginfo

# 2) DEB
./build-deb.sh

# DEB mit Debug-Symbolen generieren (optional)
./build-deb.sh --with-debuginfo

# Quellpaket kompatibel mit dpkg-source generieren (.dsc/.orig.tar/.debian.tar)
./build-deb.sh --with-source

# Debuginfo + Quellpaket generieren
./build-deb.sh --with-debuginfo --with-source
```

Standard-Ausgabeverzeichnisse:
- `dist/rpm/`: RPM / SRPM
- `dist/deb/`: DEB / Quellpakete

> Skripte lesen `name` und `version` aus `Cargo.toml`, um Paketdateien automatisch zu benennen.

### Option 4: Cross-Compiling für MacOS auf Windows

Verwenden Sie `cargo-zigbuild` + `zig` + MacOS SDK. Audio auf MacOS verwendet CoreAudio und erfordert SDK-Header.

**Voraussetzungen:**

```powershell
# 1. Zig installieren (wie bei Linux-Cross-Compiling)
pip install ziglang

# 2. cargo-zigbuild installieren
cargo install cargo-zigbuild

# 3. LLVM/Clang installieren (stellt libclang.dll für bindgen bereit)
# A: via MSYS2
pacman -S mingw-w64-x86_64-clang

# B: Offizielles LLVM
winget install LLVM.LLVM

# 4. MacOS-Targets hinzufügen
rustup target add x86_64-apple-darwin aarch64-apple-darwin
```

**MacOS SDK vorbereiten:**

Entpacken Sie `MacOSX13.3.sdk.tar.xz` in `macos-sysroot`.
Das Projekt enthält bereits `macos-sysroot/` (heruntergeladen von [macosx-sdks](https://github.com/joseluisq/macosx-sdks)).

Erneut herunterladen:

```powershell
# A: Vorgepacktes SDK von GitHub herunterladen (empfohlen, ~56 MB)
# Spiegel: https://ghfast.top/https://github.com/joseluisq/macosx-sdks/releases/download/13.3/MacOSX13.3.sdk.tar.xz
curl -L -o MacOSX13.3.sdk.tar.xz https://github.com/joseluisq/macosx-sdks/releases/download/13.3/MacOSX13.3.sdk.tar.xz
mkdir macos-sysroot
tar -xf MacOSX13.3.sdk.tar.xz -C macos-sysroot --strip-components=1
del MacOSX13.3.sdk.tar.xz

# B: Von einem MacOS-System kopieren
scp -r mac:/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk ./macos-sysroot
```

> SDK-Quelle: https://github.com/joseluisq/macosx-sdks
> Enthält Header für CoreAudio, AudioToolbox, AudioUnit, CoreMIDI, OpenAL, IOKit usw.

**Erstellen:**

```powershell
# Build-Skript verwenden (setzt automatisch alle Umgebungsvariablen)
build-mac.bat

# Oder manuell:
$env:LIBCLANG_PATH = "C:\msys64\mingw64\bin"      # Verzeichnis mit libclang.dll
$env:COREAUDIO_SDK_PATH = "./macos-sysroot"         # MacOS-SDK-Pfad (Schrägstriche)
$env:SDKROOT = "./macos-sysroot"                    # Vom Zig-Linker zum Auffinden von Systembibliotheken benötigt
$FW = "./macos-sysroot/System/Library/Frameworks"
$env:BINDGEN_EXTRA_CLANG_ARGS = "--target=x86_64-apple-darwin -isysroot ./macos-sysroot -F $FW -iframework $FW -I ./macos-sysroot/usr/include"
cargo zigbuild --release --target x86_64-apple-darwin   # Intel Mac
# Für Apple Silicon x86_64 durch aarch64 ersetzen
cargo zigbuild --release --target aarch64-apple-darwin  # Apple Silicon
```

**Ausgaben:**
- `target/x86_64-apple-darwin/release/ter-music-rust` — Intel Mac
- `target/aarch64-apple-darwin/release/ter-music-rust` — Apple Silicon (M1/M2/M3/M4)

**Auf MacOS bereitstellen**:

```bash
# 1. Auf MacOS-Host kopieren
scp ter-music-rust user@mac-host:~/

# 2. Ausführbar machen
chmod +x ter-music-rust

# 3. Ausführung unbekannter Quellen erlauben
xattr -cr ter-music-rust

# 4. Ausführen (keine zusätzlichen Audiobibliotheken erforderlich)
./ter-music-rust -o /path/to/music
```

> Hinweis: MacOS-Cross-Compiling erfordert MacOS-SDK-Header; dieses Projekt enthält bereits `macos-sysroot/`.
> Außerdem wird `libclang.dll` benötigt (über MSYS2 oder LLVM installieren).

### Toolchain wechseln

```powershell
# Aktuelle Toolchain anzeigen
rustup show

# Zu MSVC wechseln
rustup default stable-x86_64-pc-windows-msvc

# Zu GNU wechseln
rustup default stable-x86_64-pc-windows-gnu
```

### Cargo-Spiegel in China (schnellere Downloads)

Erstellen oder bearbeiten Sie `~/.cargo/config`:

```toml
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
```

---

## 🛠️ Projektstruktur

```text
src/
├── main.rs       # Programmeinstieg (Argumentanalyse, Initialisierung, Konfiguration wiederherstellen/speichern)
├── defs.rs       # Gemeinsame Definitionen (PlayMode/PlayState-Enums, MusicFile/Playlist-Strukturen)
├── audio.rs      # Audiosteuerung (rodio-Wrapper, Wiedergabe/Pause/Springen/Lautstärke/Fortschritt)
├── analyzer.rs   # Audio-Analysator (Echtzeit-RMS-Volumen, EMA-Glättung, Wellenform-Rendering)
├── playlist.rs   # Wiedergabelistenverwaltung (Verzeichnisscan, parallele Dauerladung, Ordnerauswahl)
├── lyrics.rs     # Liedtext-Analyse (LRC, lokale Suche, Kodierungserkennung, Hintergrund-Download)
├── search.rs     # Online-Suche/Download (Kuwo + Kugou + NetEase-Suche, Download, Kommentarabruf, Song-Info-Streaming-Abfrage)
├── config.rs     # Konfigurationsverwaltung (JSON-Serialisierung, 8 persistente Elemente)
└── ui.rs         # UI (Terminal-Rendering, Ereignisbehandlung, Multi-View-Modus, Theme/Sprach-System)
```

### Technologie-Stack

| Abhängigkeit | Version | Zweck |
|------|------|------|
| [rodio](https://github.com/RustAudio/rodio) | 0.19 | Audio-Dekodierung und Wiedergabe (reines Rust) |
| [crossterm](https://github.com/crossterm-rs/crossterm) | 0.28 | Terminal-UI-Steuerung |
| [reqwest](https://github.com/seanmonstar/reqwest) | 0.12 | HTTP-Anfragen |
| [serde](https://github.com/serde-rs/serde) + serde_json | 1.0 | JSON-Serialisierung |
| [rayon](https://github.com/rayon-rs/rayon) | 1.10 | Paralleles Laden der Audiodauer |
| [encoding_rs](https://github.com/hsivonen/encoding_rs) | 0.8 | GBK-Liedtext-Dekodierung |
| [walkdir](https://github.com/BurntSushi/walkdir) | 2.5 | Rekursive Verzeichnisscan |
| [rand](https://github.com/rust-random/rand) | 0.8 | Zufallswiedergabemodus |
| [unicode-width](https://github.com/unicode-rs/unicode-width) | 0.2 | CJK-Anzeigebreitenberechnung |
| [chrono](https://github.com/chronotope/chrono) | 0.4 | Kommentarzeitformatierung |
| [ctrlc](https://github.com/Detegr/rust-ctrlc) | 3.4 | Strg+C-Signalverarbeitung |
| [md5](https://github.com/johannhof/md5) | 0.7 | Kugou-Music-API-MD5-Signatur |
| [winapi](https://github.com/retep998/winapi-rs) | 0.3 | Windows-Konsole UTF-8-Unterstützung |

### Release-Build-Optimierung

```toml
[profile.release]
opt-level = 3       # höchste Optimierungsstufe
lto = true          # Link-Time-Optimierung
codegen-units = 1   # einzelne Codegen-Einheit für bessere Optimierung
strip = true        # Debug-Symbole entfernen
```

---

## Vergleich zwischen Rust und C-Version

| Eigenschaft | Rust-Version | C-Version |
|------|-----------|--------|
| Installationsgröße | ~200 MB (Rust) / ~300 MB (GNU) | ~7 GB (Visual Studio) |
| Einrichtungszeit | ~5 Minuten | ~1 Stunde |
| Kompiliergeschwindigkeit | ⚡ Schnell | 🐢 Langsamer |
| Abhängigkeitsverwaltung | ✅ Automatisch via Cargo | ❌ Manuelle Einrichtung |
| Speichersicherheit | ✅ Kompilierzeit-Garantien | ⚠️ Manuelle Verwaltung erforderlich |
| Cross-Plattform | ✅ Vollständig cross-plattform | ⚠️ Code-Änderungen erforderlich |
| Größe der ausführbaren Datei | ~2 MB | ~500 KB |
| Speicherverbrauch | ~15-20 MB | ~10 MB |
| CPU-Auslastung | < 1% | < 1% |

---

## 📊 Leistung

| Metrik | Wert |
|------|------|
| UI-Aktualisierungsintervall | 50ms |
| Tastenreaktion | < 50ms |
| Liedtext-Download | Hintergrund, nicht blockierend |
| Verzeichnisscan | Paralleles Dauerladen, 2-4x Beschleunigung |
| Startzeit | < 100ms |
| Speicherverbrauch | ~15-20 MB |

---

## 🐛 Fehlerbehebung

### Build-Fehler

```powershell
# Rust aktualisieren
rustup update

# Bereinigen und neu erstellen
cargo clean
cargo build --release
```

### `link.exe nicht gefunden`

Visual Studio Build Tools installieren (siehe Option 1 oben).

### `dlltool.exe nicht gefunden`

Vollständige MinGW-w64-Toolchain installieren (siehe Option 2 oben).

### Fehlende Runtime-DLLs (GNU-Toolchain)

```powershell
Copy-Item "C:\msys64\mingw64\bin\libgcc_s_seh-1.dll" -Destination ".\target\release\"
Copy-Item "C:\msys64\mingw64\bin\libstdc++-6.dll" -Destination ".\target\release\"
Copy-Item "C:\msys64\mingw64\bin\libwinpthread-1.dll" -Destination ".\target\release\"
```

### Kein Audiogerät gefunden

1. Stellen Sie sicher, dass das System-Audiogerät funktioniert
2. Überprüfen Sie die Windows-Lautstärkeeinstellungen
3. Versuchen Sie, einen Systemtestton abzuspielen

### UI-Rendering-Probleme

- Stellen Sie sicher, dass die Terminalfenstergröße mindestens 80×25 beträgt
- Verwenden Sie Windows Terminal für das beste Erlebnis
- Stellen Sie in CMD sicher, dass die ausgewählte Schriftart bei Bedarf CJK unterstützt

### Online-Suche / Liedtext-Download schlägt fehl

- Überprüfen Sie Ihre Netzwerkverbindung
- Einige Songs benötigen möglicherweise VIP-Zugang oder wurden entfernt
- Liedtextdatei muss im gültigen Standard-LRC-Format vorliegen

### Song-Info-Abfrage schlägt fehl

- Wenn kein API-Schlüssel konfiguriert ist, wird automatisch das kostenlose Modell von OpenRouter verwendet — keine manuelle Einrichtung erforderlich
- Um einen benutzerdefinierten Endpunkt zu verwenden, drücken Sie `k` und geben Sie nacheinander API-Basis-URL, API-Schlüssel und Modellnamen ein
- Unterstützt jede OpenAI-kompatible API (DeepSeek, OpenRouter, AIHubMix usw.)
- Überprüfen Sie die Netzwerkverbindung zum entsprechenden API-Dienst

### Langsamer erster Build

Der erste Build lädt und kompiliert alle Abhängigkeiten herunter; dies ist erwartet. Spätere Builds sind deutlich schneller.

### Downloads
[ter-music-rust-win.zip](https://storage.deepin.org/thread/202605050312131911_ter-music-rust-win.zip "附件(Attached)") 
[ter-music-rust-mac.zip](https://storage.deepin.org/thread/202605050312183967_ter-music-rust-mac.zip "附件(Attached)") 
[ter-music-rust-linux.zip](https://storage.deepin.org/thread/202605050312251425_ter-music-rust-linux.zip "附件(Attached)") 
[ter-music-rust_deb.zip](https://storage.deepin.org/thread/202605050312355690_ter-music-rust_deb.zip "附件(Attached)")

---

## Version 1.7.0 (2026-05-05)

### 🐞 Fehlerbehebungen

- 🛠️ **Unvollständige Oberfläche beim ersten Start auf Linux**: Problem behoben, bei dem die Oberfläche beim ersten Programmstart auf Linux in die obere linke Ecke des Terminals gequetscht war und ein Klick erforderlich war, um sie vollständig anzuzeigen. 50ms Wartezeit nach dem Wechsel zum Alternate Screen hinzugefügt, Terminalgröße erneut abgefragt und Bildschirm gelöscht
- 🛠️ **Fehlender Hinweis bei leerer Wiedergabeliste**: Problem behoben, bei dem die Wiedergabeliste beim ersten Start ohne ausgewähltes Musikverzeichnis ohne Hinweis leer war. Hinweis „o drücken, um Musikverzeichnis auszuwählen" hinzugefügt (gleicher Stil wie Hinweis im Liedtextbereich)
- 🛠️ **Überlauf des blauen Hintergrunds der ausgewählten Zeile**: Problem behoben, bei dem die blaue Hintergrundhervorhebung der ausgewählten Zeile über die Grenze des linken Panels in den Liedtextbereich rechts hinausging. `Clear(UntilNewLine)` durch exakte Breiten-Leerraumfüllung ersetzt
- 🛠️ **Verbleibende vorherige Liedtexte im Liedtextbereich**: Problem behoben, bei dem beim Wechsel zu einem Song ohne Liedtexte die Liedtexte des vorherigen Songs im rechten Bereich sichtbar blieben. Alle Zeilen vor dem Zeichnen gelöscht
- 🛠️ **Kein Neuzeichnen bei Fenstergrößenänderung im Pause/Stopp-Zustand**: Problem behoben, bei dem die Oberfläche bei Größenänderung des Terminals im Pause- oder Stopp-Zustand nicht sofort aktualisiert wurde. `Event::Resize`-Ereignisbehandlung hinzugefügt
- 🛠️ **Kommentar-Paginierung wird bei Pause nicht angezeigt**: Problem behoben, bei dem PageUp/PageDown im Kommentarmodus bei Pause oder Stopp nicht angezeigt wurden. Kommentar-Ladezustand zur Bedingung für periodisches Neuzeichnen hinzugefügt
- 🛠️ **Zurücksetzen der Kommentare bei Songwechsel im Kommentarmodus**: Problem behoben, bei dem Kommentare beim Songwechsel im Kommentarmodus zurückgesetzt wurden und die aktuelle Leseposition verloren ging. Zurücksetzen der Kommentare im Kommentarmodus übersprungen
- 🛠️ **Titelzeichenverlust bei Wiedergabe**: Problem des Zeichenverlusts in Songtiteln behoben, die mit Ziffern/Englisch beginnen (z.B. „17 Jahre" wurde als „1 Jahre" angezeigt). Ursache: Unicode-Symbole `►★▶■❚` haben in ostasiatischen Terminals eine mehrdeutige Breite (1 oder 2 Spalten-Unstimmigkeit), was zu Cursor-Verschiebung und Überschreiben nachfolgender Zeichen führte. Alle mehrdeutigen Unicode-Symbole durch ASCII-Zeichen mit eindeutiger Breite ersetzt: `►`→`>`, `★`→`*`, `▶`→`>>`, `■`→`||`, `❚`→`[]`

### 🔧 Verbesserungen

- 🎨 **UI-Symbole in ASCII vereinheitlicht**: Wiedergabepräfix `>>` (Wiedergabe), `||` (Pause), `[]` (Stopp), Auswahmarker `>`, Favoritenmarker `*`, aktuelles Verzeichnis-Marker `>>`, Liedtext-Hervorhebungsmarker `>`, Kommentar-Auswahlmarker `>`, Beseitigung der Breitenmehrdeutigkeit in ostasiatischen Terminals
- 📝 **Optimierung des Hinweistextes für leere Wiedergabeliste**: Änderung von „Kein verfügbares Musikverzeichnis ausgewählt, Leerer-Listen-Modus aktiviert, o drücken um Musikverzeichnis zu öffnen" zu „Kein verfügbares Musikverzeichnis, Leerer-Wiedergabelisten-Modus aktiviert, o drücken um Musikverzeichnis zu öffnen", Formulierung präziser und natürlicher
- 📂 **Standardverzeichnis bei fehlendem Verzeichnis setzen**: Wenn kein Verzeichnis verfügbar ist, automatisch das Standard-Musikverzeichnis (USERPROFILE/ter-music-rust/music) setzen und zum Musikverzeichnisverlauf hinzufügen; beim Herunterladen von Songs aus der Online-Suche das Standard-Musikverzeichnis anstelle des aktuellen Arbeitsverzeichnisses verwenden

---

## Version 1.6.0 (2026-05-04)

### 🎉 Neue Funktionen

#### Mehrsprachige Erweiterung und Internationalisierungs-Refactoring
- ✨ **6 neue UI-Sprachen hinzugefügt**: Russisch (Русский), Französisch (Français), Deutsch (Deutsch), Spanisch (Español), Italienisch (Italiano), Portugiesisch (Português) — nun insgesamt 11 Sprachen unterstützt
- ✨ **Vollständige Modul-Internationalisierung**: Alle benutzerorientierten Texte (UI-Oberfläche, CLI-Hilfe, Fehlermeldungen, Dialogtitel) sind internationalisiert, einschließlich `ui.rs`, `main.rs`, `search.rs`, `audio.rs`, `config.rs`, `playlist.rs`
- ✨ **Zentralisierte Sprachpaketverwaltung**: Modul `langs.rs` hinzugefügt, um alle Übersetzungstexte in einer Datei zentral zu verwalten, einschließlich `LangTexts`-Struktur und 11 statischer Sprachinstanzen
- ✨ **Globaler Sprachaccessor**: Funktion `langs::global_texts()` bereitgestellt, damit Nicht-UI-Module (search.rs / audio.rs / config.rs / playlist.rs) threadsicher aktuelle Übersetzungstexte abrufen können
- ✨ **Mehrsprachige AI-Prompts**: Die AI-Song-Info-Abfrage-Prompts für jede Sprache werden in der entsprechenden Sprache ausgegeben, um sicherzustellen, dass die Antwortsprache mit der UI-Sprache übereinstimmt

### 🔧 Verbesserungen

- 🌐 **CLI-Hilfe-Internationalisierung**: Kommandozeilen-Hilfe `-h` folgt nun der UI-Spracheinstellung
- 🌐 **Fehlermeldungs-Internationalisierung**: Audio-Fehler, Suchfehler, Konfigurationsfehler, Verzeichnisfehler usw. folgen nun der UI-Sprache
- 🌐 **Dialogtitel-Internationalisierung**: macOS / Linux Ordnerauswahl-Dialogtitel folgen der UI-Sprache
- ♻️ **Code-Entkopplung**: Module enthalten keine hartcodierten Textzeichenfolgen mehr; alle Texte werden über `self.t()` oder `langs::global_texts()` gelesen

### 🐞 Fehlerbehebungen

- 🛠️ **Tastaturfokus im Kommentarmodus korrigiert**: Problem behoben, bei dem im Online-Suche/Aggregat-Suche/Playlist-Suche-Modus nach Drücken von `c` zum Anzeigen von Kommentaren die Auf/Ab-Tasten die Songliste statt der Kommentarliste steuerten
- 🛠️ **Linux-Ordnerauswahl-Dialog korrigiert**: Problem behoben, bei dem Drücken von `o` unter Linux keinen grafischen Ordnerauswahl-Dialog anzeigte; korrekte Behandlung des Konflikts zwischen Raw-Modus und grafischem Dialog
- 🛠️ **UTF-8-Log-Slicing-Sicherheit korrigiert**: Möglicher Programmabsturz durch bytebasiertes Slicing von Multi-Byte-UTF-8-Zeichenfolgen behoben; auf zeichenbasierte sichere Kürzung umgestellt
- 🛠️ **Konfigurationsdatei-Formatierung korrigiert**: Problem der doppelten Ersetzung `replace("{}")` in Konfigurationsfehlermeldungen behoben, bei dem der zweite Platzhalter nicht korrekt ersetzt wurde

---

## 📝 Änderungsprotokoll

## Version 1.5.0 (2026-04-30)

### 🎉 Neue Funktionen

#### Online-Wiedergabelisten-Suche
- ✨ **Wiedergabelisten-Sucheinstieg**: `p` drücken, um direkt nach Online-Wiedergabelisten zu suchen
- ✨ **Wiedergabelisten-Inhalte durchsuchen**: nach dem Betreten einer Wiedergabeliste können Songs durchsucht und schnell abgespielt werden
- ✨ **Cache-Treffer-Wiedergabe**: bei Online-/Juhe-/Wiedergabelisten-Suche, wenn der Song bereits lokal existiert oder im Download-Cache gefunden wird, doppelten Download überspringen und direkt abspielen
- ✨ **Liedtext-Dedup-Download**: bei Online-/Juhe-/Wiedergabelisten-Suche, wenn der Song bereits lokal existiert oder im Download-Cache gefunden wird, werden Liedtextdateien nicht erneut heruntergeladen

### 🔧 Verbesserungen

- 🎵 **Liedtext-Strategie-Optimierung**: bei der Wiedergabe verwenden Liedtexte nun „Juhe zuerst, regulärer Fallback", um die Übereinstimmungsgenauigkeit zu verbessern
- 🎯 **Suchfokus-Optimierung**: Drücken von `s/n/j/p` fokussiert standardmäßig die Sucheingabe, sodass sofort getippt werden kann
- 🎯 **Suche-zu-Liste-Interaktionsoptimierung**: nach Drücken von Enter oder Klicken auf einen Song zum Starten der Wiedergabe wechselt der Fokus zur Liste, sodass Tastenkombinationen nicht mehr ins Suchfeld gelangen
- 🎯 **Online-Listen-Stilkonsistenz**: in Online-/Juhe-/Wiedergabelisten-Suchansichten sind Auswahlcursor und Wiedergabemarker getrennt und die Abstände sind an den lokalen Wiedergabelistenstil angepasst
- 🎲 **Online-Zufallswiedergabe-Konsistenzoptimierung**: im Zufallsmodus unterstützen Online-Suche und Juhe-Suche-Ergebnisse nun ein konsistentes Zufalls-Auto-Next-Verhalten wie bei der Wiedergabelistenwiedergabe
- 🛡️ **Online-Auto-Next-Schutz**: Ratenbegrenzung für Online-Auto-Skip hinzugefügt; wenn 5 aufeinanderfolgende Auto-Skips innerhalb von 3 Sekunden auftreten, wird die Wiedergabe automatisch gestoppt, um unkontrolliertes Überspringen bei nicht abspielbaren Titeln zu vermeiden

### 🐞 Fehlerkorrekturen

- 🛠️ **Liedtext-Prioritätskorrektur**: falsche Download-Prioritätsreihenfolge in Online-/Juhe-/Wiedergabelisten-Suchabläufen korrigiert
- 🛠️ **Online-Autoplay-Index-Korrektur**: Problem behoben, bei dem das Bewegen des Cursors während der Wiedergabe dazu führen konnte, dass Auto-Next von der Cursorposition statt vom tatsächlich abgespielten Song fortgesetzt wurde
- 🛠️ **Leertasten-Eingabe-Korrektur in der Suche**: Problem behoben, bei dem die Leertaste im Listenfokus-Zustand in das Suchfeld geschrieben wurde und unerwartet Ergebnisse änderte/löschte
- 🛠️ **Netzwerksuche-Initialfokus-Korrektur**: fehlender anfänglicher Eingabefokus beim Betreten der Netzwerksuche mit `n` korrigiert
- 🛠️ **Online-Zufallswiedergabe-Verhaltenskorrektur**: Problem behoben, bei dem der Zufallsmodus in Online-/Juhe-Suchergebnislisten nicht wirksam wurde
- 🛠️ **Online-Auto-Next-vorzeitiger-Stopp-Korrektur**: Problem behoben, bei dem die Wiedergabe zu früh stoppen konnte, wenn der erste Online-Titel nicht abspielbar war, durch Zählung nur tatsächlicher Auto-Next-Versuche und Zurücksetzen des Fensters nach erfolgreicher Wiedergabe

---

## Version 1.4.0 (2026-04-28)


### 🎉 Neue Funktionen

#### Juhe-Suche als Backup
- ✨ **Juhe-Suche nach Songs**: wenn die Online-Suche fehlschlägt, können Sie die Juhe-Suche verwenden, um nach Songtitel/Sänger zu suchen und herunterzuladen
- ✨ **Juhe-Suche nach Liedtexten**: wenn keine lokalen Liedtexte vorhanden sind und die Online-Suche fehlschlägt, sucht das System automatisch über Juhe nach Liedtexten nach Songtitel/Sänger und lädt sie herunter
- ✨ **Nahtloses Erlebnis**: Suche und Download erfolgen im Hintergrund ohne UI-Blockierung

#### GitHub-Token-Konfiguration
- ✨ **Benutzerdefinierter GitHub-Token**: `g` drücken, um einen eigenen GitHub-Token einzugeben, der in der Konfigurationsdatei gespeichert wird
- ✨ **Standard-Fallback**: verwendet automatisch einen Standard-Token, wenn nicht konfiguriert
- ✨ **Identitätserkennung**: bei Verwendung eines eigenen Tokens zum Einreichen von Song-Info-Diskussionen wird Ihre GitHub-Identität angezeigt

### 🔧 Verbesserungen

- 🔍 **Neues Konfigurationselement**: `github_token` (GitHub-Token, leer lassen für Standard)

---

## Version 1.3.0 (2026-04-26)

### 🎉 Neue Funktionen

#### Benutzerdefinierter AI-API-Endpunkt
- ✨ **OpenAI-kompatible API**: unterstützt jede OpenAI-kompatible API für Song-Info-Abfragen (DeepSeek, OpenRouter, OpenAI usw.)
- ✨ **3-Schritt-Konfiguration**: `k` drücken, um nacheinander API-Basis-URL → API-Schlüssel → Modellnamen einzugeben
- ✨ **Kostenloser Fallback**: verwendet automatisch das kostenlose Modell von OpenRouter (minimax/minimax-m2.5:free), wenn kein API-Schlüssel gesetzt ist
- ✨ **Direkte Abfrage**: `i` drücken, um Song-Info direkt abzufragen — keine API-Schlüssel-Vorkonfiguration erforderlich

### 🔧 Verbesserungen

- 🔍 **Prompt-Optimierung**: „Liedbedeutung" → „Liedtextbedeutung" umbenannt, „Fun Facts" → „Anekdoten"
- 🔍 **Konfigurationsfeld umbenannt**: `deepseek_api_key` → `api_key` (abwärtskompatibel mit bestehenden Konfigurationsdateien)
- 🔍 **Neue Konfigurationselemente**: `api_base_url` (API-Endpunkt, Standard DeepSeek), `api_model` (Modellname, Standard deepseek-v4-flash)

---

## Version 1.2.0 (2026-04-24)

### 🎉 Neue Funktionen

#### Song-Info-Abfrage
- ✨ **DeepSeek-Abfrage**: `i` drücken, um detaillierte Song-Info über DeepSeek per Streaming abzufragen
- ✨ **Streaming-Ausgabe**: Ergebnisse werden Zeichen für Zeichen angezeigt, kein Warten auf vollständige Generierung erforderlich
- ✨ **13 Info-Kategorien**: Interpreten, Künstlerdetails, Songwriting & Produktion, Veröffentlichungsdatum, Album (mit Titelliste), kreativer Hintergrund, Liedbedeutung, Musikstil, kommerzieller Erfolg, Auszeichnungen, Einfluss & Kritiken, Cover & Nutzung, Fun Facts
- ✨ **Mehrsprachige Antwort**: Antwortsprache folgt der UI-Sprache (SC/TC/EN/JP/KR)
- ✨ **API-Schlüssel-Verwaltung**: `k` drücken, um den DeepSeek-API-Schlüssel einzugeben, oder über die Umgebungsvariable `DEEPSEEK_API_KEY` setzen

#### Kugou-Musik-Quelle
- ✨ **Kugou Music**: Kugou als dritte Such-/Download-Plattform hinzugefügt
- ✨ **3-Plattform-Suche**: Prioritätsreihenfolge Kuwo → Kugou → NetEase
- ✨ **Weniger VIP-Einschränkungen**: Kugou bietet mehr kostenlose Download-Ressourcen
- ✨ **MD5-Signatur-Auth**: Kugou-Download-Links verwenden MD5-Signatur für höhere Erfolgsquote

### 🔧 Verbesserungen

#### Song-Info-Prompt-Optimierung
- 🔍 **Kein Präambel**: Antworten enthalten keine Begrüßungen oder Selbsteinführungen mehr
- 🔍 **Keine nummerierten Listen**: Ausgabeinhalte verwenden kein nummeriertes Listenformat mehr
- 🔍 **Künstlerdetails**: neue Kategorie mit detaillierten Künstlerinformationen (Nationalität, Geburtsort, Geburtsdatum usw.)
- 🔍 **Album-Titelliste**: Albumabschnitt enthält nun vollständige Titelliste

### 💻 Technische Details

#### Abhängigkeitsaktualisierungen
- ➕ `md5`-Abhängigkeit hinzugefügt (Kugou-Music-API-Signatur)

#### Datenstrukturen
- ♻️ `hash`-Feld zu `OnlineSong` hinzugefügt (Kugou verwendet Hash zur Identifikation von Songs)
- ♻️ `MusicSource::Kugou`-Enum-Variante hinzugefügt
- ♻️ Kugou-JSON-Parsing-Strukturen hinzugefügt

---

## Version 1.1.0 (2026-04-17)

### 🎉 Neue Funktionen

#### Liedtext-Anzeigesystem
- ✨ **Zwei-Panel-Layout**: Songliste links, Liedtexte rechts
- ✨ **Automatischer Liedtext-Download**: Download aus dem Netzwerk bei fehlenden Liedtexten
- ✨ **Intelligentes Matching**: Automatisches Finden markierter Liedtext-Dateinamen
- ✨ **Multi-Kodierungs-Unterstützung**: Unterstützt UTF-8 und GBK Liedtextdateien
- ✨ **Liedtext-Scrolling**: Automatisches Scrollen mit Wiedergabefortschritt
- ✨ **Hervorhebung**: Aktuelle Liedtextzeile in Gelb hervorgehoben
- ✨ **Songtitel-Anzeige**: Liedtext-Titel zeigt den aktuellen Songnamen an

#### Benutzererfahrung
- ✨ **Automatisches Liedtext-Matching/Download** während der Wiedergabe
- ✨ **Einheitlicher Stil**: Wiedergabeliste und Liedtextbereich verwenden konsistenten gelben Stil
- ✨ **Dynamischer Titel**: Liedtext-Titel aktualisiert sich mit dem aktuellen Song
- ✨ **Sprachwechsel**-Unterstützung
- ✨ **Themewechsel**-Unterstützung

### 🚀 Leistungsoptimierung

#### UI-Rendering
- ⚡ **Glattere Fortschrittsbalken-Updates**
- ⚡ **Weniger Neuzeichnungen** durch Optimierung der Ereignisschleife
- ⚡ **Sperr-Optimierung** zur Verbesserung der Reaktionsfähigkeit

#### Liedtext-Laden
- ⚡ **Intelligenter Cache** nach dem Laden, um wiederholtes Parsen zu vermeiden
- ⚡ **Lazy Loading** nur bei Bedarf
- ⚡ **Batch-Umbenennung-Support** zum Bereinigen von Liedtext-Dateinamenmarkierungen

### 🎨 UI-Verbesserungen

#### Visuelle Updates
- 🎨 **Einheitliches Farbschema** in Wiedergabeliste und Liedtextbereich
- 🎨 **Getrenntes Layout** für bessere Platzausnutzung
- 🎨 **Mittlere Trennlinie** für klarere visuelle Struktur

#### Informationsanzeige
- 📊 **Sichtbarer Wiedergabelistenbereich**-Anzeige
- 📊 **Songname im Liedtext-Titel**
- 📊 **Häufigere Fortschrittsbalken-Updates**

### 🔧 Funktionale Verbesserungen

#### Liedtextverwaltung
- 🔍 **Intelligente Suche** nach mehreren Liedtext-Dateinamenmustern
- 🔍 **Dateizuordnung** stellt Eins-zu-Eins-Song-Liedtext-Übereinstimmung sicher

#### Fehlerbehandlung
- 🛡️ **Freundliche Hinweise** bei Download-Fehler
- 🛡️ **Automatische Kodierungserkennung** für Liedtextdateien
- 🛡️ **10-Sekunden-Netzwerk-Timeout** zur Vermeidung langer blockierender Wartezeiten

### 🐛 Fehlerkorrekturen

- 🐛 Liedtext-Fehlzuordnung durch Dateinamenmarkierungen korrigiert
- 🐛 Kodierungsprobleme beim Liedtext-Download korrigiert
- 🐛 UI-Flackern beim Neuzeichnen korrigiert
- 🐛 Verzögerte Fortschrittsbalken-Updates korrigiert

### 💻 Technische Details

#### Abhängigkeitsaktualisierungen
- ➕ `reqwest`-HTTP-Client hinzugefügt
- ➕ `urlencoding`-Unterstützung hinzugefügt
- ➕ `encoding_rs`-Transcodierungs-Unterstützung hinzugefügt

#### Refactoring
- ♻️ Ereignisschleifenlogik optimiert
- ♻️ Liedtext-Ladeablauf verbessert
- ♻️ Farbkonstantendefinitionen vereinheitlicht

---

## Version 1.0.0 (2026-04-09)

### Kernfunktionen
- 🎵 Audiowiedergabe (Multi-Format)
- 📋 Wiedergabelistenverwaltung
- 🎹 Wiedergabesteuerung
- 🔊 Lautstärkeregelung
- 🎲 Wiedergabemoduswechsel
- 📂 Ordnernavigation

---

## 📄 KI-Unterstützung

GLM, Codex

## 📄 Lizenz

MIT License

## 🤝 Mitwirken

Issues und Pull Requests sind willkommen!
