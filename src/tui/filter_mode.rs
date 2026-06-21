//! The nav's **filter mode** — the stateful, interactive layer over the pure
//! 040 engine ([`crate::tui::filter`]). Where `filter.rs` is keys-free ranking,
//! this owns the live [`Criteria`] the user edits in mode, the ranked-list
//! selection cursor, and the surface render (the ranked rows + the engaged
//! criteria line). The keys themselves are the pure [`arbitrate`] table
//! (`src/tui/focus.rs`); the App glues the two together.
//!
//! ## Two cursors, one engine
//!
//! The [`Nav`](crate::tui::nav) surface keeps its grouped/flat shapes and its own
//! selection; this is a *separate* cursor over the **flat ranked list** the
//! engine projects. The App renders/routes to whichever is live: the grouped
//! [`Nav`] while [`Criteria`] is idle, this [`FilterMode`] while it is
//! [`engaged`](Criteria::engaged) (or while [`Focus::Filter`](crate::tui::focus::Focus)
//! is up and the needle is being typed). Keeping them apart leaves the
//! well-tested 030 nav untouched.
//!
//! ## fzf top-stick vs. background refresh
//!
//! Two reprojection intents, deliberately split:
//! - **A criteria edit** (typing the needle, flipping a toggle) re-ranks and
//!   snaps the cursor back to the top row (fzf behavior, the leaf's Notes) — the
//!   App calls [`reproject`](Self::reproject) then [`select_top`](Self::select_top).
//! - **An fs-watch tick** (a grove appeared/retired) re-ranks but *preserves* the
//!   selected grove by identity so a background refresh doesn't yank the cursor —
//!   [`reproject`](Self::reproject) alone.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Widget};

use crate::multi_repo_view::MultiRepoView;
use crate::repo_view::Lifecycle;
use crate::tui::filter::{project, Criteria, Ranked};

/// One in-mode edit to the live [`Criteria`] (the pure half of a keypress). The
/// [`arbitrate`](crate::tui::focus::arbitrate) table emits these; [`apply`](FilterMode::apply)
/// folds them in. Collapsing the five edits into one enum keeps the App's
/// `handle_event` arm single (mutate → reproject → top), not five near-copies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterEdit {
    /// Append literal text to the fuzzy needle (a typed char, or a paste).
    Insert(String),
    /// Drop the last char of the needle.
    Backspace,
    /// Flip the inbox-pending toggle (Ctrl-i / Tab).
    ToggleInbox,
    /// Advance the lifecycle cycle all → live → seed (Ctrl-l).
    CycleLifecycle,
    /// Advance the sort cycle name → recency → pending (Ctrl-s).
    CycleSort,
}

/// The filter mode's session state: the live criteria, the latest ranked
/// projection, and the cursor over it. Ephemeral per session (constraint 1) —
/// nothing persists.
#[derive(Debug, Clone, Default)]
pub struct FilterMode {
    criteria: Criteria,
    ranked: Vec<Ranked>,
    selected: usize,
}

impl FilterMode {
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether any dimension is off its default — the grouped-vs-ranked shape
    /// boundary the App switches the nav surface on.
    pub fn engaged(&self) -> bool {
        self.criteria.engaged()
    }

    /// The live criteria (for the App's draw/route decisions and tests).
    pub fn criteria(&self) -> &Criteria {
        &self.criteria
    }

    /// Fold one in-mode edit into the criteria (pure — no reprojection; the App
    /// calls [`reproject`](Self::reproject) + [`select_top`](Self::select_top)
    /// after, so the ranked list and the fzf cursor-snap stay in step).
    pub fn apply(&mut self, edit: FilterEdit) {
        match edit {
            FilterEdit::Insert(s) => self.criteria.needle.push_str(&s),
            FilterEdit::Backspace => {
                self.criteria.needle.pop();
            }
            FilterEdit::ToggleInbox => self.criteria.inbox_only = !self.criteria.inbox_only,
            FilterEdit::CycleLifecycle => self.criteria.lifecycle = self.criteria.lifecycle.cycle(),
            FilterEdit::CycleSort => self.criteria.sort = self.criteria.sort.cycle(),
        }
    }

