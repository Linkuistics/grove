//! The async draw loop: a ratatui app that owns its own render loop (D3) and
//! embeds harness panes, switched by the minimal [`Nav`] surface (030).
//!
//! Shape: connect/start the rmux daemon, ensure one deterministically-named
//! session, open the first harness pane (`grove do <name>` in that grove's
//! worktree, E3), then run an event-driven `tokio::select!` over three sources —
//! per-pane render-stream pushes, crossterm input, and fs-watch ticks —
//! redrawing only when something *visible* changed.
//!
//! ## Multi-pane (the 030 step up from 010)
//!
//! 010 had exactly one pane and quit when it exited. 030's [`Nav`] lets the user
//! open/focus a harness per grove, so several panes coexist: the app keeps a
//! `grove-name → `[`PaneEntry`] map (E3's `PaneId` map made load-bearing), every
//! pane's `render_stream` pushes onto **one** key-tagged channel, and only the
//! *focused* pane is drawn. Non-focused panes stay alive and keep their
//! [`PaneState`] warm (their render tasks run on), so switching focus is
//! instant. Park/close semantics are 050; here a deselected pane simply stays
//! open in the background.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use ratatui::backend::CrosstermBackend;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::crossterm::event::{
    self, DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture,
    Event,
};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::Terminal;
use ratatui_rmux::PaneState;
use rmux_sdk::{
    EnsureSession, EnsureSessionPolicy, PaneSnapshot, Rmux, Session, SessionName, TerminalSizeSpec,
};
use tokio::sync::mpsc;

use crate::cli::TuiArgs;
use crate::multi_repo_view::MultiRepoView;
use crate::tui::capture::{CaptureModal, CaptureOutcome, CaptureTarget};
use crate::tui::config::{resolve_leader, Leader};
use crate::tui::detail::Detail;
use crate::tui::driver::PaneDriver;
use crate::tui::editor;
use crate::tui::focus::{arbitrate, Action, Focus, ModalKind};
use crate::tui::nav::{Nav, NavItem};
use crate::tui::pane::render_pane;

/// One rmux session per `grove tui`, named deterministically (ADR-0027's
/// singleton). 050 revisits park-alive / multi-session lifecycle.
const SESSION_NAME: &str = "grove-fleet";

/// One embedded harness pane and its latest rendered grid. The map value behind
/// each grove key (and the `"shell"` fallback).
struct PaneEntry {
    driver: PaneDriver,
    state: PaneState,
    /// Where a capture submitted while this pane is focused is written — the
    /// pane's own grove (E1). `None` for the bare-shell fallback (no grove).
    target: Option<CaptureTarget>,
}

/// The harness pane's process + placement, chosen below the presentation seam
/// (sync core). The *first* pane is the first live grove anywhere in the fleet
/// (`grove do <name>` in its worktree), or the user's shell when the fleet has
/// no live groves so `grove tui` still launches with a live pane.
struct PaneProcess {
    /// Map key / focus label: the grove name, or `"shell"` for the fallback.
    key: String,
    cwd: PathBuf,
    argv: Vec<String>,
    /// Capture target for this pane (the grove), or `None` for the shell.
    target: Option<CaptureTarget>,
}

/// The running TUI: the daemon handles, the open-pane map, the focus machine,
/// and the nav surface + fleet data it renders from.
struct App {
    rmux: Rmux,
    session: Session,
    /// Open harness panes by key (grove name, or `"shell"`).
    panes: HashMap<String, PaneEntry>,
    /// The currently displayed harness key (the surface to return to from Nav).
    focused: String,
    /// Which surface owns input (E4).
    focus: Focus,
    nav: Nav,
    /// The per-grove detail panel (050/030), re-pointed at the focused pane's
    /// grove and rebuilt from the fleet snapshot on every fs-watch tick.
    detail: Detail,
    /// The fleet snapshot the nav renders from; re-scanned on fs-watch ticks.
    fleet: MultiRepoView,
    repo_roots: Vec<PathBuf>,
    leader: Leader,
    /// The capture modal (040): owns its text buffer and centered render.
    capture: CaptureModal,
    /// The move-target picker (050/040 grooming): a [`Nav`]-backed grove list,
    /// rebuilt from the fleet (source grove excluded) each time `m` opens it.
    move_picker: Nav,
    /// The observation being moved while the picker is up — captured when `m` is
    /// pressed, committed on Enter, cleared on cancel. `None` when no move is in
    /// flight.
    pending_move: Option<PathBuf>,
    /// A transient capture result, shown briefly after submit and cleared on the
    /// next keypress (the modal has already closed by then — E4 restores prior
    /// focus on Enter).
    toast: Option<CaptureOutcome>,
    /// Absolute path to the `grove` binary, for `grove do <name>` argv and for
    /// resolving the `grove-llm` sibling that performs the capture write.
    grove_exe: String,
    /// Last known terminal size, applied to newly opened panes.
    size: (u16, u16),
    /// The shared sender every pane's render task clones. Held here so the
    /// render channel never closes while the app lives (a single pane exiting
    /// must not end the loop).
    render_tx: mpsc::UnboundedSender<(String, PaneSnapshot)>,
}

