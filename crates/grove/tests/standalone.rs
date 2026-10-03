//! `grove run` under real confinement: the real `grove`, the real
//! `harness-dispatch` front and its compiled worker, a policy in a temporary
//! HOME, and a deterministic shell harness. Nothing here calls a model.
//!
//! The worker is not a cargo artifact. `task dispatch:worker` builds it, and
//! without it the front refuses with exit 5, so these cases fail rather than
//! skip (`docs/specs/harness-selection-and-execution.md`, *Agreed test seams*).

mod support;

use std::fs;
use std::os::unix::fs::PermissionsExt as _;
use std::os::unix::process::CommandExt as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

use tempfile::TempDir;

/// A harness that publishes one line and acknowledges.
const WRITES_NOTES: &str = "printf ok > notes.md\nacknowledge\n";

struct Fixture {
    _dir: TempDir,
    root: PathBuf,
    home: PathBuf,
    /// Where `grove run` is typed: a directory that is no project.
    cwd: PathBuf,
    /// The harness script, which the default policy runs with `/bin/sh`.
    script: PathBuf,
}

impl Fixture {
    /// A HOME whose policy routes `release-notes` to `/bin/sh <script>` with
    /// the prompt, the empty parameter object and cwd, and refuses every other kind.
    fn new(harness: &str) -> Self {
        // The sibling dispatch Grove launches from its own directory.
        support::harness_dispatch();

        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().canonicalize().unwrap();
        let fixture = Fixture {
            home: root.join("home"),
            cwd: root.join("cwd"),
            script: root.join("harness.sh"),
            _dir: dir,
            root,
        };
        for dir in [&fixture.home, &fixture.cwd, &fixture.root.join("tmp")] {
            fs::create_dir(dir).unwrap();
        }
        fs::write(&fixture.script, format!(
            "dispatch_exit=$(printf '%s\\n' \"$1\" | sed -n '/exact command as your final action:/{{n;p;}}')\nacknowledge() {{ eval \"$dispatch_exit\"; }}\n{harness}"
        )).unwrap();
        fixture.policy(&format!(
            r#"export const policy = {{
  schemaVersion: 2,
  version: "standalone-1",
  select(request) {{
    if (request.kind !== "release-notes") {{
      return {{ status: "refused", code: "unrouted_kind", message: `no command for ${{request.kind}}`, remedy: "route the kind in the fixture policy" }};
    }}
    return {{ status: "selected", program: "/bin/sh", args: [{script:?}, request.prompt, JSON.stringify(request.params), request.cwd], provider: "fixture", model: "none", effort: "none", reason: "the deterministic harness" }};
  }},
}};
"#,
            script = fixture.script.to_str().unwrap()
        ));
        fixture
    }

    fn dispatch_file(&self, name: &str) -> PathBuf {
        self.home.join(".config/harness-dispatch").join(name)
    }

    fn policy(&self, source: &str) {
        let entry = self.dispatch_file("policy.ts");
        fs::create_dir_all(entry.parent().unwrap()).unwrap();
        fs::write(entry, source).unwrap();
    }

    fn notes(&self) -> PathBuf {
        self.cwd.join("notes.md")
    }

    /// `grove` itself, under an environment that holds nothing but the fixture.
    fn grove(&self, grove: &Path) -> Command {
        let mut command = Command::new(grove);
        command
            .env_clear()
            .env("HOME", &self.home)
            .env("PATH", "/usr/bin:/bin")
            .env("TMPDIR", self.root.join("tmp"))
            .current_dir(&self.cwd)
            .stdin(Stdio::null());
        command
    }

    /// `grove run KIND` for one `notes.md`, with the harness script granted.
    fn run_kind(&self, kind: &str) -> Command {
        let mut command = self.grove(Path::new(env!("CARGO_BIN_EXE_grove")));
        command
            .args(["run", kind, "Produce notes.", "--ui", "inline"])
            .arg("--output")
            .arg(self.notes())
            .arg("--runtime-read")
            .arg(&self.script);
        command
    }

    fn run(&self) -> Output {
        self.run_kind("release-notes").output().unwrap()
    }

    /// Transcripts of launched invocations. A selection that launched nothing
    /// leaves a log and a failure status.
    fn transcripts(&self) -> usize {
        fs::read_dir(self.home.join(".local/state/grove/runs")).map_or(0, Iterator::count)
    }
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn assert_published(fixture: &Fixture, output: &Output, expected: &str) {
    assert!(output.status.success(), "{}", stderr(output));
    assert_eq!(fs::read_to_string(fixture.notes()).unwrap(), expected);
}