    /// Clear every dimension back to idle (Esc in mode, or the first layered Esc
    /// in normal Nav). The needle and toggles drop; the nav reverts to grouped.
    pub fn clear(&mut self) {
        self.criteria = Criteria::default();
    }

    /// Re-rank from a fresh fleet snapshot, **preserving** the selected grove by
    /// identity (`repo_root` + `name`) when it survives the new ranking, else
    /// clamping into range. Used both after a criteria edit (the App then snaps
    /// to the top with [`select_top`](Self::select_top)) and on a background
    /// fs-watch tick (where preserving identity is the whole point).
    pub fn reproject(&mut self, fleet: &MultiRepoView) {
        let prior = self
            .ranked
            .get(self.selected)
            .map(|r| (r.repo_root.clone(), r.name.clone()));
        self.ranked = project(fleet, &self.criteria);
        self.selected = prior
            .and_then(|(root, name)| {
                self.ranked
                    .iter()
                    .position(|r| r.repo_root == root && r.name == name)
            })
            .unwrap_or(0)
            .min(self.ranked.len().saturating_sub(1));
    }

    /// Snap the cursor to the top-ranked row (fzf behavior after a criteria edit).
    pub fn select_top(&mut self) {
        self.selected = 0;
    }

    /// Move the cursor up one ranked row (saturating at the top).
    pub fn select_up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    /// Move the cursor down one ranked row (saturating at the bottom).
    pub fn select_down(&mut self) {
        if self.selected + 1 < self.ranked.len() {
            self.selected += 1;
        }
    }

    /// The ranked row under the cursor, or `None` when nothing matches.
    pub fn selected(&self) -> Option<&Ranked> {
        self.ranked.get(self.selected)
    }

    /// The engaged-criteria summary the leaf calls for: the active toggle/sort
    /// chips joined with ` · `, or `None` when idle (so the App renders it
    /// *whenever any dimension is engaged; never when idle*). The needle is the
    /// in-surface prompt, not a chip, so it is omitted here — a bare needle still
    /// engages, and [`engaged`](Self::engaged) drives the boundary regardless.
    pub fn summary(&self) -> Option<String> {
        if !self.engaged() {
            return None;
        }
        let mut chips = Vec::new();
        if self.criteria.inbox_only {
            chips.push("inbox".to_string());
        }
        if self.criteria.lifecycle != crate::tui::filter::LifecycleFilter::All {
            chips.push(self.criteria.lifecycle.label().to_string());
        }
        if self.criteria.sort != crate::tui::filter::SortOrder::Name {
            chips.push(format!("\u{2193}{}", self.criteria.sort.label())); // ↓recency / ↓pending
        }
        // A bare needle engages with no chips: return an empty marker so the line
        // still renders (the needle prompt carries it), distinct from idle/None.
        Some(chips.join(" \u{00b7} "))
    }

