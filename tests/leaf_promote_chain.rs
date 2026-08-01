use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::thread;
use std::time::{Duration, Instant};
use tempfile::TempDir;

fn grove(kind: &str) -> (TempDir, PathBuf) {
    let tmp = TempDir::new().unwrap();
    std::process::Command::new("git")
        .args(["init", "-q"])
        .arg(tmp.path())
        .status()
        .unwrap();
    let root = tmp.path().join(".grove");
    fs::create_dir_all(&root).unwrap();
    fs::write(root.join("BRIEF.md"), "# root — brief\n").unwrap();
    let producer = root.join("01-sync-k1.md");
    fs::write(
        &producer,
        format!("# sync-k1\n\n**Kind:** {kind}\n\n## Goal\n\nKeep these bytes.\n"),
    )
    .unwrap();
    (tmp, producer)
}

fn run(worktree: &Path, args: &[&str]) -> (String, String, bool) {
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(worktree)
        .args(args)
        .output()
        .unwrap();
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.success(),
    )
}

fn run_with_env(worktree: &Path, args: &[&str], env: &[(&str, &str)]) -> (String, String, bool) {
    let mut command = std::process::Command::new(env!("CARGO_BIN_EXE_grove-llm"));
    command.current_dir(worktree).args(args);
    for (name, value) in env {
        command.env(name, value);
    }
    let output = command.output().unwrap();
    (
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
        output.status.success(),
    )
}

fn wait_for(path: &Path) {
    let deadline = Instant::now() + Duration::from_secs(5);
    while !path.exists() {
        assert!(
            Instant::now() < deadline,
            "timed out waiting for {}",
            path.display()
        );
        thread::sleep(Duration::from_millis(10));
    }
}

fn vcs(worktree: &Path, binary: &str, args: &[&str]) -> String {
    let output = std::process::Command::new(binary)
        .current_dir(worktree)
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("running {binary} {args:?}: {error}"));
    assert!(
        output.status.success(),
        "{binary} {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn promotion_preserves_the_producer_and_builds_a_related_review_chain() {
    let (tmp, producer) = grove("design");
    let original = fs::read(&producer).unwrap();

    let (stdout, stderr, ok) = run(
        tmp.path(),
        &["leaf-promote-chain", producer.to_str().unwrap()],
    );

    assert!(ok, "promotion failed: {stderr}");
    let paths: Vec<PathBuf> = stdout.lines().map(PathBuf::from).collect();
    assert_eq!(paths.len(), 4, "node, producer, review, integration");
    assert!(paths.iter().all(|path| path.is_absolute()));
    assert!(paths[0].ends_with("01-sync-chain-k2"), "{paths:?}");
    assert!(paths[1].ends_with("01-sync-chain-k2/01-sync-k1.md"));
    assert!(paths[2].ends_with("01-sync-chain-k2/02-sync-review-k3.md"));
    assert!(paths[3].ends_with("01-sync-chain-k2/03-sync-integrate-k4.md"));
    assert_eq!(fs::read(&paths[1]).unwrap(), original);
    assert!(!paths[0].join("BRIEF.md").exists());

    let review = fs::read_to_string(&paths[2]).unwrap();
    assert!(review.contains("**Kind:** review-design"), "{review}");
    assert!(review.contains("**Reviews:** sync-k1"), "{review}");
    let integration = fs::read_to_string(&paths[3]).unwrap();
    assert!(
        integration.contains("**Kind:** integrate-review-design"),
        "{integration}"
    );
    assert!(
        integration.contains("**Integrates:** sync-review-k3"),
        "{integration}"
    );
}

