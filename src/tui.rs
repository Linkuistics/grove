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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PendingAction {
    /// Submit `app.capture` via `grove-llm inbox-add`.
    Submit,
    /// Drop into `$EDITOR` (or `vi` fallback) to edit `app.capture.body`.
    EditBody,
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
                app.detail = Some(DetailState {
                    grove: g,
                    tree,
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
        (Screen::GroveDetail, KeyCode::Down | KeyCode::Char('j')) => {
            let rows = flat_rows_len(app);
            if let Some(d) = app.detail.as_mut() {
                move_selection(&mut d.tree, rows as isize, 1);
                d.right_scroll = 0;
            }
        }
        (Screen::GroveDetail, KeyCode::Up | KeyCode::Char('k')) => {
            let rows = flat_rows_len(app);
            if let Some(d) = app.detail.as_mut() {
                move_selection(&mut d.tree, rows as isize, -1);
                d.right_scroll = 0;
            }
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
    if app.show_help {
        render_help_overlay(f, area);
    }
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

    // Right pane.
    let (title, body) = right_pane_content(app, grove_detail, &rows);
    let para = Paragraph::new(body)
        .block(Block::default().borders(Borders::ALL).title(title))
        .wrap(Wrap { trim: false })
        .scroll((detail.right_scroll, 0));
    f.render_widget(para, right);
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
        RightPane::Inbox => {
            let title = format!("inbox ({})", detail.inbox.len());
            if detail.inbox.is_empty() {
                (title, "(no pending observations)".into())
            } else {
                let body = detail
                    .inbox
                    .iter()
                    .map(|p| {
                        p.file_name()
                            .map(|n| n.to_string_lossy().into_owned())
                            .unwrap_or_default()
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                (title, body)
            }
        }
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
        // Seed for grove "beta": inbox only, no worktree.
        let beta_inbox = root.join(".grove-meta/inboxes/beta");
        fs::create_dir_all(&beta_inbox).unwrap();
        touch(
            &beta_inbox.join("2026-05-28T10-00-00Z--note.md"),
            "first observation\n",
        );
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
        // leaf → inbox
        handle_key(&mut app, KeyCode::Tab, KeyModifiers::NONE).unwrap();
        let out = render_to_buffer(&app, 100, 16);
        assert!(out.contains("no pending observations"), "inbox empty msg missing:\n{}", out);
        // inbox → brief
        handle_key(&mut app, KeyCode::Tab, KeyModifiers::NONE).unwrap();
        let out = render_to_buffer(&app, 100, 16);
        assert!(out.contains("alpha — brief"), "root brief missing:\n{}", out);
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
