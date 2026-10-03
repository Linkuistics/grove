//! Real command and sandbox boundary, with deterministic harnesses only.
mod support;

use std::fs;
use std::os::unix::fs::symlink;

use serde_json::json;
use support::{executable, run, selecting, text, Sandbox};

fn policy(sandbox: &Sandbox, program: &str, args: &str) {
    let marker = sandbox.root.join("selected");
    sandbox.personal_policy(&format!(
        "import {{ writeFileSync }} from 'node:fs';\n{}",
        selecting("", &format!(
            "writeFileSync({:?}, 'selected'); return {{ program: {program}, args: {args}, status: 'selected', provider: 'fake', model: 'fake', effort: 'none', reason: 'probe' }};",
            text(&marker)
        ))
    ));
}

#[test]
fn protected_overlap_refuses_before_policy_evaluation_including_canonical_aliases() {
    for case in [
        "cwd",
        "exit",
        "settings",
        "alias",
        "runtime-alias",
        "inside",
        "scratch",
    ] {
        let sandbox = Sandbox::new();
        sandbox.settings(&json!({}));
        policy(&sandbox, "'/usr/bin/true'", "[]");
        let mut command = sandbox.command();
        command.args([
            "run",
            "--kind",
            "impl",
            "--prompt",
            "p",
            "--confine",
            "--json",
        ]);
        let policy_dir = sandbox.personal_path().parent().unwrap().to_owned();
        match case {
            "cwd" => {
                command.current_dir(&sandbox.home);
            }
            "exit" => {
                command.arg("--exit-dir").arg(&sandbox.home);
            }
            "settings" => {
                command.arg("--runtime-read").arg(sandbox.settings_path());
            }
            "alias" => {
                let alias = sandbox.root.join("alias");
                symlink(&policy_dir, &alias).unwrap();
                command.current_dir(alias);
            }
            "runtime-alias" => {
                let alias = sandbox.cwd.join("alias");
                symlink(sandbox.settings_path(), &alias).unwrap();
                command.arg("--runtime-read").arg(alias);
            }
            "inside" => {
                let nested = policy_dir.join("nested");
                fs::create_dir(&nested).unwrap();
                command.current_dir(nested);
            }
            "scratch" => {
                command.env("TMPDIR", &policy_dir);
            }
            _ => unreachable!(),
        }
        let refusal = run(&mut command).refusal(2);
        assert_eq!(
            refusal["error"]["code"], "confinement_overlap",
            "{case}: {refusal}"
        );
        assert!(refusal["error"]["message"]
            .as_str()
            .unwrap()
            .contains("protected"));
        assert!(
            !sandbox.root.join("selected").exists(),
            "{case}: selection ran"
        );
        assert!(!sandbox.default_store().exists(), "{case}: recorded");
    }
}

#[test]
fn unusable_runtime_grants_and_root_cwd_refuse_before_selection() {
    let sandbox = Sandbox::new();
    policy(&sandbox, "'/usr/bin/true'", "[]");
    for grant in [sandbox.cwd.clone(), sandbox.root.join("missing")] {
        let mut command = sandbox.command();
        command
            .args([
                "run",
                "--kind",
                "impl",
                "--prompt",
                "p",
                "--confine",
                "--json",
                "--runtime-read",
            ])
            .arg(grant);
        assert_eq!(
            run(&mut command).refusal(2)["error"]["code"],
            "confinement_unusable"
        );
    }
    let mut command = sandbox.command();
    command.current_dir("/").args([
        "run",
        "--kind",
        "impl",
        "--prompt",
        "p",
        "--confine",
        "--json",
    ]);
    assert_eq!(
        run(&mut command).refusal(2)["error"]["code"],
        "confinement_unusable"
    );
    assert!(!sandbox.root.join("selected").exists());
}

#[test]
fn runtime_reads_require_confinement() {
    let sandbox = Sandbox::new();
    let refusal = sandbox
        .run(&[
            "--kind",
            "impl",
            "--prompt",
            "p",
            "--runtime-read",
            "/bin/sh",
            "--json",
        ])
        .refusal(2);
    assert!(refusal["error"]["message"]
        .as_str()
        .unwrap()
        .contains("--confine"));
}

#[test]
fn confinement_preflight_can_reap_under_an_ignored_sigchld() {
    let sandbox = Sandbox::new();
    policy(&sandbox, "'/usr/bin/true'", "[]");
    let mut command = sandbox.command();
    command.args(["run", "--kind", "impl", "--prompt", "p", "--confine"]);
    support::probe::State::caller(&[libc::SIGCHLD], &[libc::SIGUSR1]).apply_to(&mut command);
    let ended = run(&mut command);
    assert_eq!(ended.code, Some(0), "{}", ended.stderr);
    assert!(sandbox.root.join("selected").exists());
}

