//! `grove run` under real confinement: the real `grove`, the real
//! independent Ravel build, a configuration in a temporary HOME, and a
//! deterministic shell harness. Nothing here calls a model.
//!
//! The build is supplied explicitly by GROVE_TEST_RAVEL, with its runtime in
//! GROVE_TEST_RAVEL_RUNTIME, and copied to an independent prefix. Missing
//! files of its layout fail rather than skip.

mod support;

use support::ravel::{quote, Runtime, RHAI_GRANTED};

use std::fs;
use std::os::unix::fs::PermissionsExt as _;
use std::os::unix::process::CommandExt as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

use tempfile::TempDir;

/// A harness that publishes one line and acknowledges.
const WRITES_NOTES: &str = "printf ok > notes.md\nacknowledge\n";

/// Removing a control from the production scrub list must make the policy
/// refuse, even when the owner explicitly grants that variable to selection.
#[test]
fn standalone_scrubs_enclosing_ravel_and_retired_controls_before_selection() {
    let fixture = Fixture::new(WRITES_NOTES);
    let controls = [
        "GROVE_SIGNAL_FILE",
        "GROVE_RUN_SIGNAL_FILE",
        "GROVE_HARNESS_PID",
        "GROVE_CLAUDE_PID",
        "GROVE_PROVIDER_EVIDENCE",
    ];
    fixture.configure(&Hooks {
        guard: Some(&controls),
        ..Hooks::default()
    });
    fs::write(
        fixture.dispatch_file("settings.json"),
        serde_json::json!({support::runtime().grant_key(): ["OWNER_GRANT", "GROVE_SIGNAL_FILE", "GROVE_RUN_SIGNAL_FILE", "GROVE_HARNESS_PID", "GROVE_CLAUDE_PID", "GROVE_PROVIDER_EVIDENCE"]}).to_string(),
    ).unwrap();

    // Dirty control: the same policy sees the granted controls when Grove is
    // bypassed, so a clean Grove run cannot come from a probe that reads nothing.
    let mut direct = fixture.grove(&support::installed_ravel());
    direct.args(["start-session", "--kind=release-notes", "--prompt=control"]);
    direct.env("OWNER_GRANT", "granted");
    for name in controls {
        let dirty = direct
            .env(name, "/inert-enclosing-control")
            .output()
            .unwrap();
        assert_eq!(dirty.status.code(), Some(3), "{name}: {dirty:?}");
        assert!(String::from_utf8_lossy(&dirty.stderr)
            .contains("enclosing authority reached selection"));
        direct.env_remove(name);
    }
    let probe = support::ravel_entry_probe(&fixture.root.join("entry-bin"));
    support::observe_dirty_entry(&probe, &fixture.home);

    let mut grove = fixture.run_kind("release-notes");
    grove
        .env("OWNER_GRANT", "granted")
        .env("PATH", support::path_with_entry(probe.parent().unwrap()))
        .arg("--runtime-read")
        .arg(&probe);
    for name in support::LOOP_CONTROL_ENV {
        grove.env(name, "/inert-enclosing-control");
    }
    let output = grove.output().unwrap();
    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(fs::read_to_string(fixture.notes()).unwrap(), "ok");
}

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
        support::installed_ravel();

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
            "dispatch_exit=$(printf '%s\\n' \"$1\" | sed -n '/exact command as your final action:/{{n;p;}}')\n\
             force_stop=$(printf '%s\\n' \"$1\" | sed -n '/To stop without completing, explain the failure and run:/{{n;p;}}')\n\
             acknowledge() {{ eval \"$dispatch_exit\"; }}\n\
             stop_work() {{ test -n \"$force_stop\"; eval \"$force_stop\"; }}\n{harness}"
        )).unwrap();
        fixture.configure(&Hooks::default());
        fixture
    }

    fn dispatch_file(&self, name: &str) -> PathBuf {
        self.home.join(".config/ravel").join(name)
    }

    /// Replace the configuration with `source`, at the default entry of the
    /// runtime under test.
    fn policy(&self, source: &str) {
        support::write_config(&self.home, source);
    }

    /// The default entry of the runtime under test.
    fn config_file(&self) -> PathBuf {
        support::config_entry(&self.home)
    }

    /// The default configuration with what a case adds. It routes
    /// `release-notes` to `/bin/sh <script>` with the prompt, any extra
    /// arguments, the parameter object and cwd, and refuses every other kind.
    fn configure(&self, hooks: &Hooks) {
        self.policy(&hooks.source(support::runtime(), &self.script));
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
            .env("PATH", support::ravel_path())
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

/// What a case adds to the fixture's default configuration.
#[derive(Default)]
struct Hooks<'a> {
    /// Refuse as `enclosing_authority` unless the granted `OWNER_GRANT` is
    /// `granted` and none of these names reaches selection.
    guard: Option<&'a [&'a str]>,
    /// Arguments between the prompt and the parameter object.
    after_prompt: Vec<Extra<'a>>,
    /// Route the kind `enclosing` to `/bin/sh` with this script.
    enclosing: Option<&'a Path>,
}

