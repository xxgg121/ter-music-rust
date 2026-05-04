<div align="center">

[简体中文](README.md) | [繁體中文](README_TC.md) | [English](README_EN.md) | [日本語](README_JA.md) | [한국어](README_KO.md) | [Русский](README_RU.md) | [Français](README_FR.md) | [Deutsch](README_DE.md) | [Español](README_ES.md) | [Italiano](README_IT.md) | [Português](README_PT.md)

# 🎵 Ter-Music-Rust - Lettore musicale da terminale 🎵

</div>

Un lettore musicale da terminale semplice e pratico, sviluppato in Rust. Supporta ricerca/download locale e online, download e visualizzazione automatica dei testi, visualizzazione dei commenti e cambio lingua/tema, con supporto per Windows, Linux e MacOS.

![preview1](preview1.png)

![preview2](preview2.png)

![preview3](preview3.png)

![preview4](preview4.png)

![preview5](preview5.png)

![preview6](preview6.png)

## ✨ Caratteristiche

### 🎵 Riproduzione audio
- **10 formati audio supportati**: MP3, WAV, FLAC, OGG, OGA, Opus, M4A, AAC, AIFF, APE
- **Controlli di riproduzione**: riproduci/pausa/stop, traccia precedente/successiva
- **Avanzamento rapido**: avanzamento rapido di 5s / 10s
- **Barra di avanzamento**: clicca sulla barra di avanzamento per saltare con precisione
- **Controllo del volume**: regolazione in tempo reale da 0 a 100, clicca sulla barra del volume per impostare

### 🔄 5 Modalità di riproduzione
| Tasto | Modalità | Descrizione |
|------|------|------|
| `1` | Riproduzione singola | Si ferma dopo il termine della traccia corrente |
| `2` | Ripetizione singola | Ripete la traccia corrente |
| `3` | Riproduzione sequenziale | Riproduce in ordine, si ferma alla fine |
| `4` | Ripetizione lista | Riproduce tutta la playlist in ciclo |
| `5` | Riproduzione casuale | Seleziona tracce casualmente |

### 📜 Sistema dei testi
- **Caricamento testi locali**: ricerca automatica dei file `.lrc` corrispondenti
- **Rilevamento codifica testi**: rilevamento automatico UTF-8 / GBK
- **Download automatico online**: download asincrono in background quando mancano i testi locali
- **Scorrimento evidenziato**: la riga corrente è evidenziata con `►`, scorrimento automatico centrato
- **Salto per timestamp del testo**: trascina l'area dei testi o usa la rotellina del mouse per saltare al timestamp

### 🔍 Ricerca
- **Ricerca locale**: premi `s` per cercare brani nella directory musicale corrente
- **Ricerca online**: premi `n` per cercare brani online per parola chiave
- **Ricerca Juhe**: Premi `j` per accedere. Cerca brani Juhe basandosi sulla corrispondenza delle parole chiave.
- **Ricerca playlist**: Premi `p` per accedere. Cerca playlist online basandosi sulla corrispondenza delle parole chiave.
- **Paginazione**: `PgUp` / `PgDn` per più risultati
- **Download online**: premi `Enter` sul risultato online selezionato per scaricarlo nella directory musicale corrente (con visualizzazione del progresso)

### 🤖 Informazioni brano
- **Interrogazione intelligente**: premi `i` per interrogare informazioni dettagliate sul brano, compatibile con qualsiasi API compatibile con OpenAI
- **Output in streaming**: i risultati vengono mostrati carattere per carattere, senza bisogno di attendere la generazione completa
- **Informazioni ricche**: copre 13 categorie inclusi dettagli sull'artista, composizione, tracklist dell'album, background creativo, significato del testo, stile musicale, aneddoti e altro
- **Supporto multilingue**: la lingua di risposta segue l'impostazione della lingua dell'interfaccia (SC/TC/EN/JP/KR)
- **API personalizzata**: premi `k` per configurare l'URL base dell'API, l'API Key e il nome del modello in 3 passaggi — compatibile con DeepSeek, OpenRouter, AIHubMix e qualsiasi endpoint compatibile con OpenAI
- **Fallback gratuito**: utilizza automaticamente il modello gratuito di OpenRouter (minimax/minimax-m2.5:free) quando nessuna API Key è configurata

### ⭐ Preferiti
- **Aggiungi/rimuovi preferiti**: premi `f` per alternare lo stato preferito della traccia corrente
- **Lista preferiti**: premi `v` per visualizzare i preferiti (con marcatore `★`)
- **Riproduzione tra directory**: cambio automatico di directory quando un preferito si trova fuori dalla directory corrente
- **Elimina preferito**: premi `d` nella lista dei preferiti

### 💬 Commenti
- **Commenti al brano**: premi `c` per visualizzare i commenti del brano corrente
- **Dettagli commento**: premi `Enter` per alternare vista lista/dettaglio (testo completo nel dettaglio)
- **Visualizzazione risposte**: mostra il testo del commento originale a cui si risponde, nickname e ora
- **Paginazione commenti**: `PgUp` / `PgDn`, 20 commenti per pagina
- **Caricamento in background**: i commenti vengono recuperati in thread in background senza bloccare l'interfaccia

### 📂 Gestione directory
- **Scegli directory musicale**: premi `o` per aprire la finestra di selezione cartella (la riproduzione inizia automaticamente dopo la prima apertura riuscita)
- **Cronologia directory**: premi `m` per visualizzare e cambiare rapidamente directory
- **Indicatore directory corrente**: `▶` indica la directory attualmente attiva
- **Elimina elemento cronologia**: premi `d` nella vista cronologia

