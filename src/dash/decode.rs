//! The controller-side input decoder: raw stdin bytes → key events.
//!
//! crossterm's `event::read()` is hard-wired to the *process's own* stdin/tty
//! and cannot be pointed at a socket, so the controller — which receives the
//! proxy's keystrokes as raw bytes over the seam (`super::proto::UpFrame::Input`)
//! — has to turn those bytes back into the `KeyCode`/`KeyModifiers` its existing
//! `handle_key` already consumes. This module is that parser. It is the exact
//! inverse of the shelved `harness-pane::input` encoder, and keeps the same
//! byte-sequence conventions (CSI `ESC[1;5C` for Ctrl-Right, SS3 `ESC O A` for
//! application-cursor arrows).
//!
//! It is `ratatui`/`RepoView`-free: it imports only `crossterm`'s key types,
//! which are presentation input, not data layer (ADR-0013 seam).
//!
//! ## The lone-ESC problem
//!
//! A bare `0x1b` is ambiguous: it is either the Escape key, or the first byte of
//! an `ESC [ …` / `ESC O …` sequence whose tail has not arrived yet. Like a real
//! terminal we resolve it on a timeout: a trailing incomplete escape stays
//! buffered (no key emitted), and [`InputDecoder::flush`] — called by the
//! controller after an idle tick — forces a pending lone ESC into an `Esc` key.

use ratatui::crossterm::event::{KeyCode, KeyModifiers};

/// A decoded key press: the `crossterm` pair the dashboard's `handle_key`
/// already takes. Mouse is intentionally out of scope for the within-repo cut.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Key {
    pub code: KeyCode,
    pub mods: KeyModifiers,
}

impl Key {
    fn plain(code: KeyCode) -> Key {
        Key {
            code,
            mods: KeyModifiers::NONE,
        }
    }
    fn ctrl(code: KeyCode) -> Key {
        Key {
            code,
            mods: KeyModifiers::CONTROL,
        }
    }
    fn with_alt(mut self) -> Key {
        self.mods |= KeyModifiers::ALT;
        self
    }
}

/// Stateful, fed by socket reads. Retains a trailing incomplete sequence (a
/// split escape or split UTF-8 char) until the rest of its bytes arrive.
#[derive(Default)]
pub struct InputDecoder {
    buf: Vec<u8>,
}

/// The outcome of trying to parse one key from the front of the buffer.
enum Parse {
    /// A key, plus how many leading bytes it consumed.
    Key(Key, usize),
    /// `n` leading bytes are unrepresentable; drop them and carry on.
    Skip(usize),
    /// Not enough bytes yet — leave the buffer and wait for more.
    Incomplete,
}

impl InputDecoder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Append raw bytes and return every complete key now decodable. A trailing
    /// partial sequence is retained for the next `feed`.
    pub fn feed(&mut self, data: &[u8]) -> Vec<Key> {
        self.buf.extend_from_slice(data);
        let mut out = Vec::new();
        loop {
            match parse_one(&self.buf) {
                Parse::Key(k, n) => {
                    self.buf.drain(..n);
                    out.push(k);
                }
                Parse::Skip(n) => {
                    self.buf.drain(..n.max(1));
                }
                Parse::Incomplete => break,
            }
        }
        out
    }

    /// Resolve a pending sequence on an idle tick. A buffered lone `ESC` becomes
    /// the `Esc` key; any bytes after it are then parsed normally (matching a
    /// real terminal's ESC-timeout behaviour for, e.g., a stuck `ESC [`).
    pub fn flush(&mut self) -> Vec<Key> {
        if self.buf.first() == Some(&0x1b) {
            self.buf.drain(..1);
            let mut out = vec![Key::plain(KeyCode::Esc)];
            out.extend(self.feed(&[]));
            out
        } else {
            Vec::new()
        }
    }
}