fn assert_nothing_published(fixture: &Fixture, output: &Output) {
    assert!(!output.status.success(), "{}", stderr(output));
    assert!(!fixture.notes().exists(), "{}", stderr(output));
}

fn write_executable(path: &Path, body: &str, mode: u32) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, body).unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(mode)).unwrap();
}

/// A harness on PATH that publishes `word`, for a policy that returns a name.
fn named_harness(word: &str) -> String {
    format!(
        "#!/bin/sh\nprintf {word} > notes.md\n'{}' exit\n",
        support::harness_dispatch()
            .canonicalize()
            .unwrap()
            .display()
    )
}

/// A policy that returns the bare program name `fake-harness`.
const NAMED: &str = r#"export const policy = {
  schemaVersion: 2,
  version: "standalone-1",
  select: () => ({ status: "selected", program: "fake-harness", args: [], provider: "fixture", model: "none", effort: "none", reason: "a program found on PATH" }),
};
"#;

// The policy selects outside the sandbox, with what the owner granted it, in
// the staged directory and with no parameter. The harness runs inside, with the
// invocation's prompt and none of the owner's personal files. Old Grove configuration files change nothing.
#[test]
fn the_policy_selects_outside_the_sandbox_and_the_harness_runs_inside_it() {
    let fixture = Fixture::new(
        "set -eu\n\
         for file in \"$HOME/.config/harness-dispatch/policy.ts\" \
         \"$HOME/.config/harness-dispatch/settings.json\" \
         \"$HOME/.local/state/harness-dispatch/records.sqlite3\"; do\n\
         if cat \"$file\" > /dev/null 2>&1; then exit 41; fi\n\
         done\n\
         test -z \"${GROVE_SIGNAL_FILE-}\"\n\
         test -z \"${GROVE_LAUNCH_DIR-}\"\n\
         test -z \"${TMUX-}\"\n\
         case \"$1\" in 'Produce notes.'*) ;; *) exit 9 ;; esac\n\
         printf '%s\\n' \"${HARNESS_DISPATCH_RUN_ID-<unset>}\" \"${OWNER_GRANT-<unset>}\" \
         \"$2\" \"$3\" \"$4\" \"$5\" \"$(pwd -P)\" > notes.md\n\
         cat message.txt >> notes.md\n\
         acknowledge\n",
    );
    // The policy reports two variables its owner granted, as two more
    // arguments. One of them is the channel of a session this invocation runs
    // inside, which the owner should never grant and here has.
    let source = fs::read_to_string(fixture.dispatch_file("policy.ts")).unwrap();
    fixture.policy(&source.replace(
        "request.prompt,",
        "request.prompt, process.env.OWNER_GRANT ?? \"<unset>\", \
         process.env.GROVE_SIGNAL_FILE ?? \"<unset>\",",
    ));
    fs::write(
        fixture.dispatch_file("settings.json"),
        r#"{ "policyEnv": ["OWNER_GRANT", "GROVE_SIGNAL_FILE"] }"#,
    )
    .unwrap();
    let store = fixture
        .home
        .join(".local/state/harness-dispatch/records.sqlite3");
    fs::create_dir_all(store.parent().unwrap()).unwrap();

    let grove_config = fixture.home.join(".config/grove/config.kdl");
    fs::create_dir_all(grove_config.parent().unwrap()).unwrap();
    for abandoned in [&grove_config, &fixture.cwd.join(".grove.kdl")] {
        fs::write(abandoned, "not valid configuration").unwrap();
    }
    let input = fixture.cwd.join("message.txt");
    fs::write(&input, "Verified release notes.\n").unwrap();
    let run = || {
        let mut command = fixture.run_kind("release-notes");
        command
            .env("OWNER_GRANT", "granted")
            // A dispatched session this invocation runs inside. Its channel
            // reaches neither the policy nor the harness, and its run
            // identity is not the harness's.
            .env("GROVE_SIGNAL_FILE", fixture.root.join("outer-signal"))
            .env("GROVE_LAUNCH_DIR", fixture.root.join("outer-launch"))
            .env(
                "HARNESS_DISPATCH_EXIT_FILE",
                fixture.root.join("outer-exit"),
            )
            .env(
                "HARNESS_DISPATCH_RUN_ID",
                "0192f0c4-7a1e-4b2c-9d3e-4f5a6b7c8d9e",
            )
            .arg("--input")
            .arg(&input);
        command
    };

    let output = run().output().unwrap();

    assert!(output.status.success(), "{}", stderr(&output));
    let notes = fs::read_to_string(fixture.notes()).unwrap();
    let lines: Vec<&str> = notes.lines().collect();
    assert_eq!(lines.len(), 8, "{notes}");
    assert_ne!(lines[0], "<unset>", "the harness has a run identity");
    assert_ne!(lines[0], "0192f0c4-7a1e-4b2c-9d3e-4f5a6b7c8d9e");
    let record = Command::new(support::harness_dispatch())
        .env_clear()
        .env("HOME", &fixture.home)
        .env("PATH", "/usr/bin:/bin")
        .args(["record", "show", "--run", lines[0], "--json"])
        .output()
        .unwrap();
    assert!(record.status.success(), "{record:?}");
    assert!(String::from_utf8_lossy(&record.stdout).contains("exit_signal"));
    assert_eq!(lines[1], "<unset>", "the grant is the policy's alone");
    assert_eq!(lines[2], "granted", "the policy saw what its owner granted");
    assert_eq!(
        lines[3], "<unset>",
        "selection never holds the outer channel"
    );
    assert_eq!(lines[4], "{}", "selection is passed no parameter");
    assert_eq!(lines[5], lines[6], "selection runs in the staged directory");
    assert_eq!(lines[7], "Verified release notes.");
    assert!(store.is_file());
    assert!(!fixture.cwd.join(".grove").exists());
    assert!(!fixture.cwd.join(".jj").exists());
    assert!(!fixture.root.join("outer-signal").exists());
    assert!(!fixture.root.join("outer-exit").exists());

    // Even an explicit runtime grant cannot expose protected owner files:
    // dispatch refuses this overlap before selecting or launching a harness.
    fs::remove_file(fixture.notes()).unwrap();
    let granted = run()
        .arg("--runtime-read")
        .arg(fixture.dispatch_file("policy.ts"))
        .output()
        .unwrap();
    assert_nothing_published(&fixture, &granted);
    assert!(
        stderr(&granted).contains("confinement_overlap"),
        "{}",
        stderr(&granted)
    );
}