### 🌐 Interfaccia multilingue
Supporta 11 lingue dell'interfaccia (scorri con `l`):

| Lingua | Valore di configurazione |
|------|--------|
| Cinese semplificato | `sc` |
| Cinese tradizionale | `tc` |
| Inglese | `en` |
| Giapponese | `ja` |
| Coreano | `ko` |
| Russo | `ru` |
| Francese | `fr` |
| Tedesco | `de` |
| Spagnolo | `es` |
| Italiano | `it` |
| Portoghese | `pt` |

### 🎨 Interfaccia multi-tema
Supporta 4 temi (scorri con `t`):

| Tema | Stile |
|------|------|
| Neon | Tono neon |
| Tramonto | Oro caldo del tramonto |
| Oceano | Blu profondo dell'oceano |
| GrigioBianco | Scala di grigi stile console |

### 🖱️ Interazione con il mouse
- **Clic sulla playlist**: clicca per riprodurre il brano direttamente
- **Clic sulla barra di avanzamento**: salta a una posizione specifica
- **Clic sulla barra del volume**: regola il volume
- **Trascinamento testo**: trascina con il tasto sinistro per saltare al timestamp del testo
- **Rotellina sul testo**: scorri su/giù per saltare alla riga di testo precedente/successiva
- **Clic su risultato di ricerca**: clic nella ricerca locale per riprodurre, clic nella ricerca online per scaricare
- **Clic su commento**: clicca per aprire il dettaglio

### 📊 Visualizzazione forma d'onda
- Barre della forma d'onda dinamiche basate sul volume RMS reale durante la riproduzione
- Smorzamento EMA per visuali più morbide
- La forma d'onda si congela quando in pausa

### ⚙️ Configurazione persistente
La configurazione è memorizzata in `USERPROFILE/ter-music-rust/config.json` nella directory del programma e viene salvata/ripristinata automaticamente:

| Elemento di configurazione | Descrizione |
|--------|------|
| `music_directory` | Ultima directory musicale aperta |
| `play_mode` | Modalità di riproduzione |
| `current_index` | Indice dell'ultimo brano riprodotto (riprendi riproduzione) |
| `volume` | Volume (0-100) |
| `favorites` | Lista dei preferiti |
| `dir_history` | Cronologia delle directory |
| `api_key` | API Key (per interrogazione informazioni brano, retrocompatibile con `deepseek_api_key`) |
| `api_base_url` | URL base dell'API (predefinito: `https://api.deepseek.com/`) |
| `api_model` | Nome del modello AI (predefinito: `deepseek-v4-flash`) |
| `github_token` | Token GitHub (usato per inviare discussioni sulle informazioni dei brani; lasciare vuoto per usare il Token predefinito) |
| `theme` | Nome del tema |
| `language` | Lingua dell'interfaccia (`sc` / `tc` / `en` / `ja` / `ko` / `ru` / `fr` / `de` / `es` / `it` / `pt`) |

**Attivatori di salvataggio automatico**: cambio traccia, cambio tema, cambio lingua, cambio preferiti, ogni 30 secondi e all'uscita (incluso Ctrl+C)

---

## 🚀 Avvio rapido

### 1. Installare Rust

```powershell
# Metodo 1: winget (consigliato)
winget install Rustlang.Rustup

# Metodo 2: programma di installazione ufficiale
# Visita https://rustup.rs/ e installa
```

Verifica dell'installazione:

```powershell
rustc --version
cargo --version
```

### 2. Compilare il progetto

```powershell
cd <directory-del-progetto>

# Metodo 1: script di compilazione (consigliato)
build-win.bat

# Metodo 2: Cargo
cargo build --release
```

### 3. Eseguire

```powershell
# Metodo 1: cargo run
cargo run --release

# Metodo 2: esegui direttamente l'eseguibile
.\target\release\ter-music-rust.exe

# Metodo 3: specifica la directory musicale
.\target\release\ter-music-rust.exe -o d:\Music
cargo run --release -- -o d:\Music
```

**Priorità di caricamento directory**: riga di comando `-o` > file di configurazione > finestra di selezione cartella

---

## 🎮 Scorciatoie da tastiera

### Vista principale

| Tasto | Azione |
|------|------|
| `↑/↓` | Seleziona brano |
| `Enter` | Riproduci brano selezionato |
| `Space` | Riproduci/Pausa |
| `Esc` | Ferma riproduzione (nella vista commenti: torna ai testi) |
| `←/→` | Brano precedente/Successivo |
| `[` | Indietro di 5s |
| `]` | Avanti di 5s |
| `,` | Indietro di 10s |
| `.` | Avanti di 10s |
| `+/-` | Volume su/giù (passo 5) |
| `1-5` | Cambia modalità di riproduzione |
| `o` | Apri directory musicale |
| `s` | Cerca brani locali |
| `n` | Cerca brani online |
| `j` | Cerca brani Juhe |
| `p` | Cerca playlist online |
| `i` | Interroga informazioni brano |
| `f` | Aggiungi/Rimuovi preferito |
| `v` | Visualizza preferiti |
| `m` | Visualizza cronologia directory |
| `h` | Mostra informazioni di aiuto |
| `c` | Visualizza commenti del brano |
| `l` | Cambia lingua dell'interfaccia |
| `t` | Cambia tema |
| `k` | Configura endpoint API |
| `g` | Configura Token GitHub |
| `q` | Esci |

### Vista di ricerca

