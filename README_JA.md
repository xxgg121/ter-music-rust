<div align="center">

[简体中文](README.md) | [繁體中文](README_TC.md) | [English](README_EN.md) | [日本語](README_JA.md) | [한국어](README_KO.md) | [Русский](README_RU.md) | [Français](README_FR.md) | [Deutsch](README_DE.md) | [Español](README_ES.md) | [Italiano](README_IT.md) | [Português](README_PT.md)

# 🎵 Ter-Music-Rust - ターミナル音楽プレーヤー 🎵

</div>

Rust で実装された、シンプルで実用的なターミナル向け音楽プレーヤーです。ローカル/ネットワークの楽曲検索とダウンロード、歌詞の自動取得と表示、コメント閲覧、言語・テーマ切り替えなどを備え、Windows・Linux・MacOS をサポートします。

![preview1](preview1.png)

![preview2](preview2.png)

![preview3](preview3.png)

![preview4](preview4.png)

![preview5](preview5.png)

![preview6](preview6.png)

## ✨ 機能

### 🎵 オーディオ再生
- **10 種類の音声フォーマット対応**: MP3、WAV、FLAC、OGG、OGA、Opus、M4A、AAC、AIFF、APE
- **再生コントロール**: 再生/一時停止/停止、前の曲/次の曲
- **シーク**: 5 秒 / 10 秒の高速シーク
- **進捗バーシーク**: 進捗バーをクリックして任意位置へジャンプ
- **音量調整**: 0-100 のリアルタイム調整、音量バーのクリックにも対応

### 🔄 5 つの再生モード
| キー | モード | 説明 |
|------|------|------|
| `1` | 単曲再生 | 現在の曲が終了したら停止 |
| `2` | 単曲リピート | 現在の曲を繰り返し再生 |
| `3` | 順次再生 | 順番に再生し、最後で停止 |
| `4` | リストループ | プレイリスト全体を繰り返し再生 |
| `5` | シャッフル再生 | 曲をランダムに選択 |

### 📜 歌詞システム
- **ローカル歌詞読み込み**: 対応する `.lrc` ファイルを自動検索
- **歌詞エンコーディング判定**: UTF-8 / GBK を自動判定
- **オンライン自動取得**: ローカルに歌詞がない場合、非同期でバックグラウンド取得
- **スクロールハイライト**: 現在行を `►` でハイライトし、自動中央スクロール
- **歌詞位置ジャンプ**: 歌詞領域のドラッグ/マウスホイールでタイムスタンプへ移動

### 🔍 検索
- **ローカル検索**: `s` で現在の音楽ディレクトリ内を検索
- **オンライン検索**: `n` でキーワードによるネット検索
- **アグリゲート検索**：`j` を押して入力し、キーワードに照らして曲を検索する
- **プレイリスト検索**：`p` を押して入力し、キーワードでオンラインのプレイリストを検索します
- **ページ切り替え**: `PgUp` / `PgDn` で結果を切り替え
- **オンラインダウンロード**: オンライン検索結果を選択して `Enter` で現在ディレクトリへ保存（進捗表示あり）

### 🤖 楽曲情報
- **スマート検索**: `i` キーで現在の楽曲の詳細情報を検索、任意の OpenAI 互換 API に対応
- **ストリーミング出力**: 検索結果が1文字ずつストリーミング表示され、全生成の待機不要
- **豊富な情報**: 歌手詳細、作詞作曲、収録アルバム（トラックリスト付き）、制作背景、歌詞の意味、音楽スタイル、興味深い逸話13項目をカバー
- **多言語対応**: 応答言語が UI 言語設定に追従（簡中/繁中/英/日/韓）
- **カスタム API**: `k` キーで3ステップ設定（API ベース URL → API Key → モデル名）— DeepSeek、OpenRouter、AIHubMix など任意の OpenAI 互換エンドポイントに対応
- **無料フォールバック**: API Key 未設定時は OpenRouter の無料モデル（minimax/minimax-m2.5:free）を自動使用

### ⭐ お気に入り
- **追加/削除**: `f` で現在曲のお気に入り状態を切り替え
- **お気に入り一覧**: `v` で表示（`★` マーカー付き）
- **ディレクトリ跨ぎ再生**: 曲が現在ディレクトリ外でも自動で切り替えて再生
- **お気に入り削除**: お気に入り一覧で `d`

### 💬 コメント
- **曲コメント閲覧**: `c` で現在曲のコメントを表示
- **コメント詳細**: `Enter` で一覧/詳細を切り替え（詳細は全文表示）
- **返信表示**: 返信先コメント本文・ニックネーム・時刻を表示
- **コメントページング**: `PgUp` / `PgDn`、1 ページ 20 件
- **バックグラウンド読み込み**: UI をブロックせず別スレッドで取得

### 📂 ディレクトリ管理
- **音楽ディレクトリ選択**: `o` でフォルダ選択ダイアログを開く（初回読み込み成功後は自動再生開始）
- **履歴表示**: `m` で開いたディレクトリ履歴を表示し、素早く切り替え
- **現在ディレクトリ表示**: `▶` が現在使用中のディレクトリ
- **履歴削除**: 履歴画面で `d`