// The file that runs is the one dispatch resolved. An unexecutable file
// earlier on PATH is passed over, as `execvp` passes it over.
#[test]
fn an_unexecutable_file_earlier_on_path_does_not_shadow_the_reported_program() {
    let fixture = Fixture::new("");
    fixture.policy(NAMED);
    let (first, second) = (fixture.root.join("first"), fixture.root.join("second"));
    write_executable(&first.join("fake-harness"), &named_harness("shadow"), 0o644);
    write_executable(
        &second.join("fake-harness"),
        &named_harness("reported"),
        0o755,
    );
    let path = std::env::join_paths([
        first.as_path(),
        second.as_path(),
        Path::new("/usr/bin"),
        Path::new("/bin"),
    ])
    .unwrap();

    let output = fixture
        .run_kind("release-notes")
        .env("PATH", path)
        .output()
        .unwrap();

    assert_published(&fixture, &output, "reported");
}

// A relative PATH entry is resolved where dispatch selected, in the staged
// directory. The file of that name under the directory `grove` was typed in is
// not looked up a second time.
#[test]
fn a_relative_path_entry_is_resolved_where_dispatch_selected() {
    let fixture = Fixture::new("");
    fixture.policy(NAMED);
    let second = fixture.root.join("second");
    write_executable(
        &fixture.cwd.join("bin/fake-harness"),
        &named_harness("decoy"),
        0o755,
    );
    write_executable(
        &second.join("fake-harness"),
        &named_harness("reported"),
        0o755,
    );
    let path = std::env::join_paths([
        Path::new("bin"),
        second.as_path(),
        Path::new("/usr/bin"),
        Path::new("/bin"),
    ])
    .unwrap();

    let output = fixture
        .run_kind("release-notes")
        .env("PATH", path)
        .output()
        .unwrap();

    assert_published(&fixture, &output, "reported");
}

