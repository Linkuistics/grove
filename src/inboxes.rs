// Inbox shape and capture: directory-of-observation-files on the
// `grove-inboxes` branch. See ADR-0004
// (`docs/adr/0004-inbox-as-directory-of-observation-files.md`) for the
// shape rationale and ADR-0005 for the sync semantics this module
// implements at the capture side.

use crate::cli::{InboxAddArgs, InboxArgs, InboxCommand, InboxDrainArgs, InboxShowArgs};
use crate::repo;
use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) const BRANCH: &str = "grove-inboxes";
const WORKTREE_DIR: &str = ".grove-inboxes";
const INBOXES_SUBDIR: &str = "inboxes";
const GITKEEP: &str = ".gitkeep";

/// `<repo>/.grove-inboxes/` — the dedicated worktree on the `grove-inboxes`
/// branch that holds inbox files.
pub fn worktree_dir(repo: &Path) -> PathBuf {
    repo.join(WORKTREE_DIR)
}

/// `<repo>/.grove-inboxes/inboxes/<name>/` — the per-grove inbox directory.
pub fn inbox_dir(repo: &Path, name: &str) -> PathBuf {
    worktree_dir(repo).join(INBOXES_SUBDIR).join(name)
}

/// Legacy single-file path. Present only during the one-time migration on
/// first capture to a previously single-file inbox.
fn legacy_inbox_file(repo: &Path, name: &str) -> PathBuf {
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

/// Capture `observation` into the inbox for `name` in `repo`.
///
/// Writes one new file at
/// `inboxes/<name>/<UTC-iso8601-seconds>Z-<slug>-<content-hash-8>.md` on the
/// `grove-inboxes` branch and commits it. Idempotent on content hash: if a
/// file with the same content-hash already exists in the target directory,
/// no new file is written and "already captured at <path>" is printed to
/// stderr (returns Ok).
///
/// If a legacy single-file inbox (`inboxes/<name>.md`) is present, it is
/// migrated into the directory shape in a dedicated commit first.
///
/// Push policy (per ADR-0005): if the `grove-inboxes` branch has an upstream
/// configured, push best-effort after commit, with one auto-fetch + replay
/// retry on non-ff. On non-network failure the push error is reported but
/// the local commit stands.
pub fn capture(repo: &Path, name: &str, observation: &str, slug_override: Option<&str>) -> Result<()> {
    if observation.trim().is_empty() {
        anyhow::bail!("empty observation; pass body via --body, --body-file, or --body-stdin");
    }
    let wt = require_worktree(repo)?;
    let dir = inbox_dir(repo, name);

    migrate_legacy_if_present(&wt, name)?;

    let hash = content_hash_8(observation);
    if let Some(existing) = find_by_hash(&dir, &hash)? {
        eprintln!("already captured at {}", existing.display());
        return Ok(());
    }

    let slug = match slug_override {
        Some(s) => sanitize_slug(s),
        None => derive_slug(observation),
    };
    let stamp = utc_iso8601_seconds(SystemTime::now());
    let filename = format!("{}Z-{}-{}.md", stamp, slug, hash);

    let dir_existed = dir.is_dir();
    std::fs::create_dir_all(&dir).with_context(|| format!("creating {}", dir.display()))?;
    let entry_path = dir.join(&filename);
    std::fs::write(&entry_path, observation.as_bytes())
        .with_context(|| format!("writing {}", entry_path.display()))?;

    let mut to_stage = vec![format!("{}/{}/{}", INBOXES_SUBDIR, name, filename)];
    if !dir_existed {
        let gitkeep = dir.join(GITKEEP);
        std::fs::write(&gitkeep, b"")
            .with_context(|| format!("writing {}", gitkeep.display()))?;
        to_stage.push(format!("{}/{}/{}", INBOXES_SUBDIR, name, GITKEEP));
    }

    with_index_lock_retry(|| {
        stage_and_commit(&wt, &to_stage, &format!("inbox: capture to {}", name))
    })?;

    push_best_effort(&wt);
    Ok(())
}

/// Phase 1 of drain: fetch the latest `grove-inboxes` state (if a remote is
/// configured) and enumerate the pending observation files for `name`.
///
/// Returns the absolute paths of observation files in lexicographic
/// (chronological) order. The `.gitkeep` placeholder and any non-`.md`
/// files are excluded. If the inbox directory does not yet exist for
/// `name`, returns an empty Vec — the directory will be created on the
/// next capture.
///
/// Fetch policy (ADR-0005): when an upstream is configured for
/// `grove-inboxes`, `git fetch` runs and then a `git merge --ff-only` is
/// attempted. Fetch failures (offline, auth) print a warning to stderr and
/// the function continues with local state. A non-ff against unpushed
/// local commits is fatal — drain refuses-and-instructs rather than
/// auto-merging. When no upstream is configured, the fetch step is a
/// silent no-op.
pub fn drain_enumerate(repo: &Path, name: &str) -> Result<Vec<PathBuf>> {
    let wt = require_worktree(repo)?;

    migrate_legacy_if_present(&wt, name)?;

    fetch_and_ff(&wt)?;

    let dir = inbox_dir(repo, name);
    if !dir.is_dir() {
        return Ok(Vec::new());
    }
    list_observations(&dir)
}

/// Phase 2 of drain: delete the LLM's triaged observation files and commit
/// with a disposition-counts message.
///
/// All paths in `incorporated`, `deferred`, and `rejected` must lie inside
/// `inboxes/<name>/` on the `grove-inboxes` worktree; any path outside is
/// rejected before any deletion happens. The total triage count must be
/// at least one — a no-op finalize call is a programming error and bails
/// rather than producing an empty commit.
///
/// The single commit message follows ADR-0004's session-granularity rule:
/// `drain <name>: N incorporated, M deferred, K rejected`. Push policy is
/// the same best-effort-with-one-auto-retry path used by capture
/// (ADR-0005).
pub fn drain_finalize(
    repo: &Path,
    name: &str,
    incorporated: &[PathBuf],
    deferred: &[PathBuf],
    rejected: &[PathBuf],
) -> Result<()> {
    let wt = require_worktree(repo)?;
    let dir = inbox_dir(repo, name);

    let total = incorporated.len() + deferred.len() + rejected.len();
    if total == 0 {
        anyhow::bail!(
            "drain finalize for {}: no paths in any disposition (programming error \
             — call enumerate instead, or omit the finalize call when no \
             observations were pending)",
            name
        );
    }

    let mut rel_paths: Vec<String> = Vec::with_capacity(total);
    for group in [incorporated, deferred, rejected] {
        for p in group {
            let rel = validate_path_inside_inbox(&wt, &dir, p)?;
            rel_paths.push(rel);
        }
    }

    for rel in &rel_paths {
        let abs = wt.join(rel);
        if abs.exists() {
            std::fs::remove_file(&abs)
                .with_context(|| format!("removing {}", abs.display()))?;
        }
    }

    let message = format!(
        "drain {}: {} incorporated, {} deferred, {} rejected",
        name,
        incorporated.len(),
        deferred.len(),
        rejected.len()
    );
    with_index_lock_retry(|| stage_and_commit(&wt, &rel_paths, &message))?;
    push_best_effort(&wt);
    Ok(())
}

/// Validate that `candidate` (as supplied by the LLM, possibly absolute or
/// relative to cwd) names a file inside `inbox_dir`. Returns the
/// `inboxes/<name>/<entry>.md` path relative to `worktree`, suitable for
/// `git add`/`git commit -- <paths>`.
///
/// Rejects: paths that don't canonicalise into the inbox dir, paths
/// pointing at `.gitkeep`, and paths whose filename doesn't end in `.md`.
/// We tolerate non-existent files because the caller may have already
/// removed them in a partial earlier run — but we still verify the
/// (notional) parent matches `inbox_dir`.
fn validate_path_inside_inbox(worktree: &Path, inbox_dir: &Path, candidate: &Path) -> Result<String> {
    let abs = if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        std::env::current_dir().context("getting cwd")?.join(candidate)
    };

    let canon_abs = canonical_parent_plus_file(&abs)?;
    let canon_inbox = inbox_dir
        .canonicalize()
        .or_else(|_| canonical_parent_plus_file(inbox_dir).map(|p| p.parent().unwrap().to_path_buf()))
        .with_context(|| format!("resolving inbox dir {}", inbox_dir.display()))?;

    let parent = canon_abs
        .parent()
        .ok_or_else(|| anyhow::anyhow!("path has no parent: {}", candidate.display()))?;
    if parent != canon_inbox {
        anyhow::bail!(
            "path {} is not inside the inbox directory {}",
            candidate.display(),
            inbox_dir.display()
        );
    }

    let filename = canon_abs
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| anyhow::anyhow!("path has no filename: {}", candidate.display()))?;
    if filename == GITKEEP {
        anyhow::bail!(".gitkeep cannot be drained: {}", candidate.display());
    }
    if !filename.ends_with(".md") {
        anyhow::bail!(
            "drain only operates on .md observation files (got {})",
            candidate.display()
        );
    }

    let rel = canon_abs
        .strip_prefix(worktree.canonicalize().with_context(|| {
            format!("canonicalising worktree {}", worktree.display())
        })?)
        .with_context(|| {
            format!(
                "path {} is not inside the inbox worktree {}",
                canon_abs.display(),
                worktree.display()
            )
        })?;
    Ok(rel.to_string_lossy().replace('\\', "/"))
}

