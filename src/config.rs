// 配置文件管理模块

use std::env;
use std::fs;
use std::path::PathBuf;

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

    /// 获取配置文件路径
    fn get_config_path() -> PathBuf {
        // 配置文件保存在程序所在目录下
        let exe_path = env::current_exe().unwrap_or_else(|_| PathBuf::from("."));

        let exe_dir = exe_path.parent().unwrap_or(std::path::Path::new("."));

        exe_dir.join("config.json")
    }

    /// 从文件加载配置
    pub fn load() -> Self {
        let config_path = Self::get_config_path();

        if config_path.exists() {
            match fs::read_to_string(&config_path) {
                Ok(content) => match serde_json::from_str(&content) {
                    Ok(config) => {
                        return config;
                    }
                    Err(e) => {
                        eprintln!("配置文件格式错误: {}，使用默认配置", e);
                    }
                },
                Err(e) => {
                    eprintln!("无法读取配置文件: {}，使用默认配置", e);
                }
            }
        }

        Config::default()
    }

    /// 保存配置到文件
    pub fn save(&self) {
        let config_path = Self::get_config_path();

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
