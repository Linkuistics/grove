//! The render path: project one pane's [`PaneState`] into a ratatui [`Buffer`]
//! and report where the hardware cursor belongs.
//!
//! This is the headless-testable seam — the whole point of the rmux migration.
//! `PaneState` is plain, `Clone` data with a public `snapshot`, and a
//! `PaneSnapshot` is constructible by hand, so the snapshot → widget → buffer
//! path runs in a unit test with no daemon and no terminal (see the tests
//! below). The async loop in [`crate::tui::app`] calls [`render_pane`] inside
//! `terminal.draw` and places the real cursor at the returned coordinates.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;
use ratatui_rmux::{PaneState, PaneWidget};

/// Which tool a grove's pane runs. `Harness` is the `grove do <name>` agent
/// session every grove opens with; the rest are the aux tools the working set
/// toggles (term / yazi / vcs, wired in 050/030). Several role-panes coexist
/// per grove — the reason the pane map is keyed by `(grove, role)`, not by the
/// grove name alone.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PaneRole {
    /// The `grove do <name>` harness session. The only live role in this leaf.
    Harness,
    /// A plain `$SHELL` (050/030).
    Term,
    /// A `yazi` file manager (050/030).
    Yazi,
    /// A version-control TUI — lazygit/lazyjj (050/030).
    Vcs,
}

/// The composite pane-map key: a grove's pane in a given role, or the bare-shell
/// fallback (no grove) `grove tui` falls back to when the fleet has no live
/// groves. Because several panes coexist per grove, the key carries the role —
/// the grove name alone is no longer a unique pane identity. The owning grove is
/// always recoverable via [`PaneKey::grove`], so the capture / reject / move /
/// detail paths still target the grove regardless of which role pane is focused.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum PaneKey {
    /// A grove's pane running `role`.
    Grove { grove: String, role: PaneRole },
    /// The no-grove shell fallback (a single pane; no role distinction needed).
    Shell,
}

impl PaneKey {
    /// The harness pane for `grove` — the role every grove opens with, and the
    /// sole live role until the aux panes land (050/030).
    pub fn harness(grove: impl Into<String>) -> Self {
        PaneKey::Grove {
            grove: grove.into(),
            role: PaneRole::Harness,
        }
    }

    /// The grove this pane belongs to, or `None` for the shell fallback. The
    /// capture/reject/move/detail paths resolve their target grove through this,
    /// so any role pane of a grove drives that grove's grooming.
    pub fn grove(&self) -> Option<&str> {
        match self {
            PaneKey::Grove { grove, .. } => Some(grove),
            PaneKey::Shell => None,
        }
    }

    /// Whether this is a working-set **aux** pane (term/yazi/vcs, 050/030) rather
    /// than a harness or the shell fallback. Aux panes size to their side-column
    /// slot (resize-on-show), so the full-terminal resize sweep skips them.
    pub fn is_aux(&self) -> bool {
        matches!(
            self,
            PaneKey::Grove {
                role: PaneRole::Term | PaneRole::Yazi | PaneRole::Vcs,
                ..
            }
        )
    }
}