/// Resolve a path's parent to its canonical form even if the file itself no
/// longer exists. We do this so a previously-removed observation file can
/// still pass the inside-inbox check.
fn canonical_parent_plus_file(p: &Path) -> Result<PathBuf> {
    let parent = p
        .parent()
        .ok_or_else(|| anyhow::anyhow!("path has no parent: {}", p.display()))?;
    let filename = p
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("path has no filename: {}", p.display()))?;
    let canon_parent = parent
        .canonicalize()
        .with_context(|| format!("canonicalising {}", parent.display()))?;
    Ok(canon_parent.join(filename))
}

/// Fetch `grove-inboxes` from its upstream (if any) and fast-forward the
/// local worktree. Per ADR-0005: warn-and-continue on fetch failure,
/// refuse on non-ff. No-op when no upstream is configured. Shared by the
/// drain bootstrap path and the `grove meta sync` pull leg — both want
/// identical soft-network/hard-conflict semantics.
pub(crate) fn fetch_and_ff(worktree: &Path) -> Result<()> {
    if !has_upstream(worktree, BRANCH) {
        return Ok(());
    }

    let fetch_out = Command::new("git")
        .arg("-C")
        .arg(worktree)
        .arg("fetch")
        .output();
    let fetch_out = match fetch_out {
        Ok(o) => o,
        Err(e) => {
            eprintln!(
                "warning: grove-inboxes fetch failed ({}); continuing with local state.",
                e
            );
            return Ok(());
        }
    };
    if !fetch_out.status.success() {
        let stderr = String::from_utf8_lossy(&fetch_out.stderr);
        if looks_like_network_failure(&stderr) {
            eprintln!(
                "warning: grove-inboxes fetch failed ({}); continuing with local state.",
                stderr.trim()
            );
            return Ok(());
        }
        eprintln!(
            "warning: grove-inboxes fetch failed ({}); continuing with local state.",
            stderr.trim()
        );
        return Ok(());
    }

    let merge_out = Command::new("git")
        .arg("-C")
        .arg(worktree)
        .arg("merge")
        .arg("--ff-only")
        .arg("@{u}")
        .output()
        .context("running git merge --ff-only @{u}")?;
    if merge_out.status.success() {
        return Ok(());
    }
    let stderr = String::from_utf8_lossy(&merge_out.stderr);
    if stderr.contains("Already up to date") {
        return Ok(());
    }
    if stderr.contains("not possible to fast-forward")
        || stderr.contains("Not possible to fast-forward")
        || stderr.contains("Refusing to merge unrelated histories")
        || stderr.contains("non-fast-forward")
    {
        anyhow::bail!(
            "grove-inboxes has diverged from its upstream — local commits and remote \
             commits both exist. Resolve by inspecting `cd {} && git log @{{u}}.. && \
             git log ..@{{u}}` and either pushing local first (if remote has no new \
             entries) or rebasing onto upstream.",
            worktree.display()
        );
    }
    anyhow::bail!("git merge --ff-only failed: {}", stderr.trim());
}

