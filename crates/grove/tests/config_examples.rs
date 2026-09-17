//! Execute the repository examples themselves: neither rewritten executable
//! names nor a second handwritten configuration can establish their contract.
#![cfg(unix)]

use std::ffi::OsString;
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use grove_loop::session_config::{DeltaRoots, ExpansionContext, SessionConfig};
use serde_json::{json, Value};

mod support;

const PROMPT: &str = "A fixed prompt\nwith spaces, 'quotes', ${param.effort}; $(touch injected)";

fn example(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/examples/modular-configuration")
        .join(name)
}

struct Fixture {
    _dir: tempfile::TempDir,
    repo: PathBuf,
    home: PathBuf,
    bin: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().canonicalize().unwrap();
        let repo = root.join("repo");
        let home = root.join("home");
        let bin = root.join("bin");
        fs::create_dir_all(&repo).unwrap();
        fs::create_dir_all(home.join(".config/grove")).unwrap();
        fs::create_dir(&bin).unwrap();
        support::init_jj_repo(&repo);
        fs::write(repo.join(".gitignore"), "/.grove.kdl\n/captured.argv\n").unwrap();
        let installation = Command::new(env!("CARGO_BIN_EXE_grove"))
            .args(["config", "examples"])
            .current_dir(&home)
            .env("HOME", &home)
            .output()
            .unwrap();
        assert_success(&installation);
        fs::copy(
            home.join(".config/grove/config.modular.example.kdl"),
            home.join(".config/grove/config.kdl"),
        )
        .unwrap();
        for executable in ["my-codex-policy", "my-claude-policy"] {
            let path = bin.join(executable);
            fs::write(
                &path,
                "#!/bin/sh\nprintf '%s\\0' \"${0##*/}\" \"$@\" > \"$CAPTURE_ARGV\"\n",
            )
            .unwrap();
            fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
        }
        Self {
            _dir: dir,
            repo,
            home,
            bin,
        }
    }

    fn local_example(&self, name: &str) {
        let delivered = self.home.join(".config/grove").join(name);
        assert_eq!(
            fs::read(&delivered).unwrap(),
            fs::read(example(name)).unwrap()
        );
        fs::copy(delivered, self.repo.join(".grove.kdl")).unwrap();
    }

    fn load(&self) -> SessionConfig {
        SessionConfig::load(
            &self.home,
            &DeltaRoots {
                worktree: &self.repo,
                repository: &self.repo,
            },
        )
        .unwrap()
    }

    fn command(&self) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_grove"));
        for name in support::grove_env_names() {
            command.env_remove(name);
        }
        command
            .current_dir(&self.repo)
            .env("HOME", &self.home)
            .env("PATH", self.path())
            .env("CAPTURE_ARGV", self.repo.join("captured.argv"));
        command
    }

    fn path(&self) -> OsString {
        let mut paths = vec![self.bin.clone()];
        paths.extend(std::env::split_paths(&std::env::var_os("PATH").unwrap()));
        std::env::join_paths(paths).unwrap()
    }

    fn inspection(&self) -> Value {
        let output = self
            .command()
            .args(["config", "show", "--json"])
            .output()
            .unwrap();
        assert_success(&output);
        serde_json::from_slice(&output.stdout).unwrap()
    }

    fn check_launch(&self, config: &SessionConfig, report: &Value, kind: &str, expected: &[&str]) {
        let context = ExpansionContext {
            prompt: PROMPT,
            session_name: "example-session",
            worktree: &self.repo,
            repository: &self.repo,
        };
        let argv = config.expand(kind, &context).unwrap();
        let expected: Vec<OsString> = expected.iter().map(OsString::from).collect();
        assert_eq!(argv.words(), expected, "{kind}");
        let record = report["commands"]
            .as_array()
            .unwrap()
            .iter()
            .find(|record| record["key"] == kind)
            .unwrap();
        let filled: Vec<OsString> = record["words"]
            .as_array()
            .unwrap()
            .iter()
            .map(|record| {
                let word = &record["word"];
                match word["type"].as_str().unwrap() {
                    "literal" => word["value"].as_str().unwrap().into(),
                    "slot" => {
                        assert_eq!(word["name"], "prompt");
                        PROMPT.into()
                    }
                    other => panic!("unexpected compiled word {other}"),
                }
            })
            .collect();
        assert_eq!(filled, expected, "inspection for {kind}");
        let capture = self.repo.join("captured.argv");
        if capture.exists() {
            fs::remove_file(&capture).unwrap();
        }
        let output = Command::new(argv.program())
            .args(argv.args())
            .current_dir(&self.repo)
            .env("PATH", self.path())
            .env("CAPTURE_ARGV", &capture)
            .output()
            .unwrap();
        assert_success(&output);
        let expected_bytes: Vec<u8> = expected
            .iter()
            .flat_map(|word| word.as_bytes().iter().copied().chain([0]))
            .collect();
        assert_eq!(
            fs::read(capture).unwrap(),
            expected_bytes,
            "child argv for {kind}"
        );
        assert!(!self.repo.join("injected").exists());
    }
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

