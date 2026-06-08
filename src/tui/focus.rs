//! The focus state machine (E4): grove-owned, **leader-dispatch** input
//! arbitration.
//!
//! grove owns the draw loop, so there is no zellij locked-mode — grove is the
//! arbiter by construction and sees every crossterm event first. [`arbitrate`] is
//! a **pure function** `(Focus, Leader, Event) → (Focus, Action)`: the
//! headless-testable transition table the brief calls for. The app
//! ([`crate::tui::app`]) owns the side effects — it applies the returned
//! [`Action`] (forward to the pane, mutate the modal buffer, redraw, quit) and
//! adopts the returned [`Focus`].
//!
//! ## The model (050/010-surfaces verdict)
//!
//! A grove's view is a *composed layout* grove draws — visible panels coexist and
//! focus moves laterally between them. [`Focus::Pane`] is the home base; the
//! leader is a **dispatch gate**, not a direct flip.
//!
//! - **[`Focus::Pane`]** — a foreign rmux pane is focused. Forward *every* key to
//!   it except the leader (maximising key fidelity). This is the generalisation of
//!   the former `Harness`: it is *any* focused foreign pane (harness today; the
//!   050 aux term/yazi/vcs panes tomorrow). Which pane is focused lives in the
//!   app's `self.focused` map key — the focus type does not name it.
//! - **[`Focus::Detail`]** — the per-grove detail panel, a focus peer beside the
//!   pane. The widget itself is 030/040; here the *transition* is wired (`Esc`
//!   returns to the pane; in-surface keys arrive once the widget exists).
//! - **[`Focus::Nav`]** — grove's grove-list surface. Its in-surface keys
//!   (`j`/`k`/arrows move, `Enter` opens, `Esc` returns to the pane) stay; the
//!   former leader-prefixed actions (`c`/`e`/`q`) have moved onto the gate.
//! - **[`Focus::Modal`]** — a focus overlay that captures all keys (text into its
//!   buffer), restoring the **prior** focus on `Esc` (cancel) or `Enter`
//!   (submit). Swallows the leader (it owns focus while up).
//! - **[`Focus::LeaderPending`]** — the transient dispatch gate. The leader (from
//!   any non-modal surface) enters it; the *next* key dispatches: `g`→Nav,
//!   `d`→Detail, `c`→Capture, `e`→OpenEditor, `q`→Quit, `Esc`/anything-else →
//!   cancel back to the surface we leadered from (`prior`). The 050 aux-pane keys
//!   (`t`/`y`/`v`) slot in here later; do not wire panes that don't exist yet.
//!
//! ## Whichkey lives in the app footer, not here (050/010 verdict)
//!
//! The leader menu and per-surface hint line are one footer the [`App`] draws (see
//! [`crate::tui::footer`]). The focus type only carries *which state* we are in;
//! the footer reads it. ADR-0019's single-hint-owner holds by construction — one
//! draw loop, one footer — with no publish/subscribe or injected pane.

use ratatui::crossterm::event::{
    Event, KeyCode, KeyEvent, KeyEventKind, MouseButton, MouseEventKind,
};

use crate::tui::config::Leader;
use crate::tui::input::{map_key, KeyToken};

/// Which kind of modal is up. Only [`ModalKind::Capture`] exists today; the enum
/// is here so later leaves can add modal kinds without reshaping the focus type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalKind {
    /// The capture modal.
    Capture,
}

/// Which surface currently owns keyboard input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Focus {
    /// A focused foreign rmux pane (forward everything but the leader). The app's
    /// `self.focused` says *which* pane; the focus type is pane-agnostic.
    Pane,
    /// The per-grove detail panel (a focus peer beside the pane).
    Detail,
    /// grove's grove-list surface.
    Nav,
    /// A focus overlay; restores `prior` on cancel/submit.
    Modal {
        kind: ModalKind,
        prior: Box<Focus>,
    },
    /// The transient leader-dispatch gate: the next key dispatches, then we leave.
    /// `prior` is the surface we leadered from, restored on cancel.
    LeaderPending {
        prior: Box<Focus>,
    },
}

