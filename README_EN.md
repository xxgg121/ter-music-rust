<div align="center">

[简体中文](README.md) | [繁體中文](README_TC.md) | [English](README_EN.md) | [日本語](README_JA.md) | [한국어](README_KO.md) | [Русский](README_RU.md) | [Français](README_FR.md) | [Deutsch](README_DE.md) | [Español](README_ES.md) | [Italiano](README_IT.md) | [Português](README_PT.md)

# 🎵 Ter-Music-Rust - Terminal Music Player 🎵

</div>

A simple and practical terminal-based music player, implemented in Rust, featuring functions such as local/network song search and download, automatic display of lyrics, comment viewing, language and theme switching, and support for Windows, Linux, and MacOS systems.

![preview](preview.gif)

![preview1](preview1.png)

![preview2](preview2.png)

![preview3](preview3.png)

![preview4](preview4.png)

![preview5](preview5.png)

![preview6](preview6.png)

## ✨ Features

### 🎵 Audio Playback
- **10 audio formats supported**: MP3, WAV, FLAC, OGG, OGA, Opus, M4A, AAC, AIFF, APE
- **Playback controls**: play/pause/stop, previous/next track
- **Seeking**: fast seek by 5s / 10s
- **Progress bar seeking**: click the progress bar to jump precisely
- **Volume control**: real-time adjustment from 0-100, click volume bar to set
- **Recommended songs**: press `r` to enable today's recommendations, press `a` to generate recommendations from natural-language requests
- **Recently played**: press `b` to view recently played songs with title, playback time, and play count
- **M3U import/export**: press `x` to import an M3U playlist and `e` to export the current playlist
- **Search history**: shows history when the search input is empty, keeps up to 20 entries, and saves automatically
- **Playback speed**: supports 50%-200% playback speed, adjust with `{`/`}` in 25% steps
- **A-B loop**: press `;` to set point A, `'` to set point B or toggle the loop, and `、` to clear it

### 🔄 Playback Modes
| Key | Mode | Description |
|------|------|------|
| `1` | Single Play | Stop after current track finishes |
| `2` | Single Loop | Repeat current track |
| `3` | Sequential Play | Play in order, stop at end |
| `4` | List Loop | Repeat the whole playlist |
| `5` | Shuffle Play | Randomly pick tracks |

### 📜 Lyrics System
- **Local lyric loading**: automatically find matching `.lrc` files
- **Lyric encoding detection**: auto-detect UTF-8 / GBK
- **Automatic online download**: async background download when local lyrics are missing
- **Scrolling highlight**: current line is highlighted with `>`, auto-centered scrolling
- **Lyric position jump**: drag lyric area or use mouse wheel to jump by lyric timestamp
- **Lyrics translation**: press `y` to show lyrics translation, with streaming translation and translation cache
- **Bilingual lyrics**: show original lyrics and translations together in the main view and desktop lyrics
- **Desktop lyrics**: press `z` to toggle floating lyrics, with vertical, horizontal, and Karaoke modes
- **Lyric calibration**: press `u` to enter lyric timing calibration and save the lyric time offset

### 🔍 Search
- **Local search**: press `s` to search songs in current music directory
- **Online search**: press `n` to search online songs by keyword
- **Juhe Search**: Press `j` to enter. Search for juhe songs based on keyword matching.
- **Playlist Search**: Press `p` to enter. Search for online playlists based on the keyword matching.
- **Paging**: `PgUp` / `PgDn` for more results
- **Online download**: press `Enter` on selected online result to download into current music directory (with progress display)

### 🤖 Song Info
- **Smart query**: press `i` to query detailed song information, supports any OpenAI-compatible API
- **Streaming output**: results are displayed character by character, no need to wait for full generation
- **Rich information**: covers 13 categories including artist details, songwriting, album track listing, creative background, lyric meaning, musical style, anecdotes, and more
- **Multi-language support**: response language follows the UI language setting (SC/TC/EN/JP/KR)
- **Custom API**: press `k` to configure API base URL, API Key, and model name in 3 steps — supports DeepSeek, OpenRouter, AIHubMix, and any OpenAI-compatible endpoint
- **Free fallback**: automatically uses OpenRouter's free model (minimax/minimax-m2.5:free) when no API Key is configured

### ⭐ Favorites
- **Add/remove favorites**: press `f` to toggle favorite state of current track
- **Favorites list**: press `v` to view favorites (with `*` marker)
- **Cross-directory playback**: auto-switch directory when a favorite is outside current directory
- **Delete favorite**: press `d` in favorites list

### 💬 Comments
- **Song comments**: press `c` to view comments of current song
- **Comment summary**: press `c` again in the comments page to let AI summarize resonance points, emotional tone, representative opinions, keywords, and disagreements
- **Comment details**: press `Enter` to toggle list/detail view (full text in detail)
- **Reply display**: shows original replied comment text, nickname, and time
- **Comment paging**: `PgUp` / `PgDn`, 20 comments per page
- **Background loading**: comments are fetched in background threads without blocking UI

### 📂 Directory Management
- **Choose music directory**: press `o` to open folder picker dialog (playback starts automatically after first successful open)
- **Open directory history**: press `m` to view and quickly switch directories
- **Current directory marker**: `>>` indicates currently active directory
- **Delete history item**: press `d` in history view

### 🌐 Multi-language UI
Supports 11 UI languages (cycle with `l`):

| Language | Config Value |
|------|--------|
| Simplified Chinese | `sc` |
| Traditional Chinese | `tc` |
| English | `en` |
| Japanese | `ja` |
| Korean | `ko` |
| Русский | `ru` |
| Français | `fr` |
| Deutsch | `de` |
| Español | `es` |
| Italiano | `it` |
| Português | `pt` |

### 🎨 Multi-theme UI
Supports 4 themes (cycle with `t`):

| Theme | Style |
|------|------|
| Neon | Neon tone |
| Sunset | Warm sunset gold |
| Ocean | Deep ocean blue |
| GrayWhite | Console-like grayscale |

