// The task tree's **rename primitive** — the single place grove moves an entry on
// disk. Every verb that renames one goes through here: `leaf-insert`'s sibling
// renumber, `leaf-decompose`'s leaf→`BRIEF.md` promotion, `leaf-retire`'s `DONE`
// infix, `leaf-prune`'s `ABANDONED` infix, and the v1→v2 migration's moves.
//
// **Why it dispatches on trackedness (issue #3).** The grow verbs are
// working-tree changes only: `root-init` and `leaf-add` write files that stay
// *untracked* until the enclosing task's commit folds them in (`SKILL.md`). So the
// ordinary rhythm of a session — grow a few leaves, then reorder, decompose, or
// retire one — routinely hands a rename an entry that git has no index entry for,
// and `git mv`, whose whole job is to move an index entry, fails it with `fatal:
// not under version control`. That was never git hitting a limitation; it was
// grove asking git to move something it had not been told about.
//
// So: move the index entry when there is one, and move only the file when there is
// not. Neither branch is a fallback for the other — each is the correct operation
// for the entry's state:
//
//   * **tracked** ⇒ `git mv`, which renames the file *and* moves the index entry,
//     so the operator's `git status` shows a clean rename before they commit.
//   * **untracked** ⇒ `fs::rename`. There is no index entry to move. The entry was
//     untracked before and is untracked after — at a new name — so the same `git
//     add` that was always going to fold it in still does. No index state is
//     touched, and therefore no new way to lose a file is introduced.
//
// A commit records no rename information either way (git infers renames at diff
// time, by content similarity), so the two branches commit byte-identical trees.

use anyhow::{bail, Context, Result};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Clone, Debug)]
pub(crate) struct GitIndexEntry {
    mode: String,
    object_id: String,
    assume_unchanged: bool,
    skip_worktree: bool,
    fsmonitor_valid: bool,
}

/// Rename the tree entry `src` → `dst` (both relative to `dir`), taking git's
/// index along when git is tracking it. A node directory carries its whole subtree
/// with it. `dst`'s parent directory must already exist.
///
/// In a **jj-enabled** tree (jj-first, colocated included) the rename is always
/// plain: jj has no index to keep in step — it snapshots the working copy — and
/// a `git mv` in a colocated tree would stage into an index jj ignores.
pub fn rename_entry(dir: &Path, src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    let (src, dst) = (src.as_ref(), dst.as_ref());
    if matches!(crate::repo::vcs_of(dir), Some(crate::repo::Vcs::Jj { .. })) {
        plain_rename(dir, src, dst)
    } else if is_tracked(dir, src) {
        git_mv(dir, src, dst)
    } else {
        plain_rename(dir, src, dst)
    }
}

/// Capture the one normal stage-0 entry `git mv` would move. Jujutsu returns
/// `None` even when a colocated Git index contains the path: jj owns that
/// working copy and promotion must not touch its index.
pub(crate) fn capture_git_index_entry(dir: &Path, path: &Path) -> Result<Option<GitIndexEntry>> {
    if matches!(crate::repo::vcs_of(dir), Some(crate::repo::Vcs::Jj { .. })) {
        return Ok(None);
    }
    let output = git_output(
        dir,
        &["ls-files", "--stage", "--full-name", "--"],
        Some(path),
    )?;
    if output.trim().is_empty() {
        return Ok(None);
    }
    let records: Vec<_> = output.lines().collect();
    if records.len() != 1 {
        bail!(
            "cannot promote an unmerged or multiply-staged producer {}: {} index records",
            path.display(),
            records.len()
        );
    }
    let (header, _) = records[0]
        .split_once('\t')
        .context("parsing git ls-files --stage output")?;
    let mut fields = header.split_whitespace();
    let mode = fields.next().context("git index entry has no mode")?;
    let object_id = fields.next().context("git index entry has no object id")?;
    let stage = fields.next().context("git index entry has no stage")?;
    if stage != "0" {
        bail!(
            "cannot promote an unmerged producer {} (index stage {stage})",
            path.display()
        );
    }
    if object_id.chars().all(|character| character == '0') {
        bail!(
            "cannot promote intent-to-add producer {} before it has an index object",
            path.display()
        );
    }

    let ordinary_tag = git_ls_files_tag(dir, "-v", path)?;
    let fsmonitor_tag = git_ls_files_tag(dir, "-f", path)?;
    Ok(Some(GitIndexEntry {
        mode: mode.to_string(),
        object_id: object_id.to_string(),
        assume_unchanged: ordinary_tag.is_ascii_lowercase(),
        skip_worktree: ordinary_tag.eq_ignore_ascii_case(&'S'),
        fsmonitor_valid: fsmonitor_tag.is_ascii_lowercase(),
    }))
}