/// One argument that [`Hooks`] adds after the prompt.
enum Extra<'a> {
    /// The value of a granted name, or `<unset>`.
    Env(&'a str),
    Text(&'a str),
}

impl Hooks<'_> {
    fn source(&self, runtime: Runtime, script: &Path) -> String {
        let script = quote(script.to_str().unwrap());
        let names = |names: &[&str]| {
            names
                .iter()
                .map(|name| quote(name))
                .collect::<Vec<_>>()
                .join(", ")
        };
        let enclosing = self.enclosing.map(|outer| quote(outer.to_str().unwrap()));
        match runtime {
            Runtime::V1_1 => {
                let guard = self.guard.map_or(String::new(), |controls| {
                    format!(
                        "    if (process.env.OWNER_GRANT !== 'granted' || \
                         [{}].some(name => process.env[name] !== undefined)) \
                         return {{ status: 'refused', code: 'enclosing_authority', \
                         message: 'enclosing authority reached selection', remedy: 'scrub before selection' }};\n",
                        names(controls)
                    )
                });
                let enclosing = enclosing.map_or(String::new(), |outer| {
                    format!(
                        "    if (request.kind === 'enclosing') return {{ status: 'selected', program: '/bin/sh', \
                         args: [{outer}], provider: 'fixture', model: 'none', effort: 'none', reason: 'outer supervisor' }};\n"
                    )
                });
                let extras: String = self
                    .after_prompt
                    .iter()
                    .map(|extra| match extra {
                        Extra::Env(name) => format!("process.env.{name} ?? \"<unset>\", "),
                        Extra::Text(text) => format!("{}, ", quote(text)),
                    })
                    .collect();
                format!(
                    r#"export const policy = {{
  schemaVersion: 2,
  version: "standalone-1",
  select(request) {{
{guard}{enclosing}    if (request.kind !== "release-notes") {{
      return {{ status: "refused", code: "unrouted_kind", message: `no command for ${{request.kind}}`, remedy: "route the kind in the fixture policy" }};
    }}
    return {{ status: "selected", program: "/bin/sh", args: [{script}, request.prompt, {extras}JSON.stringify(request.params), request.cwd], provider: "fixture", model: "none", effort: "none", reason: "the deterministic harness" }};
  }},
}};
"#
                )
            }
            Runtime::V2_0 => {
                let guard = self.guard.map_or(String::new(), |controls| {
                    format!(
                        "    let reached = or_unset(granted(\"OWNER_GRANT\")) != \"granted\";\n\
                         \x20   for name in [{}] {{ if type_of(granted(name)) != \"()\" {{ reached = true; }} }}\n\
                         \x20   if reached {{\n\
                         \x20       return #{{ status: \"refused\", code: \"enclosing_authority\", \
                         message: \"enclosing authority reached selection\", remedy: \"scrub before selection\" }};\n\
                         \x20   }}\n",
                        names(controls)
                    )
                });
                let enclosing = enclosing.map_or(String::new(), |outer| {
                    format!(
                        "    if request.kind == \"enclosing\" {{\n\
                         \x20       return #{{ status: \"selected\", program: \"/bin/sh\", args: [{outer}], \
                         provider: \"fixture\", model: \"none\", effort: \"none\", reason: \"outer supervisor\" }};\n\
                         \x20   }}\n"
                    )
                });
                let extras: String = self
                    .after_prompt
                    .iter()
                    .map(|extra| match extra {
                        Extra::Env(name) => format!("or_unset(granted({})), ", quote(name)),
                        Extra::Text(text) => format!("{}, ", quote(text)),
                    })
                    .collect();
                format!(
                    r#"export const SCHEMA_VERSION = 4;
export const VERSION = "standalone-1";

{RHAI_GRANTED}
fn select(request, context) {{
{guard}{enclosing}    if request.kind != "release-notes" {{
        return #{{ status: "refused", code: "unrouted_kind", message: `no command for ${{request.kind}}`, remedy: "route the kind in the fixture policy" }};
    }}
    #{{ status: "selected", program: "/bin/sh", args: [{script}, request.prompt, {extras}ravel::json_encode(request.params), request.cwd], provider: "fixture", model: "none", effort: "none", reason: "the deterministic harness" }}
}}
"#
                )
            }
        }
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

