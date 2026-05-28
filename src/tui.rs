// The `grove tui` subcommand: a read-only, sync, master/detail navigator
// over one repo's groves (leaf
// `020-design-seed-convention/090-tui-server/020-tui-shell-read-only.md`).
//
// Architecture:
//   - All state derives from a single `RepoView` snapshot scanned at
//     startup. Refresh on `r`; fs-watch refresh and the `c` capture
//     shell-out land in leaf 030 — explicitly out of scope here.
//   - `App` owns the snapshot plus screen/selection state. Rendering is
//     a pure function of `App` + the screen rect, which keeps the
//     `TestBackend` snapshot test honest.
//   - The Ratatui event loop is the standard sync poll/read pattern
//     (see ratatui 0.29 docs). No tokio, no notify yet.
//
// Walk-away-ability (SKILL.md constraint 6) is preserved by never
// writing — every keystroke either selects, scrolls, opens help, or
// quits. The future `c` keybind in leaf 030 will shell out to
// `grove inbox add` rather than touching grove state directly.

use anyhow::Result;
use std::path::{Path, PathBuf};
use std::time::Duration;

use ratatui::backend::Backend;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::{Frame, Terminal};

use crate::cli::RepoArgs;
use crate::repo;
use crate::repo_view::{
    self, GroveDetail, GroveSummary, Lifecycle, RepoView, TaskEntry, TaskKind,
};

pub fn run(args: &RepoArgs) -> Result<()> {
    let repo = repo::resolve(args.repo.as_deref())?;
    let view = RepoView::scan(&repo)?;
    let preselect = current_grove_name(&repo);
    let app = App::new(repo, view, preselect);

    let mut terminal = ratatui::init();
    let outcome = event_loop(&mut terminal, app);
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
        }
    }

    fn refresh(&mut self) -> Result<()> {
        // Preserve which grove the user was looking at across the rescan.
        let current_grove = match self.screen {
            Screen::GroveDetail => self.detail.as_ref().map(|d| d.grove.clone()),
            Screen::GroveList => self
                .filtered_groves()
                .get(self.list.selected().unwrap_or(0))
                .map(|g| g.name.clone()),
        };
        self.view = RepoView::scan(&self.repo)?;
        self.filter.text.clear();
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
            } else {
                // Reset detail nav — tree shape may have changed.
                if let Some(d) = self.detail.as_mut() {
                    d.tree.select(Some(0));
                    d.right_scroll = 0;
                }
            }
        }
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
// Event loop

fn event_loop<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
    loop {
        terminal.draw(|f| render(f, &app))?;
        if !event::poll(Duration::from_millis(200))? {
            continue;
        }
        match event::read()? {
            Event::Key(k) if k.kind == KeyEventKind::Press => {
                if handle_key(&mut app, k.code, k.modifiers)? {
                    return Ok(());
                }
            }
            _ => {}
        }
    }
}

/// Returns true when the app should exit.
fn handle_key(app: &mut App, code: KeyCode, mods: KeyModifiers) -> Result<bool> {
    // Help overlay: any key dismisses.
    if app.show_help {
        app.show_help = false;
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
    let [main, footer] =
        Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).areas(area);

    match app.screen {
        Screen::GroveList => render_grove_list(f, main, app),
        Screen::GroveDetail => render_grove_detail(f, main, app),
    }
    render_footer(f, footer, app);

    if app.show_help {
        render_help_overlay(f, area);
    }
}

fn render_grove_list(f: &mut Frame, area: Rect, app: &App) {
    let filtered = app.filtered_groves();
    let items: Vec<ListItem> = filtered.iter().map(|g| ListItem::new(grove_row(g))).collect();
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

fn grove_row(g: &GroveSummary) -> Line<'static> {
    let badge = match g.lifecycle {
        Lifecycle::Live => Span::styled(" live ", Style::default().fg(Color::Green)),
        Lifecycle::Seed => Span::styled(" seed ", Style::default().fg(Color::Yellow)),
    };
    let inbox = if g.inbox_pending > 0 {
        Span::styled(
            format!("  inbox:{}", g.inbox_pending),
            Style::default().fg(Color::Cyan),
        )
    } else {
        Span::raw("".to_string())
    };
    Line::from(vec![
        badge,
        Span::raw(" "),
        Span::raw(g.name.clone()),
        Span::raw(format!("  leaves:{}/{}", g.live_leaves, g.retired_leaves)),
        inbox,
    ])
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
            Screen::GroveList => "Enter=open  j/k=move  /=filter  r=refresh  ?=help  q=quit",
            Screen::GroveDetail => "Tab=cycle  j/k=move  PgUp/PgDn=scroll  /=filter  r=refresh  Esc=back  ?=help",
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
        Line::from("  r             rescan the repo"),
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
}
