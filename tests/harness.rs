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