### 🖱️ Mouse Interaction
- **Playlist click**: click to play song directly
- **Progress bar click**: jump to specific position
- **Volume bar click**: adjust volume
- **Lyric drag**: left-drag to jump to lyric timestamp
- **Lyric wheel**: scroll up/down to jump to previous/next lyric line
- **Search result click**: local search click to play, online search click to download
- **Comment click**: click to open detail

### 📊 Waveform Visualization
- Dynamic waveform bars based on real RMS volume during playback
- EMA smoothing for softer visuals
- Waveform freezes when paused

### ⚙️ Persistent Configuration
On Windows, configuration is stored in `%USERPROFILE%/AppData/Roaming/ter-music-rust/config.json`. On Linux and macOS, it is stored in `XDG_CONFIG_HOME/ter-music-rust/config.json` or `~/.config/ter-music-rust/config.json`. The following settings are auto-saved and restored:

| Config Item | Description |
|--------|------|
| `music_directory` | Last opened music directory |
| `play_mode` | Playback mode |
| `current_index` | Last played song index (resume playback) |
| `volume` | Volume (0-100) |
| `favorites` | Favorites list |
| `dir_history` | Directory history |
| `search_history` | Search history (keeps up to 20 entries) |
| `api_key` | API Key (for song info query, backward compatible with `deepseek_api_key`) |
| `api_base_url` | API base URL (default: `https://api.deepseek.com/`) |
| `api_model` | AI model name (default: `deepseek-v4-flash`) |
| `github_token` | GitHub Token (used for submitting song info discussions; leave empty to use default Token) |
| `recommand` | Today's recommended songs toggle (default `false`) |
| `theme` | Theme name |
| `language` | UI language (`sc` / `tc` / `en` / `ja` / `ko` / `ru` / `fr` / `de` / `es` / `it` / `pt`) |
| `lyrics_visible` | Whether desktop lyrics are shown (default `false`) |
| `lyrics_position` | Desktop lyrics position (`bottom` / `top`, default `bottom`) |
| `lyrics_scroll` | Desktop lyrics scroll mode (`vertical` / `horizontal` / `karaoke`, default `vertical`) |
| `lyrics_alpha` | Desktop lyrics background transparency 10-100 (default 70) |
| `lyrics_x` | Desktop lyrics window X coordinate (`-1` means auto-calculated) |
| `lyrics_y` | Desktop lyrics window Y coordinate (`-1` means auto-calculated) |
| `lyrics_offset` | Lyric time offset in seconds (used for lyric calibration) |

**Auto-save triggers**: track change, theme change, language change, favorite change, search history update, desktop lyrics control change, every 30 seconds, and on exit (including Ctrl+C)

---

## 🚀 Quick Start

### 1. Direct Install (Recommended)

If you have Rust installed, you can install from crates.io and run directly:

```powershell
cargo install ter-music-rust
ter-music-rust
```

### 2. Install Rust (Optional)

```powershell
# Method 1: winget (recommended)
winget install Rustlang.Rustup

# Method 2: official installer
# Visit https://rustup.rs/ and install
```

Verify installation:

```powershell
rustc --version
cargo --version
```

### 3. Build the project

```powershell
# Clone the repository
git clone https://github.com/xxgg121/ter-music-rust.git
cd ter-music-rust

# Method 1: build script (recommended)
build-win.bat

# Method 2: Cargo
cargo build --release
```

### 4. Run

```powershell
# Method 1: cargo run
cargo run --release

# Method 2: run executable directly
.\target\release\ter-music-rust.exe

# Method 3: specify music directory
.\target\release\ter-music-rust.exe -o d:\Music
cargo run --release -- -o d:\Music
```

**Directory loading priority**: command line `-o` > config file > folder picker dialog

---

## 🎮 Keyboard Shortcuts

### Main View

| Key | Action |
|------|------|
| `↑/↓` | Select song |
| `Enter` | Play selected song |
| `Space` | Play/Pause |
| `Esc` | Stop playback (in comments view: back to lyrics) |
| `←/→` | Previous/Next song |
| `[` | Seek backward 5s |
| `]` | Seek forward 5s |
| `,` | Seek backward 10s |
| `.` | Seek forward 10s |
| `+/-` | Volume up/down (step 5) |
| `{/}` | Increase/Decrease playback speed (step 25%) |
| `;` | Set A-B loop start point A |
| `'` | Set A-B loop end point B or toggle loop |
| `、` | Clear A-B loop |
| `1-5` | Switch playback mode |
| `o` | Open music directory |
| `s` | Search local songs |
| `n` | Search online songs |
| `j` | Search Juhe songs |
| `p` | Search online playlists |
| `i` | Song info query |
| `a` | Recommend songs |
| `f` | Favorite/Unfavorite |
| `v` | View favorites |
| `m` | View directory history |
| `h` | Display help information |
| `c` | View song comments |
| `l` | Switch UI language |
| `t` | Switch theme |
| `k` | Configure API endpoint |
| `g` | Configure GitHub Token |
| `z` | Toggle desktop lyrics |
| `r` | Toggle recommended songs |
| `y` | Lyrics translation / Toggle bilingual display |
| `b` | Open recently played list |
| `x` | Import M3U playlist |
| `e` | Export M3U playlist |
| `u` | Enter lyric time calibration mode |
| `q` | Quit |

### Search View

| Key | Action |
|------|------|
| Character input | Enter search keyword |
| `Backspace` | Delete character |
| `Enter` | Search/Play/Download |
| `↑/↓` | Select result |
| `PgUp/PgDn` | Page up/down (online search) |
| `s/n/j` | Switch local/online/juhe search |
| `Esc` | Exit search |

### Favorites View

| Key | Action |
|------|------|
| `↑/↓` | Select song |
| `Enter` | Play selected song |
| `d` | Delete favorite |
| `Esc` | Back to playlist |

### Directory History View

| Key | Action |
|------|------|
| `↑/↓` | Select directory |
| `Enter` | Switch to selected directory |
| `d` | Delete record |
| `Esc` | Back to playlist |

### Comments View

| Key | Action |
|------|------|
| `↑/↓` | Select comment |
| `Enter` | Toggle list/detail view |
| `PgUp/PgDn` | Page up/down |
| `Esc` | Back to lyrics view |

### Song Info View

