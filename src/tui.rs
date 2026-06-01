// The `grove tui` subcommand: a sync master/detail navigator over one
// repo's groves. Leaves 020 (read-only shell) and 030 (writes +
// fs-watch) under `020-design-seed-convention/090-tui-server/`.
//
// Architecture:
//   - All state derives from a `RepoView` snapshot. The snapshot is
//     re-scanned on `r`, on every fs-watch quiescence (`notify` events
//     coalesced by a 200ms debounce), and after every shell-out so the
//     round trip from capture → inbox count update is visible without
//     manual refresh.
//   - `App` owns the snapshot, screen/selection state, and the capture
//     modal. Rendering is a pure function of `App` + the screen rect,
//     which keeps the `TestBackend` snapshot test honest.
//   - The Ratatui event loop is the standard sync poll/read pattern
//     (see ratatui 0.29 docs). The shell-out from `c` suspends the
//     alternate-screen / raw-mode terminal via `ratatui::restore()` and
//     resumes it with a fresh `ratatui::init()` — no bespoke
//     alt-screen toggling.
//
// Walk-away-ability (SKILL.md constraint 6) is preserved by routing
// every write through the `grove-llm inbox-add` verb. The TUI never
// edits grove state directly.

use anyhow::{Context, Result};
use std::collections::BTreeMap;
use std::io::{self, Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::{Constraint, Direction, Layout, Rect, Size};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::{DefaultTerminal, Frame, Terminal};

use crate::cli::RepoArgs;
use crate::dash::backend::{self, ProxyBackend};
use crate::dash::decode::InputDecoder;
use crate::dash::proto::{DownFrame, FrameWriter, UpDecoder, UpFrame};
use crate::repo;
use crate::repo_view::{
    self, GroveDetail, GroveSummary, Lifecycle, RepoView, TaskEntry, TaskKind,
};

const DEBOUNCE: Duration = Duration::from_millis(200);

/// Styling for a version-drift marker — bold yellow, to draw the eye on both
/// the header (`cli`-vs-`repo`) and grove rows (`worktree`-vs-`repo`/`cli`).
/// The drift *rule* is the same plain string-equality one `grove status` uses
/// (`CONTEXT.md`); only the presentation differs (colour vs. plain text).
const DRIFT_STYLE: Style = Style::new()
    .fg(Color::Yellow)
    .add_modifier(Modifier::BOLD);

pub fn run(args: &RepoArgs) -> Result<()> {
    let repo = repo::resolve(args.repo.as_deref())?;
    let view = RepoView::scan(&repo)?;
    let preselect = current_grove_name(&repo);
    let mut app = App::new(repo.clone(), view, preselect);
    let mut watch = WatchSet::new(&repo);

    let mut terminal = ratatui::init();
    let outcome = live_event_loop(&mut terminal, &mut app, &mut watch);
    ratatui::restore();
    outcome
}

// ---------------------------------------------------------------------------
// State

/// Top-level app state — single source of truth for both screens.
pub struct App {
    repo: PathBuf,
    view: RepoView,
    screen: Screen,
    list: ListState,
    detail: Option<DetailState>,
    filter: FilterState,
    show_help: bool,
    status: Option<String>,
    capture: CaptureModal,
    /// Disposition picker for the selected inbox entry; `Some` while open.
    disposition: Option<DispositionModal>,
    /// A keystroke (Ctrl-E / Enter in body / submit) decides *that* an
    /// external action should run; the live loop then suspends the
    /// terminal and runs it. Splitting these phases keeps `handle_key`
    /// pure enough to test without a real terminal.
    pending_action: Option<PendingAction>,
}

/// Capture modal — opened by `c`, drives the two-step
/// target/body workflow. The body accepts multi-line input: `Enter`
/// inserts a newline (so a multi-line paste lands intact) and submit is
/// the deliberate `Ctrl-S` gesture, which a paste cannot trigger. A
/// longer or heavier edit still drops to `$EDITOR` via `Ctrl-E`.
#[derive(Default, Clone)]
pub struct CaptureModal {
    open: bool,
    field: CaptureField,
    target: String,
    body: String,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
enum CaptureField {
    #[default]
    Target,
    Body,
}

/// Disposition picker — opened by `d` on the selected entry while the Inbox
/// pane is focused. A sub-modal rather than three bare keys because `r` is the
/// global refresh key: scoping the choice behind a modal keeps the top-level
/// keymap unambiguous and the choice discoverable.
pub struct DispositionModal {
    /// Absolute path of the observation being dispositioned.
    path: PathBuf,
    /// Filename of that entry, shown in the picker title.
    entry: String,
}

/// The three [[Drain]] buckets. All three delete the observation file; the
/// choice only sets the `grove-llm inbox-drain` commit-message category.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Disposition {
    Incorporated,
    Deferred,
    Rejected,
}

impl Disposition {
    /// The `grove-llm inbox-drain` finalize flag for this bucket.
    fn flag(self) -> &'static str {
        match self {
            Disposition::Incorporated => "--incorporated",
            Disposition::Deferred => "--deferred",
            Disposition::Rejected => "--rejected",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Disposition::Incorporated => "incorporated",
            Disposition::Deferred => "deferred",
            Disposition::Rejected => "rejected",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PendingAction {
    /// Submit `app.capture` via `grove-llm inbox-add`.
    Submit,
    /// Drop into `$EDITOR` (or `vi` fallback) to edit `app.capture.body`.
    EditBody,
    /// Disposition the observation at `path` into `disposition`'s bucket via
    /// `grove-llm inbox-drain` — all three buckets delete the file; the bucket
    /// only sets the drain commit-message category (faithful to Drain).
    Drain {
        path: PathBuf,
        disposition: Disposition,
    },
    /// Edit the *existing committed* observation at `path`: seed `$EDITOR` with
    /// its current body, then on a non-empty change round-trip through
    /// `grove-llm inbox-edit`. Distinct from `EditBody`, which edits the
    /// in-memory capture draft before it is ever committed.
    EditObservation {
        path: PathBuf,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    GroveList,
    GroveDetail,
}

struct DetailState {
    grove: String,
    /// Selected row in the flattened task-tree view.
    tree: ListState,
    /// Selected pending observation in the inbox pane. Kept distinct from
    /// `tree` so each pane remembers its own cursor across `Tab` switches.
    inbox: ListState,
    right: RightPane,
    /// Scroll offset for the right pane (lines from top).
    right_scroll: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RightPane {
    LeafBody,
    Inbox,
    Brief,
}

impl RightPane {
    fn next(self) -> Self {
        match self {
            RightPane::LeafBody => RightPane::Inbox,
            RightPane::Inbox => RightPane::Brief,
            RightPane::Brief => RightPane::LeafBody,
        }
    }

    fn label(self) -> &'static str {
        match self {
            RightPane::LeafBody => "leaf",
            RightPane::Inbox => "inbox",
            RightPane::Brief => "brief",
        }
    }
}

#[derive(Default)]
struct FilterState {
    /// True while the user is typing into the filter input.
    editing: bool,
    /// The committed (or in-progress) substring filter — applied live.
    text: String,
}

impl App {
    pub fn new(repo: PathBuf, view: RepoView, preselect: Option<String>) -> Self {
        let mut list = ListState::default();
        let preselect_idx = preselect
            .as_deref()
            .and_then(|name| view.groves().iter().position(|g| g.name == name));
        list.select(Some(preselect_idx.unwrap_or(0)));
        Self {
            repo,
            view,
            screen: Screen::GroveList,
            list,
            detail: None,
            filter: FilterState::default(),
            show_help: false,
            status: None,
            capture: CaptureModal::default(),
            disposition: None,
            pending_action: None,
        }
    }

    /// Rescan the repo without touching the status line. Used by fs-watch.
    fn refresh_silent(&mut self) -> Result<()> {
        // Preserve which grove the user was looking at across the rescan.
        let current_grove = match self.screen {
            Screen::GroveDetail => self.detail.as_ref().map(|d| d.grove.clone()),
            Screen::GroveList => self
                .filtered_groves()
                .get(self.list.selected().unwrap_or(0))
                .map(|g| g.name.clone()),
        };
        self.view = RepoView::scan(&self.repo)?;
        // Reselect by name if possible; otherwise fall back to first row.
        let groves = self.view.groves();
        let idx = current_grove
            .as_deref()
            .and_then(|name| groves.iter().position(|g| g.name == name))
            .unwrap_or(0);
        self.list.select(if groves.is_empty() { None } else { Some(idx) });
        // If we were on detail and the grove vanished, pop to list.
        if matches!(self.screen, Screen::GroveDetail) {
            let still_there = current_grove
                .as_deref()
                .map(|name| self.view.grove(name).is_some())
                .unwrap_or(false);
            if !still_there {
                self.screen = Screen::GroveList;
                self.detail = None;
            }
            // Tree shape may have changed; let the selection clamp on
            // next render via the existing bounds check in
            // `render_grove_detail` rather than resetting to 0 here.
        }
        Ok(())
    }

    /// Rescan the repo and signal "refreshed" in the status line.
    /// Triggered by `r`. Clears any in-progress filter to make the
    /// rescan's selection deterministic for the user.
    fn refresh(&mut self) -> Result<()> {
        self.filter.text.clear();
        self.refresh_silent()?;
        self.status = Some("refreshed".into());
        Ok(())
    }

    fn filtered_groves(&self) -> Vec<&GroveSummary> {
        let needle = self.filter.text.to_lowercase();
        self.view
            .groves()
            .iter()
            .filter(|g| needle.is_empty() || g.name.to_lowercase().contains(&needle))
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Filesystem watcher
//
// `notify` runs its watcher thread; events arrive on an `mpsc` channel
// that the sync event loop polls each tick. We don't act on individual
// events — any event marks the snapshot "dirty" and starts a 200ms
// debounce window. After that window elapses with no new events, we
// rescan once. A `git checkout` that touches dozens of files therefore
// produces one rescan, not dozens.
//
// Best-effort: if `notify` cannot initialise (e.g. an exotic platform),
// or a watched directory does not yet exist, we proceed without
// fs-watch. `r` still works.

/// Watcher + receiver bundle, kept alive for the duration of the event
/// loop. Dropping `WatchSet` stops the watcher thread cleanly — the
/// brief's "exits cleanly on `q`, no leaked threads" constraint.
pub struct WatchSet {
    _watcher: Option<RecommendedWatcher>,
    rx: Option<mpsc::Receiver<notify::Result<notify::Event>>>,
    /// When set, an event arrived at this instant and we are inside the
    /// debounce window. `None` means the snapshot is settled.
    dirty_since: Option<Instant>,
}

impl WatchSet {
    pub fn new(repo: &Path) -> Self {
        let (tx, rx) = mpsc::channel();
        let mut watcher = match notify::recommended_watcher(move |res| {
            let _ = tx.send(res);
        }) {
            Ok(w) => w,
            Err(_) => {
                return Self { _watcher: None, rx: None, dirty_since: None };
            }
        };
        // Watch the two roots that hold grove state. Recursive picks up
        // new groves, new leaves, and new observation files without
        // re-registering watchers.
        for dir in [
            repo.join(".grove-worktrees"),
            repo.join(".grove-meta").join("inboxes"),
        ] {
            if dir.is_dir() {
                let _ = watcher.watch(&dir, RecursiveMode::Recursive);
            }
        }
        Self { _watcher: Some(watcher), rx: Some(rx), dirty_since: None }
    }

    /// Drain pending events from the channel; mark dirty if any arrived.
    fn drain(&mut self) {
        let Some(rx) = self.rx.as_ref() else { return };
        let mut any = false;
        while let Ok(_ev) = rx.try_recv() {
            any = true;
        }
        if any {
            self.dirty_since = Some(Instant::now());
        }
    }

    /// True when an event has arrived and the debounce window has elapsed.
    fn settled(&self) -> bool {
        match self.dirty_since {
            Some(t) => t.elapsed() >= DEBOUNCE,
            None => false,
        }
    }

    fn clear(&mut self) {
        self.dirty_since = None;
    }

    /// Poll timeout for `event::poll`. While dirty, shorten so we notice
    /// the debounce settling promptly without idling for 200ms.
    fn poll_timeout(&self) -> Duration {
        match self.dirty_since {
            Some(t) => DEBOUNCE
                .saturating_sub(t.elapsed())
                .max(Duration::from_millis(10)),
            None => Duration::from_millis(200),
        }
    }
}

// ---------------------------------------------------------------------------
// Live event loop (with fs-watch and shell-out)

fn live_event_loop(
    terminal: &mut DefaultTerminal,
    app: &mut App,
    watch: &mut WatchSet,
) -> Result<()> {
    loop {
        terminal.draw(|f| render(f, app))?;

        watch.drain();
        if watch.settled() {
            if let Err(e) = app.refresh_silent() {
                app.status = Some(format!("rescan failed: {}", e));
            }
            watch.clear();
        }

        if event::poll(watch.poll_timeout())? {
            if let Event::Key(k) = event::read()? {
                if k.kind == KeyEventKind::Press
                    && handle_key(app, k.code, k.modifiers)?
                {
                    return Ok(());
                }
            }
        }

        if let Some(action) = app.pending_action.take() {
            process_pending_action(terminal, app, action);
            // The shell-out wrote (or read) the filesystem; the watcher
            // will fire, debounce, and trigger a rescan on the next
            // settled tick. No manual refresh needed here.
        }
    }
}

/// Suspend the alt-screen / raw-mode TUI, run `f`, then re-init.
///
/// Uses `ratatui::restore()` / `ratatui::init()` rather than bespoke
/// alt-screen toggling, per ratatui 0.29 guidance.
fn suspended<F, R>(terminal: &mut DefaultTerminal, f: F) -> R
where
    F: FnOnce() -> R,
{
    ratatui::restore();
    let r = f();
    *terminal = ratatui::init();
    let _ = terminal.clear();
    r
}

fn process_pending_action(
    terminal: &mut DefaultTerminal,
    app: &mut App,
    action: PendingAction,
) {
    match action {
        PendingAction::Submit => {
            let target = app.capture.target.clone();
            let body = app.capture.body.clone();
            let outcome = suspended(terminal, || shell_capture(&target, &body));
            match outcome {
                Ok(()) => {
                    app.status = Some(format!("captured to {}", target));
                    app.capture = CaptureModal::default();
                }
                Err(e) => {
                    // Per the leaf: "No retry on capture failure in v1.
                    // Surface stderr and let the user re-press c."
                    app.status = Some(format!("capture failed: {}", short_err(&e)));
                    app.capture = CaptureModal::default();
                }
            }
        }
        PendingAction::EditBody => {
            let body = app.capture.body.clone();
            let outcome = suspended(terminal, || shell_editor(&body));
            match outcome {
                Ok(new_body) => {
                    app.capture.body = new_body;
                    app.capture.field = CaptureField::Body;
                }
                Err(e) => {
                    app.status = Some(format!("editor failed: {}", short_err(&e)));
                }
            }
        }
        PendingAction::EditObservation { path } => {
            // Seed $EDITOR with the entry's current body, then — only if the
            // user actually changed it to something non-empty — round-trip
            // through `grove-llm inbox-edit`. The drain target grove is the one
            // the detail screen is on (the inbox pane only opens from there).
            let grove = app
                .detail
                .as_ref()
                .map(|d| d.grove.clone())
                .unwrap_or_default();
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            let outcome = suspended(terminal, || shell_edit_observation(&path));
            match outcome {
                Ok(EditOutcome::Unchanged) => {
                    app.status = Some(format!("{}: unchanged", name));
                }
                Ok(EditOutcome::Saved) => {
                    // The fs-watch on `.grove-meta/inboxes` fires on the rename,
                    // debounces, and rescans — the renamed entry then appears.
                    app.status = Some(format!("edited {} in {}", name, grove));
                }
                Err(e) => {
                    // Mirror capture/disposition: surface stderr, no silent retry.
                    app.status = Some(format!("edit failed: {}", short_err(&e)));
                }
            }
        }
        PendingAction::Drain { path, disposition } => {
            // The grove the detail screen is on is the drain target; the
            // picker only opens from a focused detail inbox pane.
            let grove = app
                .detail
                .as_ref()
                .map(|d| d.grove.clone())
                .unwrap_or_default();
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            let outcome = suspended(terminal, || shell_drain(&grove, &path, disposition));
            match outcome {
                Ok(()) => {
                    // The fs-watch on `.grove-meta/inboxes` fires on the delete,
                    // debounces, and rescans — the entry then disappears and the
                    // inbox `ListState` clamps via the render-time bounds check.
                    app.status = Some(format!("{} {}", disposition.label(), name));
                }
                Err(e) => {
                    // Mirror capture's handling: surface stderr, no silent retry.
                    app.status = Some(format!("disposition failed: {}", short_err(&e)));
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Controller (ADR-0016): the dashboard, driven over the proxy seam
//
// This is the same dashboard as `run`/`live_event_loop` above — same `App`,
// `render`, `handle_key`, `WatchSet`, and shell-out writes — but its *driver*
// talks to a dumb `grove __dash-proxy` over a unix-domain socket instead of a
// local crossterm tty. Display flows down (rendered ANSI, framed as
// `DownFrame::Output`); input and resizes flow up (`UpFrame`), decoded into the
// same `KeyCode`/`KeyModifiers` `handle_key` already consumes. Only the
// transport changed; the UI did not. 030 (head binary) will launch zellij and
// place the proxy pane; until then this is exercised by running `grove
// __dash-proxy --socket <path>` against `grove __dash-controller --socket
// <path>` in two terminals.

/// The dashboard's render target over the seam: ratatui rendering into a
/// per-draw `DownFrame::Output` frame on the socket, sized to the proxy.
type ProxyTerminal = Terminal<ProxyBackend<FrameWriter<UnixStream>>>;

/// Drive the dashboard over the proxy seam: bind `socket`, accept one proxy,
/// and run the event loop against it. Functionally identical to `grove tui`,
/// but rendered remotely (ADR-0016). This is the standalone test path; the 030
/// head binary binds the listener itself (before spawning zellij) and calls
/// [`serve`] with the connected stream.
pub fn run_controller(socket: &Path, args: &RepoArgs) -> Result<()> {
    let repo = repo::resolve(args.repo.as_deref())?;
    // A stale socket file would refuse to bind.
    let _ = std::fs::remove_file(socket);
    let listener = UnixListener::bind(socket)
        .with_context(|| format!("binding controller socket {}", socket.display()))?;
    let (stream, _addr) = listener.accept().context("accepting proxy connection")?;
    let outcome = serve(stream, repo);
    // Best-effort: leave no stale socket file behind for the next launch.
    let _ = std::fs::remove_file(socket);
    outcome
}

/// Build the dashboard `App`/`WatchSet` for `repo` and run the controller loop
/// against an already-connected proxy `stream`. Shared by the standalone
/// [`run_controller`] test path and the zellij head-binary launch (leaf 030),
/// which differ only in *how* the proxy connection is obtained.
pub fn serve(stream: UnixStream, repo: PathBuf) -> Result<()> {
    let view = RepoView::scan(&repo)?;
    let preselect = current_grove_name(&repo);
    let mut app = App::new(repo.clone(), view, preselect);
    let mut watch = WatchSet::new(&repo);
    let mut controller = Controller::from_stream(stream)?;
    controller.event_loop(&mut app, &mut watch)
}

/// One connected proxy and the controller-side seam state for driving it.
struct Controller {
    /// Read half of the socket: up-frames (resize / input / editor-done).
    reader: UnixStream,
    /// A second handle for writing `RunEditor` control frames down, distinct
    /// from the render path (`term`'s `FrameWriter`, which only emits `Output`).
    /// Safe to interleave because the loop always flushes `term` (via `draw`)
    /// before processing the input that could enqueue a control frame.
    ctrl: UnixStream,
    /// ratatui render target writing framed output down the socket.
    term: ProxyTerminal,
    /// Up-frame reassembler across partial socket reads.
    frames: UpDecoder,
    /// Raw stdin bytes → key events (crossterm's reader can't read a socket).
    keys: InputDecoder,
}

impl Controller {
    /// Adopt an already-connected proxy `stream`, learn its initial size, and
    /// build the render target. The caller owns the accept (a plain blocking
    /// `accept` in [`run_controller`], or the zellij-watching accept in the 030
    /// head binary) — this only sets up the seam state once a proxy is connected.
    fn from_stream(stream: UnixStream) -> Result<Self> {
        let mut reader = stream.try_clone().context("cloning socket for reading")?;
        let ctrl = stream.try_clone().context("cloning socket for control")?;
        let render_half = stream.try_clone().context("cloning socket for rendering")?;

        let mut frames = UpDecoder::new();
        let size = wait_for_size(&mut reader, &mut frames)?;
        let term = backend::terminal(FrameWriter::new(render_half), size)
            .context("building proxy render target")?;

        Ok(Self {
            reader,
            ctrl,
            term,
            frames,
            keys: InputDecoder::new(),
        })
    }

    /// The driver loop — the seam-driven analogue of `live_event_loop`. Each
    /// tick: render to the proxy, settle the fs-watch debounce, then read the
    /// socket with the same per-tick timeout `event::poll` used, decode any
    /// up-frames, and run a queued shell-out / editor action.
    fn event_loop(&mut self, app: &mut App, watch: &mut WatchSet) -> Result<()> {
        let mut force_redraw = false;
        let mut rbuf = [0u8; 4096];
        loop {
            if force_redraw {
                // A shell-out / editor may have repainted the proxy's tty; force
                // a full redraw so the dashboard reclaims the screen cleanly.
                let _ = self.term.clear();
                force_redraw = false;
            }
            self.term.draw(|f| render(f, app))?;

            watch.drain();
            if watch.settled() {
                if let Err(e) = app.refresh_silent() {
                    app.status = Some(format!("rescan failed: {}", e));
                }
                watch.clear();
            }

            // The `event::poll(timeout)` analogue: a timed-out read means "no
            // input this tick", which is the cue to loop and re-check the watch.
            self.reader.set_read_timeout(Some(watch.poll_timeout()))?;
            let idle = match self.reader.read(&mut rbuf) {
                Ok(0) => return Ok(()), // proxy closed the seam — clean exit.
                Ok(n) => {
                    self.frames.extend(&rbuf[..n]);
                    false
                }
                Err(ref e) if is_timeout(e) => true,
                Err(e) => return Err(e).context("reading proxy seam"),
            };

            let mut quit = false;
            while let Some(frame) = self.frames.next_frame() {
                match frame {
                    UpFrame::Resize { cols, rows } => self.apply_resize(cols, rows),
                    UpFrame::Input(bytes) => {
                        for key in self.keys.feed(&bytes) {
                            if handle_key(app, key.code, key.mods)? {
                                quit = true;
                            }
                        }
                    }
                    // Only awaited synchronously inside `run_editor_on_proxy`; a
                    // stray one here is harmless.
                    UpFrame::EditorDone { .. } => {}
                }
            }
            // On a genuine idle tick, resolve a buffered lone ESC into `Esc`
            // (the decoder's documented controller-driven timeout behaviour).
            if idle {
                for key in self.keys.flush() {
                    if handle_key(app, key.code, key.mods)? {
                        quit = true;
                    }
                }
            }
            if quit {
                return Ok(());
            }

            if let Some(action) = app.pending_action.take() {
                self.process_pending_action(app, action);
                force_redraw = true;
            }
        }
    }

    /// Re-size the proxy's render target after a SIGWINCH-driven resize frame.
    fn apply_resize(&mut self, cols: u16, rows: u16) {
        let size = Size::new(cols, rows);
        self.term.backend_mut().set_size(size);
        let _ = self.term.resize(Rect::new(0, 0, cols, rows));
    }

    /// Run a queued action. The non-interactive `grove-llm` writes run directly
    /// in the controller (no tty needed — the old `suspended()` dance existed
    /// only because the dashboard process used to own the tty). Only the
    /// interactive `$EDITOR` drops route to the proxy's tty (ADR-0017).
    fn process_pending_action(&mut self, app: &mut App, action: PendingAction) {
        match action {
            PendingAction::Submit => {
                let target = app.capture.target.clone();
                let body = app.capture.body.clone();
                match shell_capture(&target, &body) {
                    Ok(()) => {
                        app.status = Some(format!("captured to {}", target));
                        app.capture = CaptureModal::default();
                    }
                    Err(e) => {
                        app.status = Some(format!("capture failed: {}", short_err(&e)));
                        app.capture = CaptureModal::default();
                    }
                }
            }
            PendingAction::EditBody => {
                let body = app.capture.body.clone();
                match self.edit_text_on_proxy(&body) {
                    Ok(new_body) => {
                        app.capture.body = new_body;
                        app.capture.field = CaptureField::Body;
                    }
                    Err(e) => {
                        app.status = Some(format!("editor failed: {}", short_err(&e)));
                    }
                }
            }
            PendingAction::EditObservation { path } => {
                let grove = app
                    .detail
                    .as_ref()
                    .map(|d| d.grove.clone())
                    .unwrap_or_default();
                let name = file_name_of(&path);
                match self.edit_observation_on_proxy(&path) {
                    Ok(EditOutcome::Unchanged) => {
                        app.status = Some(format!("{}: unchanged", name));
                    }
                    Ok(EditOutcome::Saved) => {
                        app.status = Some(format!("edited {} in {}", name, grove));
                    }
                    Err(e) => {
                        app.status = Some(format!("edit failed: {}", short_err(&e)));
                    }
                }
            }
            PendingAction::Drain { path, disposition } => {
                let grove = app
                    .detail
                    .as_ref()
                    .map(|d| d.grove.clone())
                    .unwrap_or_default();
                let name = file_name_of(&path);
                match shell_drain(&grove, &path, disposition) {
                    Ok(()) => {
                        app.status = Some(format!("{} {}", disposition.label(), name));
                    }
                    Err(e) => {
                        app.status = Some(format!("disposition failed: {}", short_err(&e)));
                    }
                }
            }
        }
    }

    /// Edit `initial` in the user's `$EDITOR` on the *proxy's* tty: seed a
    /// tempfile, ask the proxy to run the editor against it, and read the result
    /// back (the tempfile is reachable by both — local-proxy assumption,
    /// ADR-0017). The controller-side analogue of `shell_editor`.
    fn edit_text_on_proxy(&mut self, initial: &str) -> Result<String> {
        let tf = tempfile::Builder::new()
            .prefix("grove-capture-")
            .suffix(".md")
            .tempfile()
            .context("creating editor tempfile")?;
        std::fs::write(tf.path(), initial)
            .with_context(|| format!("seeding editor tempfile {}", tf.path().display()))?;
        if !self.run_editor_on_proxy(tf.path())? {
            anyhow::bail!("editor exited unsuccessfully");
        }
        std::fs::read_to_string(tf.path())
            .with_context(|| format!("reading edited body back from {}", tf.path().display()))
    }

    /// Edit an existing committed observation via the proxy's `$EDITOR`, then
    /// share the change-detection / `inbox-edit` tail with the local path.
    fn edit_observation_on_proxy(&mut self, path: &Path) -> Result<EditOutcome> {
        let current = std::fs::read_to_string(path)
            .with_context(|| format!("reading observation {}", path.display()))?;
        let edited = self.edit_text_on_proxy(&current)?;
        decide_observation_edit(path, &current, edited)
    }

    /// Send a `RunEditor` control frame and block until the proxy reports
    /// `EditorDone`. While the editor owns the proxy's tty, no `Output` frames
    /// are produced (rendering is paused here) and the proxy forwards no input,
    /// so the editor has the terminal to itself. Resizes that arrive mid-edit
    /// are still applied so the post-edit redraw uses the right size.
    fn run_editor_on_proxy(&mut self, path: &Path) -> Result<bool> {
        let frame = DownFrame::RunEditor {
            path: path.to_path_buf(),
        };
        self.ctrl
            .write_all(&frame.encode())
            .context("sending RunEditor to proxy")?;
        self.ctrl.flush().context("flushing RunEditor")?;

        // Block (no read timeout) until the editor child returns.
        self.reader
            .set_read_timeout(None)
            .context("clearing read timeout for editor")?;
        let mut rbuf = [0u8; 4096];
        loop {
            let n = self
                .reader
                .read(&mut rbuf)
                .context("awaiting EditorDone from proxy")?;
            if n == 0 {
                anyhow::bail!("proxy closed the seam while editing");
            }
            self.frames.extend(&rbuf[..n]);
            while let Some(frame) = self.frames.next_frame() {
                match frame {
                    UpFrame::EditorDone { ok } => return Ok(ok),
                    UpFrame::Resize { cols, rows } => self.apply_resize(cols, rows),
                    // The proxy isn't forwarding stdin while the editor runs; a
                    // straggler from before the drop is irrelevant to the edit.
                    UpFrame::Input(_) => {}
                }
            }
        }
    }
}

/// Read up-frames until the proxy's first `Resize` reports its size. Sent on
/// connect by the proxy before anything else (`proxy::send_size`).
fn wait_for_size(reader: &mut UnixStream, frames: &mut UpDecoder) -> Result<Size> {
    let mut rbuf = [0u8; 4096];
    loop {
        let n = reader.read(&mut rbuf).context("reading proxy initial size")?;
        if n == 0 {
            anyhow::bail!("proxy closed before reporting its size");
        }
        frames.extend(&rbuf[..n]);
        while let Some(frame) = frames.next_frame() {
            if let UpFrame::Resize { cols, rows } = frame {
                return Ok(Size::new(cols, rows));
            }
        }
    }
}

/// True when a socket read returned because its timeout elapsed (no data), the
/// `event::poll(timeout) == false` analogue. Platforms differ on which kind a
/// read-timeout surfaces, so accept both.
fn is_timeout(e: &io::Error) -> bool {
    matches!(e.kind(), io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut)
}

/// The trailing filename of a path as an owned `String`, for status messages.
fn file_name_of(p: &Path) -> String {
    p.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default()
}

fn short_err(e: &anyhow::Error) -> String {
    // `{:#}` collapses the anyhow chain into "top: cause: root", which
    // is what the status line needs — a single-line summary that names
    // the actual failure (e.g. "running grove-llm inbox-add: No such
    // file or directory") rather than just the topmost wrapper.
    format!("{:#}", e)
}

// ---------------------------------------------------------------------------
// Shell helpers

/// Locate the sibling `grove-llm` binary next to the running `grove`
/// executable. Falls back to PATH lookup. This handles both
/// `target/debug/` during development and `/usr/local/bin/` after
/// `brew install`, without assuming PATH ordering.
fn find_grove_llm() -> PathBuf {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            let bin = if cfg!(windows) { "grove-llm.exe" } else { "grove-llm" };
            let sibling = parent.join(bin);
            if sibling.is_file() {
                return sibling;
            }
        }
    }
    PathBuf::from("grove-llm")
}

fn shell_capture(target: &str, body: &str) -> Result<()> {
    if target.trim().is_empty() {
        anyhow::bail!("target grove name is empty");
    }
    if body.trim().is_empty() {
        anyhow::bail!("body is empty");
    }
    let tf = tempfile::Builder::new()
        .prefix("grove-capture-")
        .suffix(".md")
        .tempfile()
        .context("creating body tempfile")?;
    std::fs::write(tf.path(), body)
        .with_context(|| format!("writing body to {}", tf.path().display()))?;
    let status = std::process::Command::new(find_grove_llm())
        .arg("inbox-add")
        .arg(format!("--to={}", target.trim()))
        .arg("--body-file")
        .arg(tf.path())
        .status()
        .context("running grove-llm inbox-add")?;
    if !status.success() {
        anyhow::bail!(
            "grove-llm inbox-add exited with status {:?}",
            status.code()
        );
    }
    Ok(())
}

/// Disposition a single observation by shelling out to `grove-llm
/// inbox-drain --for=<grove> --<bucket>=<path>`. The CLI deletes the file and
/// commits (and pushes when a remote is configured); the TUI never touches the
/// `grove-meta` git plumbing directly, mirroring `shell_capture`.
fn shell_drain(grove: &str, path: &Path, disposition: Disposition) -> Result<()> {
    if grove.trim().is_empty() {
        anyhow::bail!("grove name is empty");
    }
    let status = std::process::Command::new(find_grove_llm())
        .arg("inbox-drain")
        .arg(format!("--for={}", grove.trim()))
        .arg(disposition.flag())
        .arg(path)
        .status()
        .context("running grove-llm inbox-drain")?;
    if !status.success() {
        anyhow::bail!(
            "grove-llm inbox-drain exited with status {:?}",
            status.code()
        );
    }
    Ok(())
}

/// The result of an inbox-edit round-trip: either the body was changed and
/// committed, or the user left it untouched (so no verb ran).
enum EditOutcome {
    Saved,
    Unchanged,
}

/// Edit an existing committed observation: read its current body, open it in
/// `$EDITOR`, and — only if the user changed it to something non-empty —
/// rewrite it via `grove-llm inbox-edit --body-file`. The CLI recomputes the
/// content-hash filename, commits, and pushes when a remote is configured; the
/// TUI never touches `grove-meta` git plumbing directly. An empty edited body
/// is rejected (mirroring capture's empty-body guard) rather than producing an
/// empty observation.
fn shell_edit_observation(path: &Path) -> Result<EditOutcome> {
    let current = std::fs::read_to_string(path)
        .with_context(|| format!("reading observation {}", path.display()))?;
    let edited = shell_editor(&current)?;
    decide_observation_edit(path, &current, edited)
}

/// Shared tail of the observation-edit flow: given the original body and the
/// edited body, no-op on no change, reject an empty result, else round-trip
/// through the `grove-llm inbox-edit` verb. Used by both the local-tty
/// `shell_edit_observation` and the controller's proxy-routed editor flow, which
/// differ only in *how* `$EDITOR` is run (local tty vs the proxy's tty).
fn decide_observation_edit(path: &Path, current: &str, edited: String) -> Result<EditOutcome> {
    if edited == current {
        return Ok(EditOutcome::Unchanged);
    }
    if edited.trim().is_empty() {
        anyhow::bail!("edited body is empty; the observation was left unchanged");
    }
    inbox_edit_verb(path, &edited)?;
    Ok(EditOutcome::Saved)
}

/// Rewrite the committed observation at `path` via `grove-llm inbox-edit`. The
/// CLI recomputes the content-hash filename, commits, and pushes when a remote
/// is configured; the dashboard never touches `grove-meta` git plumbing.
fn inbox_edit_verb(path: &Path, edited: &str) -> Result<()> {
    let tf = tempfile::Builder::new()
        .prefix("grove-edit-")
        .suffix(".md")
        .tempfile()
        .context("creating body tempfile")?;
    std::fs::write(tf.path(), edited)
        .with_context(|| format!("writing edited body to {}", tf.path().display()))?;
    let status = std::process::Command::new(find_grove_llm())
        .arg("inbox-edit")
        .arg(path)
        .arg("--body-file")
        .arg(tf.path())
        .status()
        .context("running grove-llm inbox-edit")?;
    if !status.success() {
        anyhow::bail!(
            "grove-llm inbox-edit exited with status {:?}",
            status.code()
        );
    }
    Ok(())
}

fn shell_editor(initial: &str) -> Result<String> {
    let editor = std::env::var("EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .unwrap_or_else(|_| "vi".into());
    let tf = tempfile::Builder::new()
        .prefix("grove-capture-")
        .suffix(".md")
        .tempfile()
        .context("creating editor tempfile")?;
    std::fs::write(tf.path(), initial)
        .with_context(|| format!("seeding editor tempfile {}", tf.path().display()))?;
    let status = std::process::Command::new(&editor)
        .arg(tf.path())
        .status()
        .with_context(|| format!("running editor `{}`", editor))?;
    if !status.success() {
        anyhow::bail!("editor `{}` exited with status {:?}", editor, status.code());
    }
    std::fs::read_to_string(tf.path())
        .with_context(|| format!("reading edited body back from {}", tf.path().display()))
}

/// Returns true when the app should exit.
fn handle_key(app: &mut App, code: KeyCode, mods: KeyModifiers) -> Result<bool> {
    // Help overlay: any key dismisses.
    if app.show_help {
        app.show_help = false;
        return Ok(false);
    }

    // Capture modal swallows almost everything; only Ctrl-C still quits.
    if app.capture.open {
        handle_capture_key(app, code, mods);
        return Ok(false);
    }

    // Disposition picker swallows its keys (i/d/r choose, Esc/Ctrl-C cancel).
    if app.disposition.is_some() {
        handle_disposition_key(app, code, mods);
        return Ok(false);
    }

    // Filter-edit mode swallows almost everything.
    if app.filter.editing {
        match code {
            KeyCode::Esc => {
                app.filter.editing = false;
                app.filter.text.clear();
            }
            KeyCode::Enter => {
                app.filter.editing = false;
            }
            KeyCode::Backspace => {
                app.filter.text.pop();
            }
            KeyCode::Char(c) => {
                app.filter.text.push(c);
            }
            _ => {}
        }
        return Ok(false);
    }

    // Ctrl-C always quits.
    if mods.contains(KeyModifiers::CONTROL) && matches!(code, KeyCode::Char('c')) {
        return Ok(true);
    }

    // Ctrl-E on the detail screen edits the selected inbox observation. Handled
    // ahead of the `(screen, code)` match because that match ignores modifiers,
    // and a bare `e` must not trigger an edit.
    if mods.contains(KeyModifiers::CONTROL)
        && matches!(code, KeyCode::Char('e'))
        && matches!(app.screen, Screen::GroveDetail)
    {
        request_observation_edit(app);
        return Ok(false);
    }

    match (app.screen, code) {
        (_, KeyCode::Char('?')) => {
            app.show_help = true;
        }
        (_, KeyCode::Char('/')) => {
            app.filter.editing = true;
            app.filter.text.clear();
        }
        (_, KeyCode::Char('r')) => {
            app.refresh()?;
        }
        (_, KeyCode::Char('c')) => {
            open_capture_modal(app);
        }
        (Screen::GroveList, KeyCode::Char('q')) => return Ok(true),
        (Screen::GroveList, KeyCode::Down | KeyCode::Char('j')) => {
            let len = app.filtered_groves().len() as isize;
            move_selection(&mut app.list, len, 1);
        }
        (Screen::GroveList, KeyCode::Up | KeyCode::Char('k')) => {
            let len = app.filtered_groves().len() as isize;
            move_selection(&mut app.list, len, -1);
        }
        (Screen::GroveList, KeyCode::Enter) => {
            if let Some(g) = app
                .filtered_groves()
                .get(app.list.selected().unwrap_or(0))
                .map(|g| g.name.clone())
            {
                let mut tree = ListState::default();
                tree.select(Some(0));
                let mut inbox = ListState::default();
                inbox.select(Some(0));
                app.detail = Some(DetailState {
                    grove: g,
                    tree,
                    inbox,
                    right: RightPane::LeafBody,
                    right_scroll: 0,
                });
                app.screen = Screen::GroveDetail;
                app.filter.text.clear();
            }
        }
        // Detail screen.
        (Screen::GroveDetail, KeyCode::Esc | KeyCode::Char('q')) => {
            app.screen = Screen::GroveList;
            app.detail = None;
            app.filter.text.clear();
        }
        (Screen::GroveDetail, KeyCode::Tab) => {
            if let Some(d) = app.detail.as_mut() {
                d.right = d.right.next();
                d.right_scroll = 0;
            }
        }
        (Screen::GroveDetail, KeyCode::Char('d')) => {
            // Disposition the selected inbox entry — only meaningful while the
            // Inbox pane is focused and has a selection (see open helper).
            open_disposition_modal(app);
        }
        (Screen::GroveDetail, KeyCode::Down | KeyCode::Char('j')) => {
            move_detail_selection(app, 1);
        }
        (Screen::GroveDetail, KeyCode::Up | KeyCode::Char('k')) => {
            move_detail_selection(app, -1);
        }
        (Screen::GroveDetail, KeyCode::PageDown) => {
            if let Some(d) = app.detail.as_mut() {
                d.right_scroll = d.right_scroll.saturating_add(10);
            }
        }
        (Screen::GroveDetail, KeyCode::PageUp) => {
            if let Some(d) = app.detail.as_mut() {
                d.right_scroll = d.right_scroll.saturating_sub(10);
            }
        }
        _ => {}
    }
    Ok(false)
}

fn open_capture_modal(app: &mut App) {
    // Pre-fill target from context: on detail, the current grove (jump
    // straight to body); on list, the currently-selected grove if any
    // (let the user edit it, since list-screen capture often means
    // capturing to a *different* grove).
    let (target, field) = match app.screen {
        Screen::GroveDetail => match app.detail.as_ref() {
            Some(d) => (d.grove.clone(), CaptureField::Body),
            None => (String::new(), CaptureField::Target),
        },
        Screen::GroveList => {
            let pre = app
                .filtered_groves()
                .get(app.list.selected().unwrap_or(0))
                .map(|g| g.name.clone())
                .unwrap_or_default();
            (pre, CaptureField::Target)
        }
    };
    app.capture = CaptureModal {
        open: true,
        field,
        target,
        body: String::new(),
    };
}

fn handle_capture_key(app: &mut App, code: KeyCode, mods: KeyModifiers) {
    // Ctrl-C closes the modal rather than quitting the app — the outer
    // handler already short-circuits Ctrl-C to quit *before* we get
    // here, but only when the modal is closed. Inside the modal, the
    // outer handler returned early so the user's Ctrl-C lands here.
    if mods.contains(KeyModifiers::CONTROL) && matches!(code, KeyCode::Char('c')) {
        app.capture = CaptureModal::default();
        return;
    }
    match code {
        KeyCode::Esc => {
            app.capture = CaptureModal::default();
        }
        KeyCode::Tab => {
            app.capture.field = match app.capture.field {
                CaptureField::Target => CaptureField::Body,
                CaptureField::Body => CaptureField::Target,
            };
        }
        KeyCode::Char('e') if mods.contains(KeyModifiers::CONTROL) => {
            // Drop to $EDITOR with the current body. Live loop picks
            // this up after the key handler returns.
            app.pending_action = Some(PendingAction::EditBody);
        }
        KeyCode::Char('s') if mods.contains(KeyModifiers::CONTROL) => {
            // The deliberate submit gesture. Distinct from `Enter` so a
            // pasted (or typed) newline in the body cannot fire submit by
            // accident; works from either field once both are filled.
            if !app.capture.target.trim().is_empty()
                && !app.capture.body.trim().is_empty()
            {
                app.pending_action = Some(PendingAction::Submit);
            }
        }
        KeyCode::Enter => match app.capture.field {
            CaptureField::Target => {
                if !app.capture.target.trim().is_empty() {
                    app.capture.field = CaptureField::Body;
                }
            }
            CaptureField::Body => {
                // `Enter` in the body inserts a newline rather than
                // submitting — multi-line observations are typed and
                // pasted here, and submit is the deliberate Ctrl-S above.
                app.capture.body.push('\n');
            }
        },
        KeyCode::Backspace => match app.capture.field {
            CaptureField::Target => {
                app.capture.target.pop();
            }
            CaptureField::Body => {
                app.capture.body.pop();
            }
        },
        KeyCode::Char(c) if !mods.contains(KeyModifiers::CONTROL) => {
            match app.capture.field {
                CaptureField::Target => app.capture.target.push(c),
                CaptureField::Body => app.capture.body.push(c),
            }
        }
        _ => {}
    }
}

/// Open the disposition picker for the currently-selected inbox entry. No-op
/// unless the Inbox pane is focused and holds at least one observation — `d`
/// is inert from any other pane or an empty inbox.
fn open_disposition_modal(app: &mut App) {
    let modal = {
        let Some(d) = app.detail.as_ref() else {
            return;
        };
        if d.right != RightPane::Inbox {
            return;
        }
        let Some(gd) = app.view.grove(&d.grove) else {
            return;
        };
        if gd.inbox.is_empty() {
            return;
        }
        // Clamp the selection the same way the renderer does, so the picker
        // always targets a real entry even if the cursor drifted past the end.
        let idx = d.inbox.selected().unwrap_or(0).min(gd.inbox.len() - 1);
        let path = gd.inbox[idx].clone();
        let entry = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        DispositionModal { path, entry }
    };
    app.disposition = Some(modal);
}

/// Request an `$EDITOR` round-trip for the currently-selected inbox entry. No-op
/// unless the Inbox pane is focused and holds at least one observation — Ctrl-E
/// is inert from any other pane or an empty inbox, mirroring `d`.
fn request_observation_edit(app: &mut App) {
    let path = {
        let Some(d) = app.detail.as_ref() else {
            return;
        };
        if d.right != RightPane::Inbox {
            return;
        }
        let Some(gd) = app.view.grove(&d.grove) else {
            return;
        };
        if gd.inbox.is_empty() {
            return;
        }
        // Clamp the selection the same way the renderer does, so we always
        // target a real entry even if the cursor drifted past the end.
        let idx = d.inbox.selected().unwrap_or(0).min(gd.inbox.len() - 1);
        gd.inbox[idx].clone()
    };
    app.pending_action = Some(PendingAction::EditObservation { path });
}

/// Handle a key while the disposition picker is open: `i`/`d`/`r` choose a
/// bucket and request the drain; `Esc`/`Ctrl-C` cancel. Mirrors the capture
/// modal's "decide here, run in the loop" split, so it stays unit-testable.
fn handle_disposition_key(app: &mut App, code: KeyCode, mods: KeyModifiers) {
    if mods.contains(KeyModifiers::CONTROL) && matches!(code, KeyCode::Char('c')) {
        app.disposition = None;
        return;
    }
    let disposition = match code {
        KeyCode::Esc => {
            app.disposition = None;
            return;
        }
        KeyCode::Char('i') => Disposition::Incorporated,
        KeyCode::Char('d') => Disposition::Deferred,
        KeyCode::Char('r') => Disposition::Rejected,
        _ => return,
    };
    if let Some(modal) = app.disposition.take() {
        app.pending_action = Some(PendingAction::Drain {
            path: modal.path,
            disposition,
        });
    }
}

fn move_selection(state: &mut ListState, len: isize, delta: isize) {
    if len <= 0 {
        state.select(None);
        return;
    }
    let cur = state.selected().unwrap_or(0) as isize;
    let next = (cur + delta).rem_euclid(len) as usize;
    state.select(Some(next));
}

fn flat_rows_len(app: &App) -> usize {
    flatten_for(app).len()
}

fn inbox_len(app: &App) -> usize {
    app.detail
        .as_ref()
        .and_then(|d| app.view.grove(&d.grove))
        .map(|gd| gd.inbox.len())
        .unwrap_or(0)
}

/// Move the selection in whichever detail-screen pane currently owns `j`/`k`:
/// the inbox list while the Inbox pane is focused, the task tree otherwise.
/// Lengths are read before the mutable borrow of `app.detail` to avoid
/// aliasing `app.view`.
fn move_detail_selection(app: &mut App, delta: isize) {
    let rows = flat_rows_len(app);
    let inbox = inbox_len(app);
    if let Some(d) = app.detail.as_mut() {
        if d.right == RightPane::Inbox {
            move_selection(&mut d.inbox, inbox as isize, delta);
        } else {
            move_selection(&mut d.tree, rows as isize, delta);
        }
        d.right_scroll = 0;
    }
}

// ---------------------------------------------------------------------------
// Rendering

pub fn render(f: &mut Frame, app: &App) {
    let area = f.area();
    // A one-row header (cli + repo versions) sits above the body on both
    // screens; the footer keeps its keyhint role below.
    let [header, main, footer] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(1),
        Constraint::Length(1),
    ])
    .areas(area);

    render_header(f, header, app);
    match app.screen {
        Screen::GroveList => render_grove_list(f, main, app),
        Screen::GroveDetail => render_grove_detail(f, main, app),
    }
    render_footer(f, footer, app);

    if app.capture.open {
        render_capture_modal(f, area, &app.capture);
    }
    if let Some(modal) = app.disposition.as_ref() {
        render_disposition_modal(f, area, modal);
    }
    if app.show_help {
        render_help_overlay(f, area);
    }
}

/// A small centred picker listing the three [[Drain]] buckets and their
/// hotkeys. Purely presentational — the keys are handled in
/// `handle_disposition_key`.
fn render_disposition_modal(f: &mut Frame, area: Rect, modal: &DispositionModal) {
    let popup = centered_rect(60, 30, area);
    f.render_widget(Clear, popup);

    let block = Block::default()
        .borders(Borders::ALL)
        .title("disposition — grove inbox drain");
    let inner = block.inner(popup);
    f.render_widget(block, popup);

    let lines = vec![
        Line::from(Span::styled(
            modal.entry.clone(),
            Style::default().fg(Color::Yellow),
        )),
        Line::from(""),
        Line::from("  i  incorporated"),
        Line::from("  d  deferred"),
        Line::from("  r  rejected"),
        Line::from(""),
        Line::from(Span::styled(
            "Esc cancel",
            Style::default().fg(Color::DarkGray),
        )),
    ];
    f.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), inner);
}

fn render_capture_modal(f: &mut Frame, area: Rect, modal: &CaptureModal) {
    let popup = centered_rect(70, 50, area);
    f.render_widget(Clear, popup);

    let block = Block::default()
        .borders(Borders::ALL)
        .title("capture — grove inbox add");

    let inner = block.inner(popup);
    f.render_widget(block, popup);

    let [target_area, body_area, hint_area] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(3),
        Constraint::Length(1),
    ])
    .areas(inner);

    let target_focus = matches!(modal.field, CaptureField::Target);
    let body_focus = matches!(modal.field, CaptureField::Body);

    let target_text = if target_focus {
        format!("{}_", modal.target)
    } else {
        modal.target.clone()
    };
    let target_style = if target_focus {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    let target_para = Paragraph::new(target_text)
        .style(target_style)
        .block(Block::default().borders(Borders::ALL).title("target grove"));
    f.render_widget(target_para, target_area);

    let body_text = if body_focus {
        format!("{}_", modal.body)
    } else {
        modal.body.clone()
    };
    let body_style = if body_focus {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    let body_para = Paragraph::new(body_text)
        .style(body_style)
        .wrap(Wrap { trim: false })
        .block(Block::default().borders(Borders::ALL).title("observation body"));
    f.render_widget(body_para, body_area);

    let hint =
        "Tab=switch  Enter=next(target)/newline(body)  Ctrl-S=submit  Ctrl-E=$EDITOR  Esc=cancel";
    f.render_widget(Paragraph::new(hint), hint_area);
}

/// Build the header line — the `cli` layer plus each installed harness's
/// `repo` layer — as styled spans. On a `cli`-vs-`repo` mismatch the affected
/// `repo` token is styled via [`DRIFT_STYLE`] to draw the eye, using the same
/// string-equality drift rule as the rows and `grove status`. In multi-harness
/// repos each `repo` token is disambiguated as `repo[<name>]=…`.
fn header_spans(
    cli: &str,
    repo_versions: &BTreeMap<&'static str, Option<String>>,
) -> Vec<Span<'static>> {
    let mut spans = vec![Span::raw(format!("grove cli={}", cli))];
    if repo_versions.is_empty() {
        spans.push(Span::raw("  ·  (not installed)"));
        return spans;
    }
    let multi = repo_versions.len() > 1;
    for (name, ver) in repo_versions {
        spans.push(Span::raw("  ·  "));
        let label = if multi {
            format!("repo[{}]=", name)
        } else {
            "repo=".to_string()
        };
        match ver {
            Some(v) => {
                let style = if v != cli { DRIFT_STYLE } else { Style::default() };
                spans.push(Span::styled(format!("{}{}", label, v), style));
            }
            None => spans.push(Span::raw(format!("{}(unknown)", label))),
        }
    }
    spans
}

fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let spans = header_spans(app.view.cli_version(), app.view.repo_versions());
    f.render_widget(Paragraph::new(Line::from(spans)), area);
}

fn render_grove_list(f: &mut Frame, area: Rect, app: &App) {
    let filtered = app.filtered_groves();
    let cli = app.view.cli_version();
    let repo_versions = app.view.repo_versions();
    let items: Vec<ListItem> = filtered
        .iter()
        .map(|g| ListItem::new(grove_row(g, cli, repo_versions)))
        .collect();
    let title = if app.filter.text.is_empty() {
        "groves".to_string()
    } else {
        format!("groves  /{}", app.filter.text)
    };
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
    let mut state = app.list.clone();
    // Clamp selection in case the filter narrowed the list.
    if let Some(sel) = state.selected() {
        if sel >= filtered.len() && !filtered.is_empty() {
            state.select(Some(filtered.len() - 1));
        } else if filtered.is_empty() {
            state.select(None);
        }
    }
    f.render_stateful_widget(list, area, &mut state);
}

fn grove_row(
    g: &GroveSummary,
    cli: &str,
    repo_versions: &BTreeMap<&'static str, Option<String>>,
) -> Line<'static> {
    let badge = match g.lifecycle {
        Lifecycle::Live => Span::styled(" live ", Style::default().fg(Color::Green)),
        Lifecycle::Seed => Span::styled(" seed ", Style::default().fg(Color::Yellow)),
    };
    let mut spans = vec![
        badge,
        Span::raw(" "),
        Span::raw(g.name.clone()),
        Span::raw(format!("  leaves:{}/{}", g.live_leaves, g.retired_leaves)),
    ];
    // The `worktree` layer, one segment per relevant harness (Seeds carry
    // none). Prefix the harness only when this grove spans more than one.
    let multi = g.worktree_versions.len() > 1;
    for (harness, worktree) in &g.worktree_versions {
        let repo = repo_versions.get(harness).cloned().flatten();
        spans.push(Span::raw("  "));
        spans.extend(worktree_spans(
            multi.then_some(*harness),
            worktree,
            &repo,
            cli,
        ));
    }
    if g.inbox_pending > 0 {
        spans.push(Span::styled(
            format!("  inbox:{}", g.inbox_pending),
            Style::default().fg(Color::Cyan),
        ));
    }
    Line::from(spans)
}

/// Build the trailing `worktree=…` version segment for a grove row as styled
/// spans. Mirrors `status::version_segment`'s rule exactly — plain
/// string-equality drift, `(unknown)` for a missing stamp, `repo=(none)` for
/// an orphan — but colours the `⚠` markers via [`DRIFT_STYLE`] instead of
/// emitting plain text. `harness` is `Some` only in multi-harness repos, where
/// it disambiguates the segment as `worktree[<name>]=…`.
fn worktree_spans(
    harness: Option<&str>,
    worktree: &Option<String>,
    repo: &Option<String>,
    cli: &str,
) -> Vec<Span<'static>> {
    let key = match harness {
        Some(h) => format!("worktree[{}]=", h),
        None => "worktree=".to_string(),
    };
    let Some(wt) = worktree else {
        return vec![Span::raw(format!("{}(unknown)", key))];
    };
    let mut spans = vec![Span::raw(format!("{}{}", key, wt))];
    match repo {
        None => spans.push(Span::raw(" repo=(none)")),
        Some(r) if r != wt => spans.push(Span::styled(format!(" ⚠ repo={}", r), DRIFT_STYLE)),
        Some(_) => {}
    }
    if cli != wt {
        spans.push(Span::styled(format!(" ⚠ cli={}", cli), DRIFT_STYLE));
    }
    spans
}

fn render_grove_detail(f: &mut Frame, area: Rect, app: &App) {
    let Some(detail) = app.detail.as_ref() else {
        return;
    };
    let summary_name = detail.grove.clone();
    let grove_detail = app.view.grove(&summary_name);

    let [left, right] =
        Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)]).areas(area);

    // Left pane: task tree.
    let rows = flatten_for(app);
    let items: Vec<ListItem> = rows
        .iter()
        .map(|r| {
            let indent = "  ".repeat(r.depth);
            let style = if r.is_retired {
                Style::default().fg(Color::DarkGray)
            } else if r.is_node {
                Style::default().add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(Line::from(Span::styled(format!("{}{}", indent, r.name), style)))
        })
        .collect();
    let left_title = if app.filter.text.is_empty() {
        format!("{} — tree", summary_name)
    } else {
        format!("{} — tree  /{}", summary_name, app.filter.text)
    };
    let left_list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(left_title))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
    let mut tree_state = detail.tree.clone();
    if let Some(sel) = tree_state.selected() {
        if rows.is_empty() {
            tree_state.select(None);
        } else if sel >= rows.len() {
            tree_state.select(Some(rows.len() - 1));
        }
    }
    f.render_stateful_widget(left_list, left, &mut tree_state);

    // Right pane. The inbox pane is a selectable list over a body view; the
    // other panes are a single scrollable paragraph.
    if detail.right == RightPane::Inbox {
        render_inbox_pane(f, right, app, grove_detail);
    } else {
        let (title, body) = right_pane_content(app, grove_detail, &rows);
        let para = Paragraph::new(body)
            .block(Block::default().borders(Borders::ALL).title(title))
            .wrap(Wrap { trim: false })
            .scroll((detail.right_scroll, 0));
        f.render_widget(para, right);
    }
}

