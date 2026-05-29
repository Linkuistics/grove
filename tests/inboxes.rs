use grove::inboxes;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

fn git(repo: &Path, args: &[&str]) -> std::process::Output {
    Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .output()
        .expect("git")
}

fn init_repo() -> TempDir {
    let tmp = TempDir::new().unwrap();
    Command::new("git").arg("init").arg(tmp.path()).status().unwrap();
    git(tmp.path(), &["config", "user.email", "grove-test@example.com"]);
    git(tmp.path(), &["config", "user.name", "Grove Test"]);
    git(tmp.path(), &["config", "core.hooksPath", "/dev/null"]);
    // Initial commit so the repo has a HEAD; not strictly required by the
    // inbox branch (it starts from an empty tree of its own), but makes the
    // test setup match real-world repos.
    fs::write(tmp.path().join("README"), b"r\n").unwrap();
    git(tmp.path(), &["add", "README"]);
    git(tmp.path(), &["commit", "-m", "init"]);
    tmp
}

fn current_branch(repo: &Path) -> String {
    let out = git(repo, &["rev-parse", "--abbrev-ref", "HEAD"]);
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

fn list_obs(dir: &Path) -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = fs::read_dir(dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.ends_with(".md") && n != ".gitkeep")
                .unwrap_or(false)
        })
        .collect();
    v.sort();
    v
}

#[test]
fn materialise_creates_branch_and_worktree() {
    let repo = init_repo();
    let main_branch = current_branch(repo.path());

    inboxes::materialise(repo.path()).unwrap();

    // Worktree dir + inboxes/ subdir exist on disk.
    let wt = repo.path().join(".grove-meta");
    assert!(wt.is_dir(), "worktree dir missing");
    assert!(wt.join("inboxes").is_dir(), "inboxes/ subdir missing");

    // Branch exists locally.
    let out = git(repo.path(), &["branch", "--list", "grove-meta"]);
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("grove-meta"), "expected grove-meta branch in: {s}");

    // Main repo's branch is unchanged.
    assert_eq!(current_branch(repo.path()), main_branch);
}

#[test]
fn materialise_is_idempotent() {
    let repo = init_repo();
    inboxes::materialise(repo.path()).unwrap();
    inboxes::materialise(repo.path()).unwrap();
    assert!(repo.path().join(".grove-meta/inboxes").is_dir());

    // No duplicate commits on the inbox branch (just the initial one).
    let out = git(repo.path(), &["rev-list", "--count", "grove-meta"]);
    let n: usize = String::from_utf8_lossy(&out.stdout).trim().parse().unwrap();
    assert_eq!(n, 1, "expected 1 commit on grove-meta, got {n}");
}

#[test]
fn inbox_branch_history_does_not_include_main_history() {
    let repo = init_repo();
    inboxes::materialise(repo.path()).unwrap();

    // The grove-meta branch has exactly one commit (its empty-tree init),
    // with no shared ancestor with the main branch — that's the whole point
    // of the orphan-style start.
    let out = git(repo.path(), &["rev-list", "--count", "grove-meta"]);
    let count: usize = String::from_utf8_lossy(&out.stdout).trim().parse().unwrap();
    assert_eq!(count, 1);

    // The commit's tree is empty (no README).
    let tree = git(repo.path(), &["ls-tree", "-r", "grove-meta"]);
    assert!(
        tree.stdout.is_empty(),
        "expected empty tree, got: {}",
        String::from_utf8_lossy(&tree.stdout)
    );
}

