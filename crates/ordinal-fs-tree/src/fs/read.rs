//! Turning a directory tree into a [`Snapshot`].
//!
//! This is the whole of the parse trichotomy in practice: every name in every
//! directory a walk reaches is handed to the consumer's
//! [`parse`](EntryName::parse) together with **what the listing found under it,
//! unfollowed**, and the three outcomes are the three things that can happen to
//! a name. `Entry` joins the tree. `Foreign` is skipped — and skipped
//! recursively when it is a directory, which is sound precisely because the
//! consumer said the name was not its own. `Malformed` and `Reserved` halt.
//!
//! Snapshot scope is the **whole tree**, so a halt anywhere halts everything.

use std::ffi::OsString;
use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

use crate::snapshot::{Builder, Snapshot};
use crate::{EntryName, Error, Found, Verdict};

/// Read a whole tree, or halt.
pub(super) fn snapshot<N: EntryName>(root: &Path) -> Result<Snapshot<N>, Error<N>> {
    let mut builder = Builder::new();
    // An explicit worklist rather than recursion: the depth of a tree on disk
    // is the user's to choose, and a stack overflow is not a refusal any
    // consumer can handle.
    let mut pending = vec![(root.to_path_buf(), builder.root())];
    while let Some((directory, place)) = pending.pop() {
        let mut descend = Vec::new();
        for (name, found) in listing(&directory).map_err(Unlistable::into_io)? {
            let path = directory.join(&name);
            let Some(name) = name.to_str() else {
                return Err(Error::NonUtf8Name { path });
            };
            match N::parse(name, found) {
                // Not this consumer's name, so it is not this consumer's
                // problem — and not this library's either.
                Verdict::Foreign => {}
                Verdict::Malformed(source) => return Err(Error::Malformed { path, source }),
                Verdict::Reserved(source) => return Err(Error::Reserved { path, source }),
                // A walk descends into recognised nodes and nothing else, and
                // `add` answers with a place exactly for those. A distinguished
                // child is a node's own content rather than a level of the
                // tree, and it is a regular file — a domain holding the
                // obligations cannot produce one that is not.
                Verdict::Entry(parsed) => {
                    // The seventh obligation, enforced at the first of the two
                    // boundaries where a name becomes a path. A snapshot name is
                    // rendered by `entry_path` to reach the entry a move starts
                    // from, and by `level_path` to reach a node a plan writes
                    // into, so one that is not a filename addresses outside the
                    // tree the lock covers. The rendering costs one allocation
                    // per entry, alongside the two the listing already makes,
                    // and it buys the property that *every name in a snapshot is
                    // one path component* — which is what makes both of those
                    // functions safe without repeating the check.
                    let rendered = parsed.to_string();
                    if let Some(reason) = crate::name::not_one_component(&rendered) {
                        return Err(Error::NameIsNotOneComponent {
                            root: root.to_path_buf(),
                            rendered,
                            reason,
                        });
                    }
                    if let Some(below) = builder.add(place, parsed) {
                        descend.push((path, below));
                    }
                }
            }
        }
        // Sorted order is the order the *listing* was read in; pushing the
        // subdirectories in reverse makes the stack pop them in that order, so
        // which of two broken names halts the tree does not depend on where the
        // filesystem happened to put them.
        while let Some(child) = descend.pop() {
            pending.push(child);
        }
    }
    Ok(builder.finish())
}

/// A directory that could not be listed, before either caller has decided what
/// that means.
///
/// [`listing`] has two consumers whose framing of the same failure differs —
/// reading a tree has changed nothing, while removing one may already have
/// removed a great deal — so it hands back the three parts of an error and
/// neither of the two `Error` variants they become.
pub(super) struct Unlistable {
    pub(super) path: PathBuf,
    pub(super) doing: &'static str,
    pub(super) source: io::Error,
}

impl Unlistable {
    /// The reading side's framing: an [`Error::Io`], which claims nothing about
    /// the tree because reading changed nothing.
    pub(super) fn into_io<N: EntryName>(self) -> Error<N> {
        Error::Io {
            path: self.path,
            doing: self.doing,
            source: self.source,
        }
    }
}

