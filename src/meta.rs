// Meta-branch management: opt-in remote configuration and the manual
// fetch+push sync verb. See ADR-0005
// (`docs/adr/0005-grove-meta-sync-semantics.md`) for the binding
// semantics this module implements.

use crate::cli::{
    MetaArgs, MetaCommand, MetaInitArgs, MetaRemoteAddArgs, MetaRemoteCommand,
    MetaRemoteListArgs, MetaRemoteRemoveArgs, MetaSyncArgs,
};
use crate::inboxes;
use crate::repo;
use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

const REMOTE_NAME: &str = "origin";

/// Dispatcher for `grove meta <verb>`.
pub fn run(args: &MetaArgs) -> Result<()> {
    match &args.command {
        MetaCommand::Init(a) => cmd_init(a),
        MetaCommand::Remote(r) => match &r.command {
            MetaRemoteCommand::Add(a) => cmd_remote_add(a),
            MetaRemoteCommand::Remove(a) => cmd_remote_remove(a),
            MetaRemoteCommand::List(a) => cmd_remote_list(a),
        },
        MetaCommand::Sync(a) => cmd_sync(a),
    }
}

fn cmd_init(args: &MetaInitArgs) -> Result<()> {
    let repo_path = repo::resolve(args.repo.as_deref())?;
    init(&repo_path)
}

/// Materialise the `grove-meta` branch and the `<repo>/.grove-meta/`
/// worktree. Idempotent: a no-op when both are already in place. Prints a
/// one-line summary of the materialised state to stdout in all cases so the
/// verb is useful for "is this repo set up?" diagnostics.
///
/// Used by `grove install` / `grove update` via the same code path
/// (`inboxes::materialise`), and exposed as an explicit verb for repos that
/// pre-date the feature or whose worktree has been removed.
pub fn init(repo: &Path) -> Result<()> {
    inboxes::materialise(repo)?;
    let wt = inboxes::worktree_dir(repo);
    println!("grove-meta: branch={} worktree={}", inboxes::BRANCH, wt.display());
    Ok(())
}

fn cmd_remote_add(args: &MetaRemoteAddArgs) -> Result<()> {
    let repo_path = repo::resolve(args.repo.as_deref())?;
    remote_add(&repo_path, &args.url)
}

fn cmd_remote_remove(args: &MetaRemoteRemoveArgs) -> Result<()> {
    let repo_path = repo::resolve(args.repo.as_deref())?;
    remote_remove(&repo_path)
}

fn cmd_remote_list(args: &MetaRemoteListArgs) -> Result<()> {
    let repo_path = repo::resolve(args.repo.as_deref())?;
    remote_list(&repo_path)
}

fn cmd_sync(args: &MetaSyncArgs) -> Result<()> {
    let repo_path = repo::resolve(args.repo.as_deref())?;
    sync(&repo_path)
}

/// Wire `<url>` as the meta branch's upstream remote.
///
/// Three steps in one transaction:
/// 1. Ensure `origin` points at `<url>`. The meta worktree shares
///    `.git/config` with the main repo, so if `origin` already exists with
///    the requested URL (the common case: meta branch lives on the same
///    remote as the code) it is reused. A URL mismatch bails — we never
///    overwrite the user's existing remote.
/// 2. Fetch the meta branch from `origin` (tolerating the case where the
///    remote does not yet have it — the first capture's push creates it).
/// 3. Set `branch.<BRANCH>.remote` / `branch.<BRANCH>.merge` so subsequent
///    `git push` / `git pull` from the worktree are argument-less. Setting
///    upstream tracking eliminates the ticgit failure mode where
///    side-branch refs sit outside the default refspec (see ADR-0005).
pub fn remote_add(repo: &Path, url: &str) -> Result<()> {
    let wt = inboxes::require_worktree(repo)?;

    if inboxes::has_upstream(&wt, inboxes::BRANCH) {
        anyhow::bail!(
            "branch `{}` already has an upstream configured. Run `grove meta remote remove` first.",
            inboxes::BRANCH
        );
    }

    match get_remote_url(&wt, REMOTE_NAME)? {
        Some(existing) if existing == url => {
            // origin already wired to the right URL — skip the add.
        }
        Some(existing) => {
            anyhow::bail!(
                "remote `{}` already exists with URL `{}` (differs from requested `{}`). \
                 The meta worktree shares git config with the main repo; reconcile \
                 manually (e.g. pass the existing URL) before re-running.",
                REMOTE_NAME,
                existing,
                url
            );
        }
        None => {
            run_git(&wt, &["remote", "add", REMOTE_NAME, url])
                .context("running git remote add")?;
        }
    }

    // Fetch the meta branch. If the remote does not yet have it (first-time
    // setup), git prints "couldn't find remote ref ..." — that is not a
    // failure for our purposes; the first capture push will create it.
    let fetch_refspec = format!(
        "+refs/heads/{0}:refs/remotes/{1}/{0}",
        inboxes::BRANCH,
        REMOTE_NAME
    );
    let fetch_out = Command::new("git")
        .arg("-C")
        .arg(&wt)
        .arg("fetch")
        .arg(REMOTE_NAME)
        .arg(&fetch_refspec)
        .output()
        .context("running git fetch")?;
    if !fetch_out.status.success() {
        let stderr = String::from_utf8_lossy(&fetch_out.stderr);
        let benign_missing_ref = stderr.contains("couldn't find remote ref")
            || stderr.contains("Couldn't find remote ref");
        if !benign_missing_ref {
            anyhow::bail!("git fetch failed: {}", stderr.trim());
        }
    }

    run_git(
        &wt,
        &[
            "config",
            &format!("branch.{}.remote", inboxes::BRANCH),
            REMOTE_NAME,
        ],
    )?;
    run_git(
        &wt,
        &[
            "config",
            &format!("branch.{}.merge", inboxes::BRANCH),
            &format!("refs/heads/{}", inboxes::BRANCH),
        ],
    )?;

    eprintln!(
        "configured upstream: `{}` -> `{}/{}`",
        inboxes::BRANCH,
        REMOTE_NAME,
        inboxes::BRANCH
    );
    Ok(())
}