/// What the app should do in response to one input event. The transition table
/// is pure; this enum is the instruction the impure app layer carries out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Nothing to do (and nothing changed).
    Ignore,
    /// Forward literal text to the focused pane (`send_text`).
    SendText(String),
    /// Forward a tmux key token to the focused pane (`send_key`).
    SendKey(String),
    /// Forward a bracketed paste to the pane — the app wraps it in
    /// `\e[200~…\e[201~` so multi-line pastes don't execute line-by-line.
    SendPaste(String),
    /// Forward a left-click to the focused pane at a (row, col) pane cell.
    HarnessClick { row: u16, col: u16 },
    /// Move the nav selection up one row.
    NavUp,
    /// Move the nav selection down one row.
    NavDown,
    /// Open (or focus) the harness for the nav's currently selected grove.
    /// The app applies the open/focus and lands on the pane.
    NavSelect,
    /// Scroll the detail panel's content up one line.
    DetailScrollUp,
    /// Scroll the detail panel's content down one line.
    DetailScrollDown,
    /// A grove surface changed; the app should redraw.
    Redraw,
    /// Insert literal text into the focused modal's buffer.
    ModalInsert(String),
    /// Delete the last char of the modal buffer.
    ModalBackspace,
    /// Submit the modal buffer (grove's capture write).
    ModalSubmit,
    /// Dump the focused pane's rendered history into `$EDITOR` (040, ADR-0029):
    /// leader → `e`. The app suspends the loop, shells out to stock `rmux
    /// capture-pane`, runs the editor, and restores the TUI.
    OpenEditor,
    /// Discard the modal buffer.
    ModalCancel,
    /// Quit the TUI.
    Quit,
}

/// The pure transition. Given the current focus, the configured leader, and one
/// crossterm event, return the next focus and the action to perform.
pub fn arbitrate(focus: &Focus, leader: &Leader, ev: &Event) -> (Focus, Action) {
    match focus {
        Focus::Pane => arbitrate_pane(leader, ev),
        Focus::Detail => arbitrate_detail(leader, ev),
        Focus::Nav => arbitrate_nav(leader, ev),
        Focus::Modal { kind, prior } => arbitrate_modal(*kind, prior, ev),
        Focus::LeaderPending { prior } => arbitrate_pending(prior, ev),
    }
}

/// A focused foreign pane: forward every key but the leader (which opens the
/// dispatch gate, remembering we came from the pane). Mouse/paste forward too.
fn arbitrate_pane(leader: &Leader, ev: &Event) -> (Focus, Action) {
    match ev {
        Event::Key(key) if is_press(key) => {
            if leader.matches(key) {
                return (
                    Focus::LeaderPending {
                        prior: Box::new(Focus::Pane),
                    },
                    Action::Redraw,
                );
            }
            let action = match map_key(key) {
                KeyToken::Text(s) => Action::SendText(s),
                KeyToken::Key(k) => Action::SendKey(k),
                KeyToken::Unmapped => Action::Ignore,
            };
            (Focus::Pane, action)
        }
        Event::Paste(text) => (Focus::Pane, Action::SendPaste(text.clone())),
        Event::Mouse(m) => {
            if let MouseEventKind::Down(MouseButton::Left) = m.kind {
                (
                    Focus::Pane,
                    Action::HarnessClick {
                        row: m.row,
                        col: m.column,
                    },
                )
            } else {
                // Rich mouse passthrough (drag/wheel/motion) is deferred to 050
                // (likely a rmux raw-mouse capability) — do not build a lossy
                // automation-call translator here.
                (Focus::Pane, Action::Ignore)
            }
        }
        _ => (Focus::Pane, Action::Ignore),
    }
}

/// The per-grove detail panel. The leader opens the gate (remembering Detail);
/// `Esc` returns to the home pane; `j`/`k` (or arrows) scroll the content. Any
/// other key is inert (not forwarded to the pane — detail owns focus while up).
fn arbitrate_detail(leader: &Leader, ev: &Event) -> (Focus, Action) {
    match ev {
        Event::Key(key) if is_press(key) => {
            if leader.matches(key) {
                return (
                    Focus::LeaderPending {
                        prior: Box::new(Focus::Detail),
                    },
                    Action::Redraw,
                );
            }
            match key.code {
                KeyCode::Esc => (Focus::Pane, Action::Redraw),
                KeyCode::Down | KeyCode::Char('j') => (Focus::Detail, Action::DetailScrollDown),
                KeyCode::Up | KeyCode::Char('k') => (Focus::Detail, Action::DetailScrollUp),
                _ => (Focus::Detail, Action::Ignore),
            }
        }
        _ => (Focus::Detail, Action::Ignore),
    }
}

