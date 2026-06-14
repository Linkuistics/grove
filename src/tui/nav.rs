//! The **Nav** surface: the fleet's grove list in its **grouped idle shape**
//! (rmux-tui-polish 030) — collapsible repo section headers over their groves,
//! seeds listed beside live groves, and a scrolling cursor.
//!
//! This is what makes `grove tui` usable for more than one grove — the leader
//! ([`crate::tui::config::Leader`], `Alt-g`) flips [`Focus`](crate::tui::focus::Focus)
//! to `Nav`, you pick a grove, and its harness pane opens (or the already-open
//! one is focused). Headers are selectable rows: `h` collapses / `l` expands
//! the repo under the cursor, Enter on a header toggles its fold. Fold state is
//! ephemeral per session (constraint 1). The lone header auto-hides at N=1, so
//! a single-repo fleet looks exactly like the old flat list. The flat *ranked*
//! shape (fuzzy filter, sort/lifecycle toggles) is 040/050.
//!
//! ## Below the seam, projected up
//!
//! The list is built from the presentation-agnostic core
//! ([`MultiRepoView`]/[`RepoView`]) — no new core logic, pure presentation over
//! data that already exists. Like [`render_pane`](crate::tui::pane::render_pane),
//! [`Nav::render`] is a **pure** snapshot → [`Buffer`] function (the scroll
//! offset is *computed* from the selection and the rect, never stored), so the
//! whole surface is exercised by a headless unit test with no daemon and no
//! terminal.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Widget};

use crate::multi_repo_view::MultiRepoView;
use crate::repo_view::Lifecycle;

/// A grove row's data: a grove the user can act on. Carries both the
/// **identity** the app keys panes on (`repo_root` + `name` resolve the worktree
/// and the `grove do <name>` argv) and the **display label**. In the grouped
/// shape the label is the bare grove name (the header carries the repo); in the
/// flat picker shape it is `<repo>/<grove>` when the fleet spans more than one
/// repo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavItem {
    /// The owning repo's root — paired with `name` to resolve the worktree cwd.
    pub repo_root: PathBuf,
    /// The grove name: the `grove-name → PaneId` map key (E3) and the argument
    /// to `grove do <name>`.
    pub name: String,
    /// What the row shows (see the struct doc for the two shapes).
    pub label: String,
    /// Live (worktree exists) or seed (inbox only). Enter on a live grove opens
    /// its harness; Enter on a seed opens the confirm-and-start prompt (070).
    pub lifecycle: Lifecycle,
}

/// One selectable row: a repo section header or a grove. The sum type is what
/// lets the cursor sit on a header (Q6: headers are selectable, Enter toggles
/// their fold) without the app guessing from indices.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NavRow {
    /// A repo section header. `folded` mirrors the nav's fold set at projection
    /// time so rendering stays a pure function of the row list.
    Header {
        repo_root: PathBuf,
        label: String,
        folded: bool,
    },
    /// A grove (live or seed) under its repo's section.
    Grove(NavItem),
}

impl NavRow {
    fn repo_root(&self) -> &Path {
        match self {
            NavRow::Header { repo_root, .. } => repo_root,
            NavRow::Grove(item) => &item.repo_root,
        }
    }
}

/// One repo's slice of the fleet, as scanned: the projection source the visible
/// rows are rebuilt from on every fold toggle (no re-scan needed).
#[derive(Debug, Clone)]
struct RepoEntry {
    repo_root: PathBuf,
    label: String,
    groves: Vec<NavItem>,
}

/// The nav's state: per-repo entries (the scan), the fold set (by repo root,
/// so it survives rebuilds), the visible rows (the projection), and the cursor.
/// Rebuilt from a fresh fleet scan on every fs-watch tick (groves appearing/
/// retiring), preserving the selection by identity where it survives.
#[derive(Debug, Clone, Default)]
pub struct Nav {
    /// Grouped shape (headers + seeds) for the real nav; flat live-only shape
    /// for the move-target picker.
    grouped: bool,
    entries: Vec<RepoEntry>,
    /// Repo roots whose sections are collapsed. Ephemeral per session; keyed
    /// by root (not row index) so a rebuild keeps folds, and a root that
    /// disappears just stops matching (no pruning needed).
    folded: BTreeSet<PathBuf>,
    rows: Vec<NavRow>,
    selected: usize,
}