/// Render the inbox right-pane: a selectable list of pending observation
/// filenames on top, the selected entry's body below. Bodies are read on
/// demand (`repo_view::read_path`); the scan loads none.
fn render_inbox_pane(f: &mut Frame, area: Rect, app: &App, detail: Option<&GroveDetail>) {
    let Some(d) = app.detail.as_ref() else {
        return;
    };
    let Some(detail) = detail else {
        let para = Paragraph::new(String::new())
            .block(Block::default().borders(Borders::ALL).title("inbox — no snapshot"));
        f.render_widget(para, area);
        return;
    };

    let title = format!("inbox ({})", detail.inbox.len());
    if detail.inbox.is_empty() {
        let para = Paragraph::new("(no pending observations)")
            .block(Block::default().borders(Borders::ALL).title(title))
            .wrap(Wrap { trim: false });
        f.render_widget(para, area);
        return;
    }

    let [list_area, body_area] =
        Layout::vertical([Constraint::Percentage(40), Constraint::Min(0)]).areas(area);

    // List of observation filenames.
    let items: Vec<ListItem> = detail
        .inbox
        .iter()
        .map(|p| {
            let name = p
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            ListItem::new(Line::from(name))
        })
        .collect();
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
    let mut state = d.inbox.clone();
    if let Some(sel) = state.selected() {
        if sel >= detail.inbox.len() {
            state.select(Some(detail.inbox.len() - 1));
        }
    } else {
        state.select(Some(0));
    }
    f.render_stateful_widget(list, list_area, &mut state);

    // Body of the selected entry.
    let body = state
        .selected()
        .and_then(|i| detail.inbox.get(i))
        .map(|p| repo_view::read_path(p).unwrap_or_else(|e| format!("(error: {})", e)))
        .unwrap_or_default();
    let body_para = Paragraph::new(body)
        .block(Block::default().borders(Borders::ALL).title("body"))
        .wrap(Wrap { trim: false })
        .scroll((d.right_scroll, 0));
    f.render_widget(body_para, body_area);
}