/// Read the inbox content for `name` in `repo`: concatenates every
/// observation file in chronological filename order. Used by `grove inbox
/// show`. Returns an empty string if the worktree is not materialised or
/// the inbox holds no observations.
pub fn read(repo: &Path, name: &str) -> Result<String> {
    let wt = worktree_dir(repo);
    if !wt.is_dir() {
        return Ok(String::new());
    }

    let legacy = legacy_inbox_file(repo, name);
    let dir = inbox_dir(repo, name);

    let mut out = String::new();

    if legacy.is_file() {
        let body = std::fs::read_to_string(&legacy)
            .with_context(|| format!("reading {}", legacy.display()))?;
        out.push_str(&body);
    }

    if dir.is_dir() {
        for entry in list_observations(&dir)? {
            let body = std::fs::read_to_string(&entry)
                .with_context(|| format!("reading {}", entry.display()))?;
            if !out.is_empty() && !out.ends_with('\n') {
                out.push('\n');
            }
            if !out.is_empty() {
                out.push_str("\n---\n");
            }
            out.push_str(&body);
            if !out.ends_with('\n') {
                out.push('\n');
            }
        }
    }

    Ok(out)
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
    let observation = read_body(args)?;
    capture(&repo_path, &args.to, &observation, args.slug.as_deref())
}