| Tasto | Azione |
|------|------|
| Input di caratteri | Inserisci parola chiave di ricerca |
| `Backspace` | Elimina carattere |
| `Enter` | Cerca/Riproduci/Scarica |
| `↑/↓` | Seleziona risultato |
| `PgUp/PgDn` | Pagina su/giù (ricerca online) |
| `s/n/j` | Cambia ricerca locale/online/Juhe |

| `Esc` | Esci dalla ricerca |

### Vista preferiti

| Tasto | Azione |
|------|------|
| `↑/↓` | Seleziona brano |
| `Enter` | Riproduci brano selezionato |
| `d` | Elimina preferito |
| `Esc` | Torna alla playlist |

### Vista cronologia directory

| Tasto | Azione |
|------|------|
| `↑/↓` | Seleziona directory |
| `Enter` | Passa alla directory selezionata |
| `d` | Elimina record |
| `Esc` | Torna alla playlist |

### Vista commenti

| Tasto | Azione |
|------|------|
| `↑/↓` | Seleziona commento |
| `Enter` | Alterna vista lista/dettaglio |
| `PgUp/PgDn` | Pagina su/giù |
| `Esc` | Torna alla vista testi |

### Vista informazioni brano

| Tasto | Azione |
|------|------|
| `↑/↓` | Scorri informazioni brano |
| `i` | Interroga nuovamente informazioni brano |
| `Esc` | Torna alla vista testi |

### Vista ricerca playlist

| Tasto | Azione |
|------|------|
| Input di caratteri | Inserisci parola chiave della playlist |
| `Backspace` | Elimina carattere |
| `Enter` | Cerca/Entra nella playlist/Riproduci e scarica |
| `↑/↓` | Seleziona playlist o brano |
| `PgUp/PgDn` | Pagina su/giù |
| `Esc` | Torna al livello precedente / Esci dalla ricerca |

### Vista di aiuto


| Tasto | Azione |
|------|------|
| `↑/↓` | Scorri contenuto di aiuto |
| `Esc` | Torna alla vista testi |

---

## 📦 Installazione e compilazione

### Requisiti di sistema

- **SO**: Windows 10/11
- **Rust**: 1.70+
- **Terminale**: Windows Terminal (consigliato) / CMD / PowerShell
- **Dimensione finestra**: 80×25 o superiore consigliata

### Opzione 1: Toolchain MSVC (miglior compatibilità, dimensione maggiore)

```powershell
# 1. Installare Rust
winget install Rustlang.Rustup

# 2. Installare Build Tools
winget install Microsoft.VisualStudio.2022.BuildTools
# Esegui il programma di installazione -> seleziona "Desktop development with C++" -> installa

# 3. Riavvia il terminale e compila
cargo build --release
```

### Opzione 2: Toolchain GNU (consigliata, leggera ~300 MB)

```powershell
# 1. Installare Rust
winget install Rustlang.Rustup

# 2. Installare MSYS2
winget install MSYS2.MSYS2
# Nel terminale MSYS2 esegui:
pacman -Syu
pacman -S mingw-w64-x86_64-toolchain

# 3. Aggiungi al PATH (PowerShell come Amministratore)
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\msys64\mingw64\bin", "Machine")

# 4. Cambia toolchain e compila
rustup default stable-x86_64-pc-windows-gnu
cargo build --release
```

> I programmi compilati con la toolchain GNU potrebbero richiedere queste DLL nella directory dell'eseguibile:
> `libgcc_s_seh-1.dll`, `libstdc++-6.dll`, `libwinpthread-1.dll`

### Opzione 3: Compilazione incrociata Linux su Windows

Usa `cargo-zigbuild` + `zig` come linker. Non richiede installazione di VM/sistema Linux.

```powershell
# 1. Installare zig (scegli uno)
# A: tramite pip (consigliato)
pip install ziglang

# B: tramite MSYS2
pacman -S mingw-w64-x86_64-zig

# C: download manuale
# Visita https://ziglang.org/download/, estrai e aggiungi al PATH

# 2. Installare cargo-zigbuild
cargo install cargo-zigbuild

# 3. Aggiungi target Linux
rustup target add x86_64-unknown-linux-gnu

# 4. Prepara Linux sysroot (header/librerie ALSA)
# Il progetto include già linux-sysroot/
# Se preparato manualmente, copiare da Debian/Ubuntu:
#   /usr/include/alsa/ -> linux-sysroot/usr/include/alsa/
#   /usr/lib/x86_64-linux-gnu/libasound.so* -> linux-sysroot/usr/lib/x86_64-linux-gnu/

# 5. Compila
build-linux.bat

# Oppure esegui manualmente:
cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.34
```

**Output**: `target/x86_64-unknown-linux-gnu/release/ter-music-rust`

**Distribuisci su Linux**:

```bash
# 1. Copia sull'host Linux
scp ter-music-rust user@linux-host:~/

# 2. Rendi eseguibile
chmod +x ter-music-rust

# 3. Installa ALSA runtime
sudo apt install libasound2

# 4. Esegui
./ter-music-rust -o /path/to/music
```

> `build-linux.bat` configura automaticamente `PKG_CONFIG_PATH`, `PKG_CONFIG_ALLOW_CROSS`, `RUSTFLAGS`, ecc.
> Nel target `x86_64-unknown-linux-gnu.2.34`, `.2.34` indica la versione minima di glibc per una migliore compatibilità con sistemi Linux più vecchi.

### Packaging Linux (DEB / RPM)

Se compili/crei pacchetti su Linux, usa:

```bash
# 1) RPM
./build-rpm.sh

# Genera RPM con debuginfo (opzionale)
./build-rpm.sh --with-debuginfo

# 2) DEB
./build-deb.sh

# Genera DEB con simboli di debug (opzionale)
./build-deb.sh --with-debuginfo

# Genera pacchetto sorgente conforme a dpkg-source (.dsc/.orig.tar/.debian.tar)
./build-deb.sh --with-source

# Genera entrambi: debuginfo + pacchetto sorgente
./build-deb.sh --with-debuginfo --with-source
```

Directory di output predefinite:
- `dist/rpm/`: RPM / SRPM
- `dist/deb/`: DEB / pacchetti sorgente

> Gli script leggono `name` e `version` da `Cargo.toml` per nominare automaticamente i file del pacchetto.

### Opzione 4: Compilazione incrociata MacOS su Windows

Usa `cargo-zigbuild` + `zig` + SDK MacOS. L'audio su MacOS usa CoreAudio e richiede gli header dell'SDK.

**Prerequisiti:**

```powershell
# 1. Installare zig (come per la compilazione incrociata Linux)
pip install ziglang

# 2. Installare cargo-zigbuild
cargo install cargo-zigbuild

# 3. Installare LLVM/Clang (fornisce libclang.dll per bindgen)
# A: tramite MSYS2
pacman -S mingw-w64-x86_64-clang

# B: LLVM ufficiale
winget install LLVM.LLVM

# 4. Aggiungi target MacOS
rustup target add x86_64-apple-darwin aarch64-apple-darwin
```

**Prepara l'SDK MacOS:**