// A refusal reaches the caller as dispatch stated it, and nothing is launched.
#[test]
fn a_refused_selection_reports_its_code_message_and_remedy_and_publishes_nothing() {
    let fixture = Fixture::new(WRITES_NOTES);

    let output = fixture.run_kind("summarise").output().unwrap();

    assert_nothing_published(&fixture, &output);
    let report = stderr(&output);
    for stated in [
        "policy_refused",
        "unrouted_kind",
        "no command for summarise",
        "route the kind in the fixture policy",
    ] {
        assert!(report.contains(stated), "missing {stated:?} in {report}");
    }
    assert_eq!(fixture.transcripts(), 2, "{report}");

    // No policy at all refuses the same way, with dispatch's own remedy.
    fs::remove_file(fixture.dispatch_file("policy.ts")).unwrap();
    let output = fixture.run();
    assert_nothing_published(&fixture, &output);
    let report = stderr(&output);
    assert!(report.contains("policy_missing"), "{report}");
    assert!(report.contains("harness-dispatch init"), "{report}");
    assert_eq!(fixture.transcripts(), 4, "{report}");
}

/// A policy whose `select` marks that it started and then outlasts the test.
fn stalled(marker: &Path) -> String {
    format!(
        r#"import {{ writeFileSync }} from "node:fs";
export const policy = {{
  schemaVersion: 2,
  version: "standalone-1",
  async select() {{
    writeFileSync({marker:?}, "selecting");
    await new Promise((resolve) => setTimeout(resolve, 25_000));
    return {{ status: "selected", program: "/bin/sh", args: ["-c", "printf late > notes.md"], provider: "fixture", model: "none", effort: "none", reason: "too late" }};
  }},
}};
"#,
        marker = marker.to_str().unwrap()
    )
}

#[test]
fn a_timed_out_selection_launches_nothing_and_publishes_nothing() {
    let fixture = Fixture::new("");
    fixture.policy(&stalled(&fixture.root.join("selecting")));
    fs::write(
        fixture.dispatch_file("settings.json"),
        r#"{ "timeoutMs": 1000 }"#,
    )
    .unwrap();

    let output = fixture.run();

    assert_nothing_published(&fixture, &output);
    assert!(fixture.root.join("selecting").exists(), "select never ran");
    assert!(
        stderr(&output).contains("selection_timeout"),
        "{}",
        stderr(&output)
    );
    assert_eq!(fixture.transcripts(), 2);
}

// Grove forwards cancellation to dispatch in its own session, which cancels selection: the worker is stopped, and nothing is launched.
#[test]
fn a_signal_during_selection_launches_nothing_and_publishes_nothing() {
    let fixture = Fixture::new("");
    let marker = fixture.root.join("selecting");
    fixture.policy(&stalled(&marker));
    let mut grove = fixture
        .run_kind("release-notes")
        .process_group(0)
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let group = -i32::try_from(grove.id()).unwrap();
    let deadline = Instant::now() + Duration::from_secs(20);
    while !marker.exists() {
        assert!(Instant::now() < deadline, "selection never started");
        std::thread::sleep(Duration::from_millis(20));
    }

    // SAFETY: kill(2) on the group this test created, with a valid signal.
    assert_eq!(unsafe { libc::kill(group, libc::SIGTERM) }, 0);

    let status = grove.wait().unwrap();
    assert!(!status.success(), "{status}");
    // The whole group ends well inside the policy's 25 seconds: dispatch
    // stopped its worker instead of waiting for it.
    let deadline = Instant::now() + Duration::from_secs(10);
    // SAFETY: signal 0 only asks whether the group still has a member.
    while unsafe { libc::kill(group, 0) } == 0 {
        assert!(Instant::now() < deadline, "the selection was not cancelled");
        std::thread::sleep(Duration::from_millis(20));
    }
    assert!(!fixture.notes().exists());
    assert_eq!(fixture.transcripts(), 2);
}