/// Rewrite the already-staged producer path to its final spelling under one Git
/// index lock while the `PROMOTING-*` filesystem witness still blocks readers.
/// Generated review tasks are deliberately absent from the index.
pub(crate) fn prepare_promotion_index(
    dir: &Path,
    entry: &GitIndexEntry,
    staging_path: &Path,
    final_path: &Path,
) -> Result<()> {
    rewrite_index_path(dir, entry, staging_path, final_path)
        .context("preparing final producer path in the Git index")
}

/// Best-effort normalisation used by rollback: if the final index path already
/// landed, move it back to the original spelling; if the index still names the
/// staging path, `rename_entry` will reverse that ordinary `git mv` itself.
pub(crate) fn restore_promotion_index(
    dir: &Path,
    original_entry: &GitIndexEntry,
    original_path: &Path,
    staging_path: &Path,
    final_path: &Path,
) -> Result<()> {
    if capture_git_index_entry(dir, final_path)?.is_some() {
        rewrite_index_path(dir, original_entry, final_path, original_path)?;
    } else if capture_git_index_entry(dir, staging_path)?.is_none()
        && capture_git_index_entry(dir, original_path)?.is_none()
    {
        bail!(
            "producer index entry is at none of its recoverable paths: {}, {}, {}",
            original_path.display(),
            staging_path.display(),
            final_path.display()
        );
    }
    Ok(())
}

