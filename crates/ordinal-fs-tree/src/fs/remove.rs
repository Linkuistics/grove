//! Removing a tree root and everything beneath it.
//!
//! The one mutation that is not a [`Plan`](crate::plan::Plan), and the reason is
//! that a plan is a list of effects over *names* — values the domain produced
//! and the algebra can reason about. A deletion acts on the root, so it acts on
//! everything that is there, including the entries the domain declined to parse
//! as its own. Those have no name in the algebra's sense and never appear in a
//! [`Snapshot`](crate::Snapshot), so the removal reads the directories itself.
//!
//! # There is nothing to unwind
//!
//! Every other operation's failure path is *put back what this run did*. A
//! removal has nothing to put back: an unlinked file is gone, and a library that
//! wanted one back would have had to copy it aside first — which is the staging
//! machinery this design does not have and does not want. So the failure is
//! reported as [`Error::RemovalStopped`], which claims neither of the two things
//! [`Error::Failed`] and [`Error::FailedPartiallyRolledBack`] claim: it says how
//! far the removal got, and stops.
//!
//! # Following no link is the security property
//!
//! Descent is decided by [`read::listing`](super::read::listing), whose
//! `Found` comes from `DirEntry::file_type` — `symlink_metadata`, not
//! `metadata`. A symbolic link naming a directory is therefore [`Found::Other`]
//! and is *unlinked*, never descended into, so a link pointing outside the tree
//! costs its target nothing. Nothing here follows a link **inside the tree** —
//! and the qualifier is load-bearing, not hedging: the root's own last component
//! is named by the caller rather than found by the walk, and it is the next
//! paragraph's subject.
//!
//! **The root's own last component is the exception, and it is refused rather
//! than followed.** Everything above is about the entries *inside* the tree; the
//! root itself is named by the caller, and every other operation lets the kernel
//! resolve that name — a link naming a directory is an accepted spelling of the
//! tree. A deletion cannot accept it, because it acts on the root as an object
//! rather than as a container. See [`spelled_directly`].
//!
//! **The bound on the no-link claim, stated rather than hidden.** The look and
//! the descent are two syscalls, so a hand that replaces a directory with a link
//! *between* them is not defeated by this — the same window
//! [`claim_vacant`](super::apply) already names, and the same neighbour: a
//! writer that ignores the advisory lock. `std::fs::remove_dir_all` closes it
//! with `openat`-based descent and reports nothing about what it removed, which
//! is the whole of what this operation is for; a consumer that wants the race
//! closed and no report has the standard library's own call.

use std::fs;
use std::path::{Component, Path, PathBuf};

use super::read;
use crate::{EntryName, Error, Found, Removed};

/// Remove everything beneath the root, then the root.
///
/// Post-order: a level goes only after everything it holds, because a directory
/// cannot be removed while anything is in it. Within a level the order is the
/// listing's own — sorted — for determinism and for nothing else.
///
/// **No order here buys a property, and that is worth saying beside the shift
/// rule, which does.** A sibling shift runs highest-first so that an interrupted
/// run leaves a merely *gapped* level, a shape this design admits everywhere. An
/// interrupted removal leaves a tree with entries missing and its key maximum
/// lowered, which is not a shape this design admits in any order — so the order
/// is chosen to be reproducible rather than to be safe.
pub(super) fn tree<N: EntryName>(root: &Path) -> Result<Removed, Error<N>> {
    // Before anything goes, so a refused spelling removes nothing at all.
    spelled_directly(root)?;
    let mut entries: Vec<PathBuf> = Vec::new();
    // An explicit worklist rather than recursion, for the reason
    // `read::snapshot` gives: the depth of a tree on disk is the operator's to
    // choose, and a stack overflow is not a refusal any consumer can handle.
    let mut pending = Vec::new();
    // The root's own children, and not `descend` — the root's removal is the
    // last line of this function rather than a step on the worklist, because it
    // is reported on its own field.
    if let Err(stopped) = children(root, &mut pending) {
        return Err(stopped.error(root, entries));
    }
    while let Some(step) = pending.pop() {
        let outcome = match &step {
            Step::Descend(directory) => descend(directory, &mut pending),
            Step::Unlink(path) => unlink(path),
            Step::RemoveLevel(path) => remove_level(path),
        };
        if let Err(stopped) = outcome {
            return Err(stopped.error(root, entries));
        }
        match step {
            // Descending removes nothing; what it found is on the worklist.
            Step::Descend(_) => {}
            Step::Unlink(path) | Step::RemoveLevel(path) => entries.push(path),
        }
    }
    // The root is not one of the entries — it is the level they were in, and it
    // has no name the domain ever parsed. It goes last because nothing else
    // could go after it, and it is reported on its own field for the same
    // reason `initialize` puts no row in the report for creating it.
    if let Err(stopped) = remove_level(root) {
        return Err(stopped.error(root, entries));
    }
    Ok(Removed {
        root: root.to_path_buf(),
        entries,
    })
}

