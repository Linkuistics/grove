//! The minimal **Nav** surface (030-nav): a flat, selectable list of groves and
//! the data needed to open/focus each one's harness pane.
//!
//! This is what makes `grove tui` usable for more than one grove — the leader
//! ([`crate::tui::config::Leader`], `Alt-g`) flips [`Focus`](crate::tui::focus::Focus)
//! to `Nav`, you pick a grove, and its harness pane opens (or the already-open
//! one is focused). The richness — grouped/collapsible repo headers, fuzzy
//! ranked filtering, inbox/lifecycle toggles — is **050**; here it is a flat
//! list, deliberately.
//!
//! ## Below the seam, projected up
//!
//! The list is built from the presentation-agnostic core
//! ([`MultiRepoView`]/[`RepoView`]) — no new core logic, pure presentation over
//! data that already exists. Like [`render_pane`](crate::tui::pane::render_pane),
//! [`Nav::render`] is a **pure** snapshot → [`Buffer`] function, so the whole
//! surface is exercised by a headless unit test with no daemon and no terminal.

use std::path::PathBuf;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Widget};

use crate::multi_repo_view::MultiRepoView;
use crate::repo_view::Lifecycle;

/// One row in the nav: a live grove the user can open/focus. Carries both the
/// **identity** the app keys panes on (`repo_root` + `name` resolve the worktree
/// and the `grove do <name>` argv) and the **display label** (disambiguated to
/// `<repo>/<grove>` only when the fleet spans more than one repo).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavItem {
    /// The owning repo's root — paired with `name` to resolve the worktree cwd.
    pub repo_root: PathBuf,
    /// The grove name: the `grove-name → PaneId` map key (E3) and the argument
    /// to `grove do <name>`.
    pub name: String,
    /// What the row shows: `<grove>` at a single-repo fleet, `<repo>/<grove>`
    /// when N>1 so same-named groves across repos stay distinguishable.
    pub label: String,
}

/// The nav's state: the selectable items plus the current selection. Rebuilt
/// from a fresh fleet scan on every fs-watch tick (groves appearing/retiring),
/// preserving the selection by name where it survives.
#[derive(Debug, Clone, Default)]
pub struct Nav {
    items: Vec<NavItem>,
    selected: usize,
}

impl Nav {
    /// Build the nav from a fleet snapshot: every **live** grove (the ones with
    /// a harness to open/focus), in fleet order. Seeds are omitted at this
    /// minimal scope — the lifecycle-aware listing is 050.
    pub fn from_fleet(fleet: &MultiRepoView) -> Self {
        let mut nav = Nav::default();
        nav.set_items(fleet);
        nav
    }

    /// Replace the items from a fresh fleet scan, clamping the selection into
    /// range. Use [`rebuild`](Self::rebuild) to also preserve the selection by
    /// name across the swap.
    fn set_items(&mut self, fleet: &MultiRepoView) {
        let multi = fleet.repos().len() > 1;
        let mut items = Vec::new();
        for repo in fleet.repos() {
            let repo_name = repo
                .repo_root
                .file_name()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_else(|| repo.repo_root.to_string_lossy().into_owned());
            for grove in repo.groves() {
                if grove.lifecycle != Lifecycle::Live {
                    continue;
                }
                let label = if multi {
                    format!("{repo_name}/{}", grove.name)
                } else {
                    grove.name.clone()
                };
                items.push(NavItem {
                    repo_root: repo.repo_root.clone(),
                    name: grove.name.clone(),
                    label,
                });
            }
        }
        self.items = items;
        self.clamp_selection();
    }

    /// Rebuild from a fresh fleet scan, keeping the cursor on the same grove
    /// when it still exists (so a background refresh doesn't yank the selection
    /// out from under the user), otherwise clamping into the new range.
    pub fn rebuild(&mut self, fleet: &MultiRepoView) {
        let prior = self.selected().map(|i| (i.repo_root.clone(), i.name.clone()));
        self.set_items(fleet);
        if let Some((root, name)) = prior {
            if let Some(idx) = self
                .items
                .iter()
                .position(|i| i.repo_root == root && i.name == name)
            {
                self.selected = idx;
            }
        }
    }

