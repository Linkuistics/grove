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
use std::path::Path;
use std::process::Command;

/// Rename the tree entry `src` → `dst` (both relative to `dir`), taking git's
/// index along when git is tracking it. A node directory carries its whole subtree
/// with it. `dst`'s parent directory must already exist.
pub fn rename_entry(dir: &Path, src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    let (src, dst) = (src.as_ref(), dst.as_ref());
    if is_tracked(dir, src) {
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
