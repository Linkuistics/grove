//! A read-only browser over Grove's typed tree reader.
//!
//! `Viewer` owns display data only. Every action attempts a quiet read and
//! drops the shared guard before rendering or waiting for input.

mod markdown;
mod observation;
mod terminal;

use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::{Duration, Instant};

use ratatui::{
    layout::{Constraint, Layout},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use markdown::{Anchor, Document};
use observation::{capture, safe_text, Item, Observation, Root, Row};
pub use terminal::run;

/// Inputs shared by the terminal driver and application tests.
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    Focus,
    Help,
    Dismiss,
    Toggle,
    PageUp,
    PageDown,
    Refresh,
    Quit,
}

const POLL_INTERVAL: Duration = Duration::from_millis(500);

#[derive(Clone, Default)]
struct ReadingPosition {
    source: Rc<str>,
    anchor: Anchor,
    horizontal: usize,
}

/// An in-memory view of one worktree's `.grove`, with no persistence.
pub struct Viewer {
    worktree: PathBuf,
    rows: Vec<Row>,
    selected: usize,
    tree_state: ListState,
    source: Rc<str>,
    content: Document,
    positions: HashMap<Item, ReadingPosition>,
    restore_anchor: Option<Anchor>,
    scroll: usize,
    horizontal: usize,
    content_width: usize,
    page_width: usize,
    file_active: bool,
    help: bool,
    small: bool,
    page_height: usize,
    status: String,
    next_poll: Instant,
    root: Option<Root>,
    source_item: Option<Item>,
    file_error: Option<String>,
    notice: Option<String>,
}

impl Viewer {
    /// Open an observation path. Absence and read errors become visible states.
    /// The CLI supplies an absolute path; this constructor never searches upward.
    pub fn new(worktree: PathBuf) -> Self {
        let mut viewer = Self {
            worktree,
            rows: Vec::new(),
            selected: 0,
            tree_state: ListState::default(),
            source: Rc::default(),
            content: Document::default(),
            positions: HashMap::new(),
            restore_anchor: None,
            scroll: 0,
            horizontal: 0,
            content_width: 0,
            page_width: 1,
            file_active: false,
            help: false,
            small: false,
            page_height: 1,
            status: String::new(),
            next_poll: Instant::now(),
            root: None,
            source_item: None,
            file_error: None,
            notice: None,
        };
        viewer.refresh();
        viewer
    }

    /// Deliver an input. Returns true when the caller should leave the viewer.
    pub fn act(&mut self, action: Action) -> bool {
        match action {
            Action::Quit => return true,
            Action::Refresh => {
                self.refresh();
                return false;
            }
            Action::Dismiss => {
                self.help = false;
                return false;
            }
            _ => {}
        }
        if self.small {
            return false;
        }
        if matches!(action, Action::Help) {
            self.help = !self.help;
            return false;
        }
        if self.help {
            return false;
        }
        match action {
            Action::Quit | Action::Refresh | Action::Help | Action::Dismiss => {}
            Action::Focus => self.file_active = !self.file_active,
            Action::Up | Action::Down if self.file_active => {
                self.scroll = if matches!(action, Action::Up) {
                    self.scroll.saturating_sub(1)
                } else {
                    self.scroll.saturating_add(1)
                };
                self.clamp_scroll();
            }
            Action::Up | Action::Down => {
                let visible = self.visible();
                if let Some(at) = visible.iter().position(|&row| row == self.selected) {
                    let next = if matches!(action, Action::Up) {
                        at.saturating_sub(1)
                    } else {
                        (at + 1).min(visible.len() - 1)
                    };
                    if at != next {
                        self.select(visible[next]);
                    }
                }
            }
            Action::Toggle => {
                if self.file_active {
                    return false;
                }
                if let Some(row) = self.rows.get_mut(self.selected) {
                    if row.branch {
                        row.expanded = !row.expanded;
                    }
                }
            }
            Action::Left | Action::Right if self.file_active => {
                self.horizontal = if matches!(action, Action::Left) {
                    self.horizontal.saturating_sub(1)
                } else {
                    self.horizontal.saturating_add(1)
                };
                self.clamp_scroll();
            }
            Action::Left | Action::Right => self.tree_horizontal(matches!(action, Action::Right)),
            Action::Home | Action::End => {
                if self.file_active {
                    self.scroll = if matches!(action, Action::Home) {
                        0
                    } else {
                        self.content.lines.len()
                    };
                    self.clamp_scroll();
                } else {
                    let visible = self.visible();
                    let target = if matches!(action, Action::Home) {
                        visible.first()
                    } else {
                        visible.last()
                    };
                    if let Some(&target) = target {
                        self.select(target);
                    }
                }
            }
            Action::PageUp if self.file_active => {
                self.scroll = self.scroll.saturating_sub(self.page_height);
            }
            Action::PageDown if self.file_active => {
                self.scroll = self.scroll.saturating_add(self.page_height);
                self.clamp_scroll();
            }
            Action::PageUp | Action::PageDown => {}
        }
        false
    }

