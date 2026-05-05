<div align="center">

[简体中文](README.md) | [繁體中文](README_TC.md) | [English](README_EN.md) | [日本語](README_JA.md) | [한국어](README_KO.md) | [Русский](README_RU.md) | [Français](README_FR.md) | [Deutsch](README_DE.md) | [Español](README_ES.md) | [Italiano](README_IT.md) | [Português](README_PT.md)

# 🎵 Ter-Music-Rust - 终端音乐播放器 🎵

</div>

一个简洁实用的终端音乐播放器，使用 Rust 实现，支持本地/网络歌曲搜索下载、自动下载歌词显示、评论查看、多语言与主题切换等功能，支持Windows、Linux、MacOS系统。

![preview1](preview1.png)

![preview2](preview2.png)

![preview3](preview3.png)

![preview4](preview4.png)

![preview5](preview5.png)

![preview6](preview6.png)

## ✨ 功能特性

### 🎵 音频播放
- **10 种音频格式支持**：MP3、WAV、FLAC、OGG、OGA、Opus、M4A、AAC、AIFF、APE
- **播放控制**：播放/暂停/停止、上一曲/下一曲
- **快进快退**：5 秒 / 10 秒 快速跳转
- **进度条跳转**：鼠标点击进度条精确定位
- **音量控制**：0-100 实时调节，鼠标点击音量条调整

### 🔄 5 种播放模式
| 按键 | 模式 | 说明 |
|------|------|------|
| `1` | 单曲播放 | 播放完当前歌曲后停止 |
| `2` | 单曲循环 | 循环播放当前歌曲 |
| `3` | 顺序播放 | 按顺序播放到最后一首后停止 |
| `4` | 列表循环 | 循环播放整个列表 |
| `5` | 随机播放 | 随机选择歌曲 |

### 📜 歌词系统
- **本地歌词加载**：自动查找同名 `.lrc` 文件
- **歌词文件编码**：自动识别 UTF-8 / GBK 编码
- **自动网络下载**：本地无歌词时后台异步下载
- **歌词滚动高亮**：当前歌词行 `>` 前缀高亮，自动居中滚动
- **歌词位置跳转**：鼠标拖动歌词区域 / 滚轮跳转到对应时间

### 🔍 搜索功能
- **本地搜索**：按 `s` 进入，根据关键字匹配音乐目录歌曲
- **网络搜索**：按 `n` 进入，根据关键字匹配搜索网络歌曲
- **聚合搜索**：按 `j` 进入，根据关键字匹配搜索聚合歌曲
- **歌单搜索**：按 `p` 进入，根据关键字匹配搜索在线歌单
- **翻页浏览**：`PgUp`/`PgDn` 翻页查看更多结果
- **在线下载**：搜索结果选中后按 `Enter` 下载到当前音乐目录，显示下载进度

### 🤖 歌曲信息
- **智能查询**：按 `i` 查询当前歌曲详细信息，支持任意 OpenAI 兼容接口
- **流式输出**：查询结果逐字流式显示，无需等待全部生成
- **丰富信息**：涵盖歌手详情、词曲创作、所属专辑曲目、创作背景、歌词大意、音乐风格、趣闻轶事等 13 项分类
- **多语言支持**：回复语言跟随界面语言设置（简中/繁中/英/日/韩）
- **自定义接口**：按 `k` 三步配置 API 接口地址、API Key、模型名称，支持 DeepSeek、OpenRouter、AIHubMix 等任意 OpenAI 兼容接口
- **免费兜底**：未配置 API Key 时自动使用 OpenRouter 免费模型（minimax/minimax-m2.5:free）查询

### ⭐ 收藏功能
- **添加/移除收藏**：按 `f` 切换当前歌曲收藏状态
- **收藏列表**：按 `v` 查看，带 `*` 标记
- **跨目录播放**：收藏歌曲不在当前目录时自动切换目录并播放
- **删除收藏**：收藏列表中按 `d` 删除

### 💬 评论功能
- **歌曲评论**：按 `c` 查看当前歌曲的评论
- **评论详情**：按 `Enter` 切换列表/详情视图，详情显示完整内容
- **回复展示**：显示被回复的原评论内容、昵称和时间
- **评论翻页**：`PgUp`/`PgDn` 翻页，每页 20 条
- **后台加载**：评论在后台线程拉取，不阻塞 UI

### 📂 目录管理
- **音乐目录选择**：按 `o` 打开文件夹选择对话框（首次打开目录后会自动开始播放）
- **打开目录历史**：按 `m` 查看所有打开过的目录，快速切换
- **当前目录标记**：用 `>>` 标记当前正在使用的目录
- **删除目录历史**：按 `d` 删除不需要的目录记录

### 🌐 多语言界面
支持 11 种界面语言循环切换（按 `l`）：

| 语言 | 配置值 |
|------|--------|
| 中文简体 | `sc` |
| 中文繁体 | `tc` |
| English | `en` |
| 日本語 | `ja` |
| 한국어 | `ko` |
| Русский | `ru` |
| Français | `fr` |
| Deutsch | `de` |
| Español | `es` |
| Italiano | `it` |
| Português | `pt` |

### 🎨 多主题界面
4 种主题循环切换（按 `t`）：