fn right_pane_content(
    app: &App,
    detail: Option<&GroveDetail>,
    rows: &[FlatRow],
) -> (String, String) {
    let Some(d) = app.detail.as_ref() else {
        return ("right".into(), String::new());
    };
    let Some(detail) = detail else {
        return (format!("{} — no snapshot", d.right.label()), String::new());
    };
    let selected = d
        .tree
        .selected()
        .and_then(|i| rows.get(i));

    match d.right {
        RightPane::LeafBody => {
            let Some(row) = selected else {
                return ("leaf".into(), "(no task tree)".into());
            };
            if row.is_node {
                (
                    format!("leaf — {}", row.name),
                    format!("(node selected: switch via Tab to see its BRIEF)"),
                )
            } else {
                let body = repo_view::read_path(&row.path).unwrap_or_else(|e| format!("(error: {})", e));
                (format!("leaf — {}", row.name), body)
            }
        }
        // The inbox pane is rendered by `render_inbox_pane` (selectable list +
        // body), not as a single paragraph, so it never reaches here.
        RightPane::Inbox => unreachable!("inbox pane is rendered by render_inbox_pane"),
        RightPane::Brief => {
            let brief_path = enclosing_brief(detail, selected);
            match brief_path {
                Some(p) => {
                    let body = repo_view::read_path(&p)
                        .unwrap_or_else(|e| format!("(error: {})", e));
                    (
                        format!("brief — {}", p.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default()),
                        body,
                    )
                }
                None => ("brief".into(), "(no BRIEF.md in scope)".into()),
            }
        }
    }
}

