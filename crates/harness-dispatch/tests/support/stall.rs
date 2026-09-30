//! A run stalled between its record commit and its final cancellation check,
//! so a signal can be placed at the linearization point by a stall rather
//! than by timing (`docs/specs/harness-selection-and-execution.md`,
//! *Execution and authority*).
//!
//! The front's stderr is a pipe the test has already filled, so the handoff
//! notice it writes after the commit blocks until the test drains it. The test
//! waits until the store holds the run, signals, and only then drains, so the
//! signal always arrives after the commit and before the check. The same stall
//! unsignalled is the control.

use std::fs::File;
use std::io::{Read as _, Write as _};
use std::os::fd::{AsRawFd as _, FromRawFd as _, OwnedFd};
use std::os::unix::process::ExitStatusExt as _;
use std::process::{Child, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use rusqlite::{Connection, OpenFlags};

use super::probe::State;
use super::Sandbox;

/// Far past anything a stall waits for, so a stuck front fails the test.
const WATCHDOG: Duration = Duration::from_secs(30);

/// A pipe whose buffer is already full, for the front's stderr. Its write
/// end is blocking again by the time the front inherits it, so the front's
/// first write waits until the test reads.
struct Stall {
    read: File,
    write: Option<OwnedFd>,
    /// The bytes written to fill it, all newlines.
    filled: usize,
}

impl Stall {
    fn new() -> Stall {
        let mut fds = [0; 2];
        // SAFETY: pipe fills both descriptors, and each is owned once. Both
        // close on exec, so a front another test starts meanwhile holds no
        // end of this pipe; the front here gets its write end as stderr.
        let (read, write) = unsafe {
            assert_eq!(libc::pipe(fds.as_mut_ptr()), 0);
            for fd in fds {
                assert_eq!(libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC), 0);
            }
            (File::from_raw_fd(fds[0]), OwnedFd::from_raw_fd(fds[1]))
        };
        let set_nonblocking = |on: bool| {
            // SAFETY: fcntl on a descriptor this function owns.
            unsafe {
                let flags = libc::fcntl(write.as_raw_fd(), libc::F_GETFL);
                let flags = if on {
                    flags | libc::O_NONBLOCK
                } else {
                    flags & !libc::O_NONBLOCK
                };
                assert_eq!(libc::fcntl(write.as_raw_fd(), libc::F_SETFL, flags), 0);
            }
        };
        set_nonblocking(true);
        let mut writer = File::from(write.try_clone().unwrap());
        let mut filled = 0;
        for chunk in [4096, 1] {
            let bytes = vec![b'\n'; chunk];
            loop {
                match writer.write(&bytes) {
                    Ok(written) => filled += written,
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => break,
                    Err(error) => panic!("filling the pipe: {error}"),
                }
            }
        }
        drop(writer);
        set_nonblocking(false);
        Stall {
            read,
            write: Some(write),
            filled,
        }
    }

    fn stderr(&mut self) -> Stdio {
        Stdio::from(self.write.take().expect("one front per stall"))
    }

    /// Everything after the fill: read until every writer has closed.
    fn drain(mut self) -> String {
        let mut bytes = Vec::new();
        self.read.read_to_end(&mut bytes).unwrap();
        assert!(bytes.len() >= self.filled);
        String::from_utf8(bytes.split_off(self.filled)).unwrap()
    }
}

/// How many runs the store at `path` holds, or 0 while it cannot be read.
fn runs(path: &std::path::Path) -> i64 {
    Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .and_then(|store| store.query_row("SELECT count(*) FROM runs", [], |row| row.get(0)))
        .unwrap_or(0)
}

/// What a stalled run did.
pub struct Stalled {
    pub code: Option<i32>,
    pub signal: Option<i32>,
    /// The front's stderr, after the fill.
    pub stderr: String,
}

/// Run the front as `run` with `args`, its stderr stalled; once the default
/// store holds `runs_before + 1` runs, send `signal` if there is one, then
/// drain.
pub fn stalled(
    sandbox: &Sandbox,
    args: &[&str],
    caller: Option<&State>,
    signal: Option<i32>,
    runs_before: i64,
) -> Stalled {
    let mut stall = Stall::new();
    let mut command = sandbox.command();
    command.arg("run").args(args);
    if let Some(caller) = caller {
        caller.apply_to(&mut command);
    }
    command.stdout(Stdio::null()).stderr(stall.stderr());
    let mut front: Child = command.spawn().expect("the front starts");
    // The command owns this process's copy of the write end, and the drain
    // reads until every writer has closed.
    drop(command);

    let store = sandbox.default_store();
    let started = Instant::now();
    while runs(&store) <= runs_before {
        if started.elapsed() > WATCHDOG {
            let _ = front.kill();
            panic!("the front never committed its run");
        }
        if let Some(status) = front.try_wait().unwrap() {
            panic!("the front ended before committing: {status:?}");
        }
        thread::sleep(Duration::from_millis(5));
    }
    // Committed, and the notice cannot have been written: the pipe is full.
    assert!(
        front.try_wait().unwrap().is_none(),
        "the front did not stall"
    );
    if let Some(signal) = signal {
        let pid = libc::pid_t::try_from(front.id()).unwrap();
        // SAFETY: kill only sends a signal to this test's unreaped child.
        assert_eq!(unsafe { libc::kill(pid, signal) }, 0);
    }
    let stderr = stall.drain();
    let status = front.wait().unwrap();
    Stalled {
        code: status.code(),
        signal: status.signal(),
        stderr,
    }
}
