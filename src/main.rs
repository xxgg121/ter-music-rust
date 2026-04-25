// Ter-Music-Rust: 终端音乐播放器

mod audio;
mod analyzer;
mod config;
mod defs;
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
fn show_help() {
    println!("Ter-Music-Rust - 终端音乐播放器\n");
    println!("程序用法:");
    println!(" ter-music-rust [选项]\n");
    println!("参数选项:");
    println!(" -o <目录> 打开音乐目录");
    println!(" -h, --help 显示帮助信息\n");
    println!("快捷按键:");
    println!(" ↑/↓ 上下选择歌曲");
    println!(" Enter 播放选中歌曲");
    println!(" Space 播放/暂停歌曲");
    println!(" Esc 停止播放歌曲");
    println!(" ←/→ 上一曲/下一曲");
    println!(" [/] 快退/快进5秒");
    println!(" ,/. 快退/快进10秒");
    println!(" +/- 音量大小加减");
    println!(" 1-5 切换播放模式");
    println!(" o 打开音乐目录");
    println!(" s 搜索本地歌曲");
    println!(" n 搜索网络歌曲");
    println!(" i 查看歌曲信息");
    println!(" f 添加到收藏夹");
    println!(" v 查看收藏列表");
    println!(" m 音乐目录历史");
    println!(" h 显示帮助信息");
    println!(" c 显示歌曲评论");
    println!(" l 切换界面语言");
    println!(" t 切换界面主题");
    println!(" k 设置API Key");
    println!(" q 退出音乐程序\n");
    println!("播放模式:");
    println!(" 1 - 单曲播放（歌曲播放完停止）");
    println!(" 2 - 单曲循环（循环播放当前歌曲）");
    println!(" 3 - 顺序播放（顺序播放完后回到第一首）");
    println!(" 4 - 列表循环（循环播放整个列表）");
    println!(" 5 - 随机播放（随机选择播放歌曲）\n");
    println!("支持格式:");
    println!("MP3、WAV、FLAC、OGG、OGA、Opus、M4A、AAC、AIFF、APE\n");
    println!("配置文件:");
    println!(" 配置路径: 用户配置目录/ter-music-rust/config.json");
    println!(" 自动保存: 音乐目录、播放模式、音量大小、收藏列表、当前歌曲、当前主题、当前语言\n");
}

/// 主函数
#[allow(clippy::arc_with_non_send_sync)]
fn main() {
    setup_console();
    let config = Config::load();

    if std::env::var("DEEPSEEK_API_KEY").map(|v| v.trim().is_empty()).unwrap_or(true)
        && !config.deepseek_api_key.trim().is_empty()
    {
        std::env::set_var("DEEPSEEK_API_KEY", config.deepseek_api_key.trim());
    }

    // 解析命令行参数
    let args: Vec<String> = env::args().collect();
    let mut music_dir: Option<PathBuf> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                show_help();
                return;
            }
            "-o" => {
                if i + 1 < args.len() {
                    music_dir = Some(PathBuf::from(&args[i + 1]));
                    i += 1;
                } else {
                    eprintln!("错误: -o 选项需要打开音乐目录");
                    std::process::exit(1);
                }
            }
            _ => {
                eprintln!("错误: 未知选项 '{}'", args[i]);
                eprintln!("使用 -h 或 --help 查看帮助信息");
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
    let mut loaded_playlist = music_dir
        .as_ref()
        .and_then(|dir| playlist::scan_music_directory(dir).ok())
        .map(|pl| Arc::new(Mutex::new(pl)));

    // 若未成功加载，弹出图形对话框让用户选择目录（可重复选择）
    while loaded_playlist.is_none() {
        let selected_dir = playlist::open_folder_dialog();
        match selected_dir {
            Some(dir) => {
                loaded_playlist = playlist::scan_music_directory(&dir)
                    .ok()
                    .map(|pl| Arc::new(Mutex::new(pl)));
            }
            None => break,
        }
    }

    // 若最终仍未选择到可用目录，则进入空列表模式（可按 o 再次选择目录）
    let playlist = match loaded_playlist {
        Some(pl) => pl,
        None => {
            eprintln!("未选择可用的音乐目录，已进入空列表模式，可按 o 打开音乐目录");
            Arc::new(Mutex::new(Playlist::new()))
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
    ui.set_theme_by_name(&config.theme);
    ui.set_language_by_name(&config.language);
    ui.set_deepseek_api_key(config.deepseek_api_key.clone());

    // 注册 Ctrl+C 信号处理器，优雅退出并保存配置
    {
        let should_quit = ui.get_should_quit();
        ctrlc::set_handler(move || {
            *should_quit.lock().unwrap() = true;
        }).expect("无法设置 Ctrl+C 处理器");
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
        eprintln!("播放错误: {}", e);
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
            deepseek_api_key: ui.get_deepseek_api_key(),
        };

        new_config.save();
    }

    // 清理
    audio_player.lock().unwrap().stop();
}
