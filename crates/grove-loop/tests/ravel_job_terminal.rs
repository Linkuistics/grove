//! The foreground handover of the shared function, against installed Ravel.
//!
//! A controlling terminal belongs to a session, and a test can make one only by
//! starting a session leader on a fresh pseudo-terminal. So the test re-runs its
//! own binary in a fixture role, and the role calls `launch`. The managed
//! harness reports which group holds the terminal while it runs.
//!
//! No stand-in child takes part. A stand-in cannot show what the real front does
//! with the terminal that Grove hands it.
use std::collections::HashMap;
use std::fs;
use std::io::Read as _;
use std::os::fd::{AsRawFd as _, FromRawFd as _};
use std::os::unix::process::CommandExt as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use grove_loop::ravel_job::{self, Shape};
use grove_loop::session_command::Argv;
use grove_loop::session_result::{SessionDisposition, SessionResult};

mod support;

fn var(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("the fixture needs {name}"))
}

/// The foreground group of the controlling terminal of this process.
fn foreground() -> libc::pid_t {
    // SAFETY: `open(2)` on a constant path. The descriptor is closed below.
    let tty = unsafe { libc::open(c"/dev/tty".as_ptr(), libc::O_RDWR | libc::O_NOCTTY) };
    assert!(tty >= 0, "the fixture has a controlling terminal");
    // SAFETY: `tcgetpgrp(3)` on a descriptor that this function owns.
    let group = unsafe { libc::tcgetpgrp(tty) };
    // SAFETY: closing the descriptor opened above.
    unsafe { libc::close(tty) };
    group
}

fn report(path: &Path) -> HashMap<String, String> {
    fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("no report at {}: {error}", path.display()))
        .split_whitespace()
        .filter_map(|word| word.split_once('='))
        .map(|(key, value)| (key.to_owned(), value.to_owned()))
        .collect()
}

/// The role: Grove as a foreground driver on a terminal of its own. It answers
/// only to `JOB_ROLE`, so a run of every ignored case passes it by.
#[test]
#[ignore = "subprocess fixture"]
fn launcher() {
    if std::env::var_os("JOB_ROLE").is_none() {
        return;
    }
    let work = PathBuf::from(var("JOB_WORK"));
    let argv = Argv::new(
        "/usr/bin/env".into(),
        vec![
            format!("HOME={}", work.display()).into(),
            format!("PATH={}", var("JOB_BIN")).into(),
            var("JOB_RAVEL").into(),
            "start-session".into(),
            "--kind=fixture".into(),
            "--prompt=unchanged".into(),
            "--config".into(),
            work.join("policy.ts").into_os_string(),
        ],
    );
    // SAFETY: `getpgrp(2)` takes no argument and cannot fail.
    let group = unsafe { libc::getpgrp() };
    let before = foreground();
    let capture = SessionResult::allocate().unwrap();
    let ended = ravel_job::launch(
        &argv,
        &work,
        capture.stdout().unwrap(),
        &mut |_| {},
        Shape::Foreground { launch_dir: &work },
    )
    .unwrap();
    let after = foreground();
    let continued =
        ended.status.success() && capture.read().unwrap() == SessionDisposition::Continue;
    fs::write(
        work.join("launcher"),
        format!("group={group} before={before} after={after} continued={continued}\n"),
    )
    .unwrap();
}

