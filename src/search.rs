// 网络搜索下载模块
// 支持多平台搜索：酷我音乐 + 网易音乐 + 酷狗音乐

use chrono::{Local, TimeZone};
use ferrous_opencc::{config::BuiltinConfig, OpenCC};
use serde::Deserialize;
use serde_json::json;
use std::error::Error;
use std::path::PathBuf;
use std::sync::{mpsc, Mutex, OnceLock};

/// 写入日志文件（追加模式）
macro_rules! log_file {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        let timestamp = chrono::Local::now().format("%H:%M:%S%.3f");
        let line = format!("[{}] {}\n", timestamp, msg);
        let log_path = crate::config::get_daily_log_path();
        let _ = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)
            .and_then(|mut f| std::io::Write::write_all(&mut f, line.as_bytes()));

    }};
}

fn preview_for_log(text: &str, max_chars: usize) -> String {
    text.chars().take(max_chars).collect()
}

/// 网络搜索结果
#[derive(Debug, Clone)]
pub struct OnlineSong {
    /// 歌曲名称
    pub name: String,
    /// 歌手
    pub artist: String,
    /// 歌曲ID（不同平台ID含义不同，需配合 source 使用）
    pub id: i64,
    /// 酷狗音乐 hash（仅酷狗来源使用）
    pub hash: String,
    /// 时长（毫秒）
    pub duration_ms: Option<i64>,
    /// 来源平台
    pub source: MusicSource,
    /// 聚合搜索平台标识（仅 Juhe 来源使用，如 "kw"/"kg"/"tx"/"wy"/"mg"）
    pub juhe_platform: String,
    /// 聚合搜索歌曲ID字符串（仅 Juhe 来源使用，平台特定ID的字符串形式）
    pub juhe_song_id: String,
}

impl OnlineSong {
    pub fn unresolved_juhe_candidate(name: String, artist: String) -> Self {
        Self {
            name,
            artist,
            id: 0,
            hash: String::new(),
            duration_ms: None,
            source: MusicSource::Juhe,
            juhe_platform: String::new(),
            juhe_song_id: String::new(),
        }
    }

    pub fn is_unresolved_juhe_candidate(&self) -> bool {
        self.source == MusicSource::Juhe
            && (self.juhe_platform.is_empty() || self.juhe_song_id.is_empty())
    }
}

/// 音乐来源
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MusicSource {
    /// 酷我音乐
    Kuwo,
    /// 网易音乐
    NetEase,
    /// 酷狗音乐
    Kugou,
    /// 聚合搜索（通过独家API获取播放链接）
    Juhe,
}

/// 网络搜索下载结果
pub struct SearchDownloadResult {
    /// 搜索关键字
    #[allow(dead_code)]
    pub query: String,
    /// 搜索到的歌曲列表
    pub songs: Vec<OnlineSong>,
}

/// 歌单来源平台
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaylistSource {
    Kugou,
    Kuwo,
    NetEase,
}

/// 外部播放列表来源平台。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalPlaylistSource {
    Spotify,
    AppleMusic,
}

/// 在线列表 URL 的识别结果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OnlineListUrlKind {
    Playlist(PlaylistSource, String),
    Rank(PlaylistSource, Option<String>),
    Artist(PlaylistSource, String),
    External(ExternalPlaylistSource, String),
    Unsupported(String),
}

/// 网络歌单信息
#[derive(Debug, Clone)]
pub struct OnlinePlaylist {
    /// 歌单名称
    pub name: String,
    /// 作者/创建者
    #[allow(dead_code)]
    pub author: String,

    /// 歌单ID（平台原始ID字符串）
    pub playlist_id: String,
    /// 歌单来源平台
    pub source: PlaylistSource,
    /// 歌单歌曲数量
    pub song_count: Option<usize>,
    /// 歌单简介
    #[allow(dead_code)]
    pub description: String,

    /// 播放数
    #[allow(dead_code)]
    pub play_count: Option<usize>,
    /// 日期（创建/发布时间，yyyy-MM-dd）
    #[allow(dead_code)]
    pub date_text: Option<String>,
}

/// 歌单搜索结果
pub struct PlaylistSearchResult {
    /// 搜索关键字
    #[allow(dead_code)]
    pub query: String,
    /// 搜索到的歌单列表
    pub playlists: Vec<OnlinePlaylist>,
}

/// 歌单歌曲加载结果
#[derive(Clone)]
pub struct PlaylistSongsResult {
    /// 歌单信息
    pub playlist: OnlinePlaylist,
    /// 歌单歌曲
    pub songs: Vec<OnlineSong>,
}

struct ExternalPlaylistSongsResult {
    songs: Vec<OnlineSong>,
    title: Option<String>,
}

/// 单条评论中的“回复”信息
#[derive(Debug, Clone)]
pub struct SongCommentReply {
    /// 被回复用户昵称
    pub nickname: String,
    /// 回复内容
    pub content: String,
    /// 回复时间（优先使用接口返回的可读时间）
    #[allow(dead_code)]
    pub time_text: Option<String>,
}

/// 单条歌曲评论
#[derive(Debug, Clone)]
pub struct SongCommentItem {
    /// 评论用户昵称
    pub nickname: String,
    /// 评论内容
    pub content: String,
    /// 评论时间（优先使用接口返回的可读时间）
    pub time_text: Option<String>,
    /// 被回复信息（若存在）
    pub reply: Option<SongCommentReply>,
}

/// 歌曲评论结果
#[derive(Debug, Clone)]
pub struct SongCommentsResult {
    /// 当前页码（从1开始）
    pub page: usize,
    /// 评论总数
    pub total: usize,
    /// 当前页评论列表
    pub comments: Vec<SongCommentItem>,
}

/// AI 歌曲信息查询结果（流式）
#[derive(Debug, Clone)]
pub struct SongInfoChunk {
    /// 本次新增的内容片段
    pub delta: String,
    /// 流式是否结束
    pub done: bool,
    /// 错误信息（若失败）
    pub error: Option<String>,
}

/// DeepSeek 流式响应（SSE chunk）
#[derive(Debug, Deserialize)]
struct DeepSeekStreamChunk {
    choices: Option<Vec<DeepSeekStreamChoice>>,
    error: Option<DeepSeekErrorInfo>,
}

#[derive(Debug, Deserialize)]
struct DeepSeekStreamChoice {
    delta: Option<DeepSeekStreamDelta>,
}

#[derive(Debug, Deserialize)]
struct DeepSeekStreamDelta {
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DeepSeekErrorInfo {
    message: Option<String>,
}

/// 下载完成结果
pub struct DownloadResult {
    /// 下载的歌曲信息
    #[allow(dead_code)]
    pub song: OnlineSong,
    /// 下载后的本地文件路径
    pub local_path: Option<PathBuf>,
    /// 错误信息
    pub error: Option<String>,
}

/// 下载进度消息
pub enum DownloadProgress {
    /// 下载进度更新（百分比 0-100）
    Progress(u8),
    /// 下载完成
    Done(Box<DownloadResult>),
}

// ============================================================
// 酷我音乐 JSON 结构
// ============================================================

/// 酷我搜索结果
#[derive(Debug, Deserialize)]
struct KuwoSearchResult {
    data: Option<KuwoSearchData>,
}

#[derive(Debug, Deserialize)]
struct KuwoSearchData {
    lists: Option<Vec<KuwoSong>>,
}

#[derive(Debug, Deserialize)]
struct KuwoSong {
    /// 歌曲RID
    rid: i64,
    /// 歌曲名
    name: String,
    /// 歌手
    artist: Option<String>,
    /// 时长（秒字符串，如 "245"）
    duration: Option<String>,
}

/// 酷我播放链接结果
#[derive(Debug, Deserialize)]
struct KuwoPlayResult {
    url: Option<String>,
}

// ============================================================
// 网易音乐 JSON 结构（备用）
// ============================================================

#[derive(Debug, Deserialize)]
struct NetEaseSearchResult {
    result: Option<NetEaseResult>,
}

#[derive(Debug, Deserialize)]
struct NetEaseResult {
    songs: Option<Vec<NetEaseSong>>,
}

#[derive(Debug, Deserialize)]
struct NetEaseSong {
    id: i64,
    name: String,
    artists: Option<Vec<NetEaseArtist>>,
    duration: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct NetEaseArtist {
    name: String,
}

#[derive(Debug, Deserialize)]
struct NetEaseSongDetail {
    data: Option<Vec<NetEaseSongData>>,
}

#[derive(Debug, Deserialize)]
struct NetEaseSongData {
    url: Option<String>,
    code: Option<i64>,
}

// ============================================================
// 酷狗音乐 JSON 结构
// ============================================================

/// 酷狗搜索结果
#[derive(Debug, Deserialize)]
struct KugouSearchResult {
    data: Option<KugouSearchData>,
}

#[derive(Debug, Deserialize)]
struct KugouSearchData {
    info: Option<Vec<KugouSong>>,
}

#[derive(Debug, Deserialize)]
struct KugouSong {
    /// 歌曲hash
    hash: Option<String>,
    /// 歌曲名
    songname: Option<String>,
    /// 歌手名
    singername: Option<String>,
    /// 时长（秒，可能是字符串或整数）
    duration: Option<StringOrInt>,
    /// 付费类型：0=免费，3=付费
    #[allow(dead_code)]
    pay_type: Option<i64>,
}

/// 兼容字符串和整数类型的反序列化辅助
#[derive(Debug)]
enum StringOrInt {
    S(String),
    I(i64),
}

impl<'de> serde::Deserialize<'de> for StringOrInt {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct StringOrIntVisitor;
        impl<'de> serde::de::Visitor<'de> for StringOrIntVisitor {
            type Value = StringOrInt;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a string or an integer")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<StringOrInt, E> {
                Ok(StringOrInt::S(v.to_string()))
            }
            fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<StringOrInt, E> {
                Ok(StringOrInt::I(v))
            }
            fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<StringOrInt, E> {
                Ok(StringOrInt::I(v as i64))
            }
        }
        deserializer.deserialize_any(StringOrIntVisitor)
    }
}

impl StringOrInt {
    fn to_seconds(&self) -> Option<i64> {
        match self {
            StringOrInt::S(s) => s.parse::<i64>().ok(),
            StringOrInt::I(n) => Some(*n),
        }
    }
}

// ============================================================
// HTTP 客户端
// ============================================================

/// 创建 HTTP 客户端（搜索用）
fn create_search_client() -> Option<reqwest::blocking::Client> {
    reqwest::blocking::Client::builder()
        .no_proxy()
        .timeout(std::time::Duration::from_secs(10))
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .build()
        .ok()
}

fn create_external_page_client() -> Option<reqwest::blocking::Client> {
    reqwest::blocking::Client::builder()
        .no_proxy()
        .timeout(std::time::Duration::from_secs(30))
        .connect_timeout(std::time::Duration::from_secs(15))
        .redirect(reqwest::redirect::Policy::limited(10))
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .build()
        .ok()
}

fn create_spotify_client() -> Option<reqwest::blocking::Client> {
    build_http_client_with_proxy(
        reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .connect_timeout(std::time::Duration::from_secs(15))
            .redirect(reqwest::redirect::Policy::limited(10))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"),
    )
    .build()
    .ok()
}

/// 创建 HTTP 客户端（下载用）
fn create_download_client() -> Option<reqwest::blocking::Client> {
    reqwest::blocking::Client::builder()
        .no_proxy()
        .timeout(std::time::Duration::from_secs(60))
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .build()
        .ok()
}

fn build_http_client_with_proxy(
    mut builder: reqwest::blocking::ClientBuilder,
) -> reqwest::blocking::ClientBuilder {
    builder = builder.no_proxy();
    if let Some(proxy_url) = configured_proxy_url() {
        match reqwest::Proxy::all(&proxy_url) {
            Ok(proxy) => {
                log_file!("[HTTP] 使用代理: {}", proxy_url);
                builder = builder.proxy(proxy);
            }
            Err(e) => {
                log_file!("[HTTP] 代理配置无效({}): {}", proxy_url, e);
            }
        }
    }
    builder
}

fn configured_proxy_url() -> Option<String> {
    for key in [
        "SPOTIFY_PROXY",
        "HTTPS_PROXY",
        "https_proxy",
        "HTTP_PROXY",
        "http_proxy",
        "ALL_PROXY",
        "all_proxy",
    ] {
        if let Ok(value) = std::env::var(key) {
            let mut value = value.trim().to_string();
            if !value.is_empty() {
                if value.starts_with("https://127.0.0.1:")
                    || value.starts_with("https://localhost:")
                    || value.starts_with("https://[::1]:")
                {
                    let corrected = format!("http://{}", value.trim_start_matches("https://"));
                    log_file!(
                        "[HTTP] 本地代理通常使用 http:// 协议，已将 {} 自动修正为 {}",
                        value,
                        corrected
                    );
                    value = corrected;
                }
                return Some(value);
            }
        }
    }
    None
}

fn reqwest_error_detail(e: &reqwest::Error) -> String {
    let mut parts = vec![e.to_string()];
    let mut source = e.source();
    while let Some(err) = source {
        parts.push(err.to_string());
        source = err.source();
    }
    parts.join(" | caused by: ")
}

/// 将网易毫秒时间戳转换为本地日期时间文本
fn format_datetime_from_millis(ms: i64) -> Option<String> {
    Local
        .timestamp_millis_opt(ms)
        .single()
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
}

/// 将秒/毫秒时间戳转换为日期文本（yyyy-MM-dd）
fn format_date_from_timestamp(ts: i64) -> Option<String> {
    let secs = if ts >= 1_000_000_000_000 {
        ts / 1000
    } else {
        ts
    };
    Local
        .timestamp_opt(secs, 0)
        .single()
        .map(|dt| dt.format("%Y-%m-%d").to_string())
}

/// 从 JSON 条目中提取日期字段并格式化为 yyyy-MM-dd
fn pick_date_field(v: &serde_json::Value, keys: &[&str]) -> Option<String> {
    for k in keys {
        if let Some(n) = v.get(*k).and_then(|x| x.as_i64()) {
            if let Some(d) = format_date_from_timestamp(n) {
                return Some(d);
            }
        }
        if let Some(n) = v.get(*k).and_then(|x| x.as_u64()) {
            if let Some(d) = format_date_from_timestamp(n as i64) {
                return Some(d);
            }
        }
        if let Some(s) = v.get(*k).and_then(|x| x.as_str()) {
            let t = s.trim();
            if t.is_empty() {
                continue;
            }
            if let Ok(n) = t.parse::<i64>() {
                if let Some(d) = format_date_from_timestamp(n) {
                    return Some(d);
                }
            }
            if t.len() >= 10 {
                let prefix = &t[..10];
                if prefix.chars().nth(4) == Some('-') && prefix.chars().nth(7) == Some('-') {
                    return Some(prefix.to_string());
                }
            }
        }
    }
    None
}

// ============================================================
// 公共接口
// ============================================================

/// 在后台线程中搜索网络歌曲
pub fn search_online_background(
    query: String,
    page: usize,
) -> mpsc::Receiver<SearchDownloadResult> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let result = search_online(&query, page);
        let _ = tx.send(result);
    });
    rx
}

/// 在后台线程中下载歌曲（带进度回调）
pub fn download_song_background(
    song: OnlineSong,
    save_dir: PathBuf,
) -> mpsc::Receiver<DownloadProgress> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let result = download_song_with_progress(&song, &save_dir, |percent| {
            let _ = tx.send(DownloadProgress::Progress(percent));
        });
        let _ = tx.send(DownloadProgress::Done(Box::new(result)));
    });
    rx
}

/// 在后台线程中搜索歌单（合并酷狗 + 酷我 + 网易所有平台结果）
pub fn search_playlist_background(
    query: String,
    page: usize,
) -> mpsc::Receiver<PlaylistSearchResult> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let result = search_playlist(&query, page);
        let _ = tx.send(result);
    });
    rx
}

/// 在后台线程中加载歌单歌曲
pub fn fetch_playlist_songs_background(
    playlist: OnlinePlaylist,
) -> mpsc::Receiver<PlaylistSongsResult> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let songs = fetch_playlist_songs(&playlist);
        let _ = tx.send(PlaylistSongsResult { playlist, songs });
    });
    rx
}

/// 在后台线程中按页解析在线列表 URL 并加载歌曲。
pub fn fetch_online_list_url_page_background(
    input: String,
    page: usize,
    page_size: usize,
) -> mpsc::Receiver<PlaylistSongsResult> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        log_file!(
            "[ListUrl][BG] 开始加载: input={}, page={}, page_size={}",
            input,
            page,
            page_size
        );
        let result = fetch_online_list_url_with_page(&input, page, page_size);
        let _ = tx.send(result);
    });
    rx
}

/// 在后台线程中获取歌曲评论（基于歌曲名搜索网易）
pub fn fetch_song_comments_background(
    query: String,
    page: usize,
    page_size: usize,
) -> mpsc::Receiver<SongCommentsResult> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let result = fetch_song_comments(&query, page, page_size);
        let _ = tx.send(result);
    });
    rx
}

/// AI 查询配置
pub struct AiQueryConfig {
    /// API 接口地址（如 https://api.deepseek.com/）
    pub api_base_url: String,
    /// API Key
    pub api_key: String,
    /// 模型名称
    pub api_model: String,
}

