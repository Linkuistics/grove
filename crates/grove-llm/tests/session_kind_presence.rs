//! **Presence is per-kind and just-in-time**
//! (`docs/adr/complete-session-configuration.md`).
//!
//! Grove no longer holds a set of kinds to check a configuration against, so the
//! only honest question is about the kind in hand: before writing a leaf of kind
//! K, K must resolve to exactly one complete template read whole out of one
//! file. Asserted here at the CLI boundary, over the five verbs that write a
//! leaf, because *before the tree is mutated* is half of what is being claimed.
//!
//! The other half — that every template rule is still checked eagerly over the
//! whole document — is in `tests/session_config.rs` and in the runner's own
//! suite. `tests/lifecycle_cutover.rs` holds it for the driver's own mutations.

use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

mod support;

/// A `$HOME` declaring exactly `kinds` and nothing else — the point being that
/// an incomplete document is a perfectly valid one.
fn home_declaring(kinds: &[&str]) -> TempDir {
    let home = TempDir::new().unwrap();
    let dir = home.path().join(".config/grove");
    fs::create_dir_all(&dir).unwrap();
    let document: String = kinds
        .iter()
        .map(|kind| format!("{kind} \"true ${{prompt}}\"\n"))
        .collect();
    fs::write(dir.join("config.kdl"), document).unwrap();
    home
}

fn init_repo() -> TempDir {
    let tmp = TempDir::new().unwrap();
    support::init_jj_repo(tmp.path());
    fs::write(tmp.path().join("README"), b"r\n").unwrap();
    support::jj(tmp.path(), &["commit", "-m", "init"]);
    tmp
}

fn run(repo: &Path, home: &Path, args: &[&str]) -> (String, bool) {
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .env("HOME", home)
        .current_dir(repo)
        .args(args)
        .output()
        .unwrap();
    (
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.success(),
    )
}

/// Everything under `.grove/`, so a refusal can be shown to have mutated nothing.
fn snapshot(grove: &Path) -> Vec<(PathBuf, Option<String>)> {
    let mut entries = Vec::new();
    let Ok(reading) = fs::read_dir(grove) else {
        return entries;
    };
    for entry in reading.flatten() {
        let path = entry.path();
        let body = fs::read_to_string(&path).ok();
        entries.push((path, body));
    }
    entries.sort();
    entries
}

fn seed_grove(repo: &Path) -> PathBuf {
    let grove = repo.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("BRIEF.md"), "# fixture — brief\n").unwrap();
    fs::write(grove.join("01-impl--seed-k1.md"), "# seed-k1\n").unwrap();
    grove
}

#[test]
fn leaf_add_refuses_a_kind_no_template_resolves_for_and_mutates_nothing() {
    let repo = init_repo();
    let grove = seed_grove(repo.path());
    let home = home_declaring(&["impl"]);
    let before = snapshot(&grove);

    let (stderr, ok) = run(
        repo.path(),
        home.path(),
        &["leaf-add", "--kind", "design", ".", "shape"],
    );

    assert!(!ok, "the add must be refused: {stderr}");
    assert!(
        stderr.contains("refusing to write a leaf of kind `design`"),
        "{stderr}"
    );
    assert!(stderr.contains("key `design` does not resolve"), "{stderr}");
    assert!(
        stderr.contains(
            &home
                .path()
                .join(".config/grove/config.kdl")
                .display()
                .to_string()
        ),
        "the refusal must name the file that should declare it: {stderr}"
    );
    assert_eq!(snapshot(&grove), before, "the tree was mutated");
}

#[test]
fn leaf_add_lands_when_the_kind_resolves() {
    let repo = init_repo();
    let grove = seed_grove(repo.path());
    let home = home_declaring(&["impl"]);

    let (stderr, ok) = run(
        repo.path(),
        home.path(),
        &["leaf-add", "--kind", "impl", ".", "shape"],
    );

    assert!(ok, "{stderr}");
    assert!(grove.join("02-impl--shape-k2.md").exists());
}

