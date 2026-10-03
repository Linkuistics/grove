//! `run` through the command seam: prompt and task inputs, the argv `select`
//! builds from them, program resolution, and the plain exec that hands the
//! caller's process to the fake harness.
//!
//! The fake harness records its arguments, physical cwd, PID and parent's PID,
//! so a test can show that it received the exact words, ran where the caller
//! ran, and is the front's own child, which the front supervises.

mod support;

use std::fs;
use std::io::Read as _;
use std::os::fd::AsRawFd as _;
use std::os::unix::ffi::OsStrExt as _;
use std::os::unix::process::ExitStatusExt as _;
use std::path::Path;
use std::process::Stdio;
use std::time::{Duration, Instant};

use serde_json::Value;
use support::{executable, run, text, Sandbox, FAKE_HARNESS, FRONT};

/// A policy whose `select` consults a table by kind, each entry `(kind,
/// program, args)` with its program and its argument array given as JavaScript
/// expressions that read `request`. A kind the table does not name refuses.
fn policy(commands: &[(&str, &str, &str)]) -> String {
    let table: Vec<String> = commands
        .iter()
        .map(|(kind, program, args)| {
            format!(
                r#"  "{kind}": (request) => ({{ program: {program}, args: {args}, provider: "origin-{kind}", model: "model-{kind}", effort: "effort-{kind}" }}),"#
            )
        })
        .collect();
    format!(
        r#"const table = {{
{}
}};
export const policy = {{
  schemaVersion: 2,
  version: "run-1",
  select(request) {{
    const route = Object.hasOwn(table, request.kind) ? table[request.kind] : undefined;
    if (route === undefined) {{
      return {{ status: "refused", code: "incomplete_mapping", message: `no command for kind ${{JSON.stringify(request.kind)}}`, remedy: "add one to the table" }};
    }}
    return {{ status: "selected", ...route(request), reason: `the table's entry for ${{request.kind}}` }};
  }},
}};
"#,
        table.join("\n")
    )
}

/// The prompt as the command's one argument.
const PROMPT: &str = "[request.prompt]";

/// Every caller input, among literals a shell would have mangled, with one
/// parameter a whole argument and another built into the middle of one.
const EVERY_INPUT: &str = r#"["--kind", request.kind, "--task", request.taskFile, "--id", request.taskId, "--cwd", request.cwd, request.params.session_name, `--repo=${request.params.repo};tail`, "literal $HOME {prompt} 'single' \"double\" ; & | `tick` *", request.prompt]"#;

const AWKWARD_PROMPT: &str =
    "Fix the \"parser\"; don't `rm -rf` $HOME && echo 'done' | tee *\n\nsecond line\t$(date)\n\n";

#[test]
fn run_hands_the_exact_argv_to_the_harness_as_its_own_child_in_the_callers_cwd() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&policy(&[("impl", r#""fake-harness""#, EVERY_INPUT)]));
    let cwd = sandbox
        .cwd
        .join("dir with spaces 'single' \"double\" ; & | $x\nand a newline");
    fs::create_dir_all(cwd.join("tasks")).unwrap();
    let task_id = "harness-exec-k14 with spaces \"and quotes\" ; $x";
    let session_name = "repo: a 'grove' \"session\" ; $(x) `y`\n\tsecond line\n";
    let repo = "/work/my repo's \"tree\" $x";

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
        "--param",
        &format!("session_name={session_name}"),
        "--param",
        &format!("repo={repo}"),
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
            "--kind",
            "impl",
            "--task",
            &text(&cwd.join("tasks/leaf one's \"task\"; $(x).md")),
            "--id",
            task_id,
            "--cwd",
            &text(&cwd),
            session_name,
            &format!("--repo={repo};tail"),
            "literal $HOME {prompt} 'single' \"double\" ; & | `tick` *",
            AWKWARD_PROMPT,
        ]
    );
    assert_eq!(sandbox.harness_cwd(), cwd);
    assert_eq!(
        sandbox.harness_ppid(),
        pid,
        "the harness is not the front's child"
    );
    assert_ne!(sandbox.harness_pid(), pid, "the harness replaced the front");
    assert_eq!(
        output.stdout, b"",
        "the front wrote to the harness's stdout"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.starts_with(
            "harness-dispatch: running provider origin-impl, model model-impl, effort \
             effort-impl for kind \"impl\" as run "
        ) && stderr.matches('\n').count() == 2
            && stderr.lines().nth(1).is_some_and(
                |end| end.contains(" ended by harness_exit: the harness exited 0 after ")
            ),
        "one short handoff line and one end line on stderr: {stderr}"
    );
}