/// 在后台线程中查询歌曲详细信息（支持自定义 OpenAI 兼容接口 / OpenRouter 免费模型兜底）- 流式输出
pub fn fetch_song_info_streaming(
    prompt: String,
    config: AiQueryConfig,
) -> mpsc::Receiver<SongInfoChunk> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        // 确定最终的 API URL、模型、认证头
        // 如果用户未设置 API Key，则使用 OpenRouter 免费模型兜底
        let (api_url, model, auth_header) = if !config.api_key.trim().is_empty() {
            // 用户配置了 API Key，使用自定义接口
            let base = config.api_base_url.trim().trim_end_matches('/');
            let url = format!("{}/chat/completions", base);
            (
                url,
                config.api_model.trim().to_string(),
                format!("Bearer {}", config.api_key.trim()),
            )
        } else {
            // 无 API Key，使用内置 OpenRouter 免费模型兜底
            (
                "https://openrouter.ai/api/v1/chat/completions".to_string(),
                "minimax/minimax-m2.5:free".to_string(),
                "Bearer sk-xxxxxx".to_string(),
            )
        };

        let client = match reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .user_agent("TerMusicRust/2.0.0")
            .build()
        {
            Ok(c) => c,
            Err(e) => {
                let _ = tx.send(SongInfoChunk {
                    delta: String::new(),
                    done: true,
                    error: Some(
                        crate::langs::global_texts()
                            .fmt_http_client_failed
                            .replace("{}", &e.to_string()),
                    ),
                });
                return;
            }
        };

        let request_builder = client
            .post(&api_url)
            .header("Authorization", &auth_header)
            .json(&json!({
                "model": model,
                "messages": [
                    {
                        "role": "user",
                        "content": prompt
                    }
                ],
                "temperature": 0.2,
                "stream": true
            }));

        let response = match request_builder.send() {
            Ok(r) => r,
            Err(e) => {
                let _ = tx.send(SongInfoChunk {
                    delta: String::new(),
                    done: true,
                    error: Some(
                        crate::langs::global_texts()
                            .fmt_api_request_failed
                            .replace("{}", &e.to_string()),
                    ),
                });
                return;
            }
        };

        let status = response.status();
        if !status.is_success() {
            let raw_text = response.text().unwrap_or_default();
            let msg = serde_json::from_str::<DeepSeekStreamChunk>(&raw_text)
                .ok()
                .and_then(|v| v.error)
                .and_then(|e| e.message)
                .filter(|s| !s.trim().is_empty())
                .unwrap_or_else(|| format!("HTTP {}", status.as_u16()));
            let _ = tx.send(SongInfoChunk {
                delta: String::new(),
                done: true,
                error: Some(
                    crate::langs::global_texts()
                        .fmt_api_request_error
                        .replace("{}", &msg),
                ),
            });
            return;
        }

        // 读取 SSE 流
        use std::io::{BufRead, BufReader};
        let reader = BufReader::new(response);
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    let line = line.trim().to_string();
                    if line.is_empty() {
                        continue;
                    }
                    if line == "data: [DONE]" {
                        let _ = tx.send(SongInfoChunk {
                            delta: String::new(),
                            done: true,
                            error: None,
                        });
                        return;
                    }
                    if let Some(data) = line.strip_prefix("data: ") {
                        if let Ok(chunk) = serde_json::from_str::<DeepSeekStreamChunk>(data) {
                            if let Some(error) = chunk.error {
                                let _ = tx.send(SongInfoChunk {
                                    delta: String::new(),
                                    done: true,
                                    error: Some(error.message.unwrap_or_default()),
                                });
                                return;
                            }
                            if let Some(choices) = chunk.choices {
                                if let Some(choice) = choices.into_iter().next() {
                                    if let Some(delta) = choice.delta {
                                        if let Some(content) = delta.content {
                                            if !content.is_empty() {
                                                let _ = tx.send(SongInfoChunk {
                                                    delta: content,
                                                    done: false,
                                                    error: None,
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(_) => {
                    let _ = tx.send(SongInfoChunk {
                        delta: String::new(),
                        done: true,
                        error: Some(crate::langs::global_texts().stream_read_failed.to_string()),
                    });
                    return;
                }
            }
        }

        // 流意外结束（未收到 [DONE]）
        let _ = tx.send(SongInfoChunk {
            delta: String::new(),
            done: true,
            error: None,
        });
    });
    rx
}

// ============================================================
// 搜索逻辑：酷我优先，网易备用
// ============================================================

fn kuwo_auth_cookie_and_csrf(client: &reqwest::blocking::Client) -> (String, String) {
    let mut token = "1ABCDEF0".to_string();

    if let Ok(resp) = client
        .get("https://www.kuwo.cn/search/list?key=%E9%9F%B3%E4%B9%90")
        .header("Referer", "https://www.kuwo.cn/")
        .send()
    {
        for v in resp.headers().get_all(reqwest::header::SET_COOKIE).iter() {
            if let Ok(s) = v.to_str() {
                for part in s.split(';') {
                    let part = part.trim();
                    if let Some(t) = part.strip_prefix("kw_token=") {
                        if !t.is_empty() {
                            token = t.to_string();
                            break;
                        }
                    }
                }
            }
        }
    }

    // 附带常见的统计 Cookie，模拟浏览器，提升通过率
    let now = chrono::Local::now().timestamp();
    let cookie = format!(
        "kw_token={}; Hm_lvt_cdb524f42f0ce19b169a8071123a4797={}; Hm_lpvt_cdb524f42f0ce19b169a8071123a4797={};",
        token, now, now
    );

    (cookie, token)
}

fn kuwo_req_id() -> String {
    let now_millis = chrono::Local::now().timestamp_millis() as u128;
    let seed = format!("{}-ter-music", now_millis);
    let digest = format!("{:x}", md5::compute(seed.as_bytes()));
    format!(
        "{}-{}-{}-{}-{}",
        &digest[0..8],
        &digest[8..12],
        &digest[12..16],
        &digest[16..20],
        &digest[20..32]
    )
}

/// 搜索网络歌曲（同步）
fn search_online(query: &str, page: usize) -> SearchDownloadResult {
    let client = match create_search_client() {
        Some(c) => c,
        None => {
            return SearchDownloadResult {
                query: query.to_string(),
                songs: Vec::new(),
            }
        }
    };

    // 优先使用酷我音乐搜索
    if let Some(songs) = search_kuwo(&client, query, page) {
        if !songs.is_empty() {
            return SearchDownloadResult {
                query: query.to_string(),
                songs,
            };
        }
    }

    // 酷我无结果，尝试酷狗音乐
    if let Some(songs) = search_kugou(&client, query, page) {
        if !songs.is_empty() {
            return SearchDownloadResult {
                query: query.to_string(),
                songs,
            };
        }
    }

    // 酷狗无结果，尝试网易音乐
    if let Some(songs) = search_netease(&client, query, page) {
        if !songs.is_empty() {
            return SearchDownloadResult {
                query: query.to_string(),
                songs,
            };
        }
    }

    SearchDownloadResult {
        query: query.to_string(),
        songs: Vec::new(),
    }
}

/// 酷我音乐搜索
fn search_kuwo(
    client: &reqwest::blocking::Client,
    query: &str,
    page: usize,
) -> Option<Vec<OnlineSong>> {
    let search_url = format!(
        "https://www.kuwo.cn/api/www/search/searchMusicBykeyWord?key={}&pn={}&rn=20&httpsStatus=1",
        urlencoding::encode(query),
        page
    );

    let response = client.get(&search_url)
        .header("Referer", "https://www.kuwo.cn/search/list")
        .header("Cookie", "kw_token=1ABCDEF0; Hm_lvt_cdb524f42f0ce19b169a8071123a4797=1700000000; Hm_lpvt_cdb524f42f0ce19b169a8071123a4797=1700000000;")
        .send()
        .ok()?;

    let text = response.text().ok()?;
    let search_result: KuwoSearchResult = serde_json::from_str(&text).ok()?;

    let data = search_result.data?;
    let lists = data.lists?;

    Some(
        lists
            .into_iter()
            .map(|s| {
                let duration_ms = s
                    .duration
                    .and_then(|d| d.parse::<i64>().ok())
                    .map(|secs| secs * 1000);
                OnlineSong {
                    name: s.name,
                    artist: s.artist.unwrap_or_default(),
                    id: s.rid,
                    hash: String::new(),
                    duration_ms,
                    source: MusicSource::Kuwo,
                    juhe_platform: String::new(),
                    juhe_song_id: String::new(),
                }
            })
            .collect(),
    )
}

/// 网易音乐搜索（备用）
fn search_netease(
    client: &reqwest::blocking::Client,
    query: &str,
    page: usize,
) -> Option<Vec<OnlineSong>> {
    let offset = (page.saturating_sub(1)) * 20;
    let search_url = format!(
        "https://music.163.com/api/search/get?s={}&type=1&limit=20&offset={}",
        urlencoding::encode(query),
        offset
    );

    let response = client
        .get(&search_url)
        .header("Referer", "https://music.163.com/")
        .header("Cookie", "MUSIC_U=; appver=2.0.2;")
        .send()
        .ok()?;

    let text = response.text().ok()?;
    let search_result: NetEaseSearchResult = serde_json::from_str(&text).ok()?;

    let result = search_result.result?;
    let songs = result.songs?;

    Some(
        songs
            .into_iter()
            .map(|s| {
                let artist = s
                    .artists
                    .map(|a| {
                        a.iter()
                            .map(|ar| ar.name.as_str())
                            .collect::<Vec<&str>>()
                            .join(", ")
                    })
                    .unwrap_or_default();
                OnlineSong {
                    name: s.name,
                    artist,
                    id: s.id,
                    hash: String::new(),
                    duration_ms: s.duration,
                    source: MusicSource::NetEase,
                    juhe_platform: String::new(),
                    juhe_song_id: String::new(),
                }
            })
            .collect(),
    )
}

/// 酷狗音乐搜索
fn search_kugou(
    client: &reqwest::blocking::Client,
    query: &str,
    page: usize,
) -> Option<Vec<OnlineSong>> {
    let search_url = format!(
        "http://mobilecdn.kugou.com/api/v3/search/song?format=json&keyword={}&page={}&pagesize=20",
        urlencoding::encode(query),
        page
    );
    log_file!("[Kugou] 请求URL: {}", search_url);

    let response = match client.get(&search_url).send() {
        Ok(r) => r,
        Err(e) => {
            log_file!("[Kugou] HTTP请求失败: {}", e);
            return None;
        }
    };

    let text = match response.text() {
        Ok(t) => t,
        Err(e) => {
            log_file!("[Kugou] 读取响应失败: {}", e);
            return None;
        }
    };
    log_file!("[Kugou] 响应长度: {} 字节", text.len());
    log_file!("[Kugou] 响应前200字符: {}", preview_for_log(&text, 200));

    let search_result: KugouSearchResult = match serde_json::from_str(&text) {
        Ok(r) => r,
        Err(e) => {
            log_file!("[Kugou] JSON解析失败: {}", e);
            return None;
        }
    };

    let data = match search_result.data {
        Some(d) => d,
        None => {
            log_file!("[Kugou] data字段为None");
            return None;
        }
    };

    let info = match data.info {
        Some(i) => i,
        None => {
            log_file!("[Kugou] info字段为None");
            return None;
        }
    };

    log_file!("[Kugou] 解析到 {} 首歌", info.len());

    Some(
        info.into_iter()
            .filter_map(|s| {
                let hash = s.hash.unwrap_or_default();
                if hash.is_empty() {
                    return None;
                }
                let name = s.songname.unwrap_or_default();
                let artist = s.singername.unwrap_or_default();
                let duration_ms = s
                    .duration
                    .and_then(|d| d.to_seconds())
                    .map(|secs| secs * 1000);
                Some(OnlineSong {
                    name,
                    artist,
                    id: 0,
                    hash,
                    duration_ms,
                    source: MusicSource::Kugou,
                    juhe_platform: String::new(),
                    juhe_song_id: String::new(),
                })
            })
            .collect(),
    )
}

/// 从 JSON 条目中提取字符串字段（按候选字段名）
fn pick_str_field(v: &serde_json::Value, keys: &[&str]) -> Option<String> {
    for k in keys {
        if let Some(s) = v.get(*k).and_then(|x| x.as_str()) {
            let t = s.trim();
            if !t.is_empty() {
                return Some(t.to_string());
            }
        }
        if let Some(n) = v.get(*k).and_then(|x| x.as_i64()) {
            return Some(n.to_string());
        }
        if let Some(n) = v.get(*k).and_then(|x| x.as_u64()) {
            return Some(n.to_string());
        }
    }
    None
}

/// 从 JSON 条目中提取整数字段（支持纯数字字符串）
fn pick_usize_field(v: &serde_json::Value, keys: &[&str]) -> Option<usize> {
    for k in keys {
        if let Some(n) = v.get(*k).and_then(|x| x.as_u64()) {
            return Some(n as usize);
        }
        if let Some(n) = v.get(*k).and_then(|x| x.as_i64()) {
            if n >= 0 {
                return Some(n as usize);
            }
        }
        if let Some(s) = v.get(*k).and_then(|x| x.as_str()) {
            let normalized = s.trim().replace([',', '_'], "");
            if let Ok(n) = normalized.parse::<usize>() {
                return Some(n);
            }
        }
    }
    None
}

/// 从 JSON 条目中提取歌曲数量
fn pick_song_count(v: &serde_json::Value) -> Option<usize> {
    let keys = [
        "song_count",
        "songCount",
        "songcount",
        "trackCount",
        "track_count",
        "total",
        "count",
        "num",
        "musiccount",
        "songnum",
        "total_song",
    ];
    pick_usize_field(v, &keys)
}

/// 将 JSON 条目解析为歌单信息
#[allow(dead_code)]
fn parse_playlist_item(v: &serde_json::Value, source: PlaylistSource) -> Option<OnlinePlaylist> {
    let playlist_id = pick_str_field(
        v,
        &["id", "rid", "dissid", "list_id", "playlist_id", "specialid"],
    )?;
    let name = pick_str_field(v, &["name", "title", "listname", "dissname", "specialname"])?;
    let author = pick_str_field(
        v,
        &[
            "author",
            "creator",
            "nickname",
            "nickName",
            "uname",
            "username",
            "singername",
        ],
    )
    .unwrap_or_default();
    let description = pick_str_field(v, &["description", "intro", "desc"]).unwrap_or_default();
    let song_count = pick_song_count(v);

    Some(OnlinePlaylist {
        name,
        author,
        playlist_id,
        source,
        song_count,
        description,
        play_count: None,
        date_text: pick_date_field(
            v,
            &[
                "createTime",
                "create_time",
                "publishTime",
                "publish_time",
                "publish",
                "ctime",
            ],
        ),
    })
}

/// 在任意 JSON 中提取数组（优先常见路径）
fn extract_first_array(root: &serde_json::Value) -> Option<&Vec<serde_json::Value>> {
    let candidates = [
        vec!["data", "list"],
        vec!["data", "lists"],
        vec!["data", "playlists"],
        vec!["data", "songList"],
        vec!["result", "playlists"],
        vec!["result", "list"],
        vec!["playlist"],
        vec!["playlists"],
        vec!["list"],
    ];
    for path in candidates {
        let mut cur = root;
        let mut ok = true;
        for p in path {
            if let Some(next) = cur.get(p) {
                cur = next;
            } else {
                ok = false;
                break;
            }
        }
        if ok {
            if let Some(arr) = cur.as_array() {
                return Some(arr);
            }
        }
    }

    fn dfs_find_array(v: &serde_json::Value) -> Option<&Vec<serde_json::Value>> {
        if let Some(arr) = v.as_array() {
            if !arr.is_empty() {
                return Some(arr);
            }
        }
        if let Some(obj) = v.as_object() {
            for (_, child) in obj {
                if let Some(arr) = dfs_find_array(child) {
                    return Some(arr);
                }
            }
        }
        None
    }

    dfs_find_array(root)
}

/// 解析酷我旧接口返回的“单引号 JSON”
fn parse_kuwo_legacy_value(text: &str) -> Option<serde_json::Value> {
    let raw = text.trim().trim_start_matches('\u{feff}');
    serde_json::from_str::<serde_json::Value>(raw)
        .ok()
        .or_else(|| {
            // 旧接口返回单引号 JSON，简单转为双引号后再解析
            let normalized = raw.replace('\'', "\"");
            serde_json::from_str::<serde_json::Value>(&normalized).ok()
        })
}

/// 酷我歌单搜索（旧接口兜底，规避新接口风控）
fn search_kuwo_playlists_legacy(
    client: &reqwest::blocking::Client,
    query: &str,
    page: usize,
) -> Vec<OnlinePlaylist> {
    let pn = page.saturating_sub(1);
    let url = format!(
        "http://search.kuwo.cn/r.s?all={}&ft=playlist&itemset=web_2013&client=kt&pn={}&rn=20&rformat=json&encoding=utf8",
        urlencoding::encode(query),
        pn
    );
    log_file!("[Playlist][KW][Legacy] 搜索URL: {}", url);

    let text = match client
        .get(&url)
        .header("Referer", "http://search.kuwo.cn/")
        .send()
        .and_then(|r| r.text())
    {
        Ok(t) => t,
        Err(e) => {
            log_file!("[Playlist][KW][Legacy] 请求失败: {}", e);
            return Vec::new();
        }
    };

    log_file!(
        "[Playlist][KW][Legacy] 响应(前200): {}",
        preview_for_log(&text, 200)
    );

    let v = match parse_kuwo_legacy_value(&text) {
        Some(v) => v,
        None => {
            log_file!("[Playlist][KW][Legacy] JSON解析失败");
            return Vec::new();
        }
    };

    let arr = v
        .get("abslist")
        .and_then(|x| x.as_array())
        .cloned()
        .unwrap_or_default();

    arr.into_iter()
        .filter_map(|item| {
            let playlist_id = pick_str_field(&item, &["playlistid", "DC_TARGETID", "id"])?;
            let name = pick_str_field(&item, &["name", "title"])?;
            let author =
                pick_str_field(&item, &["nickname", "uname", "author"]).unwrap_or_default();
            let song_count =
                pick_usize_field(&item, &["songnum", "song_count", "songCount", "total"]);
            let description =
                pick_str_field(&item, &["intro", "description", "desc"]).unwrap_or_default();
            let play_count = pick_usize_field(
                &item,
                &[
                    "playcnt",
                    "play_count",
                    "playCount",
                    "listencount",
                    "listen_count",
                ],
            );

            Some(OnlinePlaylist {
                name,
                author,
                playlist_id,
                source: PlaylistSource::Kuwo,
                song_count,
                description,
                play_count,
                date_text: None,
            })
        })
        .collect()
}

/// 酷狗歌单搜索
fn search_kugou_playlists(
    client: &reqwest::blocking::Client,
    query: &str,
    page: usize,
) -> Vec<OnlinePlaylist> {
    let url = format!(
        "http://mobilecdn.kugou.com/api/v3/search/special?format=json&keyword={}&page={}&pagesize=20",
        urlencoding::encode(query),
        page.max(1)
    );
    log_file!("[Playlist][KG] 搜索URL: {}", url);

    let text = match client.get(&url).send().and_then(|r| r.text()) {
        Ok(t) => t,
        Err(e) => {
            log_file!("[Playlist][KG] 请求失败: {}", e);
            return Vec::new();
        }
    };

    let v: serde_json::Value = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(e) => {
            log_file!("[Playlist][KG] JSON解析失败: {}", e);
            return Vec::new();
        }
    };

    let arr = v
        .get("data")
        .and_then(|d| d.get("info"))
        .and_then(|x| x.as_array())
        .cloned()
        .unwrap_or_default();

    arr.into_iter()
        .filter_map(|item| {
            let playlist_id = pick_str_field(&item, &["specialid", "id"])?;
            if playlist_id == "0" {
                // 酷狗搜索会返回 specialid=0 的占位歌单，详情接口无法拉取歌曲，直接过滤
                return None;
            }
            let name = pick_str_field(&item, &["specialname", "name", "title"])?;

            let author =
                pick_str_field(&item, &["nickname", "author", "username"]).unwrap_or_default();
            let song_count = pick_song_count(&item);
            let description =
                pick_str_field(&item, &["intro", "description", "desc"]).unwrap_or_default();
            let play_count = pick_usize_field(
                &item,
                &[
                    "play_count",
                    "playCount",
                    "playcount",
                    "total_play",
                    "totalPlay",
                    "listencount",
                    "listen_count",
                    "pv",
                    "visit",
                ],
            );

            Some(OnlinePlaylist {
                name,
                author,
                playlist_id,
                source: PlaylistSource::Kugou,
                song_count,
                description,
                play_count,
                date_text: pick_date_field(
                    &item,
                    &[
                        "publish_time",
                        "publishTime",
                        "publish",
                        "createTime",
                        "create_time",
                        "ctime",
                    ],
                ),
            })
        })
        .collect()
}

/// 酷我歌单搜索
fn search_kuwo_playlists(
    client: &reqwest::blocking::Client,
    query: &str,
    page: usize,
) -> Vec<OnlinePlaylist> {
    let mut arr: Vec<serde_json::Value> = Vec::new();

    // 酷我接口页码在不同环境可能有 0 基 / 1 基差异：两种都尝试一次
    let mut pn_candidates = vec![page.saturating_sub(1), page.max(1)];
    pn_candidates.dedup();

    for pn in pn_candidates {
        let url = format!(
            "https://www.kuwo.cn/api/www/search/searchPlayListBykeyWord?key={}&pn={}&rn=20&httpsStatus=1",
            urlencoding::encode(query),
            pn
        );
        log_file!("[Playlist][KW] 搜索URL: {}", url);

        let (cookie, csrf) = kuwo_auth_cookie_and_csrf(client);
        let referer = format!(
            "https://www.kuwo.cn/search/list?key={}",
            urlencoding::encode(query)
        );
        let text = match client
            .get(&url)
            .header("Referer", &referer)
            .header("Cookie", &cookie)
            .header("csrf", &csrf)
            .send()
            .and_then(|r| r.text())
        {
            Ok(t) => t,
            Err(e) => {
                log_file!("[Playlist][KW] 请求失败(pn={}): {}", pn, e);
                continue;
            }
        };
        log_file!(
            "[Playlist][KW] 响应(前200): {}",
            preview_for_log(&text, 200)
        );

        let v: serde_json::Value = match serde_json::from_str(&text) {
            Ok(v) => v,
            Err(e) => {
                log_file!("[Playlist][KW] JSON解析失败(pn={}): {}", pn, e);
                continue;
            }
        };

        let mut candidate = v
            .get("data")
            .and_then(|d| {
                d.get("list")
                    .and_then(|x| x.as_array())
                    .or_else(|| d.get("abslist").and_then(|x| x.as_array()))
            })
            .cloned()
            .or_else(|| extract_first_array(&v).cloned())
            .unwrap_or_default();

        let illegal_blocked = v
            .get("success")
            .and_then(|x| x.as_bool())
            .map(|ok| !ok)
            .unwrap_or(false)
            && pick_str_field(&v, &["message", "msg"])
                .map(|m| m.to_lowercase().contains("illegal"))
                .unwrap_or(false);

        if illegal_blocked {
            log_file!(
                "[Playlist][KW] 命中风控(pn={})，跳过备用URL与其他页，直接走旧接口",
                pn
            );
            break;
        }

        if candidate.is_empty() {
            let alt = format!(
                "https://kuwo.cn/api/v1/www/search/searchPlayListBykeyWord?key={}&pn={}&rn=20&httpsStatus=1",
                urlencoding::encode(query), pn
            );
            log_file!("[Playlist][KW] 备用URL: {}", alt);
            if let Ok(t2) = client
                .get(&alt)
                .header("Referer", &referer)
                .header("Cookie", &cookie)
                .header("csrf", &csrf)
                .send()
                .and_then(|r| r.text())
            {
                log_file!(
                    "[Playlist][KW] 备用响应(前200): {}",
                    &t2[..t2.len().min(200)]
                );
                if let Ok(v2) = serde_json::from_str::<serde_json::Value>(&t2) {
                    candidate = v2
                        .get("data")
                        .and_then(|d| {
                            d.get("list")
                                .and_then(|x| x.as_array())
                                .or_else(|| d.get("abslist").and_then(|x| x.as_array()))
                        })
                        .cloned()
                        .or_else(|| extract_first_array(&v2).cloned())
                        .unwrap_or_default();
                }
            }
        }

        if !candidate.is_empty() {
            arr = candidate;
            break;
        }
    }

    if arr.is_empty() {
        let legacy = search_kuwo_playlists_legacy(client, query, page);
        log_file!("[Playlist][KW] 旧接口兜底返回 {} 个结果", legacy.len());
        return legacy;
    }

    arr.into_iter()
        .filter_map(|item| {
            let playlist_id = pick_str_field(&item, &["id", "listid", "playlistid", "pid"])?;
            let name = pick_str_field(&item, &["name", "title"])?;
            let author = pick_str_field(&item, &["uname", "nickname", "author", "nickName"])
                .unwrap_or_default();
            let song_count = pick_song_count(&item);
            let description =
                pick_str_field(&item, &["intro", "description", "desc"]).unwrap_or_default();
            let play_count = pick_usize_field(
                &item,
                &[
                    "play_count",
                    "playCount",
                    "playcount",
                    "listencount",
                    "listen_count",
                    "listenNum",
                ],
            );
            Some(OnlinePlaylist {
                name,
                author,
                playlist_id,
                source: PlaylistSource::Kuwo,
                song_count,
                description,
                play_count,
                date_text: pick_date_field(
                    &item,
                    &[
                        "pub",
                        "publishTime",
                        "publish_time",
                        "createTime",
                        "create_time",
                        "ctime",
                    ],
                ),
            })
        })
        .collect()
}

/// 网易歌单搜索
fn search_netease_playlists(
    client: &reqwest::blocking::Client,
    query: &str,
    page: usize,
) -> Vec<OnlinePlaylist> {
    let offset = (page.max(1) - 1) * 20;
    let url = format!(
        "https://music.163.com/api/search/get?s={}&type=1000&limit=20&offset={}",
        urlencoding::encode(query),
        offset
    );
    log_file!("[Playlist][WY] 搜索URL: {}", url);

    let text = match client
        .get(&url)
        .header("Referer", "https://music.163.com/")
        .header("Cookie", "MUSIC_U=; appver=2.0.2;")
        .send()
        .and_then(|r| r.text())
    {
        Ok(t) => t,
        Err(e) => {
            log_file!("[Playlist][WY] 请求失败: {}", e);
            return Vec::new();
        }
    };

    let v: serde_json::Value = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(e) => {
            log_file!("[Playlist][WY] JSON解析失败: {}", e);
            return Vec::new();
        }
    };

    let arr = v
        .get("result")
        .and_then(|r| r.get("playlists"))
        .and_then(|x| x.as_array())
        .cloned()
        .unwrap_or_default();

    arr.into_iter()
        .filter_map(|item| {
            let playlist_id = pick_str_field(&item, &["id"])?;
            let name = pick_str_field(&item, &["name"])?;
            let author = item
                .get("creator")
                .and_then(|c| pick_str_field(c, &["nickname", "userName", "name"]))
                .unwrap_or_default();
            let song_count = pick_song_count(&item);
            let description = pick_str_field(&item, &["description"]).unwrap_or_default();
            let play_count = item
                .get("playCount")
                .and_then(|x| x.as_u64())
                .map(|n| n as usize)
                .or_else(|| {
                    pick_str_field(&item, &["playCount", "play_count", "playcount"])
                        .and_then(|s| s.parse::<usize>().ok())
                });
            Some(OnlinePlaylist {
                name,
                author,
                playlist_id,
                source: PlaylistSource::NetEase,
                song_count,
                description,
                play_count,
                date_text: pick_date_field(&item, &["createTime", "updateTime", "publishTime"]),
            })
        })
        .collect()
}

/// 搜索歌单（合并酷狗 + 酷我 + 网易所有平台结果）
fn search_playlist(query: &str, page: usize) -> PlaylistSearchResult {
    let client = match create_search_client() {
        Some(c) => c,
        None => {
            return PlaylistSearchResult {
                query: query.to_string(),
                playlists: Vec::new(),
            }
        }
    };

    log_file!("[Playlist] 搜索开始 query='{}', page={}", query, page);
    let mut playlists = Vec::new();

    // 酷狗歌单
    let kugou = search_kugou_playlists(&client, query, page);
    log_file!("[Playlist] 酷狗返回 {} 个结果", kugou.len());
    playlists.extend(kugou);

    // 酷我歌单
    let kuwo = search_kuwo_playlists(&client, query, page);
    log_file!("[Playlist] 酷我返回 {} 个结果", kuwo.len());
    playlists.extend(kuwo);

    // 网易歌单
    let netease = search_netease_playlists(&client, query, page);
    log_file!("[Playlist] 网易返回 {} 个结果", netease.len());
    playlists.extend(netease);

    log_file!("[Playlist] 搜索结束，共 {} 个结果", playlists.len());

    PlaylistSearchResult {
        query: query.to_string(),
        playlists,
    }
}

/// 解析歌单歌曲条目为 OnlineSong（统一标记 Juhe 来源，播放下载复用 get_juhe_download_url）
fn parse_duration_to_ms(raw: &str) -> Option<i64> {
    let t = raw.trim();
    if t.is_empty() {
        return None;
    }

    if let Some((m, s)) = t.split_once(':') {
        let mm = m.trim().parse::<i64>().ok()?;
        let ss = s.trim().parse::<i64>().ok()?;
        return Some((mm * 60 + ss) * 1000);
    }

    t.parse::<i64>().ok().map(|secs_or_ms| {
        if secs_or_ms > 10_000 {
            secs_or_ms
        } else {
            secs_or_ms * 1000
        }
    })
}

fn parse_playlist_song_item(
    v: &serde_json::Value,
    platform: &str,
    source: PlaylistSource,
) -> Option<OnlineSong> {
    let mut name = pick_str_field(v, &["name", "title", "songname"]).unwrap_or_default();
    let mut artist = String::new();

    if source == PlaylistSource::Kugou {
        if name.is_empty() {
            if let Some(filename) = pick_str_field(v, &["filename"]) {
                if let Some((a, n)) = filename.split_once(" - ") {
                    artist = a.trim().to_string();
                    name = n.trim().to_string();
                } else {
                    name = filename;
                }
            }
        }
        if artist.is_empty() {
            artist = pick_str_field(v, &["singername", "artist", "author"]).unwrap_or_default();
        }
    } else if source == PlaylistSource::NetEase {
        artist = v
            .get("ar")
            .and_then(|a| a.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|x| x.get("name").and_then(|s| s.as_str()))
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| {
                pick_str_field(v, &["artist", "singer", "singername", "author"]).unwrap_or_default()
            });
    } else {
        artist =
            pick_str_field(v, &["artist", "singer", "singername", "author"]).unwrap_or_default();
    }

    if name.is_empty() {
        return None;
    }

    let song_id = match source {
        PlaylistSource::Kugou => pick_str_field(v, &["hash"])
            .or_else(|| pick_str_field(v, &["audio_id", "id", "songid", "songId"])),
        PlaylistSource::Kuwo => pick_str_field(v, &["rid", "musicrid", "id"])
            .map(|s| s.trim_start_matches("MUSIC_").to_string()),
        PlaylistSource::NetEase => pick_str_field(v, &["id", "songid", "songId"]),
    }?;

    let duration_ms = if source == PlaylistSource::NetEase {
        v.get("dt").and_then(|x| x.as_i64()).or_else(|| {
            pick_str_field(v, &["duration", "interval", "timeLength"])
                .as_deref()
                .and_then(parse_duration_to_ms)
        })
    } else {
        pick_str_field(
            v,
            &[
                "duration",
                "interval",
                "timeLength",
                "songTimeMinutes",
                "songTime",
            ],
        )
        .as_deref()
        .and_then(parse_duration_to_ms)
    };

    Some(OnlineSong {
        name,
        artist,
        id: 0,
        hash: String::new(),
        duration_ms,
        source: MusicSource::Juhe,
        juhe_platform: platform.to_string(),
        juhe_song_id: song_id,
    })
}

/// 加载酷狗歌单歌曲
fn fetch_kugou_playlist_songs(
    client: &reqwest::blocking::Client,
    playlist_id: &str,
) -> Vec<OnlineSong> {
    let url = format!(
        "http://mobilecdnbj.kugou.com/api/v3/special/song?specialid={}&page=1&pagesize=200",
        playlist_id
    );
    log_file!("[Playlist][KG] 歌单详情URL: {}", url);

    let text = match client.get(&url).send().and_then(|r| r.text()) {
        Ok(t) => t,
        Err(e) => {
            log_file!("[Playlist][KG] 歌单详情请求失败: {}", e);
            return Vec::new();
        }
    };

    let v: serde_json::Value = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(e) => {
            log_file!("[Playlist][KG] 歌单详情JSON解析失败: {}", e);
            return Vec::new();
        }
    };

    let arr = v
        .get("data")
        .and_then(|d| {
            d.get("info")
                .and_then(|x| x.as_array())
                .or_else(|| d.get("list").and_then(|x| x.as_array()))
        })
        .cloned()
        .or_else(|| extract_first_array(&v).cloned())
        .unwrap_or_default();

    if arr.is_empty() {
        let status = v.get("status").and_then(|x| x.as_i64()).unwrap_or(-1);
        let errcode = v.get("errcode").and_then(|x| x.as_i64()).unwrap_or(-1);
        log_file!(
            "[Playlist][KG] 歌单详情解析为空: specialid={}, status={}, errcode={}",
            playlist_id,
            status,
            errcode
        );
    }

    arr.iter()
        .filter_map(|x| parse_playlist_song_item(x, "kg", PlaylistSource::Kugou))
        .collect::<Vec<_>>()
}

fn decode_js_escaped_string(input: &str) -> String {
    let mut out = String::new();
    let mut chars = input.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            out.push(ch);
            continue;
        }
        match chars.next() {
            Some('u') => {
                let h1 = chars.next();
                let h2 = chars.next();
                let h3 = chars.next();
                let h4 = chars.next();
                if let (Some(a), Some(b), Some(c), Some(d)) = (h1, h2, h3, h4) {
                    let hex = format!("{}{}{}{}", a, b, c, d);
                    if let Ok(v) = u32::from_str_radix(&hex, 16) {
                        if let Some(u) = char::from_u32(v) {
                            out.push(u);
                            continue;
                        }
                    }
                    out.push('\\');
                    out.push('u');
                    out.push(a);
                    out.push(b);
                    out.push(c);
                    out.push(d);
                } else {
                    out.push('\\');
                    out.push('u');
                }
            }
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('t') => out.push('\t'),
            Some('"') => out.push('"'),
            Some('\\') => out.push('\\'),
            Some('/') => out.push('/'),
            Some(other) => {
                out.push(other);
            }
            None => out.push('\\'),
        }
    }
    out
}

fn pick_js_quoted_field(src: &str, key: &str) -> Option<String> {
    let marker = format!("{}:\"", key);
    let pos = src.find(&marker)?;
    let s = &src[pos + marker.len()..];

    let mut escaped = false;
    let mut end = 0usize;
    for (i, ch) in s.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == '"' {
            end = i;
            break;
        }
    }
    if end == 0 && !s.starts_with('"') && !s.is_empty() {
        return None;
    }
    Some(decode_js_escaped_string(&s[..end]))
}

fn pick_js_number_field(src: &str, key: &str) -> Option<i64> {
    let marker = format!("{}:", key);
    let pos = src.find(&marker)?;
    let s = &src[pos + marker.len()..];
    let mut buf = String::new();
    for ch in s.chars() {
        if ch.is_ascii_digit() || (buf.is_empty() && ch == '-') {
            buf.push(ch);
        } else {
            break;
        }
    }
    buf.parse::<i64>().ok()
}

fn fetch_kuwo_playlist_songs_legacy_api(
    client: &reqwest::blocking::Client,
    playlist_id: &str,
) -> Vec<OnlineSong> {
    fetch_kuwo_playlist_songs_legacy_api_page(client, playlist_id, 1, 100, 20)
}

fn fetch_kuwo_playlist_songs_legacy_api_page(
    client: &reqwest::blocking::Client,
    playlist_id: &str,
    page: usize,
    page_size: usize,
    max_pages: usize,
) -> Vec<OnlineSong> {
    let rn = page_size.max(1);
    let mut pn = page.saturating_sub(1);
    let mut total: Option<usize> = None;
    let mut songs = Vec::new();
    let mut seen = std::collections::HashSet::<String>::new();
    let start_pn = pn;

    while pn < start_pn.saturating_add(max_pages) {
        let url = format!(
            "http://nplserver.kuwo.cn/pl.svc?op=getlistinfo&pid={}&pn={}&rn={}&encode=utf8&keyset=pl2012&vipver=MUSIC_8.7.5.0_W4",
            playlist_id, pn, rn
        );
        log_file!("[Playlist][KW][LegacyAPI] 详情URL: {}", url);

        let text = match client
            .get(&url)
            .header("Referer", "http://www.kuwo.cn/")
            .send()
            .and_then(|r| r.text())
        {
            Ok(t) => t,
            Err(e) => {
                log_file!("[Playlist][KW][LegacyAPI] 请求失败(pn={}): {}", pn, e);
                break;
            }
        };

        let v: serde_json::Value = match serde_json::from_str(&text) {
            Ok(v) => v,
            Err(e) => {
                log_file!("[Playlist][KW][LegacyAPI] JSON解析失败(pn={}): {}", pn, e);
                break;
            }
        };

        if total.is_none() {
            total = pick_usize_field(&v, &["total", "validtotal", "TOTAL"]);
        }

        let arr = v
            .get("musiclist")
            .and_then(|x| x.as_array())
            .cloned()
            .unwrap_or_default();

        log_file!(
            "[Playlist][KW][LegacyAPI] pn={} 返回 {} 首，total={}",
            pn,
            arr.len(),
            total.unwrap_or(0)
        );

        if arr.is_empty() {
            break;
        }

        for item in arr {
            if let Some(song) = parse_playlist_song_item(&item, "kw", PlaylistSource::Kuwo) {
                if seen.insert(song.juhe_song_id.clone()) {
                    songs.push(song);
                }
            }
        }

        if let Some(t) = total {
            if songs.len() >= t {
                break;
            }
        }
        pn += 1;
    }

    log_file!("[Playlist][KW][LegacyAPI] 汇总 {} 首", songs.len());
    songs
}

fn fetch_kuwo_playlist_songs_from_page(
    client: &reqwest::blocking::Client,
    playlist_id: &str,
) -> Vec<OnlineSong> {
    let url = format!("https://www.kuwo.cn/playlist_detail/{}", playlist_id);
    log_file!("[Playlist][KW][PageFallback] 页面URL: {}", url);

    let html = match client
        .get(&url)
        .header("Referer", "https://www.kuwo.cn/")
        .send()
        .and_then(|r| r.text())
    {
        Ok(t) => t,
        Err(e) => {
            log_file!("[Playlist][KW][PageFallback] 请求失败: {}", e);
            return Vec::new();
        }
    };

    log_file!("[Playlist][KW][PageFallback] 页面长度: {}", html.len());

    let mut songs = Vec::new();
    let mut seen = std::collections::HashSet::<String>::new();
    let marker = "musicrid:\"MUSIC_";
    let mut cursor = 0usize;

    while let Some(rel) = html[cursor..].find(marker) {
        let start = cursor + rel;
        let rest = &html[start..];
        let frag_owned: String = rest.chars().take(2600).collect();
        let frag = frag_owned.as_str();

        let musicrid = pick_js_quoted_field(frag, "musicrid").unwrap_or_default();
        let rid_text = musicrid.trim_start_matches("MUSIC_").to_string();
        if rid_text.is_empty() {
            cursor = start + marker.len();
            continue;
        }

        let name = pick_js_quoted_field(frag, "name").unwrap_or_default();
        if name.trim().is_empty() {
            cursor = start + marker.len();
            continue;
        }

        if !seen.insert(rid_text.clone()) {
            cursor = start + marker.len();
            continue;
        }

        let artist = pick_js_quoted_field(frag, "artist").unwrap_or_default();
        let duration_ms = pick_js_number_field(frag, "duration").map(|secs| secs * 1000);

        songs.push(OnlineSong {
            name,
            artist,
            id: 0,
            hash: String::new(),
            duration_ms,
            source: MusicSource::Juhe,
            juhe_platform: "kw".to_string(),
            juhe_song_id: rid_text,
        });

        cursor = start + marker.len();
    }

    log_file!("[Playlist][KW][PageFallback] 解析到 {} 首歌曲", songs.len());
    songs
}

/// 加载酷我歌单歌曲
fn fetch_kuwo_playlist_songs(
    client: &reqwest::blocking::Client,
    playlist_id: &str,
) -> Vec<OnlineSong> {
    fetch_kuwo_playlist_songs_page(client, playlist_id, 1, 200)
}

