//! `run` through the command seam: prompt and task inputs, argument slots,
//! program resolution, and the plain exec that hands the caller's process to
//! the fake harness.
//!
//! The fake harness records its arguments, physical cwd and PID, so a test can
//! show that it received the exact words, ran where the caller ran, and is the
//! caller's own process rather than a child of a supervisor.

mod support;

use std::fs;
use std::io::{Read as _, Write as _};
use std::os::fd::AsRawFd as _;
use std::os::unix::ffi::OsStrExt as _;
use std::os::unix::process::{CommandExt as _, ExitStatusExt as _};
use std::process::Stdio;
use std::time::{Duration, Instant};

use serde_json::Value;
use support::{executable, run, text, Sandbox, FAKE_HARNESS};

/// A policy with one candidate per routed kind, each `{ id, program, args }`
/// given as a TypeScript object literal fragment.
fn policy(candidates: &[(&str, &str, &str)]) -> String {
    let catalog: Vec<String> = candidates
        .iter()
        .map(|(id, program, args)| {
            format!(
                r#"{{ id: "{id}", provider: "origin-{id}", model: "model-{id}", effort: "effort-{id}", program: {program}, args: {args} }}"#
            )
        })
        .collect();
    let routes: Vec<String> = candidates
        .iter()
        .map(|(id, _, _)| format!(r#""{id}": "{id}""#))
        .collect();
    format!(
        "export const policy = {{ schemaVersion: 1, version: \"run-1\", catalog: [{}], routes: {{ {} }} }};\n",
        catalog.join(", "),
        routes.join(", ")
    )
}

/// Every slot, among literals a shell would have mangled.
const EVERY_SLOT: &str = r#"["--model", { slot: "model" }, "--effort", { slot: "effort" }, "--kind", { slot: "kind" }, "--task", { slot: "taskFile" }, "--id", { slot: "taskId" }, "literal $HOME {prompt} 'single' \"double\" ; & | `tick` *", { slot: "prompt" }]"#;

const AWKWARD_PROMPT: &str =
    "Fix the \"parser\"; don't `rm -rf` $HOME && echo 'done' | tee *\n\nsecond line\t$(date)\n\n";

#[test]
fn run_hands_the_exact_argv_to_the_harness_in_the_callers_own_process_and_cwd() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&policy(&[("impl", r#""fake-harness""#, EVERY_SLOT)]));
    let cwd = sandbox
        .cwd
        .join("dir with spaces 'single' \"double\" ; & | $x\nand a newline");
    fs::create_dir_all(cwd.join("tasks")).unwrap();
    let task_id = "harness-exec-k14 with spaces \"and quotes\" ; $x";

    let mut command = sandbox.command();
    command.current_dir(&cwd).args([
        "run",
        "--kind",
        "impl",
        "--prompt",
        AWKWARD_PROMPT,
        "--task-file",
        "tasks/leaf one's \"task\"; $(x).md",
        "--task-id",
        task_id,
    ]);
    let child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let pid = child.id();
    let output = child.wait_with_output().unwrap();

    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        sandbox.harness_args(),
        [
            "--model",
            "model-impl",
            "--effort",
            "effort-impl",
            "--kind",
            "impl",
            "--task",
            &text(&cwd.join("tasks/leaf one's \"task\"; $(x).md")),
            "--id",
            task_id,
            "literal $HOME {prompt} 'single' \"double\" ; & | `tick` *",
            AWKWARD_PROMPT,
        ]
    );
    assert_eq!(sandbox.harness_cwd(), cwd);
    assert_eq!(
        sandbox.harness_pid(),
        pid,
        "the harness replaced the front rather than running as its child"
    );
    assert_eq!(
        output.stdout, b"",
        "the front wrote to the harness's stdout"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.starts_with("harness-dispatch: running candidate \"impl\"")
            && stderr.matches('\n').count() == 1,
        "one short choice line on stderr: {stderr}"
    );
}

#[test]
fn a_prompt_file_is_read_once_with_its_exact_bytes_from_the_callers_cwd() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&policy(&[(
        "impl",
        r#""fake-harness""#,
        r#"[{ slot: "prompt" }]"#,
    )]));
    sandbox.file("prompts/mandate.md", AWKWARD_PROMPT);

    let run = sandbox.run(&["--kind", "impl", "--prompt-file", "prompts/mandate.md"]);

    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert_eq!(sandbox.harness_args(), [AWKWARD_PROMPT]);
}

#[test]
fn the_harness_keeps_the_callers_stdin_stdout_and_other_descriptors() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&policy(&[(
        "impl",
        r#""fake-harness""#,
        r#"[{ slot: "prompt" }]"#,
    )]));
    let through = sandbox.root.join("descriptor-7");
    let file = fs::File::create(&through).unwrap();
    let fd = file.as_raw_fd();

    let mut command = sandbox.command();
    command
        .args(["run", "--kind", "impl", "--prompt", "p"])
        .env("FAKE_HARNESS_FD7", "written through descriptor 7")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    // SAFETY: dup2 is async-signal-safe; it hands the front a descriptor 7
    // that its caller left open for the harness.
    unsafe {
        command.pre_exec(move || {
            if libc::dup2(fd, 7) == -1 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    let mut child = command.spawn().unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"caller input for the harness\n")
        .unwrap();
    let output = child.wait_with_output().unwrap();

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "caller input for the harness\n",
        "stdin reached the harness whole, and nothing else reached stdout"
    );
    assert_eq!(
        fs::read_to_string(&through).unwrap(),
        "written through descriptor 7\n"
    );
}