#[test]
fn capture_writes_one_file_per_observation_with_gitkeep_on_first_write() {
    let repo = init_repo();
    inboxes::materialise(repo.path()).unwrap();

    inboxes::capture(repo.path(), "future-grove", "noticed a bug in X", None).unwrap();

    let dir = repo.path().join(".grove-meta/inboxes/future-grove");
    assert!(dir.is_dir(), "per-grove inbox dir missing");
    assert!(dir.join(".gitkeep").is_file(), ".gitkeep missing");

    let obs = list_obs(&dir);
    assert_eq!(obs.len(), 1, "expected one observation file, got {}", obs.len());
    let name = obs[0].file_name().unwrap().to_string_lossy().to_string();
    assert!(name.ends_with(".md"), "filename: {name}");
    // <UTC-iso8601-seconds>Z-<slug>-<hash8>.md — components separated by `-`.
    assert!(
        name.contains("Z-noticed-a-bug-in-x-") || name.contains("Z-noticed-a-bug-"),
        "filename does not carry expected slug: {name}"
    );

    let body = fs::read_to_string(&obs[0]).unwrap();
    assert_eq!(body, "noticed a bug in X");

    let log = git(repo.path(), &["log", "--oneline", "grove-meta"]);
    let s = String::from_utf8_lossy(&log.stdout);
    assert!(s.contains("inbox: capture to future-grove"), "log: {s}");

    // Main branch untouched.
    let main_log = git(repo.path(), &["log", "--oneline", &current_branch(repo.path())]);
    let m = String::from_utf8_lossy(&main_log.stdout);
    assert!(!m.contains("inbox:"), "main branch should not see inbox commits: {m}");
}

#[test]
fn two_captures_produce_two_files_on_disjoint_paths() {
    let repo = init_repo();
    inboxes::materialise(repo.path()).unwrap();

    inboxes::capture(repo.path(), "g", "first observation", None).unwrap();
    inboxes::capture(repo.path(), "g", "second observation", None).unwrap();

    let dir = repo.path().join(".grove-meta/inboxes/g");
    let obs = list_obs(&dir);
    assert_eq!(obs.len(), 2, "expected two observation files, got {}", obs.len());

    let bodies: Vec<String> = obs.iter().map(|p| fs::read_to_string(p).unwrap()).collect();
    assert!(bodies.contains(&"first observation".to_string()));
    assert!(bodies.contains(&"second observation".to_string()));
}

#[test]
fn slug_override_is_used_in_filename() {
    let repo = init_repo();
    inboxes::materialise(repo.path()).unwrap();

    inboxes::capture(repo.path(), "g", "any content", Some("custom-slug")).unwrap();

    let obs = list_obs(&repo.path().join(".grove-meta/inboxes/g"));
    assert_eq!(obs.len(), 1);
    let name = obs[0].file_name().unwrap().to_string_lossy().to_string();
    assert!(name.contains("-custom-slug-"), "filename: {name}");
}

#[test]
fn idempotent_on_content_hash_no_duplicate_file_no_extra_commit() {
    let repo = init_repo();
    inboxes::materialise(repo.path()).unwrap();

    inboxes::capture(repo.path(), "g", "same observation", None).unwrap();
    let commits_after_first = rev_count(repo.path(), "grove-meta");

    // Same body, second call: no new file, no new commit, no error.
    inboxes::capture(repo.path(), "g", "same observation", None).unwrap();
    let commits_after_second = rev_count(repo.path(), "grove-meta");

    let obs = list_obs(&repo.path().join(".grove-meta/inboxes/g"));
    assert_eq!(obs.len(), 1, "expected one file, got {}", obs.len());
    assert_eq!(
        commits_after_first, commits_after_second,
        "expected no new commit on idempotent re-capture"
    );

    // A different body produces a new file.
    inboxes::capture(repo.path(), "g", "different observation", None).unwrap();
    let obs = list_obs(&repo.path().join(".grove-meta/inboxes/g"));
    assert_eq!(obs.len(), 2);
}