fn cmd_drain(args: &InboxDrainArgs) -> Result<()> {
    let repo_path = repo::resolve(args.repo.as_deref())?;
    let name = &args.for_grove;
    let has_dispositions =
        !args.incorporated.is_empty() || !args.deferred.is_empty() || !args.rejected.is_empty();

    if has_dispositions {
        return drain_finalize(
            &repo_path,
            name,
            &args.incorporated,
            &args.deferred,
            &args.rejected,
        );
    }

    let paths = drain_enumerate(&repo_path, name)?;
    for p in &paths {
        println!("{}", p.display());
    }
    if paths.is_empty() {
        eprintln!("inbox {}: no pending observations", name);
    } else {
        eprintln!(
            "inbox {}: {} pending observation{}",
            name,
            paths.len(),
            if paths.len() == 1 { "" } else { "s" }
        );
    }
    Ok(())
}

fn cmd_show(args: &InboxShowArgs) -> Result<()> {
    let repo_path = repo::resolve(args.repo.as_deref())?;
    let body = read(&repo_path, &args.name)?;
    print!("{}", body);
    Ok(())
}

fn read_body(args: &InboxAddArgs) -> Result<String> {
    if let Some(b) = &args.body {
        return Ok(b.clone());
    }
    if let Some(p) = &args.body_file {
        return std::fs::read_to_string(p)
            .with_context(|| format!("reading body file {}", p.display()));
    }
    if args.body_stdin {
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).context("reading body from stdin")?;
        return Ok(s);
    }
    anyhow::bail!("provide observation via --body, --body-file, or --body-stdin");
}

// ---------------------------------------------------------------------------
// helpers

pub(crate) fn require_worktree(repo: &Path) -> Result<PathBuf> {
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

fn list_observations(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut v = Vec::new();
    for entry in std::fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        let name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n,
            None => continue,
        };
        if name == GITKEEP || !name.ends_with(".md") {
            continue;
        }
        v.push(path);
    }
    v.sort();
    Ok(v)
}

