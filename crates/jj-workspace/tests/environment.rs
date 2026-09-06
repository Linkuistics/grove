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

    // A path that does not exist yet, so its later existence is unambiguous:
    // the index jj exports for a colocated tree is the only thing that could
    // have created it.
    let ambient_index = foreign.join(".git/ambient-index");

    // The second control on this test, alongside the colocated fixture. The
    // index below is written as part of the snapshot `is_tracked` triggers, and
    // `snapshot.auto-track` decides whether that snapshot takes the new file at
    // all — so a developer whose own configuration sets it to `none()` would see
    // both assertions below go red with the scrub perfectly intact, which is the
    // one reading this test exists to rule out. The seam deliberately leaves
    // `JJ_CONFIG` in the child's environment (it configures the *user*), so
    // pinning it here is what makes the red signal specific to the selectors.
    let config = tmp.path().join("pinned.toml");
    fs::write(&config, "[snapshot]\nauto-track = \"all()\"\n").unwrap();

    let mut env = EnvGuard::new();
    env.set("JJ_CONFIG", &config)
        .set("GIT_DIR", foreign.join(".git"))
        .set("GIT_WORK_TREE", &foreign)
        .set("GIT_COMMON_DIR", foreign.join(".git"))
        .set("GIT_INDEX_FILE", &ambient_index)
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

    // Everything above is answered by the filesystem walk and by `create_dir_all`
    // — both trees here are colocated, so `.jj/repo` is a directory, `main_repo_of`
    // returns without asking jj anything, and no child is spawned. The selectors
    // are removed at the child-process seam, so an assertion about them has to
    // reach that seam: `is_tracked` runs `jj file list`, which snapshots the
    // working copy and exports it to the colocated Git repository. That export is
    // what the four selectors redirect, and `GIT_INDEX_FILE` is the member only a
    // snapshotting call can reach — `main_repo_of` passes `--ignore-working-copy`,
    // which writes no index at all.
    let file = nested.join("f.txt");
    fs::write(&file, "hello").unwrap();
    assert!(workspace.is_tracked(&file).unwrap());

    assert!(
        intended.join(".git/index").exists(),
        "the colocated index must have been exported into the intended repository"
    );
    // The discriminating assertion is the one above: under the only mutation that
    // turns this test red — `GIT_INDEX_FILE` removed from the array — the index
    // is absent from the intended repository and that assertion fires first. This
    // one adds signal only in a world where jj wrote both, and is kept as the
    // direct statement of the property rather than as a second check.
    assert!(
        !ambient_index.exists(),
        "no index may have been written to the environment-selected path"
    );
}
