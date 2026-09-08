//! Resolution answers to the path it was given and to nothing ambient.
//!
//! **The selectors are set in a child process, never in this one.** The claim
//! can only be made against a process whose environment already carries them —
//! an environment is inherited, not addressed, so they have to be *there*
//! before `Workspace::resolve` is called. Setting them here would mean
//! `std::env::set_var`, and std's rule for that is not a caution to weigh
//! against a benefit: "In multi-threaded programs on other operating systems,
//! the only sound option is to not use `set_var` or `remove_var` at all" — the
//! requirement being that no other thread so much as *reads* the environment,
//! which no library advertises about its own functions
//! (<https://doc.rust-lang.org/std/env/fn.set_var.html>). This binary is not
//! single-threaded: the harness runs a test on a thread it spawns unless it is
//! asked for `--test-threads=1`, and a test file cannot ask for that.
//!
//! Rust 2024 marks both functions `unsafe` for exactly this reason — "starting
//! in the 2024 Edition, while not requiring `unsafe` in previous editions"
//! (<https://doc.rust-lang.org/edition-guide/rust-2024/newly-unsafe-functions.html>),
//! which is the only reason this workspace's `edition = "2021"` compiled the
//! earlier version of this file at all. No `unsafe` block here could have been
//! discharged, so the mutation is gone rather than wrapped.
//!
//! So the fixture is built here, the variables are handed to `Command::env`,
//! and the half that must observe them re-runs *this same binary* under them.
//! `Command::env` writes the child's environment map and leaves this process
//! alone, so there is no mutation, nothing to restore, and no `unsafe` — an
//! edition-2024 migration has nothing to do to this file.
//! `crates/keyed-launch/tests/reraise.rs` re-runs itself the same way, for the
//! same reason: a property of a *process* can only be asserted by starting one.
//!
//! Passing the environment to the crate's own `jj` child instead is not
//! available: `jj::raw_output` builds every `Command` behind a `pub(crate)`
//! seam an integration test cannot reach, and reaching it would test the wrong
//! thing anyway — the property is that an inherited variable does not survive
//! that seam, which is a statement about what the seam removes rather than
//! about what a caller could add.
//!
//! It remains its own integration target. That is now organisational rather
//! than load-bearing — the process-global mutation it used to isolate is gone —
//! and it is kept because the child re-runs this binary by name and a binary of
//! one test makes that spawn say exactly what it does.

use jj_workspace::Workspace;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The variable the parent half sets and the child half answers to. Its value
/// is the fixture root, so the child derives every path from the same joins the
/// parent built them with. Nothing outside this file reads it.
const REEXEC: &str = "JJ_WORKSPACE_ENVIRONMENT_TEST_CHILD";

/// The test's own name, as `--exact` needs it, kept beside the `#[test]` fn it
/// names. A rename that misses this does **not** fail loudly: a libtest binary
/// whose filter matches nothing reports `0 passed` and exits 0, so the child
/// would succeed having run no test and the parent would pass vacuously. That
/// is what the sentinel below is for.
const TEST: &str = "resolution_ignores_repository_selection_and_temporary_directory_environment";

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

#[test]
fn resolution_ignores_repository_selection_and_temporary_directory_environment() {
    if let Some(root) = std::env::var_os(REEXEC) {
        // The child half, running under the environment the parent handed it.
        return under_ambient_environment(Path::new(&root));
    }

    let tmp = tempfile::TempDir::new().unwrap();
    let intended = colocated(&tmp.path().join("intended"));
    let foreign = colocated(&tmp.path().join("foreign"));
    let ambient_tmp = tmp.path().join("ambient-tmp");
    fs::create_dir_all(&ambient_tmp).unwrap();
    fs::create_dir_all(intended.join("src")).unwrap();

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

    let out = Command::new(std::env::current_exe().unwrap())
        .args([TEST, "--exact", "--nocapture"])
        .env(REEXEC, tmp.path())
        .env("JJ_CONFIG", &config)
        .env("GIT_DIR", foreign.join(".git"))
        .env("GIT_WORK_TREE", &foreign)
        .env("GIT_COMMON_DIR", foreign.join(".git"))
        .env("GIT_INDEX_FILE", &ambient_index)
        // Not a repository selector, but the other way a derived path can be
        // redirected: a control directory that followed `TMPDIR` would let two
        // processes on one working tree derive different ones.
        .env("TMPDIR", &ambient_tmp)
        .output()
        .expect("re-running this test binary as a child");

    // The child's own output is the failure message. Without it the parent
    // reports a wait status and the assertion that actually fired is lost.
    assert!(
        out.status.success(),
        "the child half failed under the ambient environment ({}):\n{}{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    // The control on the split, and the reason a green child is worth anything:
    // a success status alone cannot tell "the assertions held" from "the filter
    // matched no test". This file is written by the child and by nothing else,
    // immediately before the two assertions that carry the claim, so its absence
    // means the child never reached them however it exited.
    assert!(
        intended.join("src/f.txt").exists(),
        "the child half did not run: nothing was written under the intended tree, \
         so its success status says only that it ran no test"
    );
}

/// The half that must run under the selectors. Every path is derived from the
/// fixture root by the same joins the parent used, and `canonicalize` is
/// repeated for the same reason it is applied there: the comparisons below are
/// against what `Workspace::resolve` returns, which is canonical.
fn under_ambient_environment(root: &Path) {
    let intended = root.join("intended").canonicalize().unwrap();
    let foreign = root.join("foreign").canonicalize().unwrap();
    let nested = intended.join("src");
    let ambient_index = foreign.join(".git/ambient-index");

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
