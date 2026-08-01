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

fn commit_fixture(repo: &Path) {
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
    commit_fixture(repo);
    (producer, review)
}

fn build_decomposed_review_chain(
    repo: &Path,
    closing_kind: &str,
    terminal_review: bool,
) -> (PathBuf, PathBuf, String) {
    let chain = repo.join(".grove/01-build-chain-k10");
    let producer = chain.join("01-build-k1");
    let closing_leaf = producer.join("03-finish-k6.md");
    let review_name = if terminal_review {
        "02-DONE-build-review-k2.md"
    } else {
        "02-build-review-k2.md"
    };
    let review = chain.join(review_name);
    write(
        &producer.join("BRIEF.md"),
        "# build-k1 — brief\n\n## Done when\n\nThe build is finished.\n",
    );
    write(
        &producer.join("01-DONE-highest-key-k9.md"),
        "# highest-key-k9\n\n**Kind:** impl\n",
    );
    write(
        &producer.join("02-DONE-earlier-k5.md"),
        "# earlier-k5\n\n**Kind:** impl\n",
    );
    write(
        &closing_leaf,
        &format!("# finish-k6\n\n**Kind:** {closing_kind}\n"),
    );
    let review_body = "# build-review-k2\n\n**Kind:** review-impl\n**Reviews:** build-k1\n\n## Goal\n\nReview it.\n";
    write(&review, review_body);
    write(
        &chain.join("03-build-integrate-k3.md"),
        "# build-integrate-k3\n\n**Kind:** integrate-review-impl\n**Integrates:** build-review-k2\n",
    );
    commit_fixture(repo);
    (closing_leaf, review, review_body.to_string())
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

fn llm(repo: &Path, args: &[&str]) -> (String, String, bool) {
    let output = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(repo)
        .args(args)
        .env_remove(SESSION_TARGET_ENV)
        .output()
        .unwrap();
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
        "**Reviews:** build-k1\n**Producer launch:** {\"producer\":\"build-k1\",\"session\":\"build-k1\",\"generation\":\"k1\",\"harness\":\"claude\",\"model\":\"opus\"}\n"
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
fn zero_malformed_and_non_leaf_review_claimants_have_explicit_cardinality_semantics() {
    // Zero claimants: this producer simply has no receipt consumer.
    let tmp = init_repo();
    let repo = tmp.path();
    let producer = repo.join(".grove/01-build-k1.md");
    write(&producer, "# build-k1\n\n**Kind:** impl\n");
    commit_fixture(repo);
    let target = session_target(repo, "build-k1");
    let (_, stderr, ok) = retire(repo, &producer, Some(&target));
    assert!(ok, "unreviewed retirement failed: {stderr}");
    assert!(
        !stderr.contains("uncheckable"),
        "zero is not ambiguity: {stderr}"
    );

    // A malformed leaf claimant makes relationship cardinality uncheckable.
    let tmp = init_repo();
    let repo = tmp.path();
    let (producer, review) = build_review_chain(repo, None);
    write(
        &repo.join(".grove/01-build-chain-k4/04-malformed-review-k8.md"),
        "# malformed-review-k8\n\n**Kind:** review-impl\n**Reviews:** not/a/handle\n",
    );
    let target = session_target(repo, "build-k1");
    let (_, stderr, ok) = retire(repo, &producer, Some(&target));
    assert!(ok, "malformed relationship blocked DONE: {stderr}");
    assert!(stderr.contains("uncheckable"), "{stderr}");
    assert!(!fs::read_to_string(review)
        .unwrap()
        .contains("**Producer launch:**"));

    // A node brief is not a claimant: only sibling task leaves can consume a
    // producer receipt, even if hand-edited node metadata says otherwise.
    let tmp = init_repo();
    let repo = tmp.path();
    let (producer, review) = build_review_chain(repo, None);
    write(
        &repo.join(".grove/01-build-chain-k4/04-not-a-claimant-k8/BRIEF.md"),
        "# not-a-claimant-k8 — brief\n\n**Reviews:** build-k1\n",
    );
    let target = session_target(repo, "build-k1");
    let (_, stderr, ok) = retire(repo, &producer, Some(&target));
    assert!(ok, "non-leaf metadata blocked retirement: {stderr}");
    assert!(fs::read_to_string(review)
        .unwrap()
        .contains("**Producer launch:**"));
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

#[test]
fn closing_a_decomposed_producer_records_the_factual_session_and_generation() {
    for closing_kind in ["impl", "review-impl", "integrate-review-impl"] {
        let tmp = init_repo();
        let repo = tmp.path();
        let (closing_leaf, review, _) = build_decomposed_review_chain(repo, closing_kind, false);
        let target = session_target(repo, "finish-k6");

        let (_, stderr, ok) = retire(repo, &closing_leaf, Some(&target));

        assert!(ok, "{closing_kind}: retirement failed: {stderr}");
        let review = fs::read_to_string(review).unwrap();
        assert!(
            review.contains(
                "**Producer launch:** {\"producer\":\"build-k1\",\"session\":\"finish-k6\",\"generation\":\"k9\",\"harness\":\"claude\",\"model\":\"opus\"}"
            ),
            "{closing_kind}: missing decomposed receipt in {review:?}"
        );
        assert!(!stderr.contains("uncheckable"), "{closing_kind}: {stderr}");
    }
}

#[test]
fn a_terminal_linked_review_is_preserved_and_diagnosed() {
    let tmp = init_repo();
    let repo = tmp.path();
    let (closing_leaf, review, original_review) = build_decomposed_review_chain(repo, "impl", true);
    let target = session_target(repo, "finish-k6");

    let (_, stderr, ok) = retire(repo, &closing_leaf, Some(&target));

    assert!(ok, "retirement failed: {stderr}");
    assert_eq!(fs::read_to_string(review).unwrap(), original_review);
    assert!(stderr.contains("review-terminal"), "{stderr}");
}

#[test]
fn one_done_transition_can_close_nested_reviewed_producers() {
    let tmp = init_repo();
    let repo = tmp.path();
    let chain = repo.join(".grove/01-outer-chain-k20");
    let outer = chain.join("01-outer-k1");
    let inner = outer.join("01-inner-k5");
    let closing_leaf = inner.join("02-finish-k9.md");
    let inner_review = outer.join("02-DONE-inner-review-k6.md");
    let outer_review = chain.join("02-outer-review-k2.md");
    write(
        &outer.join("BRIEF.md"),
        "# outer-k1 — brief\n\n## Done when\n\nOuter is complete.\n",
    );
    write(
        &inner.join("BRIEF.md"),
        "# inner-k5 — brief\n\n## Done when\n\nInner is complete.\n",
    );
    write(
        &inner.join("01-DONE-earlier-k8.md"),
        "# earlier-k8\n\n**Kind:** impl\n",
    );
    write(
        &closing_leaf,
        "# finish-k9\n\n**Kind:** integrate-review-impl\n",
    );
    let inner_review_body = "# inner-review-k6\n\n**Kind:** review-impl\n**Reviews:** inner-k5\n";
    write(&inner_review, inner_review_body);
    write(
        &outer.join("03-DONE-inner-integrate-k7.md"),
        "# inner-integrate-k7\n\n**Kind:** integrate-review-impl\n**Integrates:** inner-review-k6\n",
    );
    write(
        &outer_review,
        "# outer-review-k2\n\n**Kind:** review-impl\n**Reviews:** outer-k1\n",
    );
    write(
        &chain.join("03-outer-integrate-k3.md"),
        "# outer-integrate-k3\n\n**Kind:** integrate-review-impl\n**Integrates:** outer-review-k2\n",
    );
    commit_fixture(repo);
    let target = session_target(repo, "finish-k9");

    let (_, stderr, ok) = retire(repo, &closing_leaf, Some(&target));

    assert!(ok, "retirement failed: {stderr}");
    assert_eq!(fs::read_to_string(inner_review).unwrap(), inner_review_body);
    assert!(stderr.contains("review-terminal"), "{stderr}");
    assert!(fs::read_to_string(outer_review).unwrap().contains(
        "**Producer launch:** {\"producer\":\"outer-k1\",\"session\":\"finish-k9\",\"generation\":\"k9\",\"harness\":\"claude\",\"model\":\"opus\"}"
    ));
}

#[test]
fn a_close_cascade_materialises_at_most_one_live_linked_review() {
    let tmp = init_repo();
    let repo = tmp.path();
    let chain = repo.join(".grove/01-outer-chain-k20");
    let outer = chain.join("01-outer-k1");
    let inner = outer.join("01-inner-k5");
    let closing_leaf = inner.join("02-finish-k9.md");
    let inner_review = outer.join("02-inner-review-k6.md");
    let outer_review = chain.join("02-outer-review-k2.md");
    write(&outer.join("BRIEF.md"), "# outer-k1 — brief\n");
    write(&inner.join("BRIEF.md"), "# inner-k5 — brief\n");
    write(
        &inner.join("01-DONE-earlier-k8.md"),
        "# earlier-k8\n\n**Kind:** impl\n",
    );
    write(&closing_leaf, "# finish-k9\n\n**Kind:** impl\n");
    write(
        &inner_review,
        "# inner-review-k6\n\n**Kind:** review-impl\n**Reviews:** inner-k5\n",
    );
    write(
        &outer_review,
        "# outer-review-k2\n\n**Kind:** review-impl\n**Reviews:** outer-k1\n",
    );
    commit_fixture(repo);
    let target = session_target(repo, "finish-k9");

    let (_, stderr, ok) = retire(repo, &closing_leaf, Some(&target));

    assert!(ok, "retirement failed: {stderr}");
    assert!(fs::read_to_string(inner_review).unwrap().contains(
        "**Producer launch:** {\"producer\":\"inner-k5\",\"session\":\"finish-k9\",\"generation\":\"k9\""
    ));
    assert!(
        !fs::read_to_string(outer_review)
            .unwrap()
            .contains("**Producer launch:**"),
        "the live inner review keeps the outer producer open"
    );
}

#[test]
fn producer_generation_survives_reorder_and_changes_after_supported_reopen() {
    let tmp = init_repo();
    let repo = tmp.path();
    let (closing_leaf, review, _) = build_decomposed_review_chain(repo, "impl", false);
    let target = session_target(repo, "finish-k6");
    let (_, stderr, ok) = retire(repo, &closing_leaf, Some(&target));
    assert!(ok, "first close failed: {stderr}");

    let producer = repo.join(".grove/01-build-chain-k10/01-build-k1");
    let highest = producer.join("01-DONE-highest-key-k9.md");
    let earlier = producer.join("02-DONE-earlier-k5.md");
    let swap = producer.join("swap-highest");
    fs::rename(&highest, &swap).unwrap();
    fs::rename(&earlier, producer.join("01-DONE-earlier-k5.md")).unwrap();
    fs::rename(&swap, producer.join("02-DONE-highest-key-k9.md")).unwrap();

    let (stdout, stderr, ok) = llm(
        repo,
        &["kind", "--with-harness", "--json", review.to_str().unwrap()],
    );
    assert!(ok, "reordered evidence failed: {stderr}");
    let evidence: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(evidence["review"]["generation"], "k9");

    let (stdout, stderr, ok) = llm(
        repo,
        &[
            "leaf-add",
            producer.to_str().unwrap(),
            "refinish",
            "--kind",
            "impl",
        ],
    );
    assert!(ok, "supported reopen failed: {stderr}");
    let refinish = PathBuf::from(stdout.trim());
    // Strip only the position prefix; the remaining stable handle includes its key.
    let handle = refinish
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .trim_end_matches(".md")
        .split_once('-')
        .unwrap()
        .1
        .to_string();
    let target = session_target(repo, &handle);
    let (_, stderr, ok) = retire(repo, &refinish, Some(&target));
    assert!(ok, "reclose failed: {stderr}");

    let receipt = fs::read_to_string(review).unwrap();
    assert!(
        receipt.contains(&format!("\"session\":\"{handle}\"")),
        "{receipt}"
    );
    let generation = handle.rsplit_once('-').unwrap().1;
    assert!(
        receipt.contains(&format!("\"generation\":\"{generation}\"")),
        "{receipt}"
    );
    assert!(!receipt.contains("\"generation\":\"k9\""), "{receipt}");
}

#[test]
fn pruning_the_producer_and_pruning_the_enclosing_chain_have_distinct_scope() {
    let tmp = init_repo();
    let repo = tmp.path();
    let (closing_leaf, review, _) = build_decomposed_review_chain(repo, "impl", false);
    let producer = closing_leaf.parent().unwrap();

    let (_, stderr, ok) = llm(repo, &["leaf-prune", producer.to_str().unwrap()]);
    assert!(ok, "producer prune failed: {stderr}");
    assert!(producer.join("03-ABANDONED-finish-k6.md").is_file());
    let (picked, stderr, ok) = llm(repo, &["pick"]);
    assert!(ok, "pick after producer prune failed: {stderr}");
    assert_eq!(PathBuf::from(picked.trim()), review.canonicalize().unwrap());
    let (stdout, stderr, ok) = llm(repo, &["kind", "--with-harness", "--json"]);
    assert!(ok, "review evidence failed: {stderr}");
    let evidence: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(
        evidence["review"],
        serde_json::json!({
            "status": "uncheckable",
            "producer": "build-k1",
            "reason": "producer-receipt-missing"
        })
    );

    let tmp = init_repo();
    let repo = tmp.path();
    let (closing_leaf, review, _) = build_decomposed_review_chain(repo, "impl", false);
    let chain = closing_leaf.parent().unwrap().parent().unwrap();
    let integrate = chain.join("03-build-integrate-k3.md");
    let (_, stderr, ok) = llm(repo, &["leaf-prune", chain.to_str().unwrap()]);
    assert!(ok, "chain prune failed: {stderr}");
    assert!(closing_leaf
        .parent()
        .unwrap()
        .join("03-ABANDONED-finish-k6.md")
        .is_file());
    assert!(review
        .parent()
        .unwrap()
        .join("02-ABANDONED-build-review-k2.md")
        .is_file());
    assert!(integrate
        .parent()
        .unwrap()
        .join("03-ABANDONED-build-integrate-k3.md")
        .is_file());
    let (stdout, stderr, ok) = llm(repo, &["pick"]);
    assert!(
        ok && stdout.is_empty(),
        "chain still has a live pick: {stdout} {stderr}"
    );
}
