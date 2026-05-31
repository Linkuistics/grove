//! Encoding crossterm input events into the byte sequences a pty child expects.
//!
//! `tui-term` is render-only; turning a `KeyEvent`/`MouseEvent` back into the
//! escape sequences a real terminal would have sent is the consumer's job (the
//! 050 spike's largest piece of wiring). The functions here are pure — they
//! take an event (plus the relevant emulator mode) and return bytes — so they
//! are exhaustively unit-testable without a child.

use ratatui::crossterm::event::{
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};
use vt100::MouseProtocolEncoding;

/// Bytes that open a bracketed-paste run (DECSET 2004).
pub const BRACKETED_PASTE_START: &[u8] = b"\x1b[200~";
/// Bytes that close a bracketed-paste run.
pub const BRACKETED_PASTE_END: &[u8] = b"\x1b[201~";

/// Encode a key press into the bytes a pty child expects, or `None` if there is
/// nothing to send (a key *release*, or an unrepresentable combination).
///
/// `app_cursor` is the emulator's [`application_cursor`](crate::TerminalEmulator::application_cursor)
/// state: under DECCKM arrows and Home/End are sent as SS3 (`ESC O A`) rather
/// than CSI (`ESC [ A`).
pub fn encode_key(key: &KeyEvent, app_cursor: bool) -> Option<Vec<u8>> {
    // crossterm reports Press, Repeat and Release; a pty only wants the first
    // two. (Release events only arrive with the kitty/enhanced protocol, but
    // guarding is cheap and correct.)
    if key.kind == KeyEventKind::Release {
        return None;
    }

    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
    let alt = key.modifiers.contains(KeyModifiers::ALT);
    let modifier = xterm_modifier(key.modifiers);

    let mut out: Vec<u8> = Vec::new();
    match key.code {
        KeyCode::Char(c) => {
            if ctrl {
                let b = ctrl_byte(c)?;
                if alt {
                    out.push(0x1b);
                }
                out.push(b);
            } else {
                if alt {
                    out.push(0x1b);
                }
                let mut buf = [0u8; 4];
                out.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
            }
        }
        KeyCode::Enter => {
            if alt {
                out.push(0x1b);
            }
            out.push(b'\r');
        }
        KeyCode::Tab => {
            if alt {
                out.push(0x1b);
            }
            out.push(b'\t');
        }
        KeyCode::BackTab => out.extend_from_slice(b"\x1b[Z"),
        KeyCode::Backspace => {
            if alt {
                out.push(0x1b);
            }
            out.push(0x7f);
        }
        KeyCode::Esc => out.push(0x1b),
        KeyCode::Up => cursor_seq(&mut out, b'A', app_cursor, modifier),
        KeyCode::Down => cursor_seq(&mut out, b'B', app_cursor, modifier),
        KeyCode::Right => cursor_seq(&mut out, b'C', app_cursor, modifier),
        KeyCode::Left => cursor_seq(&mut out, b'D', app_cursor, modifier),
        KeyCode::Home => cursor_seq(&mut out, b'H', app_cursor, modifier),
        KeyCode::End => cursor_seq(&mut out, b'F', app_cursor, modifier),
        KeyCode::Insert => tilde_seq(&mut out, 2, modifier),
        KeyCode::Delete => tilde_seq(&mut out, 3, modifier),
        KeyCode::PageUp => tilde_seq(&mut out, 5, modifier),
        KeyCode::PageDown => tilde_seq(&mut out, 6, modifier),
        KeyCode::F(n) => function_seq(&mut out, n, modifier),
        _ => return None,
    }

    (!out.is_empty()).then_some(out)
}

/// Encode a mouse event as an SGR (1006) report, gated by the encoding the
/// application requested. Returns `None` for any non-SGR encoding (the legacy
/// X10/UTF-8 encodings are intentionally unimplemented — no modern TUI uses
/// them) and for unrepresentable events.
///
/// The *gating on whether the app wants mouse at all* is the caller's job (via
/// [`wants_mouse`](crate::TerminalEmulator::wants_mouse)); this function only
/// performs the encoding once that gate has opened.
pub fn encode_mouse(ev: &MouseEvent, encoding: MouseProtocolEncoding) -> Option<Vec<u8>> {
    if encoding != MouseProtocolEncoding::Sgr {
        return None;
    }

    // Base button/motion code per the xterm SGR protocol.
    let (mut cb, release): (u32, bool) = match ev.kind {
        MouseEventKind::Down(b) => (button_code(b), false),
        MouseEventKind::Up(b) => (button_code(b), true),
        MouseEventKind::Drag(b) => (button_code(b) + 32, false), // motion bit set
        MouseEventKind::Moved => (35, false),                    // no button + motion
        MouseEventKind::ScrollUp => (64, false),
        MouseEventKind::ScrollDown => (65, false),
        MouseEventKind::ScrollLeft => (66, false),
        MouseEventKind::ScrollRight => (67, false),
    };
    if ev.modifiers.contains(KeyModifiers::SHIFT) {
        cb += 4;
    }
    if ev.modifiers.contains(KeyModifiers::ALT) {
        cb += 8;
    }
    if ev.modifiers.contains(KeyModifiers::CONTROL) {
        cb += 16;
    }

    // SGR coordinates are 1-based; crossterm's are 0-based.
    let x = ev.column as u32 + 1;
    let y = ev.row as u32 + 1;
    let final_byte = if release { 'm' } else { 'M' };
    Some(format!("\x1b[<{cb};{x};{y}{final_byte}").into_bytes())
}

