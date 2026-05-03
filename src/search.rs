// 网络搜索下载模块
// 支持多平台搜索：酷我音乐 + 网易音乐 + 酷狗音乐

use chrono::{Local, TimeZone};
use serde::Deserialize;
use serde_json::json;
use std::path::PathBuf;
use std::sync::mpsc;

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
pub struct PlaylistSongsResult {
    /// 歌单信息
    pub playlist: OnlinePlaylist,
    /// 歌单歌曲
    pub songs: Vec<OnlineSong>,
}

/// 单条评论中的“回复”信息
#[derive(Debug, Clone)]
pub struct SongCommentReply {
    /// 被回复用户昵称
    pub nickname: String,
    /// 回复内容
    pub content: String,
    /// 回复时间（优先使用接口返回的可读时间）
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
        .timeout(std::time::Duration::from_secs(10))
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .build()
        .ok()
}

/// 创建 HTTP 客户端（下载用）
fn create_download_client() -> Option<reqwest::blocking::Client> {
    reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .build()
        .ok()
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
    let secs = if ts >= 1_000_000_000_000 { ts / 1000 } else { ts };
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
pub fn search_online_background(query: String, page: usize) -> mpsc::Receiver<SearchDownloadResult> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let result = search_online(&query, page);
        let _ = tx.send(result);
    });
    rx
}

/// 在后台线程中下载歌曲（带进度回调）
pub fn download_song_background(song: OnlineSong, save_dir: PathBuf) -> mpsc::Receiver<DownloadProgress> {
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
pub fn search_playlist_background(query: String, page: usize) -> mpsc::Receiver<PlaylistSearchResult> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let result = search_playlist(&query, page);
        let _ = tx.send(result);
    });
    rx
}

/// 在后台线程中加载歌单歌曲
pub fn fetch_playlist_songs_background(playlist: OnlinePlaylist) -> mpsc::Receiver<PlaylistSongsResult> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let songs = fetch_playlist_songs(&playlist);
        let _ = tx.send(PlaylistSongsResult { playlist, songs });
    });
    rx
}