// Grove finds dispatch beside its own real path. A copy of `grove` with no
// sibling names the path it looked at, and a symlink to `grove` does not move
// the lookup to the link's directory.
#[test]
fn harness_dispatch_is_found_beside_groves_real_path_and_a_missing_one_is_named() {
    let fixture = Fixture::new(WRITES_NOTES);
    let alone = fixture.root.join("alone/bin");
    fs::create_dir_all(&alone).unwrap();
    fs::copy(env!("CARGO_BIN_EXE_grove"), alone.join("grove")).unwrap();

    let output = fixture
        .grove(&alone.join("grove"))
        .args(["run", "release-notes", "Produce notes.", "--ui", "inline"])
        .arg("--output")
        .arg(fixture.notes())
        .output()
        .unwrap();

    assert_nothing_published(&fixture, &output);
    let missing = alone.join("harness-dispatch");
    assert!(
        stderr(&output).contains(missing.to_str().unwrap()),
        "{}",
        stderr(&output)
    );
    assert_eq!(fixture.transcripts(), 0);

    let linked = fixture.root.join("linked");
    fs::create_dir(&linked).unwrap();
    std::os::unix::fs::symlink(env!("CARGO_BIN_EXE_grove"), linked.join("grove")).unwrap();
    write_executable(
        &linked.join("harness-dispatch"),
        &format!(
            "#!/bin/sh\n: > '{}'\nexit 1\n",
            fixture.root.join("decoy-ran").display()
        ),
        0o755,
    );
    let mut command = fixture.grove(&linked.join("grove"));
    command
        .args(["run", "release-notes", "Produce notes.", "--ui", "inline"])
        .arg("--output")
        .arg(fixture.notes())
        .arg("--runtime-read")
        .arg(&fixture.script);

    let output = command.output().unwrap();

    assert_published(&fixture, &output, "ok");
    assert!(!fixture.root.join("decoy-ran").exists());
}

#[test]
fn failed_or_unacknowledged_harnesses_do_not_publish_outputs() {
    for tail in ["exit 0", "exit 7", "acknowledge; exit 7"] {
        let fixture = Fixture::new(&format!("printf partial > notes.md\n{tail}\n"));
        assert_nothing_published(&fixture, &fixture.run());
        assert_eq!(
            fixture.transcripts(),
            2,
            "a launch leaves its log and status"
        );
    }
}

#[test]
fn missing_output_or_symlink_prevents_publication_of_the_whole_set() {
    for body in ["printf first > notes.md", "ln -s /etc/passwd notes.md"] {
        let fixture = Fixture::new(&format!("{body}\nacknowledge\n"));
        let output = fixture
            .run_kind("release-notes")
            .arg("--output")
            .arg(fixture.cwd.join("second.md"))
            .output()
            .unwrap();
        assert_nothing_published(&fixture, &output);
    }
}

#[test]
fn existing_output_is_never_overwritten() {
    let fixture = Fixture::new("printf changed > notes.md\nacknowledge\n");
    fs::write(fixture.notes(), "preserved").unwrap();
    let output = fixture.run();
    assert!(!output.status.success(), "{}", stderr(&output));
    assert_eq!(fs::read_to_string(fixture.notes()).unwrap(), "preserved");
}

#[test]
fn replacing_the_staging_directory_cannot_redirect_export_to_host_files() {
    let outside = |fixture: &Fixture| fixture.root.join("outside");
    let fixture = Fixture::new("");
    fs::create_dir(outside(&fixture)).unwrap();
    fs::write(
        outside(&fixture).join("notes.md"),
        "private parent contents",
    )
    .unwrap();
    fs::write(
        &fixture.script,
        format!(
            "{}\nset -eu\ncd ..\nmv work original || true\nln -s '{}' work || true\nacknowledge\n",
            fs::read_to_string(&fixture.script).unwrap(),
            outside(&fixture).display()
        ),
    )
    .unwrap();
    let output = fixture.run();
    assert_nothing_published(&fixture, &output);
    assert!(
        stderr(&output).contains("reading staged output"),
        "{}",
        stderr(&output)
    );
}

