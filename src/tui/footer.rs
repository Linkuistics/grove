//! The **whichkey footer** (050/010-surfaces verdict): one footer line the
//! [`App`](crate::tui::app) draws each frame. When the leader-dispatch gate is
//! open ([`Focus::LeaderPending`]) it is the live leader menu; otherwise it is the
//! focused surface's own hint line.
//!
//! Whichkey collapsed too far to earn a surface of its own. Under the inversion
//! grove owns the draw loop, so there is exactly *one* draw loop and *one* footer
//! — ADR-0019's single-hint-owner rule holds **by construction**, with no
//! publish/subscribe host-driver seam and no injected `grove-whichkey` pane.
//!
//! Like the rest of `src/tui/`'s presentation, the footer is a **pure**
//! focus → text → [`Buffer`] path: [`footer_text`] is unit-tested directly and
//! [`render_footer`] is headless buffer-tested (no daemon, no terminal).

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Clear, Paragraph, Widget};

use crate::tui::config::Leader;
use crate::tui::focus::{DetailOrigin, Focus};

/// The leader dispatch menu, shown while [`Focus::LeaderPending`]: the keys the
/// gate routes (mirrors the [`arbitrate`](crate::tui::focus::arbitrate) table),
/// in dispatch order: surfaces (`g`/`d`/`c`), the aux-pane toggles
/// (`t`/`y`/`v`, 050/030), then the app actions (`e`/`q`).
pub fn leader_menu() -> String {
    "g nav · d detail · c capture · t term · y yazi · v vcs · e editor · q quit · ⎋ cancel"
        .to_string()
}

/// The footer text for a focus state, or `None` when no footer should draw.
///
/// - [`Focus::LeaderPending`] → the live [`leader_menu`].
/// - The home [`Focus::Pane`] → how to open the gate.
/// - [`Focus::Detail`] → its in-surface keys + the gate; the `⎋` target names
///   where the panel returns (the pane, or the nav for a 060 live preview).
/// - [`Focus::Nav`] → its keys; when a filter is `engaged` the hint swaps fold →
///   `/` re-filter and Esc → clear (the layered Esc's first rung, 050 Q5).
/// - [`Focus::Filter`] → the in-mode key hints (toggles + accept/clear).
/// - [`Focus::Modal`] → `None`: the modal draws its own title hints, and its
///   bottom row is its content — a footer there would collide.
///
/// `engaged` is the App's [`FilterMode::engaged`](crate::tui::filter_mode::FilterMode::engaged)
/// — the table can't carry it (it is mutable App state), so it rides in here to
/// pick the Nav hint variant.
pub fn footer_text(focus: &Focus, leader: &Leader, engaged: bool) -> Option<String> {
    let lead = leader.label();
    match focus {
        Focus::LeaderPending { .. } => Some(leader_menu()),
        Focus::Pane => Some(format!("{lead} leader")),
        Focus::Detail { origin } => {
            let back = match origin {
                DetailOrigin::Pane => "pane",
                DetailOrigin::Nav => "nav",
            };
            Some(format!(
                "↑/↓ select · x reject · m move · ⎋ {back} · {lead} leader"
            ))
        }
        Focus::Nav if engaged => Some(format!(
            "↑/↓ move · ⏎ open · / filter · ⎋ clear · {lead} leader"
        )),
        Focus::Nav => Some(format!(
            "↑/↓ move · h/l fold · / filter · ⏎ open · ⎋ pane · {lead} leader"
        )),
        Focus::Filter => Some(
            "type to filter · ^i inbox · ^l lifecycle · ^s sort · ⏎ accept · ⎋ clear".to_string(),
        ),
        Focus::Modal { .. } => None,
    }
}