fn enclosing_brief(detail: &GroveDetail, selected: Option<&FlatRow>) -> Option<PathBuf> {
    let tree = detail.task_tree.as_ref()?;
    let row = match selected {
        Some(r) => r,
        None => return tree.root_brief.clone(),
    };
    // If selected is itself a node with a brief, use that.
    if row.is_node {
        if let Some(p) = &row.node_brief {
            return Some(p.clone());
        }
    }
    // Otherwise the enclosing-node brief, or the root brief if at top.
    row.parent_brief.clone().or_else(|| tree.root_brief.clone())
}

fn render_footer(f: &mut Frame, area: Rect, app: &App) {
    let mut spans = Vec::new();
    if app.filter.editing {
        spans.push(Span::styled(
            format!("/{}_", app.filter.text),
            Style::default().add_modifier(Modifier::REVERSED),
        ));
        spans.push(Span::raw("  Enter=apply  Esc=cancel"));
    } else {
        let hint = match app.screen {
            Screen::GroveList => "Enter=open  j/k=move  /=filter  c=capture  r=refresh  ?=help  q=quit",
            Screen::GroveDetail => "Tab=cycle  j/k=move  PgUp/PgDn=scroll  /=filter  c=capture  r=refresh  Esc=back  ?=help",
        };
        spans.push(Span::raw(hint));
        if !app.filter.text.is_empty() {
            spans.push(Span::raw("   "));
            spans.push(Span::styled(
                format!("filter: /{}", app.filter.text),
                Style::default().fg(Color::Yellow),
            ));
        }
        if let Some(s) = app.status.as_deref() {
            spans.push(Span::raw("   "));
            spans.push(Span::styled(s.to_string(), Style::default().fg(Color::Green)));
        }
    }
    f.render_widget(Paragraph::new(Line::from(spans)), area);
}

