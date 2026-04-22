// 配置文件管理模块

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::defs::PlayMode;

fn default_theme() -> String {
    "Neon".to_string()
}

fn default_language() -> String {
    "zh-CN".to_string()
}

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// 音乐目录路径
    pub music_directory: Option<String>,
    /// 播放模式
    pub play_mode: String,
    /// 当前播放索引
    pub current_index: Option<usize>,
    /// 音量
    pub volume: u8,
    /// 收藏列表（存储歌曲文件路径）
    #[serde(default)]
    pub favorites: Vec<String>,
    /// 目录历史记录（存储目录路径）
    #[serde(default)]
    pub dir_history: Vec<String>,
    /// 界面主题
    #[serde(default = "default_theme")]
    pub theme: String,
    /// 界面语言
    #[serde(default = "default_language")]
    pub language: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            music_directory: None,
            play_mode: "Single".to_string(),
            current_index: None,
            volume: 50,
            favorites: Vec::new(),
            dir_history: Vec::new(),
            theme: default_theme(),
            language: default_language(),
        }
    }
}

impl Config {
    /// 创建新的配置
    #[allow(dead_code)]
    pub fn new() -> Self {
        Config::default()
    }

    /// 判断配置是否包含用户历史数据（非纯默认空配置）
    fn has_user_data(&self) -> bool {
        self.music_directory.is_some()
            || self.current_index.is_some()
            || !self.favorites.is_empty()
            || !self.dir_history.is_empty()
            || self.play_mode != "Single"
            || self.volume != 50
            || self.theme != default_theme()
            || self.language != default_language()
    }

    /// 获取旧版配置文件路径（程序所在目录）
    fn get_legacy_config_path() -> PathBuf {
        let exe_path = env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
        let exe_dir = exe_path.parent().unwrap_or(Path::new("."));
        exe_dir.join("config.json")
    }

    /// 获取旧版配置候选路径（程序目录、当前工作目录）
    fn get_legacy_config_candidates() -> Vec<PathBuf> {
        let mut candidates = Vec::new();

        let exe_cfg = Self::get_legacy_config_path();
        candidates.push(exe_cfg.clone());

        if let Ok(cwd) = env::current_dir() {
            let cwd_cfg = cwd.join("config.json");
            if cwd_cfg != exe_cfg {
                candidates.push(cwd_cfg);
            }
        }

        candidates
    }

    /// 获取用户可写配置文件路径
    fn get_config_path() -> PathBuf {
        #[cfg(target_os = "windows")]
        {
            if let Ok(appdata) = env::var("APPDATA") {
                return PathBuf::from(appdata).join("ter-music-rust").join("config.json");
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
                if !xdg.trim().is_empty() {
                    return PathBuf::from(xdg).join("ter-music-rust").join("config.json");
                }
            }
            if let Ok(home) = env::var("HOME") {
                return PathBuf::from(home)
                    .join(".config")
                    .join("ter-music-rust")
                    .join("config.json");
            }
        }

        // 环境变量不可用时，回退到旧路径
        Self::get_legacy_config_path()
    }

    /// 从文件加载配置
    pub fn load() -> Self {
        let mut candidates = Vec::new();

        // 新路径优先
        candidates.push(Self::get_config_path());
        // 兼容旧路径（程序目录、当前工作目录）
        for p in Self::get_legacy_config_candidates() {
            if !candidates.iter().any(|x| x == &p) {
                candidates.push(p);
            }
        }

        let mut fallback_config: Option<Self> = None;

        for config_path in candidates {
            if !config_path.exists() {
                continue;
            }

            match fs::read_to_string(&config_path) {
                Ok(content) => match serde_json::from_str::<Config>(&content) {
                    Ok(config) => {
                        // 优先返回包含用户数据的配置，避免被“新路径默认空配置”遮住
                        if config.has_user_data() {
                            return config;
                        }

                        if fallback_config.is_none() {
                            fallback_config = Some(config);
                        }
                    }
                    Err(e) => {
                        eprintln!("配置文件格式错误({}): {}，尝试下一个配置", config_path.display(), e);
                    }
                },
                Err(e) => {
                    eprintln!("无法读取配置文件({}): {}，尝试下一个配置", config_path.display(), e);
                }
            }
        }

        fallback_config.unwrap_or_default()
    }

    /// 保存配置到文件
    pub fn save(&self) {
        let config_path = Self::get_config_path();

        if let Some(parent) = config_path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!("无法创建配置目录: {}", e);
                return;
            }
        }

        match serde_json::to_string_pretty(self) {
            Ok(json) => match fs::write(&config_path, json) {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("无法保存配置文件: {}", e);
                }
            },
            Err(e) => {
                eprintln!("无法序列化配置: {}", e);
            }
        }
    }

    /// 从播放模式枚举转换为字符串
    pub fn play_mode_to_string(mode: PlayMode) -> String {
        mode.to_string()
    }

    /// 从字符串转换为播放模式枚举
    pub fn string_to_play_mode(s: &str) -> PlayMode {
        s.parse().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.music_directory, None);
        assert_eq!(config.play_mode, "Single");
        assert_eq!(config.volume, 50);
        assert_eq!(config.theme, "Neon");
        assert_eq!(config.language, "zh-CN");
    }

    #[test]
    fn test_play_mode_conversion() {
        assert_eq!(Config::play_mode_to_string(PlayMode::Single), "Single");
        assert_eq!(
            Config::string_to_play_mode("RepeatOne"),
            PlayMode::RepeatOne
        );
    }
}