    /// Move the selection towards an item with the given grove name, if present
    /// (used to land the cursor on the initially-focused grove at launch).
    pub fn select(&mut self, name: &str) {
        if let Some(idx) = self.items.iter().position(|i| i.name == name) {
            self.selected = idx;
        }
    }

    /// The currently selected item, or `None` when the list is empty.
    pub fn selected(&self) -> Option<&NavItem> {
        self.items.get(self.selected)
    }

    /// Move the cursor up one row (saturating at the top).
    pub fn select_up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    /// Move the cursor down one row (saturating at the bottom).
    pub fn select_down(&mut self) {
        if self.selected + 1 < self.items.len() {
            self.selected += 1;
        }
    }

    fn clamp_selection(&mut self) {
        if self.items.is_empty() {
            self.selected = 0;
        } else if self.selected >= self.items.len() {
            self.selected = self.items.len() - 1;
        }
    }

    /// Paint the nav into `buf` over `area`: a bordered list with the selected
    /// row reversed. Pure — no daemon, no terminal — which is the headless
    /// testability the rmux migration buys (see the tests below). [`Clear`]s
    /// `area` first so the live pane underneath does not bleed through.
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" groves — ↑/↓ (j/k) move · ⏎ open · Esc harness · q quit ");
        let inner = block.inner(area);
        block.render(area, buf);
        if inner.width == 0 || inner.height == 0 {
            return;
        }
        if self.items.is_empty() {
            Paragraph::new("no live groves").render(inner, buf);
            return;
        }
        for (i, item) in self.items.iter().enumerate() {
            let row = i as u16;
            if row >= inner.height {
                break; // off-screen; scrolling the list is 050's richer nav
            }
            let rect = Rect::new(inner.x, inner.y + row, inner.width, 1);
            let (marker, style) = if i == self.selected {
                ("\u{203a} ", Style::default().add_modifier(Modifier::REVERSED))
            } else {
                ("  ", Style::default())
            };
            Paragraph::new(Line::from(format!("{marker}{}", item.label)))
                .style(style)
                .render(rect, buf);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo_view::RepoView;
    use std::fs;
    use tempfile::TempDir;

    /// Make `parent/<repo>/` with one live grove worktree per name.
    fn make_repo(parent: &std::path::Path, repo: &str, groves: &[&str]) -> PathBuf {
        let root = parent.join(repo);
        for g in groves {
            let task_root = root.join(".grove-worktrees").join(g).join(".grove");
            fs::create_dir_all(&task_root).unwrap();
            fs::write(task_root.join("010-x.md"), "# 010-x\n").unwrap();
        }
        fs::create_dir_all(root.join(".grove-worktrees")).unwrap();
        root
    }

    fn fleet_of(repos: &[PathBuf]) -> MultiRepoView {
        MultiRepoView::from_repos(repos.iter().map(|r| RepoView::scan(r).unwrap()).collect())
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
    fn single_repo_labels_are_bare_grove_names() {
        let tmp = TempDir::new().unwrap();
        let r = make_repo(tmp.path(), "solo", &["alpha", "beta"]);
        let nav = Nav::from_fleet(&fleet_of(&[r]));
        // Items keep fleet/name order, undecorated at N=1.
        assert_eq!(nav.selected().unwrap().label, "alpha");
    }

    #[test]
    fn multi_repo_labels_are_repo_qualified() {
        let tmp = TempDir::new().unwrap();
        let r1 = make_repo(tmp.path(), "one", &["alpha"]);
        let r2 = make_repo(tmp.path(), "two", &["alpha"]);
        let nav = Nav::from_fleet(&fleet_of(&[r1, r2]));
        // Both groves are named "alpha"; the repo prefix disambiguates them.
        nav.selected().unwrap();
        let buf_text = {
            let area = Rect::new(0, 0, 30, 6);
            let mut buf = Buffer::empty(area);
            nav.render(area, &mut buf);
            buffer_text(&buf)
        };
        assert!(buf_text.contains("one/alpha"), "got:\n{buf_text}");
        assert!(buf_text.contains("two/alpha"), "got:\n{buf_text}");
    }

    #[test]
    fn navigation_saturates_at_both_ends() {
        let tmp = TempDir::new().unwrap();
        let r = make_repo(tmp.path(), "solo", &["alpha", "beta", "gamma"]);
        let mut nav = Nav::from_fleet(&fleet_of(&[r]));
        assert_eq!(nav.selected().unwrap().name, "alpha");
        nav.select_up(); // already at top
        assert_eq!(nav.selected().unwrap().name, "alpha");
        nav.select_down();
        nav.select_down();
        assert_eq!(nav.selected().unwrap().name, "gamma");
        nav.select_down(); // already at bottom
        assert_eq!(nav.selected().unwrap().name, "gamma");
    }

    #[test]
    fn render_marks_the_selected_row() {
        let tmp = TempDir::new().unwrap();
        let r = make_repo(tmp.path(), "solo", &["alpha", "beta"]);
        let mut nav = Nav::from_fleet(&fleet_of(&[r]));
        nav.select_down(); // select "beta"
        let area = Rect::new(0, 0, 20, 5);
        let mut buf = Buffer::empty(area);
        nav.render(area, &mut buf);
        // The bordered block top-left corner is drawn…
        assert_eq!(buf[(0, 0)].symbol(), "\u{250c}");
        // …and the selected row carries the reversed modifier on the marker.
        let text = buffer_text(&buf);
        assert!(text.contains("\u{203a} beta"), "got:\n{text}");
        // Find beta's row and confirm it is reversed (selection styling).
        let beta_row = (1..area.height)
            .find(|&y| {
                (0..area.width).any(|x| buf[(x, y)].symbol() == "b")
                    && (0..area.width)
                        .map(|x| buf[(x, y)].symbol().to_string())
                        .collect::<String>()
                        .contains("beta")
            })
            .expect("beta row present");
        let inner_x = 1; // inside the left border
        assert!(buf[(inner_x, beta_row)]
            .style()
            .add_modifier
            .contains(Modifier::REVERSED));
    }

    #[test]
    fn rebuild_keeps_selection_on_the_same_grove() {
        let tmp = TempDir::new().unwrap();
        let r = make_repo(tmp.path(), "solo", &["alpha", "beta"]);
        let mut nav = Nav::from_fleet(&fleet_of(&[r.clone()]));
        nav.select_down(); // beta
        assert_eq!(nav.selected().unwrap().name, "beta");

        // A new grove appears earlier in name order; rebuild from a fresh scan.
        let task_root = r.join(".grove-worktrees").join("aardvark").join(".grove");
        fs::create_dir_all(&task_root).unwrap();
        fs::write(task_root.join("010-x.md"), "# 010-x\n").unwrap();
        nav.rebuild(&fleet_of(&[r]));

        // Cursor stays on beta even though the indices shifted.
        assert_eq!(nav.selected().unwrap().name, "beta");
    }

    #[test]
    fn empty_fleet_renders_an_empty_state() {
        let tmp = TempDir::new().unwrap();
        let r = make_repo(tmp.path(), "empty", &[]);
        let nav = Nav::from_fleet(&fleet_of(&[r]));
        assert!(nav.selected().is_none());
        let area = Rect::new(0, 0, 20, 4);
        let mut buf = Buffer::empty(area);
        nav.render(area, &mut buf);
        assert!(buffer_text(&buf).contains("no live groves"));
    }
}
