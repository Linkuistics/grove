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

use crate::EntryName;

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
    /// The tree root has no containing directory, so there is nothing to lock.
    ///
    /// The advisory lock is taken on the directory *containing* the root — it
    /// exists before the root is created and persists after it is deleted, so
    /// the tree's creation and destruction fall under the same lock as every
    /// ordinary operation. A filesystem root has no such directory.
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
            Self::Io { path, doing, source } => f
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
            Self::NonUtf8Name { path } => {
                f.debug_struct("NonUtf8Name").field("path", path).finish()
            }
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
            Self::Io { path, doing, source } => {
                write!(f, "{doing} {}: {source}", path.display())
            }
            // The consumer's advice *is* the message. Prefixing it with a
            // second sentence of the library's own would push the actionable
            // half off the end of a terminal line.
            Self::Malformed { source, .. } | Self::Reserved { source, .. } => {
                fmt::Display::fmt(source, f)
            }
            Self::NonUtf8Name { path } => write!(
                f,
                "the filename {} is not valid UTF-8, so it cannot be classified: \
                 a name that cannot be read cannot be disclaimed either, and skipping it \
                 would lose it — and everything beneath it if it is a directory. \
                 Rename it to valid UTF-8, or move it out of the tree.",
                path.display()
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
            Self::NonUtf8Name { .. } | Self::NoContainingDirectory { .. } => None,
        }
    }
}
