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
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers,
};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::Terminal;
use ratatui_rmux::PaneState;
use rmux_sdk::{EnsureSession, EnsureSessionPolicy, Pane, Rmux, SessionName, TerminalSizeSpec};
use tokio::sync::mpsc;

use crate::cli::TuiArgs;
use crate::tui::driver::PaneDriver;
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
    // Mouse capture is plumbed in now (setup/teardown); mouse *handling* —
    // click-to-focus / passthrough — is 020 (E5), so events are ignored here.
    enable_raw_mode().context("enabling raw mode")?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
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
    )
    .await;

    // --- teardown (always, even on error) ---
    stop.store(true, Ordering::Relaxed);
    let _ = disable_raw_mode();
    let _ = execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture);
    let _ = terminal.show_cursor();
    result
}

/// The `tokio::select!` loop. Redraws only when a source reports a change
/// (render push, input, or fs-watch tick). Returns when the pane exits, input
/// closes, or the user quits (Ctrl-Q).
async fn event_loop(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    render_rx: &mut mpsc::UnboundedReceiver<rmux_sdk::PaneSnapshot>,
    input_rx: &mut mpsc::UnboundedReceiver<Event>,
    watch_rx: &mut mpsc::UnboundedReceiver<()>,
    pane: &Pane,
) -> Result<()> {
    let mut state = PaneState::default();
    let mut needs_redraw = true;
    loop {
        if needs_redraw {
            draw(terminal, &state)?;
            needs_redraw = false;
        }
        tokio::select! {
            maybe = render_rx.recv() => match maybe {
                Some(snapshot) => { state.set_snapshot(snapshot); needs_redraw = true; }
                None => break, // render task ended: the pane's process exited
            },
            maybe = input_rx.recv() => match maybe {
                Some(ev) => {
                    if handle_input(&ev, pane).await? {
                        break; // quit
                    }
                    needs_redraw = true;
                }
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

/// Paint one frame: the focused pane full-screen, with the hardware cursor
/// placed at the pane's cursor cell.
fn draw(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    state: &PaneState,
) -> Result<()> {
    terminal
        .draw(|frame| {
            let area = frame.area();
            if let Some((cx, cy)) = render_pane(state, area, frame.buffer_mut()) {
                frame.set_cursor_position((cx, cy));
            }
        })
        .context("drawing a frame")?;
    Ok(())
}

/// Handle one input event. This leaf forwards *nothing* to the pane (020 owns
/// the key-map and focus model); it only quits on Ctrl-Q and forwards resize.
/// Returns `true` to quit.
async fn handle_input(ev: &Event, pane: &Pane) -> Result<bool> {
    match ev {
        Event::Key(key)
            if key.code == KeyCode::Char('q')
                && key.modifiers.contains(KeyModifiers::CONTROL) =>
        {
            Ok(true)
        }
        Event::Resize(w, h) => {
            let _ = pane.resize(TerminalSizeSpec::new(*w, *h)).await;
            Ok(false)
        }
        _ => Ok(false),
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
