use grove::harness::{by_name, detect_in_repo, select, SelectMode};
use std::fs;
use tempfile::TempDir;

#[test]
fn registry_contains_claude_and_codex() {
    assert!(by_name("claude").is_some());
    assert!(by_name("codex").is_some());
    assert!(by_name("nonsense").is_none());
}

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

#[test]
fn select_explicit_overrides_detection() {
    let tmp = TempDir::new().unwrap();
    fs::create_dir_all(tmp.path().join(".claude")).unwrap();

    let selected = select(tmp.path(), &["codex".to_string()], SelectMode::Multi).unwrap();
    assert_eq!(selected.len(), 1);
    assert_eq!(selected[0].name, "codex");
}

#[test]
fn select_errors_on_unknown_harness() {
    let tmp = TempDir::new().unwrap();
    let err = select(tmp.path(), &["lemur".to_string()], SelectMode::Multi).unwrap_err();
    assert!(err.to_string().contains("unknown harness"));
}

#[test]
fn select_errors_when_no_harness_detectable() {
    let tmp = TempDir::new().unwrap();
    let err = select(tmp.path(), &[], SelectMode::Multi).unwrap_err();
    assert!(err.to_string().contains("no harness session detected"));
}

#[test]
fn select_multi_returns_both_when_both_present() {
    let tmp = TempDir::new().unwrap();
    fs::create_dir_all(tmp.path().join(".claude")).unwrap();
    fs::create_dir_all(tmp.path().join(".codex")).unwrap();

    let selected = select(tmp.path(), &[], SelectMode::Multi).unwrap();
    assert_eq!(selected.len(), 2);
}

#[test]
fn select_single_errors_when_both_present_without_explicit() {
    let tmp = TempDir::new().unwrap();
    fs::create_dir_all(tmp.path().join(".claude")).unwrap();
    fs::create_dir_all(tmp.path().join(".codex")).unwrap();

    let err = select(tmp.path(), &[], SelectMode::Single).unwrap_err();
    assert!(err.to_string().contains("multiple harnesses detected"));
    assert!(err.to_string().contains("--harness"));
}

#[test]
fn select_single_succeeds_when_one_present() {
    let tmp = TempDir::new().unwrap();
    fs::create_dir_all(tmp.path().join(".claude")).unwrap();

    let selected = select(tmp.path(), &[], SelectMode::Single).unwrap();
    assert_eq!(selected.len(), 1);
    assert_eq!(selected[0].name, "claude");
}

#[test]
fn select_deduplicates_repeated_explicit_names() {
    let tmp = TempDir::new().unwrap();
    let selected = select(
        tmp.path(),
        &["claude".to_string(), "claude".to_string()],
        SelectMode::Multi,
    )
    .unwrap();
    assert_eq!(selected.len(), 1);
}