/// Paint the footer on the bottom row of `area` (overlay, like the capture
/// toast — proper row-reservation is 050's composed layout). The leader menu
/// renders with an accent background so the open gate is unmistakable; surface
/// hints are dim. A no-op when [`footer_text`] yields `None`.
pub fn render_footer(focus: &Focus, leader: &Leader, engaged: bool, area: Rect, buf: &mut Buffer) {
    let Some(text) = footer_text(focus, leader, engaged) else {
        return;
    };
    if area.height == 0 || area.width == 0 {
        return;
    }
    let row = Rect::new(area.x, area.y + area.height - 1, area.width, 1);
    Clear.render(row, buf);
    // The open gate gets an accent background so it is unmistakable; surface
    // hints are dim so they recede.
    let style = if matches!(focus, Focus::LeaderPending { .. }) {
        Style::default().fg(Color::Black).bg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    Paragraph::new(format!(" {text} "))
        .style(style)
        .render(row, buf);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::focus::{DetailOrigin, ModalKind};

    fn leader() -> Leader {
        Leader::alt_g()
    }

    fn pending() -> Focus {
        Focus::LeaderPending {
            prior: Box::new(Focus::Pane),
        }
    }

    fn capture() -> Focus {
        Focus::Modal {
            kind: ModalKind::Capture,
            prior: Box::new(Focus::Pane),
        }
    }

    fn bottom_row(buf: &Buffer) -> String {
        let area = buf.area;
        (0..area.width)
            .map(|x| {
                buf[(area.x + x, area.y + area.height - 1)]
                    .symbol()
                    .to_string()
            })
            .collect()
    }

    // --- footer_text (pure) ------------------------------------------------

    #[test]
    fn pending_text_is_the_leader_menu() {
        let text = footer_text(&pending(), &leader(), false).expect("pending has a footer");
        for token in [
            "g nav",
            "d detail",
            "c capture",
            "t term",
            "y yazi",
            "v vcs",
            "e editor",
            "q quit",
            "cancel",
        ] {
            assert!(text.contains(token), "menu missing {token:?}: {text}");
        }
    }

    #[test]
    fn pane_text_shows_how_to_open_the_gate() {
        let text = footer_text(&Focus::Pane, &leader(), false).expect("pane has a footer");
        assert!(text.contains("⌥g"), "pane hint names the leader: {text}");
        assert!(text.contains("leader"), "pane hint labels the gate: {text}");
    }

    #[test]
    fn detail_text_offers_grooming_and_the_origin_aware_return() {
        let pane = footer_text(
            &Focus::Detail {
                origin: DetailOrigin::Pane,
            },
            &leader(),
            false,
        )
        .expect("detail has a footer");
        assert!(pane.contains("select"), "detail hint shows select: {pane}");
        assert!(pane.contains("reject"), "detail hint shows reject: {pane}");
        assert!(pane.contains("move"), "detail hint shows move: {pane}");
        assert!(
            pane.contains("⎋ pane"),
            "pane-entered detail offers ⎋ pane: {pane}"
        );
        assert!(
            pane.contains("⌥g"),
            "detail hint still names the leader: {pane}"
        );

        // The 060 live preview returns to the nav, and the hint says so.
        let nav = footer_text(
            &Focus::Detail {
                origin: DetailOrigin::Nav,
            },
            &leader(),
            false,
        )
        .expect("detail has a footer");
        assert!(
            nav.contains("⎋ nav"),
            "nav-entered detail offers ⎋ nav: {nav}"
        );
    }

    #[test]
    fn nav_text_lists_its_in_surface_keys() {
        let text = footer_text(&Focus::Nav, &leader(), false).expect("nav has a footer");
        assert!(text.contains("move"), "nav hint shows move: {text}");
        assert!(text.contains("open"), "nav hint shows open: {text}");
        assert!(
            text.contains("/ filter"),
            "nav hint advertises the `/` mode: {text}"
        );
        assert!(text.contains("fold"), "idle nav still offers fold: {text}");
    }

    #[test]
    fn engaged_nav_swaps_fold_for_filter_clear() {
        // With a filter engaged, fold is meaningless (the ranked list has no
        // headers) and Esc clears rather than returning to the pane (layer one).
        let text = footer_text(&Focus::Nav, &leader(), true).expect("nav has a footer");
        assert!(
            !text.contains("fold"),
            "engaged nav drops the fold hint: {text}"
        );
        assert!(
            text.contains("clear"),
            "engaged nav offers Esc → clear: {text}"
        );
        assert!(
            text.contains("/ filter"),
            "engaged nav offers `/` re-filter: {text}"
        );
    }

    #[test]
    fn filter_mode_lists_its_in_mode_keys() {
        let text = footer_text(&Focus::Filter, &leader(), false).expect("filter has a footer");
        for token in [
            "filter",
            "^i inbox",
            "^l lifecycle",
            "^s sort",
            "accept",
            "clear",
        ] {
            assert!(
                text.contains(token),
                "filter hint missing {token:?}: {text}"
            );
        }
    }

    #[test]
    fn modal_has_no_footer() {
        assert_eq!(footer_text(&capture(), &leader(), false), None);
    }

    // --- render_footer (headless buffer) -----------------------------------

    #[test]
    fn render_paints_the_leader_menu_on_the_bottom_row() {
        // Wide enough to fit the full menu (the aux toggles t/y/v lengthened it
        // past 80 cols — on a narrower terminal the tail simply truncates).
        let area = Rect::new(0, 0, 100, 6);
        let mut buf = Buffer::empty(area);
        render_footer(&pending(), &leader(), false, area, &mut buf);
        let row = bottom_row(&buf);
        assert!(
            row.contains("g nav"),
            "menu painted on the bottom row: {row}"
        );
        assert!(
            row.contains("t term"),
            "aux toggles painted on the bottom row: {row}"
        );
        assert!(
            row.contains("cancel"),
            "menu painted on the bottom row: {row}"
        );
    }

    #[test]
    fn render_paints_the_surface_hint_when_not_pending() {
        let area = Rect::new(0, 0, 80, 6);
        let mut buf = Buffer::empty(area);
        render_footer(&Focus::Nav, &leader(), false, area, &mut buf);
        assert!(
            bottom_row(&buf).contains("move"),
            "nav hint painted on the bottom row"
        );
    }

    #[test]
    fn render_paints_nothing_for_a_modal() {
        let area = Rect::new(0, 0, 80, 6);
        let mut buf = Buffer::empty(area);
        render_footer(&capture(), &leader(), false, area, &mut buf);
        let row = bottom_row(&buf);
        assert!(
            row.trim().is_empty(),
            "modal leaves the footer row blank: {row:?}"
        );
    }
}