pub async fn run_app(args: &TuiArgs) -> Result<()> {
    // --- below the seam: scan the fleet and pick the first pane (sync core) ---
    let repo_roots = resolve_repo_roots(args)?;
    let fleet = MultiRepoView::scan(&repo_roots);
    let grove_exe = grove_exe();
    let process = select_initial_process(&fleet, &repo_roots, &grove_exe);

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

    // --- the first harness pane, addressed by stable PaneId (E3) ---
    let pane = session.pane(0, 0);
    let pane_id = pane
        .id()
        .await
        .context("resolving harness pane id")?
        .context("harness pane has no id yet")?;

    // The grove-name → pane map (E3). The nav adds entries as groves are opened.
    let (render_tx, mut render_rx) = mpsc::unbounded_channel();
    let mut panes: HashMap<String, PaneEntry> = HashMap::new();
    let driver = PaneDriver::new(pane, pane_id);
    // Size the harness to the composed layout's pane share (the detail column
    // coexists beside it), not the full terminal — otherwise its grid is clipped.
    let pane_vp = composed_layout(Rect::new(0, 0, cols, rows)).pane;
    let _ = driver
        .pane()
        .resize(TerminalSizeSpec::new(pane_vp.width.max(1), pane_vp.height.max(1)))
        .await;
    driver
        .spawn_render_task(process.key.clone(), render_tx.clone())
        .await
        .context("opening the harness pane render stream")?;
    panes.insert(
        process.key.clone(),
        PaneEntry {
            driver,
            state: PaneState::default(),
            target: process.target.clone(),
        },
    );

    // --- the nav surface, selection landed on the initially-focused grove ---
    let mut nav = Nav::from_fleet(&fleet);
    nav.select(&process.key);

    // --- input + fs-watch event sources, surfaced into the select! ---
    let stop = Arc::new(AtomicBool::new(false));
    // The editor drop (D-E) pauses the reader so the `$EDITOR` child owns stdin.
    let input_pause = Arc::new(AtomicBool::new(false));
    let mut input_rx = spawn_input_reader(Arc::clone(&stop), Arc::clone(&input_pause));
    let (_watcher, mut watch_rx) = spawn_fs_watch(&repo_roots)?;

    // --- terminal setup ---
    // Bracketed paste (E5): a multi-line paste arrives as one `Event::Paste`
    // forwarded wrapped, not line-by-line keys that execute as typed. Mouse
    // capture feeds click-to-focus (E5).
    enable_raw_mode().context("enabling raw mode")?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture, EnableBracketedPaste)
        .context("entering alternate screen")?;
    let mut terminal =
        Terminal::new(CrosstermBackend::new(stdout)).context("creating the ratatui terminal")?;

    // --- the event-driven draw loop ---
    let mut app = App {
        rmux,
        session,
        panes,
        focused: process.key.clone(),
        focus: Focus::Pane,
        nav,
        detail: Detail::new(),
        fleet,
        repo_roots,
        leader,
        capture: CaptureModal::new(),
        move_picker: Nav::default(),
        pending_move: None,
        toast: None,
        grove_exe,
        size: (cols, rows),
        render_tx,
    };
    // Point the detail panel at the initially-focused grove before the first draw.
    app.rebuild_detail();
    let result = app
        .run(&mut terminal, &mut render_rx, &mut input_rx, &mut watch_rx, &input_pause)
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