fn fetch_kuwo_playlist_songs_page(
    client: &reqwest::blocking::Client,
    playlist_id: &str,
    page: usize,
    page_size: usize,
) -> Vec<OnlineSong> {
    let url = format!(
        "https://www.kuwo.cn/api/www/playlist/playListInfo?pid={}&pn={}&rn={}&httpsStatus=1",
        playlist_id, page, page_size
    );
    log_file!("[Playlist][KW] 歌单详情URL: {}", url);

    let (cookie, csrf) = kuwo_auth_cookie_and_csrf(client);
    let referer = format!("https://www.kuwo.cn/playlist_detail/{}", playlist_id);
    let text = match client
        .get(&url)
        .header("Referer", referer)
        .header("Cookie", cookie)
        .header("csrf", csrf)
        .send()
        .and_then(|r| r.text())
    {
        Ok(t) => t,
        Err(e) => {
            log_file!("[Playlist][KW] 歌单详情请求失败: {}", e);
            if page != 1 || page_size != 200 {
                return fetch_kuwo_playlist_songs_legacy_api_page(
                    client,
                    playlist_id,
                    page,
                    page_size,
                    1,
                );
            }
            let legacy = fetch_kuwo_playlist_songs_legacy_api(client, playlist_id);
            if !legacy.is_empty() {
                return legacy;
            }
            return fetch_kuwo_playlist_songs_from_page(client, playlist_id);
        }
    };

    log_file!(
        "[Playlist][KW] 歌单详情响应(前200): {}",
        preview_for_log(&text, 200)
    );

    let v: serde_json::Value = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(e) => {
            log_file!("[Playlist][KW] 歌单详情JSON解析失败: {}", e);
            if page != 1 || page_size != 200 {
                return fetch_kuwo_playlist_songs_legacy_api_page(
                    client,
                    playlist_id,
                    page,
                    page_size,
                    1,
                );
            }
            return fetch_kuwo_playlist_songs_from_page(client, playlist_id);
        }
    };

    let songs = v
        .get("data")
        .and_then(|d| d.get("musicList"))
        .and_then(|x| x.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| parse_playlist_song_item(x, "kw", PlaylistSource::Kuwo))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    if songs.is_empty() {
        if page != 1 || page_size != 200 {
            return fetch_kuwo_playlist_songs_legacy_api_page(
                client,
                playlist_id,
                page,
                page_size,
                1,
            );
        }
        let legacy = fetch_kuwo_playlist_songs_legacy_api(client, playlist_id);
        if !legacy.is_empty() {
            return legacy;
        }

        let fallback = fetch_kuwo_playlist_songs_from_page(client, playlist_id);
        log_file!("[Playlist][KW] 页面兜底返回 {} 首", fallback.len());
        return fallback;
    }

    songs
}

/// 网易：按 trackIds 批量拉取歌曲详情（避免 playlist.tracks 仅返回 10 首）
fn fetch_netease_song_detail_batch(
    client: &reqwest::blocking::Client,
    ids: &[i64],
) -> Vec<OnlineSong> {
    if ids.is_empty() {
        return Vec::new();
    }

    let c = format!(
        "[{}]",
        ids.iter()
            .map(|id| format!("{{\"id\":{}}}", id))
            .collect::<Vec<_>>()
            .join(",")
    );
    let url = format!(
        "https://music.163.com/api/v3/song/detail?c={}",
        urlencoding::encode(&c)
    );

    let text = match client
        .get(&url)
        .header("Referer", "https://music.163.com/")
        .header("Cookie", "MUSIC_U=; appver=2.0.2;")
        .send()
        .and_then(|r| r.text())
    {
        Ok(t) => t,
        Err(e) => {
            log_file!("[Playlist][WY] 批量详情请求失败: {}", e);
            return Vec::new();
        }
    };

    let v: serde_json::Value = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(e) => {
            log_file!("[Playlist][WY] 批量详情JSON解析失败: {}", e);
            return Vec::new();
        }
    };

    v.get("songs")
        .and_then(|x| x.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| parse_playlist_song_item(x, "wy", PlaylistSource::NetEase))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

/// 加载网易歌单歌曲
fn fetch_netease_playlist_songs(
    client: &reqwest::blocking::Client,
    playlist_id: &str,
) -> Vec<OnlineSong> {
    let url = format!(
        "https://music.163.com/api/v6/playlist/detail?id={}&n=1000",
        playlist_id
    );
    log_file!("[Playlist][WY] 歌单详情URL: {}", url);

    let text = match client
        .get(&url)
        .header("Referer", "https://music.163.com/")
        .header("Cookie", "MUSIC_U=; appver=2.0.2;")
        .send()
        .and_then(|r| r.text())
    {
        Ok(t) => t,
        Err(e) => {
            log_file!("[Playlist][WY] 歌单详情请求失败: {}", e);
            return Vec::new();
        }
    };

    let v: serde_json::Value = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(e) => {
            log_file!("[Playlist][WY] 歌单详情JSON解析失败: {}", e);
            return Vec::new();
        }
    };

    let tracks = v
        .get("playlist")
        .and_then(|p| p.get("tracks"))
        .and_then(|x| x.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| parse_playlist_song_item(x, "wy", PlaylistSource::NetEase))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let track_ids: Vec<i64> = v
        .get("playlist")
        .and_then(|p| p.get("trackIds"))
        .and_then(|x| x.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.get("id").and_then(|id| id.as_i64()))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    if track_ids.len() <= tracks.len() {
        return tracks;
    }

    let mut song_map = std::collections::HashMap::<String, OnlineSong>::new();
    for s in tracks {
        song_map.insert(s.juhe_song_id.clone(), s);
    }

    let mut missing = Vec::new();
    let mut seen = std::collections::HashSet::<i64>::new();
    for id in &track_ids {
        if !song_map.contains_key(&id.to_string()) && seen.insert(*id) {
            missing.push(*id);
        }
    }

    for chunk in missing.chunks(200) {
        for s in fetch_netease_song_detail_batch(client, chunk) {
            song_map.insert(s.juhe_song_id.clone(), s);
        }
    }

    let mut ordered = Vec::new();
    for id in track_ids {
        if let Some(song) = song_map.remove(&id.to_string()) {
            ordered.push(song);
        }
    }

    if ordered.is_empty() {
        song_map.into_values().collect()
    } else {
        ordered
    }
}

/// 加载歌单歌曲
fn fetch_playlist_songs(playlist: &OnlinePlaylist) -> Vec<OnlineSong> {
    let client = match create_search_client() {
        Some(c) => c,
        None => return Vec::new(),
    };

    log_file!(
        "[Playlist] 加载歌单歌曲: source={:?}, playlist_id={}, name={}",
        playlist.source,
        playlist.playlist_id,
        playlist.name
    );

    let songs = match playlist.source {
        PlaylistSource::Kugou => fetch_kugou_playlist_songs(&client, &playlist.playlist_id),
        PlaylistSource::Kuwo => fetch_kuwo_playlist_songs(&client, &playlist.playlist_id),
        PlaylistSource::NetEase => fetch_netease_playlist_songs(&client, &playlist.playlist_id),
    };

    log_file!("[Playlist] 歌单歌曲加载完成，共 {} 首", songs.len());
    songs
}

fn url_decode_lossy(input: &str) -> String {
    urlencoding::decode(input)
        .map(|s| s.into_owned())
        .unwrap_or_else(|_| input.to_string())
}

fn normalized_url_for_parse(input: &str) -> String {
    url_decode_lossy(input.trim())
        .replace("#/", "")
        .replace("#", "")
}

fn extract_query_value(input: &str, key: &str) -> Option<String> {
    let marker = format!("{}=", key);
    input
        .split(&['?', '&', '#'][..])
        .find_map(|part| part.strip_prefix(&marker))
        .map(|value| value.split('&').next().unwrap_or(value).trim().to_string())
        .filter(|value| !value.is_empty())
}

fn extract_path_segment_after(input: &str, marker: &str) -> Option<String> {
    let pos = input.find(marker)?;
    let rest = &input[pos + marker.len()..];
    rest.split(&['/', '?', '&', '#'][..])
        .next()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn extract_kugou_rank_id(input: &str) -> Option<String> {
    let pos = input.find("/yy/rank/home/")?;
    let rest = &input[pos + "/yy/rank/home/".len()..];
    let file = rest.split(&['/', '?', '&', '#'][..]).next().unwrap_or(rest);
    file.trim_end_matches(".html")
        .rsplit('-')
        .next()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn source_label(source: PlaylistSource) -> &'static str {
    match source {
        PlaylistSource::Kugou => "Kugou",
        PlaylistSource::Kuwo => "Kuwo",
        PlaylistSource::NetEase => "NetEase",
    }
}

/// 识别歌单、榜单、歌手和外部播放列表 URL。
pub fn parse_online_list_url(input: &str) -> Result<OnlineListUrlKind, String> {
    let raw = input.trim();
    if !(raw.starts_with("http://") || raw.starts_with("https://")) {
        return Err("not an http url".to_string());
    }

    let url = normalized_url_for_parse(raw);
    let lower = url.to_ascii_lowercase();

    if lower.contains("open.spotify.com/playlist/") {
        if let Some(id) = extract_path_segment_after(&url, "open.spotify.com/playlist/") {
            return Ok(OnlineListUrlKind::External(
                ExternalPlaylistSource::Spotify,
                id,
            ));
        }
    }
    if lower.contains("open.spotify.com/artist/") {
        if let Some(id) = extract_path_segment_after(&url, "open.spotify.com/artist/") {
            return Ok(OnlineListUrlKind::External(
                ExternalPlaylistSource::Spotify,
                format!("artist:{}", id),
            ));
        }
    }
    if lower.contains("open.spotify.com/album/") {
        if let Some(id) = extract_path_segment_after(&url, "open.spotify.com/album/") {
            return Ok(OnlineListUrlKind::External(
                ExternalPlaylistSource::Spotify,
                format!("album:{}", id),
            ));
        }
    }

    if lower.contains("music.apple.com/")
        && (lower.contains("/playlist/")
            || lower.contains("/room/")
            || lower.contains("/artist/")
            || lower.contains("/album/")
            || lower.contains("/new/top-charts/songs"))
    {
        return Ok(OnlineListUrlKind::External(
            ExternalPlaylistSource::AppleMusic,
            raw.to_string(),
        ));
    }

    if lower.contains("music.163.com/") {
        if lower.contains("discover/toplist") {
            if let Some(id) = extract_query_value(&url, "id") {
                return Ok(OnlineListUrlKind::Rank(PlaylistSource::NetEase, Some(id)));
            }
        }
        if lower.contains("/playlist") {
            if let Some(id) = extract_query_value(&url, "id") {
                return Ok(OnlineListUrlKind::Playlist(PlaylistSource::NetEase, id));
            }
        }
        if lower.contains("/artist") {
            if let Some(id) = extract_query_value(&url, "id") {
                return Ok(OnlineListUrlKind::Artist(PlaylistSource::NetEase, id));
            }
        }
    }

    if lower.contains("kuwo.cn/") {
        if let Some(id) = extract_path_segment_after(&url, "playlist_detail/") {
            return Ok(OnlineListUrlKind::Playlist(PlaylistSource::Kuwo, id));
        }
        if let Some(id) = extract_path_segment_after(&url, "singer_detail/") {
            return Ok(OnlineListUrlKind::Artist(PlaylistSource::Kuwo, id));
        }
        if lower.contains("ranklist") {
            let id = extract_query_value(&url, "bangId")
                .or_else(|| extract_query_value(&url, "rankId"))
                .or_else(|| extract_query_value(&url, "id"));
            return Ok(OnlineListUrlKind::Rank(PlaylistSource::Kuwo, id));
        }
    }

    if lower.contains("kugou.com/") {
        if let Some(gcid) = extract_path_segment_after(&url, "songlist/gcid_") {
            return Ok(OnlineListUrlKind::Playlist(
                PlaylistSource::Kugou,
                format!("gcid_{}", gcid.trim_start_matches("gcid_")),
            ));
        }
        if let Some(id) = extract_path_segment_after(&url, "singer/info/") {
            return Ok(OnlineListUrlKind::Artist(PlaylistSource::Kugou, id));
        }
        if lower.contains("/yy/rank/home/") {
            return Ok(OnlineListUrlKind::Rank(
                PlaylistSource::Kugou,
                extract_kugou_rank_id(&url),
            ));
        }
    }

    Ok(OnlineListUrlKind::Unsupported(raw.to_string()))
}

fn playlist_from_url_kind(kind: &OnlineListUrlKind, fallback_name: &str) -> OnlinePlaylist {
    let imported_title = title_for_online_list_url(kind);
    match kind {
        OnlineListUrlKind::Playlist(source, id) => OnlinePlaylist {
            name: imported_title.unwrap_or_else(|| format!("{} Playlist {}", source_label(*source), id)),
            author: "URL Import".to_string(),
            playlist_id: id.clone(),
            source: *source,
            song_count: None,
            description: fallback_name.to_string(),
            play_count: None,
            date_text: None,
        },
        OnlineListUrlKind::Rank(source, id) => OnlinePlaylist {
            name: imported_title.unwrap_or_else(|| {
                format!(
                    "{} Rank {}",
                    source_label(*source),
                    id.as_deref().unwrap_or("default")
                )
            }),
            author: "URL Import".to_string(),
            playlist_id: id.clone().unwrap_or_default(),
            source: *source,
            song_count: None,
            description: fallback_name.to_string(),
            play_count: None,
            date_text: None,
        },
        OnlineListUrlKind::Artist(source, id) => OnlinePlaylist {
            name: imported_title.unwrap_or_else(|| format!("{} Artist {}", source_label(*source), id)),
            author: "URL Import".to_string(),
            playlist_id: id.clone(),
            source: *source,
            song_count: None,
            description: fallback_name.to_string(),
            play_count: None,
            date_text: None,
        },
        OnlineListUrlKind::External(source, id) => OnlinePlaylist {
            name: imported_title.unwrap_or_else(|| format!("{:?} Playlist {}", source, id)),
            author: "URL Import".to_string(),
            playlist_id: id.clone(),
            source: PlaylistSource::NetEase,
            song_count: None,
            description: fallback_name.to_string(),
            play_count: None,
            date_text: None,
        },
        OnlineListUrlKind::Unsupported(raw) => OnlinePlaylist {
            name: "Unsupported URL".to_string(),
            author: "URL Import".to_string(),
            playlist_id: raw.clone(),
            source: PlaylistSource::NetEase,
            song_count: None,
            description: fallback_name.to_string(),
            play_count: None,
            date_text: None,
        },
    }
}

fn title_for_online_list_url(kind: &OnlineListUrlKind) -> Option<String> {
    let url = match kind {
        OnlineListUrlKind::Playlist(PlaylistSource::Kuwo, id) => {
            format!("https://www.kuwo.cn/playlist_detail/{}", id)
        }
        OnlineListUrlKind::Artist(PlaylistSource::Kuwo, id) => {
            format!("https://www.kuwo.cn/singer_detail/{}", id)
        }
        OnlineListUrlKind::Rank(PlaylistSource::Kuwo, Some(id)) => {
            format!("https://www.kuwo.cn/rankList?bangId={}", id)
        }
        OnlineListUrlKind::Playlist(PlaylistSource::Kugou, id) if id.starts_with("gcid_") => {
            format!("https://www.kugou.com/songlist/{}/", id)
        }
        OnlineListUrlKind::Artist(PlaylistSource::Kugou, id) => {
            format!("https://www.kugou.com/singer/info/{}", id)
        }
        OnlineListUrlKind::Rank(PlaylistSource::Kugou, Some(id)) => {
            format!("https://www.kugou.com/yy/rank/home/{}-8888.html", id)
        }
        OnlineListUrlKind::Playlist(PlaylistSource::NetEase, id)
        | OnlineListUrlKind::Rank(PlaylistSource::NetEase, Some(id)) => {
            format!("https://music.163.com/playlist?id={}", id)
        }
        OnlineListUrlKind::Artist(PlaylistSource::NetEase, id) => {
            format!("https://music.163.com/artist?id={}", id)
        }
        OnlineListUrlKind::External(ExternalPlaylistSource::Spotify, id) => {
            let (kind, entity_id) = parse_spotify_entity(id);
            match kind {
                SpotifyEntityKind::Playlist => format!("https://open.spotify.com/playlist/{}", entity_id),
                SpotifyEntityKind::Artist => format!("https://open.spotify.com/artist/{}", entity_id),
                SpotifyEntityKind::Album => format!("https://open.spotify.com/album/{}", entity_id),
            }
        }
        OnlineListUrlKind::External(ExternalPlaylistSource::AppleMusic, url) => url.clone(),
        _ => return None,
    };
    fetch_online_list_page_title(&url)
}

fn fetch_online_list_page_title(url: &str) -> Option<String> {
    let is_spotify = url.to_ascii_lowercase().contains("open.spotify.com/");
    let is_apple_music = url.to_ascii_lowercase().contains("music.apple.com/");
    let client = if is_spotify {
        create_spotify_client().or_else(create_external_page_client)
    } else {
        create_external_page_client().or_else(create_search_client)
    }?;
    let user_agent = if is_spotify {
        spotify_mobile_user_agent()
    } else {
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
    };
    let accept_language = if is_apple_music {
        apple_music_storefront_from_url(url)
            .as_deref()
            .map(apple_music_locale_for_storefront)
            .unwrap_or("zh-CN")
    } else {
        "zh-CN,zh;q=0.9,en-US;q=0.8,en;q=0.7"
    };
    let html = match client
        .get(url)
        .header("User-Agent", user_agent)
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .header("Accept-Language", accept_language)
        .send()
    {
        Ok(resp) => {
            let status = resp.status();
            let text = resp.text().unwrap_or_default();
            log_file!(
                "[ListUrl][Title] 页面标题请求 HTTP {} 响应(前200): {}",
                status.as_u16(),
                preview_for_log(&text, 200)
            );
            if !status.is_success() || text.trim().is_empty() {
                return None;
            }
            text
        }
        Err(e) => {
            log_file!(
                "[ListUrl][Title] 页面标题请求失败: {}",
                reqwest_error_detail(&e)
            );
            return None;
        }
    };
    let title = if is_spotify {
        extract_spotify_page_title(&html)
    } else if is_apple_music {
        extract_apple_music_page_title(&html)
    } else {
        extract_online_list_page_title(&html)
    }
    .map(clean_online_list_page_title)
    .filter(|title| !is_generic_online_list_page_title(title));
    log_file!("[ListUrl][Title] 页面标题解析结果: {:?}", title);
    title
}

fn extract_apple_music_page_title(html: &str) -> Option<String> {
    extract_html_title(html)
        .filter(|title| !is_generic_online_list_page_title(title))
        .or_else(|| extract_online_list_page_title(html))
}

fn extract_spotify_page_title(html: &str) -> Option<String> {
    extract_html_title(html)
        .filter(|title| !is_generic_online_list_page_title(title))
        .or_else(|| extract_meta_content_by_name(html, "og:title"))
        .or_else(|| extract_meta_content_by_name(html, "twitter:title"))
}

fn extract_online_list_page_title(html: &str) -> Option<String> {
    for key in ["og:title", "twitter:title"] {
        if let Some(title) = extract_meta_content_by_name(html, key) {
            return Some(title);
        }
    }
    extract_html_title(html)
}

fn extract_html_title(html: &str) -> Option<String> {
    let start = html.to_ascii_lowercase().find("<title")?;
    let rest = &html[start..];
    let open_end = rest.find('>')? + 1;
    let body = &rest[open_end..];
    let end = body.to_ascii_lowercase().find("</title>")?;
    Some(decode_html_entities_basic(strip_html_tags(&body[..end]).trim()))
}

fn is_generic_online_list_page_title(title: &str) -> bool {
    let title = title.trim();
    title.is_empty()
        || title.eq_ignore_ascii_case("Apple Music")
        || title.contains("Apple Music 网页播放器")
        || title.contains("Apple Music Web Player")
}

fn extract_meta_content_by_name(html: &str, expected: &str) -> Option<String> {
    let lower = html.to_ascii_lowercase();
    let mut cursor = 0usize;
    while let Some(rel) = lower[cursor..].find("<meta") {
        let start = cursor + rel;
        let rest = &html[start..];
        let end = rest.find('>')?;
        let tag = &rest[..=end];
        let property = extract_html_attr_value_any_quote(tag, "property")
            .or_else(|| extract_html_attr_value_any_quote(tag, "name"))
            .unwrap_or_default();
        if property.eq_ignore_ascii_case(expected) {
            if let Some(content) = extract_html_attr_value_any_quote(tag, "content") {
                return Some(decode_html_entities_basic(content.trim()));
            }
        }
        cursor = start + end + 1;
    }
    None
}

fn clean_online_list_page_title(title: String) -> String {
    let mut title = title.split_whitespace().collect::<Vec<_>>().join(" ");
    for suffix in [
        " - Apple Music",
        " on Apple Music",
        " | Spotify Playlist",
        " | Spotify",
        " - Spotify",
        " - Kuwo Music",
        " - 酷我音乐",
        "_酷狗音乐",
        " - 酷狗音乐",
        " - 网易云音乐",
    ] {
        if let Some(stripped) = title.strip_suffix(suffix) {
            title = stripped.trim().to_string();
        }
    }
    title.trim_matches(['\'', '"']).trim().to_string()
}

fn fetch_kugou_gcid_playlist_songs(
    client: &reqwest::blocking::Client,
    gcid: &str,
) -> Vec<OnlineSong> {
    let id = gcid.trim_start_matches("gcid_").trim();
    for api_url in [
        format!(
            "https://mobileservice.kugou.com/api/v5/special/song?global_collection_id=gcid_{}&page=1&pagesize=100",
            id
        ),
        format!(
            "https://mobileservice.kugou.com/api/v5/special/song?global_collection_id={}&page=1&pagesize=100",
            id
        ),
        format!(
            "https://mobileservice.kugou.com/api/v5/special/song?global_collection_id={}&plat=0&page=1&pagesize=100",
            id
        ),
        format!(
            "http://mobilecdnbj.kugou.com/api/v5/special/song?global_collection_id=gcid_{}&page=1&pagesize=100",
            id
        ),
        format!(
            "http://mobilecdnbj.kugou.com/api/v3/special/song?specialid={}&page=1&pagesize=100",
            id
        ),
    ] {
        log_file!("[ListUrl][KG] gcid 接口URL: {}", api_url);
        if let Ok(text) = client.get(&api_url).send().and_then(|r| r.text()) {
            log_file!(
                "[ListUrl][KG] gcid 接口响应(前200): {}",
                preview_for_log(&text, 200)
            );
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
                let arr = v
                    .get("data")
                    .and_then(|d| d.get("info").or_else(|| d.get("list")))
                    .and_then(|x| x.as_array())
                    .cloned()
                    .or_else(|| extract_first_array(&v).cloned())
                    .unwrap_or_default();
                let songs = arr
                    .iter()
                    .filter_map(|x| parse_playlist_song_item(x, "kg", PlaylistSource::Kugou))
                    .collect::<Vec<_>>();
                if !songs.is_empty() {
                    return songs;
                }
            }
        }
    }

    let url = format!("https://www.kugou.com/songlist/gcid_{}/", id);
    log_file!("[ListUrl][KG] gcid 页面URL: {}", url);
    let html = match client
        .get(&url)
        .header("Referer", "https://www.kugou.com/")
        .send()
        .and_then(|r| r.text())
    {
        Ok(t) => t,
        Err(e) => {
            log_file!("[ListUrl][KG] gcid 页面请求失败: {}", e);
            return Vec::new();
        }
    };
    log_file!(
        "[ListUrl][KG] gcid 页面响应(前200): {}",
        preview_for_log(&html, 200)
    );

    if let Some(specialid) = extract_query_value(&html, "specialid")
        .or_else(|| extract_digits_after_marker(&html, "specialid"))
        .filter(|s| !s.is_empty())
    {
        let songs = fetch_kugou_playlist_songs(client, &specialid);
        if !songs.is_empty() {
            return songs;
        }
    }

    parse_kugou_songs_from_html(&html)
}

fn extract_digits_after_marker(input: &str, marker: &str) -> Option<String> {
    let pos = input.find(marker)?;
    let digits: String = input[pos + marker.len()..]
        .chars()
        .skip_while(|c| !c.is_ascii_digit())
        .take_while(|c| c.is_ascii_digit())
        .collect();
    if digits.is_empty() {
        None
    } else {
        Some(digits)
    }
}

fn parse_kugou_songs_from_html(html: &str) -> Vec<OnlineSong> {
    let song_container_songs = parse_kugou_song_container_from_html(html);
    if !song_container_songs.is_empty() {
        log_file!(
            "[ListUrl][KG] song_container 解析到 {} 首",
            song_container_songs.len()
        );
        return song_container_songs;
    }

    if let Some(songs) = parse_kugou_songsdata_from_html(html) {
        if !songs.is_empty() {
            log_file!("[ListUrl][KG] songsdata 解析到 {} 首", songs.len());
            return songs;
        }
    }

    let mut songs = Vec::new();
    let mut seen = std::collections::HashSet::new();
    let mut cursor = 0usize;
    while let Some(rel) = html[cursor..].find("hash") {
        let start = cursor + rel;
        let frag_owned: String = html[start.saturating_sub(300)..]
            .chars()
            .take(2200)
            .collect();
        let frag = frag_owned.as_str();
        let item = json!({
            "hash": pick_js_quoted_field(frag, "hash").or_else(|| pick_jsonish_string_field(frag, "hash")).unwrap_or_default(),
            "filename": pick_js_quoted_field(frag, "filename").or_else(|| pick_jsonish_string_field(frag, "filename")).unwrap_or_default(),
            "songname": pick_js_quoted_field(frag, "songname").or_else(|| pick_jsonish_string_field(frag, "songname")).unwrap_or_default(),
            "singername": pick_js_quoted_field(frag, "singername").or_else(|| pick_jsonish_string_field(frag, "singername")).unwrap_or_default(),
        });
        if let Some(song) = parse_playlist_song_item(&item, "kg", PlaylistSource::Kugou) {
            if seen.insert(song.juhe_song_id.clone()) {
                songs.push(song);
            }
        }
        cursor = start + 4;
    }
    log_file!("[ListUrl][KG] 页面兜底解析到 {} 首", songs.len());
    songs
}

fn parse_kugou_song_container_from_html(html: &str) -> Vec<OnlineSong> {
    let mut songs = Vec::new();
    let mut seen = std::collections::HashSet::new();
    let mut cursor = 0usize;
    let marker = "class=\"cb song_hid\"";
    while let Some(rel) = html[cursor..].find(marker) {
        let pos = cursor + rel;
        let frag = &html[pos..html.len().min(pos + 1200)];
        if let Some(value) = extract_html_attr_value(frag, "value") {
            let decoded = decode_html_entities_basic(&value);
            let parts = decoded.split('|').collect::<Vec<_>>();
            if parts.len() >= 3 {
                let (artist, name) = split_artist_title(parts[0]);
                let hash = parts[1].trim().to_string();
                let duration_ms = parts[2].trim().parse::<i64>().ok();
                if !hash.is_empty() && seen.insert(hash.clone()) {
                    songs.push(OnlineSong {
                        name,
                        artist,
                        id: 0,
                        hash: hash.clone(),
                        duration_ms,
                        source: MusicSource::Juhe,
                        juhe_platform: "kg".to_string(),
                        juhe_song_id: hash,
                    });
                }
            }
        }
        cursor = pos + marker.len();
    }
    songs
}

fn extract_html_attr_value(fragment: &str, attr: &str) -> Option<String> {
    let marker = format!("{}=\"", attr);
    let pos = fragment.find(&marker)?;
    let rest = &fragment[pos + marker.len()..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

fn extract_html_attr_value_any_quote(fragment: &str, attr: &str) -> Option<String> {
    let marker = format!("{}=", attr);
    let pos = fragment.find(&marker)?;
    let rest = fragment[pos + marker.len()..].trim_start();
    let quote = rest.chars().next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }
    let rest = &rest[quote.len_utf8()..];
    let end = rest.find(quote)?;
    Some(rest[..end].to_string())
}

fn decode_html_entities_basic(input: &str) -> String {
    input
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
}

fn split_artist_title(input: &str) -> (String, String) {
    let parts = input.splitn(2, " - ").collect::<Vec<_>>();
    if parts.len() == 2 {
        (parts[0].trim().to_string(), parts[1].trim().to_string())
    } else {
        (String::new(), input.trim().to_string())
    }
}

fn parse_kugou_songsdata_from_html(html: &str) -> Option<Vec<OnlineSong>> {
    let marker = "songsdata =";
    let pos = html.find(marker)?;
    let rest = &html[pos + marker.len()..];
    let start_rel = rest.find('[')?;
    let json_text = extract_balanced_json_array(&rest[start_rel..])?;
    let arr = serde_json::from_str::<Vec<serde_json::Value>>(&json_text).ok()?;
    let mut seen = std::collections::HashSet::new();
    let songs = arr
        .iter()
        .filter_map(|item| {
            let normalized = json!({
                "hash": pick_str_field(item, &["hash", "hash_128", "hash_320", "hash_flac"])
                    .unwrap_or_default(),
                "songname": pick_str_field(item, &["audio_name", "songname", "name"])
                    .unwrap_or_default(),
                "singername": pick_str_field(item, &["author_name", "singername", "artist"])
                    .unwrap_or_default(),
                "duration": item.get("timelength").and_then(|x| x.as_i64()).unwrap_or(0),
            });
            parse_playlist_song_item(&normalized, "kg", PlaylistSource::Kugou)
        })
        .filter(|song| seen.insert(song.juhe_song_id.clone()))
        .collect::<Vec<_>>();
    Some(songs)
}

fn extract_balanced_json_array(input: &str) -> Option<String> {
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    for (idx, ch) in input.char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }

        match ch {
            '"' => in_string = true,
            '[' => depth += 1,
            ']' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(input[..=idx].to_string());
                }
            }
            _ => {}
        }
    }
    None
}

fn pick_jsonish_string_field(src: &str, key: &str) -> Option<String> {
    for quote in ['\"', '\''] {
        let marker = format!("{}{}{}", quote, key, quote);
        if let Some(pos) = src.find(&marker) {
            let rest = &src[pos + marker.len()..];
            let value_start = rest.find(':')? + 1;
            let rest = rest[value_start..].trim_start();
            let q = rest.chars().next()?;
            if q != '\"' && q != '\'' {
                continue;
            }
            let rest = &rest[q.len_utf8()..];
            let end = rest.find(q)?;
            return Some(decode_js_escaped_string(&rest[..end]));
        }
    }
    None
}

