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
use std::os::unix::process::{CommandExt as _, ExitStatusExt as _};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

use tempfile::TempDir;

/// A harness that publishes one line and acknowledges.
const WRITES_NOTES: &str = "printf ok > notes.md\nprintf done > \"$GROVE_RUN_SIGNAL_FILE\"\n";

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
    /// the prompt and the three parameters, and refuses every other kind.
    fn new(harness: &str) -> Self {
        // The siblings `grove` finds and copies from its own directory.
        support::harness_dispatch();
        support::grove_llm();
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
        fs::write(&fixture.script, harness).unwrap();
        fixture.policy(&format!(
            r#"export const policy = {{
  schemaVersion: 2,
  version: "standalone-1",
  select(request) {{
    if (request.kind !== "release-notes") {{
      return {{ status: "refused", code: "unrouted_kind", message: `no command for ${{request.kind}}`, remedy: "route the kind in the fixture policy" }};
    }}
    const {{ session_name, worktree, repo }} = request.params;
    return {{ status: "selected", program: "/bin/sh", args: [{script:?}, request.prompt, session_name, worktree, repo], provider: "fixture", model: "none", effort: "none", reason: "the deterministic harness" }};
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
    /// leaves none.
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
    format!("#!/bin/sh\nprintf {word} > notes.md\nprintf done > \"$GROVE_RUN_SIGNAL_FILE\"\n")
}

/// A policy that returns the bare program name `fake-harness`.
const NAMED: &str = r#"export const policy = {
  schemaVersion: 2,
  version: "standalone-1",
  select: () => ({ status: "selected", program: "fake-harness", args: [], provider: "fixture", model: "none", effort: "none", reason: "a program found on PATH" }),
};
"#;

// The policy selects outside the sandbox, with what the owner granted it. The
// harness runs inside, with the invocation's prompt and parameters and none of
// the owner's personal files. Old Grove configuration files change nothing.
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
         test -z \"${TMUX-}\"\n\
         case \"$1\" in 'Produce notes.'*) ;; *) exit 9 ;; esac\n\
         printf '%s\\n' \"${HARNESS_DISPATCH_RUN_ID-<unset>}\" \"${OWNER_GRANT-<unset>}\" \
         \"$2\" \"$3\" \"$4\" \"$5\" \"$6\" \"$(pwd -P)\" > notes.md\n\
         cat message.txt >> notes.md\n\
         printf done > \"$GROVE_RUN_SIGNAL_FILE\"\n",
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
    fs::write(&store, "a store no run is added to").unwrap();
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
    assert_eq!(lines.len(), 9, "{notes}");
    assert_eq!(lines[0], "<unset>", "the harness has no run identity");
    assert_eq!(lines[1], "<unset>", "the grant is the policy's alone");
    assert_eq!(lines[2], "granted", "the policy saw what its owner granted");
    assert_eq!(
        lines[3], "<unset>",
        "selection never holds the outer channel"
    );
    assert_eq!(lines[4], "standalone:release-notes");
    assert_eq!(lines[5], lines[7], "worktree is the staged directory");
    assert_eq!(lines[6], lines[7], "repo is the staged directory");
    assert_eq!(lines[8], "Verified release notes.");
    assert_eq!(
        fs::read_to_string(&store).unwrap(),
        "a store no run is added to"
    );
    assert!(!fixture.cwd.join(".grove").exists());
    assert!(!fixture.cwd.join(".jj").exists());
    assert!(!fixture.root.join("outer-signal").exists());

    // The control on the three reads above: the same probe fires once the
    // file is granted, so its path is the one the harness would have read.
    fs::remove_file(fixture.notes()).unwrap();
    let granted = run()
        .arg("--runtime-read")
        .arg(fixture.dispatch_file("policy.ts"))
        .output()
        .unwrap();
    assert_nothing_published(&fixture, &granted);
    assert!(
        stderr(&granted).contains("exit status: 41"),
        "{}",
        stderr(&granted)
    );
}

// The file that runs is the one inspection reported. An unexecutable file
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

// A relative PATH entry is resolved where inspection ran, in the staged
// directory. The file of that name under the directory `grove` was typed in is
// not looked up a second time.
#[test]
fn a_relative_path_entry_is_resolved_where_inspection_ran() {
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
        "grove run cannot launch kind `summarise`",
        "policy_refused: unrouted_kind",
        "no command for summarise",
        "route the kind in the fixture policy",
    ] {
        assert!(report.contains(stated), "missing {stated:?} in {report}");
    }
    assert_eq!(fixture.transcripts(), 0, "{report}");

    // No policy at all refuses the same way, with dispatch's own remedy.
    fs::remove_file(fixture.dispatch_file("policy.ts")).unwrap();
    let output = fixture.run();
    assert_nothing_published(&fixture, &output);
    let report = stderr(&output);
    assert!(report.contains("policy_missing"), "{report}");
    assert!(report.contains("harness-dispatch init"), "{report}");
    assert_eq!(fixture.transcripts(), 0, "{report}");
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
    assert_eq!(fixture.transcripts(), 0);
}

// Selection stays in Grove's own process group, so a signal to the group
// cancels it: the worker is stopped, and nothing is launched.
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
    assert_eq!(status.signal(), Some(libc::SIGTERM), "{status}");
    // The whole group ends well inside the policy's 25 seconds: dispatch
    // stopped its worker instead of waiting for it.
    let deadline = Instant::now() + Duration::from_secs(10);
    // SAFETY: signal 0 only asks whether the group still has a member.
    while unsafe { libc::kill(group, 0) } == 0 {
        assert!(Instant::now() < deadline, "the selection was not cancelled");
        std::thread::sleep(Duration::from_millis(20));
    }
    assert!(!fixture.notes().exists());
    assert_eq!(fixture.transcripts(), 0);
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
    for tail in [
        "exit 0",
        "printf bogus > \"$GROVE_RUN_SIGNAL_FILE\"",
        "printf done > \"$GROVE_RUN_SIGNAL_FILE\"; exit 7",
    ] {
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
        let fixture = Fixture::new(&format!(
            "{body}\nprintf done > \"$GROVE_RUN_SIGNAL_FILE\"\n"
        ));
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
    let fixture =
        Fixture::new("printf changed > notes.md\nprintf done > \"$GROVE_RUN_SIGNAL_FILE\"\n");
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
            "set -eu\ncd ..\nmv work original\nln -s '{}' work\nprintf done > \"$GROVE_RUN_SIGNAL_FILE\"\n",
            outside(&fixture).display()
        ),
    )
    .unwrap();
    assert_nothing_published(&fixture, &fixture.run());
}
