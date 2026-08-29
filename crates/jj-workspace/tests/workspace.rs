//! The crate's public interface, exercised **without grove** — test seam 1.
//!
//! That is the point of the file rather than a property of it. A crate can
//! claim to be domain-free in a doc comment and still be unusable without its
//! one consumer; a test binary that links only this crate is the
//! compiler-enforced form of the claim. Nothing below names grove, `.grove/`,
//! a leaf, a lease or a task tree — the namespace this suite reserves is
//! `notekeeper`, a consumer that does not exist, precisely so that the name
//! being arbitrary is visible.
//!
//! The fixtures drive the real `jj` binary against real repositories. There is
//! no fake: the crate's entire job is what jj does when asked, so a double
//! would be asserting this file's beliefs about jj rather than jj's behaviour.

use jj_workspace::Workspace;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

// ---- fixtures --------------------------------------------------------------

fn jj(dir: &Path, args: &[&str]) {
    let out = Command::new("jj")
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("running jj {args:?}: {e} (is jj installed?)"));
    assert!(
        out.status.success(),
        "jj {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// A repository whose committer identity is written into the **repo's own**
/// config rather than passed per command or exported into the environment.
/// Per-command `--config` cannot reach the commits this crate takes for itself,
/// and `JJ_USER`/`JJ_EMAIL` are process-global in a suite that runs its tests in
/// parallel.
fn init(path: &Path, colocate: bool) {
    let colocation = if colocate {
        "git.colocate=true"
    } else {
        "git.colocate=false"
    };
    jj(
        path,
        &["--config", colocation, "git", "init", "--quiet", "."],
    );
    jj(path, &["config", "set", "--repo", "user.name", "Test"]);
    jj(
        path,
        &["config", "set", "--repo", "user.email", "t@example.com"],
    );
}

/// A jj-native repository — no `.git/`. Colocation is forced *off* because the
/// ambient jj config may default it on, which would silently turn every
/// "native" fixture into a colocated one.
fn native(path: &Path) -> PathBuf {
    fs::create_dir_all(path).unwrap();
    init(path, false);
    canon(path)
}

/// A colocated repository, so a `.git` really is present beside the `.jj`.
fn colocated(path: &Path) -> PathBuf {
    fs::create_dir_all(path).unwrap();
    init(path, true);
    canon(path)
}

fn canon(path: &Path) -> PathBuf {
    path.canonicalize().unwrap()
}

fn write(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, contents).unwrap();
}

// ---- resolve: the precondition gate ----------------------------------------

#[test]
fn a_workspace_resolves_from_a_subdirectory() {
    let tmp = TempDir::new().unwrap();
    let root = native(&tmp.path().join("repo"));
    let deep = root.join("a/b/c");
    fs::create_dir_all(&deep).unwrap();

    let workspace = Workspace::resolve(&deep).unwrap();

    assert_eq!(workspace.root(), root);
}

#[test]
fn a_colocated_workspace_resolves_to_the_workspace_and_not_the_git_repository() {
    let tmp = TempDir::new().unwrap();
    let root = colocated(&tmp.path().join("repo"));

    let workspace = Workspace::resolve(&root).unwrap();

    assert!(root.join(".git").exists(), "fixture is not colocated");
    assert_eq!(workspace.root(), root);
}

// The refusal is the product of this call, so it is asserted as text: what was
// looked for, where, and the command that fixes it.
#[test]
fn a_tree_that_is_not_a_workspace_is_refused_with_the_command_that_fixes_it() {
    let tmp = TempDir::new().unwrap();

    let refusal = Workspace::resolve(tmp.path()).unwrap_err().to_string();

    assert!(
        refusal.contains("not a Jujutsu working tree"),
        "refusal must name what is wrong: {refusal}"
    );
    assert!(
        refusal.contains(&tmp.path().display().to_string()),
        "refusal must name where it looked: {refusal}"
    );
    assert!(
        refusal.contains("jj git init --colocate"),
        "refusal must name jj's own remedy: {refusal}"
    );
    assert!(
        refusal.contains("Nothing was created or changed"),
        "refusal must say the gate ran before any mutation: {refusal}"
    );
}