/// Clear the meta branch's upstream tracking config.
///
/// We deliberately do NOT `git remote remove origin`. The meta worktree
/// shares `.git/config` with the main repo, and the main repo almost
/// always has its own `origin` remote pointing at the code repository.
/// Removing it would break the main repo. Clearing the branch-level
/// tracking config is the precise undo of `remote_add` and leaves local
/// commits untouched.
pub fn remote_remove(repo: &Path) -> Result<()> {
    let wt = inboxes::require_worktree(repo)?;

    let removed_remote = unset_config(&wt, &format!("branch.{}.remote", inboxes::BRANCH))?;
    let removed_merge = unset_config(&wt, &format!("branch.{}.merge", inboxes::BRANCH))?;

    if !removed_remote && !removed_merge {
        eprintln!(
            "no upstream tracking configured for `{}`; nothing to remove.",
            inboxes::BRANCH
        );
    } else {
        eprintln!(
            "cleared upstream tracking for `{}`. The `{}` remote was not removed \
             (its config is shared with the main repo).",
            inboxes::BRANCH,
            REMOTE_NAME
        );
    }
    Ok(())
}

/// Print the meta worktree's configured remote(s) and upstream tracking.
pub fn remote_list(repo: &Path) -> Result<()> {
    let wt = inboxes::require_worktree(repo)?;

    let remotes_out = Command::new("git")
        .arg("-C")
        .arg(&wt)
        .arg("remote")
        .arg("-v")
        .output()
        .context("running git remote -v")?;
    let remotes = String::from_utf8_lossy(&remotes_out.stdout);

    if remotes.trim().is_empty() {
        println!("no remotes configured");
    } else {
        print!("{}", remotes);
    }

    let tracking_remote = get_config(&wt, &format!("branch.{}.remote", inboxes::BRANCH))?;
    let tracking_merge = get_config(&wt, &format!("branch.{}.merge", inboxes::BRANCH))?;
    match (tracking_remote, tracking_merge) {
        (Some(r), Some(m)) => {
            let short = m.strip_prefix("refs/heads/").unwrap_or(&m);
            println!("tracking: {} -> {}/{}", inboxes::BRANCH, r, short);
        }
        _ => println!("tracking: (none)"),
    }
    Ok(())
}

/// Manual sync: fetch + ff-merge the meta branch from upstream, then push
/// pending local commits. Cron-friendly: stdout silent on success/no-op,
/// stderr informative on action or failure. Exits non-zero only on
/// conflict (a real divergence the operator must resolve).
pub fn sync(repo: &Path) -> Result<()> {
    let wt = inboxes::require_worktree(repo)?;

    if !inboxes::has_upstream(&wt, inboxes::BRANCH) {
        eprintln!(
            "no upstream configured for `{}`; nothing to sync. \
             Run `grove meta remote add <url>` to opt in.",
            inboxes::BRANCH
        );
        return Ok(());
    }

    // Pull leg: shared with drain — soft on network failure, hard on non-ff.
    inboxes::fetch_and_ff(&wt)?;

    // Push leg: sync-flavoured (escalates persistent non-ff to a non-zero
    // exit so cron-driven sync surfaces the divergence loudly).
    inboxes::sync_push(&wt)?;

    Ok(())
}

// ---------------------------------------------------------------------------
// helpers

fn run_git(wt: &Path, args: &[&str]) -> Result<()> {
    let out = Command::new("git")
        .arg("-C")
        .arg(wt)
        .args(args)
        .output()
        .with_context(|| format!("running git {:?}", args))?;
    if !out.status.success() {
        anyhow::bail!(
            "git {:?} failed: {}",
            args,
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(())
}

fn get_remote_url(wt: &Path, remote: &str) -> Result<Option<String>> {
    let out = Command::new("git")
        .arg("-C")
        .arg(wt)
        .arg("remote")
        .arg("get-url")
        .arg(remote)
        .output()
        .context("running git remote get-url")?;
    if !out.status.success() {
        return Ok(None);
    }
    Ok(Some(String::from_utf8_lossy(&out.stdout).trim().to_string()))
}

fn get_config(wt: &Path, key: &str) -> Result<Option<String>> {
    let out = Command::new("git")
        .arg("-C")
        .arg(wt)
        .arg("config")
        .arg("--get")
        .arg(key)
        .output()
        .context("running git config --get")?;
    if !out.status.success() {
        return Ok(None);
    }
    Ok(Some(String::from_utf8_lossy(&out.stdout).trim().to_string()))
}

/// Unset a config key. Treats git's exit-5 ("key not found") as a benign
/// "already absent" so the verb is idempotent.
fn unset_config(wt: &Path, key: &str) -> Result<bool> {
    let out = Command::new("git")
        .arg("-C")
        .arg(wt)
        .arg("config")
        .arg("--unset")
        .arg(key)
        .output()
        .context("running git config --unset")?;
    if out.status.success() {
        return Ok(true);
    }
    if out.status.code() == Some(5) {
        return Ok(false);
    }
    anyhow::bail!(
        "git config --unset {} failed: {}",
        key,
        String::from_utf8_lossy(&out.stderr).trim()
    );
}
