// 用户界面模块

use std::collections::VecDeque;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use chrono::Local;
use crossterm::{
    cursor,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers, MouseButton, MouseEvent, MouseEventKind},
    execute, queue, style,
    terminal::{self, ClearType},
};

use crate::audio::AudioPlayer;
use crate::defs::{PlayMode, PlayState, Playlist};
use crate::langs::{UiLanguage, LangTexts};
use crate::lyrics::{Lyrics, LyricsDownloadResult};
use crate::search::{OnlineSong, OnlinePlaylist, SearchDownloadResult, PlaylistSearchResult, PlaylistSongsResult, DownloadProgress, SongCommentItem, SongCommentsResult, GitHubDiscussionResult};

const DEFAULT_GITHUB_TOKEN: &str = "github_xxxxxx";

// 主题色定义（使用显式 RGB，避免不同系统终端默认色表差异）
#[derive(Debug, Clone, Copy)]
struct ThemeColors {
    header_title: style::Color,
    section_title: style::Color,
    song_normal: style::Color,
    song_playing: style::Color,
    lyric_highlight: style::Color,
    status_text: style::Color,
    progress_text: style::Color,
    info_text: style::Color,
}

#[derive(Debug, Clone, Copy)]
enum UiTheme {
    GrayWhite,
    Neon,
    Sunset,
    Ocean,
}

impl UiTheme {
    fn next(self) -> Self {
        match self {
            UiTheme::GrayWhite => UiTheme::Neon,
            UiTheme::Neon => UiTheme::Sunset,
            UiTheme::Sunset => UiTheme::Ocean,
            UiTheme::Ocean => UiTheme::GrayWhite,
        }
    }

    fn config_key(self) -> &'static str {
        match self {
            UiTheme::GrayWhite => "GrayWhite",
            UiTheme::Neon => "Neon",
            UiTheme::Sunset => "Sunset",
            UiTheme::Ocean => "Ocean",
        }
    }

    fn from_config_key(s: &str) -> Self {
        if s.eq_ignore_ascii_case("graywhite")
            || s.eq_ignore_ascii_case("gray")
            || s == "灰白"
            || s == "灰白色"
        {
            UiTheme::GrayWhite
        } else if s.eq_ignore_ascii_case("neon") {
            UiTheme::Neon
        } else if s.eq_ignore_ascii_case("sunset") {
            UiTheme::Sunset
        } else if s.eq_ignore_ascii_case("ocean") {
            UiTheme::Ocean
        } else {
            UiTheme::GrayWhite
        }
    }

    fn colors(self) -> ThemeColors {
        match self {
            UiTheme::GrayWhite => ThemeColors {
                header_title: style::Color::Rgb { r: 238, g: 242, b: 246 },
                section_title: style::Color::Rgb { r: 223, g: 229, b: 235 },
                // 普通歌曲：中性灰
                song_normal: style::Color::Rgb { r: 188, g: 194, b: 202 },
                // 正在播放：更亮一点的冷白灰
                song_playing: style::Color::Rgb { r: 246, g: 250, b: 255 },
                // 歌词高亮：轻微偏蓝白，和播放列表形成层次
                lyric_highlight: style::Color::Rgb { r: 224, g: 233, b: 246 },
                status_text: style::Color::Rgb { r: 232, g: 237, b: 244 },
                progress_text: style::Color::Rgb { r: 210, g: 217, b: 226 },
                info_text: style::Color::Rgb { r: 216, g: 223, b: 232 },
            },
            UiTheme::Neon => ThemeColors {
                header_title: style::Color::Rgb { r: 0, g: 215, b: 255 },
                section_title: style::Color::Rgb { r: 255, g: 235, b: 80 },
                song_normal: style::Color::Rgb { r: 0, g: 255, b: 120 },
                song_playing: style::Color::Rgb { r: 0, g: 255, b: 120 },
                lyric_highlight: style::Color::Rgb { r: 255, g: 235, b: 80 },
                status_text: style::Color::Rgb { r: 255, g: 235, b: 80 },
                progress_text: style::Color::Rgb { r: 0, g: 170, b: 255 },
                info_text: style::Color::Rgb { r: 0, g: 215, b: 255 },
            },
            UiTheme::Sunset => ThemeColors {
                header_title: style::Color::Rgb { r: 255, g: 186, b: 73 },
                section_title: style::Color::Rgb { r: 255, g: 221, b: 124 },
                song_normal: style::Color::Rgb { r: 255, g: 197, b: 120 },
                song_playing: style::Color::Rgb { r: 255, g: 238, b: 176 },
                lyric_highlight: style::Color::Rgb { r: 255, g: 246, b: 120 },
                status_text: style::Color::Rgb { r: 255, g: 212, b: 96 },
                progress_text: style::Color::Rgb { r: 255, g: 170, b: 84 },
                info_text: style::Color::Rgb { r: 255, g: 205, b: 138 },
            },
            UiTheme::Ocean => ThemeColors {
                header_title: style::Color::Rgb { r: 102, g: 226, b: 255 },
                section_title: style::Color::Rgb { r: 126, g: 250, b: 228 },
                song_normal: style::Color::Rgb { r: 116, g: 243, b: 204 },
                song_playing: style::Color::Rgb { r: 166, g: 255, b: 234 },
                lyric_highlight: style::Color::Rgb { r: 168, g: 255, b: 245 },
                status_text: style::Color::Rgb { r: 134, g: 235, b: 255 },
              
              
                progress_text: style::Color::Rgb { r: 108, g: 188, b: 255 },
                info_text: style::Color::Rgb { r: 120, g: 224, b: 255 },
            },
        }
    }
}

/// 进度条布局信息（用于鼠标点击定位）
#[derive(Debug, Clone, Copy)]
struct ProgressBarLayout {
    /// 进度条所在行
    row: u16,
    /// 进度条方括号内的起始列（0-based）
    bar_start_col: usize,
    /// 进度条方括号内的宽度（字符数）
    bar_width: usize,
}

/// 音量条布局信息（用于鼠标点击定位）
#[derive(Debug, Clone, Copy)]
struct VolumeBarLayout {
    /// 音量条所在行
    row: u16,
    /// 音量条方括号内的起始列（0-based）
    bar_start_col: usize,
    /// 音量条方括号内的宽度（字符数，固定20）
    bar_width: usize,
}

/// 播放列表布局信息（用于鼠标交互）
#[derive(Debug, Clone, Copy)]
struct PlaylistLayout {
    /// 列表起始行（0-based）
    start_row: u16,
    /// 可见歌曲行数
    visible_count: usize,
    /// 左侧栏宽度
    left_width: u16,
}

/// 歌词区域布局信息（用于鼠标拖动跳转）
#[derive(Debug, Clone)]
struct LyricsAreaLayout {
    /// 歌词区域起始行（0-based）
    start_row: u16,
    /// 歌词区域起始列（0-based）
    start_col: usize,
    /// 歌词区域宽度
    width: usize,
    /// 当前可见歌词行对应的时间戳
    line_times: Vec<Duration>,
}

/// 终端保护器，确保在 Drop 时恢复终端
struct TerminalGuard;

impl TerminalGuard {
    fn new() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(io::stdout(), terminal::EnterAlternateScreen, cursor::Hide, EnableMouseCapture)?;
        Ok(TerminalGuard)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), DisableMouseCapture, terminal::LeaveAlternateScreen, cursor::Show);
        let _ = terminal::disable_raw_mode();
    }
}

/// 用户界面
pub struct UserInterface {
    /// 播放列表
    playlist: Arc<Mutex<Playlist>>,
    /// 音频播放器
    audio_player: Arc<Mutex<AudioPlayer>>,
    /// 当前选择索引
    selected_index: usize,
    /// 滚动偏移
    scroll_offset: usize,
    /// 是否应该退出
    should_quit: Arc<Mutex<bool>>,
    /// 提示消息
    status_message: String,
    /// 终端宽度
    terminal_width: u16,
    /// 终端高度
    terminal_height: u16,
    /// 当前歌词
    current_lyrics: Option<Lyrics>,
    /// 当前歌词对应的文件路径（用于判断是否需要更新）
    lyrics_file_path: Option<std::path::PathBuf>,
    /// 波形动画帧计数器
    wave_frame: u32,
    /// 缓存的歌词标题（用于避免闪烁）
    cached_lyrics_title: Option<String>,
    /// 缓存的窗口宽度（用于检测窗口大小变化）
    cached_terminal_width: u16,
    /// 进度条布局信息（用于鼠标点击定位）
    progress_bar_layout: Option<ProgressBarLayout>,
    /// 音量条布局信息（用于鼠标点击定位）
    volume_bar_layout: Option<VolumeBarLayout>,
    /// 后台歌词下载接收器
    lyrics_download_rx: Option<std::sync::mpsc::Receiver<LyricsDownloadResult>>,
    /// 是否正在下载歌词
    lyrics_downloading: bool,
    /// 当前歌曲是否跳过自动歌词下载（在线缓存/本地命中直播放时为 true）
    skip_auto_lyrics_download_for_current_song: bool,
    /// 播放列表布局信息（用于鼠标交互）
    playlist_layout: Option<PlaylistLayout>,
    /// 歌词区域布局信息（用于鼠标拖动跳转）
    lyrics_area_layout: Option<LyricsAreaLayout>,
    /// 是否正在歌词区域左键拖动
    lyrics_dragging: bool,
    /// 歌词拖动目标时间（松开左键后跳转）
    lyrics_drag_target_time: Option<Duration>,
    /// 是否显示评论视图（false=歌词视图）
    comments_mode: bool,
    /// 评论对应的歌曲文件路径（用于判断是否需要刷新）
    comments_file_path: Option<std::path::PathBuf>,
    /// 评论对应的歌曲名称（评论模式下翻页时使用，避免切歌后使用错误歌曲名）
    comments_song_name: String,
    /// 评论总数
    comments_total: usize,
    /// 当前评论页（从1开始）
    comments_page: usize,
    /// 当前页评论列表
    current_comments: Vec<SongCommentItem>,
    /// 评论列表选中索引（基于当前页）
    comments_selected_index: usize,
    /// 评论列表滚动偏移（基于当前页）
    comments_scroll_offset: usize,
    /// 右侧可见行到评论索引的映射（用于鼠标点击选择）
    comments_row_map: Vec<Option<usize>>,
    /// 后台评论拉取接收器
    comments_rx: Option<std::sync::mpsc::Receiver<SongCommentsResult>>,
    /// 是否正在加载评论
    comments_loading: bool,
    /// 是否显示评论详情（按 Enter 切换）
    comments_detail_mode: bool,
    /// 是否显示 AI 歌曲信息视图（false=歌词视图）
    song_info_mode: bool,
    /// 是否显示帮助信息视图（false=歌词视图）
    help_mode: bool,
    /// 帮助视图滚动偏移
    help_scroll_offset: usize,
    /// AI 歌曲信息对应的歌曲文件路径（用于判断是否需要重新查询）
    song_info_file_path: Option<std::path::PathBuf>,
    /// AI 歌曲信息内容
    song_info_content: String,
    /// AI 歌曲信息后台查询接收器（流式）
    song_info_rx: Option<std::sync::mpsc::Receiver<crate::search::SongInfoChunk>>,
    /// AI 歌曲信息滚动偏移
    song_info_scroll_offset: usize,
    /// 是否正在查询 AI 歌曲信息
    song_info_loading: bool,
    /// AI 歌曲信息对应的歌曲名称（用于 GitHub Discussion 标题）
    song_info_name: String,
    /// GitHub Discussion 创建结果接收器
    github_discussion_rx: Option<std::sync::mpsc::Receiver<GitHubDiscussionResult>>,
    /// GitHub Discussion 创建状态信息
    github_discussion_status: String,
    /// 是否正在创建 GitHub Discussion（用于自动滚动）
    github_discussion_loading: bool,
    /// 是否强制滚动到歌曲信息底部（Discussion 结果到达时触发）
    song_info_force_scroll: bool,
    /// 已尝试创建 Discussion 的歌曲名称（防止同一首歌重复创建）
    github_discussion_attempted_name: String,
    /// API Key（来自配置/用户输入）
    api_key: String,
    /// API 接口地址（OpenAI 兼容）
    api_base_url: String,
    /// API 模型名称
    api_model: String,
    /// GitHub Token（用于创建 Discussions）
    github_token: String,
    /// GitHub 仓库（owner/repo 格式）
    github_repo: String,
    /// 是否处于 API 配置输入模式
    api_key_input_mode: bool,
    /// API 配置输入缓存
    api_key_input_value: String,
    /// 当前输入完成后是否继续执行歌曲信息查询（由 i 触发）
    api_key_input_for_song_info: bool,
    /// API 配置输入步骤：0=接口地址, 1=API Key, 2=模型名称
    api_input_step: u8,
    /// 是否处于 GitHub Token 输入模式
    github_token_input_mode: bool,
    /// GitHub Token 输入缓存
    github_token_input_value: String,
    /// 是否处于搜索模式
    search_mode: bool,
    /// 搜索输入关键字
    search_query: String,
    /// 搜索输入框是否拥有焦点（true=键盘输入写入搜索框，false=键盘作用于列表/全局快捷键）
    search_input_focused: bool,
    /// 搜索结果：匹配的歌曲在原始播放列表中的索引
    search_results: Vec<usize>,
    /// 搜索结果列表中的选中索引
    search_selected_index: usize,
    /// 搜索结果列表的滚动偏移
    search_scroll_offset: usize,
    /// 是否处于收藏列表模式
    favorites_mode: bool,
    /// 收藏列表（存储歌曲文件路径）
    favorites: Vec<String>,
    /// 收藏列表中的选中索引
    favorites_selected_index: usize,
    /// 收藏列表的滚动偏移
    favorites_scroll_offset: usize,
    /// 是否处于音乐目录模式
    dir_history_mode: bool,
    /// 音乐目录记录（存储目录路径）
    dir_history: Vec<String>,
    /// 音乐目录中的选中索引
    dir_history_selected_index: usize,
    /// 音乐目录的滚动偏移
    dir_history_scroll_offset: usize,
    /// 上一帧的模式状态（用于检测模式切换，避免右侧标题闪烁）
    prev_mode_state: (bool, bool, bool, bool, bool, bool, bool, bool),
    /// 是否处于网络搜索模式
    online_search_mode: bool,
    /// 网络搜索结果
    online_search_results: Vec<OnlineSong>,
    /// 网络搜索结果中的选中索引
    online_selected_index: usize,
    /// 网络搜索结果的滚动偏移
    online_scroll_offset: usize,
    /// 是否正在搜索网络歌曲
    online_searching: bool,
    /// 网络搜索当前页码
    online_search_page: usize,
    /// 网络搜索结果接收器
    online_search_rx: Option<std::sync::mpsc::Receiver<SearchDownloadResult>>,
    /// 是否正在下载歌曲
    online_downloading: bool,
    /// 下载结果接收器
    online_download_rx: Option<std::sync::mpsc::Receiver<DownloadProgress>>,
    /// 下载进度百分比（0-100）
    online_download_percent: u8,
    /// 正在下载的在线结果索引（用于UI绑定进度）
    online_downloading_index: Option<usize>,
    /// 在线歌曲下载命中缓存（归一化歌曲键 -> 本地文件路径）
    downloaded_online_song_cache: std::collections::HashMap<String, std::path::PathBuf>,
    /// 是否处于聚合搜索搜索模式
    juhe_search_mode: bool,
    /// 是否处于歌单搜索模式
    playlist_search_mode: bool,
    /// 歌单搜索结果
    playlist_search_results: Vec<OnlinePlaylist>,
    /// 当前进入的歌单（None=歌单列表；Some=歌单歌曲列表）
    current_playlist: Option<OnlinePlaylist>,
    /// 进入歌单前在歌单列表中的选中索引（用于 Esc 返回时恢复）
    playlist_list_selected_index: usize,
    /// 歌单搜索结果接收器
    playlist_search_rx: Option<std::sync::mpsc::Receiver<PlaylistSearchResult>>,
    /// 歌单歌曲加载接收器
    playlist_songs_rx: Option<std::sync::mpsc::Receiver<PlaylistSongsResult>>,
    /// 聚合搜索歌词接收器
    juhe_lyrics_rx: Option<std::sync::mpsc::Receiver<crate::search::JuheLyricsResult>>,
    /// 聚合搜索歌词下载中
    juhe_lyrics_loading: bool,
    /// 当前主题
    theme: UiTheme,
    /// 当前主题颜色缓存
    theme_colors: ThemeColors,
    /// 当前界面语言
    language: UiLanguage,
    /// 在线搜索自动切歌节流窗口（记录自动切歌时间点）
    online_auto_skip_times: VecDeque<Instant>,
    /// 是否需要在启动后弹出目录选择对话框
    need_startup_dialog: bool,
}

impl UserInterface {
    /// 创建新的用户界面
    pub fn new(playlist: Arc<Mutex<Playlist>>, audio_player: Arc<Mutex<AudioPlayer>>) -> Self {
        // 获取终端大小，默认 80x24
        let (width, height) = terminal::size().unwrap_or((80, 24));

        UserInterface {
            playlist,
            audio_player,
            selected_index: 0,
            scroll_offset: 0,
            should_quit: Arc::new(Mutex::new(false)),
            status_message: String::new(),
            terminal_width: width,
            terminal_height: height,
            current_lyrics: None,
            lyrics_file_path: None,
            wave_frame: 0,
            cached_lyrics_title: None,
            cached_terminal_width: width,
            progress_bar_layout: None,
            volume_bar_layout: None,
            lyrics_download_rx: None,
            lyrics_downloading: false,
            skip_auto_lyrics_download_for_current_song: false,
            playlist_layout: None,
            lyrics_area_layout: None,
            lyrics_dragging: false,
            lyrics_drag_target_time: None,
            comments_mode: false,
            comments_file_path: None,
            comments_song_name: String::new(),
            comments_total: 0,
            comments_page: 1,
            current_comments: Vec::new(),
            comments_selected_index: 0,
            comments_scroll_offset: 0,
            comments_row_map: Vec::new(),
            comments_rx: None,
            comments_loading: false,
            comments_detail_mode: false,
            song_info_mode: false,
            song_info_file_path: None,
            song_info_content: String::new(),
            song_info_rx: None,
            song_info_scroll_offset: 0,
            song_info_loading: false,
            song_info_name: String::new(),
            github_discussion_rx: None,
            github_discussion_status: String::new(),
            github_discussion_loading: false,
            song_info_force_scroll: false,
            github_discussion_attempted_name: String::new(),
            help_mode: false,
            help_scroll_offset: 0,
            api_key: String::new(),
            api_base_url: "https://api.deepseek.com/".to_string(),
            api_model: "deepseek-v4-flash".to_string(),
            github_token: String::new(),
            github_repo: "xxgg121/ter-music-rust".to_string(),
            api_key_input_mode: false,
            api_key_input_value: String::new(),
            api_key_input_for_song_info: false,
            api_input_step: 0,
            github_token_input_mode: false,
            github_token_input_value: String::new(),
            search_mode: false,
            search_query: String::new(),
            search_input_focused: false,
            search_results: Vec::new(),
            search_selected_index: 0,
            search_scroll_offset: 0,
            favorites_mode: false,
            favorites: Vec::new(),
            favorites_selected_index: 0,
            favorites_scroll_offset: 0,
            dir_history_mode: false,
            dir_history: Vec::new(),
            dir_history_selected_index: 0,
            dir_history_scroll_offset: 0,
            prev_mode_state: (false, false, false, false, false, false, false, false),
            online_search_mode: false,
            online_search_results: Vec::new(),
            online_selected_index: 0,
            online_scroll_offset: 0,
            online_searching: false,
            online_search_page: 1,
            online_search_rx: None,
            online_downloading: false,
            online_download_rx: None,
            online_download_percent: 0,
            online_downloading_index: None,
            downloaded_online_song_cache: std::collections::HashMap::new(),
            juhe_search_mode: false,
            playlist_search_mode: false,
            playlist_search_results: Vec::new(),
            current_playlist: None,
            playlist_list_selected_index: 0,
            playlist_search_rx: None,
            playlist_songs_rx: None,
            juhe_lyrics_rx: None,
            juhe_lyrics_loading: false,
            theme: UiTheme::Neon,
            theme_colors: UiTheme::Neon.colors(),
            language: UiLanguage::Sc,
            online_auto_skip_times: VecDeque::new(),
            need_startup_dialog: false,
        }
    }

    /// 初始化终端
    fn init_terminal() -> io::Result<TerminalGuard> {
        TerminalGuard::new()
    }

    /// 调整滚动偏移，确保选中索引在可见范围内
    fn adjust_scroll_offset(selected: usize, scroll_offset: &mut usize, visible_count: usize) {
        if selected < *scroll_offset {
            *scroll_offset = selected;
        } else if selected >= *scroll_offset + visible_count {
            *scroll_offset = selected - visible_count + 1;
        }
    }

    /// 同时钳制选中索引与滚动偏移，避免越界与残影
    fn clamp_selected_and_scroll(selected: &mut usize, scroll_offset: &mut usize, total: usize, visible_count: usize) {
        if total == 0 {
            *selected = 0;
            *scroll_offset = 0;
            return;
        }

        if *selected >= total {
            *selected = total - 1;
        }

        let max_offset = total.saturating_sub(visible_count);
        if *scroll_offset > max_offset {
            *scroll_offset = max_offset;
        }

        Self::adjust_scroll_offset(*selected, scroll_offset, visible_count);
    }

