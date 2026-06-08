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
