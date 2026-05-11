use std::io;

use crossterm::{
    cursor,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute, terminal,
};

/// 终端保护器，确保在 Drop 时恢复终端
pub(super) struct TerminalGuard;

impl TerminalGuard {
    pub(super) fn new() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(
            io::stdout(),
            terminal::EnterAlternateScreen,
            cursor::Hide,
            EnableMouseCapture
        )?;
        Ok(TerminalGuard)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = execute!(
            io::stdout(),
            DisableMouseCapture,
            terminal::LeaveAlternateScreen,
            cursor::Show
        );
        let _ = terminal::disable_raw_mode();
    }
}