fn find_by_hash(dir: &Path, hash: &str) -> Result<Option<PathBuf>> {
    if !dir.is_dir() {
        return Ok(None);
    }
    let suffix = format!("-{}.md", hash);
    for entry in std::fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.ends_with(&suffix))
            .unwrap_or(false)
        {
            return Ok(Some(path));
        }
    }
    Ok(None)
}

fn migrate_legacy_if_present(wt: &Path, name: &str) -> Result<()> {
    let legacy_rel = format!("{}/{}.md", INBOXES_SUBDIR, name);
    let legacy_abs = wt.join(&legacy_rel);
    if !legacy_abs.is_file() {
        return Ok(());
    }
    let body = std::fs::read_to_string(&legacy_abs)
        .with_context(|| format!("reading legacy inbox {}", legacy_abs.display()))?;
    let hash = content_hash_8(&body);
    let stamp = utc_iso8601_seconds(SystemTime::now());
    let new_name = format!("{}Z-legacy-{}.md", stamp, hash);

    let dir = wt.join(INBOXES_SUBDIR).join(name);
    let dir_existed_before = dir.is_dir();
    std::fs::create_dir_all(&dir).with_context(|| format!("creating {}", dir.display()))?;
    let dest_rel = format!("{}/{}/{}", INBOXES_SUBDIR, name, new_name);

    // git mv stages both the deletion and the addition in one go and
    // preserves rename detection in the log.
    with_index_lock_retry(|| run_git_mv(wt, &legacy_rel, &dest_rel))?;

    let gitkeep_rel = format!("{}/{}/{}", INBOXES_SUBDIR, name, GITKEEP);
    if !dir_existed_before {
        let gitkeep_abs = dir.join(GITKEEP);
        std::fs::write(&gitkeep_abs, b"")
            .with_context(|| format!("writing {}", gitkeep_abs.display()))?;
        with_index_lock_retry(|| run_git_add(wt, std::slice::from_ref(&gitkeep_rel)))?;
    }

    let mut commit_paths = vec![legacy_rel.clone(), dest_rel.clone()];
    if !dir_existed_before {
        commit_paths.push(gitkeep_rel);
    }
    with_index_lock_retry(|| {
        run_git_commit(
            wt,
            &commit_paths,
            &format!("migrate inbox {} to directory shape", name),
        )
    })?;
    push_best_effort(wt);
    Ok(())
}