    fn t(&self) -> &'static LangTexts {
        self.language.texts()
    }

    fn resolved_api_key(&self) -> Option<String> {
        if let Ok(env_key) = std::env::var("DEEPSEEK_API_KEY") {
            let trimmed = env_key.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }

        let trimmed = self.api_key.trim();
        if !trimmed.is_empty() {
            Some(trimmed.to_string())
        } else {
            None
        }
    }

    fn open_api_key_input_mode(&mut self, for_song_info: bool) {
        self.api_key_input_mode = true;
        self.api_key_input_for_song_info = for_song_info;
        self.api_input_step = 0;
        self.api_key_input_value = self.api_base_url.clone();
        self.cached_lyrics_title = None;
    }

    fn start_song_info_mode_for_current_song(&mut self) {
        self.comments_mode = false;
        self.comments_detail_mode = false;
        self.help_mode = false;
        self.song_info_mode = true;
        self.song_info_scroll_offset = 0;

        let current_file = {
            let audio_player = self.audio_player.lock().unwrap();
            audio_player.get_current_file()
        };

        self.song_info_file_path = current_file.as_ref().map(|f| f.path.clone());
        self.song_info_rx = None;
        self.song_info_loading = false;
        self.song_info_content.clear();
        self.song_info_name.clear();
        self.github_discussion_rx = None;
        self.github_discussion_status.clear();
        self.github_discussion_loading = false;
        self.song_info_force_scroll = false;

        if let Some(file) = current_file {
            self.song_info_name = file.name.trim().to_string();
            self.start_fetch_song_info_for_current_song(&file.name);
        } else {
            self.song_info_content = self.t().please_select_song_query.to_string();
        }
    }

    fn now_playing_prefix(&self) -> &'static str {
        self.t().now_playing_prefix
    }

    fn format_playlist_play_count(&self, count: Option<usize>) -> String {
        let Some(n) = count else {
            return "--".to_string();
        };

        if n > 10_000 {
            let value = n as f64 / 10_000.0;
            let mut text = format!("{:.1}", value);
            if text.ends_with(".0") {
                text.truncate(text.len() - 2);
            }
            let unit = self.t().play_count_unit;
            format!("{}{}", text, unit)
        } else {
            n.to_string()
        }
    }

    fn is_now_playing_message(&self, message: &str) -> bool {
        self.t().now_playing_prefixes.iter().any(|p| message.starts_with(p))
    }

    pub fn update_now_playing_status(&mut self, song_name: &str) {
        let prefix = self.now_playing_prefix();
        let safe_song_name = Self::sanitize_single_line_text(song_name);
        self.update_status(&format!("{}{}", prefix, safe_song_name));
    }

    fn play_mode_text(&self, mode: PlayMode) -> &'static str {
        match mode {
            PlayMode::Single => self.t().play_mode_single,
            PlayMode::RepeatOne => self.t().play_mode_repeat_one,
            PlayMode::Sequence => self.t().play_mode_sequence,
            PlayMode::LoopAll => self.t().play_mode_loop_all,
            PlayMode::Random => self.t().play_mode_random,
        }
    }

    fn play_state_text(&self, state: PlayState) -> &'static str {
        match state {
            PlayState::Playing => self.t().play_state_playing,
            PlayState::Paused => self.t().play_state_paused,
            PlayState::Stopped => self.t().play_state_stopped,
        }
    }

    /// 获取快捷键提示文本（根据当前模式和终端宽度动态选择）
    fn get_help_tip_text(&self) -> String {
        if self.search_mode {
            if self.online_search_mode {
                let search_label = if self.playlist_search_mode {
                    self.t().search_playlist.to_string()
                } else if self.juhe_search_mode {
                    self.t().search_juhe.to_string()
                } else {
                    self.t().search_online.to_string()
                };
                if self.terminal_width >= 80 {
                    format!(
                        "{}: {}",
                        search_label,
                        self.t().tip_online_wide.to_string()
                    )
                } else if self.terminal_width >= 60 {
                    format!(
                        "{}: {}",
                        search_label,
                        self.t().tip_online_medium.to_string()
                    )
                } else {
                    format!("{}: {}", search_label, self.t().tip_online_narrow)
                }
            } else if self.terminal_width >= 60 {
                self.t().tip_local_wide.to_string()
            } else {
                self.t().tip_local_narrow.to_string()
            }
        } else if self.favorites_mode {
            if self.terminal_width >= 60 {
                self.t().tip_favorites_wide.to_string()
            } else {
                self.t().tip_favorites_narrow.to_string()
            }
        } else if self.dir_history_mode {
            if self.terminal_width >= 60 {
                self.t().tip_dir_wide.to_string()
            } else {
                self.t().tip_dir_narrow.to_string()
            }
        } else if self.help_mode {
            if self.terminal_width >= 80 {
                self.t().tip_help_wide.to_string()
            } else {
                self.t().tip_help_narrow.to_string()
            }
        } else if self.song_info_mode {
            if self.terminal_width >= 80 {
                self.t().tip_song_info_wide.to_string()
            } else {
                self.t().tip_song_info_narrow.to_string()
            }
        } else if self.comments_mode {
            if self.terminal_width >= 80 {
                self.t().tip_comments_wide.to_string()
            } else {
                self.t().tip_comments_narrow.to_string()
            }
        } else if self.terminal_width >= 100 {
            self.t().tip_keys_wide.to_string()
        } else if self.terminal_width >= 80 {
            self.t().tip_keys_medium.to_string()
        } else {
            self.t().tip_keys_narrow.to_string()
        }
    }

    /// 将文本清理为单行可显示内容，移除会破坏终端布局的控制字符
    fn sanitize_single_line_text(text: &str) -> String {
        text.chars()
            .map(|ch| match ch {
                '\n' | '\r' | '\t' => ' ',
                c if c.is_control() => ' ',
                c => c,
            })
            .collect()
    }

    /// 截断字符串到指定显示宽度，超出部分用 "..." 省略
    fn truncate_with_ellipsis(text: &str, max_width: usize) -> String {
        let text = Self::sanitize_single_line_text(text);
        if term_display_width(text.as_str()) <= max_width {
            return text;
        }

        let mut truncated = String::new();
        let mut current_width = 0;
        for ch in text.chars() {
            let ch_width = term_char_width(ch);
            if current_width + ch_width + 3 > max_width {
                break;
            }
            truncated.push(ch);
            current_width += ch_width;
        }
        format!("{}...", truncated)
    }

    /// 按显示宽度分割字符串，返回 (不超宽部分, 溢出部分)
    fn split_at_display_width(text: &str, max_width: usize) -> (&str, &str) {
        let mut current_width = 0;
        for (i, ch) in text.char_indices() {
            let ch_width = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0);
            if current_width + ch_width > max_width {
                return (&text[..i], &text[i..]);
            }
            current_width += ch_width;
        }
        (text, "")
    }

    /// 清除列表区域下方的残留行
    fn clear_remaining_rows<W: Write>(
        stdout: &mut W,
        start_row: u16,
        used_rows: usize,
        total_rows: usize,
    ) -> io::Result<()> {
        for row in used_rows..total_rows {
            queue!(
                stdout,
                cursor::MoveTo(0, start_row + row as u16),
                terminal::Clear(ClearType::UntilNewLine),
            )?;
        }
        Ok(())
    }

    /// 根据鼠标位置获取歌词拖动目标时间
    fn lyric_time_at_position(&self, col: usize, row: u16) -> Option<Duration> {
        if self.comments_mode || self.song_info_mode || self.help_mode {
            return None;
        }
        let layout = self.lyrics_area_layout.as_ref()?;

        if col < layout.start_col || col >= layout.start_col + layout.width {
            return None;
        }
        if row < layout.start_row {
            return None;
        }

        let line_index = (row - layout.start_row) as usize;
        layout.line_times.get(line_index).copied()
    }

    /// 跳转到指定歌词时间
    fn seek_to_lyric_time(&mut self, target_time: Duration) {
        let result = {
            let mut player = self.audio_player.lock().unwrap();
            let (_, total) = player.get_progress();
            if let Some(total_dur) = total {
                let total_secs = total_dur.as_secs_f64();
                if total_secs > 0.0 {
                    let ratio = (target_time.as_secs_f64() / total_secs).clamp(0.0, 1.0);
                    player.seek(ratio)
                } else {
                    Err(self.t().seek_error_zero_duration.to_string())
                }
            } else {
                Err(self.t().seek_error_unknown_duration.to_string())
            }
        };

        if let Err(e) = result {
            self.update_status(&self.t().fmt_seek_failed.replace("{}", &e));
        }
    }

    /// 歌词区域滚轮跳转（direction: -1 上一行，1 下一行）
    fn seek_by_lyric_wheel(&mut self, direction: i8) {
        if self.comments_mode || self.song_info_mode || self.help_mode {
            return;
        }

        let current_time = {
            let player = self.audio_player.lock().unwrap();
            player.get_progress().0
        };

        let target_time = self.current_lyrics.as_ref().and_then(|lyrics| {
            let lines = lyrics.get_lines();
            if lines.is_empty() {
                return None;
            }

            let idx = lines.partition_point(|line| line.time <= current_time);
            let current_idx = if idx == 0 { 0 } else { idx - 1 };
            let target_idx = if direction < 0 {
                current_idx.saturating_sub(1)
            } else {
                (current_idx + 1).min(lines.len() - 1)
            };

            Some(lines[target_idx].time)
        });

        if let Some(time) = target_time {
            self.seek_to_lyric_time(time);
        }
    }

    /// 启动后台拉取评论
    fn start_fetch_comments_for_current_song(&mut self, song_name: &str) {
        self.comments_loading = true;
        self.current_comments.clear();
        self.comments_selected_index = 0;
        self.comments_scroll_offset = 0;
        self.comments_row_map.clear();
        self.comments_detail_mode = false;
        self.comments_rx = Some(crate::search::fetch_song_comments_background(
            song_name.to_string(),
            self.comments_page,
            20,
        ));
    }

    /// 非阻塞检查评论拉取结果
    fn check_comments_result(&mut self) {
        if let Some(ref rx) = self.comments_rx {
            if let Ok(result) = rx.try_recv() {
                self.comments_page = result.page.max(1);
                self.comments_total = result.total;
                self.current_comments = result.comments;
                self.comments_loading = false;
                self.comments_rx = None;

                if self.comments_selected_index >= self.current_comments.len() {
                    self.comments_selected_index = self.current_comments.len().saturating_sub(1);
                }
            }
        }
    }

    /// 根据当前歌曲和分页状态刷新评论
    fn ensure_comments_up_to_date(&mut self, current_file: Option<&crate::defs::MusicFile>) {
        self.check_comments_result();

        // 评论模式下不因歌曲变化而重置评论，保持用户正在查看的内容
        // 翻页时使用存储的 comments_song_name 获取正确歌曲的评论
        if self.comments_mode {
            if !self.comments_loading
                && self.comments_rx.is_none()
                && self.current_comments.is_empty()
                && !self.comments_song_name.is_empty()
            {
                self.start_fetch_comments_for_current_song(&self.comments_song_name.clone());
            }
            return;
        }

        // 非评论模式下的原有逻辑
        let current_path = current_file.map(|f| f.path.clone());
        let song_changed = self.comments_file_path != current_path;

        if song_changed {
            self.comments_file_path = current_path;
            self.comments_page = 1;
            self.comments_total = 0;
            self.current_comments.clear();
            self.comments_selected_index = 0;
            self.comments_scroll_offset = 0;
            self.comments_row_map.clear();
            self.comments_detail_mode = false;
            self.comments_rx = None;
            self.comments_loading = false;
        }

        if current_file.is_none() {
            self.comments_total = 0;
            self.current_comments.clear();
            self.comments_selected_index = 0;
            self.comments_scroll_offset = 0;
            self.comments_row_map.clear();
            self.comments_detail_mode = false;
            self.comments_loading = false;
            self.comments_rx = None;
            return;
        }

        if !self.comments_loading
            && self.comments_rx.is_none()
            && self.current_comments.is_empty()
        {
            if let Some(file) = current_file {
                self.comments_song_name = file.name.trim().to_string();
                self.start_fetch_comments_for_current_song(&file.name);
            }
        }
    }

    /// 根据当前语言构造 DeepSeek 歌曲信息提示词
    fn build_song_info_prompt(&self, song_name: &str) -> String {
        let clean_name = song_name.trim();
        self.t().ai_prompt_template.replace("{}", clean_name)
    }
    /// 启动后台查询 AI 歌曲信息
    fn start_fetch_song_info_for_current_song(&mut self, song_name: &str) {
        self.song_info_loading = true;
        self.song_info_content.clear();
        let prompt = self.build_song_info_prompt(song_name);
        let config = crate::search::AiQueryConfig {
            api_base_url: self.api_base_url.clone(),
            api_key: self.resolved_api_key().unwrap_or_default(),
            api_model: self.api_model.clone(),
        };
        self.song_info_rx = Some(crate::search::fetch_song_info_streaming(prompt, config));
    }

    /// 非阻塞检查 AI 歌曲信息查询结果
    fn check_song_info_result(&mut self) {
        if let Some(ref rx) = self.song_info_rx {
            // 流式接收：每次检查时读取所有可用 chunk
            loop {
                match rx.try_recv() {
                    Ok(chunk) => {
                        if let Some(err) = chunk.error {
                            self.song_info_loading = false;
                            self.song_info_rx = None;
                            self.song_info_content = format!(
                                "{}{}\n\n{}",
                                self.t().query_failed,
                                err,
                                self.t().api_config_hint
                            );
                            break;
                        }
                        if !chunk.delta.is_empty() {
                            let delta = chunk.delta.replace("**", "").replace("*", "");
                            let delta = delta.replace("##", "").replace("#", "");
                            self.song_info_content.push_str(&delta);
                        }
                        if chunk.done {
                            self.song_info_loading = false;
                            self.song_info_rx = None;
                            self.song_info_force_scroll = true;
                            // 流式输出完成后，自动创建 GitHub Discussion
                            self.start_github_discussion_for_song_info();
                            break;
                        }
                    }
                    Err(std::sync::mpsc::TryRecvError::Empty) => break,
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                        self.song_info_loading = false;
                        self.song_info_rx = None;
                        break;
                    }
                }
            }
        }

        // 检查 GitHub Discussion 创建结果
        if let Some(ref rx) = self.github_discussion_rx {
            match rx.try_recv() {
                Ok(result) => {
                    self.github_discussion_rx = None;
                    self.github_discussion_loading = false;
                    self.song_info_force_scroll = true;
                    if let Some(url) = result.url {
                        self.github_discussion_status = format!(
                            "{} {}",
                            self.t().discussion_created,
                            url
                        );
                    } else if let Some(err) = result.error {
                        self.github_discussion_status = format!(
                            "{} {}",
                            self.t().discussion_failed,
                            err
                        );
                    }
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {}
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    self.github_discussion_rx = None;
                    self.github_discussion_loading = false;
                }
            }
        }
    }

    /// 流式输出完成后，创建 GitHub Discussion
    fn start_github_discussion_for_song_info(&mut self) {
        let content = self.song_info_content.trim().to_string();
        let name = self.song_info_name.trim().to_string();

        if content.is_empty() || name.is_empty() {
            return;
        }

        // 如果已经为这首歌创建过 Discussion，跳过（防止重复创建和重复显示 URL）
        if self.github_discussion_attempted_name == name {
            return;
        }
        self.github_discussion_attempted_name = name.clone();

        let token = self.resolved_github_token();
        let repo = self.github_repo.clone();

        self.github_discussion_loading = true;
        self.github_discussion_status = self.t().creating_discussion.to_string();

        self.github_discussion_rx = Some(
            crate::search::create_github_discussion_background(token, repo, name, content)
        );
    }

    /// 绘制 AI 歌曲信息（右侧）
    fn draw_song_info<W: Write>(
        &mut self,
        stdout: &mut W,
        start_x: u16,
        width: u16,
        visible_count: usize,
    ) -> io::Result<()> {
        // AI 信息视图不使用歌词拖动布局
        self.lyrics_area_layout = None;

        let current_file = {
            let audio_player = self.audio_player.lock().unwrap();
            audio_player.get_current_file()
        };

        self.check_song_info_result();

        if self.song_info_file_path != current_file.as_ref().map(|f| f.path.clone()) {
            self.song_info_file_path = current_file.as_ref().map(|f| f.path.clone());
            self.song_info_content.clear();
            self.song_info_loading = false;
            self.song_info_rx = None;
            self.song_info_scroll_offset = 0;
            // 切歌时清除 Discussion 状态和歌名
            self.github_discussion_status.clear();
            self.github_discussion_rx = None;
            self.github_discussion_loading = false;
            self.song_info_force_scroll = false;
            if let Some(file) = current_file.as_ref() {
                self.song_info_name = file.name.trim().to_string();
                self.start_fetch_song_info_for_current_song(&file.name);
            } else {
                self.song_info_name.clear();
            }
        }

        for i in 0..visible_count {
            queue!(
                stdout,
                cursor::MoveTo(start_x, (i + 6) as u16),
                terminal::Clear(ClearType::UntilNewLine),
            )?;
        }

        if current_file.is_none() {
            queue!(
                stdout,
                cursor::MoveTo(start_x, 8),
                style::SetForegroundColor(style::Color::DarkGrey),
                style::Print(self.t().select_song_to_play),
                style::ResetColor,
            )?;
            return Ok(());
        }

        // 流式输出：加载中但已有内容时，显示已有内容（而非等待全部完成）
        if self.song_info_content.trim().is_empty() && self.song_info_loading {
            queue!(
                stdout,
                cursor::MoveTo(start_x, 8),
                style::SetForegroundColor(style::Color::DarkGrey),
                style::Print(self.t().querying_song_info),
                style::ResetColor,
            )?;
            return Ok(());
        }

        let content = if self.song_info_content.trim().is_empty() {
            self.t().no_query_result.to_string()
        } else {
            self.song_info_content.clone()
        };

        let wrapped_lines = wrap_text_to_width(&content, width.saturating_sub(1) as usize);

        // 追加 GitHub Discussion 状态行
        // 拆分为前缀和 URL 两部分，URL 用 OSC 8 可点击，前缀为普通文本
        let (discussion_prefix, discussion_url) = if !self.github_discussion_status.is_empty() {
            if let Some(url_start) = self.github_discussion_status.find("http://").or_else(|| self.github_discussion_status.find("https://")) {
                let prefix = self.github_discussion_status[..url_start].to_string();
                let url = self.github_discussion_status[url_start..].trim_end().to_string();
                (prefix, Some(url))
            } else {
                (self.github_discussion_status.clone(), None)
            }
        } else {
            (String::new(), None)
        };

        let discussion_lines = if !discussion_prefix.is_empty() {
            let mut lines = Vec::new();
            // 空一行分隔
            lines.push(String::new());
            // 前缀行
            if !discussion_prefix.trim().is_empty() {
                lines.extend(wrap_text_to_width(&discussion_prefix, width.saturating_sub(1) as usize));
            }
            // URL 单独一行（OSC 8 处理，不换行截断）
            if let Some(ref url) = discussion_url {
                lines.push(url.clone());
            }
            lines
        } else {
            Vec::new()
        };

        let total_lines = wrapped_lines.len() + discussion_lines.len();
        let max_offset = total_lines.saturating_sub(visible_count);
        // 流式输出时自动滚动到底部，确保新内容可见；内容变化时一次性滚动
        if self.song_info_loading || self.song_info_force_scroll {
            self.song_info_scroll_offset = max_offset;
            self.song_info_force_scroll = false;
        } else if self.song_info_scroll_offset > max_offset {
            self.song_info_scroll_offset = max_offset;
        }

        // 计算可见范围内的行
        let skip = self.song_info_scroll_offset;
        let all_lines_count = wrapped_lines.len() + discussion_lines.len();
        for (screen_row, line_idx) in (skip..all_lines_count).take(visible_count).enumerate() {
            queue!(
                stdout,
                cursor::MoveTo(start_x, (screen_row + 6) as u16),
                terminal::Clear(ClearType::UntilNewLine),
            )?;

            if line_idx < wrapped_lines.len() {
                // 普通歌曲信息行
                queue!(
                    stdout,
                    style::SetForegroundColor(self.theme_colors.song_normal),
                    style::Print(truncate_to_width(&wrapped_lines[line_idx], width.saturating_sub(1) as usize)),
                    style::ResetColor,
                )?;
            } else {
                // Discussion 状态行
                let disc_idx = line_idx - wrapped_lines.len();
                let line = &discussion_lines[disc_idx];
                // URL 行用 OSC 8 超链接，前缀行为普通文本
                if discussion_url.as_ref() == Some(line) {
                    // OSC 8 超链接格式: \x1b]8;;URL\x1b\\TEXT\x1b]8;;\x1b\\
                    let hyperlink = format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", line, line);
                    queue!(
                        stdout,
                        style::SetForegroundColor(self.theme_colors.song_normal),
                        style::Print(&hyperlink),
                        style::ResetColor,
                    )?;
                } else {
                    queue!(
                        stdout,
                        style::SetForegroundColor(self.theme_colors.song_normal),
                        style::Print(truncate_to_width(line, width.saturating_sub(1) as usize)),
                        style::ResetColor,
                    )?;
                }
            }
        }

        Ok(())
    }

    /// 绘制界面
    fn draw(&mut self) -> io::Result<()> {
        let mut stdout = io::stdout();

        // 更新终端大小
        let window_size_changed = if let Ok((width, height)) = terminal::size() {
            let changed = self.terminal_width != width || self.terminal_height != height;
            self.terminal_width = width;
            self.terminal_height = height;
            changed
        } else {
            false
        };

        // 如果窗口大小改变，清屏以避免残留内容
        if window_size_changed {
            queue!(stdout, terminal::Clear(ClearType::All))?;
        }

        // 绘制开始时隐藏光标，避免在绘制过程中光标在各位置闪烁
        queue!(stdout, cursor::Hide, cursor::MoveTo(0, 0))?;

        // 绘制标题
        self.draw_header(&mut stdout)?;

        // 绘制播放列表
        self.draw_playlist(&mut stdout)?;

        // 绘制控制栏
        self.draw_controls(&mut stdout)?;

        // 绘制状态栏
        self.draw_status(&mut stdout)?;

        // 输入模式下，将光标定位到输入位置
        if self.api_key_input_mode {
            let prompt_text = match self.api_input_step {
                0 => self.t().input_api_url,
                1 => self.t().input_api_key,
                2 => self.t().input_model_name,
                _ => "",
            };
            let prompt_len = unicode_width::UnicodeWidthStr::width(prompt_text);
            let value_len = unicode_width::UnicodeWidthStr::width(self.api_key_input_value.as_str());
            let left_width = (self.terminal_width as f32 * 0.50) as u16;
            let target_col = left_width + 1 + (prompt_len + value_len) as u16;
            queue!(
                stdout,
                cursor::MoveTo(target_col, 4),
                cursor::Show,
            )?;
        } else if self.github_token_input_mode {
            let prompt_text = self.t().input_github_token;
            let prompt_len = unicode_width::UnicodeWidthStr::width(prompt_text);
            let value_len = unicode_width::UnicodeWidthStr::width(self.github_token_input_value.as_str());
            let left_width = (self.terminal_width as f32 * 0.50) as u16;
            let target_col = left_width + 1 + (prompt_len + value_len) as u16;
            queue!(
                stdout,
                cursor::MoveTo(target_col, 4),
                cursor::Show,
            )?;
        } else if self.search_mode
            && self.search_input_focused
            && !(self.playlist_search_mode && self.current_playlist.is_some()) {
            let prompt_text = if self.online_search_mode {
                if self.playlist_search_mode {
                    self.t().search_prompt_playlist
                } else if self.juhe_search_mode {
                    self.t().search_prompt_juhe
                } else {
                    self.t().search_prompt_online
                }
            } else {
                self.t().search_prompt_local
            };
            let search_prompt_len = unicode_width::UnicodeWidthStr::width(prompt_text);
            let query_len = unicode_width::UnicodeWidthStr::width(self.search_query.as_str());
            let target_col = (search_prompt_len + query_len) as u16;

            // 移动光标到搜索输入位置，然后显示光标
            // （draw 开始时光标已被隐藏，所以不会在绘制过程中出现闪烁）
            queue!(
                stdout,
                cursor::MoveTo(target_col, 4),
                cursor::Show,
            )?;
        }
        // 非输入模式/搜索模式下光标保持隐藏（draw 开始时已隐藏）

        stdout.flush()?;

        Ok(())
    }

    /// 绘制标题
    fn draw_header<W: Write>(&self, stdout: &mut W) -> io::Result<()> {
        // 根据终端宽度生成标题
        let width = self.terminal_width as usize;
        let title = self.t().app_title;

        // 计算标题居中位置（使用显示宽度而非字符数）
        let title_len = unicode_width::UnicodeWidthStr::width(title);
        let total_space = width.saturating_sub(title_len + 2); // +2 for "║" and "║"
        let left_padding = total_space / 2;
        let right_padding = total_space.saturating_sub(left_padding);
        let title_line = format!(
            "║{}{}{}║",
            " ".repeat(left_padding),
            title,
            " ".repeat(right_padding)
        );

        // 生成分隔线
        let separator = "═".repeat(width.saturating_sub(2));
        let top_line = format!("╔{}╗", separator);
        let bottom_line = format!("╚{}╝", separator);

        queue!(
            stdout,
            cursor::MoveTo(0, 0),
            style::SetForegroundColor(self.theme_colors.header_title),
            style::Print(top_line),
            cursor::MoveTo(0, 1),
            style::Print(title_line),
            cursor::MoveTo(0, 2),
            style::Print(bottom_line),
            style::ResetColor,
        )?;
        Ok(())
    }

    /// 绘制播放列表（左右分栏：左侧歌曲列表，右侧歌词）
    fn draw_playlist<W: Write>(&mut self, stdout: &mut W) -> io::Result<()> {
        // 提前获取需要的信息，避免长时间持有锁
        let (current_file, play_state) = {
            let audio_player = self.audio_player.lock().unwrap();
            (audio_player.get_current_file(), audio_player.get_state())
        };

        let playlist = self.playlist.lock().unwrap();

        // 计算左右分栏的宽度
        let left_width = (self.terminal_width as f32 * 0.50) as u16;
        let right_width = self.terminal_width.saturating_sub(left_width + 1);

        // 绘制播放列表标题（左侧）
        let visible_count = (self.terminal_height as usize).saturating_sub(12).max(5);

        if self.dir_history_mode {
            // 音乐目录模式：标题显示音乐目录
            let dir_title = self.t().fmt_dir_title.replace("{}", &self.dir_history.len().to_string());
            let dir_title_width = unicode_width::UnicodeWidthStr::width(dir_title.as_str());
            let padding = left_width as usize - dir_title_width;
            queue!(
                stdout,
                cursor::MoveTo(0, 4),
                style::SetForegroundColor(self.theme_colors.section_title),
                style::Print(dir_title),
                style::Print(" ".repeat(padding)),
                style::ResetColor,
            )?;

            // 保存播放列表布局信息
            self.playlist_layout = Some(PlaylistLayout {
                start_row: 6,
                visible_count,
                left_width,
            });

            // 绘制分割线（左侧）
            queue!(
                stdout,
                cursor::MoveTo(0, 5),
                style::SetForegroundColor(self.theme_colors.section_title),
                style::Print("─".repeat(left_width as usize)),
                style::ResetColor,
            )?;

            let total = self.dir_history.len();

            // 调整音乐目录滚动偏移
            Self::adjust_scroll_offset(self.dir_history_selected_index, &mut self.dir_history_scroll_offset, visible_count);

            // 显示音乐目录列表
            for i in self.dir_history_scroll_offset..std::cmp::min(self.dir_history_scroll_offset + visible_count, total) {
                let dir_path = &self.dir_history[i];
                let is_selected = i == self.dir_history_selected_index;

                // 提取目录名（最后一级目录名，备用）
                let _dir_name = std::path::Path::new(dir_path)
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| dir_path.clone());

                // 检查是否为当前正在播放的目录
                let is_current = playlist.directory.as_ref().map(|d| d == dir_path).unwrap_or(false);

                let selector = if is_selected { ">" } else { " " };
                let current_marker = if is_current { ">>" } else { " " };

                // 构建显示内容：● 目录名 路径
                let display_line = format!("{} {}", current_marker, dir_path);
                let max_width = left_width.saturating_sub(2) as usize;
                let display_text = Self::truncate_with_ellipsis(&display_line, max_width);

                let row_pos = (i - self.dir_history_scroll_offset + 6) as u16;
                if is_selected {
                    // 用空格填充左面板宽度，限制蓝色背景不超出左面板区域
                    queue!(
                        stdout,
                        cursor::MoveTo(0, row_pos),
                        style::SetBackgroundColor(style::Color::DarkBlue),
                        style::Print(" ".repeat(left_width as usize)),
                        cursor::MoveTo(0, row_pos),
                    )?;
                } else {
                    queue!(
                        stdout,
                        cursor::MoveTo(0, row_pos),
                        terminal::Clear(ClearType::UntilNewLine),
                    )?;
                }
                if is_current {
                    queue!(stdout, style::SetForegroundColor(self.theme_colors.song_playing))?;
                } else {
                    queue!(stdout, style::SetForegroundColor(self.theme_colors.song_normal))?;
                }

                // 确保显示行不超过左面板宽度
                let full_line = format!("{} {}", selector, display_text);
                let display_line = truncate_to_width(&full_line, left_width as usize);
                queue!(
                    stdout,
                    style::Print(&display_line),
                    style::ResetColor,
                )?;
            }

            // 如果没有音乐目录
            if total == 0 {
                queue!(
                    stdout,
                    cursor::MoveTo(1, 7),
                    style::SetForegroundColor(self.theme_colors.info_text),
                    style::Print(self.t().dir_empty_hint),
                    style::ResetColor,
                )?;
            }

            // 清除音乐目录列表下方的残留行
            let used_rows = std::cmp::min(total, visible_count);
            Self::clear_remaining_rows(stdout, 6, used_rows, visible_count)?;
        } else if self.favorites_mode {
            // 收藏列表模式：标题显示收藏列表
            let fav_title = self.t().fmt_favorites_title.replace("{}", &self.favorites.len().to_string());
            let fav_title_width = unicode_width::UnicodeWidthStr::width(fav_title.as_str());
            let padding = left_width as usize - fav_title_width;
            queue!(
                stdout,
                cursor::MoveTo(0, 4),
                style::SetForegroundColor(self.theme_colors.section_title),
                style::Print(fav_title),
                style::Print(" ".repeat(padding)),
                style::ResetColor,
            )?;

            // 保存播放列表布局信息
            self.playlist_layout = Some(PlaylistLayout {
                start_row: 6,
                visible_count,
                left_width,
            });

            // 绘制分割线（左侧）
            queue!(
                stdout,
                cursor::MoveTo(0, 5),
                style::SetForegroundColor(self.theme_colors.section_title),
                style::Print("─".repeat(left_width as usize)),
                style::ResetColor,
            )?;

            // 构建收藏列表的歌曲信息（包含当前播放列表中找不到的歌曲）
            let fav_files: Vec<(Option<usize>, String, String)> = self.favorites.iter()
                .map(|path| {
                    // 尝试在当前播放列表中查找
                    if let Some((idx, file)) = playlist.files.iter().enumerate()
                        .find(|(_, f)| f.path.to_string_lossy() == *path)
                    {
                        (Some(idx), file.name.clone(), file.format_duration())
                    } else {
                        // 不在当前播放列表中，用文件名显示
                        let name = std::path::Path::new(path)
                            .file_stem()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_else(|| path.clone());
                        (None, name, "--:--".to_string())
                    }
                })
                .collect();

            let total = fav_files.len();

            // 调整收藏列表滚动偏移
            Self::adjust_scroll_offset(self.favorites_selected_index, &mut self.favorites_scroll_offset, visible_count);

            // 显示收藏列表
            let end = std::cmp::min(self.favorites_scroll_offset + visible_count, total);
            for (i, (orig_idx, song_name, duration_str)) in fav_files
                .iter()
                .enumerate()
                .skip(self.favorites_scroll_offset)
                .take(end - self.favorites_scroll_offset)
            {
                let is_selected = i == self.favorites_selected_index;
                let in_current_dir = orig_idx.is_some();
                // 检查是否正在播放（通过路径匹配）
                let fav_path = &self.favorites[i];
                let is_playing = current_file
                    .as_ref()
                    .map(|f| f.path.to_string_lossy() == *fav_path)
                    .unwrap_or(false);

                let prefix = if is_playing {
                    match play_state {
                        PlayState::Playing => ">> ",
                        PlayState::Paused => "|| ",
                        PlayState::Stopped => "[] ",
                    }
                } else {
                    "   "
                };

                let selector = if is_selected { ">" } else { " " };
                let star = "*"; // 收藏列表中全部显示*

                // 动态计算固定部分宽度，使用最大值确保播放状态切换时歌曲名空间不变
                let fixed_width = 1 // selector 宽度 (>)
                    + 1 // star 宽度 (*)
                    + 3 // prefix 宽度 (统一为3)
                    + term_display_width(duration_str.as_str())
                    + 4; // 4个分隔空格
                let max_width = left_width.saturating_sub(fixed_width as u16) as usize;
                let safe_song_name = Self::sanitize_single_line_text(song_name);
                let name = Self::truncate_with_ellipsis(&safe_song_name, max_width);

                let row_pos = (i - self.favorites_scroll_offset + 6) as u16;
                if is_selected {
                    queue!(
                        stdout,
                        cursor::MoveTo(0, row_pos),
                        style::SetBackgroundColor(style::Color::DarkBlue),
                        style::Print(" ".repeat(left_width as usize)),
                        cursor::MoveTo(0, row_pos),
                    )?;
                } else {
                    queue!(
                        stdout,
                        cursor::MoveTo(0, row_pos),
                        terminal::Clear(ClearType::UntilNewLine),
                    )?;
                }
                if is_playing {
                    queue!(stdout, style::SetForegroundColor(self.theme_colors.song_playing))?;
                } else if in_current_dir {
                    queue!(stdout, style::SetForegroundColor(self.theme_colors.song_normal))?;
                } else {
                    queue!(stdout, style::SetForegroundColor(self.theme_colors.info_text))?;
                }

                // 确保显示行不超过左面板宽度
                let full_line = format!("{} {} {} {} {}", selector, star, prefix, name, duration_str);
                let display_line = truncate_to_width(&full_line, left_width as usize);
                queue!(
                    stdout,
                    style::Print(&display_line),
                    style::ResetColor,
                )?;
            }

            // 如果没有收藏
            if total == 0 {
                queue!(
                    stdout,
                    cursor::MoveTo(1, 7),
                    style::SetForegroundColor(style::Color::DarkGrey),
                    style::Print(self.t().favorites_empty_hint),
                    style::ResetColor,
                )?;
            }

            // 清除收藏列表下方的残留行
            let used_rows = std::cmp::min(total, visible_count);
            Self::clear_remaining_rows(stdout, 6, used_rows, visible_count)?;
        } else if self.search_mode {
            // 搜索模式：标题显示搜索栏
            let search_prompt = if self.playlist_search_mode && self.current_playlist.is_some() {
                if let Some(ref pl) = self.current_playlist {
                    let count = pl.song_count.map(|n| n.to_string()).unwrap_or_else(|| "?".to_string());
                    let play_text = self.format_playlist_play_count(pl.play_count);
                    format!(
                        "{} [🎵{} 🎧{}]",
                        pl.name,
                        count,
                        play_text,
                    )
                } else {
                    self.search_query.clone()
                }
            } else if self.online_search_mode {
                if self.playlist_search_mode {
                    format!(
                        "{}{}",
                        self.t().search_prompt_playlist,
                        self.search_query
                    )
                } else if self.juhe_search_mode {
                    format!(
                        "{}{}",
                        self.t().search_prompt_juhe,
                        self.search_query
                    )
                } else {
                    format!(
                        "{}{}",
                        self.t().search_prompt_online,
                        self.search_query
                    )
                }
            } else {
                format!(
                    "{}{}",
                    self.t().search_prompt_local,
                    self.search_query
                )
            };
            // 搜索栏标题统一做宽度裁剪，避免超长歌单名溢出到右侧歌词标题区域
            let safe_search_prompt = Self::sanitize_single_line_text(&search_prompt);
            let search_prompt_display = Self::truncate_with_ellipsis(&safe_search_prompt, left_width.saturating_sub(2) as usize);
            let search_width = unicode_width::UnicodeWidthStr::width(search_prompt_display.as_str());
            let padding = (left_width as usize).saturating_sub(search_width);
            queue!(
                stdout,
                cursor::MoveTo(0, 4),
                style::SetForegroundColor(self.theme_colors.section_title),
                style::Print(&search_prompt_display),
                style::Print(" ".repeat(padding)),
                style::ResetColor,
            )?;

            // 保存播放列表布局信息
            self.playlist_layout = Some(PlaylistLayout {
                start_row: 6,
                visible_count,
                left_width,
            });

            // 绘制分割线（左侧）
            queue!(
                stdout,
                cursor::MoveTo(0, 5),
                style::SetForegroundColor(self.theme_colors.section_title),
                style::Print("─".repeat(left_width as usize)),
                style::ResetColor,
            )?;

            if self.online_search_mode {
                if self.playlist_search_mode && self.current_playlist.is_none() {
                    // 歌单搜索结果列表
                    let total = self.playlist_search_results.len();
                    Self::clamp_selected_and_scroll(
                        &mut self.online_selected_index,
                        &mut self.online_scroll_offset,
                        total,
                        visible_count,
                    );

                    for i in self.online_scroll_offset..std::cmp::min(self.online_scroll_offset + visible_count, total) {
                        let pl = &self.playlist_search_results[i];
                        let is_selected = i == self.online_selected_index;
                        let count_text = pl.song_count.map(|n| n.to_string()).unwrap_or_else(|| "--".to_string());
                        let play_text = self.format_playlist_play_count(pl.play_count);

                        let safe_playlist_name = Self::sanitize_single_line_text(&pl.name);
                        let display = format!("{} [🎵{} 🎧{}]", safe_playlist_name, count_text, play_text);
                        let name = Self::truncate_with_ellipsis(&display, left_width.saturating_sub(4) as usize);

                        let row_pos = (i - self.online_scroll_offset + 6) as u16;
                        if is_selected {
                            queue!(
                                stdout,
                                cursor::MoveTo(0, row_pos),
                                style::SetBackgroundColor(style::Color::DarkBlue),
                                style::Print(" ".repeat(left_width as usize)),
                                cursor::MoveTo(0, row_pos),
                            )?;
                        } else {
                            queue!(
                                stdout,
                                cursor::MoveTo(0, row_pos),
                                terminal::Clear(ClearType::UntilNewLine),
                            )?;
                        }
                        queue!(
                            stdout,
                            style::SetForegroundColor(self.theme_colors.song_normal),
                            // 确保显示行不超过左面板宽度
                            style::Print(truncate_to_width(&format!("  {}", name), left_width as usize)),
                            style::ResetColor,
                        )?;
                    }

                    if total == 0 {
                        let hint = if self.search_query.is_empty() {
                            self.t().playlist_empty_hint
                        } else {
                            self.t().playlist_no_result
                        };
                        queue!(
                            stdout,
                            cursor::MoveTo(1, 7),
                            style::SetForegroundColor(self.theme_colors.info_text),
                            style::Print(hint),
                            style::ResetColor,
                        )?;
                    }
                    let used_rows = std::cmp::min(total, visible_count);
                    Self::clear_remaining_rows(stdout, 6, used_rows, visible_count)?;
                } else {
                    // 网络歌曲结果列表（普通/聚合/歌单内歌曲）
                    let total = self.online_search_results.len();
                    Self::clamp_selected_and_scroll(
                        &mut self.online_selected_index,
                        &mut self.online_scroll_offset,
                        total,
                        visible_count,
                    );

                    // 渲染搜索结果列表
                    for i in self.online_scroll_offset..std::cmp::min(self.online_scroll_offset + visible_count, total) {
                        let song = &self.online_search_results[i];
                        let is_selected = i == self.online_selected_index;
                        let is_downloading = self.online_downloading && self.online_downloading_index == Some(i);

                        let song_keys = Self::online_song_match_keys(song);
                        let is_playing = current_file
                            .as_ref()
                            .map(|f| {
                                let current_key = Self::normalize_song_key(&f.name);
                                song_keys.iter().any(|k| {
                                    *k == current_key
                                        || self
                                            .downloaded_online_song_cache
                                            .get(k)
                                            .map(|p| *p == f.path)
                                            .unwrap_or(false)
                                })
                            })
                            .unwrap_or(false)
                            && play_state != PlayState::Stopped;

                        let duration_str = song.duration_ms
                            .map(|ms| {
                                let secs = ms / 1000;
                                let mins = secs / 60;
                                let secs = secs % 60;
                                format!("{:02}:{:02}", mins, secs)
                            })
                            .unwrap_or_else(|| "--:--".to_string());

                        let display_name = if song.artist.is_empty() {
                            song.name.clone()
                        } else {
                            format!("{} - {}", song.artist, song.name)
                        };
                        let display_name = Self::sanitize_single_line_text(&display_name);

                        // 下载中时在时长后面追加进度百分比
                        let progress_suffix = if is_downloading {
                            format!(" [{}%]", self.online_download_percent)
                        } else {
                            String::new()
                        };

                        // 与本地播放列表保持一致：selector + star + prefix + name + duration
                        let prefix = if is_playing {
                            match play_state {
                                PlayState::Playing => ">> ",
                                PlayState::Paused => "|| ",
                                PlayState::Stopped => "[] ",
                            }
                        } else {
                            "   "
                        };
                        let selector = if is_selected { ">" } else { " " };
                        let star = " ";

                        // 动态计算固定部分宽度，使用最大值确保播放状态切换时歌曲名空间不变
                        let fixed_width = 1 // selector 宽度 (>)
                            + 1 // star 宽度 (固定为空格)
                            + 3 // prefix 宽度 (统一为3)
                            + term_display_width(duration_str.as_str())
                            + term_display_width(progress_suffix.as_str())
                            + 4; // 4个分隔空格
                        let max_width = left_width.saturating_sub(fixed_width as u16) as usize;
                        let full_display = display_name.clone();
                        let name = Self::truncate_with_ellipsis(&full_display, max_width);

                        let row_pos = (i - self.online_scroll_offset + 6) as u16;
                        if is_selected {
                            queue!(
                                stdout,
                                cursor::MoveTo(0, row_pos),
                                style::SetBackgroundColor(style::Color::DarkBlue),
                                style::Print(" ".repeat(left_width as usize)),
                                cursor::MoveTo(0, row_pos),
                            )?;
                        } else {
                            queue!(
                                stdout,
                                cursor::MoveTo(0, row_pos),
                                terminal::Clear(ClearType::UntilNewLine),
                            )?;
                        }

                        if is_playing {
                            queue!(stdout, style::SetForegroundColor(self.theme_colors.song_playing))?;
                        } else if is_downloading {
                            queue!(stdout, style::SetForegroundColor(self.theme_colors.section_title))?;
                        } else {
                            queue!(stdout, style::SetForegroundColor(self.theme_colors.song_normal))?;
                        }

                        // 确保显示行不超过左面板宽度
                        let full_line = format!("{} {} {} {} {}{}", selector, star, prefix, name, duration_str, progress_suffix);
                        let display_line = truncate_to_width(&full_line, left_width as usize);
                        queue!(
                            stdout,
                            style::Print(&display_line),
                            style::ResetColor,
                        )?;
                    }

                    // 如果没有搜索结果
                    if total == 0 {
                        let hint: String = if self.search_query.is_empty() {
                            if self.juhe_search_mode {
                                self.t().juhe_enter_hint.to_string()
                            } else {
                                self.t().online_enter_hint.to_string()
                            }
                        } else if self.online_search_page > 1 {
                            self.t().fmt_page_no_result.replace("{}", &self.online_search_page.to_string())
                        } else {
                            self.t().online_no_result.to_string()
                        };
                        queue!(
                            stdout,
                            cursor::MoveTo(1, 7),
                            style::SetForegroundColor(self.theme_colors.info_text),
                            style::Print(&hint),
                            style::ResetColor,
                        )?;
                    } else if total > 0 {
                        // 在结果列表底部显示页码信息
                        let page_info = self.t().fmt_page_info.replace("{}", &self.online_search_page.to_string());
                        let info_row = std::cmp::min(total, visible_count);
                        if info_row < visible_count {
                            queue!(
                                stdout,
                                cursor::MoveTo(1, (info_row + 6) as u16),
                                style::SetForegroundColor(self.theme_colors.info_text),
                                style::Print(&page_info),
                                style::ResetColor,
                            )?;
                        }
                    }

                    // 清除下方残留行
                    let used_rows = std::cmp::min(total, visible_count);
                    Self::clear_remaining_rows(stdout, 6, used_rows, visible_count)?;
                }
            } else {
                // 本地搜索结果列表
                let total = self.search_results.len();
                Self::adjust_scroll_offset(self.search_selected_index, &mut self.search_scroll_offset, visible_count);

                // 显示搜索结果列表
                for i in self.search_scroll_offset..std::cmp::min(self.search_scroll_offset + visible_count, total) {
                    let orig_idx = self.search_results[i];
                    if let Some(file) = playlist.files.get(orig_idx) {
                        let is_selected = i == self.search_selected_index;
                        let is_playing = current_file
                            .as_ref()
                            .map(|f| f.path == file.path)
                            .unwrap_or(false);
                        let is_favorite = self.favorites.contains(&file.path.to_string_lossy().to_string());

                        let prefix = if is_playing {
                            match play_state {
                                PlayState::Playing => ">> ",
                                PlayState::Paused => "|| ",
                                PlayState::Stopped => "[] ",
                            }
                        } else {
                            "   "
                        };

                        let selector = if is_selected { ">" } else { " " };
                        let star = if is_favorite { "*" } else { " " };

                        let duration_str = file.format_duration();
                        // 动态计算固定部分宽度，使用最大值确保播放状态切换时歌曲名空间不变
                        let fixed_width = 1 // selector 宽度 (>)
                            + 1 // star 宽度 (*)
                            + 3 // prefix 宽度 (统一为3)
                            + term_display_width(duration_str.as_str())
                            + 4; // 4个分隔空格
                        let max_width = left_width.saturating_sub(fixed_width as u16) as usize;
                        let safe_file_name = Self::sanitize_single_line_text(&file.name);
                        let name = Self::truncate_with_ellipsis(&safe_file_name, max_width);

                        let row_pos = (i - self.search_scroll_offset + 6) as u16;
                        if is_selected {
                            queue!(
                                stdout,
                                cursor::MoveTo(0, row_pos),
                                style::SetBackgroundColor(style::Color::DarkBlue),
                                style::Print(" ".repeat(left_width as usize)),
                                cursor::MoveTo(0, row_pos),
                            )?;
                        } else {
                            queue!(
                                stdout,
                                cursor::MoveTo(0, row_pos),
                                terminal::Clear(ClearType::UntilNewLine),
                            )?;
                        }
                        if is_playing {
                            queue!(stdout, style::SetForegroundColor(self.theme_colors.song_playing))?;
                        } else if is_favorite {
                            queue!(stdout, style::SetForegroundColor(self.theme_colors.section_title))?;
                        } else {
                            queue!(stdout, style::SetForegroundColor(self.theme_colors.song_normal))?;
                        }

                        // 确保显示行不超过左面板宽度
                        let full_line = format!("{} {} {} {} {}", selector, star, prefix, name, duration_str);
                        let display_line = truncate_to_width(&full_line, left_width as usize);
                        queue!(
                            stdout,
                            style::Print(&display_line),
                            style::ResetColor,
                        )?;
                    }
                }

                // 如果没有搜索结果
                if total == 0 {
                    let hint = if self.search_query.is_empty() {
                        self.t().local_search_empty_hint
                    } else {
                        self.t().local_search_no_result_hint
                    };
                    queue!(
                        stdout,
                        cursor::MoveTo(1, 7),
                        style::SetForegroundColor(self.theme_colors.info_text),
                        style::Print(hint),
                        style::ResetColor,
                    )?;
                }

                // 清除搜索结果列表下方的残留行
                let used_rows = std::cmp::min(total, visible_count);
                Self::clear_remaining_rows(stdout, 6, used_rows, visible_count)?;
            }
        } else {
            // 正常模式：显示播放列表
            let total = playlist.len();

            // 保存播放列表布局信息（用于鼠标交互）
            self.playlist_layout = Some(PlaylistLayout {
                start_row: 6,
                visible_count,
                left_width,
            });

            // 调整滚动偏移
            Self::adjust_scroll_offset(self.selected_index, &mut self.scroll_offset, visible_count);

            // 显示范围信息（如果有滚动）
            let range_info = if total > visible_count {
                self.t().fmt_current_range
                    .replacen("{}", &(self.scroll_offset + 1).to_string(), 1)
                    .replacen("{}", &std::cmp::min(self.scroll_offset + visible_count, total).to_string(), 1)
            } else {
                String::new()
            };

            let title_text = self.t().fmt_playlist_title
                .replacen("{}", &range_info, 1)
                .replacen("{}", &playlist.len().to_string(), 1);
            let title_width = unicode_width::UnicodeWidthStr::width(title_text.as_str());
            let title_padding = (left_width as usize).saturating_sub(title_width);
            queue!(
                stdout,
                cursor::MoveTo(0, 4),
                style::SetForegroundColor(self.theme_colors.section_title),
                style::Print(&title_text),
                style::Print(" ".repeat(title_padding)),
                style::ResetColor,
            )?;

            // 绘制分割线（左侧）
            queue!(
                stdout,
                cursor::MoveTo(0, 5),
                style::SetForegroundColor(self.theme_colors.section_title),
                style::Print("─".repeat(left_width as usize)),
                style::ResetColor,
            )?;

            // 显示歌曲列表（左侧）
            for i in self.scroll_offset..std::cmp::min(self.scroll_offset + visible_count, total) {
                if let Some(file) = playlist.files.get(i) {
                    let is_selected = i == self.selected_index;
                    let is_playing = current_file
                        .as_ref()
                        .map(|f| f.path == file.path)
                        .unwrap_or(false);
                    let is_favorite = self.favorites.contains(&file.path.to_string_lossy().to_string());

                    // 构建显示字符串
                    let prefix = if is_playing {
                        match play_state {
                            PlayState::Playing => ">> ",
                            PlayState::Paused => "|| ",
                            PlayState::Stopped => "[] ",
                        }
                    } else {
                        "   "
                    };

                    let selector = if is_selected { ">" } else { " " };
                    let star = if is_favorite { "*" } else { " " };

                    // 调整宽度为左侧栏宽度减去边距
                    let duration_str = file.format_duration();
                    // 动态计算固定部分宽度，使用最大值确保播放状态切换时歌曲名空间不变
                    let fixed_width = 1 // selector 宽度 (>)
                        + 1 // star 宽度 (*)
                        + 3 // prefix 宽度 (统一为3)
                        + term_display_width(duration_str.as_str())
                        + 4; // 4个分隔空格
                    let max_width = left_width.saturating_sub(fixed_width as u16) as usize;
                    let safe_file_name = Self::sanitize_single_line_text(&file.name);
                    let name = Self::truncate_with_ellipsis(&safe_file_name, max_width);

                    let row_pos = (i - self.scroll_offset + 6) as u16;
                    if is_selected {
                        // 用空格填充左面板宽度，限制蓝色背景不超出左面板区域
                        queue!(
                            stdout,
                            cursor::MoveTo(0, row_pos),
                            style::SetBackgroundColor(style::Color::DarkBlue),
                            style::Print(" ".repeat(left_width as usize)),
                            cursor::MoveTo(0, row_pos),
                        )?;
                    } else {
                        queue!(
                            stdout,
                            cursor::MoveTo(0, row_pos),
                            terminal::Clear(ClearType::UntilNewLine),
                        )?;
                    }
                    if is_playing {
                        queue!(stdout, style::SetForegroundColor(self.theme_colors.song_playing))?;
                    } else if is_favorite {
                        queue!(stdout, style::SetForegroundColor(self.theme_colors.section_title))?;
                    } else {
                        queue!(stdout, style::SetForegroundColor(self.theme_colors.song_normal))?;
                    }

                    // 确保显示行不超过左面板宽度
                    let full_line = format!("{} {} {} {} {}", selector, star, prefix, name, duration_str);
                    let display_line = truncate_to_width(&full_line, left_width as usize);
                    queue!(
                        stdout,
                        style::Print(&display_line),
                        style::ResetColor,
                    )?;
                }
            }

            // 清除歌曲列表下方的残留行（从搜索模式切回时可能有残留）
            let used_rows = std::cmp::min(total.saturating_sub(self.scroll_offset), visible_count);
            Self::clear_remaining_rows(stdout, 6, used_rows, visible_count)?;

            // 播放列表为空时显示提示（必须在 clear_remaining_rows 之后，否则会被清除）
            // 与歌词区域提示保持一致的样式：DarkGrey 颜色，第8行位置
            if total == 0 {
                queue! {
                    stdout,
                    cursor::MoveTo(0, 8),
                    style::SetForegroundColor(style::Color::DarkGrey),
                    style::Print(self.t().playlist_no_dir_hint),
                    style::ResetColor,
                }?;
            }
        }

        // 绘制中间竖线分隔符
        for row in 4..self.terminal_height.saturating_sub(6) {
            queue!(
                stdout,
                cursor::MoveTo(left_width, row),
                style::SetForegroundColor(style::Color::DarkGrey),
                style::Print("│"),
                style::ResetColor,
            )?;
        }

        // 绘制右侧标题（歌词/评论/API 配置输入/GitHub Token 输入）
        let lyrics_title = if self.api_key_input_mode {
            let prompt = match self.api_input_step {
                0 => self.t().input_api_url
                .to_string(),
                1 => self.t().input_api_key
                .to_string(),
                2 => self.t().input_model_name
                .to_string(),
                _ => String::new(),
            };
            let prompt_width = unicode_width::UnicodeWidthStr::width(prompt.as_str());
            let value_max_width = (right_width.saturating_sub(1) as usize).saturating_sub(prompt_width);
            let visible_value = tail_to_width(self.api_key_input_value.as_str(), value_max_width);
            format!("{}{}", prompt, visible_value)
        } else if self.github_token_input_mode {
            let prompt = self.t().input_github_token.to_string();
            let prompt_width = unicode_width::UnicodeWidthStr::width(prompt.as_str());
            let value_max_width = (right_width.saturating_sub(1) as usize).saturating_sub(prompt_width);
            let visible_value = tail_to_width(self.github_token_input_value.as_str(), value_max_width);
            format!("{}{}", prompt, visible_value)
        } else if self.comments_mode {
            self.t().fmt_comments_title
                .replacen("{}", &self.comments_total.to_string(), 1)
                .replacen("{}", &self.comments_page.to_string(), 1)
        } else if self.song_info_mode {
            let label = self.t().song_info_label;
            if let Some(ref file) = current_file {
                let clean_name = file.name.trim_end_matches(".mp3").trim_end_matches(".flac").trim_end_matches(".wav").trim_end_matches(".ogg").trim_end_matches(".m4a").trim_end_matches(".aac").trim_end_matches(".wma");
                format!("{} {}", label, clean_name)
            } else {
                label.to_string()
            }
        } else if self.help_mode {
            self.t().help_label.to_string()
        } else if let Some(ref file) = current_file {
            format!(
                "{}{}",
                self.t().lyrics_label_with_song,
                Self::sanitize_single_line_text(&file.name)
            )
        } else {
            self.t().lyrics_label_no_song.to_string()
        };
        
        // 截断标题以适应右侧宽度
        let title = truncate_to_width(&lyrics_title, right_width.saturating_sub(1) as usize);

        // 检查是否需要重绘歌词标题（标题改变、窗口大小改变、或模式切换时强制重绘）
        let window_size_changed = self.cached_terminal_width != self.terminal_width;
        let title_changed = self.cached_lyrics_title.as_ref() != Some(&title);
        let current_mode_state = (
            self.search_mode,
            self.favorites_mode,
            self.dir_history_mode,
            self.online_search_mode,
            self.comments_mode,
            self.song_info_mode,
            self.api_key_input_mode,
            self.help_mode,
        );
        let mode_changed = self.prev_mode_state != current_mode_state;
        self.prev_mode_state = current_mode_state;

        if title_changed || window_size_changed || mode_changed {
            self.cached_lyrics_title = Some(title.clone());
            if window_size_changed {
                self.cached_terminal_width = self.terminal_width;
            }
            
            queue!(
                stdout,
                cursor::MoveTo(left_width + 1, 4),
                terminal::Clear(ClearType::UntilNewLine),
                style::SetForegroundColor(self.theme_colors.section_title),
                style::Print(&title),
                style::ResetColor,
            )?;
        }

        // 绘制右侧分割线
        queue!(
            stdout,
            cursor::MoveTo(left_width + 1, 5),
            style::SetForegroundColor(self.theme_colors.section_title),
            style::Print("─".repeat(right_width as usize)),
            style::ResetColor,
        )?;

        // 显示右侧内容（歌词/评论）
        drop(playlist); // 释放 playlist 锁
        if self.comments_mode {
            self.draw_comments(stdout, left_width + 1, right_width, visible_count)?;
        } else if self.song_info_mode {
            self.draw_song_info(stdout, left_width + 1, right_width, visible_count)?;
        } else if self.help_mode {
            self.draw_help(stdout, left_width + 1, right_width, visible_count)?;
        } else {
            self.draw_lyrics(stdout, left_width + 1, right_width, visible_count)?;
        }

        Ok(())
    }

    /// 绘制帮助信息（右侧）
    fn draw_help<W: Write>(
        &mut self,
        stdout: &mut W,
        start_x: u16,
        width: u16,
        visible_count: usize,
    ) -> io::Result<()> {
        // 帮助视图不使用歌词拖动布局
        self.lyrics_area_layout = None;

        let help_lines = self.get_help_lines();
        let total_lines = help_lines.len();
        let max_offset = total_lines.saturating_sub(visible_count);
        if self.help_scroll_offset > max_offset {
            self.help_scroll_offset = max_offset;
        }

        for i in 0..visible_count {
            let line_idx = i + self.help_scroll_offset;
            queue!(
                stdout,
                cursor::MoveTo(start_x, (i + 6) as u16),
                terminal::Clear(ClearType::UntilNewLine),
            )?;

            if line_idx < help_lines.len() {
                let line = &help_lines[line_idx];
                if line.starts_with('§') {
                    // 标题行（§ 前缀标记），去掉前缀字符
                    let display_text = &line['§'.len_utf8()..];
                    queue!(
                        stdout,
                        style::SetForegroundColor(self.theme_colors.section_title),
                        style::Print(truncate_to_width(display_text, width as usize)),
                        style::ResetColor,
                    )?;
                } else if line.starts_with('→') {
                    // 快捷键行
                    queue!(
                        stdout,
                        style::SetForegroundColor(self.theme_colors.song_normal),
                        style::Print(truncate_to_width(line, width as usize)),
                        style::ResetColor,
                    )?;
                } else {
                    // 普通文本行（URL 使用 OSC 8 超链接，参考 Discussion 状态行）
                    if let Some(url_start) = line.find("http://").or_else(|| line.find("https://")) {
                        let prefix = &line[..url_start];
                        let url = line[url_start..].trim_end();

                        let max_width = width as usize;
                        let prefix_text = truncate_to_width(prefix, max_width);
                        let prefix_width = unicode_width::UnicodeWidthStr::width(prefix_text.as_str());

                        queue!(
                            stdout,
                            style::SetForegroundColor(self.theme_colors.song_normal),
                            style::Print(&prefix_text),
                        )?;

                        if prefix_width < max_width && !url.is_empty() {
                            let remain_width = max_width - prefix_width;
                            let url_text = truncate_to_width(url, remain_width);
                            let hyperlink = format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", url, url_text);
                            queue!(
                                stdout,
                                style::Print(&hyperlink),
                            )?;
                        }

                        queue!(stdout, style::ResetColor)?;
                    } else {
                        queue!(
                            stdout,
                            style::SetForegroundColor(self.theme_colors.song_normal),
                            style::Print(truncate_to_width(line, width as usize)),
                            style::ResetColor,
                        )?;
                    }
                }
            }
        }

        Ok(())
    }

    /// 获取帮助信息文本行
    fn get_help_lines(&self) -> Vec<String> {
        self.t().help_lines.iter().map(|s| (*s).to_string()).collect()
    }
    /// 绘制歌词（右侧）
    fn draw_lyrics<W: Write>(
        &mut self,
        stdout: &mut W,
        start_x: u16,
        width: u16,
        visible_count: usize,
    ) -> io::Result<()> {
        // 提前获取需要的信息后立即释放锁
        let (current_file, current_time) = {
            let audio_player = self.audio_player.lock().unwrap();
            (audio_player.get_current_file(), audio_player.get_progress().0)
        };

        // 每帧重建歌词区域布局（用于鼠标拖动跳转）
        self.lyrics_area_layout = None;

        // 检查常规歌词后台下载结果（作为聚合失败后的兜底）
        self.check_lyrics_download_result();

        // 检查是否需要更新歌词
        let needs_update = match (&current_file, &self.lyrics_file_path) {
            (Some(file), Some(cached_path)) => {
                let lrc_path = file.path.with_extension("lrc");
                cached_path != &lrc_path
            }
            (Some(_), None) => true,
            (None, _) => false,
        };

        // 更新歌词（如果需要）
        if needs_update {
            if let Some(ref file) = current_file {
                let lrc_path = file.path.with_extension("lrc");

                // 先尝试从本地加载（不阻塞）
                if let Some(lyrics) = Lyrics::from_local_lrc(&file.path) {
                    self.current_lyrics = Some(lyrics);
                } else if lrc_path.exists() {
                    // 本地已存在歌词文件（即使解析失败）也不再触发网络下载
                    self.current_lyrics = None;
                } else if !self.juhe_lyrics_loading && !self.lyrics_downloading {
                    // 本地不存在歌词文件：先走聚合搜索歌词，常规下载作为兜底
                    let file_stem = file.path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("");
                    let (artist, title) = crate::lyrics::Lyrics::parse_artist_title(file_stem)
                        .unwrap_or_else(|| ("".to_string(), file_stem.to_string()));
                    self.juhe_lyrics_loading = true;
                    self.juhe_lyrics_rx = Some(
                        crate::search::search_and_get_juhe_lyrics_background(
                            artist, title, file.path.clone()
                        )
                    );
                    // 暂时清空歌词，显示下载提示
                    self.current_lyrics = None;
                }
                self.lyrics_file_path = Some(lrc_path);
            }
        }

        // 清除歌词区域所有行（避免切换歌曲时残留旧歌词）
        for i in 0..visible_count {
            queue!(
                stdout,
                cursor::MoveTo(start_x, (i + 6) as u16),
                terminal::Clear(ClearType::UntilNewLine),
            )?;
        }

        // 绘制歌词
        if let Some(ref lyrics) = self.current_lyrics {
            if !lyrics.is_empty() {
                let (_, visible_lines, highlight_idx) =
                    lyrics.get_visible_lines(current_time, visible_count);

                self.lyrics_area_layout = Some(LyricsAreaLayout {
                    start_row: 6,
                    start_col: start_x as usize,
                    width: width as usize,
                    line_times: visible_lines.iter().map(|line| line.time).collect(),
                });

                for (i, line) in visible_lines.iter().enumerate() {
                    let is_highlight = highlight_idx.map(|h| h == i).unwrap_or(false);

                    // 截断过长的歌词
                    let safe_line = Self::sanitize_single_line_text(&line.text);
                    let text = truncate_to_width(&safe_line, width.saturating_sub(2) as usize);

                    queue!(
                        stdout,
                        cursor::MoveTo(start_x, (i + 6) as u16),
                        terminal::Clear(ClearType::UntilNewLine),
                    )?;

                    if is_highlight {
                        queue!(
                            stdout,
                            style::SetForegroundColor(self.theme_colors.lyric_highlight),
                            style::Print(format!("> {}", text)),
                            style::ResetColor,
                        )?;
                    } else {
                        queue!(
                            stdout,
                            style::SetForegroundColor(self.theme_colors.song_normal),
                            style::Print(format!("  {}", text)),
                            style::ResetColor,
                        )?;
                    }
                }
            } else {
                // 没有歌词内容
                queue!(
                    stdout,
                    cursor::MoveTo(start_x, 8),
                    style::SetForegroundColor(style::Color::DarkGrey),
                    style::Print(self.t().instrumental_no_lyrics),
                    style::ResetColor,
                )?;
            }
        } else if self.lyrics_downloading || self.juhe_lyrics_loading {
            // 正在后台下载（常规下载或聚合搜索下载）
            queue!(
                stdout,
                cursor::MoveTo(start_x, 8),
                terminal::Clear(ClearType::UntilNewLine),
                style::SetForegroundColor(style::Color::DarkGrey),
                style::Print(self.t().downloading_lyrics),
                style::ResetColor,
            )?;
        } else if current_file.is_some() {
            // 没有找到歌词文件
            queue!(
                stdout,
                cursor::MoveTo(start_x, 8),
                style::SetForegroundColor(style::Color::DarkGrey),
                style::Print(self.t().no_lyrics_found),
                style::ResetColor,
            )?;
        } else {
            queue!(
                stdout,
                cursor::MoveTo(start_x, 8),
                style::SetForegroundColor(style::Color::DarkGrey),
                style::Print(self.t().select_song_for_lyrics),
                style::ResetColor,
            )?;
        }

        Ok(())
    }

    /// 绘制评论（右侧）
    fn draw_comments<W: Write>(
        &mut self,
        stdout: &mut W,
        start_x: u16,
        width: u16,
        visible_count: usize,
    ) -> io::Result<()> {
        // 评论视图不使用歌词拖动布局
        self.lyrics_area_layout = None;

        let current_file = {
            let audio_player = self.audio_player.lock().unwrap();
            audio_player.get_current_file()
        };

        self.ensure_comments_up_to_date(current_file.as_ref());

        self.comments_row_map = vec![None; visible_count];

        for i in 0..visible_count {
            queue!(
                stdout,
                cursor::MoveTo(start_x, (i + 6) as u16),
                terminal::Clear(ClearType::UntilNewLine),
            )?;
        }

        // 仅在无当前歌曲且无存储评论时显示"请选择歌曲播放"
        // 评论模式下即使歌曲已停止/切换，仍可查看之前加载的评论
        if current_file.is_none() && self.comments_song_name.is_empty() {
            queue!(
                stdout,
                cursor::MoveTo(start_x, 8),
                style::SetForegroundColor(style::Color::DarkGrey),
                style::Print(self.t().select_song_to_play),
                style::ResetColor,
            )?;
            return Ok(());
        }

        if self.current_comments.is_empty() {
            if self.comments_loading {
                return Ok(());
            }
            queue!(
                stdout,
                cursor::MoveTo(start_x, 8),
                style::SetForegroundColor(style::Color::DarkGrey),
                style::Print(self.t().no_comments),
                style::ResetColor,
            )?;
            return Ok(());
        }

        if self.comments_selected_index >= self.current_comments.len() {
            self.comments_selected_index = self.current_comments.len().saturating_sub(1);
        }

        // 详情模式：展示选中评论的完整内容（含回复）
        if self.comments_detail_mode {
            let selected = &self.current_comments[self.comments_selected_index];
            let mut lines: Vec<String> = Vec::new();

            // 语义顺序：有 beReplied 时，先显示被回复的原评论，再显示当前这条回复
            if let Some(reply) = &selected.reply {
                lines.push(self.t().fmt_comment_name.replace("{}", &reply.nickname));
                let origin_comment_line = reply.content.to_string();
                lines.extend(wrap_text_to_width(
                    &origin_comment_line,
                    width.saturating_sub(1) as usize,
                ));
                // 时间统一显示在"评论内容"下面，不显示在"回复评论"下面
                if let Some(time_text) = reply.time_text.as_ref().or(selected.time_text.as_ref()) {
                    lines.push(time_text.clone());
                }

                lines.push(String::new());
                lines.push(self.t().fmt_comment_name.replace("{}", &selected.nickname));
                let reply_comment_line = selected.content.to_string();
                lines.extend(wrap_text_to_width(
                    &reply_comment_line,
                    width.saturating_sub(1) as usize,
                ));
            } else {
                // 非回复场景：仅显示当前评论
                lines.push(self.t().fmt_comment_name.replace("{}", &selected.nickname));
                let content_line = selected.content.to_string();
                lines.extend(wrap_text_to_width(
                    &content_line,
                    width.saturating_sub(1) as usize,
                ));
                if let Some(time_text) = &selected.time_text {
                    lines.push(time_text.to_string());
                }
            }

            for (row, line) in lines.iter().take(visible_count).enumerate() {
                queue!(
                    stdout,
                    cursor::MoveTo(start_x, (row + 6) as u16),
                    terminal::Clear(ClearType::UntilNewLine),
                    style::SetForegroundColor(self.theme_colors.song_normal),
                    style::Print(truncate_to_width(line, width.saturating_sub(1) as usize)),
                    style::ResetColor,
                )?;
            }

            return Ok(());
        }

        // 列表模式：仅显示摘要，按 Enter 查看全文
        Self::adjust_scroll_offset(
            self.comments_selected_index,
            &mut self.comments_scroll_offset,
            visible_count.max(1),
        );

        for row in 0..visible_count {
            let comment_idx = self.comments_scroll_offset + row;
            if comment_idx >= self.current_comments.len() {
                break;
            }

            self.comments_row_map[row] = Some(comment_idx);
            let comment = &self.current_comments[comment_idx];
            let is_selected = comment_idx == self.comments_selected_index;

            // 列表仅展示歌曲评论本体：若当前项是"回复评论"，则显示其被回复的原评论
            let (list_nickname, list_content) = if let Some(reply) = &comment.reply {
                (reply.nickname.as_str(), reply.content.as_str())
            } else {
                (comment.nickname.as_str(), comment.content.as_str())
            };

            let prefix = if is_selected { "> " } else { "  " };
            let nickname_part = self.t().fmt_comment_name.replace("{}", list_nickname);
            let full_text = format!("{}{}{}", prefix, nickname_part, list_content);
            let display_text = Self::truncate_with_ellipsis(&full_text, width.saturating_sub(1) as usize);

            if is_selected {
                queue!(stdout, style::SetBackgroundColor(style::Color::DarkBlue))?;
            }

            queue!(
                stdout,
                cursor::MoveTo(start_x, (row + 6) as u16),
                terminal::Clear(ClearType::UntilNewLine),
            )?;

            queue!(
                stdout,
                style::SetForegroundColor(self.theme_colors.song_normal),
                style::Print(display_text),
                style::ResetColor,
            )?;
        }

        Ok(())
    }

    /// 绘制控制栏（快捷键提示 + 播放状态信息）
    fn draw_controls<W: Write>(&mut self, stdout: &mut W) -> io::Result<()> {
        let (state, volume, mode) = {
            let audio_player = self.audio_player.lock().unwrap();
            (audio_player.get_state(), audio_player.get_volume(), audio_player.get_play_mode())
        };

        // 控制栏分隔线位置：倒数第6行
        let control_line = self.terminal_height.saturating_sub(6);

        queue!(
            stdout,
            cursor::MoveTo(0, control_line),
            terminal::Clear(ClearType::UntilNewLine),
            style::SetForegroundColor(self.theme_colors.section_title),
            style::Print("─".repeat(self.terminal_width as usize)),
            style::ResetColor,
        )?;

        // 播放状态
        let state_str = self.play_state_text(state).to_string();
        let vol_bar: String = "█".repeat(volume as usize / 5);
        let vol_empty: String = "░".repeat(20 - volume as usize / 5);

        let state_label = self.t().play_status_label;
        let volume_label = self.t().volume_label;
        let mode_label = self.t().play_mode_label;

        // 快捷键提示行和播放状态信息行
        let tip_line = self.terminal_height.saturating_sub(5);
        let info_line = self.terminal_height.saturating_sub(4);
        let term_width = self.terminal_width as usize;

        if self.terminal_height > 4 {
            // 获取快捷键提示文本
            let help_text = self.get_help_tip_text();
            let help_width = unicode_width::UnicodeWidthStr::width(help_text.as_str());

            if help_width <= term_width {
                // 快捷键文本不超长，正常显示
                queue!(
                    stdout,
                    cursor::MoveTo(0, tip_line),
                    terminal::Clear(ClearType::UntilNewLine),
                    style::SetForegroundColor(style::Color::DarkGrey),
                    style::Print(&help_text),
                    style::ResetColor,
                )?;

                // 播放状态信息（独立行）
                let vol_prefix = format!("{}: {} | {}: [", state_label, state_str, volume_label);
                let vol_prefix_width = unicode_width::UnicodeWidthStr::width(vol_prefix.as_str());
                self.volume_bar_layout = Some(VolumeBarLayout {
                    row: info_line,
                    bar_start_col: vol_prefix_width,
                    bar_width: 20,
                });

                queue!(
                    stdout,
                    cursor::MoveTo(0, info_line),
                    terminal::Clear(ClearType::UntilNewLine),
                    style::SetForegroundColor(self.theme_colors.info_text),
                    style::Print(format!("{}: {} | ", state_label, state_str)),
                    style::Print(format!(
                        "{}: [{}{}] {:3}% | ",
                        volume_label, vol_bar, vol_empty, volume
                    )),
                    style::Print(format!("{}: {}", mode_label, self.play_mode_text(mode))),
                    style::ResetColor,
                )?;
            } else {
                // 快捷键文本超长，需要换行显示
                let (first_part, overflow) = Self::split_at_display_width(&help_text, term_width);

                // 第一行：快捷键文本
                queue!(
                    stdout,
                    cursor::MoveTo(0, tip_line),
                    terminal::Clear(ClearType::UntilNewLine),
                    style::SetForegroundColor(style::Color::DarkGrey),
                    style::Print(first_part),
                    style::ResetColor,
                )?;

                // 第二行：快捷键溢出部分（DarkGrey）+ 空格分隔 + 播放状态信息（info_text）
                let separator = "  ";
                let play_info = format!("{}: {} | {}: [{}{}] {:3}% | {}: {}",
                    state_label, state_str, volume_label, vol_bar, vol_empty, volume, mode_label, self.play_mode_text(mode));
                let play_info_width = unicode_width::UnicodeWidthStr::width(play_info.as_str());
                let separator_width = unicode_width::UnicodeWidthStr::width(separator);

                // 计算溢出文本可占的最大宽度（为播放状态信息留出空间）
                let max_overflow_width = term_width.saturating_sub(play_info_width + separator_width);
                let (overflow_fit, _) = if unicode_width::UnicodeWidthStr::width(overflow) > max_overflow_width {
                    Self::split_at_display_width(overflow, max_overflow_width)
                } else {
                    (overflow, "")
                };

                let overflow_fit_width = unicode_width::UnicodeWidthStr::width(overflow_fit);

                // 计算音量条位置（需要考虑溢出文本和分隔符的偏移）
                let vol_prefix = format!("{}: {} | {}: [", state_label, state_str, volume_label);
                let vol_prefix_width = overflow_fit_width + separator_width + unicode_width::UnicodeWidthStr::width(vol_prefix.as_str());
                self.volume_bar_layout = Some(VolumeBarLayout {
                    row: info_line,
                    bar_start_col: vol_prefix_width,
                    bar_width: 20,
                });

                queue!(
                    stdout,
                    cursor::MoveTo(0, info_line),
                    terminal::Clear(ClearType::UntilNewLine),
                    // 溢出的快捷键文本（跟前面快捷键同色 DarkGrey）
                    style::SetForegroundColor(style::Color::DarkGrey),
                    style::Print(overflow_fit),
                    // 分隔空格
                    style::Print(separator),
                    // 播放状态信息（info_text 颜色）
                    style::SetForegroundColor(self.theme_colors.info_text),
                    style::Print(&play_info),
                    style::ResetColor,
                )?;
            }
        } else {
            // 终端高度不够，只显示播放状态信息
            let vol_prefix = format!("{}: {} | {}: [", state_label, state_str, volume_label);
            let vol_prefix_width = unicode_width::UnicodeWidthStr::width(vol_prefix.as_str());
            self.volume_bar_layout = Some(VolumeBarLayout {
                row: info_line,
                bar_start_col: vol_prefix_width,
                bar_width: 20,
            });

            queue!(
                stdout,
                cursor::MoveTo(0, info_line),
                terminal::Clear(ClearType::UntilNewLine),
                style::SetForegroundColor(self.theme_colors.info_text),
                style::Print(format!("{}: {} | ", state_label, state_str)),
                style::Print(format!(
                    "{}: [{}{}] {:3}% | ",
                    volume_label, vol_bar, vol_empty, volume
                )),
                style::Print(format!("{}: {}", mode_label, self.play_mode_text(mode))),
                style::ResetColor,
            )?;
        }

        Ok(())
    }

    /// 绘制状态栏
    fn draw_status<W: Write>(&mut self, stdout: &mut W) -> io::Result<()> {
        // 克隆消息字符串
        let message = self.status_message.clone();

        // 一次性获取所有需要的音频信息，避免多次加锁
        let (play_state, realtime_volume, progress_info) = {
            let audio_player = self.audio_player.lock().unwrap();
            let state = audio_player.get_state();
            let volume = audio_player.get_realtime_volume();
            let progress = if state != PlayState::Stopped {
                let (current, total) = audio_player.get_progress();
                let time_str = audio_player.format_progress();
                let progress_percent = if let Some(total_dur) = total {
                    if total_dur.as_secs() > 0 {
                        (current.as_secs_f64() / total_dur.as_secs_f64()).min(1.0)
                    } else {
                        0.0
                    }
                } else {
                    0.0
                };
                Some((time_str, progress_percent))
            } else {
                None
            };
            (state, volume, progress)
        };

        // 状态栏位置：底部第1行
        let status_line = self.terminal_height.saturating_sub(1);

        queue!(
            stdout,
            cursor::MoveTo(0, status_line),
            terminal::Clear(ClearType::UntilNewLine),
            style::SetForegroundColor(style::Color::Yellow),
            style::Print("─".repeat(self.terminal_width as usize)),
            style::ResetColor,
        )?;

        // 播放进度（状态栏上面，如果有足够空间）
        if self.terminal_height > 1 {
            let progress_line = self.terminal_height.saturating_sub(2);
            let prefix = self.t().progress_label;

            if let Some((time_str, progress_percent)) = progress_info {
                let bar_width = self.terminal_width as usize;
                let prefix_len = unicode_width::UnicodeWidthStr::width(prefix);
                let time_len = time_str.len();
                let bar_total = bar_width.saturating_sub(prefix_len + time_len + 3);
                let filled = (bar_total as f64 * progress_percent) as usize;
                let empty = bar_total.saturating_sub(filled);

                // 保存进度条布局信息（用于鼠标点击定位）
                // 格式：前缀 + 时间 + 空格 + [ + bar + ]
                self.progress_bar_layout = Some(ProgressBarLayout {
                    row: progress_line,
                    bar_start_col: prefix_len + time_len + 2, // 前缀宽度 + 时间字符数 + 空格(1) + [(1)
                    bar_width: bar_total,
                });

                let progress_bar = format!("[{}{}]", "█".repeat(filled), "░".repeat(empty));

                queue!(
                    stdout,
                    cursor::MoveTo(0, progress_line),
                    terminal::Clear(ClearType::UntilNewLine),
                    style::SetForegroundColor(self.theme_colors.progress_text),
                    style::Print(format!("{}{} {}", prefix, time_str, progress_bar)),
                    style::ResetColor,
                )?;
            } else {
                let bar_width = self.terminal_width as usize;
                let prefix_len = unicode_width::UnicodeWidthStr::width(prefix);
                let time_text = "--:--/--:-- ";
                let bar_total = bar_width.saturating_sub(prefix_len + time_text.len() + 2);

                // 停止状态也保存布局（不可点击，因为无法 seek）
                self.progress_bar_layout = None;

                queue!(
                    stdout,
                    cursor::MoveTo(0, progress_line),
                    terminal::Clear(ClearType::UntilNewLine),
                    style::SetForegroundColor(style::Color::DarkGrey),
                    style::Print(format!("{}{}[{}]", prefix, time_text, "░".repeat(bar_total))),
                    style::ResetColor,
                )?;
            }
        } else {
            // 终端高度不够显示进度条，清除布局
            self.progress_bar_layout = None;
        }

        // 状态消息（播放进度上面，如果有足够空间）
        if self.terminal_height > 2 {
            let msg_line = self.terminal_height.saturating_sub(3);

            if play_state == PlayState::Stopped {
                // 停止状态也显示进度条样式
                let stopped_msg = self.t().stopped_status;
                let msg_width = unicode_width::UnicodeWidthStr::width(stopped_msg);
                let bar_total = (self.terminal_width as usize).saturating_sub(msg_width + 3).max(10);

                queue!(
                    stdout,
                    cursor::MoveTo(0, msg_line),
                    terminal::Clear(ClearType::UntilNewLine),
                    style::SetForegroundColor(style::Color::DarkGrey),
                    style::Print(format!("{} [{}]", stopped_msg, "░".repeat(bar_total))),
                    style::ResetColor,
                )?;
            } else {
                // 构建状态消息，如果是播放中或暂停中则添加波形动画（占满整行）
                let display_msg = if (play_state == PlayState::Playing || play_state == PlayState::Paused)
                    && self.is_now_playing_message(&message)
                {
                    // 只有播放中才更新波形帧（暂停时波形固定不变）
                    if play_state == PlayState::Playing {
                        self.wave_frame = self.wave_frame.wrapping_add(1);
                    }

                    // 生成波形数据（使用真实音量）
                    self.generate_wave_visual_full_width(&message, realtime_volume)
                } else {
                    message
                };

                queue!(
                    stdout,
                    cursor::MoveTo(0, msg_line),
                    terminal::Clear(ClearType::UntilNewLine),
                    style::SetForegroundColor(self.theme_colors.status_text),
                    style::Print(display_msg),
                    style::ResetColor,
                )?;
            }
        }

        Ok(())
    }
    
    /// 生成横向波形可视化字符串（进度条样式，占满整行）
    fn generate_wave_visual_full_width(&mut self, message: &str, realtime_volume: f32) -> String {
        use std::f64::consts::PI;
        
        // 计算消息文本占用的显示宽度
        let msg_width = unicode_width::UnicodeWidthStr::width(message);
        
        // 计算波形条可用的总宽度（整行 - 消息宽度 - 3个字符：空格和方括号）
        let available_width = self.terminal_width as usize;
        let bar_total = available_width.saturating_sub(msg_width + 3);
        
        // 确保 bar_total 至少有 10 个字符
        let bar_total = bar_total.max(10);
        
        // 使用真实音量作为基础
        // realtime_volume 是 0.0-1.0 的值
        let base_volume = realtime_volume as f64;
        
        // 添加平滑处理，避免波形跳变太快
        let frame = self.wave_frame as f64;
        let time = frame * 0.1;
        
        // 添加轻微的波动效果，使波形看起来更自然
        let wave_variation = (time * PI * 2.5).sin() * 0.05;
        
        // 计算最终的音量强度
        let intensity = (base_volume + wave_variation).clamp(0.02, 1.0);
        
        // 计算填充长度
        let filled = (intensity * bar_total as f64).round() as usize;
        let filled = filled.min(bar_total);
        let empty = bar_total.saturating_sub(filled);
        
        format!("{} [{}{}]", message, "█".repeat(filled), "░".repeat(empty))
    }

    /// 处理键盘事件
    fn handle_key_event(&mut self, code: KeyCode) -> io::Result<()> {
        // API 配置输入模式（三步：接口地址 → API Key → 模型名称）
        if self.api_key_input_mode {
            match code {
                KeyCode::Esc => {
                    self.api_key_input_mode = false;
                    self.api_key_input_for_song_info = false;
                    self.api_input_step = 0;
                    self.api_key_input_value.clear();
                    self.cached_lyrics_title = None;
                }
                KeyCode::Enter => {
                    let value = self.api_key_input_value.trim().to_string();
                    match self.api_input_step {
                        0 => {
                            // 步骤1：保存接口地址，进入 API Key 输入
                            self.api_base_url = if value.is_empty() {
                                "https://api.deepseek.com/".to_string()
                            } else {
                                // 确保以 / 结尾
                                if value.ends_with('/') { value } else { format!("{}/", value) }
                            };
                            self.api_input_step = 1;
                            self.api_key_input_value = self.resolved_api_key().unwrap_or_default();
                            self.cached_lyrics_title = None;
                        }
                        1 => {
                            // 步骤2：保存 API Key，进入模型名称输入
                            self.api_key = value.clone();
                            if value.is_empty() {
                                std::env::remove_var("DEEPSEEK_API_KEY");
                            } else {
                                std::env::set_var("DEEPSEEK_API_KEY", &value);
                            }
                            self.api_input_step = 2;
                            self.api_key_input_value = self.api_model.clone();
                            self.cached_lyrics_title = None;
                        }
                        2 => {
                            // 步骤3：保存模型名称，完成配置
                            self.api_model = if value.is_empty() {
                                "deepseek-v4-flash".to_string()
                            } else {
                                value
                            };
                            self.save_config_now();
                            let continue_song_info = self.api_key_input_for_song_info;
                            self.api_key_input_mode = false;
                            self.api_key_input_for_song_info = false;
                            self.api_input_step = 0;
                            self.api_key_input_value.clear();
                            self.cached_lyrics_title = None;

                            if continue_song_info {
                                self.start_song_info_mode_for_current_song();
                            }
                        }
                        _ => {}
                    }
                }
                KeyCode::Backspace => {
                    self.api_key_input_value.pop();
                    self.cached_lyrics_title = None;
                }
                KeyCode::Char(c) => {
                    self.api_key_input_value.push(c);
                    self.cached_lyrics_title = None;
                }
                _ => {}
            }
            return Ok(());
        }

        // GitHub Token 输入模式
        if self.github_token_input_mode {
            match code {
                KeyCode::Esc => {
                    self.github_token_input_mode = false;
                    self.github_token_input_value.clear();
                    self.cached_lyrics_title = None;
                }
                KeyCode::Enter => {
                    let value = self.github_token_input_value.trim().to_string();
                    // 用户输入为空时存储空字符串，使用时再回退到默认 token
                    self.github_token = value;
                    self.github_token_input_mode = false;
                    self.github_token_input_value.clear();
                    self.cached_lyrics_title = None;
                    self.save_config_now();
                }
                KeyCode::Backspace => {
                    self.github_token_input_value.pop();
                    self.cached_lyrics_title = None;
                }
                KeyCode::Char(c) => {
                    self.github_token_input_value.push(c);
                    self.cached_lyrics_title = None;
                }
                _ => {}
            }
            return Ok(());
        }

        // 音乐目录模式下的键盘处理
        if self.dir_history_mode {
            match code {
                KeyCode::Esc => {
                    // 退出音乐目录模式，返回播放列表
                    self.dir_history_mode = false;
                    self.dir_history_selected_index = 0;
                    self.dir_history_scroll_offset = 0;
                }
                KeyCode::Enter => {
                    // 切换到选中的目录
                    if self.dir_history_selected_index < self.dir_history.len() {
                        let dir_path = self.dir_history[self.dir_history_selected_index].clone();
                        self.dir_history_mode = false;
                        self.dir_history_selected_index = 0;
                        self.dir_history_scroll_offset = 0;
                        self.load_directory(&dir_path);
                    }
                }
                KeyCode::Up => {
                    if self.dir_history_selected_index > 0 {
                        self.dir_history_selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    if self.dir_history_selected_index < self.dir_history.len().saturating_sub(1) {
                        self.dir_history_selected_index += 1;
                    }
                }
                KeyCode::Char('d') | KeyCode::Char('D') => {
                    // 删除目录记录
                    if self.dir_history_selected_index < self.dir_history.len() {
                        self.dir_history.remove(self.dir_history_selected_index);
                        if self.dir_history_selected_index >= self.dir_history.len() && self.dir_history_selected_index > 0 {
                            self.dir_history_selected_index -= 1;
                        }
                        self.save_config_now();
                    }
                }
                _ => {}
            }
            return Ok(());
        }

        // 收藏模式下的键盘处理
        if self.favorites_mode {
            match code {
                KeyCode::Esc => {
                    // 退出收藏列表模式，返回播放列表
                    self.favorites_mode = false;
                    self.favorites_selected_index = 0;
                    self.favorites_scroll_offset = 0;
                }
                KeyCode::Enter => {
                    // 播放收藏列表中选中的歌曲
                    let fav_idx = self.favorites_selected_index;
                    if fav_idx < self.favorites.len() {
                        let orig_idx = self.get_fav_orig_index(fav_idx);
                        if let Some(idx) = orig_idx {
                            // 歌曲在当前目录中，直接播放
                            self.selected_index = idx;
                            self.favorites_mode = false;
                            self.favorites_selected_index = 0;
                            self.favorites_scroll_offset = 0;
                            self.play_song_by_index(idx);
                        } else {
                            // 歌曲不在当前目录，需要先切换目录
                            let fav_path = self.favorites[fav_idx].clone();
                            let parent_dir = std::path::Path::new(&fav_path)
                                .parent()
                                .map(|p| p.to_string_lossy().to_string());
                            if let Some(dir) = parent_dir {
                                self.favorites_mode = false;
                                self.favorites_selected_index = 0;
                                self.favorites_scroll_offset = 0;
                                self.load_directory_and_play(&dir, &fav_path);
                            }
                        }
                    }
                }
                KeyCode::Up => {
                    if self.favorites_selected_index > 0 {
                        self.favorites_selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    if self.favorites_selected_index < self.favorites.len().saturating_sub(1) {
                        self.favorites_selected_index += 1;
                    }
                }
                KeyCode::Char('d') | KeyCode::Char('D') => {
                    // 删除收藏
                    if self.favorites_selected_index < self.favorites.len() {
                        self.favorites.remove(self.favorites_selected_index);
                        // 调整选中索引
                        if self.favorites_selected_index >= self.favorites.len() && self.favorites_selected_index > 0 {
                            self.favorites_selected_index -= 1;
                        }
                        self.save_config_now();
                    }
                }
                _ => {}
            }
            return Ok(());
        }

        // 搜索模式下的键盘处理（本地搜索和网络搜索共用此逻辑）
        if self.search_mode {
            let in_playlist_detail = self.online_search_mode && self.playlist_search_mode && self.current_playlist.is_some();
            let online_input_focused = self.online_search_mode && !in_playlist_detail && self.search_input_focused;
            let mut handled_in_search = true;
            match code {
                KeyCode::Esc => {
                    // 优先关闭右侧视图层（评论/歌曲信息/帮助），避免误退到歌单搜索列表
                    if self.comments_mode {
                        self.comments_mode = false;
                        self.comments_detail_mode = false;
                    } else if self.song_info_mode {
                        self.song_info_mode = false;
                    } else if self.help_mode {
                        self.help_mode = false;
                    } else if self.playlist_search_mode && self.current_playlist.is_some() {
                        // 歌单歌曲页返回歌单搜索结果页
                        self.search_input_focused = false;
                        self.current_playlist = None;
                        self.online_search_results.clear();
                        self.online_selected_index = self.playlist_list_selected_index;
                        self.online_scroll_offset = self.online_selected_index.saturating_sub(2);
                        let total = self.playlist_search_results.len();
                        Self::clamp_selected_and_scroll(
                            &mut self.online_selected_index,
                            &mut self.online_scroll_offset,
                            total,
                            (self.terminal_height as usize).saturating_sub(12).max(5),
                        );
                        self.online_searching = false;
                        self.playlist_songs_rx = None;
                    } else {
                        // 退出搜索模式
                        self.search_mode = false;
                        self.online_search_mode = false;
                        self.juhe_search_mode = false;
                        self.playlist_search_mode = false;
                        self.search_query.clear();
                        self.search_results.clear();
                        self.search_selected_index = 0;
                        self.search_scroll_offset = 0;
                        self.online_search_results.clear();
                        self.online_selected_index = 0;
                        self.online_scroll_offset = 0;
                        self.online_searching = false;
                        self.online_search_page = 1;
                        self.online_search_rx = None;
                        self.playlist_search_rx = None;
                        self.playlist_songs_rx = None;
                        self.playlist_search_results.clear();
                        self.current_playlist = None;
                    }
                }
                KeyCode::Enter => {
                    // 评论/帮助模式下，Enter 由全局处理器处理
                    if self.comments_mode || self.help_mode {
                        handled_in_search = false;
                    } else if self.online_search_mode {
                        if self.online_searching {
                            // 正在搜索中，忽略
                        } else if self.online_downloading {
                            // 正在下载中，忽略
                        } else if self.playlist_search_mode && self.current_playlist.is_none() {
                            if !self.playlist_search_results.is_empty() {
                                if let Some(pl) = self.playlist_search_results.get(self.online_selected_index).cloned() {
                                    self.playlist_list_selected_index = self.online_selected_index;
                                    self.online_searching = true;
                                    self.online_search_results.clear();
                                    self.online_selected_index = 0;
                                    self.online_scroll_offset = 0;
                                    self.current_playlist = Some(pl.clone());
                                    self.playlist_songs_rx = Some(crate::search::fetch_playlist_songs_background(pl));
                                    // 进入列表操作态：按键默认作用于列表/全局快捷键
                                    self.search_input_focused = false;
                                }
                            } else if online_input_focused && !self.search_query.is_empty() {
                                self.online_search_page = 1;
                                self.start_online_search();
                            }
                        } else if !self.online_search_results.is_empty() {
                            // 有搜索结果：下载选中的歌曲，并切到列表操作态
                            if let Some(song) = self.online_search_results.get(self.online_selected_index).cloned() {
                                // 用户主动点播在线歌曲，开始新一轮播放尝试，重置自动切歌节流窗口
                                self.online_auto_skip_times.clear();
                                self.start_download_song(song);
                                self.search_input_focused = false;
                            }
                        } else if online_input_focused && !self.search_query.is_empty() {
                            // 无搜索结果且输入框有焦点：触发网络搜索（从第1页开始）
                            self.online_search_page = 1;
                            self.start_online_search();
                        }
                    } else if !self.search_results.is_empty() {
                        // 本地搜索：有搜索结果时，播放选中的歌曲
                        if let Some(&orig_idx) = self.search_results.get(self.search_selected_index) {
                            self.selected_index = orig_idx;
                            self.search_mode = false;
                            self.search_input_focused = false;
                            self.search_query.clear();
                            self.search_results.clear();
                            self.search_scroll_offset = 0;
                            self.play_song_by_index(orig_idx);
                        }
                    } else if !self.search_query.is_empty() {
                        // 本地搜索：无搜索结果且有输入，触发搜索
                        self.update_search_results();
                    }
                }
                KeyCode::Up => {
                    // 评论/歌曲信息/帮助模式下，上下键控制右侧视图而非搜索列表
                    if self.comments_mode || self.song_info_mode || self.help_mode {
                        handled_in_search = false;
                    } else if self.online_search_mode {
                        if self.online_selected_index > 0 {
                            self.online_selected_index -= 1;
                        }
                    } else if self.search_selected_index > 0 {
                        self.search_selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    // 评论/歌曲信息/帮助模式下，上下键控制右侧视图而非搜索列表
                    if self.comments_mode || self.song_info_mode || self.help_mode {
                        handled_in_search = false;
                    } else if self.online_search_mode {
                        let total = if self.playlist_search_mode && self.current_playlist.is_none() {
                            self.playlist_search_results.len()
                        } else {
                            self.online_search_results.len()
                        };
                        if total > 0 && self.online_selected_index < total - 1 {
                            self.online_selected_index += 1;
                        }
                    } else if !self.search_results.is_empty() && self.search_selected_index < self.search_results.len() - 1 {
                        self.search_selected_index += 1;
                    }
                }
                KeyCode::Backspace => {
                    if self.online_search_mode {
                        // 在线搜索列表焦点态：Backspace 不再编辑搜索框，交给全局按键分支
                        if !online_input_focused {
                            handled_in_search = false;
                        } else if !self.search_query.is_empty() {
                            self.search_query.pop();
                            // 关键字变化时清空旧搜索结果
                            self.online_search_results.clear();
                            self.playlist_search_results.clear();
                            self.current_playlist = None;
                            self.online_selected_index = 0;
                            self.online_scroll_offset = 0;
                            self.online_search_page = 1;
                        }
                    } else {
                        // 本地搜索：删除最后一个字符，清空旧结果以便按回车重新搜索
                        self.search_query.pop();
                        self.search_results.clear();
                        self.search_selected_index = 0;
                        self.search_scroll_offset = 0;
                    }
                }
                KeyCode::Char(c) => {
                    // 进入歌单详情后，不再把键盘输入写入搜索框；空格改为播放/暂停
                    if in_playlist_detail {
                        if c == ' ' {
                            let mut audio_player = self.audio_player.lock().unwrap();
                            match audio_player.get_state() {
                                PlayState::Playing => audio_player.pause(),
                                PlayState::Paused => audio_player.resume(),
                                _ => {}
                            }
                        } else {
                            // 其余字符按键交给下方全局快捷键分支处理
                            handled_in_search = false;
                        }
                    } else if self.online_search_mode && !online_input_focused {
                        // 在线搜索列表焦点态：字符键不写入搜索框，交给全局快捷键分支
                        handled_in_search = false;
                    } else {
                        // 输入搜索关键字
                        self.search_query.push(c);
                        if self.online_search_mode {
                            // 网络搜索模式：关键字变化时清空旧搜索结果，以便按 Enter 触发新搜索
                            if !self.online_search_results.is_empty() || !self.playlist_search_results.is_empty() || self.current_playlist.is_some() {
                                self.online_search_results.clear();
                                self.playlist_search_results.clear();
                                self.current_playlist = None;
                                self.online_selected_index = 0;
                                self.online_scroll_offset = 0;
                                self.online_search_page = 1;
                            }
                        } else {
                            // 本地搜索：关键字变化时清空旧结果，按 Enter 重新搜索
                            self.search_results.clear();
                            self.search_selected_index = 0;
                            self.search_scroll_offset = 0;
                        }
                    }
                }
                KeyCode::PageUp => {
                    // 评论/歌曲信息/帮助模式下，PageUp 由全局处理器处理
                    if self.comments_mode || self.song_info_mode || self.help_mode {
                        handled_in_search = false;
                    } else if self.playlist_search_mode && self.current_playlist.is_some() {
                        // 歌单详情页：翻到上一“页”（每页20首）
                        let page_size = 20usize;
                        let total = self.online_search_results.len();
                        if total > 0 {
                            let cur_page = self.online_selected_index / page_size;
                            let prev_page = cur_page.saturating_sub(1);
                            self.online_selected_index = prev_page * page_size;
                            self.online_scroll_offset = self.online_selected_index;
                            Self::clamp_selected_and_scroll(
                                &mut self.online_selected_index,
                                &mut self.online_scroll_offset,
                                total,
                                (self.terminal_height as usize).saturating_sub(12).max(5),
                            );
                        }
                    } else if self.online_search_mode && !self.online_searching && self.online_search_page > 1 {
                        // 网络搜索翻上一页
                        self.online_search_page -= 1;
                        self.start_online_search();
                    }
                }
                KeyCode::PageDown => {
                    // 评论/歌曲信息/帮助模式下，PageDown 由全局处理器处理
                    if self.comments_mode || self.song_info_mode || self.help_mode {
                        handled_in_search = false;
                    } else if self.playlist_search_mode && self.current_playlist.is_some() {
                        // 歌单详情页：翻到下一“页”（每页20首）
                        let page_size = 20usize;
                        let total = self.online_search_results.len();
                        if total > 0 {
                            let cur_page = self.online_selected_index / page_size;
                            let next_index = (cur_page + 1) * page_size;
                            if next_index < total {
                                self.online_selected_index = next_index;
                                self.online_scroll_offset = self.online_selected_index;
                                Self::clamp_selected_and_scroll(
                                    &mut self.online_selected_index,
                                    &mut self.online_scroll_offset,
                                    total,
                                    (self.terminal_height as usize).saturating_sub(12).max(5),
                                );
                            }
                        }
                    } else if self.online_search_mode && !self.online_searching {
                        // 网络搜索翻下一页
                        let has_results = if self.playlist_search_mode && self.current_playlist.is_none() {
                            !self.playlist_search_results.is_empty()
                        } else {
                            !self.online_search_results.is_empty()
                        };
                        if has_results {
                            self.online_search_page += 1;
                            self.start_online_search();
                        }
                    }
                }
                _ => {
                    if in_playlist_detail {
                        // 歌单详情中未在搜索分支处理的按键，继续走全局快捷键
                        handled_in_search = false;
                    }
                }
            }
            if handled_in_search {
                return Ok(());
            }
        }

        // 正常模式下的键盘处理
        match code {
            KeyCode::Up => {
                if self.help_mode {
                    if self.help_scroll_offset > 0 {
                        self.help_scroll_offset -= 1;
                    }
                } else if self.song_info_mode {
                    if self.song_info_scroll_offset > 0 {
                        self.song_info_scroll_offset -= 1;
                    }
                } else if self.comments_mode {
                    self.comments_selected_index = self.comments_selected_index.saturating_sub(1);
                    let visible_count = self.terminal_height.saturating_sub(12) as usize;
                    Self::adjust_scroll_offset(
                        self.comments_selected_index,
                        &mut self.comments_scroll_offset,
                        visible_count.max(1),
                    );
                } else {
                    // 上移选择
                    let playlist = self.playlist.lock().unwrap();
                    if self.selected_index > 0 {
                        self.selected_index -= 1;
                    } else if !playlist.is_empty() {
                        self.selected_index = playlist.len() - 1;
                    }
                }
            }
            KeyCode::Down => {
                if self.help_mode {
                    let help_lines = self.get_help_lines();
                    let visible_count = self.terminal_height.saturating_sub(12) as usize;
                    let max_offset = help_lines.len().saturating_sub(visible_count);
                    if self.help_scroll_offset < max_offset {
                        self.help_scroll_offset += 1;
                    }
                } else if self.song_info_mode {
                    let visible_count = self.terminal_height.saturating_sub(12) as usize;
                    let left_width = (self.terminal_width as f32 * 0.50) as u16;
                    let right_width = self.terminal_width.saturating_sub(left_width + 1);
                    let wrapped_lines = wrap_text_to_width(&self.song_info_content, right_width.saturating_sub(1) as usize);
                    let max_offset = wrapped_lines.len().saturating_sub(visible_count);
                    if self.song_info_scroll_offset < max_offset {
                        self.song_info_scroll_offset += 1;
                    }
                } else if self.comments_mode {
                    if !self.current_comments.is_empty() {
                        let max_idx = self.current_comments.len().saturating_sub(1);
                        self.comments_selected_index = (self.comments_selected_index + 1).min(max_idx);
                        let visible_count = self.terminal_height.saturating_sub(12) as usize;
                        Self::adjust_scroll_offset(
                            self.comments_selected_index,
                            &mut self.comments_scroll_offset,
                            visible_count.max(1),
                        );
                    }
                } else {
                    // 下移选择
                    let playlist = self.playlist.lock().unwrap();
                    if self.selected_index < playlist.len() - 1 {
                        self.selected_index += 1;
                    } else {
                        self.selected_index = 0;
                    }
                }
            }
            KeyCode::Enter => {
                if self.comments_mode {
                    if !self.current_comments.is_empty() {
                        self.comments_detail_mode = !self.comments_detail_mode;
                    }
                } else if self.help_mode {
                    // 帮助视图下 Enter 不执行操作
                } else {
                    // 播放选中的歌曲
                    self.play_song_by_index(self.selected_index);
                }
            }
            KeyCode::Char(' ') => {
                // 播放/暂停
                {
                    let mut audio_player = self.audio_player.lock().unwrap();
                    let state = audio_player.get_state();
                    match state {
                        PlayState::Playing => {
                            audio_player.pause();
                        }
                        PlayState::Paused => {
                            audio_player.resume();
                        }
                        _ => {}
                    }
                }
                // 不更新状态消息，保持显示"正在播放: XXXXXX"
                // 暂停时波形会固定不变，恢复播放时波形继续动画
            }
            KeyCode::Esc => {
                if self.comments_mode {
                    // 评论视图下返回歌词视图
                    self.comments_mode = false;
                    self.comments_detail_mode = false;
                } else if self.song_info_mode {
                    // AI 信息视图下返回歌词视图
                    self.song_info_mode = false;
                } else if self.help_mode {
                    // 帮助视图下返回歌词视图
                    self.help_mode = false;
                } else {
                    // 停止播放
                    self.audio_player.lock().unwrap().stop();
                    self.update_status(self.t().state_stopped_msg);
                }
            }
            KeyCode::Left => {
                // 上一曲
                self.play_prev();
            }
            KeyCode::Right => {
                // 下一曲
                self.play_next();
            }
            KeyCode::Char('[') => {
                // 快退 5 秒
                self.seek_relative(-5.0);
            }
            KeyCode::Char(']') => {
                // 快进 5 秒
                self.seek_relative(5.0);
            }
            KeyCode::Char(',') => {
                // 快退 10 秒
                self.seek_relative(-10.0);
            }
            KeyCode::Char('.') => {
                // 快进 10 秒
                self.seek_relative(10.0);
            }
            KeyCode::Char('+') | KeyCode::Char('=') => {
                // 音量增加
                self.audio_player.lock().unwrap().volume_up();
                // 不更新状态消息，避免覆盖"正在播放:"的波形显示
                // 音量已在控制栏显示
            }
            KeyCode::Char('-') => {
                // 音量减少
                self.audio_player.lock().unwrap().volume_down();
                // 不更新状态消息，避免覆盖"正在播放:"的波形显示
                // 音量已在控制栏显示
            }
            KeyCode::Char('1') => {
                self.set_play_mode(PlayMode::Single);
            }
            KeyCode::Char('2') => {
                self.set_play_mode(PlayMode::RepeatOne);
            }
            KeyCode::Char('3') => {
                self.set_play_mode(PlayMode::Sequence);
            }
            KeyCode::Char('4') => {
                self.set_play_mode(PlayMode::LoopAll);
            }
            KeyCode::Char('5') => {
                self.set_play_mode(PlayMode::Random);
            }
            KeyCode::Char('t') | KeyCode::Char('T') => {
                // 切换主题
                self.theme = self.theme.next();
                self.theme_colors = self.theme.colors();
                // 强制重绘歌词标题，避免因标题文本未变化而保留旧主题颜色
                self.cached_lyrics_title = None;
                self.save_config_now();
            }
            KeyCode::Char('o') | KeyCode::Char('O') => {
                // 打开文件夹
                self.open_folder();
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                // 歌单详情页中禁用 s，避免误切换到搜索模式
                if self.search_mode && self.online_search_mode && self.playlist_search_mode && self.current_playlist.is_some() {
                    // ignore
                } else {
                    // 进入本地搜索模式（搜索音乐目录）
                    self.search_mode = true;
                    self.search_input_focused = true;
                    self.help_mode = false;
                    self.online_search_mode = false;
                    self.search_query.clear();
                    self.search_results.clear();
                    self.search_selected_index = 0;
                    self.search_scroll_offset = 0;
                }
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                // 歌单详情页中禁用 n，避免误切换到搜索模式
                if self.search_mode && self.online_search_mode && self.playlist_search_mode && self.current_playlist.is_some() {
                    // ignore
                } else {
                    // 进入网络搜索模式（搜索网络歌曲并下载）
                    self.search_mode = true;
                    self.search_input_focused = true;
                    self.help_mode = false;
                    self.online_search_mode = true;
                    self.juhe_search_mode = false;
                    self.playlist_search_mode = false;
                    self.search_query.clear();
                    self.online_search_results.clear();
                    self.online_selected_index = 0;
                    self.online_scroll_offset = 0;
                    self.online_searching = false;
                    self.online_search_page = 1;
                    self.online_search_rx = None;
                }
            }
            KeyCode::Char('j') | KeyCode::Char('J') => {
                // 歌单详情页中禁用 j，避免误切换到搜索模式
                if self.search_mode && self.online_search_mode && self.playlist_search_mode && self.current_playlist.is_some() {
                    // ignore
                } else {
                    // 进入聚合搜索搜索模式（通过独家API获取播放链接和歌词）
                    self.search_mode = true;
                    self.search_input_focused = true;
                    self.help_mode = false;
                    self.online_search_mode = true;
                    self.juhe_search_mode = true;
                    self.playlist_search_mode = false;
                    self.search_query.clear();
                    self.online_search_results.clear();
                    self.online_selected_index = 0;
                    self.online_scroll_offset = 0;
                    self.online_searching = false;
                    self.online_search_page = 1;
                    self.online_search_rx = None;
                }
            }
            KeyCode::Char('p') | KeyCode::Char('P') => {
                // 进入歌单搜索模式（先显示歌单，Enter进入歌单歌曲）
                self.search_mode = true;
                self.search_input_focused = true;
                self.help_mode = false;
                self.online_search_mode = true;
                self.juhe_search_mode = false;
                self.playlist_search_mode = true;
                self.search_query.clear();
                self.online_search_results.clear();
                self.playlist_search_results.clear();
                self.current_playlist = None;
                self.online_selected_index = 0;
                self.online_scroll_offset = 0;
                self.online_searching = false;
                self.online_search_page = 1;
                self.online_search_rx = None;
                self.playlist_search_rx = None;
                self.playlist_songs_rx = None;
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                // 切换到评论视图，并从第1页开始加载
                self.comments_mode = true;
                self.song_info_mode = false;
                self.help_mode = false;
                self.comments_page = 1;
                self.current_comments.clear();
                self.comments_total = 0;
                self.comments_selected_index = 0;
                self.comments_scroll_offset = 0;
                self.comments_row_map.clear();
                self.comments_detail_mode = false;
                self.comments_rx = None;
                self.comments_loading = false;
                // 设置评论对应的歌曲信息，确保翻页时使用正确的歌曲名
                let current_file = {
                    let audio_player = self.audio_player.lock().unwrap();
                    audio_player.get_current_file()
                };
                self.comments_file_path = current_file.as_ref().map(|f| f.path.clone());
                self.comments_song_name = current_file.map(|f| f.name.trim().to_string()).unwrap_or_default();
            }
            KeyCode::Char('i') | KeyCode::Char('I') => {
                // i：直接查询歌曲信息（有 DeepSeek Key 用 DeepSeek，否则用 OpenRouter 免费模型）
                self.start_song_info_mode_for_current_song();
            }
            KeyCode::Char('k') | KeyCode::Char('K') => {
                // k：进入 API 配置输入模式（接口地址 → API Key → 模型名称）
                self.open_api_key_input_mode(false);
            }
            KeyCode::Char('l') | KeyCode::Char('L') => {
                // 切换界面语言
                self.language = self.language.next();
                crate::langs::set_global_language(self.language);
                self.cached_lyrics_title = None;

                // 语言切换后立即刷新"正在播放"状态前缀，避免显示旧语言
                let current_song_name = {
                    let player = self.audio_player.lock().unwrap();
                    player.get_current_file().map(|f| f.name.clone())
                };
                if let Some(song_name) = current_song_name {
                    self.update_now_playing_status(&song_name);
                }

                // 如果正在显示歌曲信息，切换语言后重新查询
                if self.song_info_mode {
                    self.song_info_scroll_offset = 0;
                    self.song_info_rx = None;
                    self.song_info_loading = false;
                    self.song_info_content.clear();
                    self.github_discussion_status.clear();
                    self.github_discussion_rx = None;
                    self.github_discussion_loading = false;
                    self.song_info_force_scroll = false;
                    if let Some(ref file) = {
                        let player = self.audio_player.lock().unwrap();
                        player.get_current_file()
                    } {
                        self.song_info_name = file.name.trim().to_string();
                        self.start_fetch_song_info_for_current_song(&file.name);
                    }
                }

                self.save_config_now();
            }
            KeyCode::PageUp => {
                if self.comments_mode && self.comments_page > 1 {
                    self.comments_page -= 1;
                    self.current_comments.clear();
                    self.comments_selected_index = 0;
                    self.comments_scroll_offset = 0;
                    self.comments_row_map.clear();
                    self.comments_detail_mode = false;
                    self.comments_loading = false;
                    self.comments_rx = None;
                }
            }
            KeyCode::PageDown => {
                if self.comments_mode {
                    self.comments_page += 1;
                    self.current_comments.clear();
                    self.comments_selected_index = 0;
                    self.comments_scroll_offset = 0;
                    self.comments_row_map.clear();
                    self.comments_detail_mode = false;
                    self.comments_loading = false;
                    self.comments_rx = None;
                }
            }
            KeyCode::Char('f') | KeyCode::Char('F') => {
                // 在在线搜索模式（网络/聚合/歌单）下屏蔽 f 收藏，避免误操作到本地播放列表
                if self.search_mode && self.online_search_mode && self.playlist_search_mode{
                    // ignore
                } else {
                    // 切换当前选中歌曲的收藏状态（已收藏则移除，未收藏则添加）
                    let file = {
                        let playlist = self.playlist.lock().unwrap();
                        playlist.files.get(self.selected_index).cloned()
                    };
                    if let Some(file) = file {
                        let path_str = file.path.to_string_lossy().to_string();
                        if let Some(pos) = self.favorites.iter().position(|p| *p == path_str) {
                            self.favorites.remove(pos);
                            //self.update_status(&format!("已从收藏移除: {}", file.name));
                        } else {
                            self.favorites.push(path_str);
                            //self.update_status(&format!("已添加到收藏: {}", file.name));
                        }
                        self.save_config_now();
                    }
                }
            }
            KeyCode::Char('v') | KeyCode::Char('V') => {
                // 显示收藏列表
                self.favorites_mode = true;
                self.help_mode = false;
                self.favorites_selected_index = 0;
                self.favorites_scroll_offset = 0;
            }
            KeyCode::Char('m') | KeyCode::Char('M') => {
                // 显示音乐目录
                self.dir_history_mode = true;
                self.help_mode = false;
                self.dir_history_selected_index = 0;
                self.dir_history_scroll_offset = 0;
            }
            KeyCode::Char('h') | KeyCode::Char('H') => {
                // 显示帮助信息
                self.help_mode = true;
                self.help_scroll_offset = 0;
            }
            KeyCode::Char('g') | KeyCode::Char('G') => {
                // 输入 GitHub Token
                self.github_token_input_mode = true;
                // github_token 为空表示使用默认 token，输入框显示为空
                self.github_token_input_value = self.github_token.clone();
                self.cached_lyrics_title = None;
            }
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                // 退出
                *self.should_quit.lock().unwrap() = true;
            }
            _ => {}
        }

        Ok(())
    }

    /// 更新搜索结果
    fn update_search_results(&mut self) {
        let query = self.search_query.to_lowercase();
        let playlist = self.playlist.lock().unwrap();

        if query.is_empty() {
            // 空查询，结果为空
            self.search_results = Vec::new();
        } else {
            self.search_results = playlist
                .files
                .iter()
                .enumerate()
                .filter(|(_, file)| file.name.to_lowercase().contains(&query))
                .map(|(i, _)| i)
                .collect();
        }

        // 重置选择索引，确保不越界
        if self.search_selected_index >= self.search_results.len() {
            self.search_selected_index = self.search_results.len().saturating_sub(1);
        }
        self.search_scroll_offset = 0;
    }

    /// 启动网络搜索
    fn start_online_search(&mut self) {
        if self.search_query.is_empty() {
            return;
        }
        self.online_search_mode = true;
        self.online_searching = true;
        // 翻页也先清空旧结果，避免旧页内容短暂可见
        self.online_search_results.clear();
        self.playlist_search_results.clear();
        self.online_selected_index = 0;
        self.online_scroll_offset = 0;

        let query = self.search_query.clone();
        let page = self.online_search_page;
        if self.playlist_search_mode {
            let rx = crate::search::search_playlist_background(query, page);
            self.playlist_search_rx = Some(rx);
            self.online_search_rx = None;
        } else {
            let rx = if self.juhe_search_mode {
                crate::search::search_juhe_background(query, page)
            } else {
                crate::search::search_online_background(query, page)
            };
            self.online_search_rx = Some(rx);
            self.playlist_search_rx = None;
        }
    }

    /// 检查网络搜索结果
    fn check_online_search_result(&mut self) {
        if let Some(ref rx) = self.online_search_rx {
            match rx.try_recv() {
                Ok(result) => {
                    self.online_searching = false;
                    self.online_search_rx = None;
                    self.online_search_results = result.songs;
                    self.online_selected_index = 0;
                    self.online_scroll_offset = 0;
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {}
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    self.online_searching = false;
                    self.online_search_rx = None;
                    self.update_status(self.t().online_search_failed);
                }
            }
        }
        if let Some(ref rx) = self.playlist_search_rx {
            match rx.try_recv() {
                Ok(result) => {
                    self.online_searching = false;
                    self.playlist_search_rx = None;
                    self.playlist_search_results = result.playlists;
                    self.online_selected_index = 0;
                    self.online_scroll_offset = 0;
                    if self.playlist_search_results.is_empty() {
                        self.update_status(self.t().no_playlist_result);
                    }
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {}
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    self.online_searching = false;
                    self.playlist_search_rx = None;
                    self.update_status(self.t().playlist_search_failed);
                }
            }
        }
        if let Some(ref rx) = self.playlist_songs_rx {
            match rx.try_recv() {
                Ok(result) => {
                    self.online_searching = false;
                    self.playlist_songs_rx = None;
                    self.current_playlist = Some(result.playlist);
                    self.online_search_results = result.songs;
                    self.online_selected_index = 0;
                    self.online_scroll_offset = 0;
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {}
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    self.online_searching = false;
                    self.playlist_songs_rx = None;
                    self.update_status(self.t().playlist_songs_failed);
                }
            }
        }
    }

    /// 启动下载歌曲
    fn start_download_song(&mut self, song: OnlineSong) {
        // 写入日志文件
        {
            let log_msg = format!("开始下载: {} - {}, source={:?}, juhe_platform={}, juhe_song_id={}",
                song.artist, song.name, song.source, song.juhe_platform, song.juhe_song_id);
            let timestamp = Local::now().format("%H:%M:%S%.3f");
            let line = format!("[{}] {}\n", timestamp, log_msg);
            let _ = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(crate::config::get_daily_log_path())
                .and_then(|mut f| std::io::Write::write_all(&mut f, line.as_bytes()));
        }

        // 若正在下载中，统一阻塞并提示
        if self.online_downloading {
            return;
        }

        // 在线歌曲统一缓存命中：若本地已存在（缓存路径/当前目录同名同歌手），则直接播放，不再重复下载
        if let Some(path) = self.find_cached_local_path_for_online(&song) {
            let play_file = crate::defs::MusicFile::new(path.clone());
            let play_result = {
                let mut audio_player = self.audio_player.lock().unwrap();
                audio_player.play(&play_file)
            };
            if play_result.is_ok() {
                self.update_now_playing_status(&play_file.name);
                let log_msg = format!(
                    "之前已下载命中缓存，跳过下载直接播放: {} - {}, path={}",
                    song.artist,
                    song.name,
                    path.display()
                );
                let timestamp = Local::now().format("%H:%M:%S%.3f");
                let line = format!("[{}] {}\n", timestamp, log_msg);
                let _ = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(crate::config::get_daily_log_path())
                    .and_then(|mut f| std::io::Write::write_all(&mut f, line.as_bytes()));
            } else if let Err(e) = play_result {
                if self.online_auto_skip_times.is_empty() {
                    self.update_status(&format!(
                        "{}{}",
                        self.t().play_failed,
                        e
                    ));
                } else {
                    // 自动切歌链路下，缓存命中但播放失败时继续尝试下一首，不弹提示
                    self.play_next_with_flag(false);
                }
            }
            return;
        }

        if let Some(local_idx) = self.find_local_song_index_for_online(&song) {
            self.play_song_by_index(local_idx);
            self.skip_auto_lyrics_download_for_current_song = true;
            let log_msg = format!(
                "本地已存在该歌曲，跳过下载直接播放: {} - {}, local_idx={}",
                song.artist,
                song.name,
                local_idx
            );
            let timestamp = Local::now().format("%H:%M:%S%.3f");
            let line = format!("[{}] {}\n", timestamp, log_msg);
            let _ = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(crate::config::get_daily_log_path())
                .and_then(|mut f| std::io::Write::write_all(&mut f, line.as_bytes()));
            return;
        }

        let save_dir = {
            let playlist = self.playlist.lock().unwrap();
            // 保存到当前音乐目录，无目录时使用默认音乐目录（配置目录/music）
            playlist.directory
                .as_ref()
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|| crate::config::get_default_music_dir())
        };

        self.online_downloading_index = Some(self.online_selected_index);
        self.online_downloading = true;
        self.online_download_percent = 0;
        let _display_name = if song.artist.is_empty() {
            song.name.clone()
        } else {
            format!("{} - {}", song.artist, song.name)
        };


        let rx = crate::search::download_song_background(song, save_dir);
        self.online_download_rx = Some(rx);
    }

    /// 检查本地歌词后台下载结果（常规下载兜底）
    fn check_lyrics_download_result(&mut self) {
        let current_file = {
            let audio_player = self.audio_player.lock().unwrap();
            audio_player.get_current_file()
        };

        if let Some(ref rx) = self.lyrics_download_rx {
            if let Ok(result) = rx.try_recv() {
                // 仅当结果仍对应当前歌曲时才更新歌词，避免切歌后串写
                let is_current = current_file
                    .as_ref()
                    .map(|f| f.path == result.music_path)
                    .unwrap_or(false);

                if is_current {
                    if let Some(lyrics) = result.lyrics {
                        self.append_runtime_log("[LyricsFallback] 兜底歌词下载成功，已更新当前歌词");
                        self.current_lyrics = Some(lyrics);
                    } else {
                        self.append_runtime_log("[LyricsFallback] 兜底歌词下载失败（未返回歌词）");
                    }
                } else {
                    self.append_runtime_log("[LyricsFallback] 忽略旧任务结果（已切歌）");
                }

                self.lyrics_download_rx = None;
                self.lyrics_downloading = false;
            }
        }
    }

    /// 检查歌词高亮行是否变化（用于判断是否需要重绘歌词区域）
    /// 检查下载进度/结果
    fn check_download_result(&mut self) {
        if let Some(ref rx) = self.online_download_rx {
            // 非阻塞地读取所有可用消息
            while let Ok(progress) = rx.try_recv() {
                match progress {
                    DownloadProgress::Progress(percent) => {
                        self.online_download_percent = percent;
                    }
                    DownloadProgress::Done(result) => {
                        self.online_downloading = false;
                        self.online_download_rx = None;
                        self.online_download_percent = 0;

                        // 写入日志
                        {
                            let log_msg = format!("下载完成: path={:?}, error={:?}",
                                result.local_path, result.error);
                            let timestamp = Local::now().format("%H:%M:%S%.3f");
                            let line = format!("[{}] {}\n", timestamp, log_msg);
                            let _ = std::fs::OpenOptions::new()
                                .create(true)
                                .append(true)
                                .open(crate::config::get_daily_log_path())
                                .and_then(|mut f| std::io::Write::write_all(&mut f, line.as_bytes()));
                        }

                        let downloaded_song = result.song.clone();
                        match result.local_path {
                            Some(path) => {
                                self.skip_auto_lyrics_download_for_current_song = false;
                                self.remember_downloaded_online_song(&downloaded_song, &path);
                                // 歌单内歌曲：只播放下载完成的歌曲，不改写本地播放列表，避免串到本地下一首
                                if self.search_mode
                                    && self.online_search_mode
                                    && self.playlist_search_mode
                                    && self.current_playlist.is_some()
                                {
                                    let play_file = crate::defs::MusicFile::new(path.clone());
                                    let play_result = {
                                        let mut audio_player = self.audio_player.lock().unwrap();
                                        audio_player.play(&play_file)
                                    };
                                    if play_result.is_ok() {
                                        self.update_now_playing_status(&play_file.name);
                                    } else if let Err(e) = play_result {
                                        if self.online_auto_skip_times.is_empty() {
                                            self.update_status(&format!(
                                                "{}{}",
                                                self.t().play_failed,
                                                e
                                            ));
                                        } else {
                                            // 自动切歌链路下，当前首播放失败时继续尝试下一首，不弹提示
                                            self.play_next_with_flag(false);
                                        }
                                    }
                                } else {
                                    // 普通网络/聚合：沿用原逻辑，重扫目录并播放
                                    let path_str = path.to_string_lossy().to_string();
                                    let dir = {
                                        let playlist = self.playlist.lock().unwrap();
                                        playlist.directory.clone()
                                            .unwrap_or_else(|| crate::config::get_default_music_dir().to_string_lossy().to_string())
                                    };
                                    self.load_directory_and_play(&dir, &path_str);
                                }
                            }
                            None => {
                                // 下载失败，不做提示以避免覆盖波形
                                let _err = result.error.unwrap_or_else(|| self.t().unknown_error.to_string());
                            }
                        }
                        break;
                    }
                }
            }
        }
    }

    /// 检查聚合搜索歌词下载结果
    fn check_juhe_lyrics_result(&mut self) {
        if let Some(ref rx) = self.juhe_lyrics_rx {
            if let Ok(result) = rx.try_recv() {
                self.juhe_lyrics_loading = false;
                self.juhe_lyrics_rx = None;

                let target_music_path = result.music_path.clone();
                let current_music_path = {
                    let audio_player = self.audio_player.lock().unwrap();
                    audio_player.get_current_file().map(|f| f.path)
                };
                let is_current = current_music_path
                    .as_ref()
                    .map(|p| p == &target_music_path)
                    .unwrap_or(false);

                // 切歌后收到旧任务结果：直接丢弃，避免写错歌和 UI 串位
                if !is_current {
                    return;
                }

                if let Some(lyrics_content) = result.lyrics {
                    // 聚合歌词成功后，明确关闭兜底状态（若有残留）
                    self.lyrics_download_rx = None;
                    self.lyrics_downloading = false;

                    // 将歌词保存到对应歌曲同目录的 .lrc 文件
                    let mut saved_lrc_path: Option<std::path::PathBuf> = None;
                    let music_path = std::path::Path::new(&target_music_path);
                    let clean_name = music_path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown");
                    if let Some(parent) = music_path.parent() {
                        let lrc_path = parent.join(format!("{}.lrc", clean_name));
                        let _ = std::fs::write(&lrc_path, &lyrics_content);
                        saved_lrc_path = Some(lrc_path);
                    }

                    // 解析歌词并更新显示
                    if let Some(lrc_path) = saved_lrc_path {
                        self.lyrics_file_path = Some(lrc_path.clone());
                        self.current_lyrics = crate::lyrics::Lyrics::from_local_lrc(&lrc_path);
                    }
                } else if !self.lyrics_downloading && self.current_lyrics.is_none() {
                    // 仅当聚合失败且当前还没有歌词时，才回退到常规歌词下载
                    self.append_runtime_log("[JuheLyrics] 聚合歌词失败，开始兜底歌词下载");
                    self.current_lyrics = None;
                    self.lyrics_download_rx = Some(Lyrics::download_lyrics_background(&target_music_path));
                    self.lyrics_downloading = true;
                } else {
                    self.append_runtime_log("[JuheLyrics] 聚合歌词失败，但当前已有歌词，跳过兜底");
                }
            }
        }
    }

    /// 获取收藏列表中指定索引对应的原始播放列表索引
    fn get_fav_orig_index(&self, fav_index: usize) -> Option<usize> {
        let path = self.favorites.get(fav_index)?;
        let playlist = self.playlist.lock().unwrap();
        playlist.files.iter().enumerate()
            .find(|(_, f)| f.path.to_string_lossy() == *path)
            .map(|(i, _)| i)
    }

    /// 归一化歌曲匹配键（用于在线歌曲与本地文件名匹配）
    fn normalize_song_key(input: &str) -> String {
        input
            .to_lowercase()
            .chars()
            .filter(|c| !c.is_whitespace() && !matches!(c, '-' | '_' | '·' | '•' | ',' | '，' | '.' | '。' | '(' | ')' | '（' | '）' | '[' | ']' | '【' | '】'))
            .collect()
    }

    fn online_song_match_keys(song: &OnlineSong) -> Vec<String> {
        let name = song.name.trim();
        if name.is_empty() {
            return Vec::new();
        }

        let artist = song.artist.trim();
        let full = if artist.is_empty() {
            name.to_string()
        } else {
            format!("{} - {}", artist, name)
        };

        let full_key = Self::normalize_song_key(&full);
        let name_key = Self::normalize_song_key(name);
        if full_key == name_key {
            vec![name_key]
        } else {
            vec![full_key, name_key]
        }
    }

    fn remember_downloaded_online_song(&mut self, song: &OnlineSong, path: &std::path::Path) {
        for key in Self::online_song_match_keys(song) {
            self.downloaded_online_song_cache.insert(key, path.to_path_buf());
        }
    }

    fn find_cached_local_path_for_online(&mut self, song: &OnlineSong) -> Option<std::path::PathBuf> {
        for key in Self::online_song_match_keys(song) {
            if let Some(path) = self.downloaded_online_song_cache.get(&key).cloned() {
                if path.exists() {
                    return Some(path);
                }
                self.downloaded_online_song_cache.remove(&key);
            }
        }
        None
    }

    /// 在本地播放列表中查找与在线歌曲匹配的条目索引
    fn find_local_song_index_for_online(&self, song: &OnlineSong) -> Option<usize> {
        let keys = Self::online_song_match_keys(song);
        if keys.is_empty() {
            return None;
        }

        let full_key = keys.first().cloned().unwrap_or_default();
        let name_key = keys.last().cloned().unwrap_or_default();
        let artist_key = Self::normalize_song_key(song.artist.trim());

        let playlist = self.playlist.lock().unwrap();
        playlist.files.iter().enumerate().find_map(|(idx, f)| {
            let local_key = Self::normalize_song_key(&f.name);
            if local_key == full_key || local_key == name_key {
                return Some(idx);
            }
            if !artist_key.is_empty() && !name_key.is_empty() && local_key.contains(&artist_key) && local_key.contains(&name_key) {
                return Some(idx);
            }
            None
        })
    }

    /// 根据索引播放歌曲（内部辅助方法，消除重复代码）
    fn play_song_by_index(&mut self, index: usize) {
        // 常规切歌默认允许自动歌词下载（缓存命中直播放会在调用后重新置为 true）
        self.skip_auto_lyrics_download_for_current_song = false;

        let file = {
            let playlist = self.playlist.lock().unwrap();
            playlist.files.get(index).cloned()
        };

        if let Some(file) = file {
            // 切歌时重置歌词下载状态
            self.lyrics_download_rx = None;
            self.lyrics_downloading = false;
            self.juhe_lyrics_rx = None;
            self.juhe_lyrics_loading = false;

            // 切歌时重置评论状态（评论模式下不重置，保持用户正在查看的评论）
            if !self.comments_mode {
                self.comments_file_path = None;
                self.comments_song_name.clear();
                self.comments_total = 0;
                self.comments_page = 1;
                self.current_comments.clear();
                self.comments_selected_index = 0;
                self.comments_scroll_offset = 0;
                self.comments_row_map.clear();
                self.comments_detail_mode = false;
                self.comments_rx = None;
                self.comments_loading = false;
            }

            // 切歌时重置 AI 歌曲信息状态
            self.song_info_file_path = None;
            self.song_info_content.clear();
            self.song_info_rx = None;
            self.song_info_loading = false;
            self.song_info_name.clear();
            self.github_discussion_status.clear();
            self.github_discussion_rx = None;
            self.github_discussion_loading = false;
            self.song_info_force_scroll = false;

            let play_result = {
                let mut audio_player = self.audio_player.lock().unwrap();
                audio_player.play(&file)
            };

            match play_result {
                Ok(()) => {
                    {
                        let mut playlist = self.playlist.lock().unwrap();
                        playlist.current_index = Some(index);
                    }
                    self.selected_index = index;
                    self.update_now_playing_status(&file.name);
                    // 歌曲切换成功后保存配置
                    self.save_config_now();
                }
                Err(e) => {
                    self.update_status(&format!(
                        "{}{}",
                        self.t().play_failed,
                        e
                    ));
                }
            }
        }
    }

    /// 处理鼠标事件
    fn handle_mouse_event(&mut self, mouse_event: MouseEvent) -> io::Result<()> {
        let col = mouse_event.column as usize;
        let row = mouse_event.row;
        let in_playlist_detail = self.search_mode
            && self.online_search_mode
            && self.playlist_search_mode
            && self.current_playlist.is_some();

        match mouse_event.kind {
            MouseEventKind::Down(button) => {
                // 只处理左键点击
                if button != MouseButton::Left {
                    return Ok(());
                }

                // 进入新的左键流程前，先重置歌词拖动状态
                self.lyrics_dragging = false;
                self.lyrics_drag_target_time = None;

                // 所有模式：检查是否点击了音量条区域
                if let Some(layout) = self.volume_bar_layout {
                    if row == layout.row && col >= layout.bar_start_col && col < layout.bar_start_col + layout.bar_width {
                        // 音量条共20格，点击位置按比例映射到0-100，四舍五入到5的倍数
                        let ratio = (col - layout.bar_start_col) as f64 / (layout.bar_width - 1) as f64;
                        let new_volume = (ratio * 100.0 / 5.0).round() as u8 * 5;
                        let new_volume = new_volume.clamp(0, 100);

                        self.audio_player.lock().unwrap().set_volume(new_volume);
                        return Ok(());
                    }
                }

                // 所有模式：检查是否点击了进度条区域
                if let Some(layout) = self.progress_bar_layout {
                    if row == layout.row && col >= layout.bar_start_col && col < layout.bar_start_col + layout.bar_width {
                        // 计算点击位置在进度条中的比例
                        let ratio = (col - layout.bar_start_col) as f64 / layout.bar_width as f64;
                        let ratio = ratio.clamp(0.0, 1.0);

                        // 执行 seek
                        let seek_result = {
                            let mut player = self.audio_player.lock().unwrap();
                            player.seek(ratio)
                        };

                        if let Err(e) = seek_result {
                            self.update_status(&self.t().fmt_seek_failed.replace("{}", &e));
                        }
                        return Ok(());
                    }
                }

                // 所有模式：歌词区域左键按下，进入拖动选择模式（松开时才跳转）
                if let Some(target_time) = self.lyric_time_at_position(col, row) {
                    self.lyrics_dragging = true;
                    self.lyrics_drag_target_time = Some(target_time);
                    return Ok(());
                }

                // 搜索模式：鼠标点击选择/播放搜索结果
                if self.search_mode {
                    // 点击搜索栏（第4行左侧）时，切回输入框焦点
                    let left_width = (self.terminal_width as f32 * 0.50) as usize;
                    let in_playlist_detail = self.online_search_mode
                        && self.playlist_search_mode
                        && self.current_playlist.is_some();
                    if self.online_search_mode && !in_playlist_detail && row == 4 && col < left_width {
                        self.search_input_focused = true;
                        return Ok(());
                    }

                    if let Some(layout) = self.playlist_layout {
                        if col < layout.left_width as usize && row >= layout.start_row {
                            let click_row = (row - layout.start_row) as usize;
                            if click_row < layout.visible_count {
                                if self.online_search_mode {
                                    // 点击在线搜索列表后，键盘默认作用于列表/全局快捷键
                                    self.search_input_focused = false;
                                    if self.playlist_search_mode && self.current_playlist.is_none() {
                                        // 歌单搜索结果：点击进入歌单
                                        let clicked_index = self.online_scroll_offset + click_row;
                                        if clicked_index < self.playlist_search_results.len() {
                                            self.online_selected_index = clicked_index;
                                            self.playlist_list_selected_index = clicked_index;
                                            if let Some(pl) = self.playlist_search_results.get(clicked_index).cloned() {
                                                self.online_searching = true;
                                                self.online_search_results.clear();
                                                self.online_selected_index = 0;
                                                self.online_scroll_offset = 0;
                                                self.current_playlist = Some(pl.clone());
                                                self.playlist_songs_rx = Some(crate::search::fetch_playlist_songs_background(pl));
                                            }
                                        }
                                    } else {
                                        // 网络歌曲结果：点击选择并下载
                                        let clicked_index = self.online_scroll_offset + click_row;
                                        if clicked_index < self.online_search_results.len() {
                                            self.online_selected_index = clicked_index;
                                            if let Some(song) = self.online_search_results.get(clicked_index).cloned() {
                                                if !self.online_downloading {
                                                    // 用户鼠标主动点播在线歌曲，重置自动切歌节流窗口
                                                    self.online_auto_skip_times.clear();
                                                    self.start_download_song(song);
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    // 本地搜索结果：点击选择并播放
                                    let clicked_index = self.search_scroll_offset + click_row;
                                    if clicked_index < self.search_results.len() {
                                        if let Some(&orig_idx) = self.search_results.get(clicked_index) {
                                            self.selected_index = orig_idx;
                                            self.search_mode = false;
                                            self.search_input_focused = false;
                                            self.search_query.clear();
                                            self.search_results.clear();
                                            self.search_scroll_offset = 0;
                                            self.play_song_by_index(orig_idx);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    return Ok(());
                }

                // 音乐目录模式：鼠标点击选择并切换目录
                if self.dir_history_mode {
                    if let Some(layout) = self.playlist_layout {
                        if col < layout.left_width as usize && row >= layout.start_row {
                            let click_row = (row - layout.start_row) as usize;
                            if click_row < layout.visible_count {
                                let clicked_index = self.dir_history_scroll_offset + click_row;
                                if clicked_index < self.dir_history.len() {
                                    let dir_path = self.dir_history[clicked_index].clone();
                                    self.dir_history_mode = false;
                                    self.dir_history_selected_index = 0;
                                    self.dir_history_scroll_offset = 0;
                                    self.load_directory(&dir_path);
                                }
                            }
                        }
                    }
                    return Ok(());
                }

                // 收藏列表模式：鼠标点击选择并播放歌曲
                if self.favorites_mode {
                    if let Some(layout) = self.playlist_layout {
                        if col < layout.left_width as usize && row >= layout.start_row {
                            let click_row = (row - layout.start_row) as usize;
                            if click_row < layout.visible_count {
                                let clicked_index = self.favorites_scroll_offset + click_row;
                                if clicked_index < self.favorites.len() {
                                    let orig_idx = self.get_fav_orig_index(clicked_index);
                                    if let Some(idx) = orig_idx {
                                        self.selected_index = idx;
                                        self.favorites_mode = false;
                                        self.favorites_selected_index = 0;
                                        self.favorites_scroll_offset = 0;
                                        self.play_song_by_index(idx);
                                    } else {
                                        // 歌曲不在当前目录，需要先切换目录
                                        let fav_path = self.favorites[clicked_index].clone();
                                        let parent_dir = std::path::Path::new(&fav_path)
                                            .parent()
                                            .map(|p| p.to_string_lossy().to_string());
                                        if let Some(dir) = parent_dir {
                                            self.favorites_mode = false;
                                            self.favorites_selected_index = 0;
                                            self.favorites_scroll_offset = 0;
                                            self.load_directory_and_play(&dir, &fav_path);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    return Ok(());
                }

                // 评论模式：右侧区域左键点击，行为与 Enter 一致（选中并进入详情）
                if self.help_mode {
                    // 帮助视图：忽略右键点击
                    return Ok(());
                } else if self.comments_mode {
                    if !self.comments_detail_mode {
                        let left_width = (self.terminal_width as f32 * 0.50) as usize;
                        if col > left_width && row >= 6 {
                            let click_row = (row - 6) as usize;
                            if click_row < self.comments_row_map.len() {
                                if let Some(comment_idx) = self.comments_row_map[click_row] {
                                    if comment_idx < self.current_comments.len() {
                                        self.comments_selected_index = comment_idx;
                                        self.comments_detail_mode = true;
                                    }
                                }
                            }
                        }
                    }
                    return Ok(());
                }

                // 正常模式：检查是否点击了播放列表区域
                if let Some(layout) = self.playlist_layout {
                    if col < layout.left_width as usize && row >= layout.start_row {
                        let click_row = (row - layout.start_row) as usize;
                        if click_row < layout.visible_count {
                            let clicked_index = self.scroll_offset + click_row;
                            let playlist = self.playlist.lock().unwrap();
                            if clicked_index < playlist.len() {
                                drop(playlist);
                                // 双击播放：先选中，再播放
                                self.selected_index = clicked_index;
                                self.play_song_by_index(clicked_index);
                            }
                        }
                    }
                }
            }
            MouseEventKind::Drag(button) => {
                if button == MouseButton::Left && self.lyrics_dragging {
                    if let Some(target_time) = self.lyric_time_at_position(col, row) {
                        self.lyrics_drag_target_time = Some(target_time);
                    }
                }
            }
            MouseEventKind::Up(button) => {
                if button == MouseButton::Left && self.lyrics_dragging {
                    if let Some(target_time) = self.lyric_time_at_position(col, row) {
                        self.lyrics_drag_target_time = Some(target_time);
                    }
                    if let Some(target_time) = self.lyrics_drag_target_time {
                        self.seek_to_lyric_time(target_time);
                    }
                    self.lyrics_dragging = false;
                    self.lyrics_drag_target_time = None;
                }
            }
            MouseEventKind::ScrollUp => {
                // 所有模式：歌词区域滚轮向上 -> 跳转到上一句歌词
                if self.lyric_time_at_position(col, row).is_some() {
                    self.seek_by_lyric_wheel(-1);
                    return Ok(());
                }

                if self.song_info_mode {
                    // AI 歌曲信息模式：右侧区域滚轮向上滚动
                    let left_width = (self.terminal_width as f32 * 0.50) as usize;
                    if col > left_width && self.song_info_scroll_offset > 0 {
                        self.song_info_scroll_offset -= 1;
                    }
                } else if self.help_mode {
                    // 帮助视图：右侧区域滚轮向上滚动
                    let left_width = (self.terminal_width as f32 * 0.50) as usize;
                    if col > left_width && self.help_scroll_offset > 0 {
                        self.help_scroll_offset -= 1;
                    }
                } else if self.comments_mode {
                    let left_width = (self.terminal_width as f32 * 0.50) as usize;
                    if col > left_width && !self.current_comments.is_empty() {
                        self.comments_selected_index = self.comments_selected_index.saturating_sub(1);
                        Self::adjust_scroll_offset(
                            self.comments_selected_index,
                            &mut self.comments_scroll_offset,
                            self.comments_row_map.len().max(1),
                        );
                    }
                } else if self.search_mode {
                    // 搜索模式：滚轮向上
                    if let Some(layout) = self.playlist_layout {
                        if col < layout.left_width as usize {
                            if self.online_search_mode {
                                let total = if self.playlist_search_mode && self.current_playlist.is_none() {
                                    self.playlist_search_results.len()
                                } else {
                                    self.online_search_results.len()
                                };
                                if total > 0 {
                                    self.online_selected_index = self.online_selected_index.saturating_sub(1);
                                    Self::adjust_scroll_offset(
                                        self.online_selected_index,
                                        &mut self.online_scroll_offset,
                                        layout.visible_count.max(1),
                                    );
                                }
                            } else {
                                if self.search_scroll_offset > 0 {
                                    self.search_scroll_offset -= 1;
                                }
                            }
                        }
                    }
                } else if self.dir_history_mode {
                    // 音乐目录模式：滚轮向上
                    if let Some(layout) = self.playlist_layout {
                        if col < layout.left_width as usize
                            && self.dir_history_scroll_offset > 0 {
                                self.dir_history_scroll_offset -= 1;
                            }
                    }
                } else if self.favorites_mode {
                    // 收藏列表模式：滚轮向上
                    if let Some(layout) = self.playlist_layout {
                        if col < layout.left_width as usize
                            && self.favorites_scroll_offset > 0 {
                                self.favorites_scroll_offset -= 1;
                            }
                    }
                } else {
                    // 正常模式：在播放列表区域滚轮向上 → 列表上移
                    if let Some(layout) = self.playlist_layout {
                        if col < layout.left_width as usize && self.scroll_offset > 0 {
                            self.scroll_offset -= 1;

                            // 保持选中项处于当前可见区域，避免 draw 时被自动回拉
                            let total_len = {
                                let playlist = self.playlist.lock().unwrap();
                                playlist.len()
                            };
                            if total_len > 0 {
                                let view_start = self.scroll_offset;
                                let view_end = self
                                    .scroll_offset
                                    .saturating_add(layout.visible_count)
                                    .saturating_sub(1)
                                    .min(total_len - 1);
                                if self.selected_index < view_start {
                                    self.selected_index = view_start;
                                } else if self.selected_index > view_end {
                                    self.selected_index = view_end;
                                }
                            }
                        }
                    }
                }
            }
            MouseEventKind::ScrollDown => {
                // 所有模式：歌词区域滚轮向下 -> 跳转到下一句歌词
                // 歌单详情页优先用于歌单滚动，不触发歌词滚轮跳转
                if !in_playlist_detail && self.lyric_time_at_position(col, row).is_some() {
                    self.seek_by_lyric_wheel(1);
                    return Ok(());
                }

                if self.song_info_mode {
                    // AI 歌曲信息模式：右侧区域滚轮向下滚动
                    let left_width = (self.terminal_width as f32 * 0.50) as usize;
                    if col > left_width {
                        let visible_count = self.terminal_height.saturating_sub(12) as usize;
                        let content = self.song_info_content.clone();
                        let right_width = self.terminal_width.saturating_sub(left_width as u16 + 1);
                        let wrapped_lines = wrap_text_to_width(&content, right_width.saturating_sub(1) as usize);
                        let max_offset = wrapped_lines.len().saturating_sub(visible_count);
                        if self.song_info_scroll_offset < max_offset {
                            self.song_info_scroll_offset += 1;
                        }
                    }
                } else if self.help_mode {
                    // 帮助视图：右侧区域滚轮向下滚动
                    let left_width = (self.terminal_width as f32 * 0.50) as usize;
                    if col > left_width {
                        let help_lines = self.get_help_lines();
                        let visible_count = self.terminal_height.saturating_sub(12) as usize;
                        let max_offset = help_lines.len().saturating_sub(visible_count);
                        if self.help_scroll_offset < max_offset {
                            self.help_scroll_offset += 1;
                        }
                    }
                } else if self.comments_mode {
                    let left_width = (self.terminal_width as f32 * 0.50) as usize;
                    if col > left_width && !self.current_comments.is_empty() {
                        let max_idx = self.current_comments.len().saturating_sub(1);
                        self.comments_selected_index = (self.comments_selected_index + 1).min(max_idx);
                        Self::adjust_scroll_offset(
                            self.comments_selected_index,
                            &mut self.comments_scroll_offset,
                            self.comments_row_map.len().max(1),
                        );
                    }
                } else if self.search_mode {
                    // 搜索模式：滚轮向下
                    if let Some(layout) = self.playlist_layout {
                        let allow_scroll = in_playlist_detail || col < layout.left_width as usize;
                        if allow_scroll {
                            if self.online_search_mode {
                                let total = if self.playlist_search_mode && self.current_playlist.is_none() {
                                    self.playlist_search_results.len()
                                } else {
                                    self.online_search_results.len()
                                };
                                if total > 0 {
                                    let max_idx = total.saturating_sub(1);
                                    self.online_selected_index = (self.online_selected_index + 1).min(max_idx);
                                    Self::adjust_scroll_offset(
                                        self.online_selected_index,
                                        &mut self.online_scroll_offset,
                                        layout.visible_count.max(1),
                                    );
                                }
                            } else {
                                let max_offset = self.search_results.len().saturating_sub(layout.visible_count);
                                if self.search_scroll_offset < max_offset {
                                    self.search_scroll_offset += 1;
                                }
                            }
                        }
                    }
                } else if self.dir_history_mode {
                    // 音乐目录模式：滚轮向下
                    if let Some(layout) = self.playlist_layout {
                        if col < layout.left_width as usize {
                            let max_offset = self.dir_history.len().saturating_sub(layout.visible_count);
                            if self.dir_history_scroll_offset < max_offset {
                                self.dir_history_scroll_offset += 1;
                            }
                        }
                    }
                } else if self.favorites_mode {
                    // 收藏列表模式：滚轮向下
                    if let Some(layout) = self.playlist_layout {
                        if col < layout.left_width as usize {
                            let max_offset = self.favorites.len().saturating_sub(layout.visible_count);
                            if self.favorites_scroll_offset < max_offset {
                                self.favorites_scroll_offset += 1;
                            }
                        }
                    }
                } else {
                    // 正常模式：在播放列表区域滚轮向下 → 列表下移
                    if let Some(layout) = self.playlist_layout {
                        if col < layout.left_width as usize {
                            let total_len = {
                                let playlist = self.playlist.lock().unwrap();
                                playlist.len()
                            };
                            let max_offset = total_len.saturating_sub(layout.visible_count);
                            if self.scroll_offset < max_offset {
                                self.scroll_offset += 1;

                                // 保持选中项处于当前可见区域，避免 draw 时被自动回拉
                                if total_len > 0 {
                                    let view_start = self.scroll_offset;
                                    let view_end = self
                                        .scroll_offset
                                        .saturating_add(layout.visible_count)
                                        .saturating_sub(1)
                                        .min(total_len - 1);
                                    if self.selected_index < view_start {
                                        self.selected_index = view_start;
                                    } else if self.selected_index > view_end {
                                        self.selected_index = view_end;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// 设置播放模式
    fn set_play_mode(&mut self, mode: PlayMode) {
        self.audio_player.lock().unwrap().set_play_mode(mode);
    }

    /// 相对跳转（正数快进，负数快退，单位秒）
    fn seek_relative(&mut self, delta_secs: f64) {
        let result = {
            let mut player = self.audio_player.lock().unwrap();
            let (current, total) = player.get_progress();
            if let Some(total_dur) = total {
                let total_secs = total_dur.as_secs_f64();
                if total_secs > 0.0 {
                    let current_secs = current.as_secs_f64();
                    let target_secs = (current_secs + delta_secs).clamp(0.0, total_secs);
                    let ratio = target_secs / total_secs;
                    player.seek(ratio)
                } else {
                    Err(self.t().seek_error_zero_duration.to_string())
                }
            } else {
                Err(self.t().seek_error_unknown_duration.to_string())
            }
        };
        if let Err(e) = result {
            self.update_status(&self.t().fmt_seek_failed.replace("{}", &e));
        }
    }

    /// 获取当前“正在播放”的在线结果索引（与光标选中项分离）
    fn current_online_playing_index(&self) -> Option<usize> {
        let (current_file, play_state) = {
            let audio_player = self.audio_player.lock().unwrap();
            (audio_player.get_current_file(), audio_player.get_state())
        };

        if play_state == PlayState::Stopped {
            return None;
        }

        let current_file = current_file?;
        let current_key = Self::normalize_song_key(&current_file.name);

        self.online_search_results
            .iter()
            .enumerate()
            .find_map(|(idx, song)| {
                let song_keys = Self::online_song_match_keys(song);
                let matched = song_keys.iter().any(|k| {
                    *k == current_key
                        || self
                            .downloaded_online_song_cache
                            .get(k)
                            .map(|p| *p == current_file.path)
                            .unwrap_or(false)
                });
                if matched { Some(idx) } else { None }
            })
    }

    /// 播放下一曲（manual: 是否为用户手动切换）
    fn play_next_with_flag(&mut self, manual: bool) {
        // 在线搜索结果视图（网络/聚合/歌单歌曲）统一按在线结果模拟 1~5 播放模式
        if self.search_mode
            && self.online_search_mode
            && !self.online_search_results.is_empty()
        {
            // 手动切歌不计入节流窗口，并清空历史
            if manual {
                self.online_auto_skip_times.clear();
            }

            // 正在下载下一首时，禁止回落到本地播放列表继续播
            // 同时避免自动切歌节流被重复计数（否则会出现“第一首就停”）
            if self.online_downloading {
                return;
            }

            let mode = self.audio_player.lock().unwrap().get_play_mode();
            let len = self.online_search_results.len();
            let selected_cur = self.online_selected_index.min(len.saturating_sub(1));
            let mut cur = self
                .current_online_playing_index()
                .unwrap_or(selected_cur)
                .min(len.saturating_sub(1));

            // 自动切歌连续失败时，播放器里的“当前文件”可能仍停留在上一首，
            // 顺序/列表循环应以“上一次已尝试的在线索引”为基准继续往后推进，避免反复重试同一首。
            if !manual
                && !self.online_auto_skip_times.is_empty()
                && matches!(mode, PlayMode::Sequence | PlayMode::LoopAll)
            {
                cur = selected_cur;
            }

            let next_idx = match mode {
                PlayMode::Single => {
                    if manual { Some((cur + 1).min(len - 1)) } else { None }
                }
                PlayMode::RepeatOne => Some(cur),
                PlayMode::Sequence => {
                    if cur + 1 < len { Some(cur + 1) } else { None }
                }
                PlayMode::LoopAll => Some((cur + 1) % len),
                PlayMode::Random => {
                    // 随机播放：随机选择一首（排除当前播放项）
                    use rand::Rng;
                    let mut rng = rand::thread_rng();
                    if len <= 1 {
                        Some(cur)
                    } else {
                        let mut next = rng.gen_range(0..len - 1);
                        if next >= cur {
                            next += 1;
                        }
                        Some(next)
                    }
                }
            };

            if let Some(i) = next_idx {
                // 自动切歌节流：网络/聚合/歌单歌曲统一生效；3秒内自动切歌>=5次则直接停止（不提示）
                if !manual {
                    let now = Instant::now();

                    // 若与上一次自动切歌间隔已超过窗口，视为新一轮尝试，清空历史计数
                    if let Some(&last) = self.online_auto_skip_times.back() {
                        if now.duration_since(last) > Duration::from_secs(3) {
                            self.online_auto_skip_times.clear();
                        }
                    }

                    while let Some(&front) = self.online_auto_skip_times.front() {
                        if now.duration_since(front) > Duration::from_secs(3) {
                            self.online_auto_skip_times.pop_front();
                        } else {
                            break;
                        }
                    }

                    // 仅在真正准备发起“下一首”时计数，避免空转重复计数
                    self.online_auto_skip_times.push_back(now);

                    if self.online_auto_skip_times.len() >= 5 {
                        self.audio_player.lock().unwrap().stop();
                        self.online_auto_skip_times.clear();
                        return;
                    }
                }

                self.online_selected_index = i;
                Self::adjust_scroll_offset(
                    self.online_selected_index,
                    &mut self.online_scroll_offset,
                    (self.terminal_height as usize).saturating_sub(12).max(5),
                );
                if let Some(song) = self.online_search_results.get(i).cloned() {
                    self.start_download_song(song);
                }
            } else if !manual {
                self.audio_player.lock().unwrap().stop();
                self.update_status(self.t().play_complete);
            }
            return;
        }

        let mode = self.audio_player.lock().unwrap().get_play_mode();
        let next_index = {
            let playlist = self.playlist.lock().unwrap();
            playlist.next_index(mode, manual)
        };

        if let Some(index) = next_index {
            self.play_song_by_index(index);
        } else if !manual {
            // 自动播放完成，停止播放
            self.audio_player.lock().unwrap().stop();
            self.update_status(self.t().play_complete);
        }
    }

    /// 播放下一曲（用户手动切换）
    fn play_next(&mut self) {
        self.play_next_with_flag(true);
    }

    /// 播放上一曲
    fn play_prev(&mut self) {
        // 在线搜索结果视图（网络/聚合/歌单歌曲）统一按在线结果切换上一首
        if self.search_mode
            && self.online_search_mode
            && !self.online_search_results.is_empty()
            && !self.online_downloading
        {
            let mode = self.audio_player.lock().unwrap().get_play_mode();
            let len = self.online_search_results.len();
            let cur = self
                .current_online_playing_index()
                .unwrap_or(self.online_selected_index)
                .min(len.saturating_sub(1));
            let prev_idx = match mode {
                PlayMode::Random => {
                    // 随机播放：随机选择一首（排除当前播放项）
                    use rand::Rng;
                    let mut rng = rand::thread_rng();
                    if len <= 1 {
                        cur
                    } else {
                        let mut prev = rng.gen_range(0..len - 1);
                        if prev >= cur {
                            prev += 1;
                        }
                        prev
                    }
                }
                _ => cur.saturating_sub(1),
            };

            self.online_selected_index = prev_idx;
            Self::adjust_scroll_offset(
                self.online_selected_index,
                &mut self.online_scroll_offset,
                (self.terminal_height as usize).saturating_sub(12).max(5),
            );
            if let Some(song) = self.online_search_results.get(prev_idx).cloned() {
                // 手动上一曲属于人工切换，重置自动切歌节流窗口
                self.online_auto_skip_times.clear();
                self.start_download_song(song);
            }
            return;
        }

        let mode = self.audio_player.lock().unwrap().get_play_mode();
        let prev_index = {
            let playlist = self.playlist.lock().unwrap();
            playlist.prev_index(mode)
        };

        if let Some(index) = prev_index {
            self.play_song_by_index(index);
        }
    }

    /// 打开文件夹
    fn open_folder(&mut self) {
        use crate::playlist::{open_folder_dialog, FolderDialogResult};

        let dialog_result = open_folder_dialog();

        match dialog_result {
            FolderDialogResult::Selected(path) => {
                let path_str = path.to_string_lossy().to_string();
                self.load_directory(&path_str);
            }
            FolderDialogResult::Cancelled => {
                // 用户取消了图形对话框，不进行终端输入，直接返回播放界面
            }
            FolderDialogResult::NoDialogAvailable => {
                // 没有可用的图形对话框工具，Linux 下回退到终端输入
                #[cfg(target_os = "linux")]
                {
                    let path = self.terminal_input_path();
                    if let Some(path_str) = path {
                        self.load_directory(&path_str);
                    }
                }
                // Windows/macOS 下不会出现此情况
            }
        }
    }

    /// 在终端内交互式输入路径（临时退出 raw mode）
    #[cfg(target_os = "linux")]
    fn terminal_input_path(&mut self) -> Option<String> {
        use std::io::{self, Write};

        // 临时恢复终端
        let _ = execute!(io::stdout(), DisableMouseCapture, terminal::LeaveAlternateScreen, cursor::Show);
        let _ = terminal::disable_raw_mode();

        // 提示用户输入
        print!("\n{}", self.t().input_dir_path);
        let _ = io::stdout().flush();

        // 读取用户输入
        let mut input = String::new();
        let result = io::stdin().read_line(&mut input);

        // 重新进入 raw mode
        let _ = terminal::enable_raw_mode();
        let _ = execute!(io::stdout(), terminal::EnterAlternateScreen, cursor::Hide, EnableMouseCapture);

        if result.is_ok() {
            let path = input.trim().to_string();
            if !path.is_empty() {
                let path_buf = std::path::PathBuf::from(&path);
                if path_buf.is_dir() {
                    return Some(path);
                }
                // 路径无效，但不提示（会在 load_directory 中报错）
                return Some(path);
            }
        }

        None
    }

    /// 加载指定目录的歌曲
    fn load_directory(&mut self, dir_path: &str) {
        use crate::playlist::scan_music_directory;

        let path = std::path::PathBuf::from(dir_path);
        match scan_music_directory(&path) {
            Ok(new_playlist) => {
                // 添加到音乐目录（如果已存在则移到末尾）
                let path_str = dir_path.to_string();
                if let Some(pos) = self.dir_history.iter().position(|p| *p == path_str) {
                    self.dir_history.remove(pos);
                }
                self.dir_history.push(path_str);

                let song_count = new_playlist.len();
                *self.playlist.lock().unwrap() = new_playlist;
                self.selected_index = 0;
                self.scroll_offset = 0;

                // 自动播放第一首歌曲
                if song_count > 0 {
                    self.play_song_by_index(0);
                }
            }
            Err(e) => {
                self.update_status(&format!(
                    "{}{}",
                    self.t().load_failed,
                    e
                ));
            }
        }
    }

    /// 加载目录并播放指定路径的歌曲
    fn load_directory_and_play(&mut self, dir_path: &str, song_path: &str) {
        use crate::playlist::scan_music_directory;

        let path = std::path::PathBuf::from(dir_path);
        match scan_music_directory(&path) {
            Ok(new_playlist) => {
                // 添加到音乐目录（如果已存在则移到末尾）
                let path_str = dir_path.to_string();
                if let Some(pos) = self.dir_history.iter().position(|p| *p == path_str) {
                    self.dir_history.remove(pos);
                }
                self.dir_history.push(path_str);

                // 在新播放列表中查找目标歌曲的索引
                let target_idx = new_playlist.files.iter().position(|s| s.path.to_string_lossy() == song_path);

                *self.playlist.lock().unwrap() = new_playlist;
                self.scroll_offset = 0;

                if let Some(idx) = target_idx {
                    self.selected_index = idx;
                    self.play_song_by_index(idx);
                } else {
                    self.selected_index = 0;
                    if !self.playlist.lock().unwrap().is_empty() {
                        self.play_song_by_index(0);
                    }
                }
            }
            Err(e) => {
                self.update_status(&format!(
                    "{}{}",
                    self.t().load_failed,
                    e
                ));
            }
        }
    }

    /// 写入运行日志
    fn append_runtime_log(&self, message: &str) {
        let timestamp = Local::now().format("%H:%M:%S%.3f");
        let line = format!("[{}] {}\n", timestamp, message);
        let _ = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(crate::config::get_daily_log_path())
            .and_then(|mut f| std::io::Write::write_all(&mut f, line.as_bytes()));
    }

    /// 更新状态消息
    pub fn update_status(&mut self, message: &str) {
        self.status_message = message.to_string();
    }

    /// 设置是否需要在启动后弹出目录选择对话框
    pub fn set_need_startup_dialog(&mut self, need: bool) {
        self.need_startup_dialog = need;
    }

    /// 设置选中的歌曲索引
    pub fn set_selected_index(&mut self, index: usize) {
        let playlist = self.playlist.lock().unwrap();
        if index < playlist.len() {
            self.selected_index = index;
        }
    }

    /// 设置收藏列表（从配置加载）
    pub fn set_favorites(&mut self, favorites: Vec<String>) {
        self.favorites = favorites;
    }

    /// 获取收藏列表（保存到配置）
    pub fn get_favorites(&self) -> Vec<String> {
        self.favorites.clone()
    }

    /// 获取 should_quit 标志的 Arc 引用（用于注册 Ctrl+C 信号处理器）
    pub fn get_should_quit(&self) -> Arc<Mutex<bool>> {
        self.should_quit.clone()
    }

    /// 设置音乐目录（从配置加载）
    pub fn set_dir_history(&mut self, dir_history: Vec<String>) {
        self.dir_history = dir_history;
    }

    /// 获取音乐目录（保存到配置）
    pub fn get_dir_history(&self) -> Vec<String> {
        self.dir_history.clone()
    }

    /// 从配置字符串设置主题
    pub fn set_theme_by_name(&mut self, theme_name: &str) {
        self.theme = UiTheme::from_config_key(theme_name);
        self.theme_colors = self.theme.colors();
    }

    /// 获取当前主题配置键
    pub fn get_theme_key(&self) -> &'static str {
        self.theme.config_key()
    }

    /// 从配置字符串设置语言
    pub fn set_language_by_name(&mut self, language: &str) {
        self.language = UiLanguage::from_config_key(language);
        self.cached_lyrics_title = None;
    }

    /// 设置 API Key（从配置加载）
    pub fn set_api_key(&mut self, key: String) {
        self.api_key = key.trim().to_string();
    }

    /// 设置 API 接口地址（从配置加载）
    pub fn set_api_base_url(&mut self, url: String) {
        let url = url.trim().to_string();
        self.api_base_url = if url.is_empty() {
            "https://api.deepseek.com/".to_string()
        } else if url.ends_with('/') {
            url
        } else {
            format!("{}/", url)
        };
    }

    /// 设置 API 模型名称（从配置加载）
    pub fn set_api_model(&mut self, model: String) {
        let model = model.trim().to_string();
        self.api_model = if model.is_empty() {
            "deepseek-v4-flash".to_string()
        } else {
            model
        };
    }

    /// 设置 GitHub Token（空字符串或默认 Token 均视为使用内置默认值，不写入配置文件）
    pub fn set_github_token(&mut self, token: String) {
        let trimmed = token.trim().to_string();
        // 如果配置文件中存的是内置默认 Token，视为空（使用内置默认值，不回写配置）
        if trimmed == DEFAULT_GITHUB_TOKEN {
            self.github_token = String::new();
        } else {
            self.github_token = trimmed;
        }
    }

    /// 获取 GitHub Token（保存到配置，默认 Token 不写入配置文件）
    pub fn get_github_token(&self) -> String {
        // 如果是默认 Token 则返回空字符串，避免将内置默认值写入配置文件
        if self.github_token.trim() == DEFAULT_GITHUB_TOKEN {
            String::new()
        } else {
            self.github_token.clone()
        }
    }

    /// 获取实际使用的 GitHub Token（空时回退到默认 Token）
    fn resolved_github_token(&self) -> String {
        let trimmed = self.github_token.trim();
        if trimmed.is_empty() {
            DEFAULT_GITHUB_TOKEN.to_string()
        } else {
            trimmed.to_string()
        }
    }

    /// 获取 API Key（保存到配置）
    pub fn get_api_key(&self) -> String {
        self.api_key.clone()
    }

    /// 获取 API 接口地址（保存到配置）
    pub fn get_api_base_url(&self) -> String {
        self.api_base_url.clone()
    }

    /// 获取 API 模型名称（保存到配置）
    pub fn get_api_model(&self) -> String {
        self.api_model.clone()
    }

    /// 获取当前语言配置键
    pub fn get_language_key(&self) -> &'static str {
        self.language.config_key()
    }

    /// 立即保存配置到文件
    pub fn save_config_now(&self) {
        use crate::config::Config;
        let player = self.audio_player.lock().unwrap();
        let pl = self.playlist.lock().unwrap();

        let new_config = Config {
            music_directory: pl.directory.clone(),
            play_mode: Config::play_mode_to_string(player.get_play_mode()),
            current_index: pl.current_index,
            volume: player.get_volume(),
            favorites: self.favorites.clone(),
            dir_history: self.dir_history.clone(),
            theme: self.get_theme_key().to_string(),
            language: self.get_language_key().to_string(),
            api_key: self.api_key.clone(),
            api_base_url: self.api_base_url.clone(),
            api_model: self.api_model.clone(),
            github_token: self.get_github_token(),
        };

        new_config.save();
    }

    /// 运行事件循环
    pub fn run(&mut self) -> io::Result<()> {
        // 初始化终端（使用 RAII 保护）
        let _guard = Self::init_terminal()?;

        // Linux 下首次进入 alternate screen 时，终端可能尚未正确报告窗口大小，
        // 导致界面缩在左上角。短暂等待后重新获取尺寸并清屏可解决此问题。
        {
            std::thread::sleep(std::time::Duration::from_millis(50));
            if let Ok((width, height)) = terminal::size() {
                self.terminal_width = width;
                self.terminal_height = height;
            }
            let _ = execute!(io::stdout(), terminal::Clear(ClearType::All));
        }

        // 初始绘制
        self.draw()?;

        // 上次进度更新的时间
        let mut last_progress_update = std::time::Instant::now();
        let progress_update_interval = Duration::from_millis(50); // 50ms 更新一次，更流畅

        // 上次配置保存时间
        let mut last_config_save = std::time::Instant::now();
        let config_save_interval = Duration::from_secs(30); // 每 30 秒自动保存配置

        // 主循环
        while !*self.should_quit.lock().unwrap() {
            // 检查播放完成和状态
            let (should_play_next, current_state) = {
                let audio_player = self.audio_player.lock().unwrap();
                let state = audio_player.get_state();
                let finished = state == PlayState::Playing && audio_player.is_finished();
                (finished, state)
            };

            // 如果播放完成，自动播放下一首（非手动）
            if should_play_next {
                self.play_next_with_flag(false);
                self.draw()?;
            }

            // 检查网络搜索结果
            if self.online_searching {
                self.check_online_search_result();
            }

            // 检查下载结果
            if self.online_downloading {
                self.check_download_result();
            }

            // 检查聚合搜索歌词下载结果
            if self.juhe_lyrics_loading {
                self.check_juhe_lyrics_result();
            }

            // 检查是否需要更新进度条和歌词（播放中持续更新波形）
            let now = std::time::Instant::now();
            if (current_state == PlayState::Playing
                || self.song_info_loading
                || self.github_discussion_loading
                || self.comments_loading)
                && now.duration_since(last_progress_update) >= progress_update_interval
            {
                self.draw()?;
                last_progress_update = now;
            }

            // 等待事件（超时50ms，与更新频率匹配）
            if event::poll(Duration::from_millis(50))? {
                match event::read()? {
                    Event::Key(key_event) => {
                        // 只处理按键按下事件，忽略释放事件
                        if key_event.kind == KeyEventKind::Press {
                            // 处理修饰键：Ctrl 保留为快捷键，其余（含 Shift）按普通输入处理
                            if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                                // Ctrl+C 优雅退出
                                if key_event.code == KeyCode::Char('c') {
                                    *self.should_quit.lock().unwrap() = true;
                                }
                            } else {
                                self.handle_key_event(key_event.code)?;
                                self.draw()?;
                            }
                        }
                    }
                    Event::Mouse(mouse_event) => {
                        self.handle_mouse_event(mouse_event)?;
                        self.draw()?;
                    }
                    Event::Resize(_, _) => {
                        // 窗口大小改变时立即重绘，无论播放状态如何
                        self.draw()?;
                    }
                    _ => {}
                }
            }

            // 定期自动保存配置（每 30 秒）
            if now.duration_since(last_config_save) >= config_save_interval {
                self.save_config_now();
                last_config_save = now;
            }
        }

        // 退出前保存配置
        self.save_config_now();

        // TerminalGuard 会在函数退出时自动恢复终端

        Ok(())
    }
}

/// 获取字符在终端中的实际显示宽度
fn term_char_width(ch: char) -> usize {
    unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0)
}

/// 计算字符串在终端中的实际显示宽度
fn term_display_width(s: &str) -> usize {
    s.chars().map(term_char_width).sum()
}

/// 截断字符串到指定显示宽度（不加省略号，用于歌词区域）
fn truncate_to_width(text: &str, max_width: usize) -> String {
    // 全局兜底：避免任意来源文本中的控制字符（如 CR/ESC）破坏终端光标定位
    let sanitized: String = text
        .chars()
        .map(|ch| match ch {
            '\n' | '\r' | '\t' => ' ',
            c if c.is_control() => ' ',
            c => c,
        })
        .collect();

    if term_display_width(sanitized.as_str()) <= max_width {
        return sanitized;
    }

    let mut result = String::new();
    let mut current_width = 0;

    for ch in sanitized.chars() {
        let ch_width = term_char_width(ch);
        if current_width + ch_width > max_width {
            break;
        }
        result.push(ch);
        current_width += ch_width;
    }

    result
}

/// 取字符串在给定显示宽度下的“尾部可见部分”（用于超长输入框编辑）
fn tail_to_width(text: &str, max_width: usize) -> String {
    if max_width == 0 {
        return String::new();
    }

    // 与 truncate_to_width 保持一致的控制字符清理策略
    let sanitized: String = text
        .chars()
        .map(|ch| match ch {
            '\n' | '\r' | '\t' => ' ',
            c if c.is_control() => ' ',
            c => c,
        })
        .collect();

    if unicode_width::UnicodeWidthStr::width(sanitized.as_str()) <= max_width {
        return sanitized;
    }

    let mut reversed: Vec<char> = Vec::new();
    let mut current_width = 0;

    for ch in sanitized.chars().rev() {
        let ch_width = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0);
        if current_width + ch_width > max_width {
            break;
        }
        reversed.push(ch);
        current_width += ch_width;
    }

    reversed.into_iter().rev().collect()
}

/// 按显示宽度自动换行，保留原始换行
fn wrap_text_to_width(text: &str, max_width: usize) -> Vec<String> {
    if max_width == 0 {
        return vec![String::new()];
    }

    let mut out = Vec::new();

    // 保留换行语义，同时过滤会影响终端布局的控制字符
    let normalized = text.replace('\r', "\n");

    for raw_line in normalized.lines() {
        if raw_line.is_empty() {
            out.push(String::new());
            continue;
        }

        let mut buf = String::new();
        let mut width = 0;
        for ch in raw_line.chars() {
            let ch = if ch.is_control() { ' ' } else { ch };
            let ch_width = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0);
            if width + ch_width > max_width && !buf.is_empty() {
                out.push(buf);
                buf = String::new();
                width = 0;
            }
            buf.push(ch);
            width += ch_width;
        }

        if !buf.is_empty() {
            out.push(buf);
        }
    }

    if out.is_empty() {
        out.push(String::new());
    }

    out
}
