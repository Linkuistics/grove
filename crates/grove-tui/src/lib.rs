//! A read-only browser over Grove's typed tree reader.
//!
//! `Viewer` owns display data only. Every action attempts a quiet read and
//! drops the shared guard before rendering or waiting for input.

mod observation;
mod terminal;

use std::path::PathBuf;
use std::time::{Duration, Instant};

use ratatui::{
    layout::{Constraint, Layout},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, List, ListItem, ListState, Paragraph},
    Frame,
};

use observation::{capture, read_selected, safe_text, Observation, Row};
pub use terminal::run;

/// Inputs shared by the terminal driver and application tests.
pub enum Action {
    Up,
    Down,
    Toggle,
    PageUp,
    PageDown,
    Refresh,
    Quit,
}

#[derive(Clone, Copy)]
enum Request {
    Refresh,
    Selected,
}

const RETRY_INTERVAL: Duration = Duration::from_millis(500);

/// An in-memory view of one worktree's `.grove`, with no persistence.
pub struct Viewer {
    worktree: PathBuf,
    rows: Vec<Row>,
    selected: usize,
    tree_state: ListState,
    content: Vec<String>,
    scroll: usize,
    page_height: usize,
    status: String,
    pending: Option<(Request, Instant)>,
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
            content: Vec::new(),
            scroll: 0,
            page_height: 1,
            status: String::new(),
            pending: None,
        };
        viewer.refresh();
        viewer
    }

    /// Deliver an input. Returns true when the caller should leave the viewer.
    pub fn act(&mut self, action: Action) -> bool {
        match action {
            Action::Quit => return true,
            Action::Refresh => self.refresh(),
            Action::Up | Action::Down => {
                let visible = self.visible();
                if let Some(at) = visible.iter().position(|&row| row == self.selected) {
                    let next = if matches!(action, Action::Up) {
                        at.saturating_sub(1)
                    } else {
                        (at + 1).min(visible.len() - 1)
                    };
                    if at != next {
                        self.selected = visible[next];
                        self.scroll = 0;
                        self.load_selected(Instant::now());
                    }
                }
            }
            Action::Toggle => {
                if let Some(row) = self.rows.get_mut(self.selected) {
                    if row.branch {
                        row.expanded = !row.expanded;
                    }
                }
            }
            Action::PageUp => self.scroll = self.scroll.saturating_sub(self.page_height),
            Action::PageDown => {
                self.scroll = self.scroll.saturating_add(self.page_height);
                self.clamp_scroll();
            }
        }
        false
    }

    fn refresh(&mut self) {
        self.pending = None;
        self.refresh_at(Instant::now());
    }

    fn refresh_at(&mut self, now: Instant) {
        match capture(&self.worktree) {
            Ok(Observation::Ready((rows, content))) => {
                self.pending = None;
                self.rows = rows;
                self.selected = 0;
                self.scroll = 0;
                self.tree_state = ListState::default();
                self.status = "Read-only | manual refresh".into();
                self.set_content(content);
            }
            Ok(Observation::Busy) => self.waiting(Request::Refresh, now),
            Ok(Observation::Vacant) => self.missing(),
            Err(error) => self.failed(&error),
        }
    }

    fn load_selected(&mut self, now: Instant) {
        let Some(row) = self.rows.get(self.selected) else {
            return;
        };
        match read_selected(&self.worktree, &row.path) {
            Ok(Observation::Ready(content)) => {
                // A selection does not discard a pending whole-tree refresh.
                if !matches!(self.pending, Some((Request::Refresh, _))) {
                    self.pending = None;
                    self.status = "Read-only | manual refresh".into();
                }
                self.set_content(content);
            }
            Ok(Observation::Busy) => self.waiting(Request::Selected, now),
            Ok(Observation::Vacant) => self.missing(),
            Err(error) => self.failed(&error),
        }
    }

    fn waiting(&mut self, request: Request, now: Instant) {
        if self.pending.is_none() {
            self.pending = Some((request, now + RETRY_INTERVAL));
        }
        self.status = "WAITING for tree writer — previous display retained; retrying".into();
    }

    fn missing(&mut self) {
        self.pending = None;
        self.rows.clear();
        self.content.clear();
        self.selected = 0;
        self.scroll = 0;
        self.status = "Missing .grove — press r to retry".into();
    }

    fn failed(&mut self, error: &anyhow::Error) {
        // Acquisition already failed. An absent observation directory has no
        // lock to take and no old tree to retain; other I/O errors stay visible.
        if std::fs::metadata(&self.worktree)
            .is_err_and(|error| error.kind() == std::io::ErrorKind::NotFound)
        {
            self.missing();
            return;
        }
        self.pending = None;
        let state = if self.rows.is_empty() {
            "Error"
        } else {
            "STALE"
        };
        self.status = format!(
            "{state}: {} — press r to retry",
            safe_text(&error.to_string())
        );
    }

    /// Deliver the current time. Only a busy request is retried, at most once.
    pub fn tick(&mut self, now: Instant) {
        let Some((request, deadline)) = self.pending else {
            return;
        };
        if now < deadline {
            return;
        }
        self.pending = Some((request, now + RETRY_INTERVAL));
        match request {
            Request::Refresh => self.refresh_at(now),
            Request::Selected => self.load_selected(now),
        }
    }

    /// Time until the pending retry, or no deadline for an idle manual browser.
    pub fn retry_after(&self, now: Instant) -> Option<Duration> {
        self.pending
            .map(|(_, deadline)| deadline.saturating_duration_since(now))
    }

    fn set_content(&mut self, content: Result<Vec<u8>, String>) {
        let text = match content {
            Ok(bytes) => safe_text(&String::from_utf8_lossy(&bytes)),
            Err(error) => format!("File error: {} — press r to retry", safe_text(&error)),
        };
        self.content = text.lines().map(str::to_owned).collect();
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
            .min(self.content.len().saturating_sub(self.page_height));
    }

    /// Draw the same application into a real terminal or Ratatui `TestBackend`.
    pub fn render(&mut self, frame: &mut Frame) {
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
        let [tree_area, file_area] =
            Layout::horizontal([Constraint::Percentage(55), Constraint::Percentage(45)])
                .areas(body);
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
                .block(Block::bordered().title("Tree"))
                .highlight_symbol("> ")
                .highlight_style(Style::default().add_modifier(Modifier::REVERSED)),
            tree_area,
            &mut self.tree_state,
        );
        self.page_height = usize::from(file_area.height.saturating_sub(2)).max(1);
        self.clamp_scroll();
        let lines: Vec<Line<'_>> = self
            .content
            .iter()
            .skip(self.scroll)
            .take(self.page_height)
            .map(|line| Line::raw(line.as_str()))
            .collect();
        // Unwrapped plain text preserves source lines; slicing avoids u16 scroll limits.
        // https://docs.rs/ratatui/0.29.0/ratatui/widgets/struct.Paragraph.html
        frame.render_widget(
            Paragraph::new(lines).block(Block::bordered().title("File (plain text)")),
            file_area,
        );
        frame.render_widget(
            Paragraph::new(
                "↑/↓ j/k select | Enter fold | PgUp/PgDn file | r refresh | q/Ctrl-c quit",
            ),
            footer,
        );
    }
}
