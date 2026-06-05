//! The renderable, source-agnostic terminal core.

use tui_term::widget::{Cursor, PseudoTerminal};
use vt100::{MouseProtocolEncoding, MouseProtocolMode, Parser, Screen};

/// How much scrollback `vt100` retains by default. Scrollback *navigation* and
/// selection are 020's concern; this is just the buffer depth the parser keeps
/// so that history exists to navigate later.
const DEFAULT_SCROLLBACK: usize = 10_000;

/// Where the cursor is and whether the application wants it shown.
///
/// The host uses this to place the **native hardware cursor**
/// (`Frame::set_cursor_position`) — the embed deliberately hides `tui-term`'s
/// drawn cursor (see [`TerminalEmulator::widget`]) so there is exactly one
/// cursor, painted in the application's own colours rather than `tui-term`'s
/// unreadable white-on-grey overlay.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorState {
    /// 0-based row within the screen grid.
    pub row: u16,
    /// 0-based column within the screen grid.
    pub col: u16,
    /// Whether the application is showing the cursor (DECTCEM).
    pub visible: bool,
}

/// Owns a `vt100` parser and renders its screen via `tui-term`.
///
/// Fed bytes through [`process`](Self::process) from *any* source — a
/// [`PtySession`](crate::PtySession), synthetic ANSI, or a recording — so the
/// whole render/cursor/title path is exercisable with no child process.
pub struct TerminalEmulator {
    parser: Parser,
}

impl TerminalEmulator {
    /// Create an emulator with a grid of `rows`×`cols` and the default
    /// scrollback depth.
    pub fn new(rows: u16, cols: u16) -> Self {
        Self::with_scrollback(rows, cols, DEFAULT_SCROLLBACK)
    }

    /// Create an emulator with an explicit scrollback depth (in rows).
    pub fn with_scrollback(rows: u16, cols: u16, scrollback_len: usize) -> Self {
        Self {
            parser: Parser::new(rows, cols, scrollback_len),
        }
    }

    /// Feed raw terminal output bytes into the emulator. The sole mutator of
    /// the parser, so no lock is needed: the owner is the only writer.
    pub fn process(&mut self, bytes: &[u8]) {
        self.parser.process(bytes);
    }

    /// Resize the grid. The caller must keep the byte source's notion of the
    /// size in step — [`PtySession::resize`](crate::PtySession::resize) does
    /// both together.
    pub fn resize(&mut self, rows: u16, cols: u16) {
        self.parser.set_size(rows, cols);
    }

    /// The current grid size as `(rows, cols)`.
    pub fn size(&self) -> (u16, u16) {
        self.parser.screen().size()
    }

    /// Borrow the underlying `vt100` screen — the escape hatch for callers that
    /// need cell-level data (e.g. 020's selection model) beyond what this type
    /// surfaces directly.
    pub fn screen(&self) -> &Screen {
        self.parser.screen()
    }

    /// Where to paint the native hardware cursor, and whether to paint it.
    pub fn cursor(&self) -> CursorState {
        let screen = self.parser.screen();
        let (row, col) = screen.cursor_position();
        CursorState {
            row,
            col,
            visible: !screen.hide_cursor(),
        }
    }

    /// The window title set via OSC 0/2, or `""` if none.
    pub fn title(&self) -> &str {
        self.parser.screen().title()
    }

    /// Whether the focused application has requested mouse reporting.
    ///
    /// This is the **data-up** half of dynamic mouse capture (ADR-0014): the
    /// host toggles real `EnableMouseCapture` on this per focus change, so the
    /// terminal's native text-selection still works over an app (like claude)
    /// that wants no mouse. The crate never calls `execute!` itself — it only
    /// reports intent.
    pub fn wants_mouse(&self) -> bool {
        self.parser.screen().mouse_protocol_mode() != MouseProtocolMode::None
    }

    /// Whether DECCKM (application cursor keys) is active. The input encoder
    /// needs this to send arrows as `ESC O A` rather than `ESC [ A`.
    pub fn application_cursor(&self) -> bool {
        self.parser.screen().application_cursor()
    }

    /// The mouse encoding the application requested. SGR (1006) for any modern
    /// TUI; [`input::encode_mouse`](crate::input::encode_mouse) only emits SGR.
    pub fn mouse_encoding(&self) -> MouseProtocolEncoding {
        self.parser.screen().mouse_protocol_encoding()
    }

    /// Whether the application enabled bracketed paste (DECSET 2004), so the
    /// host knows whether to wrap a paste — see
    /// [`input::encode_paste`](crate::input::encode_paste).
    pub fn bracketed_paste(&self) -> bool {
        self.parser.screen().bracketed_paste()
    }

    /// A configured `tui-term` widget over the current screen, ready to render.
    ///
    /// `tui-term`'s own drawn cursor is **hidden** here: its default overlay
    /// renders white-on-grey (unreadable in vim). The host instead positions
    /// the native hardware cursor from [`cursor`](Self::cursor).
    pub fn widget(&self) -> PseudoTerminal<'_, Screen> {
        PseudoTerminal::new(self.parser.screen()).cursor(Cursor::default().visibility(false))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fed_bytes_land_on_the_grid_with_no_child() {
        let mut emu = TerminalEmulator::new(5, 20);
        emu.process(b"hi");
        assert_eq!(emu.screen().cell(0, 0).unwrap().contents(), "h");
        assert_eq!(emu.screen().cell(0, 1).unwrap().contents(), "i");
        assert_eq!(emu.cursor().col, 2);
    }

    #[test]
    fn resize_changes_the_grid_size() {
        let mut emu = TerminalEmulator::new(10, 40);
        assert_eq!(emu.size(), (10, 40));
        emu.resize(24, 80);
        assert_eq!(emu.size(), (24, 80));
    }

    #[test]
    fn wants_mouse_tracks_protocol_mode() {
        let mut emu = TerminalEmulator::new(5, 20);
        assert!(!emu.wants_mouse());
        // DECSET 1000 (VT200 mouse) then 1006 (SGR) — what a TUI app emits.
        emu.process(b"\x1b[?1000h");
        assert!(emu.wants_mouse());
        emu.process(b"\x1b[?1000l");
        assert!(!emu.wants_mouse());
    }
}