impl App {
    /// The `tokio::select!` loop. Redraws only when a source reports a *visible*
    /// change (the focused pane's render push, input, or an fs-watch tick).
    /// Returns when input closes or the user quits (leader → `q`).
    async fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
        render_rx: &mut mpsc::UnboundedReceiver<(String, PaneSnapshot)>,
        input_rx: &mut mpsc::UnboundedReceiver<Event>,
        watch_rx: &mut mpsc::UnboundedReceiver<()>,
        input_pause: &Arc<AtomicBool>,
    ) -> Result<()> {
        let mut needs_redraw = true;
        loop {
            if needs_redraw {
                self.draw(terminal)?;
                needs_redraw = false;
            }
            tokio::select! {
                maybe = render_rx.recv() => match maybe {
                    Some((key, snapshot)) => {
                        if let Some(entry) = self.panes.get_mut(&key) {
                            entry.state.set_snapshot(snapshot);
                        }
                        // Only the visible pane's updates trigger a redraw;
                        // background panes (and the Nav surface, which hides the
                        // pane) stay warm without repainting.
                        if key == self.focused && surface_shows_pane(&self.focus) {
                            needs_redraw = true;
                        }
                    }
                    None => break, // every render sender gone (only at shutdown)
                },
                maybe = input_rx.recv() => match maybe {
                    Some(ev) => match self.handle_event(&ev).await? {
                        EventOutcome::Quit => break,
                        EventOutcome::Redraw => needs_redraw = true,
                        EventOutcome::OpenEditor => {
                            // The editor drop owns the terminal + stdin, so it
                            // runs here in the loop body (not in `handle_event`,
                            // which has neither). On return, force a full repaint.
                            self.open_in_editor(terminal, input_pause, input_rx).await;
                            needs_redraw = true;
                        }
                        EventOutcome::Nothing => {}
                    },
                    None => break, // input reader gone
                },
                maybe = watch_rx.recv() => {
                    // fs-watch refreshes the nav list (groves appearing/retiring).
                    if maybe.is_some() {
                        self.refresh_fleet();
                        needs_redraw = true;
                    }
                }
            }
        }
        Ok(())
    }

    /// Apply one input event: resize is focus-independent (re-size every open
    /// pane); everything else routes through the pure [`arbitrate`] table and
    /// the returned [`Action`] is carried out here (the I/O seam).
    async fn handle_event(&mut self, ev: &Event) -> Result<EventOutcome> {
        if let Event::Resize(w, h) = ev {
            self.size = (*w, *h);
            // Panes coexist with the detail column, so they are sized to the
            // composed pane share, not the full terminal.
            let (pw, ph) = self.pane_viewport();
            for entry in self.panes.values() {
                let _ = entry.driver.pane().resize(TerminalSizeSpec::new(pw, ph)).await;
            }
            return Ok(EventOutcome::Redraw);
        }

        // A capture toast shows until the next keypress dismisses it ("briefly").
        // Dismissing forces a redraw even when the key itself implies none (e.g.
        // a char forwarded to the harness), so a stale toast never lingers.
        let dismissed_toast = matches!(ev, Event::Key(_)) && self.toast.take().is_some();

        let (next, action) = arbitrate(&self.focus, &self.leader, ev);
        self.focus = next;
        let outcome = match action {
            Action::Ignore => EventOutcome::Nothing,
            Action::SendText(text) => {
                if let Some(pane) = self.focused_pane() {
                    pane.send_text(text).await.ok();
                }
                EventOutcome::Nothing
            }
            Action::SendKey(token) => {
                if let Some(pane) = self.focused_pane() {
                    pane.send_key(token).await.ok();
                }
                EventOutcome::Nothing
            }
            Action::SendPaste(text) => {
                // Wrap in bracketed-paste markers so multi-line pastes don't
                // execute line-by-line (claude multi-line / vim paste-mode).
                if let Some(pane) = self.focused_pane() {
                    pane.send_text(format!("\x1b[200~{text}\x1b[201~")).await.ok();
                }
                EventOutcome::Nothing
            }
            Action::HarnessClick { row, col } => {
                if let Some(pane) = self.focused_pane() {
                    let _ = pane.mouse().click(row, col).await;
                }
                EventOutcome::Nothing
            }
            Action::NavUp => {
                self.nav.select_up();
                EventOutcome::Redraw
            }
            Action::NavDown => {
                self.nav.select_down();
                EventOutcome::Redraw
            }
            Action::NavSelect => {
                if let Some(item) = self.nav.selected().cloned() {
                    if let Err(e) = self.open_or_focus(&item).await {
                        // Opening must never kill the TUI; fall back to the
                        // prior harness. (No stderr — we're on the alt screen.)
                        let _ = e;
                    }
                }
                // The focused grove changed; re-point the coexisting detail panel.
                self.rebuild_detail();
                EventOutcome::Redraw
            }
            Action::DetailUp => {
                self.detail.nav_up();
                EventOutcome::Redraw
            }
            Action::DetailDown => {
                self.detail.nav_down();
                EventOutcome::Redraw
            }
            Action::DetailReject => {
                self.reject_selected().await;
                EventOutcome::Redraw
            }
            Action::DetailMove => {
                // arbitrate already flipped focus to the picker modal; open it for
                // real (populate + remember the obs), or revert if there is
                // nothing to move / nowhere to move it.
                self.begin_move();
                EventOutcome::Redraw
            }
            Action::MovePickerUp => {
                self.move_picker.select_up();
                EventOutcome::Redraw
            }
            Action::MovePickerDown => {
                self.move_picker.select_down();
                EventOutcome::Redraw
            }
            Action::MovePickerSelect => {
                self.commit_move().await;
                EventOutcome::Redraw
            }
            Action::Redraw => EventOutcome::Redraw,
            Action::ModalInsert(text) => {
                self.capture.insert(&text);
                EventOutcome::Redraw
            }
            Action::ModalBackspace => {
                self.capture.backspace();
                EventOutcome::Redraw
            }
            Action::ModalSubmit => {
                self.submit_capture().await;
                EventOutcome::Redraw
            }
            Action::ModalCancel => {
                // One cancel path for both modals: discard the capture buffer and
                // any in-flight move (whichever modal was up).
                self.capture.clear();
                self.pending_move = None;
                EventOutcome::Redraw
            }
            Action::OpenEditor => EventOutcome::OpenEditor,
            Action::Quit => EventOutcome::Quit,
        };

        // If we dismissed a toast on a key that otherwise implies no redraw,
        // upgrade so the toast actually disappears from the screen.
        Ok(match (dismissed_toast, outcome) {
            (true, EventOutcome::Nothing) => EventOutcome::Redraw,
            (_, outcome) => outcome,
        })
    }

    /// Perform the capture write for the focused pane's grove (E1): take the
    /// modal buffer, shell out to `grove-llm inbox-add` under `spawn_blocking`
    /// (it commits + best-effort pushes — must not stall the reactor), and stash
    /// the result as a toast. A no-op for an empty buffer or the shell pane.
    async fn submit_capture(&mut self) {
        let body = self.capture.take();
        if body.trim().is_empty() {
            return;
        }
        let Some(target) = self.panes.get(&self.focused).and_then(|e| e.target.clone()) else {
            self.toast = Some(CaptureOutcome::Failed("no grove focused".into()));
            return;
        };
        let name = target.name.clone();
        let exe = self.grove_exe.clone();
        let result = tokio::task::spawn_blocking(move || {
            crate::tui::capture::write_capture(&exe, &target, &body)
        })
        .await;
        self.toast = Some(match result {
            Ok(Ok(())) => CaptureOutcome::Captured(name),
            Ok(Err(e)) => CaptureOutcome::Failed(format!("{e:#}")),
            Err(e) => CaptureOutcome::Failed(format!("capture task panicked: {e}")),
        });
    }

    /// Reject the detail panel's selected inbox observation (040 grooming):
    /// shell out to `grove-llm inbox-drain --rejected` under `spawn_blocking`
    /// (commit + best-effort push must not stall the reactor, E1), toast the
    /// result, and re-scan so the shrunk inbox view refreshes. A no-op when no
    /// observation is selected (empty inbox) or the pane has no grove.
    async fn reject_selected(&mut self) {
        let Some(obs) = self.detail.selected_observation().cloned() else {
            return;
        };
        let Some(target) = self.panes.get(&self.focused).and_then(|e| e.target.clone()) else {
            self.toast = Some(CaptureOutcome::Failed("no grove focused".into()));
            return;
        };
        let exe = self.grove_exe.clone();
        let result = tokio::task::spawn_blocking(move || {
            crate::tui::capture::reject_observation(&exe, &target, &obs)
        })
        .await;
        self.toast = Some(match result {
            Ok(Ok(())) => CaptureOutcome::Rejected,
            Ok(Err(e)) => CaptureOutcome::Failed(format!("reject failed: {e:#}")),
            Err(e) => CaptureOutcome::Failed(format!("reject task panicked: {e}")),
        });
        // The inbox shrank; re-scan so the detail view drops the rejected row.
        self.refresh_fleet();
    }

    /// Open the move-target picker for the selected observation (the `m` key).
    /// arbitrate already set the focus to the picker modal; here we either arm it
    /// (snapshot the observation + build the grove list with the source excluded)
    /// or, when there is nothing to move or nowhere to move it, revert the focus
    /// back to Detail so no empty picker is shown.
    fn begin_move(&mut self) {
        let Some(obs) = self.detail.selected_observation().cloned() else {
            self.focus = Focus::Detail; // empty inbox — nothing selected
            return;
        };
        let Some(source) = self.panes.get(&self.focused).and_then(|e| e.target.clone()) else {
            self.focus = Focus::Detail;
            self.toast = Some(CaptureOutcome::Failed("no grove focused".into()));
            return;
        };
        let mut picker = Nav::from_fleet(&self.fleet);
        picker.remove(&source.repo_root, &source.name);
        if picker.selected().is_none() {
            self.focus = Focus::Detail;
            self.toast = Some(CaptureOutcome::Failed("no other grove to move to".into()));
            return;
        }
        self.move_picker = picker;
        self.pending_move = Some(obs);
    }

    /// Commit the move/re-route (Enter in the picker): copy the pending
    /// observation into the picker's selected grove then drop it from the source
    /// inbox, both under `spawn_blocking` (E1). Toast the result and re-scan so
    /// the source inbox view refreshes. Focus has already returned to Detail.
    async fn commit_move(&mut self) {
        let Some(obs) = self.pending_move.take() else {
            return;
        };
        let Some(dest_item) = self.move_picker.selected().cloned() else {
            self.toast = Some(CaptureOutcome::Failed("no target grove selected".into()));
            return;
        };
        let Some(source) = self.panes.get(&self.focused).and_then(|e| e.target.clone()) else {
            self.toast = Some(CaptureOutcome::Failed("no grove focused".into()));
            return;
        };
        let dest = CaptureTarget {
            name: dest_item.name.clone(),
            repo_root: dest_item.repo_root.clone(),
        };
        let dest_name = dest.name.clone();
        let exe = self.grove_exe.clone();
        let result = tokio::task::spawn_blocking(move || {
            crate::tui::capture::move_observation(&exe, &source, &dest, &obs)
        })
        .await;
        self.toast = Some(match result {
            Ok(Ok(())) => CaptureOutcome::Moved(dest_name),
            Ok(Err(e)) => CaptureOutcome::Failed(format!("move failed: {e:#}")),
            Err(e) => CaptureOutcome::Failed(format!("move task panicked: {e}")),
        });
        // The source inbox shrank; re-scan so the detail view drops the moved row.
        self.refresh_fleet();
    }

    /// The open-in-editor drop (040, D-B/D-E): capture the focused harness
    /// pane's full *rendered* history via stock `rmux capture-pane`, suspend the
    /// TUI + pause the input reader so the `$EDITOR` child owns the terminal,
    /// then restore and force a full repaint. A no-op-with-toast on the bare
    /// shell pane (no grove). Every failure surfaces as a toast and never
    /// crashes the loop (mirrors [`Self::submit_capture`]).
    async fn open_in_editor(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
        input_pause: &Arc<AtomicBool>,
        input_rx: &mut mpsc::UnboundedReceiver<Event>,
    ) {
        // Gate on harness-ness (a grove target); the bare shell is a no-op with a
        // toast per the leaf's "Done when". Extract what we need and drop the
        // borrow before touching `self.toast`.
        let (pane_id, is_harness) = match self.panes.get(&self.focused) {
            Some(e) => (e.driver.id(), e.target.is_some()),
            None => return,
        };
        if !is_harness {
            self.toast = Some(CaptureOutcome::Failed("no harness focused".into()));
            return;
        }

        // 1. Capture the rendered history below the seam (spawn_blocking — it
        //    talks to the daemon over IPC; keep the reactor free, E1).
        let dump = match tokio::task::spawn_blocking(move || editor::capture_history(pane_id)).await
        {
            Ok(Ok(text)) => text,
            Ok(Err(e)) => {
                self.toast = Some(CaptureOutcome::Failed(format!("capture failed: {e:#}")));
                return;
            }
            Err(e) => {
                self.toast = Some(CaptureOutcome::Failed(format!("capture task panicked: {e}")));
                return;
            }
        };

        // 2. Stage the dump in a temp file for the editor to open.
        let tmp = match write_dump_tempfile(&dump) {
            Ok(t) => t,
            Err(e) => {
                self.toast = Some(CaptureOutcome::Failed(format!("temp file: {e:#}")));
                return;
            }
        };

        // 3. Suspend the TUI and pause the input reader so the editor owns stdin
        //    (D-E). Sleep one reader poll-budget so any in-flight poll finishes
        //    before the child takes over.
        input_pause.store(true, Ordering::Relaxed);
        tokio::time::sleep(Duration::from_millis(150)).await;
        let _ = disable_raw_mode();
        let _ = execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture,
            DisableBracketedPaste
        );
        let _ = terminal.show_cursor();

        // 4. Run `$EDITOR <tmpfile>` blocking, inheriting the real terminal. On a
        //    blocking-pool thread so the reactor's render/watch tasks keep
        //    draining into their channels while the editor is up.
        let mut argv = editor::resolve_editor();
        argv.push(tmp.path().to_string_lossy().into_owned());
        let status = tokio::task::spawn_blocking(move || {
            std::process::Command::new(&argv[0]).args(&argv[1..]).status()
        })
        .await;

        // 5. Restore the TUI (setup order) and resume input.
        let _ = enable_raw_mode();
        let _ = execute!(
            terminal.backend_mut(),
            EnterAlternateScreen,
            EnableMouseCapture,
            EnableBracketedPaste
        );
        let _ = terminal.hide_cursor();
        input_pause.store(false, Ordering::Relaxed);

        // 6. Drop any keystrokes that slipped through at the boundary and clear,
        //    so the caller's redraw fully repaints over the editor's scribbles.
        while input_rx.try_recv().is_ok() {}
        let _ = terminal.clear();

        // 7. A clean editor exit is silent; anything else becomes a toast.
        match status {
            Ok(Ok(st)) if st.success() => {}
            Ok(Ok(st)) => self.toast = Some(CaptureOutcome::Failed(format!("editor exited {st}"))),
            Ok(Err(e)) => self.toast = Some(CaptureOutcome::Failed(format!("editor: {e}"))),
            Err(e) => {
                self.toast = Some(CaptureOutcome::Failed(format!("editor task panicked: {e}")))
            }
        }
    }

    /// A clone of the focused pane's handle, if one is open. Cloning (the rmux
    /// `Pane` handle is cheap to clone) keeps the `panes`/`focused` borrow short,
    /// so the input forward `.await` doesn't hold a borrow of `self` across it.
    fn focused_pane(&self) -> Option<rmux_sdk::Pane> {
        self.panes.get(&self.focused).map(|e| e.driver.pane().clone())
    }

    /// Open the selected grove's harness pane if absent, else focus the existing
    /// one — the no-duplicate-pane guarantee (E3's `grove-name → PaneId` map).
    /// On open, a new detached window runs `grove do <name>` in the worktree.
    async fn open_or_focus(&mut self, item: &NavItem) -> Result<()> {
        if self.panes.contains_key(&item.name) {
            self.focused = item.name.clone(); // focus the already-open pane
            return Ok(());
        }

        let cwd = crate::repo::grove_worktree(&item.repo_root, &item.name);
        let argv = vec![self.grove_exe.clone(), "do".to_string(), item.name.clone()];
        let window = self
            .session
            .new_window_with()
            .name(&item.name)
            .spawn(argv)
            .cwd(cwd)
            .detached(true)
            .await
            .context("opening a harness window")?;
        let wp = window
            .panes()
            .await
            .context("listing the new window's panes")?
            .into_iter()
            .next()
            .context("new harness window has no pane")?;
        let pane = self
            .rmux
            .pane(wp.target.clone())
            .await
            .context("resolving the new harness pane")?;
        let driver = PaneDriver::new(pane, wp.id);
        let (pw, ph) = self.pane_viewport();
        let _ = driver.pane().resize(TerminalSizeSpec::new(pw, ph)).await;
        driver
            .spawn_render_task(item.name.clone(), self.render_tx.clone())
            .await
            .context("opening the harness pane render stream")?;
        self.panes.insert(
            item.name.clone(),
            PaneEntry {
                driver,
                state: PaneState::default(),
                target: Some(CaptureTarget {
                    name: item.name.clone(),
                    repo_root: item.repo_root.clone(),
                }),
            },
        );
        self.focused = item.name.clone();
        Ok(())
    }

    /// Re-scan the fleet and rebuild the nav (groves appearing/retiring),
    /// keeping the selection on the same grove where it survives. Uses the
    /// warning-collecting scan so no stderr noise corrupts the alt screen.
    fn refresh_fleet(&mut self) {
        let (fleet, _warnings) = MultiRepoView::scan_with_warnings(&self.repo_roots);
        self.fleet = fleet;
        self.nav.rebuild(&self.fleet);
        // The focused grove's task tree / inbox may have changed (a leaf retired,
        // an observation captured); re-point detail off the fresh scan too.
        self.rebuild_detail();
    }

    /// Point the detail panel at the focused pane's grove, using the current
    /// fleet snapshot. The bare-shell pane (no grove target) shows the empty
    /// "no grove" state. Re-pointing to a different grove resets detail's scroll;
    /// refreshing the same grove preserves it (see [`Detail::show`]).
    fn rebuild_detail(&mut self) {
        let target = self.panes.get(&self.focused).and_then(|e| e.target.clone());
        match target {
            Some(t) => {
                let grove = self
                    .fleet
                    .repos()
                    .iter()
                    .find(|r| r.repo_root == t.repo_root)
                    .and_then(|r| r.grove(&t.name));
                self.detail.show(Some(&t.name), grove);
            }
            None => self.detail.show(None, None),
        }
    }

    /// The harness pane's size under the composed layout (the dominant left share
    /// beside the detail column), clamped to at least 1×1 for a degenerate
    /// terminal. Every pane is resized to this, not the full terminal.
    fn pane_viewport(&self) -> (u16, u16) {
        let pane = composed_layout(Rect::new(0, 0, self.size.0, self.size.1)).pane;
        (pane.width.max(1), pane.height.max(1))
    }

    /// Paint one frame for the current [`Focus`]: the surface, then the whichkey
    /// footer (leader menu when pending, the surface's hint line otherwise), then
    /// any transient capture toast on top.
    ///
    /// Whichkey is a single footer the `App` draws (050/010 verdict): one draw
    /// loop, one footer, so ADR-0019's single-hint-owner holds by construction.
    fn draw(
        &self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    ) -> Result<()> {
        terminal
            .draw(|frame| {
                let area = frame.area();
                // The surface a `LeaderPending` gate sits over is the one we
                // leadered from, so render *that* surface behind the menu footer.
                let surface = match &self.focus {
                    Focus::LeaderPending { prior } => prior.as_ref(),
                    other => other,
                };
                if let Some(pos) = self.render_surface(surface, area, frame.buffer_mut()) {
                    frame.set_cursor_position(pos);
                }
                // The whichkey footer rides the bottom row (overlay, like the
                // toast — proper row-reservation is 050's composed layout).
                crate::tui::footer::render_footer(
                    &self.focus,
                    &self.leader,
                    area,
                    frame.buffer_mut(),
                );
                // The capture toast rides on the same bottom row, briefly over
                // the footer (it appears after the modal has already closed).
                if let Some(outcome) = &self.toast {
                    crate::tui::capture::render_toast(outcome, area, frame.buffer_mut());
                }
            })
            .context("drawing a frame")?;
        Ok(())
    }

    /// Render one focus surface into `buf`, returning where the hardware cursor
    /// belongs (the pane's cursor, or the modal's text caret), or `None`. Never
    /// receives [`Focus::LeaderPending`] — the caller unwraps that to its `prior`.
    ///
    /// The composed surfaces ([`Focus::Pane`]/[`Focus::Detail`], and the modal
    /// drawn over them) tile the content region into the harness pane (left) and
    /// the coexisting detail column (right) via [`composed_layout`]; [`Focus::Nav`]
    /// is a flip-to full surface that hides the pair.
    fn render_surface(&self, focus: &Focus, area: Rect, buf: &mut Buffer) -> Option<(u16, u16)> {
        match focus {
            // Pane and Detail coexist: render the harness in the left share and the
            // detail column on the right, accenting whichever holds focus. The
            // hardware cursor shows only when the *pane* is focused (detail is a
            // scroll list with no text caret).
            Focus::Pane | Focus::Detail => {
                let layout = composed_layout(area);
                let cursor = self
                    .panes
                    .get(&self.focused)
                    .and_then(|entry| render_pane(&entry.state, layout.pane, buf));
                self.detail
                    .render(layout.detail, buf, matches!(focus, Focus::Detail));
                if matches!(focus, Focus::Pane) {
                    cursor
                } else {
                    None
                }
            }
            Focus::Nav => {
                self.nav.render(area, buf);
                None
            }
            Focus::Modal { kind, .. } => {
                // The landmark proof point: draw the composed layout (pane + detail),
                // then the centered modal *over* it (Clear punches the hole).
                let layout = composed_layout(area);
                if let Some(entry) = self.panes.get(&self.focused) {
                    render_pane(&entry.state, layout.pane, buf);
                }
                self.detail.render(layout.detail, buf, false);
                match kind {
                    ModalKind::Capture => {
                        let label = self
                            .panes
                            .get(&self.focused)
                            .and_then(|e| e.target.as_ref())
                            .map(|t| t.name.as_str())
                            .unwrap_or("(no grove)");
                        self.capture.render(area, buf, label)
                    }
                    ModalKind::MovePicker => {
                        // The move-target picker: the fleet's grove list (source
                        // excluded) reused as a centered overlay (Nav clears its
                        // own rect). j/k select · ⏎ send · ⎋ cancel.
                        let popup = crate::tui::capture::centered_rect(60, 60, area);
                        self.move_picker.render(popup, buf);
                        None
                    }
                }
            }
            // Unwrapped by the caller; rendering its `prior` instead.
            Focus::LeaderPending { .. } => None,
        }
    }
}