#[test]
fn a_prompt_file_is_read_once_with_its_exact_bytes_from_the_callers_cwd() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&policy(&[("impl", r#""fake-harness""#, PROMPT)]));
    sandbox.file("prompts/mandate.md", AWKWARD_PROMPT);

    let run = sandbox.run(&["--kind", "impl", "--prompt-file", "prompts/mandate.md"]);

    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert_eq!(sandbox.harness_args(), [AWKWARD_PROMPT]);
}

const CALLER_INPUT: &str = "caller input for the harness\n";
const THROUGH_SEVEN: &str = "written through descriptor 7\n";

/// What a harness received from a caller that had a line waiting on its stdin
/// and left descriptor 7 open on a file.
struct Handed {
    /// The harness copies its stdin to its stdout.
    stdout: String,
    /// The caller's file, which the harness writes to through descriptor 7.
    through: String,
    /// The probed descriptors the harness held.
    fds: Vec<u32>,
}

/// Start the fake harness through `front` for such a caller.
fn handed_through(sandbox: &Sandbox, front: &Path) -> Handed {
    // The input is in place before the front exists. Nothing is written
    // after the spawn, so a front that dies early is reported by its own
    // status and stderr, never as a broken pipe.
    let input = sandbox.root.join("caller-input");
    fs::write(&input, CALLER_INPUT).unwrap();
    let through = sandbox.root.join("descriptor-7");
    let file = fs::File::create(&through).unwrap();

    let mut command = sandbox.command_for(front);
    command
        .args(["run", "--kind", "impl", "--prompt", "p"])
        .env("FAKE_HARNESS_FD7", THROUGH_SEVEN.trim_end())
        .stdin(fs::File::open(&input).unwrap());
    support::caller_leaves_open(&mut command, Some((&file, 7)));
    let run = run(&mut command);

    assert_eq!(run.code, Some(0), "{}", run.stderr);
    Handed {
        stdout: run.stdout,
        through: fs::read_to_string(&through).unwrap(),
        fds: sandbox.harness_fds(),
    }
}

#[test]
fn the_harness_keeps_the_callers_stdin_stdout_and_other_descriptors() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&policy(&[("impl", r#""fake-harness""#, PROMPT)]));

    let handed = handed_through(&sandbox, Path::new(FRONT));

    assert_eq!(
        handed.stdout, CALLER_INPUT,
        "stdin reached the harness whole, and nothing else reached stdout"
    );
    assert_eq!(handed.through, THROUGH_SEVEN);
    // The caller's descriptor, and nothing of the front's own: the record
    // store was closed before exec. This is also the positive control for the
    // fake harness's descriptor probe, which `records.rs` relies on.
    assert_eq!(handed.fds, [7]);
}

/// The control for the test above: the same caller and harness, through
/// stand-in fronts that differ from a faithful one by a line each.
#[test]
fn a_stand_in_front_that_closes_replaces_or_adds_a_descriptor_or_reads_stdin_is_told_apart() {
    let stand_in = |before_exec: &str| {
        let sandbox = Sandbox::new();
        let front = sandbox.root.join("stand-in-front");
        executable(
            &front,
            &format!("#!/bin/sh\n{before_exec}\nexec fake-harness p\n"),
        );
        handed_through(&sandbox, &front)
    };

    // The faithful one reads as the front does, so each difference below is
    // its own line's doing.
    let faithful = stand_in("");
    assert_eq!(faithful.stdout, CALLER_INPUT);
    assert_eq!(faithful.through, THROUGH_SEVEN);
    assert_eq!(faithful.fds, [7]);

    let closes = stand_in("exec 7>&-");
    assert_eq!(closes.through, "");
    assert_eq!(closes.fds, Vec::<u32>::new());

    // Only the caller's file tells a replaced descriptor 7 from the caller's.
    let replaces = stand_in("exec 7>/dev/null");
    assert_eq!(replaces.through, "");
    assert_eq!(replaces.fds, [7]);

    let adds = stand_in("exec 5</dev/null");
    assert_eq!(adds.through, THROUGH_SEVEN);
    assert_eq!(adds.fds, [5, 7]);

    let reads = stand_in("read -r line");
    assert_eq!(reads.stdout, "");
}

#[test]
fn the_harness_exits_with_its_own_code_and_signal() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&policy(&[("impl", r#""fake-harness""#, PROMPT)]));

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
    sandbox.personal_policy(&policy(&[("impl", r#""fake-harness""#, PROMPT)]));
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
    sandbox.personal_policy(&policy(&[("impl", r#""fake-harness""#, PROMPT)]));
    let run = sandbox.run(&["--kind", "impl", "--prompt", "--json is not a flag here"]);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert_eq!(sandbox.harness_args(), ["--json is not a flag here"]);
}

#[test]
fn a_json_prompt_or_task_id_is_data_and_never_chooses_the_output_format() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&policy(&[
        ("impl", r#""fake-harness""#, PROMPT),
        ("broken", r#""./broken-harness""#, PROMPT),
    ]));
    // Its `#!` interpreter does not exist, so exec fails after the commit.
    executable(
        &sandbox.cwd.join("broken-harness"),
        "#!/nonexistent/interpreter\n",
    );
    let text_refusal = |run: &support::Run, exit: i32, code: &str| {
        assert_eq!(run.code, Some(exit), "{}", run.stderr);
        assert!(
            run.stderr
                .starts_with(&format!("harness-dispatch: refused ({code}, ")),
            "not a text refusal: {}",
            run.stderr
        );
    };

    for command in ["run", "inspect"] {
        let unrouted = |data: &[&str]| {
            let mut invocation = sandbox.command();
            invocation.args([command, "--kind", "unrouted"]).args(data);
            run(&mut invocation)
        };
        text_refusal(&unrouted(&["--prompt", "--json"]), 3, "policy_refused");
        text_refusal(
            &unrouted(&["--prompt", "p", "--task-id", "--json"]),
            3,
            "policy_refused",
        );
        // The control: the same refusal, asked for as JSON, is JSON.
        let json = unrouted(&["--prompt", "p", "--json"]).refusal(3);
        assert_eq!(json["error"]["code"], "policy_refused");
        assert_eq!(json["error"]["policyCode"], "incomplete_mapping");
    }

    // A routed run announces its handoff as text, and the harness receives
    // the prompt.
    let routed = sandbox.run(&["--kind", "impl", "--prompt", "--json"]);
    assert_eq!(routed.code, Some(0), "{}", routed.stderr);
    assert!(
        routed
            .stderr
            .starts_with("harness-dispatch: running provider origin-impl"),
        "{}",
        routed.stderr
    );
    assert_eq!(sandbox.harness_args(), ["--json"]);
    // An inspection reports as text.
    let inspected = sandbox.inspect(&["--kind", "impl", "--prompt", "--json"]);
    assert_eq!(inspected.code, Some(0), "{}", inspected.stderr);
    assert!(
        inspected.stdout.contains("provider   origin-impl"),
        "{}",
        inspected.stdout
    );
    // And an exec that fails after the commit refuses as text, after its
    // text handoff notice.
    let broken = sandbox.run(&["--kind", "broken", "--prompt", "--json"]);
    assert_eq!(broken.code, Some(127), "{}", broken.stderr);
    let (notice, refusal) = broken.stderr.split_once('\n').unwrap();
    assert!(
        notice.starts_with("harness-dispatch: running provider origin-broken"),
        "{}",
        broken.stderr
    );
    assert!(
        refusal.starts_with("harness-dispatch: refused (exec_failed, "),
        "not a text refusal: {}",
        broken.stderr
    );
}

#[test]
fn an_invalid_or_unreadable_prompt_is_refused_before_policy_runs() {
    let sandbox = Sandbox::new();
    let sentinel = sandbox.root.join("policy-ran");
    sandbox.personal_policy(&format!(
        "import {{ writeFileSync }} from \"node:fs\";\nwriteFileSync({:?}, \"fired\");\n{}",
        text(&sentinel),
        policy(&[(
            "impl",
            r#""fake-harness""#,
            "[String(request.prompt.length)]"
        )])
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

    // The positive control: exactly 1 MiB of valid UTF-8 is accepted, and
    // `select` receives all of it. The policy returns its length and not the
    // prompt, which would not fit a result within the protocol message bound.
    let path = write("limit", &vec![b'a'; limit]);
    let report = sandbox
        .inspect(&["--kind", "impl", "--prompt-file", &path, "--json"])
        .report();
    assert_eq!(report["prompt"]["bytes"], limit);
    assert_eq!(report["command"]["args"][0], limit.to_string());
    assert!(sentinel.exists(), "the control never evaluated the policy");
}

#[test]
fn a_terminal_is_never_read_as_the_prompt() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&policy(&[("impl", r#""fake-harness""#, PROMPT)]));
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
fn a_grove_shaped_task_file_supplies_neither_kind_nor_identity() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&policy(&[(
        "impl",
        r#""fake-harness""#,
        r#"[request.kind, String(request.taskId), request.prompt]"#,
    )]));
    let task = sandbox.file(
        ".grove/14-k12/02-impl--harness-exec-k14.md",
        "# harness-exec-k14\n\n**Reviews:** static-dispatch-k12\n",
    );

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
    assert_eq!(refusal["error"]["code"], "policy_refused");
    assert_eq!(refusal["error"]["policyCode"], "incomplete_mapping");
    assert_eq!(refusal["error"]["input"], "--kind design");
    assert!(!sandbox.harness_ran());

    // No --task-id: the handle in the file name is not an identity, and the
    // request has none.
    let run = sandbox.run(&[
        "--kind",
        "impl",
        "--prompt",
        "p",
        "--task-file",
        &text(&task),
    ]);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert_eq!(sandbox.harness_args(), ["impl", "undefined", "p"]);
}

#[test]
fn task_identity_and_file_inputs_are_checked_as_caller_data() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&policy(&[(
        "impl",
        r#""fake-harness""#,
        r#"["--id", request.taskId, request.prompt]"#,
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
        ("path", r#""fake-harness""#, PROMPT),
        ("absolute", &format!("{:?}", text(&absolute)), PROMPT),
        ("relative", r#""./tools/local-harness""#, PROMPT),
    ]));

    let expectations = [
        (
            "path",
            "fake-harness".to_owned(),
            format!("(found on PATH in {})", text(&sandbox.bin)),
            absolute.clone(),
        ),
        (
            "absolute",
            text(&absolute),
            "(an absolute path)".to_owned(),
            absolute.clone(),
        ),
        (
            "relative",
            "./tools/local-harness".to_owned(),
            "(relative to the current directory)".to_owned(),
            sandbox.cwd.join("./tools/local-harness"),
        ),
    ];
    for (kind, program, how, path) in expectations {
        let report = sandbox
            .inspect(&["--kind", kind, "--prompt", "p", "--json"])
            .report();
        // The program as `select` returned it, and the file it resolved to.
        assert_eq!(
            report["command"],
            serde_json::json!({ "program": program, "args": ["p"], "executable": text(&path) }),
            "{kind}: {report}"
        );
        let human = sandbox.inspect(&["--kind", kind, "--prompt", "p"]);
        assert!(
            human
                .stdout
                .contains(&format!("  executable {} {how}\n", text(&path))),
            "{kind}: {}",
            human.stdout
        );

        fs::remove_dir_all(&sandbox.record).ok();
        let run = sandbox.run(&["--kind", kind, "--prompt", "p"]);
        assert_eq!(run.code, Some(0), "{kind}: {}", run.stderr);
        assert_eq!(sandbox.harness_args(), ["p"], "{kind}");
    }
}

#[test]
fn a_path_search_passes_over_a_non_executable_match_as_execvp_does() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&policy(&[("impl", r#""shadowed""#, PROMPT)]));
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
    assert_eq!(
        report["command"]["executable"],
        text(&later.join("shadowed"))
    );

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
fn an_empty_path_entry_is_the_cwd_even_when_it_is_the_whole_path() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&policy(&[("impl", r#""agent""#, PROMPT)]));
    // The harness in the cwd gives the fake harness a PATH of its own, since
    // it inherits the caller's.
    executable(
        &sandbox.cwd.join("agent"),
        &format!(
            "#!/bin/sh\nPATH=/usr/bin:/bin\nexport PATH\nexec {:?} \"$@\"\n",
            text(&sandbox.bin.join("fake-harness"))
        ),
    );
    let with_path = |path: &str, command: &str| {
        let mut invocation = sandbox.command();
        invocation
            .env("PATH", path)
            .args([command, "--kind", "impl", "--prompt", "p", "--json"]);
        run(&mut invocation)
    };
    // A wholly empty PATH is one empty entry, as `:` is two.
    for path in ["", ":", "/nonexistent:"] {
        let report = with_path(path, "inspect").report();
        assert_eq!(
            report["command"]["executable"],
            text(&sandbox.cwd.join("agent")),
            "{path:?}: {report}"
        );
        let mut human = sandbox.command();
        human.env("PATH", path).args(["inspect", "--kind", "impl"]);
        let human = run(&mut human);
        assert!(
            human.stdout.contains("agent (found on PATH in )\n"),
            "{path:?}: {}",
            human.stdout
        );

        fs::remove_dir_all(&sandbox.record).ok();
        let launched = with_path(path, "run");
        assert_eq!(launched.code, Some(0), "{path:?}: {}", launched.stderr);
        assert_eq!(sandbox.harness_args(), ["p"], "{path:?}");
    }

    // An unset PATH is not empty: there is no caller's PATH to search.
    fs::remove_dir_all(&sandbox.record).ok();
    for command in ["run", "inspect"] {
        let mut invocation = sandbox.command();
        invocation
            .env_remove("PATH")
            .args([command, "--kind", "impl", "--prompt", "p", "--json"]);
        let refusal = run(&mut invocation).refusal(127);
        assert_eq!(refusal["error"]["code"], "program_not_found", "{command}");
        assert!(
            refusal["error"]["message"]
                .as_str()
                .unwrap()
                .contains("PATH is unset"),
            "{refusal}"
        );
    }
    assert!(!sandbox.harness_ran());
}

#[test]
fn a_nul_in_a_returned_argument_or_program_refuses_before_anything_is_recorded() {
    let sandbox = Sandbox::new();
    let entry = sandbox.personal_policy(&policy(&[
        (
            "argument",
            r#""fake-harness""#,
            r#"[request.prompt, "before\0after"]"#,
        ),
        ("program", r#""fake-harness\0""#, PROMPT),
    ]));
    for (kind, location) in [
        ("argument", "result.args[1]"),
        ("program", "result.program"),
    ] {
        let refusal = sandbox
            .run(&["--kind", kind, "--prompt", "p", "--json"])
            .refusal(3);
        let error = &refusal["error"];
        assert_eq!(error["code"], "selection_malformed", "{kind}: {refusal}");
        assert_eq!(error["stage"], "selection", "{kind}");
        assert_eq!(error["source"], text(&entry), "{kind}");
        assert_eq!(error["location"], location, "{kind}");
        assert!(
            error["message"].as_str().unwrap().contains("NUL"),
            "{kind}: {refusal}"
        );
    }
    assert!(!sandbox.default_store().exists(), "a run was recorded");
    assert!(!sandbox.harness_ran());
}

#[test]
fn a_missing_program_exits_127_and_nothing_runs_in_its_place() {
    let sandbox = Sandbox::new();
    let entry = sandbox.personal_policy(&policy(&[
        ("named", r#""no-such-harness""#, PROMPT),
        (
            "absolute",
            &format!("{:?}", text(&sandbox.root.join("absent/harness"))),
            PROMPT,
        ),
        ("relative", r#""./absent/harness""#, PROMPT),
        ("working", r#""fake-harness""#, PROMPT),
    ]));

    for kind in ["named", "absolute", "relative"] {
        for command in ["run", "inspect"] {
            let mut invocation = sandbox.command();
            invocation.args([command, "--kind", kind, "--prompt", "p", "--json"]);
            let refusal = run(&mut invocation).refusal(127);
            let error = &refusal["error"];
            assert_eq!(error["code"], "program_not_found", "{kind} {command}");
            assert_eq!(error["stage"], "resolution", "{kind} {command}");
            assert_eq!(error["source"], text(&entry), "{kind} {command}");
            assert_eq!(error["location"], "result.program", "{kind} {command}");
            assert!(
                error["message"]
                    .as_str()
                    .unwrap()
                    .starts_with(&format!("select in {} returned the program ", text(&entry))),
                "{refusal}"
            );
            assert!(
                error["remedy"]
                    .as_str()
                    .unwrap()
                    .contains("never runs another command"),
                "{refusal}"
            );
        }
    }
    assert!(!sandbox.harness_ran(), "another command ran instead");
    assert!(!sandbox.default_store().exists(), "a run was recorded");

    // Only the returned program is resolved: the missing ones the same table
    // holds do not stop the working one.
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
        ("plain", &format!("{:?}", text(&plain)), PROMPT),
        ("directory", &format!("{:?}", text(&directory)), PROMPT),
    ]));

    for kind in ["plain", "directory"] {
        let refusal = sandbox
            .run(&["--kind", kind, "--prompt", "p", "--json"])
            .refusal(126);
        assert_eq!(refusal["error"]["code"], "program_unexecutable", "{kind}");
        assert_eq!(refusal["error"]["stage"], "resolution", "{kind}");
        assert_eq!(refusal["error"]["location"], "result.program", "{kind}");
    }
    assert!(!sandbox.harness_ran());
    assert!(!sandbox.default_store().exists(), "a run was recorded");
}

#[test]
fn a_program_whose_resolved_path_is_not_utf8_refuses_and_is_not_passed_over() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&policy(&[
        ("named", r#""agent""#, PROMPT),
        ("relative", r#""./agent""#, PROMPT),
    ]));
    let native = sandbox.root.join(std::ffi::OsStr::from_bytes(b"bin-\xff"));
    match fs::create_dir(&native) {
        Ok(()) => {}
        // A filesystem that admits only UTF-8 names, as APFS does, has no
        // such path to resolve to, and this test has no subject there.
        Err(error) if error.raw_os_error() == Some(libc::EILSEQ) => return,
        Err(error) => panic!("create {}: {error}", native.display()),
    }
    // The firing configuration: the path's lossy string names another
    // executable, and so does a later PATH entry.
    let lossy = sandbox.root.join("bin-\u{FFFD}");
    let later = sandbox.root.join("later");
    for dir in [&native, &lossy, &later] {
        executable(&dir.join("agent"), FAKE_HARNESS);
    }
    let mut path = native.clone().into_os_string();
    path.push(format!(":{}:/usr/bin:/bin", text(&later)));

    let cases: [(&str, &Path); 2] = [("named", &sandbox.cwd), ("relative", &native)];
    for (kind, cwd) in cases {
        for subcommand in ["inspect", "run"] {
            let mut command = sandbox.command();
            command
                .env("PATH", &path)
                .current_dir(cwd)
                .args([subcommand, "--kind", kind, "--prompt", "p", "--json"]);
            let refusal = run(&mut command).refusal(126);
            let error = &refusal["error"];
            assert_eq!(error["code"], "program_unexecutable", "{kind}: {refusal}");
            assert_eq!(error["stage"], "resolution", "{kind}: {refusal}");
            assert!(
                error["message"]
                    .as_str()
                    .unwrap()
                    .contains("is not valid UTF-8"),
                "{kind}: {refusal}"
            );
        }
    }
    assert!(!sandbox.harness_ran(), "another command ran instead");
    assert!(!sandbox.default_store().exists(), "a run was recorded");
}

#[test]
fn an_exec_error_reports_errno_with_a_remedy() {
    let sandbox = Sandbox::new();
    let orphan = sandbox.root.join("orphan-script");
    executable(&orphan, "#!/nonexistent/interpreter\n");
    sandbox.personal_policy(&policy(&[
        ("impl", r#""fake-harness""#, PROMPT),
        ("orphan", &format!("{:?}", text(&orphan)), PROMPT),
    ]));

    // One argument a page short of 1 MiB fits a selection result, but not
    // exec's own limits: Linux bounds one argument far lower, and on macOS the
    // arguments and the environment, padded here, come to more than it
    // carries together. E2BIG.
    let large = sandbox.file("large", &"a".repeat(1024 * 1024 - 4096));
    let mut command = sandbox.command();
    command
        .env("PADDING", "p".repeat(16 * 1024))
        .args(["run", "--kind", "impl", "--json", "--prompt-file"])
        .arg(&large);
    let refusal = run(&mut command).stderr;
    let mut lines = refusal.lines();
    let handoff: Value = serde_json::from_str(lines.next().unwrap()).unwrap();
    assert_eq!(handoff["handoff"]["provider"], "origin-impl");
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
fn run_json_writes_a_handoff_and_an_end_line_on_stderr_and_leaves_stdout_to_the_harness() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&format!(
        "console.log(\"policy chatter\");\n{}",
        policy(&[("impl", r#""fake-harness""#, PROMPT)])
    ));

    let run = sandbox.run(&["--kind", "impl", "--prompt", "p", "--json"]);

    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert_eq!(run.stdout, "");
    let notice = run.handoff();
    assert_eq!(notice["schemaVersion"], 1);
    let handoff = &notice["handoff"];
    assert_eq!(
        handoff.as_object().unwrap().keys().collect::<Vec<_>>(),
        [
            "effort",
            "executable",
            "kind",
            "model",
            "provider",
            "reason",
            "recordedAt",
            "runId",
            "stateDir"
        ]
    );
    assert_eq!(handoff["runId"], sandbox.harness_run_id());
    assert_eq!(handoff["kind"], "impl");
    assert_eq!(handoff["provider"], "origin-impl");
    assert_eq!(handoff["model"], "model-impl");
    assert_eq!(handoff["effort"], "effort-impl");
    assert_eq!(handoff["reason"], "the table's entry for impl");
    assert_eq!(
        handoff["executable"],
        text(&sandbox.bin.join("fake-harness"))
    );
    assert!(notice["diagnostics"]["stdout"]
        .as_str()
        .unwrap()
        .contains("policy chatter"));

    // Text mode prefixes the policy's output and ends with the handoff line.
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
    sandbox.personal_policy(&policy(&[("impl", r#""fake-harness""#, PROMPT)]));
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
    assert_eq!(report["command"]["args"][0], "from a file\n");
    assert!(!sandbox.harness_ran(), "inspection launched the harness");
}
