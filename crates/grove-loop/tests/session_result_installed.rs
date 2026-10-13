//! Exercise the shared reader with the independently installed public Ravel CLI.
use std::ffi::OsStr;
use std::fs;
use std::io::{Read, Seek};
use std::time::Duration;

use grove_loop::session_result::SessionResult;
use keyed_launch::{Argv, Escalation, Group, Launch, NoninteractiveLaunch};

#[test]
#[ignore = "requires installed Ravel: task ravel:result RAVEL=/absolute/path/to/ravel"]
fn both_stdout_runner_apis_read_only_the_installed_session_result() {
    let source = fs::canonicalize(
        std::env::var_os("GROVE_TEST_RAVEL").expect("GROVE_TEST_RAVEL is required"),
    )
    .unwrap();
    let source_prefix = source.parent().unwrap().parent().unwrap();
    let temp = tempfile::tempdir().unwrap();
    let prefix = temp.path().join("independent-ravel-0.1.0");
    fs::create_dir_all(prefix.join("bin")).unwrap();
    fs::create_dir_all(prefix.join("libexec/ravel")).unwrap();
    let ravel = prefix.join("bin/ravel");
    fs::copy(&source, &ravel).unwrap();
    fs::copy(
        source_prefix.join("libexec/ravel/ravel-policy"),
        prefix.join("libexec/ravel/ravel-policy"),
    )
    .unwrap();
    let version = std::process::Command::new(&ravel)
        .arg("--version")
        .output()
        .unwrap();
    assert!(version.status.success());
    assert_eq!(version.stdout, b"ravel 0.1.0\n");

    // Both runners scrub inherited live authority independently of Cargo's
    // guard. PATH contains only the copied pair: no host Bun/Node or old sibling.
    let scrub: Vec<&OsStr> = [
        "GROVE_SIGNAL_FILE",
        "GROVE_RUN_SIGNAL_FILE",
        "GROVE_LAUNCH_DIR",
        "HARNESS_DISPATCH_EXIT_FILE",
        "HARNESS_DISPATCH_RUN_ID",
        "HARNESS_DISPATCH_STATE_DIR",
        "RAVEL_EXIT_FILE",
    ]
    .iter()
    .map(OsStr::new)
    .collect();
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
                    format!("PATH={}", prefix.join("bin").display()).into(),
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