/// Run the role as the leader of a new session on a fresh pseudo-terminal, which
/// becomes its controlling terminal. Never the terminal of the developer.
fn in_pty(mut command: Command, deadline: Duration) -> String {
    let (mut master, mut slave) = (-1, -1);
    // SAFETY: valid out pointers, with the default settings and size.
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
        0
    );
    // SAFETY: `openpty` returned two fresh descriptors that nothing else owns.
    let (master, slave) = unsafe { (fs::File::from_raw_fd(master), fs::File::from_raw_fd(slave)) };
    for fd in [master.as_raw_fd(), slave.as_raw_fd()] {
        // SAFETY: live descriptors. Neither end should leak across the exec.
        assert_ne!(
            unsafe { libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) },
            -1
        );
    }
    command
        .stdin(Stdio::from(slave.try_clone().unwrap()))
        .stdout(Stdio::from(slave.try_clone().unwrap()))
        .stderr(Stdio::from(slave));
    // SAFETY: only async-signal-safe calls run between the fork and the exec.
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
    let mut child = command.spawn().unwrap();
    drop(command);
    // Drained throughout, so that a full terminal buffer never stops the role.
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
    let until = Instant::now() + deadline;
    let status = loop {
        if let Some(status) = child.try_wait().unwrap() {
            break status;
        }
        if Instant::now() >= until {
            // SAFETY: the session that this test started. Its leader leads its group.
            unsafe { libc::kill(-(child.id() as libc::pid_t), libc::SIGKILL) };
            let _ = child.wait();
            panic!("the role on the pseudo-terminal did not finish");
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    // A survivor that holds the slave would keep the reader blocked.
    let until = Instant::now() + Duration::from_secs(5);
    while !output.is_finished() && Instant::now() < until {
        std::thread::sleep(Duration::from_millis(10));
    }
    let output = if output.is_finished() {
        output.join().unwrap()
    } else {
        String::from("(terminal output still open)")
    };
    assert!(status.success(), "{status:?}: {output}");
    output
}

// The harness reports its own group and the group that holds its terminal. The
// handover of Grove makes `ravel` the foreground job, and `ravel` passes the
// terminal on to the job of the harness. A function that hands over nothing
// leaves Grove's group in the foreground, and `ravel`, a background job, then
// leaves it there. After the reap the terminal returns to the group of Grove.
#[test]
#[ignore = "requires installed Ravel: task ravel:job RAVEL=/absolute/path/to/ravel"]
fn the_foreground_shape_hands_the_harness_the_terminal_and_takes_it_back() {
    let temp = tempfile::tempdir().unwrap();
    let source = std::env::var_os("GROVE_TEST_RAVEL").expect("GROVE_TEST_RAVEL is required");
    let pair = support::ravel::InstalledRavel::copy_from(
        Path::new(&source),
        &temp.path().join("independent-ravel-1.1.0"),
    );
    let observed = temp.path().join("harness");
    let selected = serde_json::json!({
        "status": "selected", "program": "/bin/sh",
        "args": [
            "-c",
            "/bin/ps -o pgid= -o tpgid= -p \"$$\" > \"$2\"; \"$1\" end-session grove-continue",
            "fixture", pair.executable(), observed,
        ],
        "provider": "fixture", "model": "fixture", "effort": "low",
        "reason": "foreground handover"
    });
    fs::write(
        temp.path().join("policy.ts"),
        format!(
            "import {{ definePolicy }} from 'ravel/sdk';\n\
             export const policy = definePolicy({{schemaVersion: 2, \
             version: 'grove-terminal-fixture', select: () => ({selected})}});\n"
        ),
    )
    .unwrap();

    let mut role = Command::new(std::env::current_exe().unwrap());
    role.args(["--exact", "launcher", "--ignored", "--nocapture"])
        .env("JOB_ROLE", "launcher")
        .env("JOB_WORK", temp.path())
        .env("JOB_BIN", pair.bin())
        .env("JOB_RAVEL", pair.executable());
    let terminal = in_pty(role, Duration::from_secs(60));

    let launcher = report(&temp.path().join("launcher"));
    let group = &launcher["group"];
    assert_eq!(launcher["continued"], "true", "{terminal}");
    assert_eq!(
        &launcher["before"], group,
        "Grove starts in the foreground: {launcher:?}"
    );

    let harness: Vec<String> = fs::read_to_string(&observed)
        .unwrap()
        .split_whitespace()
        .map(str::to_owned)
        .collect();
    let [harness_group, held_by] = harness.as_slice() else {
        panic!("the harness reported {harness:?}");
    };
    assert_ne!(
        harness_group, group,
        "the harness shares the group of Grove: {launcher:?} {harness:?}"
    );
    assert_eq!(
        held_by, harness_group,
        "the group of the harness does not hold the terminal: {launcher:?} {harness:?}"
    );
    assert_eq!(
        &launcher["after"], group,
        "the terminal did not come back to Grove: {launcher:?} {harness:?}"
    );
}
