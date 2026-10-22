//! The shared function that starts `ravel`, observed without a terminal.
//!
//! These cases check what a caller relies on: the notices, the environment, the
//! errors and the latch. They do not check job control. Only the installed Ravel
//! tests do that, because a stand-in child cannot prove what a real front does.
use std::fs::File;
use std::io::{Read, Seek};
use std::path::Path;
use std::process::Command;
use std::sync::Mutex;

use grove_loop::ravel_job::{self, JobEnd, JobError, LaunchEvent, Shape, CONTROL_ENV};
use grove_loop::session_command::Argv;

/// The latch is process-global, and every case below reads or writes it.
static LATCH: Mutex<()> = Mutex::new(());

fn held() -> File {
    tempfile::tempfile().unwrap()
}

fn argv(program: &str, args: &[&str]) -> Argv {
    Argv::new(program.into(), args.iter().map(Into::into).collect())
}

fn launch(
    command: &Argv,
    shape: Shape<'_>,
    stdout: File,
    events: &mut Vec<LaunchEvent>,
) -> Result<JobEnd, JobError> {
    ravel_job::launch(
        command,
        Path::new("/"),
        stdout,
        &mut |event| events.push(event),
        shape,
    )
}

fn read_back(mut file: File) -> String {
    file.rewind().unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}

// A name missing from the list hands a live authority to every child.
#[test]
fn the_scrub_helper_removes_each_control_name() {
    let mut command = Command::new("/usr/bin/env");
    for name in CONTROL_ENV {
        command.env(name, "inherited");
    }
    ravel_job::scrub(&mut command);
    let removed: Vec<_> = command
        .get_envs()
        .filter(|(_, value)| value.is_none())
        .map(|(name, _)| name.to_str().unwrap().to_owned())
        .collect();
    for name in CONTROL_ENV {
        assert!(removed.iter().any(|removed| removed == name), "{name}");
    }
    assert_eq!(CONTROL_ENV.len(), 10);
}

// Only the foreground shape grants the launch directory, and it grants it
// after the scrub that names it. The other nine names reach neither shape.
#[test]
fn only_the_foreground_shape_grants_the_launch_directory() {
    let _guard = LATCH.lock().unwrap();
    let directory = tempfile::tempdir().unwrap();
    let command = argv("/usr/bin/env", &[]);
    for detached in [false, true] {
        let stdout = held();
        let mut events = Vec::new();
        let ended = launch(
            &command,
            if detached {
                Shape::Detached { transcript: held() }
            } else {
                Shape::Foreground {
                    launch_dir: directory.path(),
                }
            },
            stdout.try_clone().unwrap(),
            &mut events,
        )
        .unwrap();
        assert!(ended.status.success(), "{ended:?}");
        let text = read_back(stdout);
        let granted = format!("GROVE_LAUNCH_DIR={}\n", directory.path().display());
        assert_eq!(text.contains(&granted), !detached, "{text}");
        for name in CONTROL_ENV
            .iter()
            .filter(|name| **name != "GROVE_LAUNCH_DIR")
        {
            assert!(!text.contains(&format!("{name}=")), "{name}: {text}");
        }
    }
}

// The order is the contract: the observer hears Started after the spawn and
// Reaped after the wait, and each only once.
#[test]
fn a_job_reports_started_then_reaped_once_each() {
    let _guard = LATCH.lock().unwrap();
    for detached in [false, true] {
        let mut events = Vec::new();
        let ended = launch(
            &argv("/usr/bin/true", &[]),
            if detached {
                Shape::Detached { transcript: held() }
            } else {
                Shape::Foreground {
                    launch_dir: Path::new("/"),
                }
            },
            held(),
            &mut events,
        )
        .unwrap();
        assert!(ended.status.success());
        assert_eq!(ended.interrupted, None);
        assert_eq!(events, [LaunchEvent::Started, LaunchEvent::Reaped]);
    }
}

// A failed spawn leaves the observer unheard. A Started with no child would
// pin a driver lease to a launch that never existed.
#[test]
fn a_failed_spawn_sends_no_notice_and_names_the_program() {
    let _guard = LATCH.lock().unwrap();
    let mut events = Vec::new();
    let error = launch(
        &argv("/does/not/exist/ravel", &[]),
        Shape::Foreground {
            launch_dir: Path::new("/"),
        },
        held(),
        &mut events,
    )
    .unwrap_err();
    assert!(events.is_empty(), "{events:?}");
    assert!(
        error.to_string().contains("/does/not/exist/ravel"),
        "{error}"
    );
    assert_eq!(error.interrupted(), None);
}

#[test]
fn a_stdout_that_is_not_a_regular_file_refuses_before_the_spawn() {
    let _guard = LATCH.lock().unwrap();
    let mut events = Vec::new();
    let error = launch(
        &argv("/usr/bin/true", &[]),
        Shape::Foreground {
            launch_dir: Path::new("/"),
        },
        File::open("/dev/null").unwrap(),
        &mut events,
    )
    .unwrap_err();
    assert!(events.is_empty(), "{events:?}");
    assert!(error.to_string().contains("regular file"), "{error}");
}

// A signal that arrives while no child exists is the driver's. The failed spawn
// that follows must hand it on in the error and must not spend it.
#[test]
fn a_caught_signal_survives_a_failed_spawn_in_the_error() {
    let _guard = LATCH.lock().unwrap();
    let mut events = Vec::new();
    // The first call installs the handlers.
    let _ = launch(
        &argv("/does/not/exist/ravel", &[]),
        Shape::Foreground {
            launch_dir: Path::new("/"),
        },
        held(),
        &mut events,
    );
    assert_eq!(ravel_job::take_interrupt(), None);
    // SAFETY: raising a signal that the first call caught with a latch handler.
    unsafe { libc::raise(libc::SIGTERM) };
    assert_eq!(ravel_job::take_interrupt(), Some(libc::SIGTERM));

    unsafe { libc::raise(libc::SIGHUP) };
    let error = launch(
        &argv("/does/not/exist/ravel", &[]),
        Shape::Detached { transcript: held() },
        held(),
        &mut events,
    )
    .unwrap_err();
    assert_eq!(error.interrupted(), Some(libc::SIGHUP));
    assert_eq!(ravel_job::take_interrupt(), None);
}