fn rewrite_index_path(
    dir: &Path,
    entry: &GitIndexEntry,
    old_path: &Path,
    new_path: &Path,
) -> Result<()> {
    let old_repo_path = repo_relative_path(dir, old_path)?;
    let new_repo_path = repo_relative_path(dir, new_path)?;
    let repo_root = git_toplevel(dir)?;
    let zero_object_id = "0".repeat(entry.object_id.len());
    let input = format!(
        "0 {zero_object_id}\t{old_repo_path}\n{} {}\t{new_repo_path}\n",
        entry.mode, entry.object_id
    );
    let mut child = Command::new("git")
        .arg("-C")
        .arg(&repo_root)
        .args(["update-index", "--index-info"])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .context("starting git update-index --index-info")?;
    child
        .stdin
        .as_mut()
        .context("git update-index stdin was not piped")?
        .write_all(input.as_bytes())
        .context("writing git index path transaction")?;
    let output = child
        .wait_with_output()
        .context("waiting for git update-index --index-info")?;
    if !output.status.success() {
        bail!(
            "git update-index {} -> {} failed: {}",
            old_path.display(),
            new_path.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    restore_index_flags(Path::new(&repo_root), entry, &new_repo_path)
}

fn restore_index_flags(dir: &Path, entry: &GitIndexEntry, repo_path: &str) -> Result<()> {
    for (enabled, flag) in [
        (entry.assume_unchanged, "--assume-unchanged"),
        (entry.skip_worktree, "--skip-worktree"),
        (entry.fsmonitor_valid, "--fsmonitor-valid"),
    ] {
        if !enabled {
            continue;
        }
        let output = Command::new("git")
            .arg("-C")
            .arg(dir)
            .arg("update-index")
            .arg(flag)
            .arg("--")
            .arg(repo_path)
            .output()
            .with_context(|| format!("restoring Git index flag {flag}"))?;
        if !output.status.success() {
            bail!(
                "git update-index {flag} failed for {repo_path}: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            );
        }
    }
    Ok(())
}

fn git_ls_files_tag(dir: &Path, flag: &str, path: &Path) -> Result<char> {
    let output = git_output(dir, &["ls-files", flag, "--full-name", "--"], Some(path))?;
    output.chars().next().with_context(|| {
        format!(
            "git ls-files {flag} returned no entry for {}",
            path.display()
        )
    })
}

fn repo_relative_path(dir: &Path, path: &Path) -> Result<String> {
    let prefix = git_output(dir, &["rev-parse", "--show-prefix"], None)?;
    let prefix = prefix.trim_end_matches(['\n', '\r']);
    Ok(format!(
        "{}{}",
        prefix,
        path.to_string_lossy().trim_start_matches('/')
    ))
}

fn git_toplevel(dir: &Path) -> Result<String> {
    Ok(git_output(dir, &["rev-parse", "--show-toplevel"], None)?
        .trim()
        .to_string())
}

fn git_output(dir: &Path, args: &[&str], path: Option<&Path>) -> Result<String> {
    let mut command = Command::new("git");
    command.arg("-C").arg(dir).args(args);
    if let Some(path) = path {
        command.arg(path);
    }
    let output = command.output().context("running git index command")?;
    if !output.status.success() {
        bail!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Whether git holds `path` (relative to `dir`) in its index — `git mv`'s
/// precondition. A `false` is the *ordinary* state of an entry grown this session,
/// not a fault. It is also what a missing git or a non-repo directory yields, which
/// routes to the plain rename: the right answer in those cases too, since there is
/// no index to keep in step.
fn is_tracked(dir: &Path, path: &Path) -> bool {
    Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(["ls-files", "--error-unmatch", "--"])
        .arg(path)
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
}

/// `git -C <dir> mv <src> <dst>` — renames the file and moves its index entry, so
/// the pending rename is staged for the enclosing task's commit.
fn git_mv(dir: &Path, src: &Path, dst: &Path) -> Result<()> {
    let out = Command::new("git")
        .arg("-C")
        .arg(dir)
        .arg("mv")
        .arg(src)
        .arg(dst)
        .output()
        .with_context(|| format!("running git mv {} {}", src.display(), dst.display()))?;
    if !out.status.success() {
        bail!(
            "git mv {} -> {} failed: {}",
            src.display(),
            dst.display(),
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(())
}

/// A plain filesystem rename, for an entry git is not tracking.
fn plain_rename(dir: &Path, src: &Path, dst: &Path) -> Result<()> {
    let (from, to) = (dir.join(src), dir.join(dst));
    fs::rename(&from, &to)
        .with_context(|| format!("renaming {} -> {}", from.display(), to.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(bin: &str, dir: &Path, args: &[&str]) -> String {
        let out = Command::new(bin)
            .current_dir(dir)
            .args(args)
            .output()
            .unwrap_or_else(|e| panic!("running {bin} {args:?}: {e} (is {bin} installed?)"));
        assert!(
            out.status.success(),
            "{bin} {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8_lossy(&out.stdout).into_owned()
    }

    fn run_jj(dir: &Path, args: &[&str]) {
        let mut full = vec![
            "--config",
            "user.name=Test",
            "--config",
            "user.email=t@example.com",
        ];
        full.extend_from_slice(args);
        run("jj", dir, &full);
    }

    #[test]
    fn rename_in_jj_native_tree_moves_the_file() {
        // No `.git/` anywhere: the rename must not require git at all.
        // (`git.colocate=false` forced — the ambient jj config may default
        // colocation on, which would sneak a `.git/` into this fixture.)
        let tmp = tempfile::TempDir::new().unwrap();
        let repo = tmp.path();
        run_jj(
            repo,
            &[
                "--config",
                "git.colocate=false",
                "git",
                "init",
                "--quiet",
                ".",
            ],
        );
        fs::write(repo.join("a.md"), "# a\n").unwrap();
        run_jj(repo, &["commit", "-m", "seed"]);

        rename_entry(repo, "a.md", "b.md").unwrap();

        assert!(!repo.join("a.md").exists());
        assert_eq!(fs::read_to_string(repo.join("b.md")).unwrap(), "# a\n");
    }

    #[test]
    fn rename_in_colocated_tree_leaves_the_git_index_alone() {
        // jj-first: in a colocated repo the entry *is* git-tracked, but a
        // `git mv` would stage a rename into an index jj ignores — jj has no
        // index; it snapshots the working copy. So the rename must be plain:
        // the file moves, git's index stays exactly as it was.
        let tmp = tempfile::TempDir::new().unwrap();
        let repo = tmp.path();
        run("git", repo, &["init", "-q", "."]);
        run("git", repo, &["config", "user.email", "t@example.com"]);
        run("git", repo, &["config", "user.name", "Test"]);
        fs::write(repo.join("a.md"), "# a\n").unwrap();
        run("git", repo, &["add", "-A"]);
        run("git", repo, &["commit", "-q", "-m", "seed"]);
        run_jj(repo, &["git", "init", "--colocate", "--quiet", "."]);

        rename_entry(repo, "a.md", "b.md").unwrap();

        assert!(!repo.join("a.md").exists());
        assert_eq!(fs::read_to_string(repo.join("b.md")).unwrap(), "# a\n");
        let index = run("git", repo, &["ls-files"]);
        assert!(
            index.lines().any(|l| l == "a.md"),
            "git index must be untouched (still listing a.md): {index:?}"
        );
        assert!(
            !index.lines().any(|l| l == "b.md"),
            "no git mv may have staged b.md: {index:?}"
        );
    }
}