Estrai `MacOSX13.3.sdk.tar.xz` in `macos-sysroot`.
Il progetto include già `macos-sysroot/` (scaricato da [macosx-sdks](https://github.com/joseluisq/macosx-sdks)).

Per scaricarlo di nuovo:

```powershell
# A: Scarica SDK preconfezionato da GitHub (consigliato, ~56 MB)
# Mirror: https://ghfast.top/https://github.com/joseluisq/macosx-sdks/releases/download/13.3/MacOSX13.3.sdk.tar.xz
curl -L -o MacOSX13.3.sdk.tar.xz https://github.com/joseluisq/macosx-sdks/releases/download/13.3/MacOSX13.3.sdk.tar.xz
mkdir macos-sysroot
tar -xf MacOSX13.3.sdk.tar.xz -C macos-sysroot --strip-components=1
del MacOSX13.3.sdk.tar.xz

# B: Copia da un sistema MacOS
scp -r mac:/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk ./macos-sysroot
```

> Fonte SDK: https://github.com/joseluisq/macosx-sdks
> Include header per CoreAudio, AudioToolbox, AudioUnit, CoreMIDI, OpenAL, IOKit, ecc.

**Compila:**

```powershell
# Usa lo script di compilazione (imposta automaticamente tutte le variabili d'ambiente)
build-mac.bat

# Oppure manualmente:
$env:LIBCLANG_PATH = "C:\msys64\mingw64\bin"      # Directory contenente libclang.dll
$env:COREAUDIO_SDK_PATH = "./macos-sysroot"         # Percorso SDK MacOS (barre in avanti)
$env:SDKROOT = "./macos-sysroot"                    # Necessario al linker zig per localizzare le librerie di sistema
$FW = "./macos-sysroot/System/Library/Frameworks"
$env:BINDGEN_EXTRA_CLANG_ARGS = "--target=x86_64-apple-darwin -isysroot ./macos-sysroot -F $FW -iframework $FW -I ./macos-sysroot/usr/include"
cargo zigbuild --release --target x86_64-apple-darwin   # Mac Intel
# Per Apple Silicon, sostituisci x86_64 con aarch64 sia nel target che negli argomenti clang
cargo zigbuild --release --target aarch64-apple-darwin  # Apple Silicon
```

**Output:**
- `target/x86_64-apple-darwin/release/ter-music-rust` — Mac Intel
- `target/aarch64-apple-darwin/release/ter-music-rust` — Apple Silicon (M1/M2/M3/M4)

**Distribuisci su MacOS**:

```bash
# 1. Copia sull'host MacOS
scp ter-music-rust user@mac-host:~/

# 2. Rendi eseguibile
chmod +x ter-music-rust

# 3. Permetti l'esecuzione di binario da fonte sconosciuta
xattr -cr ter-music-rust

# 4. Esegui (nessuna libreria audio aggiuntiva richiesta)
./ter-music-rust -o /path/to/music
```

> Nota: La compilazione incrociata MacOS richiede gli header dell'SDK MacOS; questo progetto include già `macos-sysroot/`.
> Richiede anche `libclang.dll` (installa tramite MSYS2 o LLVM).

### Cambiare toolchain

```powershell
# Mostra toolchain corrente
rustup show

# Passa a MSVC
rustup default stable-x86_64-pc-windows-msvc

# Passa a GNU
rustup default stable-x86_64-pc-windows-gnu
```

### Mirror Cargo in Cina (download più veloci)

Crea o modifica `~/.cargo/config`:

```toml
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
```

---

## 🛠️ Struttura del progetto

```text
src/
├── main.rs       # Punto di ingresso del programma (analisi args, inizializzazione, ripristino/salvataggio configurazione)
├── defs.rs       # Definizioni condivise (enum PlayMode/PlayState, struct MusicFile/Playlist)
├── audio.rs      # Controllo audio (wrapper rodio, riproduci/pausa/seek/volume/progresso)
├── analyzer.rs   # Analizzatore audio (volume RMS in tempo reale, smorzamento EMA, rendering forma d'onda)
├── playlist.rs   # Gestione playlist (scansione directory, caricamento parallelo durata, selettore cartella)
├── lyrics.rs     # Analisi testi (LRC, ricerca locale, rilevamento codifica, download in background)
├── search.rs     # Ricerca/download online (ricerca Kuwo + Kugou + NetEase, download, recupero commenti, interrogazione streaming informazioni)
├── config.rs     # Gestione configurazione (serializzazione JSON, 8 elementi persistenti)
└── ui.rs         # Interfaccia (rendering terminale, gestione eventi, modalità multi-vista, sistema tema/lingua)
```

### Stack tecnologico

| Dipendenza | Versione | Scopo |
|------|------|------|
| [rodio](https://github.com/RustAudio/rodio) | 0.19 | Decodifica e riproduzione audio (Rust puro) |
| [crossterm](https://github.com/crossterm-rs/crossterm) | 0.28 | Controllo interfaccia terminale |
| [reqwest](https://github.com/seanmonstar/reqwest) | 0.12 | Richieste HTTP |
| [serde](https://github.com/serde-rs/serde) + serde_json | 1.0 | Serializzazione JSON |
| [rayon](https://github.com/rayon-rs/rayon) | 1.10 | Caricamento parallelo durata audio |
| [encoding_rs](https://github.com/hsivonen/encoding_rs) | 0.8 | Decodifica testi GBK |
| [walkdir](https://github.com/BurntSushi/walkdir) | 2.5 | Scansione ricorsiva directory |
| [rand](https://github.com/rust-random/rand) | 0.8 | Modalità casuale |
| [unicode-width](https://github.com/unicode-rs/unicode-width) | 0.2 | Calcolo larghezza visualizzazione CJK |
| [chrono](https://github.com/chronotope/chrono) | 0.4 | Formattazione ora commenti |
| [ctrlc](https://github.com/Detegr/rust-ctrlc) | 3.4 | Gestione segnale Ctrl+C |
| [md5](https://github.com/johannhof/md5) | 0.7 | Firma MD5 API Kugou Music |
| [winapi](https://github.com/retep998/winapi-rs) | 0.3 | Supporto UTF-8 console Windows |

### Ottimizzazione compilazione Release

```toml
[profile.release]
opt-level = 3       # livello di ottimizzazione più alto
lto = true          # ottimizzazione in fase di collegamento
codegen-units = 1   # singola unità di generazione codice per migliore ottimizzazione
strip = true        # rimuovi simboli di debug
```

---

## Rust rispetto alla versione C

| Caratteristica | Versione Rust | Versione C |
|------|-----------|--------|
| Dimensione installazione | ~200 MB (Rust) / ~300 MB (GNU) | ~7 GB (Visual Studio) |
| Tempo di configurazione | ~5 min | ~1 ora |
| Velocità di compilazione | ⚡ Veloce | 🐢 Più lenta |
| Gestione dipendenze | ✅ Automatica tramite Cargo | ❌ Configurazione manuale |
| Sicurezza della memoria | ✅ Garanzie a tempo di compilazione | ⚠️ Gestione manuale necessaria |
| Multipiattaforma | ✅ Completamente multipiattaforma | ⚠️ Richiede modifiche al codice |
| Dimensione eseguibile | ~2 MB | ~500 KB |
| Utilizzo memoria | ~15-20 MB | ~10 MB |
| Utilizzo CPU | < 1% | < 1% |

---

## 📊 Prestazioni

| Metrica | Valore |
|------|------|
| Intervallo di aggiornamento UI | 50ms |
| Risposta tasto | < 50ms |
| Download testi | In background, non bloccante |
| Scansione directory | Caricamento parallelo durata, accelerazione 2-4x |
| Tempo di avvio | < 100ms |
| Utilizzo memoria | ~15-20 MB |

---

## 🐛 Risoluzione problemi

### Errori di compilazione

```powershell
# Aggiorna Rust
rustup update

# Pulisci e ricompila
cargo clean
cargo build --release
```

### `link.exe not found`

Installa Visual Studio Build Tools (vedi Opzione 1 sopra).

### `dlltool.exe not found`

Installa la toolchain MinGW-w64 completa (vedi Opzione 2 sopra).

### DLL runtime mancanti (toolchain GNU)

```powershell
Copy-Item "C:\msys64\mingw64\bin\libgcc_s_seh-1.dll" -Destination ".\target\release\"
Copy-Item "C:\msys64\mingw64\bin\libstdc++-6.dll" -Destination ".\target\release\"
Copy-Item "C:\msys64\mingw64\bin\libwinpthread-1.dll" -Destination ".\target\release\"
```

### Nessun dispositivo audio trovato

1. Assicurati che il dispositivo audio del sistema funzioni
2. Controlla le impostazioni del volume di Windows
3. Prova a riprodurre un suono di test del sistema

### Problemi di rendering UI

- Assicurati che la dimensione della finestra del terminale sia almeno 80×25
- Usa Windows Terminal per la migliore esperienza
- In CMD, assicurati che il font selezionato supporti CJK se necessario

### Ricerca online / download testi fallito

- Controlla la tua connessione di rete
- Alcuni brani potrebbero richiedere accesso VIP o essere stati rimossi
- Il file dei testi deve essere in formato LRC standard valido

### Interrogazione informazioni brano fallita

- Quando nessuna API Key è configurata, viene utilizzato automaticamente il modello gratuito di OpenRouter — nessuna configurazione manuale necessaria
- Per usare un endpoint personalizzato, premi `k` e inserisci l'URL base dell'API, l'API Key e il nome del modello in sequenza
- Compatibile con qualsiasi API compatibile con OpenAI (DeepSeek, OpenRouter, AIHubMix, ecc.)
- Controlla la connettività di rete al servizio API corrispondente

### Prima compilazione lenta

La prima compilazione scarica e compila tutte le dipendenze; questo è normale. Le compilazioni successive sono molto più veloci.

### Scarica Releases
[ter-music-rust-win.zip](https://storage.deepin.org/thread/202605041058546980_ter-music-rust-win.zip "附件(Attached)") 
[ter-music-rust-mac.zip](https://storage.deepin.org/thread/202605041059025049_ter-music-rust-mac.zip "附件(Attached)") 
[ter-music-rust-linux.zip](https://storage.deepin.org/thread/202605041059164016_ter-music-rust-linux.zip "附件(Attached)") 
[ter-music-rust_deb.zip](https://storage.deepin.org/thread/202605041059236181_ter-music-rust_deb.zip "附件(Attached)")

---

## Versione 1.6.0 (2026-05-04)

### 🎉 Nuove funzionalità

#### Espansione multilingue e refactoring dell'internazionalizzazione
- ✨ **6 nuove lingue dell'interfaccia aggiunte**: russo (Русский), francese (Français), tedesco (Deutsch), spagnolo (Español), italiano (Italiano), portoghese (Português) — ora supporta 11 lingue in totale
- ✨ **Internazionalizzazione completa dei moduli**: tutti i testi orientati all'utente (interfaccia UI, guida CLI, messaggi di errore, titoli delle finestre di dialogo) sono stati internazionalizzati, inclusi `ui.rs`, `main.rs`, `search.rs`, `audio.rs`, `config.rs`, `playlist.rs`
- ✨ **Gestione centralizzata del pacchetto lingua**: aggiunto il modulo `langs.rs` per centralizzare tutti i testi di traduzione in un unico file, inclusa la struttura `LangTexts` e 11 istanze statiche di lingue
- ✨ **Accessor globale della lingua**: fornita la funzione `langs::global_texts()` per consentire ai moduli non-UI (search.rs / audio.rs / config.rs / playlist.rs) di recuperare in modo thread-safe i testi di traduzione correnti
- ✨ **Prompt AI multilingue**: i prompt di interrogazione delle informazioni sui brani per ogni lingua vengono generati nella lingua corrispondente, garantendo che la lingua di risposta corrisponda alla lingua dell'interfaccia

### 🔧 Miglioramenti

- 🌐 **Internazionalizzazione della guida CLI**: le informazioni di guida `-h` della riga di comando ora seguono l'impostazione della lingua dell'interfaccia
- 🌐 **Internazionalizzazione dei messaggi di errore**: gli errori audio, di ricerca, di configurazione, di directory, ecc. ora seguono la lingua dell'interfaccia
- 🌐 **Internazionalizzazione dei titoli delle finestre di dialogo**: i titoli delle finestre di selezione cartella di macOS / Linux ora seguono la lingua dell'interfaccia
- ♻️ **Disaccoppiamento del codice**: i moduli non contengono più stringhe di testo hardcodificate; tutti i testi vengono letti tramite `self.t()` o `langs::global_texts()`

### 🐞 Correzioni di bug

- 🛠️ **Correzione del focus tastiera in modalità commenti**: corretto un problema per cui in modalità ricerca online/Juhe/playlist, dopo aver premuto `c` per visualizzare i commenti, i tasti su/giù controllavano la lista dei brani invece della lista dei commenti
- 🛠️ **Correzione della finestra di dialogo selezione cartella Linux**: corretto un problema per cui premendo `o` su Linux non veniva mostrata la finestra di dialogo grafica di selezione cartella; gestione corretta del conflitto tra modalità raw e finestra di dialogo grafica
- 🛠️ **Correzione di sicurezza del taglio UTF-8 nei log**: corretto un possibile crash del programma dovuto al taglio per byte di stringhe UTF-8 multibyte; passaggio al troncamento sicuro per caratteri
- 🛠️ **Correzione della formattazione del file di configurazione**: corretto un problema di doppia sostituzione `replace("{}")` nei messaggi di errore di configurazione, in cui il secondo segnaposto non veniva sostituito correttamente

---

## 📝 Registro delle modifiche

## Versione 1.5.0 (2026-04-30)

### 🎉 Nuove funzionalità

#### Ricerca playlist online
- ✨ **Ingresso ricerca playlist**: premi `p` per cercare direttamente playlist online
- ✨ **Navigazione contenuto playlist**: dopo essere entrati in una playlist, puoi navigare i brani e riprodurli rapidamente
- ✨ **Riproduzione da cache**: nella ricerca online / ricerca Juhe / ricerca playlist, se il brano esiste già localmente o corrisponde alla cache scaricata, salta il download duplicato e riproduci direttamente
- ✨ **Download testi senza duplicati**: nella ricerca online / ricerca Juhe / ricerca playlist, se il brano esiste già localmente o corrisponde alla cache scaricata, i file dei testi non vengono scaricati ripetutamente

### 🔧 Miglioramenti

- 🎵 **Ottimizzazione strategia testi**: durante la riproduzione, i testi ora usano "Juhe prima, normale come fallback" per migliorare la precisione di corrispondenza
- 🎯 **Ottimizzazione focus ricerca**: premendo `s/n/j/p` ora il focus va all'input di ricerca per impostazione predefinita, così puoi digitare immediatamente
- 🎯 **Ottimizzazione interazione ricerca-a-lista**: dopo aver premuto Invio o cliccato un brano per avviare la riproduzione, il focus passa alla lista in modo che le scorciatoie da tastiera non vadano più nella casella di ricerca
- 🎯 **Coerenza stile lista online**: nelle viste di ricerca online/Juhe/playlist, il cursore di selezione e il marcatore di riproduzione sono separati e la spaziatura è allineata con lo stile della lista locale
- 🎲 **Ottimizzazione coerenza casuale online**: in modalità Casuale, i risultati della ricerca online e Juhe ora supportano il comportamento di avanzamento automatico casuale coerente con la riproduzione della playlist
- 🛡️ **Protezione avanzamento automatico online**: aggiunta limitazione di frequenza per i salti automatici online; se si verificano 5 salti automatici consecutivi entro 3 secondi, la riproduzione si ferma automaticamente per evitare salti incontrollati su tracce non riproducibili

### 🐞 Correzioni di bug

- 🛠️ **Correzione priorità testi**: corretto l'ordine errato di priorità di download dei testi nei flussi di ricerca online / ricerca Juhe / ricerca playlist
- 🛠️ **Correzione indice riproduzione automatica online**: corretto un problema in cui spostare il cursore durante la riproduzione poteva far continuare l'avanzamento automatico dalla posizione del cursore anziché dal brano effettivamente in riproduzione
- 🛠️ **Correzione input tasto Spazio nella ricerca**: corretto un problema in cui Spazio veniva scritto nella casella di ricerca nello stato di focus lista e cambiava/cancellava inaspettatamente i risultati
- 🛠️ **Correzione focus iniziale ricerca di rete**: corretto focus di input iniziale mancante quando si accede alla ricerca di rete con `n`
- 🛠️ **Correzione comportamento casuale mancante online**: corretto un problema in cui la modalità Casuale non aveva effetto nelle liste di risultati della ricerca online / ricerca Juhe
- 🛠️ **Correzione arresto prematuro avanzamento automatico online**: corretto un problema in cui la riproduzione poteva fermarsi prematuramente quando la prima traccia online non era riproducibile contando solo i tentativi reali di avanzamento automatico e reimpostando la finestra dopo una riproduzione riuscita

---

## Versione 1.4.0 (2026-04-28)


### 🎉 Nuove funzionalità

#### Ricerca Juhe come fallback
- ✨ **Ricerca Juhe per brani**: Quando la ricerca online fallisce, puoi usare la ricerca Juhe per cercare brani per titolo/cantante e scaricarli.
- ✨ **Ricerca Juhe per testi**: Se non ci sono testi locali e la ricerca online fallisce, il sistema cercherà automaticamente testi per titolo/cantante tramite la ricerca Juhe e li scaricherà.
- ✨ **Esperienza fluida**: ricerca e download avvengono in background senza bloccare l'interfaccia

#### Configurazione Token GitHub
- ✨ **Token GitHub personalizzato**: premi `g` per inserire il tuo Token GitHub, salvato nel file di configurazione
- ✨ **Fallback predefinito**: utilizza automaticamente un Token predefinito quando non configurato
- ✨ **Riconoscimento identità**: Quando invii informazioni sui brani per la discussione usando il tuo Token, verrà visualizzata la tua identità GitHub.

### 🔧 Miglioramenti

- 🔍 **Nuovo elemento di configurazione**: `github_token` (Token GitHub, lasciare vuoto per usare quello predefinito)

---

## Versione 1.3.0 (2026-04-26)

### 🎉 Nuove funzionalità

#### Endpoint API AI personalizzato
- ✨ **API compatibile con OpenAI**: supporta qualsiasi API compatibile con OpenAI per l'interrogazione delle informazioni sui brani (DeepSeek, OpenRouter, OpenAI, ecc.)
- ✨ **Configurazione in 3 passaggi**: premi `k` per inserire sequenzialmente URL base API → API Key → nome del modello
- ✨ **Fallback gratuito**: utilizza automaticamente il modello gratuito di OpenRouter (minimax/minimax-m2.5:free) quando nessuna API Key è configurata
- ✨ **Interrogazione diretta**: premi `i` per interrogare direttamente le informazioni sul brano — non è richiesta pre-configurazione dell'API Key

### 🔧 Miglioramenti

- 🔍 **Ottimizzazione del prompt**: rinominato "Significato del brano" → "Significato del testo", "Curiosità" → "Aneddoti"
- 🔍 **Campo configurazione rinominato**: `deepseek_api_key` → `api_key` (retrocompatibile con file di configurazione esistenti)
- 🔍 **Nuovi elementi di configurazione**: `api_base_url` (endpoint API, predefinito DeepSeek), `api_model` (nome modello, predefinito deepseek-v4-flash)

---

## Versione 1.2.0 (2026-04-24)

### 🎉 Nuove funzionalità

#### Interrogazione informazioni brano
- ✨ **Interrogazione DeepSeek**: premi `i` per interrogare in streaming informazioni dettagliate sul brano tramite DeepSeek
- ✨ **Output in streaming**: i risultati vengono mostrati carattere per carattere, senza bisogno di attendere la generazione completa
- ✨ **13 categorie di informazioni**: interpreti, dettagli artista, composizione e produzione, data di rilascio, album (con tracklist), background creativo, significato del brano, stile musicale, performance commerciale, premi, impatto e recensioni, cover e utilizzi, aneddoti
- ✨ **Risposta multilingue**: la lingua di risposta segue la lingua dell'interfaccia (SC/TC/EN/JP/KR)
- ✨ **Gestione API Key**: premi `k` per inserire la API Key di DeepSeek, o impostala tramite variabile d'ambiente `DEEPSEEK_API_KEY`

#### Fonte musicale Kugou
- ✨ **Musica Kugou**: aggiunto Kugou come terza piattaforma di ricerca/download
- ✨ **Ricerca su 3 piattaforme**: ordine di priorità è Kuwo → Kugou → NetEase
- ✨ **Meno restrizioni VIP**: Kugou fornisce più risorse di download gratuite
- ✨ **Autenticazione firma MD5**: i link di download di Kugou usano firma MD5 per un tasso di successo più alto

### 🔧 Miglioramenti

#### Ottimizzazione prompt informazioni brano
- 🔍 **Senza preambolo**: le risposte non includono più saluti o auto-presentazioni
- 🔍 **Senza elenchi numerati**: il contenuto dell'output non usa più il formato di elenco numerato
- 🔍 **Dettagli artista**: nuova categoria con informazioni dettagliate sull'artista (nazionalità, luogo di nascita, data di nascita, ecc.)
- 🔍 **Tracklist dell'album**: la sezione album ora include la tracklist completa

### 💻 Dettagli tecnici

#### Aggiornamenti dipendenze
- ➕ Aggiunta dipendenza `md5` (firma API Kugou Music)

#### Strutture dati
- ♻️ Aggiunto campo `hash` a `OnlineSong` (Kugou usa l'hash per identificare i brani)
- ♻️ Aggiunta variante enum `MusicSource::Kugou`
- ♻️ Aggiunte strutture di analisi JSON Kugou

---

## Versione 1.1.0 (2026-04-17)

### 🎉 Nuove funzionalità

#### Sistema di visualizzazione testi
- ✨ **Layout a due pannelli**: lista brani a sinistra, testi a destra
- ✨ **Download automatico testi**: scarica dalla rete quando mancano i testi
- ✨ **Corrispondenza intelligente**: ricerca automatica dei nomi file dei testi contrassegnati
- ✨ **Supporto multi-codifica**: supporta file di testi UTF-8 e GBK
- ✨ **Scorrimento testi**: scorrimento automatico con il progresso di riproduzione
- ✨ **Evidenziazione**: riga di testo corrente evidenziata in giallo
- ✨ **Visualizzazione titolo brano**: il titolo dei testi mostra il nome del brano corrente

#### Esperienza utente
- ✨ **Corrispondenza/download automatico dei testi** durante la riproduzione
- ✨ **Stile unificato**: la playlist e l'area dei testi usano uno stile giallo coerente
- ✨ **Titolo dinamico**: il titolo dei testi si aggiorna con il brano corrente
- ✨ **Cambio lingua** supportato
- ✨ **Cambio tema** supportato

### 🚀 Ottimizzazione delle prestazioni

#### Rendering UI
- ⚡ **Aggiornamenti barra di avanzamento più fluidi**
- ⚡ **Riduzione ridisegni** ottimizzando il ciclo degli eventi
- ⚡ **Ottimizzazione dei lock** per migliorare la reattività

#### Caricamento testi
- ⚡ **Cache intelligente** dopo il caricamento per evitare analisi ripetute
- ⚡ **Caricamento pigro** solo quando necessario
- ⚡ **Supporto rinomina batch** per pulire i marcatori dei nomi file dei testi

### 🎨 Miglioramenti UI

#### Aggiornamenti visivi
- 🎨 **Schema colori unificato** nella playlist e nell'area dei testi
- 🎨 **Layout diviso** per migliore utilizzo dello spazio
- 🎨 **Linea separatrice centrale** per una struttura visiva più chiara

#### Visualizzazione informazioni
- 📊 **Visualizzazione range visibile** della playlist
- 📊 **Nome brano nel titolo dei testi**
- 📊 **Aggiornamenti più frequenti della barra di avanzamento**

### 🔧 Miglioramenti funzionali

#### Gestione testi
- 🔍 **Ricerca intelligente** per molteplici pattern di nomi file dei testi
- 🔍 **Mappatura file** garantisce corrispondenza uno-a-uno brano-testo

#### Gestione degli errori
- 🛡️ **Messaggi amichevoli** in caso di fallimento del download
- 🛡️ **Rilevamento automatico della codifica** per i file dei testi
- 🛡️ **Timeout di rete di 10 secondi** per evitare attese prolungate

### 🐛 Correzioni di bug

- 🐛 Corretta mancata corrispondenza dei testi causata da marcatori nei nomi file
- 🐛 Corretti problemi di codifica nel download dei testi
- 🐛 Corretto sfarfallio dell'UI durante il ridisegno
- 🐛 Corretto ritardo negli aggiornamenti della barra di avanzamento

### 💻 Dettagli tecnici

#### Aggiornamenti dipendenze
- ➕ Aggiunto client HTTP `reqwest`
- ➕ Aggiunto supporto `urlencoding`
- ➕ Aggiunto supporto transcodifica `encoding_rs`

#### Refactoring
- ♻️ Ottimizzata la logica del ciclo degli eventi
- ♻️ Migliorato il flusso di caricamento dei testi
- ♻️ Unificate le definizioni delle costanti di colore

---

## Versione 1.0.0 (2026-04-09)

### Funzionalità principali
- 🎵 Riproduzione audio (multiformato)
- 📋 Gestione playlist
- 🎹 Controlli di riproduzione
- 🔊 Controllo del volume
- 🎲 Cambio modalità di riproduzione
- 📂 Navigazione cartelle

---

## 📄 Assistenza AI

GLM, Codex

## 📄 Licenza

Licenza MIT

## 🤝 Contribuire

Issues e Pull Request sono benvenuti!