#[test]
fn promotion_is_idempotent_by_stale_path_and_json_reports_unchanged_handles() {
    let (tmp, producer) = grove("impl");
    let stale = producer.to_string_lossy().into_owned();
    let (_, stderr, ok) = run(tmp.path(), &["leaf-promote-chain", &stale]);
    assert!(ok, "first promotion failed: {stderr}");

    let (stdout, stderr, ok) = run(tmp.path(), &["leaf-promote-chain", &stale, "--json"]);

    assert!(ok, "idempotent retry failed: {stderr}");
    assert!(stdout.starts_with('{') && stdout.trim_end().ends_with('}'));
    assert!(stdout.contains("\"changed\":false"), "{stdout}");
    for handle in [
        "sync-chain-k2",
        "sync-k1",
        "sync-review-k3",
        "sync-integrate-k4",
    ] {
        assert!(stdout.contains(handle), "missing {handle}: {stdout}");
    }
}

#[test]
fn promotion_strictly_refuses_non_producers_and_garbled_kinds_without_writing() {
    for kind in ["research", "review-impl", "impll", ""] {
        let (tmp, producer) = grove(kind);
        let (stdout, stderr, ok) = run(
            tmp.path(),
            &["leaf-promote-chain", producer.to_str().unwrap()],
        );
        assert!(!ok, "kind {kind:?} unexpectedly promoted");
        assert_eq!(stdout, "");
        assert!(producer.exists(), "source changed for {kind:?}: {stderr}");
        assert_eq!(
            fs::read_dir(tmp.path().join(".grove"))
                .unwrap()
                .filter_map(Result::ok)
                .count(),
            2,
            "no transaction or chain for {kind:?}: {stderr}"
        );
    }
}

#[test]
fn a_pending_transaction_fails_every_other_reader_closed() {
    let (tmp, _) = grove("impl");
    let pending = tmp.path().join(".grove/PROMOTING-01-sync-chain-k2");
    fs::create_dir(&pending).unwrap();

    for args in [vec!["pick"], vec!["resolve", "sync-k1"]] {
        let (stdout, stderr, ok) = run(tmp.path(), &args);
        assert!(!ok, "reader {args:?} skipped the pending transaction");
        assert_eq!(stdout, "");
        assert!(stderr.contains(pending.to_str().unwrap()), "{stderr}");
        assert!(stderr.contains("leaf-promote-chain"), "{stderr}");
    }
}

#[test]
fn leaf_add_chain_writes_the_same_stable_relationships_proactively() {
    let (tmp, producer) = grove("impl");
    fs::remove_file(producer).unwrap();

    let (stdout, stderr, ok) = run(
        tmp.path(),
        &["leaf-add-chain", ".", "sync", "--kind", "design"],
    );

    assert!(ok, "chain creation failed: {stderr}");
    let paths: Vec<PathBuf> = stdout.lines().map(PathBuf::from).collect();
    let review = fs::read_to_string(&paths[2]).unwrap();
    let integration = fs::read_to_string(&paths[3]).unwrap();
    assert!(review.contains("**Reviews:** sync-k2"), "{review}");
    assert!(
        integration.contains("**Integrates:** sync-review-k3"),
        "{integration}"
    );
}

#[test]
fn promotion_is_listed_in_the_llm_help_but_not_the_human_cli() {
    let llm = run(Path::new("."), &["--help"]);
    assert!(llm.2);
    assert!(llm.0.contains("leaf-promote-chain"), "{}", llm.0);

    let out = Command::cargo_bin("grove")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let help = String::from_utf8_lossy(&out.stdout);
    assert!(!help.contains("leaf-promote-chain"), "{help}");
}