fn parse_one(b: &[u8]) -> Parse {
    let Some(&c0) = b.first() else {
        return Parse::Incomplete;
    };
    match c0 {
        0x1b => parse_esc(b),
        0x0d | 0x0a => Parse::Key(Key::plain(KeyCode::Enter), 1),
        0x09 => Parse::Key(Key::plain(KeyCode::Tab), 1),
        0x7f | 0x08 => Parse::Key(Key::plain(KeyCode::Backspace), 1),
        0x00 => Parse::Key(Key::ctrl(KeyCode::Char(' ')), 1),
        // C0 control letters (^A..^Z); ^H/^I/^J/^M and ESC are handled above.
        0x01..=0x1a => {
            let ch = (b'a' + (c0 - 1)) as char;
            Parse::Key(Key::ctrl(KeyCode::Char(ch)), 1)
        }
        // The remaining C0 controls map to their conventional Ctrl+symbol.
        0x1c..=0x1f => {
            let ch = match c0 {
                0x1c => '\\',
                0x1d => ']',
                0x1e => '^',
                0x1f => '_',
                _ => unreachable!(),
            };
            Parse::Key(Key::ctrl(KeyCode::Char(ch)), 1)
        }
        _ => parse_utf8(b),
    }
}

/// A printable byte (>= 0x20): decode one UTF-8 scalar, waiting if its
/// continuation bytes have not all arrived.
fn parse_utf8(b: &[u8]) -> Parse {
    let lead = b[0];
    let len = match lead {
        0x00..=0x7f => 1,
        0xc0..=0xdf => 2,
        0xe0..=0xef => 3,
        0xf0..=0xf7 => 4,
        // A stray continuation/invalid lead byte: unrepresentable.
        _ => return Parse::Skip(1),
    };
    if b.len() < len {
        return Parse::Incomplete;
    }
    match std::str::from_utf8(&b[..len]) {
        Ok(s) => match s.chars().next() {
            Some(c) => Parse::Key(Key::plain(KeyCode::Char(c)), len),
            None => Parse::Skip(1),
        },
        Err(_) => Parse::Skip(1),
    }
}

/// `b[0]` is ESC. Distinguish a lone ESC (incomplete), a CSI (`ESC [`), an SS3
/// (`ESC O`), and an Alt-prefixed key (`ESC` + anything else).
fn parse_esc(b: &[u8]) -> Parse {
    if b.len() < 2 {
        return Parse::Incomplete;
    }
    match b[1] {
        b'[' => parse_csi(b),
        b'O' => parse_ss3(b),
        _ => match parse_one(&b[1..]) {
            Parse::Key(k, n) => Parse::Key(k.with_alt(), 1 + n),
            Parse::Skip(n) => Parse::Skip(1 + n),
            Parse::Incomplete => Parse::Incomplete,
        },
    }
}

/// `ESC [ <params> <intermediates> <final>` — arrows, Home/End, BackTab, and the
/// tilde-form nav/function keys, with an optional xterm modifier param.
fn parse_csi(b: &[u8]) -> Parse {
    // params: 0x30..=0x3f (digits, ';', and the private '<=>?').
    let mut p = 2;
    while p < b.len() && (0x30..=0x3f).contains(&b[p]) {
        p += 1;
    }
    let params = &b[2..p];
    // intermediates: 0x20..=0x2f (ignored for our subset).
    let mut q = p;
    while q < b.len() && (0x20..=0x2f).contains(&b[q]) {
        q += 1;
    }
    if q >= b.len() {
        return Parse::Incomplete;
    }
    let final_byte = b[q];
    let consumed = q + 1;

    let nums = parse_params(params);
    let mods = xterm_mods(nums.get(1).copied().flatten());

    let code = match final_byte {
        b'A' => KeyCode::Up,
        b'B' => KeyCode::Down,
        b'C' => KeyCode::Right,
        b'D' => KeyCode::Left,
        b'H' => KeyCode::Home,
        b'F' => KeyCode::End,
        b'P' => KeyCode::F(1),
        b'Q' => KeyCode::F(2),
        b'R' => KeyCode::F(3),
        b'S' => KeyCode::F(4),
        // BackTab carries SHIFT, matching crossterm's own reader.
        b'Z' => {
            return Parse::Key(
                Key {
                    code: KeyCode::BackTab,
                    mods: KeyModifiers::SHIFT,
                },
                consumed,
            )
        }
        b'~' => match nums.first().copied().flatten().unwrap_or(0) {
            2 => KeyCode::Insert,
            3 => KeyCode::Delete,
            5 => KeyCode::PageUp,
            6 => KeyCode::PageDown,
            15 => KeyCode::F(5),
            17 => KeyCode::F(6),
            18 => KeyCode::F(7),
            19 => KeyCode::F(8),
            20 => KeyCode::F(9),
            21 => KeyCode::F(10),
            23 => KeyCode::F(11),
            24 => KeyCode::F(12),
            _ => return Parse::Skip(consumed),
        },
        _ => return Parse::Skip(consumed),
    };
    Parse::Key(Key { code, mods }, consumed)
}