// A gate that only refuses is worth nothing if it refuses *after* acting. The
// tree is compared before and after, so "nothing was created or changed" is a
// measured claim rather than a sentence in a message.
#[test]
fn a_refused_tree_is_left_exactly_as_it_was() {
    let tmp = TempDir::new().unwrap();
    write(&tmp.path().join("only.txt"), "untouched");
    let before = entries(tmp.path());

    assert!(Workspace::resolve(tmp.path()).is_err());

    assert_eq!(entries(tmp.path()), before);
}

fn entries(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(dir)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

// The walk looks for `.jj` and for nothing else, so a Git checkout nested
// inside a jj workspace resolves to the *jj* workspace rather than to itself.
// That is the whole consequence of there being one lane: no second kind of
// marker is left to be closer.
#[test]
fn a_git_checkout_nested_inside_a_workspace_still_resolves_to_the_workspace() {
    let tmp = TempDir::new().unwrap();
    let root = native(&tmp.path().join("repo"));
    let inner = root.join("vendored");
    fs::create_dir_all(inner.join(".git")).unwrap();

    assert_eq!(Workspace::resolve(&inner).unwrap().root(), root);
}

// The gate is a filesystem walk, so it refuses without ever reaching jj — the
// diagnostic is this crate's, not whatever jj would have said about the
// directory it was run in.
#[test]
fn a_git_checkout_that_is_not_a_workspace_is_refused_by_the_walk() {
    let tmp = TempDir::new().unwrap();
    fs::create_dir_all(tmp.path().join(".git")).unwrap();

    let refusal = Workspace::resolve(tmp.path()).unwrap_err().to_string();

    assert!(
        refusal.contains("not a Jujutsu working tree"),
        "the walk must own the refusal: {refusal}"
    );
}

// Aliases of one workspace must not resolve to workspaces that disagree, or a
// lease keyed on the root would admit two owners of one tree.
#[test]
#[cfg(unix)]
fn a_symlinked_alias_resolves_to_the_same_workspace() {
    let tmp = TempDir::new().unwrap();
    let root = native(&tmp.path().join("repo"));
    let alias = tmp.path().join("alias");
    std::os::unix::fs::symlink(&root, &alias).unwrap();

    assert_eq!(
        Workspace::resolve(&alias).unwrap(),
        Workspace::resolve(&root).unwrap()
    );
}

// ---- main_repo: the workspace that holds the repository --------------------

#[test]
fn a_workspace_that_holds_the_repository_is_its_own_main_repo() {
    let tmp = TempDir::new().unwrap();
    let root = native(&tmp.path().join("repo"));

    assert_eq!(Workspace::resolve(&root).unwrap().main_repo(), root);
}

// The case the filesystem cannot answer: a secondary workspace borrows another's
// repository, so the pointer has to be followed and jj is what follows it.
#[test]
fn a_secondary_workspace_reports_the_default_workspace_as_its_main_repo() {
    let tmp = TempDir::new().unwrap();
    let main = native(&tmp.path().join("main"));
    let secondary = tmp.path().join("secondary");
    jj(
        &main,
        &[
            "workspace",
            "add",
            "--name",
            "secondary",
            secondary.to_str().unwrap(),
        ],
    );
    let secondary = canon(&secondary);

    let workspace = Workspace::resolve(&secondary).unwrap();

    assert_eq!(workspace.root(), secondary);
    assert_eq!(workspace.main_repo(), main);
}

// Resolution gates on *being a workspace*, and stops there. A `.jj/` whose
// contents are damaged or absent still resolves — the repository's integrity is
// a different question with a different remedy, and jj states it at the first
// command that needs one. Asserted because the alternative reading, that
// resolution validates the repository, would make it cost a subprocess every
// time and refuse for reasons the caller did not ask about.
#[test]
fn resolution_does_not_validate_the_repository_behind_the_workspace() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("hollow");
    fs::create_dir_all(root.join(".jj")).unwrap();
    let root = canon(&root);

    let workspace = Workspace::resolve(&root).unwrap();

    assert_eq!(workspace.root(), root);
    assert_eq!(workspace.main_repo(), root);
}

// ---- control_dir: the consumer's namespace ---------------------------------

#[test]
fn a_control_directory_is_created_inside_the_workspace_administration_area() {
    let tmp = TempDir::new().unwrap();
    let root = native(&tmp.path().join("repo"));
    let workspace = Workspace::resolve(&root).unwrap();

    let control = workspace.control_dir("notekeeper").unwrap();

    assert_eq!(control, root.join(".jj/notekeeper"));
    assert!(control.is_dir(), "the directory must exist afterwards");
}