fn run_git_mv(wt: &Path, src: &str, dst: &str) -> Result<()> {
    let out = Command::new("git")
        .arg("-C")
        .arg(wt)
        .arg("mv")
        .arg(src)
        .arg(dst)
        .output()
        .context("running git mv")?;
    if !out.status.success() {
        anyhow::bail!(
            "git mv failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(())
}

fn run_git_add(wt: &Path, paths: &[String]) -> Result<()> {
    let mut cmd = Command::new("git");
    cmd.arg("-C").arg(wt).arg("add").arg("--all").arg("--");
    for p in paths {
        cmd.arg(p);
    }
    let out = cmd.output().context("running git add")?;
    if !out.status.success() {
        anyhow::bail!(
            "git add failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(())
}

fn run_git_commit(wt: &Path, paths: &[String], message: &str) -> Result<()> {
    let mut cmd = Command::new("git");
    cmd.arg("-C").arg(wt).arg("commit").arg("-m").arg(message).arg("--");
    for p in paths {
        cmd.arg(p);
    }
    let out = cmd.output().context("running git commit")?;
    if !out.status.success() {
        anyhow::bail!(
            "git commit failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
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

/// Run `git add -- <paths...>` then `git commit -m <msg> -- <paths...>` in
/// the inbox worktree. Returns Err on git failure with stderr included so
/// callers can detect index-lock contention from the message.
fn stage_and_commit(worktree: &Path, paths: &[String], message: &str) -> Result<()> {
    // `--all --` records additions, modifications, and deletions for the
    // listed paths. Deletions come up under drain (files just unlinked) and
    // under migration (after `git mv` the legacy path's removal is already
    // staged but a re-`add` of the absent path would otherwise error with
    // "did not match any files").
    let mut add = Command::new("git");
    add.arg("-C").arg(worktree).arg("add").arg("--all").arg("--");
    for p in paths {
        add.arg(p);
    }
    let add_out = add.output().context("running git add in inbox worktree")?;
    if !add_out.status.success() {
        anyhow::bail!(
            "git add failed: {}",
            String::from_utf8_lossy(&add_out.stderr).trim()
        );
    }

    let mut diff = Command::new("git");
    diff.arg("-C").arg(worktree).arg("diff").arg("--cached").arg("--quiet").arg("--");
    for p in paths {
        diff.arg(p);
    }
    let diff_status = diff.status().context("running git diff --cached")?;
    if diff_status.success() {
        return Ok(());
    }

    let mut commit = Command::new("git");
    commit.arg("-C").arg(worktree).arg("commit").arg("-m").arg(message).arg("--");
    for p in paths {
        commit.arg(p);
    }
    let commit_out = commit.output().context("running git commit in inbox worktree")?;
    if !commit_out.status.success() {
        anyhow::bail!(
            "git commit failed: {}",
            String::from_utf8_lossy(&commit_out.stderr).trim()
        );
    }
    Ok(())
}

/// Retries `op` on detected index.lock contention with bounded exponential
/// backoff plus per-process jitter (~5s worst-case total). Other errors
/// propagate immediately. Per ADR-0005, this is the entirety of grove's
/// intra-machine concurrency strategy — git's own lock plus a CLI retry.
///
/// Jitter is essential: under N-way parallel contention (the
/// `seq 1 10 | xargs -P 10` stress case), processes retrying on identical
/// schedules collide on every wake-up and exhaust the budget without
/// making progress. The jitter source is the low-order nanoseconds of
/// `SystemTime::now()` mixed with the process id, which is enough to
/// desynchronise the retry waves without pulling in a random dependency.
fn with_index_lock_retry<F>(mut op: F) -> Result<()>
where
    F: FnMut() -> Result<()>,
{
    const BASE_DELAYS_MS: [u64; 7] = [50, 100, 200, 400, 800, 1200, 1800];
    for &delay in &BASE_DELAYS_MS {
        match op() {
            Ok(()) => return Ok(()),
            Err(e) if is_index_lock_error(&e) => {
                let jitter = retry_jitter_ms(delay);
                std::thread::sleep(std::time::Duration::from_millis(delay + jitter));
            }
            Err(e) => return Err(e),
        }
    }
    op()
}

fn retry_jitter_ms(base: u64) -> u64 {
    let ns = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as u64)
        .unwrap_or(0);
    let pid = std::process::id() as u64;
    let mix = ns.wrapping_mul(2654435761).wrapping_add(pid);
    // Jitter up to the same scale as `base` so retries spread out
    // proportionally as backoff grows.
    mix % base.max(1)
}

fn is_index_lock_error(err: &anyhow::Error) -> bool {
    let s = format!("{:#}", err);
    s.contains("index.lock") || s.contains("Another git process")
}

/// Push the `grove-inboxes` branch to its upstream if one is configured.
/// Returns silently in all cases — the local commit stands either way. On
/// non-ff: fetch + ff-merge + retry push *once*; if it still fails, print
/// remediation to stderr but do not return an error from the capture path.
fn push_best_effort(worktree: &Path) {
    if !has_upstream(worktree, BRANCH) {
        return;
    }
    match try_push(worktree) {
        Ok(()) => {}
        Err(PushError::NonFastForward) => {
            // Disjoint paths under ADR-0004 mean fetch + ff-merge will not
            // textually conflict; safe to auto-retry once.
            if let Err(e) = ff_pull(worktree) {
                eprintln!(
                    "warning: grove-inboxes push rejected as non-ff and ff-merge failed: {} \
                     — commit is local; run `grove meta sync` to resolve.",
                    e
                );
                return;
            }
            match try_push(worktree) {
                Ok(()) => {}
                Err(e) => eprintln!(
                    "warning: grove-inboxes push failed after auto-retry ({:?}); commit is local. \
                     Run `grove meta sync` to publish.",
                    e
                ),
            }
        }
        Err(PushError::Network(msg)) => {
            eprintln!(
                "warning: grove-inboxes push failed ({}); commit is local. \
                 Run `grove meta sync` to publish when network is available.",
                msg.trim()
            );
        }
        Err(PushError::Other(msg)) => {
            eprintln!(
                "warning: grove-inboxes push failed ({}); commit is local. \
                 Run `grove meta sync` to publish.",
                msg.trim()
            );
        }
    }
}

#[derive(Debug)]
pub(crate) enum PushError {
    NonFastForward,
    Network(String),
    Other(String),
}

pub(crate) fn has_upstream(worktree: &Path, branch: &str) -> bool {
    let out = Command::new("git")
        .arg("-C")
        .arg(worktree)
        .arg("config")
        .arg("--get")
        .arg(format!("branch.{}.remote", branch))
        .output();
    matches!(out, Ok(o) if o.status.success() && !o.stdout.is_empty())
}

pub(crate) fn try_push(worktree: &Path) -> std::result::Result<(), PushError> {
    let out = Command::new("git")
        .arg("-C")
        .arg(worktree)
        .arg("push")
        .output();
    let out = match out {
        Ok(o) => o,
        Err(e) => return Err(PushError::Network(e.to_string())),
    };
    if out.status.success() {
        return Ok(());
    }
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    if stderr.contains("non-fast-forward")
        || stderr.contains("[rejected]")
        || stderr.contains("fetch first")
    {
        Err(PushError::NonFastForward)
    } else if looks_like_network_failure(&stderr) {
        Err(PushError::Network(stderr))
    } else {
        Err(PushError::Other(stderr))
    }
}

fn looks_like_network_failure(stderr: &str) -> bool {
    let needles = [
        "Could not resolve host",
        "Connection refused",
        "Connection timed out",
        "Network is unreachable",
        "unable to access",
        "Operation timed out",
    ];
    needles.iter().any(|n| stderr.contains(n))
}

pub(crate) fn ff_pull(worktree: &Path) -> Result<()> {
    let out = Command::new("git")
        .arg("-C")
        .arg(worktree)
        .arg("pull")
        .arg("--ff-only")
        .output()
        .context("running git pull --ff-only")?;
    if !out.status.success() {
        anyhow::bail!(
            "git pull --ff-only failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// content hash, slug, timestamp

fn content_hash_8(body: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(body.as_bytes());
    let digest = hasher.finalize();
    let mut s = String::with_capacity(8);
    for b in &digest[..4] {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

fn derive_slug(body: &str) -> String {
    // Strip leading whitespace, then any markdown header marks (`#`s and the
    // single space that follows), then sanitise the first ~40 characters.
    let mut s = body.trim_start();
    while let Some(rest) = s.strip_prefix('#') {
        s = rest;
    }
    let s = s.trim_start();
    sanitize_slug(&s.chars().take(40).collect::<String>())
}

fn sanitize_slug(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut last_dash = true;
    for c in input.chars() {
        let cl = c.to_ascii_lowercase();
        if cl.is_ascii_alphanumeric() {
            out.push(cl);
            last_dash = false;
        } else if !last_dash {
            out.push('-');
            last_dash = true;
        }
    }
    while out.ends_with('-') {
        out.pop();
    }
    if out.is_empty() {
        "untitled".to_string()
    } else {
        out
    }
}

/// `YYYY-MM-DDTHH:MM:SS` in UTC, computed from `SystemTime` via Howard
/// Hinnant's `civil_from_days`. Inlined so grove avoids depending on
/// `chrono` / `time` for one timestamp format.
fn utc_iso8601_seconds(now: SystemTime) -> String {
    let secs = now.duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0) as i64;
    let days = secs.div_euclid(86400);
    let sod = secs.rem_euclid(86400);
    let (y, mo, d) = civil_from_days(days);
    let h = (sod / 3600) as u32;
    let m = ((sod % 3600) / 60) as u32;
    let s = (sod % 60) as u32;
    format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}", y, mo, d, h, m, s)
}

/// Days since 1970-01-01 → (year, month, day). Public-domain algorithm by
/// Howard Hinnant (`http://howardhinnant.github.io/date_algorithms.html`).
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; // [0, 399]
    let y = (yoe as i64) + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

/// Sync-flavoured push for `grove meta sync`. Pushes once; on non-ff,
/// fetch-and-replays then retries push once; on persistent non-ff returns
/// Err so the cron job exits non-zero. Network failures are warned-and-
/// swallowed (the local commit stands and the next sync will retry).
///
/// Distinct from `push_best_effort` (capture path) which never returns Err
/// because losing a push on a capture write is acceptable; for sync, an
/// unresolvable conflict is the *whole point* of the verb's signal to the
/// operator.
pub(crate) fn sync_push(worktree: &Path) -> Result<()> {
    match try_push(worktree) {
        Ok(()) => Ok(()),
        Err(PushError::Network(msg)) => {
            eprintln!(
                "warning: {} push failed ({}); will retry next sync.",
                BRANCH,
                msg.trim()
            );
            Ok(())
        }
        Err(PushError::NonFastForward) => {
            ff_pull(worktree).context("ff-merge during push retry")?;
            match try_push(worktree) {
                Ok(()) => Ok(()),
                Err(PushError::Network(msg)) => {
                    eprintln!(
                        "warning: {} push failed after ff-merge retry ({}); will retry next sync.",
                        BRANCH,
                        msg.trim()
                    );
                    Ok(())
                }
                Err(e) => anyhow::bail!(
                    "{} push rejected after fetch+ff+retry: {:?}",
                    BRANCH,
                    e
                ),
            }
        }
        Err(PushError::Other(msg)) => anyhow::bail!("git push failed: {}", msg.trim()),
    }
}

#[cfg(test)]
mod inline_tests {
    use super::*;

    #[test]
    fn slug_from_plain_text_takes_first_words() {
        let s = derive_slug("noticed a bug in the racket expander where Pair has wrong arity");
        assert!(s.starts_with("noticed-a-bug"), "got: {s}");
        assert!(s.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'));
    }

    #[test]
    fn slug_strips_markdown_header_prefix() {
        assert_eq!(derive_slug("# Title here"), "title-here");
        assert_eq!(derive_slug("### Heading three"), "heading-three");
    }

    #[test]
    fn slug_collapses_runs_and_trims_dashes() {
        assert_eq!(derive_slug("---hello!!!world---"), "hello-world");
    }

    #[test]
    fn slug_falls_back_to_untitled() {
        assert_eq!(derive_slug("!!! ??? ###"), "untitled");
        assert_eq!(derive_slug(""), "untitled");
    }

    #[test]
    fn slug_override_is_sanitised_too() {
        assert_eq!(sanitize_slug("My Bug Report!"), "my-bug-report");
    }

    #[test]
    fn content_hash_is_stable_eight_hex() {
        let h = content_hash_8("hello world");
        assert_eq!(h.len(), 8);
        assert!(h.chars().all(|c| c.is_ascii_hexdigit()));
        // Known SHA-256("hello world") = b94d27b9934d3e08...; first 8 hex chars:
        assert_eq!(h, "b94d27b9");
    }

    #[test]
    fn utc_iso8601_known_epoch() {
        let t = UNIX_EPOCH;
        assert_eq!(utc_iso8601_seconds(t), "1970-01-01T00:00:00");
    }

    #[test]
    fn utc_iso8601_known_modern_date() {
        // 2026-05-28T12:34:56 UTC
        // Days from 1970-01-01 to 2026-05-28 = 20601
        // Total seconds = 20601*86400 + 12*3600 + 34*60 + 56
        let total = 20601u64 * 86400 + 12 * 3600 + 34 * 60 + 56;
        let t = UNIX_EPOCH + std::time::Duration::from_secs(total);
        assert_eq!(utc_iso8601_seconds(t), "2026-05-28T12:34:56");
    }
}