| Key | Action |
|------|------|
| `↑/↓` | Scroll song info |
| `i` | Re-query song info |
| `Esc` | Back to lyrics view |

### Playlist Search View

| Key | Action |
|------|------|
| Character input | Enter playlist keyword |
| `Backspace` | Delete character |
| `Enter` | Search/Enter playlist/Play & download |
| `↑/↓` | Select playlist or song |
| `PgUp/PgDn` | Page up/down |
| `Esc` | Back to previous level / Exit search |

### Help View


| Key | Action |
|------|------|
| `↑/↓` | Scroll help content |
| `Esc` | Back to lyrics view |

---

## 📦 Installation & Build

### System Requirements

- **OS**: Windows 10/11
- **Rust**: 1.70+
- **Terminal**: Windows Terminal (recommended) / CMD / PowerShell
- **Window size**: 80×25 or larger recommended

### Option 1: MSVC Toolchain (best compatibility, larger size)

```powershell
# 1. Install Rust
winget install Rustlang.Rustup

# 2. Install Build Tools
winget install Microsoft.VisualStudio.2022.BuildTools
# Run installer -> select "Desktop development with C++" -> install

# 3. Restart terminal and build
cargo build --release
```

### Option 2: GNU Toolchain (recommended, lightweight ~300 MB)

```powershell
# 1. Install Rust
winget install Rustlang.Rustup

# 2. Install MSYS2
winget install MSYS2.MSYS2
# In MSYS2 terminal run:
pacman -Syu
pacman -S mingw-w64-x86_64-toolchain

# 3. Add PATH (PowerShell as Administrator)
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\msys64\mingw64\bin", "Machine")

# 4. Switch toolchain and build
rustup default stable-x86_64-pc-windows-gnu
cargo build --release
```

> Programs built with GNU toolchain may require these DLLs in the executable directory:
> `libgcc_s_seh-1.dll`, `libstdc++-6.dll`, `libwinpthread-1.dll`

### Option 3: Cross-compile Linux on Windows

Use `cargo-zigbuild` + `zig` as linker. No Linux VM/system installation required.

```powershell
# 1. Install zig (choose one)
# A: via pip (recommended)
pip install ziglang

# B: via MSYS2
pacman -S mingw-w64-x86_64-zig

# C: manual download
# Visit https://ziglang.org/download/, extract and add to PATH

# 2. Install cargo-zigbuild
cargo install cargo-zigbuild

# 3. Add Linux target
rustup target add x86_64-unknown-linux-gnu

# 4. Prepare Linux sysroot (ALSA headers/libs)
# Project already includes linux-sysroot/
# If preparing manually, copy from Debian/Ubuntu:
#   /usr/include/alsa/ -> linux-sysroot/usr/include/alsa/
#   /usr/lib/x86_64-linux-gnu/libasound.so* -> linux-sysroot/usr/lib/x86_64-linux-gnu/

# 5. Build
build-linux.bat

# Or run manually:
cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.34
```

**Output**: `target/x86_64-unknown-linux-gnu/release/ter-music-rust`

**Deploy to Linux**:

```bash
# 1. Copy to Linux host
scp ter-music-rust user@linux-host:~/

# 2. Make executable
chmod +x ter-music-rust

# 3. Install ALSA runtime
sudo apt install libasound2

# 4. Run
./ter-music-rust -o /path/to/music
```

> `build-linux.bat` auto-configures `PKG_CONFIG_PATH`, `PKG_CONFIG_ALLOW_CROSS`, `RUSTFLAGS`, etc.
> In target `x86_64-unknown-linux-gnu.2.34`, `.2.34` indicates minimum glibc version for better compatibility with older Linux systems.

### Linux Packaging (DEB / RPM)

If you build/package on Linux, use:

```bash
# 1) RPM
./build-rpm.sh

# Generate debuginfo RPM (optional)
./build-rpm.sh --with-debuginfo

# 2) DEB
./build-deb.sh

# Generate debug symbols DEB (optional)
./build-deb.sh --with-debuginfo

# Generate source package compliant with dpkg-source (.dsc/.orig.tar/.debian.tar)
./build-deb.sh --with-source

# Generate both debuginfo + source package
./build-deb.sh --with-debuginfo --with-source
```

Default output directories:
- `dist/rpm/`: RPM / SRPM
- `dist/deb/`: DEB / source packages

> Scripts read `name` and `version` from `Cargo.toml` to auto-name package files.

### Option 4: Cross-compile MacOS on Windows

Use `cargo-zigbuild` + `zig` + MacOS SDK. Audio on MacOS uses CoreAudio and requires SDK headers.

**Prerequisites:**

```powershell
# 1. Install zig (same as Linux cross-compile)
pip install ziglang

# 2. Install cargo-zigbuild
cargo install cargo-zigbuild

# 3. Install LLVM/Clang (provides libclang.dll for bindgen)
# A: via MSYS2
pacman -S mingw-w64-x86_64-clang

# B: official LLVM
winget install LLVM.LLVM

# 4. Add MacOS targets
rustup target add x86_64-apple-darwin aarch64-apple-darwin
```

**Prepare MacOS SDK:**