#[test]
fn a_runtime_read_inside_a_writable_directory_still_denies_writes() {
    let sandbox = Sandbox::new();
    let credential = sandbox.file("credential", "original");
    let body = "if (printf changed > credential) 2>/dev/null; then exit 41; fi; if ln credential alias 2>/dev/null; then if (printf changed > alias) 2>/dev/null; then exit 42; fi; fi; test \"$(cat credential)\" = original";
    policy(&sandbox, "'/bin/sh'", &json!(["-c", body]).to_string());
    let mut command = sandbox.command();
    command
        .args([
            "run",
            "--kind",
            "impl",
            "--prompt",
            "p",
            "--confine",
            "--runtime-read",
        ])
        .arg(&credential);
    let ended = run(&mut command);
    assert_eq!(ended.code, Some(0), "{}", ended.stderr);
    assert_eq!(fs::read_to_string(credential).unwrap(), "original");
}

#[test]
fn a_writable_ending_file_refuses_before_selection_including_aliases() {
    for alias in [false, true] {
        let sandbox = Sandbox::new();
        policy(&sandbox, "'/usr/bin/true'", "[]");
        let directory = if alias {
            let path = sandbox.root.join("alias");
            symlink(&sandbox.cwd, &path).unwrap();
            path
        } else {
            sandbox.cwd.clone()
        };
        let mut command = sandbox.command();
        command
            .args([
                "run",
                "--kind",
                "impl",
                "--prompt",
                "p",
                "--confine",
                "--json",
                "--ending-file",
            ])
            .arg(directory.join("ending.json"));
        let refusal = run(&mut command).refusal(2);
        assert_eq!(refusal["error"]["code"], "confinement_overlap");
        assert!(!sandbox.root.join("selected").exists());
        assert!(!sandbox.default_store().exists());
    }
}

#[cfg(target_os = "macos")]
#[test]
fn an_unusable_backend_refuses_before_selection_and_records_nothing() {
    let sandbox = Sandbox::new();
    policy(&sandbox, "'/usr/bin/true'", "[]");
    let mut command = sandbox.command();
    command.args([
        "run",
        "--kind",
        "impl",
        "--prompt",
        "p",
        "--confine",
        "--json",
    ]);
    let mut outer = std::process::Command::new("/usr/bin/sandbox-exec");
    outer
        .args([
            "-p",
            "(version 1)(allow default)(deny system-mac-syscall (mac-policy-name \"Sandbox\"))",
        ])
        .arg(command.get_program())
        .args(command.get_args())
        .current_dir(command.get_current_dir().unwrap());
    for (name, value) in command.get_envs() {
        match value {
            Some(value) => {
                outer.env(name, value);
            }
            None => {
                outer.env_remove(name);
            }
        }
    }
    let refusal = run(&mut outer).refusal(2);
    assert_eq!(refusal["error"]["code"], "confinement_unusable");
    assert!(!sandbox.root.join("selected").exists());
    assert!(!sandbox.default_store().exists());
}

#[test]
fn protected_owner_paths_cannot_live_under_implicit_system_runtime_grants() {
    let sandbox = Sandbox::new();
    policy(&sandbox, "'/usr/bin/true'", "[]");
    let mut command = sandbox.command();
    command.args([
        "run",
        "--kind",
        "impl",
        "--prompt",
        "p",
        "--confine",
        "--json",
        "--state-dir",
        "/usr/share/harness-dispatch-test-state",
    ]);
    let refusal = run(&mut command).refusal(2);
    assert_eq!(refusal["error"]["code"], "confinement_overlap");
    assert!(!sandbox.root.join("selected").exists());
}

#[test]
fn a_native_runtime_filename_refuses_before_selection_rather_than_panicking_in_the_record() {
    use std::os::unix::ffi::OsStrExt as _;
    let sandbox = Sandbox::new();
    policy(&sandbox, "'/usr/bin/true'", "[]");
    let file = sandbox
        .root
        .join(std::ffi::OsStr::from_bytes(b"native-\xff"));
    match fs::write(&file, "credential") {
        Ok(()) => {}
        Err(error) if error.raw_os_error() == Some(libc::EILSEQ) => return,
        Err(error) => panic!("native filename: {error}"),
    }
    let mut command = sandbox.command();
    command
        .args([
            "run",
            "--kind",
            "impl",
            "--prompt",
            "p",
            "--confine",
            "--json",
            "--runtime-read",
        ])
        .arg(file);
    let refusal = run(&mut command).refusal(2);
    assert_eq!(refusal["error"]["code"], "confinement_unusable");
    assert!(!sandbox.root.join("selected").exists());
}

