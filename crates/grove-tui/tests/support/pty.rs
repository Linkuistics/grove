//! Live PTY fixture shared by shipped-binary and test-only lifetime probes.
use std::{
    fs::File,
    io::{Read, Write},
    os::fd::{AsRawFd, FromRawFd},
    os::unix::process::CommandExt,
    process::{Child, Command, ExitStatus, Stdio},
    thread,
    time::{Duration, Instant},
};

pub struct Pty {
    pub child: Child,
    master: Option<File>,
    slave: File,
    before: libc::termios,
    before_flags: i32,
    pub output: Vec<u8>,
}

impl Pty {
    pub fn spawn(command: &mut Command) -> Self {
        let (mut master, mut slave) = (-1, -1);
        let mut size = libc::winsize {
            ws_row: 24,
            ws_col: 100,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        // SAFETY: valid out pointers; openpty initializes two owned descriptors.
        assert_eq!(
            unsafe {
                libc::openpty(
                    &mut master,
                    &mut slave,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::addr_of_mut!(size),
                )
            },
            0
        );
        for fd in [master, slave] {
            assert_ne!(
                unsafe { libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) },
                -1
            );
        }
        let (master, mut slave) = unsafe { (File::from_raw_fd(master), File::from_raw_fd(slave)) };
        // Prime macOS's kernel-maintained "was written" file-status bit before
        // taking the exact F_GETFL baseline; it is not changed by F_SETFL.
        slave.write_all(b"PTY fixture\n").unwrap();
        // Establish canonical baseline through tcsetattr too (macOS sets PENDIN).
        let mut baseline = attributes(&slave);
        baseline.c_lflag |= libc::PENDIN;
        assert_eq!(
            unsafe { libc::tcsetattr(slave.as_raw_fd(), libc::TCSANOW, &baseline) },
            0
        );
        let before = attributes(&slave);
        let before_flags = unsafe { libc::fcntl(slave.as_raw_fd(), libc::F_GETFL) };
        assert_ne!(before_flags, -1);
        // Nonblocking reads keep every readiness/exit wait bounded.
        assert_ne!(
            unsafe { libc::fcntl(master.as_raw_fd(), libc::F_SETFL, libc::O_NONBLOCK) },
            -1
        );
        // SAFETY: setsid is async-signal-safe; isolate from the agent terminal.
        // Leave the slave unowned so macOS does not revoke it on child exit.
        unsafe {
            command.pre_exec(|| {
                if libc::setsid() == -1 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            });
        }
        let child = command
            .env("TERM", "xterm-256color")
            .stdin(Stdio::from(slave.try_clone().unwrap()))
            .stdout(Stdio::from(slave.try_clone().unwrap()))
            .stderr(Stdio::from(slave.try_clone().unwrap()))
            .spawn()
            .unwrap();
        Self {
            child,
            master: Some(master),
            slave,
            before,
            before_flags,
            output: Vec::new(),
        }
    }

    pub fn drain(&mut self) {
        let Some(master) = self.master.as_mut() else {
            return;
        };
        let mut buffer = [0; 8192];
        // A chatty/broken child must not prevent the caller checking its deadline.
        for _ in 0..32 {
            match master.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => self.output.extend_from_slice(&buffer[..n]),
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e)
                    if e.kind() == std::io::ErrorKind::WouldBlock
                        || e.raw_os_error() == Some(libc::EIO) =>
                {
                    break
                }
                Err(e) => panic!("PTY read: {e}"),
            }
        }
    }

    pub fn until(&mut self, text: &str) {
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            self.drain();
            if String::from_utf8_lossy(&self.output).contains(text) {
                return;
            }
            assert!(
                Instant::now() < deadline,
                "missing {text:?}: {}",
                String::from_utf8_lossy(&self.output)
            );
            assert!(
                self.child.try_wait().unwrap().is_none(),
                "child exited before {text:?}: {}",
                String::from_utf8_lossy(&self.output)
            );
            thread::sleep(Duration::from_millis(10));
        }
    }

    pub fn send(&mut self, bytes: &[u8]) {
        self.master.as_mut().unwrap().write_all(bytes).unwrap();
    }

    pub fn signal(&self, signal: i32) {
        assert_eq!(unsafe { libc::kill(self.child.id() as i32, signal) }, 0);
    }

    pub fn resize(&self, rows: u16, columns: u16) {
        let size = libc::winsize {
            ws_row: rows,
            ws_col: columns,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        assert_eq!(
            unsafe { libc::ioctl(self.slave.as_raw_fd(), libc::TIOCSWINSZ, &size) },
            0
        );
        self.signal(libc::SIGWINCH);
    }

    pub fn close_master(&mut self) {
        self.master.take();
    }

    pub fn wait(&mut self) -> ExitStatus {
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            self.drain();
            if let Some(status) = self.child.try_wait().unwrap() {
                self.drain();
                return status;
            }
            assert!(
                Instant::now() < deadline,
                "child failed to exit: {}",
                String::from_utf8_lossy(&self.output)
            );
            thread::sleep(Duration::from_millis(10));
        }
    }

    pub fn assert_restored(&self) {
        assert_eq!(
            unsafe { libc::fcntl(self.slave.as_raw_fd(), libc::F_GETFL) },
            self.before_flags,
            "descriptor flags"
        );
        let after = attributes(&self.slave);
        assert_eq!(after.c_iflag, self.before.c_iflag, "input flags");
        assert_eq!(after.c_oflag, self.before.c_oflag, "output flags");
        assert_eq!(after.c_cflag, self.before.c_cflag, "control flags");
        assert_eq!(
            after.c_lflag, self.before.c_lflag,
            "local flags including ICANON/ECHO"
        );
        assert_eq!(after.c_cc, self.before.c_cc, "control characters");
        assert_eq!(unsafe { libc::cfgetispeed(&after) }, unsafe {
            libc::cfgetispeed(&self.before)
        });
        assert_eq!(unsafe { libc::cfgetospeed(&after) }, unsafe {
            libc::cfgetospeed(&self.before)
        });
        let output = String::from_utf8_lossy(&self.output);
        assert!(
            output.contains("\x1b[?1049l"),
            "missing leave alternate screen: {output}"
        );
        assert!(
            output.contains("\x1b[?25h"),
            "missing show cursor: {output}"
        );
    }
}

impl Drop for Pty {
    fn drop(&mut self) {
        // Reap on assertions/timeouts too; never leave a runaway child behind.
        if self.child.try_wait().ok().flatten().is_none() {
            let _ = self.child.kill();
        }
        let _ = self.child.wait();
    }
}

fn attributes(file: &File) -> libc::termios {
    let mut value = std::mem::MaybeUninit::uninit();
    assert_eq!(
        unsafe { libc::tcgetattr(file.as_raw_fd(), value.as_mut_ptr()) },
        0
    );
    unsafe { value.assume_init() }
}