### 🌐 多言語 UI
`l` で 11 言語を循環切り替え:

| 言語 | 設定値 |
|------|--------|
| 中国語簡体字 | `sc` |
| 中国語繁体字 | `tc` |
| English | `en` |
| 日本語 | `ja` |
| 한국어 | `ko` |
| Русский | `ru` |
| Français | `fr` |
| Deutsch | `de` |
| Español | `es` |
| Italiano | `it` |
| Português | `pt` |

### 🎨 マルチテーマ UI
`t` で 4 テーマを循環切り替え:

| テーマ | スタイル |
|------|------|
| Neon | ネオン調 |
| Sunset | 夕日ゴールド調 |
| Ocean | 深海ブルー調 |
| GrayWhite | コンソール風グレースケール |

### 🖱️ マウス操作
- **プレイリストクリック**: 曲を直接再生
- **進捗バークリック**: 指定位置へジャンプ
- **音量バークリック**: 音量調整
- **歌詞ドラッグ**: 左ドラッグで該当時刻へ移動
- **歌詞ホイール**: 上下スクロールで前後の歌詞行へ移動
- **検索結果クリック**: ローカル検索は再生、オンライン検索はダウンロード
- **コメントクリック**: 詳細を表示

### 📊 波形ビジュアライザー
- 再生中に実際の RMS 音量に基づく動的波形を表示
- EMA 平滑化でより自然な見た目
- 一時停止中は波形を固定

### ⚙️ 設定の永続化
設定は `USERPROFILE/ter-music-rust/config.json` に保存され、自動保存/復元されます:

| 設定項目 | 説明 |
|--------|------|
| `music_directory` | 最後に開いた音楽ディレクトリ |
| `play_mode` | 再生モード |
| `current_index` | 最後に再生した曲のインデックス（再開用） |
| `volume` | 音量 (0-100) |
| `favorites` | お気に入り一覧 |
| `dir_history` | ディレクトリ履歴 |
| `api_key` | API Key（楽曲情報検索用、旧フィールド `deepseek_api_key` と互換） |
| `api_base_url` | API ベース URL（デフォルト: `https://api.deepseek.com/`） |
| `api_model` | AI モデル名（デフォルト: `deepseek-v4-flash`） |
| `github_token` | GitHub Token（楽曲情報 Discussion 投稿用、空欄でデフォルト Token を使用） |
| `theme` | テーマ名 |
| `language` | UI 言語（`sc` / `tc` / `en` / `ja` / `ko` / `ru` / `fr` / `de` / `es` / `it` / `pt`） |

**自動保存のタイミング**: 曲切り替え、テーマ変更、言語変更、お気に入り変更、30 秒ごと、終了時（Ctrl+C 含む）

---

## 🚀 クイックスタート

### 1. Rust のインストール

```powershell
# 方法 1: winget（推奨）
winget install Rustlang.Rustup

# 方法 2: 公式インストーラー
# https://rustup.rs/ にアクセスしてインストール
```

インストール確認:

```powershell
rustc --version
cargo --version
```

### 2. プロジェクトをビルド

```powershell
cd <project-directory>

# 方法 1: ビルドスクリプト（推奨）
build-win.bat

# 方法 2: Cargo
cargo build --release
```

### 3. 実行

```powershell
# 方法 1: cargo run
cargo run --release

# 方法 2: 実行ファイルを直接起動
.\target\release\ter-music-rust.exe

# 方法 3: 音楽ディレクトリを指定
.\target\release\ter-music-rust.exe -o d:\Music
cargo run --release -- -o d:\Music
```

**ディレクトリ読み込み優先順位**: コマンドライン `-o` > 設定ファイル > フォルダ選択ダイアログ

---

## 🎮 キーボードショートカット

### メイン画面

| キー | 動作 |
|------|------|
| `↑/↓` | 曲を選択 |
| `Enter` | 選択曲を再生 |
| `Space` | 再生/一時停止 |
| `Esc` | 再生停止（コメント画面では歌詞へ戻る） |
| `←/→` | 前の曲/次の曲 |
| `[` | 5 秒巻き戻し |
| `]` | 5 秒早送り |
| `,` | 10 秒巻き戻し |
| `.` | 10 秒早送り |
| `+/-` | 音量増減（ステップ 5） |
| `1-5` | 再生モード切り替え |
| `o` | 音楽ディレクトリを開く |
| `s` | ローカル曲検索 |
| `n` | オンライン曲検索 |
| `j` | アグリゲート検索 |
| `p` | オンラインプレイリスト検索 |
| `i` | 楽曲情報検索 |
| `f` | お気に入り/解除 |
| `v` | お気に入り一覧 |
| `m` | ディレクトリ履歴 |
| `h` | ヘルプ情報を表示 |
| `c` | 曲コメント |
| `l` | UI 言語切り替え |
| `t` | テーマ切り替え |
| `k` | API エンドポイント設定 |
| `g` | GitHub Token 設定 |
| `q` | 終了 |

### 検索画面

| キー | 動作 |
|------|------|
| 文字入力 | キーワード入力 |
| `Backspace` | 文字削除 |
| `Enter` | 検索/再生/ダウンロード |
| `↑/↓` | 結果選択 |
| `PgUp/PgDn` | ページ切り替え（オンライン検索） |
| `s/n/j` | ローカル/オンライン/アグリゲート検索切り替え |

