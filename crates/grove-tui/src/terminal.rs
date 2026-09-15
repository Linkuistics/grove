use std::{
    io::{self, IsTerminal},
    panic::{self, AssertUnwindSafe},
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};

use anyhow::{bail, Result};
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use rustix::fs::{fcntl_getfl, fcntl_setfl, OFlags};

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
    with_session(|session, stopped| {
        session.raw()?;
        session.alternate()?;
        session.hide()?;
        drive(&mut viewer, stopped, session)
    })
}

fn drive(viewer: &mut Viewer, stopped: &AtomicBool, session: &Session) -> Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    drive_with(viewer, stopped, &mut terminal, |wait| {
        session.next_event(wait)
    })
}

fn drive_with<B: Backend>(
    viewer: &mut Viewer,
    stopped: &AtomicBool,
    terminal: &mut Terminal<B>,
    mut next_event: impl FnMut(Duration) -> io::Result<Option<Event>>,
) -> Result<()> {
    while !stopped.load(Ordering::SeqCst) {
        viewer.tick(Instant::now());
        terminal.draw(|frame| viewer.render(frame))?;
        // Poll and read stay on one thread; a true poll guarantees read won't block.
        // https://docs.rs/crossterm/0.28.1/crossterm/event/fn.poll.html
        let wait = viewer
            .retry_after(Instant::now())
            .unwrap_or(Duration::from_millis(100))
            .min(Duration::from_millis(100));
        if let Some(Event::Key(key)) = next_event(wait)? {
            let action = Action::from_key(key);
            if action.is_some_and(|action| viewer.act(action)) {
                return Ok(());
            }
        }
    }
    Ok(())
}

/// One terminal and panic hook per process. Refuse overlapping runs rather than
/// allowing one session to restore another's terminal or replace its hook.
static TERMINAL_OWNER: Mutex<()> = Mutex::new(());

fn with_session(body: impl FnOnce(&Session, &AtomicBool) -> Result<()>) -> Result<()> {
    let _owner = TERMINAL_OWNER
        .try_lock()
        .map_err(|_| anyhow::anyhow!("a terminal viewer is already active"))?;
    let signals = Signals::new()?;
    let session = Arc::new(Session {
        active: AtomicBool::new(true),
        input_flags: fcntl_getfl(io::stdin())?,
    });
    let previous = Arc::new(panic::take_hook());
    let (cleanup, report) = (Arc::clone(&session), Arc::clone(&previous));
    // Hooks run BEFORE unwinding (and the default diagnostic). Drop is too late.
    // https://doc.rust-lang.org/std/panic/fn.set_hook.html
    panic::set_hook(Box::new(move |info| {
        let _ = cleanup.restore();
        report(info);
    }));
    let result = panic::catch_unwind(AssertUnwindSafe(|| body(&session, &signals.stopped)));
    let cleanup = session.restore();
    // set_hook cannot run on a panicking thread. Restore it before resuming.
    drop(panic::take_hook());
    match Arc::try_unwrap(previous) {
        Ok(hook) => panic::set_hook(hook),
        Err(hook) => panic::set_hook(Box::new(move |info| hook(info))),
    }
    match result {
        Ok(result) => result.and(cleanup),
        Err(payload) => {
            // Release the owner while not unwinding, avoiding mutex poisoning.
            drop(_owner);
            panic::resume_unwind(payload)
        }
    }
}

struct Signals {
    stopped: Arc<AtomicBool>,
    registrations: Vec<signal_hook::SigId>,
}

impl Signals {
    fn new() -> Result<Self> {
        let mut signals = Self {
            stopped: Arc::new(AtomicBool::new(false)),
            registrations: Vec::new(),
        };
        for signal in [
            signal_hook::consts::SIGINT,
            signal_hook::consts::SIGTERM,
            signal_hook::consts::SIGHUP,
        ] {
            // The handler only stores a flag; terminal I/O stays on this thread.
            // https://docs.rs/signal-hook/0.3.18/signal_hook/flag/fn.register.html
            signals.registrations.push(signal_hook::flag::register(
                signal,
                Arc::clone(&signals.stopped),
            )?);
        }
        Ok(signals)
    }
}

impl Drop for Signals {
    fn drop(&mut self) {
        for id in self.registrations.drain(..) {
            signal_hook::low_level::unregister(id);
        }
    }
}

/// Armed before setup; shared with the hook so cleanup precedes diagnostics.
struct Session {
    active: AtomicBool,
    input_flags: OFlags,
}

impl Session {
    fn raw(&self) -> Result<()> {
        enable_raw_mode()?;
        Ok(())
    }

    fn next_event(&self, wait: Duration) -> io::Result<Option<Event>> {
        // Crossterm's poll backend requires nonblocking reads even for partial
        // escape sequences. Preserve the caller's open-file-description flags.
        // https://docs.rs/crate/crossterm/0.28.1/source/src/event/source/unix/tty.rs
        // https://docs.rs/rustix/0.38.44/rustix/fs/fn.fcntl_setfl.html
        fcntl_setfl(io::stdin(), self.input_flags | OFlags::NONBLOCK)?;
        let result = (|| {
            if event::poll(wait)? {
                event::read().map(Some)
            } else {
                Ok(None)
            }
        })();
        // stdin/stdout may share an open file description. Restore blocking
        // writes before rendering; the panic hook covers an unwind in input.
        let restore = fcntl_setfl(io::stdin(), self.input_flags);
        result.and_then(|event| restore.map(|_| event).map_err(Into::into))
    }

    fn alternate(&self) -> Result<()> {
        execute!(io::stdout(), EnterAlternateScreen)?;
        Ok(())
    }

    fn hide(&self) -> Result<()> {
        execute!(io::stdout(), Hide)?;
        Ok(())
    }

    fn restore(&self) -> Result<()> {
        if !self.active.load(Ordering::SeqCst) {
            return Ok(());
        }
        // Attempt each cleanup even if an earlier one fails. Keep Drop armed
        // on failure so an interrupted write gets one final best-effort retry.
        let flags = fcntl_setfl(io::stdin(), self.input_flags);
        let raw = disable_raw_mode();
        let screen = execute!(io::stdout(), LeaveAlternateScreen);
        let cursor = execute!(io::stdout(), Show);
        flags?;
        raw?;
        screen?;
        cursor?;
        self.active.store(false, Ordering::SeqCst);
        Ok(())
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        let _ = self.restore();
    }
}

#[cfg(test)]
#[path = "terminal_tests.rs"]
mod tests;
