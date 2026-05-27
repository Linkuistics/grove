use crate::cli::{InboxAddArgs, InboxArgs, InboxCommand, InboxDrainArgs, InboxShowArgs};
use crate::repo;
use anyhow::{Context, Result};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const BRANCH: &str = "grove-inboxes";
const WORKTREE_DIR: &str = ".grove-inboxes";
const INBOXES_SUBDIR: &str = "inboxes";

/// `<repo>/.grove-inboxes/` — the dedicated worktree on the `grove-inboxes`
/// branch that holds inbox files.
pub fn worktree_dir(repo: &Path) -> PathBuf {
    repo.join(WORKTREE_DIR)
}

/// `<repo>/.grove-inboxes/inboxes/<name>.md` — the per-grove inbox file.
pub fn inbox_file(repo: &Path, name: &str) -> PathBuf {
    worktree_dir(repo).join(INBOXES_SUBDIR).join(format!("{}.md", name))
}

/// Materialise the `grove-inboxes` branch and the `.grove-inboxes/` worktree
/// in `repo`. Idempotent: returns Ok if already materialised.
///
/// The branch starts from an empty tree commit (no shared history with the
/// default branch). This is deliberately portable to git versions that
/// pre-date `git worktree add --orphan` (2.42).
pub fn materialise(repo: &Path) -> Result<()> {
    let wt = worktree_dir(repo);
    if wt.is_dir() {
        return ensure_inboxes_subdir(&wt);
    }

    if !branch_exists(repo, BRANCH)? {
        create_empty_branch(repo, BRANCH)?;
    }

    let status = Command::new("git")
        .arg("-C")
        .arg(repo)
        .arg("worktree")
        .arg("add")
        .arg(&wt)
        .arg(BRANCH)
        .status()
        .context("running git worktree add for grove-inboxes")?;
    if !status.success() {
        anyhow::bail!("git worktree add failed for grove-inboxes");
    }

    ensure_inboxes_subdir(&wt)?;
    Ok(())
}

/// Append `observation` to the inbox for `name` in `repo` and commit the
/// change on the `grove-inboxes` branch. Creates the inbox file if absent.
pub fn append(repo: &Path, name: &str, observation: &str) -> Result<()> {
    let wt = require_worktree(repo)?;
    let file = inbox_file(repo, name);
    let rel = format!("{}/{}.md", INBOXES_SUBDIR, name);

    if let Some(parent) = file.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file)
        .with_context(|| format!("opening {}", file.display()))?;
    f.write_all(format_entry(observation).as_bytes())
        .with_context(|| format!("writing to {}", file.display()))?;

    commit_inbox_file(&wt, &rel, &format!("inbox: append to {}", name))?;
    Ok(())
}

/// Truncate the inbox for `name` and commit the cleared file on the
/// `grove-inboxes` branch. No-op if the file does not exist or is empty.
pub fn drain(repo: &Path, name: &str) -> Result<()> {
    let wt = require_worktree(repo)?;
    let file = inbox_file(repo, name);
    let rel = format!("{}/{}.md", INBOXES_SUBDIR, name);

    if !file.exists() {
        return Ok(());
    }
    let cur = std::fs::read_to_string(&file)
        .with_context(|| format!("reading {}", file.display()))?;
    if cur.is_empty() {
        return Ok(());
    }
    std::fs::write(&file, b"")
        .with_context(|| format!("clearing {}", file.display()))?;

    commit_inbox_file(&wt, &rel, &format!("inbox: drain {}", name))?;
    Ok(())
}

/// Read the inbox file for `name` in `repo`. Returns empty string if the
/// worktree is not materialised or the file does not exist — drain-on-bootstrap
/// must not fail just because nothing has ever been captured.
pub fn read(repo: &Path, name: &str) -> Result<String> {
    let wt = worktree_dir(repo);
    if !wt.is_dir() {
        return Ok(String::new());
    }
    let file = inbox_file(repo, name);
    match std::fs::read_to_string(&file) {
        Ok(s) => Ok(s),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(String::new()),
        Err(e) => Err(e).with_context(|| format!("reading {}", file.display())),
    }
}

/// Dispatcher for the `grove inbox <verb>` CLI surface.
pub fn run(args: &InboxArgs) -> Result<()> {
    match &args.command {
        InboxCommand::Add(a) => cmd_add(a),
        InboxCommand::Drain(a) => cmd_drain(a),
        InboxCommand::Show(a) => cmd_show(a),
    }
}