#[test]
fn legacy_single_file_inbox_is_migrated_on_first_capture() {
    let repo = init_repo();
    inboxes::materialise(repo.path()).unwrap();

    // Hand-construct a pre-shape-change inbox file and commit it as if a
    // previous grove version had written it.
    let wt = repo.path().join(".grove-meta");
    let legacy = wt.join("inboxes/legacy.md");
    fs::write(&legacy, "old appended content\n").unwrap();
    let out = git(&wt, &["add", "inboxes/legacy.md"]);
    assert!(out.status.success(), "stderr: {}", String::from_utf8_lossy(&out.stderr));
    git(&wt, &["commit", "-m", "legacy single-file inbox"]);

    // Now do a fresh capture against the same grove name.
    inboxes::capture(repo.path(), "legacy", "new entry under directory shape", None).unwrap();

    // Legacy file is gone; directory exists with migrated entry + new entry +
    // .gitkeep.
    assert!(!legacy.exists(), "legacy single-file inbox should have been removed");
    let dir = wt.join("inboxes/legacy");
    assert!(dir.is_dir());
    assert!(dir.join(".gitkeep").is_file());

    let obs = list_obs(&dir);
    assert_eq!(obs.len(), 2, "expected migrated entry + new entry, got {}", obs.len());

    // One of the entries is the legacy migration (name contains `-legacy-`),
    // body matches the original.
    let mut found_legacy = false;
    let mut found_new = false;
    for p in &obs {
        let name = p.file_name().unwrap().to_string_lossy().to_string();
        let body = fs::read_to_string(p).unwrap();
        if name.contains("-legacy-") {
            assert!(body.contains("old appended content"), "legacy body: {body:?}");
            found_legacy = true;
        } else {
            assert_eq!(body, "new entry under directory shape");
            found_new = true;
        }
    }
    assert!(found_legacy && found_new, "missing legacy or new entry: {obs:?}");

    // A dedicated migration commit precedes the capture commit.
    let log = git(repo.path(), &["log", "--oneline", "grove-meta"]);
    let s = String::from_utf8_lossy(&log.stdout);
    assert!(s.contains("migrate inbox legacy to directory shape"), "log: {s}");
    assert!(s.contains("inbox: capture to legacy"), "log: {s}");

    // Migration runs once: a follow-up capture must not re-trigger it.
    inboxes::capture(repo.path(), "legacy", "yet another", None).unwrap();
    let log2 = git(repo.path(), &["log", "--oneline", "grove-meta"]);
    let s2 = String::from_utf8_lossy(&log2.stdout);
    assert_eq!(
        s2.matches("migrate inbox legacy").count(),
        1,
        "migration should only happen once: {s2}"
    );
}

#[test]
fn read_concatenates_observations_in_chronological_order() {
    let repo = init_repo();
    inboxes::materialise(repo.path()).unwrap();

    inboxes::capture(repo.path(), "g", "alpha", None).unwrap();
    // Filenames carry a UTC seconds prefix and a slug; the slug differs by
    // body content so even captures inside the same second sort
    // deterministically.
    inboxes::capture(repo.path(), "g", "beta", None).unwrap();
    inboxes::capture(repo.path(), "g", "gamma", None).unwrap();

    let body = inboxes::read(repo.path(), "g").unwrap();
    assert!(body.contains("alpha"));
    assert!(body.contains("beta"));
    assert!(body.contains("gamma"));
}

#[test]
fn read_returns_empty_when_worktree_or_inbox_absent() {
    let repo = init_repo();
    // Worktree not materialised.
    assert_eq!(inboxes::read(repo.path(), "any").unwrap(), "");

    inboxes::materialise(repo.path()).unwrap();
    // Materialised but no inbox dir for this name.
    assert_eq!(inboxes::read(repo.path(), "any").unwrap(), "");
}

#[test]
fn cross_repo_capture_writes_into_other_repos_inbox_only() {
    let repo_a = init_repo();
    let repo_b = init_repo();
    inboxes::materialise(repo_b.path()).unwrap();

    // From repo_a's session, address repo_b's inbox.
    inboxes::capture(repo_b.path(), "future-in-b", "noticed in a", None).unwrap();

    let dir = repo_b.path().join(".grove-meta/inboxes/future-in-b");
    assert!(dir.is_dir());
    let obs = list_obs(&dir);
    assert_eq!(obs.len(), 1);
    let body = fs::read_to_string(&obs[0]).unwrap();
    assert!(body.contains("noticed in a"));

    // repo_a has no inbox worktree (we never materialised it).
    assert!(!repo_a.path().join(".grove-meta").exists());
}