fn render_help_overlay(f: &mut Frame, area: Rect) {
    let lines = vec![
        Line::from(Span::styled(
            "grove tui — keybindings",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("  ↑/↓ or j/k    move selection"),
        Line::from("  Enter         drill into grove (list screen)"),
        Line::from("  Esc / q       back / quit"),
        Line::from("  Tab           cycle right pane (leaf → inbox → brief)"),
        Line::from("                  in the inbox pane, j/k select an observation"),
        Line::from("  d             disposition the selected observation"),
        Line::from("                  (i=incorporated, d=deferred, r=rejected, Esc=cancel)"),
        Line::from("  Ctrl-E        edit the selected observation's body in $EDITOR"),
        Line::from("  PgUp / PgDn   scroll right pane"),
        Line::from("  /             filter current pane (Enter=apply, Esc=cancel)"),
        Line::from("  c             capture an observation to a grove's inbox"),
        Line::from("                  Tab=switch field, Ctrl-E=edit in $EDITOR,"),
        Line::from("                  Enter on body=newline, Ctrl-S=submit, Esc=cancel"),
        Line::from("  r             rescan the repo (also: fs-watch auto-refreshes)"),
        Line::from("  ?             toggle this help"),
        Line::from("  Ctrl-C        force quit"),
        Line::from(""),
        Line::from("  (any key dismisses help)"),
    ];
    let block = Block::default()
        .borders(Borders::ALL)
        .title("help");
    let para = Paragraph::new(lines).block(block).wrap(Wrap { trim: false });
    let popup = centered_rect(60, 60, area);
    f.render_widget(Clear, popup);
    f.render_widget(para, popup);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1]);
    horizontal[1]
}

