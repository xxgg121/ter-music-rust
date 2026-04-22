// 公共定义和数据结构

use std::fmt;
use std::path::{Path, PathBuf};

/// 播放模式枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlayMode {
    /// 单曲播放：播放完停止
    #[default]
    Single,
    /// 单曲循环：循环播放当前歌曲
    RepeatOne,
    /// 顺序播放：顺序播放，最后回到第一首
    Sequence,
    /// 循环播放：循环播放整个列表
    LoopAll,
    /// 随机播放：随机选择歌曲
    Random,
}

impl PlayMode {
    /// 获取播放模式的中文名称
    #[allow(dead_code)]
    pub fn to_string_cn(self) -> &'static str {
        match self {
            PlayMode::Single => "单曲播放",
            PlayMode::RepeatOne => "单曲循环",
            PlayMode::Sequence => "顺序播放",
            PlayMode::LoopAll => "循环播放",
            PlayMode::Random => "随机播放",
        }
    }

    /// 获取播放模式的英文名称
    #[allow(dead_code)]
    pub fn to_string_en(self) -> &'static str {
        match self {
            PlayMode::Single => "Single",
            PlayMode::RepeatOne => "Repeat One",
            PlayMode::Sequence => "Sequence",
            PlayMode::LoopAll => "Loop All",
            PlayMode::Random => "Random",
        }
    }

    /// 从数字转换为播放模式
    #[allow(dead_code)]
    pub fn from_number(num: u8) -> Option<Self> {
        match num {
            1 => Some(PlayMode::Single),
            2 => Some(PlayMode::RepeatOne),
            3 => Some(PlayMode::Sequence),
            4 => Some(PlayMode::LoopAll),
            5 => Some(PlayMode::Random),
            _ => None,
        }
    }

    /// 转换为数字
    #[allow(dead_code)]
    pub fn to_number(self) -> u8 {
        match self {
            PlayMode::Single => 1,
            PlayMode::RepeatOne => 2,
            PlayMode::Sequence => 3,
            PlayMode::LoopAll => 4,
            PlayMode::Random => 5,
        }
    }
}


impl fmt::Display for PlayMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlayMode::Single => write!(f, "Single"),
            PlayMode::RepeatOne => write!(f, "RepeatOne"),
            PlayMode::Sequence => write!(f, "Sequence"),
            PlayMode::LoopAll => write!(f, "LoopAll"),
            PlayMode::Random => write!(f, "Random"),
        }
    }
}

impl std::str::FromStr for PlayMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RepeatOne" => Ok(PlayMode::RepeatOne),
            "Sequence" => Ok(PlayMode::Sequence),
            "LoopAll" => Ok(PlayMode::LoopAll),
            "Random" => Ok(PlayMode::Random),
            _ => Ok(PlayMode::Single),
        }
    }
}

/// 播放状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlayState {
    /// 停止
    #[default]
    Stopped,
    /// 播放中
    Playing,
    /// 暂停
    Paused,
}


/// 音乐文件信息
#[derive(Debug, Clone)]
pub struct MusicFile {
    /// 文件路径
    pub path: PathBuf,
    /// 文件名（不含扩展名）
    pub name: String,
    /// 文件扩展名
    #[allow(dead_code)]
    pub ext: String,
    /// 歌曲时长
    pub duration: Option<std::time::Duration>,
}

impl MusicFile {
    #[allow(dead_code)]
    pub fn new(path: PathBuf) -> Self {
        Self::with_duration(path, None)
    }

    pub fn with_duration(path: PathBuf, duration: Option<std::time::Duration>) -> Self {
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .to_string();

        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        MusicFile {
            path,
            name,
            ext,
            duration,
        }
    }

    pub fn format_duration(&self) -> String {
        if let Some(dur) = self.duration {
            let secs = dur.as_secs();
            let mins = secs / 60;
            let secs = secs % 60;
            format!("{:02}:{:02}", mins, secs)
        } else {
            "--:--".to_string()
        }
    }
}

/// 播放列表
#[derive(Debug, Clone)]
pub struct Playlist {
    /// 音乐文件列表
    pub files: Vec<MusicFile>,
    /// 当前播放索引
    pub current_index: Option<usize>,
    /// 音乐目录路径
    pub directory: Option<String>,
}

impl Playlist {
    pub fn new() -> Self {
        Playlist {
            files: Vec::new(),
            current_index: None,
            directory: None,
        }
    }

    /// 添加音乐文件
    #[allow(dead_code)]
    pub fn add(&mut self, file: MusicFile) {
        self.files.push(file);
    }

    /// 获取当前音乐文件
    #[allow(dead_code)]
    pub fn current(&self) -> Option<&MusicFile> {
        self.current_index.and_then(|i| self.files.get(i))
    }

    /// 获取列表长度
    pub fn len(&self) -> usize {
        self.files.len()
    }

    /// 判断是否为空
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    /// 获取下一首索引（自动播放时调用）
    /// `manual`: 是否为手动切换（用户按了下一曲）
    pub fn next_index(&self, mode: PlayMode, manual: bool) -> Option<usize> {
        if self.files.is_empty() {
            return None;
        }

        let current = self.current_index.unwrap_or(0);
        let total = self.files.len();

        match mode {
            PlayMode::Single => {
                // 单曲播放：自动播放完停止，手动切换允许到下一首
                if manual {
                    Some((current + 1) % total)
                } else {
                    None
                }
            }
            PlayMode::RepeatOne => {
                // 单曲循环：自动/手动都播放当前歌曲
                Some(current)
            }
            PlayMode::Sequence => {
                // 顺序播放：到最后一首后停止（手动可循环）
                if current + 1 >= total {
                    if manual {
                        Some(0)
                    } else {
                        None
                    }
                } else {
                    Some(current + 1)
                }
            }
            PlayMode::LoopAll => {
                // 列表循环：循环播放整个列表
                Some((current + 1) % total)
            }
            PlayMode::Random => {
                // 随机播放：随机选择一首（排除当前歌曲）
                use rand::Rng;
                let mut rng = rand::thread_rng();
                if total <= 1 {
                    Some(0)
                } else {
                    let mut next = rng.gen_range(0..total - 1);
                    if next >= current {
                        next += 1;
                    }
                    Some(next)
                }
            }
        }
    }

    /// 获取上一首索引
    pub fn prev_index(&self, mode: PlayMode) -> Option<usize> {
        if self.files.is_empty() {
            return None;
        }

        let current = self.current_index.unwrap_or(0);
        let total = self.files.len();

        match mode {
            PlayMode::Random => {
                // 随机播放：随机选择一首（排除当前歌曲）
                use rand::Rng;
                let mut rng = rand::thread_rng();
                if total <= 1 {
                    Some(0)
                } else {
                    let mut prev = rng.gen_range(0..total - 1);
                    if prev >= current {
                        prev += 1;
                    }
                    Some(prev)
                }
            }
            _ => {
                // 其他模式：返回上一首索引，如果当前是第一首，则返回最后一首
                Some(if current == 0 { total - 1 } else { current - 1 })
            }
        }
    }
}

impl Default for Playlist {
    fn default() -> Self {
        Self::new()
    }
}

/// 支持的音频格式
pub const SUPPORTED_FORMATS: &[&str] = &[
    "mp3", "wav", "flac", "ogg", "oga", "opus", "m4a", "aac", "aiff", "ape",
];

/// 判断文件是否为支持的音频格式
pub fn is_supported_audio(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| {
            let ext_lower = ext.to_lowercase();
            SUPPORTED_FORMATS.contains(&ext_lower.as_str())
        })
        .unwrap_or(false)
}
