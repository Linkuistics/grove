use grove::repo::{jj_workspace_root, main_repo_of, require_jj_workspace, toplevel, workspace_control};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

fn init_git_repo(path: &Path) {
    run("git", path, &["init", "-q", "."]);
}

/// A jj-native repo (no `.git/`). `git.colocate=false` is forced because the
/// ambient jj config may default colocation on, which would silently turn
/// every "native" fixture into a colocated one.
fn init_jj_repo(path: &Path) {
    run_jj(
        path,
        &[
            "--config",
            "git.colocate=false",
            "git",
            "init",
            "--quiet",
            ".",
        ],
    );
}

fn run(bin: &str, dir: &Path, args: &[&str]) {
    let out = Command::new(bin)
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("running {bin} {args:?}: {e} (is {bin} installed?)"));
    assert!(
        out.status.success(),
        "{bin} {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Run jj with a test-local user identity, so no global config is required.
fn run_jj(dir: &Path, args: &[&str]) {
    let mut full = vec![
        "--config",
        "user.name=Test",
        "--config",
        "user.email=t@example.com",
    ];
    full.extend_from_slice(args);
    run("jj", dir, &full);
}

fn canon(p: &Path) -> PathBuf {
    p.canonicalize().unwrap()
}

// ---- the precondition gate ------------------------------------------------

#[test]
fn a_jj_native_repo_resolves_its_workspace_root_from_a_subdirectory() {
    let tmp = TempDir::new().unwrap();
    init_jj_repo(tmp.path());
    let sub = tmp.path().join("a/b");
    fs::create_dir_all(&sub).unwrap();

    let root = require_jj_workspace(&sub).unwrap();

    assert_eq!(canon(&root), canon(tmp.path()));
}

#[test]
fn a_colocated_repo_resolves_through_its_jj_directory() {
    // `.jj/` beside `.git/` is jj's own business: the colocated repo is a jj
    // working tree, and nothing here reads the `.git` marker at all.
    let tmp = TempDir::new().unwrap();
    init_git_repo(tmp.path());
    run_jj(tmp.path(), &["git", "init", "--colocate", "--quiet", "."]);

    assert_eq!(
        canon(&require_jj_workspace(tmp.path()).unwrap()),
        canon(tmp.path())
    );
}

#[test]
fn a_plain_git_checkout_is_refused_with_the_command_that_fixes_it() {
    let tmp = TempDir::new().unwrap();
    init_git_repo(tmp.path());

    let error = require_jj_workspace(tmp.path()).unwrap_err().to_string();

    for expected in [
        "not a jj working tree",
        "looked for a `.jj` directory",
        "jj git init --colocate",
        "Nothing was created or changed",
    ] {
        assert!(error.contains(expected), "{expected:?} missing: {error}");
    }
}

#[test]
fn a_plain_git_checkout_nested_under_a_jj_tree_is_still_the_jj_tree_s() {
    // The gate walks for `.jj/` alone, so an inner plain-git checkout resolves
    // to the enclosing jj workspace rather than to itself. That is the whole
    // consequence of dropping the closest-marker-wins rule with the git lane:
    // there is no second marker left to be closer.
    let tmp = TempDir::new().unwrap();
    init_jj_repo(tmp.path());
    let inner = tmp.path().join("inner");
    fs::create_dir_all(&inner).unwrap();
    init_git_repo(&inner);

    assert_eq!(
        canon(&require_jj_workspace(&inner).unwrap()),
        canon(tmp.path())
    );
}

#[test]
fn an_unversioned_directory_has_no_workspace_root() {
    // TempDir lives under the system temp dir, which itself is unversioned.
    let tmp = TempDir::new().unwrap();
    assert!(jj_workspace_root(tmp.path()).is_none());
}

// ---- workspace_control: environment-independent coordination paths -------

#[test]
fn workspace_control_in_colocated_jj_uses_the_workspace_jj_directory() {
    let tmp = TempDir::new().unwrap();
    init_git_repo(tmp.path());
    run_jj(tmp.path(), &["git", "init", "--colocate", "--quiet", "."]);
    let nested = tmp.path().join("src/nested");
    fs::create_dir_all(&nested).unwrap();

    let control = workspace_control(&nested).unwrap();

    assert_eq!(control.worktree_root(), canon(tmp.path()));
    assert_eq!(control.control_dir(), canon(tmp.path()).join(".jj/grove"));
    assert_eq!(control.marker(), canon(tmp.path()).join(".jj"));
}

#[test]
fn workspace_control_in_a_secondary_jj_workspace_does_not_follow_the_shared_repo() {
    let tmp = TempDir::new().unwrap();
    let main = tmp.path().join("main");
    fs::create_dir_all(&main).unwrap();
    init_jj_repo(&main);
    let secondary = tmp.path().join("secondary");
    run_jj(
        &main,
        &["workspace", "add", "--quiet", secondary.to_str().unwrap()],
    );

    let control = workspace_control(&secondary).unwrap();

    assert_eq!(control.worktree_root(), canon(&secondary));
    assert_eq!(control.control_dir(), canon(&secondary).join(".jj/grove"));
    assert_ne!(control.control_dir(), canon(&main).join(".jj/grove"));
}

#[test]
fn workspace_control_outside_any_jj_workspace_gives_the_same_refusal() {
    let tmp = TempDir::new().unwrap();

    let error = workspace_control(tmp.path()).unwrap_err().to_string();

    assert!(
        error.contains("jj git init --colocate"),
        "unexpected error: {error}"
    );
}

// ---- toplevel: the working-tree root --------------------------------------

#[test]
fn toplevel_in_jj_native_repo_resolves_from_subdir() {
    let tmp = TempDir::new().unwrap();
    init_jj_repo(tmp.path());
    let sub = tmp.path().join("src");
    fs::create_dir_all(&sub).unwrap();
    assert_eq!(canon(&toplevel(&sub).unwrap()), canon(tmp.path()));
}

#[test]
fn toplevel_in_secondary_jj_workspace_is_the_workspace_root() {
    // A secondary workspace is its own working tree; grove runs *in* it, so
    // its root — not the main repo's — is the worktree.
    let tmp = TempDir::new().unwrap();
    let main = tmp.path().join("main");
    fs::create_dir_all(&main).unwrap();
    init_jj_repo(&main);
    let ws = tmp.path().join("ws2");
    run_jj(&main, &["workspace", "add", "--quiet", ws.to_str().unwrap()]);
    assert_eq!(canon(&toplevel(&ws).unwrap()), canon(&ws));
}

#[test]
fn toplevel_outside_any_repo_is_refused_before_anything_else() {
    let tmp = TempDir::new().unwrap();
    let err = toplevel(tmp.path()).unwrap_err();
    assert!(
        err.to_string().contains("not a jj working tree"),
        "unexpected error: {err}"
    );
}

// ---- main_repo_of: the main repo behind a working tree --------------------

#[test]
fn main_repo_of_jj_native_repo_is_itself() {
    let tmp = TempDir::new().unwrap();
    init_jj_repo(tmp.path());
    assert_eq!(canon(&main_repo_of(tmp.path()).unwrap()), canon(tmp.path()));
}

#[test]
fn main_repo_of_secondary_jj_workspace_is_the_default_workspace_root() {
    let tmp = TempDir::new().unwrap();
    let main = tmp.path().join("main");
    fs::create_dir_all(&main).unwrap();
    init_jj_repo(&main);
    let ws = tmp.path().join("ws2");
    run_jj(&main, &["workspace", "add", "--quiet", ws.to_str().unwrap()]);
    assert_eq!(canon(&main_repo_of(&ws).unwrap()), canon(&main));
}

#[test]
fn main_repo_of_outside_any_jj_workspace_never_reaches_the_jj_binary() {
    // The gate comes first, so the refusal is Grove's own diagnostic rather
    // than whatever `jj workspace root` would have said about the cwd.
    let tmp = TempDir::new().unwrap();
    init_git_repo(tmp.path());

    let error = main_repo_of(tmp.path()).unwrap_err().to_string();

    assert!(
        error.contains("jj git init --colocate"),
        "unexpected error: {error}"
    );
}

// `repo::resolve` used to sit here: `resolve(Some(path))` validated a
// caller-supplied repository path and `resolve(None)` delegated to
// `main_repo_of(cwd)`. Bare `grove` takes no arguments, so the explicit-path
// form lost its last caller with the `--repo` flag and only these tests kept it
// compiling. The delegating form's behaviour is `main_repo_of`'s, covered
// directly above; nothing else was lost with it.
