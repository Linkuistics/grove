use grove::repo::{main_repo_of, toplevel, vcs_of, workspace_control, Vcs};
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

// ---- vcs_of: the jj-first ownership probe ---------------------------------

#[test]
fn vcs_of_plain_git_repo_is_git() {
    let tmp = TempDir::new().unwrap();
    init_git_repo(tmp.path());
    assert!(matches!(vcs_of(tmp.path()), Some(Vcs::Git)));
}

#[test]
fn vcs_of_jj_native_repo_is_jj_with_its_workspace_root() {
    let tmp = TempDir::new().unwrap();
    init_jj_repo(tmp.path());
    let sub = tmp.path().join("a/b");
    fs::create_dir_all(&sub).unwrap();
    match vcs_of(&sub) {
        Some(Vcs::Jj { workspace_root }) => {
            assert_eq!(canon(&workspace_root), canon(tmp.path()))
        }
        other => panic!("expected Jj, got {other:?}"),
    }
}

#[test]
fn vcs_of_colocated_repo_is_jj_first() {
    // `.jj/` beside `.git/`: the jj-enabled state picks jj, per the symmetric
    // VCS rule — git is used only in not-jj-enabled trees.
    let tmp = TempDir::new().unwrap();
    init_git_repo(tmp.path());
    run_jj(tmp.path(), &["git", "init", "--colocate", "--quiet", "."]);
    assert!(matches!(vcs_of(tmp.path()), Some(Vcs::Jj { .. })));
}

#[test]
fn vcs_of_git_repo_nested_under_jj_tree_is_git() {
    // The closest marker wins: an inner plain-git checkout is not captured by
    // an outer jj-enabled tree.
    let tmp = TempDir::new().unwrap();
    init_jj_repo(tmp.path());
    let inner = tmp.path().join("inner");
    fs::create_dir_all(&inner).unwrap();
    init_git_repo(&inner);
    assert!(matches!(vcs_of(&inner), Some(Vcs::Git)));
}

#[test]
fn vcs_of_unversioned_dir_is_none() {
    // TempDir lives under the system temp dir, which itself is unversioned.
    let tmp = TempDir::new().unwrap();
    assert!(vcs_of(tmp.path()).is_none());
}

// ---- workspace_control: environment-independent coordination paths -------

#[test]
fn workspace_control_in_plain_git_uses_that_checkout_s_git_directory() {
    let tmp = TempDir::new().unwrap();
    init_git_repo(tmp.path());
    let nested = tmp.path().join("src/nested");
    fs::create_dir_all(&nested).unwrap();

    let control = workspace_control(&nested).unwrap();

    assert_eq!(control.worktree_root(), canon(tmp.path()));
    assert_eq!(
        control.control_dir(),
        canon(&tmp.path().join(".git")).join("grove")
    );
}

#[test]
fn workspace_control_in_a_linked_git_worktree_uses_its_own_gitdir() {
    let tmp = TempDir::new().unwrap();
    let main = tmp.path().join("main");
    fs::create_dir_all(&main).unwrap();
    init_git_repo(&main);
    run(
        "git",
        &main,
        &[
            "-c",
            "user.email=t@example.com",
            "-c",
            "user.name=Test",
            "commit",
            "-q",
            "--allow-empty",
            "-m",
            "seed",
        ],
    );
    let worktree = tmp.path().join("linked");
    run(
        "git",
        &main,
        &["worktree", "add", "-q", worktree.to_str().unwrap()],
    );
    let gitfile = fs::read_to_string(worktree.join(".git")).unwrap();
    let gitdir = gitfile.strip_prefix("gitdir: ").unwrap().trim();

    let control = workspace_control(&worktree).unwrap();

    assert_eq!(control.worktree_root(), canon(&worktree));
    assert_eq!(
        control.control_dir(),
        canon(&worktree.join(gitdir)).join("grove")
    );
    assert_ne!(
        control.control_dir(),
        canon(&main.join(".git")).join("grove")
    );
}

#[test]
fn workspace_control_in_colocated_jj_uses_the_workspace_jj_directory() {
    let tmp = TempDir::new().unwrap();
    init_git_repo(tmp.path());
    run_jj(tmp.path(), &["git", "init", "--colocate", "--quiet", "."]);

    let control = workspace_control(tmp.path()).unwrap();

    assert_eq!(control.worktree_root(), canon(tmp.path()));
    assert_eq!(control.control_dir(), canon(tmp.path()).join(".jj/grove"));
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

// ---- toplevel: the working-tree root, either VCS --------------------------

#[test]
fn toplevel_in_plain_git_repo_resolves_from_subdir() {
    let tmp = TempDir::new().unwrap();
    init_git_repo(tmp.path());
    let sub = tmp.path().join("src");
    fs::create_dir_all(&sub).unwrap();
    assert_eq!(canon(&toplevel(&sub).unwrap()), canon(tmp.path()));
}

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
    run_jj(
        &main,
        &["workspace", "add", "--quiet", ws.to_str().unwrap()],
    );
    assert_eq!(canon(&toplevel(&ws).unwrap()), canon(&ws));
}

#[test]
fn toplevel_outside_any_repo_errors() {
    let tmp = TempDir::new().unwrap();
    let err = toplevel(tmp.path()).unwrap_err();
    assert!(
        err.to_string().contains("not in a git or jj repo"),
        "unexpected error: {err}"
    );
}

// ---- main_repo_of: the main repo behind a working tree --------------------

#[test]
fn main_repo_of_git_linked_worktree_is_the_main_checkout() {
    let tmp = TempDir::new().unwrap();
    let main = tmp.path().join("main");
    fs::create_dir_all(&main).unwrap();
    init_git_repo(&main);
    run(
        "git",
        &main,
        &[
            "-c",
            "user.email=t@example.com",
            "-c",
            "user.name=Test",
            "commit",
            "-q",
            "--allow-empty",
            "-m",
            "seed",
        ],
    );
    let wt = tmp.path().join("wt");
    run(
        "git",
        &main,
        &["worktree", "add", "-q", wt.to_str().unwrap()],
    );
    assert_eq!(canon(&main_repo_of(&wt).unwrap()), canon(&main));
}

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
    run_jj(
        &main,
        &["workspace", "add", "--quiet", ws.to_str().unwrap()],
    );
    assert_eq!(canon(&main_repo_of(&ws).unwrap()), canon(&main));
}

// `repo::resolve` used to sit here: `resolve(Some(path))` validated a
// caller-supplied repository path and `resolve(None)` delegated to
// `main_repo_of(cwd)`. Bare `grove` takes no arguments, so the explicit-path
// form lost its last caller with the `--repo` flag and only these tests kept it
// compiling. The delegating form's behaviour is `main_repo_of`'s, covered
// directly above; nothing else was lost with it.
