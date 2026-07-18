// The stamp is how a grove *stays* bound to a harness. Bug being fixed: an
// explicit `--harness` in a single-harness repo wrote no stamp, so the next
// plain `grove do` silently fell back to the detected harness — the exact
// migration hazard for repos that carry a stray `.claude/` after the switch.

use grove::harness::by_name;
use grove::harness_stamp::{maybe_stamp, path, resolve_for_launch};
use std::fs;
use tempfile::TempDir;

#[test]
fn explicit_choice_is_stamped_even_in_a_single_harness_repo() {
    let repo = TempDir::new().unwrap();
    fs::create_dir_all(repo.path().join(".claude")).unwrap();

    let pi = by_name("pi").unwrap();
    maybe_stamp(repo.path(), "g", pi, true).unwrap();

    assert_eq!(
        fs::read_to_string(path(repo.path(), "g")).unwrap().trim(),
        "pi",
        "an explicit --harness must persist, or the next plain `grove do` \
         silently reverts to the detected harness"
    );
    // ...and the next launch resolves it from the stamp, not detection.
    let resolved = resolve_for_launch(repo.path(), "g", None).unwrap();
    assert_eq!(resolved.name, "pi");
}

#[test]
fn detected_choice_in_a_single_harness_repo_still_writes_no_stamp() {
    let repo = TempDir::new().unwrap();
    fs::create_dir_all(repo.path().join(".claude")).unwrap();

    let claude = by_name("claude").unwrap();
    maybe_stamp(repo.path(), "g", claude, false).unwrap();

    assert!(
        !path(repo.path(), "g").exists(),
        "auto-detected single-harness choice needs no disambiguation stamp"
    );
}

#[test]
fn multi_harness_repo_still_stamps_without_explicit() {
    let repo = TempDir::new().unwrap();
    fs::create_dir_all(repo.path().join(".claude")).unwrap();
    fs::create_dir_all(repo.path().join(".codex")).unwrap();

    let codex = by_name("codex").unwrap();
    maybe_stamp(repo.path(), "g", codex, false).unwrap();

    assert_eq!(
        fs::read_to_string(path(repo.path(), "g")).unwrap().trim(),
        "codex"
    );
}