/// Wrap pasted text in bracketed-paste markers when the application enabled it,
/// otherwise return the raw bytes. `bracketed` is the emulator's
/// [`bracketed_paste`](crate::TerminalEmulator::bracketed_paste) state.
pub fn encode_paste(text: &str, bracketed: bool) -> Vec<u8> {
    if !bracketed {
        return text.as_bytes().to_vec();
    }
    let mut out = Vec::with_capacity(text.len() + BRACKETED_PASTE_START.len() + BRACKETED_PASTE_END.len());
    out.extend_from_slice(BRACKETED_PASTE_START);
    out.extend_from_slice(text.as_bytes());
    out.extend_from_slice(BRACKETED_PASTE_END);
    out
}

/// The xterm modifier parameter: `1 + (shift?1) + (alt?2) + (ctrl?4)`. A value
/// of `1` means "no modifiers" and suppresses the `;m` field in CSI sequences.
fn xterm_modifier(mods: KeyModifiers) -> u8 {
    1 + u8::from(mods.contains(KeyModifiers::SHIFT))
        + 2 * u8::from(mods.contains(KeyModifiers::ALT))
        + 4 * u8::from(mods.contains(KeyModifiers::CONTROL))
}

/// Map a printable char under Ctrl to its C0 control byte, or `None` if there
/// is no conventional control code for it.
fn ctrl_byte(c: char) -> Option<u8> {
    match c.to_ascii_lowercase() {
        c @ 'a'..='z' => Some(c as u8 - b'a' + 1), // ^A=0x01 .. ^Z=0x1a
        ' ' | '@' => Some(0x00),                   // ^Space / ^@ = NUL
        '[' => Some(0x1b),                         // ^[ = ESC
        '\\' => Some(0x1c),
        ']' => Some(0x1d),
        '^' => Some(0x1e),
        '_' | '/' => Some(0x1f), // ^_ / ^/ = US
        _ => None,
    }
}

/// Cursor / Home / End keys: CSI vs SS3 by DECCKM, CSI `1;m` form when modified.
fn cursor_seq(out: &mut Vec<u8>, final_byte: u8, app_cursor: bool, modifier: u8) {
    if modifier > 1 {
        out.extend_from_slice(b"\x1b[1;");
        out.extend_from_slice(modifier.to_string().as_bytes());
        out.push(final_byte);
    } else if app_cursor {
        out.extend_from_slice(b"\x1bO");
        out.push(final_byte);
    } else {
        out.extend_from_slice(b"\x1b[");
        out.push(final_byte);
    }
}

/// Tilde-form navigation keys: `ESC [ <num> ~`, or `ESC [ <num> ; <mod> ~`.
fn tilde_seq(out: &mut Vec<u8>, num: u8, modifier: u8) {
    out.extend_from_slice(b"\x1b[");
    out.extend_from_slice(num.to_string().as_bytes());
    if modifier > 1 {
        out.push(b';');
        out.extend_from_slice(modifier.to_string().as_bytes());
    }
    out.push(b'~');
}

/// Function keys: F1–F4 as SS3 `ESC O P..S` (CSI `1;m` form when modified),
/// F5–F12 as tilde codes.
fn function_seq(out: &mut Vec<u8>, n: u8, modifier: u8) {
    match n {
        1..=4 => {
            let final_byte = b'P' + (n - 1); // P, Q, R, S
            if modifier > 1 {
                out.extend_from_slice(b"\x1b[1;");
                out.extend_from_slice(modifier.to_string().as_bytes());
                out.push(final_byte);
            } else {
                out.extend_from_slice(b"\x1bO");
                out.push(final_byte);
            }
        }
        5..=12 => {
            let code = match n {
                5 => 15,
                6 => 17,
                7 => 18,
                8 => 19,
                9 => 20,
                10 => 21,
                11 => 23,
                12 => 24,
                _ => unreachable!(),
            };
            tilde_seq(out, code, modifier);
        }
        _ => {}
    }
}

