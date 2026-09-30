//! The command seam: the real front executable, run as a subprocess against
//! temporary policies and a fake harness, with no Grove binary, configuration or
//! task tree.
//!
//! Every invocation starts from an empty environment plus a private HOME and a
//! minimal PATH, in a private working directory, so nothing from the developer's
//! own configuration or a live Grove session can take part. The compiled worker
//! must already be built (`task dispatch:worker`); if it is absent or stale the
//! front refuses with exit 5 and every test that expects a selection fails. No
//! test is skipped for want of it.

#![allow(dead_code)] // each test binary uses its own subset

use std::fs;
use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use serde_json::Value;
use tempfile::TempDir;

pub const FRONT: &str = env!("CARGO_BIN_EXE_harness-dispatch");

/// A one-candidate policy that routes `impl`, the shape most tests start from.
pub const ROUTED: &str = r#"export const policy = {
  schemaVersion: 1,
  version: "seam-1",
  catalog: [
    { id: "deep", provider: "origin-a", model: "model-large", effort: "high", program: "fake-harness", args: [{ slot: "prompt" }] },
  ],
  routes: { impl: "deep" },
};
"#;

/// The fake harness every sandbox puts on PATH. It records how it was started
/// under `$FAKE_HARNESS_RECORD` (its arguments NUL-separated, its physical cwd
/// and its PID), copies stdin to stdout, and then behaves as the environment
/// tells it: it writes to descriptor 7, kills itself with a signal, or exits
/// with a code. The record directory is created exclusively, so its presence
/// is the marker that the harness ran, and a second start fails loudly.
pub const FAKE_HARNESS: &str = r#"#!/bin/sh
record=$FAKE_HARNESS_RECORD
mkdir "$record" || exit 90
for argument in "$@"; do printf '%s\0' "$argument"; done > "$record/args"
pwd -P > "$record/cwd"
echo $$ > "$record/pid"
cat
if [ -n "$FAKE_HARNESS_FD7" ]; then echo "$FAKE_HARNESS_FD7" >&7; fi
if [ -n "$FAKE_HARNESS_SIGNAL" ]; then kill -s "$FAKE_HARNESS_SIGNAL" $$; fi
exit "${FAKE_HARNESS_EXIT:-0}"
"#;

pub struct Sandbox {
    _dir: TempDir,
    pub root: PathBuf,
    pub home: PathBuf,
    pub cwd: PathBuf,
    pub tmp: PathBuf,
    /// First on PATH, holding `fake-harness`.
    pub bin: PathBuf,
    /// Where the fake harness records a start; absent until it runs.
    pub record: PathBuf,
}

impl Sandbox {
    pub fn new() -> Self {
        let dir = tempfile::tempdir().expect("temporary directory");
        // Canonical, so paths the front reports (it canonicalizes) compare
        // equal on macOS, where the temporary directory sits behind a symlink.
        let root = fs::canonicalize(dir.path()).expect("canonical temporary directory");
        let (home, cwd, tmp, bin) = (
            root.join("home"),
            root.join("work"),
            root.join("tmp"),
            root.join("bin"),
        );
        for dir in [&home, &cwd, &tmp, &bin] {
            fs::create_dir(dir).expect("sandbox directory");
        }
        executable(&bin.join("fake-harness"), FAKE_HARNESS);
        let record = root.join("harness-record");
        Sandbox {
            _dir: dir,
            root,
            home,
            cwd,
            tmp,
            bin,
            record,
        }
    }

    /// Whether the fake harness started.
    pub fn harness_ran(&self) -> bool {
        self.record.exists()
    }

    /// The arguments the fake harness received after its argv[0], exactly.
    pub fn harness_args(&self) -> Vec<String> {
        let bytes = fs::read(self.record.join("args")).expect("the fake harness ran");
        let text = String::from_utf8(bytes).expect("UTF-8 arguments");
        let mut args: Vec<String> = text.split('\0').map(str::to_owned).collect();
        assert_eq!(args.pop().as_deref(), Some(""), "arguments end with a NUL");
        args
    }