/// 在后台线程中获取歌曲评论（基于歌曲名搜索网易）
pub fn fetch_song_comments_background(query: String, page: usize, page_size: usize) -> mpsc::Receiver<SongCommentsResult> {
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
pub fn fetch_song_info_streaming(prompt: String, config: AiQueryConfig) -> mpsc::Receiver<SongInfoChunk> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        // 确定最终的 API URL、模型、认证头
        // 如果用户未设置 API Key，则使用 OpenRouter 免费模型兜底
        let (api_url, model, auth_header) = if !config.api_key.trim().is_empty() {
            // 用户配置了 API Key，使用自定义接口
            let base = config.api_base_url.trim().trim_end_matches('/');
            let url = format!("{}/chat/completions", base);
            (url, config.api_model.trim().to_string(), format!("Bearer {}", config.api_key.trim()))
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
            .user_agent("TerMusicRust/1.5.0")
            .build()
        {
            Ok(c) => c,
            Err(e) => {
                let _ = tx.send(SongInfoChunk {
                    delta: String::new(),
                    done: true,
                    error: Some(format!("创建 HTTP 客户端失败: {}", e)),
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
                    error: Some(format!("请求API接口失败: {}", e)),
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
                error: Some(format!("请求API接口错误: {}", msg)),
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
                        error: Some("读取流式响应失败".to_string()),
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
fn search_kuwo(client: &reqwest::blocking::Client, query: &str, page: usize) -> Option<Vec<OnlineSong>> {
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

    Some(lists.into_iter().map(|s| {
        let duration_ms = s.duration
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
    }).collect())
}

/// 网易音乐搜索（备用）
fn search_netease(client: &reqwest::blocking::Client, query: &str, page: usize) -> Option<Vec<OnlineSong>> {
    let offset = (page.saturating_sub(1)) * 20;
    let search_url = format!(
        "https://music.163.com/api/search/get?s={}&type=1&limit=20&offset={}",
        urlencoding::encode(query),
        offset
    );

    let response = client.get(&search_url)
        .header("Referer", "https://music.163.com/")
        .header("Cookie", "MUSIC_U=; appver=2.0.2;")
        .send()
        .ok()?;

    let text = response.text().ok()?;
    let search_result: NetEaseSearchResult = serde_json::from_str(&text).ok()?;

    let result = search_result.result?;
    let songs = result.songs?;

    Some(songs.into_iter().map(|s| {
        let artist = s.artists
            .map(|a| a.iter().map(|ar| ar.name.as_str()).collect::<Vec<&str>>().join(", "))
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
    }).collect())
}

/// 酷狗音乐搜索
fn search_kugou(client: &reqwest::blocking::Client, query: &str, page: usize) -> Option<Vec<OnlineSong>> {
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

    Some(info.into_iter().filter_map(|s| {
        let hash = s.hash.unwrap_or_default();
        if hash.is_empty() {
            return None;
        }
        let name = s.songname.unwrap_or_default();
        let artist = s.singername.unwrap_or_default();
        let duration_ms = s.duration
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
    }).collect())
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

    let playlist_id = pick_str_field(v, &["id", "rid", "dissid", "list_id", "playlist_id", "specialid"])?;
    let name = pick_str_field(v, &["name", "title", "listname", "dissname", "specialname"])?;
    let author = pick_str_field(v, &["author", "creator", "nickname", "nickName", "uname", "username", "singername"])
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
        date_text: pick_date_field(v, &["createTime", "create_time", "publishTime", "publish_time", "publish", "ctime"]),
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
fn search_kuwo_playlists_legacy(client: &reqwest::blocking::Client, query: &str, page: usize) -> Vec<OnlinePlaylist> {
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

    log_file!("[Playlist][KW][Legacy] 响应(前200): {}", &text[..text.len().min(200)]);

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
            let author = pick_str_field(&item, &["nickname", "uname", "author"]).unwrap_or_default();
            let song_count = pick_usize_field(&item, &["songnum", "song_count", "songCount", "total"]);
            let description = pick_str_field(&item, &["intro", "description", "desc"]).unwrap_or_default();
            let play_count = pick_usize_field(&item, &["playcnt", "play_count", "playCount", "listencount", "listen_count"]);

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
fn search_kugou_playlists(client: &reqwest::blocking::Client, query: &str, page: usize) -> Vec<OnlinePlaylist> {
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

            let author = pick_str_field(&item, &["nickname", "author", "username"]).unwrap_or_default();
            let song_count = pick_song_count(&item);
            let description = pick_str_field(&item, &["intro", "description", "desc"]).unwrap_or_default();
            let play_count = pick_usize_field(
                &item,
                &["play_count", "playCount", "playcount", "total_play", "totalPlay", "listencount", "listen_count", "pv", "visit"],
            );

            Some(OnlinePlaylist {
                name,
                author,
                playlist_id,
                source: PlaylistSource::Kugou,
                song_count,
                description,
                play_count,
                date_text: pick_date_field(&item, &["publish_time", "publishTime", "publish", "createTime", "create_time", "ctime"]),
            })

        })
        .collect()
}

/// 酷我歌单搜索
fn search_kuwo_playlists(client: &reqwest::blocking::Client, query: &str, page: usize) -> Vec<OnlinePlaylist> {
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
        let referer = format!("https://www.kuwo.cn/search/list?key={}", urlencoding::encode(query));
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
        log_file!("[Playlist][KW] 响应(前200): {}", &text[..text.len().min(200)]);

        let v: serde_json::Value = match serde_json::from_str(&text) {
            Ok(v) => v,
            Err(e) => {
                log_file!("[Playlist][KW] JSON解析失败(pn={}): {}", pn, e);
                continue;
            }
        };

        let mut candidate = v
            .get("data")
            .and_then(|d| d.get("list").and_then(|x| x.as_array()).or_else(|| d.get("abslist").and_then(|x| x.as_array())))
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
            log_file!("[Playlist][KW] 命中风控(pn={})，跳过备用URL与其他页，直接走旧接口", pn);
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
                log_file!("[Playlist][KW] 备用响应(前200): {}", &t2[..t2.len().min(200)]);
                if let Ok(v2) = serde_json::from_str::<serde_json::Value>(&t2) {
                    candidate = v2
                        .get("data")
                        .and_then(|d| d.get("list").and_then(|x| x.as_array()).or_else(|| d.get("abslist").and_then(|x| x.as_array())))
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
            let author = pick_str_field(&item, &["uname", "nickname", "author", "nickName"]).unwrap_or_default();
            let song_count = pick_song_count(&item);
            let description = pick_str_field(&item, &["intro", "description", "desc"]).unwrap_or_default();
            let play_count = pick_usize_field(&item, &["play_count", "playCount", "playcount", "listencount", "listen_count", "listenNum"]);
            Some(OnlinePlaylist {
                name,
                author,
                playlist_id,
                source: PlaylistSource::Kuwo,
                song_count,
                description,
                play_count,
                date_text: pick_date_field(&item, &["pub", "publishTime", "publish_time", "createTime", "create_time", "ctime"]),
            })

        })
        .collect()
}


/// 网易歌单搜索
fn search_netease_playlists(client: &reqwest::blocking::Client, query: &str, page: usize) -> Vec<OnlinePlaylist> {
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
                .or_else(|| pick_str_field(&item, &["playCount", "play_count", "playcount"]).and_then(|s| s.parse::<usize>().ok()));
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

    t.parse::<i64>()
        .ok()
        .map(|secs_or_ms| if secs_or_ms > 10_000 { secs_or_ms } else { secs_or_ms * 1000 })
}

fn parse_playlist_song_item(v: &serde_json::Value, platform: &str, source: PlaylistSource) -> Option<OnlineSong> {
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
        artist = v.get("ar")
            .and_then(|a| a.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|x| x.get("name").and_then(|s| s.as_str()))
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| pick_str_field(v, &["artist", "singer", "singername", "author"]).unwrap_or_default());
    } else {
        artist = pick_str_field(v, &["artist", "singer", "singername", "author"]).unwrap_or_default();
    }

    if name.is_empty() {
        return None;
    }

    let song_id = match source {
        PlaylistSource::Kugou => {
            pick_str_field(v, &["hash"])
                .or_else(|| pick_str_field(v, &["audio_id", "id", "songid", "songId"]))
        }
        PlaylistSource::Kuwo => {
            pick_str_field(v, &["rid", "musicrid", "id"]).map(|s| s.trim_start_matches("MUSIC_").to_string())
        }
        PlaylistSource::NetEase => pick_str_field(v, &["id", "songid", "songId"]),
    }?;

    let duration_ms = if source == PlaylistSource::NetEase {
        v.get("dt").and_then(|x| x.as_i64()).or_else(|| {
            pick_str_field(v, &["duration", "interval", "timeLength"])
                .as_deref()
                .and_then(parse_duration_to_ms)
        })
    } else {
        pick_str_field(v, &["duration", "interval", "timeLength", "songTimeMinutes", "songTime"])
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
fn fetch_kugou_playlist_songs(client: &reqwest::blocking::Client, playlist_id: &str) -> Vec<OnlineSong> {
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
        .and_then(|d| d.get("info").and_then(|x| x.as_array()).or_else(|| d.get("list").and_then(|x| x.as_array())))
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

fn fetch_kuwo_playlist_songs_legacy_api(client: &reqwest::blocking::Client, playlist_id: &str) -> Vec<OnlineSong> {
    let rn = 100usize;
    let mut pn = 0usize;
    let mut total: Option<usize> = None;
    let mut songs = Vec::new();
    let mut seen = std::collections::HashSet::<String>::new();

    while pn < 20 {
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

fn fetch_kuwo_playlist_songs_from_page(client: &reqwest::blocking::Client, playlist_id: &str) -> Vec<OnlineSong> {
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
fn fetch_kuwo_playlist_songs(client: &reqwest::blocking::Client, playlist_id: &str) -> Vec<OnlineSong> {
    let url = format!(
        "https://www.kuwo.cn/api/www/playlist/playListInfo?pid={}&pn=1&rn=200&httpsStatus=1",
        playlist_id
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
            let legacy = fetch_kuwo_playlist_songs_legacy_api(client, playlist_id);
            if !legacy.is_empty() {
                return legacy;
            }
            return fetch_kuwo_playlist_songs_from_page(client, playlist_id);
        }
    };

    log_file!("[Playlist][KW] 歌单详情响应(前200): {}", &text[..text.len().min(200)]);

    let v: serde_json::Value = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(e) => {
            log_file!("[Playlist][KW] 歌单详情JSON解析失败: {}", e);
            return fetch_kuwo_playlist_songs_from_page(client, playlist_id);
        }
    };

    let songs = v.get("data")
        .and_then(|d| d.get("musicList"))
        .and_then(|x| x.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| parse_playlist_song_item(x, "kw", PlaylistSource::Kuwo))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    if songs.is_empty() {
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
fn fetch_netease_song_detail_batch(client: &reqwest::blocking::Client, ids: &[i64]) -> Vec<OnlineSong> {
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
fn fetch_netease_playlist_songs(client: &reqwest::blocking::Client, playlist_id: &str) -> Vec<OnlineSong> {
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
        playlist.source, playlist.playlist_id, playlist.name
    );

    let songs = match playlist.source {
        PlaylistSource::Kugou => fetch_kugou_playlist_songs(&client, &playlist.playlist_id),
        PlaylistSource::Kuwo => fetch_kuwo_playlist_songs(&client, &playlist.playlist_id),
        PlaylistSource::NetEase => fetch_netease_playlist_songs(&client, &playlist.playlist_id),
    };

    log_file!("[Playlist] 歌单歌曲加载完成，共 {} 首", songs.len());
    songs
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

    let total = value
        .get("total")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize;

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

                                SongCommentReply { nickname, content, time_text }
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
fn download_song_with_progress<F>(song: &OnlineSong, save_dir: &PathBuf, on_progress: F) -> DownloadResult
where
    F: Fn(u8) + Send + Sync,
{
    log_file!("[Download] 开始下载: {} - {}, source={:?}, juhe_platform={}, juhe_song_id={}", 
        song.artist, song.name, song.source, song.juhe_platform, song.juhe_song_id);

    let client = match create_download_client() {
        Some(c) => c,
        None => {
            log_file!("[Download] 创建HTTP客户端失败");
            return DownloadResult {
                song: song.clone(),
                local_path: None,
                error: Some("创建HTTP客户端失败".to_string()),
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
            log_file!("[Download] 无法获取下载链接, source={:?}", song.source);
            return DownloadResult {
                song: song.clone(),
                local_path: None,
                error: Some("无法获取下载链接，该歌曲可能需要VIP或已下架".to_string()),
            }
        }
    };

    log_file!("[Download] 获取到URL: {}...", &mp3_url[..mp3_url.len().min(80)]);
    on_progress(5);

    // 下载音频文件（流式读取以支持进度）
    let referer = match song.source {
        MusicSource::Kuwo | MusicSource::Kugou => "https://www.kuwo.cn/",
        MusicSource::NetEase => "https://music.163.com/",
        MusicSource::Juhe => "https://www.kuwo.cn/",
    };
    let response = match client.get(&mp3_url)
        .header("Referer", referer)
        .send()
    {
        Ok(r) => r,
        Err(e) => {
            return DownloadResult {
                song: song.clone(),
                local_path: None,
                error: Some(format!("下载请求失败: {}", e)),
            }
        }
    };

    // 检查 Content-Type
    let content_type = response.headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_lowercase();

    if content_type.contains("text/html") || content_type.contains("text/plain") {
        return DownloadResult {
            song: song.clone(),
            local_path: None,
            error: Some("该歌曲无法下载（服务器返回了网页，可能需要VIP）".to_string()),
        }
    }

    // 获取总大小
    let total_size = response.headers()
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
                    error: Some(format!("读取响应数据失败: {}", e)),
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
        }
    }

    // 确保保存目录存在
    if let Err(e) = std::fs::create_dir_all(save_dir) {
        return DownloadResult {
            song: song.clone(),
            local_path: None,
            error: Some(format!("创建目录失败: {}", e)),
        }
    }

    on_progress(98);

    // 生成文件名
    let filename = if song.artist.is_empty() {
        format!("{}.mp3", sanitize_filename(&song.name))
    } else {
        format!("{} - {}.mp3", sanitize_filename(&song.artist), sanitize_filename(&song.name))
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
            error: Some(format!("写入文件失败: {}", e)),
        }
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
                error: Some("创建HTTP客户端失败".to_string()),
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
                error: Some("无法获取下载链接，该歌曲可能需要VIP或已下架".to_string()),
            }
        }
    };

    // 下载音频文件
    let referer = match song.source {
        MusicSource::Kuwo | MusicSource::Kugou => "https://www.kuwo.cn/",
        MusicSource::NetEase => "https://music.163.com/",
        MusicSource::Juhe => "https://www.kuwo.cn/",
    };
    let response = match client.get(&mp3_url)
        .header("Referer", referer)
        .send()
    {
        Ok(r) => r,
        Err(e) => {
            return DownloadResult {
                song: song.clone(),
                local_path: None,
                error: Some(format!("下载请求失败: {}", e)),
            }
        }
    };

    // 检查 Content-Type
    let content_type = response.headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_lowercase();

    if content_type.contains("text/html") || content_type.contains("text/plain") {
        return DownloadResult {
            song: song.clone(),
            local_path: None,
            error: Some("该歌曲无法下载（服务器返回了网页，可能需要VIP）".to_string()),
        };
    }

    let bytes = match response.bytes() {
        Ok(b) => b,
        Err(e) => {
            return DownloadResult {
                song: song.clone(),
                local_path: None,
                error: Some(format!("读取响应数据失败: {}", e)),
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
            error: Some(format!("创建目录失败: {}", e)),
        };
    }

    // 生成文件名
    let filename = if song.artist.is_empty() {
        format!("{}.mp3", sanitize_filename(&song.name))
    } else {
        format!("{} - {}.mp3", sanitize_filename(&song.artist), sanitize_filename(&song.name))
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
            error: Some(format!("写入文件失败: {}", e)),
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

    if let Ok(response) = client.get(&url2)
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

    if let Ok(response) = client.get(&url_api)
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

    if let Ok(response) = client.get(&redirect_url)
        .header("Referer", "https://music.163.com/")
        .send()
    {
        let final_url = response.url().to_string();
        if final_url.contains("126.net") {
            return Some(final_url);
        }
        let ct = response.headers()
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
        return Err("下载数据过小，不是有效的音频文件".to_string());
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
        if lower.contains("<!doctype") || lower.contains("<html") || lower.contains("<head")
            || lower.contains("抱歉") || lower.contains("not found") || lower.contains("error")
        {
            return Err("该歌曲无法下载（服务器返回了网页而非音频，可能需要VIP或已下架）".to_string());
        }
    }

    // 无法识别的文件头，但可能仍是音频（某些格式）
    // 如果文件大小足够大（>10KB），则视为可能是有效音频
    if bytes.len() > 10240 {
        Ok(())
    } else {
        Err("下载数据不是有效的音频文件".to_string())
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
            error: Some("未配置 GitHub Token，无法创建 Discussion。请在配置文件中设置 github_token。".to_string()),
        };
    }

    let client = match reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .user_agent("TerMusicRust/1.5.0")
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            return GitHubDiscussionResult {
                url: None,
                error: Some(format!("创建 HTTP 客户端失败: {}", e)),
            }
        }
    };

    // 解析 owner/repo
    let parts: Vec<&str> = github_repo.trim().split('/').collect();
    if parts.len() != 2 {
        return GitHubDiscussionResult {
            url: None,
            error: Some(format!("GitHub 仓库格式错误，应为 owner/repo，实际: {}", github_repo)),
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
                error: Some(format!("请求 GitHub API 失败: {}", e)),
            }
        }
    };

    let status = response.status();
    if !status.is_success() {
        let text = response.text().unwrap_or_default();
        return GitHubDiscussionResult {
            url: None,
            error: Some(format!("GitHub API 请求失败 (HTTP {}): {}", status.as_u16(), text.chars().take(200).collect::<String>())),
        };
    }

    let resp_json: serde_json::Value = match response.json() {
        Ok(v) => v,
        Err(e) => {
            return GitHubDiscussionResult {
                url: None,
                error: Some(format!("解析 GitHub API 响应失败: {}", e)),
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
            .unwrap_or("未知 GraphQL 错误");
        return GitHubDiscussionResult {
            url: None,
            error: Some(format!("GitHub GraphQL 错误: {}", msg)),
        };
    }

    let repository = match resp_json
        .get("data")
        .and_then(|d| d.get("repository"))
    {
        Some(r) => r,
        None => {
            return GitHubDiscussionResult {
                url: None,
                error: Some("未找到仓库，请检查 github_repo 配置。".to_string()),
            }
        }
    };

    let repo_id = match repository.get("id").and_then(|v| v.as_str()) {
        Some(id) => id.to_string(),
        None => {
            return GitHubDiscussionResult {
                url: None,
                error: Some("无法获取仓库 ID。".to_string()),
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
                    node.get("id").and_then(|i| i.as_str()).map(|s| s.to_string())
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
                error: Some("仓库中未找到 'show-and-tell' Discussion 类别。请先在 GitHub 仓库设置中启用 Discussions 并确认该类别存在。".to_string()),
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
                error: Some(format!("创建 Discussion 请求失败: {}", e)),
            }
        }
    };

    let status = response.status();
    if !status.is_success() {
        let text = response.text().unwrap_or_default();
        return GitHubDiscussionResult {
            url: None,
            error: Some(format!("创建 Discussion 失败 (HTTP {}): {}", status.as_u16(), text.chars().take(200).collect::<String>())),
        };
    }

    let resp_json: serde_json::Value = match response.json() {
        Ok(v) => v,
        Err(e) => {
            return GitHubDiscussionResult {
                url: None,
                error: Some(format!("解析创建 Discussion 响应失败: {}", e)),
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
            .unwrap_or("未知 GraphQL 错误");
        return GitHubDiscussionResult {
            url: None,
            error: Some(format!("创建 Discussion GraphQL 错误: {}", msg)),
        };
    }

    let url = resp_json
        .get("data")
        .and_then(|d| d.get("createDiscussion"))
        .and_then(|c| c.get("discussion"))
        .and_then(|d| d.get("url"))
        .and_then(|u| u.as_str())
        .map(|s| s.to_string());

    GitHubDiscussionResult {
        url,
        error: None,
    }
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
            }
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
fn get_primary_juhe_download_url(client: &reqwest::blocking::Client, song: &OnlineSong) -> Option<String> {
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
                log_file!("[JuheURL-main] 响应(前200): {}", &text[..text.len().min(200)]);
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
                log_file!("[JuheURL-lerd] 响应(post,前200): {}", &text[..text.len().min(200)]);
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
        let url = format!("{}/url/{}/{}/{}", JUHE_HUIBQ_API_BASE, platform, song_id, quality);
        log_file!("[JuheURL-huibq] 请求: {}", url);

        if let Ok(response) = client
            .get(&url)
            .header("User-Agent", "lx-music-desktop/2.12.1")
            .header("X-Request-Key", HUIBQ_API_KEY)
            .timeout(std::time::Duration::from_secs(12))
            .send()
        {
            if let Ok(text) = response.text() {
                log_file!("[JuheURL-huibq] 响应(前200): {}", &text[..text.len().min(200)]);
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
        value.get("data").and_then(|d| d.get("lyric")).and_then(|l| l.as_str()),
        value.get("data").and_then(|d| d.get("lrc")).and_then(|l| l.get("lyric")).and_then(|l| l.as_str()),
        value.get("data").and_then(|d| d.get("lrc")).and_then(|l| l.as_str()),
        value.get("data").and_then(|d| d.as_str()),
        value.get("lyric").and_then(|l| l.as_str()),
        value.get("data").and_then(|d| d.get("klyric")).and_then(|l| l.as_str()),
        value.get("data").and_then(|d| d.get("krc")).and_then(|l| l.as_str()),
    ];

    for lyric in lyric_candidates.iter().flatten() {
        if !lyric.trim().is_empty() {
            return Some(lyric.to_string());
        }
    }

    None
}

/// 主域名歌词接口（独立方法）
fn get_primary_juhe_lyrics(client: &reqwest::blocking::Client, song: &OnlineSong) -> Option<String> {
    let url = format!(
        "{}{}/lyric/{}/{}?key={}",
        JUHE_API_BASE, JUHE_API_PREFIX, song.juhe_platform, song.juhe_song_id, JUHE_API_KEY
    );

    log_file!("[JuheLyric-main] 请求: {}", url);
    if let Ok(response) = client
        .get(&url)
        .header("User-Agent", "lx-music-desktop/2.12.1")
        .timeout(std::time::Duration::from_secs(10))
        .send()
    {
        if let Ok(text) = response.text() {
            log_file!("[JuheLyric-main] 响应(前200): {}", preview_for_log(&text, 200));
            if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) {
                return extract_juhe_lyric(&value);
            }
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
        .or_else(|| get_lerd_lyrics(client, song))
        .or_else(|| get_huibq_lyrics(client, song))
}

/// 通过歌名和歌手名搜索并获取聚合歌词（用于本地歌曲回退歌词下载）
/// 先搜索匹配歌曲，取第一个结果，再通过其 platform/song_id 获取歌词
pub fn search_and_get_juhe_lyrics_background(artist: String, title: String, music_path: std::path::PathBuf) -> mpsc::Receiver<JuheLyricsResult> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let query = if artist.is_empty() {
            title.clone()
        } else {
            format!("{} {}", artist, title)
        };

        log_file!("[JuheLyrics] 通过搜索获取歌词: query={}", query);

        let client = match reqwest::blocking::Client::builder()
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
                    error: Some(format!("创建HTTP客户端失败: {}", e)),
                });
                return;
            }
        };

        // 按优先级尝试搜索各平台，取第一个结果
        let mut found_song: Option<OnlineSong> = None;

        // 1. 先搜酷狗
        if let Some(songs) = search_kugou(&client, &query, 1) {
            if let Some(s) = songs.first() {
                found_song = Some(OnlineSong {
                    juhe_platform: "kg".to_string(),
                    juhe_song_id: s.hash.clone(),
                    source: MusicSource::Juhe,
                    ..s.clone()
                });
                log_file!("[JuheLyrics] 酷狗搜索命中: {} - {}", s.artist, s.name);
            }
        }

        // 2. 酷狗没搜到，试酷我
        if found_song.is_none() {
            if let Some(songs) = search_kuwo(&client, &query, 1) {
                if let Some(s) = songs.first() {
                    found_song = Some(OnlineSong {
                        juhe_platform: "kw".to_string(),
                        juhe_song_id: s.id.to_string(),
                        source: MusicSource::Juhe,
                        ..s.clone()
                    });
                    log_file!("[JuheLyrics] 酷我搜索命中: {} - {}", s.artist, s.name);
                }
            }
        }

        // 3. 酷我也没搜到，试网易
        if found_song.is_none() {
            if let Some(songs) = search_netease(&client, &query, 1) {
                if let Some(s) = songs.first() {
                    found_song = Some(OnlineSong {
                        juhe_platform: "wy".to_string(),
                        juhe_song_id: s.id.to_string(),
                        source: MusicSource::Juhe,
                        ..s.clone()
                    });
                    log_file!("[JuheLyrics] 网易搜索命中: {} - {}", s.artist, s.name);
                }
            }
        }

        let song = match found_song {
            Some(s) => s,
            None => {
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
                    error: Some("搜索无结果".to_string()),
                });
                return;
            }
        };

        // 用搜索到的歌曲信息获取歌词
        match get_juhe_lyrics(&client, &song) {
            Some(lyrics) => {
                log_file!("[JuheLyrics] 歌词获取成功，长度={}", lyrics.len());
                let _ = tx.send(JuheLyricsResult {
                    music_path: music_path.clone(),
                    song,
                    lyrics: Some(lyrics),
                    error: None,
                });
            }
            None => {
                log_file!("[JuheLyrics] 歌词获取失败");
                let _ = tx.send(JuheLyricsResult {
                    music_path,
                    song,
                    lyrics: None,
                    error: Some("无法获取歌词".to_string()),
                });
            }
        }
    });
    rx
}


