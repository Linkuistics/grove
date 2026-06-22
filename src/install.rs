use crate::cli::InstallArgs;
use crate::extract::extract_content;
use crate::fetch::{Fetcher, GithubFetcher};
use crate::harness::{self, Harness, SelectMode};
use crate::repo;
use crate::version_md;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn run(args: &InstallArgs) -> Result<()> {
    let fetcher = GithubFetcher::new();
    run_with_fetcher(args, &fetcher)
}

pub fn run_with_fetcher(args: &InstallArgs, fetcher: &dyn Fetcher) -> Result<()> {
    let repo_path = repo::resolve(args.repo.as_deref())?;
    let harnesses = harness::select(&repo_path, &args.harnesses, SelectMode::Multi)?;
    // `target` is the git tag/ref (may carry a leading `v`) and is the fetch
    // ref; `canonical` is the stamp/compare form. Strip only the stamp, never
    // the fetch ref — see `version_md::canonical` and ADR-0008.
    let target = match &args.version {
        Some(v) => v.clone(),
        None => fetcher.latest_version()?,
    };
    let canonical = version_md::canonical(&target).to_string();
    eprintln!("grove: target {} @ {}", repo_path.display(), target);

    // The install scope: every install-path we're about to touch, combined.
    let install_paths: Vec<PathBuf> = harnesses
        .iter()
        .map(|h| h.install_path(&repo_path))
        .collect();

    // Pre-flight: refuse if anything is already staged inside the install scope.
    // Unrelated staged changes elsewhere are fine and left untouched.
    assert_no_staged_install_scope(&repo_path, &install_paths)?;

    let tarball = fetcher.fetch_tarball(&target)?;

    // `grove install` is idempotent (ADR-0008): not installed → install; same
    // canonical version → no-op; different → update. The per-harness outcome is
    // *always* printed — it is the audit trail that replaces a mode-gated nudge.
    let mut any_existing = false;
    let mut any_updated = false;
    for h in &harnesses {
        let dest = h.install_path(&repo_path);
        // Read the prior stamp *before* re-materialising clears it.
        let prior = version_md::read_version(&dest).ok();
        if prior.is_some() {
            any_existing = true;
        }
        if prior.as_deref().is_some_and(|p| p != canonical) {
            any_updated = true;
        }
        materialise_one(&repo_path, h, &tarball, &target)?;
        eprintln!(
            "grove: {} → {}",
            dest.display(),
            outcome_phrase(prior.as_deref(), &canonical)
        );
    }

    if args.no_commit {
        print_staging_command(&repo_path, &install_paths, any_existing, &target);
    } else {
        commit_install_scope(
            &repo_path,
            &install_paths,
            any_existing,
            &target,
            args.message.as_deref(),
        )?;
    }

    if any_updated {
        eprintln!(
            "grove: record this bump as an ADR in docs/adr/ (grove's discipline for version changes)."
        );
    }

    Ok(())
}

/// The per-harness outcome phrase, compared on the **canonical** stamp (no
/// `v`). `prior` is the stamp already in the harness's `VERSION.md`, or `None`
/// when grove was not installed there.
///
/// - `None` → `installed @ X`
/// - `Some(p)` where `p == canonical` → `already at X, no change`
/// - `Some(p)` otherwise → `updated p → X`
fn outcome_phrase(prior: Option<&str>, canonical: &str) -> String {
    match prior {
        None => format!("installed @ {}", canonical),
        Some(p) if p == canonical => format!("already at {}, no change", canonical),
        Some(p) => format!("updated {} → {}", p, canonical),
    }
}

fn materialise_one(repo: &Path, harness: &Harness, tarball: &[u8], version: &str) -> Result<()> {
    let dest = harness.install_path(repo);
    if dest.exists() {
        std::fs::remove_dir_all(&dest)
            .with_context(|| format!("clearing previous install at {}", dest.display()))?;
    }
    extract_content(tarball, &dest)?;
    version_md::write(&dest, harness.name, version)?;
    Ok(())
}

/// Default commit subject. A fresh materialisation (no harness had a prior
/// install) reads as `Install grove <tag>`; refreshing any existing install
/// reads as `Update grove to <tag>`. `version` is the raw tag (with `v`), to
/// match the fetch ref and historical commit subjects.
fn default_message(any_existing: bool, version: &str) -> String {
    if any_existing {
        format!("Update grove to {}", version)
    } else {
        format!("Install grove {}", version)
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
         Commit or unstage them before running grove install.",
        joined
    );
}

fn commit_install_scope(
    repo: &Path,
    paths: &[PathBuf],
    any_existing: bool,
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
        .unwrap_or_else(|| default_message(any_existing, version));

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
            status
                .code()
                .map(|c| c.to_string())
                .unwrap_or_else(|| "?".into()),
            joined
        );
    }
    Ok(())
}

fn print_staging_command(repo: &Path, paths: &[PathBuf], any_existing: bool, version: &str) {
    let rel = relative_paths(repo, paths);
    let joined = rel.join(" ");
    let msg = default_message(any_existing, version);
    eprintln!("grove: --no-commit; stage and commit yourself with:");
    eprintln!("  git add -- {}", joined);
    eprintln!("  git commit -m \"{}\"", msg);
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- the three idempotent outcomes (ADR-0008) ------------------------
    // Comparison is on the canonical stamp; inputs model the post-v4 world.

    #[test]
    fn outcome_fresh_install() {
        assert_eq!(outcome_phrase(None, "4.0.0"), "installed @ 4.0.0");
    }

    #[test]
    fn outcome_same_version_is_no_change() {
        assert_eq!(
            outcome_phrase(Some("4.0.0"), "4.0.0"),
            "already at 4.0.0, no change"
        );
    }

    #[test]
    fn outcome_different_version_is_update() {
        assert_eq!(
            outcome_phrase(Some("3.0.1"), "4.0.0"),
            "updated 3.0.1 → 4.0.0"
        );
    }

    #[test]
    fn outcome_prior_v_prefixed_stamp_is_pre_v4_update() {
        // A lingering `v` stamp is a pre-v4 release: `!=` canonical, so it
        // reads as an update, shown verbatim. No normalisation masks it.
        assert_eq!(
            outcome_phrase(Some("v3.0.1"), "4.0.0"),
            "updated v3.0.1 → 4.0.0"
        );
    }

    // --- default commit subject ------------------------------------------

    #[test]
    fn message_fresh_reads_as_install() {
        assert_eq!(default_message(false, "v4.0.0"), "Install grove v4.0.0");
    }

    #[test]
    fn message_existing_reads_as_update() {
        assert_eq!(default_message(true, "v4.0.0"), "Update grove to v4.0.0");
    }
}
