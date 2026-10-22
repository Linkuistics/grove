//! Public command construction observed through shells and installed Ravel.
use std::ffi::OsStr;
use std::fs;
use std::io::{Read, Seek};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

use grove_loop::ravel_job::{self, Shape};
use grove_loop::session_command::{self, Argv};
use grove_loop::{session_result::SessionResult, Handle, Kind, Selection};

mod support;

// Dropping shell quoting would split the executable or run the embedded touch.
#[test]
fn ending_command_executes_the_supplied_path_as_one_literal_word() {
    let temp = tempfile::tempdir().unwrap();
    let executable = temp
        .path()
        .join("selected ' ; touch INJECTED ; $tool\nfront");
    fs::write(
        &executable,
        "#!/bin/sh\nprintf '%s\\n' \"$#\" \"$1\" \"$2\"\n",
    )
    .unwrap();
    fs::set_permissions(&executable, fs::Permissions::from_mode(0o755)).unwrap();
    for (ending, expected) in [
        (
            session_command::end_session(&executable).unwrap(),
            b"2\nend-session\ngrove-continue\n".as_slice(),
        ),
        (
            session_command::force_stop(&executable).unwrap(),
            b"2\nend-session\ngrove-force-stop\n".as_slice(),
        ),
    ] {
        let output = Command::new("/bin/sh")
            .args(["-c", &ending])
            .env_clear()
            .env("PATH", "/does-not-exist")
            .current_dir(temp.path())
            .output()
            .unwrap();
        assert!(output.status.success(), "{output:?}");
        assert_eq!(output.stdout, expected);
        assert!(!temp.path().join("INJECTED").exists());
    }
}

#[test]
fn a_non_utf8_ending_path_refuses_before_a_prompt_can_be_published() {
    let path = Path::new(OsStr::from_bytes(b"/selected/\xff/ravel"));
    let error = session_command::end_session(path).unwrap_err();
    assert!(error.to_string().contains("UTF-8"), "{error}");
    let error = session_command::force_stop(path).unwrap_err();
    assert!(error.to_string().contains("UTF-8"), "{error}");
}

fn installed(temp: &Path) -> support::ravel::InstalledRavel {
    let source = std::env::var_os("GROVE_TEST_RAVEL").expect("GROVE_TEST_RAVEL is required");
    support::ravel::InstalledRavel::copy_from(
        Path::new(&source),
        &temp.join("separate ravel ' 0.1.0"),
    )
}

// env supplies test-only HOME/PATH for both runner APIs without global mutation.
fn invocation(home: &Path, path: &Path, request: Argv) -> Argv {
    let mut words = vec![
        format!("HOME={}", home.display()).into(),
        format!("PATH={}", path.display()).into(),
    ];
    words.extend(request.words());
    Argv::new("/usr/bin/env".into(), words)
}

// A private report flag, wrong task data, prompt trimming or PATH fallback makes
// the real front refuse or makes the observed policy/harness behavior disagree.
#[test]
#[ignore = "requires installed Ravel: task ravel:commands RAVEL=/absolute/path/to/ravel"]
fn lifecycle_request_preserves_public_inputs_and_uses_the_selected_ending_path() {
    for force_stop in [false, true] {
        let temp = tempfile::tempdir().unwrap();
        let pair = installed(temp.path());
        let home = temp.path().join("home");
        let policy = home.join(".config/ravel/policy.ts");
        fs::create_dir_all(policy.parent().unwrap()).unwrap();
        let observed = temp.path().join("observed.json");
        fs::write(&policy, format!(
        "import {{writeFileSync}} from 'node:fs';\n\
         export const policy = {{schemaVersion: 2, version: 'caller-commands', select(request) {{\n\
         writeFileSync({observed:?}, JSON.stringify(request));\n\
         return {{status: 'selected', program: '/bin/sh',\n\
         args: ['-c', 'printf managed-stdout; printf managed-stderr >&2; eval \"$1\"', 'fixture', request.prompt],\n\
         provider: 'fixture', model: 'none', effort: 'none', reason: 'public inputs'}}; }} }};\n"
    )).unwrap();
        let task = Selection {
            path: temp
                .path()
                .canonicalize()
                .unwrap()
                .join("-task path\nwith 'quotes'.md"),
            handle: Handle::parse("subject-k12").unwrap(),
            kind: Kind::new("impl").unwrap(),
        };
        let prompt = format!(
            "\n  {}\n\n",
            if force_stop {
                session_command::force_stop(pair.executable()).unwrap()
            } else {
                session_command::end_session(pair.executable()).unwrap()
            }
        );
        let exit_dir = temp.path().join("exit ' parent");
        fs::create_dir(&exit_dir).unwrap();
        // Once selected, both launch and ending must ignore a changed lookup path.
        let decoy_bin = temp.path().join("decoy-bin");
        fs::create_dir(&decoy_bin).unwrap();
        let decoy = decoy_bin.join("ravel");
        fs::write(&decoy, "#!/bin/sh\nprintf decoy-ran >&2\nexit 99\n").unwrap();
        fs::set_permissions(&decoy, fs::Permissions::from_mode(0o755)).unwrap();
        let request = session_command::lifecycle(pair.executable(), &task, &prompt, &exit_dir);
        let argv = invocation(&home, &decoy_bin, request);
        let capture = SessionResult::allocate().unwrap();
        let launch_dir = temp.path().join("launch directory");
        fs::create_dir(&launch_dir).unwrap();
        let ended = ravel_job::launch(
            &argv,
            temp.path(),
            capture.stdout().unwrap(),
            &mut |_| {},
            Shape::Foreground {
                launch_dir: &launch_dir,
            },
        )
        .unwrap();
        assert!(ended.status.success(), "{ended:?}");
        assert_eq!(
            capture.read().unwrap(),
            if force_stop {
                grove_loop::session_result::SessionDisposition::ForceStop
            } else {
                grove_loop::session_result::SessionDisposition::Continue
            }
        );
        let request: serde_json::Value =
            serde_json::from_slice(&fs::read(observed).unwrap()).unwrap();
        assert_eq!(request["kind"], "impl");
        assert_eq!(request["taskFile"], task.path.to_str().unwrap());
        assert_eq!(request["taskId"], "subject-k12");
        assert_eq!(request["prompt"], prompt);
        assert_eq!(request["params"], serde_json::json!({}));
    }
}