/// grove's grove-list surface. In-surface keys (`j`/`k`/arrows move, `Enter`
/// opens) stay; `Esc` returns to the home pane; the leader opens the gate. The
/// former leader-prefixed actions (`c`/`e`/`q`) have moved onto the gate, so they
/// are inert here now.
fn arbitrate_nav(leader: &Leader, ev: &Event) -> (Focus, Action) {
    match ev {
        Event::Key(key) if is_press(key) => {
            if leader.matches(key) {
                return (
                    Focus::LeaderPending {
                        prior: Box::new(Focus::Nav),
                    },
                    Action::Redraw,
                );
            }
            match key.code {
                KeyCode::Esc => (Focus::Pane, Action::Redraw),
                // List navigation: arrows or vim j/k. The selection lives in the
                // app's `Nav`; arbitrate only emits the movement intent.
                KeyCode::Up | KeyCode::Char('k') => (Focus::Nav, Action::NavUp),
                KeyCode::Down | KeyCode::Char('j') => (Focus::Nav, Action::NavDown),
                // Open/focus the selected grove's harness, landing on the pane.
                KeyCode::Enter => (Focus::Pane, Action::NavSelect),
                _ => (Focus::Nav, Action::Ignore),
            }
        }
        _ => (Focus::Nav, Action::Ignore),
    }
}

/// The transient dispatch gate: the next key press routes, then we leave. Any key
/// without a route (including `Esc`) cancels back to `prior`, so the gate never
/// traps the user. Non-press / non-key events keep the gate open.
fn arbitrate_pending(prior: &Focus, ev: &Event) -> (Focus, Action) {
    let stay = || {
        (
            Focus::LeaderPending {
                prior: Box::new(prior.clone()),
            },
            Action::Ignore,
        )
    };
    let cancel = || (prior.clone(), Action::Redraw);
    match ev {
        Event::Key(key) if is_press(key) => match key.code {
            KeyCode::Char('g') => (Focus::Nav, Action::Redraw),
            KeyCode::Char('d') => (Focus::Detail, Action::Redraw),
            // The capture modal restores the surface we leadered from on cancel.
            KeyCode::Char('c') => (
                Focus::Modal {
                    kind: ModalKind::Capture,
                    prior: Box::new(prior.clone()),
                },
                Action::Redraw,
            ),
            // Open-in-editor / quit return focus to the prior surface (the editor
            // and quit operate on the app, not on which surface is focused).
            KeyCode::Char('e') => (prior.clone(), Action::OpenEditor),
            KeyCode::Char('q') => (prior.clone(), Action::Quit),
            // Esc and any unmapped key cancel the gate (redraw to clear the menu).
            _ => cancel(),
        },
        // A non-press key (kitty release) or a non-key event leaves the gate open.
        _ => stay(),
    }
}

fn arbitrate_modal(kind: ModalKind, prior: &Focus, ev: &Event) -> (Focus, Action) {
    let stay = |action| {
        (
            Focus::Modal {
                kind,
                prior: Box::new(prior.clone()),
            },
            action,
        )
    };
    match ev {
        Event::Key(key) if is_press(key) => match key.code {
            KeyCode::Esc => (prior.clone(), Action::ModalCancel),
            KeyCode::Enter => (prior.clone(), Action::ModalSubmit),
            KeyCode::Backspace => stay(Action::ModalBackspace),
            // Literal text into the buffer; modifier chords are swallowed (the
            // modal owns focus, like grove's capture).
            KeyCode::Char(c) if is_plain_char(key) => stay(Action::ModalInsert(c.to_string())),
            _ => stay(Action::Ignore),
        },
        // Paste lands in the buffer literally (not bracketed-wrapped — there is
        // no pane to forward to while the modal is up).
        Event::Paste(text) => stay(Action::ModalInsert(text.clone())),
        _ => stay(Action::Ignore),
    }
}

/// Only Press/Repeat events drive the machine; Release events (kitty progressive
/// enhancement) must not double-send or re-transition.
fn is_press(key: &KeyEvent) -> bool {
    matches!(key.kind, KeyEventKind::Press | KeyEventKind::Repeat)
}