#[test]
fn tracked_git_lands_only_the_producer_at_its_final_index_path() {
    let (tmp, producer) = grove("impl");
    vcs(
        tmp.path(),
        "git",
        &["config", "user.email", "t@example.com"],
    );
    vcs(tmp.path(), "git", &["config", "user.name", "Test"]);
    vcs(tmp.path(), "git", &["add", ".grove/01-sync-k1.md"]);
    vcs(tmp.path(), "git", &["commit", "-q", "-m", "fixture"]);
    vcs(
        tmp.path(),
        "git",
        &[
            "update-index",
            "--assume-unchanged",
            "--",
            ".grove/01-sync-k1.md",
        ],
    );
    let before = vcs(
        tmp.path(),
        "git",
        &["ls-files", "--stage", "--", ".grove/01-sync-k1.md"],
    );
    let before_mode = before.split_whitespace().next().unwrap().to_string();

    let (stdout, stderr, ok) = run(
        tmp.path(),
        &["leaf-promote-chain", producer.to_str().unwrap()],
    );

    assert!(ok, "tracked promotion failed: {stderr}");
    let index = vcs(tmp.path(), "git", &["ls-files", "--stage"]);
    assert!(!index.contains("PROMOTING-"), "{index}");
    assert!(
        index.contains(".grove/01-sync-chain-k2/01-sync-k1.md"),
        "{index}"
    );
    assert!(!index.contains("02-sync-review-k3.md"), "{index}");
    assert!(!index.contains("03-sync-integrate-k4.md"), "{index}");
    let final_line = index
        .lines()
        .find(|line| line.contains("01-sync-k1.md"))
        .unwrap();
    assert_eq!(final_line.split_whitespace().next().unwrap(), before_mode);
    let flags = vcs(
        tmp.path(),
        "git",
        &[
            "ls-files",
            "-v",
            "--",
            ".grove/01-sync-chain-k2/01-sync-k1.md",
        ],
    );
    assert!(
        flags.chars().next().unwrap().is_ascii_lowercase(),
        "assume-unchanged was not preserved: {flags:?}; stdout={stdout}"
    );
}

#[test]
fn an_empty_pending_witness_recovers_by_its_exact_path() {
    let (tmp, _) = grove("design");
    let pending = tmp.path().join(".grove/PROMOTING-01-sync-chain-k2");
    fs::create_dir(&pending).unwrap();

    let (stdout, stderr, ok) = run(
        tmp.path(),
        &["leaf-promote-chain", pending.to_str().unwrap()],
    );

    assert!(ok, "recovery failed: {stderr}");
    assert_eq!(stdout.lines().count(), 4);
    assert!(!pending.exists());
    assert!(tmp
        .path()
        .join(".grove/01-sync-chain-k2/01-sync-k1.md")
        .exists());
}

#[test]
fn a_colocated_jj_tree_moves_files_without_touching_the_git_index() {
    let (tmp, producer) = grove("impl");
    vcs(
        tmp.path(),
        "git",
        &["config", "user.email", "t@example.com"],
    );
    vcs(tmp.path(), "git", &["config", "user.name", "Test"]);
    vcs(tmp.path(), "git", &["add", ".grove/01-sync-k1.md"]);
    vcs(tmp.path(), "git", &["commit", "-q", "-m", "fixture"]);
    vcs(
        tmp.path(),
        "jj",
        &[
            "--config",
            "user.name=Test",
            "--config",
            "user.email=t@example.com",
            "git",
            "init",
            "--colocate",
            "--quiet",
            ".",
        ],
    );

    let (_, stderr, ok) = run(
        tmp.path(),
        &["leaf-promote-chain", producer.to_str().unwrap()],
    );

    assert!(ok, "colocated promotion failed: {stderr}");
    let index = vcs(tmp.path(), "git", &["ls-files"]);
    assert!(index.lines().any(|line| line == ".grove/01-sync-k1.md"));
    assert!(!index.contains("01-sync-chain-k2"), "{index}");
    assert!(tmp
        .path()
        .join(".grove/01-sync-chain-k2/01-sync-k1.md")
        .exists());
}