/// Paint `state`'s captured grid into `buf` over `area`, returning the absolute
/// buffer coordinates `(x, y)` where the hardware cursor should be placed, or
/// `None` when the pane's cursor is hidden.
///
/// ratatui hides the terminal cursor every frame unless the host re-places it,
/// and the cursor coordinates must be read from the snapshot here because the
/// widget only paints cells — it does not move the real cursor.
pub fn render_pane(state: &PaneState, area: Rect, buf: &mut Buffer) -> Option<(u16, u16)> {
    PaneWidget::new(state).render(area, buf);

    let cursor = state.snapshot.cursor;
    if !cursor.visible {
        return None;
    }
    // Clamp into the rendered area so an out-of-range cursor never points
    // outside the buffer, and offset by the area origin to absolute coords.
    let cx = area.x + cursor.col.min(area.width.saturating_sub(1));
    let cy = area.y + cursor.row.min(area.height.saturating_sub(1));
    Some((cx, cy))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmux_sdk::{PaneCell, PaneCursor, PaneGlyph, PaneSnapshot};
    use std::collections::HashMap;

    #[test]
    fn two_roles_under_one_grove_coexist_and_route_independently() {
        // The whole point of the composite key: a grove can hold several panes
        // at once, and a snapshot tagged with one role must not clobber another.
        // The render loop routes via `panes.get_mut(&key)`, so exercise exactly
        // that — a map keyed by `PaneKey` with two roles of the *same* grove.
        let harness = PaneKey::harness("alpha");
        let term = PaneKey::Grove {
            grove: "alpha".into(),
            role: PaneRole::Term,
        };
        assert_ne!(harness, term, "same grove, different role → distinct keys");

        let mut panes: HashMap<PaneKey, &str> = HashMap::new();
        panes.insert(harness.clone(), "harness-grid");
        panes.insert(term.clone(), "term-grid");
        assert_eq!(panes.len(), 2, "both roles coexist; neither overwrote");
        assert_eq!(panes.get(&harness), Some(&"harness-grid"));
        assert_eq!(panes.get(&term), Some(&"term-grid"));
    }

    #[test]
    fn grove_is_recoverable_from_any_role_key() {
        // Capture/reject/move resolve the target grove from the focused key, so
        // every role pane of a grove must report that grove; the shell reports
        // none.
        for role in [
            PaneRole::Harness,
            PaneRole::Term,
            PaneRole::Yazi,
            PaneRole::Vcs,
        ] {
            let key = PaneKey::Grove {
                grove: "alpha".into(),
                role,
            };
            assert_eq!(
                key.grove(),
                Some("alpha"),
                "{role:?} should recover the grove"
            );
        }
        assert_eq!(PaneKey::Shell.grove(), None, "the shell has no grove");
    }

    /// Build a `cols`×`rows` snapshot whose first row holds `text`, the rest
    /// blank, with the given cursor — enough to exercise the render path.
    fn snapshot_with(cols: u16, rows: u16, text: &str, cursor: PaneCursor) -> PaneSnapshot {
        let mut cells: Vec<PaneCell> = Vec::with_capacity(usize::from(cols) * usize::from(rows));
        let first: Vec<PaneCell> = text
            .chars()
            .map(|c| PaneCell::new(PaneGlyph::new(c.to_string(), 1)))
            .collect();
        for col in 0..usize::from(cols) {
            cells.push(first.get(col).cloned().unwrap_or_else(PaneCell::blank));
        }
        for _ in 0..(usize::from(cols) * usize::from(rows - 1)) {
            cells.push(PaneCell::blank());
        }
        PaneSnapshot::new(cols, rows, cells, cursor).expect("valid snapshot shape")
    }

    #[test]
    fn paints_grid_glyphs_into_the_buffer() {
        let snapshot = snapshot_with(4, 2, "Hi", PaneCursor::new(0, 2, true, 0));
        let state = PaneState::from_snapshot(snapshot);
        let area = Rect::new(0, 0, 4, 2);
        let mut buf = Buffer::empty(area);

        render_pane(&state, area, &mut buf);

        assert_eq!(buf[(0, 0)].symbol(), "H");
        assert_eq!(buf[(1, 0)].symbol(), "i");
        assert_eq!(buf[(2, 0)].symbol(), " ");
    }

    #[test]
    fn returns_visible_cursor_position_offset_by_area() {
        let snapshot = snapshot_with(4, 2, "Hi", PaneCursor::new(1, 2, true, 0));
        let state = PaneState::from_snapshot(snapshot);
        // Offset area so we prove the cursor is reported in absolute buffer coords.
        let area = Rect::new(10, 5, 4, 2);
        let mut buf = Buffer::empty(area);

        let cursor = render_pane(&state, area, &mut buf);

        assert_eq!(cursor, Some((12, 6)));
    }

    #[test]
    fn hidden_cursor_reports_none() {
        let snapshot = snapshot_with(4, 2, "Hi", PaneCursor::new(0, 0, false, 0));
        let state = PaneState::from_snapshot(snapshot);
        let area = Rect::new(0, 0, 4, 2);
        let mut buf = Buffer::empty(area);

        assert_eq!(render_pane(&state, area, &mut buf), None);
    }

    #[test]
    fn cursor_clamped_to_area_bounds() {
        // A cursor beyond the visible area must clamp to the last cell, never
        // index outside the buffer.
        let snapshot = snapshot_with(4, 2, "Hi", PaneCursor::new(99, 99, true, 0));
        let state = PaneState::from_snapshot(snapshot);
        let area = Rect::new(0, 0, 4, 2);
        let mut buf = Buffer::empty(area);

        assert_eq!(render_pane(&state, area, &mut buf), Some((3, 1)));
    }
}
