//! The one error type, and the reason it is a *refusal* rather than an error.
//!
//! Every operation in this crate either answers the question or declines to
//! act, and a decline is a value the caller can print at a human: it names what
//! is wrong, where, and the command that fixes it. That last part is the whole
//! of why the type exists — an error that only reports detection is unfinished,
//! and a caller that has to synthesise the remedy has to know jj, which is
//! exactly what taking this crate was meant to stop.
//!
//! The remedies named here are **jj's**. This crate has no consumer to speak
//! for, so it never says what the caller should do about the refusal; it says
//! what jj offers.

use std::error::Error;
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

/// Why an operation declined to act.
///
/// Opaque on purpose. A consumer that matched on the shape of a refusal would
/// be encoding this crate's internal case analysis into its own control flow,
/// and every one of these cases is a *stop*: there is nothing to branch on and
/// nothing to recover from in code. What a consumer needs is the message, which
/// [`Display`](fmt::Display) gives it, and a cause chain, which
/// [`Error::source`] gives it.
#[derive(Debug)]
pub struct Refusal(Kind);

#[derive(Debug)]
enum Kind {
    /// No `.jj/` at or above the path — the precondition gate's refusal.
    NotAWorkspace { searched_from: PathBuf },
    /// A `.jj/` was found but the path it sits on could not be made canonical.
    UnresolvablePath { path: PathBuf, cause: io::Error },
    /// The namespace a consumer asked for cannot be given to it.
    Namespace { namespace: String, reason: String },
    /// The control directory could not be created or proved writable.
    ControlDir { path: PathBuf, cause: io::Error },
    /// A path was handed to a workspace that does not contain it.
    OutsideWorkspace { path: PathBuf, root: PathBuf },
    /// An operation whose point is a narrow scope was given none.
    NotScoped { reason: String },
    /// `jj` could not be run at all.
    NotRunnable { command: String, cause: io::Error },
    /// `jj` ran and failed.
    CommandFailed {
        command: String,
        directory: PathBuf,
        stderr: String,
    },
    /// `jj` produced bytes that are not text.
    OutputNotText { command: String },
    /// A commit was attempted and did not land.
    CommitNotRecorded { root: PathBuf, cause: Box<Refusal> },
}

impl Refusal {
    pub(crate) fn not_a_workspace(searched_from: &Path) -> Self {
        Self(Kind::NotAWorkspace {
            searched_from: searched_from.to_path_buf(),
        })
    }

    pub(crate) fn unresolvable_path(path: &Path, cause: io::Error) -> Self {
        Self(Kind::UnresolvablePath {
            path: path.to_path_buf(),
            cause,
        })
    }

    pub(crate) fn namespace(namespace: &str, reason: impl Into<String>) -> Self {
        Self(Kind::Namespace {
            namespace: namespace.to_owned(),
            reason: reason.into(),
        })
    }

    pub(crate) fn control_dir(path: &Path, cause: io::Error) -> Self {
        Self(Kind::ControlDir {
            path: path.to_path_buf(),
            cause,
        })
    }

    pub(crate) fn outside_workspace(path: &Path, root: &Path) -> Self {
        Self(Kind::OutsideWorkspace {
            path: path.to_path_buf(),
            root: root.to_path_buf(),
        })
    }

    pub(crate) fn not_scoped(reason: impl Into<String>) -> Self {
        Self(Kind::NotScoped {
            reason: reason.into(),
        })
    }

    pub(crate) fn not_runnable(command: &str, cause: io::Error) -> Self {
        Self(Kind::NotRunnable {
            command: command.to_owned(),
            cause,
        })
    }

    pub(crate) fn command_failed(command: &str, directory: &Path, stderr: &str) -> Self {
        Self(Kind::CommandFailed {
            command: command.to_owned(),
            directory: directory.to_path_buf(),
            stderr: stderr.trim().to_owned(),
        })
    }

    pub(crate) fn output_not_text(command: &str) -> Self {
        Self(Kind::OutputNotText {
            command: command.to_owned(),
        })
    }

