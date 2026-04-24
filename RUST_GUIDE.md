# 🎯 Rust 版本开发指南

## 为什么选择 Rust？

相比 C 版本，Rust 版本有以下优势：

| 特性 | Rust 版本 | C 版本 |
|------|-----------|--------|
| **安装大小** | ~200 MB (MSVC) / ~300 MB (GNU) | ~7 GB (Visual Studio) |
| **安装时间** | ~5 分钟 | ~1 小时 |
| **依赖管理** | ✅ Cargo 自动管理 | ❌ 手动配置 |
| **编译速度** | ⚡ 快速 | 🐢 较慢 |
| **内存安全** | ✅ 编译时保证 | ⚠️ 需手动管理 |
| **跨平台** | ✅ 完全跨平台 | ⚠️ 需修改代码 |

---

## 📦 核心依赖

### 1. rodio - 音频播放库

```toml
[dependencies]
rodio = "0.19"
```

**特点：**
- ✅ 纯 Rust 实现，无 C 依赖
- ✅ 支持多种音频格式（MP3, WAV, FLAC, OGG, OGA, Opus, M4A, AAC, AIFF, APE）
- ✅ 跨平台（Windows, Linux, macOS）
- ✅ 简单易用的 API

**使用示例：**
```rust
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

// 初始化音频输出
let (_stream, stream_handle) = OutputStream::try_default().unwrap();

// 创建播放器
let sink = Sink::try_new(&stream_handle).unwrap();

// 加载音频文件
let file = File::open("music.mp3").unwrap();
let source = Decoder::new(BufReader::new(file)).unwrap();

// 播放
sink.append(source);
sink.sleep_while_playing();
```

### 2. crossterm - 终端 UI 库

```toml
[dependencies]
crossterm = "0.28"
```

**特点：**
- ✅ 跨平台终端控制
- ✅ 支持 Windows Terminal, CMD, PowerShell
- ✅ 键盘/鼠标事件处理
- ✅ 光标控制、颜色输出、原始模式

**使用示例：**
```rust
use crossterm::{event, terminal, cursor, style};
use std::io;

// 初始化原始模式
terminal::enable_raw_mode()?;

// 监听键盘事件
if event::poll(std::time::Duration::from_millis(50))? {
    if let event::Event::Key(key) = event::read()? {
        match key.code {
            event::KeyCode::Char('q') => break,
            event::KeyCode::Up => println!("Up arrow"),
            _ => {}
        }
    }
}

// 清理
terminal::disable_raw_mode()?;
```

### 3. reqwest - HTTP 网络请求

```toml
[dependencies]
reqwest = { version = "0.12", features = ["blocking"] }
```

用于网络搜索下载（酷我/网易云搜索）、歌词下载、评论拉取。

### 4. serde + serde_json - JSON 序列化

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

用于配置文件读写、网络 API 响应解析。

### 5. rayon - 并行计算

```toml
[dependencies]
rayon = "1.10"
```

用于并行获取音频文件时长，目录扫描提速 2-4 倍。

### 6. encoding_rs - 编码检测与转换

```toml
[dependencies]
encoding_rs = "0.8"
```

用于 GBK 编码歌词文件解码，自动检测 UTF-8 / GBK。

### 其他依赖

| 依赖 | 版本 | 用途 |
|------|------|------|
| `walkdir` | 2.5 | 递归目录扫描 |
| `rand` | 0.8 | 随机播放模式 |
| `unicode-width` | 0.2 | 中文显示宽度计算 |
| `chrono` | 0.4 | 评论时间格式化 |
| `ctrlc` | 3.4 | Ctrl+C 信号处理 |
| `winapi` | 0.3 | Windows 控制台 UTF-8 设置 |

---

## 🏗️ 架构设计

### 模块划分

```
src/
├── main.rs       # 主程序入口（参数解析、初始化、配置恢复/保存）
├── defs.rs       # 公共定义（PlayMode/PlayState 枚举、MusicFile/Playlist 结构体）
├── audio.rs      # 音频播放控制（rodio 封装、播放/暂停/跳转/音量/进度）
├── analyzer.rs   # 音频分析器（实时 RMS 音量、EMA 平滑、波形可视化）
├── playlist.rs   # 播放列表管理（目录扫描、并行获取时长、文件夹选择对话框）
├── lyrics.rs     # 歌词解析（LRC 格式、本地查找、编码检测、后台下载）
├── search.rs     # 网络搜索下载（酷我+网易搜索、在线下载、评论拉取）
├── config.rs     # 配置文件管理（JSON 序列化、8 项配置持久化）
└── ui.rs         # 用户界面（终端渲染、事件处理、多视图模式、主题/语言系统）
```