#[test]
fn completed_shape_retry_stays_idempotent_after_the_producer_retires() {
    let (tmp, producer) = grove("impl");
    let stale = producer.to_string_lossy().into_owned();
    let (stdout, stderr, ok) = run(tmp.path(), &["leaf-promote-chain", &stale]);
    assert!(ok, "promotion failed: {stderr}");
    let relocated = stdout.lines().nth(1).unwrap().to_string();
    let (_, stderr, ok) = run(tmp.path(), &["leaf-retire", &relocated]);
    assert!(ok, "retirement failed: {stderr}");

    let (stdout, stderr, ok) = run(tmp.path(), &["leaf-promote-chain", &stale, "--json"]);

    assert!(ok, "terminal idempotent retry failed: {stderr}");
    assert!(stdout.contains("\"changed\":false"), "{stdout}");
    assert!(stdout.contains("01-DONE-sync-k1.md"), "{stdout}");
}

#[test]
fn reported_failures_roll_back_without_consuming_a_key() {
    for checkpoint in [
        "after-transaction-created",
        "after-generated-steps",
        "after-producer-move",
        "after-index-prepare",
    ] {
        let (tmp, producer) = grove("impl");
        let (stdout, stderr, ok) = run_with_env(
            tmp.path(),
            &["leaf-promote-chain", producer.to_str().unwrap()],
            &[("GROVE_TEST_PROMOTION_FAIL_AT", checkpoint)],
        );
        assert!(!ok, "checkpoint {checkpoint} unexpectedly succeeded");
        assert_eq!(stdout, "", "failed promotion printed a path");
        assert!(stderr.contains(checkpoint), "{stderr}");
        assert!(producer.exists(), "producer not restored at {checkpoint}");
        assert!(
            fs::read_dir(tmp.path().join(".grove"))
                .unwrap()
                .filter_map(Result::ok)
                .all(|entry| !entry
                    .file_name()
                    .to_string_lossy()
                    .starts_with("PROMOTING-")),
            "transaction remained after reported failure at {checkpoint}"
        );

        let (added, stderr, ok) = run(tmp.path(), &["leaf-add", ".", "later"]);
        assert!(ok, "leaf-add after rollback failed: {stderr}");
        assert!(added.trim().ends_with("02-later-k2.md"), "{added}");
    }
}

