use std::{
    io::{self, IsTerminal},
    path::Path,
};

use anyhow::{bail, Result};
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::{Action, Viewer};

/// View a worktree without resolving a workspace or loading launch policy.
/// Requires interactive stdin/stdout before any terminal state changes.
pub fn run(worktree: &Path) -> Result<()> {
    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        bail!("grove view requires interactive stdin and stdout; run it directly in a terminal");
    }
    let worktree = std::path::absolute(worktree)?;
    let mut viewer = Viewer::new(worktree);
    let mut session = Session::enter()?;
    let result = drive(&mut viewer);
    let cleanup = session.restore();
    result.and(cleanup)
}

fn drive(viewer: &mut Viewer) -> Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    loop {
        terminal.draw(|frame| viewer.render(frame))?;
        // Blocking event reads and raw mode use one thread, per Crossterm's contract.
        // https://docs.rs/crossterm/0.28.1/crossterm/event/index.html
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }
            let action = match key.code {
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    Some(Action::Quit)
                }
                KeyCode::Char('q') => Some(Action::Quit),
                KeyCode::Char('r') => Some(Action::Refresh),
                KeyCode::Up | KeyCode::Char('k') => Some(Action::Up),
                KeyCode::Down | KeyCode::Char('j') => Some(Action::Down),
                KeyCode::Enter => Some(Action::Toggle),
                KeyCode::PageUp => Some(Action::PageUp),
                KeyCode::PageDown => Some(Action::PageDown),
                _ => None,
            };
            if action.is_some_and(|action| viewer.act(action)) {
                return Ok(());
            }
        }
    }
}

/// Installed before setup so a returned error also unwinds partial setup.
struct Session {
    active: bool,
}

impl Session {
    fn enter() -> Result<Self> {
        let session = Self { active: true };
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen, Hide)?;
        Ok(session)
    }

    fn restore(&mut self) -> Result<()> {
        if !self.active {
            return Ok(());
        }
        // Attempt each cleanup even if an earlier one fails. Keep Drop armed
        // on failure so an interrupted write gets one final best-effort retry.
        let raw = disable_raw_mode();
        let screen = execute!(io::stdout(), LeaveAlternateScreen);
        let cursor = execute!(io::stdout(), Show);
        raw?;
        screen?;
        cursor?;
        self.active = false;
        Ok(())
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        let _ = self.restore();
    }
}