### 数据流程

```
用户输入（键盘/鼠标）
    ↓
ui.rs（事件处理 + 界面渲染）
    ├── audio.rs ←→ rodio（播放控制、进度、音量）
    ├── analyzer.rs ←→ audio.rs（RMS 音量、波形可视化）
    ├── playlist.rs ←→ 文件系统（目录扫描、歌曲列表）
    ├── lyrics.rs ←→ 文件系统 / 网络搜索（本地 LRC / 后台下载）
    ├── search.rs ←→ 网络（酷我+网易搜索、评论拉取、在线下载）
    ├── config.rs ←→ config.json（配置持久化）
    └── defs.rs（公共数据结构）
```

---

## 🎮 核心功能实现

### 1. 播放模式

```rust
pub enum PlayMode {
    Single,      // 单曲播放
    RepeatOne,   // 单曲循环
    Sequence,    // 顺序播放
    LoopAll,     // 列表循环
    Random,      // 随机播放
}

impl Playlist {
    pub fn next_index(&self, mode: PlayMode) -> Option<usize> {
        match mode {
            PlayMode::Single => None,              // 停止
            PlayMode::RepeatOne => Some(current),  // 重复当前
            PlayMode::Sequence => {                 // 下一首或停止
                if current + 1 < total { Some(current + 1) } else { None }
            }
            PlayMode::LoopAll => Some((current + 1) % total),
            PlayMode::Random => {                  // 随机（排除当前）
                rand::thread_rng().gen_range(0..total)
            }
        }
    }
}
```

### 2. 音频播放

```rust
pub struct AudioPlayer {
    _stream: Option<OutputStream>,
    _stream_handle: Option<OutputStreamHandle>,
    sink: Option<Sink>,
    state: Arc<Mutex<PlayState>>,
    analyzer: AudioAnalyzer,
}

impl AudioPlayer {
    pub fn play(&mut self, file: &MusicFile) -> Result<(), String> {
        // 1. 停止当前播放
        self.stop();

        // 2. 初始化音频输出
        let (stream, stream_handle) = OutputStream::try_default()?;

        // 3. 加载音频文件
        let audio_file = File::open(&file.path)?;
        let source = Decoder::new(BufReader::new(audio_file))?;

        // 4. 创建 Sink 并播放
        let sink = Sink::try_new(&stream_handle)?;
        sink.append(source);

        // 5. 设置音量
        sink.set_volume(self.volume as f32 / 100.0);

        // 6. 保存状态
        self.sink = Some(sink);
        self.state = PlayState::Playing;
        Ok(())
    }
}
```

### 3. 音频分析器（波形可视化）

```rust
pub struct AudioAnalyzer {
    rms_value: f32,       // 当前 RMS 音量
    ema_value: f32,       // EMA 平滑值
    ema_alpha: f32,       // 平滑系数
    bar_values: Vec<f32>, // 波形条高度
}

impl AudioAnalyzer {
    /// 更新 RMS 值，使用 EMA 平滑
    pub fn update(&mut self, rms: f32) {
        self.ema_value = self.ema_alpha * rms + (1.0 - self.ema_alpha) * self.ema_value;
    }

    /// 获取波形条高度用于渲染
    pub fn get_bar_heights(&self) -> &[f32] {
        &self.bar_values
    }
}
```

### 4. 歌词系统

```rust
pub struct LyricsManager {
    lyrics: Vec<(u64, String)>,     // (时间戳ms, 歌词文本)
    current_line: usize,             // 当前行索引
    cache: HashMap<String, Vec<(u64, String)>>, // 歌词缓存
}

impl LyricsManager {
    /// 加载本地 LRC 歌词文件
    pub fn load_local(&mut self, song_name: &str, music_dir: &Path) {
        // 自动查找同名 .lrc 文件
        // 支持 UTF-8 / GBK 编码自动检测
    }

    /// 后台异步下载歌词
    pub fn download_async(&self, song_name: &str, artist: &str) {
        // 在后台线程中从网络下载歌词
        // 不阻塞 UI
    }

    /// 根据当前播放时间获取歌词行
    pub fn get_line_at(&self, time_ms: u64) -> usize {
        // 二分查找当前时间对应的歌词行
    }
}
```

