//! Separate stdout is transport; the runner does not interpret its bytes.
use std::fs::File;
use std::process::Command;
use std::time::Duration;

use keyed_launch::{Argv, End, Escalation, Group, Launch, NoninteractiveLaunch};

#[test]
#[ignore = "subprocess fixture driven by separate_stdout_leaves_only_stderr_in_the_log"]
fn supervisor() {
    let directory = std::path::PathBuf::from(std::env::var_os("CAPTURE_DIRECTORY").unwrap());
    let result_path = directory.join("result");
    // The caller controls positioning: preserve a prefix and append exactly
    // the child's bytes, including NUL and newline, without interpreting them.
    let mut result = File::create(&result_path).unwrap();
    std::io::Write::write_all(&mut result, b"prefix:").unwrap();
    let argv = Argv::new("/bin/sh".into(), vec!["-c".into(),
        "test ! -t 0 && test ! -t 1 && test ! -t 2 || exit 71; test -z \"$(cat)\" || exit 72; printf 'opaque\\000result\\n'; printf managed-output >&2; exit 7".into()]);
    let ended = keyed_launch::run_noninteractive_with_stdout(
        NoninteractiveLaunch {
            argv: &argv,
            scrub: &[],
            cwd: None,
            escalation: Escalation {
                grace: Duration::ZERO,
                kill_grace: Duration::from_secs(1),
            },
        },
        File::create(directory.join("log")).unwrap(),
        result,
    )
    .unwrap();
    assert_eq!(ended.end, End::Exited);
    assert_eq!(ended.status.code(), Some(7));
    assert_eq!(ended.group, Group::Gone);
    assert_eq!(
        std::fs::read(result_path).unwrap(),
        b"prefix:opaque\0result\n"
    );
    assert_eq!(
        std::fs::read(directory.join("log")).unwrap(),
        b"managed-output"
    );
}

#[test]
fn separate_stdout_leaves_only_stderr_in_the_log() {
    let directory = tempfile::tempdir().unwrap();
    let output = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "supervisor", "--ignored", "--nocapture"])
        .env("CAPTURE_DIRECTORY", directory.path())
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
}

#[test]
fn capture_rejects_nonregular_descriptors_before_spawn() {
    // A spawn of this program would fail with a different error. The observer
    // must receive no event, and both entry points must identify the file.
    let argv = Argv::new("/nonexistent-capture-fixture".into(), vec![]);
    let grace = Escalation {
        grace: Duration::ZERO,
        kill_grace: Duration::ZERO,
    };
    let mut events = Vec::new();
    let error = keyed_launch::run_observed_with_stdout(
        Launch {
            argv: &argv,
            channel: None,
            scrub: &[],
            grant: &[],
            transparent: None,
            cwd: None,
            escalation: grace,
        },
        File::options().write(true).open("/dev/null").unwrap(),
        &mut |event| events.push(event),
    )
    .unwrap_err();
    assert!(
        error.to_string().contains("stdout must be a regular file"),
        "{error}"
    );
    assert!(events.is_empty(), "{events:?}");
    let launch = || NoninteractiveLaunch {
        argv: &argv,
        scrub: &[],
        cwd: None,
        escalation: grace,
    };
    let error = keyed_launch::run_noninteractive_with_stdout(
        launch(),
        tempfile::tempfile().unwrap(),
        File::options().write(true).open("/dev/null").unwrap(),
    )
    .unwrap_err();
    assert!(
        error.to_string().contains("stdout must be a regular file"),
        "{error}"
    );
    let error = keyed_launch::run_noninteractive_with_stdout(
        launch(),
        File::options().write(true).open("/dev/null").unwrap(),
        tempfile::tempfile().unwrap(),
    )
    .unwrap_err();
    assert!(
        error.to_string().contains("log must be a regular file"),
        "{error}"
    );
}