#[test]
fn a_serialized_second_promoter_waits_then_returns_the_completed_shape() {
    let (tmp, producer) = grove("impl");
    let barrier = tmp.path().join("promotion-barrier");
    let stale = producer.to_string_lossy().into_owned();
    let first = std::process::Command::new(env!("CARGO_BIN_EXE_grove-llm"))
        .current_dir(tmp.path())
        .args(["leaf-promote-chain", &stale])
        .env("GROVE_TEST_PROMOTION_PAUSE_AT", "after-transaction-created")
        .env("GROVE_TEST_PROMOTION_BARRIER", &barrier)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    wait_for(&barrier);

    let second = std::process::Command::new(env!("CARGO_BIN_EXE_grove-llm"))
        .current_dir(tmp.path())
        .args(["leaf-promote-chain", &stale, "--json"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    thread::sleep(Duration::from_millis(100));
    fs::remove_file(&barrier).unwrap();

    let first_output = first.wait_with_output().unwrap();
    let second_output = second.wait_with_output().unwrap();
    assert!(
        first_output.status.success(),
        "{}",
        String::from_utf8_lossy(&first_output.stderr)
    );
    assert!(
        second_output.status.success(),
        "{}",
        String::from_utf8_lossy(&second_output.stderr)
    );
    let second_stdout = String::from_utf8_lossy(&second_output.stdout);
    let second_stderr = String::from_utf8_lossy(&second_output.stderr);
    assert!(
        second_stdout.contains("\"changed\":false"),
        "{second_stdout}"
    );
    assert!(
        second_stderr
            .matches("waiting for active Grove tree operation")
            .count()
            == 1,
        "{second_stderr}"
    );
}

#[test]
fn killing_after_the_producer_move_leaves_a_blocking_recoverable_witness() {
    let (tmp, producer) = grove("design");
    let barrier = tmp.path().join("promotion-barrier");
    let mut child = std::process::Command::new(env!("CARGO_BIN_EXE_grove-llm"))
        .current_dir(tmp.path())
        .args(["leaf-promote-chain", producer.to_str().unwrap()])
        .env("GROVE_TEST_PROMOTION_PAUSE_AT", "after-producer-move")
        .env("GROVE_TEST_PROMOTION_BARRIER", &barrier)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    wait_for(&barrier);
    child.kill().unwrap();
    child.wait().unwrap();

    let pending = fs::read_dir(tmp.path().join(".grove"))
        .unwrap()
        .filter_map(Result::ok)
        .find(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .starts_with("PROMOTING-")
        })
        .unwrap()
        .path();
    let (_, stderr, ok) = run(tmp.path(), &["pick"]);
    assert!(
        !ok && stderr.contains(pending.to_str().unwrap()),
        "{stderr}"
    );

    let (stdout, stderr, ok) = run(
        tmp.path(),
        &["leaf-promote-chain", pending.to_str().unwrap()],
    );
    assert!(ok, "recovery failed: {stderr}");
    assert_eq!(stdout.lines().count(), 4);
    assert!(!pending.exists());
    let (picked, stderr, ok) = run(tmp.path(), &["pick"]);
    assert!(ok, "pick after recovery failed: {stderr}");
    assert!(picked.trim().ends_with("01-sync-chain-k2/01-sync-k1.md"));
}

#[test]
fn tracked_git_recovers_after_the_index_was_prepared_but_before_landing() {
    let (tmp, producer) = grove("impl");
    vcs(
        tmp.path(),
        "git",
        &["config", "user.email", "t@example.com"],
    );
    vcs(tmp.path(), "git", &["config", "user.name", "Test"]);
    vcs(tmp.path(), "git", &["add", ".grove/01-sync-k1.md"]);
    vcs(tmp.path(), "git", &["commit", "-q", "-m", "fixture"]);
    let barrier = tmp.path().join("promotion-barrier");
    let mut child = std::process::Command::new(env!("CARGO_BIN_EXE_grove-llm"))
        .current_dir(tmp.path())
        .args(["leaf-promote-chain", producer.to_str().unwrap()])
        .env("GROVE_TEST_PROMOTION_PAUSE_AT", "after-index-prepare")
        .env("GROVE_TEST_PROMOTION_BARRIER", &barrier)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    wait_for(&barrier);
    child.kill().unwrap();
    child.wait().unwrap();
    let pending = fs::read_dir(tmp.path().join(".grove"))
        .unwrap()
        .filter_map(Result::ok)
        .find(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .starts_with("PROMOTING-")
        })
        .unwrap()
        .path();
    let before = vcs(tmp.path(), "git", &["ls-files", "--stage"]);
    assert!(!before.contains("PROMOTING-"), "{before}");
    assert!(
        before.contains("01-sync-chain-k2/01-sync-k1.md"),
        "{before}"
    );

    let (_, stderr, ok) = run(
        tmp.path(),
        &["leaf-promote-chain", pending.to_str().unwrap()],
    );

    assert!(ok, "recovery failed: {stderr}");
    let after = vcs(tmp.path(), "git", &["ls-files", "--stage"]);
    assert!(!after.contains("PROMOTING-"), "{after}");
    assert!(after.contains("01-sync-chain-k2/01-sync-k1.md"), "{after}");
    assert!(!pending.exists());
}

#[test]
fn a_proactively_scheduled_chain_is_refused_instead_of_nested_or_claimed() {
    let (tmp, producer) = grove("impl");
    fs::remove_file(producer).unwrap();
    let (stdout, stderr, ok) = run(
        tmp.path(),
        &["leaf-add-chain", ".", "sync", "--kind", "impl"],
    );
    assert!(ok, "fixture chain failed: {stderr}");
    let producer = stdout.lines().nth(1).unwrap().to_string();
    let before = fs::read_dir(tmp.path().join(".grove/01-sync-chain-k1"))
        .unwrap()
        .filter_map(Result::ok)
        .count();

    let (stdout, stderr, ok) = run(tmp.path(), &["leaf-promote-chain", &producer]);

    assert!(!ok);
    assert_eq!(stdout, "");
    assert!(
        stderr.contains("already has scheduled review work"),
        "{stderr}"
    );
    assert_eq!(
        fs::read_dir(tmp.path().join(".grove/01-sync-chain-k1"))
            .unwrap()
            .filter_map(Result::ok)
            .count(),
        before
    );
}

