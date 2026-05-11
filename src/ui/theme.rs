use crossterm::style;
use ratatui::style::{Color as TuiColor, Style as TuiStyle};

// 主题色定义（使用显式 RGB，避免不同系统终端默认色表差异）
#[derive(Debug, Clone, Copy)]
pub(super) struct ThemeColors {
    pub(super) header_title: style::Color,
    pub(super) section_title: style::Color,
    pub(super) song_normal: style::Color,
    pub(super) song_playing: style::Color,
    pub(super) lyric_highlight: style::Color,
    pub(super) status_text: style::Color,
    pub(super) progress_text: style::Color,
    pub(super) info_text: style::Color,
}

pub(super) fn tui_style(color: style::Color) -> TuiStyle {
    TuiStyle::default().fg(to_tui_color(color))
}

pub(super) fn to_tui_color(color: style::Color) -> TuiColor {
    match color {
        style::Color::Black => TuiColor::Black,
        style::Color::DarkGrey => TuiColor::DarkGray,
        style::Color::Red => TuiColor::Red,
        style::Color::DarkRed => TuiColor::DarkGray,
        style::Color::Green => TuiColor::Green,
        style::Color::DarkGreen => TuiColor::Green,
        style::Color::Yellow => TuiColor::Yellow,
        style::Color::DarkYellow => TuiColor::Yellow,
        style::Color::Blue => TuiColor::Blue,
        style::Color::DarkBlue => TuiColor::Blue,
        style::Color::Magenta => TuiColor::Magenta,
        style::Color::DarkMagenta => TuiColor::Magenta,
        style::Color::Cyan => TuiColor::Cyan,
        style::Color::DarkCyan => TuiColor::Cyan,
        style::Color::White => TuiColor::White,
        style::Color::Grey => TuiColor::Gray,
        style::Color::Rgb { r, g, b } => TuiColor::Rgb(r, g, b),
        style::Color::AnsiValue(v) => TuiColor::Indexed(v),
        style::Color::Reset => TuiColor::Reset,
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) enum UiTheme {
    GrayWhite,
    Neon,
    Sunset,
    Ocean,
}

impl UiTheme {
    pub(super) fn next(self) -> Self {
        match self {
            UiTheme::GrayWhite => UiTheme::Neon,
            UiTheme::Neon => UiTheme::Sunset,
            UiTheme::Sunset => UiTheme::Ocean,
            UiTheme::Ocean => UiTheme::GrayWhite,
        }
    }

    pub(super) fn config_key(self) -> &'static str {
        match self {
            UiTheme::GrayWhite => "GrayWhite",
            UiTheme::Neon => "Neon",
            UiTheme::Sunset => "Sunset",
            UiTheme::Ocean => "Ocean",
        }
    }

    pub(super) fn from_config_key(s: &str) -> Self {
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

    pub(super) fn colors(self) -> ThemeColors {
        match self {
            UiTheme::GrayWhite => ThemeColors {
                header_title: style::Color::Rgb {
                    r: 238,
                    g: 242,
                    b: 246,
                },
                section_title: style::Color::Rgb {
                    r: 223,
                    g: 229,
                    b: 235,
                },
                song_normal: style::Color::Rgb {
                    r: 188,
                    g: 194,
                    b: 202,
                },
                song_playing: style::Color::Rgb {
                    r: 246,
                    g: 250,
                    b: 255,
                },
                lyric_highlight: style::Color::Rgb {
                    r: 224,
                    g: 233,
                    b: 246,
                },
                status_text: style::Color::Rgb {
                    r: 232,
                    g: 237,
                    b: 244,
                },
                progress_text: style::Color::Rgb {
                    r: 210,
                    g: 217,
                    b: 226,
                },
                info_text: style::Color::Rgb {
                    r: 216,
                    g: 223,
                    b: 232,
                },
            },
            UiTheme::Neon => ThemeColors {
                header_title: style::Color::Rgb {
                    r: 0,
                    g: 215,
                    b: 255,
                },
                section_title: style::Color::Rgb {
                    r: 255,
                    g: 235,
                    b: 80,
                },
                song_normal: style::Color::Rgb {
                    r: 0,
                    g: 255,
                    b: 120,
                },
                song_playing: style::Color::Rgb {
                    r: 0,
                    g: 255,
                    b: 120,
                },
                lyric_highlight: style::Color::Rgb {
                    r: 255,
                    g: 235,
                    b: 80,
                },
                status_text: style::Color::Rgb {
                    r: 255,
                    g: 235,
                    b: 80,
                },
                progress_text: style::Color::Rgb {
                    r: 0,
                    g: 170,
                    b: 255,
                },
                info_text: style::Color::Rgb {
                    r: 0,
                    g: 215,
                    b: 255,
                },
            },
            UiTheme::Sunset => ThemeColors {
                header_title: style::Color::Rgb {
                    r: 255,
                    g: 186,
                    b: 73,
                },
                section_title: style::Color::Rgb {
                    r: 255,
                    g: 221,
                    b: 124,
                },
                song_normal: style::Color::Rgb {
                    r: 255,
                    g: 197,
                    b: 120,
                },
                song_playing: style::Color::Rgb {
                    r: 255,
                    g: 238,
                    b: 176,
                },
                lyric_highlight: style::Color::Rgb {
                    r: 255,
                    g: 246,
                    b: 120,
                },
                status_text: style::Color::Rgb {
                    r: 255,
                    g: 212,
                    b: 96,
                },
                progress_text: style::Color::Rgb {
                    r: 255,
                    g: 170,
                    b: 84,
                },
                info_text: style::Color::Rgb {
                    r: 255,
                    g: 205,
                    b: 138,
                },
            },
            UiTheme::Ocean => ThemeColors {
                header_title: style::Color::Rgb {
                    r: 102,
                    g: 226,
                    b: 255,
                },
                section_title: style::Color::Rgb {
                    r: 126,
                    g: 250,
                    b: 228,
                },
                song_normal: style::Color::Rgb {
                    r: 116,
                    g: 243,
                    b: 204,
                },
                song_playing: style::Color::Rgb {
                    r: 166,
                    g: 255,
                    b: 234,
                },
                lyric_highlight: style::Color::Rgb {
                    r: 168,
                    g: 255,
                    b: 245,
                },
                status_text: style::Color::Rgb {
                    r: 134,
                    g: 235,
                    b: 255,
                },
                progress_text: style::Color::Rgb {
                    r: 108,
                    g: 188,
                    b: 255,
                },
                info_text: style::Color::Rgb {
                    r: 120,
                    g: 224,
                    b: 255,
                },
            },
        }
    }
}
