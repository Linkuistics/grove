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

use std::collections::{HashMap, HashSet};
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
use crate::tui::filter_mode::FilterMode;
use crate::tui::focus::{arbitrate, Action, DetailOrigin, Focus, ModalKind};
use crate::tui::launch;
use crate::repo_view::Lifecycle;
use crate::tui::nav::{Nav, NavItem, NavRow};
use crate::tui::pane::{render_pane, PaneKey, PaneRole};

/// One rmux session per `grove tui`, named deterministically (ADR-0027's
/// singleton). 050 revisits park-alive / multi-session lifecycle.
const SESSION_NAME: &str = "grove-fleet";

/// One embedded pane and its latest rendered grid. The map value behind each
/// composite `(grove, role)` key (and the [`PaneKey::Shell`] fallback).
struct PaneEntry {
    driver: PaneDriver,
    state: PaneState,
    /// Where a capture submitted while this pane is focused is written — the
    /// pane's own grove (E1). `None` for the bare-shell fallback (no grove).
    target: Option<CaptureTarget>,
}

/// The aux roles in side-column stack order (Q5: term → yazi → vcs). Detail is
/// the always-present top member (slot 0); these fill the slots below it in this
/// fixed order, independent of the order the user toggled them on.
const AUX_STACK_ORDER: [PaneRole; 3] = [PaneRole::Term, PaneRole::Yazi, PaneRole::Vcs];

/// Per-grove aux-pane visibility — the ephemeral working-set toggle state (Q6).
/// Which aux roles are shown is tracked per grove and restored when you switch
/// back; it dies with the session (grove constraint 1, no persisted state).
///
/// Pure set-membership over `(grove, role)` with a fixed canonical stack order —
/// no panes, no daemon — so the toggle transitions are unit-testable on their
/// own (the spawn/focus I/O lives in [`App::toggle_aux`]).
#[derive(Default)]
struct AuxVisibility {
    by_grove: HashMap<String, HashSet<PaneRole>>,
}

impl AuxVisibility {
    /// Whether `grove`'s `role` aux pane is currently toggled on.
    fn is_visible(&self, grove: &str, role: PaneRole) -> bool {
        self.by_grove.get(grove).is_some_and(|s| s.contains(&role))
    }

    /// Mark `grove`'s `role` aux pane shown (idempotent).
    fn show(&mut self, grove: &str, role: PaneRole) {
        self.by_grove.entry(grove.to_string()).or_default().insert(role);
    }

    /// Mark `grove`'s `role` aux pane hidden — the pty stays warm (hide-not-
    /// close, Q3); this only stops it being drawn (idempotent).
    fn hide(&mut self, grove: &str, role: PaneRole) {
        if let Some(s) = self.by_grove.get_mut(grove) {
            s.remove(&role);
        }
    }

    /// `grove`'s visible aux roles in side-column stack order (term → yazi →
    /// vcs), independent of the order they were toggled on (Q5: fixed position).
    fn visible_roles(&self, grove: &str) -> Vec<PaneRole> {
        let Some(set) = self.by_grove.get(grove) else {
            return Vec::new();
        };
        AUX_STACK_ORDER.into_iter().filter(|r| set.contains(r)).collect()
    }
}

/// The harness pane's process + placement, chosen below the presentation seam
/// (sync core). The *first* pane is the first live grove anywhere in the fleet
/// (`grove do <name>` in its worktree), or the user's shell when the fleet has
/// no live groves so `grove tui` still launches with a live pane.
struct PaneProcess {
    /// Map key / focus identity: the grove's harness role, or [`PaneKey::Shell`]
    /// for the no-grove fallback.
    key: PaneKey,
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
    /// Open panes by composite `(grove, role)` key (or [`PaneKey::Shell`]).
    panes: HashMap<PaneKey, PaneEntry>,
    /// The currently displayed pane's key (the surface to return to from Nav).
    /// May be a grove's harness, the bare shell, or — with focus-follow — one of
    /// that grove's aux panes (050/030). The big-left pane is always the grove's
    /// *harness* ([`Self::left_pane_key`]); `focused` is the input/cursor target.
    focused: PaneKey,
    /// Per-grove aux-pane visibility — the ephemeral working-set toggle state
    /// (Q6). Drives the side column's membership and is restored when you switch
    /// back to a grove; it dies with the session (grove constraint 1).
    aux: AuxVisibility,
    /// Which surface owns input (E4).
    focus: Focus,
    nav: Nav,
    /// The `/` filter mode (050): the live criteria + ranked cursor the nav
    /// switches to whenever any dimension is engaged. A separate selection from
    /// `nav`'s, over the flat ranked list the 040 engine projects. Ephemeral per
    /// session (constraint 1).
    filter: FilterMode,
    /// The per-grove detail panel (050/030), rebuilt from the fleet snapshot
    /// before every paint and re-pointed at its current
    /// [`subject`](App::detail_subject): the focused pane's grove during pane
    /// work, or the nav-*highlighted* grove while the nav owns focus (060 live
    /// preview).
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
    /// The seed armed while the [`ModalKind::Confirm`] prompt is up (070) —
    /// captured when `Enter` lands on a seed row, spawned on `y`/`Enter`, dropped
    /// on cancel. `None` when no seed-start is in flight.
    pending_start: Option<NavItem>,
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
    render_tx: mpsc::UnboundedSender<(PaneKey, PaneSnapshot)>,
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
    let rmux = match Rmux::builder()
        .default_timeout(Duration::from_secs(10))
        .connect_or_start()
        .await
    {
        Ok(rmux) => rmux,
        // First-run failure: when no `rmux` resolves anywhere (env var → bundled
        // sibling → PATH), the raw transport error is opaque. Point the user at
        // the bundled daemon / the override var instead (ADR-0030 §3). A daemon
        // already running is reachable over its socket without a local binary, so
        // we only swap in the hint when nothing resolves.
        Err(e) if !launch::daemon_binary_resolves() => {
            return Err(e).context(launch::DAEMON_NOT_FOUND_HINT);
        }
        Err(e) => return Err(e).context("connecting to / starting the rmux daemon"),
    };
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