impl Nav {
    /// Build the nav in its **grouped idle shape** from a fleet snapshot:
    /// repo section headers (auto-hidden when only one repo has rows) over
    /// every grove — live *and* seed.
    pub fn from_fleet(fleet: &MultiRepoView) -> Self {
        let mut nav = Nav {
            grouped: true,
            ..Nav::default()
        };
        nav.set_entries(fleet);
        nav
    }

    /// Build the **flat live-only** shape: no headers, no seeds, labels
    /// repo-qualified at N>1. Used by the move-target picker (040 grooming),
    /// which needs a plain list of harness-capable groves.
    pub fn flat_from_fleet(fleet: &MultiRepoView) -> Self {
        let mut nav = Nav::default();
        nav.set_entries(fleet);
        nav
    }

    /// Replace the entries from a fresh fleet scan and re-project, clamping the
    /// selection into range. Use [`rebuild`](Self::rebuild) to also preserve
    /// the selection by identity across the swap.
    fn set_entries(&mut self, fleet: &MultiRepoView) {
        let multi = fleet.repos().len() > 1;
        let mut entries = Vec::new();
        for repo in fleet.repos() {
            let repo_name = repo
                .repo_root
                .file_name()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_else(|| repo.repo_root.to_string_lossy().into_owned());
            let mut groves = Vec::new();
            for grove in repo.groves() {
                if !self.grouped && grove.lifecycle != Lifecycle::Live {
                    continue; // the picker lists only groves with a worktree
                }
                let label = if !self.grouped && multi {
                    format!("{repo_name}/{}", grove.name)
                } else {
                    grove.name.clone()
                };
                groves.push(NavItem {
                    repo_root: repo.repo_root.clone(),
                    name: grove.name.clone(),
                    label,
                    lifecycle: grove.lifecycle,
                });
            }
            if groves.is_empty() {
                continue; // no header for an empty section
            }
            entries.push(RepoEntry {
                repo_root: repo.repo_root.clone(),
                label: repo_name,
                groves,
            });
        }
        self.entries = entries;
        self.project();
        self.clamp_selection();
    }

    /// Whether repo section headers are part of the projection: grouped shape
    /// with more than one repo carrying rows (the lone header auto-hides).
    fn headers_shown(&self) -> bool {
        self.grouped && self.entries.len() > 1
    }

    /// Rebuild the visible rows from the entries + fold set. Cheap (no scan),
    /// so fold toggles just re-project.
    fn project(&mut self) {
        let headers = self.headers_shown();
        let mut rows = Vec::new();
        for entry in &self.entries {
            let folded = self.folded.contains(&entry.repo_root);
            if headers {
                rows.push(NavRow::Header {
                    repo_root: entry.repo_root.clone(),
                    label: entry.label.clone(),
                    folded,
                });
                if folded {
                    continue; // collapsed: the section's groves are hidden
                }
            }
            rows.extend(entry.groves.iter().cloned().map(NavRow::Grove));
        }
        self.rows = rows;
    }

    /// Rebuild from a fresh fleet scan, keeping the cursor on the same grove
    /// (by repo root + name) or the same header (by repo root) when it still
    /// exists — so a background refresh doesn't yank the selection out from
    /// under the user — otherwise clamping into the new range. Fold state is
    /// keyed by repo root and survives untouched.
    pub fn rebuild(&mut self, fleet: &MultiRepoView) {
        let prior = self.rows.get(self.selected).cloned();
        self.set_entries(fleet);
        let found = prior.and_then(|row| match row {
            NavRow::Header { repo_root, .. } => self.header_index(&repo_root),
            NavRow::Grove(item) => self.grove_index(&item.repo_root, &item.name),
        });
        if let Some(idx) = found {
            self.selected = idx;
        }
    }

    fn header_index(&self, repo_root: &Path) -> Option<usize> {
        self.rows
            .iter()
            .position(|r| matches!(r, NavRow::Header { repo_root: rr, .. } if rr == repo_root))
    }