fn fetch_kuwo_rank_songs(
    client: &reqwest::blocking::Client,
    rank_id: Option<&str>,
    page: usize,
    page_size: usize,
) -> Vec<OnlineSong> {
    let bang_id = rank_id.filter(|s| !s.trim().is_empty()).unwrap_or("16");
    let (cookie, csrf) = kuwo_auth_cookie_and_csrf(client);
    let req_id = kuwo_req_id();
    let safe_page = page.max(1);
    let safe_page_size = page_size.max(1).min(100);
    let url = format!(
        "https://www.kuwo.cn/api/www/bang/bang/musicList?bangId={}&pn={}&rn={}&httpsStatus=1&reqId={}&plat=web_www&from=",
        bang_id, safe_page, safe_page_size, req_id
    );
    log_file!("[Rank][KW] 榜单URL: {}", url);
    let text = match client
        .get(&url)
        .header("Referer", "https://www.kuwo.cn/rankList")
        .header("Host", "www.kuwo.cn")
        .header("Cookie", cookie)
        .header("csrf", csrf)
        .send()
        .and_then(|r| r.text())
    {
        Ok(t) => t,
        Err(e) => {
            log_file!("[Rank][KW] 请求失败: {}", e);
            return Vec::new();
        }
    };
    log_file!("[Rank][KW] 响应(前200): {}", preview_for_log(&text, 200));
    let songs = serde_json::from_str::<serde_json::Value>(&text)
        .ok()
        .and_then(|v| {
            v.get("data")
                .and_then(|d| d.get("musicList"))
                .and_then(|x| x.as_array())
                .cloned()
        })
        .unwrap_or_default()
        .iter()
        .filter_map(|x| parse_playlist_song_item(x, "kw", PlaylistSource::Kuwo))
        .collect::<Vec<_>>();
    if !songs.is_empty() {
        return songs;
    }

    let fallback = fetch_kuwo_rank_songs_from_legacy_page(client);
    log_file!("[Rank][KW] HTML兜底返回 {} 首", fallback.len());
    fallback
}

fn fetch_kuwo_rank_songs_from_legacy_page(client: &reqwest::blocking::Client) -> Vec<OnlineSong> {
    let url = "http://www.kuwo.cn/bang/content?name=%E9%85%B7%E6%88%91%E7%83%AD%E6%AD%8C%E6%A6%9C&type=bang&from=pc";
    log_file!("[Rank][KW][HTML] 榜单页面URL: {}", url);
    let html = match client
        .get(url)
        .header("Referer", "http://www.kuwo.cn/bang/index")
        .send()
        .and_then(|r| r.text())
    {
        Ok(t) => t,
        Err(e) => {
            log_file!("[Rank][KW][HTML] 请求失败: {}", e);
            return Vec::new();
        }
    };
    log_file!(
        "[Rank][KW][HTML] 响应(前200): {}",
        preview_for_log(&html, 200)
    );
    let candidates = parse_kuwo_legacy_rank_text_candidates(&html);
    log_file!("[Rank][KW][HTML] 解析候选 {} 首", candidates.len());
    candidates_to_unresolved_juhe_songs(candidates)
}

fn parse_kuwo_legacy_rank_text_candidates(html: &str) -> Vec<(String, String)> {
    let data_music_candidates = parse_kuwo_legacy_rank_data_music_candidates(html);
    if !data_music_candidates.is_empty() {
        return data_music_candidates;
    }

    let row_candidates = parse_kuwo_legacy_rank_row_candidates(html);
    if !row_candidates.is_empty() {
        return row_candidates;
    }

    let text = strip_html_tags(html)
        .replace("&amp;", "&")
        .replace("&nbsp;", " ");
    let lines = text
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    let mut out = Vec::new();
    let mut i = 0usize;
    let mut expected_rank = 1usize;
    while i + 2 < lines.len() {
        if lines[i]
            .parse::<usize>()
            .map(|n| n == expected_rank)
            .unwrap_or(false)
        {
            let mut j = i + 1;
            while j + 1 < lines.len() && lines[j].parse::<usize>().is_ok() {
                j += 1;
            }
            if j + 1 >= lines.len() {
                break;
            }
            let name = lines[j].trim();
            let artist = lines[j + 1].trim();
            if looks_like_song_candidate(name) && looks_like_song_candidate(artist) {
                out.push((name.to_string(), artist.to_string()));
                expected_rank += 1;
                i = j + 2;
                continue;
            }
        }
        i += 1;
    }
    dedup_song_artist_candidates(out)
}

fn parse_kuwo_legacy_rank_data_music_candidates(html: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut cursor = 0usize;
    let marker = "data-music='";

    while let Some(rel) = html[cursor..].find(marker) {
        let start = cursor + rel + marker.len();
        let rest = &html[start..];
        let Some(end) = rest.find('\'') else {
            break;
        };
        let raw = decode_html_entities_basic(&rest[..end]);
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&raw) {
            if let Some(name) = pick_str_field(&value, &["name"]) {
                let artist = pick_str_field(&value, &["artist"]).unwrap_or_default();
                if looks_like_song_candidate(&name) {
                    out.push((name, artist));
                }
            }
        }
        cursor = start + end + 1;
    }

    dedup_song_artist_candidates(out)
}

fn parse_kuwo_legacy_rank_row_candidates(html: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut cursor = 0usize;
    let mut expected_rank = 1usize;

    while let Some(rel) = html[cursor..].find("<li") {
        let start = cursor + rel;
        let rest = &html[start..];
        let Some(end_rel) = rest.find("</li>") else {
            break;
        };
        let row = &rest[..end_rel + "</li>".len()];
        let text = strip_html_tags(row)
            .replace("&amp;", "&")
            .replace("&nbsp;", " ");
        let lines = text
            .lines()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        if let Some((name, artist)) = parse_kuwo_legacy_rank_row_lines(&lines, expected_rank) {
            out.push((name, artist));
            expected_rank += 1;
        }

        cursor = start + end_rel + "</li>".len();
    }

    dedup_song_artist_candidates(out)
}

fn parse_kuwo_legacy_rank_row_lines(
    lines: &[&str],
    expected_rank: usize,
) -> Option<(String, String)> {
    let rank_pos = lines
        .iter()
        .position(|line| line.parse::<usize>().ok() == Some(expected_rank))?;
    let values = lines[rank_pos + 1..]
        .iter()
        .copied()
        .filter(|line| line.parse::<usize>().is_err())
        .filter(|line| looks_like_song_candidate(line))
        .take(2)
        .collect::<Vec<_>>();
    if values.len() < 2 {
        return None;
    }
    Some((values[0].to_string(), values[1].to_string()))
}

fn strip_html_tags(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut in_tag = false;
    for ch in input.chars() {
        match ch {
            '<' => {
                in_tag = true;
                out.push('\n');
            }
            '>' => {
                in_tag = false;
                out.push('\n');
            }
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    out
}

fn looks_like_song_candidate(s: &str) -> bool {
    let t = s.trim();
    !t.is_empty()
        && t.len() <= 80
        && !t.chars().all(|c| c.is_ascii_digit())
        && !t.starts_with('（')
        && !t.contains("最近更新")
        && !t.contains("下载")
        && !t.contains("播放")
        && !t.contains("评论")
        && !t.contains("添加")
        && !t.contains("趋势")
        && !t.contains("歌曲")
        && !t.contains("歌手")
        && !t.contains("热度")
}

fn dedup_song_artist_candidates(candidates: Vec<(String, String)>) -> Vec<(String, String)> {
    let mut seen = std::collections::HashSet::new();
    candidates
        .into_iter()
        .filter(|(name, artist)| seen.insert(format!("{}\n{}", name, artist)))
        .collect()
}

fn candidates_to_unresolved_juhe_songs(candidates: Vec<(String, String)>) -> Vec<OnlineSong> {
    candidates
        .into_iter()
        .map(|(name, artist)| OnlineSong::unresolved_juhe_candidate(name, artist))
        .collect()
}

pub fn resolve_unresolved_juhe_song(song: &OnlineSong) -> Option<OnlineSong> {
    if !song.is_unresolved_juhe_candidate() {
        return Some(song.clone());
    }
    let query = if song.artist.trim().is_empty() {
        song.name.clone()
    } else {
        format!("{} - {}", song.name, song.artist)
    };
    search_juhe(&query, 1)
        .songs
        .into_iter()
        .next()
        .map(|resolved| OnlineSong {
            name: song.name.clone(),
            artist: if song.artist.trim().is_empty() {
                resolved.artist
            } else {
                song.artist.clone()
            },
            duration_ms: song.duration_ms.or(resolved.duration_ms),
            ..resolved
        })
}

fn fetch_kugou_rank_songs_page(
    client: &reqwest::blocking::Client,
    rank_id: Option<&str>,
    page: usize,
    page_size: usize,
) -> Vec<OnlineSong> {
    let rank_id = rank_id.filter(|s| !s.trim().is_empty()).unwrap_or("6666");
    let url = format!(
        "http://mobilecdnbj.kugou.com/api/v3/rank/song?rankid={}&page={}&pagesize={}",
        rank_id, page, page_size
    );
    log_file!("[Rank][KG] 榜单URL: {}", url);
    let text = match client.get(&url).send().and_then(|r| r.text()) {
        Ok(t) => t,
        Err(e) => {
            log_file!("[Rank][KG] 请求失败: {}", e);
            return Vec::new();
        }
    };
    serde_json::from_str::<serde_json::Value>(&text)
        .ok()
        .and_then(|v| {
            v.get("data")
                .and_then(|d| d.get("info").or_else(|| d.get("list")))
                .and_then(|x| x.as_array())
                .cloned()
        })
        .unwrap_or_default()
        .iter()
        .filter_map(|x| parse_playlist_song_item(x, "kg", PlaylistSource::Kugou))
        .collect()
}

fn fetch_kuwo_artist_songs_page(
    client: &reqwest::blocking::Client,
    artist_id: &str,
    page: usize,
    page_size: usize,
) -> Vec<OnlineSong> {
    let (cookie, csrf) = kuwo_auth_cookie_and_csrf(client);
    let req_id = kuwo_req_id();
    let url = format!(
        "https://www.kuwo.cn/api/www/artist/artistMusic?artistid={}&pn={}&rn={}&httpsStatus=1&reqId={}&plat=web_www&from=",
        artist_id, page, page_size, req_id
    );
    log_file!("[artist][KW] 歌手歌曲URL: {}", url);
    let text = match client
        .get(&url)
        .header(
            "Referer",
            format!("https://www.kuwo.cn/singer_detail/{}", artist_id),
        )
        .header("Host", "www.kuwo.cn")
        .header("Cookie", cookie)
        .header("csrf", csrf)
        .send()
        .and_then(|r| r.text())
    {
        Ok(t) => t,
        Err(e) => {
            log_file!("[artist][KW] 请求失败: {}", e);
            return Vec::new();
        }
    };
    log_file!("[artist][KW] 响应(前200): {}", preview_for_log(&text, 200));
    let songs = serde_json::from_str::<serde_json::Value>(&text)
        .ok()
        .and_then(|v| {
            v.get("data")
                .and_then(|d| d.get("list").or_else(|| d.get("musicList")))
                .and_then(|x| x.as_array())
                .cloned()
        })
        .unwrap_or_default()
        .iter()
        .filter_map(|x| parse_playlist_song_item(x, "kw", PlaylistSource::Kuwo))
        .collect::<Vec<_>>();
    if !songs.is_empty() {
        return songs;
    }

    let fallback = fetch_kuwo_artist_songs_from_page(client, artist_id, page, page_size);
    log_file!("[artist][KW] 页面/搜索兜底返回 {} 首", fallback.len());
    fallback
}

fn fetch_kuwo_artist_songs_from_page(
    client: &reqwest::blocking::Client,
    artist_id: &str,
    page: usize,
    page_size: usize,
) -> Vec<OnlineSong> {
    let url = format!("https://www.kuwo.cn/singer_detail/{}", artist_id);
    log_file!("[artist][KW][Page] 页面URL: {}", url);
    let html = match client
        .get(&url)
        .header("Referer", "https://www.kuwo.cn/")
        .send()
        .and_then(|r| r.text())
    {
        Ok(t) => t,
        Err(e) => {
            log_file!("[artist][KW][Page] 请求失败: {}", e);
            return Vec::new();
        }
    };
    if let Some(candidates) = parse_kuwo_nuxt_music_candidates(&html) {
        if !candidates.is_empty() {
            let start = page.saturating_sub(1).saturating_mul(page_size);
            return candidates_to_unresolved_juhe_songs(
                candidates.into_iter().skip(start).take(page_size).collect(),
            );
        }
    }
    let artist = extract_kuwo_artist_name_from_page(&html).unwrap_or_else(|| artist_id.to_string());
    search_juhe(&artist, page)
        .songs
        .into_iter()
        .take(page_size)
        .collect()
}

fn parse_kuwo_nuxt_music_candidates(html: &str) -> Option<Vec<(String, String)>> {
    let pos = html.find("musicList")?;
    let rest = &html[pos..];
    let array_pos = rest.find('[')?;
    let json_text = extract_balanced_json_array(&rest[array_pos..])?;
    let arr = serde_json::from_str::<Vec<serde_json::Value>>(&json_text).ok()?;
    let candidates = arr
        .iter()
        .filter_map(|item| {
            let name = pick_str_field(item, &["name", "songName", "musicName"])?;
            let artist =
                pick_str_field(item, &["artist", "artistName", "singer"]).unwrap_or_default();
            Some((name, artist))
        })
        .collect::<Vec<_>>();
    Some(dedup_song_artist_candidates(candidates))
}

fn extract_kuwo_artist_name_from_page(html: &str) -> Option<String> {
    pick_html_quoted_after(html, "name:\"")
        .or_else(|| pick_html_quoted_after(html, "name:&quot;"))
        .or_else(|| {
            let title = html.find("<title>")?;
            let rest = &html[title + "<title>".len()..];
            let end = rest.find("</title>")?;
            Some(
                rest[..end]
                    .split('单')
                    .next()
                    .unwrap_or(rest[..end].trim())
                    .to_string(),
            )
        })
        .map(|s| s.replace("&nbsp;", " ").trim().to_string())
        .filter(|s| !s.is_empty())
}

fn pick_html_quoted_after(html: &str, marker: &str) -> Option<String> {
    let pos = html.find(marker)?;
    let rest = &html[pos + marker.len()..];
    let end = rest.find(['"', '&'])?;
    Some(rest[..end].to_string())
}

fn fetch_netease_artist_songs(
    client: &reqwest::blocking::Client,
    artist_id: &str,
) -> Vec<OnlineSong> {
    let url = format!("https://music.163.com/api/artist/{}", artist_id);
    log_file!("[artist][WY] 歌手歌曲URL: {}", url);
    let text = match client
        .get(&url)
        .header("Referer", "https://music.163.com/")
        .header("Cookie", "MUSIC_U=; appver=2.0.2;")
        .send()
        .and_then(|r| r.text())
    {
        Ok(t) => t,
        Err(e) => {
            log_file!("[artist][WY] 请求失败: {}", e);
            return Vec::new();
        }
    };
    serde_json::from_str::<serde_json::Value>(&text)
        .ok()
        .and_then(|v| v.get("hotSongs").and_then(|x| x.as_array()).cloned())
        .unwrap_or_default()
        .iter()
        .filter_map(|x| parse_playlist_song_item(x, "wy", PlaylistSource::NetEase))
        .collect()
}

fn fetch_kugou_artist_songs(
    client: &reqwest::blocking::Client,
    artist_id: &str,
) -> Vec<OnlineSong> {
    if artist_id.chars().all(|c| c.is_ascii_digit()) {
        let songs = fetch_kugou_artist_songs_by_numeric_id(client, artist_id);
        if !songs.is_empty() {
            return songs;
        }
    }

    let mut html = String::new();
    let urls = [
        format!("https://www.kugou.com/singer/info/{}/", artist_id),
        format!("https://www.kugou.com/yy/singer/home/{}.html", artist_id),
    ];

    for url in urls {
        log_file!("[artist][KG] 歌手页面URL: {}", url);
        match send_kugou_browser_page_request(client, &url, "https://www.kugou.com/") {
            Ok(t) => {
                log_file!("[artist][KG] 页面响应(前200): {}", preview_for_log(&t, 200));
                html = t;
                if html.contains("songsdata =") || html.contains("song_container") {
                    break;
                }
            }
            Err(e) => {
                log_file!("[artist][KG] 页面请求失败: {}", e);
            }
        }
    }

    if html.is_empty() {
        return Vec::new();
    }

    if !html.contains("songsdata =") {
        if let Some(numeric_id) = extract_js_assignment_string(&html, "singerID") {
            let songs = fetch_kugou_artist_songs_by_numeric_id(client, &numeric_id);
            if !songs.is_empty() {
                return songs;
            }
            let url = format!("https://www.kugou.com/yy/singer/home/{}.html", numeric_id);
            log_file!("[artist][KG] 歌手数字ID页面URL: {}", url);
            if let Ok(t) = send_kugou_browser_page_request(client, &url, "https://www.kugou.com/") {
                html = t;
            }
        }
    }

    parse_kugou_songs_from_html(&html)
}

fn send_kugou_browser_page_request(
    client: &reqwest::blocking::Client,
    url: &str,
    referer: &str,
) -> Result<String, reqwest::Error> {
    let mut request = client
        .get(url)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.0 Safari/605.1.15",
        )
        .header(
            "Accept",
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
        )
        .header("Accept-Language", "zh-CN,zh;q=0.9")
        .header("Cache-Control", "max-age=0")
        .header("Upgrade-Insecure-Requests", "1")
        .header("Sec-Fetch-Dest", "document")
        .header("Sec-Fetch-Mode", "navigate")
        .header("Sec-Fetch-Site", "none")
        .header("Sec-Fetch-User", "?1")
        .header("Referer", referer);

    if let Ok(cookie) =
        std::env::var("TER_MUSIC_KUGOU_COOKIE").or_else(|_| std::env::var("KUGOU_COOKIE"))
    {
        if !cookie.trim().is_empty() {
            request = request.header("Cookie", cookie);
        }
    }

    request.send().and_then(|r| r.text())
}

fn fetch_kugou_artist_songs_by_numeric_id(
    client: &reqwest::blocking::Client,
    singer_id: &str,
) -> Vec<OnlineSong> {
    let mut songs = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for page in 1..=4 {
        let url = format!(
            "https://wwwapi.kugou.com/yy/index.php?r=singer/song&sid={}&p={}",
            singer_id, page
        );
        log_file!("[artist][KG] 歌手接口URL: {}", url);
        let text = match client
            .get(&url)
            .header(
                "Referer",
                format!("https://www.kugou.com/yy/singer/home/{}.html", singer_id),
            )
            .send()
            .and_then(|r| r.text())
        {
            Ok(t) => t,
            Err(e) => {
                log_file!("[artist][KG] 歌手接口请求失败: {}", e);
                break;
            }
        };
        log_file!(
            "[artist][KG] 歌手接口响应(前200): {}",
            preview_for_log(&text, 200)
        );
        let arr = serde_json::from_str::<serde_json::Value>(&text)
            .ok()
            .and_then(|v| v.get("data").and_then(|x| x.as_array()).cloned())
            .unwrap_or_default();
        if arr.is_empty() {
            break;
        }
        for item in arr {
            if let Some(song) = parse_playlist_song_item(&item, "kg", PlaylistSource::Kugou) {
                if seen.insert(song.juhe_song_id.clone()) {
                    songs.push(song);
                }
            }
        }
    }
    log_file!("[artist][KG] 歌手接口解析到 {} 首", songs.len());
    songs
}

fn extract_js_assignment_string(html: &str, key: &str) -> Option<String> {
    let pos = html
        .find(&format!("{} =", key))
        .or_else(|| html.find(&format!("{}=", key)))?;
    let rest = &html[pos + key.len()..];
    let quote_pos = rest.find(['\'', '"'])?;
    let quote = rest[quote_pos..].chars().next()?;
    let rest = &rest[quote_pos + quote.len_utf8()..];
    let end = rest.find(quote)?;
    Some(rest[..end].trim().to_string()).filter(|s| !s.is_empty())
}

fn fetch_external_playlist_songs_with_title(
    source: ExternalPlaylistSource,
    id_or_url: &str,
    page: usize,
    page_size: usize,
) -> ExternalPlaylistSongsResult {
    let Some(client) = (if source == ExternalPlaylistSource::Spotify {
        create_spotify_client().or_else(create_external_page_client)
    } else {
        create_external_page_client().or_else(create_search_client)
    }) else {
        return ExternalPlaylistSongsResult {
            songs: Vec::new(),
            title: None,
        };
    };
    if source == ExternalPlaylistSource::Spotify {
        if let Some((songs, title)) =
            fetch_spotify_songs_with_title(&client, id_or_url, page, page_size)
        {
            let title = title.or_else(|| fetch_spotify_page_title(id_or_url));
            log_file!("[ListUrl][Spotify] 标题解析结果: {:?}", title);
            return ExternalPlaylistSongsResult { songs, title };
        }
        log_file!(
            "[ListUrl][Spotify] API 解析失败；如 open.spotify.com 被拦截，请设置 SPOTIFY_PROXY/HTTPS_PROXY，或设置 SPOTIFY_BEARER_TOKEN_FORCE=1 使用手动 token"
        );
        return ExternalPlaylistSongsResult {
            songs: Vec::new(),
            title: None,
        };
    }
    if source == ExternalPlaylistSource::AppleMusic {
        let title = fetch_apple_music_entity_title(&client, id_or_url);
        if let Some(songs) =
            fetch_apple_music_playlist_songs_page(&client, id_or_url, page, page_size)
        {
            return ExternalPlaylistSongsResult {
                songs,
                title: title.clone(),
            };
        }
        if let Some(songs) = fetch_apple_music_room_songs_page(&client, id_or_url, page, page_size)
        {
            return ExternalPlaylistSongsResult {
                songs,
                title: title.clone(),
            };
        }
        if let Some(songs) =
            fetch_apple_music_top_chart_songs_page(&client, id_or_url, page, page_size)
        {
            return ExternalPlaylistSongsResult {
                songs,
                title: title.clone(),
            };
        }
        if let Some(songs) = fetch_apple_music_album_songs_page(&client, id_or_url, page, page_size)
        {
            return ExternalPlaylistSongsResult {
                songs,
                title: title.clone(),
            };
        }
        if let Some(songs) = fetch_apple_artist_top_songs_page(&client, id_or_url, page, 25) {
            return ExternalPlaylistSongsResult { songs, title };
        }
    }
    let url = match source {
        ExternalPlaylistSource::Spotify => {
            format!("https://open.spotify.com/playlist/{}", id_or_url)
        }
        ExternalPlaylistSource::AppleMusic => id_or_url.to_string(),
    };
    log_file!("[ListUrl][External] {:?} 页面URL: {}", source, url);
    let user_agent = match source {
        ExternalPlaylistSource::Spotify => "Mozilla/5.0 (iPhone; CPU iPhone OS 16_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.6 Mobile/15E148 Safari/604.1",
        ExternalPlaylistSource::AppleMusic => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    };
    let html = match client
        .get(&url)
        .header("User-Agent", user_agent)
        .header(
            "Accept",
            "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
        )
        .header("Accept-Language", "zh-CN,zh;q=0.9,en-US;q=0.8,en;q=0.7")
        .header("Cache-Control", "max-age=0")
        .header("Upgrade-Insecure-Requests", "1")
        .send()
        .and_then(|r| r.text())
    {
        Ok(t) => t,
        Err(e) => {
            log_file!("[ListUrl][External] 页面请求失败: {}", e);
            return ExternalPlaylistSongsResult {
                songs: Vec::new(),
                title: None,
            };
        }
    };
    log_file!(
        "[ListUrl][External] 页面响应(前200): {}",
        preview_for_log(&html, 200)
    );
    let candidates = match source {
        ExternalPlaylistSource::Spotify => parse_spotify_playlist_candidates(&html),
        ExternalPlaylistSource::AppleMusic => parse_apple_music_playlist_candidates(&html),
    };
    log_file!(
        "[ListUrl][External] {:?} 解析候选 {} 首",
        source,
        candidates.len()
    );
    ExternalPlaylistSongsResult {
        songs: candidates_to_unresolved_juhe_songs(candidates),
        title: match source {
            ExternalPlaylistSource::AppleMusic => extract_apple_music_page_title(&html),
            ExternalPlaylistSource::Spotify => extract_online_list_page_title(&html),
        }
        .map(clean_online_list_page_title)
        .filter(|title| !is_generic_online_list_page_title(title)),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpotifyEntityKind {
    Playlist,
    Artist,
    Album,
}

fn parse_spotify_entity(id_or_url: &str) -> (SpotifyEntityKind, &str) {
    if let Some(id) = id_or_url.strip_prefix("artist:") {
        (SpotifyEntityKind::Artist, id)
    } else if let Some(id) = id_or_url.strip_prefix("album:") {
        (SpotifyEntityKind::Album, id)
    } else {
        (SpotifyEntityKind::Playlist, id_or_url)
    }
}

fn fetch_spotify_songs_page_with_title(
    client: &reqwest::blocking::Client,
    id_or_url: &str,
    page: usize,
    page_size: usize,
) -> Option<(Vec<OnlineSong>, Option<String>)> {
    let (kind, id) = parse_spotify_entity(id_or_url);
    match kind {
        SpotifyEntityKind::Playlist => fetch_spotify_playlist_songs_page(client, id, page, page_size),
        SpotifyEntityKind::Artist => fetch_spotify_artist_pathfinder_tracks(client, id)
            .or_else(|| fetch_spotify_artist_top_tracks(client, id))
            .map(|songs| (songs, fetch_spotify_page_title(id_or_url))),
        SpotifyEntityKind::Album => fetch_spotify_album_pathfinder_tracks_page(client, id, page, page_size)
            .or_else(|| fetch_spotify_album_tracks_page(client, id, page, page_size))
            .map(|songs| (songs, fetch_spotify_page_title(id_or_url))),
    }
}

fn fetch_spotify_songs_with_title(
    client: &reqwest::blocking::Client,
    id_or_url: &str,
    page: usize,
    page_size: usize,
) -> Option<(Vec<OnlineSong>, Option<String>)> {
    fetch_spotify_songs_page_with_title(client, id_or_url, page, page_size)
}

fn fetch_spotify_page_title(id_or_url: &str) -> Option<String> {
    let (kind, id) = parse_spotify_entity(id_or_url);
    let url = match kind {
        SpotifyEntityKind::Playlist => format!("https://open.spotify.com/playlist/{}", id),
        SpotifyEntityKind::Artist => format!("https://open.spotify.com/artist/{}", id),
        SpotifyEntityKind::Album => format!("https://open.spotify.com/album/{}", id),
    };
    fetch_online_list_page_title(&url)
}

fn spotify_playlist_title_from_pathfinder(v: &serde_json::Value) -> Option<String> {
    for pointer in [
        "/data/playlistV2/name",
        "/data/playlistV2/profile/name",
        "/data/playlistV2/title",
    ] {
        if let Some(title) = v.pointer(pointer).and_then(|x| x.as_str()) {
            let title = title.trim();
            if !title.is_empty() {
                return Some(title.to_string());
            }
        }
    }
    None
}

fn fetch_spotify_playlist_songs_page(
    client: &reqwest::blocking::Client,
    playlist_id: &str,
    page: usize,
    page_size: usize,
) -> Option<(Vec<OnlineSong>, Option<String>)> {
    const DEFAULT_HASH: &str = "a65e12194ed5fc443a1cdebed5fabe33ca5b07b987185d63c72483867ad13cb4";
    let mut token = fetch_spotify_access_token(client)?;
    let limit = page_size.max(1).min(50);
    let offset = page.saturating_sub(1).saturating_mul(limit);
    let query_hash = std::env::var("SPOTIFY_PLAYLIST_QUERY_HASH")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| s.len() == 64 && s.chars().all(|c| c.is_ascii_hexdigit()))
        .unwrap_or_else(|| DEFAULT_HASH.to_string());
    let payload = json!({
        "variables": {
            "uri": format!("spotify:playlist:{}", playlist_id),
            "offset": offset,
            "limit": limit,
            "includeEpisodeContentRatingsV2": false
        },
        "operationName": "fetchPlaylistContents",
        "extensions": {
            "persistedQuery": {
                "version": 1,
                "sha256Hash": query_hash
            }
        }
    });
    log_file!(
        "[ListUrl][Spotify] pathfinder offset={} limit={}",
        offset,
        limit
    );
    let mut retried_after_401 = false;
    let text = loop {
        let mut req = client
            .post("https://api-partner.spotify.com/pathfinder/v2/query")
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json;charset=UTF-8")
            .header("Origin", "https://open.spotify.com")
            .header("Referer", "https://open.spotify.com/")
            .header("App-Platform", "WebPlayer")
            .header("spotify-app-version", "1.2.91.28.g7a1ea937")
            .header("User-Agent", spotify_user_agent())
            .json(&payload);
        if let Some(client_token) = spotify_client_token_from_env() {
            req = req.header("client-token", client_token);
        }
        match req.send() {
            Ok(r) => {
                let status = r.status();
                let text = r.text().unwrap_or_default();
                log_file!(
                    "[ListUrl][Spotify] pathfinder HTTP {} 响应(前200): {}",
                    status.as_u16(),
                    preview_for_log(&text, 200)
                );
                if status.as_u16() == 401 && !retried_after_401 {
                    log_file!("[ListUrl][Spotify] pathfinder 401，刷新 token 后重试");
                    token = fetch_spotify_access_token_force_refresh(client)?;
                    retried_after_401 = true;
                    continue;
                }
                if text.trim().is_empty() {
                    log_file!("[ListUrl][Spotify] pathfinder 空响应，跳过 Web API 兜底以避免 429");
                    return None;
                }
                break text;
            }
            Err(e) => {
                log_file!(
                    "[ListUrl][Spotify] pathfinder 请求失败: {}",
                    reqwest_error_detail(&e)
                );
                return None;
            }
        }
    };
    let v = match serde_json::from_str::<serde_json::Value>(&text) {
        Ok(v) => v,
        Err(e) => {
            log_file!("[ListUrl][Spotify] pathfinder JSON 解析失败: {}", e);
            return None;
        }
    };
    if let Some(errors) = v.get("errors") {
        log_file!("[ListUrl][Spotify] pathfinder 返回错误: {}", errors);
        return None;
    }
    let items = v
        .pointer("/data/playlistV2/content/items")
        .and_then(|x| x.as_array())?;
    let title = spotify_playlist_title_from_pathfinder(&v);
    let songs = items
        .iter()
        .filter_map(spotify_pathfinder_item_to_song)
        .collect::<Vec<_>>();
    log_file!("[ListUrl][Spotify] pathfinder 解析 {} 首", songs.len());
    Some((songs, title))
}

fn fetch_spotify_artist_pathfinder_tracks(
    client: &reqwest::blocking::Client,
    artist_id: &str,
) -> Option<Vec<OnlineSong>> {
    let hash = std::env::var("SPOTIFY_ARTIST_QUERY_HASH")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| s.len() == 64 && s.chars().all(|c| c.is_ascii_hexdigit()))
        .unwrap_or_else(|| {
            "b82fd661d09d47afff0d0239b165e01c7b21926923064ecc7e63f0cde2b12f4e".to_string()
        });
    let variables = json!({
        "uri": format!("spotify:artist:{}", artist_id),
        "locale": ""
    });
    let v = fetch_spotify_pathfinder_json(client, "queryArtistOverview", variables, &hash)?;
    let songs = spotify_collect_track_songs(&v);
    log_file!(
        "[ListUrl][Spotify] pathfinder artist 解析 {} 首",
        songs.len()
    );
    (!songs.is_empty()).then_some(songs)
}

fn fetch_spotify_album_pathfinder_tracks_page(
    client: &reqwest::blocking::Client,
    album_id: &str,
    page: usize,
    page_size: usize,
) -> Option<Vec<OnlineSong>> {
    let hash = std::env::var("SPOTIFY_ALBUM_QUERY_HASH")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| s.len() == 64 && s.chars().all(|c| c.is_ascii_hexdigit()))
        .unwrap_or_else(|| {
            "46ae954ef2d2fe7732b4b2b4022157b2e18b7ea84f70591ceb164e4de1b5d5d3".to_string()
        });
    let limit = page_size.max(1).min(50);
    let offset = page.saturating_sub(1).saturating_mul(limit);
    let variables = json!({
        "uri": format!("spotify:album:{}", album_id),
        "locale": "",
        "offset": offset,
        "limit": limit
    });
    let v = fetch_spotify_pathfinder_json(client, "getAlbum", variables, &hash)?;
    let songs = spotify_collect_track_songs(&v);
    log_file!(
        "[ListUrl][Spotify] pathfinder album 解析 {} 首",
        songs.len()
    );
    (!songs.is_empty()).then_some(songs)
}