### 5. 网络搜索

```rust
pub struct SearchService;

impl SearchService {
    /// 酷我音乐搜索
    pub fn search_kuwo(keyword: &str, page: usize) -> Vec<SearchResult> { ... }

    /// 网易音乐搜索
    pub fn search_netease(keyword: &str, page: usize) -> Vec<SearchResult> { ... }

    /// 在线下载歌曲
    pub fn download(url: &str, save_path: &Path) -> Result<(), String> { ... }

    /// 获取歌曲评论
    pub fn get_comments(song_id: u64, page: usize) -> Vec<Comment> { ... }
}
```

### 6. 配置持久化

```rust
#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub music_directory: Option<String>,
    pub play_mode: u8,
    pub current_index: Option<usize>,
    pub volume: u16,
    pub favorites: Vec<String>,
    pub dir_history: Vec<String>,
    pub theme: String,
}

impl AppConfig {
    /// 从 config.json 加载
    pub fn load() -> Self { ... }

    /// 保存到 config.json
    pub fn save(&self) -> Result<(), String> { ... }
}
```

**自动保存时机**：切歌、切换主题、收藏变更、每 30 秒、退出时（含 Ctrl+C）

### 7. 主题系统

```rust
pub struct Theme {
    pub name: String,
    pub title_fg: Color,       // 标题前景色
    pub highlight: Color,      // 高亮色
    pub playing: Color,        // 播放中颜色
    pub progress_bg: Color,    // 进度条背景
    pub progress_fg: Color,    // 进度条前景
    pub lyrics: Color,         // 歌词颜色
    pub waveform: Color,       // 波形颜色
}

// 4 种主题循环切换
const THEMES: [&str; 4] = ["Neon", "Sunset", "Ocean", "GrayWhite"];
```

---

## 🔧 开发技巧

### 1. 使用 Arc<Mutex<T>> 共享状态

```rust
// 在多个线程/模块间共享数据
let playlist = Arc::new(Mutex::new(Playlist::new()));
let audio_player = Arc::new(Mutex::new(AudioPlayer::new()));

// 克隆引用传递
let playlist_clone = Arc::clone(&playlist);
let audio_clone = Arc::clone(&audio_player);

// 使用时加锁
let mut pl = playlist.lock().unwrap();
pl.add(file);
```

### 2. 错误处理

```rust
// 使用 Result 传递错误
pub fn play(&mut self, file: &MusicFile) -> Result<(), String> {
    let audio_file = std::fs::File::open(&file.path)
        .map_err(|e| format!("无法打开文件: {}", e))?;

    let source = Decoder::new(BufReader::new(audio_file))
        .map_err(|e| format!("无法解码音频: {}", e))?;

    Ok(())
}
```

### 3. 播放完成检测

```rust
// 检查 sink 是否为空
pub fn is_finished(&self) -> bool {
    if let Some(sink) = &self.sink {
        sink.empty()
    } else {
        true
    }
}

// 在主循环中检测
if audio_player.get_state() == PlayState::Playing && audio_player.is_finished() {
    self.play_next();
}
```

### 4. 后台异步任务

```rust
// 歌词下载 - 在后台线程执行，不阻塞 UI
std::thread::spawn(move || {
    if let Ok(lyrics) = SearchService::download_lyrics(&song, &artist) {
        // 通过共享状态更新歌词
        let mut lm = lyrics_manager.lock().unwrap();
        lm.set_lyrics(song, lyrics);
    }
});

// 评论加载 - 同样后台线程
std::thread::spawn(move || {
    let comments = SearchService::get_comments(song_id, page);
    let mut ui_state = state.lock().unwrap();
    ui_state.comments = Some(comments);
});
```

### 5. 50ms 事件循环

