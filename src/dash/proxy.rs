//! The dumb dashboard proxy: `grove __dash-proxy` (ADR-0016, ADR-0017).
//!
//! This is the thin client that runs in a zellij pane. It holds **no** grove
//! state, logic, or ratatui — it is pure terminal transport. It:
//!
//! 1. connects to the controller's unix-domain socket,
//! 2. takes ownership of the real tty (raw mode + alternate screen + hidden
//!    cursor) — termios raw mode cannot travel over a socket, so the proxy, not
//!    the controller, sets it,
//! 3. reports its size up on connect and on every SIGWINCH,
//! 4. relays the controller's framed down stream ([`DownFrame`]): blits an
//!    `Output` frame's payload to stdout, and on a `RunEditor` frame runs the
//!    user's `$EDITOR` on its own tty (ADR-0017),
//! 5. forwards its raw stdin up as `Input` frames for the controller to decode.
//!
//! ## Why a single-threaded `poll` loop
//!
//! 010 used a detached stdin→socket thread. That races the one place the
//! dumb-proxy model meets an interactive child: while `$EDITOR` runs it must own
//! the tty's stdin, but a blocked `stdin.read()` in a background thread would
//! steal the user's keystrokes from the editor. So the relay is a single loop
//! that `poll(2)`s the socket and stdin together; when it runs the editor it is
//! simply *not* polling stdin (it is blocked in `wait()`), so the editor has the
//! tty to itself with no flag-gating or thread coordination. The SIGWINCH
//! handler stays a detached thread — it only *writes* `Resize` frames up.
//!
//! Lifecycle is **socket-governed**: the loop exits when the controller closes
//! the socket (read hits EOF), after which the [`TtyGuard`] restores the
//! terminal on the way out.

use std::io::{Read, Write};
use std::os::unix::io::AsRawFd;
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

use super::proto::{DownDecoder, DownFrame, UpFrame};

/// Run the proxy against the controller listening at `socket`.
pub fn run(socket: &Path) -> Result<()> {
    let stream = UnixStream::connect(socket)
        .with_context(|| format!("connecting to controller socket {}", socket.display()))?;

    // Own the tty before sending anything: raw mode + alt screen + hidden
    // cursor. Restored on drop, including on the EOF path below.
    let mut tty = TtyGuard::enter().context("preparing terminal")?;

    // Up-frames are written from two places — this relay loop and the detached
    // SIGWINCH handler — so the write half is shared behind a mutex and each
    // frame is written whole.
    let writer = Arc::new(Mutex::new(
        stream.try_clone().context("cloning socket for writing")?,
    ));

    send_size(&writer).context("sending initial size")?;

    // SIGWINCH → resize frame (detached; best-effort if the socket is dying).
    let winch_writer = Arc::clone(&writer);
    thread::spawn(move || winch_loop(winch_writer));

    relay(stream, &writer, &mut tty).context("relaying controller seam")
}

