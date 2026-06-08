//! The async draw loop: a ratatui app that owns its own render loop (D3) and
//! embeds one live harness pane.
//!
//! Shape (010-draw-loop-pane): connect/start the rmux daemon, ensure one
//! deterministically-named session, open one harness pane running
//! `grove do <name>` in that grove's worktree (E3), then run an event-driven
//! `tokio::select!` over three sources — per-pane render-stream pushes, crossterm
//! input, and fs-watch ticks — redrawing only when something changed. Input
//! handling beyond quit + resize is deliberately out of scope here (020); this
//! leaf proves the *render* path.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::{
    self, DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture,
    Event,
};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};
use ratatui::Terminal;
use ratatui_rmux::PaneState;
use rmux_sdk::{EnsureSession, EnsureSessionPolicy, Pane, Rmux, SessionName, TerminalSizeSpec};
use tokio::sync::mpsc;

use crate::cli::TuiArgs;
use crate::tui::config::{resolve_leader, Leader};
use crate::tui::driver::PaneDriver;
use crate::tui::focus::{arbitrate, Action, Focus, ModalKind};
use crate::tui::pane::render_pane;

/// One rmux session per `grove tui`, named deterministically (ADR-0027's
/// singleton). 050 revisits park-alive / multi-session lifecycle.
const SESSION_NAME: &str = "grove-fleet";

/// The harness pane's process + placement, chosen below the presentation seam
/// (sync core). With no nav yet (030), the single pane runs
/// `grove do <first-live-grove>` in that grove's worktree; a repo with no live
/// groves falls back to the user's shell so `grove tui` still launches and
/// renders a live pane.
struct PaneProcess {
    /// Map key / focus label: the grove name, or `"shell"` for the fallback.
    key: String,
    cwd: PathBuf,
    argv: Vec<String>,
}

pub async fn run_app(args: &TuiArgs) -> Result<()> {
    // --- below the seam: pick what the single pane runs (sync core) ---
    let repo_roots = resolve_repo_roots(args)?;
    let process = select_pane_process(&repo_roots)?;

    // The leader key (E4), configurable from day one (`tui.toml`), default Alt-g.
    let leader = resolve_leader().context("resolving the leader key")?;

    // --- connect/start the daemon and ensure one sized, detached session ---
    let (cols, rows) = ratatui::crossterm::terminal::size().unwrap_or((120, 32));
    let rmux = Rmux::builder()
        .default_timeout(Duration::from_secs(10))
        .connect_or_start()
        .await
        .context("connecting to / starting the rmux daemon")?;
    let session_name = SessionName::new(SESSION_NAME).expect("valid session name");
    let session = rmux
        .ensure_session(
            EnsureSession::named(session_name)
                .policy(EnsureSessionPolicy::CreateOrReuse)
                .detached(true)
                .size(TerminalSizeSpec::new(cols, rows))
                .window_name("main")
                .working_directory(process.cwd.to_string_lossy().into_owned())
                .argv(process.argv.clone()),
        )
        .await
        .context("ensuring the grove tui session")?;

    // --- one harness pane, addressed by stable PaneId (E3) ---
    let pane = session.pane(0, 0);
    let pane_id = pane
        .id()
        .await
        .context("resolving harness pane id")?
        .context("harness pane has no id yet")?;

    // The grove-name → pane map (E3): one entry this leaf, but the addressing
    // model is established now so 030's dynamic open/close/park can extend it.
    let mut panes: HashMap<String, PaneDriver> = HashMap::new();
    panes.insert(process.key.clone(), PaneDriver::new(pane, pane_id));
    let focused = process.key.clone();

    // --- D3 push: spawn the focused pane's render-stream task ---
    let mut render_rx = panes[&focused]
        .spawn_render_task()
        .await
        .context("opening the harness pane render stream")?;

    // --- input + fs-watch event sources, surfaced into the select! ---
    let stop = Arc::new(AtomicBool::new(false));
    let mut input_rx = spawn_input_reader(Arc::clone(&stop));
    let (_watcher, mut watch_rx) = spawn_fs_watch(&repo_roots)?;

    // --- terminal setup ---
    // Bracketed paste (E5): so a multi-line paste arrives as one `Event::Paste`
    // we forward wrapped, not as line-by-line key events that execute as typed.
    // Mouse capture feeds click-to-focus (E5).
    enable_raw_mode().context("enabling raw mode")?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture, EnableBracketedPaste)
        .context("entering alternate screen")?;
    let mut terminal =
        Terminal::new(CrosstermBackend::new(stdout)).context("creating the ratatui terminal")?;

    // --- the event-driven draw loop ---
    let result = event_loop(
        &mut terminal,
        &mut render_rx,
        &mut input_rx,
        &mut watch_rx,
        panes[&focused].pane(),
        &leader,
    )
    .await;

    // --- teardown (always, even on error) ---
    stop.store(true, Ordering::Relaxed);
    let _ = disable_raw_mode();
    let _ = execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        DisableBracketedPaste
    );
    let _ = terminal.show_cursor();
    result
}