Extract `MacOSX13.3.sdk.tar.xz` into `macos-sysroot`.
The project already includes `macos-sysroot/` (downloaded from [macosx-sdks](https://github.com/joseluisq/macosx-sdks)).

To fetch again:

```powershell
# A: Download prepackaged SDK from GitHub (recommended, ~56 MB)
# Mirror: https://ghfast.top/https://github.com/joseluisq/macosx-sdks/releases/download/13.3/MacOSX13.3.sdk.tar.xz
curl -L -o MacOSX13.3.sdk.tar.xz https://github.com/joseluisq/macosx-sdks/releases/download/13.3/MacOSX13.3.sdk.tar.xz
mkdir macos-sysroot
tar -xf MacOSX13.3.sdk.tar.xz -C macos-sysroot --strip-components=1
del MacOSX13.3.sdk.tar.xz

# B: Copy from a MacOS system
scp -r mac:/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk ./macos-sysroot
```

> SDK source: https://github.com/joseluisq/macosx-sdks
> Includes headers for CoreAudio, AudioToolbox, AudioUnit, CoreMIDI, OpenAL, IOKit, etc.

**Build:**

```powershell
# Use build script (auto-sets all env vars)
build-mac.bat

# Or manually:
$env:LIBCLANG_PATH = "C:\msys64\mingw64\bin"      # Directory containing libclang.dll
$env:COREAUDIO_SDK_PATH = "./macos-sysroot"         # MacOS SDK path (forward slashes)
$env:SDKROOT = "./macos-sysroot"                    # Needed by zig linker to locate system libs
$FW = "./macos-sysroot/System/Library/Frameworks"
$env:BINDGEN_EXTRA_CLANG_ARGS = "--target=x86_64-apple-darwin -isysroot ./macos-sysroot -F $FW -iframework $FW -I ./macos-sysroot/usr/include"
cargo zigbuild --release --target x86_64-apple-darwin   # Intel Mac
# For Apple Silicon, replace x86_64 with aarch64 in both target and clang args
cargo zigbuild --release --target aarch64-apple-darwin  # Apple Silicon
```

**Outputs:**
- `target/x86_64-apple-darwin/release/ter-music-rust` — Intel Mac
- `target/aarch64-apple-darwin/release/ter-music-rust` — Apple Silicon (M1/M2/M3/M4)

**Deploy to MacOS:**

```bash
# 1. Copy to MacOS host
scp ter-music-rust user@mac-host:~/

# 2. Make executable
chmod +x ter-music-rust

# 3. Allow running unknown-source binary
xattr -cr ter-music-rust

# 4. Run (no extra audio libs required)
./ter-music-rust -o /path/to/music
```

> Note: MacOS cross-compilation requires MacOS SDK headers; this project already includes `macos-sysroot/`.
> It also requires `libclang.dll` (install via MSYS2 or LLVM).

### Switching Toolchains

```powershell
# Show current toolchain
rustup show

# Switch to MSVC
rustup default stable-x86_64-pc-windows-msvc

# Switch to GNU
rustup default stable-x86_64-pc-windows-gnu
```

### Cargo Mirror in China (faster downloads)

Create or edit `~/.cargo/config`:

```toml
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
```

---

## 🛠️ Project Structure

```text
src/
├── main.rs       # Program entry (arg parsing, init, config restore/save)
├── defs.rs       # Shared definitions (PlayMode/PlayState enums, MusicFile/Playlist structs)
├── langs.rs      # Language packs (11 languages translation texts centralized, global language accessor)
├── audio.rs      # Audio control (rodio wrapper, play/pause/seek/volume/progress)
├── analyzer.rs   # Audio analyzer (real-time RMS volume, EMA smoothing, waveform rendering)
├── playlist.rs   # Playlist management (directory scan, parallel duration loading, folder picker)
├── lyrics.rs     # Lyric parsing (LRC format, local search, encoding detection, background download)
├── search.rs     # Online search/download (Kuwo + Kugou + NetEase search, download, comments fetch, song info streaming query)
├── config.rs     # Config management (JSON serialization, config persistence)
├── desktop_lyrics.rs # Desktop lyrics floating window (Windows API/Linux child process, transparency/position/drag/shortcuts)
├── ui.rs         # User interface (Ratatui framework, terminal rendering, event handling, multi-view mode, theme/language system)
└── ui/
    ├── input.rs      # Input handling
    ├── render.rs     # Rendering logic
    ├── layout.rs     # Layout management
    ├── theme.rs      # Theme system
    ├── mouse.rs      # Mouse interaction
    ├── terminal.rs   # Terminal management
    ├── format.rs     # Formatting tools
    └── view_model.rs # View model
```

### Tech Stack

| Dependency | Version | Purpose |
|------|------|------|
| [rodio](https://github.com/RustAudio/rodio) | 0.19 | Audio decoding and playback (pure Rust) |
| [crossterm](https://github.com/crossterm-rs/crossterm) | 0.28 | Terminal UI control |
| [reqwest](https://github.com/seanmonstar/reqwest) | 0.12 | HTTP requests |
| [serde](https://github.com/serde-rs/serde) + serde_json | 1.0 | JSON serialization |
| [rayon](https://github.com/rayon-rs/rayon) | 1.10 | Parallel audio duration loading |
| [encoding_rs](https://github.com/hsivonen/encoding_rs) | 0.8 | GBK lyric decoding |
| [walkdir](https://github.com/BurntSushi/walkdir) | 2.5 | Recursive directory scanning |
| [rand](https://github.com/rust-random/rand) | 0.8 | Shuffle mode |
| [unicode-width](https://github.com/unicode-rs/unicode-width) | 0.2 | CJK display width calculation |
| [chrono](https://github.com/chronotope/chrono) | 0.4 | Comment time formatting |
| [ctrlc](https://github.com/Detegr/rust-ctrlc) | 3.4 | Ctrl+C signal handling |
| [md5](https://github.com/johannhof/md5) | 0.7 | Kugou Music API MD5 signature |
| [winapi](https://github.com/retep998/winapi-rs) | 0.3 | Windows console UTF-8 support |

### Release Build Optimization

```toml
[profile.release]
opt-level = 3       # highest optimization level
lto = true          # link-time optimization
codegen-units = 1   # single codegen unit for better optimization
strip = true        # strip debug symbols
```

---

## Rust Compared with C Version

| Feature | Rust Version | C Version |
|------|-----------|--------|
| Installation size | ~200 MB (Rust) / ~300 MB (GNU) | ~7 GB (Visual Studio) |
| Setup time | ~5 min | ~1 hour |
| Compile speed | ⚡ Fast | 🐢 Slower |
| Dependency management | ✅ Automatic via Cargo | ❌ Manual setup |
| Memory safety | ✅ Compile-time guarantees | ⚠️ Manual management needed |
| Cross-platform | ✅ Fully cross-platform | ⚠️ Requires code changes |
| Executable size | ~2 MB | ~500 KB |
| Memory usage | ~15-20 MB | ~10 MB |
| CPU usage | < 1% | < 1% |

---

## 📊 Performance

| Metric | Value |
|------|------|
| UI refresh interval | 50ms |
| Key response | < 50ms |
| Lyric download | Background, non-blocking |
| Directory scan | Parallel duration loading, 2-4x speedup |
| Startup time | < 100ms |
| Memory usage | ~15-20 MB |

---

## 🐛 Troubleshooting

### Build errors

```powershell
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

### `link.exe not found`

Install Visual Studio Build Tools (see Option 1 above).

### `dlltool.exe not found`

Install full MinGW-w64 toolchain (see Option 2 above).

### Missing runtime DLLs (GNU toolchain)

```powershell
Copy-Item "C:\msys64\mingw64\bin\libgcc_s_seh-1.dll" -Destination ".\target\release\"
Copy-Item "C:\msys64\mingw64\bin\libstdc++-6.dll" -Destination ".\target\release\"
Copy-Item "C:\msys64\mingw64\bin\libwinpthread-1.dll" -Destination ".\target\release\"
```

### No audio device found

1. Ensure your system audio device is working
2. Check Windows volume settings
3. Try playing a system test sound

### UI rendering issues

- Ensure terminal window size is at least 80×25
- Use Windows Terminal for best experience
- In CMD, make sure the selected font supports CJK if needed

### Online search / lyric download fails

- Check your network connection
- Some songs may require VIP access or may be removed
- Lyric file must be valid standard LRC format

### Song info query fails

- When no API Key is configured, OpenRouter's free model is used automatically — no manual setup needed
- To use a custom endpoint, press `k` and enter API base URL, API Key, and model name in sequence
- Supports any OpenAI-compatible API (DeepSeek, OpenRouter, AIHubMix, etc.)
- Check network connectivity to the corresponding API service

### Slow first build

The first build downloads and compiles all dependencies; this is expected. Later builds are much faster.

### Download Releases
[ter-music-rust-win.zip](https://storage.deepin.org/thread/202605141540132256_ter-music-rust-win.zip "附件(Attached)") 
[ter-music-rust-mac.zip](https://storage.deepin.org/thread/202605141540256621_ter-music-rust-mac.zip "附件(Attached)") 
[ter-music-rust-linux.zip](https://storage.deepin.org/thread/202605141540356623_ter-music-rust-linux.zip "附件(Attached)") 
[ter-music-rust_deb.zip](https://storage.deepin.org/thread/202605141541026672_ter-music-rust_deb.zip "附件(Attached)")

---

## 📝 Changelog

## Version 2.0.0 (2026-05-14)

### 🎉 New Feature 1
- ✨ **Lyrics translation**: Press `y` to show lyrics translation, supports streaming translation and translation caching, supports switching to translated text or bilingual display.
- ✨ **Bilingual lyrics**: Support displaying original and translated lyrics simultaneously in main view and desktop lyrics; desktop bilingual layout optimized.
- ✨ **Recently played**: Record play history (history.json), press `b` to open recently played list showing song name, time, play count.
- ✨ **Playback speed**: Support playback speed 50%-200%, press `{` to increase and `}` to decrease (step 25%), UI shows current speed percent.
- ✨ **Search history**: Show search history when search input is empty, keep up to 20 items, auto-save to config.

### 🎉 New Feature 2
- ✨ **A-B loop**: Support setting A (press `;`) and B (press `'`) points and loop the interval, press `、` to clear A-B loop.
- ✨ **M3U import/export**: Press `x` to import M3U, press `e` to export current playlist to M3U.
- ✨ **Lyric calibration**: Press `u` to enter lyric time calibration mode to tweak and save `lyrics_offset`.
- ✨ **Download retry**: Network downloads support retry mechanism to improve success and stability.
- ✨ **Incremental scan**: Background incremental scan of music directory to reduce blocking and show added/removed statistics.

### 🔧 Improvements
- Implemented UI and persistent configuration support for the above features and improved synchronization logic between desktop lyrics and bilingual display.

---

## Version 1.9.0 (2026-05-11)


### 🎉 New Features

#### Ratatui UI Refactoring
- ✨ **UI Framework Upgrade**: Refactored direct crossterm UI code to use the Ratatui framework, providing higher-level TUI abstractions and better code organization
- ✨ **Modular Refactoring**: Split `ui.rs` into multiple sub-modules: `input.rs` (input handling), `render.rs` (rendering logic), `layout.rs` (layout management), `theme.rs` (theme system), `mouse.rs` (mouse interaction), `terminal.rs` (terminal management), `format.rs` (formatting tools), `view_model.rs` (view model)
- ✨ **Code Structure Optimization**: UI code is more modular, maintainable, with clear functional separation

#### Today's Recommended Songs
- ✨ **Recommended Songs Toggle**: Press `r` to enable/disable today's recommended songs feature
- ✨ **Auto Fetch Recommendations**: When enabled, automatically fetch recommended song list from network, display at the top of interface
- ✨ **Natural Language Recommendation**: Press `a` to open the recommendation input box. You can type requests like “study / work / workout / insomnia / songs for rainy days / Chinese songs for late-night coding” and press `Enter` to generate recommendations, or click the preset chips to generate them quickly
- ✨ **Keyboard Recommendation Control**: Recommended songs support `←/→` to switch items and `Enter` to directly download and play the current recommendation
- ✨ **Click to Download Play**: Click recommended song name to directly download and play
- ✨ **Mouse Wheel Scroll**: When recommended song name is long, mouse wheel can scroll horizontally to view full name
- ✨ **Song Comment Summary**: In the song comments page, press `c` again to let AI summarize listener resonance points, emotional atmosphere, representative views, keywords, and disagreements, and display the result in the right panel
- ✨ **Config Persistence**: Recommended songs toggle state is auto-saved and restored

### 🔧 Improvements

- 🎨 **UI Consistency Improvement**: Ratatui provides unified components and styles, ensuring consistency and scalability of interface elements

### 💻 Technical Details

#### Dependency Updates
- ➕ Added `ratatui` dependency (version 0.29, with crossterm feature)
- ♻️ Retained `crossterm` as underlying terminal control library

#### Project Structure Updates
- ♻️ `ui/` directory: Added multiple UI sub-modules for functional decoupling and code reuse

---

## Version 1.8.0 (2026-05-08)

### 🎉 New Features

#### Desktop Lyrics Floating Window
- ✨ **Toggle desktop lyrics**: Press `z` to show/hide desktop lyrics floating window
- ✨ **Three desktop lyrics modes**: Supports vertical scrolling, horizontal scrolling, and Karaoke display modes; right-click the desktop lyrics window to cycle modes
- ✨ **Position switching**: Press `PgUp`/`PgDn` to switch between bottom/top screen positions
- ✨ **Transparency adjustment**: Press `↑`/`↓` to adjust background transparency 10%-100%, text is always 10% more opaque than background
- ✨ **Position dragging**: Left-click drag to move window position, position auto-saved to config
- ✨ **Click to focus controls**: After clicking to focus, supports full shortcuts: `←`/`→` for prev/next track, `Space` for pause, `[`/`]` for seek, `,`/`.` for 5s/10s jump, `+/-` for volume, `1-5` for play mode, `PgUp`/`PgDn` for position, `↑`/`↓` for transparency, `T` for theme toggle
- ✨ **Cross-platform support**: Windows uses native WinAPI, Linux/MacOS uses child process approach (due to winit 0.30 limitation)
- ✨ **Lyrics scroll animation**: Smooth scrolling transition effect when switching songs
- ✨ **Karaoke mode**: Shows lyrics as two rows with four phrases, highlights the current phrase character by character, and groups lyrics by non-empty lines to avoid early group switching caused by blank lines
- ✨ **Karaoke group transition animation**: Adds fade and slight slide transitions when switching from one lyric group to the next, keeping the experience smooth like vertical/horizontal modes
- ✨ **Long-line adaptive display**: When the second Karaoke row is long, the start position automatically shifts left to show as much complete text as possible; ellipsis is used only when it exceeds the desktop lyrics area width
- ✨ **Karaoke ellipsis optimization**: In Karaoke mode, the second phrase on the top-left row and the second phrase on the bottom-right row show `...` at the end when they exceed the lyrics area, avoiding overflow past the window boundary
- ✨ **Rounded desktop lyrics area**: Desktop lyrics background now supports rounded corners, with transparent corners and cleaned edge residue for a softer window appearance
- ✨ **Highlight color consistency**: Karaoke current phrase now uses the same highlight color and bold style as other desktop lyrics modes while keeping layout width stable without shifting
- ✨ **Theme sync**: Desktop lyrics follow UI theme (Neon/Sunset/Ocean/GrayWhite)
- ✨ **Config persistence**: Desktop lyrics visibility, position, transparency, and coordinates are auto-saved and restored

### 🐞 Bug Fixes

- 🛠️ **Linux desktop lyrics theme color fix**: Fixed reversed RGB/BGR channel order when Linux desktop lyrics are rendered with `softbuffer`, which caused Neon/Sunset/Ocean theme colors to appear shifted and inconsistent with the TUI; GrayWhite was previously hard to notice because grayscale colors hide the issue
- 🛠️ **Linux desktop lyrics highlight color fix**: Fixed horizontal scrolling and Karaoke current-line highlights appearing darker than vertical scrolling on Linux, unifying current-line highlight brightness across all three modes
- 🛠️ **Linux desktop lyrics bottom positioning fix**: Fixed desktop lyrics snapping to the screen bottom instead of the top edge of the taskbar after `PgDn` in environments such as Wayland where `_NET_WORKAREA` cannot be obtained
- 🛠️ **Desktop lyrics rounded-corner transparency fix**: Fixed residual background color outside rounded corners causing visible right-angle artifacts by clearing RGB in transparent areas and fading edge colors
- 🛠️ **Chinese IME shortcut fix**: Fixed Chinese IME punctuation such as `。`, `，`, `【`, `】`, `［`, `］`, `‘`, `’` not seeking correctly or accidentally triggering track switching

### 🔧 Improvements

- 🔍 **New config items**: `lyrics_visible` (show/hide), `lyrics_position` (bottom/top), `lyrics_scroll` (scroll mode: vertical/horizontal/karaoke), `lyrics_alpha` (10-100), `lyrics_x`/`lyrics_y` (window coordinates)
- 🎨 **Desktop lyrics visual optimization**: Unified Linux desktop lyrics text alpha composition, optimized rounded-corner radius and anti-aliasing, and kept transparent backgrounds, lyric highlights, and window edges visually consistent

### 💻 Technical Details

#### Dependencies
- ➕ Added `fontdue` dependency (Linux/MacOS font rendering)
- ➕ Added `softbuffer` dependency (Linux/MacOS software rendering)

#### Project Structure
- `desktop_lyrics.rs`: Desktop lyrics module (Windows API + Linux child process, transparency/position/drag/shortcuts)

## Version 1.7.0 (2026-05-05)

### 🐞 Bug Fixes

- 🛠️ **Linux first-run display incomplete**: fixed the issue where the interface was shrunk in the top-left corner of the terminal on first run on Linux, requiring a click to display properly. Added 50ms sleep after entering alternate screen, re-query terminal size and clear screen
- 🛠️ **Empty playlist no hint**: when no music directory is selected on first run, the playlist is empty with no guidance. Added "Press o to select music directory" hint (matching the lyrics area hint style)
- 🛠️ **Selected row blue background overflow**: fixed the blue background highlight on selected rows extending past the left panel boundary into the right-side lyrics area. Replaced `Clear(UntilNewLine)` with exact-width space filling
- 🛠️ **Lyrics area residual content**: fixed the issue where old lyrics from a previous song remained visible when switching to a song with no lyrics. Added clearing loop at the beginning of lyrics drawing
- 🛠️ **Window resize not redrawing when paused/stopped**: fixed the UI not immediately redrawing when the terminal window is resized while paused or stopped. Added `Event::Resize` event handling
- 🛠️ **Comments paging not showing when paused**: fixed PageUp/PageDown in comments view not showing results when paused or stopped. Added comments loading state to periodic redraw condition
- 🛠️ **Comments reset on song change in comments mode**: fixed comments being reset when song changes while in comments mode, losing the user's current viewing content. Skip comment reset in comments mode
- 🛠️ **Title character loss when playing**: fixed characters being lost from song titles when playing (e.g. "17岁" displayed as "1岁"). Root cause: Unicode symbols `►★▶■❚` have ambiguous width (1 or 2 columns) in East Asian terminals, causing cursor offset. Replaced all ambiguous-width Unicode symbols with width-unambiguous ASCII characters: `►`→`>`, `★`→`*`, `▶`→`>>`, `■`→`||`, `❚`→`[]`

### 🔧 Improvements

- 🎨 **UI symbols unified to ASCII**: playing prefix `>>` (playing), `||` (paused), `[]` (stopped), selection marker `>`, favorite marker `*`, current directory marker `>>`, lyrics highlight marker `>`, comment selection marker `>`, eliminating East Asian terminal width ambiguity
- 📝 **Empty playlist hint wording optimization**: changed "No available music directory selected, entered empty list mode, press o to open music directory" to "No available music directory, entered empty playlist mode, press o to open music directory" for more accurate and natural wording
- 📂 **Set default directory when no directory available**: when no directory is available, automatically set the default music directory (USERPROFILE/ter-music-rust/music) and add it to music directory history; use the default music directory instead of current working directory when downloading songs from online search

---

## Version 1.6.0 (2026-05-04)

### 🎉 New Features

#### Multi-language Expansion & Internationalization Refactoring
- ✨ **6 new UI languages added**: Russian (Русский), French (Français), German (Deutsch), Spanish (Español), Italian (Italiano), Portuguese (Português) — now supporting 11 languages in total
- ✨ **Full module internationalization**: All user-facing text (UI interface, CLI help, error messages, dialog titles) has been internationalized, including `ui.rs`, `main.rs`, `search.rs`, `audio.rs`, `config.rs`, `playlist.rs`
- ✨ **Centralized language pack management**: Added `langs.rs` module to centralize all language translation texts in one file, including `LangTexts` struct and 11 language static instances
- ✨ **Global language accessor**: Provided `langs::global_texts()` function for non-UI modules (search.rs / audio.rs / config.rs / playlist.rs) to thread-safely retrieve current language translation texts
- ✨ **Multi-language AI prompts**: Each language's AI song info query prompt outputs in the corresponding language, ensuring response language matches the UI language

### 🔧 Improvements

- 🌐 **CLI help internationalization**: Command-line `-h` help information now follows the UI language setting
- 🌐 **Error message internationalization**: Audio errors, search errors, config errors, directory errors, etc. now follow the UI language
- 🌐 **Dialog title internationalization**: macOS / Linux folder selection dialog titles now follow the UI language
- ♻️ **Code decoupling**: Modules no longer hardcode text strings; all text is read via `self.t()` or `langs::global_texts()`

### 🐞 Bug Fixes

- 🛠️ **Comments mode keyboard focus fix**: fixed an issue where Up/Down keys still controlled the song list instead of the comments list when viewing comments in online search / juhe search / playlist search
- 🛠️ **Linux folder dialog fix**: fixed an issue where pressing `o` on Linux could not open a graphical folder selection dialog; now exits raw mode before calling the dialog and no longer falls back to terminal input when the dialog succeeds
- 🛠️ **UTF-8 log slicing safety fix**: fixed a potential crash caused by byte-slicing multi-byte UTF-8 characters in log output
- 🛠️ **Config file formatting fix**: fixed a bug where double `replace("{}")` in config error messages prevented the second placeholder from being replaced correctly

---

## 📝 Changelog

## Version 1.5.0 (2026-04-30)

### 🎉 New Features

#### Online Playlist Search
- ✨ **Playlist search entry**: press `p` to search online playlists directly
- ✨ **Playlist content browsing**: after entering a playlist, you can browse songs and play quickly
- ✨ **Cache-hit playback**: in online search / juhe search / playlist search, if the song already exists locally or hits downloaded cache, skip duplicate download and play directly
- ✨ **Lyrics de-dup download**: in online search / juhe search / playlist search, if the song already exists locally or hits downloaded cache, lyric files are not downloaded repeatedly

### 🔧 Improvements

- 🎵 **Lyrics strategy optimization**: during playback, lyrics now use "Juhe first, regular fallback" to improve match accuracy
- 🎯 **Search focus optimization**: pressing `s/n/j/p` now focuses the search input by default, so you can type immediately
- 🎯 **Search-to-list interaction optimization**: after pressing Enter or clicking a song to start playback, focus switches to the list so keyboard shortcuts no longer go into the search box
- 🎯 **Online list style consistency**: in online/juhe/playlist search views, selected cursor and playback marker are separated and spacing is aligned with the local playlist style
- 🎲 **Online shuffle consistency optimization**: in Shuffle mode, online search and juhe search results now support random auto-next behavior consistent with playlist playback
- 🛡️ **Online auto-next protection**: added rate limiting for online auto-skip; if 5 consecutive auto-skips occur within 3 seconds, playback stops automatically to avoid uncontrolled skipping on unplayable tracks

### 🐞 Bug Fixes

- 🛠️ **Lyrics priority fix**: fixed incorrect lyrics download priority order in online search / juhe search / playlist search flows
- 🛠️ **Online autoplay index fix**: fixed an issue where moving the cursor during playback could make auto-next continue from cursor position instead of the actually playing song
- 🛠️ **Space key input fix in search**: fixed an issue where Space was written into the search box in list-focus state and unexpectedly changed/cleared results
- 🛠️ **Network search initial focus fix**: fixed missing initial input focus when entering network search with `n`
- 🛠️ **Online shuffle missing behavior fix**: fixed an issue where Shuffle mode did not take effect in online search / juhe search result lists
- 🛠️ **Online auto-next premature stop fix**: fixed an issue where playback could stop too early when the first online track was unplayable by counting only real auto-next attempts and resetting the window after successful playback

---

## Version 1.4.0 (2026-04-28)


### 🎉 New Features

#### Song Juhe Search as a Backup
- ✨ **Juhe Search for Songs**: When searching online fails, you can use Juhe search to look for songs by song title/singer and download them.
- ✨ **Juhe Search for Lyrics**: If there are no local lyrics and the online search fails, the system will automatically search for lyrics by song title/singer through Juhe search and download them.
- ✨ **Seamless experience**: search and download happen in the background without blocking UI

#### GitHub Token Configuration
- ✨ **Custom GitHub Token**: press `g` to input your own GitHub Token, saved to config file
- ✨ **Default fallback**: automatically uses a default Token when not configured
- ✨ **Identity recognition**: When submitting song information for discussion using your own Token, it will display your own GitHub identity.

### 🔧 Improvements

- 🔍 **New config item**: `github_token` (GitHub Token, leave empty to use default)

---

## Version 1.3.0 (2026-04-26)

### 🎉 New Features

#### Custom AI API Endpoint
- ✨ **OpenAI-compatible API**: supports any OpenAI-compatible API for song info queries (DeepSeek, OpenRouter, OpenAI, etc.)
- ✨ **3-step configuration**: press `k` to enter API base URL → API Key → model name sequentially
- ✨ **Free fallback**: automatically uses OpenRouter's free model (minimax/minimax-m2.5:free) when no API Key is set
- ✨ **Direct query**: press `i` to query song info directly — no API Key pre-configuration required

### 🔧 Improvements

- 🔍 **Prompt optimization**: renamed "Song Meaning" → "Lyric Meaning", "Fun Facts" → "Anecdotes"
- 🔍 **Config field renamed**: `deepseek_api_key` → `api_key` (backward compatible with existing config files)
- 🔍 **New config items**: `api_base_url` (API endpoint, defaults to DeepSeek), `api_model` (model name, defaults to deepseek-v4-flash)

---

## Version 1.2.0 (2026-04-24)

### 🎉 New Features

#### song Info Query
- ✨ **DeepSeek query**: press `i` to stream-query detailed song info via DeepSeek
- ✨ **Streaming output**: results display character by character, no need to wait for full generation
- ✨ **13 info categories**: performers, artist details, songwriting & production, release date, album (with track listing), creative background, song meaning, musical style, commercial performance, awards, impact & reviews, covers & usage, fun facts
- ✨ **Multi-language response**: response language follows UI language (SC/TC/EN/JP/KR)
- ✨ **API Key management**: press `k` to input DeepSeek API Key, or set via `DEEPSEEK_API_KEY` environment variable

#### Kugou Music Source
- ✨ **Kugou Music**: added Kugou as a third search/download platform
- ✨ **3-platform search**: priority order is Kuwo → Kugou → NetEase
- ✨ **Reduced VIP restrictions**: Kugou provides more free download resources
- ✨ **MD5 signature auth**: Kugou download links use MD5 signature for higher success rate

### 🔧 Improvements

#### Song Info Prompt Optimization
- 🔍 **No preamble**: responses no longer include greetings or self-introductions
- 🔍 **No numbered lists**: output content no longer uses numbered list format
- 🔍 **Artist details**: new category with detailed artist information (nationality, birthplace, date of birth, etc.)
- 🔍 **Album track listing**: album section now includes complete track listing

### 💻 Technical Details

#### Dependency Updates
- ➕ Added `md5` dependency (Kugou Music API signature)

#### Data Structures
- ♻️ Added `hash` field to `OnlineSong` (Kugou uses hash to identify songs)
- ♻️ Added `MusicSource::Kugou` enum variant
- ♻️ Added Kugou JSON parsing structs

---

## Version 1.1.0 (2026-04-17)

### 🎉 New Features

#### Lyrics display system
- ✨ **Two-panel layout**: song list on the left, lyrics on the right
- ✨ **Auto lyric download**: download from network when lyrics are missing
- ✨ **Smart matching**: auto-find marked lyric filenames
- ✨ **Multi-encoding support**: supports UTF-8 and GBK lyric files
- ✨ **Lyric scrolling**: auto-scroll with playback progress
- ✨ **Highlighting**: current lyric line highlighted in yellow
- ✨ **Song title display**: lyric title shows current song name

#### User experience
- ✨ **Lyric auto-matching/downloading** during playback
- ✨ **Unified style**: playlist and lyric area use consistent yellow style
- ✨ **Dynamic title**: lyric title updates with current song
- ✨ **Language switching** support
- ✨ **Theme switching** support

### 🚀 Performance Optimization

#### UI rendering
- ⚡ **Smoother progress bar updates**
- ⚡ **Reduced redraws** by optimizing event loop
- ⚡ **Lock optimization** to improve responsiveness

#### Lyrics loading
- ⚡ **Smart cache** after loading to avoid repeated parsing
- ⚡ **Lazy loading** only when needed
- ⚡ **Batch rename support** to clean lyric filename markers

### 🎨 UI Improvements

#### Visual updates
- 🎨 **Unified color scheme** in playlist and lyric area
- 🎨 **Split layout** for better space utilization
- 🎨 **Middle separator line** for clearer visual structure

#### Information display
- 📊 **Visible playlist range** display
- 📊 **Song name in lyric title**
- 📊 **More frequent progress bar updates**

### 🔧 Functional Improvements

#### Lyrics management
- 🔍 **Smart lookup** for multiple lyric filename patterns
- 🔍 **File mapping** ensures one-to-one song-lyric matching

#### Error handling
- 🛡️ **Friendly prompts** on download failure
- 🛡️ **Automatic encoding detection** for lyric files
- 🛡️ **10-second network timeout** to avoid long blocking waits

### 🐛 Bug Fixes

- 🐛 Fixed lyric mismatch caused by filename markers
- 🐛 Fixed encoding issues in lyric downloading
- 🐛 Fixed UI flickering during redraw
- 🐛 Fixed delayed progress bar updates

### 💻 Technical Details

#### Dependency updates
- ➕ Added `reqwest` HTTP client
- ➕ Added `urlencoding` support
- ➕ Added `encoding_rs` transcoding support

#### Refactoring
- ♻️ Optimized event loop logic
- ♻️ Improved lyric loading flow
- ♻️ Unified color constant definitions

---

## Version 1.0.0 (2026-04-09)

### Core features
- 🎵 Audio playback (multi-format)
- 📋 Playlist management
- 🎹 Playback controls
- 🔊 Volume control
- 🎲 Playback mode switching
- 📂 Folder browsing

---

## 📄 AI Assistance

GLM, Codex

## 📄 License

MIT License

## 🤝 Contributing

Issues and Pull Requests are welcome!
