<div align="center">

[简体中文](README.md) | [繁體中文](README_TC.md) | [English](README_EN.md) | [日本語](README_JA.md) | [한국어](README_KO.md) | [Русский](README_RU.md) | [Français](README_FR.md) | [Deutsch](README_DE.md) | [Español](README_ES.md) | [Italiano](README_IT.md) | [Português](README_PT.md)

# 🎵 Ter-Music-Rust - 終端音樂播放器 🎵

</div>

一個簡潔實用的終端音樂播放器，使用 Rust 實現，支持本地/網絡歌曲搜尋下載、自動下載歌詞顯示、評論檢視、多語言與主題切換等功能，支持 Windows、Linux、MacOS 系統。

![preview1](preview1.png)

![preview2](preview2.png)

![preview3](preview3.png)

![preview4](preview4.png)

![preview5](preview5.png)

![preview6](preview6.png)

## ✨ 功能特色

### 🎵 音訊播放
- **支援 10 種音訊格式**：MP3、WAV、FLAC、OGG、OGA、Opus、M4A、AAC、AIFF、APE
- **播放控制**：播放/暫停/停止、上一首/下一首
- **快進快退**：5 秒 / 10 秒快速跳轉
- **進度條跳轉**：點擊進度條精確跳轉
- **音量控制**：即時調整 0-100，點擊音量條設定

### 🔄 5 種播放模式
| 按鍵 | 模式 | 說明 |
|------|------|------|
| `1` | 單曲播放 | 當前曲目結束後停止 |
| `2` | 單曲循環 | 重複播放當前曲目 |
| `3` | 順序播放 | 按順序播放，結尾停止 |
| `4` | 列表循環 | 循環播放整個播放清單 |
| `5` | 隨機播放 | 隨機挑選曲目 |

### 📜 歌詞系統
- **本機歌詞載入**：自動尋找匹配的 `.lrc` 檔案
- **歌詞編碼偵測**：自動偵測 UTF-8 / GBK
- **自動線上下載**：本機無歌詞時在背景非同步下載
- **捲動高亮**：當前行以 `>` 標記高亮，自動置中捲動
- **歌詞位置跳轉**：拖曳歌詞區域或使用滑鼠滾輪按歌詞時間戳跳轉

### 🔍 搜尋
- **本機搜尋**：按 `s` 搜尋當前音樂目錄中的歌曲
- **線上搜尋**：按 `n` 按關鍵字搜尋線上歌曲
- **聚合搜尋**：按 `j` 進入，根據關鍵字匹配搜尋聚合歌曲
- **播放清單搜尋**：按 `p` 進入，根據關鍵字匹配搜尋線上播放清單
- **分頁**：`PgUp` / `PgDn` 查看更多結果
- **線上下載**：在線上搜尋結果上按 `Enter` 下載至當前音樂目錄（帶進度顯示）

### 🤖 歌曲資訊
- **智慧查詢**：按 `i` 查詢詳細歌曲資訊，支援任何 OpenAI 相容 API
- **串流輸出**：結果逐字顯示，無需等待完整生成
- **豐富資訊**：涵蓋 13 個類別，包括藝人詳情、詞曲創作、專輯曲目列表、創作背景、歌詞含義、音樂風格、趣聞等
- **多語言支援**：回應語言跟隨 UI 語言設定（簡中/繁中/英/日/韓）
- **自訂 API**：按 `k` 分 3 步設定 API 基礎 URL、API Key 和模型名稱 — 支援 DeepSeek、OpenRouter、AIHubMix 及任何 OpenAI 相容端點
- **免費回退**：未設定 API Key 時自動使用 OpenRouter 免費模型（minimax/minimax-m2.5:free）

### ⭐ 收藏
- **新增/移除收藏**：按 `f` 切換當前曲目的收藏狀態
- **收藏清單**：按 `v` 檢視收藏（帶 `*` 標記）
- **跨目錄播放**：收藏曲目不在當前目錄時自動切換目錄
- **刪除收藏**：在收藏清單中按 `d`

### 💬 評論
- **歌曲評論**：按 `c` 檢視當前歌曲的評論
- **評論詳情**：按 `Enter` 切換列表/詳情檢視（詳情顯示全文）
- **回覆顯示**：顯示原始回覆評論文字、暱稱和時間
- **評論分頁**：`PgUp` / `PgDn`，每頁 20 條評論
- **背景載入**：評論在背景執行緒中獲取，不阻塞 UI

### 📂 目錄管理
- **選擇音樂目錄**：按 `o` 開啟資料夾選擇對話框（首次成功開啟後自動開始播放）
- **開啟目錄歷史**：按 `m` 檢視並快速切換目錄
- **當前目錄標記**：`>>` 表示當前活動目錄
- **刪除歷史記錄**：在歷史檢視中按 `d`

### 🌐 多語言 UI
支援 11 種 UI 語言（按 `l` 循環切換）：

| 語言 | 設定值 |
|------|--------|
| 簡體中文 | `sc` |
| 繁體中文 | `tc` |
| 英語 | `en` |
| 日語 | `ja` |
| 韓語 | `ko` |
| 俄語 | `ru` |
| 法語 | `fr` |
| 德語 | `de` |
| 西班牙語 | `es` |
| 意大利語 | `it` |
| 葡萄牙語 | `pt` |

