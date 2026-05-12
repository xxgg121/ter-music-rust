// 用户界面模块

#[path = "ui/input.rs"]
mod input;
#[path = "ui/layout.rs"]
mod interaction_layout;
#[path = "ui/mouse.rs"]
mod mouse;
#[path = "ui/render.rs"]
mod render;
#[path = "ui/terminal.rs"]
mod terminal_guard;
mod theme;
#[path = "ui/format.rs"]
mod ui_format;
mod view_model;

use std::collections::{HashMap, VecDeque};
use std::io;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use chrono::Local;
use crossterm::{
    cursor,
    event::{
        self, Event, KeyCode, KeyEventKind, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
    },
    execute, style,
    terminal::{self, ClearType},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color as TuiColor, Modifier, Style as TuiStyle},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use serde::{Deserialize, Serialize};

use crate::audio::AudioPlayer;
use crate::defs::{PlayMode, PlayState, Playlist};
use crate::desktop_lyrics::{DesktopLyricsHandle, DesktopLyricsPosition};
use crate::langs::{LangTexts, UiLanguage};
use crate::lyrics::{Lyrics, LyricsDownloadResult};
use crate::search::{
    DownloadProgress, GitHubDiscussionResult, OnlinePlaylist, OnlineSong, PlaylistSearchResult,
    PlaylistSongsResult, SearchDownloadResult, SongCommentItem, SongCommentsResult,
};

