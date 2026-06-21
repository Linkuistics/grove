use grove::repo::resolve;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

fn init_git_repo(path: &Path) {
    Command::new("git").arg("init").arg(path).status().unwrap();
}

#[test]
fn resolve_uses_explicit_arg_when_provided() {
    let tmp = TempDir::new().unwrap();
    init_git_repo(tmp.path());

    let resolved = resolve(Some(tmp.path())).unwrap();
    assert_eq!(
        resolved.canonicalize().unwrap(),
        tmp.path().canonicalize().unwrap()
    );
}

#[test]
fn resolve_errors_when_arg_is_not_a_git_repo() {
    let tmp = TempDir::new().unwrap();
    let err = resolve(Some(tmp.path())).unwrap_err();
    assert!(err.to_string().contains("not a git repo"));
}