#[test]
fn asking_twice_for_one_namespace_gives_the_same_directory_and_keeps_its_contents() {
    let tmp = TempDir::new().unwrap();
    let root = native(&tmp.path().join("repo"));
    let workspace = Workspace::resolve(&root).unwrap();
    let first = workspace.control_dir("notekeeper").unwrap();
    write(&first.join("lease"), "held");

    let second = workspace.control_dir("notekeeper").unwrap();

    assert_eq!(second, first);
    assert_eq!(fs::read_to_string(second.join("lease")).unwrap(), "held");
}

#[test]
fn two_namespaces_do_not_share_a_directory() {
    let tmp = TempDir::new().unwrap();
    let root = native(&tmp.path().join("repo"));
    let workspace = Workspace::resolve(&root).unwrap();

    assert_ne!(
        workspace.control_dir("notekeeper").unwrap(),
        workspace.control_dir("stopwatch").unwrap()
    );
}

// The control directory is untracked *because of where it is*, and that is the
// claim the namespace exists to keep: a coordination file must not become an
// artifact of the work it coordinates.
#[test]
fn a_control_directory_is_not_tracked_even_after_the_workspace_is_snapshotted() {
    let tmp = TempDir::new().unwrap();
    let root = native(&tmp.path().join("repo"));
    let workspace = Workspace::resolve(&root).unwrap();
    let control = workspace.control_dir("notekeeper").unwrap();
    write(&control.join("lease"), "held");
    write(&root.join("tracked.txt"), "in the working copy");

    // Any jj command snapshots; this one is here to make the snapshot happen.
    jj(&root, &["status"]);

    assert!(!workspace.is_tracked(&control).unwrap());
    assert!(workspace.is_tracked(Path::new("tracked.txt")).unwrap());
}

// Two workspaces sharing one repository must not share one control directory,
// or a lease taken in either would be seen by both.
#[test]
fn a_secondary_workspace_gets_its_own_control_directory_not_the_shared_one() {
    let tmp = TempDir::new().unwrap();
    let main = native(&tmp.path().join("main"));
    let secondary = tmp.path().join("secondary");
    jj(
        &main,
        &[
            "workspace",
            "add",
            "--name",
            "secondary",
            secondary.to_str().unwrap(),
        ],
    );
    let secondary = canon(&secondary);

    let control = Workspace::resolve(&secondary)
        .unwrap()
        .control_dir("notekeeper")
        .unwrap();

    assert_eq!(control, secondary.join(".jj/notekeeper"));
    assert!(
        !main.join(".jj/notekeeper").exists(),
        "the shared repository's workspace must be untouched"
    );
}

#[test]
fn a_namespace_that_is_a_path_is_refused() {
    let tmp = TempDir::new().unwrap();
    let root = native(&tmp.path().join("repo"));
    let workspace = Workspace::resolve(&root).unwrap();

    for escape in ["..", "../elsewhere", "nested/deeper", ""] {
        let refusal = workspace.control_dir(escape).unwrap_err().to_string();
        assert!(
            refusal.contains("control namespace"),
            "`{escape}` must be refused as a namespace: {refusal}"
        );
    }
    assert!(
        !root.parent().unwrap().join("elsewhere").exists(),
        "a refused namespace must not have created anything"
    );
}

#[test]
fn a_namespace_jujutsu_owns_is_refused() {
    let tmp = TempDir::new().unwrap();
    let root = native(&tmp.path().join("repo"));
    let workspace = Workspace::resolve(&root).unwrap();

    let refusal = workspace.control_dir("repo").unwrap_err().to_string();

    assert!(
        refusal.contains("Jujutsu owns that name"),
        "the collision must be named: {refusal}"
    );
    assert!(
        root.join(".jj/repo").is_dir(),
        "the refusal must not have disturbed jj's own directory"
    );
}

// ---- is_tracked ------------------------------------------------------------

#[test]
fn a_committed_file_is_tracked_and_an_ignored_one_is_not() {
    let tmp = TempDir::new().unwrap();
    let root = native(&tmp.path().join("repo"));
    let workspace = Workspace::resolve(&root).unwrap();
    write(&root.join("committed.txt"), "yes");
    write(&root.join(".gitignore"), "/ignored.txt\n");
    write(&root.join("ignored.txt"), "no");
    jj(&root, &["commit", "-m", "a file"]);

    assert!(workspace.is_tracked(Path::new("committed.txt")).unwrap());
    assert!(!workspace.is_tracked(Path::new("ignored.txt")).unwrap());
}