    pub(crate) fn commit_not_recorded(root: &Path, cause: Refusal) -> Self {
        Self(Kind::CommitNotRecorded {
            root: root.to_path_buf(),
            cause: Box::new(cause),
        })
    }
}

impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            // The gate's refusal, and the one a consumer's user is most likely
            // to meet. Both remedies are stated unconditionally rather than
            // chosen by probing for a `.git`: a message that guesses which one
            // applies can guess wrong, and the pair is two lines.
            Kind::NotAWorkspace { searched_from } => write!(
                f,
                "not a Jujutsu working tree\n  \
                 looked for a `.jj` directory at and above: {}\n\n\
                 Make the tree jj-enabled and rerun:\n      \
                 jj git init --colocate     # an existing Git repository, history kept\n      \
                 jj git init                # no repository here yet\n\n\
                 Nothing was created or changed.",
                searched_from.display()
            ),
            Kind::UnresolvablePath { path, cause } => write!(
                f,
                "a `.jj` directory was found at {} but the path could not be resolved: {cause}\n\n\
                 A workspace is identified by its canonical path, so aliases reach one \
                 workspace; check the path for a broken symlink or a directory that has been \
                 removed underneath this process.",
                path.display()
            ),
            Kind::Namespace { namespace, reason } => write!(
                f,
                "cannot reserve the control namespace `{namespace}`: {reason}\n\n\
                 A namespace is one plain directory name, owned by the consumer that asks for \
                 it and kept apart from Jujutsu's own.",
            ),
            Kind::ControlDir { path, cause } => write!(
                f,
                "the control directory {} is not usable: {cause}\n\n\
                 It must exist and be writable before anything can coordinate through it. \
                 Check the permissions on the workspace's `.jj` directory.",
                path.display()
            ),
            Kind::OutsideWorkspace { path, root } => write!(
                f,
                "{} is not inside the Jujutsu workspace rooted at {}\n\n\
                 A workspace answers only for its own files. Resolve the workspace that \
                 contains the path and ask that one.",
                path.display(),
                root.display()
            ),
            Kind::NotScoped { reason } => write!(
                f,
                "a path-scoped operation was given no scope: {reason}\n\n\
                 Name the paths the operation is about. Widening it to the whole working copy \
                 is not the fallback, because the scope is what the caller asked for.",
            ),
            Kind::NotRunnable { command, cause } => write!(
                f,
                "could not run `{command}`: {cause}\n\n\
                 Jujutsu drives this workspace, so its binary has to be on `PATH`. Install it \
                 (https://jj-vcs.github.io/jj/latest/install-and-setup/) and rerun."
            ),
            Kind::CommandFailed {
                command,
                directory,
                stderr,
            } => write!(f, "`{command}` failed in {}: {stderr}", directory.display()),
            Kind::OutputNotText { command } => write!(
                f,
                "`{command}` produced output that is not text, and its answer cannot be read"
            ),
            // The only refusal that has to say something about *state* rather
            // than about a command: the caller asked for a commit and does not
            // have one, so the working copy is holding whatever it prepared.
            // jj owns the repair and this names it; the crate runs none of it.
            Kind::CommitNotRecorded { root, cause } => write!(
                f,
                "the commit did not land in {}: {cause}\n\n\
                 Jujutsu snapshots the working copy before every command and its operation log \
                 is the transaction record, so the state before this attempt is still \
                 reachable:\n      \
                 jj undo                    # reverse the snapshot this attempt recorded\n      \
                 jj op log                  # inspect the operations first, if `jj undo` is \
                 not the one\n\n\
                 Nothing here runs a recovery of its own.",
                root.display()
            ),
        }
    }
}

impl Error for Refusal {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.0 {
            Kind::UnresolvablePath { cause, .. }
            | Kind::ControlDir { cause, .. }
            | Kind::NotRunnable { cause, .. } => Some(cause),
            Kind::CommitNotRecorded { cause, .. } => Some(cause.as_ref()),
            Kind::NotAWorkspace { .. }
            | Kind::Namespace { .. }
            | Kind::OutsideWorkspace { .. }
            | Kind::NotScoped { .. }
            | Kind::CommandFailed { .. }
            | Kind::OutputNotText { .. } => None,
        }
    }
}
