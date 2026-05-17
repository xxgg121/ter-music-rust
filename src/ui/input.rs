use crossterm::event::KeyCode;

use crate::defs::PlayState;

impl super::UserInterface {
    pub(super) fn handle_api_key_input(&mut self, code: KeyCode) -> bool {
        if !self.api_key_input_mode {
            return false;
        }

        match code {
            KeyCode::Esc => {
                self.api_key_input_mode = false;
                self.api_key_input_for_song_info = false;
                self.api_input_step = 0;
                self.api_key_input_value.clear();
                self.cached_lyrics_title = None;
            }
            KeyCode::Enter => {
                let value = self.api_key_input_value.trim().to_string();
                match self.api_input_step {
                    0 => {
                        self.api_base_url = if value.is_empty() {
                            "https://api.deepseek.com/".to_string()
                        } else if value.ends_with('/') {
                            value
                        } else {
                            format!("{}/", value)
                        };
                        self.api_input_step = 1;
                        self.api_key_input_value = self.resolved_api_key().unwrap_or_default();
                        self.cached_lyrics_title = None;
                    }
                    1 => {
                        self.api_key = value.clone();
                        if value.is_empty() {
                            std::env::remove_var("DEEPSEEK_API_KEY");
                        } else {
                            std::env::set_var("DEEPSEEK_API_KEY", &value);
                        }
                        self.api_input_step = 2;
                        self.api_key_input_value = self.api_model.clone();
                        self.cached_lyrics_title = None;
                    }
                    2 => {
                        self.api_model = if value.is_empty() {
                            "deepseek-v4-flash".to_string()
                        } else {
                            value
                        };
                        self.save_config_now();
                        let continue_song_info = self.api_key_input_for_song_info;
                        self.api_key_input_mode = false;
                        self.api_key_input_for_song_info = false;
                        self.api_input_step = 0;
                        self.api_key_input_value.clear();
                        self.cached_lyrics_title = None;

                        if continue_song_info {
                            self.start_song_info_mode_for_current_song();
                        }
                    }
                    _ => {}
                }
            }
            KeyCode::Backspace => {
                self.api_key_input_value.pop();
                self.cached_lyrics_title = None;
            }
            KeyCode::Char(c) => {
                self.api_key_input_value.push(c);
                self.cached_lyrics_title = None;
            }
            _ => {}
        }

        true
    }

    pub(super) fn handle_github_token_input(&mut self, code: KeyCode) -> bool {
        if !self.github_token_input_mode {
            return false;
        }

        match code {
            KeyCode::Esc => {
                self.github_token_input_mode = false;
                self.github_token_input_value.clear();
                self.cached_lyrics_title = None;
            }
            KeyCode::Enter => {
                let value = self.github_token_input_value.trim().to_string();
                self.github_token = value;
                self.github_token_input_mode = false;
                self.github_token_input_value.clear();
                self.cached_lyrics_title = None;
                self.save_config_now();
            }
            KeyCode::Backspace => {
                self.github_token_input_value.pop();
                self.cached_lyrics_title = None;
            }
            KeyCode::Char(c) => {
                self.github_token_input_value.push(c);
                self.cached_lyrics_title = None;
            }
            _ => {}
        }

        true
    }

    pub(super) fn handle_ai_recommend_input(&mut self, code: KeyCode) -> bool {
        if !self.ai_recommend_input_mode {
            return false;
        }

        match code {
            KeyCode::Esc => {
                self.ai_recommend_input_mode = false;
                self.ai_recommend_input_value.clear();
            }
            KeyCode::Enter => {
                self.start_ai_recommend_query();
            }
            KeyCode::Backspace => {
                self.ai_recommend_input_value.pop();
            }
            KeyCode::Char(c) => {
                self.ai_recommend_input_value.push(c);
            }
            _ => {}
        }

        true
    }

