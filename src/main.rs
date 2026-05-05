// Ter-Music-Rust: 终端音乐播放器

mod audio;
mod analyzer;
mod config;
mod defs;
mod langs;
mod lyrics;
mod playlist;
mod search;
mod ui;

use std::env;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use audio::AudioPlayer;
use config::Config;
use defs::Playlist;
use ui::UserInterface;

/// 设置控制台代码页为 UTF-8（仅 Windows）
#[cfg(windows)]
fn setup_console() {
    use winapi::um::wincon::{SetConsoleCP, SetConsoleOutputCP};
    unsafe {
        SetConsoleOutputCP(65001);
        SetConsoleCP(65001);
    }
}

#[cfg(not(windows))]
fn setup_console() {}

/// 显示帮助信息
fn show_help(lang: &str) {
    let ui_lang = langs::UiLanguage::from_config_key(lang);
    let texts = ui_lang.texts();
    for line in texts.cli_help_lines {
        println!("{}", line);
    }
    println!();
}

/// 主函数
#[allow(clippy::arc_with_non_send_sync)]
fn main() {
    setup_console();
    let mut config = Config::load();

    // 确保 api_base_url 和 api_model 有默认值（兼容旧配置文件）
    if config.api_base_url.trim().is_empty() {
        config.api_base_url = "https://api.deepseek.com/".to_string();
    }
    if config.api_model.trim().is_empty() {
        config.api_model = "deepseek-v4-flash".to_string();
    }

    let lang = config.language.clone();
    let ui_lang = langs::UiLanguage::from_config_key(&lang);
    let texts = ui_lang.texts();
    langs::set_global_language(ui_lang);

    // 解析命令行参数
    let args: Vec<String> = env::args().collect();
    let mut music_dir: Option<PathBuf> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                show_help(&lang);
                return;
            }
            "-o" => {
                if i + 1 < args.len() {
                    music_dir = Some(PathBuf::from(&args[i + 1]));
                    i += 1;
                } else {
                    eprintln!("{}", texts.cli_error_option_o);
                    std::process::exit(1);
                }
            }
            _ => {
                eprintln!("{} '{}'", texts.cli_error_unknown_option, args[i]);
                eprintln!("{}", texts.cli_use_help);
                std::process::exit(1);
            }
        }
        i += 1;
    }

    // 如果没有指定目录，尝试从配置文件加载
    if music_dir.is_none() {
        if let Some(dir) = &config.music_directory {
            music_dir = Some(PathBuf::from(dir));
        }
    }

    // 先尝试加载已有目录（命令行或配置）
    let loaded_playlist = music_dir
        .as_ref()
        .and_then(|dir| playlist::scan_music_directory(dir).ok())
        .map(|pl| Arc::new(Mutex::new(pl)));

    // 若未成功加载，使用默认音乐目录（配置目录/music），并设置到配置中
    let (playlist, need_startup_dialog) = match loaded_playlist {
        Some(pl) => (pl, false),
        None => {
            eprintln!("{}", texts.cli_no_dir_selected);
            let default_music_dir = config::get_default_music_dir();
            let default_music_dir_str = default_music_dir.to_string_lossy().to_string();
            // 设置 music_directory 为默认目录
            config.music_directory = Some(default_music_dir_str.clone());
            // 添加到 dir_history（如果不存在）
            if !config.dir_history.iter().any(|p| *p == default_music_dir_str) {
                config.dir_history.push(default_music_dir_str);
            }
            // 尝试扫描默认目录（可能有之前下载的歌曲）
            let pl = playlist::scan_music_directory(&default_music_dir)
                .map(|pl| Arc::new(Mutex::new(pl)))
                .unwrap_or_else(|_| Arc::new(Mutex::new(Playlist::new())));
            (pl, true)
        }
    };

    // 创建音频播放器
    let audio_player = Arc::new(Mutex::new(AudioPlayer::new()));

    // 应用配置：设置播放模式
    {
        let mut player = audio_player.lock().unwrap();
        let play_mode = Config::string_to_play_mode(&config.play_mode);
        player.set_play_mode(play_mode);
        player.set_volume(config.volume);
    }

    // 创建用户界面
    let mut ui = UserInterface::new(playlist.clone(), audio_player.clone());
    if need_startup_dialog {
        ui.set_need_startup_dialog(true);
    }
    ui.set_theme_by_name(&config.theme);
    ui.set_language_by_name(&config.language);
    ui.set_api_key(config.api_key.clone());
    ui.set_api_base_url(config.api_base_url.clone());
    ui.set_api_model(config.api_model.clone());
    ui.set_github_token(config.github_token.clone());

    // 注册 Ctrl+C 信号处理器，优雅退出并保存配置
    {
        let should_quit = ui.get_should_quit();
        ctrlc::set_handler(move || {
            *should_quit.lock().unwrap() = true;
        }).expect(texts.cli_ctrlc_error);
    }

    // 从配置加载收藏列表
    ui.set_favorites(config.favorites.clone());

    // 从配置加载目录历史
    ui.set_dir_history(config.dir_history.clone());

    // 启动后自动播放：优先恢复上次索引，否则首次运行播放第一首
    let startup_index = {
        let playlist_len = playlist.lock().unwrap().len();
        if playlist_len == 0 {
            None
        } else if let Some(index) = config.current_index {
            if index < playlist_len {
                Some(index)
            } else {
                Some(0)
            }
        } else {
            Some(0)
        }
    };

    if let Some(index) = startup_index {
        ui.set_selected_index(index);

        let file = playlist.lock().unwrap().files.get(index).cloned();
        if let Some(file) = file {
            let play_result = {
                let mut player = audio_player.lock().unwrap();
                player.play(&file)
            };
            if play_result.is_ok() {
                playlist.lock().unwrap().current_index = Some(index);
                ui.update_now_playing_status(&file.name);
            }
        }
    }

    // 运行主循环
    if let Err(e) = ui.run() {
        eprintln!("{}: {}", texts.cli_playback_error, e);
        std::process::exit(1);
    }

    // 保存配置
    {
        let player = audio_player.lock().unwrap();
        let pl = playlist.lock().unwrap();

        let new_config = Config {
            music_directory: pl.directory.clone(),
            play_mode: Config::play_mode_to_string(player.get_play_mode()),
            current_index: pl.current_index,
            volume: player.get_volume(),
            favorites: ui.get_favorites(),
            dir_history: ui.get_dir_history(),
            theme: ui.get_theme_key().to_string(),
            language: ui.get_language_key().to_string(),
            api_key: ui.get_api_key(),
            api_base_url: ui.get_api_base_url(),
            api_model: ui.get_api_model(),
            github_token: ui.get_github_token(),
        };

        new_config.save();
    }

    // 清理
    audio_player.lock().unwrap().stop();
}