    fn grove_index(&self, repo_root: &Path, name: &str) -> Option<usize> {
        self.rows.iter().position(
            |r| matches!(r, NavRow::Grove(i) if i.repo_root == repo_root && i.name == name),
        )
    }

    /// Drop the grove identified by `repo_root` + `name`, clamping the
    /// selection. Used to build the move-target picker (040 grooming) from the
    /// fleet list with the *source* grove excluded — you do not move an
    /// observation to the grove it already sits in.
    pub fn remove(&mut self, repo_root: &Path, name: &str) {
        for entry in &mut self.entries {
            entry
                .groves
                .retain(|i| !(i.repo_root == repo_root && i.name == name));
        }
        self.entries.retain(|e| !e.groves.is_empty());
        self.project();
        self.clamp_selection();
    }

    /// Move the selection towards a grove row with the given name, if present
    /// (used to land the cursor on the initially-focused grove at launch).
    pub fn select(&mut self, name: &str) {
        if let Some(idx) = self
            .rows
            .iter()
            .position(|r| matches!(r, NavRow::Grove(i) if i.name == name))
        {
            self.selected = idx;
        }
    }

    /// The row under the cursor, or `None` when the list is empty.
    pub fn selected_row(&self) -> Option<&NavRow> {
        self.rows.get(self.selected)
    }

    /// The grove under the cursor — `None` when the list is empty *or* the
    /// cursor sits on a header (use [`selected_row`](Self::selected_row) to
    /// distinguish).
    pub fn selected(&self) -> Option<&NavItem> {
        match self.rows.get(self.selected) {
            Some(NavRow::Grove(item)) => Some(item),
            _ => None,
        }
    }

    /// Move the cursor up one row (saturating at the top).
    pub fn select_up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    /// Move the cursor down one row (saturating at the bottom).
    pub fn select_down(&mut self) {
        if self.selected + 1 < self.rows.len() {
            self.selected += 1;
        }
    }

    /// Collapse the repo under the cursor (`h`): from a header *or* any of its
    /// grove rows. The cursor lands on the section's header (its grove rows
    /// just vanished). A no-op when headers are hidden (N=1) or the list is
    /// empty.
    pub fn collapse(&mut self) {
        if !self.headers_shown() {
            return;
        }
        let Some(root) = self.selected_row().map(|r| r.repo_root().to_path_buf()) else {
            return;
        };
        self.folded.insert(root.clone());
        self.project();
        if let Some(idx) = self.header_index(&root) {
            self.selected = idx;
        }
        self.clamp_selection();
    }

    /// Expand the repo under the cursor (`l`). Only a folded header can be
    /// under the cursor for this to fire (a folded section's grove rows are
    /// hidden), so `l` on a grove row is a no-op here — 060 gives it a meaning
    /// (focus into detail).
    pub fn expand(&mut self) {
        let Some(root) = self.selected_row().map(|r| r.repo_root().to_path_buf()) else {
            return;
        };
        if self.folded.remove(&root) {
            self.project();
            if let Some(idx) = self.header_index(&root) {
                self.selected = idx;
            }
            self.clamp_selection();
        }
    }

    /// Toggle the fold of the header under the cursor (Enter on a header).
    /// A no-op when the cursor is on a grove row.
    pub fn toggle_fold(&mut self) {
        match self.selected_row() {
            Some(NavRow::Header { folded: true, .. }) => self.expand(),
            Some(NavRow::Header { folded: false, .. }) => self.collapse(),
            _ => {}
        }
    }

    fn clamp_selection(&mut self) {
        if self.rows.is_empty() {
            self.selected = 0;
        } else if self.selected >= self.rows.len() {
            self.selected = self.rows.len() - 1;
        }
    }

