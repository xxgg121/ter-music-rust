// 歌词解析模块

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// 后台歌词下载任务的结果
pub struct LyricsDownloadResult {
    /// 对应的音乐文件路径
    pub music_path: PathBuf,
    /// 下载并解析后的歌词（可能为 None）
    pub lyrics: Option<Lyrics>,
}

/// 歌词行
#[derive(Debug, Clone)]
pub struct LyricLine {
    /// 时间戳（毫秒）
    pub time: Duration,
    /// 歌词文本
    pub text: String,
}

/// 歌词解析器
#[derive(Debug, Clone)]
pub struct Lyrics {
    /// 歌词行列表（按时间排序）
    lines: Vec<LyricLine>,
}

impl Lyrics {
    /// 创建空歌词
    pub fn new() -> Self {
        Lyrics { lines: Vec::new() }
    }

    /// 检查本地歌词文件是否存在
    #[allow(dead_code)]
    pub fn has_local_lyrics(music_path: &Path) -> bool {
        Self::find_local_lyrics(music_path).is_some()
    }

    /// 仅从本地 .lrc 文件加载歌词（不下载，不阻塞）
    /// 返回 None 表示本地没有歌词文件
    pub fn from_local_lrc(path: &Path) -> Option<Self> {
        let lrc_path = Self::find_local_lyrics(path)?;
        let mut file = File::open(&lrc_path).ok()?;

        // 读取文件内容为字节
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).ok()?;

        // 尝试检测编码并解码
        let content = if let Ok(utf8_content) = String::from_utf8(buffer.clone()) {
            utf8_content
        } else {
            let (cow, _encoding_used, _had_errors) = encoding_rs::GBK.decode(&buffer);
            cow.into_owned()
        };