/// `ESC O <final>` — application-cursor arrows and F1–F4. No modifiers.
fn parse_ss3(b: &[u8]) -> Parse {
    if b.len() < 3 {
        return Parse::Incomplete;
    }
    let code = match b[2] {
        b'A' => KeyCode::Up,
        b'B' => KeyCode::Down,
        b'C' => KeyCode::Right,
        b'D' => KeyCode::Left,
        b'H' => KeyCode::Home,
        b'F' => KeyCode::End,
        b'P' => KeyCode::F(1),
        b'Q' => KeyCode::F(2),
        b'R' => KeyCode::F(3),
        b'S' => KeyCode::F(4),
        _ => return Parse::Skip(3),
    };
    Parse::Key(Key::plain(code), 3)
}

/// Split a `;`-separated CSI param run into numbers; an empty or non-numeric
/// field is `None` (its default).
fn parse_params(bytes: &[u8]) -> Vec<Option<u32>> {
    if bytes.is_empty() {
        return Vec::new();
    }
    match std::str::from_utf8(bytes) {
        Ok(s) => s.split(';').map(|f| f.parse::<u32>().ok()).collect(),
        Err(_) => Vec::new(),
    }
}

/// The xterm modifier param: `1 + shift + 2*alt + 4*ctrl`; `1`/absent = none.
fn xterm_mods(param: Option<u32>) -> KeyModifiers {
    let bits = param.unwrap_or(1).saturating_sub(1);
    let mut m = KeyModifiers::NONE;
    if bits & 1 != 0 {
        m |= KeyModifiers::SHIFT;
    }
    if bits & 2 != 0 {
        m |= KeyModifiers::ALT;
    }
    if bits & 4 != 0 {
        m |= KeyModifiers::CONTROL;
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plain(code: KeyCode) -> Key {
        Key::plain(code)
    }

    #[test]
    fn csi_arrows() {
        let mut d = InputDecoder::new();
        assert_eq!(d.feed(b"\x1b[A"), vec![plain(KeyCode::Up)]);
        assert_eq!(d.feed(b"\x1b[B"), vec![plain(KeyCode::Down)]);
        assert_eq!(d.feed(b"\x1b[C"), vec![plain(KeyCode::Right)]);
        assert_eq!(d.feed(b"\x1b[D"), vec![plain(KeyCode::Left)]);
    }

    #[test]
    fn ss3_arrow_application_cursor() {
        let mut d = InputDecoder::new();
        assert_eq!(d.feed(b"\x1bOA"), vec![plain(KeyCode::Up)]);
    }

    #[test]
    fn enter_cr_and_lf() {
        let mut d = InputDecoder::new();
        assert_eq!(d.feed(b"\r"), vec![plain(KeyCode::Enter)]);
        assert_eq!(d.feed(b"\n"), vec![plain(KeyCode::Enter)]);
    }

    #[test]
    fn tab_and_backspace() {
        let mut d = InputDecoder::new();
        assert_eq!(d.feed(b"\t"), vec![plain(KeyCode::Tab)]);
        assert_eq!(d.feed(b"\x7f"), vec![plain(KeyCode::Backspace)]);
    }

    #[test]
    fn esc_alone_resolves_on_flush() {
        let mut d = InputDecoder::new();
        // A lone ESC is held — it might still be the head of a sequence.
        assert_eq!(d.feed(b"\x1b"), vec![]);
        assert_eq!(d.flush(), vec![plain(KeyCode::Esc)]);
    }

    #[test]
    fn ctrl_letters() {
        let mut d = InputDecoder::new();
        assert_eq!(d.feed(b"\x13"), vec![Key::ctrl(KeyCode::Char('s'))]);
        assert_eq!(d.feed(b"\x03"), vec![Key::ctrl(KeyCode::Char('c'))]);
        assert_eq!(d.feed(b"\x05"), vec![Key::ctrl(KeyCode::Char('e'))]);
    }

    #[test]
    fn multibyte_utf8_char() {
        let mut d = InputDecoder::new();
        // 'λ' = 0xce 0xbb
        assert_eq!(d.feed("λ".as_bytes()), vec![plain(KeyCode::Char('λ'))]);
    }

    #[test]
    fn paste_like_run_of_chars() {
        let mut d = InputDecoder::new();
        assert_eq!(
            d.feed(b"hello"),
            vec![
                plain(KeyCode::Char('h')),
                plain(KeyCode::Char('e')),
                plain(KeyCode::Char('l')),
                plain(KeyCode::Char('l')),
                plain(KeyCode::Char('o')),
            ]
        );
    }

    #[test]
    fn modified_arrow_has_ctrl() {
        let mut d = InputDecoder::new();
        // Ctrl+Right = ESC [ 1 ; 5 C
        assert_eq!(
            d.feed(b"\x1b[1;5C"),
            vec![Key {
                code: KeyCode::Right,
                mods: KeyModifiers::CONTROL,
            }]
        );
    }

    #[test]
    fn tilde_nav_keys() {
        let mut d = InputDecoder::new();
        assert_eq!(d.feed(b"\x1b[3~"), vec![plain(KeyCode::Delete)]);
        assert_eq!(d.feed(b"\x1b[5~"), vec![plain(KeyCode::PageUp)]);
        assert_eq!(d.feed(b"\x1b[6~"), vec![plain(KeyCode::PageDown)]);
    }

    #[test]
    fn alt_char() {
        let mut d = InputDecoder::new();
        assert_eq!(
            d.feed(b"\x1bx"),
            vec![Key {
                code: KeyCode::Char('x'),
                mods: KeyModifiers::ALT,
            }]
        );
    }

    #[test]
    fn split_csi_across_feeds() {
        let mut d = InputDecoder::new();
        assert_eq!(d.feed(b"\x1b["), vec![]);
        assert_eq!(d.feed(b"A"), vec![plain(KeyCode::Up)]);
    }

    #[test]
    fn split_utf8_across_feeds() {
        let mut d = InputDecoder::new();
        let bytes = "λ".as_bytes();
        assert_eq!(d.feed(&bytes[..1]), vec![]);
        assert_eq!(d.feed(&bytes[1..]), vec![plain(KeyCode::Char('λ'))]);
    }

    #[test]
    fn mixed_run_in_one_feed() {
        let mut d = InputDecoder::new();
        assert_eq!(
            d.feed(b"ab\r"),
            vec![
                plain(KeyCode::Char('a')),
                plain(KeyCode::Char('b')),
                plain(KeyCode::Enter),
            ]
        );
    }

    #[test]
    fn backtab_is_shift() {
        let mut d = InputDecoder::new();
        assert_eq!(
            d.feed(b"\x1b[Z"),
            vec![Key {
                code: KeyCode::BackTab,
                mods: KeyModifiers::SHIFT,
            }]
        );
    }
}