    /// Paint the nav into `buf` over `area`: a bordered list with the selected
    /// row reversed, scrolled so the cursor stays visible for fleets taller
    /// than the rect. Pure — no daemon, no terminal — which is the headless
    /// testability the rmux migration buys (see the tests below). [`Clear`]s
    /// `area` first so the live pane underneath does not bleed through.
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        // Just the surface name; the key hints live in the single whichkey
        // footer the App draws ([`crate::tui::footer`]), not duplicated here.
        let block = Block::default().borders(Borders::ALL).title(" groves ");
        let inner = block.inner(area);
        block.render(area, buf);
        if inner.width == 0 || inner.height == 0 {
            return;
        }
        if self.rows.is_empty() {
            Paragraph::new("no groves").render(inner, buf);
            return;
        }
        // The offset is a pure function of (selected, height): 0 while the
        // cursor fits, then the minimal scroll that keeps it on the last line.
        let height = inner.height as usize;
        let offset = self.selected.saturating_sub(height - 1);
        let indent = if self.headers_shown() { "  " } else { "" };
        for (i, row) in self.rows.iter().enumerate().skip(offset).take(height) {
            let rect = Rect::new(inner.x, inner.y + (i - offset) as u16, inner.width, 1);
            let (marker, sel_style) = if i == self.selected {
                ("\u{203a} ", Style::default().add_modifier(Modifier::REVERSED))
            } else {
                ("  ", Style::default())
            };
            let (text, style) = match row {
                NavRow::Header { label, folded, .. } => {
                    let glyph = if *folded { "\u{25b8}" } else { "\u{25be}" };
                    (
                        format!("{marker}{glyph} {label}"),
                        sel_style.add_modifier(Modifier::BOLD),
                    )
                }
                NavRow::Grove(item) => {
                    let (suffix, style) = if item.lifecycle == Lifecycle::Seed {
                        (" (seed)", sel_style.add_modifier(Modifier::DIM))
                    } else {
                        ("", sel_style)
                    };
                    (format!("{marker}{indent}{}{suffix}", item.label), style)
                }
            };
            Paragraph::new(Line::from(text)).style(style).render(rect, buf);
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

    /// Add a seed to a repo: an inbox directory with no matching worktree.
    fn add_seed(root: &std::path::Path, name: &str) {
        fs::create_dir_all(root.join(".grove-meta").join("inboxes").join(name)).unwrap();
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

    fn render_to_text(nav: &Nav, width: u16, height: u16) -> String {
        let area = Rect::new(0, 0, width, height);
        let mut buf = Buffer::empty(area);
        nav.render(area, &mut buf);
        buffer_text(&buf)
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
    fn single_repo_auto_hides_the_lone_header() {
        let tmp = TempDir::new().unwrap();
        let r = make_repo(tmp.path(), "solo", &["alpha", "beta"]);
        let nav = Nav::from_fleet(&fleet_of(&[r]));
        let text = render_to_text(&nav, 30, 6);
        // No header row, no fold glyph, no indent: the old flat look at N=1.
        assert!(!text.contains("solo"), "got:\n{text}");
        assert!(!text.contains('\u{25be}'), "got:\n{text}");
        assert!(text.contains("\u{203a} alpha"), "got:\n{text}");
    }

    #[test]
    fn multi_repo_groups_groves_under_headers() {
        let tmp = TempDir::new().unwrap();
        let r1 = make_repo(tmp.path(), "one", &["alpha"]);
        let r2 = make_repo(tmp.path(), "two", &["alpha"]);
        let nav = Nav::from_fleet(&fleet_of(&[r1, r2]));
        let text = render_to_text(&nav, 30, 8);
        // Headers carry the repo name with an expanded-fold glyph; the groves
        // sit indented under them with bare labels (no `<repo>/` prefix —
        // that's the flat ranked shape, 040/050).
        assert!(text.contains("\u{25be} one"), "got:\n{text}");
        assert!(text.contains("\u{25be} two"), "got:\n{text}");
        assert!(!text.contains("one/alpha"), "got:\n{text}");
        // The cursor starts on the first header.
        assert!(text.contains("\u{203a} \u{25be} one"), "got:\n{text}");
    }

    #[test]
    fn collapse_hides_a_section_and_expand_restores_it() {
        let tmp = TempDir::new().unwrap();
        let r1 = make_repo(tmp.path(), "one", &["alpha"]);
        let r2 = make_repo(tmp.path(), "two", &["beta"]);
        let mut nav = Nav::from_fleet(&fleet_of(&[r1, r2]));
        // Cursor on the "one" header; collapse hides alpha, shows the folded glyph.
        nav.collapse();
        let text = render_to_text(&nav, 30, 8);
        assert!(text.contains("\u{25b8} one"), "got:\n{text}");
        assert!(!text.contains("alpha"), "got:\n{text}");
        assert!(text.contains("beta"), "other sections unaffected:\n{text}");
        // Expand brings the section back.
        nav.expand();
        let text = render_to_text(&nav, 30, 8);
        assert!(text.contains("\u{25be} one"), "got:\n{text}");
        assert!(text.contains("alpha"), "got:\n{text}");
    }

    #[test]
    fn collapse_from_a_grove_row_lands_the_cursor_on_its_header() {
        let tmp = TempDir::new().unwrap();
        let r1 = make_repo(tmp.path(), "one", &["alpha"]);
        let r2 = make_repo(tmp.path(), "two", &["beta"]);
        let mut nav = Nav::from_fleet(&fleet_of(&[r1, r2.clone()]));
        // Walk down to "beta" (header one, alpha, header two, beta).
        for _ in 0..3 {
            nav.select_down();
        }
        assert_eq!(nav.selected().unwrap().name, "beta");
        nav.collapse();
        // The repo under the cursor folded; the cursor sits on its header.
        match nav.selected_row().unwrap() {
            NavRow::Header { repo_root, folded, .. } => {
                assert_eq!(repo_root, &r2);
                assert!(folded);
            }
            other => panic!("expected header, got {other:?}"),
        }
    }

    #[test]
    fn enter_on_a_header_toggles_its_fold() {
        let tmp = TempDir::new().unwrap();
        let r1 = make_repo(tmp.path(), "one", &["alpha"]);
        let r2 = make_repo(tmp.path(), "two", &["beta"]);
        let mut nav = Nav::from_fleet(&fleet_of(&[r1, r2]));
        // Cursor on a header: selected() is None, selected_row() is the header.
        assert!(nav.selected().is_none());
        assert!(matches!(nav.selected_row(), Some(NavRow::Header { .. })));
        nav.toggle_fold();
        assert!(matches!(
            nav.selected_row(),
            Some(NavRow::Header { folded: true, .. })
        ));
        nav.toggle_fold();
        assert!(matches!(
            nav.selected_row(),
            Some(NavRow::Header { folded: false, .. })
        ));
        // On a grove row the toggle is inert.
        nav.select_down();
        let before = nav.selected_row().cloned();
        nav.toggle_fold();
        assert_eq!(nav.selected_row().cloned(), before);
    }

    #[test]
    fn fold_keys_are_noops_at_a_single_repo_fleet() {
        let tmp = TempDir::new().unwrap();
        let r = make_repo(tmp.path(), "solo", &["alpha", "beta"]);
        let mut nav = Nav::from_fleet(&fleet_of(&[r]));
        nav.collapse();
        nav.expand();
        // No headers exist at N=1; everything stays listed and selected.
        assert_eq!(nav.selected().unwrap().name, "alpha");
        assert!(render_to_text(&nav, 30, 6).contains("beta"));
    }

    #[test]
    fn seeds_are_listed_with_a_lifecycle_marker() {
        let tmp = TempDir::new().unwrap();
        let r = make_repo(tmp.path(), "solo", &["alpha"]);
        add_seed(&r, "sprout");
        let nav = Nav::from_fleet(&fleet_of(&[r]));
        let text = render_to_text(&nav, 30, 6);
        assert!(text.contains("sprout (seed)"), "got:\n{text}");
        // Live groves are unchanged at N=1: bare label, no marker.
        assert!(text.contains("alpha\n") || text.contains("alpha "), "got:\n{text}");
        assert!(!text.contains("alpha (seed)"), "got:\n{text}");
    }

    #[test]
    fn flat_shape_keeps_repo_prefixed_labels_and_skips_seeds() {
        let tmp = TempDir::new().unwrap();
        let r1 = make_repo(tmp.path(), "one", &["alpha"]);
        let r2 = make_repo(tmp.path(), "two", &["alpha"]);
        add_seed(&r1, "sprout");
        let nav = Nav::flat_from_fleet(&fleet_of(&[r1, r2]));
        let text = render_to_text(&nav, 30, 6);
        // The picker shape: no headers, `<repo>/<grove>` disambiguation, and
        // only harness-capable (live) groves.
        assert!(text.contains("one/alpha"), "got:\n{text}");
        assert!(text.contains("two/alpha"), "got:\n{text}");
        assert!(!text.contains("sprout"), "got:\n{text}");
        assert!(!text.contains('\u{25be}'), "got:\n{text}");
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
    fn the_list_scrolls_to_keep_the_cursor_visible() {
        let tmp = TempDir::new().unwrap();
        let names: Vec<String> = (0..8).map(|i| format!("grove-{i}")).collect();
        let refs: Vec<&str> = names.iter().map(|s| s.as_str()).collect();
        let r = make_repo(tmp.path(), "solo", &refs);
        let mut nav = Nav::from_fleet(&fleet_of(&[r]));
        for _ in 0..7 {
            nav.select_down();
        }
        assert_eq!(nav.selected().unwrap().name, "grove-7");
        // Inner height is 3 (5 minus borders): only the window ending at the
        // cursor is painted.
        let text = render_to_text(&nav, 20, 5);
        assert!(text.contains("\u{203a} grove-7"), "got:\n{text}");
        assert!(text.contains("grove-5"), "got:\n{text}");
        assert!(!text.contains("grove-0"), "got:\n{text}");
        // Back at the top the window follows.
        for _ in 0..7 {
            nav.select_up();
        }
        let text = render_to_text(&nav, 20, 5);
        assert!(text.contains("\u{203a} grove-0"), "got:\n{text}");
        assert!(!text.contains("grove-7"), "got:\n{text}");
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
        let mut nav = Nav::from_fleet(&fleet_of(std::slice::from_ref(&r)));
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
    fn rebuild_keeps_fold_state_and_a_header_cursor() {
        let tmp = TempDir::new().unwrap();
        let r1 = make_repo(tmp.path(), "one", &["alpha"]);
        let r2 = make_repo(tmp.path(), "two", &["beta"]);
        let mut nav = Nav::from_fleet(&fleet_of(&[r1.clone(), r2.clone()]));
        nav.collapse(); // fold "one"; cursor on its header

        // A grove appears in "one" while it is folded; rebuild from fresh scans.
        let task_root = r1.join(".grove-worktrees").join("aardvark").join(".grove");
        fs::create_dir_all(&task_root).unwrap();
        fs::write(task_root.join("010-x.md"), "# 010-x\n").unwrap();
        nav.rebuild(&fleet_of(&[r1.clone(), r2]));

        // The fold survives (keyed by repo root) and the cursor is still on
        // the "one" header.
        let text = render_to_text(&nav, 30, 8);
        assert!(text.contains("\u{25b8} one"), "got:\n{text}");
        assert!(!text.contains("aardvark"), "folded section stays hidden:\n{text}");
        match nav.selected_row().unwrap() {
            NavRow::Header { repo_root, .. } => assert_eq!(repo_root, &r1),
            other => panic!("expected header, got {other:?}"),
        }
    }

    #[test]
    fn remove_drops_a_grove_and_clamps_the_selection() {
        let tmp = TempDir::new().unwrap();
        let r = make_repo(tmp.path(), "solo", &["alpha", "beta", "gamma"]);
        let mut nav = Nav::flat_from_fleet(&fleet_of(std::slice::from_ref(&r)));
        nav.select_down();
        nav.select_down(); // gamma (last)
        assert_eq!(nav.selected().unwrap().name, "gamma");
        // Excluding the now-selected last item clamps the cursor into range.
        nav.remove(&r, "gamma");
        assert!(nav.selected().is_some());
        assert_eq!(nav.selected().unwrap().name, "beta");
        // The dropped grove is gone from the list entirely.
        nav.remove(&r, "alpha");
        nav.remove(&r, "beta");
        assert!(nav.selected().is_none(), "removing every item empties the picker");
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
        assert!(buffer_text(&buf).contains("no groves"));
    }
}