const CODEX_MEDIUM: &[&str] = &[
    "my-codex-policy",
    "--model",
    "your-codex-model",
    "-c",
    "model_reasoning_effort=medium",
    PROMPT,
];
const CODEX_HIGH: &[&str] = &[
    "my-codex-policy",
    "--model",
    "your-codex-model",
    "-c",
    "model_reasoning_effort=high",
    PROMPT,
];
const CODEX_LOW: &[&str] = &[
    "my-codex-policy",
    "--model",
    "your-codex-model",
    "-c",
    "model_reasoning_effort=low",
    PROMPT,
];
const CLAUDE_MEDIUM: &[&str] = &[
    "my-claude-policy",
    "--model",
    "your-claude-model",
    "--effort",
    "medium",
    PROMPT,
];
const CLAUDE_HIGH: &[&str] = &[
    "my-claude-policy",
    "--model",
    "your-claude-model",
    "--effort",
    "high",
    PROMPT,
];

#[test]
fn repository_examples_resolve_inspect_and_launch_the_promised_argv() {
    // Wrong profile ordering, leaked global defaults, lost specificity or a
    // changed example template must fail against these independent expectations.
    let cases = [
        (
            None,
            vec!["daily"],
            vec!["daily", "routes", "codex-led"],
            CODEX_MEDIUM,
            CLAUDE_MEDIUM,
            CLAUDE_HIGH,
        ),
        (
            Some("grove.codex-led.example.kdl"),
            vec!["routes", "codex-led"],
            vec!["routes", "codex-led"],
            CODEX_MEDIUM,
            CLAUDE_MEDIUM,
            CLAUDE_HIGH,
        ),
        (
            Some("grove.claude-led.example.kdl"),
            vec!["routes", "claude-led"],
            vec!["routes", "claude-led"],
            CLAUDE_MEDIUM,
            CODEX_MEDIUM,
            CODEX_HIGH,
        ),
        (
            Some("grove.high-effort.example.kdl"),
            vec!["daily", "high-effort"],
            vec!["daily", "routes", "codex-led", "high-effort"],
            CODEX_HIGH,
            CLAUDE_HIGH,
            CLAUDE_HIGH,
        ),
        (
            Some("grove.local-override.example.kdl"),
            vec!["daily"],
            vec!["daily", "routes", "codex-led"],
            CODEX_LOW,
            CLAUDE_MEDIUM,
            CLAUDE_MEDIUM,
        ),
    ];
    for (local, selection, occurrences, implementation, review, proof) in cases {
        let fixture = Fixture::new();
        if let Some(local) = local {
            fixture.local_example(local);
        }
        let personal_before = fs::read(fixture.home.join(".config/grove/config.kdl")).unwrap();
        let local_before = fs::read(fixture.repo.join(".grove.kdl")).ok();
        let config = fixture.load();
        let report = fixture.inspection();
        assert_eq!(
            report["selection"]["profiles"],
            json!(selection),
            "{local:?}"
        );
        let actual_occurrences: Vec<_> = report["profile_occurrences"]
            .as_array()
            .unwrap()
            .iter()
            .map(|o| o["profile"].as_str().unwrap())
            .collect();
        assert_eq!(actual_occurrences, occurrences, "{local:?}");
        for (kind, expected) in [
            ("impl", implementation),
            ("review-impl", review),
            ("proof", proof),
        ] {
            fixture.check_launch(&config, &report, kind, expected);
        }
        assert_eq!(
            fs::read(fixture.home.join(".config/grove/config.kdl")).unwrap(),
            personal_before
        );
        assert_eq!(fs::read(fixture.repo.join(".grove.kdl")).ok(), local_before);
    }
}