    pub(super) fn handle_search_input(&mut self, code: KeyCode) -> bool {
        if !self.search_mode {
            return false;
        }

        let in_playlist_detail =
            self.online_search_mode && self.playlist_search_mode && self.current_playlist.is_some();
        let online_input_focused =
            self.online_search_mode && !in_playlist_detail && self.search_input_focused;
        let mut handled_in_search = true;

        match code {
            KeyCode::Esc => {
                if self.comments_mode {
                    self.comments_mode = false;
                    self.comments_detail_mode = false;
                } else if self.song_info_mode {
                    if self.song_info_kind == super::SongInfoKind::CommentSummary {
                        self.comments_mode = true;
                        self.comments_detail_mode = false;
                    }
                    self.song_info_mode = false;
                } else if self.help_mode {
                    self.help_mode = false;
                } else if self.playlist_search_mode && self.current_playlist.is_some() {
                    let was_url_import = self.online_list_url_import_mode
                        || self.online_list_url_source.is_some()
                        || !self.lazy_online_all_results.is_empty();
                    self.clear_online_download_state();
                    self.current_playlist = None;
                    self.online_search_results.clear();
                    self.lazy_online_page_rx = None;
                    self.online_list_url_page_rx = None;
                    self.online_list_url_import_mode = false;
                    self.online_selected_index = self.playlist_list_selected_index;
                    self.online_scroll_offset = self.online_selected_index.saturating_sub(2);
                    let total = self.playlist_search_results.len();
                    Self::clamp_selected_and_scroll(
                        &mut self.online_selected_index,
                        &mut self.online_scroll_offset,
                        total,
                        (self.terminal_height as usize).saturating_sub(12).max(5),
                    );
                    self.online_searching = false;
                    self.playlist_songs_rx = None;
                    if was_url_import {
                        self.search_input_focused = true;
                        self.search_query.clear();
                        self.active_preset_rank_title = None;
                        self.playlist_search_results.clear();
                        self.online_selected_index = 0;
                        self.online_scroll_offset = 0;
                    } else {
                        self.search_input_focused = false;
                    }
                } else {
                    self.clear_online_download_state();
                    self.search_mode = false;
                    self.online_search_mode = false;
                    self.juhe_search_mode = false;
                    self.playlist_search_mode = false;
                    self.search_query.clear();
                    self.search_results.clear();
                    self.search_selected_index = 0;
                    self.search_scroll_offset = 0;
                    self.online_search_results.clear();
                    self.clear_lazy_online_page_state();
                    self.online_list_url_import_mode = false;
                    self.online_selected_index = 0;
                    self.online_scroll_offset = 0;
                    self.online_searching = false;
                    self.online_search_page = 1;
                    self.online_search_rx = None;
                    self.playlist_search_rx = None;
                    self.playlist_songs_rx = None;
                    self.playlist_search_results.clear();
                    self.current_playlist = None;
                }
            }
            KeyCode::Enter => {
                if self.comments_mode || self.help_mode {
                    handled_in_search = false;
                } else if self.preset_rank_has_focus() {
                    let _ = self.activate_selected_preset_rank();
                } else if self.try_use_selected_search_history() {
                } else if self.preset_rank_grid_visible() {
                    let _ = self.activate_selected_preset_rank();
                } else if self.online_search_mode {
                    if self.online_searching || self.online_downloading {
                    } else if online_input_focused
                        && Self::is_online_list_url_input(&self.search_query)
                    {
                        self.start_online_list_url_import();
                    } else if self.playlist_search_mode && self.current_playlist.is_none() {
                        if !self.playlist_search_results.is_empty() {
                            if let Some(pl) = self
                                .playlist_search_results
                                .get(self.online_selected_index)
                                .cloned()
                            {
                                self.clear_online_download_state();
                                self.playlist_list_selected_index = self.online_selected_index;
                                self.online_searching = true;
                                self.online_search_results.clear();
                                self.clear_lazy_online_page_state();
                                self.online_list_url_import_mode = false;
                                self.online_selected_index = 0;
                                self.online_scroll_offset = 0;
                                self.current_playlist = Some(pl.clone());
                                self.playlist_songs_rx =
                                    Some(crate::search::fetch_playlist_songs_background(pl));
                                self.search_input_focused = false;
                            }
                        } else if online_input_focused && !self.search_query.is_empty() {
                            self.online_search_page = 1;
                            self.start_online_search();
                        }
                    } else if !self.online_search_results.is_empty() {
                        if let Some(song) = self.resolved_online_song_at(self.online_selected_index)
                        {
                            self.online_auto_skip_times.clear();
                            self.start_download_song(song);
                            self.search_input_focused = false;
                        }
                    } else if online_input_focused && !self.search_query.is_empty() {
                        self.online_search_page = 1;
                        self.start_online_search();
                    }
                } else if !self.search_results.is_empty() {
                    if let Some(&orig_idx) = self.search_results.get(self.search_selected_index) {
                        self.selected_index = orig_idx;
                        self.search_mode = false;
                        self.search_input_focused = false;
                        self.search_query.clear();
                        self.search_results.clear();
                        self.search_scroll_offset = 0;
                        self.play_song_by_index(orig_idx);
                    }
                } else if !self.search_query.is_empty() {
                    self.update_search_results();
                }
            }
            KeyCode::Up => {
                if self.comments_mode || self.song_info_mode || self.help_mode {
                    handled_in_search = false;
                } else if self.preset_rank_has_focus() {
                    self.move_preset_rank_selection(-3);
                    if self.preset_rank_selected_index == Some(0) && self.search_history_visible() {
                        self.preset_rank_selected_index = None;
                        let len = self.visible_search_history_items().len();
                        self.search_history_selected_index = len.saturating_sub(1);
                    }
                } else if self.search_history_visible() {
                    if self.search_history_selected_index == 0 && self.preset_rank_grid_visible() {
                        self.preset_rank_selected_index = Some(0);
                    } else {
                        self.move_search_history_selection(-1);
                    }
                } else if self.online_search_mode {
                    if self.online_selected_index > 0 {
                        self.online_selected_index -= 1;
                    }
                    self.search_input_focused = false;
                } else if self.search_selected_index > 0 {
                    self.search_selected_index -= 1;
                    self.search_input_focused = false;
                }
            }
            KeyCode::Down => {
                if self.comments_mode || self.song_info_mode || self.help_mode {
                    handled_in_search = false;
                } else if self.preset_rank_has_focus() {
                    self.move_preset_rank_selection(3);
                } else if self.search_history_visible() {
                    let len = self.visible_search_history_items().len();
                    if self.search_history_selected_index + 1 >= len && self.preset_rank_grid_visible() {
                        self.preset_rank_selected_index = Some(0);
                    } else {
                        self.move_search_history_selection(1);
                    }
                } else if self.online_search_mode {
                    let total = if self.playlist_search_mode && self.current_playlist.is_none() {
                        self.playlist_search_results.len()
                    } else {
                        self.online_search_results.len()
                    };
                    if total > 0 && self.online_selected_index < total - 1 {
                        self.online_selected_index += 1;
                    }
                    self.search_input_focused = false;
                } else if !self.search_results.is_empty()
                    && self.search_selected_index < self.search_results.len() - 1
                {
                    self.search_selected_index += 1;
                    self.search_input_focused = false;
                }
            }
            KeyCode::Backspace => {
                if self.online_search_mode {
                    if !online_input_focused {
                        handled_in_search = false;
                    } else if !self.search_query.is_empty() {
                        self.search_query.pop();
                        self.online_search_results.clear();
                        self.clear_lazy_online_page_state();
                        self.online_list_url_import_mode = false;
                        self.playlist_search_results.clear();
                        self.current_playlist = None;
                        self.online_selected_index = 0;
                        self.online_scroll_offset = 0;
                        self.online_search_page = 1;
                    }
                } else {
                    self.search_query.pop();
                    self.search_results.clear();
                    self.search_selected_index = 0;
                    self.search_scroll_offset = 0;
                }
            }
            KeyCode::Char(c) => {
                if in_playlist_detail {
                    if c == ' ' {
                        let mut audio_player = self.audio_player.lock().unwrap();
                        match audio_player.get_state() {
                            PlayState::Playing => audio_player.pause(),
                            PlayState::Paused => audio_player.resume(),
                            _ => {}
                        }
                    } else {
                        handled_in_search = false;
                    }
                } else if self.search_history_visible() && (c == 'd' || c == 'D') {
                    let _ = self.delete_selected_search_history();
                } else if self.online_search_mode && !online_input_focused {
                    handled_in_search = false;
                } else {
                    self.search_query.push(c);
                    if self.online_search_mode {
                        if !self.online_search_results.is_empty()
                            || !self.playlist_search_results.is_empty()
                            || self.current_playlist.is_some()
                        {
                            self.online_search_results.clear();
                            self.clear_lazy_online_page_state();
                            self.online_list_url_import_mode = false;
                            self.playlist_search_results.clear();
                            self.current_playlist = None;
                            self.online_selected_index = 0;
                            self.online_scroll_offset = 0;
                            self.online_search_page = 1;
                        }
                    } else {
                        self.search_results.clear();
                        self.search_selected_index = 0;
                        self.search_scroll_offset = 0;
                    }
                }
            }
            KeyCode::PageUp => {
                if self.comments_mode || self.song_info_mode || self.help_mode {
                    handled_in_search = false;
                } else if self.playlist_search_mode && self.current_playlist.is_some() {
                    if self.online_list_url_source.is_some() {
                        if self.online_list_url_page > 1 && !self.online_searching {
                            self.start_online_list_url_page_load(self.online_list_url_page - 1);
                        }
                    } else if !self.lazy_online_all_results.is_empty() {
                        if self.lazy_online_page > 0 && !self.online_searching {
                            self.start_lazy_online_page_load(self.lazy_online_page - 1, 20);
                        }
                    } else {
                        let page_size = 20usize;
                        let total = self.online_search_results.len();
                        if total > 0 {
                            let cur_page = self.online_selected_index / page_size;
                            let prev_page = cur_page.saturating_sub(1);
                            self.online_selected_index = prev_page * page_size;
                            self.online_scroll_offset = self.online_selected_index;
                            Self::clamp_selected_and_scroll(
                                &mut self.online_selected_index,
                                &mut self.online_scroll_offset,
                                total,
                                (self.terminal_height as usize).saturating_sub(12).max(5),
                            );
                        }
                    }
                    self.search_input_focused = false;
                } else if self.online_search_mode
                    && !self.online_searching
                    && self.online_search_page > 1
                {
                    self.online_search_page -= 1;
                    self.start_online_search();
                    self.search_input_focused = false;
                }
            }
            KeyCode::Left => {
                if self.preset_rank_grid_visible() {
                    self.move_preset_rank_selection(-1);
                } else {
                    handled_in_search = false;
                }
            }
            KeyCode::Right => {
                if self.preset_rank_grid_visible() {
                    self.move_preset_rank_selection(1);
                } else {
                    handled_in_search = false;
                }
            }
            KeyCode::PageDown => {
                if self.comments_mode || self.song_info_mode || self.help_mode {
                    handled_in_search = false;
                } else if self.playlist_search_mode && self.current_playlist.is_some() {
                    if self.online_list_url_source.is_some() {
                        if !self.online_searching {
                            self.start_online_list_url_page_load(self.online_list_url_page + 1);
                        }
                    } else if !self.lazy_online_all_results.is_empty() {
                        let page_size = 20usize;
                        let total_pages =
                            (self.lazy_online_all_results.len() + page_size - 1) / page_size;
                        if self.lazy_online_page + 1 < total_pages && !self.online_searching {
                            self.start_lazy_online_page_load(self.lazy_online_page + 1, page_size);
                        }
                    } else {
                        let page_size = 20usize;
                        let total = self.online_search_results.len();
                        if total > 0 {
                            let cur_page = self.online_selected_index / page_size;
                            let next_index = (cur_page + 1) * page_size;
                            if next_index < total {
                                self.online_selected_index = next_index;
                                self.online_scroll_offset = self.online_selected_index;
                                Self::clamp_selected_and_scroll(
                                    &mut self.online_selected_index,
                                    &mut self.online_scroll_offset,
                                    total,
                                    (self.terminal_height as usize).saturating_sub(12).max(5),
                                );
                            }
                        }
                    }
                    self.search_input_focused = false;
                } else if self.online_search_mode && !self.online_searching {
                    let has_results =
                        if self.playlist_search_mode && self.current_playlist.is_none() {
                            !self.playlist_search_results.is_empty()
                        } else {
                            !self.online_search_results.is_empty()
                        };
                    if has_results {
                        self.online_search_page += 1;
                        self.start_online_search();
                        self.search_input_focused = false;
                    }
                }
            }
            _ => {
                if in_playlist_detail {
                    handled_in_search = false;
                }
            }
        }

        handled_in_search
    }