    /// Paint the ranked-list surface into `buf`: a bordered ` groves ` block (the
    /// nav's frame) with a header row — the live `/needle` prompt on the left and
    /// the engaged criteria chips on the right — then the ranked rows below,
    /// scrolled to keep the cursor visible. `active` (the [`Focus::Filter`] typing
    /// state) shows the caret and returns its position so the App can place the
    /// hardware cursor in the needle. Pure snapshot → `Buffer`, headless-tested.
    pub fn render(&self, area: Rect, buf: &mut Buffer, active: bool) -> Option<(u16, u16)> {
        Clear.render(area, buf);
        let block = Block::default().borders(Borders::ALL).title(" groves ");
        let inner = block.inner(area);
        block.render(area, buf);
        if inner.width == 0 || inner.height == 0 {
            return None;
        }

        // Header row: the needle prompt (left) + the criteria chips (right).
        let prompt = format!("/{}", self.criteria.needle);
        let header_rect = Rect::new(inner.x, inner.y, inner.width, 1);
        Paragraph::new(prompt.clone())
            .style(Style::default().add_modifier(Modifier::BOLD))
            .render(header_rect, buf);
        if let Some(chips) = self.summary() {
            if !chips.is_empty() {
                let w = chips.chars().count() as u16;
                if w < inner.width {
                    let chip_rect = Rect::new(inner.right() - w, inner.y, w, 1);
                    Paragraph::new(chips)
                        .style(Style::default().add_modifier(Modifier::DIM))
                        .render(chip_rect, buf);
                }
            }
        }
        // The caret sits just past the `/` + needle (active typing only).
        let caret = if active {
            Some((inner.x + prompt.chars().count() as u16, inner.y))
        } else {
            None
        };

        // The ranked rows fill the region below the header.
        let list_y = inner.y + 1;
        let list_h = inner.height.saturating_sub(1) as usize;
        if list_h == 0 {
            return caret;
        }
        if self.ranked.is_empty() {
            Paragraph::new("no matches").render(Rect::new(inner.x, list_y, inner.width, 1), buf);
            return caret;
        }
        // Same scroll math as the nav: 0 while the cursor fits, then the minimal
        // offset that keeps it on the last visible line.
        let offset = self.selected.saturating_sub(list_h - 1);
        for (i, row) in self.ranked.iter().enumerate().skip(offset).take(list_h) {
            let rect = Rect::new(inner.x, list_y + (i - offset) as u16, inner.width, 1);
            let (marker, sel_style) = if i == self.selected {
                (
                    "\u{203a} ",
                    Style::default().add_modifier(Modifier::REVERSED),
                )
            } else {
                ("  ", Style::default())
            };
            let (suffix, style) = if row.lifecycle == Lifecycle::Seed {
                (" (seed)", sel_style.add_modifier(Modifier::DIM))
            } else {
                ("", sel_style)
            };
            Paragraph::new(Line::from(format!("{marker}{}{suffix}", row.label)))
                .style(style)
                .render(rect, buf);
        }
        caret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo_view::RepoView;
    use crate::tui::filter::{LifecycleFilter, SortOrder};
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    // --- fixtures (same shape as the nav/filter tests) ----------------------

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

    fn render_to_text(fm: &FilterMode, width: u16, height: u16, active: bool) -> String {
        let area = Rect::new(0, 0, width, height);
        let mut buf = Buffer::empty(area);
        fm.render(area, &mut buf, active);
        let mut s = String::new();
        for y in 0..area.height {
            for x in 0..area.width {
                s.push_str(buf[(x, y)].symbol());
            }
            s.push('\n');
        }
        s
    }

    // --- engaged / summary boundary -----------------------------------------

    #[test]
    fn idle_is_not_engaged_and_has_no_summary() {
        let fm = FilterMode::new();
        assert!(!fm.engaged());
        assert_eq!(fm.summary(), None);
    }

    #[test]
    fn each_dimension_engages_and_summary_lists_only_non_defaults() {
        // A bare needle engages but has no chips (the prompt carries it).
        let mut fm = FilterMode::new();
        fm.apply(FilterEdit::Insert("ab".into()));
        assert!(fm.engaged());
        assert_eq!(fm.summary().as_deref(), Some(""));

        // Each toggle/sort adds exactly its own chip.
        let mut fm = FilterMode::new();
        fm.apply(FilterEdit::ToggleInbox);
        assert_eq!(fm.summary().as_deref(), Some("inbox"));

        let mut fm = FilterMode::new();
        fm.apply(FilterEdit::CycleLifecycle); // all → live
        assert_eq!(fm.summary().as_deref(), Some("live"));

        let mut fm = FilterMode::new();
        fm.apply(FilterEdit::CycleSort); // name → recency
        assert_eq!(fm.summary().as_deref(), Some("\u{2193}recency"));
    }

    #[test]
    fn summary_joins_multiple_engaged_dimensions() {
        let mut fm = FilterMode::new();
        fm.apply(FilterEdit::ToggleInbox);
        fm.apply(FilterEdit::CycleLifecycle); // live
        fm.apply(FilterEdit::CycleSort); // recency
        assert_eq!(
            fm.summary().as_deref(),
            Some("inbox \u{00b7} live \u{00b7} \u{2193}recency")
        );
    }

    // --- needle editing ------------------------------------------------------

    #[test]
    fn insert_and_backspace_edit_the_needle() {
        let mut fm = FilterMode::new();
        fm.apply(FilterEdit::Insert("a".into()));
        fm.apply(FilterEdit::Insert("bc".into()));
        assert_eq!(fm.criteria().needle, "abc");
        fm.apply(FilterEdit::Backspace);
        assert_eq!(fm.criteria().needle, "ab");
    }

    #[test]
    fn reproject_preserves_the_needle_so_re_entry_keeps_it() {
        // Accept (App: no-op on the model) then re-enter (App: FilterEnter →
        // reproject) must keep the needle: only `clear` drops it. This is the
        // "re-enter with needle preserved" leg of the done-when.
        let tmp = TempDir::new().unwrap();
        let r = make_repo(tmp.path(), "solo", &["alpha"]);
        let mut fm = FilterMode::new();
        fm.apply(FilterEdit::Insert("al".into()));
        fm.reproject(&fleet_of(std::slice::from_ref(&r))); // accept leaves it engaged
        assert_eq!(fm.criteria().needle, "al");
        fm.reproject(&fleet_of(&[r])); // re-entry reprojects, never clears
        assert_eq!(fm.criteria().needle, "al", "the needle survives re-entry");
        assert!(fm.engaged());
    }

    #[test]
    fn clear_resets_every_dimension() {
        let mut fm = FilterMode::new();
        fm.apply(FilterEdit::Insert("ab".into()));
        fm.apply(FilterEdit::ToggleInbox);
        fm.apply(FilterEdit::CycleSort);
        assert!(fm.engaged());
        fm.clear();
        assert!(!fm.engaged());
        assert_eq!(fm.criteria().needle, "");
    }

    // --- toggle cycles -------------------------------------------------------

    #[test]
    fn inbox_toggles_off_again_and_disengages() {
        let mut fm = FilterMode::new();
        fm.apply(FilterEdit::ToggleInbox);
        assert!(fm.criteria().inbox_only);
        fm.apply(FilterEdit::ToggleInbox);
        assert!(!fm.criteria().inbox_only);
        assert!(!fm.engaged(), "back to default disengages");
    }

    #[test]
    fn lifecycle_and_sort_cycle_through_their_orders() {
        let mut fm = FilterMode::new();
        fm.apply(FilterEdit::CycleLifecycle);
        assert_eq!(fm.criteria().lifecycle, LifecycleFilter::Live);
        fm.apply(FilterEdit::CycleLifecycle);
        assert_eq!(fm.criteria().lifecycle, LifecycleFilter::Seed);
        fm.apply(FilterEdit::CycleSort);
        assert_eq!(fm.criteria().sort, SortOrder::Recency);
    }

    // --- reprojection + selection (fzf top-stick vs identity-preserve) -------

    #[test]
    fn criteria_edit_then_select_top_snaps_to_the_top_row() {
        let tmp = TempDir::new().unwrap();
        let r = make_repo(tmp.path(), "solo", &["alpha", "beta", "gamma"]);
        let mut fm = FilterMode::new();
        fm.reproject(&fleet_of(std::slice::from_ref(&r)));
        // Move down, then a criteria edit + top-snap returns to row 0 (fzf).
        fm.select_down();
        fm.select_down();
        assert_eq!(fm.selected().unwrap().name, "gamma");
        fm.apply(FilterEdit::Insert("a".into()));
        fm.reproject(&fleet_of(&[r]));
        fm.select_top();
        assert_eq!(
            fm.selected().unwrap().name,
            "alpha",
            "needle snaps the cursor to the top"
        );
    }

    #[test]
    fn background_reproject_preserves_the_selected_grove() {
        let tmp = TempDir::new().unwrap();
        let r = make_repo(tmp.path(), "solo", &["alpha", "beta"]);
        let mut fm = FilterMode::new();
        fm.reproject(&fleet_of(std::slice::from_ref(&r)));
        fm.select_down();
        assert_eq!(fm.selected().unwrap().name, "beta");
        // A grove appears earlier in name order; a plain reproject (fs-watch) must
        // keep the cursor on beta, not snap it.
        let task_root = r.join(".grove-worktrees").join("aardvark").join(".grove");
        fs::create_dir_all(&task_root).unwrap();
        fs::write(task_root.join("010-x.md"), "# 010-x\n").unwrap();
        fm.reproject(&fleet_of(&[r]));
        assert_eq!(
            fm.selected().unwrap().name,
            "beta",
            "background refresh keeps identity"
        );
    }

    #[test]
    fn selection_saturates_and_clamps_on_a_shrinking_list() {
        let tmp = TempDir::new().unwrap();
        let r = make_repo(tmp.path(), "solo", &["alpha", "beta", "gamma"]);
        let mut fm = FilterMode::new();
        fm.reproject(&fleet_of(std::slice::from_ref(&r)));
        for _ in 0..5 {
            fm.select_down();
        }
        assert_eq!(
            fm.selected().unwrap().name,
            "gamma",
            "saturates at the bottom"
        );
        // Narrow the needle so only alpha survives; the cursor clamps into range.
        fm.apply(FilterEdit::Insert("alp".into()));
        fm.reproject(&fleet_of(&[r]));
        assert_eq!(fm.selected().unwrap().name, "alpha");
    }

    // --- render --------------------------------------------------------------

    #[test]
    fn render_shows_the_needle_prompt_and_ranked_rows() {
        let tmp = TempDir::new().unwrap();
        let r = make_repo(tmp.path(), "solo", &["alpha", "beta"]);
        let mut fm = FilterMode::new();
        fm.apply(FilterEdit::Insert("al".into()));
        fm.reproject(&fleet_of(&[r]));
        let text = render_to_text(&fm, 30, 8, true);
        assert!(text.contains("/al"), "the needle prompt renders: {text}");
        assert!(text.contains("alpha"), "the match renders: {text}");
        assert!(
            !text.contains("beta"),
            "the non-match is filtered out: {text}"
        );
    }

    #[test]
    fn render_shows_the_criteria_chips_when_engaged_and_not_when_idle() {
        let tmp = TempDir::new().unwrap();
        let r = make_repo(tmp.path(), "solo", &["alpha"]);
        let fleet = fleet_of(&[r]);

        let mut fm = FilterMode::new();
        fm.apply(FilterEdit::ToggleInbox);
        fm.apply(FilterEdit::CycleLifecycle); // live
        fm.reproject(&fleet);
        let text = render_to_text(&fm, 40, 6, false);
        assert!(text.contains("inbox"), "engaged chips render: {text}");
        assert!(text.contains("live"), "engaged chips render: {text}");

        // Idle: the engine still projects (all groves), but no chips.
        let mut idle = FilterMode::new();
        idle.reproject(&fleet);
        let text = render_to_text(&idle, 40, 6, false);
        assert!(
            !text.contains("inbox"),
            "idle shows no criteria chips: {text}"
        );
    }

    #[test]
    fn render_caret_tracks_the_needle_only_when_active() {
        let tmp = TempDir::new().unwrap();
        let r = make_repo(tmp.path(), "solo", &["alpha"]);
        let mut fm = FilterMode::new();
        fm.apply(FilterEdit::Insert("ab".into()));
        fm.reproject(&fleet_of(&[r]));
        let area = Rect::new(0, 0, 30, 6);
        let mut buf = Buffer::empty(area);
        // Active typing: caret sits just past `/ab` inside the left border.
        let caret = fm.render(area, &mut buf, true);
        assert_eq!(caret, Some((1 + 3, 1)), "caret at `/ab`| inside the border");
        // Not active (engaged Nav browse): no caret.
        let caret = fm.render(area, &mut buf, false);
        assert_eq!(caret, None);
    }

    #[test]
    fn render_empty_match_set_shows_a_no_matches_line() {
        let tmp = TempDir::new().unwrap();
        let r = make_repo(tmp.path(), "solo", &["alpha"]);
        let mut fm = FilterMode::new();
        fm.apply(FilterEdit::Insert("zzz".into()));
        fm.reproject(&fleet_of(&[r]));
        let text = render_to_text(&fm, 30, 6, true);
        assert!(
            text.contains("no matches"),
            "empty result set is explicit: {text}"
        );
    }
}