/// A printable char with no Ctrl/Alt (Shift is already in the glyph) — the chars
/// that should land literally in a modal buffer.
fn is_plain_char(key: &KeyEvent) -> bool {
    use ratatui::crossterm::event::KeyModifiers;
    !key.modifiers.contains(KeyModifiers::CONTROL) && !key.modifiers.contains(KeyModifiers::ALT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::crossterm::event::{KeyModifiers, MouseEvent};

    fn leader() -> Leader {
        Leader::alt_g()
    }

    fn key_ev(code: KeyCode, mods: KeyModifiers) -> Event {
        Event::Key(KeyEvent::new(code, mods))
    }

    fn leader_ev() -> Event {
        key_ev(KeyCode::Char('g'), KeyModifiers::ALT)
    }

    fn release_ev(code: KeyCode) -> Event {
        Event::Key(KeyEvent::new_with_kind(
            code,
            KeyModifiers::NONE,
            KeyEventKind::Release,
        ))
    }

    fn left_click(row: u16, col: u16) -> Event {
        Event::Mouse(MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            row,
            column: col,
            modifiers: KeyModifiers::NONE,
        })
    }

    fn pending_over(prior: Focus) -> Focus {
        Focus::LeaderPending {
            prior: Box::new(prior),
        }
    }

    fn capture_over(prior: Focus) -> Focus {
        Focus::Modal {
            kind: ModalKind::Capture,
            prior: Box::new(prior),
        }
    }

    // --- Pane focus (the former Harness, generalised) ----------------------

    #[test]
    fn pane_forwards_plain_char_as_text() {
        let (focus, action) =
            arbitrate(&Focus::Pane, &leader(), &key_ev(KeyCode::Char('a'), KeyModifiers::NONE));
        assert_eq!(focus, Focus::Pane);
        assert_eq!(action, Action::SendText("a".into()));
    }

    #[test]
    fn pane_forwards_special_key_as_token() {
        let (_, action) =
            arbitrate(&Focus::Pane, &leader(), &key_ev(KeyCode::Up, KeyModifiers::CONTROL));
        assert_eq!(action, Action::SendKey("C-Up".into()));
    }

    #[test]
    fn pane_leader_enters_leader_pending_remembering_the_pane() {
        let (focus, action) = arbitrate(&Focus::Pane, &leader(), &leader_ev());
        assert_eq!(focus, pending_over(Focus::Pane));
        assert_eq!(action, Action::Redraw);
    }

    #[test]
    fn pane_paste_is_forwarded_wrapped() {
        let (focus, action) =
            arbitrate(&Focus::Pane, &leader(), &Event::Paste("line1\nline2".into()));
        assert_eq!(focus, Focus::Pane);
        assert_eq!(action, Action::SendPaste("line1\nline2".into()));
    }

    #[test]
    fn pane_left_click_forwards_a_click() {
        let (focus, action) = arbitrate(&Focus::Pane, &leader(), &left_click(4, 7));
        assert_eq!(focus, Focus::Pane);
        assert_eq!(action, Action::HarnessClick { row: 4, col: 7 });
    }

    #[test]
    fn pane_ignores_key_release() {
        let (focus, action) = arbitrate(&Focus::Pane, &leader(), &release_ev(KeyCode::Char('a')));
        assert_eq!(focus, Focus::Pane);
        assert_eq!(action, Action::Ignore);
    }

    // --- LeaderPending: the dispatch gate ----------------------------------

    #[test]
    fn pending_g_dispatches_to_nav() {
        let (focus, action) = arbitrate(
            &pending_over(Focus::Pane),
            &leader(),
            &key_ev(KeyCode::Char('g'), KeyModifiers::NONE),
        );
        assert_eq!(focus, Focus::Nav);
        assert_eq!(action, Action::Redraw);
    }

    #[test]
    fn pending_d_dispatches_to_detail() {
        let (focus, action) = arbitrate(
            &pending_over(Focus::Pane),
            &leader(),
            &key_ev(KeyCode::Char('d'), KeyModifiers::NONE),
        );
        assert_eq!(focus, Focus::Detail);
        assert_eq!(action, Action::Redraw);
    }

    #[test]
    fn pending_c_opens_capture_modal_restoring_the_prior_surface() {
        // Leadered from Detail → the capture modal's prior is Detail, so cancel
        // returns there, not to the pane.
        let (focus, action) = arbitrate(
            &pending_over(Focus::Detail),
            &leader(),
            &key_ev(KeyCode::Char('c'), KeyModifiers::NONE),
        );
        assert_eq!(focus, capture_over(Focus::Detail));
        assert_eq!(action, Action::Redraw);
    }

    #[test]
    fn pending_e_opens_editor_and_returns_to_prior() {
        let (focus, action) = arbitrate(
            &pending_over(Focus::Pane),
            &leader(),
            &key_ev(KeyCode::Char('e'), KeyModifiers::NONE),
        );
        assert_eq!(focus, Focus::Pane);
        assert_eq!(action, Action::OpenEditor);
    }

    #[test]
    fn pending_q_quits() {
        let (_, action) = arbitrate(
            &pending_over(Focus::Pane),
            &leader(),
            &key_ev(KeyCode::Char('q'), KeyModifiers::NONE),
        );
        assert_eq!(action, Action::Quit);
    }

    #[test]
    fn pending_esc_cancels_back_to_prior() {
        let (focus, action) = arbitrate(
            &pending_over(Focus::Nav),
            &leader(),
            &key_ev(KeyCode::Esc, KeyModifiers::NONE),
        );
        assert_eq!(focus, Focus::Nav, "cancel restores the surface we leadered from");
        assert_eq!(action, Action::Redraw, "redraw to clear the menu footer");
    }

    #[test]
    fn pending_unmapped_key_cancels_back_to_prior() {
        // A key with no dispatch meaning does not trap the user — it cancels.
        let (focus, action) = arbitrate(
            &pending_over(Focus::Pane),
            &leader(),
            &key_ev(KeyCode::Char('z'), KeyModifiers::NONE),
        );
        assert_eq!(focus, Focus::Pane);
        assert_eq!(action, Action::Redraw);
    }

    #[test]
    fn pending_ignores_key_release_and_stays_pending() {
        // A release event (kitty) must not be mistaken for a dispatch key.
        let start = pending_over(Focus::Pane);
        let (focus, action) = arbitrate(&start, &leader(), &release_ev(KeyCode::Char('g')));
        assert_eq!(focus, start);
        assert_eq!(action, Action::Ignore);
    }

    // --- Detail focus (transition wired; widget is 030) --------------------

    #[test]
    fn detail_leader_enters_pending_remembering_detail() {
        let (focus, action) = arbitrate(&Focus::Detail, &leader(), &leader_ev());
        assert_eq!(focus, pending_over(Focus::Detail));
        assert_eq!(action, Action::Redraw);
    }

    #[test]
    fn detail_esc_returns_to_pane() {
        let (focus, action) =
            arbitrate(&Focus::Detail, &leader(), &key_ev(KeyCode::Esc, KeyModifiers::NONE));
        assert_eq!(focus, Focus::Pane);
        assert_eq!(action, Action::Redraw);
    }

    #[test]
    fn detail_jk_and_arrows_scroll_the_widget() {
        for code in [KeyCode::Down, KeyCode::Char('j')] {
            let (focus, action) = arbitrate(&Focus::Detail, &leader(), &key_ev(code, KeyModifiers::NONE));
            assert_eq!(focus, Focus::Detail);
            assert_eq!(action, Action::DetailScrollDown);
        }
        for code in [KeyCode::Up, KeyCode::Char('k')] {
            let (focus, action) = arbitrate(&Focus::Detail, &leader(), &key_ev(code, KeyModifiers::NONE));
            assert_eq!(focus, Focus::Detail);
            assert_eq!(action, Action::DetailScrollUp);
        }
    }

    #[test]
    fn detail_swallows_unmapped_keys() {
        // A key with no in-surface meaning is inert (not forwarded to the pane).
        let (focus, action) =
            arbitrate(&Focus::Detail, &leader(), &key_ev(KeyCode::Char('x'), KeyModifiers::NONE));
        assert_eq!(focus, Focus::Detail);
        assert_eq!(action, Action::Ignore);
    }

    // --- Nav focus ---------------------------------------------------------

    #[test]
    fn nav_leader_enters_pending_remembering_nav() {
        let (focus, action) = arbitrate(&Focus::Nav, &leader(), &leader_ev());
        assert_eq!(focus, pending_over(Focus::Nav));
        assert_eq!(action, Action::Redraw);
    }

    #[test]
    fn nav_esc_returns_to_pane() {
        let (focus, action) =
            arbitrate(&Focus::Nav, &leader(), &key_ev(KeyCode::Esc, KeyModifiers::NONE));
        assert_eq!(focus, Focus::Pane);
        assert_eq!(action, Action::Redraw);
    }

    #[test]
    fn nav_arrows_and_jk_move_the_selection() {
        for code in [KeyCode::Up, KeyCode::Char('k')] {
            let (focus, action) = arbitrate(&Focus::Nav, &leader(), &key_ev(code, KeyModifiers::NONE));
            assert_eq!(focus, Focus::Nav);
            assert_eq!(action, Action::NavUp);
        }
        for code in [KeyCode::Down, KeyCode::Char('j')] {
            let (focus, action) = arbitrate(&Focus::Nav, &leader(), &key_ev(code, KeyModifiers::NONE));
            assert_eq!(focus, Focus::Nav);
            assert_eq!(action, Action::NavDown);
        }
    }

    #[test]
    fn nav_enter_selects_and_lands_on_pane() {
        let (focus, action) =
            arbitrate(&Focus::Nav, &leader(), &key_ev(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(focus, Focus::Pane);
        assert_eq!(action, Action::NavSelect);
    }

    #[test]
    fn nav_no_longer_handles_leader_prefixed_actions_directly() {
        // c / e / q moved onto the dispatch gate; in Nav they are plain,
        // unhandled keys now (the gate owns those routes).
        for code in [KeyCode::Char('c'), KeyCode::Char('e'), KeyCode::Char('q')] {
            let (focus, action) = arbitrate(&Focus::Nav, &leader(), &key_ev(code, KeyModifiers::NONE));
            assert_eq!(focus, Focus::Nav);
            assert_eq!(action, Action::Ignore, "{code:?} should be inert in Nav now");
        }
    }

    // --- Modal focus -------------------------------------------------------

    #[test]
    fn modal_char_inserts_into_buffer_and_stays() {
        let (focus, action) = arbitrate(
            &capture_over(Focus::Nav),
            &leader(),
            &key_ev(KeyCode::Char('h'), KeyModifiers::NONE),
        );
        assert_eq!(focus, capture_over(Focus::Nav));
        assert_eq!(action, Action::ModalInsert("h".into()));
    }

    #[test]
    fn modal_esc_cancels_and_restores_prior() {
        let (focus, action) = arbitrate(
            &capture_over(Focus::Nav),
            &leader(),
            &key_ev(KeyCode::Esc, KeyModifiers::NONE),
        );
        assert_eq!(focus, Focus::Nav);
        assert_eq!(action, Action::ModalCancel);
    }

    #[test]
    fn modal_enter_submits_and_restores_prior() {
        let (focus, action) = arbitrate(
            &capture_over(Focus::Pane),
            &leader(),
            &key_ev(KeyCode::Enter, KeyModifiers::NONE),
        );
        assert_eq!(focus, Focus::Pane);
        assert_eq!(action, Action::ModalSubmit);
    }

    #[test]
    fn modal_backspace_edits_buffer() {
        let (_, action) = arbitrate(
            &capture_over(Focus::Nav),
            &leader(),
            &key_ev(KeyCode::Backspace, KeyModifiers::NONE),
        );
        assert_eq!(action, Action::ModalBackspace);
    }

    #[test]
    fn modal_paste_inserts_literally() {
        let (focus, action) = arbitrate(
            &capture_over(Focus::Nav),
            &leader(),
            &Event::Paste("pasted".into()),
        );
        assert_eq!(focus, capture_over(Focus::Nav));
        assert_eq!(action, Action::ModalInsert("pasted".into()));
    }

    #[test]
    fn modal_swallows_the_leader() {
        // The modal owns focus; the leader is just another swallowed key (it does
        // NOT open the dispatch gate while a modal is up).
        let (focus, action) = arbitrate(&capture_over(Focus::Nav), &leader(), &leader_ev());
        assert_eq!(focus, capture_over(Focus::Nav));
        assert_eq!(action, Action::Ignore);
    }
}
