// Producer handoff: retiring a reviewed producer writes its effective launch
// target into the linked review task, but every metadata failure remains
// advisory. These tests drive the real `grove-llm leaf-retire` interface so the
// ordering is observable: `DONE` must land even when no receipt can.

use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;
use tempfile::TempDir;

const SESSION_TARGET_ENV: &str = "GROVE_SESSION_TARGET";

fn init_repo() -> TempDir {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    assert!(ProcessCommand::new("git")
        .args(["init", "-q"])
        .current_dir(repo)
        .status()
        .unwrap()
        .success());
    for args in [
        ["config", "user.email", "grove-test@example.com"],
        ["config", "user.name", "Grove Test"],
        ["config", "core.hooksPath", "/dev/null"],
    ] {
        assert!(ProcessCommand::new("git")
            .args(args)
            .current_dir(repo)
            .status()
            .unwrap()
            .success());
    }
    tmp
}

fn write(path: &Path, body: &str) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, body).unwrap();
}

fn build_review_chain(repo: &Path, existing_receipt: Option<&str>) -> (PathBuf, PathBuf) {
    let chain = repo.join(".grove/01-build-chain-k4");
    let producer = chain.join("01-build-k1.md");
    let review = chain.join("02-build-review-k2.md");
    write(
        &producer,
        "# build-k1\n\n**Kind:** impl\n\n## Goal\n\nBuild it.\n",
    );
    let receipt = existing_receipt
        .map(|line| format!("**Producer launch:** {line}\n"))
        .unwrap_or_default();
    write(
        &review,
        &format!(
            "# build-review-k2\n\n**Kind:** review-impl\n**Reviews:** build-k1\n{receipt}\n## Goal\n\nReview it.\n"
        ),
    );
    write(
        &chain.join("03-build-integrate-k3.md"),
        "# build-integrate-k3\n\n**Kind:** integrate-review-impl\n**Integrates:** build-review-k2\n",
    );
    assert!(ProcessCommand::new("git")
        .args(["add", "-A"])
        .current_dir(repo)
        .status()
        .unwrap()
        .success());
    assert!(ProcessCommand::new("git")
        .args(["commit", "-q", "-m", "fixture"])
        .current_dir(repo)
        .status()
        .unwrap()
        .success());
    (producer, review)
}

fn session_target(repo: &Path, handle: &str) -> String {
    format!(
        "{{\"worktree\":\"{}\",\"handle\":\"{handle}\",\"harness\":\"claude\",\"model\":\"opus\"}}",
        grove::json::escape(&repo.canonicalize().unwrap().display().to_string())
    )
}

fn retire(repo: &Path, producer: &Path, target: Option<&str>) -> (String, String, bool) {
    let mut command = Command::cargo_bin("grove-llm").unwrap();
    command
        .current_dir(repo)
        .arg("leaf-retire")
        .arg(producer)
        .env_remove(SESSION_TARGET_ENV);
    if let Some(target) = target {
        command.env(SESSION_TARGET_ENV, target);
    }
    let output = command.output().unwrap();
    (
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
        output.status.success(),
    )
}

#[test]
fn successful_retirement_materialises_the_producer_launch_receipt() {
    let tmp = init_repo();
    let repo = tmp.path();
    let (producer, review) = build_review_chain(repo, None);
    let target = session_target(repo, "build-k1");

    let (_, stderr, ok) = retire(repo, &producer, Some(&target));

    assert!(ok, "retirement failed: {stderr}");
    assert!(repo
        .join(".grove/01-build-chain-k4/01-DONE-build-k1.md")
        .is_file());
    let review = fs::read_to_string(review).unwrap();
    assert!(review.contains(
        "**Reviews:** build-k1\n**Producer launch:** {\"producer\":\"build-k1\",\"harness\":\"claude\",\"model\":\"opus\"}\n"
    ));
    assert!(
        !stderr.contains("uncheckable"),
        "valid metadata warned: {stderr}"
    );
}