### 🎨 多主題 UI
支援 4 種主題（按 `t` 循環切換）：

| 主題 | 風格 |
|------|------|
| Neon | 霓虹色調 |
| Sunset | 溫暖日落金 |
| Ocean | 深海藍 |
| GrayWhite | 終端機風格灰階 |

### 🖱️ 滑鼠互動
- **播放清單點擊**：點擊直接播放歌曲
- **進度條點擊**：跳轉至指定位置
- **音量條點擊**：調整音量
- **歌詞拖曳**：左鍵拖曳跳轉至歌詞時間戳
- **歌詞滾輪**：上下滾動跳轉至上一行/下一行歌詞
- **搜尋結果點擊**：本機搜尋點擊播放，線上搜尋點擊下載
- **評論點擊**：點擊開啟詳情

### 📊 波形視覺化
- 根據播放時即時 RMS 音量繪製動態波形條
- EMA 平滑處理使視覺效果更柔和
- 暫停時波形凍結

### ⚙️ 持久化設定
設定儲存在程式目錄的 `USERPROFILE/ter-music-rust/config.json` 中，自動儲存/恢復：

| 設定項 | 說明 |
|--------|------|
| `music_directory` | 上次開啟的音樂目錄 |
| `play_mode` | 播放模式 |
| `current_index` | 上次播放的歌曲索引（恢復播放） |
| `volume` | 音量（0-100） |
| `favorites` | 收藏清單 |
| `dir_history` | 目錄歷史 |
| `api_key` | API Key（用於歌曲資訊查詢，向下相容 `deepseek_api_key`） |
| `api_base_url` | API 基礎 URL（預設：`https://api.deepseek.com/`） |
| `api_model` | AI 模型名稱（預設：`deepseek-v4-flash`） |
| `github_token` | GitHub Token（用於提交歌曲資訊討論；留空則使用預設 Token） |
| `theme` | 主題名稱 |
| `language` | UI 語言（`sc` / `tc` / `en` / `ja` / `ko` / `ru` / `fr` / `de` / `es` / `it` / `pt`） |

**自動儲存觸發條件**：切換曲目、切換主題、切換語言、變更收藏、每 30 秒、退出時（包括 Ctrl+C）

---

## 🚀 快速開始

### 1. 安裝 Rust

```powershell
# 方法一：winget（推薦）
winget install Rustlang.Rustup

# 方法二：官方安裝程式
# 訪問 https://rustup.rs/ 並安裝
```

驗證安裝：

```powershell
rustc --version
cargo --version
```

### 2. 建置專案

```powershell
cd <專案目錄>

# 方法一：建置腳本（推薦）
build-win.bat

# 方法二：Cargo
cargo build --release
```

### 3. 執行

```powershell
# 方法一：cargo run
cargo run --release

# 方法二：直接執行
.\target\release\ter-music-rust.exe

# 方法三：指定音樂目錄
.\target\release\ter-music-rust.exe -o d:\Music
cargo run --release -- -o d:\Music
```

**目錄載入優先順序**：命令列 `-o` > 設定檔 > 資料夾選擇對話框

---

## 🎮 鍵盤快速鍵

### 主檢視

| 按鍵 | 動作 |
|------|------|
| `↑/↓` | 選擇歌曲 |
| `Enter` | 播放選中歌曲 |
| `Space` | 播放/暫停 |
| `Esc` | 停止播放（在評論檢視中：返回歌詞） |
| `←/→` | 上一首/下一首 |
| `[` | 後退 5 秒 |
| `]` | 前進 5 秒 |
| `,` | 後退 10 秒 |
| `.` | 前進 10 秒 |
| `+/-` | 音量增/減（步長 5） |
| `1-5` | 切換播放模式 |
| `o` | 開啟音樂目錄 |
| `s` | 搜尋本機歌曲 |
| `n` | 搜尋線上歌曲 |
| `j` | 搜尋聚合歌曲 |
| `p` | 搜尋線上播放清單 |
| `i` | 歌曲資訊查詢 |
| `f` | 收藏/取消收藏 |
| `v` | 檢視收藏 |
| `m` | 檢視目錄歷史 |
| `h` | 顯示說明資訊 |
| `c` | 檢視歌曲評論 |
| `l` | 切換 UI 語言 |
| `t` | 切換主題 |
| `k` | 設定 API 端點 |
| `g` | 設定 GitHub Token |
| `q` | 退出 |

### 搜尋檢視

| 按鍵 | 動作 |
|------|------|
| 字元輸入 | 輸入搜尋關鍵字 |
| `Backspace` | 刪除字元 |
| `Enter` | 搜尋/播放/下載 |
| `↑/↓` | 選擇結果 |
| `PgUp/PgDn` | 上/下翻頁（線上搜尋） |
| `s/n/j` | 切換本機/線上/聚合搜尋 |

| `Esc` | 退出搜尋 |

### 收藏檢視

| 按鍵 | 動作 |
|------|------|
| `↑/↓` | 選擇歌曲 |
| `Enter` | 播放選中歌曲 |
| `d` | 刪除收藏 |
| `Esc` | 返回播放清單 |

### 目錄歷史檢視

| 按鍵 | 動作 |
|------|------|
| `↑/↓` | 選擇目錄 |
| `Enter` | 切換至選中目錄 |
| `d` | 刪除記錄 |
| `Esc` | 返回播放清單 |