#[test]
fn root_without_a_brief_is_not_misclassified_as_composition_managed() {
    let (tmp, producer) = grove("prototype");
    fs::remove_file(tmp.path().join(".grove/BRIEF.md")).unwrap();

    let (_, stderr, ok) = run(
        tmp.path(),
        &["leaf-promote-chain", producer.to_str().unwrap()],
    );

    assert!(
        ok,
        "root-level producer should promote without a root brief: {stderr}"
    );
}

#[test]
fn strict_read_accepts_all_five_producers_and_the_legacy_work_alias() {
    for (kind, review_kind) in [
        ("requirements", "review-requirements"),
        ("design", "review-design"),
        ("planning", "review-planning"),
        ("prototype", "review-prototype"),
        ("impl", "review-impl"),
        ("work", "review-impl"),
    ] {
        let (tmp, producer) = grove(kind);
        let (stdout, stderr, ok) = run(
            tmp.path(),
            &["leaf-promote-chain", producer.to_str().unwrap()],
        );
        assert!(ok, "{kind} failed: {stderr}");
        let review_path = stdout.lines().nth(2).unwrap();
        let review = fs::read_to_string(review_path).unwrap();
        assert!(
            review.contains(&format!("**Kind:** {review_kind}")),
            "{review}"
        );
    }
}

#[test]
fn native_jj_promotion_needs_no_git_repository() {
    let tmp = TempDir::new().unwrap();
    vcs(
        tmp.path(),
        "jj",
        &[
            "--config",
            "user.name=Test",
            "--config",
            "user.email=t@example.com",
            "--config",
            "git.colocate=false",
            "git",
            "init",
            "--quiet",
            ".",
        ],
    );
    let root = tmp.path().join(".grove");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("BRIEF.md"), "# root — brief\n").unwrap();
    let producer = root.join("01-sync-k1.md");
    fs::write(&producer, "# sync-k1\n\n**Kind:** impl\n").unwrap();

    let (_, stderr, ok) = run(
        tmp.path(),
        &["leaf-promote-chain", producer.to_str().unwrap()],
    );

    assert!(ok, "native jj promotion failed: {stderr}");
    assert!(!tmp.path().join(".git").exists());
    assert!(root.join("01-sync-chain-k2/01-sync-k1.md").exists());
}

#[test]
fn tracked_git_reported_failure_restores_the_original_index_path() {
    let (tmp, producer) = grove("impl");
    vcs(
        tmp.path(),
        "git",
        &["config", "user.email", "t@example.com"],
    );
    vcs(tmp.path(), "git", &["config", "user.name", "Test"]);
    vcs(tmp.path(), "git", &["add", ".grove/01-sync-k1.md"]);
    vcs(tmp.path(), "git", &["commit", "-q", "-m", "fixture"]);

    let (stdout, _, ok) = run_with_env(
        tmp.path(),
        &["leaf-promote-chain", producer.to_str().unwrap()],
        &[("GROVE_TEST_PROMOTION_FAIL_AT", "after-index-prepare")],
    );

    assert!(!ok);
    assert_eq!(stdout, "");
    assert!(producer.exists());
    let index = vcs(tmp.path(), "git", &["ls-files", "--stage"]);
    assert!(index.contains(".grove/01-sync-k1.md"), "{index}");
    assert!(
        !index.contains("PROMOTING-") && !index.contains("sync-chain"),
        "{index}"
    );
}