#[test]
fn the_confined_harness_cannot_forge_the_supervisors_ending_report() {
    let fixture = Fixture::new(
        "printf forged > notes.md\n\
         if printf '%s' '{\"schemaVersion\":1,\"source\":\"harness-dispatch\",\"measurements\":{\"ending\":{\"state\":\"observed\",\"value\":\"exit_signal\"}}}' > ../control/ending.json; then exit 41; fi\n\
         printf report-write-denied\nexit 0\n",
    );
    let output = fixture.run();
    assert_nothing_published(&fixture, &output);
    assert!(
        stderr(&output).contains("report-write-denied"),
        "{}",
        stderr(&output)
    );
    assert!(
        stderr(&output).contains("without an exit-signal ending"),
        "{}",
        stderr(&output)
    );
}

#[test]
fn a_collision_on_the_second_output_reports_the_published_prefix() {
    let fixture = Fixture::new(
        "printf first > notes.md\nprintf second > second.md\necho ready\nsleep 1\nacknowledge\n",
    );
    let second = fixture.cwd.join("second.md");
    let grove = fixture
        .run_kind("release-notes")
        .arg("--output")
        .arg(&second)
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    wait_for_transcript(&fixture, "ready");
    fs::write(&second, "preserved").unwrap();
    let output = grove.wait_with_output().unwrap();
    assert!(!output.status.success());
    assert_eq!(fs::read_to_string(fixture.notes()).unwrap(), "first");
    assert_eq!(fs::read_to_string(&second).unwrap(), "preserved");
    let report = stderr(&output);
    assert!(report.contains("already published:"), "{report}");
    assert!(
        report.contains(fixture.notes().to_str().unwrap()),
        "{report}"
    );
}

#[test]
fn cancellation_while_staging_publication_publishes_nothing() {
    // A large output gives the observer a copy interval after dispatch has
    // exited. Host-side temp files appear only in publication, not in the run.
    let fixture =
        Fixture::new("dd if=/dev/zero of=notes.md bs=1048576 count=512 2>/dev/null\nacknowledge\n");
    let grove = fixture
        .run_kind("release-notes")
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        if fs::read_dir(&fixture.cwd).unwrap().any(|entry| {
            entry
                .unwrap()
                .file_name()
                .to_string_lossy()
                .starts_with(".tmp")
        }) {
            break;
        }
        assert!(Instant::now() < deadline, "publication copy never started");
        std::thread::sleep(Duration::from_millis(1));
    }
    // SAFETY: only the live Grove child owned by this test is signalled.
    assert_eq!(
        unsafe { libc::kill(i32::try_from(grove.id()).unwrap(), libc::SIGTERM) },
        0
    );
    let output = grove.wait_with_output().unwrap();
    assert_nothing_published(&fixture, &output);
    assert!(
        stderr(&output).contains("publication cancelled; already published: []"),
        "{}",
        stderr(&output)
    );
}

/// Dropping dispatch's group cleanup would let this writer change the output
/// after validation. A stable exported value and absent PID prove the boundary.
#[test]
fn acknowledged_writer_is_stopped_before_publication() {
    let fixture = Fixture::new(
        "set -eu\n( trap '' TERM; while :; do printf writer > notes.md; sleep 0.02; done ) &\n\
         writer=$!\nprintf '%s' \"$writer\" > writer.pid\nsleep 0.1\nacknowledge\n",
    );
    let pid_file = fixture.cwd.join("writer.pid");
    let output = fixture
        .run_kind("release-notes")
        .arg("--output")
        .arg(&pid_file)
        .output()
        .unwrap();
    assert_published(&fixture, &output, "writer");
    let pid: i32 = fs::read_to_string(pid_file).unwrap().parse().unwrap();
    // SAFETY: signal 0 queries the PID the confined harness reported.
    assert_eq!(unsafe { libc::kill(pid, 0) }, -1);
    std::thread::sleep(Duration::from_millis(150));
    assert_eq!(fs::read_to_string(fixture.notes()).unwrap(), "writer");
}