```rust
// 精确 50ms 刷新间隔，保证流畅体验
loop {
    let start = Instant::now();

    // 绘制界面
    self.draw()?;

    // 处理事件（非阻塞）
    while event::poll(Duration::from_millis(0))? {
        if let Event::Key(key) = event::read()? {
            self.handle_key_event(key.code)?;
        }
    }

    // 精确控制刷新间隔
    let elapsed = start.elapsed();
    if elapsed < Duration::from_millis(50) {
        std::thread::sleep(Duration::from_millis(50) - elapsed);
    }
}
```

---

## 📊 性能优化

### 1. 发布版本优化

```toml
[profile.release]
opt-level = 3       # 最高优化级别
lto = true          # 链接时优化
codegen-units = 1   # 单编译单元，更好优化
strip = true        # 去除调试符号
```

### 2. 减少锁竞争

```rust
// ✅ 好的做法：快速释放锁
{
    let playlist = playlist.lock().unwrap();
    let file = playlist.files.get(index).cloned();
} // 锁在这里释放

if let Some(file) = file {
    audio_player.lock().unwrap().play(&file);
}

// ❌ 不好的做法：长时间持有锁
let playlist = playlist.lock().unwrap();
let audio_player = audio_player.lock().unwrap();
// ... 大量代码 ...
```

### 3. 歌词智能缓存

```rust
// 歌词加载后缓存，切换歌曲再切回来不会重新解析
pub struct LyricsManager {
    cache: HashMap<String, Vec<(u64, String)>>,
}

impl LyricsManager {
    pub fn get_lyrics(&mut self, song: &str) -> Option<&Vec<(u64, String)>> {
        if self.cache.contains_key(song) {
            Some(&self.cache[song])
        } else {
            None
        }
    }
}
```

### 4. 并行获取时长

```rust
use rayon::prelude::*;

// 并行获取所有音频文件时长，2-4 倍提速
files.par_iter_mut().for_each(|file| {
    if let Ok(duration) = get_duration(&file.path) {
        file.duration = Some(duration);
    }
});
```

---

## 🧪 测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_mode_conversion() {
        assert_eq!(PlayMode::from_number(1), Some(PlayMode::Single));
        assert_eq!(PlayMode::from_number(2), Some(PlayMode::RepeatOne));
    }

    #[test]
    fn test_playlist_next() {
        let mut playlist = Playlist::new();
        playlist.add(MusicFile::new(PathBuf::from("test.mp3")));
        playlist.add(MusicFile::new(PathBuf::from("test2.mp3")));
        playlist.current_index = Some(0);

        assert_eq!(playlist.next_index(PlayMode::Sequence), Some(1));
        assert_eq!(playlist.next_index(PlayMode::Single), None);
    }

    #[test]
    fn test_lrc_parsing() {
        let lrc = "[00:01.00]Hello\n[00:05.00]World";
        let lyrics = parse_lrc(lrc);
        assert_eq!(lyrics.len(), 2);
        assert_eq!(lyrics[0].1, "Hello");
    }

    #[test]
    fn test_config_serialize() {
        let config = AppConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("volume"));
    }
}
```

---

## 🐧🍎 交叉编译

### Linux 版本

在 Windows 上使用 `cargo-zigbuild` + `zig` 交叉编译 Linux 可执行文件，无需安装 Linux 系统或虚拟机。

**安装依赖：**

```powershell
# 1. 安装 zig
pip install ziglang

# 2. 安装 cargo-zigbuild
cargo install cargo-zigbuild

# 3. 添加 Linux 目标
rustup target add x86_64-unknown-linux-gnu
```

**Linux sysroot**：项目已内置 `linux-sysroot/` 目录，包含 ALSA 开发文件。

如需自行准备，可从 Debian/Ubuntu 系统复制：
- `/usr/include/alsa/` → `linux-sysroot/usr/include/alsa/`
- `/usr/lib/x86_64-linux-gnu/libasound.so*` → `linux-sysroot/usr/lib/x86_64-linux-gnu/`

**编译：**

```powershell
# 使用构建脚本（自动设置环境变量）
build-linux.bat

