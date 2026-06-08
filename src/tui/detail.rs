//! The per-grove **detail** surface (050/030): a ratatui widget grove draws from
//! [`RepoView`], not a proxy. It shows the focused grove's **task tree** (live
//! leaves + `done/`), its **brief chain** (the ancestor `BRIEF.md` spine to the
//! current pick, root→leaf), and its **inbox view** (pending count + the
//! observation filenames).
//!
//! ## A grove-drawn widget, not a dumb proxy
//!
//! Under the trellis zellij-fork detail was a `grove __dash-proxy` pane wired over
//! a controller socket (ADR-0016). Owning the draw loop dissolves all of that:
//! detail is a coexisting panel beside the harness pane, drawn each frame from the
//! same presentation-agnostic core ([`RepoView`]/[`GroveDetail`]) the nav already
//! projects — no socket, no proxy, no `RunEditor` frame.
//!
//! Like [`Nav`](crate::tui::nav) and the rest of `src/tui/`, [`Detail::render`] is
//! a **pure** snapshot → [`Buffer`] function (no daemon, no terminal), so the whole
//! surface is exercised by headless unit tests. It renders only the *structure* the
//! scan already loaded (paths, not bodies); opening observation/brief bodies is the
//! next leaf (040 triage).

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Widget};

use crate::repo_view::GroveDetail;

/// The detail surface's state: the grove it currently shows (the focused pane's
/// grove), the pre-rendered content lines built from that grove's
/// [`GroveDetail`] snapshot, and the vertical scroll offset.
#[derive(Debug, Clone, Default)]
pub struct Detail {
    /// The grove name shown in the title, or `None` for the bare-shell (no grove).
    grove: Option<String>,
    /// Content lines built at [`show`](Self::show) time from the snapshot, so
    /// [`render`](Self::render) is a trivial blit of the scrolled window.
    lines: Vec<Line<'static>>,
    /// First visible content line (the scroll offset).
    scroll: usize,
}

impl Detail {
    /// A fresh, empty detail (no grove shown yet).
    pub fn new() -> Self {
        Self::default()
    }

    /// Re-point the panel at `name`'s grove (the focused pane's grove), rebuilding
    /// the content lines from its [`GroveDetail`] snapshot. `name`/`detail` are
    /// `None` for the bare-shell pane (no grove → the empty state). Switching
    /// groves resets the scroll; refreshing the *same* grove preserves it.
    pub fn show(&mut self, name: Option<&str>, detail: Option<&GroveDetail>) {
        let next = name.map(str::to_owned);
        if next != self.grove {
            self.scroll = 0;
        }
        self.grove = next;
        self.lines = match detail {
            Some(d) => build_lines(d),
            None => Vec::new(),
        };
        // A refresh may shrink the content (a leaf retired, an observation
        // drained); keep the offset inside the new bounds so it never scrolls
        // into the void.
        self.scroll = self.scroll.min(self.lines.len().saturating_sub(1));
    }

    /// Scroll the content window down one line (clamped so at least the last line
    /// stays visible).
    pub fn scroll_down(&mut self) {
        self.scroll = (self.scroll + 1).min(self.lines.len().saturating_sub(1));
    }

    /// Scroll the content window up one line (saturating at the top).
    pub fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }

    /// Paint the detail panel into `buf` over `area`: a bordered block (accent
    /// border when `focused`) holding the scrolled content window. [`Clear`]s
    /// `area` first so the pane beside it does not bleed through.
    pub fn render(&self, area: Rect, buf: &mut Buffer, focused: bool) {
        use ratatui::style::{Color, Style};
        Clear.render(area, buf);
        let title = match &self.grove {
            Some(name) => format!(" detail: {name} "),
            None => " detail ".to_string(),
        };
        let mut block = Block::default().borders(Borders::ALL).title(title);
        if focused {
            block = block.border_style(Style::default().fg(Color::Cyan));
        }
        let inner = block.inner(area);
        block.render(area, buf);
        if inner.width == 0 || inner.height == 0 {
            return;
        }
        if self.grove.is_none() {
            Paragraph::new("no grove").render(inner, buf);
            return;
        }
        Paragraph::new(self.lines.clone())
            .scroll((self.scroll as u16, 0))
            .render(inner, buf);
    }
}