fn wait_for_transcript(fixture: &Fixture, marker: &str) -> String {
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        if let Ok(files) = fs::read_dir(fixture.home.join(".local/state/grove/runs")) {
            for file in files.flatten() {
                if file.path().extension().is_some_and(|ext| ext == "log") {
                    let text = fs::read_to_string(file.path()).unwrap();
                    if text.contains(marker) {
                        return text;
                    }
                }
            }
        }
        assert!(Instant::now() < deadline, "no transcript marker {marker}");
        std::thread::sleep(Duration::from_millis(20));
    }
}

/// Forgetting to forward cancellation would leave the confined run alive and
/// could publish after its eventual acknowledgement.
#[test]
fn cancellation_during_run_reaps_the_harness_and_preserves_enclosing_controls() {
    let fixture = Fixture::new(
        "trap '' TERM INT\nprintf partial > notes.md\nprintf 'ready:%s\\n' \"$$\"\n\
         while :; do sleep 0.05; done\n",
    );
    let mut grove = fixture
        .run_kind("release-notes")
        .env("GROVE_SIGNAL_FILE", fixture.root.join("outer-signal"))
        .env("GROVE_LAUNCH_DIR", fixture.root.join("outer-launch"))
        .env(
            "HARNESS_DISPATCH_EXIT_FILE",
            fixture.root.join("outer-exit"),
        )
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let text = wait_for_transcript(&fixture, "ready:");
    let pid: i32 = text
        .lines()
        .find_map(|line| line.strip_prefix("ready:"))
        .unwrap()
        .parse()
        .unwrap();
    let started = Instant::now();
    // SAFETY: the test owns this live Grove child.
    assert_eq!(
        unsafe { libc::kill(i32::try_from(grove.id()).unwrap(), libc::SIGTERM) },
        0
    );
    assert!(!grove.wait().unwrap().success());
    assert!(started.elapsed() < Duration::from_secs(5));
    // SAFETY: signal 0 queries the reported harness PID.
    assert_eq!(unsafe { libc::kill(pid, 0) }, -1);
    assert!(!fixture.notes().exists());
    assert!(!fixture.root.join("outer-signal").exists());
    assert!(!fixture.root.join("outer-exit").exists());
}

#[test]
fn destination_created_during_run_is_never_overwritten() {
    let fixture = Fixture::new("printf changed > notes.md\necho ready\nsleep 1\nacknowledge\n");
    let grove = fixture
        .run_kind("release-notes")
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    wait_for_transcript(&fixture, "ready");
    fs::write(fixture.notes(), "preserved").unwrap();
    let output = grove.wait_with_output().unwrap();
    assert!(!output.status.success(), "{}", stderr(&output));
    assert!(stderr(&output).contains("without overwriting"));
    assert_eq!(fs::read_to_string(fixture.notes()).unwrap(), "preserved");
}

#[test]
fn confined_harness_cannot_consume_stdin_or_parent_handles_but_can_read_runtime_grants() {
    let fixture = Fixture::new(
        "set -eu\nif read word; then exit 41; fi\nif test -r /dev/tty; then exit 42; fi\n\
         test -z \"${TMUX-}\"\ntest -z \"${ZELLIJ-}\"\n\
         test -z \"${JJ_REPO-}\"\ntest -z \"${GIT_DIR-}\"\n\
         cat credential.txt > notes.md\nacknowledge\n",
    );
    let runtime = fixture.root.join("credential.txt");
    fs::write(&runtime, "runtime grant").unwrap();
    // Pass the credential's path through the policy, without exposing it as input.
    let source = fs::read_to_string(fixture.dispatch_file("policy.ts")).unwrap();
    fixture.policy(&source.replace("request.prompt,", &format!("request.prompt, {runtime:?},")));
    let body = fs::read_to_string(&fixture.script)
        .unwrap()
        .replace("cat credential.txt", "cat \"$2\"");
    fs::write(&fixture.script, body).unwrap();
    let mut grove = fixture
        .run_kind("release-notes")
        .env("JJ_REPO", fixture.root.join("not-a-repository"))
        .env("GIT_DIR", fixture.root.join("not-a-repository"))
        .env("TMUX", "parent-socket")
        .env("ZELLIJ", "parent-pane")
        .arg("--runtime-read")
        .arg(runtime)
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    use std::io::Write as _;
    let _ = grove.stdin.take().unwrap().write_all(b"caller input\n");
    assert_published(
        &fixture,
        &grove.wait_with_output().unwrap(),
        "runtime grant",
    );
}

