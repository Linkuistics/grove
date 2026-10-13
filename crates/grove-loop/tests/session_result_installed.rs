//! Exercise the shared reader with the independently installed public Ravel CLI.
use std::ffi::OsStr;
use std::fs;
use std::io::{Read, Seek};
use std::time::Duration;

use grove_loop::session_result::SessionResult;
use keyed_launch::{Argv, Escalation, Group, Launch, NoninteractiveLaunch};

mod support;

#[test]
#[ignore = "requires installed Ravel: task ravel:fixtures RAVEL=/absolute/path/to/ravel"]
fn copied_pair_refuses_a_missing_worker_without_host_fallback() {
    let temp = tempfile::tempdir().unwrap();
    let prefix = temp.path().join("ravel-prefix");
    let source = std::env::var_os("GROVE_TEST_RAVEL").expect("GROVE_TEST_RAVEL is required");
    let pair = support::ravel::InstalledRavel::copy_from(std::path::Path::new(&source), &prefix);
    let policy = temp.path().join("policy.ts");
    let selected = serde_json::json!({
        "status": "selected", "program": "/bin/sh",
        "args": ["-c", "\"$1\" end-session grove-continue", "fixture", pair.executable()],
        "provider": "fixture", "model": "none", "effort": "none", "reason": "copied pair"
    });
    fs::write(&policy, format!("export const policy = {{schemaVersion: 2, version: 'fixture', select: () => ({selected})}};")).unwrap();
    let invoke = || {
        pair.command()
            .args([
                "start-session",
                "--kind=fixture",
                "--prompt=unchanged",
                "--config",
            ])
            .arg(&policy)
            .env("HOME", temp.path())
            .current_dir(temp.path())
            .output()
            .unwrap()
    };
    let worker = prefix.join("libexec/ravel/ravel-policy");
    let held = prefix.join("held-worker");
    fs::rename(&worker, &held).unwrap();
    let missing = invoke();
    assert_eq!(missing.status.code(), Some(5), "{missing:?}");
    assert!(missing.stdout.is_empty(), "{missing:?}");
    assert!(
        String::from_utf8_lossy(&missing.stderr).contains("ravel-policy"),
        "{missing:?}"
    );
    fs::rename(held, worker).unwrap();
    let complete = invoke();
    assert!(complete.status.success(), "{complete:?}");
    assert_eq!(complete.stdout, b"grove-continue");
}

#[test]
#[ignore = "requires installed Ravel: task ravel:fixtures RAVEL=/absolute/path/to/ravel"]
fn copied_pair_scrubs_inherited_authority_before_the_front() {
    let controls = [
        "GROVE_SIGNAL_FILE",
        "GROVE_RUN_SIGNAL_FILE",
        "GROVE_LAUNCH_DIR",
        "HARNESS_DISPATCH_EXIT_FILE",
        "HARNESS_DISPATCH_RUN_ID",
        "HARNESS_DISPATCH_STATE_DIR",
        "RAVEL_EXIT_FILE",
        "GROVE_HARNESS_PID",
        "GROVE_CLAUDE_PID",
    ];
    // Re-exec under real inherited controls; Cargo's guard cannot satisfy this.
    if std::env::var_os("GROVE_TEST_RAVEL_AUTHORITY_ROLE").is_none() {
        let mut child = std::process::Command::new(std::env::current_exe().unwrap());
        child
            .args([
                "--exact",
                "copied_pair_scrubs_inherited_authority_before_the_front",
                "--ignored",
                "--nocapture",
            ])
            .env("GROVE_TEST_RAVEL_AUTHORITY_ROLE", "child");
        for name in controls {
            child.env(name, "/inert-enclosing-fixture-control");
        }
        let output = child.output().unwrap();
        assert!(output.status.success(), "{output:?}");
        return;
    }
    for name in controls {
        assert_eq!(
            std::env::var(name).unwrap(),
            "/inert-enclosing-fixture-control"
        );
    }
    let temp = tempfile::tempdir().unwrap();
    let source = std::env::var_os("GROVE_TEST_RAVEL").expect("GROVE_TEST_RAVEL is required");
    let pair = support::ravel::InstalledRavel::copy_from(
        std::path::Path::new(&source),
        &temp.path().join("ravel-prefix"),
    );
    let policy = temp.path().join("policy.ts");
    let selected = serde_json::json!({
        "status": "selected", "program": "/bin/sh",
        "args": ["-c", "\"$1\" end-session grove-continue", "fixture", pair.executable()],
        "provider": "fixture", "model": "none", "effort": "none", "reason": "scrubbed pair"
    });
    fs::write(&policy, format!("export const policy = {{schemaVersion: 2, version: 'fixture', select: () => ({selected})}};")).unwrap();
    // Observe the environment at process entry, then exec the real installed
    // front. Ravel itself forbids owner grants for RAVEL_* and legacy dispatch
    // controls, so a policy-env probe could not measure their upstream scrub.
    let mut script = String::new();
    for name in controls {
        script.push_str(&format!("if [ \"${{{name}+present}}\" = present ]; then printf '{name} reached front' >&2; exit 97; fi\n"));
    }
    script.push_str("exec \"$@\"");
    let invocation = || {
        let mut command = std::process::Command::new("/bin/sh");
        pair.isolate(&mut command);
        command
            .args(["-c", &script, "fixture"])
            .arg(pair.executable())
            .args([
                "start-session",
                "--kind=fixture",
                "--prompt=unchanged",
                "--config",
            ])
            .arg(&policy)
            .env("HOME", temp.path())
            .current_dir(temp.path());
        command
    };
    // Each dirty control must make the same entry probe fail independently.
    for name in controls {
        let dirty = invocation()
            .env(name, "/inert-enclosing-fixture-control")
            .output()
            .unwrap();
        assert_eq!(dirty.status.code(), Some(97), "{name}: {dirty:?}");
        assert_eq!(dirty.stderr, format!("{name} reached front").as_bytes());
        assert!(dirty.stdout.is_empty(), "{name}: {dirty:?}");
    }
    let clean = invocation().output().unwrap();
    assert!(clean.status.success(), "{clean:?}");
    assert_eq!(clean.stdout, b"grove-continue");
}

