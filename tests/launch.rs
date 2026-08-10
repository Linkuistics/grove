//! The removed launch-policy surface, asserted as removed.
//!
//! Harness detection, the harness stamp, the `GROVE_<KIND|FAMILY>_HARNESS` /
//! `GROVE_*_MODEL` routing lattice, the executable / skill-dir / kill-grace /
//! `GROVE_LLM_BIN` overrides, the codex sandbox probe and its `--add-dir`
//! grants, hidden session-name and model arguments, and the `do` / `migrate` /
//! `retire` / `--harness` / `--no-launch` command surface are all gone.
//!
//! Removal is a claim about behaviour, not about the absence of a symbol, so
//! these tests drive the **real bare `grove` process** with the whole legacy
//! surface set to values that would be catastrophic if they were still read,
//! and check that the launch is byte-identical to one run without them. The
//! control in the other direction — that this fixture can detect a changed
//! launch at all — is asserted alongside, by changing the one input that *is*
//! still supposed to steer it: the configuration.

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::{Command, Output};

use tempfile::TempDir;

const SESSION_KINDS: &[&str] = &[
    "requirements",
    "review-requirements",
    "integrate-review-requirements",
    "design",
    "review-design",
    "integrate-review-design",
    "planning",
    "review-planning",
    "integrate-review-planning",
    "prototype",
    "review-prototype",
    "integrate-review-prototype",
    "impl",
    "review-impl",
    "integrate-review-impl",
    "research-a",
    "research-b",
    "combine-research",
    "finish",
];

/// Every launch-policy variable Grove used to read, with a value chosen so that
/// *reading* it would be visible: a harness that is not the configured command,
/// a model that would appear in argv, binaries that cannot run, a skill dir that
/// does not exist, and graces that would change the watcher's timing.
///
/// The `review-impl` spellings are included deliberately — the family axis was
/// the one an operator set once and never looked at again, so it is the one
/// whose silent survival would go unnoticed longest.
const REMOVED_ROUTING_ENV: &[(&str, &str)] = &[
    ("GROVE_IMPL_HARNESS", "codex"),
    ("GROVE_REVIEW_HARNESS", "pi"),
    ("GROVE_REVIEW_IMPL_HARNESS", "pi"),
    ("GROVE_INTEGRATE_REVIEW_HARNESS", "codex"),
    ("GROVE_IMPL_MODEL", "routed-model"),
    ("GROVE_REVIEW_MODEL", "routed-model"),
    ("GROVE_CLAUDE_IMPL_MODEL", "routed-model"),
    ("GROVE_CODEX_REVIEW_IMPL_MODEL", "routed-profile"),
    ("GROVE_REQUIREMENTS_MODEL", "routed-model"),
    ("GROVE_HARNESS_BIN", "/definitely/not/a/harness"),
    ("GROVE_HARNESS_BIN_CLAUDE", "/definitely/not/a/harness"),
    ("GROVE_HARNESS_BIN_CODEX", "/definitely/not/a/harness"),
    ("GROVE_LLM_BIN", "/definitely/not/an/agent/cli"),
    ("GROVE_SKILL_DIR", "/definitely/not/a/skill/dir"),
    ("GROVE_KILL_GRACE", "900"),
    ("GROVE_KILL_GRACE_KILL", "900"),
];

fn init_git_worktree(path: &Path) {
    fs::create_dir_all(path).unwrap();
    assert!(Command::new("git")
        .args(["init", "-q"])
        .current_dir(path)
        .status()
        .unwrap()
        .success());
}

fn write_executable(path: &Path, body: &str) {
    fs::write(path, body).unwrap();
    let mut permissions = fs::metadata(path).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).unwrap();
}

fn shell_quote(path: &Path) -> String {
    let value = path.to_str().unwrap();
    assert!(!value.contains('\''), "test fixture path contains a quote");
    format!("'{value}'")
}

fn write_complete_config(home: &Path, template: &str) {
    let config_dir = home.join(".config/grove");
    fs::create_dir_all(&config_dir).unwrap();
    let document = SESSION_KINDS
        .iter()
        .map(|kind| format!("{kind} {template:?}\n"))
        .collect::<String>();
    fs::write(config_dir.join("config.kdl"), document).unwrap();
}

/// One fixture: a git worktree holding a single live `impl` leaf, an isolated
/// home carrying a complete config, and a fake command that records `$0` plus
/// its whole argv and then signals completion so the loop ends.
struct Fixture {
    root: TempDir,
    home: std::path::PathBuf,
    worktree: std::path::PathBuf,
    argv_log: std::path::PathBuf,
}