    fn save_position(&mut self) {
        if let Some(row) = self.rows.get(self.selected) {
            if self.source_item == Some(row.key) && self.restore_anchor.is_none() {
                self.positions.insert(
                    row.key,
                    ReadingPosition {
                        source: Rc::clone(&self.source),
                        anchor: self.content.anchor(self.scroll),
                        horizontal: self.horizontal,
                    },
                );
            }
        }
    }

    fn select(&mut self, target: usize) {
        if self.selected != target {
            self.save_position();
            self.selected = target;
            self.notice = None;
            self.restore_position();
            self.refresh();
        }
    }

    fn restore_position(&mut self) {
        let position = self
            .rows
            .get(self.selected)
            .and_then(|row| self.positions.get(&row.key))
            .cloned()
            .unwrap_or_default();
        self.restore_anchor = Some(position.anchor);
        self.scroll = 0;
        self.horizontal = position.horizontal;
    }

    fn tree_horizontal(&mut self, right: bool) {
        let Some(row) = self.rows.get_mut(self.selected) else {
            return;
        };
        let depth = row.depth;
        if right {
            if !row.branch {
                return;
            }
            if !row.expanded {
                row.expanded = true;
            } else if self
                .rows
                .get(self.selected + 1)
                .is_some_and(|child| child.depth > depth)
            {
                self.select(self.selected + 1);
            }
        } else if row.branch && row.expanded {
            row.expanded = false;
        } else if let Some(parent) = self.rows[..self.selected]
            .iter()
            .rposition(|row| row.depth < depth)
        {
            self.select(parent);
        }
    }

    fn refresh(&mut self) {
        self.refresh_at(Instant::now());
    }

    /// Observe root identity outside the tree guard, even when a writer is busy.
    fn sync_root(&mut self) -> anyhow::Result<bool> {
        // Metadata remains available when the new directory cannot be opened.
        // Discard the old lifetime before attempting that fallible open.
        let removed = match &self.root {
            Some(root) => !root.at(&self.worktree)?,
            None => false,
        };
        if removed {
            self.clear();
            self.root = None;
        }
        let current = Root::open(&self.worktree)?;
        let changed = match (&self.root, &current) {
            (Some(old), Some(new)) => !old.same(new)?,
            (None, None) => false,
            _ => true,
        };
        if changed {
            self.clear();
            self.root = current;
        }
        Ok(changed || removed)
    }