#[test]
fn removing_the_effort_experiment_or_unset_restores_the_documented_values() {
    let fixture = Fixture::new();
    fixture.local_example("grove.high-effort.example.kdl");
    fixture.check_launch(&fixture.load(), &fixture.inspection(), "impl", CODEX_HIGH);
    fs::remove_file(fixture.repo.join(".grove.kdl")).unwrap();
    let config = fixture.load();
    let report = fixture.inspection();
    fixture.check_launch(&config, &report, "impl", CODEX_MEDIUM);
    fixture.check_launch(&config, &report, "review-impl", CLAUDE_MEDIUM);

    fixture.local_example("grove.local-override.example.kdl");
    fixture.check_launch(
        &fixture.load(),
        &fixture.inspection(),
        "proof",
        CLAUDE_MEDIUM,
    );
    let local = fs::read_to_string(fixture.repo.join(".grove.kdl")).unwrap();
    let without_unset = local.replace("unset \"effort\";", "");
    assert_ne!(local, without_unset);
    fs::write(fixture.repo.join(".grove.kdl"), without_unset).unwrap();
    let config = fixture.load();
    let report = fixture.inspection();
    fixture.check_launch(&config, &report, "proof", CLAUDE_HIGH);
    fixture.check_launch(&config, &report, "impl", CODEX_LOW);
}

#[test]
fn selecting_the_unfinished_example_fails_before_driver_launch_or_tree_mutation() {
    let fixture = Fixture::new();
    // Establish the inactive case before changing only the workspace selection.
    fixture.load().require("impl").unwrap();
    fs::write(
        fixture.repo.join(".grove.kdl"),
        "config { select \"daily\" \"unfinished\"; }\n",
    )
    .unwrap();
    let personal_before = fs::read(fixture.home.join(".config/grove/config.kdl")).unwrap();
    let local_before = fs::read(fixture.repo.join(".grove.kdl")).unwrap();
    let output = fixture
        .command()
        .args(["config", "show", "--json"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let report: Value = serde_json::from_slice(&output.stderr).unwrap();
    let diagnostic = report["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .find(|d| d["command"] == "not-written-yet")
        .unwrap();
    assert_eq!(diagnostic["binding"], "lead");
    assert_eq!(diagnostic["source"]["role"], "primary");
    assert!(diagnostic["primary"].is_object());
    assert!(!diagnostic["remedy"].as_str().unwrap().is_empty());
    assert!(diagnostic["occurrence_chain"]
        .as_array()
        .unwrap()
        .iter()
        .any(|o| o["profile"] == "unfinished"));

    // Exercise the actual human driver, both before bootstrap and with work.
    for existing_tree in [false, true] {
        let tree = fixture.repo.join(".grove");
        if existing_tree {
            fs::create_dir(&tree).unwrap();
            fs::write(tree.join("_BRIEF.md"), "# Example work\n").unwrap();
            fs::write(tree.join("01-impl--example-k1.md"), "# example-k1\n").unwrap();
        }
        let output = fixture.command().output().unwrap();
        assert_eq!(output.status.code(), Some(1));
        let error = String::from_utf8(output.stderr).unwrap();
        assert!(error.contains("not-written-yet"), "{error}");
        assert!(error.contains("config.kdl"), "{error}");
        assert!(!fixture.repo.join("captured.argv").exists());
        if existing_tree {
            assert_eq!(
                fs::read_to_string(tree.join("_BRIEF.md")).unwrap(),
                "# Example work\n"
            );
            assert_eq!(
                fs::read_to_string(tree.join("01-impl--example-k1.md")).unwrap(),
                "# example-k1\n"
            );
            assert_eq!(fs::read_dir(tree).unwrap().count(), 2);
        } else {
            assert!(!tree.exists());
        }
    }
    assert_eq!(
        fs::read(fixture.home.join(".config/grove/config.kdl")).unwrap(),
        personal_before
    );
    assert_eq!(
        fs::read(fixture.repo.join(".grove.kdl")).unwrap(),
        local_before
    );
}
