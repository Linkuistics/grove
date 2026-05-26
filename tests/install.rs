mod common;

use common::{fixture_tarball, StubFetcher};
use grove::cli::InstallArgs;
use grove::install::{run_with_fetcher, Mode};
use std::fs;
use std::path::Path;
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

fn init_repo_with(claude: bool, codex: bool) -> TempDir {
    let tmp = TempDir::new().unwrap();
    Command::new("git").arg("init").arg(tmp.path()).status().unwrap();
    // Local identity so the auto-commit can succeed regardless of global config.
    git(tmp.path(), &["config", "user.email", "grove-test@example.com"]);
    git(tmp.path(), &["config", "user.name", "Grove Test"]);
    // Disable hooks that might run (e.g. host pre-commit).
    git(tmp.path(), &["config", "core.hooksPath", "/dev/null"]);
    if claude {
        fs::create_dir_all(tmp.path().join(".claude")).unwrap();
    }
    if codex {
        fs::create_dir_all(tmp.path().join(".codex")).unwrap();
    }
    tmp
}

fn fetcher_at(tag: &str) -> StubFetcher {
    StubFetcher {
        latest: tag.to_string(),
        tarball: fixture_tarball(
            tag.trim_start_matches('v'),
            &[
                ("content/SKILL.md", b"# SKILL"),
                ("content/prompts/start.md", b"start"),
            ],
        ),
    }
}

fn args_for(repo: &Path) -> InstallArgs {
    InstallArgs {
        repo: Some(repo.to_path_buf()),
        harnesses: vec![],
        version: Some("v0.1.0".into()),
        no_commit: false,
        message: None,
    }
}

fn last_subject(repo: &Path) -> String {
    let out = git(repo, &["log", "-1", "--pretty=%s"]);
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

fn head_count(repo: &Path) -> usize {
    let out = git(repo, &["rev-list", "--count", "HEAD"]);
    String::from_utf8_lossy(&out.stdout)
        .trim()
        .parse()
        .unwrap_or(0)
}

#[test]
fn install_writes_content_and_version() {
    let repo = init_repo_with(true, false);
    let args = args_for(repo.path());

    run_with_fetcher(&args, Mode::Install, &fetcher_at("v0.1.0")).unwrap();

    let dest = repo.path().join(".claude/skills/grove");
    assert_eq!(fs::read_to_string(dest.join("SKILL.md")).unwrap(), "# SKILL");
    assert!(dest.join("VERSION.md").exists());
    assert!(dest.join("prompts/start.md").exists());
}

#[test]
fn install_errors_when_grove_already_present() {
    let repo = init_repo_with(true, false);
    fs::create_dir_all(repo.path().join(".claude/skills/grove")).unwrap();

    let args = args_for(repo.path());
    let err = run_with_fetcher(&args, Mode::Install, &fetcher_at("v0.1.0")).unwrap_err();
    assert!(err.to_string().contains("already installed"));
}

#[test]
fn update_errors_when_grove_not_present() {
    let repo = init_repo_with(true, false);

    let args = args_for(repo.path());
    let err = run_with_fetcher(&args, Mode::Update, &fetcher_at("v0.1.0")).unwrap_err();
    assert!(err.to_string().contains("not installed"));
}

#[test]
fn install_into_both_harnesses_when_both_present() {
    let repo = init_repo_with(true, true);

    let args = args_for(repo.path());
    run_with_fetcher(&args, Mode::Install, &fetcher_at("v0.1.0")).unwrap();

    assert!(repo.path().join(".claude/skills/grove/SKILL.md").exists());
    assert!(repo.path().join(".codex/skills/grove/SKILL.md").exists());
}

#[test]
fn install_into_only_claude_when_harness_flag_narrows() {
    let repo = init_repo_with(true, true);

    let mut args = args_for(repo.path());
    args.harnesses = vec!["claude".to_string()];
    run_with_fetcher(&args, Mode::Install, &fetcher_at("v0.1.0")).unwrap();

    assert!(repo.path().join(".claude/skills/grove/SKILL.md").exists());
    assert!(!repo.path().join(".codex/skills/grove/SKILL.md").exists());
}

// --- auto-commit behavior ---------------------------------------------------

#[test]
fn install_creates_commit_by_default() {
    let repo = init_repo_with(true, false);
    let args = args_for(repo.path());

    run_with_fetcher(&args, Mode::Install, &fetcher_at("v0.1.0")).unwrap();

    assert_eq!(head_count(repo.path()), 1);
    assert_eq!(last_subject(repo.path()), "Install grove v0.1.0");
    // The installed files are tracked.
    let out = git(repo.path(), &["ls-files", ".claude/skills/grove"]);
    let listing = String::from_utf8_lossy(&out.stdout);
    assert!(listing.contains(".claude/skills/grove/SKILL.md"));
    assert!(listing.contains(".claude/skills/grove/VERSION.md"));
}

#[test]
fn update_creates_commit_with_update_message() {
    let repo = init_repo_with(true, false);
    // First install (commit #1).
    run_with_fetcher(&args_for(repo.path()), Mode::Install, &fetcher_at("v0.1.0")).unwrap();
    assert_eq!(head_count(repo.path()), 1);

    // Update to a different version (commit #2).
    let mut args = args_for(repo.path());
    args.version = Some("v0.2.0".into());
    run_with_fetcher(&args, Mode::Update, &fetcher_at("v0.2.0")).unwrap();

    assert_eq!(head_count(repo.path()), 2);
    assert_eq!(last_subject(repo.path()), "Update grove to v0.2.0");
}

#[test]
fn no_commit_flag_skips_commit() {
    let repo = init_repo_with(true, false);
    let mut args = args_for(repo.path());
    args.no_commit = true;

    run_with_fetcher(&args, Mode::Install, &fetcher_at("v0.1.0")).unwrap();

    // Materialisation happened.
    assert!(repo.path().join(".claude/skills/grove/SKILL.md").exists());
    // But no commit was made.
    assert_eq!(head_count(repo.path()), 0);
    // And nothing was staged (we leave the user in full control).
    let status = git(repo.path(), &["status", "--porcelain"]);
    let porcelain = String::from_utf8_lossy(&status.stdout);
    assert!(
        porcelain.lines().all(|l| l.starts_with("??")),
        "expected only untracked entries with --no-commit, got: {porcelain}"
    );
}

#[test]
fn message_flag_overrides_default() {
    let repo = init_repo_with(true, false);
    let mut args = args_for(repo.path());
    args.message = Some("chore(deps): bump grove skill to v0.1.0".into());

    run_with_fetcher(&args, Mode::Install, &fetcher_at("v0.1.0")).unwrap();

    assert_eq!(last_subject(repo.path()), "chore(deps): bump grove skill to v0.1.0");
}

#[test]
fn unrelated_dirty_state_is_preserved() {
    let repo = init_repo_with(true, false);
    // Stage an unrelated change outside the install scope.
    fs::write(repo.path().join("other.txt"), "wip\n").unwrap();
    git(repo.path(), &["add", "other.txt"]);

    run_with_fetcher(&args_for(repo.path()), Mode::Install, &fetcher_at("v0.1.0")).unwrap();

    // The grove commit landed.
    assert_eq!(last_subject(repo.path()), "Install grove v0.1.0");
    // The unrelated file is still staged and was NOT swept into the grove commit.
    let status = git(repo.path(), &["diff", "--cached", "--name-only"]);
    let staged = String::from_utf8_lossy(&status.stdout);
    assert!(staged.contains("other.txt"), "expected other.txt still staged, got: {staged}");

    let show = git(repo.path(), &["show", "--name-only", "--pretty=", "HEAD"]);
    let touched = String::from_utf8_lossy(&show.stdout);
    assert!(
        !touched.contains("other.txt"),
        "grove commit must not include other.txt, got: {touched}"
    );
}

#[test]
fn pre_existing_staged_install_scope_is_refused() {
    let repo = init_repo_with(true, false);
    // Stage something INSIDE the install scope.
    fs::create_dir_all(repo.path().join(".claude/skills/grove")).unwrap();
    fs::write(repo.path().join(".claude/skills/grove/SKILL.md"), "stale\n").unwrap();
    git(repo.path(), &["add", ".claude/skills/grove/SKILL.md"]);

    // It's already present, so we use Update mode (Install would error first on existence).
    let err = run_with_fetcher(&args_for(repo.path()), Mode::Update, &fetcher_at("v0.1.0"))
        .unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("pre-existing staged"),
        "expected refusal error, got: {msg}"
    );

    // And materialisation did NOT happen (the staged content is untouched).
    let on_disk = fs::read_to_string(repo.path().join(".claude/skills/grove/SKILL.md")).unwrap();
    assert_eq!(on_disk, "stale\n");
}