    // The (grove, role) → pane map (E3). The nav adds entries as groves are opened.
    let (render_tx, mut render_rx) = mpsc::unbounded_channel();
    let mut panes: HashMap<PaneKey, PaneEntry> = HashMap::new();
    let driver = PaneDriver::new(pane, pane_id);
    // Size the harness to the composed layout's pane share (the side column
    // coexists beside it), not the full terminal — otherwise its grid is clipped.
    // Side count is 1 here (detail only; no aux panes yet) — and the harness
    // share is count-independent anyway (it depends only on the column width).
    let pane_vp = composed_layout(Rect::new(0, 0, cols, rows), 1).pane;
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
    // The shell fallback has no grove → no nav row to land on (a harmless no-op).
    if let Some(name) = process.key.grove() {
        nav.select(name);
    }

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
        aux: AuxVisibility::default(),
        focus: Focus::Pane,
        nav,
        filter: FilterMode::new(),
        detail: Detail::new(),
        fleet,
        repo_roots,
        leader,
        capture: CaptureModal::new(),
        move_picker: Nav::default(),
        pending_move: None,
        pending_start: None,
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
        render_rx: &mut mpsc::UnboundedReceiver<(PaneKey, PaneSnapshot)>,
        input_rx: &mut mpsc::UnboundedReceiver<Event>,
        watch_rx: &mut mpsc::UnboundedReceiver<()>,
        input_pause: &Arc<AtomicBool>,
    ) -> Result<()> {
        let mut needs_redraw = true;
        loop {
            if needs_redraw {
                // Re-point the detail panel at its current subject before every
                // paint (060): while the nav owns focus that is the *highlighted*
                // grove (live preview re-points as the cursor moves), otherwise
                // the focused pane's grove. One call site keeps it consistent;
                // [`Detail::show`] preserves scroll when the grove is unchanged.
                self.rebuild_detail();
                self.draw(terminal)?;
                needs_redraw = false;
            }
            tokio::select! {
                maybe = render_rx.recv() => match maybe {
                    Some((key, snapshot)) => {
                        if let Some(entry) = self.panes.get_mut(&key) {
                            entry.state.set_snapshot(snapshot);
                        }
                        // Only a *currently-drawn* pane's updates trigger a
                        // redraw — the grove's harness (big-left) or one of its
                        // visible aux panes (side column). Background groves' panes
                        // and hidden aux stay warm without repainting, and Nav
                        // hides the lot. This is the [`surface_shows_pane`] gate
                        // widened from "the focused key" to "any visible pane" so a
                        // background aux push behind a focused harness still paints.
                        if self.pane_is_visible(&key) && surface_shows_pane(&self.focus) {
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
            // Harness/shell panes get the dominant-left share (count-independent —
            // it tracks only the side-column *width*). Aux panes size to their own
            // side-column slot, so skip them here and let `resize_visible_aux`
            // size the visible ones; hidden aux re-size on next show.
            let (pw, ph) = self.pane_viewport();
            for (key, entry) in &self.panes {
                if !key.is_aux() {
                    let _ = entry.driver.pane().resize(TerminalSizeSpec::new(pw, ph)).await;
                }
            }
            self.resize_visible_aux().await;
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
                // The cursor lives in the filter when engaged (the ranked list),
                // else in the grouped nav.
                if self.filter.engaged() {
                    self.filter.select_up();
                } else {
                    self.nav.select_up();
                }
                EventOutcome::Redraw
            }
            Action::NavDown => {
                if self.filter.engaged() {
                    self.filter.select_down();
                } else {
                    self.nav.select_down();
                }
                EventOutcome::Redraw
            }
            Action::NavSelect => {
                // arbitrate landed the focus on the pane optimistically; only
                // Enter on a *live grove* actually goes there. On a header or
                // a seed the app reverts to Nav (the begin_move precedent).
                if self.filter.engaged() {
                    self.select_ranked().await;
                } else {
                    // The pure row-kind resolution (the table can't see it); the
                    // app then applies the I/O half of whichever it returns.
                    match nav_enter(self.nav.selected_row()) {
                        NavEnter::ToggleFold => {
                            self.nav.toggle_fold();
                            self.focus = Focus::Nav;
                        }
                        // A seed has no harness yet — arm the confirm modal (070);
                        // `y` spawns `grove do <name>`, `n` cancels.
                        NavEnter::StartSeed(item) => self.begin_start(item),
                        NavEnter::Open(item) => {
                            if let Err(e) = self.open_or_focus(&item).await {
                                // Opening must never kill the TUI; fall back to the
                                // prior harness. (No stderr — we're on the alt screen.)
                                let _ = e;
                            }
                            // The focused grove changed; re-point the detail panel.
                            self.rebuild_detail();
                        }
                        NavEnter::Ignore => {}
                    }
                }
                EventOutcome::Redraw
            }
            Action::NavCollapse => {
                // No headers in the ranked shape — fold is inert while engaged.
                if !self.filter.engaged() {
                    self.nav.collapse();
                }
                EventOutcome::Redraw
            }
            Action::NavPeek => {
                // Tab → detail live preview (060). arbitrate optimistically set
                // Detail{Nav}; revert to Nav when the cursor has nothing to read
                // (a header, or an empty list). The preview itself re-points via
                // `rebuild_detail` before the paint.
                if !self.peek_has_subject() {
                    self.focus = Focus::Nav;
                }
                EventOutcome::Redraw
            }
            Action::NavPeekOrExpand => {
                // `l` → preview on a grove row, expand the fold on a header. The
                // table optimistically landed on Detail{Nav}; resolve the row kind
                // here (the [`Action::NavSelect`] revert precedent).
                if self.filter.engaged() {
                    // The ranked list has no headers; behave like NavPeek.
                    if self.filter.selected().is_none() {
                        self.focus = Focus::Nav;
                    }
                } else if matches!(self.nav.selected_row(), Some(NavRow::Header { .. })) {
                    self.focus = Focus::Nav;
                    self.nav.expand();
                } else if self.nav.selected().is_none() {
                    self.focus = Focus::Nav; // empty list — nothing to preview
                }
                EventOutcome::Redraw
            }
            Action::NavEsc => {
                // The layered Esc (050 Q5): arbitrate optimistically set the focus
                // to the pane. First press clears an engaged filter and stays in
                // Nav; only an *idle* Nav actually returns to the pane.
                if self.filter.engaged() {
                    self.filter.clear();
                    self.focus = Focus::Nav;
                }
                EventOutcome::Redraw
            }
            Action::FilterEnter => {
                // Refresh the ranked projection on entry (the fleet may have moved
                // since the last edit); the needle/toggles are preserved.
                self.filter.reproject(&self.fleet);
                EventOutcome::Redraw
            }
            Action::FilterEdit(edit) => {
                // Fold the edit in, re-rank, and snap to the top row (fzf — the
                // leaf's Notes: keep the cursor on the top match as it re-ranks).
                self.filter.apply(edit);
                self.filter.reproject(&self.fleet);
                self.filter.select_top();
                EventOutcome::Redraw
            }
            Action::FilterAccept => {
                // arbitrate already returned to Nav; the criteria stay engaged so
                // the nav now browses the ranked list. Nothing more to do.
                EventOutcome::Redraw
            }
            Action::FilterClear => {
                // arbitrate already returned to Nav; drop every dimension so the
                // nav reverts to its grouped shape.
                self.filter.clear();
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
            Action::ConfirmStart => {
                self.confirm_start().await;
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
                // One cancel path for every modal: discard the capture buffer, any
                // in-flight move, and any armed seed-start (whichever was up).
                self.capture.clear();
                self.pending_move = None;
                self.pending_start = None;
                EventOutcome::Redraw
            }
            Action::OpenEditor => EventOutcome::OpenEditor,
            Action::ToggleAux(role) => {
                self.toggle_aux(role).await;
                EventOutcome::Redraw
            }
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
        // Capture targets the grove the user is looking at (060): the previewed
        // grove while the nav owns focus, else the focused pane's grove.
        let Some(target) = self.detail_subject() else {
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
        // Grooming acts on the grove detail is showing (060): a nav-previewed
        // grove with no pane open still grooms correctly.
        let Some(target) = self.detail_subject() else {
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
        // arbitrate set focus to the move-picker modal over Detail{origin}; if we
        // bail, revert to that same detail surface so its Esc origin survives.
        let detail_focus = match &self.focus {
            Focus::Modal { prior, .. } => (**prior).clone(),
            other => other.clone(),
        };
        let Some(obs) = self.detail.selected_observation().cloned() else {
            self.focus = detail_focus; // empty inbox — nothing selected
            return;
        };
        // The source is the grove detail is showing (060) — the nav-previewed
        // grove when there is no pane open, not the focused pane's.
        let Some(source) = self.detail_subject() else {
            self.focus = detail_focus;
            self.toast = Some(CaptureOutcome::Failed("no grove focused".into()));
            return;
        };
        let mut picker = Nav::flat_from_fleet(&self.fleet);
        picker.remove(&source.repo_root, &source.name);
        if picker.selected().is_none() {
            self.focus = detail_focus;
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
        // Same subject begin_move armed from (stable while the picker is up).
        let Some(source) = self.detail_subject() else {
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

    /// Arm the seed-start confirm prompt (070): `Enter` landed on a seed row, so
    /// open the [`ModalKind::Confirm`] modal over the nav and remember the seed.
    /// `y`/`Enter` then spawns it ([`Self::confirm_start`]); `n`/`Esc` drops the
    /// arm (the shared [`Action::ModalCancel`] path). The modal restores the nav
    /// on cancel, so `prior` is Nav regardless of whether the seed came from the
    /// grouped list or the ranked filter.
    fn begin_start(&mut self, seed: NavItem) {
        self.focus = Focus::Modal {
            kind: ModalKind::Confirm,
            prior: Box::new(Focus::Nav),
        };
        self.pending_start = Some(seed);
    }

    /// Confirm the seed-start (`y`/`Enter`, 070): spawn `grove do <name>` for the
    /// armed seed and focus its new harness pane, then re-scan so the row flips
    /// Seed → Live (the rebuild re-finds the selection by name). arbitrate landed
    /// the focus on the pane optimistically; revert to Nav when nothing is armed
    /// or the spawn fails (the open-must-never-kill-the-TUI posture —
    /// [`Self::open_or_focus`]). For a seed the worktree does not exist yet, so
    /// `open_or_focus` runs `grove do` from the repo root, where it creates the
    /// worktree + branch + bootstrap session.
    async fn confirm_start(&mut self) {
        let Some(seed) = self.pending_start.take() else {
            self.focus = Focus::Nav; // nothing armed — back to the nav
            return;
        };
        if let Err(e) = self.open_or_focus(&seed).await {
            let _ = e; // spawn failed; no stderr on the alt screen
            self.focus = Focus::Nav;
            return;
        }
        // The seed is now a live grove: re-scan so the nav row flips Seed → Live
        // (selection preserved by name) and the detail panel re-points.
        self.refresh_fleet();
        self.rebuild_detail();
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
        // Nav opens a grove's *harness* pane; aux roles arrive via the leader
        // gate (050/030). The grove's other role-panes (if any) are distinct keys.
        let key = PaneKey::harness(item.name.clone());
        if self.panes.contains_key(&key) {
            self.focused = key; // focus the already-open pane
            return Ok(());
        }

        // `grove do <name>` resolves its repo from the cwd's git root. A live
        // grove's worktree works; a **seed** has no worktree yet (070), so spawn
        // from the repo root, where `grove do` creates the worktree + branch.
        let worktree = crate::repo::grove_worktree(&item.repo_root, &item.name);
        let cwd = if worktree.is_dir() {
            worktree
        } else {
            item.repo_root.clone()
        };
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
            .spawn_render_task(key.clone(), self.render_tx.clone())
            .await
            .context("opening the harness pane render stream")?;
        self.panes.insert(
            key.clone(),
            PaneEntry {
                driver,
                state: PaneState::default(),
                target: Some(CaptureTarget {
                    name: item.name.clone(),
                    repo_root: item.repo_root.clone(),
                }),
            },
        );
        self.focused = key;
        Ok(())
    }

    /// Enter on the filter mode's ranked-selected row (the engaged-Nav counterpart
    /// of the grouped [`Action::NavSelect`] branch). arbitrate optimistically set
    /// the focus to the pane; a live grove opens/focuses there, while a seed (no
    /// harness until 070) or an empty result set reverts to Nav.
    async fn select_ranked(&mut self) {
        match self.filter.selected().cloned() {
            Some(r) if r.lifecycle == Lifecycle::Seed => {
                // Arm the seed-start confirm (070), as the grouped-nav branch does.
                self.begin_start(NavItem {
                    repo_root: r.repo_root,
                    name: r.name,
                    label: r.label,
                    lifecycle: r.lifecycle,
                });
            }
            Some(r) => {
                let item = NavItem {
                    repo_root: r.repo_root,
                    name: r.name,
                    label: r.label,
                    lifecycle: r.lifecycle,
                };
                if let Err(e) = self.open_or_focus(&item).await {
                    let _ = e; // opening must never kill the TUI (no stderr here)
                }
                self.rebuild_detail();
            }
            None => self.focus = Focus::Nav, // no matches — nothing to open
        }
    }

    /// Re-scan the fleet and rebuild the nav (groves appearing/retiring),
    /// keeping the selection on the same grove where it survives. Uses the
    /// warning-collecting scan so no stderr noise corrupts the alt screen.
    fn refresh_fleet(&mut self) {
        let (fleet, _warnings) = MultiRepoView::scan_with_warnings(&self.repo_roots);
        self.fleet = fleet;
        self.nav.rebuild(&self.fleet);
        // The filter's ranked list is projected off the fleet too; re-rank it,
        // preserving the selected grove by identity (a background refresh must
        // not yank the cursor). A no-op-ish pass when idle (criteria default).
        self.filter.reproject(&self.fleet);
        // The focused grove's task tree / inbox may have changed (a leaf retired,
        // an observation captured); re-point detail off the fresh scan too.
        self.rebuild_detail();
    }

    /// Point the detail panel at its current [`subject`](Self::detail_subject),
    /// using the current fleet snapshot. No subject (bare shell / a header / an
    /// empty list) shows the empty "no grove" state. Re-pointing to a different
    /// grove resets detail's scroll; refreshing the same grove preserves it (see
    /// [`Detail::show`]). A seed resolves too — its `GroveDetail` has no task tree
    /// but a populated inbox, so the panel previews the inbox-only state (060).
    fn rebuild_detail(&mut self) {
        match self.detail_subject() {
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

    /// The grove the detail panel currently shows — and the grove every
    /// grove-targeted action (capture, reject, move) acts on (060). While the nav
    /// owns focus (or a nav-originated detail / a modal over one), it is the
    /// *highlighted* grove (the live preview, no harness spawn); otherwise the
    /// focused pane's grove (the pre-060 pane-coupling, restored when focus
    /// returns to a pane). `None` for a header row, an empty list, or the bare
    /// shell.
    fn detail_subject(&self) -> Option<CaptureTarget> {
        if focus_follows_nav(&self.focus) {
            self.nav_highlighted_target()
        } else {
            self.panes.get(&self.focused).and_then(|e| e.target.clone())
        }
    }

    /// The grove the nav cursor highlights, as a capture target: the ranked
    /// cursor when a filter is engaged, else the grouped selection. `None` on a
    /// header row or an empty list. Seeds resolve like live groves (preview-only).
    fn nav_highlighted_target(&self) -> Option<CaptureTarget> {
        if self.filter.engaged() {
            self.filter.selected().map(|r| CaptureTarget {
                name: r.name.clone(),
                repo_root: r.repo_root.clone(),
            })
        } else {
            self.nav.selected().map(|item| CaptureTarget {
                name: item.name.clone(),
                repo_root: item.repo_root.clone(),
            })
        }
    }

    /// Whether the nav currently highlights a grove worth previewing (Tab / `l`):
    /// a grove row in the grouped nav, or a ranked match in the engaged filter. A
    /// header row or an empty list has nothing to inspect.
    fn peek_has_subject(&self) -> bool {
        self.nav_highlighted_target().is_some()
    }

    /// The harness pane's size under the composed layout (the dominant left share
    /// beside the side column), clamped to at least 1×1 for a degenerate
    /// terminal. The harness/shell panes are resized to this; aux panes size to
    /// their own side-column slot (resize-on-show, 050/030).
    fn pane_viewport(&self) -> (u16, u16) {
        let pane = composed_layout(Rect::new(0, 0, self.size.0, self.size.1), self.side_count()).pane;
        (pane.width.max(1), pane.height.max(1))
    }

    /// The number of side-column members this frame: the always-present detail
    /// widget (`side[0]`) plus each visible aux pane. Drives the vertical split
    /// in [`composed_layout`]; matches [`Self::visible_aux_keys`] + 1.
    fn side_count(&self) -> usize {
        1 + self.visible_aux_keys().len()
    }

    /// The aux panes occupying the side column this frame, in stack order
    /// (term → yazi → vcs), each paired with `side[1..]` by [`render_side_column`].
    /// Resolved from the **current grove** (the focused pane's grove) and its
    /// per-grove visibility (Q6); empty for the bare shell (no grove → no working
    /// set). The fixed stack order comes from [`AuxVisibility::visible_roles`].
    fn visible_aux_keys(&self) -> Vec<PaneKey> {
        match self.focused.grove() {
            Some(grove) => self
                .aux
                .visible_roles(grove)
                .into_iter()
                .map(|role| PaneKey::Grove {
                    grove: grove.to_string(),
                    role,
                })
                .collect(),
            None => Vec::new(),
        }
    }

    /// The key of the **big-left** pane: always the current grove's *harness*
    /// (the dominant pane stays the harness even when focus follows to an aux
    /// pane in the side column), or the bare shell when there is no grove.
    fn left_pane_key(&self) -> PaneKey {
        match self.focused.grove() {
            Some(grove) => PaneKey::harness(grove),
            None => self.focused.clone(),
        }
    }

    /// Whether `key`'s pane is drawn this frame — the big-left harness/shell or
    /// one of the current grove's visible aux panes. Drives the render-push
    /// redraw gate so only on-screen panes repaint (a background grove's panes
    /// and hidden aux stay warm but quiet).
    fn pane_is_visible(&self, key: &PaneKey) -> bool {
        *key == self.left_pane_key() || self.visible_aux_keys().contains(key)
    }

    /// Toggle the current grove's `role` aux pane (leader `t`/`y`/`v`, Q6): a
    /// **toggle-with-focus-follow**. Visible → hide (the pty stays warm) and
    /// return focus to the harness. Hidden → **lazy-spawn on first toggle**, then
    /// show and focus-follow to the pane. The bare shell has no per-grove working
    /// set, so toggling there is a no-op.
    async fn toggle_aux(&mut self, role: PaneRole) {
        // The working grove is the focused pane's grove (harness, or an aux of the
        // same grove). Clone the target up front so the borrow is released before
        // the spawn/mutation below.
        let Some(target) = self.panes.get(&self.focused).and_then(|e| e.target.clone()) else {
            return; // shell fallback — no grove, no aux
        };
        let grove = target.name.clone();
        let key = PaneKey::Grove {
            grove: grove.clone(),
            role,
        };

        if self.aux.is_visible(&grove, role) {
            // Hide: stop drawing it (pty stays live, close only at TUI exit) and
            // return focus to the grove's harness.
            self.aux.hide(&grove, role);
            self.focused = PaneKey::harness(grove.clone());
        } else {
            // Show: spawn the window on first toggle, *before* recording the
            // pane visible — so a hard spawn failure (e.g. daemon trouble) leaves
            // no phantom-visible slot to render. A missing tool *binary* is not a
            // spawn failure: the window opens and the process fails visibly inside
            // it (Q3), exactly like the harness.
            // No stderr on the alt screen; a hard spawn failure stays put silently
            // (mirrors `open_or_focus`'s open-must-never-kill-the-TUI posture).
            if !self.panes.contains_key(&key)
                && self.spawn_aux(&grove, role, &target.repo_root).await.is_err()
            {
                return;
            }
            self.aux.show(&grove, role);
            self.focused = key;
        }
        // Detail re-points at the (unchanged) grove and the side column re-tiles;
        // size the now-visible aux panes to their fresh slots (resize-on-show).
        self.rebuild_detail();
        self.resize_visible_aux().await;
    }

    /// Lazy-spawn the `role` aux pane for `grove` as one detached rmux window
    /// (Q1 — same mechanism as the harness; only the argv and cwd differ), open
    /// its render stream onto the shared channel, and insert it into the pane map
    /// keyed by composite `(grove, role)`. cwd is the grove's worktree (Q3); the
    /// capture target is the grove, so grooming from an aux pane still targets it.
    /// Not resized here — the caller sizes it to its side-column slot on show.
    async fn spawn_aux(&mut self, grove: &str, role: PaneRole, repo_root: &Path) -> Result<()> {
        let key = PaneKey::Grove {
            grove: grove.to_string(),
            role,
        };
        let cwd = crate::repo::grove_worktree(repo_root, grove);
        let window = self
            .session
            .new_window_with()
            .name(format!("{grove}:{}", aux_label(role)))
            .spawn(aux_argv(role))
            .cwd(cwd)
            .detached(true)
            .await
            .context("opening an aux window")?;
        let wp = window
            .panes()
            .await
            .context("listing the new aux window's panes")?
            .into_iter()
            .next()
            .context("new aux window has no pane")?;
        let pane = self
            .rmux
            .pane(wp.target.clone())
            .await
            .context("resolving the new aux pane")?;
        let driver = PaneDriver::new(pane, wp.id);
        driver
            .spawn_render_task(key.clone(), self.render_tx.clone())
            .await
            .context("opening the aux pane render stream")?;
        self.panes.insert(
            key,
            PaneEntry {
                driver,
                state: PaneState::default(),
                target: Some(CaptureTarget {
                    name: grove.to_string(),
                    repo_root: repo_root.to_path_buf(),
                }),
            },
        );
        Ok(())
    }

    /// Size each currently-visible aux pane to its side-column slot (resize-on-
    /// show, Q2 residual): a hidden pane has no `Rect`, so it is sized only when
    /// it next becomes visible — and showing/hiding any member re-tiles the whole
    /// column, so every visible aux is re-sized, not just the one that changed.
    async fn resize_visible_aux(&self) {
        let layout = composed_layout(Rect::new(0, 0, self.size.0, self.size.1), self.side_count());
        // side[0] is detail; side[1..] are the aux slots, paired with the visible
        // aux in the same stack order `render_side_column` zips.
        let aux_slots = layout.side.get(1..).unwrap_or(&[]);
        for (slot, key) in aux_slots.iter().zip(self.visible_aux_keys()) {
            if let Some(entry) = self.panes.get(&key) {
                let _ = entry
                    .driver
                    .pane()
                    .resize(TerminalSizeSpec::new(slot.width.max(1), slot.height.max(1)))
                    .await;
            }
        }
    }

    /// Render the side-column members into their slots: the detail widget into
    /// `slots[0]` (always present, the stable anchor — Q5), then each visible aux
    /// pane into the slots below it (term → yazi → vcs). Detail is accented when
    /// `detail_focused`. Returns the **focused aux pane's** hardware cursor (in
    /// its slot) when an aux pane holds focus, so the caller can place the real
    /// cursor there; `None` otherwise. The caller decides whether to use it (only
    /// when a [`Focus::Pane`] is focused, not detail/modal).
    fn render_side_column(&self, slots: &[Rect], buf: &mut Buffer, detail_focused: bool) -> Option<(u16, u16)> {
        let Some((detail_slot, aux_slots)) = slots.split_first() else {
            return None; // floored at 1 in composed_layout, but stay panic-free
        };
        self.detail.render(*detail_slot, buf, detail_focused);
        let mut aux_cursor = None;
        for (slot, key) in aux_slots.iter().zip(self.visible_aux_keys()) {
            if let Some(entry) = self.panes.get(&key) {
                let cursor = render_pane(&entry.state, *slot, buf);
                // The focused aux pane (if any) owns the hardware cursor — its
                // grid is small but it is the input target while focus-followed.
                if key == self.focused {
                    aux_cursor = cursor;
                }
            }
        }
        aux_cursor
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
                    self.filter.engaged(),
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

    /// Render the **working set** surface (the pre-060 composed layout): the
    /// grove's harness in the dominant left pane and the side column (detail +
    /// visible aux) on the right. `detail_focused` accents the detail border;
    /// `pane_cursor` returns the focused pane's hardware cursor (the harness, or a
    /// focus-followed aux pane) — used only when a [`Focus::Pane`] is focused (a
    /// detail/modal surface has no pane caret).
    fn render_working_set(
        &self,
        area: Rect,
        buf: &mut Buffer,
        detail_focused: bool,
        pane_cursor: bool,
    ) -> Option<(u16, u16)> {
        let layout = composed_layout(area, self.side_count());
        let left_key = self.left_pane_key();
        let harness_cursor = self
            .panes
            .get(&left_key)
            .and_then(|entry| render_pane(&entry.state, layout.pane, buf));
        let aux_cursor = self.render_side_column(&layout.side, buf, detail_focused);
        if pane_cursor {
            if self.focused == left_key {
                harness_cursor
            } else {
                aux_cursor
            }
        } else {
            None
        }
    }

    /// Render the **nav + live-preview** surface (060): the grove list (or the
    /// ranked filter list when engaged / being typed) fills the dominant left
    /// pane, and the detail widget previews the highlighted grove in the side
    /// column's top slot. No harness/aux — a nav surface hides the focused grove's
    /// panes (it is a file-manager list+preview, not the working set). The detail
    /// panel is re-pointed by [`rebuild_detail`] before the paint, so moving the
    /// nav cursor re-previews live. `detail_focused` accents the detail border
    /// (focus moved *into* the preview via Tab / `l`); `typing` shows the needle
    /// caret ([`Focus::Filter`]) and is returned so the App places the cursor.
    fn render_nav_preview(
        &self,
        area: Rect,
        buf: &mut Buffer,
        detail_focused: bool,
        typing: bool,
    ) -> Option<(u16, u16)> {
        let layout = composed_layout(area, 1); // nav + detail only — no aux
        let caret = if typing || self.filter.engaged() {
            // Engaged → the flat ranked list (+ criteria line); a caret only while
            // actively typing the needle.
            self.filter.render(layout.pane, buf, typing)
        } else {
            self.nav.render(layout.pane, buf);
            None
        };
        if let Some(detail_slot) = layout.side.first() {
            self.detail.render(*detail_slot, buf, detail_focused);
        }
        caret
    }

    /// Render one focus surface into `buf`, returning where the hardware cursor
    /// belongs (the pane's cursor, or the modal's text caret), or `None`. Never
    /// receives [`Focus::LeaderPending`] — the caller unwraps that to its `prior`.
    ///
    /// Two surface families tile the content region (both via [`composed_layout`],
    /// with the footer row reserved): the **working set** ([`Focus::Pane`] and
    /// pane-entered [`Focus::Detail`]) — harness left, side column right — and the
    /// **nav preview** ([`Focus::Nav`], [`Focus::Filter`], and nav-entered
    /// [`Focus::Detail`], the 060 live preview) — the grove list left, the
    /// highlighted grove's detail right. A modal draws whichever its `prior` was,
    /// then the centered overlay on top.
    fn render_surface(&self, focus: &Focus, area: Rect, buf: &mut Buffer) -> Option<(u16, u16)> {
        match focus {
            // Working set: the cursor shows only when a *pane* is focused (detail
            // is a scroll list with no text caret).
            Focus::Pane => self.render_working_set(area, buf, false, true),
            Focus::Detail {
                origin: DetailOrigin::Pane,
            } => self.render_working_set(area, buf, true, false),
            // Nav preview (060): the list on the left, the highlighted grove's
            // detail on the right. Detail is accented once focus moves into it.
            Focus::Nav => self.render_nav_preview(area, buf, false, false),
            Focus::Detail {
                origin: DetailOrigin::Nav,
            } => self.render_nav_preview(area, buf, true, false),
            Focus::Filter => self.render_nav_preview(area, buf, false, true),
            Focus::Modal { kind, prior } => {
                // Draw the surface the modal opened over (working set or nav
                // preview, per `prior`), then the centered modal *over* it (Clear
                // punches the hole). The underlying cursor is unused — the modal
                // owns the caret.
                let detail_focused = matches!(**prior, Focus::Detail { .. });
                if focus_follows_nav(prior) {
                    self.render_nav_preview(area, buf, detail_focused, false);
                } else {
                    self.render_working_set(area, buf, detail_focused, false);
                }
                match kind {
                    ModalKind::Capture => {
                        // The capture targets the grove the user is looking at
                        // (the detail subject), so name *that* grove in the modal.
                        let label = self
                            .detail_subject()
                            .map(|t| t.name)
                            .unwrap_or_else(|| "(no grove)".to_string());
                        self.capture.render(area, buf, &label)
                    }
                    ModalKind::MovePicker => {
                        // The move-target picker: the fleet's grove list (source
                        // excluded) reused as a centered overlay (Nav clears its
                        // own rect). j/k select · ⏎ send · ⎋ cancel.
                        let popup = crate::tui::capture::centered_rect(60, 60, area);
                        self.move_picker.render(popup, buf);
                        None
                    }
                    ModalKind::Confirm => {
                        // The seed-start y/n prompt (070), naming the armed seed.
                        let label = self
                            .pending_start
                            .as_ref()
                            .map(|s| s.name.as_str())
                            .unwrap_or("(no grove)");
                        crate::tui::capture::render_confirm(label, area, buf);
                        None
                    }
                }
            }
            // Unwrapped by the caller; rendering its `prior` instead.
            Focus::LeaderPending { .. } => None,
        }
    }
}

/// The composed-layout rects (050/020): the harness pane (dominant left share),
/// an ordered **side column** of member rects (detail first, then a slot per
/// visible aux pane), and the reserved footer row. The harness pane and the side
/// column **coexist** — focus moves laterally between them — which is the
/// 050/010-surfaces verdict the trellis flip-to model lacked. The side column is
/// a *heterogeneous stack* (Q5): `side[0]` is the always-present detail widget,
/// `side[1..]` are the visible aux-pane slots (term/yazi/vcs, populated in
/// 050/030). `side` always has at least one rect.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ComposedLayout {
    pane: Rect,
    side: Vec<Rect>,
    footer: Rect,
}

/// The ~220-col responsive breakpoint (Q4): the side column gets a more generous
/// width cap on wide terminals and stays the narrower ≈laptop column below it.
/// Geometry only — membership is user-toggle (050/030), never width-driven.
const WIDE_BREAKPOINT: u16 = 220;
/// The side column targets this fraction of the content region…
const SIDE_COLUMN_FRACTION: u16 = 34;
/// …capped here on wide terminals (raises the old flat 48 cap) …
const SIDE_COLUMN_CAP_WIDE: u16 = 64;
/// …and here on laptops (≈the pre-050 column), keeping the harness dominant.
const SIDE_COLUMN_CAP_NARROW: u16 = 48;

/// The side column's width as a fraction of the content region, capped by the
/// responsive breakpoint so it stays a *column* (harness keeps the dominant
/// share) and widens a little on big displays. Width is the **sole** responsive
/// geometry lever: per-member height is pure equal-share tiling (see
/// [`stack_evenly`]), so the breakpoint has no height job to do (Q4 "let it
/// tile").
fn side_column_width(content_width: u16) -> u16 {
    let cap = if content_width >= WIDE_BREAKPOINT {
        SIDE_COLUMN_CAP_WIDE
    } else {
        SIDE_COLUMN_CAP_NARROW
    };
    (content_width.saturating_mul(SIDE_COLUMN_FRACTION) / 100).min(cap)
}

/// Split `area` into `n` vertically-stacked rects of equal height, the remainder
/// rows handed to the topmost members so the stack tiles `area` exactly. This is
/// the Q4 "let it tile" rule: members always share the column evenly, with no
/// overflow/scroll machinery — too many toggled-on members is a degradation the
/// user manages via the toggles. Degrades without panic on a zero-height area
/// (every member gets a 0-height rect) and returns empty for `n == 0`.
fn stack_evenly(area: Rect, n: usize) -> Vec<Rect> {
    if n == 0 {
        return Vec::new();
    }
    let n_u16 = n as u16;
    let base = area.height / n_u16;
    let extra = area.height % n_u16;
    let mut rects = Vec::with_capacity(n);
    let mut y = area.y;
    for i in 0..n_u16 {
        let h = base + u16::from(i < extra);
        rects.push(Rect::new(area.x, y, area.width, h));
        y += h;
    }
    rects
}

/// Split `area` into the composed layout: a reserved bottom footer row, then the
/// content region divided into the harness pane (left, dominant) and a side
/// column of `side_count` stacked member rects (detail at `side[0]`, then aux
/// slots). The layout math is **key-agnostic** — it takes a member *count*, not
/// the panes — so 050/030 only has to grow the count. Degrades without panic on
/// a tiny area; `side_count` is floored at 1 so detail always has a slot.
fn composed_layout(area: Rect, side_count: usize) -> ComposedLayout {
    let footer_h = area.height.min(1);
    let content_h = area.height - footer_h;
    let footer = Rect::new(area.x, area.y + content_h, area.width, footer_h);
    let content = Rect::new(area.x, area.y, area.width, content_h);

    let side_w = side_column_width(content.width);
    let pane_w = content.width - side_w;
    let pane = Rect::new(content.x, content.y, pane_w, content.height);
    let side_area = Rect::new(content.x + pane_w, content.y, side_w, content.height);
    let side = stack_evenly(side_area, side_count.max(1));
    ComposedLayout { pane, side, footer }
}

/// Whether `focus` is a **nav surface** — one that previews the nav-highlighted
/// grove rather than the focused pane's grove (060). True for Nav and its `/`
/// filter sub-mode, a nav-originated [`Focus::Detail`] (the live preview), and
/// any gate/modal layered over one of those; false for a pane and a pane-entered
/// detail. Drives both [`App::detail_subject`] (which grove detail shows) and
/// which underlying surface a modal/preview renders behind itself.
fn focus_follows_nav(focus: &Focus) -> bool {
    match focus {
        Focus::Nav | Focus::Filter => true,
        Focus::Detail { origin } => matches!(origin, DetailOrigin::Nav),
        Focus::Modal { prior, .. } | Focus::LeaderPending { prior } => focus_follows_nav(prior),
        Focus::Pane => false,
    }
}

/// Whether the focused pane is visible on the current surface (so a background
/// render push should trigger a repaint). Nav and the 060 nav-preview detail hide
/// the pane; a gate/modal inherits the surface it opened over.
fn surface_shows_pane(focus: &Focus) -> bool {
    match focus {
        Focus::Pane => true,
        // Pane-entered detail keeps the working set (harness) behind it; the 060
        // nav-preview detail shows nav + detail with no pane.
        Focus::Detail { origin } => matches!(origin, DetailOrigin::Pane),
        // A modal/gate inherits the visibility of the surface it sits over.
        Focus::Modal { prior, .. } | Focus::LeaderPending { prior } => surface_shows_pane(prior),
        // Nav and its `/` filter sub-mode are flip-to surfaces that hide the pane.
        Focus::Nav | Focus::Filter => false,
    }
}

/// What `Enter` on the grouped nav resolves to — the **pure** half of the
/// `NavSelect` resolution ([`arbitrate`] optimistically lands on the pane and
/// can't see the row kind, so the app classifies it here, then applies the I/O).
/// Mirrored loosely by [`App::select_ranked`] for the engaged filter (the ranked
/// list has no headers, so only the seed/live split applies there).
#[derive(Debug, PartialEq, Eq)]
enum NavEnter {
    /// The cursor sits on a repo section header → toggle its fold (stay in Nav).
    ToggleFold,
    /// A **seed** row → arm the seed-start confirm prompt (070).
    StartSeed(NavItem),
    /// A **live** grove row → open/focus its harness pane.
    Open(NavItem),
    /// Nothing selected (empty list) → no-op.
    Ignore,
}

/// Classify `Enter` on the grouped nav's selected row (070): a header folds, a
/// seed opens the confirm prompt, a live grove opens its harness, an empty list
/// is inert. Pure so the seed-row → confirm wiring is headless-testable.
fn nav_enter(row: Option<&NavRow>) -> NavEnter {
    match row {
        Some(NavRow::Header { .. }) => NavEnter::ToggleFold,
        Some(NavRow::Grove(item)) if item.lifecycle == Lifecycle::Seed => {
            NavEnter::StartSeed(item.clone())
        }
        Some(NavRow::Grove(item)) => NavEnter::Open(item.clone()),
        None => NavEnter::Ignore,
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

/// The argv a working-set aux pane runs (Q3): `term` = the user's `$SHELL`,
/// `yazi` = `yazi`, `vcs` = **lazygit** (hardcoded). A missing binary is not
/// pre-checked — the window opens and the process fails visibly inside the pane
/// (same posture as the harness). `Harness` never reaches here (it spawns via
/// [`select_initial_process`] / [`App::open_or_focus`], not the aux path).
fn aux_argv(role: PaneRole) -> Vec<String> {
    match role {
        PaneRole::Term => vec![std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string())],
        PaneRole::Yazi => vec!["yazi".to_string()],
        // jj detection is a follow-up: grove worktrees are git today, so lazygit
        // is correct now; lazyjj-vs-lazygit selection is a seam left for later.
        PaneRole::Vcs => vec!["lazygit".to_string()],
        PaneRole::Harness => unreachable!("the harness is not an aux role"),
    }
}

/// The short role label used in an aux pane's rmux window name (`<grove>:<role>`).
fn aux_label(role: PaneRole) -> &'static str {
    match role {
        PaneRole::Term => "term",
        PaneRole::Yazi => "yazi",
        PaneRole::Vcs => "vcs",
        PaneRole::Harness => "harness",
    }
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
                key: PaneKey::harness(grove.name.clone()),
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
        key: PaneKey::Shell,
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

    // --- AuxVisibility: per-grove working-set toggle state (Q6) -------------

    #[test]
    fn aux_visibility_is_per_grove_and_survives_hide_as_warm() {
        let mut v = AuxVisibility::default();
        assert!(!v.is_visible("a", PaneRole::Term), "starts hidden");
        v.show("a", PaneRole::Term);
        assert!(v.is_visible("a", PaneRole::Term));
        // Toggling one grove leaves another untouched — visibility is per-grove,
        // so switching groves restores *that* grove's set.
        assert!(!v.is_visible("b", PaneRole::Term), "visibility is per-grove");
        v.hide("a", PaneRole::Term);
        assert!(!v.is_visible("a", PaneRole::Term), "hide stops drawing it");
    }

    #[test]
    fn aux_visible_roles_use_the_fixed_stack_order_not_toggle_order() {
        let mut v = AuxVisibility::default();
        // Toggle on out of order; the stack position is fixed term→yazi→vcs (Q5).
        v.show("a", PaneRole::Vcs);
        v.show("a", PaneRole::Term);
        assert_eq!(
            v.visible_roles("a"),
            vec![PaneRole::Term, PaneRole::Vcs],
            "canonical order regardless of toggle order"
        );
        v.show("a", PaneRole::Yazi);
        assert_eq!(
            v.visible_roles("a"),
            vec![PaneRole::Term, PaneRole::Yazi, PaneRole::Vcs]
        );
        // A grove never toggled has no visible aux.
        assert!(v.visible_roles("untouched").is_empty());
    }

    #[test]
    fn aux_show_is_idempotent_and_hide_tolerates_absent() {
        let mut v = AuxVisibility::default();
        v.show("a", PaneRole::Yazi);
        v.show("a", PaneRole::Yazi); // no duplicate, no panic
        assert_eq!(v.visible_roles("a"), vec![PaneRole::Yazi]);
        v.hide("a", PaneRole::Term); // hiding something never shown is a no-op
        v.hide("never", PaneRole::Term);
        assert_eq!(v.visible_roles("a"), vec![PaneRole::Yazi]);
    }

    #[test]
    fn aux_argv_resolves_each_role_and_vcs_is_lazygit() {
        assert_eq!(aux_argv(PaneRole::Yazi), vec!["yazi".to_string()]);
        // vcs is lazygit (hardcoded; jj detection is a follow-up).
        assert_eq!(aux_argv(PaneRole::Vcs), vec!["lazygit".to_string()]);
        // term is a single-element argv (the shell), whatever $SHELL resolves to.
        assert_eq!(aux_argv(PaneRole::Term).len(), 1);
    }

    // --- nav_enter: the pure Enter-on-a-row resolution (070) ----------------

    fn grove_row(name: &str, lifecycle: Lifecycle) -> NavRow {
        NavRow::Grove(NavItem {
            repo_root: PathBuf::from("/repo"),
            name: name.to_string(),
            label: name.to_string(),
            lifecycle,
        })
    }

    #[test]
    fn nav_enter_on_a_seed_arms_the_confirm_start() {
        // The 070 wiring: Enter on a seed row resolves to StartSeed (which the app
        // turns into the confirm modal), not Open and not a silent no-op.
        let row = grove_row("newgrove", Lifecycle::Seed);
        match nav_enter(Some(&row)) {
            NavEnter::StartSeed(item) => assert_eq!(item.name, "newgrove"),
            other => panic!("seed Enter should arm the confirm, got {other:?}"),
        }
    }

    #[test]
    fn nav_enter_on_a_live_grove_opens_it() {
        let row = grove_row("alpha", Lifecycle::Live);
        match nav_enter(Some(&row)) {
            NavEnter::Open(item) => assert_eq!(item.name, "alpha"),
            other => panic!("live Enter should open, got {other:?}"),
        }
    }

    #[test]
    fn nav_enter_on_a_header_toggles_the_fold_and_empty_is_inert() {
        let header = NavRow::Header {
            repo_root: PathBuf::from("/repo"),
            label: "repo".into(),
            folded: false,
        };
        assert_eq!(nav_enter(Some(&header)), NavEnter::ToggleFold);
        assert_eq!(nav_enter(None), NavEnter::Ignore);
    }

    #[test]
    fn composed_layout_reserves_the_bottom_footer_row() {
        let area = Rect::new(0, 0, 120, 40);
        let layout = composed_layout(area, 1);
        assert_eq!(layout.footer, Rect::new(0, 39, 120, 1));
        // pane + the side column occupy the content region above the footer.
        assert_eq!(layout.pane.height, 39);
        assert_eq!(layout.side[0].height, 39);
    }

    #[test]
    fn composed_layout_places_the_side_column_on_the_right() {
        let area = Rect::new(0, 0, 120, 40);
        let layout = composed_layout(area, 1);
        // The side column abuts the pane on the right; together they tile the
        // full width.
        let side = layout.side[0];
        assert_eq!(side.x, layout.pane.x + layout.pane.width);
        assert_eq!(layout.pane.width + side.width, 120);
        // The harness keeps the dominant share.
        assert!(layout.pane.width > side.width, "{layout:?}");
    }

    #[test]
    fn side_count_maps_one_to_one_to_side_slots() {
        let area = Rect::new(0, 0, 200, 60);
        for n in 1..=5 {
            assert_eq!(composed_layout(area, n).side.len(), n, "{n} members → {n} slots");
        }
        // Floored at 1 so detail always has a slot even if a caller passes 0.
        assert_eq!(composed_layout(area, 0).side.len(), 1);
    }

    #[test]
    fn side_column_stacks_detail_on_top_in_order() {
        // Three members tile the column top-to-bottom with no gaps/overlaps and
        // detail (slot 0) is the topmost — the Q5 stable anchor.
        let area = Rect::new(0, 0, 200, 61); // 60 content rows / 3 = 20 each
        let layout = composed_layout(area, 3);
        let s = &layout.side;
        assert_eq!(s[0].y, layout.pane.y, "detail sits at the top of the column");
        assert!(s[0].y < s[1].y && s[1].y < s[2].y, "stacked top-to-bottom: {s:?}");
        // Contiguous: each slot begins where the previous ends.
        assert_eq!(s[1].y, s[0].y + s[0].height);
        assert_eq!(s[2].y, s[1].y + s[1].height);
        // They tile the content height exactly (last slot ends at the footer).
        assert_eq!(s[2].y + s[2].height, layout.footer.y);
        // Equal shares (60 / 3 = 20, no remainder).
        assert!(s.iter().all(|r| r.height == 20), "{s:?}");
    }

    #[test]
    fn side_column_tiles_remainder_rows_onto_the_top_members() {
        // 61 content rows / 4 members = 15 each + 1 remainder → the topmost
        // member absorbs it; still tiles the column exactly (let-it-tile, Q4).
        let area = Rect::new(0, 0, 200, 62); // 61 content rows after the footer
        let layout = composed_layout(area, 4);
        let heights: Vec<u16> = layout.side.iter().map(|r| r.height).collect();
        assert_eq!(heights, vec![16, 15, 15, 15], "{heights:?}");
        assert_eq!(heights.iter().sum::<u16>(), 61, "tiles the content exactly");
    }

    #[test]
    fn breakpoint_widens_the_side_column_above_220_cols() {
        // Below the breakpoint: the narrower ≈laptop cap (48). The fraction is
        // dominated by the cap on any realistic laptop width.
        let narrow = composed_layout(Rect::new(0, 0, 200, 50), 1);
        assert_eq!(narrow.side[0].width, 48, "laptop column stays narrow");
        // At/above the breakpoint: the wider cap (64).
        let wide = composed_layout(Rect::new(0, 0, 400, 50), 1);
        assert_eq!(wide.side[0].width, 64, "wide displays get a roomier column");
        // The harness stays dominant in both regimes.
        assert!(narrow.pane.width > narrow.side[0].width);
        assert!(wide.pane.width > wide.side[0].width);
    }

    #[test]
    fn breakpoint_is_exactly_220_cols() {
        // The content region (not the terminal) is what the breakpoint reads, so
        // probe widths straddling 220 directly.
        assert_eq!(side_column_width(219), 48);
        assert_eq!(side_column_width(220), 64);
    }

    #[test]
    fn composed_layout_degrades_without_panic_on_a_tiny_area() {
        // Zero-height, one-row, and multi-member tiny areas must not underflow.
        let _ = composed_layout(Rect::new(0, 0, 0, 0), 3);
        let l = composed_layout(Rect::new(0, 0, 10, 1), 1);
        assert_eq!(l.footer.height, 1);
        assert_eq!(l.pane.height, 0);
        // A column too short to give every member a row tiles to zero-height
        // slots rather than panicking.
        let cramped = composed_layout(Rect::new(0, 0, 10, 3), 5);
        assert_eq!(cramped.side.len(), 5);
        assert_eq!(cramped.side.iter().map(|r| r.height).sum::<u16>(), 2); // 3 - footer
    }
}
