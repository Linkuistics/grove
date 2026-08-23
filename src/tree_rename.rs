// The task tree's **rename primitive**, for the verbs the flip has not yet moved
// — `leaf-insert`'s sibling renumber, `leaf-decompose`'s leaf→`BRIEF.md`
// promotion, and the v1→v2 migration's moves. `sweep-k37` deletes this module
// when the last of them has gone.
//
// **It is no longer *the* place grove moves an entry, and the difference is the
// point.** `leaf-retire` and `leaf-prune` mark through `ordinal-fs-tree`, which
// renames with `rename(2)`, detects no repository and stages nothing — so on the
// Git lane a mark now leaves an unstaged deletion beside an untracked file where
// this module would have staged a rename. That is the decision, not an
// oversight: `docs/adr/grove-does-not-stage-its-own-renames.md` says why grove
// does not buy the old status output back from outside the library, and
// `tests/leaf_ops.rs` asserts what an operator sees instead. Nothing below is
// authority for how a flipped verb should behave.
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
use std::path::Path;
use std::process::Command;

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