fn fetch_spotify_pathfinder_json(
    client: &reqwest::blocking::Client,
    operation_name: &str,
    variables: serde_json::Value,
    query_hash: &str,
) -> Option<serde_json::Value> {
    let mut token = fetch_spotify_access_token(client)?;
    let payload = json!({
        "variables": variables,
        "operationName": operation_name,
        "extensions": {
            "persistedQuery": {
                "version": 1,
                "sha256Hash": query_hash
            }
        }
    });
    log_file!("[ListUrl][Spotify] pathfinder operation={}", operation_name);
    let mut retried_after_401 = false;
    let (status, text) = loop {
        let mut req = client
            .post("https://api-partner.spotify.com/pathfinder/v1/query")
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json;charset=UTF-8")
            .header("Origin", "https://open.spotify.com")
            .header("Referer", "https://open.spotify.com/")
            .header("App-Platform", "WebPlayer")
            .header("spotify-app-version", "1.2.91.28.g7a1ea937")
            .header("User-Agent", spotify_user_agent())
            .json(&payload);
        if let Some(client_token) = spotify_client_token_from_env() {
            req = req.header("client-token", client_token);
        }
        let r = match req.send() {
            Ok(r) => r,
            Err(e) => {
                log_file!(
                    "[ListUrl][Spotify] pathfinder {} 请求失败: {}",
                    operation_name,
                    reqwest_error_detail(&e)
                );
                return None;
            }
        };
        let status = r.status();
        let text = r.text().unwrap_or_default();
        log_file!(
            "[ListUrl][Spotify] pathfinder {} HTTP {} 响应(前200): {}",
            operation_name,
            status.as_u16(),
            preview_for_log(&text, 200)
        );
        if status.as_u16() == 401 && !retried_after_401 {
            log_file!(
                "[ListUrl][Spotify] pathfinder {} 401，刷新 token 后重试",
                operation_name
            );
            token = fetch_spotify_access_token_force_refresh(client)?;
            retried_after_401 = true;
            continue;
        }
        break (status, text);
    };
    if !status.is_success() || text.trim().is_empty() {
        return None;
    }
    let v = serde_json::from_str::<serde_json::Value>(&text).ok()?;
    if let Some(errors) = v.get("errors") {
        log_file!(
            "[ListUrl][Spotify] pathfinder {} 返回错误: {}",
            operation_name,
            errors
        );
        return None;
    }
    Some(v)
}

fn fetch_spotify_album_tracks_page(
    client: &reqwest::blocking::Client,
    album_id: &str,
    page: usize,
    page_size: usize,
) -> Option<Vec<OnlineSong>> {
    let token = fetch_spotify_access_token(client)?;
    let limit = page_size.max(1).min(50);
    let offset = page.saturating_sub(1).saturating_mul(limit);
    let url = format!(
        "https://api.spotify.com/v1/albums/{}/tracks?market=from_token&offset={}&limit={}",
        album_id, offset, limit
    );
    let v = fetch_spotify_web_api_json(client, &token, &url)?;
    let items = v.get("items").and_then(|x| x.as_array())?;
    let songs = items
        .iter()
        .filter_map(spotify_web_api_track_to_song)
        .collect::<Vec<_>>();
    log_file!("[ListUrl][Spotify] Web API album 解析 {} 首", songs.len());
    Some(songs)
}

fn fetch_spotify_artist_top_tracks(
    client: &reqwest::blocking::Client,
    artist_id: &str,
) -> Option<Vec<OnlineSong>> {
    let token = fetch_spotify_access_token(client)?;
    let url = format!(
        "https://api.spotify.com/v1/artists/{}/top-tracks?market=from_token",
        artist_id
    );
    let v = fetch_spotify_web_api_json(client, &token, &url)?;
    let tracks = v.get("tracks").and_then(|x| x.as_array())?;
    let songs = tracks
        .iter()
        .filter_map(spotify_web_api_track_to_song)
        .collect::<Vec<_>>();
    log_file!(
        "[ListUrl][Spotify] Web API artist top-tracks 解析 {} 首",
        songs.len()
    );
    Some(songs)
}

fn fetch_spotify_web_api_json(
    client: &reqwest::blocking::Client,
    token: &str,
    url: &str,
) -> Option<serde_json::Value> {
    log_file!("[ListUrl][Spotify] Web API URL: {}", url);
    let mut token = token.to_string();
    let mut retried_after_401 = false;
    let text = loop {
        let mut req = client
            .get(url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "application/json")
            .header("Origin", "https://open.spotify.com")
            .header("Referer", "https://open.spotify.com/")
            .header("User-Agent", spotify_user_agent());
        if let Some(client_token) = spotify_client_token_from_env() {
            req = req.header("client-token", client_token);
        }
        let r = match req.send() {
            Ok(r) => r,
            Err(e) => {
                log_file!(
                    "[ListUrl][Spotify] Web API 请求失败: {}",
                    reqwest_error_detail(&e)
                );
                return None;
            }
        };
        let status = r.status();
        let text = r.text().unwrap_or_default();
        log_file!(
            "[ListUrl][Spotify] Web API HTTP {} 响应(前200): {}",
            status.as_u16(),
            preview_for_log(&text, 200)
        );
        if status.as_u16() == 401 && !retried_after_401 {
            log_file!("[ListUrl][Spotify] Web API 401，刷新 token 后重试");
            token = fetch_spotify_access_token_force_refresh(client)?;
            retried_after_401 = true;
            continue;
        }
        break text;
    };
    if text.trim().is_empty() {
        return None;
    }
    serde_json::from_str::<serde_json::Value>(&text).ok()
}

fn fetch_spotify_access_token(client: &reqwest::blocking::Client) -> Option<String> {
    if spotify_force_manual_bearer_token() {
        if let Some(token) = spotify_bearer_token_from_env() {
            log_file!("[ListUrl][Spotify] 使用 SPOTIFY_BEARER_TOKEN_FORCE 指定的手动 token");
            return Some(token);
        }
    }
    if let Some(token) = cached_spotify_access_token() {
        return Some(token);
    }
    if let Some(token) = fetch_spotify_totp_access_token(client, false) {
        return Some(token);
    }
    for url in [
        "https://open.spotify.com/api/token",
        "https://open.spotify.com/get_access_token?reason=transport&productType=web_player",
    ] {
        match client
            .get(url)
            .header("Accept", "application/json")
            .header("Referer", "https://open.spotify.com/")
            .header("Origin", "https://open.spotify.com")
            .header("App-Platform", "WebPlayer")
            .header("spotify-app-version", "1.2.91.28.g7a1ea937")
            .header("User-Agent", spotify_user_agent())
            .send()
            .and_then(|r| r.text())
        {
            Ok(text) => {
                log_file!(
                    "[ListUrl][Spotify] token 响应(前200): {}",
                    preview_for_log(&text, 200)
                );
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
                    if let Some(token) = pick_str_field(&v, &["accessToken", "access_token"]) {
                        return Some(token);
                    }
                }
            }
            Err(e) => {
                log_file!(
                    "[ListUrl][Spotify] token 请求失败({}): {}",
                    url,
                    reqwest_error_detail(&e)
                );
            }
        }
    }
    None
}

fn fetch_spotify_access_token_force_refresh(client: &reqwest::blocking::Client) -> Option<String> {
    clear_spotify_token_cache();
    if let Some(token) = fetch_spotify_totp_access_token(client, false) {
        return Some(token);
    }
    if spotify_force_manual_bearer_token() {
        return spotify_bearer_token_from_env();
    }
    None
}

fn spotify_bearer_token_from_env() -> Option<String> {
    std::env::var("SPOTIFY_BEARER_TOKEN")
        .ok()
        .map(|token| token.trim().trim_start_matches("Bearer ").to_string())
        .filter(|token| !token.is_empty())
}

fn spotify_force_manual_bearer_token() -> bool {
    std::env::var("SPOTIFY_BEARER_TOKEN_FORCE")
        .ok()
        .map(|value| matches!(value.trim(), "1" | "true" | "TRUE" | "yes" | "YES"))
        .unwrap_or(false)
}

#[derive(Debug, Clone)]
struct SpotifyTokenCache {
    access_token: String,
    expires_at_ms: i64,
}

#[derive(Debug, Clone)]
struct SpotifyTotpSecret {
    version: u32,
    secret: Vec<u8>,
}

static SPOTIFY_TOKEN_CACHE: OnceLock<Mutex<Option<SpotifyTokenCache>>> = OnceLock::new();
static OPENCC_T2S_CACHE: OnceLock<Option<OpenCC>> = OnceLock::new();

fn spotify_token_cache() -> &'static Mutex<Option<SpotifyTokenCache>> {
    SPOTIFY_TOKEN_CACHE.get_or_init(|| Mutex::new(None))
}

fn cached_spotify_access_token() -> Option<String> {
    let now_ms = Local::now().timestamp_millis();
    let guard = spotify_token_cache().lock().ok()?;
    let cache = guard.as_ref()?;
    if cache.expires_at_ms.saturating_sub(now_ms) > 60_000 {
        Some(cache.access_token.clone())
    } else {
        None
    }
}

fn clear_spotify_token_cache() {
    if let Ok(mut guard) = spotify_token_cache().lock() {
        *guard = None;
    }
}

fn store_spotify_access_token(token: String, expires_at_ms: i64) {
    if token.trim().is_empty() {
        return;
    }
    if let Ok(mut guard) = spotify_token_cache().lock() {
        *guard = Some(SpotifyTokenCache {
            access_token: token,
            expires_at_ms,
        });
    }
}

fn fetch_spotify_totp_access_token(
    client: &reqwest::blocking::Client,
    force_remote_secrets: bool,
) -> Option<String> {
    let secret = spotify_totp_secret(client, force_remote_secrets)?;
    let local_ms = Local::now().timestamp_millis();
    let server_ms = fetch_spotify_server_time_ms(client).unwrap_or(local_ms);
    let totp = spotify_totp_code(local_ms, &secret.secret)?;
    let totp_server = spotify_totp_code(server_ms, &secret.secret)?;
    let url = format!(
        "https://open.spotify.com/api/token?reason=init&productType=mobile-web-player&totp={}&totpServer={}&totpVer={}",
        totp, totp_server, secret.version
    );
    let response = match client
        .get(&url)
        .header("Accept", "application/json")
        .header("Referer", "https://open.spotify.com/")
        .header("Origin", "https://open.spotify.com")
        .header("User-Agent", spotify_mobile_user_agent())
        .send()
    {
        Ok(r) => r,
        Err(e) => {
            log_file!(
                "[ListUrl][Spotify] TOTP token 请求失败: {}",
                reqwest_error_detail(&e)
            );
            return None;
        }
    };
    let status = response.status();
    let text = response.text().unwrap_or_default();
    log_file!(
        "[ListUrl][Spotify] TOTP token HTTP {} 响应(前200): {}",
        status.as_u16(),
        preview_for_log(&text, 200)
    );
    if !status.is_success() && !force_remote_secrets {
        return fetch_spotify_totp_access_token(client, true);
    }
    let v = serde_json::from_str::<serde_json::Value>(&text).ok()?;
    let token = pick_str_field(&v, &["accessToken", "access_token"])?;
    let expires_at_ms = v
        .get("accessTokenExpirationTimestampMs")
        .and_then(|x| x.as_i64())
        .unwrap_or_else(|| Local::now().timestamp_millis() + 3_000_000);
    store_spotify_access_token(token.clone(), expires_at_ms);
    Some(token)
}

fn fetch_spotify_server_time_ms(client: &reqwest::blocking::Client) -> Option<i64> {
    let text = client
        .get("https://open.spotify.com/api/server-time")
        .header("Accept", "application/json")
        .header("Referer", "https://open.spotify.com/")
        .header("Origin", "https://open.spotify.com")
        .header("User-Agent", spotify_mobile_user_agent())
        .send()
        .and_then(|r| r.text())
        .ok()?;
    let v = serde_json::from_str::<serde_json::Value>(&text).ok()?;
    let seconds = v
        .get("serverTime")
        .and_then(|x| x.as_f64())
        .or_else(|| v.get("serverTime").and_then(|x| x.as_str())?.parse().ok())?;
    Some((seconds * 1000.0) as i64)
}

fn spotify_totp_secret(
    client: &reqwest::blocking::Client,
    force_remote: bool,
) -> Option<SpotifyTotpSecret> {
    if let Some(secret) = spotify_totp_secret_from_env() {
        return Some(secret);
    }
    if force_remote {
        if let Some(secret) = fetch_spotify_totp_secret_remote(client) {
            return Some(secret);
        }
    }
    Some(SpotifyTotpSecret {
        version: 61,
        secret: vec![
            44, 55, 47, 42, 70, 40, 34, 114, 76, 74, 50, 111, 120, 97, 75, 76, 94, 102, 43, 69, 49,
            120, 118, 80, 64, 78,
        ],
    })
}

fn spotify_totp_secret_from_env() -> Option<SpotifyTotpSecret> {
    let version = std::env::var("SPOTIFY_TOTP_VER")
        .ok()
        .and_then(|s| s.trim().parse::<u32>().ok())?;
    let secret = std::env::var("SPOTIFY_TOTP_SECRET")
        .ok()
        .and_then(|s| parse_spotify_totp_secret_bytes(&s))?;
    Some(SpotifyTotpSecret { version, secret })
}

fn fetch_spotify_totp_secret_remote(
    client: &reqwest::blocking::Client,
) -> Option<SpotifyTotpSecret> {
    let url = std::env::var("SPOTIFY_TOTP_SECRET_URL").unwrap_or_else(|_| {
        "https://git.gay/manhgdev/totp-secrets/raw/branch/main/secrets/secretBytes.json".to_string()
    });
    let text = client.get(&url).send().and_then(|r| r.text()).ok()?;
    let arr = serde_json::from_str::<serde_json::Value>(&text).ok()?;
    let entries = arr.as_array()?;
    entries
        .iter()
        .filter_map(|item| {
            let version = item.get("version")?.as_u64()? as u32;
            let secret = item
                .get("secret")?
                .as_array()?
                .iter()
                .filter_map(|x| x.as_u64().map(|n| n as u8))
                .collect::<Vec<_>>();
            (!secret.is_empty()).then_some(SpotifyTotpSecret { version, secret })
        })
        .max_by_key(|item| item.version)
}

fn parse_spotify_totp_secret_bytes(raw: &str) -> Option<Vec<u8>> {
    let raw = raw.trim();
    if raw.starts_with('[') {
        return serde_json::from_str::<Vec<u8>>(raw).ok();
    }
    let bytes = raw
        .split([',', ' ', ';'])
        .filter(|s| !s.trim().is_empty())
        .filter_map(|s| s.trim().parse::<u8>().ok())
        .collect::<Vec<_>>();
    (!bytes.is_empty()).then_some(bytes)
}

fn spotify_totp_code(timestamp_ms: i64, secret_data: &[u8]) -> Option<String> {
    let key_string = secret_data
        .iter()
        .enumerate()
        .map(|(idx, value)| (value ^ (((idx % 33) + 9) as u8)).to_string())
        .collect::<String>();
    let counter = (timestamp_ms / 1000 / 30).max(0) as u64;
    let mut counter_bytes = [0u8; 8];
    counter_bytes.copy_from_slice(&counter.to_be_bytes());
    let hash = hmac_sha1(key_string.as_bytes(), &counter_bytes);
    let offset = (hash[hash.len() - 1] & 0x0f) as usize;
    let slice = hash.get(offset..offset + 4)?;
    let binary = u32::from_be_bytes([slice[0], slice[1], slice[2], slice[3]]) & 0x7fff_ffff;
    Some(format!("{:06}", binary % 1_000_000))
}

fn hmac_sha1(key: &[u8], message: &[u8]) -> [u8; 20] {
    const BLOCK_SIZE: usize = 64;
    let mut key_block = [0u8; BLOCK_SIZE];
    if key.len() > BLOCK_SIZE {
        key_block[..20].copy_from_slice(&sha1_digest(key));
    } else {
        key_block[..key.len()].copy_from_slice(key);
    }
    let mut ipad = [0x36u8; BLOCK_SIZE];
    let mut opad = [0x5cu8; BLOCK_SIZE];
    for i in 0..BLOCK_SIZE {
        ipad[i] ^= key_block[i];
        opad[i] ^= key_block[i];
    }
    let mut inner = Vec::with_capacity(BLOCK_SIZE + message.len());
    inner.extend_from_slice(&ipad);
    inner.extend_from_slice(message);
    let inner_hash = sha1_digest(&inner);
    let mut outer = Vec::with_capacity(BLOCK_SIZE + inner_hash.len());
    outer.extend_from_slice(&opad);
    outer.extend_from_slice(&inner_hash);
    sha1_digest(&outer)
}

fn sha1_digest(input: &[u8]) -> [u8; 20] {
    let mut h0: u32 = 0x6745_2301;
    let mut h1: u32 = 0xefcd_ab89;
    let mut h2: u32 = 0x98ba_dcfe;
    let mut h3: u32 = 0x1032_5476;
    let mut h4: u32 = 0xc3d2_e1f0;

    let bit_len = (input.len() as u64) * 8;
    let mut msg = input.to_vec();
    msg.push(0x80);
    while (msg.len() % 64) != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    for chunk in msg.chunks_exact(64) {
        let mut w = [0u32; 80];
        for i in 0..16 {
            let j = i * 4;
            w[i] = u32::from_be_bytes([chunk[j], chunk[j + 1], chunk[j + 2], chunk[j + 3]]);
        }
        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
        }

        let mut a = h0;
        let mut b = h1;
        let mut c = h2;
        let mut d = h3;
        let mut e = h4;

        for i in 0..80 {
            let (f, k) = match i {
                0..=19 => ((b & c) | ((!b) & d), 0x5a82_7999),
                20..=39 => (b ^ c ^ d, 0x6ed9_eba1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8f1b_bcdc),
                _ => (b ^ c ^ d, 0xca62_c1d6),
            };
            let temp = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(w[i]);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }

        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
    }

    let mut out = [0u8; 20];
    out[..4].copy_from_slice(&h0.to_be_bytes());
    out[4..8].copy_from_slice(&h1.to_be_bytes());
    out[8..12].copy_from_slice(&h2.to_be_bytes());
    out[12..16].copy_from_slice(&h3.to_be_bytes());
    out[16..20].copy_from_slice(&h4.to_be_bytes());
    out
}

fn spotify_client_token_from_env() -> Option<String> {
    std::env::var("SPOTIFY_CLIENT_TOKEN")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn spotify_user_agent() -> &'static str {
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.0 Safari/605.1.15"
}

fn spotify_mobile_user_agent() -> &'static str {
    "Mozilla/5.0 (iPhone; CPU iPhone OS 16_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.6 Mobile/15E148 Safari/604.1"
}

fn spotify_pathfinder_item_to_song(item: &serde_json::Value) -> Option<OnlineSong> {
    let track = item.pointer("/itemV2/data")?;
    if track.get("__typename").and_then(|x| x.as_str()) != Some("Track") {
        return None;
    }
    let name = pick_str_field(track, &["name"])?;
    let artist = track
        .pointer("/artists/items")
        .and_then(|x| x.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|artist| artist.pointer("/profile/name").and_then(|x| x.as_str()))
                .filter(|name| !name.trim().is_empty())
                .collect::<Vec<_>>()
                .join(" / ")
        })
        .filter(|s| !s.trim().is_empty())
        .or_else(|| {
            track
                .pointer("/albumOfTrack/artists/items/0/profile/name")
                .and_then(|x| x.as_str())
                .map(|s| s.to_string())
        })
        .unwrap_or_default();
    let duration_ms = track
        .pointer("/trackDuration/totalMilliseconds")
        .and_then(|x| x.as_i64());
    Some(unresolved_juhe_candidate_with_duration(
        name,
        artist,
        duration_ms,
    ))
}

fn spotify_web_api_track_to_song(track: &serde_json::Value) -> Option<OnlineSong> {
    if track.get("type").and_then(|x| x.as_str()) != Some("track") {
        return None;
    }
    let name = pick_str_field(track, &["name"])?;
    let artist = track
        .get("artists")
        .and_then(|x| x.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|artist| artist.get("name").and_then(|x| x.as_str()))
                .filter(|name| !name.trim().is_empty())
                .collect::<Vec<_>>()
                .join(" / ")
        })
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_default();
    let duration_ms = track.get("duration_ms").and_then(|x| x.as_i64());
    Some(unresolved_juhe_candidate_with_duration(
        name,
        artist,
        duration_ms,
    ))
}

fn spotify_collect_track_songs(v: &serde_json::Value) -> Vec<OnlineSong> {
    let mut out = Vec::new();
    spotify_collect_track_songs_inner(v, &mut out);
    dedup_online_songs_by_name_artist(out)
}

fn spotify_collect_track_songs_inner(v: &serde_json::Value, out: &mut Vec<OnlineSong>) {
    match v {
        serde_json::Value::Object(map) => {
            if let Some(song) = spotify_pathfinder_track_object_to_song(v) {
                out.push(song);
                return;
            }
            for value in map.values() {
                spotify_collect_track_songs_inner(value, out);
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                spotify_collect_track_songs_inner(item, out);
            }
        }
        _ => {}
    }
}

fn spotify_pathfinder_track_object_to_song(track: &serde_json::Value) -> Option<OnlineSong> {
    let typename = track.get("__typename").and_then(|x| x.as_str());
    let uri = track
        .get("uri")
        .and_then(|x| x.as_str())
        .unwrap_or_default();
    let is_track = typename == Some("Track")
        || typename == Some("TrackResponseWrapper")
        || typename == Some("TrackUnion")
        || track.get("type").and_then(|x| x.as_str()) == Some("track")
        || uri.starts_with("spotify:track:");
    if !is_track {
        return None;
    }
    if let Some(data) = track.get("data") {
        if !data.is_null() && !std::ptr::eq(data, track) {
            if let Some(song) = spotify_pathfinder_track_object_to_song(data) {
                return Some(song);
            }
        }
    }
    if let Some(track_union) = track.get("trackUnion") {
        if let Some(song) = spotify_pathfinder_track_object_to_song(track_union) {
            return Some(song);
        }
    }
    let name = pick_str_field(track, &["name", "title"])?;
    let artist = spotify_artist_names_from_pathfinder_track(track);
    let duration_ms = track
        .pointer("/trackDuration/totalMilliseconds")
        .and_then(|x| x.as_i64())
        .or_else(|| {
            track
                .pointer("/duration/totalMilliseconds")
                .and_then(|x| x.as_i64())
        })
        .or_else(|| track.get("duration_ms").and_then(|x| x.as_i64()))
        .or_else(|| {
            track
                .pointer("/duration/seconds")
                .and_then(|x| x.as_i64())
                .map(|seconds| seconds * 1000)
        });
    Some(unresolved_juhe_candidate_with_duration(
        name,
        artist,
        duration_ms,
    ))
}

fn spotify_artist_names_from_pathfinder_track(track: &serde_json::Value) -> String {
    for pointer in [
        "/artists/items",
        "/artistsWithRoles/items",
        "/firstArtist/items",
        "/albumOfTrack/artists/items",
    ] {
        if let Some(items) = track.pointer(pointer).and_then(|x| x.as_array()) {
            let names = items
                .iter()
                .filter_map(|item| {
                    item.pointer("/profile/name")
                        .or_else(|| item.pointer("/artist/profile/name"))
                        .or_else(|| item.get("name"))
                        .and_then(|x| x.as_str())
                })
                .filter(|name| !name.trim().is_empty())
                .collect::<Vec<_>>();
            if !names.is_empty() {
                return names.join(" / ");
            }
        }
    }
    String::new()
}

fn dedup_online_songs_by_name_artist(songs: Vec<OnlineSong>) -> Vec<OnlineSong> {
    let mut out = Vec::new();
    for song in songs {
        if song.name.trim().is_empty() {
            continue;
        }
        let duplicate = out.iter().any(|existing: &OnlineSong| {
            existing.name.eq_ignore_ascii_case(&song.name)
                && existing.artist.eq_ignore_ascii_case(&song.artist)
        });
        if !duplicate {
            out.push(song);
        }
    }
    out
}

fn parse_spotify_playlist_candidates(html: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut cursor = 0usize;
    while let Some(rel) = html[cursor..].find("data-testid=\"track-row\"") {
        let row_start = cursor + rel;
        let row_end = html[row_start + 1..]
            .find("data-testid=\"track-row\"")
            .map(|x| row_start + 1 + x)
            .unwrap_or_else(|| html.len().min(row_start + 6000));
        let row = &html[row_start..row_end];
        let title = extract_between(row, "<span class=\"e-10451-line-clamp\"", "</span>")
            .and_then(|s| s.rsplit('>').next().map(|x| x.to_string()))
            .map(|s| decode_html_entities_basic(&s))
            .or_else(|| {
                extract_html_attr_value(row, "aria-label").map(|s| decode_html_entities_basic(&s))
            })
            .unwrap_or_default();
        let artist = extract_between(row, "data-testid=\"internal-artist-link\"", "</a>")
            .and_then(|s| s.rsplit('>').next().map(|x| x.to_string()))
            .map(|s| decode_html_entities_basic(&s))
            .unwrap_or_default();
        if !title.trim().is_empty() {
            out.push((title.trim().to_string(), artist.trim().to_string()));
        }
        cursor = row_end;
    }
    dedup_song_artist_candidates(out)
}

fn apple_artist_top_songs_id(url: &str) -> Option<String> {
    let normalized = normalized_url_for_parse(url);
    let lower = normalized.to_ascii_lowercase();
    if !lower.contains("music.apple.com/")
        || !lower.contains("/artist/")
        || !lower.contains("/top-songs")
    {
        return None;
    }
    normalized
        .trim_end_matches('/')
        .split('/')
        .rev()
        .nth(1)
        .map(|s| s.to_string())
        .filter(|s| s.chars().all(|c| c.is_ascii_digit()))
}

fn apple_music_storefront_from_url(url: &str) -> Option<String> {
    let normalized = normalized_url_for_parse(url);
    let marker = "music.apple.com/";
    let pos = normalized.to_ascii_lowercase().find(marker)?;
    normalized[pos + marker.len()..]
        .split(&['/', '?', '&', '#'][..])
        .next()
        .map(|s| s.trim().to_ascii_lowercase())
        .filter(|s| {
            !s.is_empty()
                && s.len() <= 8
                && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
        })
}

fn apple_music_playlist_id_from_url(url: &str) -> Option<String> {
    let normalized = normalized_url_for_parse(url);
    let lower = normalized.to_ascii_lowercase();
    if !lower.contains("music.apple.com/") || !lower.contains("/playlist/") {
        return None;
    }
    normalized
        .split(&['/', '?', '&', '#'][..])
        .find(|part| part.starts_with("pl."))
        .map(|s| s.trim().to_string())
        .filter(|s| s.len() > 3)
}