    /// The fake harness's physical working directory.
    pub fn harness_cwd(&self) -> PathBuf {
        let cwd = fs::read_to_string(self.record.join("cwd")).expect("the fake harness ran");
        PathBuf::from(cwd.strip_suffix('\n').expect("pwd ends its line"))
    }

    pub fn harness_pid(&self) -> u32 {
        let pid = fs::read_to_string(self.record.join("pid")).expect("the fake harness ran");
        pid.trim().parse().expect("a PID")
    }

    pub fn personal_path(&self) -> PathBuf {
        self.home.join(".config/harness-dispatch/policy.ts")
    }

    pub fn personal_policy(&self, source: &str) -> PathBuf {
        let path = self.personal_path();
        write(&path, source);
        path
    }

    /// Write a file relative to the sandbox's working directory.
    pub fn file(&self, relative: &str, contents: &str) -> PathBuf {
        let path = self.cwd.join(relative);
        write(&path, contents);
        path
    }

    /// The front with an empty environment plus HOME, PATH, TMPDIR and the
    /// fake harness's record location.
    pub fn command(&self) -> Command {
        self.command_for(Path::new(FRONT))
    }

    /// The same, for a front executable at another path (a copy or symlink).
    pub fn command_for(&self, front: &Path) -> Command {
        let mut command = Command::new(front);
        command
            .env_clear()
            .env("HOME", &self.home)
            .env("PATH", format!("{}:/usr/bin:/bin", text(&self.bin)))
            .env("TMPDIR", &self.tmp)
            .env("FAKE_HARNESS_RECORD", &self.record)
            .current_dir(&self.cwd)
            .stdin(Stdio::null());
        command
    }

    pub fn inspect(&self, args: &[&str]) -> Run {
        let mut command = self.command();
        command.arg("inspect").args(args);
        run(&mut command)
    }

    pub fn run(&self, args: &[&str]) -> Run {
        let mut command = self.command();
        command.arg("run").args(args);
        run(&mut command)
    }
}

pub fn write(path: &Path, contents: &str) {
    fs::create_dir_all(path.parent().expect("a parent")).expect("parent directory");
    fs::write(path, contents).expect("write file");
}

pub fn executable(path: &Path, script: &str) {
    write(path, script);
    fs::set_permissions(path, fs::Permissions::from_mode(0o755)).expect("chmod");
}

pub struct Run {
    pub code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

pub fn run(command: &mut Command) -> Run {
    Run::from(command.output().expect("the front executable runs"))
}

impl From<Output> for Run {
    fn from(output: Output) -> Self {
        Run {
            code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        }
    }
}

impl Run {
    /// The whole of stdout as one JSON document, after asserting success.
    pub fn report(&self) -> Value {
        assert_eq!(
            self.code,
            Some(0),
            "expected success\nstdout: {}\nstderr: {}",
            self.stdout,
            self.stderr
        );
        serde_json::from_str(&self.stdout).unwrap_or_else(|error| {
            panic!("stdout is not one JSON document ({error}): {}", self.stdout)
        })
    }

    /// The JSON refusal on stderr, after asserting the exit result and that
    /// stdout carries no partial object.
    pub fn refusal(&self, exit: i32) -> Value {
        assert_eq!(
            self.code,
            Some(exit),
            "expected exit {exit}\nstdout: {}\nstderr: {}",
            self.stdout,
            self.stderr
        );
        assert_eq!(self.stdout, "", "a refusal prints nothing on stdout");
        let document: Value = serde_json::from_str(&self.stderr).unwrap_or_else(|error| {
            panic!("stderr is not one JSON document ({error}): {}", self.stderr)
        });
        assert_eq!(document["schemaVersion"], 1);
        assert_eq!(document["error"]["exit"], exit);
        for field in ["code", "stage", "message", "remedy"] {
            assert!(
                document["error"][field]
                    .as_str()
                    .is_some_and(|text| !text.is_empty()),
                "refusal lacks {field}: {document}"
            );
        }
        document
    }
}

pub fn text(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}