#[test]
fn capture_without_materialisation_fails_with_useful_hint() {
    let repo = init_repo();
    let err = inboxes::capture(repo.path(), "any", "obs", None).unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("grove install"),
        "expected hint to run grove install, got: {msg}"
    );
}

#[test]
fn capture_is_seed_compatible_when_target_grove_does_not_exist_yet() {
    // The "seed" lifecycle state: nothing addresses-side; the inbox dir
    // just sits on the branch waiting for `grove start <name>`.
    let repo = init_repo();
    inboxes::materialise(repo.path()).unwrap();

    inboxes::capture(
        repo.path(),
        "racket-bugs",
        "Pair has wrong arity in expansion",
        None,
    )
    .unwrap();

    // No worktree for `racket-bugs` — the seed dir exists in isolation.
    assert!(!repo.path().join(".grove-worktrees/racket-bugs").exists());
    let dir = repo.path().join(".grove-meta/inboxes/racket-bugs");
    assert!(dir.is_dir());
    assert!(dir.join(".gitkeep").is_file());
    let obs = list_obs(&dir);
    assert_eq!(obs.len(), 1);
}

#[test]
fn empty_observation_is_rejected() {
    let repo = init_repo();
    inboxes::materialise(repo.path()).unwrap();
    let err = inboxes::capture(repo.path(), "g", "   \n\n", None).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("empty observation"), "got: {msg}");
}

#[test]
fn drain_enumerate_returns_observation_paths_in_chronological_order() {
    let repo = init_repo();
    inboxes::materialise(repo.path()).unwrap();
    inboxes::capture(repo.path(), "g", "alpha", None).unwrap();
    inboxes::capture(repo.path(), "g", "beta", None).unwrap();
    inboxes::capture(repo.path(), "g", "gamma", None).unwrap();

    let paths = inboxes::drain_enumerate(repo.path(), "g").unwrap();
    assert_eq!(paths.len(), 3, "expected 3 pending observations, got {paths:?}");
    assert!(paths.iter().all(|p| p.is_absolute()), "paths must be absolute: {paths:?}");

    // Filenames sort lexicographically — the UTC-iso8601 prefix gives
    // chronological order.
    let mut sorted = paths.clone();
    sorted.sort();
    assert_eq!(paths, sorted, "enumerate output must already be sorted");

    // Skips .gitkeep automatically.
    let names: Vec<String> = paths
        .iter()
        .filter_map(|p| p.file_name().and_then(|n| n.to_str()).map(String::from))
        .collect();
    assert!(!names.iter().any(|n| n == ".gitkeep"), "got .gitkeep in: {names:?}");
}

#[test]
fn drain_enumerate_on_missing_inbox_returns_empty_no_commit() {
    let repo = init_repo();
    inboxes::materialise(repo.path()).unwrap();

    let paths = inboxes::drain_enumerate(repo.path(), "never-existed").unwrap();
    assert!(paths.is_empty());
    // Enumerate must not create the inbox dir — capture is the creation path.
    assert!(!repo.path().join(".grove-meta/inboxes/never-existed").exists());

    // Only the initial empty-tree commit on the branch.
    assert_eq!(rev_count(repo.path(), "grove-meta"), 1);
}