fn apple_music_room_id_from_url(url: &str) -> Option<String> {
    let normalized = normalized_url_for_parse(url);
    let lower = normalized.to_ascii_lowercase();
    if !lower.contains("music.apple.com/") || !lower.contains("/room/") {
        return None;
    }
    let pos = lower.find("/room/")?;
    normalized[pos + "/room/".len()..]
        .split(&['/', '?', '&', '#'][..])
        .next()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty() && s.chars().all(|c| c.is_ascii_digit()))
}

fn apple_music_album_id_from_url(url: &str) -> Option<String> {
    let normalized = normalized_url_for_parse(url);
    let lower = normalized.to_ascii_lowercase();
    if !lower.contains("music.apple.com/") || !lower.contains("/album/") {
        return None;
    }
    normalized
        .trim_end_matches(&['/', '?', '#'][..])
        .split(&['/', '?', '&', '#'][..])
        .rev()
        .find(|part| part.chars().all(|c| c.is_ascii_digit()))
        .map(|s| s.to_string())
}

fn apple_music_is_top_charts_songs_url(url: &str) -> bool {
    let normalized = normalized_url_for_parse(url);
    let lower = normalized.to_ascii_lowercase();
    lower.contains("music.apple.com/") && lower.contains("/new/top-charts/songs")
}

fn apple_music_locale_for_storefront(storefront: &str) -> &'static str {
    match storefront {
        "cn" => "zh-Hans-CN",
        "tw" => "zh-Hant-TW",
        "hk" => "zh-Hant-HK",
        "jp" => "ja-JP",
        "kr" => "ko-KR",
        _ => "en-US",
    }
}

fn apple_music_browse_url_for_storefront(storefront: &str) -> String {
    format!("https://music.apple.com/{}/browse", storefront)
}

fn fetch_apple_music_entity_title(
    client: &reqwest::blocking::Client,
    url: &str,
) -> Option<String> {
    let storefront = apple_music_storefront_from_url(url).unwrap_or_else(|| "cn".to_string());
    let token = fetch_apple_music_developer_token(client, &storefront)?;
    let locale = apple_music_locale_for_storefront(&storefront);
    let api_url = if let Some(playlist_id) = apple_music_playlist_id_from_url(url) {
        format!(
            "https://amp-api.music.apple.com/v1/catalog/{}/playlists/{}?l={}&platform=web",
            storefront, playlist_id, locale
        )
    } else if let Some(album_id) = apple_music_album_id_from_url(url) {
        format!(
            "https://amp-api.music.apple.com/v1/catalog/{}/albums/{}?l={}&platform=web",
            storefront, album_id, locale
        )
    } else if let Some(artist_id) = apple_artist_top_songs_id(url) {
        format!(
            "https://amp-api.music.apple.com/v1/catalog/{}/artists/{}?l={}&platform=web",
            storefront, artist_id, locale
        )
    } else {
        return fetch_online_list_page_title(url);
    };
    log_file!("[ListUrl][AppleMusic] title API: {}", api_url);
    let text = client
        .get(&api_url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Origin", "https://music.apple.com")
        .header("Referer", url)
        .header("Accept", "application/json")
        .header("Accept-Language", locale)
        .send()
        .and_then(|r| r.text())
        .ok()?;
    log_file!(
        "[ListUrl][AppleMusic] title 响应(前200): {}",
        preview_for_log(&text, 200)
    );
    let v = serde_json::from_str::<serde_json::Value>(&text).ok()?;
    let title = v
        .pointer("/data/0/attributes/name")
        .and_then(|x| x.as_str())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .or_else(|| fetch_online_list_page_title(url));
    log_file!("[ListUrl][AppleMusic] 标题解析结果: {:?}", title);
    title
}

fn fetch_apple_music_playlist_songs_page(
    client: &reqwest::blocking::Client,
    url: &str,
    page: usize,
    page_size: usize,
) -> Option<Vec<OnlineSong>> {
    let playlist_id = apple_music_playlist_id_from_url(url)?;
    let storefront = apple_music_storefront_from_url(url).unwrap_or_else(|| "cn".to_string());
    let token = fetch_apple_music_developer_token(client, &storefront)?;
    let limit = page_size.max(1).min(100);
    let offset = page.saturating_sub(1).saturating_mul(limit);
    let locale = apple_music_locale_for_storefront(&storefront);
    let api_url = format!(
        "https://amp-api.music.apple.com/v1/catalog/{}/playlists/{}/tracks?l={}&offset={}&limit={}&art%5Burl%5D=f&extend=artistUrl&include=artists&platform=web",
        storefront, playlist_id, locale, offset, limit
    );
    log_file!("[ListUrl][AppleMusic] playlist tracks API: {}", api_url);
    let text = client
        .get(&api_url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Origin", "https://music.apple.com")
        .header("Referer", url)
        .header("Accept", "application/json")
        .send()
        .and_then(|r| r.text())
        .ok()?;
    log_file!(
        "[ListUrl][AppleMusic] playlist tracks 响应(前200): {}",
        preview_for_log(&text, 200)
    );
    let v = serde_json::from_str::<serde_json::Value>(&text).ok()?;
    let items = v.get("data").and_then(|d| d.as_array())?;
    let songs = items
        .iter()
        .filter_map(apple_music_catalog_song_to_online_song)
        .collect::<Vec<_>>();
    (!songs.is_empty()).then_some(songs)
}

fn apple_music_catalog_song_to_online_song(item: &serde_json::Value) -> Option<OnlineSong> {
    let item_type = item.get("type").and_then(|x| x.as_str())?;
    if item_type != "songs" && item_type != "music-videos" {
        return None;
    }
    let attrs = item.get("attributes")?;
    let name = pick_str_field(attrs, &["name"])?;
    let artist = pick_str_field(attrs, &["artistName"]).unwrap_or_default();
    let duration_ms = attrs.get("durationInMillis").and_then(|x| x.as_i64());
    Some(unresolved_juhe_candidate_with_duration(
        name,
        artist,
        duration_ms,
    ))
}

fn fetch_apple_music_room_songs_page(
    client: &reqwest::blocking::Client,
    url: &str,
    page: usize,
    page_size: usize,
) -> Option<Vec<OnlineSong>> {
    let room_id = apple_music_room_id_from_url(url)?;
    let storefront = apple_music_storefront_from_url(url).unwrap_or_else(|| "cn".to_string());
    let token = fetch_apple_music_developer_token(client, &storefront)?;
    let limit = page_size.max(1).min(100);
    let offset = page.saturating_sub(1).saturating_mul(limit);
    let locale = apple_music_locale_for_storefront(&storefront);
    let api_url = format!(
        "https://amp-api.music.apple.com/v1/editorial/{}/rooms/{}/contents?l={}&offset={}&limit={}&art%5Burl%5D=f&format%5Bresources%5D=map&include%5Bsongs%5D=artists&platform=web",
        storefront, room_id, locale, offset, limit
    );
    log_file!("[ListUrl][AppleMusic] room contents API: {}", api_url);
    let text = client
        .get(&api_url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Origin", "https://music.apple.com")
        .header("Referer", url)
        .header("Accept", "application/json")
        .header("Accept-Language", locale)
        .send()
        .and_then(|r| r.text())
        .ok()?;
    log_file!(
        "[ListUrl][AppleMusic] room contents 响应(前200): {}",
        preview_for_log(&text, 200)
    );
    let v = serde_json::from_str::<serde_json::Value>(&text).ok()?;
    let song_resources = v
        .get("resources")
        .and_then(|r| r.get("songs"))
        .and_then(|s| s.as_object())?;
    let items = v.get("data").and_then(|d| d.as_array())?;
    let songs = items
        .iter()
        .filter(|item| item.get("type").and_then(|x| x.as_str()) == Some("songs"))
        .filter_map(|item| {
            let id = item.get("id").and_then(|x| x.as_str())?;
            let song_resource = song_resources.get(id)?;
            apple_music_catalog_song_to_online_song(song_resource)
        })
        .collect::<Vec<_>>();
    (!songs.is_empty()).then_some(songs)
}

fn fetch_apple_music_top_chart_songs_page(
    client: &reqwest::blocking::Client,
    url: &str,
    page: usize,
    page_size: usize,
) -> Option<Vec<OnlineSong>> {
    if !apple_music_is_top_charts_songs_url(url) {
        return None;
    }
    let storefront = apple_music_storefront_from_url(url).unwrap_or_else(|| "cn".to_string());
    let token = fetch_apple_music_developer_token(client, &storefront)?;
    let limit = page_size.max(1).min(100);
    let offset = page.saturating_sub(1).saturating_mul(limit);
    let locale = apple_music_locale_for_storefront(&storefront);
    let api_url = format!(
        "https://amp-api.music.apple.com/v1/catalog/{}/charts?types=songs&chart=most-played&limit={}&offset={}&l={}&art%5Burl%5D=f&platform=web",
        storefront, limit, offset, locale
    );
    log_file!("[ListUrl][AppleMusic] top-charts API: {}", api_url);
    let text = client
        .get(&api_url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Origin", "https://music.apple.com")
        .header("Referer", url)
        .header("Accept", "application/json")
        .header("Accept-Language", locale)
        .send()
        .and_then(|r| r.text())
        .ok()?;
    log_file!(
        "[ListUrl][AppleMusic] top-charts 响应(前200): {}",
        preview_for_log(&text, 200)
    );
    let v = serde_json::from_str::<serde_json::Value>(&text).ok()?;
    let items = v
        .pointer("/results/songs/0/data")
        .and_then(|d| d.as_array())?;
    let songs = items
        .iter()
        .filter_map(apple_music_catalog_song_to_online_song)
        .collect::<Vec<_>>();
    (!songs.is_empty()).then_some(songs)
}

fn fetch_apple_music_album_songs_page(
    client: &reqwest::blocking::Client,
    url: &str,
    page: usize,
    page_size: usize,
) -> Option<Vec<OnlineSong>> {
    let album_id = apple_music_album_id_from_url(url)?;
    let storefront = apple_music_storefront_from_url(url).unwrap_or_else(|| "cn".to_string());
    let token = fetch_apple_music_developer_token(client, &storefront)?;
    let limit = page_size.max(1).min(100);
    let offset = page.saturating_sub(1).saturating_mul(limit);
    let locale = apple_music_locale_for_storefront(&storefront);
    let api_url = format!(
        "https://amp-api.music.apple.com/v1/catalog/{}/albums/{}/tracks?l={}&offset={}&limit={}&art%5Burl%5D=f&platform=web",
        storefront, album_id, locale, offset, limit
    );
    log_file!("[ListUrl][AppleMusic] album tracks API: {}", api_url);
    let text = client
        .get(&api_url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Origin", "https://music.apple.com")
        .header("Referer", url)
        .header("Accept", "application/json")
        .header("Accept-Language", locale)
        .send()
        .and_then(|r| r.text())
        .ok()?;
    log_file!(
        "[ListUrl][AppleMusic] album tracks 响应(前200): {}",
        preview_for_log(&text, 200)
    );
    let v = serde_json::from_str::<serde_json::Value>(&text).ok()?;
    let items = v.get("data").and_then(|d| d.as_array())?;
    let songs = items
        .iter()
        .filter_map(apple_music_catalog_song_to_online_song)
        .collect::<Vec<_>>();
    (!songs.is_empty()).then_some(songs)
}

fn fetch_apple_artist_top_songs_page(
    client: &reqwest::blocking::Client,
    url: &str,
    page: usize,
    page_size: usize,
) -> Option<Vec<OnlineSong>> {
    let artist_id = apple_artist_top_songs_id(url)?;
    let storefront = apple_music_storefront_from_url(url).unwrap_or_else(|| "cn".to_string());
    let token = fetch_apple_music_developer_token(client, &storefront)?;
    let limit = page_size.max(1);
    let offset = page.saturating_sub(1).saturating_mul(limit);
    let locale = apple_music_locale_for_storefront(&storefront);
    let api_url = format!(
        "https://amp-api.music.apple.com/v1/catalog/{}/artists/{}/view/top-songs?l={}&offset={}&art%5Burl%5D=f&extend=artistUrl&format%5Bresources%5D=map&include%5Bsongs%5D=artists%2Ccomposers&limit={}&platform=web",
        storefront, artist_id, locale, offset, limit
    );
    log_file!("[ListUrl][AppleMusic] top-songs API: {}", api_url);
    let text = client
        .get(&api_url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Origin", "https://music.apple.com")
        .header("Referer", url)
        .header("Accept", "application/json")
        .header("Accept-Language", locale)
        .send()
        .and_then(|r| r.text())
        .ok()?;
    log_file!(
        "[ListUrl][AppleMusic] top-songs 响应(前200): {}",
        preview_for_log(&text, 200)
    );
    let v = serde_json::from_str::<serde_json::Value>(&text).ok()?;
    let song_resources = v
        .get("resources")
        .and_then(|r| r.get("songs"))
        .and_then(|s| s.as_object())?;
    let songs = v
        .get("data")
        .and_then(|d| d.as_array())?
        .iter()
        .filter_map(|song| {
            let id = song.get("id").and_then(|x| x.as_str())?;
            let attrs = song_resources.get(id)?.get("attributes")?;
            let name = pick_str_field(attrs, &["name"])?;
            let artist = pick_str_field(attrs, &["artistName"]).unwrap_or_default();
            let duration_ms = attrs.get("durationInMillis").and_then(|x| x.as_i64());
            Some(unresolved_juhe_candidate_with_duration(
                name,
                artist,
                duration_ms,
            ))
        })
        .collect::<Vec<_>>();
    Some(songs)
}

fn unresolved_juhe_candidate_with_duration(
    name: String,
    artist: String,
    duration_ms: Option<i64>,
) -> OnlineSong {
    OnlineSong {
        name,
        artist,
        id: 0,
        hash: String::new(),
        duration_ms,
        source: MusicSource::Juhe,
        juhe_platform: String::new(),
        juhe_song_id: String::new(),
    }
}

fn extract_apple_music_asset_script_path(html: &str) -> Option<String> {
    for marker in ["src=\"/assets/index", "src='/assets/index", "/assets/index"] {
        if let Some(pos) = html.find(marker) {
            let rest = &html[pos + marker.len()..];
            let end = rest.find('"').or_else(|| rest.find('\''))?;
            let path = format!("/assets/index{}", &rest[..end]);
            if path.ends_with(".js") {
                return Some(path);
            }
        }
    }
    None
}

fn extract_apple_music_jwt_like_token(text: &str) -> Option<String> {
    let mut best: Option<String> = None;
    let mut start = None;

    let is_token_char = |c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.';

    for (idx, ch) in text.char_indices().chain(std::iter::once((text.len(), '\0'))) {
        if ch != '\0' && is_token_char(ch) {
            if start.is_none() {
                start = Some(idx);
            }
            continue;
        }

        if let Some(begin) = start.take() {
            let candidate = &text[begin..idx];
            let dot_count = candidate.chars().filter(|&c| c == '.').count();
            if dot_count == 2 && candidate.len() >= 100 {
                match &best {
                    Some(existing) if existing.len() >= candidate.len() => {}
                    _ => best = Some(candidate.to_string()),
                }
            }
        }
    }

    best
}

fn fetch_apple_music_developer_token(
    client: &reqwest::blocking::Client,
    storefront: &str,
) -> Option<String> {
    let browse_url = apple_music_browse_url_for_storefront(storefront);
    let locale = apple_music_locale_for_storefront(storefront);
    let html = client
        .get(&browse_url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Accept-Language", locale)
        .send()
        .and_then(|r| r.text())
        .ok()?;
    let js_path = extract_apple_music_asset_script_path(&html).or_else(|| {
        extract_between(&html, "src=\"/assets/index", "\"")
            .map(|suffix| format!("/assets/index{}", suffix))
    })?;
    let js_url = if js_path.starts_with("http://") || js_path.starts_with("https://") {
        js_path
    } else {
        format!("https://music.apple.com{}", js_path)
    };
    let js = client
        .get(&js_url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Accept-Language", locale)
        .send()
        .and_then(|r| r.text())
        .ok()?;
    extract_between(&js, "const Oc=\"", "\"")
        .or_else(|| extract_between(&js, "var Oc=\"", "\""))
        .or_else(|| extract_apple_music_jwt_like_token(&js))
}

fn parse_apple_music_playlist_candidates(html: &str) -> Vec<(String, String)> {
    for id in [
        "schema:music-playlist",
        "schema:album",
        "schema:music-album",
    ] {
        if let Some(json) = extract_script_json_by_id(html, id) {
            if let Some(candidates) = parse_apple_music_jsonld_candidates(&json) {
                return candidates;
            }
        }
    }
    for json in extract_all_jsonld_scripts(html) {
        if let Some(candidates) = parse_apple_music_jsonld_candidates(&json) {
            return candidates;
        }
    }
    if let Some(json) = extract_script_json_by_id(html, "serialized-server-data") {
        if let Some(candidates) = parse_apple_music_serialized_candidates(&json) {
            return candidates;
        }
    }
    parse_apple_music_track_lockups(html)
}

fn parse_apple_music_serialized_candidates(json: &str) -> Option<Vec<(String, String)>> {
    let v = serde_json::from_str::<serde_json::Value>(json).ok()?;
    let mut out = Vec::new();
    collect_apple_serialized_tracks(&v, &mut out);
    (!out.is_empty()).then(|| dedup_song_artist_candidates(out))
}

fn collect_apple_serialized_tracks(v: &serde_json::Value, out: &mut Vec<(String, String)>) {
    match v {
        serde_json::Value::Object(map) => {
            if map.get("itemKind").and_then(|x| x.as_str()) == Some("trackLockup") {
                if let Some(items) = map.get("items").and_then(|x| x.as_array()) {
                    for item in items {
                        if let Some(candidate) = apple_track_lockup_candidate(item) {
                            out.push(candidate);
                        }
                    }
                }
            }
            if map
                .get("contentDescriptor")
                .and_then(|d| d.get("kind"))
                .and_then(|x| x.as_str())
                == Some("song")
            {
                if let Some(candidate) = apple_track_lockup_candidate(v) {
                    out.push(candidate);
                }
            }
            for value in map.values() {
                collect_apple_serialized_tracks(value, out);
            }
        }
        serde_json::Value::Array(arr) => {
            for value in arr {
                collect_apple_serialized_tracks(value, out);
            }
        }
        _ => {}
    }
}

fn apple_track_lockup_candidate(item: &serde_json::Value) -> Option<(String, String)> {
    let url = item
        .get("contentDescriptor")
        .and_then(|d| d.get("url"))
        .and_then(|x| x.as_str());
    let title = url
        .and_then(song_title_from_url)
        .or_else(|| pick_str_field(item, &["title", "name"]))?;
    let artist = item
        .get("subtitleLinks")
        .and_then(|x| x.as_array())
        .and_then(|arr| arr.first())
        .and_then(|x| pick_str_field(x, &["title", "name"]))
        .or_else(|| {
            item.get("artistName")
                .and_then(|x| x.as_str())
                .map(|s| s.to_string())
        })
        .unwrap_or_default();
    Some((title, artist))
}

fn parse_apple_music_jsonld_candidates(json: &str) -> Option<Vec<(String, String)>> {
    let v = serde_json::from_str::<serde_json::Value>(json).ok()?;
    let arr = v
        .get("track")
        .or_else(|| v.get("tracks"))
        .or_else(|| v.get("itemListElement"))
        .and_then(|x| x.as_array())?;
    let candidates = arr
        .iter()
        .filter_map(|item| {
            let item = item.get("item").unwrap_or(item);
            let name = pick_str_field(item, &["name"]).or_else(|| {
                item.get("url")
                    .and_then(|u| u.as_str())
                    .and_then(song_title_from_url)
            })?;
            let artist = item
                .get("byArtist")
                .or_else(|| item.get("artist"))
                .or_else(|| item.get("creator"))
                .and_then(|a| {
                    if let Some(arr) = a.as_array() {
                        Some(
                            arr.iter()
                                .filter_map(|x| pick_str_field(x, &["name"]))
                                .collect::<Vec<_>>()
                                .join("/"),
                        )
                    } else {
                        pick_str_field(a, &["name"])
                    }
                })
                .unwrap_or_default();
            Some((name, artist))
        })
        .collect::<Vec<_>>();
    (!candidates.is_empty()).then(|| dedup_song_artist_candidates(candidates))
}

fn extract_all_jsonld_scripts(html: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cursor = 0usize;
    while let Some(rel) = html[cursor..].find("application/ld+json") {
        let pos = cursor + rel;
        let before = &html[..pos];
        let script_start = before.rfind("<script").unwrap_or(pos);
        let rest = &html[script_start..];
        if let Some(open_end) = rest.find('>') {
            let body = &rest[open_end + 1..];
            if let Some(end) = body.find("</script>") {
                out.push(decode_html_entities_basic(body[..end].trim()));
                cursor = script_start + open_end + 1 + end + "</script>".len();
                continue;
            }
        }
        cursor = pos + "application/ld+json".len();
    }
    out
}

fn parse_apple_music_track_lockups(html: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut cursor = 0usize;
    while let Some(rel) = html[cursor..].find("track-lockup") {
        let start = cursor + rel;
        let end = html[start + 1..]
            .find("track-lockup")
            .map(|x| start + 1 + x)
            .unwrap_or_else(|| html.len().min(start + 5000));
        let row = &html[start..end];
        let name = extract_html_attr_value(row, "title")
            .or_else(|| extract_between(row, "songs-list-row__song-name", "</"))
            .map(|s| decode_html_entities_basic(strip_html_tags(&s).trim().as_ref()))
            .unwrap_or_default();
        let artist = extract_between(row, "songs-list-row__by-line", "</")
            .map(|s| decode_html_entities_basic(strip_html_tags(&s).trim().as_ref()))
            .unwrap_or_default();
        if !name.is_empty() {
            out.push((name, artist));
        }
        cursor = end;
    }
    dedup_song_artist_candidates(out)
}

fn extract_script_json_by_id(html: &str, id: &str) -> Option<String> {
    let marker = format!("id={}", id);
    let pos = html
        .find(&marker)
        .or_else(|| html.find(&format!("id=\"{}\"", id)))?;
    let rest = &html[pos..];
    let open_end = rest.find('>')? + 1;
    let rest = &rest[open_end..];
    let end = rest.find("</script>")?;
    Some(decode_html_entities_basic(rest[..end].trim()))
}

fn extract_between(input: &str, start_marker: &str, end_marker: &str) -> Option<String> {
    let start = input.find(start_marker)? + start_marker.len();
    let rest = &input[start..];
    let end = rest.find(end_marker)?;
    Some(rest[..end].to_string())
}

fn song_title_from_url(url: &str) -> Option<String> {
    let part = url.split("/song/").nth(1)?.split('/').next()?;
    Some(url_decode_lossy(part).replace('-', " ").trim().to_string()).filter(|s| !s.is_empty())
}

fn fetch_online_list_url_with_page(
    input: &str,
    page: usize,
    page_size: usize,
) -> PlaylistSongsResult {
    let kind = parse_online_list_url(input).unwrap_or_else(|e| OnlineListUrlKind::Unsupported(e));
    let mut playlist = playlist_from_url_kind(&kind, input);
    let client = create_search_client();
    let songs = match (&kind, client.as_ref()) {
        (OnlineListUrlKind::Playlist(PlaylistSource::Kuwo, id), Some(client)) => {
            fetch_kuwo_playlist_songs_page(client, id, page, page_size)
        }
        (OnlineListUrlKind::Playlist(PlaylistSource::Kugou, id), Some(client))
            if id.starts_with("gcid_") =>
        {
            fetch_kugou_gcid_playlist_songs(client, id)
        }
        (OnlineListUrlKind::Playlist(_, _), _) => fetch_playlist_songs(&playlist),
        (OnlineListUrlKind::Rank(PlaylistSource::NetEase, Some(id)), Some(client)) => {
            fetch_netease_playlist_songs(client, id)
        }
        (OnlineListUrlKind::Rank(PlaylistSource::Kuwo, id), Some(client)) => {
            fetch_kuwo_rank_songs(client, id.as_deref(), page, page_size)
        }
        (OnlineListUrlKind::Rank(PlaylistSource::Kugou, id), Some(client)) => {
            fetch_kugou_rank_songs_page(client, id.as_deref(), page, page_size)
        }
        (OnlineListUrlKind::Artist(PlaylistSource::Kuwo, id), Some(client)) => {
            fetch_kuwo_artist_songs_page(client, id, page, page_size)
        }
        (OnlineListUrlKind::Artist(PlaylistSource::NetEase, id), Some(client)) => {
            fetch_netease_artist_songs(client, id)
        }
        (OnlineListUrlKind::Artist(PlaylistSource::Kugou, id), Some(client)) => {
            fetch_kugou_artist_songs(client, id)
        }
        (OnlineListUrlKind::External(source, id), _) => {
            let result = fetch_external_playlist_songs_with_title(*source, id, page, page_size);
            if let Some(title) = result.title {
                if !title.trim().is_empty() {
                    playlist.name = title;
                }
            }
            result.songs
        }
        _ => Vec::new(),
    };

    PlaylistSongsResult { playlist, songs }
}

// ============================================================
// 评论拉取逻辑（网易）
// ============================================================

/// 获取歌曲评论（通过歌曲名先搜索歌曲ID）
fn fetch_song_comments(query: &str, page: usize, page_size: usize) -> SongCommentsResult {
    let safe_page = page.max(1);
    let safe_page_size = page_size.max(1);

    let empty = SongCommentsResult {
        page: safe_page,
        total: 0,
        comments: Vec::new(),
    };

    if query.trim().is_empty() {
        return empty;
    }

    let client = match create_search_client() {
        Some(c) => c,
        None => return empty,
    };

    // 先用歌曲名搜索一个网易歌曲ID
    let search_url = format!(
        "https://music.163.com/api/search/get?s={}&type=1&limit=1&offset=0",
        urlencoding::encode(query)
    );

    let song_id = client
        .get(&search_url)
        .header("Referer", "https://music.163.com/")
        .header("Cookie", "MUSIC_U=; appver=2.0.2;")
        .send()
        .ok()
        .and_then(|resp| resp.text().ok())
        .and_then(|text| serde_json::from_str::<NetEaseSearchResult>(&text).ok())
        .and_then(|res| res.result)
        .and_then(|res| res.songs)
        .and_then(|songs| songs.into_iter().next())
        .map(|song| song.id);

    let song_id = match song_id {
        Some(id) => id,
        None => return empty,
    };

    let offset = (safe_page - 1) * safe_page_size;
    let comments_url = format!(
        "https://music.163.com/api/v1/resource/comments/R_SO_4_{}?limit={}&offset={}",
        song_id, safe_page_size, offset
    );

    let value = match client
        .get(&comments_url)
        .header("Referer", "https://music.163.com/")
        .header("Cookie", "MUSIC_U=; appver=2.0.2;")
        .send()
        .ok()
        .and_then(|resp| resp.text().ok())
        .and_then(|text| serde_json::from_str::<serde_json::Value>(&text).ok())
    {
        Some(v) => v,
        None => return empty,
    };

    let total = value.get("total").and_then(|v| v.as_u64()).unwrap_or(0) as usize;

    let comments = value
        .get("comments")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|item| {
                    let nickname = item
                        .get("user")
                        .and_then(|u| u.get("nickname"))
                        .and_then(|n| n.as_str())
                        .unwrap_or("匿名用户")
                        .trim()
                        .to_string();

                    let content = item
                        .get("content")
                        .and_then(|c| c.as_str())
                        .unwrap_or("")
                        .replace('\n', " ")
                        .trim()
                        .to_string();

                    if content.is_empty() {
                        return None;
                    }

                    let time_text = item
                        .get("time")
                        .and_then(|t| t.as_i64())
                        .and_then(format_datetime_from_millis)
                        .or_else(|| {
                            item.get("timeStr")
                                .and_then(|t| t.as_str())
                                .map(|s| s.trim().to_string())
                                .filter(|s| !s.is_empty())
                        });

                    let reply = item
                        .get("beReplied")
                        .and_then(|r| r.as_array())
                        .and_then(|arr| arr.first())
                        .and_then(|first| {
                            let reply_content = first
                                .get("content")
                                .and_then(|c| c.as_str())
                                .map(|s| s.replace('\n', " ").trim().to_string())
                                .filter(|s| !s.is_empty());

                            reply_content.map(|content| {
                                let nickname = first
                                    .get("user")
                                    .and_then(|u| u.get("nickname"))
                                    .and_then(|n| n.as_str())
                                    .unwrap_or("匿名用户")
                                    .trim()
                                    .to_string();

                                let time_text = first
                                    .get("time")
                                    .and_then(|t| t.as_i64())
                                    .and_then(format_datetime_from_millis)
                                    .or_else(|| {
                                        first
                                            .get("timeStr")
                                            .and_then(|t| t.as_str())
                                            .map(|s| s.trim().to_string())
                                            .filter(|s| !s.is_empty())
                                    });

                                SongCommentReply {
                                    nickname,
                                    content,
                                    time_text,
                                }
                            })
                        });

                    Some(SongCommentItem {
                        nickname,
                        content,
                        time_text,
                        reply,
                    })
                })
                .collect::<Vec<SongCommentItem>>()
        })
        .unwrap_or_default();

    SongCommentsResult {
        page: safe_page,
        total,
        comments,
    }
}

// ============================================================
// 下载逻辑
// ============================================================

/// 下载歌曲到本地（带进度回调）
fn download_song_with_progress<F>(
    song: &OnlineSong,
    save_dir: &PathBuf,
    on_progress: F,
) -> DownloadResult
where
    F: Fn(u8) + Send + Sync,
{
    log_file!(
        "[Download] 开始下载: {} - {}, source={:?}, juhe_platform={}, juhe_song_id={}",
        song.artist,
        song.name,
        song.source,
        song.juhe_platform,
        song.juhe_song_id
    );

    let client = match create_download_client() {
        Some(c) => c,
        None => {
            log_file!("[Download] 创建HTTP客户端失败");
            return DownloadResult {
                song: song.clone(),
                local_path: None,
                error: Some(
                    crate::langs::global_texts()
                        .fmt_http_client_failed
                        .replace("{}", "")
                        .to_string(),
                ),
            };
        }
    };

    let song = if song.is_unresolved_juhe_candidate() {
        log_file!(
            "[Download] Juhe 参数为空，尝试重新解析: {} - {}",
            song.artist,
            song.name
        );
        match resolve_unresolved_juhe_song(song) {
            Some(resolved) if !resolved.is_unresolved_juhe_candidate() => {
                log_file!(
                    "[Download] Juhe 参数解析成功: platform={}, song_id={}",
                    resolved.juhe_platform,
                    resolved.juhe_song_id
                );
                resolved
            }
            _ => {
                log_file!(
                    "[Download] Juhe 参数解析失败: {} - {}",
                    song.artist,
                    song.name
                );
                return DownloadResult {
                    song: song.clone(),
                    local_path: None,
                    error: Some(
                        crate::langs::global_texts()
                            .no_download_link_vip
                            .to_string(),
                    ),
                };
            }
        }
    } else {
        song.clone()
    };

    // 根据来源平台获取下载链接
    let mp3_url = match song.source {
        MusicSource::Kuwo => get_kuwo_download_url(&client, song.id),
        MusicSource::Kugou => get_kugou_download_url(&client, &song.hash),
        MusicSource::NetEase => get_netease_download_url(&client, song.id),
        MusicSource::Juhe => get_juhe_download_url(&client, &song),
    };

    let mp3_url = match mp3_url {
        Some(u) if !u.is_empty() => u,
        _ => {
            log_file!("[Download] 无法获取下载链接, source={:?}", song.source);
            return DownloadResult {
                song: song.clone(),
                local_path: None,
                error: Some(
                    crate::langs::global_texts()
                        .no_download_link_vip
                        .to_string(),
                ),
            };
        }
    };

    log_file!("[Download] 获取到URL: {}...", preview_for_log(&mp3_url, 80));
    on_progress(5);

    // 下载音频文件（流式读取以支持进度）
    let referer = match song.source {
        MusicSource::Kuwo | MusicSource::Kugou => "https://www.kuwo.cn/",
        MusicSource::NetEase => "https://music.163.com/",
        MusicSource::Juhe => "https://www.kuwo.cn/",
    };
    let response = match client.get(&mp3_url).header("Referer", referer).send() {
        Ok(r) => r,
        Err(e) => {
            return DownloadResult {
                song: song.clone(),
                local_path: None,
                error: Some(
                    crate::langs::global_texts()
                        .fmt_download_request_failed
                        .replace("{}", &e.to_string()),
                ),
            }
        }
    };

    // 检查 Content-Type
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_lowercase();

    if content_type.contains("text/html") || content_type.contains("text/plain") {
        return DownloadResult {
            song: song.clone(),
            local_path: None,
            error: Some(
                crate::langs::global_texts()
                    .download_vip_required
                    .to_string(),
            ),
        };
    }

    // 获取总大小
    let total_size = response
        .headers()
        .get("content-length")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok());

    on_progress(10);

    // 流式读取响应体
    let mut downloaded: u64 = 0;
    let mut all_bytes = Vec::new();
    let mut last_reported_percent: u8 = 10;

    // 使用 chunk 读取
    use std::io::Read;
    let mut reader = response;
    let mut buffer = [0u8; 8192];

    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                all_bytes.extend_from_slice(&buffer[..n]);
                downloaded += n as u64;

                if let Some(total) = total_size {
                    if total > 0 {
                        let percent = (downloaded as f64 / total as f64 * 85.0) as u8 + 10; // 10-95%
                        let percent = percent.min(95);
                        if percent > last_reported_percent {
                            last_reported_percent = percent;
                            on_progress(percent);
                        }
                    }
                }
            }
            Err(e) => {
                return DownloadResult {
                    song: song.clone(),
                    local_path: None,
                    error: Some(
                        crate::langs::global_texts()
                            .fmt_read_response_failed
                            .replace("{}", &e.to_string()),
                    ),
                }
            }
        }
    }

    on_progress(96);

    // 验证下载数据是否为有效音频
    if let Err(e) = validate_audio_data(&all_bytes) {
        return DownloadResult {
            song: song.clone(),
            local_path: None,
            error: Some(e),
        };
    }

    // 确保保存目录存在
    if let Err(e) = std::fs::create_dir_all(save_dir) {
        return DownloadResult {
            song: song.clone(),
            local_path: None,
            error: Some(
                crate::langs::global_texts()
                    .fmt_mkdir_failed
                    .replace("{}", &e.to_string()),
            ),
        };
    }

    on_progress(98);

    // 生成文件名
    let filename = if song.artist.is_empty() {
        format!("{}.mp3", sanitize_filename(&song.name))
    } else {
        format!(
            "{} - {}.mp3",
            sanitize_filename(&song.artist),
            sanitize_filename(&song.name)
        )
    };

    let save_path = save_dir.join(&filename);

    match std::fs::write(&save_path, &all_bytes) {
        Ok(_) => {
            on_progress(100);

            // 歌词不在下载阶段处理；统一在播放阶段按“常规API优先、聚合兜底”自动下载

            DownloadResult {
                song: song.clone(),
                local_path: Some(save_path),
                error: None,
            }
        }
        Err(e) => DownloadResult {
            song: song.clone(),
            local_path: None,
            error: Some(
                crate::langs::global_texts()
                    .fmt_write_file_failed
                    .replace("{}", &e.to_string()),
            ),
        },
    }
}

/// 下载歌曲到本地（同步，无进度回调）
#[allow(dead_code)]
fn download_song(song: &OnlineSong, save_dir: &PathBuf) -> DownloadResult {
    let client = match create_download_client() {
        Some(c) => c,
        None => {
            return DownloadResult {
                song: song.clone(),
                local_path: None,
                error: Some(
                    crate::langs::global_texts()
                        .fmt_http_client_failed
                        .replace("{}", "")
                        .to_string(),
                ),
            }
        }
    };

    // 根据来源平台获取下载链接
    let mp3_url = match song.source {
        MusicSource::Kuwo => get_kuwo_download_url(&client, song.id),
        MusicSource::Kugou => get_kugou_download_url(&client, &song.hash),
        MusicSource::NetEase => get_netease_download_url(&client, song.id),
        MusicSource::Juhe => get_juhe_download_url(&client, song),
    };

    let mp3_url = match mp3_url {
        Some(u) if !u.is_empty() => u,
        _ => {
            return DownloadResult {
                song: song.clone(),
                local_path: None,
                error: Some(
                    crate::langs::global_texts()
                        .no_download_link_vip
                        .to_string(),
                ),
            }
        }
    };

    // 下载音频文件
    let referer = match song.source {
        MusicSource::Kuwo | MusicSource::Kugou => "https://www.kuwo.cn/",
        MusicSource::NetEase => "https://music.163.com/",
        MusicSource::Juhe => "https://www.kuwo.cn/",
    };
    let response = match client.get(&mp3_url).header("Referer", referer).send() {
        Ok(r) => r,
        Err(e) => {
            return DownloadResult {
                song: song.clone(),
                local_path: None,
                error: Some(
                    crate::langs::global_texts()
                        .fmt_download_request_failed
                        .replace("{}", &e.to_string()),
                ),
            }
        }
    };

    // 检查 Content-Type
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_lowercase();

    if content_type.contains("text/html") || content_type.contains("text/plain") {
        return DownloadResult {
            song: song.clone(),
            local_path: None,
            error: Some(
                crate::langs::global_texts()
                    .download_vip_required
                    .to_string(),
            ),
        };
    }

    let bytes = match response.bytes() {
        Ok(b) => b,
        Err(e) => {
            return DownloadResult {
                song: song.clone(),
                local_path: None,
                error: Some(
                    crate::langs::global_texts()
                        .fmt_read_response_failed
                        .replace("{}", &e.to_string()),
                ),
            }
        }
    };

    // 验证下载数据是否为有效音频
    if let Err(e) = validate_audio_data(&bytes) {
        return DownloadResult {
            song: song.clone(),
            local_path: None,
            error: Some(e),
        };
    }

    // 确保保存目录存在
    if let Err(e) = std::fs::create_dir_all(save_dir) {
        return DownloadResult {
            song: song.clone(),
            local_path: None,
            error: Some(
                crate::langs::global_texts()
                    .fmt_mkdir_failed
                    .replace("{}", &e.to_string()),
            ),
        };
    }

    // 生成文件名
    let filename = if song.artist.is_empty() {
        format!("{}.mp3", sanitize_filename(&song.name))
    } else {
        format!(
            "{} - {}.mp3",
            sanitize_filename(&song.artist),
            sanitize_filename(&song.name)
        )
    };

    let save_path = save_dir.join(&filename);

    match std::fs::write(&save_path, &bytes) {
        Ok(_) => DownloadResult {
            song: song.clone(),
            local_path: Some(save_path),
            error: None,
        },
        Err(e) => DownloadResult {
            song: song.clone(),
            local_path: None,
            error: Some(
                crate::langs::global_texts()
                    .fmt_write_file_failed
                    .replace("{}", &e.to_string()),
            ),
        },
    }
}

// ============================================================
// 下载链接获取
// ============================================================

/// 获取酷我音乐下载链接
fn get_kuwo_download_url(client: &reqwest::blocking::Client, rid: i64) -> Option<String> {
    // 酷我音乐播放链接API
    // type: convert_url3 支持更多格式
    // br: 128kmp3 / 192kmp3 / 320kmp3
    let url = format!(
        "https://www.kuwo.cn/api/v1/www/music/playInfo?mid={}&type=music&httpsStatus=1",
        rid
    );

    if let Ok(response) = client.get(&url)
        .header("Referer", "https://www.kuwo.cn/play_detail")
        .header("Cookie", "kw_token=1ABCDEF0; Hm_lvt_cdb524f42f0ce19b169a8071123a4797=1700000000; Hm_lpvt_cdb524f42f0ce19b169a8071123a4797=1700000000;")
        .send()
    {
        if let Ok(text) = response.text() {
            if let Ok(result) = serde_json::from_str::<KuwoPlayResult>(&text) {
                if let Some(url) = result.url {
                    if !url.is_empty() {
                        return Some(url);
                    }
                }
            }
        }
    }

    // 备用：使用 kuwoyy 解析
    let url2 = format!(
        "https://kuwo.cn/api/v1/www/music/playInfo?mid={}&type=convert_url3&br=128kmp3",
        rid
    );

    if let Ok(response) = client.get(&url2)
        .header("Referer", "https://www.kuwo.cn/")
        .header("Cookie", "kw_token=1ABCDEF0; Hm_lvt_cdb524f42f0ce19b169a8071123a4797=1700000000; Hm_lpvt_cdb524f42f0ce19b169a8071123a4797=1700000000;")
        .send()
    {
        if let Ok(text) = response.text() {
            if let Ok(result) = serde_json::from_str::<KuwoPlayResult>(&text) {
                if let Some(url) = result.url {
                    if !url.is_empty() {
                        return Some(url);
                    }
                }
            }
        }
    }

    None
}

/// 获取酷狗音乐下载链接
fn get_kugou_download_url(client: &reqwest::blocking::Client, hash: &str) -> Option<String> {
    // 方式1: 通过 getSongInfo 获取播放链接
    let url = format!(
        "https://m.kugou.com/app/i/getSongInfo.php?cmd=playInfo&hash={}",
        hash
    );

    if let Ok(response) = client.get(&url)
        .header("Referer", "https://m.kugou.com/")
        .header("User-Agent", "Mozilla/5.0 (iPhone; CPU iPhone OS 16_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.0 Mobile/15E148 Safari/604.1")
        .send()
    {
        if let Ok(text) = response.text() {
            if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) {
                // 尝试从 url 字段获取
                if let Some(url) = value.get("url").and_then(|v| v.as_str()) {
                    if !url.is_empty() {
                        return Some(url.to_string());
                    }
                }
                // 尝试从 backup_url 字段获取
                if let Some(url) = value.get("backup_url").and_then(|v| v.as_str()) {
                    if !url.is_empty() {
                        return Some(url.to_string());
                    }
                }
            }
        }
    }

    // 方式2: 通过 trackercdn 获取下载链接
    // key = MD5(hash + "kgcloud")
    let key_input = format!("{}kgcloud", hash);
    let key = format!("{:x}", md5::compute(key_input.as_bytes()));
    let url2 = format!(
        "http://trackercdn.kugou.com/i/?cmd=4&hash={}&key={}&pid=1&forceDown=0&vip=1",
        hash, key
    );

    if let Ok(response) = client
        .get(&url2)
        .header("Referer", "https://m.kugou.com/")
        .send()
    {
        if let Ok(text) = response.text() {
            if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) {
                if let Some(url) = value.get("url").and_then(|v| v.as_str()) {
                    if !url.is_empty() {
                        return Some(url.to_string());
                    }
                }
            }
        }
    }

    None
}