/// The `tokio::select!` loop. Redraws only when a source reports a change
/// (render push, input, or fs-watch tick). Owns the focus state machine
/// ([`Focus`]) and the modal buffer; input is routed through the pure
/// [`arbitrate`] table and the resulting [`Action`] applied here (the impure
/// seam). Returns when the pane exits, input closes, or the user quits
/// (leader → `q`).
async fn event_loop(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    render_rx: &mut mpsc::UnboundedReceiver<rmux_sdk::PaneSnapshot>,
    input_rx: &mut mpsc::UnboundedReceiver<Event>,
    watch_rx: &mut mpsc::UnboundedReceiver<()>,
    pane: &Pane,
    leader: &Leader,
) -> Result<()> {
    let mut state = PaneState::default();
    let mut focus = Focus::Harness;
    let mut modal_buf = String::new();
    let mut needs_redraw = true;
    loop {
        if needs_redraw {
            draw(terminal, &state, &focus, &modal_buf)?;
            needs_redraw = false;
        }
        tokio::select! {
            maybe = render_rx.recv() => match maybe {
                Some(snapshot) => { state.set_snapshot(snapshot); needs_redraw = true; }
                None => break, // render task ended: the pane's process exited
            },
            maybe = input_rx.recv() => match maybe {
                Some(ev) => match handle_event(&ev, pane, leader, &mut focus, &mut modal_buf).await? {
                    EventOutcome::Quit => break,
                    EventOutcome::Redraw => needs_redraw = true,
                    EventOutcome::Nothing => {}
                },
                None => break, // input reader gone
            },
            maybe = watch_rx.recv() => {
                // fs-watch only triggers a redraw this leaf; nav rescan is 030.
                if maybe.is_some() {
                    needs_redraw = true;
                }
            }
        }
    }
    Ok(())
}

/// What one handled input event implies for the loop.
enum EventOutcome {
    /// Quit the loop.
    Quit,
    /// Something changed; redraw before the next event.
    Redraw,
    /// Forwarded to the pane (or ignored); no grove-surface redraw needed —
    /// the pane's own output will arrive as a render push if it changed.
    Nothing,
}