/// One directory's names and what is under each, sorted.
///
/// Sorted because the halt has to be deterministic: a tree carrying two names
/// the consumer cannot parse would otherwise report whichever one `read_dir`
/// reached first, so the recovery advice a consumer sees would depend on the
/// filesystem rather than on the tree.
///
/// Shared with [`remove`](super::remove), which needs the same determinism and
/// the same *unfollowed* look at each name. One listing rather than two is what
/// stops those two properties drifting apart between reading a tree and
/// destroying one.
pub(super) fn listing(directory: &Path) -> Result<Vec<(OsString, Found)>, Unlistable> {
    let reading = fs::read_dir(directory).map_err(|source| Unlistable {
        path: directory.to_path_buf(),
        doing: "reading the directory",
        source,
    });
    let mut found = Vec::new();
    for entry in reading? {
        let entry = entry.map_err(|source| Unlistable {
            path: directory.to_path_buf(),
            doing: "reading the directory",
            source,
        })?;
        // `DirEntry::file_type` does not traverse a symbolic link — it is
        // `symlink_metadata`, not `metadata`. That is what makes a symbolic
        // link wearing an entry's name `Found::Other`, and therefore
        // `Malformed`, rather than whatever it points at.
        // <https://doc.rust-lang.org/std/fs/struct.DirEntry.html#method.file_type>
        let kind = entry.file_type().map_err(|source| Unlistable {
            path: entry.path(),
            doing: "inspecting",
            source,
        })?;
        let kind = if kind.is_file() {
            Found::File
        } else if kind.is_dir() {
            Found::Dir
        } else {
            Found::Other
        };
        found.push((entry.file_name(), kind));
    }
    // By name: two entries of one directory cannot share one, so this is total.
    found.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(found)
}

/// What is at the tree root: a tree, nothing at all, or something a tree cannot
/// be.
///
/// The trichotomy [`fs::read`](crate::fs::read) and
/// [`fs::write`](crate::fs::write) answer with, and it is read **under the
/// lock** — that is the whole reason it is a separate step from
/// [`containing_directory`] rather than folded into it. A vacancy that were
/// decided before the lock was taken would be a check-then-act split, and the
/// initialization that follows it would race every other writer.
pub(super) enum Presence {
    /// A directory, which is what a tree is.
    Tree,
    /// Nothing is there. The root may be created under the lock now held.
    Vacant,
    /// Something else is there, and this is what it turned out to be.
    NotATree(Found),
}

/// Which of the three the root is.
///
/// **One observation decides it, and the follow-up only classifies.**
/// `symlink_metadata` answers *is anything here, and what sort of name is it*
/// without following the last component; only where that says **symbolic link**
/// is `metadata` asked, because a link is the one final component the kernel
/// follows and a link naming a directory is an accepted spelling of a root (see
/// [`containing_directory`], and `reading_on_disk.rs`'s round-about spellings).
///
/// Deriving *dangling* from the two calls **disagreeing** is what this shape
/// avoids, and the case that forces it is not exotic: an ordinary directory
/// removed between the two calls answers `symlink_metadata` yes and `metadata`
/// `NotFound`, which is the identical pair a dangling link gives. Read that way,
/// a tree someone deleted underneath is reported as a symbolic link occupying
/// the root — the wrong one of the three answers, and one whose advice names a
/// file that is not there. Asking whether the *first* answer was a link cannot
/// make that mistake: where it was not, `NotFound` from `metadata` is a
/// disappearance, and a disappearance is a vacancy.
pub(super) fn presence<N: EntryName>(root: &Path) -> Result<Presence, Error<N>> {
    let here = match fs::symlink_metadata(root) {
        Ok(here) => here,
        Err(source) if source.kind() == io::ErrorKind::NotFound => return Ok(Presence::Vacant),
        Err(source) => {
            return Err(Error::Io {
                path: root.to_path_buf(),
                doing: "looking at the tree root",
                source,
            })
        }
    };
    if !here.file_type().is_symlink() {
        return Ok(if here.is_dir() {
            Presence::Tree
        } else {
            Presence::NotATree(if here.is_file() {
                Found::File
            } else {
                Found::Other
            })
        });
    }
    match fs::metadata(root) {
        Ok(target) if target.is_dir() => Ok(Presence::Tree),
        Ok(target) => Ok(Presence::NotATree(if target.is_file() {
            Found::File
        } else {
            Found::Other
        })),
        // A link that names nothing. It is not a vacancy — it occupies the name,
        // and an `initialize` sent at it would collide — and `Found::Other` is
        // what an ordinary listing calls the link itself, so this is the same
        // answer from the same place in the vocabulary.
        Err(source) if source.kind() == io::ErrorKind::NotFound => {
            Ok(Presence::NotATree(Found::Other))
        }
        Err(source) => Err(Error::Io {
            path: root.to_path_buf(),
            doing: "reading the tree root",
            source,
        }),
    }
}