### 評論檢視

| 按鍵 | 動作 |
|------|------|
| `↑/↓` | 選擇評論 |
| `Enter` | 切換列表/詳情檢視 |
| `PgUp/PgDn` | 上/下翻頁 |
| `Esc` | 返回歌詞檢視 |

### 歌曲資訊檢視

| 按鍵 | 動作 |
|------|------|
| `↑/↓` | 捲動歌曲資訊 |
| `i` | 重新查詢歌曲資訊 |
| `Esc` | 返回歌詞檢視 |

### 播放清單搜尋檢視

| 按鍵 | 動作 |
|------|------|
| 字元輸入 | 輸入播放清單關鍵字 |
| `Backspace` | 刪除字元 |
| `Enter` | 搜尋/進入播放清單/播放並下載 |
| `↑/↓` | 選擇播放清單或歌曲 |
| `PgUp/PgDn` | 上/下翻頁 |
| `Esc` | 返回上一層 / 退出搜尋 |

### 說明檢視


| 按鍵 | 動作 |
|------|------|
| `↑/↓` | 捲動說明內容 |
| `Esc` | 返回歌詞檢視 |

---

## 📦 安裝與建置

### 系統需求

- **作業系統**：Windows 10/11
- **Rust**：1.70+
- **終端機**：Windows Terminal（推薦）/ CMD / PowerShell
- **視窗大小**：建議 80×25 或更大

### 選項一：MSVC 工具鏈（最佳相容性，體積較大）

```powershell
# 1. 安裝 Rust
winget install Rustlang.Rustup

# 2. 安裝建置工具
winget install Microsoft.VisualStudio.2022.BuildTools
# 執行安裝程式 -> 選擇「使用 C++ 的桌面開發」-> 安裝

# 3. 重啟終端機並建置
cargo build --release
```

### 選項二：GNU 工具鏈（推薦，輕量約 300 MB）

```powershell
# 1. 安裝 Rust
winget install Rustlang.Rustup

# 2. 安裝 MSYS2
winget install MSYS2.MSYS2
# 在 MSYS2 終端機中執行：
pacman -Syu
pacman -S mingw-w64-x86_64-toolchain

# 3. 新增 PATH（以管理員身份執行 PowerShell）
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\msys64\mingw64\bin", "Machine")

# 4. 切換工具鏈並建置
rustup default stable-x86_64-pc-windows-gnu
cargo build --release
```

> 使用 GNU 工具鏈建置的程式可能需要在可執行檔目錄中包含以下 DLL：
> `libgcc_s_seh-1.dll`、`libstdc++-6.dll`、`libwinpthread-1.dll`

### 選項三：在 Windows 上交叉編譯 Linux

使用 `cargo-zigbuild` + `zig` 作為連結器。無需安裝 Linux 虛擬機/系統。

```powershell
# 1. 安裝 zig（擇一）
# A：透過 pip（推薦）
pip install ziglang

# B：透過 MSYS2
pacman -S mingw-w64-x86_64-zig

# C：手動下載
# 訪問 https://ziglang.org/download/，解壓並新增至 PATH

# 2. 安裝 cargo-zigbuild
cargo install cargo-zigbuild

# 3. 新增 Linux 目標
rustup target add x86_64-unknown-linux-gnu

# 4. 準備 Linux sysroot（ALSA 頭檔案/庫）
# 專案已包含 linux-sysroot/
# 若手動準備，從 Debian/Ubuntu 複製：
#   /usr/include/alsa/ -> linux-sysroot/usr/include/alsa/
#   /usr/lib/x86_64-linux-gnu/libasound.so* -> linux-sysroot/usr/lib/x86_64-linux-gnu/

# 5. 建置
build-linux.bat

# 或手動執行：
cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.34
```

**輸出**：`target/x86_64-unknown-linux-gnu/release/ter-music-rust`

**部署至 Linux**：

```bash
# 1. 複製至 Linux 主機
scp ter-music-rust user@linux-host:~/

# 2. 賦予執行權限
chmod +x ter-music-rust

# 3. 安裝 ALSA 執行時
sudo apt install libasound2

# 4. 執行
./ter-music-rust -o /path/to/music
```

> `build-linux.bat` 會自動設定 `PKG_CONFIG_PATH`、`PKG_CONFIG_ALLOW_CROSS`、`RUSTFLAGS` 等。
> 目標 `x86_64-unknown-linux-gnu.2.34` 中，`.2.34` 表示最低 glibc 版本，以提高與舊版 Linux 系統的相容性。

### Linux 打包（DEB / RPM）

若在 Linux 上建置/打包，請使用：

```bash
# 1) RPM
./build-rpm.sh

# 產生 debuginfo RPM（可選）
./build-rpm.sh --with-debuginfo

# 2) DEB
./build-deb.sh

# 產生除錯符號 DEB（可選）
./build-deb.sh --with-debuginfo

# 產生符合 dpkg-source 的原始碼包（.dsc/.orig.tar/.debian.tar）
./build-deb.sh --with-source

# 同時產生 debuginfo + 原始碼包
./build-deb.sh --with-debuginfo --with-source
```

預設輸出目錄：
- `dist/rpm/`：RPM / SRPM
- `dist/deb/`：DEB / 原始碼包