use interaction_layout::{LyricsAreaLayout, PlaylistLayout, ProgressBarLayout, VolumeBarLayout};
use terminal_guard::TerminalGuard;
use theme::{ThemeColors, UiTheme};
use ui_format::{
    format_duration_ms, format_progress, slice_at_display_offset, term_char_width,
    term_display_width, truncate_to_width, wrap_text_to_width,
};
use view_model::{
    CommentsListView, ControlsView, HighlightedTextRow, LyricsPanelView, PlaylistPanelView,
    PlaylistRowView, SearchResultsView, SelectableListView, SelectableTextRow, TextPanelView,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PlayHistoryRecord {
    name: String,
    path: String,
    last_played: String,
    play_count: u64,
}

#[derive(Debug, Clone)]
struct RecommendationItem {
    name: String,
    search_query: String,
    start_col: usize,
    end_col: usize,
}

#[derive(Debug, Clone)]
struct AiRecommendPresetItem {
    query: String,
    start_col: usize,
    end_col: usize,
}

const DEFAULT_GITHUB_TOKEN: &str =
    "github_xxxxxx";
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct AiRecommendCacheKey {
    language: UiLanguage,
    query: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MainFocus {
    Playlist,
    Recommendation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SongInfoKind {
    SongInfo,
    CommentSummary,
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
    #[allow(dead_code)]
    wave_frame: u32,
    /// 缓存的歌词标题（用于避免闪烁）
    cached_lyrics_title: Option<String>,
    /// 缓存的窗口宽度（用于检测窗口大小变化）
    #[allow(dead_code)]
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
    /// 评论面板内部区域起始行（用于鼠标点击定位）
    comment_panel_inner_y: Option<u16>,
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
    /// 右侧 AI 信息面板类型
    song_info_kind: SongInfoKind,
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
    #[allow(dead_code)]
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
    /// 桌面歌词句柄
    desktop_lyrics: DesktopLyricsHandle,
    /// 今日推荐歌曲开关
    recommand: bool,
    /// 推荐歌曲列表
    recommendations: Vec<String>,
    /// 推荐歌曲点击区域
    recommendation_items: Vec<RecommendationItem>,
    /// 推荐歌曲当前选中项
    recommendation_selected_index: Option<usize>,
    /// 普通播放界面的焦点
    main_focus: MainFocus,
    /// 推荐歌曲生成中
    recommendations_loading: bool,
    /// 推荐歌曲流式返回临时内容
    recommendations_content: String,
    /// 推荐歌曲 AI 返回接收器
    recommendations_rx: Option<std::sync::mpsc::Receiver<crate::search::SongInfoChunk>>,
    /// 推荐歌曲搜索接收器（点击推荐后用聚合搜索获取下载项）
    recommendation_search_rx: Option<std::sync::mpsc::Receiver<SearchDownloadResult>>,
    /// 是否正在下载推荐歌曲
    recommendation_downloading: bool,
    /// 推荐歌曲下载进度百分比（0-100）
    recommendation_download_percent: u8,
    /// 正在下载的推荐歌曲名称
    recommendation_downloading_name: Option<String>,
    /// 推荐歌曲水平滚动偏移
    recommendation_scroll_offset: usize,
    /// 是否处于 AI 自然语言推荐输入模式
    ai_recommend_input_mode: bool,
    /// AI 自然语言推荐输入内容
    ai_recommend_input_value: String,
    /// AI 自然语言推荐当前请求需求
    ai_recommend_current_query: Option<String>,
    /// AI 自然语言推荐预设点击区域
    ai_recommend_preset_items: Vec<AiRecommendPresetItem>,
    /// AI 自然语言推荐缓存
    ai_recommend_cache: HashMap<AiRecommendCacheKey, Vec<String>>,
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
            comment_panel_inner_y: None,
            song_info_mode: false,
            song_info_file_path: None,
            song_info_content: String::new(),
            song_info_rx: None,
            song_info_scroll_offset: 0,
            song_info_loading: false,
            song_info_kind: SongInfoKind::SongInfo,
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
            desktop_lyrics: DesktopLyricsHandle::new(),
            recommand: false,
            recommendations: Vec::new(),
            recommendation_items: Vec::new(),
            recommendation_selected_index: None,
            main_focus: MainFocus::Playlist,
            recommendations_loading: false,
            recommendations_content: String::new(),
            recommendations_rx: None,
            recommendation_search_rx: None,
            recommendation_downloading: false,
            recommendation_download_percent: 0,
            recommendation_downloading_name: None,
            recommendation_scroll_offset: 0,
            ai_recommend_input_mode: false,
            ai_recommend_input_value: String::new(),
            ai_recommend_current_query: None,
            ai_recommend_preset_items: Vec::new(),
            ai_recommend_cache: HashMap::new(),
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
    fn clamp_selected_and_scroll(
        selected: &mut usize,
        scroll_offset: &mut usize,
        total: usize,
        visible_count: usize,
    ) {
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

    fn url_at_display_col(line: &str, col: usize) -> Option<String> {
        let start_byte = match (line.find("https://"), line.find("http://")) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }?;
        let tail = &line[start_byte..];
        let mut end_len = tail.find(char::is_whitespace).unwrap_or_else(|| tail.len());
        let trim_chars = ['.', ',', ';', ':', ')', ']', '}', '>', '。', '，', '；'];
        while end_len > 0 {
            let candidate = &tail[..end_len];
            let Some(ch) = candidate.chars().next_back() else {
                break;
            };
            if trim_chars.contains(&ch) {
                end_len -= ch.len_utf8();
            } else {
                break;
            }
        }
        if end_len == 0 {
            return None;
        }

        let url = &tail[..end_len];
        let start_col = term_display_width(&line[..start_byte]);
        let end_col = start_col + term_display_width(url);
        if col >= start_col && col < end_col {
            Some(url.to_string())
        } else {
            None
        }
    }

    fn display_url_line(url: &str, width: u16) -> String {
        let max_width = width.saturating_sub(1) as usize;
        let mut line = truncate_to_width(url.trim_end(), max_width);
        let display_width = term_display_width(&line);
        if display_width < max_width {
            line.push_str(&" ".repeat(max_width - display_width));
        }
        line
    }

    fn split_at_display_width(text: &str, max_width: usize) -> (String, String) {
        if term_display_width(text) <= max_width {
            return (text.to_string(), String::new());
        }

        let mut head = String::new();
        let mut width = 0;
        let mut split_byte = text.len();
        for (idx, ch) in text.char_indices() {
            let ch_width = term_char_width(ch);
            if width + ch_width > max_width {
                split_byte = idx;
                break;
            }
            head.push(ch);
            width += ch_width;
        }

        (head, text[split_byte..].to_string())
    }

    fn restore_mouse_capture() {
        let _ = terminal::enable_raw_mode();
        let _ = execute!(
            io::stdout(),
            cursor::Hide,
            crossterm::event::EnableMouseCapture
        );
    }

    fn open_url(&mut self, url: &str) {
        let result = {
            #[cfg(target_os = "windows")]
            {
                std::process::Command::new("rundll32")
                    .args(["url.dll,FileProtocolHandler", url])
                    .stdin(std::process::Stdio::null())
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .spawn()
            }
            #[cfg(target_os = "macos")]
            {
                std::process::Command::new("open")
                    .arg(url)
                    .stdin(std::process::Stdio::null())
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .spawn()
            }
            #[cfg(all(unix, not(target_os = "macos")))]
            {
                std::process::Command::new("xdg-open")
                    .arg(url)
                    .stdin(std::process::Stdio::null())
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .spawn()
            }
        };

        if result.is_err() {
            self.update_status(url);
        }

        Self::restore_mouse_capture();
    }

    fn clear_online_download_state(&mut self) {
        self.online_downloading = false;
        self.online_download_rx = None;
        self.online_download_percent = 0;
        self.online_downloading_index = None;
    }

    fn history_path() -> std::path::PathBuf {
        crate::config::get_app_config_dir().join("history.json")
    }

    fn load_play_history() -> Vec<PlayHistoryRecord> {
        let path = Self::history_path();
        std::fs::read_to_string(path)
            .ok()
            .and_then(|text| serde_json::from_str::<Vec<PlayHistoryRecord>>(&text).ok())
            .unwrap_or_default()
    }

    fn save_play_history(records: &[PlayHistoryRecord]) {
        let path = Self::history_path();
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(records) {
            let _ = std::fs::write(path, json);
        }
    }

    fn record_play_history(&self, name: &str, path: &std::path::Path) {
        let path_text = path.to_string_lossy().to_string();
        let mut records = Self::load_play_history();
        if let Some(record) = records.iter_mut().find(|record| record.path == path_text) {
            record.name = name.to_string();
            record.last_played = Local::now().to_rfc3339();
            record.play_count = record.play_count.saturating_add(1);
        } else {
            records.push(PlayHistoryRecord {
                name: name.to_string(),
                path: path_text,
                last_played: Local::now().to_rfc3339(),
                play_count: 1,
            });
        }
        Self::save_play_history(&records);
    }

    fn recent_history_for_recommendation() -> Vec<PlayHistoryRecord> {
        let mut records = Self::load_play_history();
        records.sort_by(|a, b| {
            b.play_count
                .cmp(&a.play_count)
                .then_with(|| b.last_played.cmp(&a.last_played))
        });
        records.truncate(30);
        records
    }

    fn build_recommendation_prompt(&mut self, history: &[PlayHistoryRecord]) -> String {
        let history_text = history
            .iter()
            .map(|record| {
                format!(
                    "{} | count={} | last={}",
                    record.name, record.play_count, record.last_played
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            "{}{}",
            self.t().recommendation_prompt_template,
            history_text
        )
    }

    fn start_recommendations_if_enabled(&mut self) {
        if !self.recommand || self.recommendations_loading || self.recommendations_rx.is_some() {
            return;
        }
        let history = Self::recent_history_for_recommendation();
        if history.is_empty() {
            return;
        }
        let prompt = Self::build_recommendation_prompt(self, &history);
        let config = crate::search::AiQueryConfig {
            api_base_url: self.api_base_url.clone(),
            api_key: self.resolved_api_key().unwrap_or_default(),
            api_model: self.api_model.clone(),
        };
        self.recommendations.clear();
        self.recommendations_content.clear();
        self.recommendations_loading = true;
        self.recommendations_rx = Some(crate::search::fetch_song_info_streaming(prompt, config));
        self.main_focus = MainFocus::Recommendation;
    }

    fn parse_recommendations(text: &str) -> Vec<String> {
        let mut out = Vec::new();
        for line in text.lines() {
            let name = line
                .trim()
                .trim_start_matches(|ch: char| {
                    ch.is_ascii_digit() || ch == '.' || ch == '-' || ch == '、' || ch == '•' || ch == '*'
                })
                .trim()
                .trim_matches('"')
                .trim_matches('“')
                .trim_matches('”')
                .to_string();
            if !name.is_empty() && !out.iter().any(|existing| existing == &name) {
                out.push(name);
            }
            if out.len() >= 10 {
                break;
            }
        }
        out
    }

    fn recommendation_search_query(text: &str) -> String {
        let cleaned = Self::sanitize_single_line_text(text).trim().to_string();
        let separators = [" - ", " — ", " – ", " | ", " ｜ ", " / ", "：", ":"];

        for separator in separators {
            if let Some(index) = cleaned.find(separator) {
                let title = cleaned[..index].trim();
                let artist = cleaned[index + separator.len()..].trim();
                if !title.is_empty() && !artist.is_empty() {
                    return format!("{} {}", title, artist);
                }
            }
        }

        cleaned
    }

    fn check_recommendation_result(&mut self) {
        if let Some(ref rx) = self.recommendations_rx {
            loop {
                match rx.try_recv() {
                    Ok(chunk) => {
                        if chunk.error.is_some() {
                            self.recommendations_loading = false;
                            self.recommendations_rx = None;
                            break;
                        }
                        self.recommendations_content.push_str(&chunk.delta);
                        if chunk.done {
                            self.recommendations_loading = false;
                            self.recommendations_rx = None;
                            self.recommendations =
                                Self::parse_recommendations(&self.recommendations_content);
                            self.recommendation_selected_index = None;
                            self.main_focus = MainFocus::Recommendation;
                            if !self.recommendations_content.trim().is_empty() {
                                let key = AiRecommendCacheKey {
                                    language: self.language,
                                    query: self
                                        .ai_recommend_current_query
                                        .clone()
                                        .unwrap_or_default(),
                                };
                                self.ai_recommend_cache
                                    .insert(key, self.recommendations.clone());
                            }
                            self.ai_recommend_current_query = None;
                            break;
                        }
                    }
                    Err(std::sync::mpsc::TryRecvError::Empty) => break,
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                        self.recommendations_loading = false;
                        self.recommendations_rx = None;
                        break;
                    }
                }
            }
        }

        if let Some(ref rx) = self.recommendation_search_rx {
            match rx.try_recv() {
                Ok(result) => {
                    self.recommendation_search_rx = None;
                    if let Some(song) = result.songs.into_iter().next() {
                        self.online_auto_skip_times.clear();
                        self.start_download_song(song);
                    }
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {}
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    self.recommendation_search_rx = None;
                }
            }
        }
    }

    fn start_recommendation_download(&mut self, display_name: &str, search_query: &str) {
        self.recommendation_downloading = true;
        self.recommendation_download_percent = 0;
        self.recommendation_downloading_name = Some(display_name.to_string());
        self.recommendation_search_rx = Some(crate::search::search_juhe_background(
            search_query.to_string(),
            1,
        ));
    }

    fn has_selectable_recommendations(&self) -> bool {
        self.recommand && !self.recommendations_loading && !self.recommendations.is_empty()
    }

    fn move_recommendation_selection(&mut self, delta: isize) {
        if !self.has_selectable_recommendations() {
            return;
        }
        self.main_focus = MainFocus::Recommendation;
        let len = self.recommendations.len();
        let Some(current) = self.recommendation_selected_index else {
            self.recommendation_selected_index = Some(0);
            self.ensure_selected_recommendation_visible();
            return;
        };
        if delta < 0 {
            self.recommendation_selected_index = Some(
                current
                    .saturating_sub(delta.unsigned_abs())
                    .min(len.saturating_sub(1)),
            );
        } else {
            self.recommendation_selected_index = Some(
                (current + delta as usize).min(len.saturating_sub(1)),
            );
        }
        self.ensure_selected_recommendation_visible();
    }

    fn ensure_selected_recommendation_visible(&mut self) {
        if !self.has_selectable_recommendations() {
            return;
        }
        let Some(selected_index) = self.recommendation_selected_index else {
            self.recommendation_scroll_offset = 0;
            return;
        };

        if selected_index == 0 {
            self.recommendation_scroll_offset = 0;
            return;
        }

        if let Some(item) = self.recommendation_items.get(selected_index) {
            let visible_width = self.terminal_width.saturating_sub(2) as usize;
            if item.start_col <= visible_width / 2 {
                self.recommendation_scroll_offset = 0;
                return;
            }
            if item.start_col < self.recommendation_scroll_offset {
                self.recommendation_scroll_offset = item.start_col.saturating_sub(1);
            } else if item.end_col > self.recommendation_scroll_offset + visible_width {
                self.recommendation_scroll_offset = item
                    .end_col
                    .saturating_sub(visible_width)
                    .saturating_sub(1);
            }
        }
    }

    fn activate_selected_recommendation(&mut self) {
        if !self.has_selectable_recommendations() {
            return;
        }
        if self.recommendation_selected_index.is_none() {
            self.recommendation_selected_index = Some(0);
        };
        let Some(selected_index) = self.recommendation_selected_index else {
            return;
        };
        let selected = self.recommendations.get(selected_index).cloned();
        if let Some(display_name) = selected {
            let search_query = Self::recommendation_search_query(&display_name);
            if !self.recommendation_downloading
                || self.recommendation_downloading_name.as_ref() != Some(&display_name)
            {
                self.start_recommendation_download(&display_name, &search_query);
            }
        }
    }

    fn open_ai_recommend_input_mode(&mut self) {
        self.recommand = true;
        self.ai_recommend_input_mode = true;
        self.ai_recommend_input_value.clear();
        self.ai_recommend_current_query = None;
        self.cached_lyrics_title = None;
    }

    fn start_ai_recommend_query_with(&mut self, query: String) {
        let query = query.trim().to_string();
        if query.is_empty() {
            return;
        }
        let cache_key = AiRecommendCacheKey {
            language: self.language,
            query: query.clone(),
        };
        if let Some(cached) = self.ai_recommend_cache.get(&cache_key).cloned() {
            self.recommand = true;
            self.recommendations = cached;
            self.recommendation_items.clear();
            self.recommendation_selected_index = None;
            self.recommendations_content.clear();
            self.recommendations_loading = false;
            self.recommendation_scroll_offset = 0;
            self.ai_recommend_input_mode = false;
            self.ai_recommend_input_value.clear();
            self.ai_recommend_current_query = None;
            self.main_focus = MainFocus::Recommendation;
            return;
        }
        let prompt = self.t().ai_recommend_prompt_template.replace("{}", &query);
        let config = crate::search::AiQueryConfig {
            api_base_url: self.api_base_url.clone(),
            api_key: self.resolved_api_key().unwrap_or_default(),
            api_model: self.api_model.clone(),
        };
        self.recommand = true;
        self.recommendations.clear();
        self.recommendation_items.clear();
        self.recommendation_selected_index = None;
        self.recommendations_content.clear();
        self.recommendations_loading = true;
        self.recommendation_scroll_offset = 0;
        self.ai_recommend_current_query = Some(query.clone());
        self.recommendations_rx = Some(crate::search::fetch_song_info_streaming(prompt, config));
        self.ai_recommend_input_mode = false;
        self.ai_recommend_input_value.clear();
        self.main_focus = MainFocus::Recommendation;
        self.save_config_now();
    }

    fn start_ai_recommend_query(&mut self) {
        let query = self.ai_recommend_input_value.clone();
        self.start_ai_recommend_query_with(query);
    }

    fn song_info_lines(&self, width: u16) -> Vec<String> {
        let content = if self.song_info_loading && self.song_info_content.is_empty() {
            self.t().querying_song_info.to_string()
        } else if self.song_info_content.trim().is_empty() {
            self.t().no_query_result.to_string()
        } else {
            self.song_info_content.clone()
        };

        let mut lines = wrap_text_to_width(&content, width.saturating_sub(1) as usize);

        if !self.github_discussion_status.is_empty() {
            let (discussion_prefix, discussion_url) = if let Some(url_start) = self
                .github_discussion_status
                .find("http://")
                .or_else(|| self.github_discussion_status.find("https://"))
            {
                let prefix = self.github_discussion_status[..url_start].to_string();
                let url = self.github_discussion_status[url_start..]
                    .trim_end()
                    .to_string();
                (prefix, Some(url))
            } else {
                (self.github_discussion_status.clone(), None)
            };

            lines.push(String::new());
            if !discussion_prefix.trim().is_empty() {
                lines.extend(wrap_text_to_width(
                    &discussion_prefix,
                    width.saturating_sub(1) as usize,
                ));
            }
            if let Some(url) = discussion_url {
                lines.push(Self::display_url_line(&url, width));
            }
        }

        lines
    }

    fn current_body_layout(&self) -> Option<(Rect, Rect)> {
        if self.terminal_height < 10 || self.terminal_width < 40 {
            return None;
        }

        let area = Rect::new(0, 0, self.terminal_width, self.terminal_height);
        let root = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(8),
                Constraint::Length(5),
            ])
            .split(area);
        let body = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(root[1]);
        Some((body[0], body[1]))
    }

    fn open_clicked_right_panel_url(&mut self, col: usize, row: u16) -> bool {
        let Some((_, right_panel)) = self.current_body_layout() else {
            return false;
        };
        let inner = Self::inner_area(right_panel);
        let col = col as u16;
        if col < inner.x || col >= inner.right() || row < inner.y || row >= inner.bottom() {
            return false;
        }

        let line_idx = (row - inner.y) as usize;
        let visible_count = inner.height as usize;
        let lines = if self.help_mode {
            self.help_view(visible_count).lines
        } else if self.song_info_mode {
            self.song_info_view(inner.width, visible_count).lines
        } else {
            return false;
        };

        let Some(line) = lines.get(line_idx) else {
            return false;
        };
        let relative_col = (col - inner.x) as usize;
        if let Some(url) = Self::url_at_display_col(line, relative_col) {
            self.open_url(&url);
            true
        } else {
            false
        }
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
        self.song_info_kind = SongInfoKind::SongInfo;
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

    fn build_comment_summary_prompt(&self, song_name: &str) -> String {
        let mut comments = String::new();
        for (idx, comment) in self.current_comments.iter().take(30).enumerate() {
            comments.push_str(&format!(
                "{}. {}: {}\n",
                idx + 1,
                Self::sanitize_single_line_text(&comment.nickname),
                Self::sanitize_single_line_text(&comment.content)
            ));
            if let Some(reply) = &comment.reply {
                comments.push_str(&format!(
                    "   回复 {}: {}\n",
                    Self::sanitize_single_line_text(&reply.nickname),
                    Self::sanitize_single_line_text(&reply.content)
                ));
            }
        }

        self.t()
            .comment_summary_prompt_template
            .replacen("{}", song_name.trim(), 1)
            .replacen("{}", comments.trim(), 1)
    }

    fn start_comment_summary_mode(&mut self) {
        if !self.comments_mode {
            return;
        }

        self.song_info_mode = true;
        self.comments_mode = false;
        self.comments_detail_mode = false;
        self.help_mode = false;
        self.song_info_kind = SongInfoKind::CommentSummary;
        self.song_info_scroll_offset = 0;
        self.song_info_rx = None;
        self.song_info_loading = false;
        self.song_info_content.clear();
        self.song_info_name = self.comments_song_name.trim().to_string();
        self.song_info_file_path = self.comments_file_path.clone();
        self.github_discussion_rx = None;
        self.github_discussion_status.clear();
        self.github_discussion_loading = false;
        self.song_info_force_scroll = false;

        if self.current_comments.is_empty() {
            self.song_info_content = self.t().no_comments.to_string();
            return;
        }

        let prompt = self.build_comment_summary_prompt(&self.comments_song_name);
        let config = crate::search::AiQueryConfig {
            api_base_url: self.api_base_url.clone(),
            api_key: self.resolved_api_key().unwrap_or_default(),
            api_model: self.api_model.clone(),
        };
        self.song_info_loading = true;
        self.song_info_rx = Some(crate::search::fetch_song_info_streaming(prompt, config));
    }

    fn now_playing_prefix(&self) -> &'static str {
        self.t().now_playing_prefix
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    fn is_now_playing_message(&self, message: &str) -> bool {
        self.t()
            .now_playing_prefixes
            .iter()
            .any(|p| message.starts_with(p))
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
        if self.ai_recommend_input_mode {
            self.t().ai_recommend_input_hint.to_string()
        } else if self.search_mode {
            if self.online_search_mode {
                let search_label = if self.playlist_search_mode {
                    self.t().search_playlist.to_string()
                } else if self.juhe_search_mode {
                    self.t().search_juhe.to_string()
                } else {
                    self.t().search_online.to_string()
                };
                if self.terminal_width >= 80 {
                    format!("{}: {}", search_label, self.t().tip_online_wide.to_string())
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

        if !self.comments_loading && self.comments_rx.is_none() && self.current_comments.is_empty()
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
                            self.song_info_force_scroll = true;
                        }
                        if chunk.done {
                            self.song_info_loading = false;
                            self.song_info_rx = None;
                            self.song_info_force_scroll = true;
                            // 流式输出完成后，自动创建 GitHub Discussion
                            if self.song_info_kind == SongInfoKind::SongInfo {
                                self.start_github_discussion_for_song_info();
                            }
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
                        self.github_discussion_status =
                            format!("{} {}", self.t().discussion_created, url);
                    } else if let Some(err) = result.error {
                        self.github_discussion_status =
                            format!("{} {}", self.t().discussion_failed, err);
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

        self.github_discussion_rx = Some(crate::search::create_github_discussion_background(
            token, repo, name, content,
        ));
    }

    /// 绘制界面（Ratatui Frame 渲染入口）
    fn draw(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
        terminal.draw(|frame| self.render(frame))?;
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame<'_>) {
        let area = frame.area();
        self.terminal_width = area.width;
        self.terminal_height = area.height;
        self.playlist_layout = None;
        self.lyrics_area_layout = None;
        self.comments_row_map.clear();

        if area.height < 10 || area.width < 40 {
            let warning = Paragraph::new(self.t().app_title)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Ter Music Rust"),
                )
                .style(self.tui_style(self.theme_colors.info_text));
            frame.render_widget(warning, area);
            return;
        }

        let root = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(8),
                Constraint::Length(5),
            ])
            .split(area);

        self.render_header(frame, root[0]);

        let body = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(root[1]);
        self.render_left_panel(frame, body[0]);
        self.render_right_panel(frame, body[1]);
        self.render_controls(frame, root[2]);
        self.render_cursor(frame, body[0], body[1]);
    }

    fn render_header(&mut self, frame: &mut Frame<'_>, area: Rect) {
        self.recommendation_items.clear();
        self.ai_recommend_preset_items.clear();
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(self.tui_style(self.theme_colors.header_title))
            .title(Line::from(Span::styled(
                self.t().app_title,
                self.tui_style(self.theme_colors.header_title)
                    .add_modifier(Modifier::BOLD),
            )))
            .title_alignment(Alignment::Center);
        frame.render_widget(block, area);

        if self.ai_recommend_input_mode {
            let prompt = self.t().recommendation_title.trim_end_matches(':').trim_end_matches("：");
            let input_line = format!("{}> {}", prompt, self.ai_recommend_input_value);
            let input_style = self.tui_style(self.theme_colors.song_playing)
                .add_modifier(Modifier::BOLD);
            let max_width = area.width.saturating_sub(2) as usize;
            let mut text = input_line;
            if self.ai_recommend_input_value.is_empty() {
                let mut col = area.x as usize + 1 + term_display_width(&text);
                for preset in self.t().ai_recommend_presets {
                    let chip = format!(" [{}]", preset);
                    let chip_width = term_display_width(&chip);
                    let start_col = col + 2;
                    let end_col = start_col + term_display_width(preset);
                    text.push_str(&chip);
                    self.ai_recommend_preset_items.push(AiRecommendPresetItem {
                        query: (*preset).to_string(),
                        start_col,
                        end_col,
                    });
                    col += chip_width;
                }
            }
            let line = slice_at_display_offset(&text, 0, max_width);
            frame.render_widget(
                Paragraph::new(line).style(input_style),
                Rect::new(area.x + 1, area.y + 1, area.width.saturating_sub(2), 1),
            );
        } else if self.recommand {
            let mut text = if self.recommendations_loading {
                format!(
                    "{}{}",
                    self.t().recommendation_title,
                    self.t().recommendation_loading
                )
            } else if self.recommendations.is_empty() {
                format!(
                    "{}{}",
                    self.t().recommendation_title,
                    self.t().recommendation_no_data
                )
            } else {
                self.t().recommendation_title.to_string()
            };
            if let Some(selected_index) = self.recommendation_selected_index {
                if selected_index >= self.recommendations.len() {
                    self.recommendation_selected_index = None;
                }
            }
            let mut spans = Vec::new();
            spans.push(Span::styled(
                text.clone(),
                self.tui_style(self.theme_colors.info_text),
            ));
            let mut col = area.x as usize + 1 + term_display_width(&text);
            for (idx, name) in self.recommendations.iter().enumerate() {
                if !text.ends_with('：') {
                    text.push(' ');
                    spans.push(Span::raw(" "));
                    col += 1;
                }
                let selected = self.recommendation_selected_index == Some(idx);
                let display_name = if selected {
                    format!("[{}]", name)
                } else {
                    name.clone()
                };
                if selected {
                    text.push('[');
                    col += 1;
                }
                let start_col = col;
                text.push_str(name);
                let name_end_col = col + term_display_width(name);
                col = name_end_col;
                if selected {
                    text.push(']');
                    col += 1;
                }
                let mut span_style = self.tui_style(self.theme_colors.info_text);
                if selected {
                    span_style = self
                        .tui_style(self.theme_colors.song_playing)
                        .add_modifier(Modifier::BOLD);
                }
                spans.push(Span::styled(display_name, span_style));
                if self.recommendation_downloading
                    && self.recommendation_downloading_name.as_ref() == Some(name)
                {
                    let progress_text = format!(" [{}%]", self.recommendation_download_percent);
                    text.push_str(&progress_text);
                    spans.push(Span::styled(
                        progress_text.clone(),
                        self.tui_style(self.theme_colors.info_text),
                    ));
                    col += term_display_width(&progress_text);
                }
                self.recommendation_items.push(RecommendationItem {
                    name: name.clone(),
                    search_query: Self::recommendation_search_query(name),
                    start_col,
                    end_col: name_end_col,
                });
            }
            let max_width = area.width.saturating_sub(2) as usize;
            let full_width = term_display_width(&text);
            let max_offset = full_width.saturating_sub(max_width);
            self.recommendation_scroll_offset = self.recommendation_scroll_offset.min(max_offset);
            frame.render_widget(
                Paragraph::new(Line::from(spans)).scroll((0, self.recommendation_scroll_offset as u16)),
                Rect::new(area.x + 1, area.y + 1, area.width.saturating_sub(2), 1),
            );
        }
    }

    fn render_left_panel(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let inner = Self::inner_area(area);
        let visible_count = inner.height as usize;
        self.playlist_layout = Some(PlaylistLayout {
            start_row: inner.y,
            visible_count,
            left_width: area.width,
        });

        if self.dir_history_mode {
            self.render_dir_history(frame, area, visible_count);
        } else if self.favorites_mode {
            self.render_favorites(frame, area, visible_count);
        } else if self.search_mode {
            self.render_search_results(frame, area, visible_count);
        } else {
            self.render_playlist_panel(frame, area, visible_count);
        }
    }

    fn render_playlist_panel(&mut self, frame: &mut Frame<'_>, area: Rect, visible_count: usize) {
        let view = self.playlist_panel_view(visible_count);
        let items: Vec<ListItem> = if view.is_empty {
            vec![ListItem::new(self.t().playlist_no_dir_hint)
                .style(self.tui_style(self.theme_colors.info_text))]
        } else {
            view.rows
                .into_iter()
                .map(|row| {
                    let mut style = if row.playing {
                        self.tui_style(self.theme_colors.song_playing)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        self.tui_style(self.theme_colors.song_normal)
                    };
                    if row.selected {
                        style = style.bg(TuiColor::DarkGray);
                    }
                    ListItem::new(row.text).style(style)
                })
                .collect()
        };
        self.render_list(frame, area, &view.title, items);
    }

    fn playlist_panel_view(&mut self, visible_count: usize) -> PlaylistPanelView {
        let (files, current_index, directory) = {
            let playlist = self.playlist.lock().unwrap();
            (
                playlist.files.clone(),
                playlist.current_index,
                playlist.directory.clone(),
            )
        };
        let title = self
            .t()
            .fmt_playlist_title
            .replacen("{}", directory.as_deref().unwrap_or(""), 1)
            .replacen("{}", &files.len().to_string(), 1);
        Self::clamp_selected_and_scroll(
            &mut self.selected_index,
            &mut self.scroll_offset,
            files.len(),
            visible_count.max(1),
        );
        let rows = files
            .iter()
            .enumerate()
            .skip(self.scroll_offset)
            .take(visible_count)
            .map(|(idx, file)| {
                let selected = idx == self.selected_index;
                let playing = current_index == Some(idx);
                let favorite = self
                    .favorites
                    .iter()
                    .any(|p| p == &file.path.to_string_lossy());
                let prefix = if playing {
                    "▶"
                } else if selected {
                    ">"
                } else {
                    " "
                };
                let star = if favorite { "*" } else { " " };
                PlaylistRowView {
                    text: format!(
                        "{}{} {:02}. {} [{}]",
                        prefix,
                        star,
                        idx + 1,
                        file.name,
                        file.format_duration()
                    ),
                    selected,
                    playing,
                }
            })
            .collect();
        PlaylistPanelView {
            title,
            rows,
            is_empty: files.is_empty(),
        }
    }

    fn render_dir_history(&mut self, frame: &mut Frame<'_>, area: Rect, visible_count: usize) {
        let view = self.dir_history_view(visible_count);
        let items = if let Some(hint) = view.empty_hint {
            vec![ListItem::new(hint).style(self.tui_style(self.theme_colors.info_text))]
        } else {
            view.rows
                .into_iter()
                .map(|row| ListItem::new(row.text).style(self.selection_style(row.selected)))
                .collect()
        };
        self.render_list(frame, area, &view.title, items);
    }

    fn dir_history_view(&mut self, visible_count: usize) -> SelectableListView {
        Self::clamp_selected_and_scroll(
            &mut self.dir_history_selected_index,
            &mut self.dir_history_scroll_offset,
            self.dir_history.len(),
            visible_count.max(1),
        );
        let current_dir = self.playlist.lock().unwrap().directory.clone();
        let rows = self
            .dir_history
            .iter()
            .enumerate()
            .skip(self.dir_history_scroll_offset)
            .take(visible_count)
            .map(|(idx, dir)| {
                let selected = idx == self.dir_history_selected_index;
                let marker = if current_dir.as_ref() == Some(dir) {
                    ">>"
                } else {
                    "  "
                };
                SelectableTextRow {
                    text: format!("{} {}", marker, dir),
                    selected,
                }
            })
            .collect();
        let title = self
            .t()
            .fmt_dir_title
            .replace("{}", &self.dir_history.len().to_string());
        SelectableListView {
            title,
            rows,
            empty_hint: if self.dir_history.is_empty() {
                Some(self.t().dir_empty_hint)
            } else {
                None
            },
        }
    }

    fn render_favorites(&mut self, frame: &mut Frame<'_>, area: Rect, visible_count: usize) {
        let view = self.favorites_view(visible_count);
        let items = if let Some(hint) = view.empty_hint {
            vec![ListItem::new(hint).style(self.tui_style(self.theme_colors.info_text))]
        } else {
            view.rows
                .into_iter()
                .map(|row| ListItem::new(row.text).style(self.selection_style(row.selected)))
                .collect()
        };
        self.render_list(frame, area, &view.title, items);
    }

    fn favorites_view(&mut self, visible_count: usize) -> SelectableListView {
        Self::clamp_selected_and_scroll(
            &mut self.favorites_selected_index,
            &mut self.favorites_scroll_offset,
            self.favorites.len(),
            visible_count.max(1),
        );
        let rows = self
            .favorites
            .iter()
            .enumerate()
            .skip(self.favorites_scroll_offset)
            .take(visible_count)
            .map(|(idx, path)| {
                let selected = idx == self.favorites_selected_index;
                let name = std::path::Path::new(path)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or(path);
                SelectableTextRow {
                    text: format!("* {}", name),
                    selected,
                }
            })
            .collect();
        let title = self
            .t()
            .fmt_favorites_title
            .replace("{}", &self.favorites.len().to_string());
        SelectableListView {
            title,
            rows,
            empty_hint: if self.favorites.is_empty() {
                Some(self.t().favorites_empty_hint)
            } else {
                None
            },
        }
    }

    fn render_search_results(&mut self, frame: &mut Frame<'_>, area: Rect, visible_count: usize) {
        let result_visible = visible_count.saturating_sub(1);
        let view = self.search_results_view(result_visible);

        let block = render::panel_block(&view.title, self.theme_colors);
        let inner = render::inner_area(area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .split(inner);

        let input_line = format!("> {}", self.search_query);
        let input_style = if self.search_input_focused {
            self.tui_style(self.theme_colors.song_playing)
                .add_modifier(Modifier::BOLD)
        } else {
            TuiStyle::default().fg(TuiColor::DarkGray)
        };
        frame.render_widget(Paragraph::new(input_line).style(input_style), chunks[0]);

        let items: Vec<ListItem> = if let Some(hint_str) = view.empty_hint {
            vec![ListItem::new(hint_str).style(self.tui_style(self.theme_colors.info_text))]
        } else {
            view.rows
                .into_iter()
                .map(|row| ListItem::new(row.text).style(self.selection_style(row.selected)))
                .collect()
        };
        frame.render_widget(
            List::new(items).style(self.tui_style(self.theme_colors.song_normal)),
            chunks[1],
        );

        frame.render_widget(block, area);
    }

    fn search_results_view(&mut self, visible_count: usize) -> SearchResultsView {
        let prompt = if self.online_search_mode {
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
        let title = prompt
            .trim_end_matches(": ")
            .trim_end_matches(':')
            .to_string();

        let rows: Vec<SelectableTextRow> = if self.online_search_mode {
            if self.playlist_search_mode && self.current_playlist.is_none() {
                Self::clamp_selected_and_scroll(
                    &mut self.online_selected_index,
                    &mut self.online_scroll_offset,
                    self.playlist_search_results.len(),
                    visible_count.max(1),
                );
                self.playlist_search_results
                    .iter()
                    .enumerate()
                    .skip(self.online_scroll_offset)
                    .take(visible_count)
                    .map(|(idx, playlist)| {
                        let selected = idx == self.online_selected_index;
                        let count = playlist
                            .song_count
                            .map(|n| n.to_string())
                            .unwrap_or_else(|| "--".to_string());
                        let plays = self.format_playlist_play_count(playlist.play_count);
                        let text = format!(
                            "{} {} [🎵{} 🎧{}]",
                            if selected { ">" } else { " " },
                            playlist.name,
                            count,
                            plays
                        );
                        SelectableTextRow { text, selected }
                    })
                    .collect()
            } else {
                Self::clamp_selected_and_scroll(
                    &mut self.online_selected_index,
                    &mut self.online_scroll_offset,
                    self.online_search_results.len(),
                    visible_count.max(1),
                );
                self.online_search_results
                    .iter()
                    .enumerate()
                    .skip(self.online_scroll_offset)
                    .take(visible_count)
                    .map(|(idx, song)| {
                        let selected = idx == self.online_selected_index;
                        let duration = song
                            .duration_ms
                            .map(format_duration_ms)
                            .unwrap_or_else(|| "--:--".to_string());
                        let download = if self.online_downloading
                            && self.online_downloading_index == Some(idx)
                        {
                            format!(" [{}%]", self.online_download_percent)
                        } else {
                            String::new()
                        };
                        let text = format!(
                            "{} {} - {} [{}]{download}",
                            if selected { ">" } else { " " },
                            song.name,
                            song.artist,
                            duration
                        );
                        SelectableTextRow { text, selected }
                    })
                    .collect()
            }
        } else {
            Self::clamp_selected_and_scroll(
                &mut self.search_selected_index,
                &mut self.search_scroll_offset,
                self.search_results.len(),
                visible_count.max(1),
            );
            let files = self.playlist.lock().unwrap().files.clone();
            self.search_results
                .iter()
                .enumerate()
                .skip(self.search_scroll_offset)
                .take(visible_count)
                .filter_map(|(result_idx, &orig_idx)| {
                    files.get(orig_idx).map(|file| {
                        let selected = result_idx == self.search_selected_index;
                        let text = format!("{} {}", if selected { ">" } else { " " }, file.name);
                        SelectableTextRow { text, selected }
                    })
                })
                .collect()
        };

        let empty_hint = if rows.is_empty() {
            Some(if self.online_search_mode {
                if self.online_searching {
                    self.t().querying_song_info
                } else if self.juhe_search_mode {
                    self.t().juhe_enter_hint
                } else if self.playlist_search_mode {
                    self.t().playlist_empty_hint
                } else {
                    self.t().online_enter_hint
                }
            } else if self.search_query.is_empty() {
                self.t().local_search_empty_hint
            } else {
                self.t().local_search_empty_hint
            })
        } else {
            None
        };
        SearchResultsView {
            title,
            rows,
            empty_hint,
        }
    }

    fn render_right_panel(&mut self, frame: &mut Frame<'_>, area: Rect) {
        if self.api_key_input_mode {
            self.render_api_input_panel(frame, area);
        } else if self.github_token_input_mode {
            self.render_github_token_input_panel(frame, area);
        } else if self.help_mode {
            self.render_help_panel(frame, area);
        } else if self.song_info_mode {
            self.render_song_info_panel(frame, area);
        } else if self.comments_mode {
            self.render_comments_panel(frame, area);
        } else {
            self.render_lyrics_panel(frame, area);
        }
    }

    fn render_api_input_panel(&self, frame: &mut Frame<'_>, area: Rect) {
        let prompt = match self.api_input_step {
            0 => self.t().input_api_url,
            1 => self.t().input_api_key,
            2 => self.t().input_model_name,
            _ => "",
        };
        let step_text = match self.api_input_step {
            0 => "1/3",
            1 => "2/3",
            2 => "3/3",
            _ => "",
        };
        let value = if self.api_input_step == 1 {
            "*".repeat(self.api_key_input_value.chars().count().min(64))
        } else {
            self.api_key_input_value.clone()
        };
        let lines = vec![
            Line::from(Span::styled(
                format!("API 配置 {}", step_text),
                self.tui_style(self.theme_colors.section_title)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled(prompt, self.tui_style(self.theme_colors.info_text)),
                Span::styled(value, self.tui_style(self.theme_colors.song_playing)),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "Enter 保存并继续 | Esc 取消",
                TuiStyle::default().fg(TuiColor::DarkGray),
            )),
        ];
        self.render_paragraph(frame, area, "配置", lines);
    }

    fn render_github_token_input_panel(&self, frame: &mut Frame<'_>, area: Rect) {
        let value = "*".repeat(self.github_token_input_value.chars().count().min(64));
        let lines = vec![
            Line::from(Span::styled(
                "GitHub Token",
                self.tui_style(self.theme_colors.section_title)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    self.t().input_github_token,
                    self.tui_style(self.theme_colors.info_text),
                ),
                Span::styled(value, self.tui_style(self.theme_colors.song_playing)),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "Enter 保存 | Esc 取消",
                TuiStyle::default().fg(TuiColor::DarkGray),
            )),
        ];
        self.render_paragraph(frame, area, "配置", lines);
    }

    fn render_help_panel(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let view = self.help_view(Self::inner_area(area).height as usize);
        let text = view.lines.into_iter().map(Line::from).collect::<Vec<_>>();
        self.render_paragraph(frame, area, &view.title, text);
    }

    fn help_view(&mut self, visible_count: usize) -> TextPanelView {
        let lines = self.get_help_lines();
        let max_offset = lines.len().saturating_sub(visible_count);
        self.help_scroll_offset = self.help_scroll_offset.min(max_offset);
        TextPanelView {
            title: self.t().help_label.to_string(),
            lines: lines
                .into_iter()
                .skip(self.help_scroll_offset)
                .take(visible_count)
                .collect(),
        }
    }

    fn render_song_info_panel(&mut self, frame: &mut Frame<'_>, area: Rect) {
        self.check_song_info_result();
        let inner = Self::inner_area(area);
        let view = self.song_info_view(inner.width, inner.height as usize);
        let lines = view.lines.into_iter().map(Line::from).collect::<Vec<_>>();
        self.render_paragraph(frame, area, &view.title, lines);
    }

    fn song_info_view(&mut self, width: u16, visible_count: usize) -> TextPanelView {
        let mut wrapped = self.song_info_lines(width);
        let max_offset = wrapped.len().saturating_sub(visible_count);
        if self.song_info_force_scroll {
            self.song_info_scroll_offset = max_offset;
            self.song_info_force_scroll = false;
        } else {
            self.song_info_scroll_offset = self.song_info_scroll_offset.min(max_offset);
        }
        wrapped = wrapped
            .into_iter()
            .skip(self.song_info_scroll_offset)
            .take(visible_count)
            .collect();
        TextPanelView {
            title: match self.song_info_kind {
                SongInfoKind::SongInfo => self.t().song_info_label,
                SongInfoKind::CommentSummary => self.t().comment_summary_label,
            }
            .to_string(),
            lines: wrapped,
        }
    }

    fn render_comments_panel(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let current_file = self.audio_player.lock().unwrap().get_current_file();
        self.ensure_comments_up_to_date(current_file.as_ref());
        let inner = Self::inner_area(area);
        self.comment_panel_inner_y = Some(inner.y);
        self.comments_row_map.clear();
        let title = self
            .t()
            .fmt_comments_title
            .replacen("{}", &self.comments_total.to_string(), 1)
            .replacen("{}", &self.comments_page.to_string(), 1);

        if self.comments_detail_mode {
            let comment = self.current_comments.get(self.comments_selected_index);
            let text = comment
                .map(|c| {
                    let mut s = format!(
                        "{}\n{}\n",
                        c.nickname,
                        c.time_text.clone().unwrap_or_default()
                    );
                    s.push_str(&c.content);
                    if let Some(reply) = &c.reply {
                        s.push_str("\n\n---\n");
                        s.push_str(&format!("{}: {}", reply.nickname, reply.content));
                    }
                    s
                })
                .unwrap_or_else(|| self.t().no_comments.to_string());
            let lines = wrap_text_to_width(&text, inner.width.saturating_sub(1) as usize)
                .into_iter()
                .take(inner.height as usize)
                .map(Line::from)
                .collect::<Vec<_>>();
            self.render_paragraph(frame, area, &title, lines);
            return;
        }

        Self::clamp_selected_and_scroll(
            &mut self.comments_selected_index,
            &mut self.comments_scroll_offset,
            self.current_comments.len(),
            inner.height.max(1) as usize,
        );
        let view = self.comments_list_view(inner.width, inner.height as usize, &title);
        self.comments_row_map = view.row_map;
        let items: Vec<ListItem> = view
            .rows
            .into_iter()
            .map(|row| ListItem::new(row.text).style(self.selection_style(row.selected)))
            .collect();
        let items = if let Some(hint) = view.empty_hint {
            vec![ListItem::new(hint).style(self.tui_style(self.theme_colors.info_text))]
        } else {
            items
        };
        self.render_list(frame, area, &view.title, items);
    }

    fn comments_list_view(
        &self,
        width: u16,
        visible_count: usize,
        title: &str,
    ) -> CommentsListView {
        let comment_rows: Vec<(usize, SongCommentItem)> = self
            .current_comments
            .iter()
            .cloned()
            .enumerate()
            .skip(self.comments_scroll_offset)
            .take(visible_count)
            .collect();
        let row_map = comment_rows.iter().map(|(idx, _)| Some(*idx)).collect();
        let rows = comment_rows
            .into_iter()
            .map(|(idx, comment)| {
                let selected = idx == self.comments_selected_index;
                let preview = Self::truncate_with_ellipsis(
                    &comment.content,
                    width.saturating_sub(8) as usize,
                );
                SelectableTextRow {
                    text: format!(
                        "{} {}: {}",
                        if selected { ">" } else { " " },
                        comment.nickname,
                        preview
                    ),
                    selected,
                }
            })
            .collect::<Vec<_>>();
        let empty_hint = if self.comments_loading {
            Some(self.t().querying_song_info)
        } else if rows.is_empty() {
            Some(self.t().no_comments)
        } else {
            None
        };
        CommentsListView {
            title: title.to_string(),
            rows,
            row_map,
            empty_hint,
        }
    }

    fn render_lyrics_panel(&mut self, frame: &mut Frame<'_>, area: Rect) {
        self.refresh_current_lyrics();
        let inner = Self::inner_area(area);
        let view = self.lyrics_panel_view(inner.height as usize);
        self.lyrics_area_layout = Some(LyricsAreaLayout {
            start_row: inner.y,
            start_col: inner.x as usize,
            width: inner.width as usize,
            line_times: view.line_times,
        });
        let lines = view
            .rows
            .into_iter()
            .map(|row| {
                let style = if row.highlighted {
                    self.tui_style(self.theme_colors.lyric_highlight)
                        .add_modifier(Modifier::BOLD)
                } else {
                    self.tui_style(self.theme_colors.song_normal)
                };
                Line::from(Span::styled(row.text, style))
            })
            .collect::<Vec<_>>();
        self.render_paragraph(frame, area, &view.title, lines);
    }

    fn lyrics_panel_view(&self, visible_count: usize) -> LyricsPanelView {
        let (current_file, current_time) = {
            let player = self.audio_player.lock().unwrap();
            (player.get_current_file(), player.get_progress().0)
        };
        let title = current_file
            .as_ref()
            .map(|file| format!("{}{}", self.t().lyrics_label_with_song, file.name))
            .unwrap_or_else(|| self.t().lyrics_label_no_song.to_string());

        let mut line_times = Vec::new();
        let rows = if let Some(lyrics) = &self.current_lyrics {
            let (_, visible, highlight_idx) = lyrics.get_visible_lines(current_time, visible_count);
            visible
                .iter()
                .enumerate()
                .map(|(idx, lyric)| {
                    line_times.push(lyric.time);
                    let is_highlighted = Some(idx) == highlight_idx;
                    let prefix = if is_highlighted { "> " } else { "  " };
                    let text = format!("{}{}", prefix, lyric.text);
                    HighlightedTextRow {
                        text,
                        highlighted: is_highlighted,
                    }
                })
                .collect::<Vec<_>>()
        } else {
            let message = if self.lyrics_downloading || self.juhe_lyrics_loading {
                self.t().downloading_lyrics
            } else if current_file.is_some() {
                self.t().no_lyrics_found
            } else {
                self.t().select_song_for_lyrics
            };
            vec![HighlightedTextRow {
                text: message.to_string(),
                highlighted: false,
            }]
        };
        LyricsPanelView {
            title,
            rows,
            line_times,
        }
    }

    fn render_controls(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let view = self.controls_view();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(area);

        let (tip, tip_tail) = Self::split_at_display_width(&view.tip, chunks[0].width as usize);
        frame.render_widget(
            Paragraph::new(tip).style(TuiStyle::default().fg(TuiColor::DarkGray)),
            chunks[0],
        );

        let status_prefix = if tip_tail.is_empty() {
            String::new()
        } else {
            format!("{} ", tip_tail)
        };
        let volume_prefix = format!(
            "{} | {}:{:3}% ",
            view.play_status_text,
            self.t().volume_label,
            view.volume_percent
        );
        let full_volume_prefix = format!("{}{}", status_prefix, volume_prefix);
        let volume_width = chunks[1]
            .width
            .saturating_sub(term_display_width(&full_volume_prefix) as u16 + 2)
            .max(1) as usize;
        let volume_filled = (view.volume_percent as usize * volume_width / 100).min(volume_width);
        let volume_bar = format!(
            "[{}{}]",
            "█".repeat(volume_filled),
            "░".repeat(volume_width.saturating_sub(volume_filled))
        );
        let mut volume_line = Vec::new();
        if !status_prefix.is_empty() {
            volume_line.push(Span::styled(
                status_prefix,
                TuiStyle::default().fg(TuiColor::DarkGray),
            ));
        }
        volume_line.push(Span::styled(
            format!("{}{}", volume_prefix, volume_bar),
            self.tui_style(self.theme_colors.info_text),
        ));
        frame.render_widget(Paragraph::new(Line::from(volume_line)), chunks[1]);
        self.volume_bar_layout = Some(VolumeBarLayout {
            row: chunks[1].y,
            bar_start_col: chunks[1].x as usize + term_display_width(&full_volume_prefix) + 1,
            bar_width: volume_width,
        });

        let rms_prefix = format!("{} ", view.now_playing_text);
        let rms_width = chunks[2]
            .width
            .saturating_sub(term_display_width(&rms_prefix) as u16 + 2)
            .max(1) as usize;
        let rms_filled = (view.realtime_percent as usize * rms_width / 100).min(rms_width);
        let rms_bar = format!(
            "[{}{}]",
            "█".repeat(rms_filled),
            "░".repeat(rms_width.saturating_sub(rms_filled))
        );
        frame.render_widget(
            Paragraph::new(format!("{}{}", rms_prefix, rms_bar))
                .style(self.tui_style(self.theme_colors.status_text)),
            chunks[2],
        );

        let progress_prefix = format!("{} ", view.progress_label);
        let progress_width = chunks[3]
            .width
            .saturating_sub(term_display_width(&progress_prefix) as u16 + 2)
            .max(1) as usize;
        let progress_filled =
            ((view.progress_ratio * progress_width as f64).round() as usize).min(progress_width);
        let progress_bar = format!(
            "[{}{}]",
            "█".repeat(progress_filled),
            "░".repeat(progress_width.saturating_sub(progress_filled))
        );
        frame.render_widget(
            Paragraph::new(format!("{}{}", progress_prefix, progress_bar))
                .style(self.tui_style(self.theme_colors.progress_text)),
            chunks[3],
        );
        self.progress_bar_layout = Some(ProgressBarLayout {
            row: chunks[3].y,
            bar_start_col: chunks[3].x as usize + term_display_width(&progress_prefix) + 1,
            bar_width: progress_width,
        });

        let separator = "─".repeat(chunks[4].width as usize);
        frame.render_widget(
            Paragraph::new(separator).style(self.tui_style(self.theme_colors.section_title)),
            chunks[4],
        );
    }

    fn controls_view(&self) -> ControlsView {
        let (state, volume, mode, progress, total, realtime) = {
            let player = self.audio_player.lock().unwrap();
            let (progress, total) = player.get_progress();
            (
                player.get_state(),
                player.get_volume(),
                player.get_play_mode(),
                progress,
                total,
                player.get_realtime_volume(),
            )
        };
        let current_song_name = {
            let player = self.audio_player.lock().unwrap();
            player.get_current_file().map(|file| file.name)
        };
        let progress_ratio = total
            .map(|t| {
                let total_secs = t.as_secs_f64();
                if total_secs > 0.0 {
                    progress.as_secs_f64() / total_secs
                } else {
                    0.0
                }
            })
            .unwrap_or(0.0)
            .clamp(0.0, 1.0);
        let progress_text = format_progress(progress, total);
        let now_playing_text = if state == PlayState::Stopped {
            format!("{}{}", self.now_playing_prefix(), progress_text)
        } else if let Some(song_name) = current_song_name {
            format!("{}{}", self.now_playing_prefix(), song_name)
        } else {
            self.t().state_stopped_msg.to_string()
        };
        ControlsView {
            tip: self.get_help_tip_text(),
            play_status_text: format!(
                "{}: {} | {}: {}",
                self.t().play_status_label,
                self.play_state_text(state),
                self.t().play_mode_label,
                self.play_mode_text(mode),
            ),
            now_playing_text,
            progress_label: format!("{}{}", self.t().progress_label, progress_text),
            progress_ratio,
            volume_percent: volume,
            realtime_percent: (realtime * 100.0).round() as u8,
        }
    }

    fn render_cursor(&self, frame: &mut Frame<'_>, left: Rect, right: Rect) {
        if self.ai_recommend_input_mode {
            let header_x = 1 + term_display_width(self.t().recommendation_title) as u16 + term_display_width(&self.ai_recommend_input_value) as u16;
            let x = header_x.min(self.terminal_width.saturating_sub(2));
            frame.set_cursor_position((x, 1));
        } else if self.api_key_input_mode {
            let prompt = match self.api_input_step {
                0 => self.t().input_api_url,
                1 => self.t().input_api_key,
                2 => self.t().input_model_name,
                _ => "",
            };
            let x = right.x
                + 1
                + (term_display_width(prompt) + term_display_width(&self.api_key_input_value))
                    as u16;
            frame.set_cursor_position((x.min(right.right().saturating_sub(1)), right.y + 3));
        } else if self.github_token_input_mode {
            let x = right.x
                + 1
                + (term_display_width(self.t().input_github_token)
                    + term_display_width(&self.github_token_input_value)) as u16;
            frame.set_cursor_position((x.min(right.right().saturating_sub(1)), right.y + 3));
        } else if self.search_mode
            && self.search_input_focused
            && !(self.playlist_search_mode && self.current_playlist.is_some())
        {
            let x = left.x + 1 + 2 + term_display_width(&self.search_query) as u16;
            frame.set_cursor_position((x.min(left.right().saturating_sub(1)), left.y + 1));
        }
    }

    fn refresh_current_lyrics(&mut self) {
        self.check_lyrics_download_result();
        let current_file = self.audio_player.lock().unwrap().get_current_file();
        let needs_update = match (&current_file, &self.lyrics_file_path) {
            (Some(file), Some(cached_path)) => cached_path != &file.path.with_extension("lrc"),
            (Some(_), None) => true,
            (None, _) => false,
        };
        if !needs_update {
            return;
        }
        if let Some(file) = current_file {
            let lrc_path = file.path.with_extension("lrc");
            if let Some(lyrics) = Lyrics::from_local_lrc(&file.path) {
                self.current_lyrics = Some(lyrics);
            } else if lrc_path.exists() {
                self.current_lyrics = None;
            } else if !self.juhe_lyrics_loading && !self.lyrics_downloading {
                let file_stem = file.path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
                let (artist, title) = crate::lyrics::Lyrics::parse_artist_title(file_stem)
                    .unwrap_or_else(|| (String::new(), file_stem.to_string()));
                self.juhe_lyrics_loading = true;
                self.juhe_lyrics_rx = Some(crate::search::search_and_get_juhe_lyrics_background(
                    artist,
                    title,
                    file.path.clone(),
                ));
                self.current_lyrics = None;
            }
            self.lyrics_file_path = Some(lrc_path);
        }
    }

    fn render_list(&self, frame: &mut Frame<'_>, area: Rect, title: &str, items: Vec<ListItem>) {
        render::render_list(frame, area, title, items, self.theme_colors);
    }

    fn render_paragraph(
        &self,
        frame: &mut Frame<'_>,
        area: Rect,
        title: &str,
        lines: Vec<Line<'_>>,
    ) {
        render::render_paragraph(frame, area, title, lines, self.theme_colors);
    }

    fn inner_area(area: Rect) -> Rect {
        render::inner_area(area)
    }

    fn selection_style(&self, selected: bool) -> TuiStyle {
        render::selection_style(selected, self.theme_colors)
    }

    fn tui_style(&self, color: style::Color) -> TuiStyle {
        theme::tui_style(color)
    }

    /// 获取帮助信息文本行
    fn get_help_lines(&self) -> Vec<String> {
        self.t()
            .help_lines
            .iter()
            .map(|s| (*s).to_string())
            .collect()
    }
    /// 处理键盘事件
    fn handle_desktop_key(&mut self, key: &str) {
        match key {
            "LEFT" => self.play_prev(),
            "RIGHT" => self.play_next(),
            "UP" => {
                self.desktop_lyrics.adjust_alpha(1);
                self.save_config_now();
            }
            "DOWN" => {
                self.desktop_lyrics.adjust_alpha(-1);
                self.save_config_now();
            }
            "PAGEUP" | "PAGEDOWN" if !self.comments_mode => {
                self.desktop_lyrics.toggle_position();
                self.save_config_now();
            }
            "SPACE" => {
                let player = self.audio_player.lock().unwrap();
                match player.get_state() {
                    crate::defs::PlayState::Playing => {
                        drop(player);
                        self.audio_player.lock().unwrap().pause();
                    }
                    crate::defs::PlayState::Paused => {
                        drop(player);
                        self.audio_player.lock().unwrap().resume();
                    }
                    _ => {}
                }
            }
            "-" => {
                self.audio_player.lock().unwrap().volume_down();
            }
            "=" => {
                self.audio_player.lock().unwrap().volume_up();
            }
            "[" | "【" | "［" => self.seek_relative(-5.0),
            "]" | "】" | "］" | "’" | "‘" => self.seek_relative(5.0),
            "," | "，" | "、" => self.seek_relative(-10.0),
            "." | "。" => self.seek_relative(10.0),
            "1" => self.set_play_mode(crate::defs::PlayMode::Single),
            "2" => self.set_play_mode(crate::defs::PlayMode::RepeatOne),
            "3" => self.set_play_mode(crate::defs::PlayMode::Sequence),
            "4" => self.set_play_mode(crate::defs::PlayMode::LoopAll),
            "5" => self.set_play_mode(crate::defs::PlayMode::Random),

            "T" => {
                self.theme = self.theme.next();
                self.theme_colors = self.theme.colors();
                self.desktop_lyrics.update_theme(self.theme.config_key());
                self.cached_lyrics_title = None;
                self.save_config_now();
            }
            _ => {}
        }
    }

    fn handle_key_event(&mut self, code: KeyCode) -> io::Result<()> {
        if self.handle_api_key_input(code) {
            return Ok(());
        }

        if self.handle_github_token_input(code) {
            return Ok(());
        }

        if self.handle_ai_recommend_input(code) {
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
                        if self.dir_history_selected_index >= self.dir_history.len()
                            && self.dir_history_selected_index > 0
                        {
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
                        if self.favorites_selected_index >= self.favorites.len()
                            && self.favorites_selected_index > 0
                        {
                            self.favorites_selected_index -= 1;
                        }
                        self.save_config_now();
                    }
                }
                _ => {}
            }
            return Ok(());
        }

        if self.search_mode && self.handle_search_input(code) {
            return Ok(());
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
                    self.main_focus = MainFocus::Playlist;
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
                    let wrapped_lines = wrap_text_to_width(
                        &self.song_info_content,
                        right_width.saturating_sub(1) as usize,
                    );
                    let max_offset = wrapped_lines.len().saturating_sub(visible_count);
                    if self.song_info_scroll_offset < max_offset {
                        self.song_info_scroll_offset += 1;
                    }
                } else if self.comments_mode {
                    if !self.current_comments.is_empty() {
                        let max_idx = self.current_comments.len().saturating_sub(1);
                        self.comments_selected_index =
                            (self.comments_selected_index + 1).min(max_idx);
                        let visible_count = self.terminal_height.saturating_sub(12) as usize;
                        Self::adjust_scroll_offset(
                            self.comments_selected_index,
                            &mut self.comments_scroll_offset,
                            visible_count.max(1),
                        );
                    }
                } else {
                    self.main_focus = MainFocus::Playlist;
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
                } else if self.main_focus == MainFocus::Recommendation
                    && self.has_selectable_recommendations()
                {
                    self.activate_selected_recommendation();
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
                    if self.comments_detail_mode {
                        // 先从详情返回评论列表
                        self.comments_detail_mode = false;
                    } else {
                        // 再从评论列表返回歌词视图
                        self.comments_mode = false;
                        self.comments_detail_mode = false;
                    }
                } else if self.song_info_mode {
                    // 评论总结从评论列表进入，Esc 返回评论列表；普通歌曲信息返回歌词视图
                    if self.song_info_kind == SongInfoKind::CommentSummary {
                        self.comments_mode = true;
                        self.comments_detail_mode = false;
                    }
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
                if self.main_focus == MainFocus::Recommendation && self.has_selectable_recommendations()
                {
                    self.move_recommendation_selection(-1);
                } else {
                    // 上一曲
                    self.play_prev();
                }
            }
            KeyCode::Right => {
                if self.main_focus == MainFocus::Recommendation && self.has_selectable_recommendations()
                {
                    self.move_recommendation_selection(1);
                } else {
                    // 下一曲
                    self.play_next();
                }
            }
            KeyCode::Char('[') | KeyCode::Char('【') | KeyCode::Char('［') => {
                // 快退 5 秒
                self.seek_relative(-5.0);
            }
            KeyCode::Char(']')
            | KeyCode::Char('】')
            | KeyCode::Char('］')
            | KeyCode::Char('’')
            | KeyCode::Char('‘') => {
                // 快进 5 秒
                self.seek_relative(5.0);
            }
            KeyCode::Char(',') | KeyCode::Char('，') | KeyCode::Char('、') => {
                // 快退 10 秒
                self.seek_relative(-10.0);
            }
            KeyCode::Char('.') | KeyCode::Char('。') => {
                // 快进 10 秒
                self.seek_relative(10.0);
            }
            KeyCode::Char('+') | KeyCode::Char('=') => {
                self.audio_player.lock().unwrap().volume_up();
            }
            KeyCode::Char('-') => {
                self.audio_player.lock().unwrap().volume_down();
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
                // 同步更新桌面歌词主题
                self.desktop_lyrics.update_theme(self.theme.config_key());
                // 立即触发歌词更新，确保主题同步（因为歌词更新也会带主题信息）
                self.push_current_lyrics_to_desktop();
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
                if self.search_mode
                    && self.online_search_mode
                    && self.playlist_search_mode
                    && self.current_playlist.is_some()
                {
                    // ignore
                } else {
                    // 进入本地搜索模式（搜索音乐目录）
                    self.clear_online_download_state();
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
                if self.search_mode
                    && self.online_search_mode
                    && self.playlist_search_mode
                    && self.current_playlist.is_some()
                {
                    // ignore
                } else {
                    // 进入网络搜索模式（搜索网络歌曲并下载）
                    self.clear_online_download_state();
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
                if self.search_mode
                    && self.online_search_mode
                    && self.playlist_search_mode
                    && self.current_playlist.is_some()
                {
                    // ignore
                } else {
                    // 进入聚合搜索搜索模式（通过独家API获取播放链接和歌词）
                    self.clear_online_download_state();
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
                self.clear_online_download_state();
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
                if self.comments_mode {
                    self.start_comment_summary_mode();
                    return Ok(());
                }
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
                self.comments_song_name = current_file
                    .map(|f| f.name.trim().to_string())
                    .unwrap_or_default();
            }
            KeyCode::Char('i') | KeyCode::Char('I') => {
                // i：直接查询歌曲信息（有 DeepSeek Key 用 DeepSeek，否则用 OpenRouter 免费模型）
                self.start_song_info_mode_for_current_song();
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.recommand = true;
                self.start_recommendations_if_enabled();
                self.save_config_now();
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                self.open_ai_recommend_input_mode();
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
                if self.comments_mode {
                    if self.comments_page > 1 {
                        self.comments_page -= 1;
                    }
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
                } else if self.desktop_lyrics.is_active() {
                    // 桌面歌词激活时 PgUp/PgDn 切换位置。评论翻页时不触发。
                    self.desktop_lyrics.toggle_position();
                }
            }
            KeyCode::Char('f') | KeyCode::Char('F') => {
                // 在在线搜索模式（网络/聚合/歌单）下屏蔽 f 收藏，避免误操作到本地播放列表
                if self.search_mode && self.online_search_mode && self.playlist_search_mode {
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
            KeyCode::Char('z') | KeyCode::Char('Z') => {
                let theme_name = self.theme.config_key();
                self.desktop_lyrics.toggle(theme_name);
                if self.desktop_lyrics.is_active() {
                    self.push_current_lyrics_to_desktop();
                }
                self.save_config_now();
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
        self.clear_online_download_state();
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
                    self.clear_online_download_state();
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
            let log_msg = format!(
                "开始下载: {} - {}, source={:?}, juhe_platform={}, juhe_song_id={}",
                song.artist, song.name, song.source, song.juhe_platform, song.juhe_song_id
            );
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
                self.record_play_history(&play_file.name, &play_file.path);
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
                // 重置推荐下载状态
                self.recommendation_downloading = false;
                self.recommendation_download_percent = 0;
                self.recommendation_downloading_name = None;
            } else if let Err(e) = play_result {
                if self.online_auto_skip_times.is_empty() {
                    self.update_status(&format!("{}{}", self.t().play_failed, e));
                } else {
                    // 自动切歌链路下，缓存命中但播放失败时继续尝试下一首，不弹提示
                    self.play_next_with_flag(false);
                }
                // 重置推荐下载状态
                self.recommendation_downloading = false;
                self.recommendation_download_percent = 0;
                self.recommendation_downloading_name = None;
            }
            return;
        }

        if let Some(local_idx) = self.find_local_song_index_for_online(&song) {
            self.play_song_by_index(local_idx);
            self.skip_auto_lyrics_download_for_current_song = true;
            let log_msg = format!(
                "本地已存在该歌曲，跳过下载直接播放: {} - {}, local_idx={}",
                song.artist, song.name, local_idx
            );
            let timestamp = Local::now().format("%H:%M:%S%.3f");
            let line = format!("[{}] {}\n", timestamp, log_msg);
            let _ = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(crate::config::get_daily_log_path())
                .and_then(|mut f| std::io::Write::write_all(&mut f, line.as_bytes()));
            // 重置推荐下载状态
            self.recommendation_downloading = false;
            self.recommendation_download_percent = 0;
            self.recommendation_downloading_name = None;
            return;
        }

        let save_dir = {
            let playlist = self.playlist.lock().unwrap();
            // 保存到当前音乐目录，无目录时使用默认音乐目录（配置目录/music）
            playlist
                .directory
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
                        self.append_runtime_log(
                            "[LyricsFallback] 兜底歌词下载成功，已更新当前歌词",
                        );
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
                        if self.recommendation_downloading {
                            self.recommendation_download_percent = percent;
                        }
                    }
                    DownloadProgress::Done(result) => {
                        self.online_downloading = false;
                        self.online_download_rx = None;
                        self.online_download_percent = 0;
                        self.online_downloading_index = None;
                        self.recommendation_downloading = false;
                        self.recommendation_download_percent = 0;
                        self.recommendation_downloading_name = None;

                        // 写入日志
                        {
                            let log_msg = format!(
                                "下载完成: path={:?}, error={:?}",
                                result.local_path, result.error
                            );
                            let timestamp = Local::now().format("%H:%M:%S%.3f");
                            let line = format!("[{}] {}\n", timestamp, log_msg);
                            let _ = std::fs::OpenOptions::new()
                                .create(true)
                                .append(true)
                                .open(crate::config::get_daily_log_path())
                                .and_then(|mut f| {
                                    std::io::Write::write_all(&mut f, line.as_bytes())
                                });
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
                                        self.record_play_history(&play_file.name, &play_file.path);
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
                                        playlist.directory.clone().unwrap_or_else(|| {
                                            crate::config::get_default_music_dir()
                                                .to_string_lossy()
                                                .to_string()
                                        })
                                    };
                                    self.load_directory_and_play(&dir, &path_str);
                                }
                            }
                            None => {
                                // 下载失败，不做提示以避免覆盖波形
                                let _err = result
                                    .error
                                    .unwrap_or_else(|| self.t().unknown_error.to_string());
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
                    self.lyrics_download_rx =
                        Some(Lyrics::download_lyrics_background(&target_music_path));
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
        playlist
            .files
            .iter()
            .enumerate()
            .find(|(_, f)| f.path.to_string_lossy() == *path)
            .map(|(i, _)| i)
    }

    /// 归一化歌曲匹配键（用于在线歌曲与本地文件名匹配）
    fn normalize_song_key(input: &str) -> String {
        input
            .to_lowercase()
            .chars()
            .filter(|c| {
                !c.is_whitespace()
                    && !matches!(
                        c,
                        '-' | '_'
                            | '·'
                            | '•'
                            | ','
                            | '，'
                            | '.'
                            | '。'
                            | '('
                            | ')'
                            | '（'
                            | '）'
                            | '['
                            | ']'
                            | '【'
                            | '】'
                    )
            })
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
            self.downloaded_online_song_cache
                .insert(key, path.to_path_buf());
        }
    }

    fn find_cached_local_path_for_online(
        &mut self,
        song: &OnlineSong,
    ) -> Option<std::path::PathBuf> {
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
            if !artist_key.is_empty()
                && !name_key.is_empty()
                && local_key.contains(&artist_key)
                && local_key.contains(&name_key)
            {
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
                    self.record_play_history(&file.name, &file.path);
                    // 歌曲切换成功后保存配置
                    self.save_config_now();
                }
                Err(e) => {
                    self.update_status(&format!("{}{}", self.t().play_failed, e));
                }
            }
        }
    }

    /// 处理鼠标事件
    fn handle_mouse_event_impl(&mut self, mouse_event: MouseEvent) -> io::Result<()> {
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

                if self.ai_recommend_input_mode && row == 1 {
                    let click_offset = col;
                    if let Some(query) = self
                        .ai_recommend_preset_items
                        .iter()
                        .find(|item| click_offset >= item.start_col && click_offset < item.end_col)
                        .map(|item| item.query.clone())
                    {
                        self.start_ai_recommend_query_with(query);
                    }
                    return Ok(());
                }

                if self.recommand && row <= 2 {
                    // 计算点击位置相对于显示文本的列偏移
                    // Header 区域从第0列开始，文本从第1列开始显示
                    let text_start_col = 1;
                    if col >= text_start_col {
                        let click_offset = (col as usize - text_start_col) + self.recommendation_scroll_offset;
                        if let Some((idx, _name)) = self
                            .recommendation_items
                            .iter()
                            .enumerate()
                            .find(|(_, item)| click_offset >= item.start_col && click_offset < item.end_col)
                            .map(|(idx, item)| (idx, (item.name.clone(), item.search_query.clone())))
                        {
                            self.recommendation_selected_index = Some(idx);
                            self.main_focus = MainFocus::Recommendation;
                            self.activate_selected_recommendation();
                            return Ok(());
                        }
                    }
                    // 点击推荐区域空白处，打开 AI 自然语言推荐输入框
                    self.open_ai_recommend_input_mode();
                    return Ok(());
                }

                // 所有模式：检查是否点击了音量条区域
                if let Some(layout) = self.volume_bar_layout {
                    if row == layout.row
                        && col >= layout.bar_start_col
                        && col < layout.bar_start_col + layout.bar_width
                    {
                        // 音量条共20格，点击位置按比例映射到0-100，四舍五入到5的倍数
                        let denominator = layout.bar_width.saturating_sub(1).max(1);
                        let ratio = (col - layout.bar_start_col) as f64 / denominator as f64;
                        let new_volume = (ratio * 100.0 / 5.0).round() as u8 * 5;
                        let new_volume = new_volume.clamp(0, 100);

                        self.audio_player.lock().unwrap().set_volume(new_volume);
                        return Ok(());
                    }
                }

                // 所有模式：检查是否点击了进度条区域
                if let Some(layout) = self.progress_bar_layout {
                    if row == layout.row
                        && col >= layout.bar_start_col
                        && col < layout.bar_start_col + layout.bar_width
                    {
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
                    // 点击搜索栏时，切回输入框焦点。搜索结果紧跟在输入行之后，不能拦截第一条结果。
                    let left_width = (self.terminal_width as f32 * 0.50) as usize;
                    let in_playlist_detail = self.online_search_mode
                        && self.playlist_search_mode
                        && self.current_playlist.is_some();
                    if !in_playlist_detail && row == 4 && col < left_width {
                        self.search_input_focused = true;
                        return Ok(());
                    }

                    if let Some(layout) = self.playlist_layout {
                        if col < layout.left_width as usize && row >= layout.start_row {
                            let click_row = (row - layout.start_row) as usize;
                            if click_row >= 1 && click_row < layout.visible_count {
                                let result_row = click_row - 1;
                                if self.online_search_mode {
                                    self.search_input_focused = false;
                                    if self.playlist_search_mode && self.current_playlist.is_none()
                                    {
                                        let clicked_index = self.online_scroll_offset + result_row;
                                        if clicked_index < self.playlist_search_results.len() {
                                            let already_selected =
                                                self.online_selected_index == clicked_index;
                                            self.online_selected_index = clicked_index;
                                            self.playlist_list_selected_index = clicked_index;
                                            self.main_focus = MainFocus::Playlist;
                                            if !already_selected {
                                                return Ok(());
                                            }
                                            if let Some(pl) = self
                                                .playlist_search_results
                                                .get(clicked_index)
                                                .cloned()
                                            {
                                                self.clear_online_download_state();
                                                self.online_searching = true;
                                                self.online_search_results.clear();
                                                self.online_selected_index = 0;
                                                self.online_scroll_offset = 0;
                                                self.current_playlist = Some(pl.clone());
                                                self.playlist_songs_rx = Some(
                                                    crate::search::fetch_playlist_songs_background(
                                                        pl,
                                                    ),
                                                );
                                            }
                                        }
                                    } else {
                                        let clicked_index = self.online_scroll_offset + result_row;
                                        if clicked_index < self.online_search_results.len() {
                                            let already_selected =
                                                self.online_selected_index == clicked_index;
                                            self.online_selected_index = clicked_index;
                                            self.main_focus = MainFocus::Playlist;
                                            if !already_selected {
                                                return Ok(());
                                            }
                                            if let Some(song) = self
                                                .online_search_results
                                                .get(clicked_index)
                                                .cloned()
                                            {
                                                if !self.online_downloading {
                                                    self.online_auto_skip_times.clear();
                                                    self.start_download_song(song);
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    let clicked_index = self.search_scroll_offset + result_row;
                                    if clicked_index < self.search_results.len() {
                                        let already_selected =
                                            self.search_selected_index == clicked_index;
                                        self.search_selected_index = clicked_index;
                                        self.main_focus = MainFocus::Playlist;
                                        if !already_selected {
                                            return Ok(());
                                        }
                                        if let Some(&orig_idx) =
                                            self.search_results.get(clicked_index)
                                        {
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

                // 帮助/歌曲信息：右侧 URL 可点击打开
                if self.help_mode || self.song_info_mode {
                    let _ = self.open_clicked_right_panel_url(col, row);
                    return Ok(());
                } else if self.comments_mode {
                    if !self.comments_detail_mode {
                        let left_width = (self.terminal_width as f32 * 0.50) as usize;
                        if col > left_width {
                            if let Some(inner_y) = self.comment_panel_inner_y {
                                if row as usize >= inner_y as usize {
                                    let click_row = (row as usize) - (inner_y as usize);
                                    if click_row < self.comments_row_map.len() {
                                        if let Some(comment_idx) = self.comments_row_map[click_row]
                                        {
                                            if comment_idx < self.current_comments.len() {
                                                let already_selected =
                                                    self.comments_selected_index == comment_idx;
                                                self.comments_selected_index = comment_idx;
                                                if !already_selected {
                                                    return Ok(());
                                                }
                                                self.comments_detail_mode = true;
                                            }
                                        }
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

                // 推荐歌曲区域滚轮向上 -> 水平向左滚动
                if self.recommand && row <= 2 {
                    self.recommendation_scroll_offset =
                        self.recommendation_scroll_offset.saturating_sub(3);
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
                        self.comments_selected_index =
                            self.comments_selected_index.saturating_sub(1);
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
                                let total = if self.playlist_search_mode
                                    && self.current_playlist.is_none()
                                {
                                    self.playlist_search_results.len()
                                } else {
                                    self.online_search_results.len()
                                };
                                if total > 0 {
                                    self.online_selected_index =
                                        self.online_selected_index.saturating_sub(1);
                                    Self::adjust_scroll_offset(
                                        self.online_selected_index,
                                        &mut self.online_scroll_offset,
                                        layout.visible_count.max(1),
                                    );
                                    self.search_input_focused = false;
                                }
                            } else {
                                let total = self.search_results.len();
                                if total > 0 {
                                    self.search_selected_index =
                                        self.search_selected_index.saturating_sub(1);
                                    Self::adjust_scroll_offset(
                                        self.search_selected_index,
                                        &mut self.search_scroll_offset,
                                        layout.visible_count.max(1),
                                    );
                                    self.search_input_focused = false;
                                }
                            }
                        }
                    }
                } else if self.dir_history_mode {
                    // 音乐目录模式：滚轮向上
                    if let Some(layout) = self.playlist_layout {
                        if col < layout.left_width as usize && self.dir_history_scroll_offset > 0 {
                            self.dir_history_scroll_offset -= 1;
                            let total_len = self.dir_history.len();
                            if total_len > 0 {
                                let view_start = self.dir_history_scroll_offset;
                                let view_end = self
                                    .dir_history_scroll_offset
                                    .saturating_add(layout.visible_count)
                                    .saturating_sub(1)
                                    .min(total_len - 1);
                                if self.dir_history_selected_index < view_start {
                                    self.dir_history_selected_index = view_start;
                                } else if self.dir_history_selected_index > view_end {
                                    self.dir_history_selected_index = view_end;
                                }
                            }
                        }
                    }
                } else if self.favorites_mode {
                    // 收藏列表模式：滚轮向上
                    if let Some(layout) = self.playlist_layout {
                        if col < layout.left_width as usize && self.favorites_scroll_offset > 0 {
                            self.favorites_scroll_offset -= 1;
                            let total_len = self.favorites.len();
                            if total_len > 0 {
                                let view_start = self.favorites_scroll_offset;
                                let view_end = self
                                    .favorites_scroll_offset
                                    .saturating_add(layout.visible_count)
                                    .saturating_sub(1)
                                    .min(total_len - 1);
                                if self.favorites_selected_index < view_start {
                                    self.favorites_selected_index = view_start;
                                } else if self.favorites_selected_index > view_end {
                                    self.favorites_selected_index = view_end;
                                }
                            }
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

                // 推荐歌曲区域滚轮向下 -> 水平向右滚动
                if self.recommand && row <= 2 {
                    self.recommendation_scroll_offset += 3;
                    return Ok(());
                }

                if self.song_info_mode {
                    // AI 歌曲信息模式：右侧区域滚轮向下滚动
                    let left_width = (self.terminal_width as f32 * 0.50) as usize;
                    if col > left_width {
                        let visible_count = self.terminal_height.saturating_sub(12) as usize;
                        let content = self.song_info_content.clone();
                        let right_width = self.terminal_width.saturating_sub(left_width as u16 + 1);
                        let wrapped_lines =
                            wrap_text_to_width(&content, right_width.saturating_sub(1) as usize);
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
                        self.comments_selected_index =
                            (self.comments_selected_index + 1).min(max_idx);
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
                                let total = if self.playlist_search_mode
                                    && self.current_playlist.is_none()
                                {
                                    self.playlist_search_results.len()
                                } else {
                                    self.online_search_results.len()
                                };
                                if total > 0 {
                                    let max_idx = total.saturating_sub(1);
                                    self.online_selected_index =
                                        (self.online_selected_index + 1).min(max_idx);
                                    Self::adjust_scroll_offset(
                                        self.online_selected_index,
                                        &mut self.online_scroll_offset,
                                        layout.visible_count.max(1),
                                    );
                                    self.search_input_focused = false;
                                }
                            } else {
                                let total = self.search_results.len();
                                if total > 0 {
                                    let max_idx = total.saturating_sub(1);
                                    self.search_selected_index =
                                        (self.search_selected_index + 1).min(max_idx);
                                    Self::adjust_scroll_offset(
                                        self.search_selected_index,
                                        &mut self.search_scroll_offset,
                                        layout.visible_count.max(1),
                                    );
                                    self.search_input_focused = false;
                                }
                            }
                        }
                    }
                } else if self.dir_history_mode {
                    // 音乐目录模式：滚轮向下
                    if let Some(layout) = self.playlist_layout {
                        if col < layout.left_width as usize {
                            let max_offset =
                                self.dir_history.len().saturating_sub(layout.visible_count);
                            if self.dir_history_scroll_offset < max_offset {
                                self.dir_history_scroll_offset += 1;
                                let total_len = self.dir_history.len();
                                if total_len > 0 {
                                    let view_start = self.dir_history_scroll_offset;
                                    let view_end = self
                                        .dir_history_scroll_offset
                                        .saturating_add(layout.visible_count)
                                        .saturating_sub(1)
                                        .min(total_len - 1);
                                    if self.dir_history_selected_index < view_start {
                                        self.dir_history_selected_index = view_start;
                                    } else if self.dir_history_selected_index > view_end {
                                        self.dir_history_selected_index = view_end;
                                    }
                                }
                            }
                        }
                    }
                } else if self.favorites_mode {
                    // 收藏列表模式：滚轮向下
                    if let Some(layout) = self.playlist_layout {
                        if col < layout.left_width as usize {
                            let max_offset =
                                self.favorites.len().saturating_sub(layout.visible_count);
                            if self.favorites_scroll_offset < max_offset {
                                self.favorites_scroll_offset += 1;
                                let total_len = self.favorites.len();
                                if total_len > 0 {
                                    let view_start = self.favorites_scroll_offset;
                                    let view_end = self
                                        .favorites_scroll_offset
                                        .saturating_add(layout.visible_count)
                                        .saturating_sub(1)
                                        .min(total_len - 1);
                                    if self.favorites_selected_index < view_start {
                                        self.favorites_selected_index = view_start;
                                    } else if self.favorites_selected_index > view_end {
                                        self.favorites_selected_index = view_end;
                                    }
                                }
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
                if matched {
                    Some(idx)
                } else {
                    None
                }
            })
    }

    /// 播放下一曲（manual: 是否为用户手动切换）
    fn play_next_with_flag(&mut self, manual: bool) {
        // 在线搜索结果视图（网络/聚合/歌单歌曲）统一按在线结果模拟 1~5 播放模式
        if self.search_mode && self.online_search_mode && !self.online_search_results.is_empty() {
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
                    if manual {
                        Some((cur + 1).min(len - 1))
                    } else {
                        None
                    }
                }
                PlayMode::RepeatOne => Some(cur),
                PlayMode::Sequence => {
                    if cur + 1 < len {
                        Some(cur + 1)
                    } else {
                        None
                    }
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
        use crossterm::{
            cursor,
            event::{DisableMouseCapture, EnableMouseCapture},
            execute, terminal,
        };
        use std::io::{self, Write};

        // 临时恢复终端
        let _ = execute!(
            io::stdout(),
            DisableMouseCapture,
            terminal::LeaveAlternateScreen,
            cursor::Show
        );
        let _ = terminal::disable_raw_mode();

        // 提示用户输入
        print!("\n{}", self.t().input_dir_path);
        let _ = io::stdout().flush();

        // 读取用户输入
        let mut input = String::new();
        let result = io::stdin().read_line(&mut input);

        // 重新进入 raw mode
        let _ = terminal::enable_raw_mode();
        let _ = execute!(
            io::stdout(),
            terminal::EnterAlternateScreen,
            cursor::Hide,
            EnableMouseCapture
        );

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
                self.update_status(&format!("{}{}", self.t().load_failed, e));
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
                let target_idx = new_playlist
                    .files
                    .iter()
                    .position(|s| s.path.to_string_lossy() == song_path);

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
                self.update_status(&format!("{}{}", self.t().load_failed, e));
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

    pub fn record_played_file(&self, file: &crate::defs::MusicFile) {
        self.record_play_history(&file.name, &file.path);
    }

    /// 推送当前歌词到桌面悬浮窗
    fn push_current_lyrics_to_desktop(&mut self) {
        if !self.desktop_lyrics.is_active() {
            return;
        }
        let theme_name = self.theme.config_key();
        if let Some(ref lyrics) = self.current_lyrics {
            if !lyrics.is_empty() {
                let current_time = {
                    let player = self.audio_player.lock().unwrap();
                    player.get_progress().0
                };
                let lines = lyrics.get_lines();

                // Convert to the format expected by desktop lyrics: Vec<(String, f64)>
                let all_lyrics: Vec<(String, f64)> = lines
                    .iter()
                    .map(|line| (line.text.clone(), line.time.as_secs_f64()))
                    .collect();

                let current_time_sec = current_time.as_secs_f64();

                self.desktop_lyrics
                    .update_all_lyrics(&all_lyrics, current_time_sec, theme_name);

                // Also update the traditional three-line format for backward compatibility
                let idx = lines.partition_point(|line| line.time <= current_time);
                let current_idx = if idx == 0 { 0 } else { idx - 1 };
                let prev_text = if current_idx > 0 {
                    &lines[current_idx - 1].text
                } else {
                    ""
                };
                let curr_text = &lines[current_idx].text;
                let next_text = if current_idx + 1 < lines.len() {
                    &lines[current_idx + 1].text
                } else {
                    ""
                };
                let combined = format!("{}\n{}\n{}", prev_text, curr_text, next_text);
                self.desktop_lyrics.update_lyrics(&combined, theme_name);
                return;
            }
        }
        // No lyrics available
        let empty_lyrics: Vec<(String, f64)> = vec![];
        self.desktop_lyrics
            .update_all_lyrics(&empty_lyrics, 0.0, theme_name);
        self.desktop_lyrics.update_lyrics("\n\n", theme_name);
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

    pub fn set_recommand(&mut self, recommand: bool) {
        self.recommand = recommand;
        self.start_recommendations_if_enabled();
    }

    pub fn get_recommand(&self) -> bool {
        self.recommand
    }

    /// 设置 GitHub Token（空字符串或默认 Token 均视为使用内置默认值，不写入配置文件）
    pub fn set_lyrics_position(&mut self, position: String) {
        self.desktop_lyrics
            .set_position(DesktopLyricsPosition::from_config_key(&position));
    }

    pub fn set_lyrics_alpha(&mut self, alpha: u8) {
        self.desktop_lyrics.set_alpha(alpha);
    }

    pub fn set_lyrics_coords(&mut self, x: i32, y: i32) {
        self.desktop_lyrics.set_coords(x, y);
    }

    pub fn set_lyrics_scroll(&mut self, mode: crate::desktop_lyrics::DesktopLyricsScrollMode) {
        self.desktop_lyrics.set_scroll_mode(mode);
    }

    pub fn open_desktop_lyrics(&mut self, theme_name: &str) {
        self.desktop_lyrics.open(theme_name);
        if self.desktop_lyrics.is_active() {
            self.push_current_lyrics_to_desktop();
        }
    }

    pub fn is_lyrics_active(&self) -> bool {
        self.desktop_lyrics.is_active()
    }

    /// 获取桌面歌词位置配置键
    pub fn get_lyrics_position_key(&self) -> String {
        self.desktop_lyrics.position().config_key().to_string()
    }

    pub fn get_lyrics_alpha(&self) -> u8 {
        self.desktop_lyrics.alpha()
    }

    pub fn get_lyrics_scroll(&self) -> String {
        self.desktop_lyrics.scroll_mode().config_key().to_string()
    }

    pub fn get_lyrics_x(&self) -> i32 {
        self.desktop_lyrics.get_position_coords().0
    }

    pub fn get_lyrics_y(&self) -> i32 {
        self.desktop_lyrics.get_position_coords().1
    }

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
            recommand: self.recommand,
            lyrics_visible: self.desktop_lyrics.is_active(),
            lyrics_position: self.get_lyrics_position_key(),
            lyrics_alpha: self.get_lyrics_alpha(),
            lyrics_x: self.get_lyrics_x(),
            lyrics_y: self.get_lyrics_y(),
            lyrics_scroll: self.get_lyrics_scroll(),
        };

        new_config.save();
    }

    /// 运行事件循环
    pub fn run(&mut self) -> io::Result<()> {
        // 初始化终端（使用 RAII 保护）
        let _guard = Self::init_terminal()?;
        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend)?;

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
        Self::restore_mouse_capture();
        self.draw(&mut terminal)?;

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
                self.draw(&mut terminal)?;
            }

            // 检查网络搜索结果
            if self.online_searching {
                self.check_online_search_result();
            }

            self.check_recommendation_result();

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
                || self.comments_loading
                || self.recommendations_loading
                || self.recommendation_search_rx.is_some())
                && now.duration_since(last_progress_update) >= progress_update_interval
            {
                self.push_current_lyrics_to_desktop();
                self.draw(&mut terminal)?;
                last_progress_update = now;
            }

            // 轮询桌面歌词窗口事件
            if let Some(ev) = self.desktop_lyrics.try_recv_event() {
                match ev {
                    crate::desktop_lyrics::DesktopLyricsEvent::PositionChanged { x, y } => {
                        self.desktop_lyrics.move_window(x, y);
                        self.save_config_now();
                    }
                    crate::desktop_lyrics::DesktopLyricsEvent::KeyPress { key } => {
                        self.handle_desktop_key(&key);
                    }
                    crate::desktop_lyrics::DesktopLyricsEvent::ScrollModeChanged {
                        scroll_mode,
                    } => {
                        self.desktop_lyrics.set_scroll_mode(scroll_mode);
                        self.save_config_now();
                    }
                }
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
                                self.draw(&mut terminal)?;
                            }
                        }
                    }
                    Event::Mouse(mouse_event) => {
                        self.handle_mouse_event(mouse_event)?;
                        self.draw(&mut terminal)?;
                    }
                    Event::Resize(_, _) => {
                        // 窗口大小改变时立即重绘，无论播放状态如何
                        self.draw(&mut terminal)?;
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

        // TerminalGuard 会在函数结束时自动恢复终端设置
        Ok(())
    }
}
