// Current session-kind trees keep launch routing out of task bodies. Retiring
// or pruning a producer must preserve relationship prose byte-for-byte and
// must not materialise the legacy `**Producer launch:**` receipt.
//
// There is no ambient session target left to feed these commands: the receipt
// writer read its harness and model from one advisory launch variable, and the
// variable and the writer are both gone. So the claim is unconditional —
// retirement writes no receipt because it has no way to, not because this
// fixture withheld one.

use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;
use tempfile::TempDir;

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

fn build_review_chain(repo: &Path, review_body: &str) -> (PathBuf, PathBuf) {
    let grove = repo.join(".grove");
    let chain = grove.join("01-build-chain-k4");
    let producer = chain.join("01-impl-build-k1.md");
    let review = chain.join("02-review-impl-build-review-k2.md");
    write(&grove.join("FORMAT"), "session-kinds-v1\n");
    write(&grove.join("BRIEF.md"), "# build — brief\n");
    write(&producer, "# build-k1\n\n## Goal\n\nBuild it.\n");
    write(&review, review_body);
    write(
        &chain.join("03-integrate-review-impl-build-integrate-k3.md"),
        "# build-integrate-k3\n\n**Integrates:** build-review-k2\n",
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

fn llm(repo: &Path, args: &[&str]) -> (String, String, bool) {
    let output = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(repo)
        .args(args)
        .output()
        .unwrap();
    (
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
        output.status.success(),
    )
}

#[test]
fn retiring_a_reviewed_producer_writes_no_launch_receipt() {
    let tmp = init_repo();
    let repo = tmp.path();
    let review_body =
        "# build-review-k2\n\n**Reviews:** build-k1\n\n## Goal\n\nReview the implementation.\n";
    let (producer, review) = build_review_chain(repo, review_body);

    let (_, stderr, ok) = llm(repo, &["leaf-retire", producer.to_str().unwrap()]);

    assert!(ok, "retirement failed: {stderr}");
    assert_eq!(fs::read_to_string(&review).unwrap(), review_body);
    assert!(!review_body.contains("**Producer launch:**"));
    assert!(producer.with_file_name("01-DONE-impl-build-k1.md").exists());
    assert!(
        !stderr.contains("receipt"),
        "retirement must not report on a receipt it never attempts: {stderr}"
    );
}

#[test]
fn retiring_preserves_preexisting_legacy_body_metadata_byte_for_byte() {
    let tmp = init_repo();
    let repo = tmp.path();
    let review_body = "# build-review-k2\n\n**Reviews:** build-k1\n**Producer launch:** {\"producer\":\"old-k9\",\"harness\":\"pi\",\"model\":\"old\"}\n";
    let (producer, review) = build_review_chain(repo, review_body);

    let (_, stderr, ok) = llm(repo, &["leaf-retire", producer.to_str().unwrap()]);

    assert!(ok, "retirement failed: {stderr}");
    assert_eq!(fs::read_to_string(review).unwrap(), review_body);
}

#[test]
fn pruning_a_reviewed_producer_writes_no_launch_receipt() {
    let tmp = init_repo();
    let repo = tmp.path();
    let review_body = "# build-review-k2\n\n**Reviews:** build-k1\n";
    let (producer, review) = build_review_chain(repo, review_body);

    let (_, stderr, ok) = llm(repo, &["leaf-prune", producer.to_str().unwrap()]);

    assert!(ok, "prune failed: {stderr}");
    assert_eq!(fs::read_to_string(&review).unwrap(), review_body);
    assert!(producer
        .with_file_name("01-ABANDONED-impl-build-k1.md")
        .exists());
}
