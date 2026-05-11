use std::time::Duration;

pub(super) struct PlaylistPanelView {
    pub(super) title: String,
    pub(super) rows: Vec<PlaylistRowView>,
    pub(super) is_empty: bool,
}

pub(super) struct PlaylistRowView {
    pub(super) text: String,
    pub(super) selected: bool,
    pub(super) playing: bool,
}

pub(super) struct ControlsView {
    pub(super) tip: String,
    pub(super) play_status_text: String,
    pub(super) now_playing_text: String,
    pub(super) progress_label: String,
    pub(super) progress_ratio: f64,
    pub(super) volume_percent: u8,
    pub(super) realtime_percent: u8,
}

pub(super) struct SearchResultsView {
    pub(super) title: String,
    pub(super) rows: Vec<SelectableTextRow>,
    pub(super) empty_hint: Option<&'static str>,
}

pub(super) struct SelectableListView {
    pub(super) title: String,
    pub(super) rows: Vec<SelectableTextRow>,
    pub(super) empty_hint: Option<&'static str>,
}

pub(super) struct TextPanelView {
    pub(super) title: String,
    pub(super) lines: Vec<String>,
}

pub(super) struct LyricsPanelView {
    pub(super) title: String,
    pub(super) rows: Vec<HighlightedTextRow>,
    pub(super) line_times: Vec<Duration>,
}

pub(super) struct CommentsListView {
    pub(super) title: String,
    pub(super) rows: Vec<SelectableTextRow>,
    pub(super) row_map: Vec<Option<usize>>,
    pub(super) empty_hint: Option<&'static str>,
}

pub(super) struct SelectableTextRow {
    pub(super) text: String,
    pub(super) selected: bool,
}

pub(super) struct HighlightedTextRow {
    pub(super) text: String,
    pub(super) highlighted: bool,
}