#[test]
fn the_harness_exits_with_its_own_code_and_signal() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&policy(&[(
        "impl",
        r#""fake-harness""#,
        r#"[{ slot: "prompt" }]"#,
    )]));

    // 3 is also a preflight exit; after exec it is simply the harness's own.
    for code in [42, 3, 0] {
        fs::remove_dir_all(&sandbox.record).ok();
        let mut command = sandbox.command();
        command
            .args(["run", "--kind", "impl", "--prompt", "p"])
            .env("FAKE_HARNESS_EXIT", code.to_string());
        let run = run(&mut command);
        assert_eq!(run.code, Some(code), "{}", run.stderr);
        assert!(!run.stderr.contains("refused"), "{}", run.stderr);
    }

    fs::remove_dir_all(&sandbox.record).ok();
    let mut command = sandbox.command();
    command
        .args(["run", "--kind", "impl", "--prompt", "p"])
        .env("FAKE_HARNESS_SIGNAL", "TERM");
    let status = command.status().unwrap();
    assert_eq!(status.signal(), Some(libc::SIGTERM), "{status:?}");
}

#[test]
fn run_requires_exactly_one_prompt_input() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&policy(&[(
        "impl",
        r#""fake-harness""#,
        r#"[{ slot: "prompt" }]"#,
    )]));
    let file = sandbox.file("prompt.md", "p");

    let refusal = sandbox.run(&["--kind", "impl", "--json"]).refusal(2);
    assert_eq!(refusal["error"]["code"], "malformed_input");
    assert_eq!(refusal["error"]["input"], "--prompt");

    let refusal = sandbox
        .run(&[
            "--kind",
            "impl",
            "--prompt",
            "p",
            "--prompt-file",
            &text(&file),
            "--json",
        ])
        .refusal(2);
    assert_eq!(refusal["error"]["code"], "malformed_input");
    assert!(!sandbox.harness_ran());

    // The positive control: each alone launches.
    assert_eq!(
        sandbox.run(&["--kind", "impl", "--prompt", "p"]).code,
        Some(0)
    );
    fs::remove_dir_all(&sandbox.record).unwrap();
    let run = sandbox.run(&["--kind", "impl", "--prompt-file", &text(&file)]);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
}

#[test]
fn a_prompt_that_starts_with_hyphens_is_still_the_prompt() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&policy(&[(
        "impl",
        r#""fake-harness""#,
        r#"[{ slot: "prompt" }]"#,
    )]));
    let run = sandbox.run(&["--kind", "impl", "--prompt", "--json is not a flag here"]);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert_eq!(sandbox.harness_args(), ["--json is not a flag here"]);
}