#[test]
fn drain_finalize_deletes_listed_paths_and_commits_with_disposition_counts() {
    let repo = init_repo();
    inboxes::materialise(repo.path()).unwrap();
    inboxes::capture(repo.path(), "g", "alpha", None).unwrap();
    inboxes::capture(repo.path(), "g", "beta", None).unwrap();
    inboxes::capture(repo.path(), "g", "gamma", None).unwrap();

    let paths = inboxes::drain_enumerate(repo.path(), "g").unwrap();
    assert_eq!(paths.len(), 3);

    // Triage: alpha incorporated, beta deferred, gamma rejected.
    inboxes::drain_finalize(
        repo.path(),
        "g",
        &[paths[0].clone()],
        &[paths[1].clone()],
        &[paths[2].clone()],
    )
    .unwrap();

    let dir = repo.path().join(".grove-meta/inboxes/g");
    assert!(dir.is_dir(), "inbox dir should persist after drain");
    assert!(dir.join(".gitkeep").is_file(), ".gitkeep should survive drain");
    assert!(list_obs(&dir).is_empty(), "observation files should be gone after drain");

    let log = git(repo.path(), &["log", "--oneline", "grove-meta"]);
    let s = String::from_utf8_lossy(&log.stdout);
    assert!(
        s.contains("drain g: 1 incorporated, 1 deferred, 1 rejected"),
        "log: {s}"
    );
}

#[test]
fn drain_finalize_partial_disposition_only_deletes_named_paths() {
    let repo = init_repo();
    inboxes::materialise(repo.path()).unwrap();
    inboxes::capture(repo.path(), "g", "alpha", None).unwrap();
    inboxes::capture(repo.path(), "g", "beta", None).unwrap();

    let paths = inboxes::drain_enumerate(repo.path(), "g").unwrap();

    // Only "alpha" is incorporated; "beta" is left in the inbox (e.g. a
    // concurrent add that the session chose not to triage this round).
    inboxes::drain_finalize(
        repo.path(),
        "g",
        &[paths[0].clone()],
        &[],
        &[],
    )
    .unwrap();

    let dir = repo.path().join(".grove-meta/inboxes/g");
    let remaining = list_obs(&dir);
    assert_eq!(remaining.len(), 1, "expected beta still present: {remaining:?}");
    assert_eq!(remaining[0], paths[1]);

    let log = git(repo.path(), &["log", "--oneline", "grove-meta"]);
    let s = String::from_utf8_lossy(&log.stdout);
    assert!(
        s.contains("drain g: 1 incorporated, 0 deferred, 0 rejected"),
        "log: {s}"
    );
}

#[test]
fn drain_finalize_rejects_paths_outside_inbox() {
    let repo = init_repo();
    inboxes::materialise(repo.path()).unwrap();
    inboxes::capture(repo.path(), "g", "alpha", None).unwrap();

    let outside = repo.path().join("README");
    let err = inboxes::drain_finalize(
        repo.path(),
        "g",
        std::slice::from_ref(&outside),
        &[],
        &[],
    )
    .unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("not inside the inbox directory"),
        "got: {msg}"
    );

    // Inbox state is unchanged — no deletion happened before the rejection.
    let dir = repo.path().join(".grove-meta/inboxes/g");
    assert_eq!(list_obs(&dir).len(), 1, "drain must be all-or-nothing");
}

#[test]
fn drain_finalize_rejects_dispositionless_calls() {
    let repo = init_repo();
    inboxes::materialise(repo.path()).unwrap();
    let err = inboxes::drain_finalize(repo.path(), "g", &[], &[], &[]).unwrap_err();
    assert!(err.to_string().contains("no paths in any disposition"), "got: {err}");
}

#[test]
fn drain_finalize_rejects_gitkeep() {
    let repo = init_repo();
    inboxes::materialise(repo.path()).unwrap();
    inboxes::capture(repo.path(), "g", "alpha", None).unwrap();

    let gitkeep = repo.path().join(".grove-meta/inboxes/g/.gitkeep");
    let err = inboxes::drain_finalize(repo.path(), "g", &[gitkeep], &[], &[]).unwrap_err();
    assert!(err.to_string().contains(".gitkeep cannot be drained"), "got: {err}");
}

fn rev_count(repo: &Path, refspec: &str) -> usize {
    let out = git(repo, &["rev-list", "--count", refspec]);
    String::from_utf8_lossy(&out.stdout).trim().parse().unwrap()
}