#[test]
fn multi_harness_install_produces_single_combined_commit() {
    let repo = init_repo_with(true, true);

    run_with_fetcher(&args_for(repo.path()), Mode::Install, &fetcher_at("v0.1.0")).unwrap();

    assert_eq!(head_count(repo.path()), 1);
    let show = git(repo.path(), &["show", "--name-only", "--pretty=", "HEAD"]);
    let touched = String::from_utf8_lossy(&show.stdout);
    assert!(touched.contains(".claude/skills/grove/SKILL.md"), "{touched}");
    assert!(touched.contains(".codex/skills/grove/SKILL.md"), "{touched}");
}

#[test]
fn commit_failure_leaves_materialisation_in_place() {
    let repo = init_repo_with(true, false);
    // Install a pre-commit hook that always rejects.
    let hooks = repo.path().join(".githooks");
    fs::create_dir_all(&hooks).unwrap();
    let hook = hooks.join("pre-commit");
    fs::write(&hook, "#!/bin/sh\nexit 1\n").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&hook).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook, perms).unwrap();
    }
    // Point hooksPath at our hook dir (overriding the /dev/null disable in init_repo_with).
    git(repo.path(), &["config", "core.hooksPath", hooks.to_str().unwrap()]);

    let err = run_with_fetcher(&args_for(repo.path()), Mode::Install, &fetcher_at("v0.1.0"))
        .unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("git commit failed"), "got: {msg}");
    assert!(msg.contains("git commit -- "), "follow-up not in msg: {msg}");

    // Materialisation is on disk despite the failure.
    assert!(repo.path().join(".claude/skills/grove/SKILL.md").exists());
    // No commit was produced.
    assert_eq!(head_count(repo.path()), 0);
    // The install scope is left staged for the user to retry.
    let staged = git(repo.path(), &["diff", "--cached", "--name-only"]);
    let staged = String::from_utf8_lossy(&staged.stdout);
    assert!(staged.contains(".claude/skills/grove/SKILL.md"), "{staged}");
}

#[test]
fn noop_update_does_not_create_empty_commit() {
    let repo = init_repo_with(true, false);
    // First install creates one commit.
    run_with_fetcher(&args_for(repo.path()), Mode::Install, &fetcher_at("v0.1.0")).unwrap();
    assert_eq!(head_count(repo.path()), 1);

    // Re-running update with the same tarball produces no diff → no commit.
    run_with_fetcher(&args_for(repo.path()), Mode::Update, &fetcher_at("v0.1.0")).unwrap();
    assert_eq!(
        head_count(repo.path()),
        1,
        "no-op update must not create a second commit"
    );
}