#[test]
fn successful_retirement_unconditionally_replaces_a_prior_receipt() {
    let tmp = init_repo();
    let repo = tmp.path();
    let old = r#"{"producer":"build-k1","harness":"codex","model":"old"}"#;
    let (producer, review) = build_review_chain(repo, Some(old));
    let target = session_target(repo, "build-k1");

    let (_, stderr, ok) = retire(repo, &producer, Some(&target));

    assert!(ok, "retirement failed: {stderr}");
    let review = fs::read_to_string(review).unwrap();
    assert_eq!(review.matches("**Producer launch:**").count(), 1);
    assert!(review.contains("\"harness\":\"claude\",\"model\":\"opus\""));
    assert!(!review.contains("\"harness\":\"codex\""));
}

#[test]
fn missing_or_malformed_session_context_never_blocks_done() {
    for target in [None, Some("not-json")] {
        let tmp = init_repo();
        let repo = tmp.path();
        let (producer, review) = build_review_chain(repo, None);

        let (_, stderr, ok) = retire(repo, &producer, target);

        assert!(ok, "advisory metadata blocked retirement: {stderr}");
        assert!(repo
            .join(".grove/01-build-chain-k4/01-DONE-build-k1.md")
            .is_file());
        assert!(!fs::read_to_string(review)
            .unwrap()
            .contains("**Producer launch:**"));
        assert!(
            stderr.contains("uncheckable"),
            "missing diagnostic: {stderr}"
        );
    }
}

#[test]
fn stale_worktree_routed_handle_or_factual_pick_is_uncheckable() {
    for (case, target_handle, target_worktree, preempt) in [
        ("worktree", "build-k1", Some("/a/different/worktree"), false),
        ("handle", "other-k99", None, false),
        ("pick", "build-k1", None, true),
    ] {
        let tmp = init_repo();
        let repo = tmp.path();
        let (producer, review) = build_review_chain(repo, None);
        if preempt {
            write(
                &repo.join(".grove/00-earlier-k9.md"),
                "# earlier-k9\n\n**Kind:** impl\n",
            );
        }
        let mut target = session_target(repo, target_handle);
        if let Some(other) = target_worktree {
            let current = grove::json::escape(&repo.canonicalize().unwrap().display().to_string());
            target = target.replace(&current, other);
        }

        let (_, stderr, ok) = retire(repo, &producer, Some(&target));

        assert!(ok, "{case}: advisory mismatch blocked retirement: {stderr}");
        assert!(repo
            .join(".grove/01-build-chain-k4/01-DONE-build-k1.md")
            .is_file());
        assert!(!fs::read_to_string(review)
            .unwrap()
            .contains("**Producer launch:**"));
        assert!(stderr.contains("uncheckable"), "{case}: {stderr}");
    }
}

#[test]
fn duplicate_review_claimants_are_uncheckable_but_done_still_lands() {
    let tmp = init_repo();
    let repo = tmp.path();
    let (producer, review) = build_review_chain(repo, None);
    let duplicate = repo.join(".grove/01-build-chain-k4/04-other-review-k8.md");
    write(
        &duplicate,
        "# other-review-k8\n\n**Kind:** review-impl\n**Reviews:** build-k1\n",
    );
    let target = session_target(repo, "build-k1");

    let (_, stderr, ok) = retire(repo, &producer, Some(&target));

    assert!(ok, "duplicate metadata blocked retirement: {stderr}");
    assert!(repo
        .join(".grove/01-build-chain-k4/01-DONE-build-k1.md")
        .is_file());
    assert!(!fs::read_to_string(review)
        .unwrap()
        .contains("**Producer launch:**"));
    assert!(!fs::read_to_string(duplicate)
        .unwrap()
        .contains("**Producer launch:**"));
    assert!(
        stderr.contains("uncheckable") && stderr.contains("ambiguous"),
        "{stderr}"
    );
}

#[test]
fn a_failed_done_rename_writes_no_new_receipt() {
    let tmp = init_repo();
    let repo = tmp.path();
    let old = r#"{"producer":"build-k1","harness":"codex","model":"old"}"#;
    let (producer, review) = build_review_chain(repo, Some(old));
    write(
        &repo.join(".grove/01-build-chain-k4/01-DONE-build-k1.md"),
        "collision\n",
    );
    let target = session_target(repo, "build-k1");

    let (_, _, ok) = retire(repo, &producer, Some(&target));

    assert!(!ok, "a colliding DONE destination must fail retirement");
    let review = fs::read_to_string(review).unwrap();
    assert!(review.contains("\"harness\":\"codex\",\"model\":\"old\""));
    assert!(!review.contains("\"harness\":\"claude\""));
}
