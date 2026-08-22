//! The advisory lock, on the directory *containing* the tree root.
//!
//! Not on the root itself. The containing directory exists before the root is
//! created and persists after it is deleted, so the tree's creation and
//! destruction fall under the same lock as every ordinary operation. That
//! reasoning is general rather than domain-specific, which is why locking is the
//! library's own rule and not a parameter: consumers never mention it.
//!
//! # Nothing here canonicalises a path
//!
//! The lock follows **inode identity through the descriptor**: `flock` attaches
//! to the open file description, so two processes that opened the same directory
//! by different spellings — `/var/x` and `/private/var/x` on macOS, a relative
//! path and an absolute one, a route through a symbolic link — hold the *same*
//! lock without anything comparing their paths. Canonicalising here would buy
//! nothing and cost something visible: every path a read verb returns would come
//! back in a spelling the caller never used, so merely *adding* locking would
//! rewrite the library's output.
//!
//! # The directory is named `<root>/..`, and the kernel resolves it
//!
//! Not `Path::parent`. That is a lexical operation on a string, and `reading-k19`
//! disproved the claim that it converges: the accepted spelling `x/y/..` reads
//! the tree `x/y/..` — that is, `x` — while its lexical parent is `x/y`, which is
//! not the directory `x`'s own spelling locks. A final-component symbolic link
//! has the same shape. Two spellings of one tree took two locks, so a writer
//! through one did not exclude a reader through the other, and the intermediate
//! states a mutation is entitled to leave were observable.
//!
//! `read::containing_directory` therefore hands this module `<root>/..` and lets
//! the kernel resolve it: the root's own components first, symbolic links
//! followed, then one step to the directory that really contains it. That keeps
//! the no-canonicalisation rule intact — the path is still built from the
//! caller's spelling, character for character — while making the *lock* follow
//! the tree rather than the spelling.

use std::fs::File;
use std::io;
use std::os::fd::AsRawFd;
use std::path::Path;

/// Which lock to take.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Mode {
    /// Shared: other readers may hold it at the same time.
    Shared,
    /// Exclusive: nothing else holds it while this does.
    Exclusive,
}

impl Mode {
    const fn operation(self) -> libc::c_int {
        match self {
            Self::Shared => libc::LOCK_SH,
            Self::Exclusive => libc::LOCK_EX,
        }
    }
}

/// Take the lock on `directory`, blocking until it is available.
///
/// The returned [`File`] *is* the lock: `flock` is released when the last
/// descriptor of the open file description is closed, so holding the file is
/// holding the lock and dropping it releases it. There is no unlock call and no
/// unlock path to get wrong.
///
/// Blocking, with no way to ask for a refusal instead, because the architecture
/// document says consumers never mention locking — an API that offered
/// *try-lock* would be an API that mentioned it.
pub(crate) fn take(directory: &Path, mode: Mode) -> io::Result<File> {
    // A read-only open is enough: `flock` is advisory and attaches to the
    // descriptor, not to the file's contents, and a directory cannot be opened
    // for writing anyway.
    let handle = File::open(directory)?;
    let descriptor = handle.as_raw_fd();
    loop {
        // SAFETY: `descriptor` is open for the whole call — `handle` owns it and
        // is still alive — and `flock` touches nothing else.
        let result = unsafe { libc::flock(descriptor, mode.operation()) };
        if result == 0 {
            return Ok(handle);
        }
        let error = io::Error::last_os_error();
        // A signal delivered while waiting is not a failure to lock. Without
        // this the library would report a spurious error to a consumer whose
        // process merely received, say, a window-resize signal.
        if error.kind() != io::ErrorKind::Interrupted {
            return Err(error);
        }
    }
}
