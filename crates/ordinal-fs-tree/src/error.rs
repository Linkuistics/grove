//! What the library says when it cannot proceed.
//!
//! Two of these carry the **consumer's own** error value, because the
//! architecture document requires a refusal to say what to *do* about the
//! problem and only the domain knows that: the library halts the whole tree on a
//! `Malformed` or a `Reserved` name wherever it sits, and an error saying only
//! *something is wrong* leaves whoever hit it with a frozen tree and no next
//! step.
//!
//! The rest are the library's own, and each is a case where it can see a problem
//! and has no domain error value with which to report it.

use core::fmt;
use std::path::PathBuf;

use crate::{EntryName, Refusal};

/// Why an operation could not proceed.
///
/// Generic over the name type so that a consumer can match on its **own** error
/// rather than on a string: `Malformed` and `Reserved` carry
/// [`EntryName::Err`] verbatim, and it is reachable through
/// [`std::error::Error::source`] as well.
pub enum Error<N: EntryName> {
    /// The filesystem refused. `doing` is the step, in the imperative, for a
    /// message that reads `reading the tree root /x/y: permission denied`.
    Io {
        /// What was being acted on, in the caller's own spelling. Paths are
        /// never canonicalised, so what comes back is what went in.
        path: PathBuf,
        /// The step that failed.
        doing: &'static str,
        /// What the filesystem said.
        source: std::io::Error,
    },
    /// A name the consumer recognises as its own, and cannot parse. Halts every
    /// operation, wherever in the tree it sits — snapshot scope is the whole
    /// tree, and that blast radius is the point.
    Malformed {
        /// The offending name, in the caller's spelling of its path.
        path: PathBuf,
        /// The consumer's own error, carrying the recovery advice.
        source: N::Err,
    },
    /// A name the consumer owns that is deliberately not an entry — a
    /// transaction witness, a lock marker, a sentinel left by an interrupted
    /// operation. Halts the same way, and for the same reason: the library
    /// cannot know what it means, so proceeding past it is a guess.
    Reserved {
        /// The offending name, in the caller's spelling of its path.
        path: PathBuf,
        /// The consumer's own error, carrying the recovery advice.
        source: N::Err,
    },
    /// The algebra refused: a stated outcome in which the operation changed
    /// nothing.
    ///
    /// A refusal is not an error thrown from inside the algebra — the algebra
    /// *returns* it, as one of the two halves of a decision, and this variant is
    /// where it becomes an `Err` at the filesystem boundary. Every one of them
    /// is total and stated: `ARCHITECTURE.md`'s *Refusals* section is the list,
    /// and [`Refusal`] carries the recovery advice.
    Refused(Refusal),
    /// An effect failed, and the run unwound everything it had applied.
    ///
    /// **The tree is as it was found**: this is *plan atomicity*, which
    /// `operations.qnt` checks as `inv_atomicity` — after a mutation returns an
    /// error, either every effect landed or none did. The promise covers
    /// reported errors and nothing else; a process killed mid-apply is not
    /// recoverable, and the library says so rather than implying otherwise.
    Failed {
        /// What the failing effect was acting on, in the caller's spelling.
        path: PathBuf,
        /// The step that failed, in the imperative.
        doing: &'static str,
        /// What the filesystem said.
        source: std::io::Error,
    },
    /// An effect failed, and unwinding it failed too.
    ///
    /// **The tree is in neither the state it was found in nor the one
    /// intended.** This is the exception *When rollback fails* states, and the
    /// one path by which this library damages a tree it was handed — which is
    /// why it is a variant of its own rather than an `Io` with a longer
    /// message. On the promotion path it leaves a leaf and a node sharing an
    /// ordinal and a key, and the `Display` below says how to resolve that,
    /// because a consumer meeting this needs a next step and not a diagnosis.
    ///
    /// `operations.qnt`'s `rollback_fails` instance exists to exhibit it:
    /// `wit_partialRollbackLeavesADuplicateKey` is reached, and that instance is
    /// the only one that does not claim key uniqueness at rest.
    FailedPartiallyRolledBack {
        /// What the failing effect was acting on.
        path: PathBuf,
        /// The step that failed.
        doing: &'static str,
        /// What the filesystem said about it.
        source: std::io::Error,
        /// What the failing *unwind* step was acting on.
        unwinding: PathBuf,
        /// The unwind step that failed.
        undoing: &'static str,
        /// What the filesystem said about that.
        unwind_source: std::io::Error,
    },
    /// A removal stopped partway, and **nothing was put back**.
    ///
    /// The third of the three failure shapes, and the one a deletion needs
    /// because neither of the other two is honest about it. [`Error::Failed`]
    /// promises *the tree is as it was found — every effect this operation had
    /// applied was undone*, and a removal has nothing to undo: an unlinked file
    /// is gone, and putting it back would mean having copied it aside first,
    /// which is staging machinery this library does not have.
    /// [`Error::FailedPartiallyRolledBack`] is wrong in the other direction —
    /// it reports an unwind that failed, and here no unwind was ever attempted.
    ///
    /// So this variant claims nothing and reports: what stopped it, and the
    /// paths that had already gone. `removed` is **empty** exactly when the
    /// failure came before anything was removed, which is the difference
    /// between a tree that is as it was found and one that is in neither state;
    /// the `Display` below says which.
    ///
    /// *Neither model can pose this*: both hold trees, and neither models a
    /// root that ceases to exist.
    RemovalStopped {
        /// The tree root the removal was asked for, in the caller's spelling.
        root: PathBuf,
        /// What the failing step was acting on.
        path: PathBuf,
        /// The step that failed, in the imperative.
        doing: &'static str,
        /// What the filesystem said.
        source: std::io::Error,
        /// What had already been removed, in the order it went. Paths and not
        /// names, for the reason [`Removed`](crate::Removed) gives.
        removed: Vec<PathBuf>,
    },
    /// A root spelled a way a deletion cannot act on.
    ///
    /// Every other operation uses the root as a **container**, so the kernel
    /// resolving its last component is exactly right: a symbolic link naming a
    /// directory is an accepted spelling of the tree it names, and
    /// `read::containing_directory` goes out of its way to make every spelling
    /// of one tree take one lock.
    ///
    /// A deletion is the one operation that acts on the root as an **object**,
    /// and there the last component decides *which* object — a link and what it
    /// names are two, and only one of them is the tree. It is also the one
    /// operation that removes the very components a path is built from, so a
    /// spelling that descends into the tree and comes back out through `..`
    /// stops resolving partway through its own removal.
    ///
    /// Both are refused before anything is removed, and refused rather than
    /// guessed at: this library will not decide on a caller's behalf whether a
    /// link or its target was meant. `reason` says which spelling it was, so
    /// the message can name the fix.
    RootIsNotSpelledDirectly {
        /// The root as the caller spelled it.
        root: PathBuf,
        /// What is wrong with that spelling, as a clause.
        reason: &'static str,
    },
    /// A filename that is not UTF-8, which halts.
    ///
    /// [`EntryName::parse`] takes a `&str`, so the library cannot ask the
    /// domain about such a name at all — there is no verdict to be had and
    /// therefore no domain error to carry, which is why the advice here is the
    /// library's own. It halts rather than being skipped because skipping is
    /// the failure the parse trichotomy exists to prevent: a hand-edit that
    /// mangles one byte of a real name produces exactly this, and a skipped
    /// *directory* takes its whole subtree out of every traversal while the
    /// tree reports itself healthy. The cost is that a genuinely foreign file
    /// with a non-UTF-8 name freezes the tree too, and that is the same blast
    /// radius `Malformed` already carries.
    NonUtf8Name {
        /// The offending name, lossily rendered, joined to its directory.
        path: PathBuf,
    },
    /// A name rendered as something that is not one filename.
    ///
    /// The seventh obligation — *a name renders as one path component* — and the
    /// only one the library enforces rather than assumes. Everywhere else a
    /// broken obligation is a tree the library quietly corrupts; this one would
    /// take it **outside the tree**, because the rendering is what gets joined to
    /// a level's directory. Occupancy compares views, so the offending name looks
    /// perfectly canonical to the algebra and only the path betrays it.
    ///
    /// Checked at both boundaries where a name becomes a path: every name a
    /// snapshot admits, and every name a plan will place — the latter before any
    /// effect runs, so a plan carrying one changes nothing at all.
    ///
    /// *Neither model can pose this*, in the same position as
    /// [`Refusal::ContentForANode`]: both models hold no strings by design, so
    /// there is no witness behind this variant and none to look for.
    ///
    /// [`Refusal::ContentForANode`]: crate::Refusal::ContentForANode
    NameIsNotOneComponent {
        /// The tree root, in the caller's own spelling.
        root: PathBuf,
        /// What the domain rendered.
        rendered: String,
        /// Why that is not a filename.
        reason: &'static str,
    },
    /// Something is at the tree root that a tree cannot be: a regular file, a
    /// socket, a symbolic link naming one of those, or a dangling one.
    ///
    /// The **third** answer to *is there a tree here*, and the reason that
    /// question is a trichotomy rather than a pair. [`fs::read`] and
    /// [`fs::write`] answer the other two as shapes — a tree, or a vacancy the
    /// caller may initialize — and neither of those is honest about a root
    /// occupied by something else: reporting it as a vacancy would send
    /// `initialize` at a name that is already taken, and reporting it as a tree
    /// would ask the listing for a directory that is not one.
    ///
    /// It is an error and not a variant of either shape for the reason every
    /// halt in this module is one: nothing here can know whether the thing
    /// sitting there is precious, so removing it is not the library's to
    /// choose. `found` is what it turned out to be, so the message can say.
    ///
    /// Decided by **following** symbolic links, because a link naming a
    /// directory is an accepted spelling of a root — see
    /// `fs::read::containing_directory`. A link naming nothing is therefore
    /// [`Found::Other`]: it is not a directory, and it is not nothing either,
    /// since it occupies the name.
    ///
    /// *Neither model can pose this*: both hold trees, and a root that is not a
    /// directory is not a tree for them to hold.
    ///
    /// [`fs::read`]: crate::fs::read
    /// [`fs::write`]: crate::fs::write
    /// [`Found::Other`]: crate::Found::Other
    RootIsNotATree {
        /// The root as the caller spelled it.
        root: PathBuf,
        /// What is there instead of a tree.
        found: crate::Found,
    },
    /// The tree root has no containing directory, so there is nothing to lock.
    ///
    /// The advisory lock is taken on the directory *containing* the root — it
    /// exists before the root is created and persists after it is deleted, so
    /// the tree's creation and destruction fall under the same lock as every
    /// ordinary operation. A filesystem root has no such directory.
    ///
    /// Decided on the filesystem's own identity rather than on the shape of the
    /// path, so `/`, `/..` and a symbolic link naming `/` are refused alike: a
    /// root that *is* its own containing directory has nothing above it to
    /// lock.
    NoContainingDirectory {
        /// The root as the caller spelled it.
        root: PathBuf,
    },
}

