//! A session on a fresh pseudo-terminal, for the cases where a controlling
//! terminal matters: who holds it, in what modes, and where a typed Ctrl-C
//! goes. Never the developer's terminal, so what a run does to the foreground
//! and the modes is the test's alone to observe.

use std::fs::File;
use std::io::{Read as _, Write as _};
use std::os::fd::{AsRawFd as _, FromRawFd as _};
use std::os::unix::process::CommandExt as _;
use std::process::{Command, ExitStatus, Stdio};
use std::time::{Duration, Instant};

/// The terminal's master side, for typing into the session.
pub struct Keyboard(File);

impl Keyboard {
    /// Type `bytes`, as a human at the terminal would: `\x03` is Ctrl-C.
    pub fn type_bytes(&mut self, bytes: &[u8]) {
        self.0.write_all(bytes).expect("type into the terminal");
    }
}

/// Run `command` as the leader of a new session on a fresh pseudo-terminal,
/// which becomes its controlling terminal and its three standard streams.
///
/// `during` runs once the session has started. Returns the leader's status
/// and everything written to the terminal, for a failure message. A session
/// still running at `deadline` is killed and fails the test.
pub fn in_pty(
    mut command: Command,
    deadline: Duration,
    during: impl FnOnce(&mut Keyboard),
) -> (ExitStatus, String) {
    let (mut master, mut slave) = (-1, -1);
    // SAFETY: valid out pointers; default terminal settings and size.
    assert_eq!(
        unsafe {
            libc::openpty(
                &mut master,
                &mut slave,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        },
        0,
        "openpty"
    );
    // SAFETY: openpty returned two fresh descriptors this function owns.
    let (master, slave) = unsafe { (File::from_raw_fd(master), File::from_raw_fd(slave)) };
    for fd in [master.as_raw_fd(), slave.as_raw_fd()] {
        // SAFETY: live descriptors; neither end should leak on exec.
        assert_ne!(
            unsafe { libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) },
            -1
        );
    }
    command
        .stdin(Stdio::from(slave.try_clone().unwrap()))
        .stdout(Stdio::from(slave.try_clone().unwrap()))
        .stderr(Stdio::from(slave));
    // SAFETY: only async-signal-safe calls between fork and exec.
    unsafe {
        command.pre_exec(|| {
            if libc::setsid() == -1
                || libc::ioctl(libc::STDIN_FILENO, libc::TIOCSCTTY as _, 0) == -1
            {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    let mut child = command.spawn().expect("the session leader starts");
    drop(command);
    let mut keyboard = Keyboard(master.try_clone().unwrap());
    // Drained throughout, so a full terminal buffer never stops the session.
    // The read ends in EIO once every holder of the slave is gone.
    let mut reader = master;
    let output = std::thread::spawn(move || {
        let mut output = Vec::new();
        let mut buffer = [0u8; 4096];
        while let Ok(read) = reader.read(&mut buffer) {
            if read == 0 {
                break;
            }
            output.extend_from_slice(&buffer[..read]);
        }
        String::from_utf8_lossy(&output).into_owned()
    });
    during(&mut keyboard);
    drop(keyboard);
    let until = Instant::now() + deadline;
    let status = loop {
        if let Some(status) = child.try_wait().unwrap() {
            break status;
        }
        if Instant::now() >= until {
            // SAFETY: the session this test started; its leader leads its
            // group.
            unsafe { libc::kill(-(child.id() as libc::pid_t), libc::SIGKILL) };
            let _ = child.wait();
            panic!("the session on the pseudo-terminal did not finish");
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    let until = Instant::now() + Duration::from_secs(5);
    while !output.is_finished() && Instant::now() < until {
        std::thread::sleep(Duration::from_millis(10));
    }
    let output = if output.is_finished() {
        output.join().unwrap()
    } else {
        String::from("(terminal output still open)")
    };
    (status, output)
}