| 主题 | 风格 |
|------|------|
| Neon | 霓虹灯色调 |
| Sunset | 落日金色调 |
| Ocean | 深海蓝色调 |
| GrayWhite | 控制台色调 |

### 🖱️ 鼠标交互
- **播放列表点击**：直接点击歌曲播放
- **进度条点击**：跳转到对应位置
- **音量条点击**：调整音量
- **歌词拖动**：左键拖动跳转到歌词时间
- **歌词滚轮**：上下滚动跳转到上/下一句歌词
- **搜索结果点击**：本地搜索点击播放，网络搜索点击下载
- **评论点击**：点击进入详情

### 📊 波形可视化
- 播放时显示基于真实 RMS 音量的动态波形条
- EMA 平滑处理，视觉更柔和
- 暂停时波形冻结

### ⚙️ 配置持久化
配置保存在 `USERPROFILE/ter-music-rust/config.json`，自动保存和恢复：

| 配置项 | 说明 |
|--------|------|
| `music_directory` | 上次打开的音乐目录 |
| `play_mode` | 播放模式 |
| `current_index` | 上次播放的歌曲索引（恢复播放） |
| `volume` | 音量大小 (0-100) |
| `favorites` | 收藏列表 |
| `dir_history` | 目录历史记录 |
| `api_key` | API Key（歌曲信息查询用，兼容旧字段 `deepseek_api_key`） |
| `api_base_url` | API 接口地址（默认 `https://api.deepseek.com/`） |
| `api_model` | AI 模型名称（默认 `deepseek-v4-flash`） |
| `github_token` | GitHub Token（用于提交歌曲信息 Discussion，留空使用默认 Token） |
| `theme` | 界面主题名称 |
| `language` | 界面语言（`sc` / `tc` / `en` / `ja` / `ko` / `ru` / `fr` / `de` / `es` / `it` / `pt`） |

**自动保存时机**：切歌、切换主题、切换语言、收藏变更、每 30 秒、退出时（含 Ctrl+C）

---

## 🚀 快速开始

### 1. 安装 Rust

```powershell
# 方法一：winget（推荐）
winget install Rustlang.Rustup

# 方法二：官方安装器
# 访问 https://rustup.rs/ 下载安装
```

验证安装：
```powershell
rustc --version
cargo --version
```

### 2. 编译项目

```powershell
cd <项目目录>

# 方法一：使用构建脚本（推荐）
build-win.bat

# 方法二：使用 Cargo
cargo build --release
```

### 3. 运行程序

```powershell
# 方法一：cargo run
cargo run --release

# 方法二：直接运行
.\target\release\ter-music-rust.exe

# 方法三：指定音乐目录
.\target\release\ter-music-rust.exe -o d:\Music
cargo run --release -- -o d:\Music
```

**目录加载优先级**：命令行 `-o` > 配置文件 > 文件夹选择对话框

---

## 🎮 快捷键

### 主界面控制键

| 按键 | 功能 |
|------|------|
| `↑/↓` | 选择歌曲 |
| `Enter` | 播放选中歌曲 |
| `Space` | 播放/暂停 |
| `Esc` | 停止播放（评论视图中为返回歌词） |
| `←/→` | 上一曲/下一曲 |
| `[` | 快退 5 秒 |
| `]` | 快进 5 秒 |
| `,` | 快退 10 秒 |
| `.` | 快进 10 秒 |
| `+/-` | 音量加减 (步长 5) |
| `1-5` | 切换播放模式 |
| `o` | 打开音乐目录 |
| `s` | 搜索本地歌曲 |
| `n` | 搜索网络歌曲 |
| `j` | 搜索聚合歌曲 |
| `p` | 搜索在线歌单 |
| `i` | 查看歌曲信息 |
| `f` | 收藏/取消收藏 |
| `v` | 查看收藏列表 |
| `m` | 查看目录历史 |
| `h` | 显示帮助信息 |
| `c` | 查看歌曲评论 |
| `l` | 切换界面语言 |
| `t` | 切换界面主题 |
| `k` | 配置API 接口 |
| `g` | 配置Github Token |
| `q` | 退出音乐程序 |

### 歌曲搜索按键

| 按键 | 功能 |
|------|------|
| 输入字符 | 输入搜索关键字 |
| `Backspace` | 删除字符 |
| `Enter` | 搜索/播放/下载 |
| `↑/↓` | 选择搜索结果 |
| `PgUp/PgDn` | 翻页（网络搜索） |
| `s/n/j` | 切换本地/网络/聚合搜索 |

| `Esc` | 退出搜索 |

### 收藏列表按键

| 按键 | 功能 |
|------|------|
| `↑/↓` | 选择歌曲 |
| `Enter` | 播放选中歌曲 |
| `d` | 删除收藏 |
| `Esc` | 返回播放列表 |

### 目录历史按键

| 按键 | 功能 |
|------|------|
| `↑/↓` | 选择目录 |
| `Enter` | 切换到选中目录 |
| `d` | 删除目录记录 |
| `Esc` | 返回播放列表 |

### 歌曲评论按键

| 按键 | 功能 |
|------|------|
| `↑/↓` | 选择评论 |
| `Enter` | 切换列表/详情视图 |
| `PgUp/PgDn` | 翻页 |
| `Esc` | 返回歌词视图 |