// Debug by hand rather than by derive: a derive would bound `N: Debug`, and this
// type mentions `N` only through `N::Err` — which is already `Debug`, since
// `std::error::Error` requires it. The same spurious-bound avoidance `Triple`
// and `Entry` make.
impl<N: EntryName> fmt::Debug for Error<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io {
                path,
                doing,
                source,
            } => f
                .debug_struct("Io")
                .field("path", path)
                .field("doing", doing)
                .field("source", source)
                .finish(),
            Self::Malformed { path, source } => f
                .debug_struct("Malformed")
                .field("path", path)
                .field("source", source)
                .finish(),
            Self::Reserved { path, source } => f
                .debug_struct("Reserved")
                .field("path", path)
                .field("source", source)
                .finish(),
            Self::Refused(refusal) => f.debug_tuple("Refused").field(refusal).finish(),
            Self::Failed {
                path,
                doing,
                source,
            } => f
                .debug_struct("Failed")
                .field("path", path)
                .field("doing", doing)
                .field("source", source)
                .finish(),
            Self::FailedPartiallyRolledBack {
                path,
                doing,
                source,
                unwinding,
                undoing,
                unwind_source,
            } => f
                .debug_struct("FailedPartiallyRolledBack")
                .field("path", path)
                .field("doing", doing)
                .field("source", source)
                .field("unwinding", unwinding)
                .field("undoing", undoing)
                .field("unwind_source", unwind_source)
                .finish(),
            Self::RemovalStopped {
                root,
                path,
                doing,
                source,
                removed,
            } => f
                .debug_struct("RemovalStopped")
                .field("root", root)
                .field("path", path)
                .field("doing", doing)
                .field("source", source)
                .field("removed", removed)
                .finish(),
            Self::RootIsNotSpelledDirectly { root, reason } => f
                .debug_struct("RootIsNotSpelledDirectly")
                .field("root", root)
                .field("reason", reason)
                .finish(),
            Self::NonUtf8Name { path } => {
                f.debug_struct("NonUtf8Name").field("path", path).finish()
            }
            Self::NameIsNotOneComponent {
                root,
                rendered,
                reason,
            } => f
                .debug_struct("NameIsNotOneComponent")
                .field("root", root)
                .field("rendered", rendered)
                .field("reason", reason)
                .finish(),
            Self::RootIsNotATree { root, found } => f
                .debug_struct("RootIsNotATree")
                .field("root", root)
                .field("found", found)
                .finish(),
            Self::NoContainingDirectory { root } => f
                .debug_struct("NoContainingDirectory")
                .field("root", root)
                .finish(),
        }
    }
}

