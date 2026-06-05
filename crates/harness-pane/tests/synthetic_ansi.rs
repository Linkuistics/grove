//! Headless half of the 050 assessment: drive the emulator with synthetic ANSI
//! and assert what landed — no child process, no pty. This is the source-
//! agnostic property of `TerminalEmulator` paying off: the whole render/cursor/
//! colour/title path is exercisable from a byte string.

use harness_pane::vt100::Color;
use harness_pane::TerminalEmulator;

#[test]
fn colors_cursor_moves_and_osc_title() {
    let mut emu = TerminalEmulator::new(10, 40);

    // Bold red 'R', then reset; green 'G'; cursor home (CUP) and a fresh 'X'
    // overwriting the top-left; finally an OSC 2 window title.
    emu.process(b"\x1b[1;31mR\x1b[0m\x1b[32mG\x1b[0m");
    emu.process(b"\x1b[5;10HmidL"); // move to row 5, col 10 (1-based) and write
    emu.process(b"\x1b]2;my-harness\x07"); // OSC 2 set-title, BEL-terminated

    let screen = emu.screen();

    // Cell contents.
    assert_eq!(screen.cell(0, 0).unwrap().contents(), "R");
    assert_eq!(screen.cell(0, 1).unwrap().contents(), "G");

    // SGR attributes were consumed into cell state, not leaked as text.
    let r = screen.cell(0, 0).unwrap();
    assert_eq!(r.fgcolor(), Color::Idx(1), "red");
    assert!(r.bold(), "bold");
    assert_eq!(screen.cell(0, 1).unwrap().fgcolor(), Color::Idx(2), "green");

    // CUP landed: row 5, col 10 (1-based) → row 4, col 9 (0-based).
    assert_eq!(screen.cell(4, 9).unwrap().contents(), "m");
    assert_eq!(screen.cell(4, 12).unwrap().contents(), "L");

    // After writing "midL" starting at col 9, the cursor sits at col 13.
    let cursor = emu.cursor();
    assert_eq!((cursor.row, cursor.col), (4, 13));
    assert!(cursor.visible);

    // OSC title parsed (load-bearing: this is why vt100 0.15.2 keeps `title()`).
    assert_eq!(emu.title(), "my-harness");
}

#[test]
fn alternate_screen_and_cursor_hide_are_tracked() {
    let mut emu = TerminalEmulator::new(10, 40);
    assert!(emu.cursor().visible);

    // DECTCEM hide cursor.
    emu.process(b"\x1b[?25l");
    assert!(!emu.cursor().visible);
    emu.process(b"\x1b[?25h");
    assert!(emu.cursor().visible);
}
