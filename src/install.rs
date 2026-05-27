use crate::cli::InstallArgs;
use crate::extract::extract_content;
use crate::fetch::{Fetcher, GithubFetcher};
use crate::harness::{self, Harness, SelectMode};
use crate::inboxes;
use crate::repo;
use crate::version_md;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Mode: `Install` errors if grove already present; `Update` errors if absent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Install,
    Update,
}

pub fn run(args: &InstallArgs, mode: Mode) -> Result<()> {
    let fetcher = GithubFetcher::new();
    run_with_fetcher(args, mode, &fetcher)
}

pub fn run_with_fetcher(
    args: &InstallArgs,
    mode: Mode,
    fetcher: &dyn Fetcher,
) -> Result<()> {
    let repo_path = repo::resolve(args.repo.as_deref())?;
    let harnesses = harness::select(&repo_path, &args.harnesses, SelectMode::Multi)?;
    let version = match &args.version {
        Some(v) => v.clone(),
        None => fetcher.latest_version()?,
    };
    eprintln!("grove: target {} @ {}", repo_path.display(), version);

    // Pre-flight: enforce create-only / update-only per mode.
    for h in &harnesses {
        let dest = h.install_path(&repo_path);
        match (mode, dest.exists()) {
            (Mode::Install, true) => anyhow::bail!(
                "grove already installed at {} (use `grove update`)",
                dest.display()
            ),
            (Mode::Update, false) => anyhow::bail!(
                "grove not installed at {} (use `grove install`)",
                dest.display()
            ),
            _ => {}
        }
    }

    // The install scope: every install-path we're about to touch, combined.
    let install_paths: Vec<PathBuf> =
        harnesses.iter().map(|h| h.install_path(&repo_path)).collect();

    // Pre-flight: refuse if anything is already staged inside the install scope.
    // Unrelated staged changes elsewhere are fine and left untouched.
    assert_no_staged_install_scope(&repo_path, &install_paths)?;

    let tarball = fetcher.fetch_tarball(&version)?;

    for h in &harnesses {
        materialise_one(&repo_path, h, &tarball, &version)?;
        eprintln!(
            "grove: {} → {} @ {}",
            match mode {
                Mode::Install => "installed",
                Mode::Update => "updated",
            },
            h.install_path(&repo_path).display(),
            version
        );
    }

    if args.no_commit {
        print_staging_command(&repo_path, &install_paths, mode, &version);
    } else {
        commit_install_scope(
            &repo_path,
            &install_paths,
            mode,
            &version,
            args.message.as_deref(),
        )?;
    }

    // Materialise the grove-inboxes branch + worktree alongside the install.
    // Idempotent; safe on both Install and Update. Lives on its own branch,
    // so it doesn't add to the install-scope commit above.
    inboxes::materialise(&repo_path)?;

    if mode == Mode::Update {
        eprintln!(
            "grove: record this bump as an ADR in docs/adr/ (grove's discipline for version changes)."
        );
    }

    Ok(())
}

fn materialise_one(
    repo: &Path,
    harness: &Harness,
    tarball: &[u8],
    version: &str,
) -> Result<()> {
    let dest = harness.install_path(repo);
    if dest.exists() {
        std::fs::remove_dir_all(&dest)
            .with_context(|| format!("clearing previous install at {}", dest.display()))?;
    }
    extract_content(tarball, &dest)?;
    version_md::write(&dest, harness.name, version)?;
    Ok(())
}

fn default_message(mode: Mode, version: &str) -> String {
    match mode {
        Mode::Install => format!("Install grove {}", version),
        Mode::Update => format!("Update grove to {}", version),
    }
}

/// Render install-scope paths as repo-relative strings, for both git invocation
/// and user-facing follow-up commands.
fn relative_paths(repo: &Path, paths: &[PathBuf]) -> Vec<String> {
    paths
        .iter()
        .map(|p| {
            p.strip_prefix(repo)
                .map(|r| r.to_string_lossy().into_owned())
                .unwrap_or_else(|_| p.to_string_lossy().into_owned())
        })
        .collect()
}

/// Returns true iff `git diff --cached --quiet -- <paths>` exits 0 (no staged
/// diff). Exit 1 means staged diff present. Any other exit code is propagated
/// as an error.
fn install_scope_clean(repo: &Path, rel_paths: &[String]) -> Result<bool> {
    let mut cmd = Command::new("git");
    cmd.arg("-C")
        .arg(repo)
        .arg("diff")
        .arg("--cached")
        .arg("--quiet")
        .arg("--");
    for p in rel_paths {
        cmd.arg(p);
    }
    let status = cmd.status().context("running git diff --cached")?;
    match status.code() {
        Some(0) => Ok(true),
        Some(1) => Ok(false),
        _ => anyhow::bail!("git diff --cached failed: {}", status),
    }
}

fn assert_no_staged_install_scope(repo: &Path, paths: &[PathBuf]) -> Result<()> {
    let rel = relative_paths(repo, paths);
    if install_scope_clean(repo, &rel)? {
        return Ok(());
    }
    let joined = rel.join(" ");
    anyhow::bail!(
        "refusing to proceed: install-scope paths have pre-existing staged changes ({}). \
         Commit or unstage them before running grove install/update.",
        joined
    );
}

fn commit_install_scope(
    repo: &Path,
    paths: &[PathBuf],
    mode: Mode,
    version: &str,
    message: Option<&str>,
) -> Result<()> {
    let rel = relative_paths(repo, paths);

    // Stage everything under the install scope.
    let mut add = Command::new("git");
    add.arg("-C").arg(repo).arg("add").arg("--").args(&rel);
    let status = add.status().context("running git add")?;
    if !status.success() {
        anyhow::bail!("git add failed: {}", status);
    }

    // No-op short-circuit: if nothing got staged (target identical to current),
    // skip the commit entirely.
    if install_scope_clean(repo, &rel)? {
        eprintln!("grove: no changes to commit");
        return Ok(());
    }

    let msg = message
        .map(|s| s.to_string())
        .unwrap_or_else(|| default_message(mode, version));

    let mut commit = Command::new("git");
    commit
        .arg("-C")
        .arg(repo)
        .arg("commit")
        .arg("-m")
        .arg(&msg)
        .arg("--")
        .args(&rel);
    let status = commit.status().context("running git commit")?;
    if !status.success() {
        let joined = rel.join(" ");
        anyhow::bail!(
            "git commit failed (exit {}). The materialisation is in place and the install-scope \
             paths are staged. Resolve the issue (e.g. fix a pre-commit hook), then run:\n  \
             git commit -- {}",
            status.code().map(|c| c.to_string()).unwrap_or_else(|| "?".into()),
            joined
        );
    }
    Ok(())
}

fn print_staging_command(repo: &Path, paths: &[PathBuf], mode: Mode, version: &str) {
    let rel = relative_paths(repo, paths);
    let joined = rel.join(" ");
    let msg = default_message(mode, version);
    eprintln!("grove: --no-commit; stage and commit yourself with:");
    eprintln!("  git add -- {}", joined);
    eprintln!("  git commit -m \"{}\"", msg);
}
