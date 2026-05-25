mod common;

use common::{fixture_tarball, StubFetcher};
use grove::cli::InstallArgs;
use grove::install::{run_with_fetcher, Mode};
use std::fs;
use std::process::Command;
use tempfile::TempDir;

fn init_repo_with(claude: bool, codex: bool) -> TempDir {
    let tmp = TempDir::new().unwrap();
    Command::new("git").arg("init").arg(tmp.path()).status().unwrap();
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

#[test]
fn install_writes_content_and_version() {
    let repo = init_repo_with(true, false);
    let args = InstallArgs {
        repo: Some(repo.path().to_path_buf()),
        harnesses: vec![],
        version: Some("v0.1.0".into()),
    };

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

    let args = InstallArgs {
        repo: Some(repo.path().to_path_buf()),
        harnesses: vec![],
        version: Some("v0.1.0".into()),
    };
    let err = run_with_fetcher(&args, Mode::Install, &fetcher_at("v0.1.0")).unwrap_err();
    assert!(err.to_string().contains("already installed"));
}

#[test]
fn update_errors_when_grove_not_present() {
    let repo = init_repo_with(true, false);

    let args = InstallArgs {
        repo: Some(repo.path().to_path_buf()),
        harnesses: vec![],
        version: Some("v0.1.0".into()),
    };
    let err = run_with_fetcher(&args, Mode::Update, &fetcher_at("v0.1.0")).unwrap_err();
    assert!(err.to_string().contains("not installed"));
}

#[test]
fn install_into_both_harnesses_when_both_present() {
    let repo = init_repo_with(true, true);

    let args = InstallArgs {
        repo: Some(repo.path().to_path_buf()),
        harnesses: vec![],
        version: Some("v0.1.0".into()),
    };
    run_with_fetcher(&args, Mode::Install, &fetcher_at("v0.1.0")).unwrap();

    assert!(repo.path().join(".claude/skills/grove/SKILL.md").exists());
    assert!(repo.path().join(".codex/skills/grove/SKILL.md").exists());
}

#[test]
fn install_into_only_claude_when_harness_flag_narrows() {
    let repo = init_repo_with(true, true);

    let args = InstallArgs {
        repo: Some(repo.path().to_path_buf()),
        harnesses: vec!["claude".to_string()],
        version: Some("v0.1.0".into()),
    };
    run_with_fetcher(&args, Mode::Install, &fetcher_at("v0.1.0")).unwrap();

    assert!(repo.path().join(".claude/skills/grove/SKILL.md").exists());
    assert!(!repo.path().join(".codex/skills/grove/SKILL.md").exists());
}