/// 获取网易音乐下载链接（备用）
fn get_netease_download_url(client: &reqwest::blocking::Client, song_id: i64) -> Option<String> {
    // 方式1: 使用歌曲详情 API
    let url_api = format!(
        "https://music.163.com/api/song/enhance/player/url?id={}&ids=[{}]&level=standard&encodeType=aac",
        song_id, song_id
    );

    if let Ok(response) = client
        .get(&url_api)
        .header("Referer", "https://music.163.com/")
        .header("Cookie", "MUSIC_U=; appver=2.0.2;")
        .send()
    {
        if let Ok(text) = response.text() {
            if let Ok(detail) = serde_json::from_str::<NetEaseSongDetail>(&text) {
                if let Some(data) = detail.data {
                    if let Some(first) = data.first() {
                        let code = first.code.unwrap_or(0);
                        if code == 200 {
                            if let Some(url) = &first.url {
                                if !url.is_empty() {
                                    return Some(url.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // 方式2: outer/url 重定向
    let redirect_url = format!(
        "https://music.163.com/song/media/outer/url?id={}.mp3",
        song_id
    );

    if let Ok(response) = client
        .get(&redirect_url)
        .header("Referer", "https://music.163.com/")
        .send()
    {
        let final_url = response.url().to_string();
        if final_url.contains("126.net") {
            return Some(final_url);
        }
        let ct = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_lowercase();
        if ct.contains("audio") {
            return Some(redirect_url);
        }
    }

    None
}

// ============================================================
// 工具函数
// ============================================================

/// 验证下载的数据是否为有效音频
fn validate_audio_data(bytes: &[u8]) -> Result<(), String> {
    if bytes.len() < 4 {
        return Err(crate::langs::global_texts()
            .download_data_too_small
            .to_string());
    }

    let header = &bytes[0..4];

    // 常见音频文件头标识
    let is_audio = header[0] == 0xFF && (header[1] & 0xE0) == 0xE0  // MP3 frame sync (0xFF 0xFB/0xF3/0xF2...)
        || header[0] == 0x49 && header[1] == 0x44 && header[2] == 0x33  // ID3 tag
        || header[0] == 0x66 && header[1] == 0x74 && header[2] == 0x79 && header[3] == 0x70  // ftyp (M4A/AAC)
        || header[0] == 0x52 && header[1] == 0x49 && header[2] == 0x46 && header[3] == 0x46  // RIFF (WAV)
        || header[0] == 0x4F && header[1] == 0x67 && header[2] == 0x67 && header[3] == 0x53  // OggS
        || header[0] == 0x66 && header[1] == 0x4C && header[2] == 0x61 && header[3] == 0x43; // fLaC

    if is_audio {
        return Ok(());
    }

    // 检查是否是 HTML 内容
    let check_len = std::cmp::min(200, bytes.len());
    if let Ok(text) = std::str::from_utf8(&bytes[0..check_len]) {
        let lower = text.to_lowercase();
        if lower.contains("<!doctype")
            || lower.contains("<html")
            || lower.contains("<head")
            || lower.contains("抱歉")
            || lower.contains("not found")
            || lower.contains("error")
        {
            return Err(crate::langs::global_texts()
                .download_not_audio_vip
                .to_string());
        }
    }

    // 无法识别的文件头，但可能仍是音频（某些格式）
    // 如果文件大小足够大（>10KB），则视为可能是有效音频
    if bytes.len() > 10240 {
        Ok(())
    } else {
        Err(crate::langs::global_texts()
            .download_data_not_audio
            .to_string())
    }
}

/// 清理文件名中的非法字符
fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect()
}

// ============================================================
// GitHub Discussions API
// ============================================================

/// GitHub Discussion 创建结果
#[derive(Debug, Clone)]
pub struct GitHubDiscussionResult {
    /// 创建成功时的 Discussion URL
    pub url: Option<String>,
    /// 错误信息
    pub error: Option<String>,
}

/// 在后台线程中创建 GitHub Discussion
pub fn create_github_discussion_background(
    github_token: String,
    github_repo: String,
    title: String,
    body: String,
) -> mpsc::Receiver<GitHubDiscussionResult> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let result = create_github_discussion(&github_token, &github_repo, &title, &body);
        let _ = tx.send(result);
    });
    rx
}

/// 创建 GitHub Discussion（show-and-tell 类别）
fn create_github_discussion(
    github_token: &str,
    github_repo: &str,
    title: &str,
    body: &str,
) -> GitHubDiscussionResult {
    if github_token.trim().is_empty() {
        return GitHubDiscussionResult {
            url: None,
            error: Some(
                crate::langs::global_texts()
                    .github_token_not_configured
                    .to_string(),
            ),
        };
    }

    let client = match reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .user_agent("TerMusicRust/2.0.0")
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            return GitHubDiscussionResult {
                url: None,
                error: Some(
                    crate::langs::global_texts()
                        .fmt_http_client_failed
                        .replace("{}", &e.to_string()),
                ),
            }
        }
    };

    // 解析 owner/repo
    let parts: Vec<&str> = github_repo.trim().split('/').collect();
    if parts.len() != 2 {
        return GitHubDiscussionResult {
            url: None,
            error: Some(
                crate::langs::global_texts()
                    .fmt_github_repo_format_error
                    .replace("{}", github_repo),
            ),
        };
    }
    let owner = parts[0];
    let repo_name = parts[1];

    // 第一步：列出仓库最近的 Discussions，按标题精确匹配检查是否已存在
    // （不使用 search API，因为搜索索引有延迟且对中文标题匹配不可靠）
    let check_query = json!({
        "query": format!(
            r#"{{
                repository(owner: "{}", name: "{}") {{
                    discussions(first: 100, orderBy: {{field: CREATED_AT, direction: DESC}}) {{
                        nodes {{
                            title
                            url
                        }}
                    }}
                }}
            }}"#,
            owner, repo_name
        )
    });

    let mut existing_url: Option<String> = None;
    if let Ok(response) = client
        .post("https://api.github.com/graphql")
        .header("Authorization", format!("Bearer {}", github_token.trim()))
        .json(&check_query)
        .send()
    {
        if let Ok(resp_json) = response.json::<serde_json::Value>() {
            if let Some(nodes) = resp_json
                .get("data")
                .and_then(|d| d.get("repository"))
                .and_then(|r| r.get("discussions"))
                .and_then(|d| d.get("nodes"))
                .and_then(|n| n.as_array())
            {
                for node in nodes {
                    let existing_title = node.get("title").and_then(|t| t.as_str()).unwrap_or("");
                    if existing_title.trim() == title.trim() {
                        if let Some(url) = node.get("url").and_then(|u| u.as_str()) {
                            existing_url = Some(url.to_string());
                            break;
                        }
                    }
                }
            }
        }
    }

    if let Some(url) = existing_url {
        return GitHubDiscussionResult {
            url: Some(url),
            error: None,
        };
    }

    // 第二步：获取仓库 ID 和 discussion category ID
    let query_repo = json!({
        "query": format!(
            r#"{{
                repository(owner: "{}", name: "{}") {{
                    id
                    discussionCategories(first: 20) {{
                        nodes {{
                            id
                            name
                            slug
                        }}
                    }}
                }}
            }}"#,
            owner, repo_name
        )
    });

    let response = match client
        .post("https://api.github.com/graphql")
        .header("Authorization", format!("Bearer {}", github_token.trim()))
        .json(&query_repo)
        .send()
    {
        Ok(r) => r,
        Err(e) => {
            return GitHubDiscussionResult {
                url: None,
                error: Some(
                    crate::langs::global_texts()
                        .fmt_github_api_request_failed
                        .replace("{}", &e.to_string()),
                ),
            }
        }
    };

    let status = response.status();
    if !status.is_success() {
        let text = response.text().unwrap_or_default();
        return GitHubDiscussionResult {
            url: None,
            error: Some(
                crate::langs::global_texts()
                    .fmt_github_api_http_error
                    .replace("{}", &status.as_u16().to_string())
                    .replace("{}", &text.chars().take(200).collect::<String>()),
            ),
        };
    }

    let resp_json: serde_json::Value = match response.json() {
        Ok(v) => v,
        Err(e) => {
            return GitHubDiscussionResult {
                url: None,
                error: Some(
                    crate::langs::global_texts()
                        .fmt_github_api_parse_failed
                        .replace("{}", &e.to_string()),
                ),
            }
        }
    };

    // 检查 GraphQL 错误
    if let Some(errors) = resp_json.get("errors") {
        let msg = errors
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|e| e.get("message"))
            .and_then(|m| m.as_str())
            .unwrap_or(crate::langs::global_texts().unknown_graphql_error);
        return GitHubDiscussionResult {
            url: None,
            error: Some(
                crate::langs::global_texts()
                    .fmt_github_graphql_error
                    .replace("{}", msg),
            ),
        };
    }

    let repository = match resp_json.get("data").and_then(|d| d.get("repository")) {
        Some(r) => r,
        None => {
            return GitHubDiscussionResult {
                url: None,
                error: Some(
                    crate::langs::global_texts()
                        .github_repo_not_found
                        .to_string(),
                ),
            }
        }
    };

    let repo_id = match repository.get("id").and_then(|v| v.as_str()) {
        Some(id) => id.to_string(),
        None => {
            return GitHubDiscussionResult {
                url: None,
                error: Some(
                    crate::langs::global_texts()
                        .github_repo_id_failed
                        .to_string(),
                ),
            }
        }
    };

    // 查找 "show-and-tell" 类别
    let category_id = repository
        .get("discussionCategories")
        .and_then(|c| c.get("nodes"))
        .and_then(|n| n.as_array())
        .and_then(|nodes| {
            nodes.iter().find_map(|node| {
                let slug = node.get("slug").and_then(|s| s.as_str()).unwrap_or("");
                if slug == "show-and-tell" {
                    node.get("id")
                        .and_then(|i| i.as_str())
                        .map(|s| s.to_string())
                } else {
                    None
                }
            })
        });

    let category_id = match category_id {
        Some(id) => id,
        None => {
            return GitHubDiscussionResult {
                url: None,
                error: Some(
                    crate::langs::global_texts()
                        .github_discussion_category_not_found
                        .to_string(),
                ),
            }
        }
    };

    // 第二步：创建 Discussion（使用 variables 避免 body 中的特殊字符破坏 GraphQL 语法）
    let mutation = json!({
        "query": r#"mutation($repoId: ID!, $categoryId: ID!, $title: String!, $body: String!) {
            createDiscussion(input: {repositoryId: $repoId, categoryId: $categoryId, title: $title, body: $body}) {
                discussion {
                    url
                }
            }
        }"#,
        "variables": {
            "repoId": repo_id,
            "categoryId": category_id,
            "title": title,
            "body": body
        }
    });

    let response = match client
        .post("https://api.github.com/graphql")
        .header("Authorization", format!("Bearer {}", github_token.trim()))
        .json(&mutation)
        .send()
    {
        Ok(r) => r,
        Err(e) => {
            return GitHubDiscussionResult {
                url: None,
                error: Some(
                    crate::langs::global_texts()
                        .fmt_discussion_request_failed
                        .replace("{}", &e.to_string()),
                ),
            }
        }
    };

    let status = response.status();
    if !status.is_success() {
        let text = response.text().unwrap_or_default();
        return GitHubDiscussionResult {
            url: None,
            error: Some(
                crate::langs::global_texts()
                    .fmt_discussion_http_error
                    .replace("{}", &status.as_u16().to_string())
                    .replace("{}", &text.chars().take(200).collect::<String>()),
            ),
        };
    }

    let resp_json: serde_json::Value = match response.json() {
        Ok(v) => v,
        Err(e) => {
            return GitHubDiscussionResult {
                url: None,
                error: Some(
                    crate::langs::global_texts()
                        .fmt_discussion_parse_failed
                        .replace("{}", &e.to_string()),
                ),
            }
        }
    };

    // 检查 GraphQL 错误
    if let Some(errors) = resp_json.get("errors") {
        let msg = errors
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|e| e.get("message"))
            .and_then(|m| m.as_str())
            .unwrap_or(crate::langs::global_texts().unknown_graphql_error);
        return GitHubDiscussionResult {
            url: None,
            error: Some(
                crate::langs::global_texts()
                    .fmt_discussion_graphql_error
                    .replace("{}", msg),
            ),
        };
    }

    let url = resp_json
        .get("data")
        .and_then(|d| d.get("createDiscussion"))
        .and_then(|c| c.get("discussion"))
        .and_then(|d| d.get("url"))
        .and_then(|u| u.as_str())
        .map(|s| s.to_string());

    GitHubDiscussionResult { url, error: None }
}

// ============================================================
// 聚合搜索 API
// ============================================================

/// 聚合搜索 API 主地址
const JUHE_API_BASE: &str = "https://88.lxmusic.xn--fiqs8s";
/// lerd 聚合备用域名
const JUHE_LERD_API_BASE: &str = "https://api.music.lerd.dpdns.org";
/// huibq 聚合备用域名
const JUHE_HUIBQ_API_BASE: &str = "https://lxmusicapi.onrender.com";
/// 聚合搜索 API 密钥
const JUHE_API_KEY: &str = "lxmusic";
/// Huibq 接口密钥（仅 huibq 兜底接口使用）
const HUIBQ_API_KEY: &str = "share-v3";
/// 聚合搜索 API 路径前缀
const JUHE_API_PREFIX: &str = "/v4";

/// 聚合搜索歌词下载结果
pub struct JuheLyricsResult {
    /// 触发本次歌词下载的音乐文件路径
    pub music_path: std::path::PathBuf,
    /// 歌曲信息
    #[allow(dead_code)]
    pub song: OnlineSong,
    /// 歌词内容（LRC 格式）
    pub lyrics: Option<String>,
    /// 错误信息
    #[allow(dead_code)]
    pub error: Option<String>,
}

/// 在后台线程中搜索聚合搜索
pub fn search_juhe_background(query: String, page: usize) -> mpsc::Receiver<SearchDownloadResult> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let result = search_juhe(&query, page);
        let _ = tx.send(result);
    });
    rx
}

pub fn search_juhe_sync(query: &str, page: usize) -> SearchDownloadResult {
    search_juhe(query, page)
}

/// 搜索聚合搜索（搜索所有平台，标记为 Juhe 来源）
fn search_juhe(query: &str, page: usize) -> SearchDownloadResult {
    log_file!("[Juhe] 搜索开始: query={}, page={}", query, page);
    let client = match create_search_client() {
        Some(c) => c,
        None => {
            log_file!("[Juhe] 创建HTTP客户端失败");
            return SearchDownloadResult {
                query: query.to_string(),
                songs: Vec::new(),
            };
        }
    };

    let mut all_songs = Vec::new();

    // 搜索酷狗音乐（主要平台，API 可正常获取播放URL）
    if let Some(songs) = search_kugou(&client, query, page) {
        log_file!("[Juhe] 酷狗搜索返回 {} 首歌", songs.len());
        for s in songs {
            all_songs.push(OnlineSong {
                juhe_platform: "kg".to_string(),
                juhe_song_id: s.hash.clone(),
                source: MusicSource::Juhe,
                ..s
            });
        }
    } else {
        log_file!("[Juhe] 酷狗搜索返回 None");
    }

    // 如果酷狗无结果，尝试酷我作为备用
    if all_songs.is_empty() {
        if let Some(songs) = search_kuwo(&client, query, page) {
            log_file!("[Juhe] 酷我搜索返回 {} 首歌（备用）", songs.len());
            for s in songs {
                all_songs.push(OnlineSong {
                    juhe_platform: "kw".to_string(),
                    juhe_song_id: s.id.to_string(),
                    source: MusicSource::Juhe,
                    ..s
                });
            }
        }
    }

    // 如果酷我也无结果，尝试网易作为备用
    if all_songs.is_empty() {
        if let Some(songs) = search_netease(&client, query, page) {
            log_file!("[Juhe] 网易搜索返回 {} 首歌（备用）", songs.len());
            for s in songs {
                all_songs.push(OnlineSong {
                    juhe_platform: "wy".to_string(),
                    juhe_song_id: s.id.to_string(),
                    source: MusicSource::Juhe,
                    ..s
                });
            }
        }
    }

    log_file!("[Juhe] 搜索完成: {} 首歌", all_songs.len());
    SearchDownloadResult {
        query: query.to_string(),
        songs: all_songs,
    }
}