/// Build the panel's content lines from a grove's [`GroveDetail`] snapshot: the
/// three sections — brief chain, tasks, inbox — stacked top to bottom. Reads only
/// the snapshot *structure* (paths, not bodies), so it stays pure and headless.
fn build_lines(detail: &GroveDetail) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    push_brief_chain(&mut lines, detail);
    lines.push(plain_line(""));
    push_tasks(&mut lines, detail);
    lines.push(plain_line(""));
    push_inbox(&mut lines, detail);
    lines
}

/// The **brief chain** section: the ancestor spine root→leaf to the current pick
/// (the first live leaf, depth-first — `grove-llm brief-chain`/`pick` semantics).
/// The grove is the root; each node on the path descends with `›`; the pick leaf
/// is marked `(current)`. A grove with no live leaf reports it.
fn push_brief_chain(lines: &mut Vec<Line<'static>>, detail: &GroveDetail) {
    lines.push(header_line("brief chain"));
    let Some(tree) = &detail.task_tree else {
        lines.push(plain_line("  (no .grove/ yet)"));
        return;
    };
    lines.push(plain_line(format!("  {}", detail.name)));
    let Some(pick) = first_live_leaf(&tree.entries) else {
        lines.push(plain_line("  (no live leaf — grove done)"));
        return;
    };
    let path = path_to(&tree.entries, &pick.path).unwrap_or_default();
    let last = path.len().saturating_sub(1);
    for (i, entry) in path.iter().enumerate() {
        if i == last {
            lines.push(plain_line(format!("  \u{25b8} {}  (current)", entry.name)));
        } else {
            lines.push(plain_line(format!("  \u{203a} {}", entry.name)));
        }
    }
}

/// The first live leaf depth-first (entries are already in `pick` order and carry
/// `is_retired`), skipping retired material — the grove's current pick.
fn first_live_leaf(entries: &[crate::repo_view::TaskEntry]) -> Option<&crate::repo_view::TaskEntry> {
    use crate::repo_view::TaskKind;
    for entry in entries {
        if entry.is_retired {
            continue;
        }
        match &entry.kind {
            TaskKind::Leaf => return Some(entry),
            TaskKind::Node { children, .. } => {
                if let Some(found) = first_live_leaf(children) {
                    return Some(found);
                }
            }
        }
    }
    None
}

/// The entries on the path from the top level down to (and including) the entry
/// at `target`, or `None` when no such entry exists.
fn path_to<'a>(
    entries: &'a [crate::repo_view::TaskEntry],
    target: &std::path::Path,
) -> Option<Vec<&'a crate::repo_view::TaskEntry>> {
    use crate::repo_view::TaskKind;
    for entry in entries {
        if entry.path == target {
            return Some(vec![entry]);
        }
        if let TaskKind::Node { children, .. } = &entry.kind {
            if let Some(mut sub) = path_to(children, target) {
                sub.insert(0, entry);
                return Some(sub);
            }
        }
    }
    None
}

/// The **inbox** section: the pending count in the header, then one line per
/// pending observation (filename only — opening bodies is the 040 triage leaf).
fn push_inbox(lines: &mut Vec<Line<'static>>, detail: &GroveDetail) {
    lines.push(header_line(&format!("inbox ({})", detail.inbox.len())));
    if detail.inbox.is_empty() {
        lines.push(plain_line("  (empty)"));
        return;
    }
    for obs in &detail.inbox {
        let name = obs
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        lines.push(plain_line(format!("  {name}")));
    }
}

/// The **tasks** section: the task tree, indented, nodes suffixed `/`.
fn push_tasks(lines: &mut Vec<Line<'static>>, detail: &GroveDetail) {
    lines.push(header_line("tasks"));
    match &detail.task_tree {
        Some(tree) if !tree.entries.is_empty() => {
            for entry in &tree.entries {
                push_entry(lines, entry, 1);
            }
        }
        Some(_) => lines.push(plain_line("  (no tasks)")),
        None => lines.push(plain_line("  (no .grove/ yet)")),
    }
}

/// Append one task-tree entry (and its children, depth-first) as indented lines.
fn push_entry(lines: &mut Vec<Line<'static>>, entry: &crate::repo_view::TaskEntry, depth: usize) {
    use crate::repo_view::TaskKind;
    let indent = "  ".repeat(depth);
    let is_node = matches!(entry.kind, TaskKind::Node { .. });
    let suffix = if is_node { "/" } else { "" };
    let text = format!("{indent}{}{suffix}", entry.name);
    // Retired material (`done/` and everything under it) is dimmed so live work
    // stands out — the `is_retired` flag the scan already propagated.
    let line = if entry.is_retired {
        use ratatui::style::{Modifier, Style};
        Line::styled(text, Style::default().add_modifier(Modifier::DIM))
    } else {
        plain_line(text)
    };
    lines.push(line);
    if let TaskKind::Node { children, .. } = &entry.kind {
        for child in children {
            push_entry(lines, child, depth + 1);
        }
    }
}

