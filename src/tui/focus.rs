//! The focus state machine (E4): grove-owned, leader-gated input arbitration.
//!
//! grove owns the draw loop, so there is no zellij locked-mode — grove is the
//! arbiter by construction and sees every crossterm event first. [`arbitrate`] is
//! a **pure function** `(Focus, Leader, Event) → (Focus, Action)`: the
//! headless-testable transition table the brief calls for. The app
//! ([`crate::tui::app`]) owns the side effects — it applies the returned
//! [`Action`] (forward to the pane, mutate the modal buffer, redraw, quit) and
//! adopts the returned [`Focus`].
//!
//! ## The model (E4)
//!
//! - **[`Focus::Harness`]** — the live pane is focused. Forward *every* key to it
//!   except the leader (maximising harness key fidelity); the leader flips to
//!   [`Focus::Nav`].
//! - **[`Focus::Nav`]** — grove's command surface. This leaf ships it as a
//!   **stub** (the real nav is 030): a "grove focus" indicator with three
//!   bindings — `Esc`/leader return to the harness, `q` quits, `c` opens the
//!   capture modal. 030 replaces the body; the *arbitration* is what this leaf
//!   nails down.
//! - **[`Focus::Modal`]** — a focus overlay that captures all keys (text into its
//!   buffer), restoring the **prior** focus on `Esc` (cancel) or `Enter`
//!   (submit). The real capture modal is 040; here it proves the capture +
//!   restore mechanics.
//!
//! ## Why quit moved (vs 010)
//!
//! 010 quit on a global `Ctrl-Q`. Under leader-gating that key belongs to the
//! harness (vim/claude lean on Ctrl-chords; every stolen key is a fidelity loss),
//! so quit is now a grove-surface action: leader → `q`.

use ratatui::crossterm::event::{
    Event, KeyCode, KeyEvent, KeyEventKind, MouseButton, MouseEventKind,
};

use crate::tui::config::Leader;
use crate::tui::input::{map_key, KeyToken};

/// Which kind of modal is up. Only [`ModalKind::Capture`] exists today (040
/// builds the real capture modal); the enum is here so 040/050 can add modal
/// kinds without reshaping the focus type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalKind {
    /// The capture modal (the 040 proof point).
    Capture,
}

/// Which surface currently owns keyboard input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Focus {
    /// The live harness pane (forward everything but the leader).
    Harness,
    /// grove's command surface (stub this leaf).
    Nav,
    /// A focus overlay; restores `prior` on cancel/submit.
    Modal {
        kind: ModalKind,
        prior: Box<Focus>,
    },
}

/// What the app should do in response to one input event. The transition table
/// is pure; this enum is the instruction the impure app layer carries out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Nothing to do (and nothing changed).
    Ignore,
    /// Forward literal text to the focused harness pane (`send_text`).
    SendText(String),
    /// Forward a tmux key token to the focused harness pane (`send_key`).
    SendKey(String),
    /// Forward a bracketed paste to the harness — the app wraps it in
    /// `\e[200~…\e[201~` so multi-line pastes don't execute line-by-line.
    SendPaste(String),
    /// Forward a left-click to the harness at a (row, col) pane cell.
    HarnessClick { row: u16, col: u16 },
    /// A grove surface changed; the app should redraw.
    Redraw,
    /// Insert literal text into the focused modal's buffer.
    ModalInsert(String),
    /// Delete the last char of the modal buffer.
    ModalBackspace,
    /// Submit the modal buffer (040 wires this to grove's capture write).
    ModalSubmit,
    /// Discard the modal buffer.
    ModalCancel,
    /// Quit the TUI.
    Quit,
}

/// The pure transition. Given the current focus, the configured leader, and one
/// crossterm event, return the next focus and the action to perform.
pub fn arbitrate(focus: &Focus, leader: &Leader, ev: &Event) -> (Focus, Action) {
    match focus {
        Focus::Harness => arbitrate_harness(leader, ev),
        Focus::Nav => arbitrate_nav(leader, ev),
        Focus::Modal { kind, prior } => arbitrate_modal(*kind, prior, ev),
    }
}

fn arbitrate_harness(leader: &Leader, ev: &Event) -> (Focus, Action) {
    match ev {
        Event::Key(key) if is_press(key) => {
            if leader.matches(key) {
                return (Focus::Nav, Action::Redraw);
            }
            let action = match map_key(key) {
                KeyToken::Text(s) => Action::SendText(s),
                KeyToken::Key(k) => Action::SendKey(k),
                KeyToken::Unmapped => Action::Ignore,
            };
            (Focus::Harness, action)
        }
        Event::Paste(text) => (Focus::Harness, Action::SendPaste(text.clone())),
        Event::Mouse(m) => {
            if let MouseEventKind::Down(MouseButton::Left) = m.kind {
                (
                    Focus::Harness,
                    Action::HarnessClick {
                        row: m.row,
                        col: m.column,
                    },
                )
            } else {
                // Rich mouse passthrough (drag/wheel/motion) is deferred to 050
                // (likely a rmux-fork raw-mouse capability) — do not build a
                // lossy automation-call translator here.
                (Focus::Harness, Action::Ignore)
            }
        }
        _ => (Focus::Harness, Action::Ignore),
    }
}