| `Esc` | 検索終了 |

### お気に入り画面

| キー | 動作 |
|------|------|
| `↑/↓` | 曲を選択 |
| `Enter` | 選択曲を再生 |
| `d` | お気に入り削除 |
| `Esc` | プレイリストへ戻る |

### ディレクトリ履歴画面

| キー | 動作 |
|------|------|
| `↑/↓` | ディレクトリを選択 |
| `Enter` | 選択ディレクトリへ切り替え |
| `d` | 履歴削除 |
| `Esc` | プレイリストへ戻る |

### コメント画面

| キー | 動作 |
|------|------|
| `↑/↓` | コメント選択 |
| `Enter` | 一覧/詳細切り替え |
| `PgUp/PgDn` | ページ切り替え |
| `Esc` | 歌詞画面へ戻る |

### 楽曲情報画面

| キー | 動作 |
|------|------|
| `↑/↓` | 情報をスクロール |
| `i` | 楽曲情報を再取得 |
| `Esc` | 歌詞画面へ戻る |

### プレイリスト検索画面

| キー | 動作 |
|------|------|
| 文字入力 | プレイリスト検索キーワード入力 |
| `Backspace` | 文字削除 |
| `Enter` | 検索/プレイリストへ入る/再生ダウンロード |
| `↑/↓` | プレイリストまたは楽曲を選択 |
| `PgUp/PgDn` | ページ切り替え |
| `Esc` | 1つ前に戻る/検索終了 |

### ヘルプ画面


| キー | 動作 |
|------|------|
| `↑/↓` | ヘルプをスクロール |
| `Esc` | 歌詞画面へ戻る |

---

## 📦 インストールとビルド

### システム要件

- **OS**: Windows 10/11
- **Rust**: 1.70+
- **ターミナル**: Windows Terminal（推奨）/ CMD / PowerShell
- **ウィンドウサイズ**: 80×25 以上を推奨

### 方法 1: MSVC ツールチェーン（互換性最優先、サイズ大）

```powershell
# 1. Rust をインストール
winget install Rustlang.Rustup

# 2. Build Tools をインストール
winget install Microsoft.VisualStudio.2022.BuildTools
# インストーラー起動 -> "C++ によるデスクトップ開発" を選択 -> インストール

# 3. ターミナルを再起動してビルド
cargo build --release
```

### 方法 2: GNU ツールチェーン（推奨、軽量 約 300 MB）

```powershell
# 1. Rust をインストール
winget install Rustlang.Rustup

# 2. MSYS2 をインストール
winget install MSYS2.MSYS2
# MSYS2 ターミナルで実行:
pacman -Syu
pacman -S mingw-w64-x86_64-toolchain

# 3. PATH を追加（管理者 PowerShell）
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\msys64\mingw64\bin", "Machine")

# 4. ツールチェーンを切り替えてビルド
rustup default stable-x86_64-pc-windows-gnu
cargo build --release
```

> GNU ツールチェーンでビルドした実行ファイルは、実行時に以下 DLL が必要な場合があります:
> `libgcc_s_seh-1.dll`、`libstdc++-6.dll`、`libwinpthread-1.dll`

### 方法 3: Windows 上で Linux 向けクロスコンパイル

`cargo-zigbuild` + `zig` をリンカーとして使用します。Linux 環境や VM を用意せずにクロスコンパイル可能です。

```powershell
# 1. zig をインストール（いずれか）
# A: pip（推奨）
pip install ziglang

# B: MSYS2
pacman -S mingw-w64-x86_64-zig

# C: 手動ダウンロード
# https://ziglang.org/download/ から取得して PATH に追加

# 2. cargo-zigbuild をインストール
cargo install cargo-zigbuild

# 3. Linux ターゲットを追加
rustup target add x86_64-unknown-linux-gnu

# 4. Linux sysroot を準備（ALSA ヘッダー/ライブラリ）
# プロジェクトには linux-sysroot/ が同梱済み
# 手動準備する場合は Debian/Ubuntu から以下をコピー:
#   /usr/include/alsa/ -> linux-sysroot/usr/include/alsa/
#   /usr/lib/x86_64-linux-gnu/libasound.so* -> linux-sysroot/usr/lib/x86_64-linux-gnu/

# 5. ビルド
build-linux.bat

# または手動実行:
cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.34
```

**出力**: `target/x86_64-unknown-linux-gnu/release/ter-music-rust`

**Linux へデプロイ**:

```bash
# 1. Linux ホストへコピー
scp ter-music-rust user@linux-host:~/

# 2. 実行権限を付与
chmod +x ter-music-rust

# 3. ALSA ランタイムをインストール
sudo apt install libasound2

# 4. 実行
./ter-music-rust -o /path/to/music
```

> `build-linux.bat` は `PKG_CONFIG_PATH`、`PKG_CONFIG_ALLOW_CROSS`、`RUSTFLAGS` などを自動設定します。
> ターゲット `x86_64-unknown-linux-gnu.2.34` の `.2.34` は最小 glibc バージョンを示し、古い Linux との互換性を高めます。