# 或手动执行：
$env:PKG_CONFIG_PATH = "linux-sysroot\usr\lib\x86_64-linux-gnu\pkgconfig"
$env:PKG_CONFIG_ALLOW_CROSS = "1"
$env:PKG_CONFIG_SYSROOT_DIR = "linux-sysroot"
$env:RUSTFLAGS = "-L linux-sysroot\usr\lib\x86_64-linux-gnu"
cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.34
```

**输出文件**：`target/x86_64-unknown-linux-gnu/release/ter-music-rust`

> 目标 `x86_64-unknown-linux-gnu.2.34` 中的 `.2.34` 指定 glibc 最低版本，确保在较旧 Linux 系统上也能运行。

**部署到 Linux：**

```bash
scp ter-music-rust user@linux-host:~/
chmod +x ter-music-rust
sudo apt install libasound2
./ter-music-rust -o /path/to/music
```

### macOS 版本

在 Windows 上使用 `cargo-zigbuild` + `zig` + macOS SDK 交叉编译。macOS 使用系统 CoreAudio，需要额外的 SDK 头文件和 libclang。

**安装依赖：**

```powershell
# 1. 安装 zig
pip install ziglang

# 2. 安装 cargo-zigbuild
cargo install cargo-zigbuild

# 3. 安装 LLVM/Clang（提供 libclang.dll，bindgen 需要）
# MSYS2 方式：
pacman -S mingw-w64-x86_64-clang
# 或 LLVM 官方：
winget install LLVM.LLVM

# 4. 添加 macOS 目标
rustup target add x86_64-apple-darwin aarch64-apple-darwin
```

**准备 macOS SDK：**

把MacOSX13.3.sdk.tar.xz解压到macos-sysroot目录
项目已内置 `macos-sysroot/` 目录（从 [macosx-sdks](https://github.com/joseluisq/macosx-sdks) 下载）。

如需重新获取：

```powershell
# 从 GitHub 下载预打包 SDK（约 56 MB）
加速: https://ghfast.top/https://github.com/joseluisq/macosx-sdks/releases/download/13.3/MacOSX13.3.sdk.tar.xz
curl -L -o MacOSX13.3.sdk.tar.xz https://github.com/joseluisq/macosx-sdks/releases/download/13.3/MacOSX13.3.sdk.tar.xz
mkdir macos-sysroot
tar -xf MacOSX13.3.sdk.tar.xz -C macos-sysroot --strip-components=1
del MacOSX13.3.sdk.tar.xz

# 或从 macOS 系统复制
scp -r mac:/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk ./macos-sysroot
```

**编译：**

```powershell
# 使用构建脚本（自动设置所有环境变量）
build-mac.bat

# 或手动执行：
$env:LIBCLANG_PATH = "C:\msys64\mingw64\bin"
$env:COREAUDIO_SDK_PATH = "./macos-sysroot"       # 正斜杠路径
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

**部署到 macOS：**

```bash
scp ter-music-rust user@mac-host:~/
chmod +x ter-music-rust
xattr -cr ter-music-rust    # macos允许运行未知来源应用
./ter-music-rust -o /path/to/music
```

> **注意**：macOS 交叉编译需要 macOS SDK 头文件，项目已内置 `macos-sysroot/` 目录（从 [macosx-sdks](https://github.com/joseluisq/macosx-sdks) 获取）。
> 编译时还需 `libclang.dll`（通过 MSYS2 或 LLVM 安装）。

---

## 🛠️ 编译环境

### 方案一：MSVC 工具链（推荐，兼容性最好）

```powershell
# 1. 安装 Rust
winget install Rustlang.Rustup

# 2. 安装 Build Tools
winget install Microsoft.VisualStudio.2022.BuildTools
# 运行安装器 → 选择"使用 C++ 的桌面开发" → 安装

# 3. 重启终端并编译
cargo build --release
```

### 方案二：GNU 工具链（轻量，约 300 MB）

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

## 📚 学习资源

- **Rust 官方书籍**: https://doc.rust-lang.org/book/
- **rodio 文档**: https://docs.rs/rodio/
- **crossterm 文档**: https://docs.rs/crossterm/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
- **cargo-zigbuild**: https://github.com/rust-cross/cargo-zigbuild

---

## 🚀 下一步

1. ✅ 阅读源代码注释
2. 🔧 尝试修改功能
3. 🎨 自定义界面主题
4. 📦 添加新功能（如：均衡器、播放列表导出、快捷键自定义等）
5. 🐧 在 Linux 上运行和测试

祝编码愉快！