impl<N: EntryName> fmt::Display for Error<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io {
                path,
                doing,
                source,
            } => {
                write!(f, "{doing} {}: {source}", path.display())
            }
            // The consumer's advice *is* the message. Prefixing it with a
            // second sentence of the library's own would push the actionable
            // half off the end of a terminal line.
            Self::Malformed { source, .. } | Self::Reserved { source, .. } => {
                fmt::Display::fmt(source, f)
            }
            // The refusal's own advice *is* the message, for the reason the
            // domain's is: a second sentence in front of it pushes the
            // actionable half off the end of a terminal line.
            Self::Refused(refusal) => fmt::Display::fmt(refusal, f),
            Self::Failed {
                path,
                doing,
                source,
            } => write!(
                f,
                "{doing} {}: {source}. Nothing was changed \u{2014} every effect this \
                 operation had applied was undone.",
                path.display()
            ),
            Self::FailedPartiallyRolledBack {
                path,
                doing,
                source,
                unwinding,
                undoing,
                unwind_source,
            } => write!(
                f,
                "{doing} {}: {source}. Undoing what had already been applied then \
                 failed as well \u{2014} {undoing} {}: {unwind_source} \u{2014} so this tree is \
                 now in neither the state the operation found nor the one it \
                 intended, and it needs a human. If a node and a leaf share an \
                 ordinal and a key and the node holds no distinguished child, \
                 that is an interrupted promotion: removing either half resolves \
                 it.",
                path.display(),
                unwinding.display()
            ),
            // Two sentences and not one, because the two cases want different
            // next steps: a removal that got nowhere left a tree a consumer can
            // still use, and one that got partway left a tree it cannot.
            Self::RemovalStopped {
                root,
                path,
                doing,
                source,
                removed,
            } if removed.is_empty() => write!(
                f,
                "{doing} {}: {source}. Nothing was removed, so the tree {} is as it \
                 was found.",
                path.display(),
                root.display()
            ),
            Self::RemovalStopped {
                path,
                doing,
                source,
                removed,
                root,
            } => write!(
                f,
                "{doing} {}: {source}. {} path{} beneath the tree root {} had already \
                 been removed, and a removal has nothing to put back \u{2014} so this \
                 tree is now in neither the state the operation found nor the one it \
                 intended, and it needs a human. Deleting again removes what is left; \
                 nothing here restores what has gone.",
                path.display(),
                removed.len(),
                if removed.len() == 1 { "" } else { "s" },
                root.display()
            ),
            Self::RootIsNotSpelledDirectly { root, reason } => write!(
                f,
                "the tree root {} {reason}. Nothing was removed. Every other operation \
                 accepts this spelling, because it uses the root as the directory \
                 things are in; a deletion acts on the root itself, so its last \
                 component decides what gets destroyed and this library will not guess \
                 which you meant. Name the directory itself and delete that.",
                root.display()
            ),
            Self::NonUtf8Name { path } => write!(
                f,
                "the filename {} is not valid UTF-8, so it cannot be classified: \
                 a name that cannot be read cannot be disclaimed either, and skipping it \
                 would lose it — and everything beneath it if it is a directory. \
                 Rename it to valid UTF-8, or move it out of the tree.",
                path.display()
            ),
            Self::NameIsNotOneComponent {
                root,
                rendered,
                reason,
            } => write!(
                f,
                "this domain rendered a name as `{rendered}`, which {reason}, so it is \
                 not one filename. A name is exactly one path component: the library \
                 joins it to a level's directory to reach the entry, so this one would \
                 address outside the tree {} \u{2014} whose containing directory is the \
                 only thing the lock covers. Nothing was changed. Fix the domain's \
                 `Display`, `compose` or `parse` so that every name it renders is one \
                 filename, and check it with `conformance::check`, which reports this \
                 before there is a tree.",
                root.display()
            ),
            Self::RootIsNotATree { root, found } => write!(
                f,
                "the tree root {} is {found}, and a tree is a directory. Nothing was \
                 changed: this library will not move aside or replace something it \
                 did not put there, because it cannot know what it is. Move it out \
                 of the way yourself, or name a different root.",
                root.display()
            ),
            Self::NoContainingDirectory { root } => write!(
                f,
                "the tree root {} has no containing directory to lock. \
                 The advisory lock is taken on the directory holding the root, because \
                 that directory outlives the root itself; a filesystem root has none.",
                root.display()
            ),
        }
    }
}

impl<N: EntryName> std::error::Error for Error<N> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::Malformed { source, .. } | Self::Reserved { source, .. } => Some(source),
            // The failing *effect*, not the failing unwind: `source` is a chain
            // of causes and the effect is what caused the unwind to be needed.
            // The unwind's own error is in the `Display`, where a consumer
            // reading the message meets both.
            Self::Failed { source, .. }
            | Self::FailedPartiallyRolledBack { source, .. }
            | Self::RemovalStopped { source, .. } => Some(source),
            Self::Refused(_)
            | Self::RootIsNotSpelledDirectly { .. }
            | Self::NonUtf8Name { .. }
            | Self::NameIsNotOneComponent { .. }
            | Self::RootIsNotATree { .. }
            | Self::NoContainingDirectory { .. } => None,
        }
    }
}