/// Apply one input event: resize is focus-independent and handled directly; all
/// other events route through the pure [`arbitrate`] table, and the returned
/// [`Action`] is carried out here (the I/O seam). Modal buffer mutation lives
/// here because the buffer is loop state, not transition state.
async fn handle_event(
    ev: &Event,
    pane: &Pane,
    leader: &Leader,
    focus: &mut Focus,
    modal_buf: &mut String,
) -> Result<EventOutcome> {
    // Resize is not a focus decision — always re-size the pane and redraw.
    if let Event::Resize(w, h) = ev {
        let _ = pane.resize(TerminalSizeSpec::new(*w, *h)).await;
        return Ok(EventOutcome::Redraw);
    }

    let (next, action) = arbitrate(focus, leader, ev);
    *focus = next;
    match action {
        Action::Ignore => Ok(EventOutcome::Nothing),
        Action::SendText(text) => {
            pane.send_text(text).await.ok();
            Ok(EventOutcome::Nothing)
        }
        Action::SendKey(token) => {
            pane.send_key(token).await.ok();
            Ok(EventOutcome::Nothing)
        }
        Action::SendPaste(text) => {
            // Wrap in bracketed-paste markers so multi-line pastes don't execute
            // line-by-line (claude multi-line input / vim paste-mode depend on it).
            pane.send_text(format!("\x1b[200~{text}\x1b[201~")).await.ok();
            Ok(EventOutcome::Nothing)
        }
        Action::HarnessClick { row, col } => {
            let _ = pane.mouse().click(row, col).await;
            Ok(EventOutcome::Nothing)
        }
        Action::Redraw => Ok(EventOutcome::Redraw),
        Action::ModalInsert(text) => {
            modal_buf.push_str(&text);
            Ok(EventOutcome::Redraw)
        }
        Action::ModalBackspace => {
            modal_buf.pop();
            Ok(EventOutcome::Redraw)
        }
        Action::ModalSubmit => {
            // 040 wires submit to grove's capture write; this leaf proves the
            // mechanics, so it just clears the buffer.
            modal_buf.clear();
            Ok(EventOutcome::Redraw)
        }
        Action::ModalCancel => {
            modal_buf.clear();
            Ok(EventOutcome::Redraw)
        }
        Action::Quit => Ok(EventOutcome::Quit),
    }
}

/// Paint one frame: the focused pane full-screen, with grove's own surfaces
/// drawn over it per [`Focus`]. The hardware cursor goes to the pane cursor when
/// the harness is focused, to the modal buffer's end when a modal is up, and is
/// hidden over the Nav stub.
fn draw(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    state: &PaneState,
    focus: &Focus,
    modal_buf: &str,
) -> Result<()> {
    terminal
        .draw(|frame| {
            let area = frame.area();
            let pane_cursor = render_pane(state, area, frame.buffer_mut());
            match focus {
                Focus::Harness => {
                    if let Some((cx, cy)) = pane_cursor {
                        frame.set_cursor_position((cx, cy));
                    }
                }
                Focus::Nav => draw_nav_indicator(frame, area),
                Focus::Modal { kind, .. } => {
                    if let Some(pos) = draw_modal(frame, area, *kind, modal_buf) {
                        frame.set_cursor_position(pos);
                    }
                }
            }
        })
        .context("drawing a frame")?;
    Ok(())
}

/// The Nav stub (030 builds the real surface): a one-line "grove focus" banner
/// across the top, listing the stub bindings.
fn draw_nav_indicator(frame: &mut ratatui::Frame, area: Rect) {
    if area.height == 0 {
        return;
    }
    let bar = Rect::new(area.x, area.y, area.width, 1);
    frame.render_widget(Clear, bar);
    frame.render_widget(
        Paragraph::new(Line::from(
            " grove (nav stub) — Esc/leader: harness · c: capture · q: quit ",
        ))
        .style(Style::default().fg(Color::Black).bg(Color::Cyan)),
        bar,
    );
}

/// The Modal stub (040 builds the real capture modal): a centered box over the
/// live pane showing the buffer. Returns where the hardware cursor belongs (end
/// of the buffer text), or `None` if it cannot be placed.
fn draw_modal(
    frame: &mut ratatui::Frame,
    area: Rect,
    kind: ModalKind,
    buf: &str,
) -> Option<(u16, u16)> {
    let popup = centered_rect(70, 50, area);
    frame.render_widget(Clear, popup);
    let title = match kind {
        ModalKind::Capture => " capture (modal stub) — Enter: submit · Esc: cancel ",
    };
    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .style(Style::default().fg(Color::Yellow));
    let inner = block.inner(popup);
    frame.render_widget(block, popup);
    frame.render_widget(
        Paragraph::new(buf).wrap(Wrap { trim: false }),
        inner,
    );
    if inner.width == 0 || inner.height == 0 {
        return None;
    }
    // Place the cursor just past the buffer text, wrapping across the inner box.
    let len = buf.chars().count() as u16;
    let cx = inner.x + (len % inner.width);
    let cy = inner.y + (len / inner.width).min(inner.height - 1);
    Some((cx, cy))
}