### 歌曲信息按键

| 按键 | 功能 |
|------|------|
| `↑/↓` | 滚动查看信息 |
| `i` | 重新查询歌曲信息 |
| `Esc` | 返回歌词视图 |

### 歌单搜索按键

| 按键 | 功能 |
|------|------|
| 输入字符 | 输入歌单关键字 |
| `Backspace` | 删除字符 |
| `Enter` | 搜索/进入歌单/播放下载 |
| `↑/↓` | 选择歌单或歌曲 |
| `PgUp/PgDn` | 翻页 |
| `Esc` | 返回上一层/退出搜索 |

### 帮助信息按键


| 按键 | 功能 |
|------|------|
| `↑/↓` | 滚动查看帮助 |
| `Esc` | 返回歌词视图 |

---

## 📦 安装与编译

### 系统要求

- **操作系统**：Windows 10/11
- **Rust 版本**：1.70+
- **终端**：Windows Terminal（推荐）/ CMD / PowerShell
- **窗口大小**：建议 80×25 或更大

### 方案一：MSVC 工具链（兼容性最好，体积大）

```powershell
# 1. 安装 Rust
winget install Rustlang.Rustup

# 2. 安装 Build Tools
winget install Microsoft.VisualStudio.2022.BuildTools
# 运行安装器 → 选择"使用 C++ 的桌面开发" → 安装

# 3. 重启终端并编译
cargo build --release
```

### 方案二：GNU 工具链（推荐，轻量约 300 MB）

```powershell
# 1. 安装 Rust
winget install Rustlang.Rustup

# 2. 安装 MSYS2
winget install MSYS2.MSYS2
# 打开 MSYS2 终端运行：
pacman -Syu
pacman -S mingw-w64-x86_64-toolchain

# 3. 添加 PATH（PowerShell 管理员）
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\msys64\mingw64\bin", "Machine")

# 4. 切换工具链并编译
rustup default stable-x86_64-pc-windows-gnu
cargo build --release
```

> GNU 工具链编译的程序运行时可能需要复制以下 DLL 到程序目录：
> `libgcc_s_seh-1.dll`、`libstdc++-6.dll`、`libwinpthread-1.dll`

### 方案三：交叉编译 Linux 版本（在 Windows 上编译 Linux 可执行文件）

使用 `cargo-zigbuild` + `zig` 作为链接器，无需安装 Linux 系统或虚拟机即可交叉编译。

```powershell
# 1. 安装 zig（任选一种方式）
# 方式 A：pip 安装（推荐）
pip install ziglang

# 方式 B：MSYS2 安装
pacman -S mingw-w64-x86_64-zig

# 方式 C：手动下载
# 访问 https://ziglang.org/download/ 下载解压，添加到 PATH

# 2. 安装 cargo-zigbuild
cargo install cargo-zigbuild

# 3. 添加 Linux 目标
rustup target add x86_64-unknown-linux-gnu

# 4. 准备 Linux sysroot（ALSA 头文件和库）
# 项目已内置 linux-sysroot/ 目录，包含交叉编译所需的 ALSA 开发文件
# 如需自行准备，可从 Debian/Ubuntu 系统复制以下文件：
#   /usr/include/alsa/         → linux-sysroot/usr/include/alsa/
#   /usr/lib/x86_64-linux-gnu/libasound.so* → linux-sysroot/usr/lib/x86_64-linux-gnu/

# 5. 编译
build-linux.bat

# 或手动执行：
cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.34
```

**输出文件**：`target/x86_64-unknown-linux-gnu/release/ter-music-rust`

**部署到 Linux**：
```bash
# 1. 复制到 Linux 系统
scp ter-music-rust user@linux-host:~/

# 2. 添加执行权限
chmod +x ter-music-rust

# 3. 安装 ALSA 运行库
sudo apt install libasound2

# 4. 运行
./ter-music-rust -o /path/to/music
```

> **说明**：`build-linux.bat` 会自动设置 `PKG_CONFIG_PATH`、`PKG_CONFIG_ALLOW_CROSS`、`RUSTFLAGS` 等环境变量。
> 目标 `x86_64-unknown-linux-gnu.2.34` 中的 `.2.34` 指定 glibc 最低版本，确保在较旧 Linux 系统上也能运行。

### Linux 打包（DEB / RPM）

如果你在 Linux 环境下构建并打包，可直接使用以下脚本：

```bash
# 1) RPM 包
./build-rpm.sh

# 生成 debuginfo RPM（可选）
./build-rpm.sh --with-debuginfo

# 2) DEB 包
./build-deb.sh

# 生成 debug symbols DEB（可选）
./build-deb.sh --with-debuginfo

# 生成符合 dpkg-source 规范的源码包（.dsc/.orig.tar/.debian.tar）
./build-deb.sh --with-source

# 同时生成 debuginfo + 源码包
./build-deb.sh --with-debuginfo --with-source
```

默认输出目录：
- `dist/rpm/`：RPM / SRPM
- `dist/deb/`：DEB / 源码包

> 提示：脚本会读取 `Cargo.toml` 中的 `name` 与 `version` 自动命名包文件。

