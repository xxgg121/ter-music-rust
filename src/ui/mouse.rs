use std::io;

use crossterm::event::MouseEvent;

impl super::UserInterface {
    pub(super) fn handle_mouse_event(&mut self, mouse_event: MouseEvent) -> io::Result<()> {
        self.handle_mouse_event_impl(mouse_event)
    }
}