/// A bold section header line.
fn header_line(text: &str) -> Line<'static> {
    use ratatui::style::{Modifier, Style};
    Line::styled(text.to_string(), Style::default().add_modifier(Modifier::BOLD))
}

/// A plain content line.
fn plain_line(text: impl Into<String>) -> Line<'static> {
    Line::from(text.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo_view::RepoView;
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

    /// Make a one-grove repo whose `.grove/` is populated by `build`, scan it, and
    /// return the scan (kept alive so `view.grove("g")` borrows stay valid).
    fn grove_view(build: impl FnOnce(&Path)) -> (TempDir, RepoView) {
        let tmp = TempDir::new().unwrap();
        let grove_dir = tmp.path().join(".grove-worktrees").join("g").join(".grove");
        fs::create_dir_all(&grove_dir).unwrap();
        build(&grove_dir);
        let view = RepoView::scan(tmp.path()).unwrap();
        (tmp, view)
    }

    /// Like [`grove_view`] but also seeds the grove's `.grove-meta` inbox with
    /// the named observation files, so [`GroveDetail::inbox`] is populated.
    fn grove_view_with_inbox(leaves: &[&str], obs: &[&str]) -> (TempDir, RepoView) {
        let tmp = TempDir::new().unwrap();
        let grove_dir = tmp.path().join(".grove-worktrees").join("g").join(".grove");
        fs::create_dir_all(&grove_dir).unwrap();
        for l in leaves {
            fs::write(grove_dir.join(l), format!("# {l}\n")).unwrap();
        }
        let inbox = tmp.path().join(".grove-meta").join("inboxes").join("g");
        fs::create_dir_all(&inbox).unwrap();
        for o in obs {
            fs::write(inbox.join(o), "an observation\n").unwrap();
        }
        let view = RepoView::scan(tmp.path()).unwrap();
        (tmp, view)
    }

    fn buffer_text(buf: &Buffer) -> String {
        let area = buf.area;
        let mut s = String::new();
        for y in 0..area.height {
            for x in 0..area.width {
                s.push_str(buf[(area.x + x, area.y + y)].symbol());
            }
            s.push('\n');
        }
        s
    }

    #[test]
    fn tasks_section_lists_live_leaves() {
        let (_tmp, view) = grove_view(|g| {
            fs::write(g.join("010-design.md"), "# 010-design\n").unwrap();
            fs::write(g.join("020-build.md"), "# 020-build\n").unwrap();
        });
        let mut detail = Detail::new();
        detail.show(Some("g"), view.grove("g"));
        let area = Rect::new(0, 0, 44, 16);
        let mut buf = Buffer::empty(area);
        detail.render(area, &mut buf, false);
        let text = buffer_text(&buf);
        assert!(text.contains("010-design.md"), "got:\n{text}");
        assert!(text.contains("020-build.md"), "got:\n{text}");
    }

    #[test]
    fn retired_leaves_appear_dimmed_under_done() {
        use ratatui::style::Modifier;
        let (_tmp, view) = grove_view(|g| {
            fs::write(g.join("010-live.md"), "# 010-live\n").unwrap();
            let done = g.join("done");
            fs::create_dir_all(&done).unwrap();
            fs::write(done.join("005-old.md"), "# 005-old\n").unwrap();
        });
        let mut detail = Detail::new();
        detail.show(Some("g"), view.grove("g"));
        let area = Rect::new(0, 0, 44, 16);
        let mut buf = Buffer::empty(area);
        detail.render(area, &mut buf, false);
        let text = buffer_text(&buf);
        assert!(text.contains("done/"), "got:\n{text}");
        assert!(text.contains("005-old.md"), "got:\n{text}");

        // The retired leaf row is dimmed so it recedes from live work.
        let old_row = (0..area.height)
            .find(|&y| {
                (0..area.width)
                    .map(|x| buf[(x, y)].symbol().to_string())
                    .collect::<String>()
                    .contains("005-old")
            })
            .expect("005-old row present");
        let dimmed = (0..area.width)
            .any(|x| buf[(x, old_row)].style().add_modifier.contains(Modifier::DIM));
        assert!(dimmed, "retired leaf row should be dimmed");
    }

    #[test]
    fn inbox_section_shows_pending_count_and_filenames() {
        let (_tmp, view) = grove_view_with_inbox(
            &["010-x.md"],
            &["2026-06-08-120000-first.md", "2026-06-08-130000-second.md"],
        );
        let mut detail = Detail::new();
        detail.show(Some("g"), view.grove("g"));
        let area = Rect::new(0, 0, 50, 20);
        let mut buf = Buffer::empty(area);
        detail.render(area, &mut buf, false);
        let text = buffer_text(&buf);
        assert!(text.contains("inbox (2)"), "got:\n{text}");
        assert!(text.contains("first.md"), "got:\n{text}");
        assert!(text.contains("second.md"), "got:\n{text}");
    }

    #[test]
    fn empty_inbox_shows_a_hint_not_a_count() {
        let (_tmp, view) = grove_view_with_inbox(&["010-x.md"], &[]);
        let mut detail = Detail::new();
        detail.show(Some("g"), view.grove("g"));
        let area = Rect::new(0, 0, 50, 20);
        let mut buf = Buffer::empty(area);
        detail.render(area, &mut buf, false);
        let text = buffer_text(&buf);
        assert!(text.contains("inbox (0)"), "got:\n{text}");
        assert!(text.contains("empty"), "got:\n{text}");
    }

    #[test]
    fn brief_chain_shows_the_spine_to_the_current_pick() {
        // root brief + a node with its own brief + a leaf inside it. The pick is
        // the first live leaf depth-first (030-detail.md), and the chain is the
        // ancestor spine root→leaf.
        let (_tmp, view) = grove_view(|g| {
            fs::write(g.join("BRIEF.md"), "# g — brief\n").unwrap();
            let node = g.join("050-rebuild");
            fs::create_dir_all(&node).unwrap();
            fs::write(node.join("BRIEF.md"), "# 050-rebuild — brief\n").unwrap();
            fs::write(node.join("030-detail.md"), "# 030-detail\n").unwrap();
        });
        let mut detail = Detail::new();
        detail.show(Some("g"), view.grove("g"));
        let area = Rect::new(0, 0, 50, 24);
        let mut buf = Buffer::empty(area);
        detail.render(area, &mut buf, false);
        let text = buffer_text(&buf);
        assert!(text.contains("brief chain"), "got:\n{text}");
        // The pick is marked "(current)" — a marker unique to the chain section.
        assert!(text.contains("030-detail.md  (current)"), "got:\n{text}");
        // The intervening node sits on the spine between root and the pick.
        assert!(text.contains("050-rebuild"), "got:\n{text}");
    }

    #[test]
    fn brief_chain_reports_a_done_grove_has_no_live_leaf() {
        let (_tmp, view) = grove_view(|g| {
            let done = g.join("done");
            fs::create_dir_all(&done).unwrap();
            fs::write(done.join("010-old.md"), "# 010-old\n").unwrap();
        });
        let mut detail = Detail::new();
        detail.show(Some("g"), view.grove("g"));
        let area = Rect::new(0, 0, 50, 20);
        let mut buf = Buffer::empty(area);
        detail.render(area, &mut buf, false);
        assert!(
            buffer_text(&buf).contains("no live leaf"),
            "got:\n{}",
            buffer_text(&buf)
        );
    }

    /// The first inner row's text (row 1, inside the top border).
    fn first_inner_row(buf: &Buffer) -> String {
        let area = buf.area;
        (0..area.width)
            .map(|x| buf[(area.x + x, area.y + 1)].symbol().to_string())
            .collect()
    }

    #[test]
    fn title_names_the_focused_grove() {
        let (_tmp, view) = grove_view(|g| {
            fs::write(g.join("010-x.md"), "# 010-x\n").unwrap();
        });
        let mut detail = Detail::new();
        detail.show(Some("g"), view.grove("g"));
        let area = Rect::new(0, 0, 40, 12);
        let mut buf = Buffer::empty(area);
        detail.render(area, &mut buf, false);
        assert!(buffer_text(&buf).contains("detail: g"), "got:\n{}", buffer_text(&buf));
    }

    #[test]
    fn scrolling_advances_the_visible_window() {
        let (_tmp, view) = grove_view(|g| {
            for i in 1..=20 {
                fs::write(g.join(format!("{:03}-leaf.md", i * 10)), "# x\n").unwrap();
            }
        });
        let mut detail = Detail::new();
        detail.show(Some("g"), view.grove("g"));
        // A short panel so content overflows and scrolling is observable.
        let area = Rect::new(0, 0, 40, 8);
        let mut buf = Buffer::empty(area);
        detail.render(area, &mut buf, false);
        assert!(
            first_inner_row(&buf).contains("brief chain"),
            "top starts at the brief-chain header: {}",
            first_inner_row(&buf)
        );

        for _ in 0..4 {
            detail.scroll_down();
        }
        let mut buf2 = Buffer::empty(area);
        detail.render(area, &mut buf2, false);
        assert!(
            !first_inner_row(&buf2).contains("brief chain"),
            "after scrolling the header has scrolled off: {}",
            first_inner_row(&buf2)
        );

        // Scrolling back up returns to the top.
        for _ in 0..10 {
            detail.scroll_up();
        }
        let mut buf3 = Buffer::empty(area);
        detail.render(area, &mut buf3, false);
        assert!(
            first_inner_row(&buf3).contains("brief chain"),
            "scroll_up saturates back at the top: {}",
            first_inner_row(&buf3)
        );
    }

    #[test]
    fn refreshing_the_same_grove_preserves_scroll_but_clamps_to_new_bounds() {
        let (_tmp, big) = grove_view(|g| {
            for i in 1..=20 {
                fs::write(g.join(format!("{:03}-leaf.md", i * 10)), "# x\n").unwrap();
            }
        });
        let mut detail = Detail::new();
        detail.show(Some("g"), big.grove("g"));
        for _ in 0..15 {
            detail.scroll_down();
        }
        // Refresh the *same* grove with far fewer lines: scroll clamps into range,
        // never past the last line.
        let (_tmp2, small) = grove_view(|g| {
            fs::write(g.join("010-only.md"), "# x\n").unwrap();
        });
        detail.show(Some("g"), small.grove("g"));
        let area = Rect::new(0, 0, 40, 8);
        let mut buf = Buffer::empty(area);
        detail.render(area, &mut buf, false);
        // The offset clamped into the smaller content, so the top visible row is
        // real content — never blank space scrolled past the end.
        assert!(
            !first_inner_row(&buf).trim().is_empty(),
            "clamped scroll keeps content visible, not blank:\n{}",
            buffer_text(&buf)
        );
    }

    #[test]
    fn switching_groves_resets_scroll() {
        let (_tmp, view) = grove_view(|g| {
            for i in 1..=20 {
                fs::write(g.join(format!("{:03}-leaf.md", i * 10)), "# x\n").unwrap();
            }
        });
        let mut detail = Detail::new();
        detail.show(Some("g"), view.grove("g"));
        for _ in 0..5 {
            detail.scroll_down();
        }
        // A different grove name resets the offset to the top (brief-chain header),
        // even though the underlying data here happens to be the same shape.
        detail.show(Some("g2"), view.grove("g"));
        let area = Rect::new(0, 0, 40, 8);
        let mut buf = Buffer::empty(area);
        detail.render(area, &mut buf, false);
        assert!(
            first_inner_row(&buf).contains("brief chain"),
            "a freshly-shown grove starts at the top: {}",
            first_inner_row(&buf)
        );
    }

    #[test]
    fn focus_accents_the_border() {
        use ratatui::style::Color;
        let (_tmp, view) = grove_view(|g| {
            fs::write(g.join("010-x.md"), "# 010-x\n").unwrap();
        });
        let mut detail = Detail::new();
        detail.show(Some("g"), view.grove("g"));
        let area = Rect::new(0, 0, 40, 10);

        let mut focused = Buffer::empty(area);
        detail.render(area, &mut focused, true);
        assert_eq!(
            focused[(0, 0)].style().fg,
            Some(Color::Cyan),
            "focused border is accented"
        );

        let mut blurred = Buffer::empty(area);
        detail.render(area, &mut blurred, false);
        assert_ne!(
            blurred[(0, 0)].style().fg,
            Some(Color::Cyan),
            "unfocused border is not accented"
        );
    }

    #[test]
    fn no_grove_shows_an_empty_state() {
        let detail = Detail::new();
        let area = Rect::new(0, 0, 30, 8);
        let mut buf = Buffer::empty(area);
        detail.render(area, &mut buf, false);
        assert!(
            buffer_text(&buf).contains("no grove"),
            "got:\n{}",
            buffer_text(&buf)
        );
    }
}
