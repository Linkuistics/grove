use std::{
    io::{self, IsTerminal},
    path::Path,
    time::Instant,
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

impl Action {
    /// Decode the same terminal key events used by the interactive driver.
    pub fn from_key(key: event::KeyEvent) -> Option<Self> {
        if key.kind == KeyEventKind::Release {
            return None;
        }
        match key.code {
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Some(Self::Quit),
            KeyCode::Char('q') => Some(Self::Quit),
            KeyCode::Char('r') => Some(Self::Refresh),
            KeyCode::Up | KeyCode::Char('k') => Some(Self::Up),
            KeyCode::Down | KeyCode::Char('j') => Some(Self::Down),
            KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                Some(Self::PageUp)
            }
            KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                Some(Self::PageDown)
            }
            KeyCode::Left | KeyCode::Char('h') => Some(Self::Left),
            KeyCode::Right | KeyCode::Char('l') => Some(Self::Right),
            KeyCode::Home => Some(Self::Home),
            KeyCode::End => Some(Self::End),
            KeyCode::Tab => Some(Self::Focus),
            KeyCode::Char('?') => Some(Self::Help),
            KeyCode::Esc => Some(Self::Dismiss),
            KeyCode::Enter | KeyCode::Char(' ') => Some(Self::Toggle),
            KeyCode::PageUp => Some(Self::PageUp),
            KeyCode::PageDown => Some(Self::PageDown),
            _ => None,
        }
    }
}

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
        viewer.tick(Instant::now());
        terminal.draw(|frame| viewer.render(frame))?;
        // Poll and read stay on one thread; a true poll guarantees read won't block.
        // https://docs.rs/crossterm/0.28.1/crossterm/event/fn.poll.html
        if let Some(wait) = viewer.retry_after(Instant::now()) {
            if !event::poll(wait)? {
                continue;
            }
        }
        if let Event::Key(key) = event::read()? {
            let action = Action::from_key(key);
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
