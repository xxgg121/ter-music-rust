// 网络搜索下载模块
// 支持多平台搜索：酷我音乐 + 网易云音乐 + 酷狗音乐

use chrono::{Local, TimeZone};
use serde::Deserialize;
use serde_json::json;
use std::path::PathBuf;
use std::sync::mpsc;

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
}

/// 音乐来源
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MusicSource {
    /// 酷我音乐
    Kuwo,
    /// 网易云音乐
    NetEase,
    /// 酷狗音乐
    Kugou,
}

/// 网络搜索下载结果
pub struct SearchDownloadResult {
    /// 搜索关键字
    #[allow(dead_code)]
    pub query: String,
    /// 搜索到的歌曲列表
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
    Done(DownloadResult),
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
// 网易云音乐 JSON 结构（备用）
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
    /// 时长（秒字符串）
    duration: Option<String>,
    /// 付费类型：0=免费，3=付费
    #[allow(dead_code)]
    pay_type: Option<i64>,
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

/// 将网易云毫秒时间戳转换为本地日期时间文本
fn format_datetime_from_millis(ms: i64) -> Option<String> {
    Local
        .timestamp_millis_opt(ms)
        .single()
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
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
        let _ = tx.send(DownloadProgress::Done(result));
    });
    rx
}

/// 在后台线程中获取歌曲评论（基于歌曲名搜索网易云）
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
            .user_agent("TerMusicRust/1.2.0")
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
// 搜索逻辑：酷我优先，网易云备用
// ============================================================



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

    // 酷狗无结果，尝试网易云音乐
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
        }
    }).collect())
}

/// 网易云音乐搜索（备用）
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

    let response = client.get(&search_url)
        .send()
        .ok()?;

    let text = response.text().ok()?;
    let search_result: KugouSearchResult = serde_json::from_str(&text).ok()?;

    let data = search_result.data?;
    let info = data.info?;

    Some(info.into_iter().filter_map(|s| {
        let hash = s.hash.unwrap_or_default();
        if hash.is_empty() {
            return None;
        }
        let name = s.songname.unwrap_or_default();
        let artist = s.singername.unwrap_or_default();
        let duration_ms = s.duration
            .and_then(|d| d.parse::<i64>().ok())
            .map(|secs| secs * 1000);
        Some(OnlineSong {
            name,
            artist,
            id: 0, // 酷狗不用数字ID，用 hash
            hash,
            duration_ms,
            source: MusicSource::Kugou,
        })
    }).collect())
}

// ============================================================
// 评论拉取逻辑（网易云）
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

    // 先用歌曲名搜索一个网易云歌曲ID
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

    on_progress(5);

    // 下载音频文件（流式读取以支持进度）
    let referer = match song.source {
        MusicSource::Kuwo | MusicSource::Kugou => "https://www.kuwo.cn/",
        MusicSource::NetEase => "https://music.163.com/",
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
    let response = match client.get(&mp3_url)
        .header("Referer", "https://www.kuwo.cn/")
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

/// 获取网易云音乐下载链接（备用）
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