    fn refresh_at(&mut self, now: Instant) {
        // A single deadline coalesces all selection/manual/timed observations.
        self.next_poll = now + POLL_INTERVAL;
        if let Err(error) = self.sync_root() {
            self.failed(&error);
            return;
        }
        if self.root.is_none() {
            self.missing();
            return;
        }
        self.save_position();
        let old_key = self.rows.get(self.selected).map(|row| row.key);
        let mut candidates = Vec::new();
        if let Some(row) = self.rows.get(self.selected) {
            candidates.push(row.key);
            let mut depth = row.depth;
            for ancestor in self.rows[..self.selected].iter().rev() {
                if ancestor.depth < depth {
                    candidates.push(ancestor.key);
                    depth = ancestor.depth;
                }
            }
        }
        let observation = capture(&self.worktree, &candidates);
        // A replacement during capture invalidates even an otherwise valid tree.
        match self.sync_root() {
            Ok(true) => {
                self.status = "WAITING — root changed during observation; retrying".into();
                return;
            }
            Err(error) => {
                self.failed(&error);
                return;
            }
            Ok(false) => {}
        }
        match observation {
            Ok(Observation::Ready((mut rows, selected, content))) => {
                let old: HashMap<_, _> = self
                    .rows
                    .iter()
                    .filter(|row| row.branch)
                    .map(|row| (row.key, row.expanded))
                    .collect();
                for row in &mut rows {
                    if row.branch {
                        row.expanded = old.get(&row.key).copied().unwrap_or(true);
                    }
                }
                // Only reveal ancestors when selection changed location or is now
                // hidden. Other collapsed branches keep the user's choice.
                let mut depth = rows[selected].depth;
                for row in rows[..selected].iter_mut().rev() {
                    if row.depth < depth {
                        row.expanded = true;
                        depth = row.depth;
                    }
                }
                let key = rows[selected].key;
                if old_key.is_some_and(|old| old != key) {
                    let disappeared = self
                        .rows
                        .get(self.selected)
                        .map(|row| row.label.clone())
                        .unwrap_or_default();
                    self.notice = Some(format!(
                        "{disappeared} disappeared; selected surviving ancestor"
                    ));
                }
                self.positions
                    .retain(|key, _| rows.iter().any(|row| &row.key == key));
                self.rows = rows;
                self.selected = selected;
                if old_key != Some(key) {
                    self.restore_position();
                } else if self.restore_anchor.is_none() {
                    self.restore_anchor = Some(self.content.anchor(self.scroll));
                }
                self.status = self
                    .notice
                    .clone()
                    .unwrap_or_else(|| "Read-only | live refresh 500 ms".into());
                self.set_content(content);
            }
            Ok(Observation::Busy) => {
                self.status =
                    "WAITING for tree writer — previous display retained; retrying".into();
            }
            Ok(Observation::Vacant) => self.missing(),
            Err(error) => self.failed(&error),
        }
    }

    fn clear(&mut self) {
        self.rows.clear();
        self.source = Rc::default();
        self.content = Document::default();
        self.positions.clear();
        self.restore_anchor = None;
        self.source_item = None;
        self.file_error = None;
        self.notice = None;
        self.selected = 0;
        self.scroll = 0;
        self.horizontal = 0;
        self.content_width = 0;
        self.tree_state = ListState::default();
        self.file_active = false;
    }

    fn missing(&mut self) {
        self.clear();
        self.root = None;
        self.status = "Missing .grove — WAITING; retrying automatically".into();
    }

    fn failed(&mut self, error: &anyhow::Error) {
        let state = if self.rows.is_empty() {
            "Error"
        } else {
            "STALE"
        };
        self.status = format!(
            "{state}: {} — retrying automatically",
            safe_text(&error.to_string())
        );
    }

    /// Deliver current time; perform at most one observation, without backlog.
    pub fn tick(&mut self, now: Instant) {
        if now >= self.next_poll {
            self.refresh_at(now);
        }
    }

    /// Time until the next automatic observation (also used after errors).
    pub fn retry_after(&self, now: Instant) -> Option<Duration> {
        Some(self.next_poll.saturating_duration_since(now))
    }

    fn set_content(&mut self, content: Result<Vec<u8>, String>) {
        let bytes = match content {
            Ok(bytes) => bytes,
            Err(error) => {
                self.file_error = Some(format!(
                    "File error: {} — retrying automatically",
                    safe_text(&error)
                ));
                return;
            }
        };
        self.file_error = None;
        let source = String::from_utf8_lossy(&bytes);
        let key = self.rows.get(self.selected).map(|row| row.key);
        if let Some(position) = key.and_then(|key| self.positions.get(&key)) {
            self.restore_anchor = Some(position.anchor.remap(&position.source, &source));
        }
        if self.source.as_ref() != source {
            self.source = Rc::from(source.as_ref());
            self.content = Document::layout(&self.source, self.page_width);
            self.content_width = self.content.width();
        }
        self.source_item = self.rows.get(self.selected).map(|row| row.key);
        if let Some(anchor) = self.restore_anchor.take() {
            self.scroll = self.content.row_for(anchor);
        }
    }

    fn visible(&self) -> Vec<usize> {
        let mut hidden_below = None;
        self.rows
            .iter()
            .enumerate()
            .filter_map(|(i, row)| {
                if hidden_below.is_some_and(|depth| row.depth > depth) {
                    return None;
                }
                hidden_below = if row.branch && !row.expanded {
                    Some(row.depth)
                } else {
                    None
                };
                Some(i)
            })
            .collect()
    }