/// The composed-layout rects (050/030): the harness pane (dominant left share),
/// the detail panel (a side column on the right), and the reserved footer row.
/// The harness pane and detail panel **coexist** — focus moves laterally between
/// them — which is the 050/010-surfaces verdict the trellis flip-to model lacked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ComposedLayout {
    pane: Rect,
    detail: Rect,
    footer: Rect,
}

/// The detail side column's width as a fraction of the content region, capped so
/// it stays a *column* on wide terminals and the harness keeps the dominant
/// share. The full responsive tiers + aux-pane placement are 050; this is the
/// minimal split that proves coexistence + lateral focus.
fn detail_column_width(content_width: u16) -> u16 {
    (content_width.saturating_mul(34) / 100).min(48)
}

/// Split `area` into the composed layout: a reserved bottom footer row, then the
/// content region divided into the harness pane (left, dominant) and the detail
/// column (right). Degrades without panic on a tiny area.
fn composed_layout(area: Rect) -> ComposedLayout {
    let footer_h = area.height.min(1);
    let content_h = area.height - footer_h;
    let footer = Rect::new(area.x, area.y + content_h, area.width, footer_h);
    let content = Rect::new(area.x, area.y, area.width, content_h);

    let detail_w = detail_column_width(content.width);
    let pane_w = content.width - detail_w;
    let pane = Rect::new(content.x, content.y, pane_w, content.height);
    let detail = Rect::new(content.x + pane_w, content.y, detail_w, content.height);
    ComposedLayout {
        pane,
        detail,
        footer,
    }
}

