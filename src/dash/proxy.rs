//! The dumb dashboard proxy: `grove __dash-proxy` (ADR-0016).
//!
//! This is the thin client that runs in a zellij pane. It holds **no** grove
//! state, logic, or ratatui — it is pure terminal transport. It:
//!
//! 1. connects to the controller's unix-domain socket,
//! 2. takes ownership of the real tty (raw mode + alternate screen + hidden
//!    cursor) — termios raw mode cannot travel over a socket, so the proxy, not
//!    the controller, sets it,
//! 3. reports its size up on connect and on every SIGWINCH,
//! 4. blits controller-sent output bytes straight to its stdout (the down
//!    direction is unframed raw bytes — `super::proto`),
//! 5. forwards its raw stdin up as `Input` frames for the controller to decode.
//!
//! Lifecycle is **socket-governed**: the main thread runs the down-pump, so when
//! the controller closes the socket the pump hits EOF, `run` returns, and the
//! [`TtyGuard`] restores the terminal on the way out. The stdin and SIGWINCH
//! threads are detached; process exit reaps them.

use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

use anyhow::{Context, Result};
use ratatui::crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen},
};

use super::proto::UpFrame;

/// Run the proxy against the controller listening at `socket`.
pub fn run(socket: &Path) -> Result<()> {
    let stream = UnixStream::connect(socket)
        .with_context(|| format!("connecting to controller socket {}", socket.display()))?;

    // Own the tty before sending anything: raw mode + alt screen + hidden
    // cursor. Restored on drop, including on the EOF path below.
    let _tty = TtyGuard::enter().context("preparing terminal")?;

    // Frames go *up* from two places — the stdin pump and the SIGWINCH handler —
    // so the write half is shared behind a mutex and each frame is written whole.
    let writer = Arc::new(Mutex::new(
        stream.try_clone().context("cloning socket for writing")?,
    ));

    send_size(&writer).context("sending initial size")?;

    // SIGWINCH → resize frame (detached; best-effort if the socket is dying).
    let winch_writer = Arc::clone(&writer);
    thread::spawn(move || winch_loop(winch_writer));

    // stdin → socket (detached; ends on stdin EOF, which is rare for a tty).
    let stdin_writer = Arc::clone(&writer);
    thread::spawn(move || {
        let _ = up_pump(stdin_writer);
    });

    // Main thread: socket → stdout. Governs the lifetime; returns when the
    // controller closes the socket, after which `_tty` drops and restores.
    down_pump(stream).context("relaying controller output")
}

/// Pump controller output to stdout until the socket closes.
fn down_pump(mut stream: UnixStream) -> Result<()> {
    let mut stdout = std::io::stdout().lock();
    let mut buf = [0u8; 8192];
    loop {
        let n = stream.read(&mut buf)?;
        if n == 0 {
            return Ok(()); // controller closed the seam — clean shutdown.
        }
        stdout.write_all(&buf[..n])?;
        stdout.flush()?;
    }
}

/// Pump raw stdin bytes up as `Input` frames until stdin EOF.
fn up_pump(writer: Arc<Mutex<UnixStream>>) -> Result<()> {
    let mut stdin = std::io::stdin().lock();
    let mut buf = [0u8; 4096];
    loop {
        let n = stdin.read(&mut buf)?;
        if n == 0 {
            return Ok(());
        }
        send_frame(&writer, &UpFrame::Input(buf[..n].to_vec()))?;
    }
}

/// Block on SIGWINCH and send a fresh size up on each one.
fn winch_loop(writer: Arc<Mutex<UnixStream>>) {
    use signal_hook::consts::SIGWINCH;
    use signal_hook::iterator::Signals;

    let mut signals = match Signals::new([SIGWINCH]) {
        Ok(s) => s,
        Err(_) => return,
    };
    for _ in signals.forever() {
        // Best-effort: if the socket is gone the proxy is already exiting.
        if send_size(&writer).is_err() {
            return;
        }
    }
}

/// Query the current terminal size and send it as a `Resize` frame.
fn send_size(writer: &Arc<Mutex<UnixStream>>) -> Result<()> {
    let (cols, rows) = size().context("querying terminal size")?;
    send_frame(writer, &UpFrame::Resize { cols, rows })
}

/// Write one whole frame up the socket under the shared lock.
fn send_frame(writer: &Arc<Mutex<UnixStream>>, frame: &UpFrame) -> Result<()> {
    let bytes = frame.encode();
    let mut guard = writer.lock().expect("up-socket mutex poisoned");
    guard.write_all(&bytes)?;
    guard.flush()?;
    Ok(())
}

/// RAII ownership of the real tty: raw mode + alternate screen + hidden cursor,
/// all restored on drop (so the EOF return path leaves the terminal sane).
struct TtyGuard;

impl TtyGuard {
    fn enter() -> Result<Self> {
        enable_raw_mode()?;
        execute!(std::io::stdout(), EnterAlternateScreen, Hide)?;
        Ok(TtyGuard)
    }
}

impl Drop for TtyGuard {
    fn drop(&mut self) {
        let _ = execute!(std::io::stdout(), Show, LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}
