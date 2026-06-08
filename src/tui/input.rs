//! The crossterm → tmux-token key-map (E5).
//!
//! [`map_key`] is a **pure function** over a `crossterm::KeyEvent`: it is the
//! headless-testable seam the brief calls for — no terminal, no daemon. The app
//! ([`crate::tui::app`]) applies its result by calling `pane.send_text` /
//! `pane.send_key`. It extends the spike's `forward_key` to the full modifier
//! matrix on special keys (`C-Left`, `S-Up`, `C-Enter`, …) with correct Shift
//! handling.
//!
//! ## Mapping rules
//!
//! - **Plain printables** → [`KeyToken::Text`] (sent via `send_text`, no implicit
//!   newline). Shift is already baked into the char by crossterm (it reports the
//!   uppercase glyph), so a shifted printable carries **no** `S-` prefix — adding
//!   one would double-apply Shift.
//! - **Ctrl/Alt-modified printables and all special keys** → [`KeyToken::Key`], a
//!   tmux `send-keys` token. Modifier prefixes are emitted in the fixed order
//!   `C-`, `M-`, `S-` (e.g. `C-S-Left`), which the daemon's tmux-compatible
//!   `send-keys` parser accepts.
//! - **Genuinely unmappable keys** → [`KeyToken::Unmapped`] (dropped). See the
//!   *known gaps* list below — these are the keys 050 must revisit.
//!
//! ## Known gaps (for 050)
//!
//! Faithful forwarding is the goal (every dropped key is a fidelity loss), but a
//! few crossterm inputs have no clean tmux token today and are dropped:
//!   - `Super`/`Hyper`/`Meta` modifiers (only Ctrl/Alt/Shift map; macOS Cmd is
//!     Super). A `Cmd-c` arrives as plain `c`'s token without the Super bit.
//!   - Media/Menu/CapsLock/ScrollLock/NumLock/PrintScreen/Pause and the
//!     `KeyCode::Modifier` bare-modifier events — no shell/editor token.
//!   - `KeyCode::Null`.
//!
//! Rich keyboard protocols (kitty progressive enhancement: key-release events,
//! disambiguated modifiers) are out of scope; we forward Press/Repeat only and
//! the app filters Release.

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// The result of mapping one key event: either literal text to send verbatim, a
/// tmux key token to interpret, or nothing (an unmappable key — a known gap).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyToken {
    /// Literal UTF-8 to forward via `send_text` (no implicit newline).
    Text(String),
    /// A tmux `send-keys` token to forward via `send_key`.
    Key(String),
    /// No faithful mapping exists; the key is dropped (see *known gaps*).
    Unmapped,
}

/// Map one crossterm key event to a [`KeyToken`]. Pure — no I/O.
pub fn map_key(key: &KeyEvent) -> KeyToken {
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
    let alt = key.modifiers.contains(KeyModifiers::ALT);
    let shift = key.modifiers.contains(KeyModifiers::SHIFT);

    match key.code {
        // Plain printable, no Ctrl/Alt: send as literal text. Shift is already
        // reflected in `c` (crossterm reports the uppercase glyph), so it is not
        // re-encoded as an `S-` prefix.
        KeyCode::Char(c) if !ctrl && !alt => KeyToken::Text(c.to_string()),
        // Ctrl/Alt-modified printable: a tmux char token. Ctrl lowercases the
        // letter (`C-c`, not `C-C`); Shift only carries when neither Ctrl nor
        // the already-applied case covers it — for letters Shift is in the glyph,
        // so we drop a redundant `S-` on Ctrl/Alt chars.
        KeyCode::Char(c) => {
            let base = if ctrl {
                c.to_ascii_lowercase().to_string()
            } else {
                c.to_string()
            };
            KeyToken::Key(with_modifiers(ctrl, alt, false, &base))
        }
        // Special keys carry their modifiers as a prefix (Shift included here,
        // since crossterm reports it separately for non-char keys).
        KeyCode::Enter => special(ctrl, alt, shift, "Enter"),
        KeyCode::Esc => special(ctrl, alt, shift, "Escape"),
        KeyCode::Tab => special(ctrl, alt, shift, "Tab"),
        KeyCode::BackTab => special(ctrl, alt, shift, "BTab"),
        KeyCode::Backspace => special(ctrl, alt, shift, "BSpace"),
        KeyCode::Up => special(ctrl, alt, shift, "Up"),
        KeyCode::Down => special(ctrl, alt, shift, "Down"),
        KeyCode::Left => special(ctrl, alt, shift, "Left"),
        KeyCode::Right => special(ctrl, alt, shift, "Right"),
        KeyCode::Home => special(ctrl, alt, shift, "Home"),
        KeyCode::End => special(ctrl, alt, shift, "End"),
        KeyCode::PageUp => special(ctrl, alt, shift, "PgUp"),
        KeyCode::PageDown => special(ctrl, alt, shift, "PgDn"),
        KeyCode::Delete => special(ctrl, alt, shift, "DC"),
        KeyCode::Insert => special(ctrl, alt, shift, "IC"),
        KeyCode::F(n) => special(ctrl, alt, shift, &format!("F{n}")),
        // No clean token — a known gap (see module docs).
        _ => KeyToken::Unmapped,
    }
}