#[test]
fn a_directory_is_tracked_when_the_snapshot_holds_anything_beneath_it() {
    let tmp = TempDir::new().unwrap();
    let root = native(&tmp.path().join("repo"));
    let workspace = Workspace::resolve(&root).unwrap();
    write(&root.join("notes/deep/one.md"), "content");
    jj(&root, &["commit", "-m", "a tree"]);

    assert!(workspace.is_tracked(Path::new("notes")).unwrap());
    assert!(!workspace.is_tracked(Path::new("absent")).unwrap());
}

#[test]
fn an_absolute_path_and_a_root_relative_one_answer_alike() {
    let tmp = TempDir::new().unwrap();
    let root = native(&tmp.path().join("repo"));
    let workspace = Workspace::resolve(&root).unwrap();
    write(&root.join("committed.txt"), "yes");
    jj(&root, &["commit", "-m", "a file"]);

    assert!(workspace.is_tracked(&root.join("committed.txt")).unwrap());
    assert!(workspace.is_tracked(Path::new("committed.txt")).unwrap());
}

// Two halves of one claim, both measured rather than reasoned about.
//
// The answer is about the tree as it is now: a file that appeared since the
// last snapshot reads tracked, because that is what a caller deciding whether
// removing it could be undone needs to know.
//
// And letting jj snapshot costs no *extra* history: the operation it records is
// the one the next jj command would have recorded anyway, and an unchanged
// working copy records none at all.
#[test]
fn what_is_tracked_is_answered_about_the_tree_as_it_is_now() {
    let tmp = TempDir::new().unwrap();
    let root = native(&tmp.path().join("repo"));
    let workspace = Workspace::resolve(&root).unwrap();
    write(&root.join("committed.txt"), "yes");
    jj(&root, &["commit", "-m", "a file"]);

    write(&root.join("appeared-since.txt"), "not yet snapshotted");
    assert!(workspace.is_tracked(Path::new("appeared-since.txt")).unwrap());
}

#[test]
fn asking_twice_over_an_unchanged_working_copy_records_no_operation() {
    let tmp = TempDir::new().unwrap();
    let root = native(&tmp.path().join("repo"));
    let workspace = Workspace::resolve(&root).unwrap();
    write(&root.join("committed.txt"), "yes");
    jj(&root, &["commit", "-m", "a file"]);
    assert!(workspace.is_tracked(Path::new("committed.txt")).unwrap());
    let before = operation_count(&root);

    assert!(workspace.is_tracked(Path::new("committed.txt")).unwrap());
    assert!(!workspace.is_tracked(Path::new("absent.txt")).unwrap());

    assert_eq!(operation_count(&root), before);
}

fn operation_count(root: &Path) -> usize {
    let out = Command::new("jj")
        .current_dir(root)
        .args([
            "op",
            "log",
            "--no-graph",
            "--ignore-working-copy",
            "-T",
            "id",
        ])
        .output()
        .unwrap();
    assert!(out.status.success());
    String::from_utf8(out.stdout).unwrap().lines().count()
}

#[test]
fn a_path_outside_the_workspace_is_refused_rather_than_answered() {
    let tmp = TempDir::new().unwrap();
    let root = native(&tmp.path().join("repo"));
    let outside = tmp.path().join("outside.txt");
    write(&outside, "not ours");

    let refusal = Workspace::resolve(&root)
        .unwrap()
        .is_tracked(&outside)
        .unwrap_err()
        .to_string();

    assert!(
        refusal.contains("is not inside the Jujutsu workspace"),
        "a workspace answers only for its own files: {refusal}"
    );
}

// ---- commit ----------------------------------------------------------------

#[test]
fn a_commit_is_scoped_to_the_named_paths_and_leaves_the_rest_in_the_working_copy() {
    let tmp = TempDir::new().unwrap();
    let root = native(&tmp.path().join("repo"));
    let workspace = Workspace::resolve(&root).unwrap();
    write(&root.join("wanted/one.md"), "in scope");
    write(&root.join("unrelated.txt"), "out of scope");

    let commit = workspace
        .commit(&[Path::new("wanted")], "record the wanted tree")
        .unwrap();

    assert_eq!(files_in(&root, "@-"), vec!["wanted/one.md".to_string()]);
    assert_eq!(
        change_description(&root, &commit.change_id),
        "record the wanted tree"
    );
    assert!(
        root.join("unrelated.txt").exists(),
        "an out-of-scope file must stay in the working copy"
    );
}