fn button_code(b: MouseButton) -> u32 {
    match b {
        MouseButton::Left => 0,
        MouseButton::Middle => 1,
        MouseButton::Right => 2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::crossterm::event::{KeyEventState, MouseEvent};

    fn key(code: KeyCode, mods: KeyModifiers) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: mods,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    }

    #[test]
    fn plain_char_is_its_utf8() {
        assert_eq!(encode_key(&key(KeyCode::Char('a'), KeyModifiers::NONE), false), Some(b"a".to_vec()));
        assert_eq!(encode_key(&key(KeyCode::Char('λ'), KeyModifiers::NONE), false), Some("λ".as_bytes().to_vec()));
    }

    #[test]
    fn ctrl_letters_map_to_c0() {
        assert_eq!(encode_key(&key(KeyCode::Char('c'), KeyModifiers::CONTROL), false), Some(vec![0x03]));
        assert_eq!(encode_key(&key(KeyCode::Char('a'), KeyModifiers::CONTROL), false), Some(vec![0x01]));
    }

    #[test]
    fn alt_char_gets_esc_prefix() {
        assert_eq!(encode_key(&key(KeyCode::Char('x'), KeyModifiers::ALT), false), Some(vec![0x1b, b'x']));
    }

    #[test]
    fn arrows_respect_application_cursor_mode() {
        assert_eq!(encode_key(&key(KeyCode::Up, KeyModifiers::NONE), false), Some(b"\x1b[A".to_vec()));
        assert_eq!(encode_key(&key(KeyCode::Up, KeyModifiers::NONE), true), Some(b"\x1bOA".to_vec()));
    }

    #[test]
    fn modified_arrow_uses_csi_with_modifier_param() {
        // Ctrl+Right → ESC [ 1 ; 5 C
        assert_eq!(encode_key(&key(KeyCode::Right, KeyModifiers::CONTROL), false), Some(b"\x1b[1;5C".to_vec()));
    }

    #[test]
    fn nav_and_function_keys() {
        assert_eq!(encode_key(&key(KeyCode::PageUp, KeyModifiers::NONE), false), Some(b"\x1b[5~".to_vec()));
        assert_eq!(encode_key(&key(KeyCode::Delete, KeyModifiers::NONE), false), Some(b"\x1b[3~".to_vec()));
        assert_eq!(encode_key(&key(KeyCode::F(1), KeyModifiers::NONE), false), Some(b"\x1bOP".to_vec()));
        assert_eq!(encode_key(&key(KeyCode::F(5), KeyModifiers::NONE), false), Some(b"\x1b[15~".to_vec()));
        assert_eq!(encode_key(&key(KeyCode::BackTab, KeyModifiers::NONE), false), Some(b"\x1b[Z".to_vec()));
    }

    #[test]
    fn release_events_emit_nothing() {
        let mut k = key(KeyCode::Char('a'), KeyModifiers::NONE);
        k.kind = KeyEventKind::Release;
        assert_eq!(encode_key(&k, false), None);
    }

    #[test]
    fn sgr_mouse_press_and_release() {
        let down = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 4,
            row: 9,
            modifiers: KeyModifiers::NONE,
        };
        // 1-based coords: column 4 → 5, row 9 → 10.
        assert_eq!(encode_mouse(&down, MouseProtocolEncoding::Sgr), Some(b"\x1b[<0;5;10M".to_vec()));

        let up = MouseEvent {
            kind: MouseEventKind::Up(MouseButton::Left),
            column: 4,
            row: 9,
            modifiers: KeyModifiers::NONE,
        };
        assert_eq!(encode_mouse(&up, MouseProtocolEncoding::Sgr), Some(b"\x1b[<0;5;10m".to_vec()));
    }

    #[test]
    fn scroll_up_is_code_64() {
        let ev = MouseEvent {
            kind: MouseEventKind::ScrollUp,
            column: 0,
            row: 0,
            modifiers: KeyModifiers::NONE,
        };
        assert_eq!(encode_mouse(&ev, MouseProtocolEncoding::Sgr), Some(b"\x1b[<64;1;1M".to_vec()));
    }

    #[test]
    fn non_sgr_encoding_is_unsupported() {
        let ev = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 0,
            row: 0,
            modifiers: KeyModifiers::NONE,
        };
        assert_eq!(encode_mouse(&ev, MouseProtocolEncoding::Default), None);
    }

    #[test]
    fn paste_wraps_only_when_bracketed() {
        assert_eq!(encode_paste("hi", false), b"hi".to_vec());
        assert_eq!(encode_paste("hi", true), b"\x1b[200~hi\x1b[201~".to_vec());
    }
}