> 腳本會從 `Cargo.toml` 讀取 `name` 和 `version` 以自動命名包檔案。

### 選項四：在 Windows 上交叉編譯 MacOS

使用 `cargo-zigbuild` + `zig` + MacOS SDK。MacOS 上的音訊使用 CoreAudio，需要 SDK 頭檔案。

**前置條件：**

```powershell
# 1. 安裝 zig（同 Linux 交叉編譯）
pip install ziglang

# 2. 安裝 cargo-zigbuild
cargo install cargo-zigbuild

# 3. 安裝 LLVM/Clang（提供 bindgen 所需的 libclang.dll）
# A：透過 MSYS2
pacman -S mingw-w64-x86_64-clang

# B：官方 LLVM
winget install LLVM.LLVM

# 4. 新增 MacOS 目標
rustup target add x86_64-apple-darwin aarch64-apple-darwin
```

**準備 MacOS SDK：**

將 `MacOSX13.3.sdk.tar.xz` 解壓至 `macos-sysroot`。
專案已包含 `macos-sysroot/`（從 [macosx-sdks](https://github.com/joseluisq/macosx-sdks) 下載）。

若需重新獲取：

```powershell
# A：從 GitHub 下載預打包 SDK（推薦，約 56 MB）
# 鏡像：https://ghfast.top/https://github.com/joseluisq/macosx-sdks/releases/download/13.3/MacOSX13.3.sdk.tar.xz
curl -L -o MacOSX13.3.sdk.tar.xz https://github.com/joseluisq/macosx-sdks/releases/download/13.3/MacOSX13.3.sdk.tar.xz
mkdir macos-sysroot
tar -xf MacOSX13.3.sdk.tar.xz -C macos-sysroot --strip-components=1
del MacOSX13.3.sdk.tar.xz

# B：從 MacOS 系統複製
scp -r mac:/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk ./macos-sysroot
```

> SDK 來源：https://github.com/joseluisq/macosx-sdks
> 包含 CoreAudio、AudioToolbox、AudioUnit、CoreMIDI、OpenAL、IOKit 等的頭檔案。

**建置：**

```powershell
# 使用建置腳本（自動設定所有環境變數）
build-mac.bat

# 或手動設定：
$env:LIBCLANG_PATH = "C:\msys64\mingw64\bin"      # 包含 libclang.dll 的目錄
$env:COREAUDIO_SDK_PATH = "./macos-sysroot"         # MacOS SDK 路徑（使用正斜線）
$env:SDKROOT = "./macos-sysroot"                    # zig 連結器定位系統庫所需
$FW = "./macos-sysroot/System/Library/Frameworks"
$env:BINDGEN_EXTRA_CLANG_ARGS = "--target=x86_64-apple-darwin -isysroot ./macos-sysroot -F $FW -iframework $FW -I ./macos-sysroot/usr/include"
cargo zigbuild --release --target x86_64-apple-darwin   # Intel Mac
# Apple Silicon 請將 x86_64 替換為 aarch64
cargo zigbuild --release --target aarch64-apple-darwin  # Apple Silicon
```

**輸出：**
- `target/x86_64-apple-darwin/release/ter-music-rust` — Intel Mac
- `target/aarch64-apple-darwin/release/ter-music-rust` — Apple Silicon（M1/M2/M3/M4）

**部署至 MacOS**：

```bash
# 1. 複製至 MacOS 主機
scp ter-music-rust user@mac-host:~/

# 2. 賦予執行權限
chmod +x ter-music-rust

# 3. 允許執行未知來源的二進位檔案
xattr -cr ter-music-rust

# 4. 執行（無需額外音訊庫）
./ter-music-rust -o /path/to/music
```

> 注意：MacOS 交叉編譯需要 MacOS SDK 頭檔案；本專案已包含 `macos-sysroot/`。
> 另外需要 `libclang.dll`（透過 MSYS2 或 LLVM 安裝）。

### 切換工具鏈

```powershell
# 顯示當前工具鏈
rustup show

# 切換至 MSVC
rustup default stable-x86_64-pc-windows-msvc

# 切換至 GNU
rustup default stable-x86_64-pc-windows-gnu
```

### 中國大陸 Cargo 鏡像（加速下載）

建立或編輯 `~/.cargo/config`：

```toml
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
```

---

## 🛠️ 專案結構

```text
src/
├── main.rs       # 程式入口（引數解析、初始化、設定恢復/儲存）
├── defs.rs       # 共用定義（PlayMode/PlayState 列舉、MusicFile/Playlist 結構體）
├── audio.rs      # 音訊控制（rodio 封裝、播放/暫停/跳轉/音量/進度）
├── analyzer.rs   # 音訊分析器（即時 RMS 音量、EMA 平滑、波形渲染）
├── playlist.rs   # 播放清單管理（目錄掃描、並行時長載入、資料夾選擇器）
├── lyrics.rs     # 歌詞解析（LRC、本機搜尋、編碼偵測、背景下載）
├── search.rs     # 線上搜尋/下載（酷我 + 酷狗 + 網易雲搜尋、下載、評論獲取、歌曲資訊串流查詢）
├── config.rs     # 設定管理（JSON 序列化、8 個持久化項目）
└── ui.rs         # UI（終端機渲染、事件處理、多檢視模式、主題/語言系統）
```

### 技術棧

| 依賴 | 版本 | 用途 |
|------|------|------|
| [rodio](https://github.com/RustAudio/rodio) | 0.19 | 音訊解碼與播放（純 Rust） |
| [crossterm](https://github.com/crossterm-rs/crossterm) | 0.28 | 終端機 UI 控制 |
| [reqwest](https://github.com/seanmonstar/reqwest) | 0.12 | HTTP 請求 |
| [serde](https://github.com/serde-rs/serde) + serde_json | 1.0 | JSON 序列化 |
| [rayon](https://github.com/rayon-rs/rayon) | 1.10 | 並行音訊時長載入 |
| [encoding_rs](https://github.com/hsivonen/encoding_rs) | 0.8 | GBK 歌詞解碼 |
| [walkdir](https://github.com/BurntSushi/walkdir) | 2.5 | 遞迴目錄掃描 |
| [rand](https://github.com/rust-random/rand) | 0.8 | 隨機播放模式 |
| [unicode-width](https://github.com/unicode-rs/unicode-width) | 0.2 | CJK 顯示寬度計算 |
| [chrono](https://github.com/chronotope/chrono) | 0.4 | 評論時間格式化 |
| [ctrlc](https://github.com/Detegr/rust-ctrlc) | 3.4 | Ctrl+C 訊號處理 |
| [md5](https://github.com/johannhof/md5) | 0.7 | 酷狗音樂 API MD5 簽章 |
| [winapi](https://github.com/retep998/winapi-rs) | 0.3 | Windows 主控台 UTF-8 支援 |

### Release 建置最佳化

```toml
[profile.release]
opt-level = 3       # 最高最佳化等級
lto = true          # 連結時最佳化
codegen-units = 1   # 單一程式碼生成單元以獲得更好的最佳化
strip = true        # 剝除除錯符號
```

---

## Rust 與 C 版本比較

| 特性 | Rust 版本 | C 版本 |
|------|-----------|--------|
| 安裝大小 | ~200 MB（Rust）/ ~300 MB（GNU） | ~7 GB（Visual Studio） |
| 安裝時間 | ~5 分鐘 | ~1 小時 |
| 編譯速度 | ⚡ 快 | 🐢 較慢 |
| 依賴管理 | ✅ Cargo 自動管理 | ❌ 手動設定 |
| 記憶體安全 | ✅ 編譯時保證 | ⚠️ 需手動管理 |
| 跨平台 | ✅ 完全跨平台 | ⚠️ 需修改程式碼 |
| 可執行檔大小 | ~2 MB | ~500 KB |
| 記憶體使用 | ~15-20 MB | ~10 MB |
| CPU 使用率 | < 1% | < 1% |

---

## 📊 效能

| 指標 | 數值 |
|------|------|
| UI 刷新間隔 | 50ms |
| 按鍵回應 | < 50ms |
| 歌詞下載 | 背景，非阻塞 |
| 目錄掃描 | 並行時長載入，2-4 倍加速 |
| 啟動時間 | < 100ms |
| 記憶體使用 | ~15-20 MB |

---

## 🐛 疑難排解

### 建置錯誤

```powershell
# 更新 Rust
rustup update

# 清理並重新建置
cargo clean
cargo build --release
```

### 找不到 `link.exe`

安裝 Visual Studio Build Tools（見上方選項一）。

### 找不到 `dlltool.exe`

安裝完整的 MinGW-w64 工具鏈（見上方選項二）。

### 缺少執行時 DLL（GNU 工具鏈）

```powershell
Copy-Item "C:\msys64\mingw64\bin\libgcc_s_seh-1.dll" -Destination ".\target\release\"
Copy-Item "C:\msys64\mingw64\bin\libstdc++-6.dll" -Destination ".\target\release\"
Copy-Item "C:\msys64\mingw64\bin\libwinpthread-1.dll" -Destination ".\target\release\"
```

### 找不到音訊裝置

1. 確保系統音訊裝置正常運作
2. 檢查 Windows 音量設定
3. 嘗試播放系統測試音效

### UI 渲染問題

- 確保終端機視窗大小至少為 80×25
- 推薦使用 Windows Terminal 以獲得最佳體驗
- 在 CMD 中，如需顯示 CJK 字元，請確保選擇支援 CJK 的字型

### 線上搜尋/歌詞下載失敗

- 檢查網路連線
- 部分歌曲可能需要 VIP 或已下架
- 歌詞檔案必須為有效的標準 LRC 格式

### 歌曲資訊查詢失敗

- 未設定 API Key 時，會自動使用 OpenRouter 的免費模型 — 無需手動設定
- 若要使用自訂端點，按 `k` 依序輸入 API 基礎 URL、API Key 和模型名稱
- 支援任何 OpenAI 相容 API（DeepSeek、OpenRouter、AIHubMix 等）
- 檢查與對應 API 服務的網路連線

### 首次建置緩慢

首次建置會下載並編譯所有依賴；這是正常現象。後續建置會快得多。

### 下載發行版
[ter-music-rust-win.zip](https://storage.deepin.org/thread/202605050312131911_ter-music-rust-win.zip "附件(Attached)") 
[ter-music-rust-mac.zip](https://storage.deepin.org/thread/202605050312183967_ter-music-rust-mac.zip "附件(Attached)") 
[ter-music-rust-linux.zip](https://storage.deepin.org/thread/202605050312251425_ter-music-rust-linux.zip "附件(Attached)") 
[ter-music-rust_deb.zip](https://storage.deepin.org/thread/202605050312355690_ter-music-rust_deb.zip "附件(Attached)")

---

## 📝 更新日誌
## 版本 1.7.0（2026-05-05）

### 🐞 錯誤修復

- 🛠️ **Linux首次運行界面顯示不完整**：修復 Linux 下首次運行程序時界面縮在終端左上角、需點擊才顯示完整的問題，進入 alternate screen 後等待 50ms 重新獲取終端尺寸並清屏
- 🛠️ **空播放列表無提示**：首次運行未選擇音樂目錄時播放列表為空但無引導提示，現添加「請按 o 選擇音樂目錄」提示（與歌詞區域提示樣式一致）
- 🛠️ **選中行藍色背景溢出**：修復選中行藍色背景延伸超出左面板邊界到右側歌詞區域的問題，改用精確寬度空格填充替代 `Clear(UntilNewLine)`
- 🛠️ **歌詞區域殘留舊內容**：修復切換歌曲後若新歌曲無歌詞，右側歌詞區域仍殘留上一首歌詞內容的問題，繪製前先清除所有行
- 🛠️ **視窗resize暫停/停止時不重繪**：修復調整終端視窗大小後，若播放處於暫停或停止狀態界面不立即更新的問題，添加 `Event::Resize` 事件處理
- 🛠️ **評論翻頁暫停時不顯示**：修復評論視圖中 PageUp/PageDown 操作後，若播放暫停或停止界面不更新的問題，將評論載入狀態加入週期性重繪條件
- 🛠️ **評論模式切歌後評論重置**：修復評論模式下切歌時評論被重置、丟失用戶正在查看的內容的問題，評論模式下跳過評論重置
- 🛠️ **播放時標題字符丟失**：修復播放狀態下數字/英文開頭的歌曲標題丟失字符的問題（如「17歲」顯示為「1歲」），根本原因是 `►★▶■❚` 等 Unicode 符號在東亞終端中寬度模糊（1列或2列不一致），導致游標偏移覆蓋後續字符。將所有模糊寬度 Unicode 符號替換為寬度明確的 ASCII 字符：`►`→`>`、`★`→`*`、`▶`→`>>`、`■`→`||`、`❚`→`[]`

### 🔧 功能改進

- 🎨 **UI 符號統一為 ASCII**：播放狀態前綴 `>>`（播放）、`||`（暫停）、`[]`（停止），選中標記 `>`，收藏標記 `*`，目錄當前標記 `>>`，歌詞高亮標記 `>`，評論選中標記 `>`，消除東亞終端寬度歧義
- 📝 **空播放列表提示語優化**：將「未選擇可用的音樂目錄，已進入空列表模式，可按 o 開啟音樂目錄」改為「沒有可用的音樂目錄，已進入空播放列表模式，可按o開啟音樂目錄」，措辭更準確自然
- 📂 **無可用目錄設定預設目錄**：無可用目錄時自動設定預設音樂目錄（USERPROFILE/ter-music-rust/music）並添加到音樂目錄歷史，搜尋下載歌曲時使用預設音樂目錄而非當前工作目錄

---

## 版本 1.6.0（2026-05-04）

### 🎉 新功能

#### 多語言擴展與國際化重構
- ✨ **新增 6 種界面語言**：新增俄語（Русский）、法語（Français）、德語（Deutsch）、西班牙語（Español）、意大利語（Italiano）、葡萄牙語（Português），共支援 11 種語言
- ✨ **全模組國際化**：所有面向用戶的文字（UI 界面、命令列幫助、錯誤訊息、對話框標題）均已國際化，包括 `ui.rs`、`main.rs`、`search.rs`、`audio.rs`、`config.rs`、`playlist.rs`
- ✨ **語言包集中管理**：新增 `langs.rs` 模組，將所有語言的翻譯文本集中到一個檔案管理，包含 `LangTexts` 結構體和 11 個語言靜態實例
- ✨ **全域語言存取器**：提供 `langs::global_texts()` 函式，供非 UI 模組（search.rs / audio.rs / config.rs / playlist.rs）執行緒安全地取得當前語言翻譯文本
- ✨ **AI 提示詞多語言**：每種語言的 AI 歌曲資訊查詢提示詞均使用對應語言輸出，確保回覆語言與界面語言一致

### 🔧 功能改進

- 🌐 **CLI 幫助國際化**：命令列 `-h` 幫助資訊跟隨界面語言顯示
- 🌐 **錯誤訊息國際化**：音訊錯誤、搜尋錯誤、設定錯誤、目錄錯誤等訊息均跟隨界面語言
- 🌐 **對話框標題國際化**：macOS / Linux 資料夾選擇對話框標題跟隨界面語言
- ♻️ **程式碼解耦**：各模組不再硬編碼中文文字，統一透過 `self.t()` 或 `langs::global_texts()` 讀取翻譯文本

### 🐞 錯誤修復

- 🛠️ **評論模式鍵盤焦點修復**：修復在線上搜尋/聚合搜尋/歌單搜尋模式下，按 `c` 查看評論後，上下鍵仍控制歌曲列表而非評論列表的問題
- 🛠️ **Linux目錄選擇對話框修復**：修復 Linux 下按 `o` 無法彈出圖形化目錄選擇對話框的問題，正確處理 raw mode 與圖形對話框的衝突
- 🛠️ **UTF-8日誌切片安全修復**：修復多位元組 UTF-8 字串按位元組切片可能導致程式崩潰的問題，改為按字元安全截斷
- 🛠️ **設定檔格式化修復**：修復設定檔錯誤訊息中 `replace("{}")` 雙重替換導致第二個佔位符無法正確替換的問題

---


## 版本 1.5.0（2026-04-30）

### 🎉 新功能

#### 線上播放清單搜尋
- ✨ **播放清單搜尋入口**：按 `p` 直接搜尋線上播放清單
- ✨ **播放清單內容瀏覽**：進入播放清單後可瀏覽歌曲並快速播放
- ✨ **快取命中播放**：在線上搜尋/聚合搜尋/播放清單搜尋中，若歌曲已存在於本機或命中下載快取，跳過重複下載直接播放
- ✨ **歌詞去重下載**：在線上搜尋/聚合搜尋/播放清單搜尋中，若歌曲已存在於本機或命中下載快取，不重複下載歌詞檔案

### 🔧 改進

- 🎵 **歌詞策略最佳化**：播放時歌詞採用「聚合優先，常規回退」以提高匹配準確度
- 🎯 **搜尋焦點最佳化**：按 `s/n/j/p` 時預設聚焦搜尋輸入框，可立即輸入
- 🎯 **搜尋到列表互動最佳化**：按 Enter 或點擊歌曲開始播放後，焦點切換至列表，鍵盤快速鍵不再進入搜尋框
- 🎯 **線上列表風格一致性**：線上/聚合/播放清單搜尋檢視中，選中游標和播放標記分離，間距與本機播放清單風格對齊
- 🎲 **線上隨機播放一致性最佳化**：隨機播放模式下，線上搜尋和聚合搜尋結果支援與播放清單播放一致的隨機自動下一首行為
- 🛡️ **線上自動跳過保護**：新增線上自動跳過限速；若 3 秒內連續 5 次自動跳過，播放自動停止，避免不可播放曲目導致失控跳過

### 🐞 錯誤修復

- 🛠️ **歌詞優先順序修復**：修復線上搜尋/聚合搜尋/播放清單搜尋流程中歌詞下載優先順序不正確的問題
- 🛠️ **線上自動播放索引修復**：修復播放時移動游標會使自動下一首從游標位置而非實際播放歌曲繼續的問題
- 🛠️ **搜尋中空格鍵輸入修復**：修復列表焦點狀態下空格鍵被寫入搜尋框並意外更改/清除結果的問題
- 🛠️ **網路搜尋初始焦點修復**：修復按 `n` 進入網路搜尋時缺少初始輸入焦點的問題
- 🛠️ **線上隨機播放缺失行為修復**：修復隨機播放模式在線上搜尋/聚合搜尋結果列表中未生效的問題
- 🛠️ **線上自動下一首過早停止修復**：修復首個線上曲目不可播放時播放可能過早停止的問題，透過僅計算實際自動下一首嘗試並在成功播放後重置時間窗口

---

## 版本 1.4.0（2026-04-28）


### 🎉 新功能

#### 聚合搜尋作為備用
- ✨ **歌曲聚合搜尋**：線上搜尋失敗時，可使用聚合搜尋按歌曲標題/歌手搜尋並下載歌曲
- ✨ **歌詞聚合搜尋**：若本機無歌詞且線上搜尋失敗，系統會自動透過聚合搜尋按歌曲標題/歌手搜尋歌詞並下載
- ✨ **無縫體驗**：搜尋和下載在背景進行，不阻塞 UI

#### GitHub Token 設定
- ✨ **自訂 GitHub Token**：按 `g` 輸入自己的 GitHub Token，儲存至設定檔
- ✨ **預設回退**：未設定時自動使用預設 Token
- ✨ **身分識別**：使用自己的 Token 提交歌曲資訊討論時，會顯示您的 GitHub 身分

### 🔧 改進

- 🔍 **新增設定項**：`github_token`（GitHub Token，留空使用預設）

---

## 版本 1.3.0（2026-04-26）

### 🎉 新功能

#### 自訂 AI API 端點
- ✨ **OpenAI 相容 API**：支援任何 OpenAI 相容 API 進行歌曲資訊查詢（DeepSeek、OpenRouter、OpenAI 等）
- ✨ **3 步設定**：按 `k` 依序輸入 API 基礎 URL → API Key → 模型名稱
- ✨ **免費回退**：未設定 API Key 時自動使用 OpenRouter 免費模型（minimax/minimax-m2.5:free）
- ✨ **直接查詢**：按 `i` 直接查詢歌曲資訊 — 無需預先設定 API Key

### 🔧 改進

- 🔍 **提示詞最佳化**：將「歌曲含義」重新命名為「歌詞含義」，「趣聞」重新命名為「軼事」
- 🔍 **設定欄位重新命名**：`deepseek_api_key` → `api_key`（向下相容現有設定檔）
- 🔍 **新增設定項**：`api_base_url`（API 端點，預設 DeepSeek）、`api_model`（模型名稱，預設 deepseek-v4-flash）

---

## 版本 1.2.0（2026-04-24）

### 🎉 新功能

#### 歌曲資訊查詢
- ✨ **DeepSeek 查詢**：按 `i` 透過 DeepSeek 串流查詢詳細歌曲資訊
- ✨ **串流輸出**：結果逐字顯示，無需等待完整生成
- ✨ **13 個資訊類別**：演出者、藝人詳情、詞曲創作與製作、發行日期、專輯（含曲目列表）、創作背景、歌曲含義、音樂風格、商業表現、獲獎情況、影響與評價、翻唱與使用、趣聞
- ✨ **多語言回應**：回應語言跟隨 UI 語言（簡中/繁中/英/日/韓）
- ✨ **API Key 管理**：按 `k` 輸入 DeepSeek API Key，或透過 `DEEPSEEK_API_KEY` 環境變數設定

#### 酷狗音樂來源
- ✨ **酷狗音樂**：新增酷狗作為第三個搜尋/下載平台
- ✨ **3 平台搜尋**：優先順序為酷我 → 酷狗 → 網易雲
- ✨ **減少 VIP 限制**：酷狗提供更多免費下載資源
- ✨ **MD5 簽章認證**：酷狗下載連結使用 MD5 簽章，成功率更高

### 🔧 改進

#### 歌曲資訊提示詞最佳化
- 🔍 **無前言**：回應不再包含問候或自我介紹
- 🔍 **無編號列表**：輸出內容不再使用編號列表格式
- 🔍 **藝人詳情**：新增包含詳細藝人資訊的類別（國籍、出生地、出生日期等）
- 🔍 **專輯曲目列表**：專輯部分現包含完整曲目列表

### 💻 技術細節

#### 依賴更新
- ➕ 新增 `md5` 依賴（酷狗音樂 API 簽章）

#### 資料結構
- ♻️ 在 `OnlineSong` 中新增 `hash` 欄位（酷狗使用 hash 識別歌曲）
- ♻️ 新增 `MusicSource::Kugou` 列舉變體
- ♻️ 新增酷狗 JSON 解析結構體

---

## 版本 1.1.0（2026-04-17）

### 🎉 新功能

#### 歌詞顯示系統
- ✨ **雙面板佈局**：左側歌曲列表，右側歌詞
- ✨ **自動歌詞下載**：缺少歌詞時從網路下載
- ✨ **智慧匹配**：自動尋找帶標記的歌詞檔案名
- ✨ **多編碼支援**：支援 UTF-8 和 GBK 歌詞檔案
- ✨ **歌詞捲動**：隨播放進度自動捲動
- ✨ **高亮顯示**：當前歌詞行以黃色高亮
- ✨ **歌名顯示**：歌詞標題顯示當前歌曲名稱

#### 使用者體驗
- ✨ **播放時歌詞自動匹配/下載**
- ✨ **統一風格**：播放清單和歌詞區域使用一致的黃色風格
- ✨ **動態標題**：歌詞標題隨當前歌曲更新
- ✨ **語言切換**支援
- ✨ **主題切換**支援

### 🚀 效能最佳化

#### UI 渲染
- ⚡ **更流暢的進度條更新**
- ⚡ **透過最佳化事件迴圈減少重繪**
- ⚡ **鎖最佳化**提高回應性

#### 歌詞載入
- ⚡ **智慧快取**：載入後快取以避免重複解析
- ⚡ **延遲載入**：僅在需要時載入
- ⚡ **批次重新命名支援**：清理歌詞檔案名標記

### 🎨 UI 改進

#### 視覺更新
- 🎨 **統一配色方案**：播放清單和歌詞區域
- 🎨 **分割佈局**：更好的空間利用
- 🎨 **中間分隔線**：更清晰的視覺結構

#### 資訊顯示
- 📊 **可見播放清單範圍**顯示
- 📊 **歌詞標題中的歌曲名稱**
- 📊 **更頻繁的進度條更新**

### 🔧 功能改進

#### 歌詞管理
- 🔍 **智慧查找**多種歌詞檔案名模式
- 🔍 **檔案映射**確保歌曲與歌詞一對一匹配

#### 錯誤處理
- 🛡️ 下載失敗時**友善提示**
- 🛡️ 歌詞檔案**自動編碼偵測**
- 🛡️ **10 秒網路超時**避免長時間阻塞等待

### 🐛 錯誤修復

- 🐛 修復檔案名標記導致的歌詞不匹配
- 🐛 修復歌詞下載中的編碼問題
- 🐛 修復重繪時的 UI 閃爍
- 🐛 修復進度條更新延遲

### 💻 技術細節

#### 依賴更新
- ➕ 新增 `reqwest` HTTP 用戶端
- ➕ 新增 `urlencoding` 支援
- ➕ 新增 `encoding_rs` 轉碼支援

#### 重構
- ♻️ 最佳化事件迴圈邏輯
- ♻️ 改進歌詞載入流程
- ♻️ 統一顏色常數定義

---

## 版本 1.0.0（2026-04-09）

### 核心功能
- 🎵 音訊播放（多格式）
- 📋 播放清單管理
- 🎹 播放控制
- 🔊 音量控制
- 🎲 播放模式切換
- 📂 資料夾瀏覽

---

## 📄 AI 協助

GLM、Codex

## 📄 授權

MIT License

## 🤝 貢獻

歡迎提交 Issue 和 Pull Request！