/// 判断接口响应 code 是否表示成功
fn is_api_success(value: &serde_json::Value) -> bool {
    match value.get("code") {
        Some(c) if c.as_i64() == Some(0) || c.as_i64() == Some(200) => true,
        Some(c) if c.as_str() == Some("0") || c.as_str() == Some("200") => true,
        _ => false,
    }
}

/// 从响应 JSON 中提取下载 URL
fn extract_juhe_url(value: &serde_json::Value) -> Option<String> {
    if !is_api_success(value) {
        return None;
    }

    if let Some(url_str) = value.get("data").and_then(|d| d.as_str()) {
        if !url_str.trim().is_empty() && url_str.starts_with("http") {
            return Some(url_str.to_string());
        }
    }

    if let Some(url_str) = value
        .get("data")
        .and_then(|d| d.get("url"))
        .and_then(|u| u.as_str())
    {
        if !url_str.trim().is_empty() && url_str.starts_with("http") {
            return Some(url_str.to_string());
        }
    }

    if let Some(url_str) = value.get("url").and_then(|u| u.as_str()) {
        if !url_str.trim().is_empty() && url_str.starts_with("http") {
            return Some(url_str.to_string());
        }
    }

    let alt_fields = ["playUrl", "src", "mp3Url", "play_url"];
    for field in &alt_fields {
        if let Some(url_str) = value
            .get("data")
            .and_then(|d| d.get(*field))
            .and_then(|u| u.as_str())
        {
            if !url_str.trim().is_empty() && url_str.starts_with("http") {
                return Some(url_str.to_string());
            }
        }
    }

    None
}

/// 主域名下载接口：GET /v4/url/{platform}/{songId}/{quality}?key=xxx
fn get_primary_juhe_download_url(
    client: &reqwest::blocking::Client,
    song: &OnlineSong,
) -> Option<String> {
    let platform = &song.juhe_platform;
    let song_id = &song.juhe_song_id;
    let qualities = ["128k", "320k", "192kmp3", "128kmp3", "flac"];

    for quality in &qualities {
        let url = format!(
            "{}{}/url/{}/{}/{}?key={}",
            JUHE_API_BASE, JUHE_API_PREFIX, platform, song_id, quality, JUHE_API_KEY
        );
        log_file!("[JuheURL-main] 请求: {}", url);

        if let Ok(response) = client
            .get(&url)
            .header("User-Agent", "lx-music-desktop/2.12.1")
            .timeout(std::time::Duration::from_secs(15))
            .send()
        {
            if let Ok(text) = response.text() {
                log_file!(
                    "[JuheURL-main] 响应(前200): {}",
                    preview_for_log(&text, 200)
                );
                if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) {
                    if let Some(url) = extract_juhe_url(&value) {
                        return Some(url);
                    }
                }
            }
        }
    }

    None
}

/// lerd 兜底下载接口（独立方法）
/// 按 lx-juhe.js 协议：POST /{platform}，body 中 type 为音质，musicInfo.source 必填
fn get_lerd_download_url(client: &reqwest::blocking::Client, song: &OnlineSong) -> Option<String> {
    let platform = &song.juhe_platform;
    let song_id = &song.juhe_song_id;
    let qualities = ["128k", "320k", "flac", "flac24bit"];

    for quality in &qualities {
        let endpoint = format!("{}/{}", JUHE_LERD_API_BASE, platform);
        let body = json!({
            "type": quality,
            "musicInfo": {
                "id": song_id,
                "songmid": song_id,
                "hash": song_id,
                "source": platform,
            }
        });

        log_file!("[JuheURL-lerd] 请求(post): {}, body={}", endpoint, body);
        if let Ok(response) = client
            .post(&endpoint)
            .header("Content-Type", "application/json")
            .header("User-Agent", "lx-music-desktop/2.12.1")
            .body(body.to_string())
            .timeout(std::time::Duration::from_secs(12))
            .send()
        {
            if let Ok(text) = response.text() {
                log_file!(
                    "[JuheURL-lerd] 响应(post,前200): {}",
                    preview_for_log(&text, 200)
                );
                if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) {
                    if let Some(url) = extract_juhe_url(&value) {
                        return Some(url);
                    }
                }
            }
        }
    }

    None
}

/// huibq 兜底下载接口（独立方法）：GET /url/{platform}/{songId}/{quality}
fn get_huibq_download_url(client: &reqwest::blocking::Client, song: &OnlineSong) -> Option<String> {
    let platform = &song.juhe_platform;
    let song_id = &song.juhe_song_id;
    // 按 lx-huibq.js：仅声明 128k / 320k
    let qualities = ["128k", "320k"];

    for quality in &qualities {
        let url = format!(
            "{}/url/{}/{}/{}",
            JUHE_HUIBQ_API_BASE, platform, song_id, quality
        );
        log_file!("[JuheURL-huibq] 请求: {}", url);

        if let Ok(response) = client
            .get(&url)
            .header("User-Agent", "lx-music-desktop/2.12.1")
            .header("X-Request-Key", HUIBQ_API_KEY)
            .timeout(std::time::Duration::from_secs(12))
            .send()
        {
            if let Ok(text) = response.text() {
                log_file!(
                    "[JuheURL-lerd] 响应(post,前200): {}",
                    preview_for_log(&text, 200)
                );
                if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) {
                    if let Some(url) = extract_juhe_url(&value) {
                        return Some(url);
                    }
                }
            }
        }
    }

    None
}

/// 获取聚合搜索下载链接（按主域名 -> lerd -> huibq 兜底）
fn get_juhe_download_url(client: &reqwest::blocking::Client, song: &OnlineSong) -> Option<String> {
    log_file!(
        "[JuheURL] 获取下载链接: platform={}, song_id={}",
        song.juhe_platform,
        song.juhe_song_id
    );

    if song.juhe_platform.trim().is_empty() || song.juhe_song_id.trim().is_empty() {
        log_file!("[JuheURL] 缺少 platform 或 song_id，跳过聚合URL请求");
        return None;
    }

    get_primary_juhe_download_url(client, song)
        .or_else(|| get_lerd_download_url(client, song))
        .or_else(|| get_huibq_download_url(client, song))
}

/// 尝试从不同响应格式中提取歌词文本
fn extract_juhe_lyric(value: &serde_json::Value) -> Option<String> {
    if !is_api_success(value) {
        return None;
    }

    let lyric_candidates = [
        value
            .get("data")
            .and_then(|d| d.get("lyric"))
            .and_then(|l| l.as_str()),
        value
            .get("data")
            .and_then(|d| d.get("lrc"))
            .and_then(|l| l.get("lyric"))
            .and_then(|l| l.as_str()),
        value
            .get("data")
            .and_then(|d| d.get("lrc"))
            .and_then(|l| l.as_str()),
        value.get("data").and_then(|d| d.as_str()),
        value.get("lyric").and_then(|l| l.as_str()),
        value
            .get("data")
            .and_then(|d| d.get("klyric"))
            .and_then(|l| l.as_str()),
        value
            .get("data")
            .and_then(|d| d.get("krc"))
            .and_then(|l| l.as_str()),
    ];

    for lyric in lyric_candidates.iter().flatten() {
        if !lyric.trim().is_empty() {
            return Some(lyric.to_string());
        }
    }

    None
}

/// 主域名歌词接口（独立方法）
fn get_primary_juhe_lyrics(
    client: &reqwest::blocking::Client,
    song: &OnlineSong,
) -> Option<String> {
    let url = format!(
        "{}{}/lyric/{}/{}?key={}",
        JUHE_API_BASE, JUHE_API_PREFIX, song.juhe_platform, song.juhe_song_id, JUHE_API_KEY
    );

    log_file!("[JuheLyric-main] 请求: {}", url);
    match client
        .get(&url)
        .header("User-Agent", "lx-music-desktop/2.12.1")
        .timeout(std::time::Duration::from_secs(10))
        .send()
    {
        Ok(response) => {
            let status = response.status();
            match response.text() {
                Ok(text) => {
                    log_file!(
                        "[JuheLyric-main] HTTP {} 响应(前200): {}",
                        status.as_u16(),
                        preview_for_log(&text, 200)
                    );
                    if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) {
                        return extract_juhe_lyric(&value);
                    }
                }
                Err(e) => {
                    log_file!(
                        "[JuheLyric-main] 读取响应失败: {}",
                        reqwest_error_detail(&e)
                    );
                }
            }
        }
        Err(e) => {
            log_file!("[JuheLyric-main] 请求失败: {}", reqwest_error_detail(&e));
        }
    }

    None
}

fn extract_netease_lyric(value: &serde_json::Value) -> Option<String> {
    value
        .get("lrc")
        .and_then(|l| l.get("lyric"))
        .and_then(|l| l.as_str())
        .filter(|lyric| !lyric.trim().is_empty())
        .map(|lyric| lyric.to_string())
}

fn get_netease_lyrics_by_id(client: &reqwest::blocking::Client, song_id: &str) -> Option<String> {
    let url = format!(
        "https://music.163.com/api/song/lyric?id={}&lv=1&kv=1&tv=-1",
        song_id
    );

    log_file!("[JuheLyric-netease] 请求: {}", url);
    match client
        .get(&url)
        .header("Referer", "https://music.163.com/")
        .header("User-Agent", "Mozilla/5.0")
        .timeout(std::time::Duration::from_secs(10))
        .send()
    {
        Ok(response) => {
            let status = response.status();
            match response.text() {
                Ok(text) => {
                    log_file!(
                        "[JuheLyric-netease] HTTP {} 响应(前200): {}",
                        status.as_u16(),
                        preview_for_log(&text, 200)
                    );
                    if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) {
                        return extract_netease_lyric(&value);
                    }
                }
                Err(e) => {
                    log_file!(
                        "[JuheLyric-netease] 读取响应失败: {}",
                        reqwest_error_detail(&e)
                    );
                }
            }
        }
        Err(e) => {
            log_file!("[JuheLyric-netease] 请求失败: {}", reqwest_error_detail(&e));
        }
    }

    None
}

/// lerd 兜底歌词接口（独立方法）
/// 按 lx-juhe.js 与 init.conf，当前仅支持 musicUrl，不提供歌词 action
fn get_lerd_lyrics(_client: &reqwest::blocking::Client, song: &OnlineSong) -> Option<String> {
    log_file!(
        "[JuheLyric-lerd] 跳过: {} {}，lerd 脚本未暴露歌词端点",
        song.juhe_platform,
        song.juhe_song_id
    );
    None
}

/// huibq 兜底歌词接口（独立方法）
/// 按 lx-huibq.js，actions 仅有 musicUrl，不支持歌词
fn get_huibq_lyrics(_client: &reqwest::blocking::Client, song: &OnlineSong) -> Option<String> {
    log_file!(
        "[JuheLyric-huibq] 跳过: {} {}，huibq 脚本仅支持 musicUrl",
        song.juhe_platform,
        song.juhe_song_id
    );
    None
}

/// 获取聚合搜索歌词（仅主域名；其余域名脚本未暴露歌词 action）
fn get_juhe_lyrics(client: &reqwest::blocking::Client, song: &OnlineSong) -> Option<String> {
    get_primary_juhe_lyrics(client, song)
        .or_else(|| {
            if song.juhe_platform == "wy" {
                get_netease_lyrics_by_id(client, &song.juhe_song_id)
            } else {
                None
            }
        })
        .or_else(|| get_lerd_lyrics(client, song))
        .or_else(|| get_huibq_lyrics(client, song))
}

fn normalize_lyric_chinese_text(input: &str) -> String {
    let opencc = OPENCC_T2S_CACHE.get_or_init(|| OpenCC::from_config(BuiltinConfig::T2s).ok());
    opencc
        .as_ref()
        .map(|converter| converter.convert(input))
        .unwrap_or_else(|| input.to_string())
}

fn normalize_lyric_match_text(input: &str) -> String {
    normalize_lyric_chinese_text(input)
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
                        | '"'
                        | '\''
                        | '/'
                        | '\\'
                        | ':'
                        | '：'
                )
        })
        .collect()
}

fn juhe_lyrics_candidate_matches(
    target_artist: &str,
    target_title: &str,
    song: &OnlineSong,
) -> bool {
    let target_title_key = normalize_lyric_match_text(target_title);
    let candidate_title_key = normalize_lyric_match_text(&song.name);
    if target_title_key.is_empty() || candidate_title_key != target_title_key {
        return false;
    }

    let target_artist_key = normalize_lyric_match_text(target_artist);
    if target_artist_key.is_empty() {
        return true;
    }

    let candidate_artist_key = normalize_lyric_match_text(&song.artist);
    !candidate_artist_key.is_empty()
        && (candidate_artist_key.contains(&target_artist_key)
            || target_artist_key.contains(&candidate_artist_key))
}

/// 通过歌名和歌手名搜索并获取聚合歌词（用于本地歌曲回退歌词下载）
/// 只尝试歌名/歌手匹配的候选，避免同歌手相邻搜索结果歌词串歌。
pub fn search_and_get_juhe_lyrics_background(
    artist: String,
    title: String,
    music_path: std::path::PathBuf,
) -> mpsc::Receiver<JuheLyricsResult> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let query = if artist.is_empty() {
            title.clone()
        } else {
            format!("{} {}", artist, title)
        };

        log_file!("[JuheLyrics] 通过搜索获取歌词: query={}", query);

        let client = match reqwest::blocking::Client::builder()
            .no_proxy()
            .timeout(std::time::Duration::from_secs(15))
            .user_agent("lx-music-desktop/2.12.1")
            .build()
        {
            Ok(c) => c,
            Err(e) => {
                let _ = tx.send(JuheLyricsResult {
                    music_path: music_path.clone(),
                    song: OnlineSong {
                        name: title,
                        artist,
                        id: 0,
                        hash: String::new(),
                        duration_ms: None,
                        source: MusicSource::Juhe,
                        juhe_platform: String::new(),
                        juhe_song_id: String::new(),
                    },
                    lyrics: None,
                    error: Some(
                        crate::langs::global_texts()
                            .fmt_http_client_failed
                            .replace("{}", &e.to_string()),
                    ),
                });
                return;
            }
        };

        // 按优先级尝试搜索各平台；歌词接口可能对某个 hash 无歌词，因此保留多个候选逐个尝试。
        let mut candidates: Vec<OnlineSong> = Vec::new();

        // 1. 先搜酷狗
        if let Some(songs) = search_kugou(&client, &query, 1) {
            for s in songs.into_iter().take(5) {
                log_file!("[JuheLyrics] 酷狗搜索候选: {} - {}", s.artist, s.name);
                candidates.push(OnlineSong {
                    juhe_platform: "kg".to_string(),
                    juhe_song_id: s.hash.clone(),
                    source: MusicSource::Juhe,
                    ..s
                });
            }
        }

        // 2. 酷我候选
        if let Some(songs) = search_kuwo(&client, &query, 1) {
            for s in songs.into_iter().take(5) {
                log_file!("[JuheLyrics] 酷我搜索候选: {} - {}", s.artist, s.name);
                candidates.push(OnlineSong {
                    juhe_platform: "kw".to_string(),
                    juhe_song_id: s.id.to_string(),
                    source: MusicSource::Juhe,
                    ..s
                });
            }
        }

        // 3. 网易候选
        if let Some(songs) = search_netease(&client, &query, 1) {
            for s in songs.into_iter().take(5) {
                log_file!("[JuheLyrics] 网易搜索候选: {} - {}", s.artist, s.name);
                candidates.push(OnlineSong {
                    juhe_platform: "wy".to_string(),
                    juhe_song_id: s.id.to_string(),
                    source: MusicSource::Juhe,
                    ..s
                });
            }
        }

        if candidates.is_empty() {
            log_file!("[JuheLyrics] 搜索无结果: {}", query);
            let _ = tx.send(JuheLyricsResult {
                music_path: music_path.clone(),
                song: OnlineSong {
                    name: title,
                    artist,
                    id: 0,
                    hash: String::new(),
                    duration_ms: None,
                    source: MusicSource::Juhe,
                    juhe_platform: String::new(),
                    juhe_song_id: String::new(),
                },
                lyrics: None,
                error: Some(crate::langs::global_texts().search_no_result.to_string()),
            });
            return;
        }

        candidates.retain(|song| {
            let matched = juhe_lyrics_candidate_matches(&artist, &title, song);
            if !matched {
                log_file!(
                    "[JuheLyrics] 跳过不匹配候选: {} - {}，目标: {} - {}",
                    song.artist,
                    song.name,
                    artist,
                    title
                );
            }
            matched
        });

        if candidates.is_empty() {
            log_file!("[JuheLyrics] 无严格匹配候选: {} - {}", artist, title);
            let _ = tx.send(JuheLyricsResult {
                music_path,
                song: OnlineSong {
                    name: title,
                    artist,
                    id: 0,
                    hash: String::new(),
                    duration_ms: None,
                    source: MusicSource::Juhe,
                    juhe_platform: String::new(),
                    juhe_song_id: String::new(),
                },
                lyrics: None,
                error: Some(crate::langs::global_texts().lyrics_fetch_failed.to_string()),
            });
            return;
        }

        let mut last_song = candidates.first().cloned().unwrap_or_else(|| OnlineSong {
            name: title.clone(),
            artist: artist.clone(),
            id: 0,
            hash: String::new(),
            duration_ms: None,
            source: MusicSource::Juhe,
            juhe_platform: String::new(),
            juhe_song_id: String::new(),
        });

        for song in candidates {
            log_file!(
                "[JuheLyrics] 尝试歌词候选: {} - {} [{}:{}]",
                song.artist,
                song.name,
                song.juhe_platform,
                song.juhe_song_id
            );
            last_song = song.clone();
            if let Some(lyrics) = get_juhe_lyrics(&client, &song) {
                log_file!("[JuheLyrics] 歌词获取成功，长度={}", lyrics.len());
                let _ = tx.send(JuheLyricsResult {
                    music_path: music_path.clone(),
                    song,
                    lyrics: Some(lyrics),
                    error: None,
                });
                return;
            }
        }

        log_file!("[JuheLyrics] 所有候选歌词获取失败");
        let _ = tx.send(JuheLyricsResult {
            music_path,
            song: last_song,
            lyrics: None,
            error: Some(crate::langs::global_texts().lyrics_fetch_failed.to_string()),
        });
    });
    rx
}

/// 在后台线程中流式翻译歌词
pub fn fetch_lyrics_translation_streaming(
    original_lyrics: String,
    target_language: String,
    config: AiQueryConfig,
) -> mpsc::Receiver<SongInfoChunk> {
    let prompt = format!(
        "Detect the primary language of the lyrics below. If the lyrics are already in {}, return each line unchanged. Otherwise, translate each line into {}. Return only the resulting lyrics, with no explanations, notes, greetings, or extra text.\nThe output must preserve a strict one-to-one line mapping with the input: every input line must produce exactly one output line, in the same order.\n\nOriginal lyrics:\n{}",
        target_language,
        target_language, original_lyrics
    );

    fetch_song_info_streaming(prompt, config)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn juhe_test_song(name: &str, artist: &str) -> OnlineSong {
        OnlineSong {
            name: name.to_string(),
            artist: artist.to_string(),
            id: 1,
            hash: String::new(),
            duration_ms: None,
            source: MusicSource::Juhe,
            juhe_platform: "kg".to_string(),
            juhe_song_id: "song-id".to_string(),
        }
    }

    #[test]
    fn juhe_lyrics_candidates_require_matching_title() {
        let matching = juhe_test_song("昨天吹的风", "李宜柏PAULYBLEE");
        let wrong_title = juhe_test_song("比昨天更想念你", "李宜柏PAULYBLEE");
        let traditional_match = juhe_test_song("仍留在这里", "邓丽欣");
        let hk_tw_match = juhe_test_song("爱你到不爱我那天", "李宜柏PAULYBLEE");

        assert!(juhe_lyrics_candidate_matches(
            "李宜柏PAULYBLEE",
            "昨天吹的風",
            &matching
        ));
        assert!(!juhe_lyrics_candidate_matches(
            "李宜柏PAULYBLEE",
            "昨天吹的風",
            &wrong_title
        ));
        assert!(juhe_lyrics_candidate_matches(
            "鄧麗欣",
            "仍留在這裏",
            &traditional_match
        ));
        assert!(juhe_lyrics_candidate_matches(
            "李宜柏PAULYBLEE",
            "愛妳到不愛我那天",
            &hk_tw_match
        ));
    }

    #[test]
    fn extracts_netease_lrc_lyrics() {
        let value = serde_json::json!({
            "code": 200,
            "lrc": {
                "version": 1,
                "lyric": "[00:01.00]昨天吹的風\n[00:02.00]下一句"
            }
        });

        assert_eq!(
            extract_netease_lyric(&value).as_deref(),
            Some("[00:01.00]昨天吹的風\n[00:02.00]下一句")
        );
    }

    #[test]
    fn parses_supported_playlist_rank_and_artist_urls() {
        assert_eq!(
            parse_online_list_url("https://www.kuwo.cn/rankList").unwrap(),
            OnlineListUrlKind::Rank(PlaylistSource::Kuwo, None)
        );
        assert_eq!(
            parse_online_list_url("https://www.kugou.com/yy/rank/home/1-6666.html?from=rank")
                .unwrap(),
            OnlineListUrlKind::Rank(PlaylistSource::Kugou, Some("6666".to_string()))
        );
        assert_eq!(
            parse_online_list_url("https://music.163.com/#/discover/toplist?id=19723756").unwrap(),
            OnlineListUrlKind::Rank(PlaylistSource::NetEase, Some("19723756".to_string()))
        );
        assert_eq!(
            parse_online_list_url("https://www.kuwo.cn/playlist_detail/3596743037").unwrap(),
            OnlineListUrlKind::Playlist(PlaylistSource::Kuwo, "3596743037".to_string())
        );
        assert_eq!(
            parse_online_list_url("https://www.kugou.com/songlist/gcid_3zjy761gz1maz03e/").unwrap(),
            OnlineListUrlKind::Playlist(PlaylistSource::Kugou, "gcid_3zjy761gz1maz03e".to_string())
        );
        assert_eq!(
            parse_online_list_url("https://music.163.com/#/playlist?id=17911828410").unwrap(),
            OnlineListUrlKind::Playlist(PlaylistSource::NetEase, "17911828410".to_string())
        );
        assert_eq!(
            parse_online_list_url("https://www.kuwo.cn/singer_detail/336").unwrap(),
            OnlineListUrlKind::Artist(PlaylistSource::Kuwo, "336".to_string())
        );
        assert_eq!(
            parse_online_list_url("https://www.kugou.com/singer/info/3E0KCC3671E7/").unwrap(),
            OnlineListUrlKind::Artist(PlaylistSource::Kugou, "3E0KCC3671E7".to_string())
        );
        assert_eq!(
            parse_online_list_url("https://music.163.com/#/artist?id=10559").unwrap(),
            OnlineListUrlKind::Artist(PlaylistSource::NetEase, "10559".to_string())
        );
    }

    #[test]
    fn parses_external_playlist_urls() {
        assert_eq!(
            parse_online_list_url("https://open.spotify.com/playlist/4jAH3HKTS1mOGI8h2wA0Ba")
                .unwrap(),
            OnlineListUrlKind::External(
                ExternalPlaylistSource::Spotify,
                "4jAH3HKTS1mOGI8h2wA0Ba".to_string()
            )
        );
        assert!(matches!(
            parse_online_list_url("https://music.apple.com/cn/playlist/%E5%91%A8%E6%9D%B0%E4%BC%A6%E4%BB%A3%E8%A1%A8%E4%BD%9C/pl.d467987f72384448b2bebe52c0b212d6").unwrap(),
            OnlineListUrlKind::External(ExternalPlaylistSource::AppleMusic, _)
        ));
        for url in [
            "https://music.apple.com/cn/room/6769587728",
            "https://music.apple.com/cn/artist/%E5%91%A8%E6%9D%B0%E4%BC%A6/300117743/top-songs",
            "https://music.apple.com/cn/album/11%E6%9C%88%E7%9A%84%E8%95%AD%E9%82%A6/536009641",
        ] {
            assert!(matches!(
                parse_online_list_url(url).unwrap(),
                OnlineListUrlKind::External(ExternalPlaylistSource::AppleMusic, _)
            ));
        }
    }

    #[test]
    fn extracts_apple_music_playlist_storefront_and_id() {
        let url = "https://music.apple.com/tw/playlist/top-100-%E5%8F%B0%E7%81%A3/pl.741ff34016704547853b953ec5181d83";
        assert_eq!(apple_music_storefront_from_url(url), Some("tw".to_string()));
        assert_eq!(
            apple_music_playlist_id_from_url(url),
            Some("pl.741ff34016704547853b953ec5181d83".to_string())
        );

        let url = "https://music.apple.com/hk/playlist/%E7%99%BE%E5%A4%A7%E6%A6%9C-%E9%A6%99%E6%B8%AF/pl.7f35cffa10b54b91aab128ccc547f6ef";
        assert_eq!(apple_music_storefront_from_url(url), Some("hk".to_string()));
        assert_eq!(
            apple_music_playlist_id_from_url(url),
            Some("pl.7f35cffa10b54b91aab128ccc547f6ef".to_string())
        );

        let url = "https://music.apple.com/hk/room/6761185183";
        assert_eq!(apple_music_storefront_from_url(url), Some("hk".to_string()));
        assert_eq!(
            apple_music_room_id_from_url(url),
            Some("6761185183".to_string())
        );

        let url = "https://music.apple.com/tw/room/6769458811";
        assert_eq!(apple_music_storefront_from_url(url), Some("tw".to_string()));
        assert_eq!(
            apple_music_room_id_from_url(url),
            Some("6769458811".to_string())
        );

        let url =
            "https://music.apple.com/hk/album/11%E6%9C%88%E7%9A%84%E8%95%AD%E9%82%A6/536009641";
        assert_eq!(apple_music_storefront_from_url(url), Some("hk".to_string()));
        assert_eq!(
            apple_music_album_id_from_url(url),
            Some("536009641".to_string())
        );

        let url = "https://music.apple.com/us/artist/taylor-swift/159260351/top-songs";
        assert_eq!(apple_music_storefront_from_url(url), Some("us".to_string()));
        assert_eq!(
            apple_artist_top_songs_id(url),
            Some("159260351".to_string())
        );

        for (url, storefront) in [
            ("https://music.apple.com/cn/new/top-charts/songs", "cn"),
            ("https://music.apple.com/hk/new/top-charts/songs", "hk"),
            ("https://music.apple.com/tw/new/top-charts/songs", "tw"),
            ("https://music.apple.com/us/new/top-charts/songs", "us"),
            ("https://music.apple.com/kr/new/top-charts/songs", "kr"),
            ("https://music.apple.com/jp/new/top-charts/songs", "jp"),
        ] {
            assert!(apple_music_is_top_charts_songs_url(url));
            assert_eq!(
                apple_music_storefront_from_url(url),
                Some(storefront.to_string())
            );
        }
    }

    #[test]
    fn parses_kuwo_legacy_rank_rows_sequentially() {
        let html = r#"
            <div>最近更新：2026-05-15</div>
            <div>1</div><div>风又吹过眼里的愁 (女版)</div><div>归来小易</div>
            <div>2</div><div>阿凡提怎么天天都这么开心</div><div>Pao7oO</div>
            <div>14</div><div>谁</div><div>曾至锋</div>
            <div>3</div><div>谁</div><div>曾至锋</div>
        "#;
        let candidates = parse_kuwo_legacy_rank_text_candidates(html);
        assert_eq!(
            candidates,
            vec![
                (
                    "风又吹过眼里的愁 (女版)".to_string(),
                    "归来小易".to_string()
                ),
                ("阿凡提怎么天天都这么开心".to_string(), "Pao7oO".to_string()),
                ("谁".to_string(), "曾至锋".to_string()),
            ]
        );
    }

    #[test]
    fn parses_kuwo_legacy_rank_data_music_items() {
        let html = r#"
            <div class="tools" data-music='{"id":"MUSIC_384997","name":"爱情讯息","artist":"郭静","album":"下一个天亮","pay":"16711935"}' style="display:block;"></div>
            <div class="tools" data-music='{"id":"MUSIC_103134","name":"爱我还是他","artist":"陶喆","album":"太平盛世","pay":"16711935"}' style="display:block;"></div>
            <div class="tools" data-music='{"id":"MUSIC_398013033","name":"白鸽乌鸦相爱的戏码","artist":"潘成（皮卡潘）","album":"白鸽乌鸦相爱的戏码","pay":"16711935"}' style="display:block;"></div>
            <div class="tools" data-music='{"id":"MUSIC_228266349","name":"把回忆拼好给你（正版授权）","artist":"苏星婕","album":"把回忆拼好给你","pay":"16711935"}' style="display:block;"></div>
        "#;
        let candidates = parse_kuwo_legacy_rank_text_candidates(html);
        assert_eq!(candidates.len(), 4);
        assert_eq!(candidates[0], ("爱情讯息".to_string(), "郭静".to_string()));
        assert_eq!(
            candidates[1],
            ("爱我还是他".to_string(), "陶喆".to_string())
        );
        assert_eq!(
            candidates[3],
            (
                "把回忆拼好给你（正版授权）".to_string(),
                "苏星婕".to_string()
            )
        );
    }

    #[test]
    fn parses_apple_music_video_playlist_items() {
        let item = serde_json::json!({
            "type": "music-videos",
            "attributes": {
                "name": "Broke Boys",
                "artistName": "Drake & 21 Savage",
                "durationInMillis": 226000
            }
        });
        let song = apple_music_catalog_song_to_online_song(&item).unwrap();
        assert_eq!(song.name, "Broke Boys");
        assert_eq!(song.artist, "Drake & 21 Savage");
        assert_eq!(song.duration_ms, Some(226000));
    }
}
