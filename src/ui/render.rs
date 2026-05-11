use ratatui::{
    layout::Rect,
    style::{Color as TuiColor, Modifier, Style as TuiStyle},
    text::Line,
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use super::theme::{self, ThemeColors};

pub(super) fn render_list(
    frame: &mut Frame<'_>,
    area: Rect,
    title: &str,
    items: Vec<ListItem>,
    colors: ThemeColors,
) {
    let list = List::new(items)
        .block(panel_block(title, colors))
        .style(theme::tui_style(colors.song_normal));
    frame.render_widget(list, area);
}

pub(super) fn render_paragraph(
    frame: &mut Frame<'_>,
    area: Rect,
    title: &str,
    lines: Vec<Line<'_>>,
    colors: ThemeColors,
) {
    let paragraph = Paragraph::new(lines)
        .block(panel_block(title, colors))
        .wrap(Wrap { trim: false })
        .style(theme::tui_style(colors.song_normal));
    frame.render_widget(paragraph, area);
}

pub(super) fn panel_block(title: &str, colors: ThemeColors) -> Block<'_> {
    Block::default()
        .borders(Borders::ALL)
        .title(title.to_string())
        .border_style(theme::tui_style(colors.section_title))
}

pub(super) fn inner_area(area: Rect) -> Rect {
    Rect {
        x: area.x.saturating_add(1),
        y: area.y.saturating_add(1),
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    }
}

pub(super) fn selection_style(selected: bool, colors: ThemeColors) -> TuiStyle {
    let style = theme::tui_style(colors.song_normal);
    if selected {
        style.bg(TuiColor::DarkGray).add_modifier(Modifier::BOLD)
    } else {
        style
    }
}
