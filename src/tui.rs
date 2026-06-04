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
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::{DefaultTerminal, Frame};

use crate::cli::RepoArgs;
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
    /// **Detail-locked** mode (130-native-detail/020): the `App` is bound to one
    /// grove's detail, mounted as a per-grove host surface beside its harness in the
    /// content region. There is no grove list to return to (the constant nav is a
    /// *separate* surface), so the list/filter navigation that the master/detail
    /// dashboard offers is suppressed: `Esc`/`q` stay in detail (they do not pop to a
    /// list), and `/` does not open a filter. Everything else (tree/inbox nav, `c`
    /// capture, `d` disposition, `Ctrl-E`, `Tab`) works unchanged.
    detail_locked: bool,
    /// **Native frame** mode (leaf 140): this `App` renders as a trellis host
    /// surface (the constant nav or a per-grove detail), where the grove-owned
    /// [[whichkey bar]] is the *single* owner of the bottom hint line. When set,
    /// `render` suppresses this surface's own footer and the capture modal's inline
    /// hint — those hints are published to the whichkey instead ([`footer_line`]).
    /// The legacy `--local` in-terminal dashboard leaves it `false` and keeps
    /// drawing its own footer (there is no whichkey pane to delegate to).
    native_chrome: bool,
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
    /// Select grove `name`: swap its `grove do <name>` harness (run in `repo`) into
    /// the **content slot** beside the constant nav (ADR-0022/0023). The native
    /// dashboard drives this by a direct in-process `HostDriver::swap_content` call;
    /// the screen thread parks the previously-selected harness alive and mounts (or
    /// restores) this one — no `zellij action`, no tabs. `repo` is explicit so the
    /// cross-repo fleet (070) reuses the driving layer unchanged. Native-path only —
    /// the `--local` in-terminal dashboard has no substrate to drive.
    OpenHarness {
        name: String,
        repo: PathBuf,
    },
    /// Legacy "close the acting grove's harness" request (the `x` key). The
    /// content-swap model parks harnesses alive instead of closing them, so the
    /// native surface treats this as a no-op-with-status; retained for the `--local`
    /// path's key handling.
    CloseHarness {
        name: String,
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
            detail_locked: false,
            native_chrome: false,
        }
    }

    /// Build an `App` **locked to one grove's detail** (130-native-detail/020) —
    /// the state behind a per-grove [`DetailSurface`]. Starts straight in
    /// [`Screen::GroveDetail`] for `grove`, with fresh tree/inbox cursors and
    /// list/filter navigation suppressed (see [`App::detail_locked`]); the detail
    /// data itself is read from `view` by name on each render, exactly as the
    /// master/detail dashboard does after a drill-in.
    pub fn new_detail(repo: PathBuf, view: RepoView, grove: String) -> Self {
        let mut tree = ListState::default();
        tree.select(Some(0));
        let mut inbox = ListState::default();
        inbox.select(Some(0));
        Self {
            repo,
            view,
            screen: Screen::GroveDetail,
            list: ListState::default(),
            detail: Some(DetailState {
                grove,
                tree,
                inbox,
                right: RightPane::LeafBody,
                right_scroll: 0,
            }),
            filter: FilterState::default(),
            show_help: false,
            status: None,
            capture: CaptureModal::default(),
            disposition: None,
            pending_action: None,
            detail_locked: true,
            // A per-grove detail surface always runs inside the native frame, where
            // the grove-owned whichkey bar owns the bottom hint line.
            native_chrome: true,
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

    /// The grove the **native nav's `Enter` (select)** should act on: the
    /// highlighted row, but only while the plain grove list is showing — no modal,
    /// filter, or help intercepting keys. `None` otherwise, so `Enter` falls
    /// through to the shared key handler (e.g. inserting a newline in the capture
    /// body, or advancing the capture target field).
    ///
    /// In the native nav, selecting a grove **opens or switches to its
    /// [[workspace]] tab** (leaf 120); the v1 master/detail drill-in is superseded
    /// by per-grove detail tabs (130). Only the native [`DashboardSurface`]
    /// consults this — the legacy `--local` dashboard keeps `Enter` = drill-in.
    pub fn nav_enter_target(&self) -> Option<String> {
        if self.show_help
            || self.capture.open
            || self.disposition.is_some()
            || self.filter.editing
        {
            return None;
        }
        if !matches!(self.screen, Screen::GroveList) {
            return None;
        }
        self.filtered_groves()
            .get(self.list.selected().unwrap_or(0))
            .map(|g| g.name.clone())
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
        PendingAction::OpenHarness { name, .. } | PendingAction::CloseHarness { name } => {
            // Harness tabs are driven natively by the trellis `HostDriver`; the
            // legacy `--local` in-terminal dashboard has no embedding to drive,
            // so this is a no-op beyond an explanatory status line. The native
            // `grove tui` (default) opens/closes harness tabs for real.
            app.status = Some(format!(
                "workspace tabs need the native dashboard — run `grove tui` (not --local): {}",
                name
            ));
        }
    }
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
        // Filtering navigates the grove list; in detail-locked mode there is no
        // list (the constant nav is a separate surface), so `/` is inert.
        (_, KeyCode::Char('/')) if !app.detail_locked => {
            app.filter.editing = true;
            app.filter.text.clear();
        }
        (_, KeyCode::Char('r')) => {
            app.refresh()?;
        }
        (_, KeyCode::Char('c')) => {
            open_capture_modal(app);
        }
        // Harness driving (native path): `o` selects the acting grove — swapping
        // its harness into the content slot beside the constant nav (the same action
        // as `Enter` on the nav list). `x` is the retired close affordance (the swap
        // model parks harnesses alive; see `PendingAction::CloseHarness`). The
        // decision is recorded here and enacted by the native `process_action`
        // (`HostDriver::swap_content`); the `--local` path can't drive a substrate.
        (_, KeyCode::Char('o')) => {
            request_open_harness(app);
        }
        (_, KeyCode::Char('x')) => {
            request_close_harness(app);
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
        // Detail screen. `Esc`/`q` pop back to the grove list in the master/detail
        // dashboard — but in detail-locked mode there is no list to return to (the
        // nav is a separate constant surface), so they are inert: the detail surface
        // stays put, and the user moves focus to the nav with the leader instead.
        (Screen::GroveDetail, KeyCode::Esc | KeyCode::Char('q')) if !app.detail_locked => {
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

/// The grove the user is acting on for harness driving: the highlighted row on
/// the list screen, or the open grove on the detail screen. `None` when the
/// list is empty / unselected.
fn acting_grove_name(app: &App) -> Option<String> {
    match app.screen {
        Screen::GroveDetail => app.detail.as_ref().map(|d| d.grove.clone()),
        Screen::GroveList => app
            .filtered_groves()
            .get(app.list.selected().unwrap_or(0))
            .map(|g| g.name.clone()),
    }
}

/// Request "select the acting grove" — swap its harness into the content slot
/// (ADR-0022/0023). The repo is carried explicitly so the cross-repo fleet (070)
/// reuses this path unchanged. No-op when no grove is selected.
fn request_open_harness(app: &mut App) {
    if let Some(name) = acting_grove_name(app) {
        let repo = app.repo.clone();
        app.pending_action = Some(PendingAction::OpenHarness { name, repo });
    }
}

/// Request the retired "close the acting grove's harness" affordance (`x`). No-op
/// when no grove is selected; the native surface parks rather than closes.
fn request_close_harness(app: &mut App) {
    if let Some(name) = acting_grove_name(app) {
        app.pending_action = Some(PendingAction::CloseHarness { name });
    }
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
    // A one-row header (cli + repo versions) sits above the body on both screens.
    // In the **native frame** the grove-owned whichkey bar owns the bottom hint
    // line (a separate full-width pane), so this surface draws no footer; the
    // legacy `--local` dashboard keeps its own footer below the body.
    let main = if app.native_chrome {
        let [header, main] = Layout::vertical([Constraint::Length(1), Constraint::Min(1)]).areas(area);
        render_header(f, header, app);
        main
    } else {
        let [header, main, footer] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .areas(area);
        render_header(f, header, app);
        f.render_widget(Paragraph::new(footer_line(app)), footer);
        main
    };

    match app.screen {
        Screen::GroveList => render_grove_list(f, main, app),
        Screen::GroveDetail => render_grove_detail(f, main, app),
    }

    if app.capture.open {
        render_capture_modal(f, area, app);
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

fn render_capture_modal(f: &mut Frame, area: Rect, app: &App) {
    let modal = &app.capture;
    let popup = centered_rect(70, 50, area);
    f.render_widget(Clear, popup);

    let block = Block::default()
        .borders(Borders::ALL)
        .title("capture — grove inbox add");

    let inner = block.inner(popup);
    f.render_widget(block, popup);

    // In the native frame the whichkey bar shows the capture keys (`footer_line`),
    // so the modal drops its inline hint row; the legacy `--local` dashboard keeps
    // the hint inside the modal (it has no whichkey to delegate to).
    let (target_area, body_area, hint_area) = if app.native_chrome {
        let [target_area, body_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Min(3)]).areas(inner);
        (target_area, body_area, None)
    } else {
        let [target_area, body_area, hint_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(3),
            Constraint::Length(1),
        ])
        .areas(inner);
        (target_area, body_area, Some(hint_area))
    };

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

    if let Some(hint_area) = hint_area {
        let hint =
            "Tab=switch  Enter=next(target)/newline(body)  Ctrl-S=submit  Ctrl-E=$EDITOR  Esc=cancel";
        f.render_widget(Paragraph::new(hint), hint_area);
    }
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

/// The bottom hint line for the focused surface, as an owned [`Line`]. This is
/// the **single source** of grove's key hints (leaf 140): the legacy `--local`
/// dashboard renders it as its own footer, and in the native frame each surface
/// *publishes* it to the grove-owned [[whichkey bar]] (`publish_whichkey`) when it
/// gains focus or changes state, so the bar always reflects the focused surface.
///
/// Hints use sigils (⏎ Enter, ⎋ Esc, ⇥ Tab, ⌃o the leader). The line is
/// context-sensitive: a modal or filter takes precedence over the base
/// screen hints, so the keys shown are always the ones currently live.
fn footer_line(app: &App) -> Line<'static> {
    if app.show_help {
        return Line::from("⎋ / ? close help");
    }
    if app.capture.open {
        return Line::from(
            "⇥ switch field · ⏎ next / newline · ⌃s submit · ⌃e $EDITOR · ⎋ cancel",
        );
    }
    if app.disposition.is_some() {
        return Line::from("i incorporated · d deferred · r rejected · ⎋ cancel");
    }
    if app.filter.editing {
        return Line::from(vec![
            Span::styled(
                format!("/{}_", app.filter.text),
                Style::default().add_modifier(Modifier::REVERSED),
            ),
            Span::raw("  ⏎ apply · ⎋ cancel"),
        ]);
    }
    // The base per-screen hints. `⌥1-9` tab switching is gone with ADR-0023 (the
    // constant nav + content swap has no tabs), so the nav advertises only the
    // leader.
    let hint = match app.screen {
        Screen::GroveList => "⏎ open · j/k move · ⌃o nav · x close · / filter · c capture · r refresh · ? help · q quit",
        Screen::GroveDetail => "⇥ cycle · j/k move · o open · x close · PgUp/PgDn scroll · / filter · c capture · r refresh · ⎋ back · ? help",
    };
    let mut spans = vec![Span::raw(hint)];
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
    Line::from(spans)
}

fn render_help_overlay(f: &mut Frame, area: Rect) {
    let lines = vec![
        Line::from(Span::styled(
            "grove tui — keybindings",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("  ↑/↓ or j/k    move selection"),
        Line::from("  ⏎ Enter       open (or switch to) the grove's workspace tab"),
        Line::from("  ⌃o            focus this nav from any pane (the leader)"),
        Line::from("  ⌥1-9 / ⌥[ ]   switch between workspace tabs"),
        Line::from("  ⎋ Esc / q     back / quit"),
        Line::from("  Tab           cycle right pane (leaf → inbox → brief)"),
        Line::from("                  in the inbox pane, j/k select an observation"),
        Line::from("  o             open (or switch to) the grove's workspace tab"),
        Line::from("  x             close the grove's workspace tab"),
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
        // Footer hints (sigils — leaf 120-native-nav).
        assert!(out.contains("⏎ open"), "footer missing:\n{}", out);
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

    /// The whichkey/footer hint line (leaf 140) is the single hint source for both
    /// the local footer and the native bar, and is context-sensitive: a modal or a
    /// live filter takes precedence over the base per-screen hints.
    #[test]
    fn footer_line_is_context_sensitive() {
        fn text(app: &App) -> String {
            footer_line(app)
                .spans
                .iter()
                .map(|s| s.content.as_ref())
                .collect()
        }
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, None);

        // Base grove-list hints: the leader + open, but no retired tab-switch hint
        // (ADR-0023 removed `⌥1-9 switch`).
        let base = text(&app);
        assert!(base.contains("⏎ open"), "list hints: {base}");
        assert!(base.contains("⌃o nav"), "leader hint: {base}");
        assert!(!base.contains("⌥1-9"), "no tab-switch hint: {base}");

        // A live filter edit echoes the query and shows apply/cancel.
        app.filter.editing = true;
        app.filter.text = "au".into();
        let filtering = text(&app);
        assert!(filtering.contains("/au_"), "filter echo: {filtering}");
        assert!(filtering.contains("⏎ apply"), "filter keys: {filtering}");
        app.filter.editing = false;
        app.filter.text.clear();

        // The capture modal's keys take precedence over the base hints.
        app.capture.open = true;
        let capturing = text(&app);
        assert!(capturing.contains("⌃s submit"), "capture submit: {capturing}");
        assert!(capturing.contains("⎋ cancel"), "capture cancel: {capturing}");
        app.capture.open = false;

        // Help.
        app.show_help = true;
        assert!(text(&app).contains("close help"), "help hint");
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

    // --- Detail-locked mode (130-native-detail/020): the App behind a per-grove
    // DetailSurface, bound to one grove with list/filter navigation suppressed. ---

    fn detail_locked_app(tmp: &TempDir, grove: &str) -> App {
        let view = RepoView::scan(tmp.path()).unwrap();
        App::new_detail(tmp.path().to_path_buf(), view, grove.to_string())
    }

    #[test]
    fn new_detail_locks_onto_the_named_grove() {
        let tmp = fixture_repo();
        let app = detail_locked_app(&tmp, "alpha");
        assert_eq!(app.screen, Screen::GroveDetail);
        assert_eq!(app.detail.as_ref().unwrap().grove, "alpha");
        assert!(app.detail_locked);
        // It renders that grove's tree + first-leaf body straight away.
        let out = render_to_buffer(&app, 100, 16);
        assert!(out.contains("010-first.md"), "leaf row missing:\n{}", out);
        assert!(out.contains("Work here."), "leaf body missing:\n{}", out);
    }

    #[test]
    fn detail_locked_q_and_esc_stay_in_detail() {
        let tmp = fixture_repo();
        let mut app = detail_locked_app(&tmp, "alpha");
        // `q` must neither quit the session nor pop to a (non-existent) list.
        let quit = handle_key(&mut app, KeyCode::Char('q'), KeyModifiers::NONE).unwrap();
        assert!(!quit, "q must not quit from a detail surface");
        assert_eq!(app.screen, Screen::GroveDetail, "q stays in detail");
        handle_key(&mut app, KeyCode::Esc, KeyModifiers::NONE).unwrap();
        assert_eq!(app.screen, Screen::GroveDetail, "Esc stays in detail");
        assert!(app.detail.is_some(), "the detail is not dropped");
    }

    #[test]
    fn detail_locked_slash_does_not_open_filter() {
        let tmp = fixture_repo();
        let mut app = detail_locked_app(&tmp, "alpha");
        handle_key(&mut app, KeyCode::Char('/'), KeyModifiers::NONE).unwrap();
        assert!(!app.filter.editing, "no list to filter in detail-locked mode");
    }

    #[test]
    fn detail_locked_c_opens_capture_prefilled_with_the_grove() {
        let tmp = fixture_repo();
        let mut app = detail_locked_app(&tmp, "alpha");
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap();
        assert!(app.capture.open);
        assert_eq!(app.capture.target, "alpha", "capture targets the locked grove");
    }

    #[test]
    fn detail_locked_d_on_inbox_requests_drain_for_the_locked_grove() {
        let tmp = fixture_repo();
        let mut app = detail_locked_app(&tmp, "alpha");
        // Tab cycles the right pane LeafBody → Inbox, so `d` acts on the inbox.
        handle_key(&mut app, KeyCode::Tab, KeyModifiers::NONE).unwrap();
        assert_eq!(app.detail.as_ref().unwrap().right, RightPane::Inbox);
        handle_key(&mut app, KeyCode::Char('d'), KeyModifiers::NONE).unwrap();
        assert!(app.disposition.is_some(), "disposition picker open on the inbox");
        handle_key(&mut app, KeyCode::Char('i'), KeyModifiers::NONE).unwrap();
        match app.pending_action.take() {
            Some(PendingAction::Drain { disposition, path }) => {
                assert_eq!(disposition, Disposition::Incorporated);
                assert!(
                    path.to_string_lossy().contains("inboxes/alpha/"),
                    "drains an alpha observation: {path:?}"
                );
            }
            other => panic!("expected a Drain action, got {other:?}"),
        }
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
    // Harness driving keys (`o` open/switch, `x` close)
    //
    // `handle_key` only records the intent (a `PendingAction`); the controller
    // executes it against zellij (see `harness_drive` for that logic). These
    // assert the pure decision: which grove, and that the repo is carried so
    // 070's cross-repo fleet reuses the path unchanged.

    #[test]
    fn o_on_list_requests_open_harness_for_selected_grove() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Char('o'), KeyModifiers::NONE).unwrap();
        match app.pending_action.as_ref() {
            Some(PendingAction::OpenHarness { name, repo }) => {
                assert_eq!(name, "alpha");
                assert_eq!(repo, tmp.path(), "open carries the repo for cross-repo reuse");
            }
            other => panic!("expected OpenHarness, got {:?}", other),
        }
    }

    #[test]
    fn o_on_detail_requests_open_harness_for_current_grove() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap(); // → detail
        handle_key(&mut app, KeyCode::Char('o'), KeyModifiers::NONE).unwrap();
        match app.pending_action.as_ref() {
            Some(PendingAction::OpenHarness { name, .. }) => assert_eq!(name, "alpha"),
            other => panic!("expected OpenHarness, got {:?}", other),
        }
    }

    #[test]
    fn x_on_list_requests_close_harness_for_selected_grove() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("beta".into()));
        handle_key(&mut app, KeyCode::Char('x'), KeyModifiers::NONE).unwrap();
        match app.pending_action.as_ref() {
            Some(PendingAction::CloseHarness { name }) => assert_eq!(name, "beta"),
            other => panic!("expected CloseHarness, got {:?}", other),
        }
    }

    #[test]
    fn harness_keys_inert_inside_capture_modal() {
        // While typing an observation, `o`/`x` must type into the field, not
        // drive harnesses.
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap(); // open capture
        handle_key(&mut app, KeyCode::Char('o'), KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('x'), KeyModifiers::NONE).unwrap();
        assert_eq!(app.pending_action, None, "harness keys must be inert in the modal");
        assert!(app.capture.open, "the capture modal stays open");
    }

    #[test]
    fn footer_shows_harness_hints() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let app = App::new(tmp.path().to_path_buf(), view, None);
        let out = render_to_buffer(&app, 120, 12);
        assert!(out.contains("⏎ open"), "list footer missing ⏎ open:\n{}", out);
        assert!(out.contains("x close"), "list footer missing x close:\n{}", out);
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
        assert!(out.contains("c capture"), "list footer missing c capture:\n{}", out);
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

    #[test]
    fn vcs_tool_defaults_to_lazygit_for_a_plain_git_worktree() {
        // 020-aux-tool-panes: the vcs pane is not hard-wired to git — but a
        // worktree with no `.jj/` is the default (lazygit) case.
        let tmp = TempDir::new().unwrap();
        assert_eq!(vcs_tool(tmp.path()), "lazygit");
    }

    #[test]
    fn vcs_tool_selects_lazyjj_for_a_jj_worktree() {
        // The detection seam (brief): a `.jj/` present routes to lazyjj, so the
        // tool lands later as a one-point change without re-touching the spawn.
        let tmp = TempDir::new().unwrap();
        fs::create_dir(tmp.path().join(".jj")).unwrap();
        assert_eq!(vcs_tool(tmp.path()), "lazyjj");
    }

    #[test]
    fn on_path_finds_a_binary_in_a_path_dir() {
        // A file present in one of the PATH dirs resolves to its full path; a
        // name absent from every PATH dir resolves to `None` (the graceful-
        // fallback trigger for an uninstalled aux tool).
        let tmp = TempDir::new().unwrap();
        let bin = tmp.path().join("grove-fake-tool");
        fs::write(&bin, "#!/bin/sh\n").unwrap();
        let found = on_path("grove-fake-tool", &std::ffi::OsString::from(tmp.path()));
        assert_eq!(found.as_deref(), Some(bin.as_path()));
        assert_eq!(
            on_path("grove-definitely-absent-xyz", &std::ffi::OsString::from(tmp.path())),
            None
        );
    }
}

// ===========================================================================
// Native host surface (ADR-0021): the v1 dashboard, rendered as a trellis
// `HostPane` in-process — no proxy socket, no `zellij action`, no WASM.
//
// This is leaf 030-port-dashboard-drive. The whole point is **port, not
// rewrite**: the `App`, `render`, `handle_key`, `WatchSet` debounce, and
// shell-out writes above are reused *unchanged*. Only the transport is new —
// instead of a crossterm tty loop (`run`) or a proxy socket (`Controller`),
// the dashboard draws into an off-screen `ratatui` buffer that trellis blits
/// The VCS TUI to embed for `worktree`'s [[working set]] (020-aux-tool-panes):
/// **lazyjj** for a Jujutsu worktree (a `.jj/` is present), else **lazygit**.
/// This single indirection is the seam the brief asks for — default lazygit now,
/// lazyjj a one-point change later — so the aux-spawn path never branches on the
/// VCS itself. Pure over the worktree path (no shell-out), so it is unit-testable
/// and sits below the ADR-0013 presentation boundary.
//
// Consumed by the `trellis-seam`-gated `mod native` (the aux-spawn path) and by
// the always-on unit tests; without the feature only the tests use it, so the
// dead-code lint is silenced for that build.
#[cfg_attr(not(feature = "trellis-seam"), allow(dead_code))]
fn vcs_tool(worktree: &Path) -> &'static str {
    if worktree.join(".jj").is_dir() {
        "lazyjj"
    } else {
        "lazygit"
    }
}

/// Resolve `bin` against the `PATH`-style search list `path`, returning the first
/// matching existing file. A bare name is looked up in each `PATH` entry; a name
/// containing `/` is treated as a path and checked directly. `None` means "not
/// found on `PATH`" — the signal the aux-spawn path uses to substitute a graceful
/// in-pane message rather than failing the whole working-set mount when an aux
/// tool (yazi/lazygit) is not installed. Pure over `(bin, path)`, so it is
/// testable without touching the process environment.
#[cfg_attr(not(feature = "trellis-seam"), allow(dead_code))]
fn on_path(bin: &str, path: &std::ffi::OsStr) -> Option<PathBuf> {
    if bin.contains('/') {
        let p = PathBuf::from(bin);
        return p.is_file().then_some(p);
    }
    std::env::split_paths(path).find_map(|dir| {
        let full = dir.join(bin);
        full.is_file().then_some(full)
    })
}

// as a real pane, receives keys in-process, drives tabs/panes by direct
// `HostDriver` call, and wakes fs-watch redraws via a tick instruction.
//
// It lives in a child module so it can reuse this module's private items
// (`render`, `handle_key`, `App`'s fields, `refresh_silent`, the `shell_*`
// writers) without widening their visibility; it is feature-gated because it
// links the forked `zellij-*` crates (the `trellis-seam` feature).
// ===========================================================================
#[cfg(feature = "trellis-seam")]
mod native {
    use std::collections::BTreeSet;
    use std::path::{Path, PathBuf};
    use std::sync::{mpsc, Mutex};

    use anyhow::{Context, Result};
    use notify::{RecommendedWatcher, RecursiveMode, Watcher};
    use ratatui::backend::TestBackend;
    use ratatui::buffer::Buffer;
    use ratatui::crossterm::event::{KeyCode, KeyEvent};
    use ratatui::layout::Rect;
    use ratatui::style::{Color, Style};
    use ratatui::text::Line;
    use ratatui::widgets::{Paragraph, Widget};
    use ratatui::Terminal;
    use tempfile::NamedTempFile;

    use trellis::input::command::RunCommand;
    use trellis_server::panes::host_pane::{
        register_keyed_host_surface, HostDriver, HostSurface,
    };

    use super::{
        current_grove_name, decide_observation_edit, footer_line, handle_key, on_path, render,
        shell_capture, shell_drain, short_err, vcs_tool, App, CaptureField, CaptureModal,
        EditOutcome, PendingAction, RepoView, DEBOUNCE,
    };

    // -----------------------------------------------------------------------
    // The grove-owned whichkey bar (ADR-0019, leaf 140).
    //
    // One full-width line across the bottom of the native frame, the **single**
    // owner of grove's key hints: the nav/detail surfaces suppress their own
    // footers (`App::native_chrome`) and instead *publish* their hint line here
    // when they gain focus or change state. The harness draws no hint of its own —
    // when it is focused, the surface that lost focus relinquishes the bar to a
    // "keys go to the harness" line.
    //
    // The bar is a passive, non-selectable host pane; it never has focus and so
    // cannot learn the focused context by `set_focused`. Instead the *publishing*
    // surface is always the focused one (input only routes to the focused pane), so
    // it knows its own hints — and a `request_tick` through the whichkey's stored
    // `HostDriver` wakes the bar to redraw. No new trellis focus hook is needed.
    // -----------------------------------------------------------------------

    /// Which surface currently owns the whichkey line. Tracked so a surface only
    /// relinquishes the bar (on losing focus) when it still owns it — keeping the
    /// hand-off order-independent across the paired `set_focused(false)` /
    /// `set_focused(true)` calls trellis makes when focus moves.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum WhichkeyOwner {
        Nav,
        Detail,
        Harness,
    }

    /// The published whichkey content: its owner plus the line the bar renders.
    /// `None` until the first surface publishes (the nav, when it is focused at
    /// first-layout).
    static WHICHKEY_LINE: Mutex<Option<(WhichkeyOwner, Line<'static>)>> = Mutex::new(None);

    /// The whichkey pane's own [`HostDriver`], stashed by [`WhichkeySurface::set_driver`]
    /// so any focused surface can `request_tick` the bar to redraw when it publishes
    /// a new line. `None` until the whichkey pane is injected.
    static WHICHKEY_DRIVER: Mutex<Option<HostDriver>> = Mutex::new(None);

    /// Publish `line` as the whichkey content owned by `owner`, and wake the bar to
    /// redraw — but only when the content actually changed, so a held key (or a
    /// no-op redraw) does not tick the bar every keystroke. Called by the focused
    /// nav/detail surface; a no-op-but-safe call when no whichkey pane exists (the
    /// `--local` path) since the driver is then `None`.
    fn publish_whichkey(owner: WhichkeyOwner, line: Line<'static>) {
        let changed = {
            let mut slot = WHICHKEY_LINE.lock().unwrap();
            let same = matches!(&*slot, Some((o, l)) if *o == owner && *l == line);
            if !same {
                *slot = Some((owner, line));
            }
            !same
        };
        if changed {
            if let Some(driver) = WHICHKEY_DRIVER.lock().unwrap().as_ref() {
                driver.request_tick();
            }
        }
    }

    /// Hand the whichkey to the harness context when `owner` loses focus — but only
    /// if `owner` still holds the bar (the newly-focused surface may already have
    /// claimed it, in which case its line must stand).
    fn relinquish_whichkey(owner: WhichkeyOwner) {
        let owns = matches!(&*WHICHKEY_LINE.lock().unwrap(), Some((o, _)) if *o == owner);
        if owns {
            publish_whichkey(WhichkeyOwner::Harness, harness_whichkey_line());
        }
    }

    /// The whichkey line shown while a harness pane is focused: grove owns only the
    /// leader (locked mode passes every other key to the harness itself).
    fn harness_whichkey_line() -> Line<'static> {
        Line::from("⌃o nav   ·   keys go to the focused harness")
    }

    /// The grove-owned whichkey bar as a trellis [`HostSurface`] (leaf 140): a
    /// stateless renderer of whatever [`WHICHKEY_LINE`] the focused surface has
    /// published. Injected as a full-width, non-selectable bottom pane
    /// ([`Tab::inject_whichkey_pane`](trellis_server::tab)); never focused.
    pub struct WhichkeySurface;

    /// Build the (stateless) whichkey surface for [`register_whichkey_surface`].
    pub fn whichkey_surface() -> WhichkeySurface {
        WhichkeySurface
    }

    impl HostSurface for WhichkeySurface {
        fn draw(&mut self, area: Rect, buf: &mut Buffer) {
            let line = WHICHKEY_LINE
                .lock()
                .unwrap()
                .as_ref()
                .map(|(_, l)| l.clone())
                .unwrap_or_else(|| Line::from(""));
            // A subtle bar background sets the hint line apart from the surfaces
            // above it; the published spans keep their own foreground colours.
            Paragraph::new(line)
                .style(Style::default().bg(Color::Indexed(236)).fg(Color::Gray))
                .render(area, buf);
        }

        fn set_driver(&mut self, driver: HostDriver) {
            *WHICHKEY_DRIVER.lock().unwrap() = Some(driver);
        }

        fn tick(&mut self) -> bool {
            // A publishing surface ticks the bar when it changes the line; always
            // redraw on a tick (the surface only ticks when something changed).
            true
        }
    }

    // -----------------------------------------------------------------------
    // The native `$EDITOR` drop (130-native-detail/030; first slice of ADR-0020
    // §6 embedded-tool observability).
    //
    // v1 ran `$EDITOR` by suspending the tty (`process_pending_action`'s
    // `suspended()`); a host surface has no tty (it renders in the server daemon —
    // ADR-0021), so instead it asks trellis to run `$EDITOR <tempfile>` as a real
    // terminal pane and signal `editor_exited` when the child exits. The flow is
    // two-phase: `begin_pending_edit` seeds the tempfile and opens the pane, then
    // `finish_edit` reads it back once trellis reports the exit. Shared verbatim by
    // the home dashboard and the per-grove detail surfaces.
    // -----------------------------------------------------------------------

    /// A `$EDITOR` drop in flight: the seeded tempfile (held alive until the editor
    /// exits) plus what to do with the edited text. A surface stashes one between
    /// its `open_editor` request and the `editor_exited` that completes the flow.
    enum PendingEdit {
        /// Edit the in-memory capture draft; on exit the edited text becomes the
        /// capture body (v1 `PendingAction::EditBody`).
        Body { tempfile: NamedTempFile },
        /// Edit a committed inbox observation; on a non-empty change, round-trip
        /// through `grove-llm inbox-edit` (v1 `PendingAction::EditObservation`).
        Observation {
            tempfile: NamedTempFile,
            path: PathBuf,
            original: String,
            grove: String,
        },
    }

    /// Seed a tempfile with `initial` and ask trellis to open `$EDITOR` on it as a
    /// terminal pane whose exit it will observe. Returns the live tempfile handle to
    /// hold until [`finish_edit`] reads it back; `Err` if the tempfile can't be
    /// created or written.
    fn seed_editor(driver: &HostDriver, initial: &str) -> Result<NamedTempFile> {
        let tf = tempfile::Builder::new()
            .prefix("grove-edit-")
            .suffix(".md")
            .tempfile()
            .context("creating editor tempfile")?;
        std::fs::write(tf.path(), initial)
            .with_context(|| format!("seeding editor tempfile {}", tf.path().display()))?;
        driver.open_editor(tf.path().to_path_buf());
        Ok(tf)
    }

    /// Begin a `$EDITOR` drop for an `EditBody`/`EditObservation` the v1 handler
    /// queued: seed the tempfile and open the editor pane. Returns the in-flight
    /// [`PendingEdit`] to stash, or `None` (recording a status) when there is no
    /// driver yet or the seed/read fails.
    fn begin_pending_edit(
        app: &mut App,
        driver: &Option<HostDriver>,
        action: PendingAction,
    ) -> Option<PendingEdit> {
        let Some(driver) = driver.as_ref() else {
            app.status = Some("editor unavailable (surface not mounted)".to_string());
            return None;
        };
        match action {
            PendingAction::EditBody => match seed_editor(driver, &app.capture.body) {
                Ok(tempfile) => Some(PendingEdit::Body { tempfile }),
                Err(e) => {
                    app.status = Some(format!("editor failed: {}", short_err(&e)));
                    None
                }
            },
            PendingAction::EditObservation { path } => {
                let original = match std::fs::read_to_string(&path) {
                    Ok(s) => s,
                    Err(e) => {
                        app.status = Some(format!(
                            "reading observation failed: {}",
                            short_err(&anyhow::anyhow!(e))
                        ));
                        return None;
                    }
                };
                let grove = app
                    .detail
                    .as_ref()
                    .map(|d| d.grove.clone())
                    .unwrap_or_default();
                match seed_editor(driver, &original) {
                    Ok(tempfile) => Some(PendingEdit::Observation {
                        tempfile,
                        path,
                        original,
                        grove,
                    }),
                    Err(e) => {
                        app.status = Some(format!("editor failed: {}", short_err(&e)));
                        None
                    }
                }
            }
            // Only the two edit actions reach here; anything else is a caller bug.
            _ => None,
        }
    }

    /// Complete a `$EDITOR` drop once trellis reports the editor exited: read the
    /// tempfile back and apply it. Mirrors `process_pending_action`'s
    /// `EditBody`/`EditObservation` arms minus the tty suspend (the editor ran as a
    /// real pane). A non-zero/signalled exit is treated as "abort, leave unchanged",
    /// faithful to v1's `shell_editor`, which errored on a non-zero status.
    fn finish_edit(app: &mut App, pending: PendingEdit, exit_status: Option<i32>) {
        if exit_status != Some(0) {
            app.status = Some("editor exited without saving; left unchanged".to_string());
            return;
        }
        match pending {
            PendingEdit::Body { tempfile } => match std::fs::read_to_string(tempfile.path()) {
                Ok(new_body) => {
                    app.capture.body = new_body;
                    app.capture.field = CaptureField::Body;
                }
                Err(e) => {
                    app.status = Some(format!(
                        "reading edited body failed: {}",
                        short_err(&anyhow::anyhow!(e))
                    ));
                }
            },
            PendingEdit::Observation {
                tempfile,
                path,
                original,
                grove,
            } => {
                let name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_default();
                let edited = match std::fs::read_to_string(tempfile.path()) {
                    Ok(s) => s,
                    Err(e) => {
                        app.status = Some(format!(
                            "reading edited observation failed: {}",
                            short_err(&anyhow::anyhow!(e))
                        ));
                        return;
                    }
                };
                match decide_observation_edit(&path, &original, edited) {
                    Ok(EditOutcome::Unchanged) => app.status = Some(format!("{name}: unchanged")),
                    // The fs-watch on the inbox fires on the rename, debounces, and
                    // rescans — the renamed entry then reappears.
                    Ok(EditOutcome::Saved) => {
                        app.status = Some(format!("edited {name} in {grove}"))
                    }
                    Err(e) => app.status = Some(format!("edit failed: {}", short_err(&e))),
                }
                let _ = app.refresh_silent();
            }
        }
    }

    /// Build the dashboard surface for `repo`: scan it, seed `App`, and prepare
    /// an off-screen render target. The fs-watch thread is *not* started here —
    /// it needs the [`HostDriver`], which only arrives in [`set_driver`] once the
    /// pane is constructed on the server's screen thread.
    ///
    /// Returns `Err` only if the initial scan fails (a repo with no readable
    /// grove state) — the caller surfaces that rather than rendering a broken
    /// dashboard.
    pub fn dashboard_surface(repo: PathBuf) -> Result<DashboardSurface> {
        let view = RepoView::scan(&repo)?;
        let preselect = current_grove_name(&repo);
        let mut app = App::new(repo.clone(), view, preselect);
        // The nav renders in the native frame: suppress its own footer; the
        // grove-owned whichkey bar (leaf 140) owns the bottom hint line.
        app.native_chrome = true;
        // Initial size is a placeholder; the first `draw` resizes to the pane.
        let terminal = Terminal::new(TestBackend::new(80, 24))
            .map_err(|e| anyhow::anyhow!("building the off-screen render target: {e}"))?;
        Ok(DashboardSurface {
            repo,
            app,
            terminal,
            driver: None,
            open_harnesses: BTreeSet::new(),
            pending_edit: None,
            _watcher: None,
        })
    }

    /// The v1 dashboard as a trellis [`HostSurface`]. Wraps the unchanged `App`
    /// and renders it through an off-screen [`TestBackend`] terminal so the v1
    /// `render(f, app)` is reused verbatim; the resulting cells are blitted into
    /// the pane buffer trellis composites.
    pub struct DashboardSurface {
        /// The repo whose groves the dashboard surfaces; fs-watch roots derive
        /// from it.
        repo: PathBuf,
        /// The unchanged v1 dashboard state.
        app: App,
        /// Off-screen render target — the trick that lets the v1 `render`
        /// (which needs a `Frame`) run without a real terminal. We draw into it,
        /// then copy its buffer into the host pane buffer.
        terminal: Terminal<TestBackend>,
        /// The layout/redraw handle, set once at first-layout. `None` until then.
        driver: Option<HostDriver>,
        /// Grove names with an open harness tab — the native, name-keyed analogue
        /// of the retired `HarnessTabs` id map (the screen thread addresses tabs
        /// by name, so no numeric id round-trip is needed).
        open_harnesses: BTreeSet<String>,
        /// A `$EDITOR` drop in flight (a capture-body edit from the nav), held from
        /// the `open_editor` request until `editor_exited` reads the tempfile back.
        pending_edit: Option<PendingEdit>,
        /// Kept alive so the fs-watch thread's channel stays open; dropping it
        /// (on surface drop) closes the channel and the thread exits cleanly.
        _watcher: Option<RecommendedWatcher>,
    }

    impl DashboardSurface {
        /// Run a deferred action the v1 `handle_key` queued. The non-interactive
        /// `grove-llm` writes run **synchronously on the screen thread** (sub-second
        /// git commits; the v1 `suspended()` tty dance is gone because the surface
        /// no longer owns a tty). Harness open/close/focus drive tabs by direct
        /// `HostDriver` call. A `$EDITOR` drop (`Ctrl-E` on a capture draft) opens a
        /// real trellis editor pane and reads the result back on exit
        /// ([`begin_pending_edit`] / [`finish_edit`], 130-native-detail/030).
        fn process_action(&mut self, action: PendingAction) {
            match action {
                PendingAction::Submit => {
                    let target = self.app.capture.target.clone();
                    let body = self.app.capture.body.clone();
                    match shell_capture(&target, &body) {
                        Ok(()) => self.app.status = Some(format!("captured to {target}")),
                        Err(e) => {
                            self.app.status = Some(format!("capture failed: {}", short_err(&e)))
                        }
                    }
                    self.app.capture = CaptureModal::default();
                    let _ = self.app.refresh_silent();
                }
                PendingAction::Drain { path, disposition } => {
                    let grove = self
                        .app
                        .detail
                        .as_ref()
                        .map(|d| d.grove.clone())
                        .unwrap_or_default();
                    let name = path
                        .file_name()
                        .map(|n| n.to_string_lossy().into_owned())
                        .unwrap_or_default();
                    match shell_drain(&grove, &path, disposition) {
                        Ok(()) => self.app.status = Some(format!("{} {}", disposition.label(), name)),
                        Err(e) => {
                            self.app.status =
                                Some(format!("disposition failed: {}", short_err(&e)))
                        }
                    }
                    let _ = self.app.refresh_silent();
                }
                PendingAction::OpenHarness { name, repo } => {
                    let Some(driver) = self.driver.clone() else {
                        return;
                    };
                    // Drive the content swap (ADR-0022/0023, pair-aware per 130/020):
                    // mount this grove's `grove do <name>` harness **and** its
                    // per-grove detail into the content region beside the constant
                    // nav, parking the previously-selected grove's pair alive
                    // off-screen. The screen thread dedupes (already-shown → no-op,
                    // parked → restore, first time → spawn); grove only tracks names
                    // for the status line + first-open detail build. Run the same
                    // binary the server is (dev `target/debug/grove` or an installed).
                    let first_open = !self.open_harnesses.contains(&name);
                    let secondary_surface_key = if first_open {
                        // Build the per-grove detail surface and stash it in the keyed
                        // registry; the screen thread takes it when it mounts the pair.
                        // On a scan failure the harness still swaps in (detail-less);
                        // grove notes it on the status line rather than blocking the
                        // selection.
                        match detail_surface(repo.clone(), name.clone()) {
                            Ok(detail) => {
                                let key = detail_surface_key(&name);
                                register_keyed_host_surface(key.clone(), Box::new(detail));
                                Some(key)
                            }
                            Err(e) => {
                                self.app.status =
                                    Some(format!("detail unavailable: {}", short_err(&e)));
                                None
                            }
                        }
                    } else {
                        // Already open: its detail pane is parked alive and restored by
                        // the swap; no rebuild, no registry entry.
                        None
                    };
                    let grove_bin =
                        std::env::current_exe().unwrap_or_else(|_| PathBuf::from("grove"));
                    // The aux working-set members (terminal/yazi/vcs) run in the grove's
                    // worktree cwd (020-aux-tool-panes), so yazi/lazygit act on the
                    // grove's tree. Passed every swap; the screen thread only spawns them
                    // on first-open (a re-selection restores the parked-alive set).
                    let worktree = crate::repo::grove_worktree(&repo, &name);
                    let shell = std::env::var_os("SHELL").unwrap_or_else(|| "/bin/sh".into());
                    let path = std::env::var_os("PATH").unwrap_or_default();
                    let aux = aux_members(&worktree, &shell, &path);
                    driver.swap_content(
                        &name,
                        repo,
                        grove_bin,
                        vec!["do".to_string(), name.clone()],
                        secondary_surface_key,
                        aux,
                    );
                    if self.open_harnesses.insert(name.clone()) {
                        self.app.status = Some(format!("opened harness: {name}"));
                    } else {
                        self.app.status = Some(format!("switched to harness: {name}"));
                    }
                }
                PendingAction::CloseHarness { name } => {
                    // The content-swap model parks harnesses alive rather than
                    // closing them; there is no per-grove close from the nav (a
                    // lifecycle concern beyond this leaf — `GoToTab`/close-tab
                    // retired with ADR-0023).
                    self.app.status =
                        Some(format!("{name} stays parked (no close in the swap model)"));
                }
                action @ (PendingAction::EditBody | PendingAction::EditObservation { .. }) => {
                    // Open `$EDITOR` as a real trellis pane; `editor_exited` reads it
                    // back. Disjoint borrows of `app` (mut) and `driver` (shared).
                    self.pending_edit = begin_pending_edit(&mut self.app, &self.driver, action);
                }
            }
        }
    }

    impl HostSurface for DashboardSurface {
        fn draw(&mut self, area: Rect, buf: &mut Buffer) {
            // Match the off-screen terminal to the pane's content rect, render the
            // *unchanged* v1 dashboard into it, then blit its cells into the host
            // buffer trellis composites. `area` always has origin (0,0) (the host
            // pane builds it that way), so source and destination coords align.
            let _ = self.terminal.resize(Rect::new(0, 0, area.width, area.height));
            let Self { app, terminal, .. } = self;
            let _ = terminal.draw(|f| render(f, app));
            let src = terminal.backend().buffer();
            let w = area.width.min(src.area.width);
            let h = area.height.min(src.area.height);
            for y in 0..h {
                for x in 0..w {
                    buf[(area.x + x, area.y + y)] = src[(x, y)].clone();
                }
            }
        }

        fn handle_key(&mut self, key: KeyEvent) -> bool {
            // Native-nav select: `Enter` on the grove list opens (or switches to)
            // the selected grove's [[workspace]] tab via the `HostDriver`, instead
            // of the v1 master/detail drill-in. `nav_enter_target` returns `None`
            // when a modal / filter / help is up, so `Enter` still falls through to
            // the shared handler there (e.g. a newline in the capture body). Detail
            // is per-grove-tab in 130; the `--local` dashboard keeps the drill-in.
            if matches!(key.code, KeyCode::Enter) && key.modifiers.is_empty() {
                if let Some(name) = self.app.nav_enter_target() {
                    self.process_action(PendingAction::OpenHarness {
                        name,
                        repo: self.repo.clone(),
                    });
                    return true;
                }
            }

            // Route to the unchanged v1 key handler, then drain any action it
            // queued (capture/drain writes, harness drive). v1's `q` returns
            // `true` ("quit the loop"); natively that means quit the session.
            let quit = match handle_key(&mut self.app, key.code, key.modifiers) {
                Ok(q) => q,
                Err(e) => {
                    self.app.status = Some(format!("key error: {}", short_err(&e)));
                    false
                }
            };
            if let Some(action) = self.app.pending_action.take() {
                self.process_action(action);
            }
            if quit {
                if let Some(driver) = &self.driver {
                    driver.quit();
                }
            }
            // The nav is the focused surface (input only routes to the focused
            // pane), so republish its hints to the whichkey — the key likely moved
            // the selection, toggled a modal, or set a status line.
            publish_whichkey(WhichkeyOwner::Nav, footer_line(&self.app));
            // Always re-render: a key almost always moves the selection or sets a
            // status line, and the cost of an extra draw is trivial.
            true
        }

        fn set_focused(&mut self, focused: bool) {
            // The whichkey bar reflects the focused surface (leaf 140): claim it
            // with the nav's hints on focus, and hand it to the harness context on
            // blur (if the nav still owns it).
            if focused {
                publish_whichkey(WhichkeyOwner::Nav, footer_line(&self.app));
            } else {
                relinquish_whichkey(WhichkeyOwner::Nav);
            }
        }

        fn set_driver(&mut self, driver: HostDriver) {
            self.driver = Some(driver.clone());
            // Replace v1's in-loop `WatchSet` polling with a dedicated fs-watch
            // thread (the shared 110/030 pattern): the home dashboard watches the
            // whole repo (every grove's worktree + every inbox), since the nav lists
            // them all.
            self._watcher = spawn_grove_watch(
                vec![
                    self.repo.join(".grove-worktrees"),
                    self.repo.join(".grove-meta").join("inboxes"),
                ],
                driver,
            );
        }

        fn tick(&mut self) -> bool {
            // An fs-watch settle (or any out-of-band wake): re-scan and redraw.
            if let Err(e) = self.app.refresh_silent() {
                self.app.status = Some(format!("rescan failed: {}", short_err(&e)));
            }
            true
        }

        fn editor_exited(&mut self, exit_status: Option<i32>) -> bool {
            // The `$EDITOR` pane this surface opened (Ctrl-E) exited: read the
            // tempfile back and apply it. `take` so a stray double-signal is inert.
            match self.pending_edit.take() {
                Some(pending) => {
                    finish_edit(&mut self.app, pending, exit_status);
                    true
                }
                None => false,
            }
        }
    }

    /// Spawn the shared fs-watch → debounce → `request_tick` thread for a host
    /// surface (the 110/030 pattern, factored so the home dashboard and each
    /// per-grove detail reuse it). Watches each existing dir in `dirs` recursively,
    /// coalesces bursts under [`DEBOUNCE`], and posts a tick through `driver` when
    /// the filesystem settles — so the surface is only ever mutated on the screen
    /// thread (in `tick`), never from this thread. Returns the [`RecommendedWatcher`]
    /// to keep alive (dropping it closes the channel and ends the thread); `None` on
    /// an exotic platform with no watcher, where manual `r` refresh still works.
    fn spawn_grove_watch(dirs: Vec<PathBuf>, driver: HostDriver) -> Option<RecommendedWatcher> {
        let (tx, rx) = mpsc::channel::<()>();
        let mut watcher = notify::recommended_watcher(move |_res| {
            let _ = tx.send(());
        })
        .ok()?;
        for dir in dirs {
            if dir.is_dir() {
                let _ = watcher.watch(&dir, RecursiveMode::Recursive);
            }
        }
        std::thread::spawn(move || {
            // Block for the first event, then coalesce until quiet for DEBOUNCE,
            // then tick. Mirrors v1's `WatchSet` debounce, moved off the (now
            // event-driven) render path.
            while rx.recv().is_ok() {
                loop {
                    match rx.recv_timeout(DEBOUNCE) {
                        Ok(()) => continue,
                        Err(mpsc::RecvTimeoutError::Timeout) => break,
                        Err(mpsc::RecvTimeoutError::Disconnected) => return,
                    }
                }
                driver.request_tick();
            }
        });
        Some(watcher)
    }

    /// The keyed-registry key under which grove stashes a grove's [`DetailSurface`]
    /// for the content-swap to mount as the secondary pane (ADR-0023). Namespaced so
    /// it never collides with the opaque grove-name key the swap uses for the pair.
    fn detail_surface_key(grove: &str) -> String {
        format!("grove-detail:{grove}")
    }

    /// Build the **aux working-set members** (020-aux-tool-panes) for `worktree`: a
    /// plain **terminal** (`shell`), **yazi** (files), and the **vcs** TUI (lazygit /
    /// lazyjj via [`vcs_tool`]), in that order — matching the `grove-term`/`grove-yazi`/
    /// `grove-vcs` slot order in `GROVE_TUI_LAYOUT`, since `swap_content` maps the aux
    /// members onto the aux slots positionally. Each runs as a command pane in the
    /// grove's `worktree` cwd, so yazi/lazygit operate on the grove's tree. A tool not
    /// found on `path` is replaced by a graceful in-pane message (a held shell printing
    /// "not installed") rather than failing the whole working-set mount — yazi/lazygit
    /// are not guaranteed present. The terminal is the user's `shell`, always present,
    /// so it needs no fallback. `shell`/`path` are passed in (not read from the
    /// environment here) so the composition is deterministic and unit-testable.
    fn aux_members(
        worktree: &Path,
        shell: &std::ffi::OsStr,
        path: &std::ffi::OsStr,
    ) -> Vec<(String, RunCommand)> {
        let in_worktree = |command: PathBuf, args: Vec<String>| RunCommand {
            command,
            args,
            cwd: Some(worktree.to_path_buf()),
            // Hold the pane open if the child exits, so a quick failure (or a tool
            // that exits) shows its output instead of the pane vanishing.
            hold_on_close: true,
            ..RunCommand::default()
        };
        // A tool resolved on `path`, or a graceful "<bin> not installed" shell message.
        let tool = |role: &str, bin: &str| -> (String, RunCommand) {
            let run = match on_path(bin, path) {
                Some(full) => in_worktree(full, vec![]),
                None => in_worktree(
                    PathBuf::from(shell),
                    vec![
                        "-c".to_string(),
                        format!("printf '%s\\n' '{bin} is not installed (not on PATH)'"),
                    ],
                ),
            };
            (role.to_string(), run)
        };
        vec![
            ("terminal".to_string(), in_worktree(PathBuf::from(shell), vec![])),
            tool("yazi", "yazi"),
            tool("vcs", vcs_tool(worktree)),
        ]
    }

    /// Build the per-grove **detail surface** (130-native-detail/020): an `App`
    /// locked to `grove`'s detail (task tree + inbox + capture), scanning `repo`.
    /// Mounted beside that grove's harness in the content region the first time the
    /// grove is selected, then parked alive on switch-away. `Err` only if the initial
    /// scan fails — the caller swaps the harness in detail-less and notes it.
    pub fn detail_surface(repo: PathBuf, grove: String) -> Result<DetailSurface> {
        let view = RepoView::scan(&repo)?;
        let app = App::new_detail(repo.clone(), view, grove.clone());
        let terminal = Terminal::new(TestBackend::new(80, 24))
            .map_err(|e| anyhow::anyhow!("building the off-screen render target: {e}"))?;
        Ok(DetailSurface {
            repo,
            grove,
            app,
            terminal,
            driver: None,
            pending_edit: None,
            _watcher: None,
        })
    }

    /// One grove's **detail** as a trellis [`HostSurface`] (130-native-detail/020):
    /// the v1 master/detail dashboard reused in detail-locked mode (see
    /// [`App::new_detail`]) and rendered through an off-screen [`TestBackend`], so
    /// the v1 `render(f, app)` / `handle_key(app, …)` are reused verbatim. One
    /// instance per grove, mounted beside its harness and **parked alive** (never
    /// dropped) when another grove is selected — so its task/inbox cursor, an
    /// in-flight capture, and its fs-watch all survive a switch-away, with no
    /// cross-talk between groves.
    pub struct DetailSurface {
        /// The repo the grove lives in; fs-watch roots derive from it + the grove
        /// name. The `App` re-scans the whole repo (cheap) but the watch is scoped to
        /// just this grove.
        repo: PathBuf,
        /// The grove this surface is bound to (its worktree + inbox are watched).
        grove: String,
        /// The detail-locked v1 dashboard state, bound to `grove`.
        app: App,
        /// Off-screen render target (same blit trick as the home dashboard).
        terminal: Terminal<TestBackend>,
        /// The redraw/tick handle, set once at mount. `None` until then.
        driver: Option<HostDriver>,
        /// A `$EDITOR` drop in flight (a capture-body or inbox-observation edit),
        /// held from the `open_editor` request until `editor_exited` reads it back.
        pending_edit: Option<PendingEdit>,
        /// Kept alive so the per-grove fs-watch thread's channel stays open.
        _watcher: Option<RecommendedWatcher>,
    }

    impl DetailSurface {
        /// Run a deferred action the v1 `handle_key` queued, scoped to the detail's
        /// in-process powers. Capture (`c` → `Ctrl-S`) and inbox triage (`d`) run the
        /// same synchronous `grove-llm` shell-outs the home dashboard does; a
        /// `$EDITOR` drop (`Ctrl-E` on a capture draft or a selected inbox entry)
        /// opens a real trellis editor pane and reads the result back on exit
        /// ([`begin_pending_edit`] / [`finish_edit`], 130-native-detail/030). Harness
        /// open/close belong to the **nav** (it owns the content swap), so this
        /// surface ignores them — its grove's harness is already mounted beside it.
        fn process_action(&mut self, action: PendingAction) {
            match action {
                PendingAction::Submit => {
                    let target = self.app.capture.target.clone();
                    let body = self.app.capture.body.clone();
                    match shell_capture(&target, &body) {
                        Ok(()) => self.app.status = Some(format!("captured to {target}")),
                        Err(e) => {
                            self.app.status = Some(format!("capture failed: {}", short_err(&e)))
                        }
                    }
                    self.app.capture = CaptureModal::default();
                    let _ = self.app.refresh_silent();
                }
                PendingAction::Drain { path, disposition } => {
                    let name = path
                        .file_name()
                        .map(|n| n.to_string_lossy().into_owned())
                        .unwrap_or_default();
                    match shell_drain(&self.grove, &path, disposition) {
                        Ok(()) => {
                            self.app.status = Some(format!("{} {}", disposition.label(), name))
                        }
                        Err(e) => {
                            self.app.status =
                                Some(format!("disposition failed: {}", short_err(&e)))
                        }
                    }
                    let _ = self.app.refresh_silent();
                }
                action @ (PendingAction::EditBody | PendingAction::EditObservation { .. }) => {
                    // Open `$EDITOR` as a real trellis pane in place of this detail
                    // pane; `editor_exited` reads the tempfile back. For an inbox
                    // entry the readback round-trips through `grove-llm inbox-edit`.
                    self.pending_edit = begin_pending_edit(&mut self.app, &self.driver, action);
                }
                // The nav drives the content swap; the detail surface must never
                // call `swap_content` (it would fight the nav for the content
                // region). Its grove's harness is already mounted beside it.
                PendingAction::OpenHarness { .. } | PendingAction::CloseHarness { .. } => {}
            }
        }
    }

    impl HostSurface for DetailSurface {
        fn draw(&mut self, area: Rect, buf: &mut Buffer) {
            // Same off-screen render + blit as the home dashboard: render the v1
            // (detail-locked) `App` into a `TestBackend` sized to the pane, then copy
            // its cells into the host buffer trellis composites.
            let _ = self.terminal.resize(Rect::new(0, 0, area.width, area.height));
            let Self { app, terminal, .. } = self;
            let _ = terminal.draw(|f| render(f, app));
            let src = terminal.backend().buffer();
            let w = area.width.min(src.area.width);
            let h = area.height.min(src.area.height);
            for y in 0..h {
                for x in 0..w {
                    buf[(area.x + x, area.y + y)] = src[(x, y)].clone();
                }
            }
        }

        fn handle_key(&mut self, key: KeyEvent) -> bool {
            // Route to the unchanged v1 key handler (in detail-locked mode), then
            // drain any action it queued. Unlike the nav, the detail surface never
            // quits the session on the handler's `true` — session lifecycle (`q` /
            // quit) belongs to the nav; here `q`/`Esc` are inert (detail-locked).
            if let Err(e) = handle_key(&mut self.app, key.code, key.modifiers) {
                self.app.status = Some(format!("key error: {}", short_err(&e)));
            }
            if let Some(action) = self.app.pending_action.take() {
                self.process_action(action);
            }
            // This detail surface is the focused one (input only routes to the
            // focused pane); republish its hints to the whichkey.
            publish_whichkey(WhichkeyOwner::Detail, footer_line(&self.app));
            true
        }

        fn set_focused(&mut self, focused: bool) {
            // Claim the whichkey with this grove's detail hints on focus; hand it to
            // the harness context on blur (if this detail still owns it).
            if focused {
                publish_whichkey(WhichkeyOwner::Detail, footer_line(&self.app));
            } else {
                relinquish_whichkey(WhichkeyOwner::Detail);
            }
        }

        fn set_driver(&mut self, driver: HostDriver) {
            self.driver = Some(driver.clone());
            // Per-grove fs-watch (narrower than the home dashboard's repo-wide
            // watch): just this grove's worktree `.grove/` tree and its inbox. Fewer
            // handles and far less `.git/` churn, and a tick only ever refreshes
            // *this* surface (the driver carries this pane's id) — no cross-talk.
            self._watcher = spawn_grove_watch(
                vec![
                    self.repo
                        .join(".grove-worktrees")
                        .join(&self.grove)
                        .join(".grove"),
                    self.repo.join(".grove-meta").join("inboxes").join(&self.grove),
                ],
                driver,
            );
        }

        fn tick(&mut self) -> bool {
            if let Err(e) = self.app.refresh_silent() {
                self.app.status = Some(format!("rescan failed: {}", short_err(&e)));
            }
            true
        }

        fn editor_exited(&mut self, exit_status: Option<i32>) -> bool {
            // The `$EDITOR` pane this detail opened (Ctrl-E) exited: read it back and
            // apply (capture body, or an inbox-edit round-trip). `take` so a stray
            // double-signal is inert.
            match self.pending_edit.take() {
                Some(pending) => {
                    finish_edit(&mut self.app, pending, exit_status);
                    true
                }
                None => false,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        /// A bare repo with the two dirs `RepoView::scan` reads, so an `App` can be
        /// built without a full grove fixture (these tests only exercise the
        /// `$EDITOR` readback, not rendering).
        fn empty_app() -> App {
            let tmp = tempfile::tempdir().unwrap();
            std::fs::create_dir_all(tmp.path().join(".grove-worktrees")).unwrap();
            std::fs::create_dir_all(tmp.path().join(".grove-meta").join("inboxes")).unwrap();
            let view = RepoView::scan(tmp.path()).unwrap();
            // Keep the tempdir alive for the App's lifetime by leaking it — the test
            // process is short-lived and never re-scans.
            std::mem::forget(tmp);
            App::new(std::env::temp_dir(), view, None)
        }

        #[test]
        fn finish_edit_body_reads_tempfile_into_the_capture_draft() {
            let mut app = empty_app();
            app.capture.body = "old draft".into();
            let tf = tempfile::Builder::new().suffix(".md").tempfile().unwrap();
            std::fs::write(tf.path(), "edited in $EDITOR").unwrap();

            finish_edit(&mut app, PendingEdit::Body { tempfile: tf }, Some(0));
            assert_eq!(app.capture.body, "edited in $EDITOR");
            assert_eq!(app.capture.field, CaptureField::Body);
        }

        #[test]
        fn finish_edit_aborts_on_a_nonzero_exit() {
            // A non-zero / signalled editor exit (e.g. vim `:cq`) means "abort":
            // the draft is left exactly as it was, faithful to v1's `shell_editor`.
            let mut app = empty_app();
            app.capture.body = "untouched".into();
            let tf = tempfile::Builder::new().suffix(".md").tempfile().unwrap();
            std::fs::write(tf.path(), "edited but aborted").unwrap();

            finish_edit(&mut app, PendingEdit::Body { tempfile: tf }, Some(1));
            assert_eq!(app.capture.body, "untouched");
        }

        /// The whichkey bar renders whatever line the focused surface published, and
        /// the publish/relinquish ownership keeps the hand-off order-independent
        /// (leaf 140). One test because the published line is a process-global —
        /// keeping the steps in a single test makes them sequential (no inter-test
        /// races on the shared `WHICHKEY_LINE`).
        #[test]
        fn whichkey_publishes_renders_and_hands_off() {
            // The bar renders the published line.
            publish_whichkey(WhichkeyOwner::Nav, Line::from("⏎ open · ⌃o nav"));
            let mut wk = whichkey_surface();
            let area = Rect::new(0, 0, 40, 1);
            let mut buf = Buffer::empty(area);
            wk.draw(area, &mut buf);
            let row: String = (0..area.width)
                .map(|x| buf[(x, 0)].symbol().chars().next().unwrap_or(' '))
                .collect();
            assert!(row.contains("open"), "whichkey shows published hints: {row:?}");
            assert!(row.contains("nav"), "whichkey shows the leader hint: {row:?}");

            // A non-owner losing focus is inert: the nav never owned the bar here
            // (detail does), so detail's line stands.
            publish_whichkey(WhichkeyOwner::Detail, Line::from("detail hints"));
            relinquish_whichkey(WhichkeyOwner::Nav);
            let owner = WHICHKEY_LINE.lock().unwrap().as_ref().map(|(o, _)| *o);
            assert_eq!(owner, Some(WhichkeyOwner::Detail), "non-owner blur is inert");

            // The owner losing focus hands the bar to the harness context.
            relinquish_whichkey(WhichkeyOwner::Detail);
            let owner = WHICHKEY_LINE.lock().unwrap().as_ref().map(|(o, _)| *o);
            assert_eq!(owner, Some(WhichkeyOwner::Harness), "owner blur → harness");
        }

        #[test]
        fn aux_members_compose_terminal_yazi_vcs_in_the_worktree_cwd() {
            use std::ffi::OsStr;
            // A worktree with no `.jj/` (vcs → lazygit) and an empty PATH dir (no aux
            // tool resolves) → every member runs in the worktree cwd, and the unfound
            // tools fall back to a graceful shell message rather than failing the mount.
            let tmp = tempfile::tempdir().unwrap();
            let worktree = tmp.path();
            let members = aux_members(worktree, OsStr::new("/bin/zsh"), OsStr::new(""));

            let roles: Vec<&str> = members.iter().map(|(r, _)| r.as_str()).collect();
            assert_eq!(
                roles,
                ["terminal", "yazi", "vcs"],
                "aux members are terminal, yazi, vcs in slot order"
            );
            for (_, run) in &members {
                assert_eq!(
                    run.cwd.as_deref(),
                    Some(worktree),
                    "every aux member runs in the grove's worktree cwd"
                );
            }
            // The terminal is the shell itself; the unfound yazi/vcs fall back to the
            // shell printing a "not installed" message (graceful, not a failed mount).
            assert_eq!(members[0].1.command, PathBuf::from("/bin/zsh"));
            assert!(members[0].1.args.is_empty(), "the terminal is a bare shell");
            assert_eq!(members[1].1.command, PathBuf::from("/bin/zsh"), "yazi falls back to the shell");
            assert!(
                members[1].1.args.last().is_some_and(|a| a.contains("yazi is not installed")),
                "the yazi fallback prints a graceful message"
            );
            assert!(
                members[2].1.args.last().is_some_and(|a| a.contains("lazygit is not installed")),
                "the vcs fallback names the resolved tool (lazygit for a git worktree)"
            );
        }

        #[test]
        fn aux_members_resolve_a_present_tool_to_its_full_path() {
            use std::ffi::OsStr;
            // With lazygit present on PATH, the vcs member runs that binary directly
            // (no fallback) — in the worktree cwd, no args.
            let tmp = tempfile::tempdir().unwrap();
            let bindir = tmp.path().join("bin");
            std::fs::create_dir(&bindir).unwrap();
            std::fs::write(bindir.join("lazygit"), "#!/bin/sh\n").unwrap();
            let members = aux_members(tmp.path(), OsStr::new("/bin/sh"), bindir.as_os_str());

            let vcs = &members[2].1;
            assert_eq!(vcs.command, bindir.join("lazygit"), "vcs resolves to the found lazygit");
            assert!(vcs.args.is_empty(), "a found tool runs with no wrapper args");
        }
    }
}

#[cfg(feature = "trellis-seam")]
pub use native::{dashboard_surface, whichkey_surface};
