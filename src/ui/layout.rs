use std::time::Duration;

/// 进度条布局信息（用于鼠标点击定位）
#[derive(Debug, Clone, Copy)]
pub(super) struct ProgressBarLayout {
    /// 进度条所在行
    pub(super) row: u16,
    /// 进度条方括号内的起始列（0-based）
    pub(super) bar_start_col: usize,
    /// 进度条方括号内的宽度（字符数）
    pub(super) bar_width: usize,
}

/// 音量条布局信息（用于鼠标点击定位）
#[derive(Debug, Clone, Copy)]
pub(super) struct VolumeBarLayout {
    /// 音量条所在行
    pub(super) row: u16,
    /// 音量条方括号内的起始列（0-based）
    pub(super) bar_start_col: usize,
    /// 音量条方括号内的宽度（字符数）
    pub(super) bar_width: usize,
}

/// 播放列表布局信息（用于鼠标交互）
#[derive(Debug, Clone, Copy)]
pub(super) struct PlaylistLayout {
    /// 列表起始行（0-based）
    pub(super) start_row: u16,
    /// 可见歌曲行数
    pub(super) visible_count: usize,
    /// 左侧栏宽度
    pub(super) left_width: u16,
}

/// 歌词区域布局信息（用于鼠标拖动跳转）
#[derive(Debug, Clone)]
pub(super) struct LyricsAreaLayout {
    /// 歌词区域起始行（0-based）
    pub(super) start_row: u16,
    /// 歌词区域起始列（0-based）
    pub(super) start_col: usize,
    /// 歌词区域宽度
    pub(super) width: usize,
    /// 当前可见歌词行对应的时间戳
    pub(super) line_times: Vec<Duration>,
}
