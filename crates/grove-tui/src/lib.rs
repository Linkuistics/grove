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
    horizontal: usize,
    content_width: usize,
    page_width: usize,
    file_focus: bool,
    help: bool,
    small: bool,
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
            horizontal: 0,
            content_width: 0,
            page_width: 1,
            file_focus: false,
            help: false,
            small: false,
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
            Action::Focus => self.file_focus = !self.file_focus,
            Action::Up | Action::Down if self.file_focus => {
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
                if self.file_focus {
                    return false;
                }
                if let Some(row) = self.rows.get_mut(self.selected) {
                    if row.branch {
                        row.expanded = !row.expanded;
                    }
                }
            }
            Action::Left | Action::Right if self.file_focus => {
                self.horizontal = if matches!(action, Action::Left) {
                    self.horizontal.saturating_sub(1)
                } else {
                    self.horizontal.saturating_add(1)
                };
                self.clamp_scroll();
            }
            Action::Left | Action::Right => self.tree_horizontal(matches!(action, Action::Right)),
            Action::Home | Action::End => {
                if self.file_focus {
                    self.scroll = if matches!(action, Action::Home) {
                        0
                    } else {
                        self.content.len()
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
            Action::PageUp => self.scroll = self.scroll.saturating_sub(self.page_height),
            Action::PageDown => {
                self.scroll = self.scroll.saturating_add(self.page_height);
                self.clamp_scroll();
            }
        }
        false
    }

    fn select(&mut self, target: usize) {
        if self.selected != target {
            self.selected = target;
            self.scroll = 0;
            self.horizontal = 0;
            self.load_selected(Instant::now());
        }
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
                self.horizontal = 0;
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
        self.horizontal = 0;
        self.content_width = 0;
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
        self.content_width = self
            .content
            .iter()
            .map(|line| Line::raw(line.as_str()).width())
            .max()
            .unwrap_or(0);
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
                    "Tab: switch pane | r: refresh | q/Ctrl-c: quit\n\
Tree: Up/Down j/k select | Home/End first/last\n\
Right/l expand/enter | Left/h collapse/parent\n\
Enter/Space: toggle branch\n\
File: Up/Down j/k line | Left/Right h/l column\n\
PageUp/PageDown Ctrl-u/Ctrl-d: page | Home/End: top/end\n\
Escape: close help | ?: toggle help",
                )
                .block(Block::bordered().title("Key help")),
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
                .block(Block::bordered().title(if self.file_focus {
                    "Tree"
                } else {
                    "Tree [focus]"
                }))
                .highlight_symbol("> ")
                .highlight_style(Style::default().add_modifier(Modifier::REVERSED)),
            tree_area,
            &mut self.tree_state,
        );
        self.page_height = usize::from(file_area.height.saturating_sub(2)).max(1);
        self.page_width = usize::from(file_area.width.saturating_sub(2)).max(1);
        self.clamp_scroll();
        let lines: Vec<Line<'_>> = self
            .content
            .iter()
            .skip(self.scroll)
            .take(self.page_height)
            .map(|line| Line::raw(clip_columns(line, self.horizontal, self.page_width)))
            .collect();
        // Unwrapped plain text preserves source lines; slicing avoids u16 scroll limits.
        // https://docs.rs/ratatui/0.29.0/ratatui/widgets/struct.Paragraph.html
        frame.render_widget(
            Paragraph::new(lines).block(Block::bordered().title(if self.file_focus {
                "File [focus] (plain text)"
            } else {
                "File (plain text)"
            })),
            file_area,
        );
        frame.render_widget(
            Paragraph::new("Tab focus | ? help | r refresh | q/Ctrl-c quit"),
            footer,
        );
    }
}

/// Clip terminal columns without splitting graphemes or narrowing offsets to u16.
fn clip_columns(text: &str, offset: usize, width: usize) -> String {
    let line = Line::raw(text);
    let mut column: usize = 0;
    let mut output = String::new();
    // Ratatui's graphemes preserve combining sequences; Line measures display width.
    // https://docs.rs/ratatui/0.29.0/ratatui/text/struct.Line.html#method.styled_graphemes
    for grapheme in line.styled_graphemes(Style::default()) {
        let start = column;
        column += Line::raw(grapheme.symbol).width();
        if column <= offset {
            continue;
        }
        if start >= offset.saturating_add(width) {
            break;
        }
        if start < offset || column > offset.saturating_add(width) {
            let cells = column.min(offset.saturating_add(width)) - start.max(offset);
            output.extend(std::iter::repeat_n(' ', cells));
        } else {
            output.push_str(grapheme.symbol);
        }
    }
    output
}