#[test]
fn a_deletion_is_committable_after_the_path_is_gone() {
    let tmp = TempDir::new().unwrap();
    let root = native(&tmp.path().join("repo"));
    let workspace = Workspace::resolve(&root).unwrap();
    write(&root.join("notes/one.md"), "content");
    workspace
        .commit(&[Path::new("notes")], "record the notes")
        .unwrap();

    fs::remove_dir_all(root.join("notes")).unwrap();
    workspace
        .commit(&[Path::new("notes")], "remove the notes")
        .unwrap();

    assert!(files_in(&root, "@-").is_empty());
}

// The change id, not the commit id: it has to survive a rewrite, or it is not
// the identity of the work.
#[test]
fn the_returned_change_id_still_names_the_commit_after_it_is_rewritten() {
    let tmp = TempDir::new().unwrap();
    let root = native(&tmp.path().join("repo"));
    let workspace = Workspace::resolve(&root).unwrap();
    write(&root.join("one.md"), "content");
    let commit = workspace
        .commit(&[Path::new("one.md")], "first wording")
        .unwrap();

    jj(
        &root,
        &["describe", "-r", &commit.change_id, "-m", "second wording"],
    );

    assert_eq!(
        change_description(&root, &commit.change_id),
        "second wording"
    );
}

#[test]
fn a_commit_with_no_paths_is_refused_rather_than_widened() {
    let tmp = TempDir::new().unwrap();
    let root = native(&tmp.path().join("repo"));
    let workspace = Workspace::resolve(&root).unwrap();
    write(&root.join("unrelated.txt"), "must not be swept in");
    let before = operation_count(&root);

    let refusal = workspace.commit(&[], "everything").unwrap_err().to_string();

    assert!(
        refusal.contains("no scope"),
        "the refusal must name the missing scope: {refusal}"
    );
    assert_eq!(
        operation_count(&root),
        before,
        "a refused commit must not have run jj at all"
    );
}

// The refusal that says *there is no commit* must hand back jj's own repair,
// not a repair of this crate's.
#[test]
#[cfg(unix)]
fn a_commit_that_cannot_land_names_the_operation_log_repair() {
    use std::os::unix::fs::PermissionsExt;

    let tmp = TempDir::new().unwrap();
    let root = native(&tmp.path().join("repo"));
    let workspace = Workspace::resolve(&root).unwrap();
    write(&root.join("unreadable/one.md"), "content");
    // jj snapshots the whole working copy before it commits any part of it, so
    // a directory this process cannot read is a genuine failure of the command
    // rather than a simulated one. Restored below, or the fixture cannot be
    // removed.
    let unreadable = root.join("unreadable");
    fs::set_permissions(&unreadable, fs::Permissions::from_mode(0o000)).unwrap();

    let outcome = workspace.commit(&[Path::new("unreadable")], "record it");

    fs::set_permissions(&unreadable, fs::Permissions::from_mode(0o755)).unwrap();
    let refusal = outcome.unwrap_err().to_string();

    assert!(
        refusal.contains("the commit did not land"),
        "the refusal must say the commit is absent: {refusal}"
    );
    assert!(
        refusal.contains("jj undo") && refusal.contains("jj op log"),
        "the refusal must name jj's repair: {refusal}"
    );
    assert!(
        refusal.contains("Nothing here runs a recovery of its own"),
        "the refusal must disclaim repairing anything: {refusal}"
    );
}

fn files_in(root: &Path, revision: &str) -> Vec<String> {
    let out = Command::new("jj")
        .current_dir(root)
        .args(["file", "list", "--ignore-working-copy", "-r", revision])
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "jj file list failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout)
        .unwrap()
        .lines()
        .map(str::to_owned)
        .collect()
}

fn change_description(root: &Path, change_id: &str) -> String {
    let out = Command::new("jj")
        .current_dir(root)
        .args([
            "log",
            "--no-graph",
            "--ignore-working-copy",
            "-r",
            change_id,
            "-T",
            "description",
        ])
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "jj log failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap().trim().to_owned()
}
