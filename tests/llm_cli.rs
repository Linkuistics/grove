// Smoke tests for the `grove-llm` binary's clap wiring. The behavioural
// surface of each verb is exercised against the library API in
// `tests/inboxes.rs`; these tests verify only that the binary dispatches
// the LLM verbs to that library code with no behaviour change after the
// audience-split in ADR-0006.

use assert_cmd::Command;
use std::fs;
use std::path::Path;
use std::process::Command as Pcmd;
use tempfile::TempDir;

fn init_repo() -> TempDir {
    let tmp = TempDir::new().unwrap();
    Pcmd::new("git").arg("init").arg(tmp.path()).status().unwrap();
    git(tmp.path(), &["config", "user.email", "grove-test@example.com"]);
    git(tmp.path(), &["config", "user.name", "Grove Test"]);
    git(tmp.path(), &["config", "core.hooksPath", "/dev/null"]);
    fs::write(tmp.path().join("README"), b"r\n").unwrap();
    git(tmp.path(), &["add", "README"]);
    git(tmp.path(), &["commit", "-m", "init"]);
    tmp
}

fn git(repo: &Path, args: &[&str]) {
    Pcmd::new("git").arg("-C").arg(repo).args(args).status().unwrap();
}

#[test]
fn help_lists_only_llm_verbs() {
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("inbox-add"), "missing inbox-add: {s}");
    assert!(s.contains("inbox-drain"), "missing inbox-drain: {s}");
    // Launcher and admin verbs must not bleed into grove-llm's surface.
    for forbidden in ["start", "continue", "takeover", "install", "update", "meta"] {
        assert!(!s.contains(&format!(" {} ", forbidden)),
                "grove-llm --help should not list `{}`: {s}", forbidden);
    }
}

#[test]
fn grove_help_no_longer_lists_inbox_add_or_drain() {
    let out = Command::cargo_bin("grove")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(!s.contains("inbox-add"), "grove --help leaked inbox-add: {s}");
    assert!(!s.contains("inbox-drain"), "grove --help leaked inbox-drain: {s}");
    // The diagnostic verb stays on the human surface as a nested noun-verb
    // cluster (`grove inbox show`), not flattened.
    assert!(s.contains(" inbox "), "grove --help should still list the `inbox` cluster: {s}");
}

#[test]
fn inbox_add_dispatches_through_grove_llm() {
    let repo = init_repo();
    // Materialise the grove-meta worktree the same way `grove install` does.
    grove::inboxes::materialise(repo.path()).unwrap();

    Command::cargo_bin("grove-llm")
        .unwrap()
        .args([
            "inbox-add",
            "--to=future",
            "--body=observation from grove-llm",
            "--repo",
        ])
        .arg(repo.path())
        .assert()
        .success();

    let dir = repo.path().join(".grove-meta/inboxes/future");
    assert!(dir.is_dir(), "per-grove inbox dir missing");
    let mut obs: Vec<_> = fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("md"))
        .collect();
    obs.sort();
    assert_eq!(obs.len(), 1, "expected one observation, got {obs:?}");
    let body = fs::read_to_string(&obs[0]).unwrap();
    assert_eq!(body, "observation from grove-llm");
}

#[test]
fn inbox_edit_dispatches_through_grove_llm() {
    let repo = init_repo();
    grove::inboxes::materialise(repo.path()).unwrap();
    grove::inboxes::capture(repo.path(), "g", "original", None).unwrap();

    let dir = repo.path().join(".grove-meta/inboxes/g");
    let path = fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .find(|p| p.extension().and_then(|s| s.to_str()) == Some("md"))
        .unwrap();

    Command::cargo_bin("grove-llm")
        .unwrap()
        .arg("inbox-edit")
        .arg(&path)
        .arg("--body=rewritten via grove-llm")
        .args(["--repo"])
        .arg(repo.path())
        .assert()
        .success();

    let obs: Vec<_> = fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("md"))
        .collect();
    assert_eq!(obs.len(), 1, "edit must not add a file, got {obs:?}");
    assert_eq!(fs::read_to_string(&obs[0]).unwrap(), "rewritten via grove-llm");
}

#[test]
fn inbox_drain_enumerate_and_finalize_dispatch_through_grove_llm() {
    let repo = init_repo();
    grove::inboxes::materialise(repo.path()).unwrap();
    grove::inboxes::capture(repo.path(), "g", "alpha", None).unwrap();
    grove::inboxes::capture(repo.path(), "g", "beta", None).unwrap();

    // Phase 1: enumerate prints absolute paths on stdout.
    let enum_out = Command::cargo_bin("grove-llm")
        .unwrap()
        .args(["inbox-drain", "--for=g", "--repo"])
        .arg(repo.path())
        .output()
        .unwrap();
    assert!(enum_out.status.success());
    let listed: Vec<String> = String::from_utf8_lossy(&enum_out.stdout)
        .lines()
        .map(|l| l.to_string())
        .collect();
    assert_eq!(listed.len(), 2, "expected two paths, got {listed:?}");

    // Phase 2: finalize with all-incorporated removes them.
    Command::cargo_bin("grove-llm")
        .unwrap()
        .args(["inbox-drain", "--for=g"])
        .args(["--incorporated", &listed[0]])
        .args(["--incorporated", &listed[1]])
        .args(["--repo"])
        .arg(repo.path())
        .assert()
        .success();

    let dir = repo.path().join(".grove-meta/inboxes/g");
    let remaining: Vec<_> = fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_string_lossy().ends_with(".md"))
        .collect();
    assert!(remaining.is_empty(), "expected empty inbox after drain, got {remaining:?}");
}