/// The one precondition a deletion has that no other operation does: the root's
/// spelling must **name the root**, and must stay resolvable while the root is
/// being taken apart.
///
/// # Why this operation and no other
///
/// Everywhere else the root is a **container**, and the kernel resolving its
/// last component is exactly right — `read::containing_directory` goes to some
/// trouble to make a symbolic link naming a directory, and a spelling ending in
/// `..`, reach the same tree and take the same lock as the direct spelling.
/// This is the one operation that acts on the root as an **object**, and there
/// the last component decides *which* object. A link and what it names are two
/// things, and destroying the second while leaving the first is not what either
/// answer would have been.
///
/// It is also the one operation that removes the components a path is built
/// from. `syllabus/topic/..` names the tree perfectly well until the walk
/// removes `topic`, after which every path built on that spelling stops
/// resolving — with the tree half destroyed and nothing left that can finish
/// the job.
///
/// # The two conditions, and why they are between them complete
///
/// **No `..` may cancel a name.** With that rule, every component before the
/// last resolves to a strict *ancestor* of the root, and the walk removes only
/// what is at or below the root — so no component of the spelling is one the
/// removal can take away. A **leading** `..` cancels nothing and is fine:
/// `../course` is an ordinary spelling and is accepted.
///
/// The rule is **coarser than the danger, deliberately**. A `..` cancelling a
/// component *above* the tree — `/a/../b/course` — is harmless, and is refused
/// with the rest of the class. Telling the two apart means resolving the path to
/// learn which components lie inside the tree, and this module resolves nothing;
/// the cost of the coarse rule is one message asking for a direct spelling, and
/// the cost of the precise one is a resolution step on the destructive path.
///
/// **The last component must be a name, and must not be a symbolic link.** That
/// leaves exactly one object it can mean. `.` and a bare separator name no
/// object to remove at all — `rmdir(".")` is `EINVAL`, and refusing it with a
/// sentence beats passing that through.
///
/// Both are decided on a path rebuilt from its own components, because a
/// trailing separator makes `symlink_metadata` resolve the final link — which
/// is how `link/` would otherwise slip past the very check that exists to catch
/// `link`.
fn spelled_directly<N: EntryName>(root: &Path) -> Result<(), Error<N>> {
    let mut previous_was_a_name = false;
    for component in root.components() {
        if previous_was_a_name && component == Component::ParentDir {
            return Err(Error::RootIsNotSpelledDirectly {
                root: root.to_path_buf(),
                reason: "descends into a name and comes back out through `..`, so one of its \
                         own components is something this removal would take \
                         away — after which nothing else under the root \
                         resolves, and the tree is left half gone with no \
                         spelling that can finish it",
            });
        }
        previous_was_a_name = matches!(component, Component::Normal(_));
    }
    let direct: PathBuf = root.components().collect();
    if !matches!(direct.components().next_back(), Some(Component::Normal(_))) {
        return Err(Error::RootIsNotSpelledDirectly {
            root: root.to_path_buf(),
            reason: "does not end in a name, so its last component names no object \
                     this operation could remove",
        });
    }
    let here = fs::symlink_metadata(&direct).map_err(|source| Error::Io {
        path: root.to_path_buf(),
        doing: "looking at the tree root",
        source,
    })?;
    if here.file_type().is_symlink() {
        return Err(Error::RootIsNotSpelledDirectly {
            root: root.to_path_buf(),
            reason: "is a symbolic link, and a link is not the tree it names",
        });
    }
    Ok(())
}

/// One thing left to do.
enum Step {
    /// List this directory and schedule what is in it, then remove it.
    Descend(PathBuf),
    /// Remove something that is not a directory — a file, a socket, or a
    /// symbolic link **as a link**.
    Unlink(PathBuf),
    /// Remove a directory this walk has already emptied.
    RemoveLevel(PathBuf),
}

/// Schedule a directory's own removal, and then its contents.
///
/// Pushing the level's removal **first** is what makes the worklist post-order:
/// everything pushed after it pops before it.
fn descend(directory: &Path, pending: &mut Vec<Step>) -> Result<(), Stopped> {
    pending.push(Step::RemoveLevel(directory.to_path_buf()));
    children(directory, pending)
}

/// Schedule what is in a directory, in the listing's own sorted order.
///
/// The children go on in *reverse* of that order so that popping restores it,
/// which is the same trick — and the same reason — as `read::snapshot`'s.
fn children(directory: &Path, pending: &mut Vec<Step>) -> Result<(), Stopped> {
    let found = read::listing(directory).map_err(Stopped::from_unlistable)?;
    for (name, found) in found.into_iter().rev() {
        let path = directory.join(name);
        pending.push(match found {
            Found::Dir => Step::Descend(path),
            // A symbolic link — even one naming a directory — is `Found::Other`,
            // because the listing did not follow it. Unlinking is what removes
            // the link and leaves whatever it named alone.
            Found::File | Found::Other => Step::Unlink(path),
        });
    }
    Ok(())
}

fn unlink(path: &Path) -> Result<(), Stopped> {
    fs::remove_file(path).map_err(|source| Stopped {
        path: path.to_path_buf(),
        doing: "removing",
        source,
    })
}

fn remove_level(path: &Path) -> Result<(), Stopped> {
    // `remove_dir` and never `remove_dir_all`: this walk has already emptied the
    // directory, so a refusal here is a check — something arrived under it while
    // the removal was running — rather than an obstacle to route around.
    fs::remove_dir(path).map_err(|source| Stopped {
        path: path.to_path_buf(),
        doing: "removing the directory",
        source,
    })
}

/// One removal, or one listing, that did not happen.
struct Stopped {
    path: PathBuf,
    doing: &'static str,
    source: std::io::Error,
}

impl Stopped {
    fn from_unlistable(unlistable: read::Unlistable) -> Self {
        Self {
            path: unlistable.path,
            doing: unlistable.doing,
            source: unlistable.source,
        }
    }

    /// The removal's framing of a failure: what stopped it, and how far it had
    /// got. `removed` is moved in rather than counted, because a caller that has
    /// to say what it destroyed needs the paths and not the number.
    fn error<N: EntryName>(self, root: &Path, removed: Vec<PathBuf>) -> Error<N> {
        Error::RemovalStopped {
            root: root.to_path_buf(),
            path: self.path,
            doing: self.doing,
            source: self.source,
            removed,
        }
    }
}