// ---------------------------------------------------------------------------
// Task-tree flattening
//
// The data layer's `TaskTree` is nested; the list widget wants a flat
// sequence with depth. We flatten depth-first, apply the substring
// filter, and decorate each row with the bits of context the right pane
// later needs (the row's own `BRIEF.md` if it's a node, the enclosing
// node's `BRIEF.md` otherwise) so right-pane rendering does not have to
// re-walk the tree.

#[derive(Debug, Clone)]
struct FlatRow {
    depth: usize,
    name: String,
    path: PathBuf,
    is_node: bool,
    is_retired: bool,
    node_brief: Option<PathBuf>,
    parent_brief: Option<PathBuf>,
}

fn flatten_for(app: &App) -> Vec<FlatRow> {
    let Some(d) = app.detail.as_ref() else {
        return Vec::new();
    };
    let Some(detail) = app.view.grove(&d.grove) else {
        return Vec::new();
    };
    let Some(tree) = detail.task_tree.as_ref() else {
        return Vec::new();
    };
    let needle = app.filter.text.to_lowercase();
    let mut rows = Vec::new();
    for entry in &tree.entries {
        push_entry(entry, 0, tree.root_brief.as_deref(), &needle, &mut rows);
    }
    rows
}

fn push_entry(
    entry: &TaskEntry,
    depth: usize,
    parent_brief: Option<&Path>,
    needle: &str,
    out: &mut Vec<FlatRow>,
) {
    let matches = needle.is_empty() || entry.name.to_lowercase().contains(needle);
    match &entry.kind {
        TaskKind::Leaf => {
            if matches {
                out.push(FlatRow {
                    depth,
                    name: entry.name.clone(),
                    path: entry.path.clone(),
                    is_node: false,
                    is_retired: entry.is_retired,
                    node_brief: None,
                    parent_brief: parent_brief.map(Path::to_path_buf),
                });
            }
        }
        TaskKind::Node { brief, children } => {
            if matches {
                out.push(FlatRow {
                    depth,
                    name: entry.name.clone(),
                    path: entry.path.clone(),
                    is_node: true,
                    is_retired: entry.is_retired,
                    node_brief: brief.clone(),
                    parent_brief: parent_brief.map(Path::to_path_buf),
                });
            }
            let next_parent_brief = brief.as_deref().or(parent_brief);
            for c in children {
                push_entry(c, depth + 1, next_parent_brief, needle, out);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Pre-selection: which grove is `cwd` inside?

fn current_grove_name(repo: &Path) -> Option<String> {
    let cwd = std::env::current_dir().ok()?;
    let top = repo::git_toplevel(&cwd).ok()?;
    let worktrees = repo.join(".grove-worktrees");
    let rel = top.strip_prefix(&worktrees).ok()?;
    rel.components()
        .next()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
}

// ---------------------------------------------------------------------------
// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;
    use std::fs;
    use tempfile::TempDir;

    fn touch(p: &Path, body: &str) {
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(p, body).unwrap();
    }

    fn fixture_repo() -> TempDir {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        // Live grove "alpha" with a node, a leaf, and a retired leaf.
        let alpha = root.join(".grove-worktrees/alpha/.grove");
        touch(&alpha.join("BRIEF.md"), "# alpha — brief\n\nGoal: ship.\n");
        touch(&alpha.join("010-first.md"), "# 010-first\n\nWork here.\n");
        touch(&alpha.join("020-node/BRIEF.md"), "# 020-node — brief\n");
        touch(&alpha.join("020-node/010-child.md"), "# 010-child\n");
        touch(&alpha.join("done/000-old.md"), "# old\n");
        // alpha has two pending inbox observations (sorted chronologically).
        let alpha_inbox = root.join(".grove-meta/inboxes/alpha");
        fs::create_dir_all(&alpha_inbox).unwrap();
        touch(
            &alpha_inbox.join("2026-05-27T09-00-00Z-aaa-1111.md"),
            "first alpha observation body\n",
        );
        touch(
            &alpha_inbox.join("2026-05-28T09-00-00Z-bbb-2222.md"),
            "second alpha observation body\n",
        );
        // Seed for grove "beta": inbox only, no worktree.
        let beta_inbox = root.join(".grove-meta/inboxes/beta");
        fs::create_dir_all(&beta_inbox).unwrap();
        touch(
            &beta_inbox.join("2026-05-28T10-00-00Z--note.md"),
            "first observation\n",
        );
        // Live grove "gamma" with a leaf but an empty inbox (no inbox dir) —
        // the substrate for "disposition is a no-op when the inbox is empty".
        let gamma = root.join(".grove-worktrees/gamma/.grove");
        touch(&gamma.join("BRIEF.md"), "# gamma — brief\n");
        touch(&gamma.join("010-task.md"), "# 010-task\n\nWork.\n");
        tmp
    }

    fn render_to_buffer(app: &App, width: u16, height: u16) -> String {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|f| render(f, app)).unwrap();
        let buf = terminal.backend().buffer().clone();
        // Concatenate row contents into a newline-delimited string.
        let mut s = String::new();
        for y in 0..buf.area.height {
            for x in 0..buf.area.width {
                s.push_str(buf[(x, y)].symbol());
            }
            s.push('\n');
        }
        s
    }

    #[test]
    fn grove_list_renders_live_and_seed() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let app = App::new(tmp.path().to_path_buf(), view, None);
        let out = render_to_buffer(&app, 80, 12);
        assert!(out.contains("alpha"), "grove name missing:\n{}", out);
        assert!(out.contains("beta"), "seed grove name missing:\n{}", out);
        assert!(out.contains("live"), "live badge missing:\n{}", out);
        assert!(out.contains("seed"), "seed badge missing:\n{}", out);
        assert!(out.contains("inbox:1"), "inbox count missing:\n{}", out);
        // Footer hints.
        assert!(out.contains("Enter=open"), "footer missing:\n{}", out);
    }

    #[test]
    fn preselection_targets_named_grove() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let app = App::new(
            tmp.path().to_path_buf(),
            view,
            Some("beta".into()),
        );
        assert_eq!(app.list.selected(), Some(1)); // alphabetical: alpha=0, beta=1
    }

    #[test]
    fn enter_drills_into_detail_and_tree_is_visible() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        assert_eq!(app.screen, Screen::GroveDetail);
        let out = render_to_buffer(&app, 100, 16);
        assert!(out.contains("alpha"), "title missing:\n{}", out);
        assert!(out.contains("010-first.md"), "leaf row missing:\n{}", out);
        assert!(out.contains("020-node"), "node row missing:\n{}", out);
        assert!(out.contains("done"), "done row missing:\n{}", out);
        // Right pane defaults to LeafBody — should show first-leaf body.
        assert!(out.contains("Work here."), "leaf body missing:\n{}", out);
    }

    #[test]
    fn tab_cycles_right_pane_to_brief() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        // leaf → inbox: alpha has pending observations, so the list + body render.
        handle_key(&mut app, KeyCode::Tab, KeyModifiers::NONE).unwrap();
        let out = render_to_buffer(&app, 100, 16);
        assert!(out.contains("inbox (2)"), "inbox count title missing:\n{}", out);
        assert!(out.contains("aaa-1111"), "observation filename missing:\n{}", out);
        // inbox → brief
        handle_key(&mut app, KeyCode::Tab, KeyModifiers::NONE).unwrap();
        let out = render_to_buffer(&app, 100, 16);
        assert!(out.contains("alpha — brief"), "root brief missing:\n{}", out);
    }

    #[test]
    fn inbox_pane_jk_moves_inbox_selection_not_tree() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        // Focus the inbox pane.
        handle_key(&mut app, KeyCode::Tab, KeyModifiers::NONE).unwrap();
        let tree_before = app.detail.as_ref().unwrap().tree.selected();
        assert_eq!(app.detail.as_ref().unwrap().inbox.selected(), Some(0));
        // j moves the inbox selection, leaving the tree cursor untouched.
        handle_key(&mut app, KeyCode::Char('j'), KeyModifiers::NONE).unwrap();
        let d = app.detail.as_ref().unwrap();
        assert_eq!(d.inbox.selected(), Some(1), "inbox selection should advance");
        assert_eq!(d.tree.selected(), tree_before, "tree selection must not move");
        // The selected entry's body renders.
        let out = render_to_buffer(&app, 100, 20);
        assert!(
            out.contains("second alpha observation body"),
            "selected body missing:\n{}",
            out
        );
    }

    #[test]
    fn inbox_selection_survives_pane_cycle() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Tab, KeyModifiers::NONE).unwrap(); // → inbox
        handle_key(&mut app, KeyCode::Char('j'), KeyModifiers::NONE).unwrap(); // select #1
        // Cycle all the way around: inbox → brief → leaf → inbox.
        handle_key(&mut app, KeyCode::Tab, KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Tab, KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Tab, KeyModifiers::NONE).unwrap();
        assert_eq!(
            app.detail.as_ref().unwrap().inbox.selected(),
            Some(1),
            "inbox selection must persist across Tab cycling"
        );
    }

    /// Drive the detail screen into the focused inbox pane, ready for a
    /// disposition keystroke. Returns the app preselected on `alpha` (which
    /// the fixture gives two pending observations).
    fn app_focused_on_alpha_inbox(tmp: &TempDir) -> App {
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap(); // → detail
        handle_key(&mut app, KeyCode::Tab, KeyModifiers::NONE).unwrap(); // → inbox pane
        app
    }

    #[test]
    fn d_on_inbox_entry_opens_disposition_picker() {
        let tmp = fixture_repo();
        let mut app = app_focused_on_alpha_inbox(&tmp);
        handle_key(&mut app, KeyCode::Char('d'), KeyModifiers::NONE).unwrap();
        assert!(app.disposition.is_some(), "disposition picker should open");
        assert_eq!(app.pending_action, None, "opening the picker is not yet an action");
    }

    #[test]
    fn disposition_choice_requests_drain_with_path_and_bucket() {
        let tmp = fixture_repo();
        let mut app = app_focused_on_alpha_inbox(&tmp);
        // Select the second observation, then disposition it as rejected.
        handle_key(&mut app, KeyCode::Char('j'), KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('d'), KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('r'), KeyModifiers::NONE).unwrap();
        match app.pending_action.as_ref() {
            Some(PendingAction::Drain { path, disposition }) => {
                assert_eq!(*disposition, Disposition::Rejected);
                assert!(
                    path.ends_with("2026-05-28T09-00-00Z-bbb-2222.md"),
                    "drain should target the selected entry, got {:?}",
                    path
                );
            }
            other => panic!("expected Drain pending action, got {:?}", other),
        }
        assert!(app.disposition.is_none(), "picker closes once a bucket is chosen");
    }

    #[test]
    fn disposition_hotkeys_map_to_each_bucket() {
        for (key, expected) in [
            ('i', Disposition::Incorporated),
            ('d', Disposition::Deferred),
            ('r', Disposition::Rejected),
        ] {
            let tmp = fixture_repo();
            let mut app = app_focused_on_alpha_inbox(&tmp);
            handle_key(&mut app, KeyCode::Char('d'), KeyModifiers::NONE).unwrap(); // open
            handle_key(&mut app, KeyCode::Char(key), KeyModifiers::NONE).unwrap(); // choose
            match app.pending_action.as_ref() {
                Some(PendingAction::Drain { disposition, .. }) => {
                    assert_eq!(*disposition, expected, "key {:?} → wrong bucket", key);
                }
                other => panic!("key {:?}: expected Drain, got {:?}", key, other),
            }
        }
    }

    #[test]
    fn ctrl_e_on_inbox_entry_requests_edit_with_selected_path() {
        let tmp = fixture_repo();
        let mut app = app_focused_on_alpha_inbox(&tmp);
        // Select the second observation, then Ctrl-E to edit it.
        handle_key(&mut app, KeyCode::Char('j'), KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('e'), KeyModifiers::CONTROL).unwrap();
        match app.pending_action.as_ref() {
            Some(PendingAction::EditObservation { path }) => {
                assert!(
                    path.ends_with("2026-05-28T09-00-00Z-bbb-2222.md"),
                    "edit should target the selected entry, got {:?}",
                    path
                );
            }
            other => panic!("expected EditObservation pending action, got {:?}", other),
        }
    }

    #[test]
    fn ctrl_e_is_noop_when_inbox_empty() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        // gamma is a live grove with no pending observations.
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("gamma".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Tab, KeyModifiers::NONE).unwrap(); // → inbox pane (empty)
        handle_key(&mut app, KeyCode::Char('e'), KeyModifiers::CONTROL).unwrap();
        assert_eq!(app.pending_action, None, "no edit for an empty inbox");
    }

    #[test]
    fn ctrl_e_is_noop_when_inbox_pane_not_focused() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        // Right pane defaults to LeafBody — Ctrl-E must not request an edit.
        handle_key(&mut app, KeyCode::Char('e'), KeyModifiers::CONTROL).unwrap();
        assert_eq!(app.pending_action, None, "edit only fires from the inbox pane");
    }

    #[test]
    fn esc_cancels_disposition_picker() {
        let tmp = fixture_repo();
        let mut app = app_focused_on_alpha_inbox(&tmp);
        handle_key(&mut app, KeyCode::Char('d'), KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Esc, KeyModifiers::NONE).unwrap();
        assert!(app.disposition.is_none(), "Esc should cancel the picker");
        assert_eq!(app.pending_action, None, "cancel must not request a drain");
    }

    #[test]
    fn d_is_noop_when_inbox_empty() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        // gamma is a live grove with no pending observations.
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("gamma".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Tab, KeyModifiers::NONE).unwrap(); // → inbox pane (empty)
        handle_key(&mut app, KeyCode::Char('d'), KeyModifiers::NONE).unwrap();
        assert!(app.disposition.is_none(), "no picker for an empty inbox");
        assert_eq!(app.pending_action, None, "no drain for an empty inbox");
    }

    #[test]
    fn d_is_noop_when_inbox_pane_not_focused() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        // Right pane defaults to LeafBody — `d` must not open the picker.
        handle_key(&mut app, KeyCode::Char('d'), KeyModifiers::NONE).unwrap();
        assert!(app.disposition.is_none(), "picker only opens from the inbox pane");
    }

    #[test]
    fn escape_returns_to_list_from_detail() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        assert_eq!(app.screen, Screen::GroveDetail);
        handle_key(&mut app, KeyCode::Esc, KeyModifiers::NONE).unwrap();
        assert_eq!(app.screen, Screen::GroveList);
    }

    #[test]
    fn slash_filters_grove_list() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, None);
        handle_key(&mut app, KeyCode::Char('/'), KeyModifiers::NONE).unwrap();
        for c in "bet".chars() {
            handle_key(&mut app, KeyCode::Char(c), KeyModifiers::NONE).unwrap();
        }
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        assert_eq!(app.filtered_groves().len(), 1);
        assert_eq!(app.filtered_groves()[0].name, "beta");
    }

    #[test]
    fn help_overlay_toggles() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, None);
        handle_key(&mut app, KeyCode::Char('?'), KeyModifiers::NONE).unwrap();
        assert!(app.show_help);
        let out = render_to_buffer(&app, 80, 16);
        assert!(out.contains("keybindings"), "help title missing:\n{}", out);
    }

    #[test]
    fn q_on_list_quits() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, None);
        let quit = handle_key(&mut app, KeyCode::Char('q'), KeyModifiers::NONE).unwrap();
        assert!(quit);
    }

    // -----------------------------------------------------------------
    // Capture modal

    fn type_str(app: &mut App, s: &str) {
        for c in s.chars() {
            handle_key(app, KeyCode::Char(c), KeyModifiers::NONE).unwrap();
        }
    }

    #[test]
    fn c_on_list_opens_modal_with_selected_as_target() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        // Preselect alpha; pressing `c` should pre-fill target with the
        // selected grove and start on the target field so the user can
        // edit it (capture to a *different* grove is the common case from
        // the list screen).
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap();
        assert!(app.capture.open);
        assert_eq!(app.capture.target, "alpha");
        assert_eq!(app.capture.field, CaptureField::Target);
    }

    #[test]
    fn c_on_detail_jumps_to_body_with_grove_prefilled() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap();
        assert!(app.capture.open);
        assert_eq!(app.capture.target, "alpha");
        // On detail, capturing-to-current-grove is the common case; jump
        // straight to body to save a Tab.
        assert_eq!(app.capture.field, CaptureField::Body);
    }

    #[test]
    fn typing_fills_active_field_and_enter_advances() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, None);
        // Open from list (no preselect) → starts on Target field, empty.
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap();
        // Clear whatever preselect put there.
        for _ in 0..app.capture.target.len() {
            handle_key(&mut app, KeyCode::Backspace, KeyModifiers::NONE).unwrap();
        }
        assert_eq!(app.capture.target, "");
        type_str(&mut app, "newgrove");
        assert_eq!(app.capture.target, "newgrove");
        // Enter on target advances to body.
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        assert_eq!(app.capture.field, CaptureField::Body);
        type_str(&mut app, "noticed a thing");
        assert_eq!(app.capture.body, "noticed a thing");
    }

    #[test]
    fn enter_on_empty_target_does_not_advance() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, None);
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap();
        for _ in 0..app.capture.target.len() {
            handle_key(&mut app, KeyCode::Backspace, KeyModifiers::NONE).unwrap();
        }
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        assert_eq!(app.capture.field, CaptureField::Target);
    }

    #[test]
    fn ctrl_s_in_body_with_both_fields_requests_submit() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        // Detail jumps to body field, target=alpha.
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap();
        type_str(&mut app, "first observation");
        handle_key(&mut app, KeyCode::Char('s'), KeyModifiers::CONTROL).unwrap();
        assert_eq!(app.pending_action, Some(PendingAction::Submit));
        // Submitting does *not* close the modal in the key handler;
        // the live loop will close it after the shell-out finishes.
        assert!(app.capture.open);
    }

    #[test]
    fn ctrl_s_with_empty_body_does_not_submit() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('s'), KeyModifiers::CONTROL).unwrap();
        assert_eq!(app.pending_action, None);
    }

    #[test]
    fn enter_in_body_inserts_newline_and_does_not_submit() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap();
        type_str(&mut app, "a");
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        type_str(&mut app, "b");
        assert_eq!(app.capture.body, "a\nb");
        assert_eq!(app.pending_action, None);
        assert!(app.capture.open);
    }

    #[test]
    fn multiline_paste_in_body_does_not_submit_or_truncate() {
        // A terminal paste (no bracketed-paste) arrives as chars
        // interspersed with Enter key events. The whole string must land
        // in the body intact, and submit must not fire.
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap();
        for c in "line1\nline2\nline3".chars() {
            if c == '\n' {
                handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
            } else {
                handle_key(&mut app, KeyCode::Char(c), KeyModifiers::NONE).unwrap();
            }
        }
        assert_eq!(app.capture.body, "line1\nline2\nline3");
        assert_eq!(app.pending_action, None);
    }

    #[test]
    fn enter_in_target_still_advances_to_body() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, None);
        // List screen opens on the target field, prefilled with a grove.
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap();
        assert_eq!(app.capture.field, CaptureField::Target);
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        assert_eq!(app.capture.field, CaptureField::Body);
        assert_eq!(app.pending_action, None);
    }

    #[test]
    fn ctrl_e_in_body_requests_editor() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('e'), KeyModifiers::CONTROL).unwrap();
        assert_eq!(app.pending_action, Some(PendingAction::EditBody));
    }

    #[test]
    fn tab_toggles_field() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap();
        assert_eq!(app.capture.field, CaptureField::Body);
        handle_key(&mut app, KeyCode::Tab, KeyModifiers::NONE).unwrap();
        assert_eq!(app.capture.field, CaptureField::Target);
        handle_key(&mut app, KeyCode::Tab, KeyModifiers::NONE).unwrap();
        assert_eq!(app.capture.field, CaptureField::Body);
    }

    #[test]
    fn esc_closes_modal() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap();
        type_str(&mut app, "abc");
        handle_key(&mut app, KeyCode::Esc, KeyModifiers::NONE).unwrap();
        assert!(!app.capture.open);
        assert_eq!(app.capture.body, "");
    }

    #[test]
    fn capture_modal_renders_with_target_and_body() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap();
        type_str(&mut app, "hello");
        let out = render_to_buffer(&app, 100, 24);
        assert!(out.contains("capture"), "modal title missing:\n{}", out);
        assert!(out.contains("target grove"), "target field label missing:\n{}", out);
        assert!(out.contains("alpha"), "target value missing:\n{}", out);
        assert!(out.contains("hello"), "body value missing:\n{}", out);
        assert!(out.contains("Ctrl-E"), "modal hint missing:\n{}", out);
    }

    #[test]
    fn footer_shows_c_hint() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let app = App::new(tmp.path().to_path_buf(), view, None);
        let out = render_to_buffer(&app, 100, 12);
        assert!(out.contains("c=capture"), "list footer missing c=capture:\n{}", out);
    }

    // -----------------------------------------------------------------
    // Version surfaces end-to-end (header on both screens + row segment)

    fn write_version(p: &Path, version: &str) {
        touch(p, &format!("| version | `{}` |\n", version));
    }

    #[test]
    fn header_and_row_show_versions_on_both_screens() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        let claude = crate::harness::by_name("claude").unwrap();
        write_version(&claude.install_path(root).join("VERSION.md"), "4.0.0");
        let alpha = root.join(".grove-worktrees/alpha");
        write_version(&claude.install_path(&alpha).join("VERSION.md"), "4.0.0");
        touch(&alpha.join(".grove/010-first.md"), "# 010-first\n");

        let view = RepoView::scan(root).unwrap();
        let mut app = App::new(root.to_path_buf(), view, Some("alpha".into()));

        let out = render_to_buffer(&app, 100, 12);
        assert!(out.contains("cli="), "header cli missing on list:\n{out}");
        assert!(out.contains("repo=4.0.0"), "header repo missing on list:\n{out}");
        assert!(out.contains("worktree=4.0.0"), "row worktree missing:\n{out}");

        // The header is drawn on the detail screen too.
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        let out = render_to_buffer(&app, 100, 16);
        assert!(out.contains("cli="), "header missing on detail:\n{out}");
    }

    #[test]
    fn row_flags_worktree_repo_drift() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        let claude = crate::harness::by_name("claude").unwrap();
        write_version(&claude.install_path(root).join("VERSION.md"), "9.9.9");
        let alpha = root.join(".grove-worktrees/alpha");
        write_version(&claude.install_path(&alpha).join("VERSION.md"), "4.0.0");
        touch(&alpha.join(".grove/010-first.md"), "# 010-first\n");

        let view = RepoView::scan(root).unwrap();
        let app = App::new(root.to_path_buf(), view, Some("alpha".into()));
        let out = render_to_buffer(&app, 120, 12);
        assert!(out.contains("worktree=4.0.0"), "worktree missing:\n{out}");
        assert!(out.contains("⚠ repo=9.9.9"), "drift marker missing:\n{out}");
    }

    // -----------------------------------------------------------------
    // Header spans (cli + repo layers)

    fn repo_map(pairs: &[(&'static str, Option<&str>)]) -> BTreeMap<&'static str, Option<String>> {
        pairs
            .iter()
            .map(|(k, v)| (*k, v.map(|s| s.to_string())))
            .collect()
    }

    #[test]
    fn header_spans_single_harness_aligned_has_no_drift() {
        let repo = repo_map(&[("claude", Some("4.0.0"))]);
        let spans = header_spans("4.0.0", &repo);
        let text: String = spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(text.contains("cli=4.0.0"), "got: {text}");
        assert!(text.contains("repo=4.0.0"), "got: {text}");
        assert!(!text.contains("repo[claude]"), "single harness must not prefix: {text}");
        assert!(spans.iter().all(|sp| sp.style != DRIFT_STYLE));
    }

    #[test]
    fn header_spans_flags_repo_drift_token() {
        let repo = repo_map(&[("claude", Some("3.0.1"))]);
        let spans = header_spans("4.0.0", &repo);
        let marker = spans.iter().find(|sp| sp.content.contains("3.0.1")).unwrap();
        assert_eq!(marker.style, DRIFT_STYLE);
    }

    #[test]
    fn header_spans_multi_harness_prefixes_repo_tokens() {
        let repo = repo_map(&[("claude", Some("4.0.0")), ("codex", Some("4.0.0"))]);
        let spans = header_spans("4.0.0", &repo);
        let text: String = spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(text.contains("repo[claude]=4.0.0"), "got: {text}");
        assert!(text.contains("repo[codex]=4.0.0"), "got: {text}");
    }

    #[test]
    fn header_spans_not_installed_shows_cli_only() {
        let repo = repo_map(&[]);
        let spans = header_spans("4.0.0", &repo);
        let text: String = spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(text.contains("cli=4.0.0"), "got: {text}");
        assert!(text.contains("not installed"), "got: {text}");
    }

    // -----------------------------------------------------------------
    // Version segment spans (worktree layer)

    fn text_of(spans: &[Span]) -> String {
        spans.iter().map(|s| s.content.as_ref()).collect()
    }

    fn s(v: &str) -> Option<String> {
        Some(v.to_string())
    }

    #[test]
    fn worktree_spans_aligned_has_no_drift_style() {
        let spans = worktree_spans(None, &s("4.0.0"), &s("4.0.0"), "4.0.0");
        assert_eq!(text_of(&spans), "worktree=4.0.0");
        assert!(
            spans.iter().all(|sp| sp.style != DRIFT_STYLE),
            "aligned segment must carry no drift styling"
        );
    }

    #[test]
    fn worktree_spans_flags_repo_drift_in_drift_style() {
        let spans = worktree_spans(None, &s("3.0.1"), &s("4.0.0"), "3.0.1");
        assert_eq!(text_of(&spans), "worktree=3.0.1 ⚠ repo=4.0.0");
        let marker = spans
            .iter()
            .find(|sp| sp.content.contains("repo=4.0.0"))
            .unwrap();
        assert_eq!(marker.style, DRIFT_STYLE);
    }

    #[test]
    fn worktree_spans_flags_cli_drift_in_drift_style() {
        let spans = worktree_spans(None, &s("3.0.1"), &s("3.0.1"), "4.0.0");
        assert_eq!(text_of(&spans), "worktree=3.0.1 ⚠ cli=4.0.0");
        let marker = spans
            .iter()
            .find(|sp| sp.content.contains("cli=4.0.0"))
            .unwrap();
        assert_eq!(marker.style, DRIFT_STYLE);
    }

    #[test]
    fn worktree_spans_unknown_is_not_drift() {
        let spans = worktree_spans(None, &None, &s("4.0.0"), "4.0.0");
        assert_eq!(text_of(&spans), "worktree=(unknown)");
        assert!(spans.iter().all(|sp| sp.style != DRIFT_STYLE));
    }

    #[test]
    fn worktree_spans_orphan_shows_repo_none_without_warning() {
        let spans = worktree_spans(None, &s("4.0.0"), &None, "4.0.0");
        assert_eq!(text_of(&spans), "worktree=4.0.0 repo=(none)");
        assert!(spans.iter().all(|sp| sp.style != DRIFT_STYLE));
    }

    #[test]
    fn worktree_spans_prefixes_harness_when_named() {
        let spans = worktree_spans(Some("codex"), &s("4.0.0"), &s("4.0.0"), "4.0.0");
        assert_eq!(text_of(&spans), "worktree[codex]=4.0.0");
    }

    // -----------------------------------------------------------------
    // Controller seam wiring: raw proxy bytes → decoder → handle_key
    //
    // The controller decodes the proxy's forwarded stdin into the same
    // crossterm keys `handle_key` already consumes. These exercise that exact
    // pump (`InputDecoder::feed` → `handle_key`) on real `App` state, standing
    // in for the socket the live loop reads from.

    /// Drive raw input bytes through the controller's decode→dispatch path.
    fn pump_bytes(app: &mut App, keys: &mut InputDecoder, bytes: &[u8]) {
        for key in keys.feed(bytes) {
            handle_key(app, key.code, key.mods).unwrap();
        }
    }

    #[test]
    fn seam_csi_arrow_moves_list_selection() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        let mut keys = InputDecoder::new();
        assert_eq!(app.list.selected(), Some(0));
        // A real terminal's Down arrow: the CSI sequence the proxy forwards up.
        pump_bytes(&mut app, &mut keys, b"\x1b[B");
        assert_eq!(app.list.selected(), Some(1), "CSI down must move like `j`");
    }

    #[test]
    fn seam_enter_drills_into_detail() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        let mut keys = InputDecoder::new();
        pump_bytes(&mut app, &mut keys, b"\r"); // Enter over the seam.
        assert_eq!(app.screen, Screen::GroveDetail);
    }

    #[test]
    fn seam_ctrl_s_submits_capture_from_decoded_bytes() {
        // The full capture gesture arriving as raw proxy bytes: open, type a
        // body, then Ctrl-S (0x13) — which the decoder maps to Ctrl+`s` and
        // `handle_capture_key` treats as submit.
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        let mut keys = InputDecoder::new();
        pump_bytes(&mut app, &mut keys, b"\r"); // → detail (alpha)
        pump_bytes(&mut app, &mut keys, b"c"); // open capture, jumps to body
        pump_bytes(&mut app, &mut keys, b"noted"); // type body
        pump_bytes(&mut app, &mut keys, &[0x13]); // Ctrl-S
        assert_eq!(app.pending_action, Some(PendingAction::Submit));
        assert_eq!(app.capture.body, "noted");
    }

    #[test]
    fn seam_lone_esc_flushes_to_escape_key() {
        // A bare ESC byte is buffered (it may start a CSI); the controller's
        // idle-tick `flush()` resolves it to `Esc`, popping detail → list.
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        let mut keys = InputDecoder::new();
        pump_bytes(&mut app, &mut keys, b"\r"); // → detail
        assert_eq!(app.screen, Screen::GroveDetail);
        // Lone ESC: nothing decodes yet (it may be the head of a sequence).
        assert!(keys.feed(b"\x1b").is_empty());
        assert_eq!(app.screen, Screen::GroveDetail, "bare ESC must not act yet");
        // Idle tick: the controller calls flush(), which yields Esc.
        for key in keys.flush() {
            handle_key(&mut app, key.code, key.mods).unwrap();
        }
        assert_eq!(app.screen, Screen::GroveList, "flushed ESC pops to the list");
    }

    // -----------------------------------------------------------------
    // Filesystem watcher debounce predicate

    #[test]
    fn watchset_settled_only_after_debounce() {
        // Construct without spawning a real watcher; just exercise the
        // debounce predicate, which is the public-facing
        // contract the event loop depends on.
        let mut w = WatchSet { _watcher: None, rx: None, dirty_since: None };
        assert!(!w.settled());

        w.dirty_since = Some(Instant::now());
        assert!(!w.settled(), "fresh dirty mark must not be settled");

        w.dirty_since = Some(Instant::now() - Duration::from_millis(250));
        assert!(w.settled(), "older-than-debounce mark must be settled");

        w.clear();
        assert!(!w.settled());
    }

    #[test]
    fn watchset_poll_timeout_shortens_when_dirty() {
        let mut w = WatchSet { _watcher: None, rx: None, dirty_since: None };
        assert_eq!(w.poll_timeout(), Duration::from_millis(200));

        w.dirty_since = Some(Instant::now());
        // Within the window, the timeout is some positive value below
        // 200ms — clamped to at least 10ms so we don't busy-spin.
        let t = w.poll_timeout();
        assert!(t <= Duration::from_millis(200));
        assert!(t >= Duration::from_millis(10));
    }
}
