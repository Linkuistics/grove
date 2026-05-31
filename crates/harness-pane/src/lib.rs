//! A source-agnostic, in-process terminal embed for Ratatui.
//!
//! This crate is the reusable component the grove 050 spike identified: the
//! consumer wiring that `tui-term` (render-only, WIP) leaves open. It owns
//! `vt100`/`portable-pty`/input below the [presentation
//! boundary](https://) and hands back a renderable widget plus plain data.
//!
//! Three layers, named to keep *pane* meaning a layout region (grove's
//! concern), distinct from the pty-backed thing inside it:
//!
//! - [`TerminalEmulator`] — owns the `vt100` parser/`Screen`, is **fed bytes**
//!   via [`TerminalEmulator::process`], and renders through `tui-term`'s
//!   `PseudoTerminal`. Source-agnostic: a [`PtySession`], synthetic ANSI, or a
//!   recording can all feed it, which makes it unit-testable with no child
//!   process. This is the testable core.
//! - [`PtySession`] — the byte *source*: a `portable-pty` master + child + the
//!   reader thread that pumps output into an `mpsc` channel.
//! - a *pane* — a layout region (grove's concern) pairing an emulator with
//!   input routing. Not a type in this crate.
//!
//! # Wiring sketch (the 050 sync pump)
//!
//! ```no_run
//! use harness_pane::{PtySession, TerminalEmulator};
//!
//! let mut emu = TerminalEmulator::new(40, 120);
//! let mut pty = PtySession::spawn(&["bash".into()], None, &[], 40, 120)?;
//! loop {
//!     // Between event-loop ticks, drain the reader thread into the emulator.
//!     emu.process(&pty.drain());
//!     // ... render `emu.widget()`, position the native cursor at `emu.cursor()`,
//!     //     toggle host mouse capture on `emu.wants_mouse()`, feed input via
//!     //     `pty.write_input(&harness_pane::input::encode_key(&key, emu.application_cursor()))` ...
//!     # break;
//! }
//! # Ok::<(), anyhow::Error>(())
//! ```

mod emulator;
pub mod input;
mod pty;

pub use emulator::{CursorState, TerminalEmulator};
pub use pty::PtySession;

// Re-export the pinned vt100 so consumers name the *same* crate version this
// embed compiles against (the 0.15.2 pin is load-bearing — see Cargo.toml).
pub use tui_term::vt100;
