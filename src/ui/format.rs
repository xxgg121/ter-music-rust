use std::time::Duration;

/// 获取字符在终端中的实际显示宽度
pub(super) fn term_char_width(ch: char) -> usize {
    unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0)
}

/// 计算字符串在终端中的实际显示宽度
pub(super) fn term_display_width(s: &str) -> usize {
    s.chars().map(term_char_width).sum()
}

pub(super) fn format_duration_ms(ms: i64) -> String {
    let secs = (ms.max(0) / 1000) as u64;
    format!("{:02}:{:02}", secs / 60, secs % 60)
}

pub(super) fn format_progress(current: Duration, total: Option<Duration>) -> String {
    let current_secs = current.as_secs();
    let current_text = format!("{:02}:{:02}", current_secs / 60, current_secs % 60);
    if let Some(total) = total {
        let total_secs = total.as_secs();
        format!(
            "{}/{}",
            current_text,
            format!("{:02}:{:02}", total_secs / 60, total_secs % 60)
        )
    } else {
        format!("{}/--:--", current_text)
    }
}

/// 截断字符串到指定显示宽度（不加省略号，用于歌词区域）
#[allow(dead_code)]
pub(super) fn truncate_to_width(text: &str, max_width: usize) -> String {
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
#[allow(dead_code)]
pub(super) fn tail_to_width(text: &str, max_width: usize) -> String {
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

/// 从指定显示宽度偏移处切片字符串，取最多 max_width 显示宽度的子串
pub(super) fn slice_at_display_offset(text: &str, start_offset: usize, max_width: usize) -> String {
    let mut result = String::new();
    let mut current_width = 0;
    let mut skipped = 0;

    for ch in text.chars() {
        let ch_width = term_char_width(ch);
        if skipped < start_offset {
            skipped += ch_width;
            continue;
        }
        if current_width + ch_width > max_width {
            break;
        }
        result.push(ch);
        current_width += ch_width;
    }
    result
}

/// 按显示宽度自动换行，保留原始换行
pub(super) fn wrap_text_to_width(text: &str, max_width: usize) -> Vec<String> {
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