// Omitting --confine lets the denied control read the credential; omitting the
// grant prevents the clean case from completing. Stderr must stay a transcript.
#[test]
#[ignore = "requires installed Ravel: task ravel:commands RAVEL=/absolute/path/to/ravel"]
fn confined_request_grants_only_the_named_runtime_file_and_isolates_the_result() {
    let caller_dir = std::env::current_dir().unwrap().canonicalize().unwrap();
    let temp = tempfile::tempdir_in(&caller_dir).unwrap();
    let pair = installed(temp.path());
    let home = temp.path().join("home");
    let policy = home.join(".config/ravel/policy.ts");
    fs::create_dir_all(policy.parent().unwrap()).unwrap();
    let work = temp.path().join("staged work");
    fs::create_dir(&work).unwrap();
    let exit_dir = temp.path().join("control");
    fs::create_dir(&exit_dir).unwrap();
    let credential = temp.path().join("runtime credential");
    fs::write(&credential, "granted\n").unwrap();
    let script = format!(
        "set -eu\nif IFS= read -r word < {credential:?}; then \
         printf 'runtime:%s' \"$word\"; else printf runtime-denied; exit 41; fi\n\
         if test -r {policy:?}; then printf policy-leaked; exit 42; fi\n\
         printf managed-stderr >&2\neval \"$1\""
    );
    let selected = serde_json::json!({
        "status": "selected", "program": "/bin/sh", "args": ["-c", script, "fixture"],
        "provider": "fixture", "model": "none", "effort": "none", "reason": "confined command"
    });
    fs::write(&policy, format!(
        "export const policy = {{schemaVersion: 2, version: 'confined-caller', select(request) {{\n\
         if (request.kind !== 'release-notes' || request.taskFile !== undefined || request.taskId !== undefined) \
         return {{status:'refused', code:'wrong_inputs', message:'unexpected task data', remedy:'fix the caller'}};\n\
         const selected = {selected}; selected.args.push(request.prompt); return selected; }} }};"
    )).unwrap();
    for granted in [false, true] {
        let reads = if granted {
            vec![credential.strip_prefix(&caller_dir).unwrap().to_owned()]
        } else {
            vec![]
        };
        let prompt = session_command::end_session(pair.executable()).unwrap();
        let request = session_command::confined(
            pair.executable(),
            "release-notes",
            &prompt,
            &exit_dir,
            &reads,
        )
        .unwrap();
        let argv = invocation(&home, pair.bin(), request);
        let capture = SessionResult::allocate().unwrap();
        let mut transcript = tempfile::tempfile().unwrap();
        let ended = ravel_job::launch(
            &argv,
            &work,
            capture.stdout().unwrap(),
            &mut |_| {},
            Shape::Detached {
                transcript: transcript.try_clone().unwrap(),
            },
        )
        .unwrap();
        assert_eq!(ended.status.success(), granted, "{ended:?}");
        assert_eq!(capture.read().is_ok(), granted);
        transcript.rewind().unwrap();
        let mut text = String::new();
        transcript.read_to_string(&mut text).unwrap();
        if granted {
            assert!(text.contains("runtime:granted"), "{text}");
            assert!(text.contains("managed-stderr"), "{text}");
            assert!(!text.contains("policy-leaked"), "{text}");
        } else {
            assert!(text.contains("runtime-denied"), "{text}");
        }
    }
}
