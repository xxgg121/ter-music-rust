// 桌面歌词悬浮窗模块
// 按键映射:
//   z — 开关桌面歌词
//   PgUp/PgDn — 切换屏幕底部/顶部位置
//   ↑/↓ — 调整背景透明度 0%-100%，歌词文字始终比背景高10%
//   鼠标拖动 — 移动窗口位置
//   点击聚焦后支持: ←→上下曲, Space暂停, []/,.快进快退, +/-音量(含小键盘),
//                    1-5模式(含小键盘), PgUp/PgDn位置, ↑↓透明度, T切换主题

use std::sync::mpsc;
use chrono::Local;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DesktopLyricsPosition { Bottom, Top }

impl DesktopLyricsPosition {
    pub fn toggle(self) -> Self {
        match self { Self::Bottom => Self::Top, Self::Top => Self::Bottom }
    }
    pub fn config_key(self) -> &'static str {
        match self { Self::Bottom => "bottom", Self::Top => "top" }
    }
    pub fn from_config_key(s: &str) -> Self {
        if s.eq_ignore_ascii_case("top") { Self::Top } else { Self::Bottom }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DesktopLyricsScrollMode { Vertical, Horizontal, Karaoke }

impl DesktopLyricsScrollMode {
    pub fn toggle(self) -> Self {
        match self {
            Self::Vertical => Self::Horizontal,
            Self::Horizontal => Self::Karaoke,
            Self::Karaoke => Self::Vertical,
        }
    }
    pub fn config_key(self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
            Self::Karaoke => "karaoke",
        }
    }
    pub fn from_config_key(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "horizontal" => Self::Horizontal,
            "karaoke" => Self::Karaoke,
            _ => Self::Vertical,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DesktopLyricsCommand {
    pub lyrics_text: String,
    pub position: DesktopLyricsPosition,
    pub theme_name: String,
    pub alpha: u8,        // 255=不变, 0-100=设定值
    pub x: i32,           // -1=不变
    pub y: i32,           // -1=不变
    pub visible: i8,      // -1=不变, 0=隐藏, 1=显示
    pub scroll_mode: DesktopLyricsScrollMode,
    /// 所有歌词的JSON格式: [{"text": "歌词", "time": 秒}, ...]
    pub all_lyrics_json: String,
    /// 当前播放时间（秒）
    pub current_time_sec: f64,
}

impl DesktopLyricsCommand {
    /// 创建一个基础命令，其他字段可按需覆盖
    fn basic() -> Self {
        Self {
            lyrics_text: String::new(),
            position: DesktopLyricsPosition::Bottom,
            theme_name: String::new(),
            alpha: 255,
            x: -1,
            y: -1,
            visible: -1,
            scroll_mode: DesktopLyricsScrollMode::Vertical,
            all_lyrics_json: String::new(),
            current_time_sec: 0.0,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum DesktopLyricsEvent {
    PositionChanged { x: i32, y: i32 },
    KeyPress { key: String },
    ScrollModeChanged { scroll_mode: DesktopLyricsScrollMode },
}

pub struct DesktopLyricsHandle {
    sender: Option<mpsc::Sender<DesktopLyricsCommand>>,
    event_rx: Option<mpsc::Receiver<DesktopLyricsEvent>>,
    #[cfg(not(windows))]
    child_process: Option<std::process::Child>,
    active: bool,
    position: DesktopLyricsPosition,
    alpha: u8,
    x: i32,
    y: i32,
    scroll_mode: DesktopLyricsScrollMode,
}

fn append_desktop_lyrics_log(message: &str) {
    let timestamp = Local::now().format("%H:%M:%S%.3f");
    let line = format!("[{}] [desktop_lyrics] {}\n", timestamp, message);
    let _ = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(crate::config::get_daily_log_path())
        .and_then(|mut f| std::io::Write::write_all(&mut f, line.as_bytes()));
}

impl DesktopLyricsHandle {
    pub fn new() -> Self {
        Self {
            sender: None,
            event_rx: None,
            #[cfg(not(windows))]
            child_process: None,
            active: false,
            position: DesktopLyricsPosition::Bottom,
            alpha: 70,
            x: -1,
            y: -1,
            scroll_mode: DesktopLyricsScrollMode::Vertical,
        }
    }
    pub fn is_active(&self) -> bool { self.active }
    pub fn position(&self) -> DesktopLyricsPosition { self.position }
    pub fn alpha(&self) -> u8 { self.alpha }
    pub fn scroll_mode(&self) -> DesktopLyricsScrollMode { self.scroll_mode }
    pub fn set_position(&mut self, p: DesktopLyricsPosition) { self.position = p; }
    pub fn set_alpha(&mut self, a: u8) { self.alpha = a.clamp(0, 100); }
    pub fn set_coords(&mut self, x: i32, y: i32) { self.x = x; self.y = y; }
    pub fn set_scroll_mode(&mut self, m: DesktopLyricsScrollMode) { self.scroll_mode = m; }
    #[allow(dead_code)]
    pub fn toggle_scroll_mode(&mut self) {
        self.scroll_mode = self.scroll_mode.toggle();
        let mut cmd = DesktopLyricsCommand::basic();
        cmd.position = self.position;
        cmd.scroll_mode = self.scroll_mode;
        self.send_cmd(cmd);
    }

    pub fn toggle_position(&mut self) {
        self.position = self.position.toggle();
        self.x = -1; self.y = -1;
        let mut cmd = DesktopLyricsCommand::basic();
        cmd.position = self.position;
        cmd.scroll_mode = self.scroll_mode;
        self.send_cmd(cmd);
    }

    pub fn adjust_alpha(&mut self, step: i8) {
        let new_a = if step > 0 { (self.alpha as i16 + 5).min(100) as u8 }
                    else { (self.alpha as i16 - 5).max(0) as u8 };
        if new_a != self.alpha { self.alpha = new_a; self.send_alpha(); }
    }

    fn send_alpha(&self) {
        let mut cmd = DesktopLyricsCommand::basic();
        cmd.alpha = self.alpha;
        cmd.position = self.position;
        cmd.scroll_mode = self.scroll_mode;
        self.send_cmd(cmd);
    }

    fn send_cmd(&self, cmd: DesktopLyricsCommand) {
        if let Some(ref s) = self.sender { let _ = s.send(cmd); }
    }

    pub fn open(&mut self, theme_name: &str) {
        if self.active { return; }

        // 检测已有 sender 是否仍然存活
        let sender_alive = self.sender.as_ref().map_or(false, |s| {
            let mut cmd = DesktopLyricsCommand::basic();
            cmd.position = self.position;
            cmd.scroll_mode = self.scroll_mode;
            s.send(cmd).is_ok()
        });

        if !sender_alive {
            if self.sender.is_some() {
                append_desktop_lyrics_log("open: existing sender is dead, clearing for re-launch");
            }
            self.sender = None;
            self.event_rx = None;
            #[cfg(not(windows))]
            {
                self.kill_child_process();
            }
        }

        if self.sender.is_none() {
            self.launch_window(theme_name);
        }

        if self.sender.is_some() {
            self.active = true;
            let mut cmd = DesktopLyricsCommand::basic();
            cmd.theme_name = theme_name.to_string();
            cmd.alpha = self.alpha;
            cmd.x = self.x;
            cmd.y = self.y;
            cmd.visible = 1;
            cmd.position = self.position;
            cmd.scroll_mode = self.scroll_mode;
            self.send_cmd(cmd);
        } else {
            self.active = false;
            let msg = "open requested but sender is not ready, keep active=false";
            eprintln!("[desktop_lyrics] {}", msg);
            append_desktop_lyrics_log(msg);
        }
    }

    pub fn close(&mut self) {
        self.active = false;
        let mut cmd = DesktopLyricsCommand::basic();
        cmd.visible = 0;
        cmd.position = self.position;
        cmd.scroll_mode = self.scroll_mode;
        self.send_cmd(cmd);
        #[cfg(not(windows))]
        {
            self.kill_child_process();
        }
    }

    #[cfg(not(windows))]
    fn kill_child_process(&mut self) {
        if let Some(ref mut child) = self.child_process {
            append_desktop_lyrics_log("killing desktop lyrics child process");
            let _ = child.kill();
            let _ = child.wait();
        }
        self.child_process = None;
    }

    pub fn toggle(&mut self, theme_name: &str) {
        if self.active { self.close(); } else { self.open(theme_name); }
    }

    pub fn update_lyrics(&self, text: &str, theme_name: &str) {
        let mut cmd = DesktopLyricsCommand::basic();
        cmd.lyrics_text = text.to_string();
        cmd.theme_name = theme_name.to_string();
        cmd.position = self.position;
        cmd.scroll_mode = self.scroll_mode;
        self.send_cmd(cmd);
    }

    pub fn update_all_lyrics(&self, all_lyrics: &[(String, f64)], current_time_sec: f64, theme_name: &str) {
        let mut cmd = DesktopLyricsCommand::basic();
        cmd.all_lyrics_json = serde_json::to_string(all_lyrics).unwrap_or_default();
        cmd.current_time_sec = current_time_sec;
        cmd.theme_name = theme_name.to_string();
        cmd.position = self.position;
        cmd.scroll_mode = self.scroll_mode;
        self.send_cmd(cmd);
    }

    pub fn update_theme(&self, theme_name: &str) {
        // 更新本地缓存的主题名，确保下次命令处理时使用最新值
        if let Some(ref s) = self.sender {
            let mut cmd = DesktopLyricsCommand::basic();
            cmd.theme_name = theme_name.to_string();
            cmd.position = self.position;
            cmd.scroll_mode = self.scroll_mode;
            let _ = s.send(cmd);
        }
    }

    pub fn move_window(&mut self, x: i32, y: i32) {
        self.x = x; self.y = y;
        let mut cmd = DesktopLyricsCommand::basic();
        cmd.x = x;
        cmd.y = y;
        cmd.position = self.position;
        cmd.scroll_mode = self.scroll_mode;
        self.send_cmd(cmd);
    }

    pub fn try_recv_event(&self) -> Option<DesktopLyricsEvent> {
        self.event_rx.as_ref().and_then(|rx| rx.try_recv().ok())
    }

    pub fn get_position_coords(&self) -> (i32, i32) { (self.x, self.y) }

    #[cfg(windows)]
    fn launch_window(&mut self, theme_name: &str) {
        let (tx, rx) = mpsc::channel();
        let (ev_tx, ev_rx) = mpsc::channel();
        let pos = self.position;
        let theme = theme_name.to_string();
        let alpha = self.alpha;
        let (ix, iy) = (self.x, self.y);
        let scroll_mode = self.scroll_mode;
        std::thread::spawn(move || {
            windows_impl::run_desktop_lyrics_window(rx, pos, &theme, alpha, ix, iy, ev_tx, scroll_mode);
        });
        self.sender = Some(tx);
        self.event_rx = Some(ev_rx);
        self.active = true;
    }

    #[cfg(not(windows))]
    fn launch_window(&mut self, theme_name: &str) {
        // Linux 上 winit 0.30 限制整个进程只能有一个 EventLoop，
        // 因此通过子进程方式创建桌面歌词窗口。
        // 父进程通过 stdin 发送 JSON 命令，通过 stdout 接收 JSON 事件。
        let exe = match std::env::current_exe() {
            Ok(p) => p,
            Err(e) => {
                let msg = format!("open failed: cannot get current exe path: {:?}", e);
                eprintln!("[desktop_lyrics] {}", msg);
                append_desktop_lyrics_log(&msg);
                self.active = false;
                return;
            }
        };

        let mut cmd = std::process::Command::new(&exe);
        cmd.arg("--desktop-lyrics")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::inherit());

        // 传递初始参数作为环境变量
        cmd.env("TER_DESKTOP_LYRICS_POSITION", self.position.config_key())
            .env("TER_DESKTOP_LYRICS_THEME", theme_name)
            .env("TER_DESKTOP_LYRICS_ALPHA", self.alpha.to_string())
            .env("TER_DESKTOP_LYRICS_X", self.x.to_string())
            .env("TER_DESKTOP_LYRICS_Y", self.y.to_string())
            .env("TER_DESKTOP_LYRICS_SCROLL_MODE", self.scroll_mode.config_key());

        let mut child = match cmd.spawn() {
            Ok(c) => c,
            Err(e) => {
                let msg = format!("open failed: cannot spawn child process: {:?}", e);
                eprintln!("[desktop_lyrics] {}", msg);
                append_desktop_lyrics_log(&msg);
                self.active = false;
                return;
            }
        };

        let child_stdin = match child.stdin.take() {
            Some(s) => s,
            None => {
                let msg = "open failed: child stdin is None";
                eprintln!("[desktop_lyrics] {}", msg);
                append_desktop_lyrics_log(msg);
                let _ = child.kill();
                let _ = child.wait();
                self.active = false;
                return;
            }
        };
        let child_stdout = match child.stdout.take() {
            Some(s) => s,
            None => {
                let msg = "open failed: child stdout is None";
                eprintln!("[desktop_lyrics] {}", msg);
                append_desktop_lyrics_log(msg);
                let _ = child.kill();
                let _ = child.wait();
                self.active = false;
                return;
            }
        };

        // 创建双向通道：父线程通过 mpsc channel 发送命令，
        // 后台线程将命令序列化为 JSON 写入子进程 stdin。
        let (tx, rx) = mpsc::channel::<DesktopLyricsCommand>();
        let (ev_tx, ev_rx) = mpsc::channel::<DesktopLyricsEvent>();

        // 写入线程：从 mpsc channel 读取命令，写入子进程 stdin
        let mut stdin_writer = std::io::BufWriter::new(child_stdin);
        std::thread::spawn(move || {
            for cmd in rx {
                match serde_json::to_string(&cmd) {
                    Ok(json) => {
                        use std::io::Write;
                        if writeln!(stdin_writer, "{}", json).is_err() {
                            break;
                        }
                        if stdin_writer.flush().is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        // 读取线程：从子进程 stdout 读取 JSON 事件，发送到 mpsc channel
        let mut stdout_reader = std::io::BufReader::new(child_stdout);
        std::thread::spawn(move || {
            use std::io::BufRead;
            let mut line = String::new();
            loop {
                line.clear();
                match stdout_reader.read_line(&mut line) {
                    Ok(0) => break, // EOF
                    Ok(_) => {
                        if let Ok(event) = serde_json::from_str::<DesktopLyricsEvent>(line.trim()) {
                            if ev_tx.send(event).is_err() {
                                break;
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        self.sender = Some(tx);
        self.event_rx = Some(ev_rx);
        self.child_process = Some(child);
        self.active = true;
        append_desktop_lyrics_log("unix child process spawned successfully");
    }
}

impl Drop for DesktopLyricsHandle {
    fn drop(&mut self) {
        self.close();
        #[cfg(not(windows))]
        {
            self.kill_child_process();
        }
    }
}

#[cfg(windows)]
mod windows_impl {
    use std::sync::mpsc;
    use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
    use winapi::shared::windef::{HDC, HWND, POINT, RECT};
    use winapi::um::libloaderapi::GetModuleHandleW;
    use winapi::um::wingdi::{
        CreateCompatibleDC, CreateDIBSection, CreateFontIndirectW,
        CreateSolidBrush, DeleteDC, DeleteObject, LOGFONTW,
        SelectObject, SetBkMode, SetTextColor, RGB,
        BI_RGB, DIB_RGB_COLORS, FW_NORMAL, FW_BOLD,
        TRANSPARENT, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS,
        DEFAULT_CHARSET, CLEARTYPE_QUALITY,
        AC_SRC_OVER, AC_SRC_ALPHA, BLENDFUNCTION,
    };
    use winapi::um::winuser::{
        CreateWindowExW, DefWindowProcW, DispatchMessageW, DrawTextW, FillRect,
        GetClientRect, GetCursorPos, GetMessageW, GetWindowLongPtrW, GetWindowRect,
        GWLP_USERDATA, KillTimer, LoadCursorW, PostQuitMessage,
        RegisterClassW, ReleaseCapture, SetCapture,
        SetFocus, SetTimer, SetWindowLongPtrW, SetWindowPos,
        ShowWindow, SystemParametersInfoW, TranslateMessage, UpdateWindow,
        UpdateLayeredWindow,
        CS_HREDRAW, CS_VREDRAW, DT_CENTER, DT_NOPREFIX, DT_SINGLELINE,
        DT_VCENTER, DT_WORD_ELLIPSIS, DT_LEFT, DT_CALCRECT, HTCLIENT, HTTRANSPARENT,
        HWND_TOPMOST, IDC_ARROW, ULW_ALPHA,
        SPI_GETWORKAREA, SWP_NOACTIVATE, SWP_NOSIZE, SWP_SHOWWINDOW, SW_SHOW,
        WM_CREATE, WM_DESTROY, WM_ERASEBKGND, WM_KEYDOWN, WM_LBUTTONDOWN,
        WM_LBUTTONUP, WM_MOUSEMOVE, WM_NCHITTEST, WM_RBUTTONUP, WM_TIMER, WM_SIZE,
        WS_EX_LAYERED, WS_EX_TOOLWINDOW, WS_EX_TOPMOST,
        WS_POPUP, WS_VISIBLE, MSG, WNDCLASSW,
    };

    use super::{DesktopLyricsCommand, DesktopLyricsEvent, DesktopLyricsPosition, DesktopLyricsScrollMode};

    const TIMER_ID: usize = 1;
    const TIMER_INTERVAL_MS: u32 = 50;

    struct WindowData {
        all_lyrics: Vec<(String, f64)>,
        current_time_sec: f64,
        scroll_offset: f32,
        old_all_lyrics: Vec<(String, f64)>,
        old_scroll_offset: f32,
        old_prev_text: String, old_curr_text: String, old_next_text: String,
        prev_text: String, curr_text: String, next_text: String,
        scroll_progress: f32,
        transitioning: bool,
        position: DesktopLyricsPosition,
        theme_name: String,
        alpha: u8,
        x: i32, y: i32,
        dragging: bool,
        drag_offset_x: i32, drag_offset_y: i32,
        rx: mpsc::Receiver<DesktopLyricsCommand>,
        ev_tx: mpsc::Sender<DesktopLyricsEvent>,
        scroll_mode: DesktopLyricsScrollMode,
        horizontal_progress: f32,
        karaoke_line_group: usize,
        old_karaoke_line_group: usize,
        karaoke_transition_progress: f32,
        horizontal_target_offset: f32,
    }

    fn theme_colors(name: &str) -> ((u8,u8,u8),(u8,u8,u8),(u8,u8,u8)) {
        match name {
            "GrayWhite" => ((30, 30, 30), (224, 233, 246), (188, 194, 202)),
            "Neon" => ((0, 20, 40), (255, 235, 80), (0, 255, 120)),
            "Sunset" => ((40, 15, 0), (255, 246, 120), (255, 197, 120)),
            "Ocean" => ((0, 20, 40), (168, 255, 245), (116, 243, 204)),
            _ => ((30, 30, 30), (224, 233, 246), (188, 194, 202)),
        }
    }

    unsafe fn create_font(h: i32, bold: bool) -> winapi::shared::windef::HFONT {
        let mut lf = std::mem::zeroed::<LOGFONTW>();
        lf.lfHeight = h;
        lf.lfWeight = if bold { FW_BOLD as i32 } else { FW_NORMAL as i32 };
        lf.lfQuality = CLEARTYPE_QUALITY as u8;
        lf.lfOutPrecision = OUT_DEFAULT_PRECIS as u8;
        lf.lfClipPrecision = CLIP_DEFAULT_PRECIS as u8;
        lf.lfPitchAndFamily = DEFAULT_CHARSET as u8;
        let face: Vec<u16> = "Consolas\0".encode_utf16().collect();//Cascadia Mono
        std::ptr::copy_nonoverlapping(face.as_ptr(), lf.lfFaceName.as_mut_ptr(), face.len().min(32));
        CreateFontIndirectW(&lf)
    }

    unsafe fn get_work_area() -> RECT {
        let mut r = std::mem::zeroed::<RECT>();
        SystemParametersInfoW(SPI_GETWORKAREA, 0, &mut r as *mut _ as _, 0);
        r
    }

    fn clean_text(s: &str) -> String {
        s.trim_end_matches(|c: char| c.is_control() || c == '\u{3000}').to_string()
    }

    fn karaoke_group_start(all_lyrics: &[(String, f64)], current_idx: usize) -> usize {
        let visible_count = all_lyrics
            .iter()
            .take(current_idx.saturating_add(1))
            .filter(|(text, _)| !clean_text(text).is_empty())
            .count();
        if visible_count == 0 {
            return 0;
        }

        let target_visible_idx = ((visible_count - 1) / 4) * 4;
        let mut visible_idx = 0usize;
        for (idx, (text, _)) in all_lyrics.iter().enumerate() {
            if clean_text(text).is_empty() {
                continue;
            }
            if visible_idx == target_visible_idx {
                return idx;
            }
            visible_idx += 1;
        }

        0
    }

    fn karaoke_lines_for_group(all_lyrics: &[(String, f64)], group_start: usize, current_idx: usize) -> (Vec<(String, bool)>, bool) {
        let mut lines = Vec::new();
        let mut current_visible = false;
        let mut idx = group_start;
        while lines.len() < 4 && idx < all_lyrics.len() {
            let cleaned = clean_text(&all_lyrics[idx].0);
            if !cleaned.is_empty() {
                let is_current = idx == current_idx;
                current_visible |= is_current;
                lines.push((cleaned, is_current));
            }
            idx += 1;
        }
        (lines, current_visible)
    }

    unsafe fn draw_line(mem_dc: HDC, text: &str, font: winapi::shared::windef::HFONT,
                        color: u32, rect: RECT) {
        let old_font = SelectObject(mem_dc, font as _);
        SetTextColor(mem_dc, color);
        SetBkMode(mem_dc, TRANSPARENT as i32);
        let clean = clean_text(text);
        if !clean.is_empty() {
            let wide: Vec<u16> = clean.encode_utf16().chain(std::iter::once(0)).collect();
            let mut r = rect;
            DrawTextW(mem_dc, wide.as_ptr(), -1, &mut r,
                DT_CENTER | DT_VCENTER | DT_SINGLELINE | DT_NOPREFIX | DT_WORD_ELLIPSIS);
        }
        SelectObject(mem_dc, old_font);
    }

    fn lerp_color(a: (u8,u8,u8), b: (u8,u8,u8), t: f32) -> (u8,u8,u8) {
        let t = t.clamp(0.0, 1.0);
        ((a.0 as f32 + (b.0 as f32 - a.0 as f32) * t) as u8,
         (a.1 as f32 + (b.1 as f32 - a.1 as f32) * t) as u8,
         (a.2 as f32 + (b.2 as f32 - a.2 as f32) * t) as u8)
    }

    unsafe fn render(hwnd: HWND, data: &mut WindowData) {
        let mut client = std::mem::zeroed::<RECT>();
        GetClientRect(hwnd, &mut client);
        let w = client.right - client.left;
        let h = client.bottom - client.top;
        if w <= 0 || h <= 0 { return; }

        let hdc = winapi::um::winuser::GetDC(hwnd);
        if hdc.is_null() { return; }

        let mem_dc = CreateCompatibleDC(hdc);
        let mut bits_ptr: *mut u32 = std::ptr::null_mut();
        let bmp = CreateDIBSection(hdc, &{
            let mut bmi = std::mem::zeroed::<winapi::um::wingdi::BITMAPINFO>();
            bmi.bmiHeader.biSize = std::mem::size_of::<winapi::um::wingdi::BITMAPINFOHEADER>() as u32;
            bmi.bmiHeader.biWidth = w; bmi.bmiHeader.biHeight = h;
            bmi.bmiHeader.biPlanes = 1; bmi.bmiHeader.biBitCount = 32;
            bmi.bmiHeader.biCompression = BI_RGB;
            bmi
        }, DIB_RGB_COLORS, &mut bits_ptr as *mut _ as _, std::ptr::null_mut(), 0);
        if bmp.is_null() || bits_ptr.is_null() {
            if !bmp.is_null() { DeleteObject(bmp as _); }
            DeleteDC(mem_dc);
            winapi::um::winuser::ReleaseDC(hwnd, hdc);
            return;
        }
        let old_bmp = SelectObject(mem_dc, bmp as _);

        let (bg, fg_bright, fg_dim) = theme_colors(&data.theme_name);
        let bg_a255 = ((data.alpha as u32 * 255 + 50) / 100).min(255) as u8;
        let txt_pct = (data.alpha as u32 + 10).min(100);
        let txt_a255 = ((txt_pct * 255 + 50) / 100).min(255) as u8;

        let bg_brush = CreateSolidBrush(RGB(bg.0, bg.1, bg.2));
        FillRect(mem_dc, &client, bg_brush);
        DeleteObject(bg_brush as _);

        match data.scroll_mode {
            DesktopLyricsScrollMode::Vertical => {
                render_vertical(mem_dc, data, w, h, bg, fg_bright, fg_dim, txt_a255);
            }
            DesktopLyricsScrollMode::Horizontal => {
                render_horizontal(mem_dc, data, w, h, bg, fg_bright, fg_dim, txt_a255);
            }
            DesktopLyricsScrollMode::Karaoke => {
                render_karaoke(mem_dc, data, w, h, bg, fg_bright, fg_dim, txt_a255);
            }
        }

        let pixel_count = (w * h) as usize;
        for i in 0..pixel_count {
            let px = *bits_ptr.add(i);
            let pr = (px & 0xFF) as u32;
            let pg = ((px >> 8) & 0xFF) as u32;
            let pb = ((px >> 16) & 0xFF) as u32;
            let is_text = pr != bg.0 as u32 || pg != bg.1 as u32 || pb != bg.2 as u32;
            let a = if is_text { txt_a255 } else { bg_a255 };
            *bits_ptr.add(i) = (pb << 16) | (pg << 8) | pr | ((a as u32) << 24);
        }

        let mut pt_src = POINT { x: 0, y: 0 };
        let mut size = std::mem::zeroed::<winapi::shared::windef::SIZE>();
        size.cx = w; size.cy = h;
        let mut blend = BLENDFUNCTION {
            BlendOp: AC_SRC_OVER as u8,
            BlendFlags: 0,
            SourceConstantAlpha: 255,
            AlphaFormat: AC_SRC_ALPHA as u8,
        };
        UpdateLayeredWindow(hwnd, std::ptr::null_mut(), std::ptr::null_mut(), &mut size,
            mem_dc, &mut pt_src, 0, &mut blend, ULW_ALPHA);

        SelectObject(mem_dc, old_bmp); DeleteObject(bmp as _); DeleteDC(mem_dc);
        winapi::um::winuser::ReleaseDC(hwnd, hdc);
    }

    unsafe fn render_vertical(mem_dc: HDC, data: &WindowData, w: i32, h: i32,
                              bg: (u8,u8,u8), fg_bright: (u8,u8,u8), fg_dim: (u8,u8,u8), _txt_a255: u8) {
        let main_fs = (h as f32 * 0.26) as i32;
        let sub_fs = (main_fs as f32 * 0.70) as i32;
        let main_font = create_font(main_fs, true);
        let sub_font = create_font(sub_fs, false);

        let main_line_h = main_fs + 8;
        let sub_line_h = sub_fs + 6;
        let gap = 4i32;
        let total_h = sub_line_h + gap + main_line_h + gap + sub_line_h;
        let y_start = (h - total_h) / 2;
        let y_prev = y_start;
        let y_curr = y_prev + sub_line_h + gap;
        let y_next = y_curr + main_line_h + gap;
        let spacing = y_curr - y_prev;

        let empty = data.prev_text.is_empty() && data.curr_text.is_empty() && data.next_text.is_empty()
            && data.old_prev_text.is_empty() && data.old_curr_text.is_empty() && data.old_next_text.is_empty();

        if empty {
            draw_line(mem_dc, "♫ ...", main_font,
                RGB(fg_bright.0, fg_bright.1, fg_bright.2),
                RECT{left:4, top:0, right:w-4, bottom:h});
        } else if data.transitioning {
            let t = data.scroll_progress;
            let te = t * t * (3.0 - 2.0 * t);
            let off = (te * spacing as f32) as i32;
            let a_out = 1.0 - te;
            let a_in = te;
            if a_out > 0.05 {
                let ob = lerp_color(fg_bright, bg, 1.0 - a_out);
                let od = lerp_color(fg_dim, bg, 1.0 - a_out);
                draw_line(mem_dc, &data.old_prev_text, sub_font, RGB(od.0,od.1,od.2),
                    RECT{left:4, top:y_prev-off, right:w-4, bottom:y_prev-off+sub_line_h});
                draw_line(mem_dc, &data.old_curr_text, main_font, RGB(ob.0,ob.1,ob.2),
                    RECT{left:4, top:y_curr-off, right:w-4, bottom:y_curr-off+main_line_h});
                draw_line(mem_dc, &data.old_next_text, sub_font, RGB(od.0,od.1,od.2),
                    RECT{left:4, top:y_next-off, right:w-4, bottom:y_next-off+sub_line_h});
            }
            if a_in > 0.05 {
                let nb = lerp_color(fg_bright, bg, 1.0 - a_in);
                let nd = lerp_color(fg_dim, bg, 1.0 - a_in);
                draw_line(mem_dc, &data.prev_text, sub_font, RGB(nd.0,nd.1,nd.2),
                    RECT{left:4, top:y_prev+spacing-off, right:w-4, bottom:y_prev+spacing-off+sub_line_h});
                draw_line(mem_dc, &data.curr_text, main_font, RGB(nb.0,nb.1,nb.2),
                    RECT{left:4, top:y_curr+spacing-off, right:w-4, bottom:y_curr+spacing-off+main_line_h});
                draw_line(mem_dc, &data.next_text, sub_font, RGB(nd.0,nd.1,nd.2),
                    RECT{left:4, top:y_next+spacing-off, right:w-4, bottom:y_next+spacing-off+sub_line_h});
            }
        } else {
            draw_line(mem_dc, &data.prev_text, sub_font, RGB(fg_dim.0,fg_dim.1,fg_dim.2),
                RECT{left:4, top:y_prev, right:w-4, bottom:y_prev+sub_line_h});
            draw_line(mem_dc, &data.curr_text, main_font, RGB(fg_bright.0,fg_bright.1,fg_bright.2),
                RECT{left:4, top:y_curr, right:w-4, bottom:y_curr+main_line_h});
            draw_line(mem_dc, &data.next_text, sub_font, RGB(fg_dim.0,fg_dim.1,fg_dim.2),
                RECT{left:4, top:y_next, right:w-4, bottom:y_next+sub_line_h});
        }

        DeleteObject(main_font as _); DeleteObject(sub_font as _);
    }

    unsafe fn render_horizontal(mem_dc: HDC, data: &mut WindowData, w: i32, h: i32,
                               bg: (u8,u8,u8), fg_bright: (u8,u8,u8), fg_dim: (u8,u8,u8), _txt_a255: u8) {
        let fs = (h as f32 * 0.26) as i32;

        if !data.all_lyrics.is_empty() {
            let current_idx = data.all_lyrics.partition_point(|&(_, t)| t <= data.current_time_sec);
            let current_idx = if current_idx == 0 { 0 } else { current_idx - 1 };

            let gap = 40i32;
            let mut widths = vec![0i32; data.all_lyrics.len()];
            for (i, (text, _)) in data.all_lyrics.iter().enumerate() {
                if text.is_empty() { continue; }
                let font = if i == current_idx { create_font(fs, true) } else { create_font(fs, false) };
                let clean = clean_text(text);
                let wide: Vec<u16> = clean.encode_utf16().chain(std::iter::once(0)).collect();
                let old_f = SelectObject(mem_dc, font as _);
                let mut calc_rect = RECT { left: 0, top: 0, right: 0, bottom: 0 };
                DrawTextW(mem_dc, wide.as_ptr(), -1, &mut calc_rect,
                    DT_LEFT | DT_SINGLELINE | DT_NOPREFIX | DT_CALCRECT);
                widths[i] = calc_rect.right - calc_rect.left;
                SelectObject(mem_dc, old_f);
                DeleteObject(font as _);
            }

            let mut positions = vec![0i32; data.all_lyrics.len()];
            let mut current_x = w;
            for i in 0..data.all_lyrics.len() {
                positions[i] = current_x;
                current_x += widths[i] + gap;
            }

            let current_line_center = if current_idx < positions.len() {
                positions[current_idx] as f32 + widths[current_idx] as f32 * 0.5
            } else {
                w as f32 * 0.5
            };
            let target_offset = (current_line_center - w as f32 * 0.50).max(0.0);
            data.horizontal_target_offset = target_offset;

            let scroll_offset = data.scroll_offset;
            let final_offset = scroll_offset;

            SetBkMode(mem_dc, TRANSPARENT as i32);
            let text_h = fs + 8;
            let y = (h - text_h) / 2;

            for (i, (text, _)) in data.all_lyrics.iter().enumerate() {
                if text.is_empty() { continue; }
                let is_current = i == current_idx;
                let color = if is_current { fg_bright } else { fg_dim };
                let alpha = if is_current { 1.0 } else { 0.6 };
                let blended_color = if alpha >= 1.0 {
                    color
                } else if alpha <= 0.0 {
                    bg
                } else {
                    lerp_color(color, bg, 1.0 - alpha)
                };
                let font = if is_current { create_font(fs, true) } else { create_font(fs, false) };
                let clean = clean_text(text);
                let wide: Vec<u16> = clean.encode_utf16().chain(std::iter::once(0)).collect();
                let old_f = SelectObject(mem_dc, font as _);
                SetTextColor(mem_dc, RGB(blended_color.0, blended_color.1, blended_color.2));
                let x = (positions[i] as f32 - final_offset) as i32;
                let mut r = RECT { left: x, top: y, right: x + widths[i], bottom: y + text_h };
                DrawTextW(mem_dc, wide.as_ptr(), -1, &mut r,
                    DT_LEFT | DT_VCENTER | DT_SINGLELINE | DT_NOPREFIX);
                SelectObject(mem_dc, old_f);
                DeleteObject(font as _);
            }
            return;
        }

        let texts = vec![&data.prev_text, &data.curr_text, &data.next_text];
        let is_curr = vec![false, true, false];

        SetBkMode(mem_dc, TRANSPARENT as i32);
        let text_h = fs + 8;
        let y = (h - text_h) / 2;
        let gap = 40i32;

        let mut widths = vec![];
        let mut total_w = 0i32;
        for text in &texts {
            if text.is_empty() {
                widths.push(0);
                continue;
            }
            let font = create_font(fs, false);
            let clean = clean_text(text);
            let wide: Vec<u16> = clean.encode_utf16().chain(std::iter::once(0)).collect();
            let old_f = SelectObject(mem_dc, font as _);
            let mut calc_rect = RECT { left: 0, top: 0, right: 0, bottom: 0 };
            DrawTextW(mem_dc, wide.as_ptr(), -1, &mut calc_rect,
                DT_LEFT | DT_SINGLELINE | DT_NOPREFIX | DT_CALCRECT);
            let text_w = calc_rect.right - calc_rect.left;
            widths.push(text_w);
            total_w += text_w;
            SelectObject(mem_dc, old_f);
            DeleteObject(font as _);
        }
        total_w += gap * 2;

        let start_x = (w - total_w) / 2;
        let mut current_x = start_x;

        for (i, text) in texts.iter().enumerate() {
            if text.is_empty() { continue; }
            let color = if is_curr[i] { fg_bright } else { fg_dim };
            let font = create_font(fs, is_curr[i]);
            let clean = clean_text(text);
            let wide: Vec<u16> = clean.encode_utf16().chain(std::iter::once(0)).collect();
            let old_f = SelectObject(mem_dc, font as _);
            SetTextColor(mem_dc, RGB(color.0, color.1, color.2));
            let mut r = RECT { left: current_x, top: y, right: current_x + widths[i], bottom: y + text_h };
            DrawTextW(mem_dc, wide.as_ptr(), -1, &mut r,
                DT_LEFT | DT_VCENTER | DT_SINGLELINE | DT_NOPREFIX);
            SelectObject(mem_dc, old_f);
            DeleteObject(font as _);
            current_x += widths[i] + gap;
        }
    }

    unsafe fn render_karaoke(mem_dc: HDC, data: &mut WindowData, w: i32, h: i32,
                              bg: (u8,u8,u8), fg_bright: (u8,u8,u8), fg_dim: (u8,u8,u8), _txt_a255: u8) {
        let fs = (h as f32 * 0.26) as i32;

        let lines: Vec<(String, bool)>;
        let mut old_lines: Vec<(String, bool)> = Vec::new();
        let mut line_char_progress: f64 = 0.0;

        if !data.all_lyrics.is_empty() {
            let current_idx = data.all_lyrics.partition_point(|&(_, t)| t <= data.current_time_sec);
            let current_idx = if current_idx == 0 { 0 } else { current_idx - 1 };

            if data.karaoke_line_group == usize::MAX {
                data.karaoke_line_group = karaoke_group_start(&data.all_lyrics, current_idx);
            }
            let group_start = data.karaoke_line_group;

            let current_visible;
            (lines, current_visible) = karaoke_lines_for_group(&data.all_lyrics, group_start, current_idx);
            if data.karaoke_transition_progress < 1.0 && data.old_karaoke_line_group != usize::MAX && data.old_karaoke_line_group != group_start {
                old_lines = karaoke_lines_for_group(&data.all_lyrics, data.old_karaoke_line_group, current_idx).0;
            }

            if current_visible {
                let current_line_time = data.all_lyrics[current_idx].1;
                let next_line_time = data.all_lyrics.get(current_idx + 1).map(|&(_, t)| t).unwrap_or(current_line_time + 4.0);
                let duration = (next_line_time - current_line_time).max(0.1);
                line_char_progress = ((data.current_time_sec - current_line_time) / duration).clamp(0.0, 1.0);
            }
        } else {
            lines = vec![
                (clean_text(&data.prev_text), false),
                (clean_text(&data.curr_text), true),
                (clean_text(&data.next_text), false),
                (String::new(), false),
            ];
        }

        SetBkMode(mem_dc, TRANSPARENT as i32);

        if !old_lines.is_empty() {
            let t = data.karaoke_transition_progress.clamp(0.0, 1.0);
            let eased = t * t * (3.0 - 2.0 * t);
            let slide = (h as f32 * 0.12) as i32;
            render_karaoke_group(mem_dc, &old_lines, w, h, fs, bg, fg_bright, fg_dim, 1.0 - eased, -(slide as f32 * eased) as i32, 0.0);
            render_karaoke_group(mem_dc, &lines, w, h, fs, bg, fg_bright, fg_dim, eased, (slide as f32 * (1.0 - eased)) as i32, line_char_progress);
        } else {
            render_karaoke_group(mem_dc, &lines, w, h, fs, bg, fg_bright, fg_dim, 1.0, 0, line_char_progress);
        }
    }

    unsafe fn render_karaoke_group(mem_dc: HDC, lines: &[(String, bool)], w: i32, h: i32, fs: i32,
                                   bg: (u8,u8,u8), fg_bright: (u8,u8,u8), fg_dim: (u8,u8,u8), alpha: f32,
                                   y_offset: i32, line_char_progress: f64) {
        if alpha <= 0.01 {
            return;
        }

        let text_h = fs + 6;
        let char_spacing = 0i32; // 字间距
        let gap = (w as f32 * 0.01) as i32; // 句间间隔，逗号大小
        let top_y = (h as f32 * 0.18) as i32;
        let bottom_y = (h as f32 * 0.55) as i32;
        let left_margin = (w as f32 * 0.04) as i32;

        // 预计算左上一行两句的宽度
        let mut top_widths = [0i32; 2];
        for i in 0..2 {
            if i < lines.len() && !lines[i].0.is_empty() {
                let f = create_font(fs, false);
                let clean = clean_text(&lines[i].0);
                let wide: Vec<u16> = clean.encode_utf16().chain(std::iter::once(0)).collect();
                let old_f = SelectObject(mem_dc, f as _);
                let mut calc_r = RECT { left: 0, top: 0, right: 0, bottom: 0 };
                DrawTextW(mem_dc, wide.as_ptr(), -1, &mut calc_r,
                    DT_LEFT | DT_SINGLELINE | DT_NOPREFIX | DT_CALCRECT);
                top_widths[i] = calc_r.right - calc_r.left;
                SelectObject(mem_dc, old_f);
                DeleteObject(f as _);
            }
        }

        // 预计算右下一行两句的宽度
        let mut bot_widths = [0i32; 2];
        for i in 0..2 {
            let idx = i + 2;
            if idx < lines.len() && !lines[idx].0.is_empty() {
                let f = create_font(fs, false);
                let clean = clean_text(&lines[idx].0);
                let wide: Vec<u16> = clean.encode_utf16().chain(std::iter::once(0)).collect();
                let old_f = SelectObject(mem_dc, f as _);
                let mut calc_r = RECT { left: 0, top: 0, right: 0, bottom: 0 };
                DrawTextW(mem_dc, wide.as_ptr(), -1, &mut calc_r,
                    DT_LEFT | DT_SINGLELINE | DT_NOPREFIX | DT_CALCRECT);
                bot_widths[i] = calc_r.right - calc_r.left;
                SelectObject(mem_dc, old_f);
                DeleteObject(f as _);
            }
        }

        // 左上一行：两句并排
        let top_xs = [
            left_margin,
            left_margin + top_widths[0] + gap,
        ];

        // 右下一行：优先保持右半区布局；放不下时整体左移，只有超出整行可用宽度才省略。
        let bot_total_width = bot_widths[0] + gap + bot_widths[1];
        let bot_max_x = w - left_margin;
        let bot_start_x = if bot_total_width > 0 && (w / 2 + left_margin) + bot_total_width > bot_max_x {
            (bot_max_x - bot_total_width).max(left_margin)
        } else {
            w / 2 + left_margin
        };
        let bot_xs = [
            bot_start_x,
            bot_start_x + bot_widths[0] + gap,
        ];

        for (i, (text, is_current)) in lines.iter().enumerate() {
            if text.is_empty() { continue; }

            let (x, y) = match i {
                0 => (top_xs[0], top_y + y_offset),
                1 => (top_xs[1], top_y + y_offset),
                2 => (bot_xs[0], bottom_y + y_offset),
                3 => (bot_xs[1], bottom_y + y_offset),
                _ => continue,
            };
            let max_right = match i {
                0 => top_xs[1] - 2,
                1 => w - left_margin,
                2 => (bot_xs[1] - 2).min(w - left_margin),
                3 => w - left_margin,
                _ => w - left_margin,
            };
            let text_width = match i {
                0 | 1 => top_widths[i],
                2 | 3 => bot_widths[i - 2],
                _ => 0,
            };

            if *is_current && line_char_progress > 0.0 && line_char_progress < 1.0 && x + text_width <= max_right {
                let chars: Vec<char> = text.chars().collect();
                let total_chars = chars.len().max(1);
                let nf = create_font(fs, true);
                let measure_font = create_font(fs, false);
                // Pre-calculate widths using normal font
                let mut char_widths = Vec::with_capacity(chars.len());
                for ch in &chars {
                    let ch_str: String = ch.to_string();
                    let wide: Vec<u16> = ch_str.encode_utf16().chain(std::iter::once(0)).collect();
                    let old_f = SelectObject(mem_dc, measure_font as _);
                    let mut calc_r = RECT { left: 0, top: 0, right: 0, bottom: 0 };
                    DrawTextW(mem_dc, wide.as_ptr(), -1, &mut calc_r,
                        DT_LEFT | DT_SINGLELINE | DT_NOPREFIX | DT_CALCRECT);
                    char_widths.push(calc_r.right - calc_r.left);
                    SelectObject(mem_dc, old_f);
                }
                let mut draw_x = x;
                for (j, ch) in chars.iter().enumerate() {
                    let is_highlighted = (j as f64) < (line_char_progress * total_chars as f64);
                    let color = if is_highlighted {
                        fg_bright
                    } else {
                        lerp_color(fg_bright, bg, 0.25)
                    };
                    let color = lerp_color(color, bg, 1.0 - alpha);
                    let old_f = SelectObject(mem_dc, nf as _);
                    SetTextColor(mem_dc, RGB(color.0, color.1, color.2));
                    let ch_str: String = ch.to_string();
                    let wide: Vec<u16> = ch_str.encode_utf16().chain(std::iter::once(0)).collect();
                    let mut r = RECT { left: draw_x, top: y, right: draw_x + 200, bottom: y + text_h };
                    DrawTextW(mem_dc, wide.as_ptr(), -1, &mut r,
                        DT_LEFT | DT_VCENTER | DT_SINGLELINE | DT_NOPREFIX);
                    draw_x += char_widths[j] + char_spacing;
                    SelectObject(mem_dc, old_f);
                }
                DeleteObject(nf as _);
                DeleteObject(measure_font as _);
            } else {
                let color = if *is_current { fg_bright } else { fg_dim };
                let color = lerp_color(color, bg, 1.0 - alpha);
                let f = create_font(fs, *is_current);
                let clean = text.clone();
                let wide: Vec<u16> = clean.encode_utf16().chain(std::iter::once(0)).collect();
                let old_f = SelectObject(mem_dc, f as _);
                SetTextColor(mem_dc, RGB(color.0, color.1, color.2));
                let mut r = RECT { left: x, top: y, right: max_right, bottom: y + text_h };
                DrawTextW(mem_dc, wide.as_ptr(), -1, &mut r,
                    DT_LEFT | DT_VCENTER | DT_SINGLELINE | DT_NOPREFIX | DT_WORD_ELLIPSIS);
                SelectObject(mem_dc, old_f);
                DeleteObject(f as _);
            }
        }
    }

    fn key_to_string(vk: i32) -> String {
        match vk {
            0x25 => "LEFT".into(), 0x27 => "RIGHT".into(),
            0x26 => "UP".into(),   0x28 => "DOWN".into(),
            0x20 => "SPACE".into(), 0x21 => "PAGEUP".into(), 0x22 => "PAGEDOWN".into(),
            0xBD => "-".into(), 0xBB => "=".into(), 0x6D => "-".into(), 0x6B => "=".into(),
            0xDB => "[".into(), 0xDD => "]".into(),
            0xBC => ",".into(), 0xBE => ".".into(),
            0x31 | 0x61 => "1".into(), 0x32 | 0x62 => "2".into(),
            0x33 | 0x63 => "3".into(), 0x34 | 0x64 => "4".into(),
            0x35 | 0x65 => "5".into(), 0x54 => "T".into(),
            _ => String::new(),
        }
    }

    unsafe extern "system" fn wnd_proc(
        hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT
    {
        match msg {
            WM_NCHITTEST => {
                let x = (lparam & 0xFFFF) as i16 as i32;
                let y = ((lparam >> 16) & 0xFFFF) as i16 as i32;
                let mut pt = POINT { x, y };
                winapi::um::winuser::ScreenToClient(hwnd, &mut pt);
                let mut r = std::mem::zeroed::<RECT>();
                GetClientRect(hwnd, &mut r);
                if pt.x >= r.left && pt.x < r.right && pt.y >= r.top && pt.y < r.bottom {
                    HTCLIENT as _
                } else {
                    HTTRANSPARENT as _
                }
            }
            WM_CREATE => 0,
            WM_ERASEBKGND => 1,
            WM_LBUTTONDOWN => {
                let p = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut WindowData;
                if !p.is_null() {
                    let data = &mut *p;
                    data.dragging = true;
                    let mut cursor = std::mem::zeroed::<POINT>();
                    GetCursorPos(&mut cursor);
                    let mut wr = std::mem::zeroed::<RECT>();
                    GetWindowRect(hwnd, &mut wr);
                    data.drag_offset_x = wr.left - cursor.x;
                    data.drag_offset_y = wr.top - cursor.y;
                    SetCapture(hwnd);
                }
                SetFocus(hwnd);
                0
            }
            WM_MOUSEMOVE => {
                let p = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut WindowData;
                if !p.is_null() {
                    let data = &mut *p;
                    if data.dragging {
                        let mut cursor = std::mem::zeroed::<POINT>();
                        GetCursorPos(&mut cursor);
                        let new_x = cursor.x + data.drag_offset_x;
                        let new_y = cursor.y + data.drag_offset_y;
                        data.x = new_x; data.y = new_y;
                        SetWindowPos(hwnd, HWND_TOPMOST, new_x, new_y, 0, 0,
                            SWP_NOSIZE | SWP_NOACTIVATE | SWP_SHOWWINDOW);
                    }
                }
                0
            }
            WM_LBUTTONUP => {
                let p = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut WindowData;
                if !p.is_null() {
                    let data = &mut *p;
                    if data.dragging {
                        data.dragging = false;
                        ReleaseCapture();
                        let _ = data.ev_tx.send(DesktopLyricsEvent::PositionChanged {
                            x: data.x, y: data.y });
                    }
                }
                0
            }
            WM_RBUTTONUP => {
                let p = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut WindowData;
                if !p.is_null() {
                    let data = &mut *p;
                    let new_mode = data.scroll_mode.toggle();
                    data.scroll_mode = new_mode;
                    let _ = data.ev_tx.send(DesktopLyricsEvent::ScrollModeChanged { scroll_mode: new_mode });
                }
                0
            }
            WM_KEYDOWN => {
                let k = key_to_string(wparam as i32);
                if !k.is_empty() {
                    let p = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut WindowData;
                    if !p.is_null() {
                        let _ = (*p).ev_tx.send(DesktopLyricsEvent::KeyPress { key: k });
                    }
                }
                0
            }
            WM_TIMER => {
                if wparam == TIMER_ID {
                    let p = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut WindowData;
                    if !p.is_null() {
                        let data = &mut *p;
                        let mut changed = false;
                        match data.scroll_mode {
                            DesktopLyricsScrollMode::Vertical => {
                                if data.transitioning {
                                    data.scroll_progress += 0.08;
                                    if data.scroll_progress >= 1.0 {
                                        data.scroll_progress = 1.0;
                                        data.transitioning = false;
                                        data.old_prev_text.clear();
                                        data.old_curr_text.clear();
                                        data.old_next_text.clear();
                                    }
                                    changed = true;
                                }
                            }
                            DesktopLyricsScrollMode::Horizontal => {
                                let smooth_factor = 0.12;
                                data.scroll_offset += (data.horizontal_target_offset - data.scroll_offset) * smooth_factor;
                                changed = true;
                            }
                            DesktopLyricsScrollMode::Karaoke => {
                                let current_idx = data.all_lyrics.partition_point(|&(_, t)| t <= data.current_time_sec);
                                let current_idx = if current_idx == 0 { 0 } else { current_idx - 1 };
                                let target_group = karaoke_group_start(&data.all_lyrics, current_idx);
                                if data.karaoke_line_group != target_group && !data.all_lyrics.is_empty() {
                                    data.old_karaoke_line_group = data.karaoke_line_group;
                                    data.karaoke_line_group = target_group;
                                    data.karaoke_transition_progress = 0.0;
                                }
                                if data.karaoke_transition_progress < 1.0 {
                                    data.karaoke_transition_progress = (data.karaoke_transition_progress + 0.08).min(1.0);
                                    if data.karaoke_transition_progress >= 1.0 {
                                        data.old_karaoke_line_group = usize::MAX;
                                    }
                                }
                                changed = true;
                            }
                        }
                        loop {
                            match data.rx.try_recv() {
                                Ok(cmd) => {
                                    if cmd.visible == 0 {
                                        ShowWindow(hwnd, 0);
                                    } else if cmd.visible == 1 {
                                        ShowWindow(hwnd, SW_SHOW);
                                        SetWindowPos(hwnd, HWND_TOPMOST, data.x, data.y, 0, 0,
                                            SWP_NOSIZE | SWP_NOACTIVATE | SWP_SHOWWINDOW);
                                    }
                                    if cmd.alpha != 255 && cmd.alpha != data.alpha {
                                        data.alpha = cmd.alpha.clamp(0, 100);
                                        changed = true;
                                    }
                                    if !cmd.theme_name.is_empty() && cmd.theme_name != data.theme_name {
                                        data.theme_name = cmd.theme_name;
                                        changed = true;
                                    }
                                    if cmd.x >= 0 && cmd.y >= 0 && (cmd.x != data.x || cmd.y != data.y) {
                                        data.x = cmd.x; data.y = cmd.y;
                                        SetWindowPos(hwnd, HWND_TOPMOST, cmd.x, cmd.y, 0, 0,
                                            SWP_NOSIZE | SWP_NOACTIVATE | SWP_SHOWWINDOW);
                                    }
                                    if cmd.position != data.position {
                                        data.position = cmd.position;
                                        reposition_window(hwnd, data.position, &mut data.x, &mut data.y);
                                        changed = true;
                                    }
                                    if cmd.scroll_mode != data.scroll_mode {
                                        data.scroll_mode = cmd.scroll_mode;
                                        changed = true;
                                    }
                                     if !cmd.lyrics_text.is_empty() || !cmd.all_lyrics_json.is_empty() {
                                         if !cmd.all_lyrics_json.is_empty() {
                                             // 新格式：所有歌词的 JSON
                                             if let Ok(new_lyrics) = serde_json::from_str::<Vec<(String, f64)>>(&cmd.all_lyrics_json) {
                                                 if new_lyrics != data.all_lyrics || (cmd.current_time_sec - data.current_time_sec).abs() > 0.1 {
                                                     data.old_all_lyrics = data.all_lyrics.clone();
                                                     data.old_scroll_offset = data.scroll_offset;
                                                     data.all_lyrics = new_lyrics;
                                                     data.current_time_sec = cmd.current_time_sec;
                                                     data.horizontal_progress = 0.0;
                                                     data.transitioning = true;
                                                     changed = true;
                                                 }
                                             }
                                         } else {
                                             // 旧格式：三句歌词
                                             let parts: Vec<&str> = cmd.lyrics_text.splitn(3, '\n').collect();
                                             let np = clean_text(parts.first().copied().unwrap_or(""));
                                             let nc = clean_text(parts.get(1).copied().unwrap_or(""));
                                             let nn = clean_text(parts.get(2).copied().unwrap_or(""));
                                             if np != data.prev_text || nc != data.curr_text || nn != data.next_text {
                                                 data.old_prev_text = data.prev_text.clone();
                                                 data.old_curr_text = data.curr_text.clone();
                                                 data.old_next_text = data.next_text.clone();
                                                 data.prev_text = np;
                                                 data.curr_text = nc;
                                                 data.next_text = nn;
                                                 data.scroll_progress = 0.0;
                                                 data.horizontal_progress = 0.0;
                                                 data.transitioning = true;
                                                 changed = true;
                                             }
                                         }
                                     }
                                }
                                Err(mpsc::TryRecvError::Empty) => break,
                                Err(mpsc::TryRecvError::Disconnected) => { PostQuitMessage(0); return 0; }
                            }
                        }
                        if changed { render(hwnd, data); }
                    }
                }
                0
            }
            WM_SIZE => {
                let p = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut WindowData;
                if !p.is_null() { render(hwnd, &mut *p); }
                0
            }
            WM_DESTROY => {
                KillTimer(hwnd, TIMER_ID);
                let p = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut WindowData;
                if !p.is_null() { drop(Box::from_raw(p)); }
                PostQuitMessage(0); 0
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }

    unsafe fn reposition_window(hwnd: HWND, position: DesktopLyricsPosition,
                                 x: &mut i32, y: &mut i32) {
        let work = get_work_area();
        let mut r = std::mem::zeroed::<RECT>();
        GetClientRect(hwnd, &mut r);
        let ww = r.right - r.left;
        let wh = r.bottom - r.top;
        if ww <= 0 || wh <= 0 { return; }
        *x = ((work.right - work.left) - ww) / 2 + work.left;
        *y = match position {
            DesktopLyricsPosition::Bottom => work.bottom - wh,
            DesktopLyricsPosition::Top => work.top,
        };
        SetWindowPos(hwnd, HWND_TOPMOST, *x, *y, 0, 0,
            SWP_NOSIZE | SWP_NOACTIVATE | SWP_SHOWWINDOW);
    }

    pub fn run_desktop_lyrics_window(
        rx: mpsc::Receiver<DesktopLyricsCommand>,
        position: DesktopLyricsPosition,
        theme_name: &str,
        alpha: u8,
        x: i32, y: i32,
        ev_tx: mpsc::Sender<DesktopLyricsEvent>,
        scroll_mode: DesktopLyricsScrollMode,
    ) {
        unsafe {
            let hi = GetModuleHandleW(std::ptr::null());
            let cn: Vec<u16> = "TerMusicDesktopLyrics\0".encode_utf16().collect();
            let wc = WNDCLASSW {
                style: CS_HREDRAW | CS_VREDRAW, lpfnWndProc: Some(wnd_proc),
                cbClsExtra: 0, cbWndExtra: 0, hInstance: hi,
                hIcon: std::ptr::null_mut(), hCursor: LoadCursorW(std::ptr::null_mut(), IDC_ARROW),
                hbrBackground: std::ptr::null_mut(), lpszMenuName: std::ptr::null(),
                lpszClassName: cn.as_ptr(),
            };
            RegisterClassW(&wc);

            let work = get_work_area();
            let sw = work.right - work.left;
            let ww = ((sw as f32 * 0.58) as i32).min(1115).max(500);
            let wh: i32 = 100;

            let (ix, iy) = if x >= 0 && y >= 0 { (x, y) } else {
                let cx = (sw - ww) / 2 + work.left;
                let cy = match position {
                    DesktopLyricsPosition::Bottom => work.bottom - wh,
                    DesktopLyricsPosition::Top => work.top,
                };
                (cx, cy)
            };

            let title: Vec<u16> = "Ter Music Lyrics\0".encode_utf16().collect();
            let hwnd = CreateWindowExW(
                WS_EX_LAYERED | WS_EX_TOPMOST | WS_EX_TOOLWINDOW,
                cn.as_ptr(), title.as_ptr(), WS_POPUP | WS_VISIBLE,
                ix, iy, ww, wh, std::ptr::null_mut(), std::ptr::null_mut(), hi, std::ptr::null_mut(),
            );
            if hwnd.is_null() { return; }

            let data = Box::new(WindowData {
                all_lyrics: Vec::new(),
                current_time_sec: 0.0,
                scroll_offset: 0.0,
                old_all_lyrics: Vec::new(),
                old_scroll_offset: 0.0,
                prev_text: String::new(), curr_text: String::new(), next_text: String::new(),
                old_prev_text: String::new(), old_curr_text: String::new(), old_next_text: String::new(),
                scroll_progress: 1.0, transitioning: false,
                position, theme_name: theme_name.to_string(), alpha: alpha.clamp(0, 100),
                x: ix, y: iy,
                dragging: false, drag_offset_x: 0, drag_offset_y: 0,
                rx, ev_tx,
                scroll_mode,
                horizontal_progress: 0.0,
                karaoke_line_group: usize::MAX,
                old_karaoke_line_group: usize::MAX,
                karaoke_transition_progress: 1.0,
                horizontal_target_offset: 0.0,
            });
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, Box::into_raw(data) as _);
            SetTimer(hwnd, TIMER_ID, TIMER_INTERVAL_MS, None);
            ShowWindow(hwnd, SW_SHOW); UpdateWindow(hwnd);

            let mut msg = std::mem::zeroed::<MSG>();
            loop {
                if GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) <= 0 { break; }
                TranslateMessage(&msg); DispatchMessageW(&msg);
            }
        }
    }
}

#[cfg(not(windows))]
pub mod unix_impl {
    use std::num::NonZeroU32;
    use std::sync::mpsc;
    use std::time::Instant;

    use chrono::Local;
    use fontdue::Font;
    use softbuffer::Surface;
    use winit::application::ApplicationHandler;
    use winit::dpi::{LogicalPosition, LogicalSize};
    use winit::event::{ElementState, KeyEvent, WindowEvent};
    use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy};
    use winit::keyboard::{Key, NamedKey};
    use winit::window::{Window, WindowAttributes, WindowId, WindowLevel};

    use super::{DesktopLyricsCommand, DesktopLyricsEvent, DesktopLyricsPosition, DesktopLyricsScrollMode};

    const TIMER_INTERVAL_MS: u64 = 16;
    const WINDOW_HEIGHT: u32 = 100;
    const SCROLL_ANIMATION_STEP: f32 = 0.08;

    fn theme_colors(name: &str) -> ((u8, u8, u8), (u8, u8, u8), (u8, u8, u8)) {
        match name {
            "GrayWhite" => ((30, 30, 30), (224, 233, 246), (188, 194, 202)),
            "Neon" => ((0, 20, 40), (255, 235, 80), (0, 255, 120)),
            "Sunset" => ((40, 15, 0), (255, 246, 120), (255, 197, 120)),
            "Ocean" => ((0, 20, 40), (168, 255, 245), (116, 243, 204)),
            _ => ((30, 30, 30), (224, 233, 246), (188, 194, 202)),
        }
    }

    fn clean_text(s: &str) -> String {
        s.trim_end_matches(|c: char| c.is_control() || c == '\u{3000}').to_string()
    }

    fn karaoke_group_start(all_lyrics: &[(String, f64)], current_idx: usize) -> usize {
        let visible_count = all_lyrics
            .iter()
            .take(current_idx.saturating_add(1))
            .filter(|(text, _)| !clean_text(text).is_empty())
            .count();
        if visible_count == 0 {
            return 0;
        }

        let target_visible_idx = ((visible_count - 1) / 4) * 4;
        let mut visible_idx = 0usize;
        for (idx, (text, _)) in all_lyrics.iter().enumerate() {
            if clean_text(text).is_empty() {
                continue;
            }
            if visible_idx == target_visible_idx {
                return idx;
            }
            visible_idx += 1;
        }

        0
    }

    fn karaoke_lines_for_group(all_lyrics: &[(String, f64)], group_start: usize, current_idx: usize) -> (Vec<(String, bool)>, bool) {
        let mut lines = Vec::new();
        let mut current_visible = false;
        let mut idx = group_start;
        while lines.len() < 4 && idx < all_lyrics.len() {
            let cleaned = clean_text(&all_lyrics[idx].0);
            if !cleaned.is_empty() {
                let is_current = idx == current_idx;
                current_visible |= is_current;
                lines.push((cleaned, is_current));
            }
            idx += 1;
        }
        (lines, current_visible)
    }

    fn lerp_color(a: (u8, u8, u8), b: (u8, u8, u8), t: f32) -> (u8, u8, u8) {
        let t = t.clamp(0.0, 1.0);
        ((a.0 as f32 + (b.0 as f32 - a.0 as f32) * t) as u8,
         (a.1 as f32 + (b.1 as f32 - a.1 as f32) * t) as u8,
         (a.2 as f32 + (b.2 as f32 - a.2 as f32) * t) as u8)
    }

    fn key_to_string(key: &Key) -> String {
        match key {
            Key::Named(NamedKey::ArrowLeft) => "LEFT".into(),
            Key::Named(NamedKey::ArrowRight) => "RIGHT".into(),
            Key::Named(NamedKey::ArrowUp) => "UP".into(),
            Key::Named(NamedKey::ArrowDown) => "DOWN".into(),
            Key::Named(NamedKey::Space) => "SPACE".into(),
            Key::Named(NamedKey::PageUp) => "PAGEUP".into(),
            Key::Named(NamedKey::PageDown) => "PAGEDOWN".into(),
            Key::Character(c) => {
                let s = c.as_str();
                match s {
                    "-" | "=" | "[" | "]" | "," | "." | "1" | "2" | "3" | "4" | "5" => s.to_string(),
                    "t" | "T" => "T".into(),
                    _ => String::new(),
                }
            }
            _ => String::new(),
        }
    }

    struct GlyphQuad {
        x: f32, y: f32,
        width: usize, height: usize,
        pixels: Vec<f32>,
    }

    fn rasterize_font_text(font: &Font, _latin_font: &Font, text: &str, scale: f32) -> Vec<GlyphQuad> {
        let mut quads = Vec::new();
        let mut pen_x = 0.0f32;
        for ch in text.chars() {
            let (metrics, bitmap) = font.rasterize(ch, scale);
            let w = metrics.width;
            let h = metrics.height;
            let x = pen_x + metrics.xmin as f32;
            let y = metrics.ymin as f32;
            pen_x += metrics.advance_width;
            if w > 0 && h > 0 {
                let pixels: Vec<f32> = bitmap.iter().map(|&c| c as f32 / 255.0).collect();
                quads.push(GlyphQuad { x, y, width: w, height: h, pixels });
            }
        }
        quads
    }

    fn glyph_text_width(quads: &[GlyphQuad]) -> f32 {
        quads.last().map(|q| q.x + q.width as f32).unwrap_or(0.0)
    }

    fn glyph_centered_baseline(quads: &[GlyphQuad], center_y: f32) -> f32 {
        let min_y = quads.iter().map(|q| q.y).fold(0.0f32, f32::min);
        let max_y = quads.iter().map(|q| q.y + q.height as f32).fold(0.0f32, f32::max);
        center_y - (min_y + max_y) * 0.5
    }

    fn fill_buffer(buf: &mut [u32], bg: (u8, u8, u8), bg_alpha: u8) {
        let bgc = ((bg.0 as u32) << 16) | ((bg.1 as u32) << 8) | (bg.2 as u32) | ((bg_alpha as u32) << 24);
        for p in buf.iter_mut() { *p = bgc; }
    }

    fn render_to_buffer(buf: &mut [u32], w: u32, h: u32, state: &mut RenderState) {
        let (bg, fg_bright, fg_dim) = theme_colors(&state.theme_name);
        let bg_a = ((state.alpha as u32 * 255 + 50) / 100).min(255) as u8;
        let txt_pct = (state.alpha as u32 + 10).min(100);
        let txt_a = ((txt_pct * 255 + 50) / 100).min(255) as u8;

        fill_buffer(buf, bg, bg_a);

        match state.scroll_mode {
            DesktopLyricsScrollMode::Vertical => {
                render_to_buffer_vertical(buf, w, h, state, bg, fg_bright, fg_dim, txt_a);
            }
            DesktopLyricsScrollMode::Horizontal => {
                render_to_buffer_horizontal(buf, w, h, state, bg, fg_bright, fg_dim, txt_a);
            }
            DesktopLyricsScrollMode::Karaoke => {
                render_to_buffer_karaoke(buf, w, h, state, bg, fg_bright, fg_dim, txt_a);
            }
        }
    }

    fn render_to_buffer_vertical(buf: &mut [u32], w: u32, h: u32, state: &RenderState,
                                   bg: (u8,u8,u8), fg_bright: (u8,u8,u8), fg_dim: (u8,u8,u8), txt_a: u8) {
        let wh = h as f32;
        let main_fs = wh * 0.26;
        let sub_fs = main_fs * 0.70;

        let main_lh = main_fs + 8.0;
        let sub_lh = sub_fs + 6.0;
        let gap = 4.0;
        let spacing = (sub_lh + main_lh) * 0.5 + gap;
        let y_center_curr = wh * 0.5;
        let y_center_prev = y_center_curr - spacing;
        let y_center_next = y_center_curr + spacing;

        let ww = w as f32;

        let render_line = |buf: &mut [u32], text: &str, font: &Font, latin_font: &Font, fs: f32, center_y: f32, col: (u8,u8,u8), a: u8| {
            let quads = rasterize_font_text(font, latin_font, text, fs);
            let text_w = glyph_text_width(&quads);
            let min_y = quads.iter().map(|q| q.y).fold(0.0f32, f32::min);
            let max_y = quads.iter().map(|q| q.y + q.height as f32).fold(0.0f32, f32::max);
            let baseline = center_y - (min_y + max_y) * 0.5;
            let off_x = (ww - text_w) / 2.0;
            for q in &quads {
                let ox = (off_x + q.x) as i32;
                let oy = (baseline + q.y) as i32;
                for gy in 0..q.height {
                    let py = oy + gy as i32;
                    if py < 0 || py >= h as i32 { continue; }
                    for gx in 0..q.width {
                        let px = ox + gx as i32;
                        if px < 0 || px >= w as i32 { continue; }
                        let cov = q.pixels[gy * q.width + gx];
                        if cov <= 0.001 { continue; }
                        let cov = cov.powf(0.75).min(1.0);
                        let blend = cov;
                        let rr = (bg.0 as f32 + (col.0 as f32 - bg.0 as f32) * blend) as u32;
                        let gg = (bg.1 as f32 + (col.1 as f32 - bg.1 as f32) * blend) as u32;
                        let bb = (bg.2 as f32 + (col.2 as f32 - bg.2 as f32) * blend) as u32;
                        let ca = a as u32;
                        if ca == 0 { continue; }
                        let idx = (py as u32 * w + px as u32) as usize;
                        buf[idx] = (rr << 16) | (gg << 8) | bb | (ca << 24);
                    }
                }
            }
        };

        let empty = state.prev_text.is_empty() && state.curr_text.is_empty() && state.next_text.is_empty()
            && state.old_prev_text.is_empty() && state.old_curr_text.is_empty() && state.old_next_text.is_empty();

        if empty {
            render_line(buf, "♫ ...", &state.font_bold, &state.font_latin_bold, main_fs, y_center_curr, fg_bright, txt_a);
        } else if state.transitioning {
            let t = state.scroll_progress;
            let te = t * t * (3.0 - 2.0 * t);
            let off = te * spacing;
            let a_out = 1.0 - te;
            let a_in = te;

            if a_out > 0.01 {
                let ob = lerp_color(fg_bright, bg, 1.0 - a_out);
                let od = lerp_color(fg_dim, bg, 1.0 - a_out);
                render_line(buf, &state.old_prev_text, &state.font, &state.font_latin, sub_fs, y_center_prev - off, od, txt_a);
                render_line(buf, &state.old_curr_text, &state.font_bold, &state.font_latin_bold, main_fs, y_center_curr - off, ob, txt_a);
                render_line(buf, &state.old_next_text, &state.font, &state.font_latin, sub_fs, y_center_next - off, od, txt_a);
            }
            if a_in > 0.01 {
                let nb = lerp_color(fg_bright, bg, 1.0 - a_in);
                let nd = lerp_color(fg_dim, bg, 1.0 - a_in);
                render_line(buf, &state.prev_text, &state.font, &state.font_latin, sub_fs, y_center_prev + spacing - off, nd, txt_a);
                render_line(buf, &state.curr_text, &state.font_bold, &state.font_latin_bold, main_fs, y_center_curr + spacing - off, nb, txt_a);
                render_line(buf, &state.next_text, &state.font, &state.font_latin, sub_fs, y_center_next + spacing - off, nd, txt_a);
            }
        } else {
            render_line(buf, &state.prev_text, &state.font, &state.font_latin, sub_fs, y_center_prev, fg_dim, txt_a);
            render_line(buf, &state.curr_text, &state.font_bold, &state.font_latin_bold, main_fs, y_center_curr, fg_bright, txt_a);
            render_line(buf, &state.next_text, &state.font, &state.font_latin, sub_fs, y_center_next, fg_dim, txt_a);
        }
    }

    fn render_to_buffer_horizontal(buf: &mut [u32], w: u32, h: u32, state: &mut RenderState,
                                      bg: (u8,u8,u8), fg_bright: (u8,u8,u8), fg_dim: (u8,u8,u8), txt_a: u8) {
        let fs = h as f32 * 0.26;

        if !state.all_lyrics.is_empty() {
            let current_idx = state.all_lyrics.partition_point(|&(_, t)| t <= state.current_time_sec);
            let current_idx = if current_idx == 0 { 0 } else { current_idx - 1 };

            let gap = 40.0;
            let mut widths = vec![0.0; state.all_lyrics.len()];
            for (i, (text, _)) in state.all_lyrics.iter().enumerate() {
                if text.is_empty() { continue; }
                let font_to_use = if i == current_idx { &state.font_bold } else { &state.font };
                let latin_font_to_use = if i == current_idx { &state.font_latin_bold } else { &state.font_latin };
                let clean = clean_text(text);
                let quads = rasterize_font_text(font_to_use, latin_font_to_use, &clean, fs);
                widths[i] = glyph_text_width(&quads);
            }

            let mut positions = vec![0.0f32; state.all_lyrics.len()];
            let mut current_x = w as f32;
            for i in 0..state.all_lyrics.len() {
                positions[i] = current_x;
                current_x += widths[i] + gap;
            }

            let current_line_center = if current_idx < positions.len() {
                positions[current_idx] + widths[current_idx] * 0.5
            } else {
                w as f32 * 0.5
            };
            let target_offset = (current_line_center - w as f32 * 0.50).max(0.0);
            state.horizontal_target_offset = target_offset;

            let final_offset = state.scroll_offset;

            let center_y = h as f32 * 0.5;

            for (i, (text, _)) in state.all_lyrics.iter().enumerate() {
                if text.is_empty() { continue; }
                let is_current = i == current_idx;
                let color = if is_current { fg_bright } else { fg_dim };
                let font_to_use = if is_current { &state.font_bold } else { &state.font };
                let latin_font_to_use = if is_current { &state.font_latin_bold } else { &state.font_latin };
                let clean = clean_text(text);
                let quads = rasterize_font_text(font_to_use, latin_font_to_use, &clean, fs);
                let x = positions[i] - final_offset;
                let baseline = glyph_centered_baseline(&quads, center_y);
                for q in &quads {
                    let ox = (x + q.x) as i32;
                    let oy = (baseline + q.y) as i32;
                    for gy in 0..q.height {
                        let py = oy + gy as i32;
                        if py < 0 || py >= h as i32 { continue; }
                        for gx in 0..q.width {
                            let px = ox + gx as i32;
                            if px < 0 || px >= w as i32 { continue; }
                            let cov = q.pixels[gy * q.width + gx];
                            if cov <= 0.001 { continue; }
                            let cov = cov.powf(0.75).min(1.0);
                            let blend = cov;
                            let rr = (bg.0 as f32 + (color.0 as f32 - bg.0 as f32) * blend) as u32;
                            let gg = (bg.1 as f32 + (color.1 as f32 - bg.1 as f32) * blend) as u32;
                            let bb = (bg.2 as f32 + (color.2 as f32 - bg.2 as f32) * blend) as u32;
                            let ca = (txt_a as f32 * blend) as u32;
                            if ca == 0 { continue; }
                            let idx = (py as u32 * w + px as u32) as usize;
                            buf[idx] = (rr << 16) | (gg << 8) | bb | (ca << 24);
                        }
                    }
                }
            }
            return;
        }

        let texts = vec![&state.prev_text, &state.curr_text, &state.next_text];
        let is_curr = vec![false, true, false];

        let center_y = h as f32 * 0.5;
        let gap = 40.0;

        let mut widths = vec![];
        let mut total_w = 0.0;
        for text in &texts {
            if text.is_empty() {
                widths.push(0.0);
                continue;
            }
            let quads = rasterize_font_text(&state.font, &state.font_latin, text, fs);
            let text_w = glyph_text_width(&quads);
            widths.push(text_w);
            total_w += text_w;
        }
        total_w += gap * 2.0;

        let start_x = (w as f32 - total_w) * 0.5;
        let mut current_x = start_x;

        for (i, text) in texts.iter().enumerate() {
            if text.is_empty() { continue; }
            let color = if is_curr[i] { fg_bright } else { fg_dim };
            let quads = rasterize_font_text(&state.font, &state.font_latin, text, fs);
            let baseline = glyph_centered_baseline(&quads, center_y);
            for q in &quads {
                let ox = (current_x + q.x) as i32;
                let oy = (baseline + q.y) as i32;
                for gy in 0..q.height {
                    let py = oy + gy as i32;
                    if py < 0 || py >= h as i32 { continue; }
                    for gx in 0..q.width {
                        let px = ox + gx as i32;
                        if px < 0 || px >= w as i32 { continue; }
                        let cov = q.pixels[gy * q.width + gx];
                        if cov <= 0.001 { continue; }
                        let cov = cov.powf(0.75).min(1.0);
                        let blend = cov;
                        let rr = (bg.0 as f32 + (color.0 as f32 - bg.0 as f32) * blend) as u32;
                        let gg = (bg.1 as f32 + (color.1 as f32 - bg.1 as f32) * blend) as u32;
                        let bb = (bg.2 as f32 + (color.2 as f32 - bg.2 as f32) * blend) as u32;
                        let ca = (txt_a as f32 * blend) as u32;
                        if ca == 0 { continue; }
                        let idx = (py as u32 * w + px as u32) as usize;
                        buf[idx] = (rr << 16) | (gg << 8) | bb | (ca << 24);
                    }
                }
            }
            current_x += widths[i] + gap;
        }
    }

    fn render_to_buffer_karaoke(buf: &mut [u32], w: u32, h: u32, state: &RenderState,
                                bg: (u8,u8,u8), fg_bright: (u8,u8,u8), fg_dim: (u8,u8,u8), txt_a: u8) {
        let fs = h as f32 * 0.26;

        let lines: Vec<(String, bool)>;
        let mut old_lines: Vec<(String, bool)> = Vec::new();
        let mut line_char_progress: f64 = 0.0;

        if !state.all_lyrics.is_empty() {
            let current_idx = state.all_lyrics.partition_point(|&(_, t)| t <= state.current_time_sec);
            let current_idx = if current_idx == 0 { 0 } else { current_idx - 1 };

            let group_start = if state.karaoke_line_group == usize::MAX {
                karaoke_group_start(&state.all_lyrics, current_idx)
            } else {
                state.karaoke_line_group
            };

            let current_visible;
            (lines, current_visible) = karaoke_lines_for_group(&state.all_lyrics, group_start, current_idx);
            if state.karaoke_transition_progress < 1.0 && state.old_karaoke_line_group != usize::MAX && state.old_karaoke_line_group != group_start {
                old_lines = karaoke_lines_for_group(&state.all_lyrics, state.old_karaoke_line_group, current_idx).0;
            }

            if current_visible {
                let current_line_time = state.all_lyrics[current_idx].1;
                let next_line_time = state.all_lyrics.get(current_idx + 1).map(|&(_, t)| t).unwrap_or(current_line_time + 4.0);
                let duration = (next_line_time - current_line_time).max(0.1);
                line_char_progress = ((state.current_time_sec - current_line_time) / duration).clamp(0.0, 1.0);
            }
        } else {
            lines = vec![
                (clean_text(&state.prev_text), false),
                (clean_text(&state.curr_text), true),
                (clean_text(&state.next_text), false),
                (String::new(), false),
            ];
        }

        if !old_lines.is_empty() {
            let t = state.karaoke_transition_progress.clamp(0.0, 1.0);
            let eased = t * t * (3.0 - 2.0 * t);
            let slide = h as f32 * 0.56;
            render_to_buffer_karaoke_group(buf, w, h, state, &old_lines, fs, bg, fg_bright, fg_dim, txt_a, 1.0, -slide * eased, 0.0);
            render_to_buffer_karaoke_group(buf, w, h, state, &lines, fs, bg, fg_bright, fg_dim, txt_a, 1.0, slide * (1.0 - eased), line_char_progress);
        } else {
            render_to_buffer_karaoke_group(buf, w, h, state, &lines, fs, bg, fg_bright, fg_dim, txt_a, 1.0, 0.0, line_char_progress);
        }
    }

    fn render_to_buffer_karaoke_group(buf: &mut [u32], w: u32, h: u32, state: &RenderState,
                                      lines: &[(String, bool)], fs: f32, bg: (u8,u8,u8), fg_bright: (u8,u8,u8),
                                      fg_dim: (u8,u8,u8), txt_a: u8, alpha: f32, y_offset: f32,
                                      line_char_progress: f64) {
        if alpha <= 0.01 {
            return;
        }

        let _text_h = fs + 6.0;
        let char_spacing = 0.0; // 字间距
        let gap = w as f32 * 0.01; // 句间间隔，逗号大小
        let top_y = h as f32 * 0.18;
        let bottom_y = h as f32 * 0.55;
        let left_margin = w as f32 * 0.06;

        // 预计算左上一行两句的宽度
        let mut top_widths = [0.0f32; 2];
        for i in 0..2 {
            if i < lines.len() && !lines[i].0.is_empty() {
                let quads = rasterize_font_text(&state.font, &state.font_latin, &lines[i].0, fs);
                top_widths[i] = quads.last().map(|q| q.x + q.width as f32).unwrap_or(0.0);
            }
        }

        // 预计算右下一行两句的宽度
        let mut bot_widths = [0.0f32; 2];
        for i in 0..2 {
            let idx = i + 2;
            if idx < lines.len() && !lines[idx].0.is_empty() {
                let quads = rasterize_font_text(&state.font, &state.font_latin, &lines[idx].0, fs);
                bot_widths[i] = quads.last().map(|q| q.x + q.width as f32).unwrap_or(0.0);
            }
        }

        // 左上一行：两句并排的x坐标
        let top_xs = [
            left_margin,
            left_margin + top_widths[0] + gap,
        ];

        // 右下一行：优先保持右半区布局；放不下时整体左移，只有超出整行可用宽度才裁剪。
        let bot_total_width = bot_widths[0] + gap + bot_widths[1];
        let bot_max_x = w as f32 - left_margin;
        let preferred_bot_start_x = w as f32 / 2.0 + left_margin;
        let bot_start_x = if bot_total_width > 0.0 && preferred_bot_start_x + bot_total_width > bot_max_x {
            (bot_max_x - bot_total_width).max(left_margin)
        } else {
            preferred_bot_start_x
        };
        let bot_xs = [
            bot_start_x,
            bot_start_x + bot_widths[0] + gap,
        ];

        for (i, (text, is_current)) in lines.iter().enumerate() {
            if text.is_empty() { continue; }

            let (x, y) = match i {
                0 => (top_xs[0], top_y + y_offset),
                1 => (top_xs[1], top_y + y_offset),
                2 => (bot_xs[0], bottom_y + y_offset),
                3 => (bot_xs[1], bottom_y + y_offset),
                _ => continue,
            };

            // 逐字符渲染，带字间距，使用颜色高亮而不改变字体
            let chars: Vec<char> = text.chars().collect();
            let total_chars = chars.len().max(1);
            // Pre-calculate character widths using regular font
            let mut char_widths = Vec::with_capacity(chars.len());
            for ch in &chars {
                let quads = rasterize_font_text(&state.font, &state.font_latin, &ch.to_string(), fs);
                let width = quads.last().map(|q| q.x + q.width as f32).unwrap_or(0.0);
                char_widths.push(width);
            }
            let mut draw_x = x;
            let (reg_font, reg_latin) = (&state.font, &state.font_latin);
            for (j, ch) in chars.iter().enumerate() {
                let is_highlighted = *is_current
                    && line_char_progress > 0.0
                    && line_char_progress < 1.0
                    && (j as f64) < (line_char_progress * total_chars as f64);
                let color = if *is_current {
                    if is_highlighted { fg_bright } else { lerp_color(fg_bright, bg, 0.25) }
                } else {
                    fg_dim
                };
                let quads = rasterize_font_text(reg_font, reg_latin, &ch.to_string(), fs);
                for q in &quads {
                    let ox = (draw_x + q.x) as i32;
                    let oy = (y + q.y) as i32;
                    for gy in 0..q.height {
                        let py = oy + gy as i32;
                        if py < 0 || py >= h as i32 { continue; }
                        for gx in 0..q.width {
                            let px = ox + gx as i32;
                            if px < 0 || px >= w as i32 { continue; }
                            let cov = q.pixels[gy * q.width + gx];
                            if cov <= 0.001 { continue; }
                            let cov = cov.powf(0.75).min(1.0);
                            let blend = cov;
                            let rr = (bg.0 as f32 + (color.0 as f32 - bg.0 as f32) * blend) as u32;
                            let gg = (bg.1 as f32 + (color.1 as f32 - bg.1 as f32) * blend) as u32;
                            let bb = (bg.2 as f32 + (color.2 as f32 - bg.2 as f32) * blend) as u32;
                            let ca = (txt_a as f32 * alpha * blend) as u32;
                            if ca == 0 { continue; }
                            let idx = (py as u32 * w + px as u32) as usize;
                            buf[idx] = (rr << 16) | (gg << 8) | bb | (ca << 24);
                        }
                    }
                }
                draw_x += char_widths[j] + char_spacing;
            }
        }
    }

    struct RenderState {
        all_lyrics: Vec<(String, f64)>,
        current_time_sec: f64,
        scroll_offset: f32,
        old_all_lyrics: Vec<(String, f64)>,
        old_scroll_offset: f32,
        old_prev_text: String,
        old_curr_text: String,
        old_next_text: String,
        prev_text: String,
        curr_text: String,
        next_text: String,
        scroll_progress: f32,
        transitioning: bool,
        theme_name: String,
        alpha: u8,
        font: Font,
        font_bold: Font,
        font_latin: Font,
        font_latin_bold: Font,
        scroll_mode: DesktopLyricsScrollMode,
        horizontal_progress: f32,
        horizontal_target_offset: f32,
        karaoke_line_group: usize,
        old_karaoke_line_group: usize,
        karaoke_transition_progress: f32,
    }

    struct AppState {
        rx: mpsc::Receiver<DesktopLyricsCommand>,
        ev_tx: mpsc::Sender<DesktopLyricsEvent>,
        proxy: EventLoopProxy<UserEvent>,
        position: DesktopLyricsPosition,
        theme_name: String,
        alpha: u8,
        x: i32,
        y: i32,
        window: Option<std::rc::Rc<Window>>,
        surface: Option<Surface<std::rc::Rc<Window>, std::rc::Rc<Window>>>,
        font_regular_bytes: Vec<u8>,
        font_bold_bytes: Vec<u8>,
        font_latin_regular_bytes: Vec<u8>,
        font_latin_bold_bytes: Vec<u8>,
        render_state: RenderState,
        dragging: bool,
        drag_offset_x: i32,
        drag_offset_y: i32,
        last_render: Instant,
        buffer: Vec<u32>,
        cursor_x: f64,
        cursor_y: f64,
    }

    impl AppState {
        fn make_font(bytes: &[u8], scale: f32) -> Option<Font> {
            fontdue::Font::from_bytes(bytes, fontdue::FontSettings { scale, ..Default::default() }).ok()
        }

        fn reload_fonts(regular: &[u8], bold: &[u8], latin_regular: &[u8], latin_bold: &[u8], scale: f32) -> Option<(Font, Font, Font, Font)> {
            Some((
                Self::make_font(regular, scale)?,
                Self::make_font(bold, scale)?,
                Self::make_font(latin_regular, scale)?,
                Self::make_font(latin_bold, scale)?,
            ))
        }

        fn request_redraw(&self) {
            if let Some(ref w) = self.window { w.request_redraw(); }
        }

        fn reposition_window(&mut self) {
            if let Some(ref w) = self.window {
                if let Some(monitor) = w.current_monitor() {
                    let sz = w.outer_size();
                    let (work_x, work_y, work_w, work_h) = get_linux_work_area(&monitor);
                    let nx = work_x + (work_w as i32 - sz.width as i32) / 2;
                    let ny = match self.position {
                        DesktopLyricsPosition::Bottom => work_y + work_h as i32 - sz.height as i32,
                        DesktopLyricsPosition::Top => work_y,
                    };
                    self.x = nx; self.y = ny;
                    let _ = w.set_outer_position(LogicalPosition::new(nx, ny));
                }
            }
        }

        fn process_commands(&mut self) -> bool {
            let mut changed = false;
            loop {
                match self.rx.try_recv() {
                    Ok(cmd) => {
                        if cmd.visible == 0 {
                            super::append_desktop_lyrics_log("unix cmd: visible=0 (hide)");
                            if let Some(ref w) = self.window { w.set_visible(false); }
                        } else if cmd.visible == 1 {
                            super::append_desktop_lyrics_log("unix cmd: visible=1 (show)");
                            if let Some(ref w) = self.window {
                                w.set_visible(true);
                                w.request_redraw();
                            } else {
                                super::append_desktop_lyrics_log("unix cmd: visible=1 but window is None");
                            }
                        }
                        if cmd.alpha != 255 && cmd.alpha != self.alpha {
                            self.alpha = cmd.alpha.clamp(0, 100);
                            self.render_state.alpha = self.alpha;
                            changed = true;
                        }
                        if !cmd.theme_name.is_empty() && cmd.theme_name != self.theme_name {
                            self.theme_name = cmd.theme_name.clone();
                            self.render_state.theme_name = cmd.theme_name.clone();
                            changed = true;
                        }
                        if cmd.x >= 0 && cmd.y >= 0 && (cmd.x != self.x || cmd.y != self.y) {
                            self.x = cmd.x; self.y = cmd.y;
                            if let Some(ref w) = self.window {
                                let _ = w.set_outer_position(LogicalPosition::new(self.x, self.y));
                            }
                        }
                        if cmd.position != self.position {
                            self.position = cmd.position;
                            self.reposition_window();
                            changed = true;
                        }
                        if cmd.scroll_mode != self.render_state.scroll_mode {
                            self.render_state.scroll_mode = cmd.scroll_mode;
                            changed = true;
                        }
                        if !cmd.lyrics_text.is_empty() || !cmd.all_lyrics_json.is_empty() {
                            if !cmd.all_lyrics_json.is_empty() {
                                // 新格式：所有歌词的 JSON
                                if let Ok(new_lyrics) = serde_json::from_str::<Vec<(String, f64)>>(&cmd.all_lyrics_json) {
                                    if new_lyrics != self.render_state.all_lyrics || (cmd.current_time_sec - self.render_state.current_time_sec).abs() > 0.1 {
                                        self.render_state.old_all_lyrics = self.render_state.all_lyrics.clone();
                                        self.render_state.old_scroll_offset = self.render_state.scroll_offset;
                                        self.render_state.all_lyrics = new_lyrics;
                                        self.render_state.current_time_sec = cmd.current_time_sec;
                                        self.render_state.horizontal_progress = 0.0;
                                        self.render_state.transitioning = true;
                                        changed = true;
                                    }
                                }
                            } else {
                                // 旧格式：三句歌词
                                let parts: Vec<&str> = cmd.lyrics_text.splitn(3, '\n').collect();
                                let np = clean_text(parts.first().copied().unwrap_or(""));
                                let nc = clean_text(parts.get(1).copied().unwrap_or(""));
                                let nn = clean_text(parts.get(2).copied().unwrap_or(""));
                                if np != self.render_state.prev_text || nc != self.render_state.curr_text || nn != self.render_state.next_text {
                                    self.render_state.old_prev_text = self.render_state.prev_text.clone();
                                    self.render_state.old_curr_text = self.render_state.curr_text.clone();
                                    self.render_state.old_next_text = self.render_state.next_text.clone();
                                    self.render_state.prev_text = np;
                                    self.render_state.curr_text = nc;
                                    self.render_state.next_text = nn;
                                    self.render_state.scroll_progress = 0.0;
                                    self.render_state.horizontal_progress = 0.0;
                                    self.render_state.transitioning = true;
                                    changed = true;
                                }
                            }
                        }
                    }
                    Err(mpsc::TryRecvError::Empty) => break,
                    Err(mpsc::TryRecvError::Disconnected) => {
                        super::append_desktop_lyrics_log("unix cmd channel disconnected, request app exit");
                        if let Some(ref w) = self.window {
                            w.set_visible(false);
                        }
                        return false;
                    }
                }
            }
            changed
        }
    }

    /// 通过 X11 `_NET_WORKAREA` 属性获取当前显示器的工作区矩形（排除任务栏/面板）。
    /// 返回 (x, y, width, height)。如果无法获取（如 Wayland 环境），则回退到 monitor 信息。
    fn get_linux_work_area(monitor: &winit::monitor::MonitorHandle) -> (i32, i32, u32, u32) {
        // 尝试通过 X11 获取 _NET_WORKAREA
        #[cfg(target_os = "linux")]
        {
            if let Some(work) = try_x11_work_area(monitor) {
                return work;
            }
        }
        // 回退：使用完整显示器尺寸（无任务栏偏移）
        let mpos = monitor.position();
        let sz = monitor.size();
        (mpos.x, mpos.y, sz.width, sz.height)
    }

    #[cfg(target_os = "linux")]
    fn try_x11_work_area(monitor: &winit::monitor::MonitorHandle) -> Option<(i32, i32, u32, u32)> {
        use std::ffi::CString;

        // 动态加载 X11 函数，避免硬链接 libX11
        unsafe {
            let lib = libc::dlopen(
                b"libX11.so.6\0".as_ptr() as *const libc::c_char,
                libc::RTLD_NOW | libc::RTLD_GLOBAL,
            );
            if lib.is_null() {
                return None;
            }

            type XOpenDisplay = unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_void;
            type XCloseDisplay = unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int;
            type XDefaultRootWindow = unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong;
            type XInternAtom = unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, libc::c_int) -> libc::c_ulong;
            type XGetWindowProperty = unsafe extern "C" fn(
                *mut libc::c_void, libc::c_ulong, libc::c_ulong, libc::c_long, libc::c_long,
                libc::c_int, libc::c_ulong, *mut libc::c_ulong, *mut libc::c_int,
                *mut libc::c_ulong, *mut libc::c_ulong, *mut *mut libc::c_uchar,
            ) -> libc::c_int;
            type XFree = unsafe extern "C" fn(*mut libc::c_void);

            macro_rules! sym {
                ($lib:expr, $name:literal) => {
                    libc::dlsym($lib, concat!($name, "\0").as_ptr() as *const libc::c_char)
                };
            }

            let x_open_display: XOpenDisplay = std::mem::transmute(sym!(lib, "XOpenDisplay"));
            let x_close_display: XCloseDisplay = std::mem::transmute(sym!(lib, "XCloseDisplay"));
            let x_default_root_window: XDefaultRootWindow = std::mem::transmute(sym!(lib, "XDefaultRootWindow"));
            let x_intern_atom: XInternAtom = std::mem::transmute(sym!(lib, "XInternAtom"));
            let x_get_window_property: XGetWindowProperty = std::mem::transmute(sym!(lib, "XGetWindowProperty"));
            let x_free: XFree = std::mem::transmute(sym!(lib, "XFree"));

            let display = x_open_display(std::ptr::null());
            if display.is_null() {
                libc::dlclose(lib);
                return None;
            }

            let root = x_default_root_window(display);

            let atom_name = CString::new("_NET_WORKAREA").ok()?;
            let atom = x_intern_atom(display, atom_name.as_ptr(), 0);
            if atom == 0 {
                x_close_display(display);
                libc::dlclose(lib);
                return None;
            }

            let mut actual_type: libc::c_ulong = 0;
            let mut actual_format: libc::c_int = 0;
            let mut nitems: libc::c_ulong = 0;
            let mut bytes_after: libc::c_ulong = 0;
            let mut prop: *mut libc::c_uchar = std::ptr::null_mut();

            // XGetWindowProperty returns 0 (Success) on success
            let ret = x_get_window_property(
                display, root, atom,
                0, 1024, // offset, length (enough for many desktops)
                0, // delete = False
                0, // AnyPropertyType
                &mut actual_type, &mut actual_format,
                &mut nitems, &mut bytes_after,
                &mut prop,
            );

            if ret != 0 || actual_format != 32 || nitems < 4 || prop.is_null() {
                if !prop.is_null() { x_free(prop as *mut libc::c_void); }
                x_close_display(display);
                libc::dlclose(lib);
                return None;
            }

            // _NET_WORKAREA returns array of CARDINAL (32-bit): [x, y, w, h] per desktop
            let data = prop as *const libc::c_long;
            let mpos = monitor.position();
            let msz = monitor.size();

            let num_areas = nitems as usize / 4;
            for i in 0..num_areas {
                let wx = *data.add(i * 4) as i32;
                let wy = *data.add(i * 4 + 1) as i32;
                let ww = *data.add(i * 4 + 2) as u32;
                let wh = *data.add(i * 4 + 3) as u32;

                // 检查此工作区是否与当前显示器重叠
                if wx < mpos.x + msz.width as i32
                    && wx + ww as i32 > mpos.x
                    && wy < mpos.y + msz.height as i32
                    && wy + wh as i32 > mpos.y
                {
                    x_free(prop as *mut libc::c_void);
                    x_close_display(display);
                    libc::dlclose(lib);
                    return Some((wx, wy, ww, wh));
                }
            }

            x_free(prop as *mut libc::c_void);
            x_close_display(display);
            libc::dlclose(lib);
            None
        }
    }

    enum UserEvent {
        Timer,
    }

    impl ApplicationHandler<UserEvent> for AppState {
        fn resumed(&mut self, event_loop: &ActiveEventLoop) {
            if self.window.is_some() { return; }

            super::append_desktop_lyrics_log("unix resumed: begin create window");

            let wa = WindowAttributes::default()
                .with_title("Ter Music Lyrics")
                .with_decorations(false)
                .with_transparent(true)
                .with_window_level(WindowLevel::AlwaysOnTop)
                .with_resizable(false)
                .with_inner_size(LogicalSize::new(1115.0, WINDOW_HEIGHT as f64));

            let window = match event_loop.create_window(wa) {
                Ok(w) => w,
                Err(e) => {
                    let msg = format!("unix resumed: create_window failed: {:?}", e);
                    super::append_desktop_lyrics_log(&msg);
                    return;
                }
            };

            let monitor = match window.current_monitor() {
                Some(m) => m,
                None => {
                    super::append_desktop_lyrics_log("unix resumed: current_monitor is None, skip window init");
                    return;
                }
            };
            let (work_x, work_y, work_w, work_h) = get_linux_work_area(&monitor);
            let ww = ((work_w as f32 * 0.58) as u32).min(1115).max(500);

            let _ = window.request_inner_size(LogicalSize::new(ww as f64, WINDOW_HEIGHT as f64));

            let inner = window.inner_size();
            let phys_w = inner.width.max(1);
            let phys_h = inner.height.max(1);

            if self.x >= 0 && self.y >= 0 {
                let _ = window.set_outer_position(LogicalPosition::new(self.x, self.y));
            } else {
                let nx = work_x + (work_w as i32 - ww as i32) / 2;
                let ny = match self.position {
                    DesktopLyricsPosition::Bottom => work_y + work_h as i32 - WINDOW_HEIGHT as i32,
                    DesktopLyricsPosition::Top => work_y,
                };
                self.x = nx; self.y = ny;
                let _ = window.set_outer_position(LogicalPosition::new(nx, ny));
            }

            let rc = std::rc::Rc::new(window);

            let ctx = match softbuffer::Context::new(std::rc::Rc::clone(&rc)) {
                Ok(c) => c,
                Err(e) => {
                    let msg = format!("unix resumed: softbuffer context create failed: {:?}", e);
                    super::append_desktop_lyrics_log(&msg);
                    return;
                }
            };
            let mut surface = match softbuffer::Surface::new(&ctx, std::rc::Rc::clone(&rc)) {
                Ok(s) => s,
                Err(e) => {
                    let msg = format!("unix resumed: softbuffer surface create failed: {:?}", e);
                    super::append_desktop_lyrics_log(&msg);
                    return;
                }
            };

            if let (Some(pw), Some(ph)) = (NonZeroU32::new(phys_w), NonZeroU32::new(phys_h)) {
                if let Err(e) = surface.resize(pw, ph) {
                    let msg = format!("unix resumed: surface.resize failed: {:?}", e);
                    super::append_desktop_lyrics_log(&msg);
                }
            }

            self.buffer = vec![0u32; (phys_w * phys_h) as usize];
            self.window = Some(rc);
            self.surface = Some(surface);

            if let Some(ref w) = self.window {
                w.set_visible(true);
                w.request_redraw();
                let out = w.outer_size();
                let inn = w.inner_size();
                let msg = format!(
                    "unix resumed: window ready; outer=({}x{}), inner=({}x{}), pos=({}, {})",
                    out.width, out.height, inn.width, inn.height, self.x, self.y
                );
                super::append_desktop_lyrics_log(&msg);
            }

            let proxy = self.proxy.clone();
            std::thread::spawn(move || loop {
                std::thread::sleep(std::time::Duration::from_millis(TIMER_INTERVAL_MS));
                if proxy.send_event(UserEvent::Timer).is_err() { break; }
            });
        }

        fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, ev: WindowEvent) {
            match ev {
                WindowEvent::RedrawRequested => {
                    if self.process_commands() { self.request_redraw(); }
                    if let (Some(ref w), Some(ref mut surf)) = (&self.window, &mut self.surface) {
                        let dpi = w.scale_factor();
                        let sz = w.inner_size();
                        let pw = (sz.width as f64 * dpi) as u32;
                        let ph = (sz.height as f64 * dpi) as u32;
                        let ps = (pw * ph) as usize;

                        if ps > 0 {
                            if self.buffer.len() != ps {
                                self.buffer = vec![0u32; ps];
                                if let Some((f, fb, fl, flb)) = Self::reload_fonts(
                                    &self.font_regular_bytes,
                                    &self.font_bold_bytes,
                                    &self.font_latin_regular_bytes,
                                    &self.font_latin_bold_bytes,
                                    sz.height as f32,
                                ) {
                                    self.render_state.font = f;
                                    self.render_state.font_bold = fb;
                                    self.render_state.font_latin = fl;
                                    self.render_state.font_latin_bold = flb;
                                }
                            }

                            render_to_buffer(&mut self.buffer, pw, ph, &mut self.render_state);

                            if let (Some(wpw), Some(wph)) = (NonZeroU32::new(pw), NonZeroU32::new(ph)) {
                                let _ = surf.resize(wpw, wph);
                                let mut dst = surf.buffer_mut().unwrap();
                                let n = dst.len().min(self.buffer.len());
                                dst[..n].copy_from_slice(&self.buffer[..n]);
                                if let Err(e) = dst.present() {
                                    let msg = format!("unix redraw: present failed: {:?}", e);
                                    super::append_desktop_lyrics_log(&msg);
                                }
                            }
                        }
                    }
                    self.last_render = Instant::now();
                }
                WindowEvent::MouseInput { state: ElementState::Pressed, button: winit::event::MouseButton::Left, .. } => {
                    self.dragging = true;
                    if let Some(ref w) = self.window {
                        let _ = w.set_cursor(winit::window::CursorIcon::Grabbing);
                        // 记录拖动起始时窗口位置与鼠标屏幕坐标的偏移
                        // winit 0.30 没有 cursor_position()，使用缓存的 CursorMoved 坐标
                        let outer = w.outer_position().unwrap_or(winit::dpi::PhysicalPosition::new(0, 0));
                        let screen_x = outer.x + self.cursor_x as i32;
                        let screen_y = outer.y + self.cursor_y as i32;
                        self.drag_offset_x = self.x - screen_x;
                        self.drag_offset_y = self.y - screen_y;
                    }
                }
                WindowEvent::MouseInput { state: ElementState::Released, button: winit::event::MouseButton::Left, .. } => {
                    if self.dragging {
                        self.dragging = false;
                        if let Some(ref w) = self.window {
                            let _ = w.set_cursor(winit::window::CursorIcon::Default);
                        }
                        let _ = self.ev_tx.send(DesktopLyricsEvent::PositionChanged { x: self.x, y: self.y });
                    }
                }
                WindowEvent::MouseInput { state: ElementState::Pressed, button: winit::event::MouseButton::Right, .. } => {
                    // 右键切换滚动模式
                    let new_mode = self.render_state.scroll_mode.toggle();
                    self.render_state.scroll_mode = new_mode;
                    let _ = self.ev_tx.send(DesktopLyricsEvent::ScrollModeChanged { scroll_mode: new_mode });
                    self.request_redraw();
                }
                WindowEvent::CursorMoved { position, .. } => {
                    // 缓存光标在窗口内的相对坐标
                    self.cursor_x = position.x;
                    self.cursor_y = position.y;
                    if self.dragging {
                        if let Some(ref w) = self.window {
                            // position 是窗口内相对坐标，需要加上窗口当前位置得到屏幕坐标，
                            // 再加上拖动起始偏移，避免窗口跳动。
                            let outer = w.outer_position().unwrap_or(winit::dpi::PhysicalPosition::new(0, 0));
                            let screen_x = outer.x + position.x as i32;
                            let screen_y = outer.y + position.y as i32;
                            let new_x = screen_x + self.drag_offset_x;
                            let new_y = screen_y + self.drag_offset_y;
                            self.x = new_x; self.y = new_y;
                            let _ = w.set_outer_position(LogicalPosition::new(new_x, new_y));
                        }
                    }
                }
                WindowEvent::KeyboardInput { event: KeyEvent { state: ElementState::Pressed, logical_key, .. }, .. } => {
                    let k = key_to_string(&logical_key);
                    if !k.is_empty() {
                        let _ = self.ev_tx.send(DesktopLyricsEvent::KeyPress { key: k });
                    }
                }
                WindowEvent::CloseRequested => event_loop.exit(),
                _ => {}
            }
        }

        fn user_event(&mut self, _: &ActiveEventLoop, _: UserEvent) {
            let mut dirty = self.process_commands();
            match self.render_state.scroll_mode {
                DesktopLyricsScrollMode::Vertical => {
                    if self.render_state.transitioning {
                        self.render_state.scroll_progress += SCROLL_ANIMATION_STEP;
                        if self.render_state.scroll_progress >= 1.0 {
                            self.render_state.scroll_progress = 1.0;
                            self.render_state.transitioning = false;
                            self.render_state.old_prev_text.clear();
                            self.render_state.old_curr_text.clear();
                            self.render_state.old_next_text.clear();
                        }
                        dirty = true;
                    }
                }
                DesktopLyricsScrollMode::Horizontal => {
                    let smooth_factor = 0.12;
                    self.render_state.scroll_offset += (self.render_state.horizontal_target_offset - self.render_state.scroll_offset) * smooth_factor;
                    dirty = true;
                }
                DesktopLyricsScrollMode::Karaoke => {
                    let current_idx = self.render_state.all_lyrics.partition_point(|&(_, t)| t <= self.render_state.current_time_sec);
                    let current_idx = if current_idx == 0 { 0 } else { current_idx - 1 };
                    let target_group = karaoke_group_start(&self.render_state.all_lyrics, current_idx);
                    if self.render_state.karaoke_line_group != target_group && !self.render_state.all_lyrics.is_empty() {
                        self.render_state.old_karaoke_line_group = self.render_state.karaoke_line_group;
                        self.render_state.karaoke_line_group = target_group;
                        self.render_state.karaoke_transition_progress = 0.0;
                    }
                    if self.render_state.karaoke_transition_progress < 1.0 {
                        self.render_state.karaoke_transition_progress = (self.render_state.karaoke_transition_progress + SCROLL_ANIMATION_STEP).min(1.0);
                        if self.render_state.karaoke_transition_progress >= 1.0 {
                            self.render_state.old_karaoke_line_group = usize::MAX;
                        }
                    }
                    dirty = true;
                }
            }
            if dirty { self.request_redraw(); }
        }
    }

    fn try_read_font(paths: &[&str]) -> Option<Vec<u8>> {
        for p in paths {
            let path = if let Some(rest) = p.strip_prefix("$HOME/") {
                std::env::var_os("HOME").map(|home| std::path::PathBuf::from(home).join(rest))
            } else {
                Some(std::path::PathBuf::from(p))
            };
            if let Some(path) = path {
                if let Ok(data) = std::fs::read(path) {
                    return Some(data);
                }
            }
        }
        None
    }

    fn try_read_fonts(paths: &[&str]) -> Vec<Vec<u8>> {
        let mut fonts = Vec::new();
        for p in paths {
            let path = if let Some(rest) = p.strip_prefix("$HOME/") {
                std::env::var_os("HOME").map(|home| std::path::PathBuf::from(home).join(rest))
            } else {
                Some(std::path::PathBuf::from(p))
            };
            if let Some(path) = path {
                if let Ok(data) = std::fs::read(path) {
                    fonts.push(data);
                }
            }
        }
        fonts
    }

    fn find_first_parseable_font(paths: &[&str]) -> Option<Vec<u8>> {
        for data in try_read_fonts(paths) {
            if Font::from_bytes(data.as_slice(), fontdue::FontSettings::default()).is_ok() {
                return Some(data);
            }
        }
        None
    }

    #[cfg(target_os = "linux")]
    fn read_fontconfig_match(pattern: &str) -> Option<Vec<u8>> {
        let output = std::process::Command::new("fc-match")
            .args(["-f", "%{file}\n", pattern])
            .output()
            .ok()?;
        if !output.status.success() {
            return None;
        }
        let path = String::from_utf8_lossy(&output.stdout).lines().next()?.trim().to_string();
        if path.is_empty() {
            return None;
        }
        let data = std::fs::read(path).ok()?;
        if Font::from_bytes(data.as_slice(), fontdue::FontSettings::default()).is_ok() {
            Some(data)
        } else {
            None
        }
    }

    fn find_system_font_regular() -> Vec<u8> {
        #[cfg(target_os = "macos")]
        let paths = &[
            "/System/Library/Fonts/PingFang.ttc",
            "/System/Library/Fonts/Helvetica.ttc",
            "/Library/Fonts/Arial.ttf",
            "/System/Library/Fonts/STHeiti Light.ttc",
        ][..];

        #[cfg(target_os = "linux")]
        let paths = &[
            // == 桌面 UI/CJK 字体（优先，轮廓更圆润，适合悬浮歌词）==
            "/usr/share/fonts/truetype/misans/MiSans-Regular.ttf",
            "/usr/share/fonts/misans/MiSans-Regular.ttf",
            "/usr/share/fonts/truetype/harmonyos-sans/HarmonyOS_Sans_SC_Regular.ttf",
            "/usr/share/fonts/harmonyos-sans/HarmonyOS_Sans_SC_Regular.ttf",
            "/usr/share/fonts/opentype/source-han-sans/SourceHanSansSC-Regular.otf",
            "/usr/share/fonts/opentype/adobe-source-han-sans/SourceHanSansSC-Regular.otf",
            "/usr/share/fonts/source-han-sans/SourceHanSansSC-Regular.otf",
            "/usr/share/fonts/opentype/noto/NotoSansSC-Regular.otf",
            "/usr/share/fonts/truetype/noto/NotoSansSC-Regular.ttf",
            "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
            "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
            "/usr/share/fonts/opentype/noto/NotoSansCJKsc-Regular.otf",
            // == 常见中文字体回退 ==
            "/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc",
            "/usr/share/fonts/wenquanyi/wqy-zenhei/wqy-zenhei.ttc",
            "/usr/share/fonts/truetype/wqy/wqy-microhei.ttc",
            "/usr/share/fonts/wenquanyi/wqy-microhei/wqy-microhei.ttc",
            "/usr/share/fonts/truetype/arphic/ukai.ttc",
            "/usr/share/fonts/truetype/arphic/uming.ttc",
            // == 拉丁 UI 字体回退 ==
            "/usr/share/fonts/truetype/ubuntu/Ubuntu-R.ttf",
            "/usr/share/fonts/ubuntu/Ubuntu-R.ttf",
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            "/usr/share/fonts/TTF/DejaVuSans.ttf",
            "/usr/share/fonts/dejavu-sans-fonts/DejaVuSans.ttf",
            "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
            "/usr/share/fonts/liberation/LiberationSans-Regular.ttf",
            "/usr/share/fonts/truetype/liberation2/LiberationSans-Regular.ttf",
            "/usr/share/fonts/liberation2/LiberationSans-Regular.ttf",
            "/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf",
            "/usr/share/fonts/noto/NotoSans-Regular.ttf",
            // == 最终回退：等宽字体 ==
            "/usr/share/fonts/sarasa-gothic/Sarasa-Regular.ttc",
            "/usr/share/fonts/truetype/sarasa-gothic/Sarasa-Regular.ttc",
            "/usr/share/fonts/sarasa-term/Sarasa-Term-Regular.ttc",
            "/usr/share/fonts/opentype/noto/NotoSansMonoCJKsc-Regular.otf",
            "/usr/share/fonts/noto-cjk/NotoSansMonoCJKsc-Regular.otf",
            "/usr/share/fonts/noto/NotoSansMonoCJKsc-Regular.otf",
            "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf",
            "/usr/share/fonts/TTF/DejaVuSansMono.ttf",
            "/usr/share/fonts/dejavu-sans-mono-fonts/DejaVuSansMono.ttf",
            "/usr/share/fonts/truetype/liberation/LiberationMono-Regular.ttf",
            "/usr/share/fonts/liberation/LiberationMono-Regular.ttf",
            "/usr/share/fonts/truetype/ubuntu/UbuntuMono-R.ttf",
            "/usr/share/fonts/ubuntu/UbuntuMono-R.ttf",
            "/usr/share/fonts/truetype/noto/NotoSansMono-Regular.ttf",
            "/usr/share/fonts/noto/NotoSansMono-Regular.ttf",
            "/usr/share/fonts/truetype/sourcecodepro/SourceCodePro-Regular.ttf",
        ][..];

        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        let paths = &[][..];

        #[cfg(target_os = "linux")]
        if let Some(data) = read_fontconfig_match("sans-serif") {
            return data;
        }

        try_read_font(paths).unwrap_or_else(|| Vec::new())
    }

    fn find_system_font_bold() -> Vec<u8> {
        #[cfg(target_os = "macos")]
        let paths = &[
            "/System/Library/Fonts/PingFang.ttc",
            "/System/Library/Fonts/Helvetica.ttc",
            "/Library/Fonts/Arial Bold.ttf",
            "/System/Library/Fonts/STHeiti Light.ttc",
        ][..];

        #[cfg(target_os = "linux")]
        let paths = &[
            // == 桌面 UI/CJK 字体（优先，轮廓更圆润，适合悬浮歌词）==
            "/usr/share/fonts/truetype/misans/MiSans-Regular.ttf",
            "/usr/share/fonts/misans/MiSans-Regular.ttf",
            "/usr/share/fonts/truetype/harmonyos-sans/HarmonyOS_Sans_SC_Regular.ttf",
            "/usr/share/fonts/harmonyos-sans/HarmonyOS_Sans_SC_Regular.ttf",
            "/usr/share/fonts/opentype/source-han-sans/SourceHanSansSC-Regular.otf",
            "/usr/share/fonts/opentype/adobe-source-han-sans/SourceHanSansSC-Regular.otf",
            "/usr/share/fonts/source-han-sans/SourceHanSansSC-Regular.otf",
            "/usr/share/fonts/opentype/noto/NotoSansSC-Regular.otf",
            "/usr/share/fonts/truetype/noto/NotoSansSC-Regular.ttf",
            "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
            "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
            "/usr/share/fonts/opentype/noto/NotoSansCJKsc-Regular.otf",
            // == 常见中文字体回退 ==
            "/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc",
            "/usr/share/fonts/wenquanyi/wqy-zenhei/wqy-zenhei.ttc",
            "/usr/share/fonts/truetype/wqy/wqy-microhei.ttc",
            "/usr/share/fonts/wenquanyi/wqy-microhei/wqy-microhei.ttc",
            "/usr/share/fonts/truetype/arphic/ukai.ttc",
            "/usr/share/fonts/truetype/arphic/uming.ttc",
            // == 拉丁 UI 字体回退 ==
            "/usr/share/fonts/truetype/ubuntu/Ubuntu-R.ttf",
            "/usr/share/fonts/ubuntu/Ubuntu-R.ttf",
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            "/usr/share/fonts/TTF/DejaVuSans.ttf",
            "/usr/share/fonts/dejavu-sans-fonts/DejaVuSans.ttf",
            "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
            "/usr/share/fonts/liberation/LiberationSans-Regular.ttf",
            "/usr/share/fonts/truetype/liberation2/LiberationSans-Regular.ttf",
            "/usr/share/fonts/liberation2/LiberationSans-Regular.ttf",
            "/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf",
            "/usr/share/fonts/noto/NotoSans-Regular.ttf",
            // == 最终回退：等宽字体 ==
            "/usr/share/fonts/sarasa-gothic/Sarasa-Regular.ttc",
            "/usr/share/fonts/truetype/sarasa-gothic/Sarasa-Regular.ttc",
            "/usr/share/fonts/sarasa-term/Sarasa-Term-Regular.ttc",
            "/usr/share/fonts/opentype/noto/NotoSansMonoCJKsc-Regular.otf",
            "/usr/share/fonts/noto-cjk/NotoSansMonoCJKsc-Regular.otf",
            "/usr/share/fonts/noto/NotoSansMonoCJKsc-Regular.otf",
            "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf",
            "/usr/share/fonts/TTF/DejaVuSansMono.ttf",
            "/usr/share/fonts/dejavu-sans-mono-fonts/DejaVuSansMono.ttf",
            "/usr/share/fonts/truetype/liberation/LiberationMono-Regular.ttf",
            "/usr/share/fonts/liberation/LiberationMono-Regular.ttf",
            "/usr/share/fonts/truetype/ubuntu/UbuntuMono-R.ttf",
            "/usr/share/fonts/ubuntu/UbuntuMono-R.ttf",
            "/usr/share/fonts/truetype/noto/NotoSansMono-Regular.ttf",
            "/usr/share/fonts/noto/NotoSansMono-Regular.ttf",
            "/usr/share/fonts/truetype/sourcecodepro/SourceCodePro-Regular.ttf",
        ][..];

        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        let paths = &[][..];

        find_first_parseable_font(paths).unwrap_or_else(find_system_font_regular)
    }

    fn find_system_font_latin_regular() -> Vec<u8> {
        #[cfg(target_os = "macos")]
        let paths = &[
            "/System/Library/Fonts/Menlo.ttc",
            "/System/Library/Fonts/Monaco.ttf",
            "/Library/Fonts/JetBrainsMono-Regular.ttf",
            "/Library/Fonts/JetBrainsMono/JetBrainsMono-Regular.ttf",
            "/Library/Fonts/FiraCode-Regular.ttf",
            "/Library/Fonts/FiraCode/FiraCode-Regular.ttf",
            "/Library/Fonts/Arial.ttf",
        ][..];

        #[cfg(target_os = "linux")]
        let paths = &[
            // == 歌词显示优先：通用 UI 无衬线字体，比终端等宽字体更适合英文大字号显示 ==
            "/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf",
            "/usr/share/fonts/noto/NotoSans-Regular.ttf",
            "/usr/share/fonts/opentype/noto/NotoSans-Regular.ttf",
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            "/usr/share/fonts/TTF/DejaVuSans.ttf",
            "/usr/share/fonts/dejavu-sans-fonts/DejaVuSans.ttf",
            "/usr/share/fonts/truetype/liberation2/LiberationSans-Regular.ttf",
            "/usr/share/fonts/liberation2/LiberationSans-Regular.ttf",
            "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
            "/usr/share/fonts/liberation/LiberationSans-Regular.ttf",
            "/usr/share/fonts/truetype/ubuntu/Ubuntu-R.ttf",
            "/usr/share/fonts/ubuntu/Ubuntu-R.ttf",
            "/usr/share/fonts/truetype/open-sans/OpenSans-Regular.ttf",
            "/usr/share/fonts/opentype/cantarell/Cantarell-Regular.otf",
            // == 可选现代英文字体 ==
            "$HOME/.local/share/fonts/Inter-Regular.ttf",
            "$HOME/.local/share/fonts/Inter/Inter-Regular.ttf",
            "$HOME/.local/share/fonts/Roboto-Regular.ttf",
            "$HOME/.local/share/fonts/Roboto/Roboto-Regular.ttf",
            "/usr/share/fonts/truetype/inter/Inter-Regular.ttf",
            "/usr/share/fonts/inter/Inter-Regular.ttf",
            "/usr/share/fonts/truetype/roboto/Roboto-Regular.ttf",
            "/usr/share/fonts/roboto/Roboto-Regular.ttf",
            // == 桌面 UI/CJK 字体回退 ==
            "/usr/share/fonts/truetype/misans/MiSans-Regular.ttf",
            "/usr/share/fonts/misans/MiSans-Regular.ttf",
            "/usr/share/fonts/truetype/harmonyos-sans/HarmonyOS_Sans_SC_Regular.ttf",
            "/usr/share/fonts/harmonyos-sans/HarmonyOS_Sans_SC_Regular.ttf",
            "/usr/share/fonts/opentype/source-han-sans/SourceHanSansSC-Regular.otf",
            "/usr/share/fonts/opentype/noto/NotoSansSC-Regular.otf",
            "/usr/share/fonts/truetype/noto/NotoSansSC-Regular.ttf",
        ][..];

        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        let paths = &[][..];

        #[cfg(target_os = "linux")]
        if let Some(data) = find_first_parseable_font(paths) {
            return data;
        }

        #[cfg(target_os = "linux")]
        for pattern in ["Noto Sans", "DejaVu Sans", "Liberation Sans", "Ubuntu", "sans-serif"] {
            if let Some(data) = read_fontconfig_match(pattern) {
                return data;
            }
        }

        try_read_font(paths).unwrap_or_else(|| {
            let fb = find_system_font_regular();
            if !fb.is_empty() { fb } else { Vec::new() }
        })
    }

    fn find_system_font_latin_bold() -> Vec<u8> {
        #[cfg(target_os = "macos")]
        let paths = &[
            "/System/Library/Fonts/Menlo.ttc",
            "/System/Library/Fonts/Monaco.ttf",
            "/Library/Fonts/JetBrainsMono-Bold.ttf",
            "/Library/Fonts/JetBrainsMono/JetBrainsMono-Bold.ttf",
            "/Library/Fonts/FiraCode-Bold.ttf",
            "/Library/Fonts/FiraCode/FiraCode-Bold.ttf",
            "/Library/Fonts/Arial Bold.ttf",
        ][..];

        #[cfg(target_os = "linux")]
        let paths = &[
            // == 歌词显示优先：通用 UI 无衬线粗体 ==
            "/usr/share/fonts/truetype/noto/NotoSans-Bold.ttf",
            "/usr/share/fonts/noto/NotoSans-Bold.ttf",
            "/usr/share/fonts/opentype/noto/NotoSans-Bold.ttf",
            "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
            "/usr/share/fonts/TTF/DejaVuSans-Bold.ttf",
            "/usr/share/fonts/dejavu-sans-fonts/DejaVuSans-Bold.ttf",
            "/usr/share/fonts/truetype/liberation2/LiberationSans-Bold.ttf",
            "/usr/share/fonts/liberation2/LiberationSans-Bold.ttf",
            "/usr/share/fonts/truetype/liberation/LiberationSans-Bold.ttf",
            "/usr/share/fonts/liberation/LiberationSans-Bold.ttf",
            "/usr/share/fonts/truetype/ubuntu/Ubuntu-B.ttf",
            "/usr/share/fonts/ubuntu/Ubuntu-B.ttf",
            "/usr/share/fonts/truetype/open-sans/OpenSans-Bold.ttf",
            "/usr/share/fonts/opentype/cantarell/Cantarell-Bold.otf",
            // == 可选现代英文字体 ==
            "$HOME/.local/share/fonts/Inter-Bold.ttf",
            "$HOME/.local/share/fonts/Inter/Inter-Bold.ttf",
            "$HOME/.local/share/fonts/Roboto-Bold.ttf",
            "$HOME/.local/share/fonts/Roboto/Roboto-Bold.ttf",
            "/usr/share/fonts/truetype/inter/Inter-Bold.ttf",
            "/usr/share/fonts/inter/Inter-Bold.ttf",
            "/usr/share/fonts/truetype/roboto/Roboto-Bold.ttf",
            "/usr/share/fonts/roboto/Roboto-Bold.ttf",
        ][..];

        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        let paths = &[][..];

        #[cfg(target_os = "linux")]
        if let Some(data) = find_first_parseable_font(paths) {
            return data;
        }

        #[cfg(target_os = "linux")]
        for pattern in ["Noto Sans:style=Bold", "DejaVu Sans:style=Bold", "Liberation Sans:style=Bold", "Ubuntu:style=Bold", "sans-serif:style=Bold"] {
            if let Some(data) = read_fontconfig_match(pattern) {
                return data;
            }
        }

        find_first_parseable_font(paths).unwrap_or_else(find_system_font_latin_regular)
    }

    pub fn run_desktop_lyrics_window(
        rx: mpsc::Receiver<DesktopLyricsCommand>,
        position: DesktopLyricsPosition,
        theme_name: &str,
        alpha: u8,
        x: i32,
        y: i32,
        ev_tx: mpsc::Sender<DesktopLyricsEvent>,
        _font_bytes: Vec<u8>,
    ) {
        // 读取滚动模式环境变量
        let scroll_mode = std::env::var("TER_DESKTOP_LYRICS_SCROLL_MODE")
            .ok()
            .map(|s| DesktopLyricsScrollMode::from_config_key(&s))
            .unwrap_or(DesktopLyricsScrollMode::Vertical);

        // 安装 panic hook，确保子线程 panic 时能将错误信息写入日志文件，
        // 避免 panic 信息仅输出到 stderr 导致 TUI 界面抖动且无法定位问题。
        // 注意：此 hook 仅在当前线程生效，不会影响主线程。
        let log_path = crate::config::get_daily_log_path();
        std::panic::set_hook(Box::new(move |info| {
            let timestamp = Local::now().format("%H:%M:%S%.3f");
            let payload = if let Some(s) = info.payload().downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = info.payload().downcast_ref::<String>() {
                s.clone()
            } else {
                "unknown panic payload".to_string()
            };
            let location = info
                .location()
                .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
                .unwrap_or_else(|| "unknown location".to_string());
            let line = format!(
                "[{}] [desktop_lyrics] PANIC at {}: {}\n",
                timestamp, location, payload
            );
            let _ = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&log_path)
                .and_then(|mut f| std::io::Write::write_all(&mut f, line.as_bytes()));
            // 仍然输出到 stderr 以便终端可见
            eprintln!("[desktop_lyrics] PANIC: {}", payload);
        }));

        // 使用 catch_unwind 包裹整个窗口创建与事件循环，
        // 防止 winit EventLoop::build() 或 run_app() 内部 panic 导致线程静默终止。
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let fb = find_system_font_regular();
            if fb.is_empty() {
                let msg = "no system font found, aborting";
                eprintln!("[desktop_lyrics] {}", msg);
                super::append_desktop_lyrics_log(msg);
                return;
            }
            let fbb = find_system_font_bold();
            let flb = find_system_font_latin_regular();
            let flbb = find_system_font_latin_bold();

            let font = match AppState::make_font(&fb, 24.0) {
                Some(f) => f,
                None => {
                    let msg = "failed to parse font, aborting";
                    eprintln!("[desktop_lyrics] {}", msg);
                    super::append_desktop_lyrics_log(msg);
                    return;
                }
            };
            let font_bold = match AppState::make_font(&fbb, 24.0) {
                Some(f) => f,
                None => {
                    let msg = "failed to parse bold font, aborting";
                    eprintln!("[desktop_lyrics] {}", msg);
                    super::append_desktop_lyrics_log(msg);
                    return;
                }
            };
            let font_latin = match AppState::make_font(&flb, 24.0) {
                Some(f) => f,
                None => {
                    let msg = "failed to parse latin font, aborting";
                    eprintln!("[desktop_lyrics] {}", msg);
                    super::append_desktop_lyrics_log(msg);
                    return;
                }
            };
            let font_latin_bold = match AppState::make_font(&flbb, 24.0) {
                Some(f) => f,
                None => {
                    let msg = "failed to parse latin bold font, aborting";
                    eprintln!("[desktop_lyrics] {}", msg);
                    super::append_desktop_lyrics_log(msg);
                    return;
                }
            };

            let wayland = std::env::var("WAYLAND_DISPLAY").ok().unwrap_or_default();
            let x11 = std::env::var("DISPLAY").ok().unwrap_or_default();
            let backend_msg = format!("unix env: WAYLAND_DISPLAY='{}', DISPLAY='{}'", wayland, x11);
            super::append_desktop_lyrics_log(&backend_msg);

            // 尝试检测 XDG_SESSION_TYPE 以辅助诊断
            if let Ok(session) = std::env::var("XDG_SESSION_TYPE") {
                super::append_desktop_lyrics_log(
                    &format!("unix env: XDG_SESSION_TYPE='{}'", session),
                );
            }

            super::append_desktop_lyrics_log("unix build: creating EventLoop...");
            let el = match EventLoop::<UserEvent>::with_user_event().build() {
                Ok(loop_ok) => loop_ok,
                Err(e) => {
                    let msg = format!("failed to create event loop: {:?}", e);
                    eprintln!("[desktop_lyrics] {}", msg);
                    super::append_desktop_lyrics_log(&msg);
                    return;
                }
            };
            super::append_desktop_lyrics_log("unix build: EventLoop created successfully");

            let proxy = el.create_proxy();
            super::append_desktop_lyrics_log("unix build: proxy created");

            let mut state = AppState {
                rx, ev_tx, proxy,
                position,
                theme_name: theme_name.to_string(),
                alpha: alpha.clamp(0, 100),
                x, y,
                window: None,
                surface: None,
                font_regular_bytes: fb,
                font_bold_bytes: fbb,
                font_latin_regular_bytes: flb,
                font_latin_bold_bytes: flbb,
                render_state: RenderState {
                    all_lyrics: Vec::new(),
                    current_time_sec: 0.0,
                    scroll_offset: 0.0,
                    old_all_lyrics: Vec::new(),
                    old_scroll_offset: 0.0,
                    prev_text: String::new(),
                    curr_text: String::new(),
                    next_text: String::new(),
                    old_prev_text: String::new(),
                    old_curr_text: String::new(),
                    old_next_text: String::new(),
                    scroll_progress: 1.0,
                    transitioning: false,
                    theme_name: theme_name.to_string(),
                    alpha: alpha.clamp(0, 100),
                    font,
                    font_bold,
                    font_latin,
                    font_latin_bold,
                    scroll_mode,
                    horizontal_progress: 0.0,
                    horizontal_target_offset: 0.0,
                    karaoke_line_group: usize::MAX,
                    old_karaoke_line_group: usize::MAX,
                    karaoke_transition_progress: 1.0,
                },
                dragging: false,
                drag_offset_x: 0,
                drag_offset_y: 0,
                last_render: Instant::now(),
                buffer: Vec::new(),
                cursor_x: 0.0,
                cursor_y: 0.0,
            };

            super::append_desktop_lyrics_log("unix run_app: starting");
            el.set_control_flow(ControlFlow::Poll);
            if let Err(e) = el.run_app(&mut state) {
                let msg = format!("event loop terminated with error: {:?}", e);
                eprintln!("[desktop_lyrics] {}", msg);
                super::append_desktop_lyrics_log(&msg);
            } else {
                super::append_desktop_lyrics_log("unix run_app: exited without error");
            }
        }));

        if let Err(e) = result {
            let payload = if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else {
                "unknown panic payload".to_string()
            };
            let msg = format!("unix window thread panicked: {}", payload);
            eprintln!("[desktop_lyrics] {}", msg);
            super::append_desktop_lyrics_log(&msg);
        }
    }
}