#[test]
fn a_real_confined_run_has_only_its_grants_environment_and_recorded_identity() {
    let sandbox = Sandbox::new();
    let credential = sandbox.root.join("credential");
    fs::write(&credential, "credential").unwrap();
    sandbox.settings(&json!({ "policyEnv": ["OWNER_SELECTION"] }));
    let harness = sandbox.bin.join("probe");
    let dispatch = std::path::Path::new(support::FRONT).canonicalize().unwrap();
    executable(
        &harness,
        &format!(
            r#"#!/bin/sh
set -eu
test -n "$HARNESS_DISPATCH_RUN_ID"
test "$LANG" = C
test "$LC_ALL" = C
test "${{SECRET-unset}}" = unset
test "${{OWNER_SELECTION-unset}}" = unset
test "${{GROVE_SIGNAL_FILE-unset}}" = unset
test "${{FAKE_HARNESS_RECORD-unset}}" = unset
test "$TMPDIR" = "$TMP"
test "$TMP" = "$TEMP"
test -d "$TMPDIR"
if (printf leaked >&7) 2>/dev/null; then exit 45; fi
test "$(cat '{}')" = credential
if (printf changed > '{}') 2>/dev/null; then exit 41; fi
for path in '{}' '{}' "$HARNESS_DISPATCH_STATE_DIR/records.sqlite3"; do
  if cat "$path" 2>/dev/null; then exit 42; fi
done
if (printf outside > '{}') 2>/dev/null; then exit 43; fi
test -z "$(cat)"
if (: </dev/tty) 2>/dev/null; then exit 44; fi
printf temporary > "$TMPDIR/test"
printf '%s' "$HARNESS_DISPATCH_RUN_ID" > run-id
printf 'probe-output\n'
printf 'probe-error\n' >&2
'{}' exit
"#,
            text(&credential),
            text(&credential),
            text(&sandbox.personal_path()),
            text(&sandbox.settings_path()),
            text(&sandbox.root.join("outside")),
            text(&dispatch)
        ),
    );
    // The selected PATH match is explicit; another file of the same name in
    // cwd cannot replace it. The policy sees its grant outside confinement.
    executable(&sandbox.cwd.join("probe"), "#!/bin/sh\nexit 99\n");
    policy(&sandbox, "'probe'", "[]");
    let entry = fs::read_to_string(sandbox.personal_path()).unwrap();
    fs::write(sandbox.personal_path(), entry.replace("return { program", "if (process.env.OWNER_SELECTION !== 'allowed') throw new Error('missing owner grant'); return { program")).unwrap();
    let mut command = sandbox.command();
    let inherited = fs::File::create(sandbox.root.join("inherited-descriptor")).unwrap();
    support::caller_leaves_open(&mut command, Some((&inherited, 7)));
    command
        .env("SECRET", "private")
        .env("OWNER_SELECTION", "allowed")
        .env("GROVE_SIGNAL_FILE", "inert-test-value")
        .env("LANG", "C")
        .env("LC_ALL", "C")
        .args([
            "run",
            "--kind",
            "impl",
            "--prompt",
            "p",
            "--confine",
            "--runtime-read",
        ])
        .arg(&credential);
    let ended = run(&mut command);
    assert_eq!(ended.code, Some(0), "{}", ended.stderr);
    assert_eq!(ended.stdout, "probe-output\n");
    assert!(ended.stderr.contains("probe-error\n"));
    let id = fs::read_to_string(sandbox.cwd.join("run-id")).unwrap();
    let mut show = sandbox.command();
    show.args(["record", "show", "--run", &id, "--json"]);
    let record = run(&mut show).report();
    assert_eq!(
        record["launch"]["confinement"],
        json!({ "runtimeRead": [text(&credential)] })
    );
    assert_eq!(record["evidence"], "execution_confirmed");
    assert!(!sandbox.root.join("outside").exists());
    assert_eq!(
        fs::read(sandbox.root.join("inherited-descriptor")).unwrap(),
        b""
    );
    assert_eq!(fs::read_to_string(&credential).unwrap(), "credential");
    assert!(
        fs::read_dir(&sandbox.tmp).unwrap().next().is_none(),
        "private run directory leaked"
    );
}

#[test]
fn the_selected_executable_is_exempt_even_beside_the_policy() {
    let sandbox = Sandbox::new();
    policy(&sandbox, "'/bin/sh'", "[]");
    let harness = sandbox
        .personal_path()
        .parent()
        .unwrap()
        .join("exempt-harness");
    executable(&harness, "#!/bin/sh\nprintf exempt\n");
    policy(&sandbox, &json!(text(&harness)).to_string(), "[]");
    let ended = sandbox.run(&["--kind", "impl", "--prompt", "p", "--confine"]);
    assert_eq!(ended.code, Some(0), "{}", ended.stderr);
    assert_eq!(ended.stdout, "exempt");
}
