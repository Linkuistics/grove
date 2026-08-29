//! Retiring or pruning a producer that a review names: what may change, and
//! what may not.
//!
//! This file used to assert that neither verb materialised a
//! `**Producer launch:**` receipt into the review that consumes the producer.
//! With the receipt format deleted, that phrasing is no longer a claim anything
//! could violate — a `!contains("**Producer launch:**")` assertion would pass
//! forever, including on the day some *other* sibling write is introduced, and
//! would name a marker that survives only inside migration's stripper.
//!
//! So the claim is stated by enumeration instead, over the whole `.grove/`
//! subtree: retirement and pruning are **filename-only** operations on their
//! operand. Every other path keeps its bytes, and the operand keeps its own.
//! That subsumes the receipt claim (a receipt was a sibling write), it subsumes
//! producer generations and source sessions with it, and unlike the negative it
//! fails on a sibling write nobody has thought of yet.
//!
//! The positive control matters as much as the sweep: a comparison that cannot
//! observe a difference is not evidence, so each test also asserts the one
//! change it *does* expect, by exact path.

use assert_cmd::Command;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

mod support;

fn init_repo() -> TempDir {
    let tmp = TempDir::new().unwrap();
    support::init_jj_repo(tmp.path());
    tmp
}

fn write(path: &Path, body: &str) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, body).unwrap();
}

const REVIEW_BODY: &str =
    "# build-k2\n\n**Reviews:** build-k1\n\n## Goal\n\nReview the implementation.\n";

/// A committed review chain — flat siblings off one stem (flat-lazy-review):
/// producer, the review that names it, and the integration that names the
/// review, each cut by the session before it.
fn build_review_chain(repo: &Path) -> PathBuf {
    let grove = repo.join(".grove");
    let producer = grove.join("01-impl-build-k1.md");
    write(&grove.join("BRIEF.md"), "# build — brief\n");
    write(&producer, "# build-k1\n\n## Goal\n\nBuild it.\n");
    write(&grove.join("02-review-impl-build-k2.md"), REVIEW_BODY);
    write(
        &grove.join("03-integrate-review-impl-build-k3.md"),
        "# build-k3\n\n**Integrates:** build-k2\n",
    );
    support::jj(repo, &["commit", "-m", "fixture"]);
    producer
}

/// Every file under `.grove/`, keyed by its path relative to the working tree.
fn snapshot(repo: &Path) -> BTreeMap<PathBuf, String> {
    fn walk(dir: &Path, repo: &Path, out: &mut BTreeMap<PathBuf, String>) {
        let mut entries: Vec<PathBuf> = fs::read_dir(dir)
            .unwrap_or_else(|error| panic!("reading {}: {error}", dir.display()))
            .map(|entry| entry.unwrap().path())
            .collect();
        entries.sort();
        for entry in entries {
            if entry.is_dir() {
                walk(&entry, repo, out);
            } else {
                let relative = entry.strip_prefix(repo).unwrap().to_path_buf();
                out.insert(relative, fs::read_to_string(&entry).unwrap());
            }
        }
    }
    let mut out = BTreeMap::new();
    walk(&repo.join(".grove"), repo, &mut out);
    assert!(
        out.len() > 3,
        "the walk found {} files — a mis-scoped snapshot compares nothing",
        out.len()
    );
    out
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

/// The shared assertion: the only difference between the two snapshots is that
/// `from` is now named `to`, with its bytes intact.
fn assert_only_rename(
    before: &BTreeMap<PathBuf, String>,
    after: &BTreeMap<PathBuf, String>,
    from: &str,
    to: &str,
) {
    let mut expected = before.clone();
    let body = expected
        .remove(Path::new(from))
        .unwrap_or_else(|| panic!("fixture never held {from}"));
    expected.insert(PathBuf::from(to), body);

    assert_eq!(
        after.keys().collect::<Vec<_>>(),
        expected.keys().collect::<Vec<_>>(),
        "the mark is filename-only: no path may appear or disappear beyond \
         {from} -> {to}"
    );
    for (path, body) in &expected {
        assert_eq!(
            after.get(path),
            Some(body),
            "{} was rewritten; only the operand's *name* may change",
            path.display()
        );
    }
}

#[test]
fn retiring_a_reviewed_producer_changes_only_its_own_filename() {
    let tmp = init_repo();
    let repo = tmp.path();
    let producer = build_review_chain(repo);
    let before = snapshot(repo);

    let (_, stderr, ok) = llm(repo, &["leaf-retire", producer.to_str().unwrap()]);

    assert!(ok, "retirement failed: {stderr}");
    assert_only_rename(
        &before,
        &snapshot(repo),
        ".grove/01-impl-build-k1.md",
        ".grove/01-DONE-impl-build-k1.md",
    );
    // Named separately so a failure says which half broke: the rename is the
    // control that proves the comparison above can see anything at all.
    assert!(producer.with_file_name("01-DONE-impl-build-k1.md").exists());
    assert_eq!(
        fs::read_to_string(producer.with_file_name("02-review-impl-build-k2.md")).unwrap(),
        REVIEW_BODY
    );
}

#[test]
fn pruning_a_reviewed_producer_changes_only_its_own_filename() {
    let tmp = init_repo();
    let repo = tmp.path();
    let producer = build_review_chain(repo);
    let before = snapshot(repo);

    let (_, stderr, ok) = llm(repo, &["leaf-prune", producer.to_str().unwrap()]);

    assert!(ok, "prune failed: {stderr}");
    assert_only_rename(
        &before,
        &snapshot(repo),
        ".grove/01-impl-build-k1.md",
        ".grove/01-ABANDONED-impl-build-k1.md",
    );
    // Pruning a producer deliberately leaves its review live and next
    // (`src/llm_cli.rs`); this is the fixture that would notice it being
    // silently marked or annotated instead.
    assert!(producer
        .with_file_name("01-ABANDONED-impl-build-k1.md")
        .exists());
    assert!(producer
        .with_file_name("02-review-impl-build-k2.md")
        .exists());
}

/// The snapshot comparison's positive control. `assert_only_rename` is the
/// whole evidence in both tests above, so it has to be shown rejecting the
/// thing it exists to catch: a sibling body that changed as well.
#[test]
fn the_snapshot_comparison_rejects_a_sibling_write() {
    let tmp = init_repo();
    let repo = tmp.path();
    let producer = build_review_chain(repo);
    let before = snapshot(repo);

    let (_, stderr, ok) = llm(repo, &["leaf-retire", producer.to_str().unwrap()]);
    assert!(ok, "retirement failed: {stderr}");
    // Stand in for the write the deleted receipt used to perform.
    let review = producer.with_file_name("02-review-impl-build-k2.md");
    fs::write(
        &review,
        format!("{REVIEW_BODY}**Producer launch:** {{\"producer\":\"build-k1\"}}\n"),
    )
    .unwrap();

    let after = snapshot(repo);
    let panicked = std::panic::catch_unwind(|| {
        assert_only_rename(
            &before,
            &after,
            ".grove/01-impl-build-k1.md",
            ".grove/01-DONE-impl-build-k1.md",
        )
    })
    .is_err();
    assert!(
        panicked,
        "the comparison accepted a rewritten review, so it is not evidence of anything"
    );
}