fn arbitrate_nav(leader: &Leader, ev: &Event) -> (Focus, Action) {
    match ev {
        Event::Key(key) if is_press(key) => {
            if leader.matches(key) || key.code == KeyCode::Esc {
                return (Focus::Harness, Action::Redraw);
            }
            // Stub bindings (030 replaces the nav body): q quits, c opens the
            // capture modal restoring back into Nav.
            match key.code {
                KeyCode::Char('q') => (Focus::Nav, Action::Quit),
                KeyCode::Char('c') => (
                    Focus::Modal {
                        kind: ModalKind::Capture,
                        prior: Box::new(Focus::Nav),
                    },
                    Action::Redraw,
                ),
                _ => (Focus::Nav, Action::Ignore),
            }
        }
        _ => (Focus::Nav, Action::Ignore),
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

    // --- Harness focus -----------------------------------------------------

    #[test]
    fn harness_forwards_plain_char_as_text() {
        let (focus, action) =
            arbitrate(&Focus::Harness, &leader(), &key_ev(KeyCode::Char('a'), KeyModifiers::NONE));
        assert_eq!(focus, Focus::Harness);
        assert_eq!(action, Action::SendText("a".into()));
    }

    #[test]
    fn harness_forwards_special_key_as_token() {
        let (_, action) =
            arbitrate(&Focus::Harness, &leader(), &key_ev(KeyCode::Up, KeyModifiers::CONTROL));
        assert_eq!(action, Action::SendKey("C-Up".into()));
    }

    #[test]
    fn harness_leader_flips_to_nav() {
        let (focus, action) =
            arbitrate(&Focus::Harness, &leader(), &key_ev(KeyCode::Char('g'), KeyModifiers::ALT));
        assert_eq!(focus, Focus::Nav);
        assert_eq!(action, Action::Redraw);
    }

    #[test]
    fn harness_paste_is_forwarded_wrapped() {
        let (focus, action) = arbitrate(
            &Focus::Harness,
            &leader(),
            &Event::Paste("line1\nline2".into()),
        );
        assert_eq!(focus, Focus::Harness);
        assert_eq!(action, Action::SendPaste("line1\nline2".into()));
    }

    #[test]
    fn harness_left_click_forwards_a_click() {
        let (focus, action) = arbitrate(&Focus::Harness, &leader(), &left_click(4, 7));
        assert_eq!(focus, Focus::Harness);
        assert_eq!(action, Action::HarnessClick { row: 4, col: 7 });
    }

    #[test]
    fn harness_ignores_key_release() {
        let (focus, action) = arbitrate(&Focus::Harness, &leader(), &release_ev(KeyCode::Char('a')));
        assert_eq!(focus, Focus::Harness);
        assert_eq!(action, Action::Ignore);
    }

    // --- Nav focus (stub) --------------------------------------------------

    #[test]
    fn nav_esc_returns_to_harness() {
        let (focus, action) =
            arbitrate(&Focus::Nav, &leader(), &key_ev(KeyCode::Esc, KeyModifiers::NONE));
        assert_eq!(focus, Focus::Harness);
        assert_eq!(action, Action::Redraw);
    }

    #[test]
    fn nav_leader_toggles_back_to_harness() {
        let (focus, _) =
            arbitrate(&Focus::Nav, &leader(), &key_ev(KeyCode::Char('g'), KeyModifiers::ALT));
        assert_eq!(focus, Focus::Harness);
    }

    #[test]
    fn nav_q_quits() {
        let (_, action) =
            arbitrate(&Focus::Nav, &leader(), &key_ev(KeyCode::Char('q'), KeyModifiers::NONE));
        assert_eq!(action, Action::Quit);
    }

    #[test]
    fn nav_c_opens_capture_modal_with_nav_as_prior() {
        let (focus, action) =
            arbitrate(&Focus::Nav, &leader(), &key_ev(KeyCode::Char('c'), KeyModifiers::NONE));
        assert_eq!(
            focus,
            Focus::Modal {
                kind: ModalKind::Capture,
                prior: Box::new(Focus::Nav),
            }
        );
        assert_eq!(action, Action::Redraw);
    }

    // --- Modal focus -------------------------------------------------------

    fn capture_over(prior: Focus) -> Focus {
        Focus::Modal {
            kind: ModalKind::Capture,
            prior: Box::new(prior),
        }
    }

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
            &capture_over(Focus::Harness),
            &leader(),
            &key_ev(KeyCode::Enter, KeyModifiers::NONE),
        );
        assert_eq!(focus, Focus::Harness);
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
        // The modal owns focus; the leader is just another swallowed key.
        let (focus, action) = arbitrate(
            &capture_over(Focus::Nav),
            &leader(),
            &key_ev(KeyCode::Char('g'), KeyModifiers::ALT),
        );
        assert_eq!(focus, capture_over(Focus::Nav));
        assert_eq!(action, Action::Ignore);
    }
}