### Linux パッケージング（DEB / RPM）

Linux 上でビルド/パッケージングする場合は、以下を使用します:

```bash
# 1) RPM
./build-rpm.sh

# debuginfo RPM を生成（任意）
./build-rpm.sh --with-debuginfo

# 2) DEB
./build-deb.sh

# debug symbols DEB を生成（任意）
./build-deb.sh --with-debuginfo

# dpkg-source 準拠のソースパッケージを生成（.dsc/.orig.tar/.debian.tar）
./build-deb.sh --with-source

# debuginfo + ソースパッケージを同時生成
./build-deb.sh --with-debuginfo --with-source
```

既定の出力先:
- `dist/rpm/`: RPM / SRPM
- `dist/deb/`: DEB / ソースパッケージ

> スクリプトは `Cargo.toml` の `name` と `version` を読み取り、パッケージ名を自動生成します。

### 方法 4: Windows 上で MacOS 向けクロスコンパイル

`cargo-zigbuild` + `zig` + MacOS SDK を使用します。MacOS の音声は CoreAudio を使用するため、SDK ヘッダーが必要です。

**事前準備:**

```powershell
# 1. zig をインストール（Linux クロスコンパイルと同様）
pip install ziglang

# 2. cargo-zigbuild をインストール
cargo install cargo-zigbuild

# 3. LLVM/Clang をインストール（bindgen 用の libclang.dll を提供）
# A: MSYS2
pacman -S mingw-w64-x86_64-clang

# B: 公式 LLVM
winget install LLVM.LLVM

# 4. MacOS ターゲットを追加
rustup target add x86_64-apple-darwin aarch64-apple-darwin
```

**MacOS SDK の準備:**