### 方案四：交叉编译 MacOS 版本（在 Windows 上编译 MacOS 可执行文件）

使用 `cargo-zigbuild` + `zig` + MacOS SDK 交叉编译。MacOS 上的音频使用系统 CoreAudio，需要额外的 SDK 头文件。

**前置准备：**

```powershell
# 1. 安装 zig（同 Linux 交叉编译）
pip install ziglang

# 2. 安装 cargo-zigbuild
cargo install cargo-zigbuild

# 3. 安装 LLVM/Clang（提供 libclang.dll，bindgen 需要）
# 方式 A：MSYS2
pacman -S mingw-w64-x86_64-clang

# 方式 B：LLVM 官方安装
winget install LLVM.LLVM

# 4. 添加 MacOS 目标
rustup target add x86_64-apple-darwin aarch64-apple-darwin
```

**准备 MacOS SDK：**

把MacOSX13.3.sdk.tar.xz解压到macos-sysroot目录
项目已内置 `macos-sysroot/` 目录（从 [macosx-sdks](https://github.com/joseluisq/macosx-sdks) 下载）。

如需重新获取：

```powershell
# 方式 A：从 GitHub 下载预打包 SDK（推荐，约 56 MB）
# 加速: https://ghfast.top/https://github.com/joseluisq/macosx-sdks/releases/download/13.3/MacOSX13.3.sdk.tar.xz
curl -L -o MacOSX13.3.sdk.tar.xz https://github.com/joseluisq/macosx-sdks/releases/download/13.3/MacOSX13.3.sdk.tar.xz
mkdir macos-sysroot
tar -xf MacOSX13.3.sdk.tar.xz -C macos-sysroot --strip-components=1
del MacOSX13.3.sdk.tar.xz

# 方式 B：从 MacOS 系统复制
scp -r mac:/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk ./macos-sysroot
```

> SDK 来源：https://github.com/joseluisq/macosx-sdks
> 包含 CoreAudio、AudioToolbox、AudioUnit、CoreMIDI、OpenAL、IOKit 等框架头文件

**编译：**

```powershell
# 使用构建脚本（自动设置所有环境变量）
build-mac.bat

# 或手动执行：
$env:LIBCLANG_PATH = "C:\msys64\mingw64\bin"    # libclang.dll 所在目录
$env:COREAUDIO_SDK_PATH = "./macos-sysroot"       # MacOS SDK 路径（正斜杠）
$env:SDKROOT = "./macos-sysroot"                   # zig 链接器需要，找系统库
$FW = "./macos-sysroot/System/Library/Frameworks"
$env:BINDGEN_EXTRA_CLANG_ARGS = "--target=x86_64-apple-darwin -isysroot ./macos-sysroot -F $FW -iframework $FW -I ./macos-sysroot/usr/include"
cargo zigbuild --release --target x86_64-apple-darwin    # Intel Mac
# 编译 Apple Silicon 时将 --target 和 BINDGEN_EXTRA_CLANG_ARGS 中的 x86_64 改为 aarch64
cargo zigbuild --release --target aarch64-apple-darwin   # Apple Silicon
```

**输出文件**：
- `target/x86_64-apple-darwin/release/ter-music-rust` — Intel Mac
- `target/aarch64-apple-darwin/release/ter-music-rust` — Apple Silicon (M1/M2/M3/M4)

**部署到 MacOS**：
```bash
# 1. 复制到 MacOS 系统
scp ter-music-rust user@mac-host:~/

# 2. 添加执行权限
chmod +x ter-music-rust

# 3. MacOS允许运行未知来源应用
xattr -cr ter-music-rust

# 4. 运行（无需额外安装音频库）
./ter-music-rust -o /path/to/music
```

> **注意**：MacOS 交叉编译需要 MacOS SDK 头文件，项目已内置 `macos-sysroot/` 目录（从 [macosx-sdks](https://github.com/joseluisq/macosx-sdks) 获取）。
> 编译时还需 `libclang.dll`（通过 MSYS2 或 LLVM 安装）。

### 工具链切换

```powershell
# 查看当前工具链
rustup show

# 切换到 MSVC
rustup default stable-x86_64-pc-windows-msvc

# 切换到 GNU
rustup default stable-x86_64-pc-windows-gnu
```

### Cargo 国内镜像（加速下载）

创建或编辑 `~/.cargo/config`：

```toml
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
```

---

## 🛠️ 项目结构

```
src/
├── main.rs       # 主程序入口（参数解析、初始化、配置恢复/保存）
├── defs.rs       # 公共定义（PlayMode/PlayState 枚举、MusicFile/Playlist 结构体）
├── langs.rs      # 语言包（11 种语言翻译文本集中管理、全局语言访问器）
├── audio.rs      # 音频播放控制（rodio 封装、播放/暂停/跳转/音量/进度）
├── analyzer.rs   # 音频分析器（实时 RMS 音量、EMA 平滑、波形可视化）
├── playlist.rs   # 播放列表管理（目录扫描、并行获取时长、文件夹选择对话框）
├── lyrics.rs     # 歌词解析（LRC 格式、本地查找、编码检测、后台下载）
├── search.rs     # 网络搜索下载（酷我+酷狗+网易搜索、在线下载、评论拉取、歌曲信息流式查询）
├── config.rs     # 配置文件管理（JSON 序列化、8 项配置持久化）
└── ui.rs         # 用户界面（终端渲染、事件处理、多视图模式、主题/语言系统）
```

### 技术栈

| 依赖 | 版本 | 用途 |
|------|------|------|
| [rodio](https://github.com/RustAudio/rodio) | 0.19 | 音频解码和播放（纯 Rust） |
| [crossterm](https://github.com/crossterm-rs/crossterm) | 0.28 | 终端 UI 控制 |
| [reqwest](https://github.com/seanmonstar/reqwest) | 0.12 | HTTP 网络请求 |
| [serde](https://github.com/serde-rs/serde) + serde_json | 1.0 | JSON 序列化 |
| [rayon](https://github.com/rayon-rs/rayon) | 1.10 | 并行获取音频时长 |
| [encoding_rs](https://github.com/hsivonen/encoding_rs) | 0.8 | GBK 编码歌词解码 |
| [walkdir](https://github.com/BurntSushi/walkdir) | 2.5 | 递归目录扫描 |
| [rand](https://github.com/rust-random/rand) | 0.8 | 随机播放模式 |
| [unicode-width](https://github.com/unicode-rs/unicode-width) | 0.2 | 中文显示宽度计算 |
| [chrono](https://github.com/chronotope/chrono) | 0.4 | 评论时间格式化 |
| [ctrlc](https://github.com/Detegr/rust-ctrlc) | 3.4 | Ctrl+C 信号处理 |
| [md5](https://github.com/johannhof/md5) | 0.7 | 酷狗音乐 API MD5 签名 |
| [winapi](https://github.com/retep998/winapi-rs) | 0.3 | Windows 控制台 UTF-8 |

### 编译优化

```toml
[profile.release]
opt-level = 3       # 最高优化级别
lto = true          # 链接时优化
codegen-units = 1   # 单编译单元，更好优化
strip = true        # 去除调试符号
```


---

## Rust 与 C 版本的对比

| 特性 | Rust 版本 | C 版本 |
|------|-----------|--------|
| 安装大小 | ~200 MB (Rust) / ~300 MB (GNU) | ~7 GB (Visual Studio) |
| 安装时间 | ~5 分钟 | ~1 小时 |
| 编译速度 | ⚡ 快速 | 🐢 较慢 |
| 依赖管理 | ✅ Cargo 自动 | ❌ 手动配置 |
| 内存安全 | ✅ 编译时保证 | ⚠️ 需手动管理 |
| 跨平台 | ✅ 完全跨平台 | ⚠️ 需修改代码 |
| 可执行文件 | ~2 MB | ~500 KB |
| 内存占用 | ~15-20 MB | ~10 MB |
| CPU 占用 | < 1% | < 1% |

---

## 📊 性能指标

| 指标 | 数值 |
|------|------|
| 界面刷新 | 50ms |
| 键盘响应 | < 50ms |
| 歌词下载 | 后台进行，不阻塞 UI |
| 目录扫描 | 并行获取时长，2-4 倍提速 |
| 启动时间 | < 100ms |
| 内存占用 | ~15-20 MB |

---

## 🐛 故障排除

### 编译错误

```powershell
# 更新 Rust
rustup update

# 清理重编
cargo clean
cargo build --release
```

### link.exe not found

安装 Visual Studio Build Tools（见上方编译方案一）。

### dlltool.exe not found

安装完整的 MinGW-w64 工具链（见上方编译方案二）。

### 运行时缺少 DLL（GNU 工具链）

```powershell
Copy-Item "C:\msys64\mingw64\bin\libgcc_s_seh-1.dll" -Destination ".\target\release\"
Copy-Item "C:\msys64\mingw64\bin\libstdc++-6.dll" -Destination ".\target\release\"
Copy-Item "C:\msys64\mingw64\bin\libwinpthread-1.dll" -Destination ".\target\release\"
```

### 找不到音频设备

1. 确保系统音频设备正常
2. 检查 Windows 音量设置
3. 尝试播放系统测试音

### 界面显示异常

- 确保终端窗口 ≥ 80×25
- 使用 Windows Terminal 获得最佳体验
- CMD 中确保字体支持中文

### 网络搜索/歌词下载失败

- 检查网络连接
- 某些歌曲可能需要 VIP 或已下架
- 歌词格式必须为标准 LRC 格式

### 歌曲信息查询失败

- 未配置 API Key 时会自动使用 OpenRouter 免费模型，无需手动设置
- 如需使用自定义接口，按 `k` 依次输入接口地址、API Key、模型名称
- 支持任何 OpenAI 兼容接口（DeepSeek、OpenRouter、AIHubMix 等）
- 检查网络连接是否可访问对应 API 服务

### 首次编译慢

首次编译需要下载和编译所有依赖，是正常现象，后续编译会快很多。

### 下载Release
[ter-music-rust-win.zip](https://storage.deepin.org/thread/202605050312131911_ter-music-rust-win.zip "附件(Attached)") 
[ter-music-rust-mac.zip](https://storage.deepin.org/thread/202605050312183967_ter-music-rust-mac.zip "附件(Attached)") 
[ter-music-rust-linux.zip](https://storage.deepin.org/thread/202605050312251425_ter-music-rust-linux.zip "附件(Attached)") 
[ter-music-rust_deb.zip](https://storage.deepin.org/thread/202605050312355690_ter-music-rust_deb.zip "附件(Attached)")

---

## 📝 更新日志

## 版本 1.7.0 (2026-05-05)

### 🐞 Bug 修复

- 🛠️ **Linux首次运行界面显示不完整**：修复 Linux 下首次运行程序时界面缩在终端左上角、需点击才显示完整的问题，进入 alternate screen 后等待 50ms 重新获取终端尺寸并清屏
- 🛠️ **空播放列表无提示**：首次运行未选择音乐目录时播放列表为空但无引导提示，现添加"请按o选择音乐目录"提示（与歌词区域提示样式一致）
- 🛠️ **选中行蓝色背景溢出**：修复选中行蓝色背景延伸超出左面板边界到右侧歌词区域的问题，改用精确宽度空格填充替代 `Clear(UntilNewLine)`
- 🛠️ **歌词区域残留旧内容**：修复切换歌曲后若新歌曲无歌词，右侧歌词区域仍残留上一首歌词内容的问题，绘制前先清除所有行
- 🛠️ **窗口resize暂停/停止时不重绘**：修复调整终端窗口大小后，若播放处于暂停或停止状态界面不立即更新的问题，添加 `Event::Resize` 事件处理
- 🛠️ **评论翻页暂停时不显示**：修复评论视图中 PageUp/PageDown 操作后，若播放暂停或停止界面不更新的问题，将评论加载状态加入周期性重绘条件
- 🛠️ **评论模式切歌后评论重置**：修复评论模式下切歌时评论被重置、丢失用户正在查看的内容的问题，评论模式下跳过评论重置，翻页使用存储的歌曲名
- 🛠️ **播放时标题字符丢失**：修复播放状态下数字/英文开头的歌曲标题丢失字符的问题（如"17岁"显示为"1岁"），根本原因是 `►★▶■❚` 等 Unicode 符号在东亚终端中宽度模糊（1列或2列不一致），导致光标偏移覆盖后续字符。将所有模糊宽度 Unicode 符号替换为宽度明确的 ASCII 字符：`►`→`>`、`★`→`*`、`▶`→`>>`、`■`→`||`、`❚`→`[]`

### 🔧 功能改进

- 🎨 **UI 符号统一为 ASCII**：播放状态前缀 `>>`（播放）、`||`（暂停）、`[]`（停止），选中标记 `>`，收藏标记 `*`，目录当前标记 `>>`，歌词高亮标记 `>`，评论选中标记 `>`，消除东亚终端宽度歧义
- 📝 **空播放列表提示语优化**：将"未选择可用的音乐目录，已进入空列表模式，可按 o 打开音乐目录"改为"没有可用的音乐目录，已进入空播放列表模式，可按o打开音乐目录"，措辞更准确自然
- 📂 **无可用目录设置默认目录**：无可用目录时自动设置默认音乐目录（USERPROFILE/ter-music-rust/music）并添加到音乐目录历史，搜索下载歌曲时使用默认音乐目录而非当前工作目录

---

## 版本 1.6.0 (2026-05-04)

### 🎉 新功能

#### 多语言扩展与国际化重构
- ✨ **新增 6 种界面语言**：新增俄语（Русский）、法语（Français）、德语（Deutsch）、西班牙语（Español）、意大利语（Italiano）、葡萄牙语（Português），共支持 11 种语言
- ✨ **全模块国际化**：所有用户面向的文字（UI 界面、命令行帮助、错误消息、对话框标题）均已国际化，包括 `ui.rs`、`main.rs`、`search.rs`、`audio.rs`、`config.rs`、`playlist.rs`
- ✨ **语言包集中管理**：新增 `langs.rs` 模块，将所有语言的翻译文本集中到一个文件管理，包含 `LangTexts` 结构体和 11 个语言静态实例
- ✨ **全局语言访问器**：提供 `langs::global_texts()` 函数，供非 UI 模块（search.rs / audio.rs / config.rs / playlist.rs）线程安全地获取当前语言翻译文本
- ✨ **AI 提示词多语言**：每种语言的 AI 歌曲信息查询提示词均使用对应语言输出，确保回复语言与界面语言一致

### 🔧 功能改进

- 🌐 **CLI 帮助国际化**：命令行 `-h` 帮助信息跟随界面语言显示
- 🌐 **错误消息国际化**：音频错误、搜索错误、配置错误、目录错误等消息均跟随界面语言
- 🌐 **对话框标题国际化**：macOS / Linux 文件夹选择对话框标题跟随界面语言
- ♻️ **代码解耦**：各模块不再硬编码中文文字，统一通过 `self.t()` 或 `langs::global_texts()` 读取翻译文本

### 🐞 Bug 修复

- 🛠️ **评论模式键盘焦点修复**：修复网络搜索/聚合搜索/歌单搜索中按 `c` 查看评论后，上下键仍控制歌曲列表而非评论列表的问题
- 🛠️ **Linux目录选择对话框修复**：修复 Linux 下按 `o` 无法弹出图形化目录选择对话框的问题，改为先退出 raw mode 再调用对话框，且对话框成功后不再回退到终端输入
- 🛠️ **UTF-8日志切片安全修复**：修复日志输出中使用字节切片截断多字节 UTF-8 字符可能导致程序崩溃的问题
- 🛠️ **配置文件格式化修复**：修复配置文件错误提示中双重 `replace("{}")` 导致第二个占位符无法正确替换的问题

---

## 版本 1.5.0 (2026-04-30)

### 🎉 新功能

#### 在线歌单搜索
- ✨ **歌单检索入口**：按 `p` 可直接搜索网络歌单
- ✨ **歌单内容浏览**：进入歌单后可查看歌曲列表并快速播放
- ✨ **缓存命中播放**：网络搜索/聚合搜索/歌单搜索本地存在或者下载命中缓存时，跳过重复下载直接播放
- ✨ **歌词去重下载**：网络搜索/聚合搜索/歌单搜索本地存在或者下载命中缓存时，歌词文件跳过重复下载

### 🔧 功能改进

- 🎵 **歌词策略优化**：播放时改为“聚合歌词优先，常规歌词兜底”，提升歌词命中准确率
- 🎯 **在线搜索焦点优化**：按 `s/n/j/p` 进入搜索时默认聚焦输入栏，可直接输入关键词
- 🎯 **搜索与列表交互优化**：回车或点击歌曲开始播放后自动切到列表焦点，键盘快捷键不再误写入搜索框
- 🎯 **在线列表样式统一**：在线搜索/聚合搜索/歌单搜索中，选中箭头与播放标记分离显示，并与本地播放列表空格对齐一致
- 🎲 **在线随机播放一致性优化**：随机播放模式下，网络搜索/聚合搜索结果现已支持随机切歌，行为与歌单场景保持一致
- 🛡️ **在线自动切歌保护**：新增在线自动切歌限流保护，3 秒内连续自动跳过达 5 首时自动停止，避免不可播歌曲导致无控制连跳

### 🐞 Bug 修复

- 🛠️ **歌词下载优先级修复**：修正网络搜索/聚合搜索/歌单搜索场景下歌词下载顺序异常的问题
- 🛠️ **在线续播索引修复**：修复播放过程中移动光标后，自动下一首从光标位置续播的问题，现按播放模式基于实际播放项切歌
- 🛠️ **搜索空格误输入修复**：修复在线搜索列表焦点下空格被写入搜索框并影响搜索结果的问题
- 🛠️ **网络搜索初始焦点修复**：修复按 `n` 进入网络搜索时输入焦点未落在搜索栏的问题
- 🛠️ **在线随机播放缺失修复**：修复随机播放模式在网络搜索/聚合搜索结果中未生效的问题
- 🛠️ **在线自动切歌误停修复**：修复在线场景下遇到首个不可播歌曲时可能提前停止的问题，改为仅在真实自动切歌时计数并在成功播放后重置计数窗口

---

## 版本 1.4.0 (2026-04-28)

### 🎉 新功能

#### 歌曲聚合搜索兜底
- ✨ **聚合搜索歌曲**：网络搜索不到时可以通过聚合搜索按歌名/歌手搜索并下载歌曲
- ✨ **聚合搜索歌词**：本地无歌词且网络下载失败时，自动通过聚合搜索按歌名/歌手搜索并下载歌词
- ✨ **无缝体验**：搜索和下载均在后台进行，不阻塞 UI

#### GitHub Token 配置
- ✨ **自定义 GitHub Token**：按 `g` 输入自己的 GitHub Token，保存到配置文件
- ✨ **默认兜底**：未配置 Token 时自动使用默认 Token
- ✨ **身份识别**：使用自己的 Token 提交歌曲信息 Discussion 时显示为自己的 GitHub 身份



### 🔧 功能改进

- 🔍 **新增配置项**：`github_token`（GitHub Token，留空使用默认）

---

## 版本 1.3.0 (2026-04-26)

### 🎉 新功能

#### 自定义 AI 接口
- ✨ **OpenAI 兼容接口**：支持任意 OpenAI 兼容 API 接口查询歌曲信息（DeepSeek、OpenRouter、AIHubMix 等）
- ✨ **三步配置**：按 `k` 依次输入接口地址 → API Key → 模型名称，完成自定义接口配置
- ✨ **免费兜底**：未配置 API Key 时自动使用 OpenRouter 免费模型（minimax/minimax-m2.5:free）查询
- ✨ **直接查询**：按 `i` 直接查询歌曲信息，无需预先配置 API Key

### 🔧 功能改进

- 🔍 **提示词优化**：「歌曲大意」改为「歌词大意」，「有趣事实」改为「趣闻轶事」
- 🔍 **配置字段重命名**：`deepseek_api_key` → `api_key`（兼容旧配置文件）
- 🔍 **新增配置项**：`api_base_url`（接口地址，默认 DeepSeek）、`api_model`（模型名称，默认 deepseek-v4-flash）

---

## 版本 1.2.0 (2026-04-24)

### 🎉 新功能

#### 歌曲信息查询
- ✨ **DeepSeek查询**：按 `i` 键使用 DeepSeek 流式查询当前歌曲详细信息
- ✨ **内容流式输出**：查询结果逐字显示，无需等待全部生成
- ✨ **13项分类信息**：演唱歌手、歌手详情、词曲创作、发行时间、所属专辑（含曲目列表）、创作背景、歌曲大意、音乐风格、商业成绩、获奖记录、影响评价、翻唱引用、有趣事实
- ✨ **多语言回复**：回复语言跟随界面语言（简中/繁中/英/日/韩）
- ✨ **API Key 管理**：按 `k` 输入 DeepSeek API Key，支持环境变量 `DEEPSEEK_API_KEY`

#### 酷狗音乐搜索
- ✨ **酷狗音乐源**：新增酷狗音乐作为第三个搜索/下载平台
- ✨ **三平台搜索**：搜索优先级为 酷我 → 酷狗 → 网易
- ✨ **减少 VIP 限制**：酷狗音乐源提供更多免费下载资源
- ✨ **MD5 签名认证**：酷狗下载链接使用 MD5 签名，提高下载成功率

### 🔧 功能改进

#### 歌曲信息提示词优化
- 🔍 **无开场白**：回复不再输出问候语和自我介绍
- 🔍 **无序号编号**：输出内容不再使用编号列表格式
- 🔍 **歌手详情**：新增歌手详细信息分类（国籍、出生地、出生日期等）
- 🔍 **专辑曲目**：所属专辑包含完整曲目列表

### 💻 技术细节

#### 依赖更新
- ➕ 添加 `md5` 依赖（酷狗音乐 API 签名）

#### 数据结构
- ♻️ `OnlineSong` 新增 `hash` 字段（酷狗音乐使用 hash 标识歌曲）
- ♻️ 新增 `MusicSource::Kugou` 枚举变体
- ♻️ 新增酷狗 JSON 解析结构体

---

## 版本 1.1.0 (2026-04-17)

### 🎉 新功能

#### 歌词显示系统
- ✨ **左右分栏布局**: 左侧歌曲列表，右侧歌词显示
- ✨ **自动下载歌词**: 未找到歌词时自动从网络下载
- ✨ **智能匹配**: 自动查找带标记的歌词文件
- ✨ **多编码支持**: 支持 UTF-8 和 GBK 编码的歌词文件
- ✨ **歌词滚动**: 歌词随播放进度自动滚动
- ✨ **高亮显示**: 当前播放歌词黄色高亮显示
- ✨ **歌曲名显示**: 歌词标题显示当前播放歌曲名称

#### 用户体验
- ✨ **歌词下载**: 播放歌曲自动匹配下载歌词显示
- ✨ **统一样式**: 播放列表和歌词区域使用统一的黄色样式
- ✨ **动态标题**: 歌词标题动态显示歌曲名称
- ✨ **语言选择**: 支持多语言界面切换选择
- ✨ **主题选择**: 支持多主题界面切换选择

### 🚀 性能优化

#### 界面渲染
- ⚡ **进度条**: 1 秒优显示更流畅
- ⚡ **减少重绘**: 优化事件循环，避免不必要的重绘
- ⚡ **锁优化**: 减少锁持有时间，提升响应速度

#### 歌词加载
- ⚡ **智能缓存**: 歌词加载后缓存，避免重复解析
- ⚡ **延迟加载**: 只在需要时加载歌词
- ⚡ **批量重命名**: 支持批量清理歌词文件名标记

### 🎨 界面改进

#### 视觉优化
- 🎨 **统一配色**: 播放列表和歌词区域使用统一的黄色
- 🎨 **分栏布局**: 左右分栏，空间利用率更高
- 🎨 **中间分隔线**: 使用竖线分隔，视觉更清晰

#### 信息显示
- 📊 **播放列表范围**: 显示当前可见范围
- 📊 **歌曲名称**: 歌词标题显示歌曲名
- 📊 **进度条**: 更新频率提升，显示更流畅

### 🔧 功能改进

#### 歌词管理
- 🔍 **智能查找**: 自动查找多种格式的歌词文件
- 🔍 **文件对应**: 确保歌词文件名与歌曲文件名一一对应

#### 错误处理
- 🛡️ **下载失败处理**: 下载失败时显示友好提示
- 🛡️ **编码检测**: 自动检测歌词文件编码
- 🛡️ **网络超时**: 设置 10 秒超时，避免长时间等待


### 🐛 Bug 修复

- 🐛 修复歌词文件名标记导致无法匹配的问题
- 🐛 修复歌词下载时的编码问题
- 🐛 修复界面重绘时的闪烁问题
- 🐛 修复进度条更新不及时的问题

### 💻 技术细节

#### 依赖更新
- ➕ 添加 `reqwest` HTTP 客户端
- ➕ 添加 `urlencoding` URL 编码支持
- ➕ 添加 `encoding_rs` 编码转换支持

#### 代码重构
- ♻️ 优化事件循环逻辑
- ♻️ 改进歌词加载流程
- ♻️ 统一颜色常量定义

---

## 版本 1.0.0 (2026-04-09)

### 基础功能
- 🎵 音频播放（支持多种格式）
- 📋 播放列表管理
- 🎹 播放控制
- 🔊 音量调节
- 🎲 播放模式切换
- 📂 文件夹浏览

---

## 📄 AI辅助

GLM、Codex

## 📄 许可证

MIT License

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！