fn cmd_add(args: &InboxAddArgs) -> Result<()> {
    let repo_path = repo::resolve(args.repo.as_deref())?;
    let observation = match &args.observation {
        Some(s) => s.clone(),
        None => read_stdin()?,
    };
    if observation.trim().is_empty() {
        anyhow::bail!("empty observation; pass text as an argument or via stdin");
    }
    append(&repo_path, &args.name, &observation)
}

fn cmd_drain(args: &InboxDrainArgs) -> Result<()> {
    let repo_path = repo::resolve(args.repo.as_deref())?;
    drain(&repo_path, &args.name)
}

fn cmd_show(args: &InboxShowArgs) -> Result<()> {
    let repo_path = repo::resolve(args.repo.as_deref())?;
    let body = read(&repo_path, &args.name)?;
    print!("{}", body);
    Ok(())
}

// ---------------------------------------------------------------------------
// helpers

fn require_worktree(repo: &Path) -> Result<PathBuf> {
    let wt = worktree_dir(repo);
    if !wt.is_dir() {
        anyhow::bail!(
            "grove-inboxes worktree not present at {} — run `grove install` (or `grove update`) first",
            wt.display()
        );
    }
    Ok(wt)
}

fn ensure_inboxes_subdir(wt: &Path) -> Result<()> {
    let dir = wt.join(INBOXES_SUBDIR);
    std::fs::create_dir_all(&dir).with_context(|| format!("creating {}", dir.display()))?;
    Ok(())
}

fn branch_exists(repo: &Path, branch: &str) -> Result<bool> {
    let status = Command::new("git")
        .arg("-C")
        .arg(repo)
        .arg("show-ref")
        .arg("--verify")
        .arg("--quiet")
        .arg(format!("refs/heads/{}", branch))
        .status()
        .context("running git show-ref")?;
    Ok(status.success())
}

fn create_empty_branch(repo: &Path, branch: &str) -> Result<()> {
    let empty_tree = git_capture(repo, &["mktree"])?;
    let empty_tree = empty_tree.trim();
    let commit = git_capture(
        repo,
        &["commit-tree", empty_tree, "-m", "init grove-inboxes"],
    )?;
    let commit = commit.trim();

    let status = Command::new("git")
        .arg("-C")
        .arg(repo)
        .arg("branch")
        .arg(branch)
        .arg(commit)
        .status()
        .context("creating grove-inboxes branch")?;
    if !status.success() {
        anyhow::bail!("git branch failed for {}", branch);
    }
    Ok(())
}

fn git_capture(repo: &Path, args: &[&str]) -> Result<String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .stdin(Stdio::null())
        .output()
        .with_context(|| format!("running git {:?}", args))?;
    if !out.status.success() {
        anyhow::bail!(
            "git {:?} failed: {}",
            args,
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    String::from_utf8(out.stdout).with_context(|| format!("git {:?} output not utf-8", args))
}

fn format_entry(observation: &str) -> String {
    // Newline + separator + observation + trailing newline. Separator gives
    // the receiving grove a clean drain boundary; the trim_end avoids
    // accumulating trailing whitespace across many appends.
    format!("\n---\n{}\n", observation.trim_end())
}

fn commit_inbox_file(worktree: &Path, rel: &str, message: &str) -> Result<()> {
    let add = Command::new("git")
        .arg("-C")
        .arg(worktree)
        .arg("add")
        .arg("--")
        .arg(rel)
        .status()
        .context("running git add in inbox worktree")?;
    if !add.success() {
        anyhow::bail!("git add failed for {}", rel);
    }

    let diff = Command::new("git")
        .arg("-C")
        .arg(worktree)
        .arg("diff")
        .arg("--cached")
        .arg("--quiet")
        .arg("--")
        .arg(rel)
        .status()
        .context("running git diff --cached in inbox worktree")?;
    if diff.success() {
        return Ok(());
    }

    let commit = Command::new("git")
        .arg("-C")
        .arg(worktree)
        .arg("commit")
        .arg("-m")
        .arg(message)
        .arg("--")
        .arg(rel)
        .status()
        .context("running git commit in inbox worktree")?;
    if !commit.success() {
        anyhow::bail!("git commit failed for {}", rel);
    }
    Ok(())
}

fn read_stdin() -> Result<String> {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).context("reading stdin")?;
    Ok(s)
}