        Self::parse_lyrics_content(&content)
    }

    /// 在后台线程中下载歌词
    /// 返回接收器，调用者在主循环中 poll 检查结果
    pub fn download_lyrics_background(music_path: &Path) -> mpsc::Receiver<LyricsDownloadResult> {
        let path = music_path.to_path_buf();
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let result = Self::download_and_parse(&path);
            let _ = tx.send(LyricsDownloadResult {
                music_path: path,
                lyrics: result,
            });
        });

        rx
    }

    /// 下载歌词并保存到本地（后台线程调用）
    fn download_and_parse(music_path: &Path) -> Option<Self> {
        if let Some(content) = Self::download_lyrics(music_path) {
            // 构建干净的歌词文件名（去掉标记和首尾空白/全角空格）
            let clean_name = music_path
                .file_stem()?
                .to_str()?
                .replace("[网易]", "")
                .replace("【QQ音乐v2】", "")
                .trim()
                .trim_matches('\u{3000}')
                .to_string();

            let parent = music_path.parent()?;
            let lrc_path = parent.join(format!("{}.lrc", clean_name));

            // 保存歌词文件
            let _ = std::fs::write(&lrc_path, &content);

            return Self::parse_lyrics_content(&content);
        }
        None
    }

    /// 从 .lrc 文件加载歌词，如果不存在则自动下载（同步版本，保留向后兼容）
    #[allow(dead_code)]
    pub fn from_lrc_file(path: &Path) -> Option<Self> {
        // 先尝试本地
        if let Some(lyrics) = Self::from_local_lrc(path) {
            return Some(lyrics);
        }

        // 本地没有，同步下载
        Self::download_and_parse(path)
    }

    /// 查找本地歌词文件（包括带标记的文件名）
    fn find_local_lyrics(music_path: &Path) -> Option<PathBuf> {
        let file_stem = music_path.file_stem()?.to_str()?;
        let parent = music_path.parent()?;

        // 规范化文件名：去除首尾空白（包括全角空格 \u{3000}）
        let trimmed_stem = file_stem.trim().trim_matches('\u{3000}');

        // 尝试多种可能的文件名（同时尝试带/不带首尾空白）
        let possible_names = vec![
            format!("{}.lrc", file_stem),       // 原始名称
            format!("{}.lrc", trimmed_stem),     // 去除首尾空白后的名称
            format!("{} [网易].lrc", file_stem), // 带网易标记
            format!("{} [网易].lrc", trimmed_stem),
            format!("{}【QQ音乐v2】.lrc", file_stem), // 带QQ音乐标记
            format!("{}【QQ音乐v2】.lrc", trimmed_stem),
        ];

        for name in possible_names {
            let lrc_path = parent.join(&name);
            if lrc_path.exists() {
                return Some(lrc_path);
            }
        }

        // 如果还没找到，尝试在目录中查找相似的文件名
        if let Ok(entries) = std::fs::read_dir(parent) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|ext| ext == "lrc").unwrap_or(false) {
                if let Some(lrc_stem) = path.file_stem().and_then(|s| s.to_str()) {
                    // 去掉标记和首尾空白（包括全角空格）后比较
                    let clean_lrc = lrc_stem
                        .replace("[网易]", "")
                        .replace("【QQ音乐v2】", "");
                    let clean_lrc = clean_lrc.trim().trim_matches('\u{3000}');

                    if clean_lrc == trimmed_stem {
                        return Some(path);
                    }
                }
                }
            }
        }

        None
    }

    /// 解析歌词内容
    fn parse_lyrics_content(content: &str) -> Option<Self> {
        let mut lines = Vec::new();
        
        for line in content.lines() {
            let parsed = parse_lrc_line(line);
            lines.extend(parsed);
        }

        // 按时间排序
        lines.sort_by_key(|l| l.time);

        if lines.is_empty() {
            None
        } else {
            Some(Lyrics { lines })
        }
    }

    /// 从网络下载歌词
    fn download_lyrics(music_path: &Path) -> Option<String> {
        // 创建复用的 HTTP 客户端
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .ok()?;

        // 从文件名提取歌手和歌名
        let file_stem = music_path.file_stem()?.to_str()?;
        let (artist, title) = Self::parse_artist_title(file_stem)?;
        
        // 尝试多个歌词 API
        let apis = vec![
            // API 1: 歌词下载网站
            format!(
                "https://api.lrc.cx/v1/search?name={}",
                urlencoding::encode(&format!("{} {}", artist, title))
            ),
            
            // API 2: 备用 API
            format!(
                "https://music.163.com/api/search/get?s={}&type=1",
                urlencoding::encode(&format!("{} {}", artist, title))
            ),
        ];
        
        for api_url in apis {
            if let Some(content) = Self::try_download_from_api(&client, &api_url) {
                return Some(content);
            }
        }
        
        None
    }
    
    /// 尝试从指定 API 下载歌词
    fn try_download_from_api(client: &reqwest::blocking::Client, api_url: &str) -> Option<String> {
        // 发送请求
        let response = client.get(api_url).send().ok()?;
        let text = response.text().ok()?;
        
        // 尝试解析 JSON 响应
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
            // 尝试不同的 JSON 格式
            
            // 格式1: 直接返回歌词
            if let Some(lyric) = json.get("lyric").and_then(|l| l.as_str()) {
                return Some(lyric.to_string());
            }
            
            // 格式2: results 数组
            if let Some(results) = json.get("results").and_then(|r| r.as_array()) {
                if let Some(first) = results.first() {
                    // 尝试获取歌词内容
                    if let Some(lyric) = first.get("lyric").and_then(|l| l.as_str()) {
                        return Some(lyric.to_string());
                    }
                    // 尝试获取下载链接
                    if let Some(url) = first.get("url").and_then(|u| u.as_str()) {
                        if let Some(content) = Self::download_lyric_file(client, url) {
                            return Some(content);
                        }
                    }
                }
            }
            
            // 格式3: result 对象
            if let Some(result) = json.get("result") {
                if let Some(songs) = result.get("songs").and_then(|s| s.as_array()) {
                    if let Some(song) = songs.first() {
                        if let Some(id) = song.get("id").and_then(|i| i.as_i64()) {
                            // 使用歌曲 ID 获取歌词
                            let lrc_url = format!("https://music.163.com/api/song/lyric?id={}&lv=1", id);
                            if let Some(content) = Self::fetch_lyric_by_id(client, &lrc_url) {
                                return Some(content);
                            }
                        }
                    }
                }
            }
            
            // 格式4: data 数组
            if let Some(data) = json.get("data").and_then(|d| d.as_array()) {
                if let Some(first) = data.first() {
                    if let Some(url) = first.get("url").and_then(|u| u.as_str()) {
                        if let Some(content) = Self::download_lyric_file(client, url) {
                            return Some(content);
                        }
                    }
                }
            }
        }
        
        None
    }
    
    /// 下载歌词文件
    fn download_lyric_file(client: &reqwest::blocking::Client, url: &str) -> Option<String> {
        let response = client.get(url).send().ok()?;
        let bytes = response.bytes().ok()?;
        
        // 尝试解码
        if let Ok(utf8_content) = String::from_utf8(bytes.to_vec()) {
            Some(utf8_content)
        } else {
            // 尝试 GBK 解码
            let (cow, _, _) = encoding_rs::GBK.decode(&bytes);
            Some(cow.into_owned())
        }
    }
    
    /// 通过歌曲 ID 获取歌词
    fn fetch_lyric_by_id(client: &reqwest::blocking::Client, url: &str) -> Option<String> {
        let response = client.get(url).send().ok()?;
        let text = response.text().ok()?;
        let json: serde_json::Value = serde_json::from_str(&text).ok()?;
        
        json.get("lrc")
            .and_then(|l| l.get("lyric"))
            .and_then(|l| l.as_str())
            .map(|lrc| lrc.to_string())
    }

    /// 从文件名解析歌手和歌名
    /// 格式: "歌手 - 歌名" 或 "歌名"
    fn parse_artist_title(filename: &str) -> Option<(String, String)> {
        // 尝试按 " - " 分割
        if let Some(pos) = filename.find(" - ") {
            let artist = filename[..pos].trim().to_string();
            let title = filename[pos + 3..].trim().to_string();
            Some((artist, title))
        } else {
            // 没有分隔符，整个作为歌名
            Some(("".to_string(), filename.to_string()))
        }
    }

    /// 根据当前播放时间获取歌词行索引
    /// 使用二分查找提升性能（歌词行按时间排序）
    pub fn get_current_index(&self, current_time: Duration) -> Option<usize> {
        if self.lines.is_empty() {
            return None;
        }

        // 二分查找：找到最后一个 time <= current_time 的歌词行
        let idx = self.lines.partition_point(|line| line.time <= current_time);
        if idx == 0 {
            None
        } else {
            Some(idx - 1)
        }
    }

    /// 获取歌词行
    #[allow(dead_code)]
    pub fn get_lines(&self) -> &[LyricLine] {
        &self.lines
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    /// 获取可见歌词范围
    /// 返回 (起始索引, 可见歌词列表, 当前高亮索引)
    pub fn get_visible_lines(
        &self,
        current_time: Duration,
        visible_count: usize,
    ) -> (usize, Vec<&LyricLine>, Option<usize>) {
        if self.lines.is_empty() {
            return (0, Vec::new(), None);
        }

        let current_idx = self.get_current_index(current_time).unwrap_or(0);
        
        // 计算可见范围，让当前歌词尽量在中间
        let half = visible_count / 2;
        let start = current_idx.saturating_sub(half);

        let end = std::cmp::min(start + visible_count, self.lines.len());
        let actual_start = if end == self.lines.len() && self.lines.len() > visible_count {
            self.lines.len() - visible_count
        } else {
            start
        };

        let visible: Vec<&LyricLine> = self.lines[actual_start..end].iter().collect();
        let highlight_idx = current_idx.checked_sub(actual_start);

        (actual_start, visible, highlight_idx)
    }
}