impl Fixture {
    fn new(command_name: &str) -> Self {
        let root = TempDir::new().unwrap();
        let home = root.path().join("home");
        fs::create_dir_all(home.join(".codex")).unwrap();
        let worktree = root.path().join("worktree");
        init_git_worktree(&worktree);
        let grove = worktree.join(".grove");
        fs::create_dir_all(&grove).unwrap();
        fs::write(grove.join("FORMAT"), "session-kinds-v1\n").unwrap();
        fs::write(grove.join("BRIEF.md"), "# removed-surface — brief\n").unwrap();
        fs::write(grove.join("01-impl-subject-k1.md"), "# subject-k1\n").unwrap();

        let argv_log = root.path().join("argv-log");
        let configured = root.path().join(command_name);
        // The mandate prompt is multi-line, so newlines are folded to spaces:
        // one launch must be one comparable line.
        write_executable(
            &configured,
            &format!(
                "#!/bin/sh\n{{ printf '%s|%s' \"$0\" \"$*\" | tr '\\n' ' '; printf '\\n'; }} >> {}\nexit 0\n",
                shell_quote(&argv_log)
            ),
        );
        write_complete_config(
            &home,
            &format!(
                "{} --configured-flag '${{prompt}}'",
                shell_quote(&configured)
            ),
        );

        Self {
            root,
            home,
            worktree,
            argv_log,
        }
    }

    fn run(&self, extra_env: &[(&str, &str)]) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_grove"));
        command.current_dir(&self.worktree).env("HOME", &self.home);
        // A subprocess inherits this process's *whole* ambient environment, and
        // this repo dogfoods Grove — so scrub the legacy surface first and let
        // each case put back exactly what it means to test.
        for (name, _) in REMOVED_ROUTING_ENV {
            command.env_remove(name);
        }
        for (name, value) in extra_env {
            command.env(name, value);
        }
        command.output().unwrap()
    }

    /// The `$0|argv` lines the fake recorded, one per launch. Runs share one
    /// fixture — the same temp paths, the same leaf, the same handle — so two
    /// lines from the same log compare directly, with nothing to normalise.
    fn launch_lines(&self) -> Vec<String> {
        fs::read_to_string(&self.argv_log)
            .unwrap_or_default()
            .lines()
            .map(str::to_string)
            .collect()
    }
}

// The whole legacy launch-policy surface, set at once, must change nothing.
// Every value here is one a *live* reader would act on visibly — reroute to a
// harness the config never names, append a `--model`/`--profile` argument, exec
// a binary that does not exist — so a single surviving reader fails this
// loudly rather than subtly.
#[test]
fn a_configured_launch_ignores_every_removed_routing_variable() {
    let fixture = Fixture::new("configured-command.sh");

    let control = fixture.run(&[]);
    assert!(
        control.status.success(),
        "control run failed: {}",
        String::from_utf8_lossy(&control.stderr)
    );

    let routed = fixture.run(REMOVED_ROUTING_ENV);
    assert!(
        routed.status.success(),
        "a launch under the legacy routing surface must be unaffected, not merely \
         different: {}",
        String::from_utf8_lossy(&routed.stderr)
    );

    let lines = fixture.launch_lines();
    assert_eq!(
        lines.len(),
        2,
        "both runs must reach the configured command"
    );
    assert!(
        lines[0].contains("--configured-flag"),
        "the control run must reach the configured command: {:?}",
        lines[0]
    );
    assert_eq!(
        lines[0], lines[1],
        "the configured argv must be identical with and without the legacy \
         routing environment"
    );

    // Belt and braces on the equality: the tokens a *live* reader would have
    // introduced, named individually, so a failure says which axis survived
    // rather than only that something differs.
    for token in [
        "--model",
        "--profile",
        "--add-dir",
        "routed-model",
        "routed-profile",
    ] {
        assert!(
            !lines[1].contains(token),
            "{token:?} reached the configured argv: {:?}",
            lines[1]
        );
    }
    let stderr = String::from_utf8_lossy(&routed.stderr);
    assert!(
        !stderr.contains("sandbox"),
        "no sandbox pre-flight survives: {stderr}"
    );
}

// The positive control for the test above: the same fixture *does* observe a
// changed launch, so "identical argv" is a finding rather than an artefact of
// nothing being watched. The one input still allowed to steer a launch is the
// configuration, and changing it changes the recorded argv.
#[test]
fn the_same_fixture_still_observes_a_configuration_driven_change() {
    let fixture = Fixture::new("configured-command.sh");
    assert!(fixture.run(&[]).status.success());

    write_complete_config(
        &fixture.home,
        &format!(
            "{} --a-different-flag '${{prompt}}'",
            shell_quote(&fixture.root.path().join("configured-command.sh"))
        ),
    );
    assert!(fixture.run(&[]).status.success());

    let lines = fixture.launch_lines();
    assert_eq!(lines.len(), 2);
    assert!(
        lines[1].contains("--a-different-flag"),
        "the configuration must still steer the launch: {:?}",
        lines[1]
    );
    assert_ne!(
        lines[0], lines[1],
        "the fixture must be able to observe a changed launch"
    );
}

// The removed human verbs and flags stay removed. `retire` is the one a user is
// likeliest to reach for by muscle memory, and `--harness` / `--no-launch` are
// the two flags whose survival would mean launch policy had a command-line home
// again.
#[test]
fn the_bare_human_cli_accepts_no_launch_verbs_or_flags() {
    for arguments in [
        vec!["retire", "--help"],
        vec!["do"],
        vec!["migrate"],
        vec!["--harness", "claude"],
        vec!["--no-launch"],
    ] {
        let out = Command::new(env!("CARGO_BIN_EXE_grove"))
            .args(&arguments)
            .output()
            .unwrap();
        assert!(
            !out.status.success(),
            "the bare command must reject {arguments:?}: {out:?}"
        );
    }
}