#[test]
fn an_invalid_or_unreadable_prompt_is_refused_before_policy_runs() {
    let sandbox = Sandbox::new();
    let sentinel = sandbox.root.join("policy-ran");
    sandbox.personal_policy(&format!(
        "import {{ writeFileSync }} from \"node:fs\";\nwriteFileSync({:?}, \"fired\");\n{}",
        text(&sentinel),
        policy(&[("impl", r#""fake-harness""#, r#"[{ slot: "prompt" }]"#)])
    ));
    let limit = 1024 * 1024;
    let write = |name: &str, bytes: &[u8]| {
        let path = sandbox.cwd.join(name);
        fs::write(&path, bytes).unwrap();
        text(&path)
    };
    let cases = [
        (
            "too large",
            write("large", &vec![b'a'; limit + 1]),
            "prompt_invalid",
        ),
        ("not UTF-8", write("latin1", b"caf\xe9\n"), "prompt_invalid"),
        ("a NUL", write("nul", b"before\0after"), "prompt_invalid"),
        (
            "missing",
            text(&sandbox.cwd.join("absent.md")),
            "prompt_unreadable",
        ),
        ("a directory", text(&sandbox.cwd), "prompt_unreadable"),
    ];
    for (name, path, code) in cases {
        let refusal = sandbox
            .run(&["--kind", "impl", "--prompt-file", &path, "--json"])
            .refusal(2);
        assert_eq!(refusal["error"]["code"], code, "{name}: {refusal}");
        assert_eq!(refusal["error"]["stage"], "cli", "{name}");
        assert_eq!(refusal["error"]["input"], "--prompt-file", "{name}");
        assert_eq!(refusal["error"]["source"], path, "{name}");
    }
    // A command-line prompt that is not UTF-8.
    let mut command = sandbox.command();
    command.args(["run", "--kind", "impl", "--json", "--prompt"]);
    command.arg(std::ffi::OsStr::from_bytes(b"caf\xe9"));
    let refusal = run(&mut command).refusal(2);
    assert_eq!(refusal["error"]["code"], "prompt_invalid");
    assert_eq!(refusal["error"]["input"], "--prompt");

    assert!(
        !sentinel.exists(),
        "policy ran before the prompt was checked"
    );
    assert!(!sandbox.harness_ran());

    // The positive control: exactly 1 MiB of valid UTF-8 is accepted. It goes
    // through `inspect`, since one argument that long exceeds exec's own
    // limits on both supported platforms.
    let path = write("limit", &vec![b'a'; limit]);
    let report = sandbox
        .inspect(&["--kind", "impl", "--prompt-file", &path, "--json"])
        .report();
    assert_eq!(report["prompt"]["bytes"], limit);
    assert!(sentinel.exists(), "the control never evaluated the policy");
}

#[test]
fn a_terminal_is_never_read_as_the_prompt() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&policy(&[(
        "impl",
        r#""fake-harness""#,
        r#"[{ slot: "prompt" }]"#,
    )]));
    // The controlling side stays open and writes nothing. A front that read
    // the terminal would block on it, which the deadline below turns into a
    // failure rather than a hung test.
    let (_master, terminal) = pseudo_terminal();

    let mut command = sandbox.command();
    command
        .args([
            "run",
            "--kind",
            "impl",
            "--prompt-file",
            &terminal,
            "--json",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn().unwrap();
    let started = Instant::now();
    while child.try_wait().unwrap().is_none() {
        if started.elapsed() > Duration::from_secs(20) {
            child.kill().unwrap();
            panic!("the front blocked reading the terminal named as its prompt");
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    let mut stderr = String::new();
    child
        .stderr
        .take()
        .unwrap()
        .read_to_string(&mut stderr)
        .unwrap();
    let status = child.wait().unwrap();
    assert_eq!(status.code(), Some(2), "{stderr}");
    let refusal: Value = serde_json::from_str(&stderr).unwrap();
    assert_eq!(refusal["error"]["code"], "prompt_unreadable");
    assert!(
        refusal["error"]["message"]
            .as_str()
            .unwrap()
            .contains("terminal"),
        "{refusal}"
    );
    assert!(!sandbox.harness_ran());
}

/// A pseudo-terminal's controlling side and the path of its terminal side.
fn pseudo_terminal() -> (std::os::fd::OwnedFd, String) {
    use std::os::fd::FromRawFd as _;
    // SAFETY: each call's result is checked; `ptsname` returns a pointer to a
    // NUL-terminated name that is copied before any other call can reuse it.
    unsafe {
        let master = libc::posix_openpt(libc::O_RDWR | libc::O_NOCTTY);
        assert!(
            master >= 0,
            "posix_openpt: {}",
            std::io::Error::last_os_error()
        );
        let master = std::os::fd::OwnedFd::from_raw_fd(master);
        assert_eq!(libc::grantpt(master.as_raw_fd()), 0);
        assert_eq!(libc::unlockpt(master.as_raw_fd()), 0);
        let name = libc::ptsname(master.as_raw_fd());
        assert!(!name.is_null());
        let name = std::ffi::CStr::from_ptr(name).to_str().unwrap().to_owned();
        (master, name)
    }
}

#[test]
fn an_absent_optional_input_satisfies_no_slot() {
    let sandbox = Sandbox::new();
    let entry = sandbox.personal_policy(&policy(&[
        (
            "file",
            r#""fake-harness""#,
            r#"[{ slot: "prompt" }, { slot: "taskFile" }]"#,
        ),
        (
            "id",
            r#""fake-harness""#,
            r#"["--id", { slot: "taskId" }, { slot: "prompt" }]"#,
        ),
    ]));

    for (kind, flag, location) in [
        ("file", "--task-file", "policy.catalog[0].args[1]"),
        ("id", "--task-id", "policy.catalog[1].args[1]"),
    ] {
        for command in ["run", "inspect"] {
            let mut invocation = sandbox.command();
            invocation.args([command, "--kind", kind, "--prompt", "p", "--json"]);
            let refusal = run(&mut invocation).refusal(3);
            let error = &refusal["error"];
            assert_eq!(error["code"], "missing_input", "{kind} {command}");
            assert_eq!(error["stage"], "expansion", "{kind} {command}");
            assert_eq!(error["input"], flag, "{kind} {command}");
            assert_eq!(error["location"], location, "{kind} {command}");
            assert_eq!(error["source"], text(&entry), "{kind} {command}");
        }
    }
    assert!(!sandbox.harness_ran());

    // The positive control: the same candidates with their inputs launch.
    let run = sandbox.run(&["--kind", "file", "--prompt", "p", "--task-file", "t.md"]);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert_eq!(
        sandbox.harness_args(),
        ["p", &text(&sandbox.cwd.join("t.md"))]
    );
    fs::remove_dir_all(&sandbox.record).unwrap();
    let run = sandbox.run(&["--kind", "id", "--prompt", "p", "--task-id", "T-1"]);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert_eq!(sandbox.harness_args(), ["--id", "T-1", "p"]);
}

#[test]
fn a_grove_shaped_task_file_supplies_neither_kind_nor_identity() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&policy(&[(
        "impl",
        r#""fake-harness""#,
        r#"[{ slot: "taskId" }, { slot: "prompt" }]"#,
    )]));
    let task = sandbox.file(
        ".grove/14-k12/02-impl--harness-exec-k14.md",
        "# harness-exec-k14\n\n**Reviews:** static-dispatch-k12\n",
    );

    // No --task-id: the handle in the file name is not an identity.
    let refusal = sandbox
        .run(&[
            "--kind",
            "impl",
            "--prompt",
            "p",
            "--task-file",
            &text(&task),
            "--json",
        ])
        .refusal(3);
    assert_eq!(refusal["error"]["code"], "missing_input");
    assert_eq!(refusal["error"]["input"], "--task-id");

    // The file name says `impl`, but only --kind names the kind.
    let refusal = sandbox
        .run(&[
            "--kind",
            "design",
            "--prompt",
            "p",
            "--task-file",
            &text(&task),
            "--task-id",
            "T-1",
            "--json",
        ])
        .refusal(3);
    assert_eq!(refusal["error"]["code"], "incomplete_mapping");
    assert!(!sandbox.harness_ran());
}

#[test]
fn task_identity_and_file_inputs_are_checked_as_caller_data() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&policy(&[(
        "impl",
        r#""fake-harness""#,
        r#"["--id", { slot: "taskId" }, { slot: "prompt" }]"#,
    )]));
    let at_limit = "k".repeat(1024);
    let over = "k".repeat(1025);
    for (name, flag, value) in [
        ("empty identity", "--task-id", ""),
        ("identity over 1024 bytes", "--task-id", over.as_str()),
        ("empty task file", "--task-file", ""),
    ] {
        let refusal = sandbox
            .run(&["--kind", "impl", "--prompt", "p", flag, value, "--json"])
            .refusal(2);
        assert_eq!(refusal["error"]["code"], "malformed_input", "{name}");
        assert_eq!(refusal["error"]["input"], flag, "{name}");
    }
    let mut command = sandbox.command();
    command.args([
        "run",
        "--kind",
        "impl",
        "--prompt",
        "p",
        "--json",
        "--task-id",
    ]);
    command.arg(std::ffi::OsStr::from_bytes(b"k\xff"));
    let refusal = run(&mut command).refusal(2);
    assert_eq!(refusal["error"]["input"], "--task-id");
    assert!(!sandbox.harness_ran());

    // The positive control: exactly 1024 bytes, multibyte UTF-8 included.
    let multibyte = format!("{}é", "k".repeat(1022));
    for id in [at_limit.as_str(), multibyte.as_str()] {
        fs::remove_dir_all(&sandbox.record).ok();
        let run = sandbox.run(&["--kind", "impl", "--prompt", "p", "--task-id", id]);
        assert_eq!(run.code, Some(0), "{}", run.stderr);
        assert_eq!(sandbox.harness_args(), ["--id", id, "p"]);
    }
}

#[test]
fn the_program_resolves_as_a_path_name_an_absolute_path_or_a_cwd_relative_path() {
    let sandbox = Sandbox::new();
    let absolute = sandbox.bin.join("fake-harness");
    executable(&sandbox.cwd.join("tools/local-harness"), FAKE_HARNESS);
    sandbox.personal_policy(&policy(&[
        ("path", r#""fake-harness""#, r#"[{ slot: "prompt" }]"#),
        (
            "absolute",
            &format!("{:?}", text(&absolute)),
            r#"[{ slot: "prompt" }]"#,
        ),
        (
            "relative",
            r#""./tools/local-harness""#,
            r#"[{ slot: "prompt" }]"#,
        ),
    ]));

    let expectations = [
        ("path", "fake-harness".to_owned(), "PATH", absolute.clone()),
        ("absolute", text(&absolute), "absolute", absolute.clone()),
        (
            "relative",
            "./tools/local-harness".to_owned(),
            "cwd",
            sandbox.cwd.join("./tools/local-harness"),
        ),
    ];
    for (kind, program, resolved_by, path) in expectations {
        let report = sandbox
            .inspect(&["--kind", kind, "--prompt", "p", "--json"])
            .report();
        let executable = &report["executable"];
        assert_eq!(executable["program"], program.as_str(), "{kind}: {report}");
        assert_eq!(executable["resolvedBy"], resolved_by, "{kind}");
        assert_eq!(executable["path"], text(&path), "{kind}");
        assert_eq!(report["argv"][0], program.as_str(), "{kind}");
        if kind == "path" {
            assert_eq!(executable["pathEntry"], text(&sandbox.bin));
        } else {
            assert!(executable.get("pathEntry").is_none(), "{kind}");
        }

        fs::remove_dir_all(&sandbox.record).ok();
        let run = sandbox.run(&["--kind", kind, "--prompt", "p"]);
        assert_eq!(run.code, Some(0), "{kind}: {}", run.stderr);
        assert_eq!(sandbox.harness_args(), ["p"], "{kind}");
    }
}

#[test]
fn a_path_search_passes_over_a_non_executable_match_as_execvp_does() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&policy(&[(
        "impl",
        r#""shadowed""#,
        r#"[{ slot: "prompt" }]"#,
    )]));
    let first = sandbox.root.join("first");
    let later = sandbox.root.join("later");
    support::write(&first.join("shadowed"), FAKE_HARNESS); // not executable
    fs::create_dir_all(later.join("shadowed-dir")).unwrap();
    executable(&later.join("shadowed"), FAKE_HARNESS);

    let path = format!("{}:{}:/usr/bin:/bin", text(&first), text(&later));
    let mut command = sandbox.command();
    command
        .env("PATH", &path)
        .args(["inspect", "--kind", "impl", "--json"]);
    let report = run(&mut command).report();
    assert_eq!(report["executable"]["pathEntry"], text(&later));

    // Only a non-executable match anywhere is unexecutable, not missing.
    let mut command = sandbox.command();
    command
        .env("PATH", format!("{}:/usr/bin:/bin", text(&first)))
        .args(["run", "--kind", "impl", "--prompt", "p", "--json"]);
    let refusal = run(&mut command).refusal(126);
    assert_eq!(refusal["error"]["code"], "program_unexecutable");
    assert!(!sandbox.harness_ran());
}