#[test]
fn leaf_insert_asks_the_same_question() {
    let repo = init_repo();
    let grove = seed_grove(repo.path());
    let home = home_declaring(&["impl"]);
    let before = snapshot(&grove);

    let (stderr, ok) = run(
        repo.path(),
        home.path(),
        &["leaf-insert", "--kind", "review-impl", "1", "check"],
    );

    assert!(!ok, "{stderr}");
    assert!(
        stderr.contains("refusing to write a leaf of kind `review-impl`"),
        "{stderr}"
    );
    assert_eq!(snapshot(&grove), before);
}

/// A kind **list** writes several leaves as one unit, so every kind in it has to
/// resolve before any of them lands. The research pair is the shape that made
/// this matter, and since `open-kind-k20` it is spelled as an ordinary add.
#[test]
fn a_kind_list_requires_every_one_of_its_kinds() {
    let repo = init_repo();
    let grove = seed_grove(repo.path());
    let home = home_declaring(&["impl", "research-a", "research-b"]);
    let before = snapshot(&grove);

    let (stderr, ok) = run(
        repo.path(),
        home.path(),
        &[
            "leaf-add",
            ".",
            "survey",
            "--kind",
            "research-a",
            "--kind",
            "research-b",
            "--kind",
            "combine-research",
        ],
    );

    assert!(!ok, "{stderr}");
    assert!(
        stderr.contains("refusing to write a leaf of kind `combine-research`"),
        "{stderr}"
    );
    assert_eq!(snapshot(&grove), before);
}

/// `leaf-decompose`'s first child inherits the decomposed leaf's own kind, so
/// that is the kind the rule is asked about — read off the filename before the
/// mutation, not after it.
#[test]
fn leaf_decompose_asks_about_the_kind_its_first_child_will_carry() {
    let repo = init_repo();
    let grove = seed_grove(repo.path());
    let home = home_declaring(&["design"]);
    let before = snapshot(&grove);

    let (stderr, ok) = run(
        repo.path(),
        home.path(),
        &["leaf-decompose", ".grove/01-impl--seed-k1.md", "first"],
    );

    assert!(!ok, "{stderr}");
    assert!(
        stderr.contains("refusing to write a leaf of kind `impl`"),
        "{stderr}"
    );
    assert_eq!(snapshot(&grove), before);

    // …and `--kind` moves the question onto the kind it names.
    let (stderr, ok) = run(
        repo.path(),
        home.path(),
        &[
            "leaf-decompose",
            "--kind",
            "design",
            ".grove/01-impl--seed-k1.md",
            "first",
        ],
    );
    assert!(ok, "{stderr}");
}

/// A verb's own preconditions still speak first: a presence check that errored
/// on an unreadable kind would replace `leaf-decompose`'s refusal of a brief
/// with a complaint about configuration.
#[test]
fn a_verbs_own_refusal_is_not_replaced_by_a_configuration_complaint() {
    let repo = init_repo();
    let grove = seed_grove(repo.path());
    let home = home_declaring(&["impl"]);

    let (stderr, ok) = run(
        repo.path(),
        home.path(),
        &["leaf-decompose", ".grove/BRIEF.md", "first"],
    );

    assert!(!ok, "{stderr}");
    assert!(
        !stderr.contains("refusing to write a leaf of kind"),
        "the brief refusal must survive: {stderr}"
    );
    assert!(grove.join("BRIEF.md").exists());
}

#[test]
fn root_init_asks_about_the_requirements_leaf_it_mints() {
    let repo = init_repo();
    let home = home_declaring(&["impl"]);

    let (stderr, ok) = run(repo.path(), home.path(), &["root-init", "plan"]);

    assert!(!ok, "{stderr}");
    assert!(
        stderr.contains("refusing to write a leaf of kind `requirements`"),
        "{stderr}"
    );
    assert!(
        !repo.path().join(".grove").exists(),
        "a refused root-init leaves no grove behind"
    );
}