/// Whether the focused pane is visible on the current surface (so a background
/// render push should trigger a repaint). Nav hides the pane; a gate inherits its
/// `prior`'s visibility.
fn surface_shows_pane(focus: &Focus) -> bool {
    match focus {
        Focus::Pane | Focus::Detail | Focus::Modal { .. } => true,
        Focus::Nav => false,
        Focus::LeaderPending { prior } => surface_shows_pane(prior),
    }
}

/// What one handled input event implies for the loop.
enum EventOutcome {
    /// Quit the loop.
    Quit,
    /// Something changed; redraw before the next event.
    Redraw,
    /// Suspend the loop and dump the focused harness pane into `$EDITOR` (040).
    /// Carried out by the loop body, which owns the terminal + input stream.
    OpenEditor,
    /// Forwarded to the pane (or ignored); no grove-surface redraw needed —
    /// the pane's own output will arrive as a render push if it changed.
    Nothing,
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

/// Stage the captured history `dump` in a uniquely-named temp file for the
/// editor to open. Returned as a [`tempfile::NamedTempFile`] so the file is
/// cleaned up when the handle drops — which the caller holds until *after* the
/// editor has exited.
fn write_dump_tempfile(dump: &str) -> Result<tempfile::NamedTempFile> {
    use std::io::Write;
    let mut tmp = tempfile::Builder::new()
        .prefix("grove-history-")
        .suffix(".txt")
        .tempfile()
        .context("creating the history temp file")?;
    tmp.write_all(dump.as_bytes())
        .context("writing the history temp file")?;
    tmp.flush().context("flushing the history temp file")?;
    Ok(tmp)
}

/// Absolute path to the running `grove` binary, for `grove do <name>` argv.
fn grove_exe() -> String {
    std::env::current_exe()
        .ok()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|| "grove".to_string())
}