/// Either nested acknowledgement or cancellation must finish only the inner
/// invocation. The real enclosing dispatch then acknowledges independently.
#[test]
fn nested_exit_and_cancellation_leave_the_enclosing_dispatched_run_intact() {
    for cancel in [false, true] {
        let fixture = Fixture::new(if cancel {
            "trap '' TERM INT\necho nested-ready\nwhile :; do sleep 0.05; done\n"
        } else {
            WRITES_NOTES
        });
        let outer = fixture.root.join("outer.sh");
        let grove = Path::new(env!("CARGO_BIN_EXE_grove"));
        let dispatch = support::harness_dispatch().canonicalize().unwrap();
        let body = format!(
            "set -eu\nouter_id=$HARNESS_DISPATCH_RUN_ID\nouter_exit=$HARNESS_DISPATCH_EXIT_FILE\n\
             '{}' run release-notes 'Produce notes.' --ui inline --runtime-read '{}' --output '{}' &\ninner=$!\n{}\n\
             test \"$outer_id\" = \"$HARNESS_DISPATCH_RUN_ID\"\ntest ! -e \"$outer_exit\"\n\
             printf survived > '{}'\n'{}' exit\n",
            grove.display(), fixture.script.display(), fixture.notes().display(),
            if cancel {
                format!("while ! grep -q nested-ready '{}'/runs/*.log 2>/dev/null; do sleep 0.02; done\nkill -TERM \"$inner\"\nif wait \"$inner\"; then exit 41; fi", fixture.home.join(".local/state/grove").display())
            } else { "wait \"$inner\"".to_owned() },
            fixture.cwd.join("outer-survived").display(), dispatch.display()
        );
        fs::write(&outer, body).unwrap();
        let source = fs::read_to_string(fixture.dispatch_file("policy.ts")).unwrap();
        fixture.policy(&source.replace("if (request.kind !==", &format!(
            "if (request.kind === 'enclosing') return {{ status: 'selected', program: '/bin/sh', args: [{outer:?}], provider: 'fixture', model: 'none', effort: 'none', reason: 'outer supervisor' }};\n    if (request.kind !=="
        )));
        let output = fixture
            .grove(&dispatch)
            .args([
                "run",
                "--kind",
                "enclosing",
                "--prompt",
                "Enclose the invocation.",
            ])
            .output()
            .unwrap();
        assert!(output.status.success(), "{}", stderr(&output));
        assert_eq!(
            fs::read_to_string(fixture.cwd.join("outer-survived")).unwrap(),
            "survived"
        );
        assert_eq!(fixture.notes().exists(), !cancel);
    }
}

#[test]
fn acknowledgement_names_the_canonical_dispatch_path_when_the_sibling_is_a_symlink() {
    let fixture = Fixture::new("printf '%s' \"$dispatch_exit\" > notes.md\nacknowledge\n");
    let bin = fixture.root.join("aliased-bin");
    fs::create_dir(&bin).unwrap();
    fs::copy(env!("CARGO_BIN_EXE_grove"), bin.join("grove")).unwrap();
    let dispatch = support::harness_dispatch().canonicalize().unwrap();
    std::os::unix::fs::symlink(&dispatch, bin.join("harness-dispatch")).unwrap();
    let output = fixture
        .grove(&bin.join("grove"))
        .args(["run", "release-notes", "Produce notes.", "--ui", "inline"])
        .arg("--runtime-read")
        .arg(&fixture.script)
        .arg("--output")
        .arg(fixture.notes())
        .output()
        .unwrap();
    assert_published(&fixture, &output, &format!("'{}' exit", dispatch.display()));
}
