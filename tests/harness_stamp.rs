// The stamp is how a grove *stays* bound to a harness. Bug being fixed: an
// explicit `--harness` in a single-harness repo wrote no stamp, so the next
// plain `grove do` silently fell back to the detected harness — the exact
// migration hazard for repos that carry a stray `.claude/` after the switch.

use grove::harness::by_name;
use grove::harness_stamp::{maybe_stamp, path, resolve_for_launch};
use std::fs;
use std::process::Command;
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

// T6: an unknown name in the stamp *file* itself (hand-edited, or written by a
// newer grove with a harness this binary doesn't know) must fail loudly, not
// silently fall through to detection.
#[test]
fn unknown_name_in_the_stamp_file_fails_loudly() {
    let repo = TempDir::new().unwrap();
    fs::create_dir_all(repo.path().join(".claude")).unwrap();
    let stamp = path(repo.path(), "g");
    fs::create_dir_all(stamp.parent().unwrap()).unwrap();
    fs::write(&stamp, "lemur\n").unwrap();

    let err = resolve_for_launch(repo.path(), "g", None)
        .unwrap_err()
        .to_string();

    assert!(
        err.contains("lemur") && err.contains(stamp.to_string_lossy().as_ref()),
        "the error must name the bad stamp value and the stamp file (err: {err})"
    );
    assert!(
        err.contains("claude") && err.contains("codex") && err.contains("pi"),
        "the error must list the known harnesses (err: {err})"
    );
}

// `.grove-stamps/` must not dirty `git status` in every migrated repo — the
// migration runbook's step 6 (`docs/superpowers/specs/2026-07-18-codex-pi-
// harness-switch-design.md`) mandates one explicit `grove do --harness` per
// grove, which creates it, and that now happens in the *main* repo, where the
// user is likely mid-work.
#[test]
fn grove_stamps_dir_is_gitignored_by_the_projects_own_gitignore() {
    let gitignore = fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/.gitignore")).unwrap();

    let repo = TempDir::new().unwrap();
    Command::new("git")
        .args(["init", "-q"])
        .arg(repo.path())
        .status()
        .unwrap();
    fs::write(repo.path().join(".gitignore"), &gitignore).unwrap();
    fs::create_dir_all(repo.path().join(".grove-stamps")).unwrap();
    fs::write(repo.path().join(".grove-stamps/some-grove"), "pi\n").unwrap();

    let output = Command::new("git")
        .arg("-C")
        .arg(repo.path())
        .args(["status", "--porcelain"])
        .output()
        .unwrap();
    let status = String::from_utf8_lossy(&output.stdout);

    assert!(
        !status.contains(".grove-stamps"),
        ".grove-stamps/ must be gitignored, not reported as untracked (status: {status})"
    );
}

// T6: an unknown name via the explicit `--harness` flag must fail loudly too
// — the same contract as an unknown stamp, on the other input to
// `resolve_for_launch`.
#[test]
fn unknown_name_via_explicit_harness_fails_loudly() {
    let repo = TempDir::new().unwrap();

    let err = resolve_for_launch(repo.path(), "g", Some("lemur"))
        .unwrap_err()
        .to_string();

    assert!(err.contains("lemur"), "err must name the bad value: {err}");
    assert!(
        err.contains("claude") && err.contains("codex") && err.contains("pi"),
        "the error must list the known harnesses (err: {err})"
    );
}