/// A percentage box centered in `r` (grove's historical `centered_rect`).
fn centered_rect(pct_x: u16, pct_y: u16, r: Rect) -> Rect {
    let w = r.width * pct_x / 100;
    let h = r.height * pct_y / 100;
    Rect {
        x: r.x + (r.width - w) / 2,
        y: r.y + (r.height - h) / 2,
        width: w,
        height: h,
    }
}

/// Resolve the fleet repo roots: explicit `--repo` flags + manifest (below the
/// seam), falling back to the cwd's repo when nothing is configured.
fn resolve_repo_roots(args: &TuiArgs) -> Result<Vec<PathBuf>> {
    let roots = crate::fleet::resolve(&args.repo);
    if roots.is_empty() {
        Ok(vec![crate::repo::resolve(None)?])
    } else {
        Ok(roots)
    }
}

/// Pick the single harness pane's process (E3). First live grove of the first
/// repo → `grove do <name>` in its worktree; otherwise the user's shell.
fn select_pane_process(repo_roots: &[PathBuf]) -> Result<PaneProcess> {
    let grove_exe = std::env::current_exe()
        .ok()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|| "grove".to_string());

    for repo in repo_roots {
        let view = crate::repo_view::RepoView::scan(repo)
            .with_context(|| format!("scanning repo {}", repo.display()))?;
        if let Some(grove) = view
            .groves()
            .iter()
            .find(|g| g.lifecycle == crate::repo_view::Lifecycle::Live)
        {
            return Ok(PaneProcess {
                key: grove.name.clone(),
                cwd: crate::repo::grove_worktree(repo, &grove.name),
                argv: vec![grove_exe, "do".to_string(), grove.name.clone()],
            });
        }
    }

    // No live grove anywhere: a shell so the render path still demos.
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    let cwd = repo_roots
        .first()
        .cloned()
        .unwrap_or_else(|| PathBuf::from("."));
    Ok(PaneProcess {
        key: "shell".to_string(),
        cwd,
        argv: vec![shell],
    })
}

/// Spawn a dedicated OS thread reading crossterm events and forwarding them onto
/// a channel the async loop selects over. crossterm's reader is blocking, so it
/// cannot live inside the reactor; the thread polls on a budget and checks
/// `stop` so it releases stdin promptly at teardown.
fn spawn_input_reader(stop: Arc<AtomicBool>) -> mpsc::UnboundedReceiver<Event> {
    let (tx, rx) = mpsc::unbounded_channel();
    std::thread::spawn(move || loop {
        if stop.load(Ordering::Relaxed) {
            break;
        }
        match event::poll(Duration::from_millis(100)) {
            Ok(true) => match event::read() {
                Ok(ev) => {
                    if tx.send(ev).is_err() {
                        break; // async loop gone
                    }
                }
                Err(_) => break,
            },
            Ok(false) => {} // idle; loop and re-check stop
            Err(_) => break,
        }
    });
    rx
}

/// Build the fleet fs-watch (one `notify` watcher over every repo's grove-state
/// dirs, 070 Q6) and bridge its callback into the async loop via a channel. The
/// watcher object is returned so the caller keeps it alive for the loop's
/// lifetime; dropping it stops the watch. Git-internal churn is filtered with
/// the existing `fleet` helper so `.git/` noise does not wake the loop.
fn spawn_fs_watch(
    repo_roots: &[PathBuf],
) -> Result<(RecommendedWatcher, mpsc::UnboundedReceiver<()>)> {
    let (tx, rx) = mpsc::unbounded_channel();
    let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
        if let Ok(ev) = res {
            // Ignore pure git-internal churn (below-seam fleet helper).
            if ev.paths.iter().all(|p| crate::fleet::path_is_git_internal(p)) {
                return;
            }
            let _ = tx.send(());
        }
    })
    .context("creating the fs-watch")?;

    for dir in crate::fleet::fleet_watch_dirs(repo_roots) {
        // The watch set lists dirs unconditionally; skip any not yet on disk.
        if Path::new(&dir).is_dir() {
            watcher
                .watch(&dir, RecursiveMode::Recursive)
                .with_context(|| format!("watching {}", dir.display()))?;
        }
    }
    Ok((watcher, rx))
}
