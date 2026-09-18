//! Exercise the operating-system boundary, not just the generated policy text.
use std::fs;
use std::time::Duration;

use keyed_launch::{Channel, Escalation, Launch, Templates, Vocabulary};

#[test]
fn confined_child_and_descendants_cannot_read_or_write_outside_the_invocation() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();
    let work = root.join("work");
    fs::create_dir(&work).unwrap();
    let secret = root.join("parent-project.txt");
    fs::write(&secret, "parent contents").unwrap();
    std::os::unix::fs::symlink(&secret, work.join("link")).unwrap();
    let script = work.join("probe.sh");
    fs::write(&script, format!(
        "set -eu\nif cat '{}' ; then exit 41; fi\nif cat link; then exit 42; fi\nif sh -c 'echo changed > \"$1\"' probe '{}'; then exit 43; fi\nprintf allowed > result\nprintf done > \"$TEST_CHANNEL\"\n",
        secret.display(), secret.display()
    )).unwrap();
    let config = root.join("config.kdl");
    let template = format!("/bin/sh '{}'", script.display());
    fs::write(&config, format!("config {{\ncommand \"probe\" {template:?}\nbind \"probe\" \"probe\"\nroute \"probe\" \"probe\"\n}}\n")).unwrap();
    let templates = Templates::load(&config, None, Vocabulary { slots: &[] }).unwrap();
    let argv = templates.expand("probe", &[]).unwrap();
    let channel = Channel::allocate(&work).unwrap();
    let log = root.join("log");
    let ended = keyed_launch::run_confined(
        Launch {
            argv: &argv,
            channel: &channel,
            channel_var: "TEST_CHANNEL",
            scrub: &[],
            cwd: Some(&work),
            escalation: Escalation {
                grace: Duration::ZERO,
                kill_grace: Duration::from_millis(100),
            },
        },
        fs::File::create(&log).unwrap(),
        &keyed_launch::Confinement {
            writable: &work,
            runtime_read: &[],
        },
    )
    .unwrap();
    assert!(
        ended.status.success(),
        "sandbox failed: {:?}\n{}",
        ended.status,
        fs::read_to_string(log).unwrap()
    );
    assert_eq!(ended.token.unwrap().as_str(), "done");
    assert_eq!(fs::read_to_string(secret).unwrap(), "parent contents");
    assert_eq!(fs::read_to_string(work.join("result")).unwrap(), "allowed");
}

#[test]
fn explicit_runtime_file_grant_allows_reads_but_denies_writes() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();
    let work = root.join("work");
    fs::create_dir(&work).unwrap();
    let runtime_file = root.join("runtime-credential.txt");
    fs::write(&runtime_file, "runtime contents").unwrap();
    let script = work.join("runtime-probe.sh");
    fs::write(
        &script,
        "set -eu\n\
         test \"$(cat \"$1\")\" = 'runtime contents'\n\
         if sh -c 'printf changed > \"$1\"' probe \"$1\"; then exit 41; fi\n\
         if sh -c 'rm \"$1\"' probe \"$1\"; then exit 42; fi\n\
         printf read-only > result\n\
         printf done > \"$TEST_CHANNEL\"\n",
    )
    .unwrap();
    let config = root.join("config.kdl");
    let template = format!(
        "/bin/sh '{}' '{}'",
        script.display(),
        runtime_file.display()
    );
    fs::write(
        &config,
        format!("config {{\ncommand \"probe\" {template:?}\nbind \"probe\" \"probe\"\nroute \"probe\" \"probe\"\n}}\n"),
    )
    .unwrap();
    let templates = Templates::load(&config, None, Vocabulary { slots: &[] }).unwrap();
    let argv = templates.expand("probe", &[]).unwrap();
    let channel = Channel::allocate(&work).unwrap();
    let log = root.join("log");
    let ended = keyed_launch::run_confined(
        Launch {
            argv: &argv,
            channel: &channel,
            channel_var: "TEST_CHANNEL",
            scrub: &[],
            cwd: Some(&work),
            escalation: Escalation {
                grace: Duration::ZERO,
                kill_grace: Duration::from_millis(100),
            },
        },
        fs::File::create(&log).unwrap(),
        &keyed_launch::Confinement {
            writable: &work,
            runtime_read: std::slice::from_ref(&runtime_file),
        },
    )
    .unwrap();
    assert!(
        ended.status.success(),
        "sandbox failed: {:?}\n{}",
        ended.status,
        fs::read_to_string(log).unwrap()
    );
    assert_eq!(ended.token.unwrap().as_str(), "done");
    assert_eq!(
        fs::read_to_string(runtime_file).unwrap(),
        "runtime contents"
    );
    assert_eq!(
        fs::read_to_string(work.join("result")).unwrap(),
        "read-only"
    );
}