    fn clamp_scroll(&mut self) {
        self.scroll = self
            .scroll
            .min(self.content.lines.len().saturating_sub(self.page_height));
        self.horizontal = self
            .horizontal
            .min(self.content_width.saturating_sub(self.page_width));
    }

    /// Draw the same application into a real terminal or Ratatui `TestBackend`.
    pub fn render(&mut self, frame: &mut Frame) {
        self.small = frame.area().width < 60 || frame.area().height < 10;
        if self.small {
            frame.render_widget(
                Paragraph::new(
                    "Resize to at least 60 columns × 10 rows\nr refresh | q/Ctrl-c quit",
                ),
                frame.area(),
            );
            return;
        }
        if self.help {
            frame.render_widget(
                Paragraph::new(
                    "Tab: switch full-width view | r: refresh | q/Ctrl-c: quit\n\
Tree: Up/Down j/k select | Home/End first/last\n\
Right/l expand/enter | Left/h collapse/parent\n\
Enter/Space: toggle branch\n\
File: Up/Down j/k line | Left/Right h/l code/table\n\
PageUp/PageDown Ctrl-u/Ctrl-d: page | Home/End: top/end\n\
Escape: close help | ?: toggle help",
                )
                .block(Block::bordered().title(if self.file_active {
                    "Key help — File active | Tab: Tree"
                } else {
                    "Key help — Tree active | Tab: File"
                })),
                frame.area(),
            );
            return;
        }
        let [header, body, footer] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .areas(frame.area());
        frame.render_widget(
            Paragraph::new(format!(
                "{}\n{}",
                safe_text(&self.worktree.join(".grove").display().to_string()),
                self.status
            )),
            header,
        );
        if !self.file_active {
            let visible = self.visible();
            self.tree_state
                .select(visible.iter().position(|&i| i == self.selected));
            let items: Vec<_> = visible
                .iter()
                .map(|&i| {
                    let row = &self.rows[i];
                    let marker = if row.branch {
                        if row.expanded {
                            "-"
                        } else {
                            "+"
                        }
                    } else {
                        " "
                    };
                    ListItem::new(format!(
                        "{}{marker} {}",
                        "  ".repeat(row.depth.min(100)),
                        row.label
                    ))
                })
                .collect();
            // Stateful List keeps the selection visible; the state owns no tree data.
            // https://docs.rs/ratatui/0.29.0/ratatui/widgets/struct.List.html
            frame.render_stateful_widget(
                List::new(items)
                    .block(Block::bordered().title("Tree [active] | Tab: File"))
                    .highlight_symbol("> ")
                    .highlight_style(Style::default().add_modifier(Modifier::REVERSED)),
                body,
                &mut self.tree_state,
            );
        } else {
            self.page_height = usize::from(body.height.saturating_sub(2)).max(1);
            let width = usize::from(body.width.saturating_sub(2)).max(1);
            if self.page_width != width {
                let anchor = self.content.anchor(self.scroll);
                self.page_width = width;
                self.content = Document::layout(&self.source, width);
                self.content_width = self.content.width();
                self.scroll = self.content.row_for(anchor);
            }
            self.clamp_scroll();
            let lines: Vec<Line<'_>> = self
                .content
                .lines
                .iter()
                .skip(self.scroll)
                .take(self.page_height)
                .map(|line| markdown::clip(line, self.horizontal, self.page_width))
                .collect();
            let paragraph = if let Some(error) = &self.file_error {
                // Diagnostics wrap independently of the saved document/scroll state.
                // https://docs.rs/ratatui/0.29.0/ratatui/widgets/struct.Paragraph.html#method.wrap
                Paragraph::new(error.as_str()).wrap(Wrap { trim: false })
            } else {
                Paragraph::new(lines)
            };
            // Layout already wraps prose; slicing avoids u16 scroll limits.
            // https://docs.rs/ratatui/0.29.0/ratatui/widgets/struct.Paragraph.html
            frame.render_widget(
                paragraph.block(Block::bordered().title("File [active] (Markdown) | Tab: Tree")),
                body,
            );
        }
        frame.render_widget(
            Paragraph::new(if self.file_active {
                "File | Tab: Tree | ? help | r refresh | q/Ctrl-c quit"
            } else {
                "Tree | Tab: File | ? help | r refresh | q/Ctrl-c quit"
            }),
            footer,
        );
    }
}