/// Missing/wrong prompt commands must prevent completion or an explicit stop;
/// treating force-stop as success must never publish even a valid output set.
#[test]
fn prompt_commands_complete_or_force_stop_without_touching_enclosing_channels() {
    for stop in [false, true] {
        let fixture = Fixture::new(&format!(
            "set -eu\nprintf first > notes.md\nprintf second > second.md\n\
             ( trap '' TERM; while :; do sleep 0.05; done ) &\n\
             printf 'writer:%s\\n' \"$!\"\n{}\n",
            if stop { "stop_work" } else { "acknowledge" }
        ));
        let second = fixture.cwd.join("second.md");
        let controls = [
            ("GROVE_SIGNAL_FILE", "outer-signal"),
            ("GROVE_RUN_SIGNAL_FILE", "outer-standalone"),
            ("HARNESS_DISPATCH_EXIT_FILE", "outer-legacy-exit"),
            ("RAVEL_EXIT_FILE", "outer-exit"),
        ];
        let mut command = fixture.run_kind("release-notes");
        command.arg("--output").arg(&second);
        for (name, path) in controls {
            fs::write(fixture.root.join(path), "enclosing").unwrap();
            command.env(name, fixture.root.join(path));
        }
        let output = command.output().unwrap();
        let writer: i32 = stderr(&output)
            .lines()
            .find_map(|line| line.strip_prefix("writer:"))
            .unwrap()
            .parse()
            .unwrap();
        // SAFETY: signal 0 only queries the reported confined descendant PID.
        assert_eq!(unsafe { libc::kill(writer, 0) }, -1);
        if stop {
            assert_eq!(output.status.code(), Some(1), "{output:?}");
            assert_nothing_published(&fixture, &output);
            assert!(!second.exists());
            assert!(
                stderr(&output).contains("force-stop requested"),
                "{output:?}"
            );
            assert!(
                stderr(&output).contains("outputs were not published"),
                "{output:?}"
            );
        } else {
            assert_published(&fixture, &output, "first");
            assert_eq!(fs::read_to_string(second).unwrap(), "second");
        }
        let status = fs::read_dir(fixture.home.join(".local/state/grove/runs"))
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .find(|path| path.extension().is_some_and(|ext| ext == "status"))
            .unwrap();
        assert_eq!(
            fs::read_to_string(status).unwrap(),
            if stop { "failed" } else { "completed" }
        );
        for (_, path) in controls {
            assert_eq!(
                fs::read_to_string(fixture.root.join(path)).unwrap(),
                "enclosing"
            );
        }
    }
}

fn write_executable(path: &Path, body: &str, mode: u32) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, body).unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(mode)).unwrap();
}

/// A harness on PATH that publishes `word`, for a policy that returns a name.
fn named_harness(word: &str) -> String {
    format!(
        "#!/bin/sh\nprintf {word} > notes.md\n'{}' end-session grove-continue\n",
        support::installed_ravel().canonicalize().unwrap().display()
    )
}

/// A configuration that returns the bare program name `fake-harness`.
fn named() -> String {
    let mut route = support::ravel::Route::new("standalone-1", "fake-harness", vec![]);
    route.reason = "a program found on PATH";
    route.source(support::runtime())
}

