//! The command seam: the real front executable, run as a subprocess against
//! temporary policies, with no Grove binary, configuration or task tree.
//!
//! Every invocation starts from an empty environment plus a private HOME and a
//! minimal PATH, in a private working directory, so nothing from the developer's
//! own configuration or a live Grove session can take part. The compiled worker
//! must already be built (`task dispatch:worker`); if it is absent or stale the
//! front refuses with exit 5 and every test that expects a selection fails. No
//! test is skipped for want of it.

#![allow(dead_code)] // each test binary uses its own subset

use std::fs;
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

pub struct Sandbox {
    _dir: TempDir,
    pub root: PathBuf,
    pub home: PathBuf,
    pub cwd: PathBuf,
    pub tmp: PathBuf,
}

impl Sandbox {
    pub fn new() -> Self {
        let dir = tempfile::tempdir().expect("temporary directory");
        // Canonical, so paths the front reports (it canonicalizes) compare
        // equal on macOS, where the temporary directory sits behind a symlink.
        let root = fs::canonicalize(dir.path()).expect("canonical temporary directory");
        let (home, cwd, tmp) = (root.join("home"), root.join("work"), root.join("tmp"));
        for dir in [&home, &cwd, &tmp] {
            fs::create_dir(dir).expect("sandbox directory");
        }
        Sandbox {
            _dir: dir,
            root,
            home,
            cwd,
            tmp,
        }
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

    /// The front with an empty environment plus HOME, PATH and TMPDIR.
    pub fn command(&self) -> Command {
        self.command_for(Path::new(FRONT))
    }

    /// The same, for a front executable at another path (a copy or symlink).
    pub fn command_for(&self, front: &Path) -> Command {
        let mut command = Command::new(front);
        command
            .env_clear()
            .env("HOME", &self.home)
            .env("PATH", "/usr/bin:/bin")
            .env("TMPDIR", &self.tmp)
            .current_dir(&self.cwd)
            .stdin(Stdio::null());
        command
    }

    pub fn inspect(&self, args: &[&str]) -> Run {
        let mut command = self.command();
        command.arg("inspect").args(args);
        run(&mut command)
    }
}

pub fn write(path: &Path, contents: &str) {
    fs::create_dir_all(path.parent().expect("a parent")).expect("parent directory");
    fs::write(path, contents).expect("write file");
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