/// The single-threaded relay: `poll(2)` the socket and stdin together, blitting
/// down `Output` frames to stdout, forwarding stdin up as `Input` frames, and
/// running `$EDITOR` on a `RunEditor` frame. Returns when the controller closes
/// the socket (down EOF).
fn relay(mut stream: UnixStream, writer: &Arc<Mutex<UnixStream>>, tty: &mut TtyGuard) -> Result<()> {
    let socket_fd = stream.as_raw_fd();
    let stdin_fd = libc::STDIN_FILENO;

    let mut down = DownDecoder::new();
    let mut dbuf = [0u8; 8192];
    let mut sbuf = [0u8; 4096];

    loop {
        let mut fds = [
            libc::pollfd { fd: socket_fd, events: libc::POLLIN, revents: 0 },
            libc::pollfd { fd: stdin_fd, events: libc::POLLIN, revents: 0 },
        ];
        // Block until either fd is ready. SIGWINCH interrupts `poll` with
        // EINTR; just retry (the winch thread has already sent the resize).
        let rc = unsafe { libc::poll(fds.as_mut_ptr(), fds.len() as libc::nfds_t, -1) };
        if rc < 0 {
            let err = std::io::Error::last_os_error();
            if err.kind() == std::io::ErrorKind::Interrupted {
                continue;
            }
            return Err(err).context("poll on socket/stdin");
        }

        // Down: controller → stdout / editor.
        if fds[0].revents & (libc::POLLIN | libc::POLLHUP) != 0 {
            let n = stream.read(&mut dbuf).context("reading controller stream")?;
            if n == 0 {
                return Ok(()); // controller closed the seam — clean shutdown.
            }
            down.extend(&dbuf[..n]);
            let mut stdout = std::io::stdout().lock();
            while let Some(frame) = down.next_frame() {
                match frame {
                    DownFrame::Output(bytes) => {
                        stdout.write_all(&bytes)?;
                        stdout.flush()?;
                    }
                    DownFrame::RunEditor { path } => {
                        // Release the stdout lock for the duration of the editor
                        // so the child has the tty to itself.
                        drop(stdout);
                        let ok = run_editor(&path, tty);
                        send_frame(writer, &UpFrame::EditorDone { ok })?;
                        stdout = std::io::stdout().lock();
                    }
                }
            }
        }

        // Up: stdin → socket. Never reached while an editor child runs (we are
        // blocked in `run_editor`), so the child keeps the tty's keystrokes.
        // Read the raw fd directly — `std::io::stdin()` wraps an 8 KiB
        // `BufReader`, which could buffer bytes in userspace that `poll` (on the
        // raw fd) would never report, stalling input.
        if fds[1].revents & libc::POLLIN != 0 {
            let n = unsafe {
                libc::read(stdin_fd, sbuf.as_mut_ptr() as *mut libc::c_void, sbuf.len())
            };
            if n < 0 {
                let err = std::io::Error::last_os_error();
                if err.kind() == std::io::ErrorKind::Interrupted {
                    continue;
                }
                return Err(err).context("reading stdin");
            }
            if n > 0 {
                send_frame(writer, &UpFrame::Input(sbuf[..n as usize].to_vec()))?;
            }
            // n == 0 is stdin EOF (rare for a tty); ignore and keep relaying the
            // down direction, which still governs shutdown.
        }
    }
}

/// Run the user's `$EDITOR` (or `$VISUAL`, else `vi`) on `path` against the
/// proxy's real tty, suspending grove's tty ownership around it (ADR-0017).
/// Returns whether the editor exited successfully. The controller created
/// `path` (a shared-filesystem tempfile) and reads it back after `EditorDone`.
fn run_editor(path: &Path, tty: &mut TtyGuard) -> bool {
    let editor = std::env::var("EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .unwrap_or_else(|_| "vi".into());

    // Hand the real tty to the editor: leave the alternate screen and raw mode
    // so a full-screen editor sees a normal terminal, then restore after.
    tty.suspend();
    let status = std::process::Command::new(&editor).arg(path).status();
    tty.resume();

    matches!(status, Ok(s) if s.success())
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
/// [`suspend`](Self::suspend)/[`resume`](Self::resume) temporarily hand the tty
/// to an interactive child (`$EDITOR`) and take it back.
struct TtyGuard;

impl TtyGuard {
    fn enter() -> Result<Self> {
        enable_raw_mode()?;
        execute!(std::io::stdout(), EnterAlternateScreen, Hide)?;
        Ok(TtyGuard)
    }

    /// Hand the tty to a child: leave the alternate screen, show the cursor, and
    /// drop raw mode, so a full-screen child program sees a normal terminal.
    fn suspend(&mut self) {
        let _ = execute!(std::io::stdout(), Show, LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }

    /// Take the tty back after the child exits. The controller forces a full
    /// redraw on its side, so the freshly-cleared alternate screen repaints.
    fn resume(&mut self) {
        let _ = enable_raw_mode();
        let _ = execute!(std::io::stdout(), EnterAlternateScreen, Hide);
    }
}

impl Drop for TtyGuard {
    fn drop(&mut self) {
        let _ = execute!(std::io::stdout(), Show, LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}