/// Pick the first pane's process (E3) from an already-scanned fleet. First live
/// grove anywhere → `grove do <name>` in its worktree; otherwise the user's
/// shell so the render path still demos.
fn select_initial_process(
    fleet: &MultiRepoView,
    repo_roots: &[PathBuf],
    grove_exe: &str,
) -> PaneProcess {
    for repo in fleet.repos() {
        if let Some(grove) = repo
            .groves()
            .iter()
            .find(|g| g.lifecycle == crate::repo_view::Lifecycle::Live)
        {
            return PaneProcess {
                key: grove.name.clone(),
                cwd: crate::repo::grove_worktree(&repo.repo_root, &grove.name),
                argv: vec![grove_exe.to_string(), "do".to_string(), grove.name.clone()],
                target: Some(CaptureTarget {
                    name: grove.name.clone(),
                    repo_root: repo.repo_root.clone(),
                }),
            };
        }
    }

    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    let cwd = repo_roots
        .first()
        .cloned()
        .unwrap_or_else(|| PathBuf::from("."));
    PaneProcess {
        key: "shell".to_string(),
        cwd,
        argv: vec![shell],
        target: None,
    }
}

/// Spawn a dedicated OS thread reading crossterm events and forwarding them onto
/// a channel the async loop selects over. crossterm's reader is blocking, so it
/// cannot live inside the reactor; the thread polls on a budget and checks
/// `stop` so it releases stdin promptly at teardown.
///
/// `pause` (D-E) lets the open-in-editor drop hand stdin to the `$EDITOR` child:
/// while set, the thread does **not** poll/read the terminal, so it never races
/// the editor for keystrokes. It resumes (and the screen repaints) on restore.
fn spawn_input_reader(stop: Arc<AtomicBool>, pause: Arc<AtomicBool>) -> mpsc::UnboundedReceiver<Event> {
    let (tx, rx) = mpsc::unbounded_channel();
    std::thread::spawn(move || loop {
        if stop.load(Ordering::Relaxed) {
            break;
        }
        if pause.load(Ordering::Relaxed) {
            // Paused for the editor drop: don't touch stdin; the child owns it.
            std::thread::sleep(Duration::from_millis(20));
            continue;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composed_layout_reserves_the_bottom_footer_row() {
        let area = Rect::new(0, 0, 120, 40);
        let layout = composed_layout(area);
        assert_eq!(layout.footer, Rect::new(0, 39, 120, 1));
        // pane + detail occupy the content region above the footer.
        assert_eq!(layout.pane.height, 39);
        assert_eq!(layout.detail.height, 39);
    }

    #[test]
    fn composed_layout_places_detail_as_a_right_side_column() {
        let area = Rect::new(0, 0, 120, 40);
        let layout = composed_layout(area);
        // Detail abuts the pane on the right; together they tile the full width.
        assert_eq!(layout.detail.x, layout.pane.x + layout.pane.width);
        assert_eq!(layout.pane.width + layout.detail.width, 120);
        // The harness keeps the dominant share.
        assert!(layout.pane.width > layout.detail.width, "{layout:?}");
    }

    #[test]
    fn composed_layout_caps_the_detail_column_on_wide_terminals() {
        let layout = composed_layout(Rect::new(0, 0, 400, 50));
        assert_eq!(layout.detail.width, 48, "detail stays a column, not a half");
    }

    #[test]
    fn composed_layout_degrades_without_panic_on_a_tiny_area() {
        // Zero-height and one-row areas must not underflow.
        let _ = composed_layout(Rect::new(0, 0, 0, 0));
        let l = composed_layout(Rect::new(0, 0, 10, 1));
        assert_eq!(l.footer.height, 1);
        assert_eq!(l.pane.height, 0);
    }
}