/// Build a special-key [`KeyToken::Key`] with its modifier prefix.
fn special(ctrl: bool, alt: bool, shift: bool, name: &str) -> KeyToken {
    KeyToken::Key(with_modifiers(ctrl, alt, shift, name))
}

/// Prefix `base` with tmux modifier flags in the fixed order `C-`, `M-`, `S-`.
fn with_modifiers(ctrl: bool, alt: bool, shift: bool, base: &str) -> String {
    let mut token = String::new();
    if ctrl {
        token.push_str("C-");
    }
    if alt {
        token.push_str("M-");
    }
    if shift {
        token.push_str("S-");
    }
    token.push_str(base);
    token
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(code: KeyCode, mods: KeyModifiers) -> KeyEvent {
        KeyEvent::new(code, mods)
    }

    #[test]
    fn plain_printable_maps_to_text() {
        assert_eq!(
            map_key(&key(KeyCode::Char('a'), KeyModifiers::NONE)),
            KeyToken::Text("a".into())
        );
    }

    #[test]
    fn shifted_printable_is_text_without_shift_prefix() {
        // crossterm reports the uppercase glyph + SHIFT; the glyph already
        // carries the shift, so it must stay literal text, not `S-A`.
        assert_eq!(
            map_key(&key(KeyCode::Char('A'), KeyModifiers::SHIFT)),
            KeyToken::Text("A".into())
        );
    }

    #[test]
    fn ctrl_char_lowercases_into_a_tmux_token() {
        assert_eq!(
            map_key(&key(KeyCode::Char('C'), KeyModifiers::CONTROL)),
            KeyToken::Key("C-c".into())
        );
    }

    #[test]
    fn alt_char_maps_to_meta_token() {
        assert_eq!(
            map_key(&key(KeyCode::Char('g'), KeyModifiers::ALT)),
            KeyToken::Key("M-g".into())
        );
    }

    #[test]
    fn ctrl_alt_char_combines_prefixes_in_order() {
        assert_eq!(
            map_key(&key(
                KeyCode::Char('x'),
                KeyModifiers::CONTROL | KeyModifiers::ALT
            )),
            KeyToken::Key("C-M-x".into())
        );
    }

    #[test]
    fn bare_special_keys_map_to_their_tokens() {
        assert_eq!(
            map_key(&key(KeyCode::Up, KeyModifiers::NONE)),
            KeyToken::Key("Up".into())
        );
        assert_eq!(
            map_key(&key(KeyCode::Enter, KeyModifiers::NONE)),
            KeyToken::Key("Enter".into())
        );
        assert_eq!(
            map_key(&key(KeyCode::F(3), KeyModifiers::NONE)),
            KeyToken::Key("F3".into())
        );
    }

    #[test]
    fn shift_on_special_key_carries_as_prefix() {
        // Unlike printables, Shift is reported separately for special keys, so it
        // must be encoded.
        assert_eq!(
            map_key(&key(KeyCode::Up, KeyModifiers::SHIFT)),
            KeyToken::Key("S-Up".into())
        );
    }

    #[test]
    fn ctrl_on_special_key_carries_as_prefix() {
        assert_eq!(
            map_key(&key(KeyCode::Left, KeyModifiers::CONTROL)),
            KeyToken::Key("C-Left".into())
        );
    }

    #[test]
    fn full_modifier_matrix_orders_prefixes_c_m_s() {
        assert_eq!(
            map_key(&key(
                KeyCode::Left,
                KeyModifiers::CONTROL | KeyModifiers::ALT | KeyModifiers::SHIFT
            )),
            KeyToken::Key("C-M-S-Left".into())
        );
    }

    #[test]
    fn ctrl_enter_forwards_faithfully_rather_than_dropping() {
        assert_eq!(
            map_key(&key(KeyCode::Enter, KeyModifiers::CONTROL)),
            KeyToken::Key("C-Enter".into())
        );
    }

    #[test]
    fn unmappable_key_is_dropped() {
        assert_eq!(
            map_key(&key(KeyCode::CapsLock, KeyModifiers::NONE)),
            KeyToken::Unmapped
        );
    }
}
