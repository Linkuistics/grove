//! The live-process byte source: a `portable-pty` master + child + the reader
//! thread that pumps output into an `mpsc` channel.

use std::io::{ErrorKind, Read, Result as IoResult, Write};
use std::path::Path;
use std::sync::mpsc::{self, Receiver};
use std::thread::{self, JoinHandle};

use anyhow::Context;
use portable_pty::{Child, CommandBuilder, ExitStatus, MasterPty, PtySize, native_pty_system};

use crate::TerminalEmulator;

/// Read buffer for the reader thread. Sized generously so a startup burst
/// (claude's was ~5 KB / 15 chunks in the 050 spike) coalesces into few sends.
const READ_BUF: usize = 8192;

/// A spawned child process attached to a pseudo-terminal, with its output
/// pumped off a reader thread into a channel the owner drains between
/// event-loop ticks (the 050 sync pump — no lock on the parser because the
/// single sync owner is the only mutator).
pub struct PtySession {
    master: Box<dyn MasterPty + Send>,
    writer: Box<dyn Write + Send>,
    child: Box<dyn Child + Send + Sync>,
    rx: Receiver<Vec<u8>>,
    reader_thread: Option<JoinHandle<()>>,
}

impl PtySession {
    /// Spawn `argv` (with `argv[0]` the program) attached to a fresh pty of
    /// `rows`×`cols`, in `cwd` if given, with `env` overlaid on the inherited
    /// environment.
    ///
    /// `TERM=xterm-256color` is set as a default so a fresh app sees a sane
    /// terminal type; an explicit `("TERM", ...)` in `env` overrides it.
    pub fn spawn(
        argv: &[String],
        cwd: Option<&Path>,
        env: &[(String, String)],
        rows: u16,
        cols: u16,
    ) -> anyhow::Result<Self> {
        let program = argv
            .first()
            .context("PtySession::spawn requires a non-empty argv")?;

        let pair = native_pty_system()
            .openpty(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .context("openpty failed")?;

        let mut cmd = CommandBuilder::new(program);
        cmd.args(&argv[1..]);
        if let Some(dir) = cwd {
            cmd.cwd(dir);
        }
        // Default first so caller-supplied env wins on collision.
        cmd.env("TERM", "xterm-256color");
        for (key, value) in env {
            cmd.env(key, value);
        }

        let child = pair
            .slave
            .spawn_command(cmd)
            .context("spawning the harness child failed")?;
        // Drop the slave handle: the child holds its own copy, and keeping ours
        // open would prevent the reader from ever seeing EOF when the child exits.
        drop(pair.slave);

        let mut reader = pair
            .master
            .try_clone_reader()
            .context("cloning the pty reader failed")?;
        let writer = pair
            .master
            .take_writer()
            .context("taking the pty writer failed")?;

        let (tx, rx) = mpsc::channel::<Vec<u8>>();
        let reader_thread = thread::spawn(move || {
            let mut buf = [0u8; READ_BUF];
            loop {
                match reader.read(&mut buf) {
                    Ok(0) => break, // EOF: child closed the pty.
                    Ok(n) => {
                        if tx.send(buf[..n].to_vec()).is_err() {
                            break; // Receiver dropped: session is gone.
                        }
                    }
                    Err(e) if e.kind() == ErrorKind::Interrupted => continue,
                    Err(_) => break,
                }
            }
        });

        Ok(Self {
            master: pair.master,
            writer,
            child,
            rx,
            reader_thread: Some(reader_thread),
        })
    }

    /// Write bytes to the child's stdin (the encoded keystrokes / paste from
    /// [`crate::input`]).
    pub fn write_input(&mut self, bytes: &[u8]) -> IoResult<()> {
        self.writer.write_all(bytes)?;
        self.writer.flush()
    }

    /// Take the next pending output chunk, if any. Non-blocking.
    pub fn try_recv(&self) -> Option<Vec<u8>> {
        self.rx.try_recv().ok()
    }

    /// Drain *all* currently-pending output into one buffer, ready to feed
    /// straight into [`TerminalEmulator::process`]. Non-blocking; returns an
    /// empty `Vec` when nothing is waiting.
    pub fn drain(&self) -> Vec<u8> {
        let mut out = Vec::new();
        while let Ok(chunk) = self.rx.try_recv() {
            out.extend_from_slice(&chunk);
        }
        out
    }

    /// Resize the pty master **and** the paired emulator together — the two
    /// must move in lockstep or `vt100` reflows against a stale geometry.
    pub fn resize(
        &mut self,
        emulator: &mut TerminalEmulator,
        rows: u16,
        cols: u16,
    ) -> anyhow::Result<()> {
        self.master
            .resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .context("resizing the pty master failed")?;
        emulator.resize(rows, cols);
        Ok(())
    }

    /// Poll the child's exit status without blocking; `Ok(None)` while it is
    /// still running.
    pub fn try_wait(&mut self) -> IoResult<Option<ExitStatus>> {
        self.child.try_wait()
    }
}

impl Drop for PtySession {
    fn drop(&mut self) {
        // Terminate the child so the reader thread sees EOF and exits, then
        // join it so no thread outlives the session. Both are best-effort:
        // the child may already be dead.
        let _ = self.child.kill();
        if let Some(handle) = self.reader_thread.take() {
            let _ = handle.join();
        }
    }
}