#[test]
fn a_missing_program_exits_127_and_never_falls_back_to_another_candidate() {
    let sandbox = Sandbox::new();
    let entry = sandbox.personal_policy(&policy(&[
        ("named", r#""no-such-harness""#, r#"[{ slot: "prompt" }]"#),
        (
            "absolute",
            &format!("{:?}", text(&sandbox.root.join("absent/harness"))),
            r#"[{ slot: "prompt" }]"#,
        ),
        (
            "relative",
            r#""./absent/harness""#,
            r#"[{ slot: "prompt" }]"#,
        ),
        ("working", r#""fake-harness""#, r#"[{ slot: "prompt" }]"#),
    ]));

    for (index, kind) in ["named", "absolute", "relative"].iter().enumerate() {
        for command in ["run", "inspect"] {
            let mut invocation = sandbox.command();
            invocation.args([command, "--kind", kind, "--prompt", "p", "--json"]);
            let refusal = run(&mut invocation).refusal(127);
            let error = &refusal["error"];
            assert_eq!(error["code"], "program_not_found", "{kind} {command}");
            assert_eq!(error["stage"], "resolution", "{kind} {command}");
            assert_eq!(error["source"], text(&entry), "{kind} {command}");
            assert_eq!(
                error["location"],
                format!("policy.catalog[{index}].program"),
                "{kind} {command}"
            );
            assert!(
                error["remedy"]
                    .as_str()
                    .unwrap()
                    .contains("never runs another candidate"),
                "{refusal}"
            );
        }
    }
    assert!(!sandbox.harness_ran(), "another candidate ran instead");

    // Only the selected program is checked: the missing ones in the same
    // catalog do not stop the working candidate.
    let run = sandbox.run(&["--kind", "working", "--prompt", "p"]);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
}

#[test]
fn an_unexecutable_program_exits_126() {
    let sandbox = Sandbox::new();
    let plain = sandbox.root.join("plain-file");
    support::write(&plain, FAKE_HARNESS);
    let directory = sandbox.root.join("a-directory");
    fs::create_dir(&directory).unwrap();
    sandbox.personal_policy(&policy(&[
        (
            "plain",
            &format!("{:?}", text(&plain)),
            r#"[{ slot: "prompt" }]"#,
        ),
        (
            "directory",
            &format!("{:?}", text(&directory)),
            r#"[{ slot: "prompt" }]"#,
        ),
    ]));

    for kind in ["plain", "directory"] {
        let refusal = sandbox
            .run(&["--kind", kind, "--prompt", "p", "--json"])
            .refusal(126);
        assert_eq!(refusal["error"]["code"], "program_unexecutable", "{kind}");
        assert_eq!(refusal["error"]["stage"], "resolution", "{kind}");
    }
    assert!(!sandbox.harness_ran());
}

#[test]
fn an_exec_error_reports_errno_with_a_remedy() {
    let sandbox = Sandbox::new();
    let orphan = sandbox.root.join("orphan-script");
    executable(&orphan, "#!/nonexistent/interpreter\n");
    sandbox.personal_policy(&policy(&[
        ("impl", r#""fake-harness""#, r#"[{ slot: "prompt" }]"#),
        (
            "orphan",
            &format!("{:?}", text(&orphan)),
            r#"[{ slot: "prompt" }]"#,
        ),
    ]));

    // One argument of 1 MiB passes the prompt bound but not exec's own
    // argument limits, on macOS and on Linux: E2BIG.
    let large = sandbox.file("large", &"a".repeat(1024 * 1024));
    let refusal = sandbox
        .run(&["--kind", "impl", "--prompt-file", &text(&large), "--json"])
        .stderr;
    let mut lines = refusal.lines();
    let handoff: Value = serde_json::from_str(lines.next().unwrap()).unwrap();
    assert_eq!(handoff["handoff"]["candidateId"], "impl");
    let error: Value = serde_json::from_str(lines.next().unwrap()).unwrap();
    assert!(lines.next().is_none(), "{refusal}");
    assert_eq!(error["error"]["code"], "exec_failed");
    assert_eq!(error["error"]["stage"], "exec");
    assert_eq!(error["error"]["exit"], 126);
    let message = error["error"]["message"].as_str().unwrap();
    assert!(message.contains("os error 7"), "{message}");
    assert!(
        error["error"]["remedy"]
            .as_str()
            .unwrap()
            .contains("prompt"),
        "{error}"
    );

    // A script whose interpreter is missing is found and executable, and exec
    // itself reports ENOENT.
    let run = sandbox.run(&["--kind", "orphan", "--prompt", "p"]);
    assert_eq!(run.code, Some(127), "{}", run.stderr);
    assert!(run.stderr.contains("exec_failed"), "{}", run.stderr);
    assert!(run.stderr.contains("interpreter"), "{}", run.stderr);
    assert!(!sandbox.harness_ran());
}

#[test]
fn run_json_writes_one_handoff_line_on_stderr_and_leaves_stdout_to_the_harness() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&format!(
        "console.log(\"policy chatter\");\n{}",
        policy(&[("impl", r#""fake-harness""#, r#"[{ slot: "prompt" }]"#)])
    ));

    let run = sandbox.run(&["--kind", "impl", "--prompt", "p", "--json"]);

    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert_eq!(run.stdout, "");
    assert!(run.stderr.ends_with("}\n") && run.stderr.matches('\n').count() == 1);
    let notice: Value = serde_json::from_str(&run.stderr).unwrap();
    assert_eq!(notice["schemaVersion"], 1);
    let handoff = &notice["handoff"];
    assert_eq!(handoff["kind"], "impl");
    assert_eq!(handoff["candidateId"], "impl");
    assert_eq!(handoff["provider"], "origin-impl");
    assert_eq!(handoff["model"], "model-impl");
    assert_eq!(handoff["effort"], "effort-impl");
    assert_eq!(
        handoff["executable"],
        text(&sandbox.bin.join("fake-harness"))
    );
    assert!(notice["diagnostics"]["stdout"]
        .as_str()
        .unwrap()
        .contains("policy chatter"));

    // Text mode prefixes the policy's output and ends with the choice line.
    fs::remove_dir_all(&sandbox.record).unwrap();
    let run = sandbox.run(&["--kind", "impl", "--prompt", "p"]);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert_eq!(run.stdout, "");
    assert!(
        run.stderr
            .starts_with("policy stdout: policy chatter\nharness-dispatch: running"),
        "{}",
        run.stderr
    );
    assert!(run
        .stderr
        .contains(&text(&sandbox.bin.join("fake-harness"))));
}

#[test]
fn inspection_reports_where_a_relative_prompt_file_was_read() {
    // A relative --prompt-file resolves against the caller's cwd, like
    // --task-file.
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&policy(&[(
        "impl",
        r#""fake-harness""#,
        r#"[{ slot: "prompt" }]"#,
    )]));
    sandbox.file("in/prompt.md", "from a file\n");
    let report = sandbox
        .inspect(&["--kind", "impl", "--prompt-file", "in/prompt.md", "--json"])
        .report();
    assert_eq!(report["prompt"]["supplied"], true);
    assert_eq!(report["prompt"]["from"], "--prompt-file");
    assert_eq!(
        report["prompt"]["path"],
        text(&sandbox.cwd.join("in/prompt.md"))
    );
    assert_eq!(report["prompt"]["bytes"], 12);
    assert_eq!(report["argv"][1], "from a file\n");
    assert!(!sandbox.harness_ran(), "inspection launched the harness");
}