`MacOSX13.3.sdk.tar.xz` を `macos-sysroot` に展開してください。  
プロジェクトには `macos-sysroot/` が同梱されています（[macosx-sdks](https://github.com/joseluisq/macosx-sdks) 由来）。

再取得する場合:

```powershell
# A: GitHub から事前パッケージ済み SDK を取得（推奨、約 56 MB）
# ミラー: https://ghfast.top/https://github.com/joseluisq/macosx-sdks/releases/download/13.3/MacOSX13.3.sdk.tar.xz
curl -L -o MacOSX13.3.sdk.tar.xz https://github.com/joseluisq/macosx-sdks/releases/download/13.3/MacOSX13.3.sdk.tar.xz
mkdir macos-sysroot
tar -xf MacOSX13.3.sdk.tar.xz -C macos-sysroot --strip-components=1
del MacOSX13.3.sdk.tar.xz

# B: MacOS システムからコピー
scp -r mac:/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk ./macos-sysroot
```

> SDK 出典: https://github.com/joseluisq/macosx-sdks
> CoreAudio、AudioToolbox、AudioUnit、CoreMIDI、OpenAL、IOKit などのヘッダーを含みます。

**ビルド:**

```powershell
# ビルドスクリプトを使用（環境変数を自動設定）
build-mac.bat

# または手動実行:
$env:LIBCLANG_PATH = "C:\msys64\mingw64\bin"      # libclang.dll のあるディレクトリ
$env:COREAUDIO_SDK_PATH = "./macos-sysroot"         # MacOS SDK パス（スラッシュ区切り）
$env:SDKROOT = "./macos-sysroot"                    # zig リンカーがシステムライブラリ探索に使用
$FW = "./macos-sysroot/System/Library/Frameworks"
$env:BINDGEN_EXTRA_CLANG_ARGS = "--target=x86_64-apple-darwin -isysroot ./macos-sysroot -F $FW -iframework $FW -I ./macos-sysroot/usr/include"
cargo zigbuild --release --target x86_64-apple-darwin   # Intel Mac
# Apple Silicon は target と clang args の x86_64 を aarch64 に置換
cargo zigbuild --release --target aarch64-apple-darwin  # Apple Silicon
```

**出力:**
- `target/x86_64-apple-darwin/release/ter-music-rust` — Intel Mac
- `target/aarch64-apple-darwin/release/ter-music-rust` — Apple Silicon (M1/M2/M3/M4)

**MacOS へデプロイ:**

```bash
# 1. MacOS ホストへコピー
scp ter-music-rust user@mac-host:~/

# 2. 実行権限を付与
chmod +x ter-music-rust

# 3. 未署名バイナリの実行許可
xattr -cr ter-music-rust

# 4. 実行（追加の音声ライブラリは不要）
./ter-music-rust -o /path/to/music
```

> 注意: MacOS クロスコンパイルには MacOS SDK ヘッダーが必要です（本プロジェクトには `macos-sysroot/` を同梱）。
> あわせて `libclang.dll` も必要です（MSYS2 または LLVM で導入）。

### ツールチェーンの切り替え

```powershell
# 現在のツールチェーンを表示
rustup show

# MSVC へ切り替え
rustup default stable-x86_64-pc-windows-msvc

# GNU へ切り替え
rustup default stable-x86_64-pc-windows-gnu
```

### 中国国内向け Cargo ミラー（高速化）

`~/.cargo/config` を作成または編集:

```toml
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
```

---

## 🛠️ プロジェクト構成

```text
src/
├── main.rs       # エントリーポイント（引数解析、初期化、設定復元/保存）
├── defs.rs       # 共通定義（PlayMode/PlayState 列挙、MusicFile/Playlist 構造体）
├── audio.rs      # 音声制御（rodio ラッパー、再生/停止/シーク/音量/進捗）
├── analyzer.rs   # 音声解析（リアルタイム RMS、EMA 平滑化、波形表示）
├── playlist.rs   # プレイリスト管理（ディレクトリ走査、再生時間並列取得、フォルダ選択）
├── lyrics.rs     # 歌詞解析（LRC、ローカル検索、エンコード判定、バックグラウンド取得）
├── search.rs     # ネット検索/取得（酷我+酷狗+網易検索、ダウンロード、コメント取得、楽曲情報ストリーミング検索）
├── config.rs     # 設定管理（JSON シリアライズ、8 項目の永続化）
└── ui.rs         # UI（ターミナル描画、イベント処理、マルチビュー、テーマ/言語）
```

### 技術スタック

| 依存ライブラリ | バージョン | 用途 |
|------|------|------|
| [rodio](https://github.com/RustAudio/rodio) | 0.19 | 音声デコードと再生（Pure Rust） |
| [crossterm](https://github.com/crossterm-rs/crossterm) | 0.28 | ターミナル UI 制御 |
| [reqwest](https://github.com/seanmonstar/reqwest) | 0.12 | HTTP リクエスト |
| [serde](https://github.com/serde-rs/serde) + serde_json | 1.0 | JSON シリアライズ |
| [rayon](https://github.com/rayon-rs/rayon) | 1.10 | 再生時間の並列取得 |
| [encoding_rs](https://github.com/hsivonen/encoding_rs) | 0.8 | GBK 歌詞デコード |
| [walkdir](https://github.com/BurntSushi/walkdir) | 2.5 | 再帰ディレクトリ走査 |
| [rand](https://github.com/rust-random/rand) | 0.8 | シャッフル再生 |
| [unicode-width](https://github.com/unicode-rs/unicode-width) | 0.2 | CJK 表示幅計算 |
| [chrono](https://github.com/chronotope/chrono) | 0.4 | コメント時刻フォーマット |
| [ctrlc](https://github.com/Detegr/rust-ctrlc) | 3.4 | Ctrl+C シグナル処理 |
| [md5](https://github.com/johannhof/md5) | 0.7 | 酷狗音楽 API MD5 署名 |
| [winapi](https://github.com/retep998/winapi-rs) | 0.3 | Windows コンソール UTF-8 対応 |

### リリースビルド最適化

```toml
[profile.release]
opt-level = 3       # 最高最適化レベル
lto = true          # リンク時最適化
codegen-units = 1   # 単一 codegen unit で最適化向上
strip = true        # デバッグシンボルを削除
```

---

## Rust C 版との比較

| 項目 | Rust 版 | C 版 |
|------|-----------|--------|
| インストール容量 | ~200 MB (Rust) / ~300 MB (GNU) | ~7 GB (Visual Studio) |
| セットアップ時間 | 約 5 分 | 約 1 時間 |
| コンパイル速度 | ⚡ 高速 | 🐢 やや低速 |
| 依存管理 | ✅ Cargo により自動 | ❌ 手動設定 |
| メモリ安全性 | ✅ コンパイル時保証 | ⚠️ 手動管理が必要 |
| クロスプラットフォーム | ✅ 完全対応 | ⚠️ コード修正が必要 |
| 実行ファイルサイズ | ~2 MB | ~500 KB |
| メモリ使用量 | ~15-20 MB | ~10 MB |
| CPU 使用率 | < 1% | < 1% |

---

## 📊 パフォーマンス

| 指標 | 値 |
|------|------|
| UI 更新間隔 | 50ms |
| キー入力応答 | < 50ms |
| 歌詞ダウンロード | バックグラウンド実行（非ブロッキング） |
| ディレクトリ走査 | 再生時間の並列取得で 2-4 倍高速化 |
| 起動時間 | < 100ms |
| メモリ使用量 | ~15-20 MB |

---

## 🐛 トラブルシューティング

### ビルドエラー

```powershell
# Rust を更新
rustup update

# クリーンして再ビルド
cargo clean
cargo build --release
```

### `link.exe not found`

Visual Studio Build Tools をインストールしてください（上記「方法 1」参照）。

### `dlltool.exe not found`

MinGW-w64 ツールチェーン一式をインストールしてください（上記「方法 2」参照）。

### 実行時 DLL 不足（GNU ツールチェーン）

```powershell
Copy-Item "C:\msys64\mingw64\bin\libgcc_s_seh-1.dll" -Destination ".\target\release\"
Copy-Item "C:\msys64\mingw64\bin\libstdc++-6.dll" -Destination ".\target\release\"
Copy-Item "C:\msys64\mingw64\bin\libwinpthread-1.dll" -Destination ".\target\release\"
```

### 音声デバイスが見つからない

1. システムの音声デバイスが正常か確認
2. Windows の音量設定を確認
3. システムテスト音を再生して確認

### UI 表示が崩れる

- ターミナルサイズが 80×25 以上か確認
- Windows Terminal の使用を推奨
- CMD 利用時は CJK 対応フォントを選択

### オンライン検索/歌詞ダウンロード失敗

- ネットワーク接続を確認
- 一部楽曲は VIP 限定または配信終了の可能性あり
- 歌詞ファイルは標準 LRC 形式である必要があります

### 楽曲情報検索失敗

- API Key 未設定時は OpenRouter の無料モデルが自動使用されるため、手動設定は不要
- カスタムエンドポイントを使用する場合、`k` キーで API ベース URL、API Key、モデル名を順に入力
- 任意の OpenAI 互換 API に対応（DeepSeek、OpenRouter、AIHubMix など）
- 対応する API サービスへのネットワーク接続を確認

### 初回ビルドが遅い

初回は依存関係のダウンロードとコンパイルが発生するため時間がかかります。2 回目以降は大幅に高速化されます。

### Release のダウンロード
[ter-music-rust-win.zip](https://storage.deepin.org/thread/202605030941394786_ter-music-rust-win.zip "附件(Attached)")
[ter-music-rust-mac.zip](https://storage.deepin.org/thread/202605030941519730_ter-music-rust-mac.zip "附件(Attached)")
[ter-music-rust-linux.zip](https://storage.deepin.org/thread/20260503094157446_ter-music-rust-linux.zip "附件(Attached)") 
[ter-music-rust_deb.zip](https://storage.deepin.org/thread/202605030942036738_ter-music-rust_deb.zip "附件(Attached)")

---

## バージョン 1.6.0 (2026-05-04)

### 🎉 新機能

#### 多言語拡張と国際化リファクタリング
- ✨ **6 种のUI言語を追加**：ロシア語（Русский）、フランス語（Français）、ドイツ語（Deutsch）、スペイン語（Español）、イタリア語（Italiano）、ポルトガル語（Português）を追加し、合計11言語に対応
- ✨ **全モジュールの国際化**：ユーザー向けのすべてのテキスト（UI画面、コマンドラインヘルプ、エラーメッセージ、ダイアログタイトル）が国際化され、`ui.rs`、`main.rs`、`search.rs`、`audio.rs`、`config.rs`、`playlist.rs`を含む
- ✨ **言語パックの一元管理**：`langs.rs`モジュールを追加し、すべての言語の翻訳テキストを1つのファイルに集約管理。`LangTexts`構造体と11の言語静的インスタンスを含む
- ✨ **グローバル言語アクセサー**：非UIモジュール（search.rs / audio.rs / config.rs / playlist.rs）がスレッドセーフに現在の言語翻訳テキストを取得できる`langs::global_texts()`関数を提供
- ✨ **AIプロンプトの多言語化**：各言語のAI楽曲情報クエリプロンプトが対応言語で出力され、応答言語がUI言語と一致することを保証

### 🔧 機能改善

- 🌐 **CLIヘルプの国際化**：コマンドライン`-h`ヘルプ情報がUI言語設定に追従
- 🌐 **エラーメッセージの国際化**：音声エラー、検索エラー、設定エラー、ディレクトリエラーなどのメッセージがUI言語に追従
- 🌐 **ダイアログタイトルの国際化**：macOS / Linux フォルダ選択ダイアログのタイトルがUI言語に追従
- ♻️ **コードの分離**：各モジュールはハードコードされたテキスト文字列を含まず、すべて`self.t()`または`langs::global_texts()`を通じて翻訳テキストを読み取る

### 🐞 バグ修正

- 🛠️ **コメントモードのキーボードフォーカス修正**：オンライン検索/統合検索/プレイリスト検索モードで`c`を押してコメントを表示した後、上下キーがコメントリストではなく曲リストを操作していた問題を修正
- 🛠️ **Linuxフォルダ選択ダイアログ修正**：Linuxで`o`を押した際にグラフィカルフォルダ選択ダイアログが表示されない問題を修正。rawモードとグラフィカルダイアログの競合を正しく処理
- 🛠️ **UTF-8ログスライスの安全性修正**：マルチバイト UTF-8 文字列のバイト単位スライスがプログラムクラッシュを引き起こす可能性があった問題を修正。文字単位の安全な切り詰めに変更
- 🛠️ **設定ファイルフォーマット修正**：設定ファイルエラーメッセージの`replace("{}")`二重置換により、2番目のプレースホルダーが正しく置換されない問題を修正

---

## 📝 変更履歴

## バージョン 1.5.0 (2026-04-30)

### 🎉 新機能

#### オンラインプレイリスト検索
- ✨ **プレイリスト検索入口**: `p` キーでオンラインプレイリストを直接検索
- ✨ **プレイリスト内容の閲覧**: プレイリストに入ると楽曲一覧を閲覧し、すばやく再生可能
- ✨ **キャッシュヒット再生**: オンライン検索 / 統合検索 / プレイリスト検索でローカルに存在する、またはダウンロード済みキャッシュにヒットした場合、重複ダウンロードをスキップして直接再生
- ✨ **歌詞の重複ダウンロード防止**: オンライン検索 / 統合検索 / プレイリスト検索でローカルに存在する、またはダウンロード済みキャッシュにヒットした場合、歌詞ファイルの重複ダウンロードをスキップ

### 🔧 機能改善

- 🎵 **歌詞取得戦略の最適化**: 再生時は「統合歌詞を優先、通常歌詞をフォールバック」に変更し、歌詞一致精度を向上
- 🎯 **検索フォーカス改善**: `s/n/j/p` で検索モードに入った際、検索入力欄へ自動フォーカスされ、すぐ入力可能
- 🎯 **検索とリスト操作の改善**: Enter または曲クリックで再生開始後、フォーカスをリストへ切り替え、キー入力が検索欄へ誤入力されないように調整
- 🎯 **オンライン一覧表示の統一**: オンライン検索 / 統合検索 / プレイリスト検索で、選択カーソルと再生マークを分離し、ローカルプレイリストと同じ余白・表示形式に統一
- 🎲 **オンラインシャッフル再生の一貫性改善**: シャッフル再生モードで、オンライン検索 / 統合検索結果でもランダム切替に対応し、プレイリスト再生と同じ挙動に統一
- 🛡️ **オンライン自動次曲の保護**: オンライン検索 / 統合検索 / プレイリスト再生に統一適用し、3 秒以内に 5 回連続で自動スキップした場合は無通知で停止して、再生不可曲の連続発生によるリスクを低減

### 🐞 バグ修正

- 🛠️ **歌詞優先順の不具合を修正**: オンライン検索 / 統合検索 / プレイリスト検索で歌詞ダウンロード順序が正しくない問題を修正
- 🛠️ **オンライン自動次曲インデックス修正**: 再生中にカーソル移動した後、次曲がカーソル位置基準で再生される問題を修正し、実際の再生曲基準でモード通りに遷移
- 🛠️ **検索中スペース誤入力修正**: リストフォーカス時に Space が検索欄へ入力され、結果が崩れる問題を修正
- 🛠️ **ネット検索初期フォーカス修正**: `n` でネット検索に入った際、入力フォーカスが検索欄に当たらない問題を修正
- 🛠️ **オンラインシャッフル未反映修正**: シャッフル再生モードがオンライン検索 / 統合検索の結果一覧で有効にならない不具合を修正
- 🛠️ **オンライン自動次曲チェーン修正**: 順次再生 / 全曲ループで再生不可曲に遭遇した際に同じ曲を繰り返し再試行する不具合を修正し、再生モードに従って次曲へ継続遷移、実際の自動次曲のみをカウントして閾値到達時のみ無通知停止するよう改善

---

## バージョン 1.4.0 (2026-04-28)


### 🎉 新機能

#### 歌曲の統合検索で安心
- ✨ **統合検索による楽曲検索**：ネット上で見つからない場合、統合検索を使って楽曲名や歌手で検索し、ダウンロード可能  
- ✨ **統合検索による歌詞検索**：ローカルに歌詞がなく、ネットからダウンロードできない場合、自動的に統合検索で楽曲名や歌手を検索し、歌詞をダウンロード
- ✨ **シームレス体験**: 検索とダウンロードはバックグラウンドで実行され、UI をブロックしません

#### GitHub Token 設定
- ✨ **カスタム GitHub Token**: `g` キーで自分の GitHub Token を入力、設定ファイルに保存
- ✨ **デフォルトフォールバック**: 未設定時はデフォルト Token を自動使用
- ✨ **アイデンティティ認証**：自身のトークンを使用して楽曲情報のディスカッションに投稿すると、自分のGitHubアカウントとして表示されます

### 🔧 機能改善

- 🔍 **新規設定項目**: `github_token`（GitHub Token、空欄でデフォルト使用）

---

## バージョン 1.3.0 (2026-04-26)

### 🎉 新機能

#### カスタム AI API エンドポイント
- ✨ **OpenAI 互換 API**: 任意の OpenAI 互換 API で楽曲情報検索に対応（DeepSeek、OpenRouter、AIHubMix など）
- ✨ **3ステップ設定**: `k` キーで API ベース URL → API Key → モデル名を順に入力
- ✨ **無料フォールバック**: API Key 未設定時は OpenRouter の無料モデル（minimax/minimax-m2.5:free）を自動使用
- ✨ **直接検索**: `i` キーで事前の API Key 設定なしに楽曲情報を検索可能

### 🔧 機能改善

- 🔍 **プロンプト最適化**: 「楽曲の意味」→「歌詞の意味」、「面白い事実」→「興味深い逸話」に変更
- 🔍 **設定フィールド名変更**: `deepseek_api_key` → `api_key`（既存の設定ファイルと互換）
- 🔍 **新規設定項目**: `api_base_url`（API エンドポイント、デフォルトは DeepSeek）、`api_model`（モデル名、デフォルトは deepseek-v4-flash）

---

## バージョン 1.2.0 (2026-04-24)

### 🎉 新機能

#### 楽曲情報検索
- ✨ **DeepSeek 検索**: `i` キーで DeepSeek を使用して現在の楽曲情報をストリーミング検索
- ✨ **ストリーミング出力**: 検索結果が1文字ずつ表示され、全生成の待機不要
- ✨ **13 項目の情報**: 歌手、歌手詳細、作詞作曲、リリース日、収録アルバム（トラックリスト付き）、制作背景、楽曲の意味、音楽スタイル、商業成績、受賞記録、影響評価、カバーと使用例、面白い事実
- ✨ **多言語応答**: 応答言語が UI 言語に追従（簡中/繁中/英/日/韓）
- ✨ **API Key 管理**: `k` キーで DeepSeek API Key を入力、環境変数 `DEEPSEEK_API_KEY` でも設定可能

#### 酷狗音楽ソース
- ✨ **酷狗音楽**: 3つ目の検索/ダウンロードプラットフォームとして酷狗音楽を追加
- ✨ **3プラットフォーム検索**: 検索優先度は 酷我 → 酷狗 → 網易
- ✨ **VIP 制限の軽減**: 酷狗はより多くの無料ダウンロードリソースを提供
- ✨ **MD5 署名認証**: 酷狗ダウンロードリンクは MD5 署名を使用し、ダウンロード成功率が向上

### 🔧 機能改善

#### 楽曲情報プロンプト最適化
- 🔍 **冒頭なし**: 応答に挨拶や自己紹介を含めない
- 🔍 **番号なし**: 出力内容に番号付きリストを使用しない
- 🔍 **歌手詳細**: 国籍、出身地、生年月日などの詳細情報カテゴリを追加
- 🔍 **アルバムトラックリスト**: 収録アルバムに完全なトラックリストを含む

### 💻 技術詳細

#### 依存関係更新
- ➕ `md5` 依存関係を追加（酷狗音楽 API 署名用）

#### データ構造
- ♻️ `OnlineSong` に `hash` フィールドを追加（酷狗は hash で楽曲を識別）
- ♻️ `MusicSource::Kugou` 列挙子を追加
- ♻️ 酷狗 JSON 解析構造体を追加

---

## バージョン 1.1.0 (2026-04-17)

### 🎉 新機能

#### 歌詞表示システム
- ✨ **2 ペインレイアウト**: 左に曲リスト、右に歌詞表示
- ✨ **歌詞自動ダウンロード**: 歌詞未検出時にネットから自動取得
- ✨ **スマートマッチング**: マーク付き歌詞ファイル名を自動検出
- ✨ **複数エンコード対応**: UTF-8 / GBK の歌詞ファイルに対応
- ✨ **歌詞スクロール**: 再生進行に合わせて自動スクロール
- ✨ **ハイライト表示**: 現在行を黄色で強調表示
- ✨ **曲名表示**: 歌詞タイトルに現在曲名を表示

#### ユーザー体験
- ✨ **再生中の歌詞自動マッチ/取得**
- ✨ **統一スタイル**: プレイリストと歌詞エリアを同一の黄色系で統一
- ✨ **動的タイトル**: 現在曲に応じて歌詞タイトル更新
- ✨ **言語切り替え**対応
- ✨ **テーマ切り替え**対応

### 🚀 パフォーマンス最適化

#### UI 描画
- ⚡ **進捗バー更新の滑らかさ向上**
- ⚡ **イベントループ最適化による再描画削減**
- ⚡ **ロック最適化で応答性向上**

#### 歌詞読み込み
- ⚡ **スマートキャッシュ**で再解析を回避
- ⚡ **遅延読み込み**（必要時のみ）
- ⚡ **一括リネーム対応**（歌詞ファイル名マーカーの整理）

### 🎨 UI 改善

#### 視覚面
- 🎨 **統一配色**（プレイリスト/歌詞エリア）
- 🎨 **分割レイアウト**で空間効率向上
- 🎨 **中央セパレーター**で視認性向上

#### 情報表示
- 📊 **可視プレイリスト範囲**の表示
- 📊 **歌詞タイトルに曲名表示**
- 📊 **進捗バー更新頻度の向上**

### 🔧 機能改善

#### 歌詞管理
- 🔍 **多パターン歌詞名のスマート検索**
- 🔍 **曲と歌詞の 1 対 1 対応を保証**

#### エラーハンドリング
- 🛡️ **ダウンロード失敗時の分かりやすい通知**
- 🛡️ **歌詞ファイルの自動エンコード判定**
- 🛡️ **10 秒ネットワークタイムアウト**で長時間待機を回避

### 🐛 バグ修正

- 🐛 ファイル名マーカー起因の歌詞不一致を修正
- 🐛 歌詞ダウンロード時のエンコード問題を修正
- 🐛 再描画時の UI 点滅を修正
- 🐛 進捗バー更新遅延を修正

### 💻 技術詳細

#### 依存関係更新
- ➕ `reqwest` HTTP クライアントを追加
- ➕ `urlencoding` サポートを追加
- ➕ `encoding_rs` 文字コード変換サポートを追加

#### リファクタリング
- ♻️ イベントループロジックを最適化
- ♻️ 歌詞読み込みフローを改善
- ♻️ 色定数定義を統一

---

## バージョン 1.0.0 (2026-04-09)

### 基本機能
- 🎵 音声再生（複数フォーマット対応）
- 📋 プレイリスト管理
- 🎹 再生コントロール
- 🔊 音量調整
- 🎲 再生モード切り替え
- 📂 フォルダ参照

---

## 📄 支援

GLM、Codex

## 📄 ライセンス

MIT License

## 🤝 コントリビューション

Issue と Pull Request を歓迎します！