impl Default for Lyrics {
    fn default() -> Self {
        Self::new()
    }
}

/// 解析 LRC 歌词行
/// 格式: [00:12.34]歌词文本
/// 或: [00:12.34][00:45.67]歌词文本 (多个时间标签)
fn parse_lrc_line(line: &str) -> Vec<LyricLine> {
    let mut results = Vec::new();
    let mut times = Vec::new();
    let mut text_start = 0;

    // 查找所有时间标签
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        if chars[i] == '[' {
            // 查找对应的 ]
            let mut j = i + 1;
            while j < chars.len() && chars[j] != ']' {
                j += 1;
            }
            
            if j < chars.len() {
                let tag: String = chars[i+1..j].iter().collect();
                
                // 尝试解析时间标签
                if let Some(duration) = parse_time_tag(&tag) {
                    times.push(duration);
                }
                
                text_start = j + 1;
                i = j + 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    // 提取歌词文本
    let text: String = chars[text_start..].iter().collect();
    let text = text.trim();

    // 如果没有时间标签，跳过这一行
    if times.is_empty() {
        return results;
    }

    // 为每个时间标签创建歌词行
    for time in times {
        // 即使文本为空也创建歌词行（可能是间奏部分）
        results.push(LyricLine {
            time,
            text: text.to_string(),
        });
    }

    results
}

/// 解析时间标签
/// 格式: mm:ss.xx 或 mm:ss.xxx
fn parse_time_tag(tag: &str) -> Option<Duration> {
    // 格式: mm:ss.xx
    let parts: Vec<&str> = tag.split(':').collect();
    if parts.len() != 2 {
        return None;
    }

    let minutes: u64 = parts[0].parse().ok()?;
    
    // 秒数部分可能包含小数点
    let seconds_parts: Vec<&str> = parts[1].split('.').collect();
    let seconds: u64 = seconds_parts.first()?.parse().ok()?;
    
    let milliseconds: u64 = if seconds_parts.len() > 1 {
        // 补齐到3位
        let ms_str = seconds_parts[1];
        let ms: u64 = ms_str.parse().unwrap_or(0);
        if ms_str.len() == 1 {
            ms * 100
        } else if ms_str.len() == 2 {
            ms * 10
        } else {
            ms
        }
    } else {
        0
    };

    let total_ms = minutes * 60 * 1000 + seconds * 1000 + milliseconds;
    Some(Duration::from_millis(total_ms))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_time_tag() {
        assert_eq!(
            parse_time_tag("01:23.45"),
            Some(Duration::from_millis(83450))
        );
        assert_eq!(
            parse_time_tag("00:05.00"),
            Some(Duration::from_millis(5000))
        );
    }

    #[test]
    fn test_parse_lrc_line() {
        let lines = parse_lrc_line("[00:12.34]Hello World");
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].time, Duration::from_millis(12340));
        assert_eq!(lines[0].text, "Hello World");
    }
}
