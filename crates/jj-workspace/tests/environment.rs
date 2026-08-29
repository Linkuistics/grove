//! Resolution answers to the path it was given and to nothing ambient.
//!
//! **This is its own test binary on purpose.** The claim can only be made by
//! mutating the process environment, and `cargo test` runs the tests within one
//! binary in parallel threads that share it — so a `GIT_DIR` set here would be
//! visible to every other test in the file. A separate integration target is a
//! separate process, which makes the mutation local without a lock every
//! unrelated test would have to take.

use jj_workspace::Workspace;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A **colocated** repository, so the `GIT_*` selectors below are live rather
/// than inert: a colocated tree has a real `.git` for them to point somewhere
/// else, which is what makes the assertion worth making.
fn colocated(path: &Path) -> PathBuf {
    fs::create_dir_all(path).unwrap();
    let out = Command::new("jj")
        .current_dir(path)
        .args(["git", "init", "--colocate", "--quiet", "."])
        .output()
        .expect("running jj git init (is jj installed?)");
    assert!(
        out.status.success(),
        "jj git init failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    path.canonicalize().unwrap()
}

struct EnvGuard {
    saved: Vec<(&'static str, Option<std::ffi::OsString>)>,
}

impl EnvGuard {
    fn new() -> Self {
        Self { saved: Vec::new() }
    }

    fn set(&mut self, key: &'static str, value: impl AsRef<OsStr>) -> &mut Self {
        self.saved.push((key, std::env::var_os(key)));
        std::env::set_var(key, value);
        self
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        for (key, value) in self.saved.drain(..) {
            match value {
                Some(value) => std::env::set_var(key, value),
                None => std::env::remove_var(key),
            }
        }
    }
}

#[test]
fn resolution_ignores_repository_selection_and_temporary_directory_environment() {
    let tmp = tempfile::TempDir::new().unwrap();
    let intended = colocated(&tmp.path().join("intended"));
    let foreign = colocated(&tmp.path().join("foreign"));
    let ambient_tmp = tmp.path().join("ambient-tmp");
    fs::create_dir_all(&ambient_tmp).unwrap();
    let nested = intended.join("src");
    fs::create_dir_all(&nested).unwrap();

    let mut env = EnvGuard::new();
    env.set("GIT_DIR", foreign.join(".git"))
        .set("GIT_WORK_TREE", &foreign)
        .set("GIT_COMMON_DIR", foreign.join(".git"))
        // Not a repository selector, but the other way a derived path can be
        // redirected: a control directory that followed `TMPDIR` would let two
        // processes on one working tree derive different ones.
        .set("TMPDIR", &ambient_tmp);

    let workspace = Workspace::resolve(&nested).unwrap();

    assert_eq!(workspace.root(), intended);
    assert_eq!(workspace.main_repo(), intended);
    assert_eq!(
        workspace.control_dir("notekeeper").unwrap(),
        intended.join(".jj/notekeeper")
    );
    assert!(
        !foreign.join(".jj/notekeeper").exists(),
        "nothing may have been created in the environment-selected repository"
    );
}