#[test]
#[ignore = "requires installed Ravel: task ravel:result RAVEL=/absolute/path/to/ravel"]
fn both_stdout_runner_apis_read_only_the_installed_session_result() {
    let temp = tempfile::tempdir().unwrap();
    let prefix = temp.path().join("independent-ravel-0.1.0");
    let source = std::env::var_os("GROVE_TEST_RAVEL").expect("GROVE_TEST_RAVEL is required");
    let pair = support::ravel::InstalledRavel::copy_from(std::path::Path::new(&source), &prefix);
    let ravel = pair.executable();
    let version = pair.command().arg("--version").output().unwrap();
    assert!(version.status.success());
    assert_eq!(version.stdout, b"ravel 0.1.0\n");

    // Both runners scrub inherited live authority independently of Cargo's
    // guard. PATH contains only the copied pair: no host Bun/Node or old sibling.
    let names = support::grove_env_names();
    let scrub: Vec<&OsStr> = names.iter().map(OsStr::new).collect();
    for noninteractive in [false, true] {
        for (supplied, valid, successful) in [
            (Some("grove-continue"), true, true),
            (Some("grove-continue\n"), false, true),
            (Some(""), false, true),
            (None, false, false),
        ] {
            let script = if supplied.is_some() {
                r#"printf session-stdout-noise; printf session-stderr-noise >&2; "$1" end-session "$2""#
            } else {
                "printf session-stdout-noise; printf session-stderr-noise >&2; exit 0"
            };
            let selected = serde_json::json!({
                "status": "selected", "program": "/bin/sh",
                "args": ["-c", script, "fixture", ravel, supplied.unwrap_or("")],
                "provider": "fixture", "model": "fixture", "effort": "low",
                "reason": "installed public result boundary"
            });
            let policy = temp.path().join("policy.ts");
            fs::write(
                &policy,
                format!(
                    "import {{ definePolicy }} from 'ravel/sdk';\n\
                     export const policy = definePolicy({{schemaVersion: 2, \
                     version: 'grove-result-fixture', select: () => ({selected})}});\n"
                ),
            )
            .unwrap();
            // env execs the same public front, supplying an isolated PATH even
            // for NoninteractiveLaunch, which deliberately has no env grants.
            let argv = Argv::new(
                "/usr/bin/env".into(),
                vec![
                    format!("HOME={}", temp.path().display()).into(),
                    format!("PATH={}", pair.bin().display()).into(),
                    ravel.as_os_str().to_owned(),
                    "start-session".into(),
                    "--kind=fixture".into(),
                    "--prompt=unchanged".into(),
                    "--config".into(),
                    policy.into_os_string(),
                ],
            );
            let capture = SessionResult::allocate().unwrap();
            let escalation = Escalation {
                grace: Duration::from_secs(2),
                kill_grace: Duration::from_secs(5),
            };
            let mut transcript = tempfile::tempfile().unwrap();
            let ended = if noninteractive {
                keyed_launch::run_noninteractive_with_stdout(
                    NoninteractiveLaunch {
                        argv: &argv,
                        scrub: &scrub,
                        cwd: Some(temp.path()),
                        escalation,
                    },
                    transcript.try_clone().unwrap(),
                    capture.stdout().unwrap(),
                )
            } else {
                keyed_launch::run_observed_with_stdout(
                    Launch {
                        argv: &argv,
                        channel: None,
                        scrub: &scrub,
                        grant: &[],
                        transparent: None,
                        cwd: Some(temp.path()),
                        escalation,
                    },
                    capture.stdout().unwrap(),
                    &mut |_| {},
                )
            }
            .unwrap();
            assert!(!matches!(ended.group, Group::Present { .. }));
            assert_eq!(ended.status.success(), successful);
            assert_eq!(capture.read_continue().is_ok(), valid);
            if noninteractive {
                transcript.rewind().unwrap();
                let mut text = String::new();
                transcript.read_to_string(&mut text).unwrap();
                assert!(text.contains("session-stdout-noise"), "{text}");
                assert!(text.contains("session-stderr-noise"), "{text}");
            }
        }
    }
}