    pub(super) fn handle_m3u_input(&mut self, code: KeyCode) -> bool {
        if !self.m3u_file_input_mode {
            return false;
        }

        match code {
            KeyCode::Esc => {
                self.m3u_file_input_mode = false;
                self.m3u_file_input.clear();
                self.m3u_export_mode = false;
            }
            KeyCode::Enter => {
                let path = self.m3u_file_input.trim().to_string();
                if !path.is_empty() {
                    let path = std::path::Path::new(&path);
                    if self.m3u_export_mode {
                        let playlist = self.playlist.lock().unwrap();
                        if let Err(e) = playlist.export_m3u(path) {
                            self.status_message = format!("{}: {}", self.t().m3u_export_failed, e);
                        } else {
                            self.status_message = format!(
                                "{}: {} -> {}",
                                self.t().m3u_export_success,
                                playlist.len(),
                                path.display()
                            );
                        }
                    } else {
                        let mut playlist = self.playlist.lock().unwrap();
                        match playlist.import_m3u(path) {
                            Ok(count) => {
                                self.status_message =
                                    format!("{}: {}", self.t().m3u_import_success, count);
                            }
                            Err(e) => {
                                self.status_message =
                                    format!("{}: {}", self.t().m3u_import_failed, e);
                            }
                        }
                    }
                }
                self.m3u_file_input_mode = false;
                self.m3u_file_input.clear();
                self.m3u_export_mode = false;
            }
            KeyCode::Backspace => {
                self.m3u_file_input.pop();
            }
            KeyCode::Char(c) => {
                self.m3u_file_input.push(c);
            }
            _ => {}
        }

        true
    }

    pub(super) fn handle_lyrics_calibration(&mut self, code: KeyCode) -> bool {
        if !self.lyrics_calibration_mode {
            return false;
        }

        match code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.lyrics_calibration_mode = false;
            }
            KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('H') => {
                self.lyrics_offset -= 0.1;
            }
            KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('L') => {
                self.lyrics_offset += 0.1;
            }
            KeyCode::Up => {
                self.lyrics_offset -= 0.5;
            }
            KeyCode::Down => {
                self.lyrics_offset += 0.5;
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.lyrics_offset = 0.0;
            }
            KeyCode::Enter => {
                self.lyrics_calibration_mode = false;
                self.save_config_now();
            }
            _ => {}
        }

        true
    }
}