// The policy selects outside the sandbox, with what the owner granted it, in
// the staged directory and with no parameter. The harness runs inside, with the
// invocation's prompt and none of the owner's personal files. Old Grove configuration files change nothing.
#[test]
fn the_policy_selects_outside_the_sandbox_and_the_harness_runs_inside_it() {
    let fixture = Fixture::new(&format!(
        "set -eu\n\
         for file in \"$HOME/.config/ravel/{config}\" \
         \"$HOME/.config/ravel/settings.json\" \
         \"$HOME/.local/state/harness-dispatch/records.sqlite3\"; do\n\
         if cat \"$file\" > /dev/null 2>&1; then exit 41; fi\n\
         done\n\
         test -z \"${{GROVE_SIGNAL_FILE-}}\"\n\
         test -z \"${{GROVE_LAUNCH_DIR-}}\"\n\
         test -z \"${{TMUX-}}\"\n\
         case \"$1\" in 'Produce notes.'*) ;; *) exit 9 ;; esac\n\
         printf '%s\\n' \"${{RAVEL_EXIT_FILE-<unset>}}\" \"${{OWNER_GRANT-<unset>}}\" \
         \"$2\" \"$3\" \"$4\" \"$5\" \"$(pwd -P)\" > notes.md\n\
         cat message.txt >> notes.md\n\
         acknowledge\n",
        config = support::runtime().config_file(),
    ));
    // The policy reports two variables its owner granted, as two more
    // arguments. One of them is the channel of a session this invocation runs
    // inside, which the owner should never grant and here has.
    fixture.configure(&Hooks {
        after_prompt: vec![Extra::Env("OWNER_GRANT"), Extra::Env("GROVE_SIGNAL_FILE")],
        ..Hooks::default()
    });
    fs::write(
        fixture.dispatch_file("settings.json"),
        serde_json::json!({support::runtime().grant_key(): ["OWNER_GRANT", "GROVE_SIGNAL_FILE"]})
            .to_string(),
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
            .env("RAVEL_EXIT_FILE", fixture.root.join("outer-exit"))
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
    assert!(
        !Path::new(lines[0]).exists(),
        "the session context is stale after reap"
    );
    assert_eq!(lines[1], "<unset>", "the grant is the policy's alone");
    assert_eq!(lines[2], "granted", "the policy saw what its owner granted");
    assert_eq!(
        lines[3], "<unset>",
        "selection never holds the outer channel"
    );
    assert_eq!(lines[4], "{}", "selection is passed no parameter");
    assert_eq!(lines[5], lines[6], "selection runs in the staged directory");
    assert_eq!(lines[7], "Verified release notes.");
    assert!(
        !store.exists(),
        "the independent router creates no record store"
    );
    assert!(!fixture.cwd.join(".grove").exists());
    assert!(!fixture.cwd.join(".jj").exists());
    assert!(!fixture.root.join("outer-signal").exists());
    assert!(!fixture.root.join("outer-exit").exists());

    // Even an explicit runtime grant cannot expose protected owner files:
    // dispatch refuses this overlap before selecting or launching a harness.
    fs::remove_file(fixture.notes()).unwrap();
    let granted = run()
        .arg("--runtime-read")
        .arg(fixture.config_file())
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
    fixture.policy(&named());
    let (first, second) = (fixture.root.join("first"), fixture.root.join("second"));
    write_executable(&first.join("fake-harness"), &named_harness("shadow"), 0o644);
    write_executable(
        &second.join("fake-harness"),
        &named_harness("reported"),
        0o755,
    );
    let path = std::env::join_paths([
        support::installed_ravel().parent().unwrap(),
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
    fixture.policy(&named());
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
        support::installed_ravel().parent().unwrap(),
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
    let runtime = support::runtime();
    for stated in [
        &runtime.code("refused"),
        "unrouted_kind",
        "no command for summarise",
        "route the kind in the fixture policy",
    ] {
        assert!(report.contains(stated), "missing {stated:?} in {report}");
    }
    assert_eq!(fixture.transcripts(), 2, "{report}");

    // No policy at all refuses the same way, with dispatch's own remedy.
    fs::remove_file(fixture.config_file()).unwrap();
    let output = fixture.run();
    assert_nothing_published(&fixture, &output);
    let report = stderr(&output);
    assert!(report.contains(&runtime.code("missing")), "{report}");
    assert!(report.contains(runtime.missing_remedy()), "{report}");
    assert_eq!(fixture.transcripts(), 4, "{report}");
}

/// A configuration whose `select` outlasts the test. The 1.1 policy marks
/// that it started. A Rhai script writes no file, so a 2.0 case finds its
/// worker instead.
fn stalled(marker: &Path) -> String {
    if support::runtime() == Runtime::V2_0 {
        return r#"export const SCHEMA_VERSION = 4;
export const VERSION = "standalone-1";

fn select(request, context) {
    sleep(25);
    #{ status: "selected", program: "/bin/sh", args: ["-c", "printf late > notes.md"], provider: "fixture", model: "none", effort: "none", reason: "too late" }
}
"#
        .to_owned();
    }
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
    if support::runtime() == Runtime::V1_1 {
        assert!(fixture.root.join("selecting").exists(), "select never ran");
    }
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
    if support::runtime() == Runtime::V2_0 {
        support::wait_for_rhai_worker(grove.id(), Duration::from_secs(20));
    } else {
        let deadline = Instant::now() + Duration::from_secs(20);
        while !marker.exists() {
            assert!(Instant::now() < deadline, "selection never started");
            std::thread::sleep(Duration::from_millis(20));
        }
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
fn ravel_is_found_on_path_independently_of_groves_real_path() {
    let fixture = Fixture::new(WRITES_NOTES);
    let alone = fixture.root.join("alone/bin");
    fs::create_dir_all(&alone).unwrap();
    fs::copy(env!("CARGO_BIN_EXE_grove"), alone.join("grove")).unwrap();

    let output = fixture
        .grove(&alone.join("grove"))
        .env("PATH", "/usr/bin:/bin")
        .args(["run", "release-notes", "Produce notes.", "--ui", "inline"])
        .arg("--output")
        .arg(fixture.notes())
        .output()
        .unwrap();

    assert_nothing_published(&fixture, &output);
    assert!(
        stderr(&output).contains("brew install linkuistics/taps/ravel"),
        "{output:?}"
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
fn unacknowledged_harnesses_do_not_publish_outputs() {
    for tail in ["exit 0", "exit 7"] {
        let fixture = Fixture::new(&format!("printf partial > notes.md\n{tail}\n"));
        assert_nothing_published(&fixture, &fixture.run());
        assert_eq!(
            fixture.transcripts(),
            2,
            "a launch leaves its log and status"
        );
    }
}

/// Accepting padded, malformed or unsupported results would publish a valid
/// staged artifact even though the harness never acknowledged completion.
#[test]
fn unsupported_results_do_not_publish_valid_outputs() {
    for result in [
        "other",
        "grove-force-stop\n",
        " grove-continue",
        "grove-continue\n",
    ] {
        let encoded = result.replace('\n', "\\n");
        // The underscore preserves a trailing newline across shell command
        // substitution, then parameter expansion removes only that sentinel.
        let fixture = Fixture::new(&format!(
            "set -eu\nprintf valid > notes.md\nresult=$(printf '{encoded}_')\n\
             '{}' end-session \"${{result%_}}\"\n",
            support::installed_ravel().canonicalize().unwrap().display()
        ));
        let output = fixture.run();
        assert_nothing_published(&fixture, &output);
        assert!(
            stderr(&output).contains("unsupported session result"),
            "{output:?}"
        );
    }
}

#[test]
fn an_acknowledged_result_survives_the_managed_process_nonzero_exit() {
    let fixture = Fixture::new("printf published > notes.md\nacknowledge; exit 7\n");
    assert_published(&fixture, &fixture.run(), "published");
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
fn a_legacy_report_cannot_forge_the_held_public_result() {
    let fixture = Fixture::new(
        "printf forged > notes.md\n\
         printf grove-continue > ../control/ending.json || true\n\
         printf legacy-report-ignored\nexit 0\n",
    );
    let output = fixture.run();
    assert_nothing_published(&fixture, &output);
    assert!(
        stderr(&output).contains("legacy-report-ignored"),
        "{}",
        stderr(&output)
    );
    assert!(
        stderr(&output).contains("session_result_missing"),
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
    // Cleanup may kill the writer at any instruction. Replace complete files
    // so stopping between truncation and writing cannot leave an empty value.
    let fixture = Fixture::new(
        "set -eu\nprintf writer > notes.md\n\
         ( trap '' TERM; while :; do printf writer > notes.next; mv notes.next notes.md; sleep 0.02; done ) &\n\
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
        .env("RAVEL_EXIT_FILE", fixture.root.join("outer-exit"))
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

/// The group that `pid` leads and the foreground group of its terminal.
fn groups(pid: u32) -> (u32, u32) {
    let output = Command::new("/bin/ps")
        .args(["-o", "pgid=", "-o", "tpgid=", "-p", &pid.to_string()])
        .output()
        .unwrap();
    let text = String::from_utf8_lossy(&output.stdout).into_owned();
    let numbers: Vec<u32> = text
        .split_whitespace()
        .map(|word| word.parse().unwrap())
        .collect();
    let [group, foreground] = numbers.as_slice() else {
        panic!("ps reported {text:?} for {pid}");
    };
    (*group, *foreground)
}

/// `grove run` is a detached job: it keeps the terminal, so a Ctrl-C typed
/// there reaches Grove and no other process. The case types it on a
/// pseudo-terminal of its own, never the terminal of the developer.
///
/// A foreground that moved to `ravel` would send the interrupt past Grove. The
/// run would then end as a failed Ravel and not as a cancellation, so the
/// message of the error is part of the check.
#[test]
fn a_typed_ctrl_c_cancels_grove_run_and_the_foreground_stays_with_it() {
    use std::io::{Read as _, Write as _};
    use std::os::fd::{AsRawFd as _, FromRawFd as _};
    use std::sync::{Arc, Mutex};

    const INTERRUPT: u8 = 0x03;

    let fixture = Fixture::new(
        "trap '' TERM INT\nprintf partial > notes.md\nprintf 'ready:%s\\n' \"$$\"\n\
         while :; do sleep 0.05; done\n",
    );
    let (mut master, mut slave) = (-1, -1);
    // SAFETY: valid out pointers, with the default settings and size.
    assert_eq!(
        unsafe {
            libc::openpty(
                &mut master,
                &mut slave,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        },
        0
    );
    // SAFETY: `openpty` returned two fresh descriptors that nothing else owns.
    let (mut master, slave) =
        unsafe { (fs::File::from_raw_fd(master), fs::File::from_raw_fd(slave)) };
    for fd in [master.as_raw_fd(), slave.as_raw_fd()] {
        // SAFETY: live descriptors. Neither end should leak across the exec.
        assert_ne!(
            unsafe { libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) },
            -1
        );
    }
    // The line discipline must turn the typed character into SIGINT, whatever
    // the defaults of this platform are.
    // SAFETY: a termios read from, and written back to, a live terminal.
    unsafe {
        let mut settings: libc::termios = std::mem::zeroed();
        assert_eq!(libc::tcgetattr(slave.as_raw_fd(), &mut settings), 0);
        settings.c_lflag |= libc::ISIG;
        settings.c_cc[libc::VINTR] = INTERRUPT;
        assert_eq!(
            libc::tcsetattr(slave.as_raw_fd(), libc::TCSANOW, &settings),
            0
        );
    }
    // A reader, as a terminal emulator is. On macOS a session leader's exit
    // first drains its terminal, and the echo of the typed character is output.
    let mut reader = master.try_clone().unwrap();
    let shown = Arc::new(Mutex::new(Vec::new()));
    let screen = Arc::clone(&shown);
    std::thread::spawn(move || {
        let mut buffer = [0; 1024];
        while let Ok(read @ 1..) = reader.read(&mut buffer) {
            screen.lock().unwrap().extend_from_slice(&buffer[..read]);
        }
    });

    let mut command = fixture.run_kind("release-notes");
    command
        .stdin(Stdio::from(slave.try_clone().unwrap()))
        .stdout(Stdio::from(slave.try_clone().unwrap()))
        .stderr(Stdio::from(slave));
    // SAFETY: only `setsid` and `ioctl` run between the fork and the exec.
    // Grove then leads a session whose controlling terminal is the pseudo-terminal,
    // as a shell starts a foreground job.
    unsafe {
        command.pre_exec(|| {
            if libc::setsid() == -1
                || libc::ioctl(libc::STDIN_FILENO, libc::TIOCSCTTY as _, 0) == -1
            {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    let mut grove = command.spawn().unwrap();
    drop(command);

    let text = wait_for_transcript(&fixture, "ready:");
    let harness: i32 = text
        .lines()
        .find_map(|line| line.strip_prefix("ready:"))
        .unwrap()
        .parse()
        .unwrap();
    let (group, foreground) = groups(grove.id());
    assert_eq!(group, grove.id(), "Grove leads its own group");
    assert_eq!(foreground, group, "the terminal stayed with Grove");

    master.write_all(&[INTERRUPT]).unwrap();
    let deadline = Instant::now() + Duration::from_secs(20);
    let status = loop {
        if let Some(status) = grove.try_wait().unwrap() {
            break status;
        }
        if Instant::now() >= deadline {
            // SAFETY: the session that this test started. Grove leads its group.
            unsafe { libc::kill(-i32::try_from(grove.id()).unwrap(), libc::SIGKILL) };
            let _ = grove.wait();
            panic!("Ctrl-C did not end grove run");
        }
        std::thread::sleep(Duration::from_millis(20));
    };
    let screen = String::from_utf8_lossy(&shown.lock().unwrap()).into_owned();
    assert!(!status.success(), "{status:?}: {screen}");
    assert!(
        screen.contains("standalone invocation was cancelled"),
        "the interrupt did not reach Grove: {screen}"
    );
    assert!(!fixture.notes().exists(), "{screen}");
    // SAFETY: signal 0 queries the reported harness PID.
    assert_eq!(unsafe { libc::kill(harness, 0) }, -1, "{screen}");
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
    fixture.configure(&Hooks {
        after_prompt: vec![Extra::Text(runtime.to_str().unwrap())],
        ..Hooks::default()
    });
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

/// A scripted agent that tries to read each native copy that its prompt
/// names, on a line `Native guidance: <path>`. It reports each read or failed
/// read on stderr, which goes to the transcript, and it writes a valid output
/// either way. When any copy is unreadable, the script runs the force-stop
/// command of its prompt. Otherwise it acknowledges.
///
/// The decision to stop is this script's, not a property of a real agent.
/// Grove's standalone prompt names no guidance and holds no missing-guidance
/// rule, so nothing here is evidence of what a real agent does.
const READS_NATIVE_COPIES: &str = "printf 'fake agent: started\\n' >&2\n\
     unreadable=\n\
     for file in $(printf '%s\\n' \"$1\" | sed -n 's/^Native guidance: //p'); do\n\
     if cat \"$file\" > /dev/null 2>&1; then printf 'fake agent: read %s\\n' \"$file\" >&2\n\
     else printf 'fake agent: cannot read guidance %s\\n' \"$file\" >&2; unreadable=yes; fi\n\
     done\n\
     printf valid > notes.md\n\
     if [ -n \"$unreadable\" ]; then\n\
     printf 'fake agent: a native copy is unreadable\\n' >&2\n\
     stop_work\n\
     fi\n\
     acknowledge\n";

impl Fixture {
    /// Native copies of the spine in the usual harness locations under HOME.
    /// The test process reads each one, so each copy exists and is readable
    /// outside the sandbox.
    fn native_copies(&self) -> Vec<PathBuf> {
        [".claude/skills", ".codex/skills", ".agents/skills"]
            .iter()
            .map(|tree| {
                let copy = self.home.join(tree).join("grove/SKILL.md");
                fs::create_dir_all(copy.parent().unwrap()).unwrap();
                fs::write(&copy, "# grove spine (native copy)\n").unwrap();
                assert_eq!(
                    fs::read_to_string(&copy).unwrap(),
                    "# grove spine (native copy)\n"
                );
                copy
            })
            .collect()
    }

    /// `grove run` with a prompt that names `native`, and `grants` as
    /// additional regular-file grants.
    fn run_guided(&self, native: &[PathBuf], grants: &[PathBuf]) -> Output {
        let mut prompt = String::from("Produce notes after you read the native copies.\n");
        for copy in native {
            prompt.push_str(&format!("Native guidance: {}\n", copy.display()));
        }
        let mut command = self.grove(Path::new(env!("CARGO_BIN_EXE_grove")));
        command
            .args(["run", "release-notes", &prompt, "--ui", "inline"])
            .arg("--output")
            .arg(self.notes())
            .arg("--runtime-read")
            .arg(&self.script);
        for grant in grants {
            command.arg("--runtime-read").arg(grant);
        }
        command.output().unwrap()
    }

    /// The text of every transcript, in no particular order.
    fn transcript_text(&self) -> String {
        let Ok(files) = fs::read_dir(self.home.join(".local/state/grove/runs")) else {
            return String::new();
        };
        files
            .flatten()
            .filter(|file| file.path().extension().is_some_and(|ext| ext == "log"))
            .map(|file| fs::read_to_string(file.path()).unwrap())
            .collect()
    }
}

/// The outcome of a force-stop after unreadable native copies. The one
/// confined agent fails to read each copy, Grove reports the force-stop, and
/// nothing is published. A second start of the agent, which an unconfined
/// retry would cause, fails the check.
fn force_stopped_after_unreadable_copies(
    fixture: &Fixture,
    output: &Output,
    native: &[PathBuf],
) -> Result<(), String> {
    let said = stderr(output);
    let transcript = fixture.transcript_text();
    if output.status.success() {
        return Err(format!("grove run succeeded: {said}"));
    }
    if !said.contains("Grove force-stop requested; outputs were not published") {
        return Err(format!("Grove did not report the force-stop: {said}"));
    }
    if fixture.notes().exists() {
        return Err("Grove published the output".into());
    }
    let starts = transcript.matches("fake agent: started").count();
    if starts != 1 {
        return Err(format!("the agent started {starts} times: {transcript}"));
    }
    for copy in native {
        let report = format!("fake agent: cannot read guidance {}", copy.display());
        if !transcript.contains(&report) {
            return Err(format!("no {report:?} in: {transcript}"));
        }
    }
    if transcript.contains("fake agent: read ")
        || !transcript.contains("fake agent: a native copy is unreadable")
    {
        return Err(format!(
            "the agent did not report the unreadable copy: {transcript}"
        ));
    }
    Ok(())
}

// This case proves two facts of Grove and its sandbox. With skill injection
// disabled, native skill trees under HOME lie outside every grant of the
// confined invocation, and Grove adds no grant for them. Grove interprets the
// exact force-stop command as failure: it publishes nothing, although the
// output is valid, and it does not launch the agent again. The case runs on
// each runtime: the 2.0 fixture HOME holds no `skills` settings.
//
// The scripted agent stops because its script says so. That a real agent
// stops is not proved here, and Grove does not promise it: a standalone
// prompt names no guidance. The record holds the real-agent result as an
// explicit gap.
//
// The control grants each native copy as a regular file. The agent then reads
// each copy and acknowledges, so the same check fails.
#[test]
fn native_copies_outside_every_grant_stay_unreadable_and_a_force_stop_publishes_nothing() {
    let fixture = Fixture::new(READS_NATIVE_COPIES);
    assert!(!fixture.dispatch_file("settings.json").exists());
    let native = fixture.native_copies();
    let output = fixture.run_guided(&native, &[]);
    force_stopped_after_unreadable_copies(&fixture, &output, &native).unwrap();

    let granted = Fixture::new(READS_NATIVE_COPIES);
    let native = granted.native_copies();
    let output = granted.run_guided(&native, &native);
    let refused = force_stopped_after_unreadable_copies(&granted, &output, &native).unwrap_err();
    assert!(refused.contains("grove run succeeded"), "{refused}");
    assert_published(&granted, &output, "valid");
}

/// Nested completion, force-stop or cancellation finishes only the inner
/// invocation. The real enclosing Ravel then acknowledges independently.
#[test]
fn nested_exit_and_cancellation_leave_the_enclosing_dispatched_run_intact() {
    for ending in ["continue", "force-stop", "cancel"] {
        let fixture = Fixture::new(match ending {
            "cancel" => "trap '' TERM INT\necho nested-ready\nwhile :; do sleep 0.05; done\n",
            "force-stop" => "set -eu\nprintf partial > notes.md\nstop_work\n",
            _ => WRITES_NOTES,
        });
        let outer = fixture.root.join("outer.sh");
        let grove = Path::new(env!("CARGO_BIN_EXE_grove"));
        let dispatch = support::installed_ravel().canonicalize().unwrap();
        let body = format!(
            "set -eu\nouter_exit=$RAVEL_EXIT_FILE\n\
             '{}' run release-notes 'Produce notes.' --ui inline --runtime-read '{}' --output '{}' &\ninner=$!\n{}\n\
             test ! -e \"$outer_exit\"\n\
             printf survived > '{}'\n'{}' end-session grove-continue\n",
            grove.display(), fixture.script.display(), fixture.notes().display(),
            if ending == "cancel" {
                format!("while ! grep -q nested-ready '{}'/runs/*.log 2>/dev/null; do sleep 0.02; done\nkill -TERM \"$inner\"\nif wait \"$inner\"; then exit 41; fi", fixture.home.join(".local/state/grove").display())
            } else if ending == "force-stop" {
                "if wait \"$inner\"; then exit 41; fi".to_owned()
            } else { "wait \"$inner\"".to_owned() },
            fixture.cwd.join("outer-survived").display(), dispatch.display()
        );
        fs::write(&outer, body).unwrap();
        fixture.configure(&Hooks {
            enclosing: Some(&outer),
            ..Hooks::default()
        });
        let output = fixture
            .grove(&dispatch)
            .args([
                "start-session",
                "--kind",
                "enclosing",
                "--prompt",
                "Enclose the invocation.",
            ])
            .output()
            .unwrap();
        assert!(output.status.success(), "{}", stderr(&output));
        assert_eq!(output.stdout, b"grove-continue");
        assert_eq!(
            fs::read_to_string(fixture.cwd.join("outer-survived")).unwrap(),
            "survived"
        );
        assert_eq!(fixture.notes().exists(), ending == "continue");
    }
}

#[test]
fn acknowledgement_names_the_canonical_dispatch_path_when_the_sibling_is_a_symlink() {
    let fixture = Fixture::new(
        "printf '%s\\n%s' \"$dispatch_exit\" \"$force_stop\" > notes.md\nacknowledge\n",
    );
    let bin = fixture.root.join("aliased-bin");
    fs::create_dir(&bin).unwrap();
    fs::copy(env!("CARGO_BIN_EXE_grove"), bin.join("grove")).unwrap();
    let dispatch = support::installed_ravel().canonicalize().unwrap();
    std::os::unix::fs::symlink(&dispatch, bin.join("ravel")).unwrap();
    let output = fixture
        .grove(&bin.join("grove"))
        .args(["run", "release-notes", "Produce notes.", "--ui", "inline"])
        .arg("--runtime-read")
        .arg(&fixture.script)
        .arg("--output")
        .arg(fixture.notes())
        .output()
        .unwrap();
    assert_published(
        &fixture,
        &output,
        &format!(
            "'{}' end-session grove-continue\n'{}' end-session grove-force-stop",
            dispatch.display(),
            dispatch.display()
        ),
    );
}

impl Fixture {
    /// A bare `grove run`, with no kind, from `directory`. The prompt comes
    /// from a file, because one positional argument is the kind.
    fn run_bare(&self, directory: &Path) -> Output {
        let prompt = self.root.join("prompt.md");
        fs::write(&prompt, "Produce notes.\n").unwrap();
        let mut command = self.grove(Path::new(env!("CARGO_BIN_EXE_grove")));
        command
            .current_dir(directory)
            .arg("run")
            .arg("--prompt-file")
            .arg(&prompt)
            .args(["--ui", "inline", "--output"])
            .arg(self.notes())
            .arg("--runtime-read")
            .arg(&self.script);
        command.output().unwrap()
    }

    /// A grove at `worktree`, which holds `.jj/`, with these leaf files.
    fn grove_tree(&self, worktree: &Path, leaves: &[&str]) {
        fs::create_dir_all(worktree.join(".jj")).unwrap();
        fs::create_dir_all(worktree.join(".grove")).unwrap();
        fs::write(worktree.join(".grove/_BRIEF.md"), "# brief\n").unwrap();
        for leaf in leaves {
            let handle = leaf.rsplit("--").next().unwrap().trim_end_matches(".md");
            fs::write(worktree.join(".grove").join(leaf), format!("# {handle}\n")).unwrap();
        }
    }
}

/// The fixture policy refuses each kind except `release-notes`, and names the
/// refused kind. The refusal therefore shows which kind `grove run` routed.
fn assert_routed(fixture: &Fixture, output: &Output, kind: &str) {
    assert_nothing_published(fixture, output);
    let said = stderr(output);
    assert!(said.contains(&format!("no command for {kind}")), "{said}");
}

// Outside an active grove, a bare run routes `standalone`. A `.grove/` with
// no `.jj/` above it, and a `.jj/` with no `.grove/`, are both outside one.
#[test]
fn a_bare_run_outside_an_active_grove_routes_standalone() {
    let fixture = Fixture::new(WRITES_NOTES);
    assert!(
        !fixture.cwd.ancestors().any(|dir| dir.join(".jj").exists()),
        "the fixture lies inside a jj workspace"
    );
    assert_routed(&fixture, &fixture.run_bare(&fixture.cwd), "standalone");

    let stray = fixture.cwd.join("stray");
    fs::create_dir_all(stray.join(".grove")).unwrap();
    assert_routed(&fixture, &fixture.run_bare(&stray), "standalone");

    let workspace = fixture.cwd.join("workspace");
    fs::create_dir_all(workspace.join(".jj")).unwrap();
    assert_routed(&fixture, &fixture.run_bare(&workspace), "standalone");
}

// Inside an active grove, a bare run routes the next live leaf's kind, from
// any directory of the workspace. An explicit kind keeps its route there.
#[test]
fn a_bare_run_inside_an_active_grove_routes_the_next_live_leaf_kind() {
    let fixture = Fixture::new(WRITES_NOTES);
    let worktree = fixture.cwd.join("worktree");
    fixture.grove_tree(
        &worktree,
        &["01-DONE-impl--first-k1.md", "02-draft--second-k2.md"],
    );
    let deep = worktree.join("a/b");
    fs::create_dir_all(&deep).unwrap();
    assert_routed(&fixture, &fixture.run_bare(&deep), "draft");

    let output = fixture
        .run_kind("release-notes")
        .current_dir(&deep)
        .output()
        .unwrap();
    assert_published(&fixture, &output, "ok");
}

// An active grove with no live leaf gives a bare run no kind. It refuses
// before selection and names the remedy, and nothing launches.
#[test]
fn a_bare_run_in_a_grove_without_a_live_leaf_refuses_before_launch() {
    let fixture = Fixture::new(WRITES_NOTES);
    let worktree = fixture.cwd.join("worktree");
    fixture.grove_tree(&worktree, &["01-DONE-impl--first-k1.md"]);
    let output = fixture.run_bare(&worktree);
    assert_nothing_published(&fixture, &output);
    let said = stderr(&output);
    assert!(said.contains("has no live leaf"), "{said}");
    assert!(said.contains("give an explicit kind"), "{said}");
    assert!(!said.contains("no command for"), "{said}");
    assert_eq!(fixture.transcripts(), 0, "{said}");
}
