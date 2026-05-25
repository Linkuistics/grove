use grove::harness::{by_name, HARNESSES};
use std::path::Path;

#[test]
fn registry_contains_claude_and_codex() {
    assert!(by_name("claude").is_some());
    assert!(by_name("codex").is_some());
    assert!(by_name("nonsense").is_none());
}

#[test]
fn install_path_is_under_project_dir() {
    let h = by_name("claude").unwrap();
    let path = h.install_path(Path::new("/tmp/repo"));
    assert_eq!(path, Path::new("/tmp/repo/.claude/skills/grove"));
}

use grove::harness::detect_in_repo;
use std::fs;
use tempfile::TempDir;

#[test]
fn detect_finds_claude_when_only_claude_present() {
    let tmp = TempDir::new().unwrap();
    fs::create_dir_all(tmp.path().join(".claude")).unwrap();

    let detected = detect_in_repo(tmp.path());
    assert_eq!(detected.len(), 1);
    assert_eq!(detected[0].name, "claude");
}

#[test]
fn detect_finds_both_when_both_present() {
    let tmp = TempDir::new().unwrap();
    fs::create_dir_all(tmp.path().join(".claude")).unwrap();
    fs::create_dir_all(tmp.path().join(".codex")).unwrap();

    let detected = detect_in_repo(tmp.path());
    assert_eq!(detected.len(), 2);
}

#[test]
fn detect_finds_none_in_empty_repo() {
    let tmp = TempDir::new().unwrap();
    assert!(detect_in_repo(tmp.path()).is_empty());
}