/// The directory whose lock covers this tree: the one **containing** the root.
///
/// Asked for as `<root>/..` wherever the root resolves to a directory, and that
/// spelling is the whole of the fix `reading-k19` found necessary. A lexical
/// `Path::parent` chops a component off a string, but `..` and symbolic links
/// are resolved by the kernel, one component at a time, against the directory
/// actually reached — so the accepted spelling
/// `syllabus/02-linear-algebra-i2/..` reads the tree `syllabus` while its
/// lexical parent is `syllabus/02-linear-algebra-i2`, a different directory from
/// the one the direct spelling locks. Two spellings of one tree would then take
/// two locks, and the premise that a snapshot is read under the lock covering it
/// would be false.
///
/// Handing the kernel `<root>/..` makes it resolve the root — following a final
/// symbolic link, because a component in the middle of a path is followed — and
/// then step to that directory's real parent. Every spelling of one tree
/// therefore reaches one inode, and nothing here canonicalises: the path is
/// still built from the caller's own spelling, so what a refusal reports is
/// still what went in.
///
/// # Where no directory resolves, the route is chosen by *one* question
///
/// `<root>/..` is meaningful only when a directory is there — nothing at all and
/// the kernel has no directory to step out of, a regular file and it is
/// `ENOTDIR`. Both of those still have to be lockable: a vacancy because
/// creating the tree happens under the lock, and a root that is not a tree
/// because the message saying so is decided under the lock like every other
/// answer.
///
/// **The question that decides the route is whether the kernel follows the last
/// component**, and that is `symlink_metadata`, not resolvability. Where the
/// last component is *not* a symbolic link — a name with nothing at it, a
/// regular file, a socket — the lexical parent is the directory that component
/// literally sits in, so the two routes cannot disagree, and every component
/// before it is still resolved by the kernel exactly as before. That is the
/// whole of why the fallback is exact rather than approximate: the two spellings
/// that made a lexical parent wrong, a final `..` and a followed final symbolic
/// link, both require the last component to be followed.
///
/// # A **dangling** symbolic link is refused here rather than locked
///
/// It is the one case where the two questions come apart, and reading it as
/// *nothing is there* is what makes it dangerous. Its last component **is**
/// followed, so its lexical parent is the directory holding the *link* while
/// `<root>/..` would be the directory holding the *target* — and if the target
/// appears a moment later, a caller through the link and a caller through the
/// target path hold two different locks over one tree. `reading-k19`'s defect,
/// re-entering through the door absence opened.
///
/// So it is answered before any lock is taken, with the error [`presence`] would
/// have given it: a link naming nothing is not a tree, and there is no operation
/// to protect by locking first. What that costs is that a link which becomes
/// resolvable in the same instant is reported stale — an observation, never a
/// mutation, and a retry sees the tree.
///
/// A link naming something that is not a directory keeps the lexical route: it
/// is not a tree either, so nothing proceeds under whichever lock it took.
///
/// # What a symbolic link spelling still costs, and it is stated rather than fixed
///
/// For a root spelled through a link, `<root>/..` is the directory containing
/// the **target**. That is what makes every spelling of one tree converge, and
/// it is also the price: the lock does not cover creation or deletion of the
/// *link's own name*, and a hand that re-points the link between one operation
/// and the next moves the tree out from under a spelling the caller thinks is
/// stable. That hand is a writer ignoring the advisory lock, which is already
/// outside what this library defends against — the same neighbour `claim_vacant`
/// names — and nothing path-based can defend against it, because nothing here
/// canonicalises or holds the root open.
pub(super) fn containing_directory<N: EntryName>(root: &Path) -> Result<PathBuf, Error<N>> {
    // The spellings with nothing to open at all — a filesystem root, and the
    // empty path. Refused lexically because there is no directory to ask about.
    let Some(lexical) = root.parent() else {
        return Err(Error::NoContainingDirectory {
            root: root.to_path_buf(),
        });
    };
    // `Path::parent` yields the empty path for a one-component root, which names
    // no directory; the directory such a root sits in is the working directory,
    // spelled `.`.
    let lexical = if lexical.as_os_str().is_empty() {
        PathBuf::from(".")
    } else {
        lexical.to_path_buf()
    };
    match fs::symlink_metadata(root) {
        // Nothing at the root: the last component is a plain name, so the
        // lexical parent is the directory it would be created in.
        Err(source) if source.kind() == io::ErrorKind::NotFound => return Ok(lexical),
        // Including `ENOTDIR`, which says a component *before* the last is a
        // regular file. There is no directory anywhere on this path to lock, and
        // opening the lexical parent would `flock` that regular file.
        Err(source) => {
            return Err(Error::Io {
                path: root.to_path_buf(),
                doing: "looking at the tree root",
                source,
            })
        }
        // A plain name that is there and is not a directory: still lexical, and
        // still exact, because nothing about it is followed.
        Ok(here) if !here.file_type().is_symlink() && !here.is_dir() => return Ok(lexical),
        // A directory, spelled directly: the `<root>/..` route below.
        Ok(here) if !here.file_type().is_symlink() => debug_assert!(here.is_dir()),
        Ok(_) => match fs::metadata(root) {
            Ok(target) if target.is_dir() => {}
            Ok(_) => return Ok(lexical),
            Err(source) if source.kind() == io::ErrorKind::NotFound => {
                return Err(Error::RootIsNotATree {
                    root: root.to_path_buf(),
                    found: Found::Other,
                })
            }
            Err(source) => {
                return Err(Error::Io {
                    path: root.to_path_buf(),
                    doing: "reading the tree root",
                    source,
                })
            }
        },
    }
    let directory = root.join("..");
    // And the spellings that *do* open and land back on the root: `/..` is `/`,
    // and so is any symbolic link to it. The identity is the filesystem's own —
    // device and inode — because that is the identity `flock` attaches to, and a
    // lexical rule is exactly what was wrong before.
    // Both sides are `Some` on this path — the match above reached it only for a
    // directory, and `<a directory>/..` always resolves to one — so the
    // comparison is written to require it rather than letting two `None`s read
    // as *the same inode*.
    match (
        directory_identity::<N>(&directory)?,
        directory_identity::<N>(root)?,
    ) {
        (Some(above), Some(here)) if above != here => Ok(directory),
        _ => Err(Error::NoContainingDirectory {
            root: root.to_path_buf(),
        }),
    }
}

/// The pair `flock` attaches to for a path that resolves to a directory, or
/// `None` for one that does not.
///
/// `metadata` follows symbolic links, deliberately — the question is which
/// directory the caller's spelling *names*, not what its last component is
/// stored as.
fn directory_identity<N: EntryName>(path: &Path) -> Result<Option<(u64, u64)>, Error<N>> {
    match fs::metadata(path) {
        Ok(metadata) if metadata.is_dir() => Ok(Some((metadata.dev(), metadata.ino()))),
        Ok(_) => Ok(None),
        // `NotADirectory` is what a *component* of the path being a plain file
        // reports — `<a-regular-file>/..`. It is stable since 1.83, below this
        // workspace's 1.85 floor:
        // <https://doc.rust-lang.org/std/io/enum.ErrorKind.html#variant.NotADirectory>
        Err(source)
            if matches!(
                source.kind(),
                io::ErrorKind::NotFound | io::ErrorKind::NotADirectory
            ) =>
        {
            Ok(None)
        }
        Err(source) => Err(Error::<N>::Io {
            path: path.to_path_buf(),
            doing: "reading the directory containing the tree",
            source,
        }),
    }
}
