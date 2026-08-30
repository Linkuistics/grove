//! **Grove's loop: the task tree in grove's own vocabulary, and the verbs a
//! session invokes over it.**
//!
//! This is the one library crate in the workspace that is allowed to be
//! domain-bound (`docs/specs/module-decomposition.md`, decision 1). The other
//! three say nothing about grove — `ordinal-fs-tree` has an ordered tree,
//! `keyed-launch` has a key and a template, `jj-workspace` has a workspace and a
//! commit — and none of them has a word for a *kind*, a *brief chain*, an
//! *outcome*, a *handle* or *finishing*. Those are here, and so are the twelve
//! verbs stated in them.
//!
//! Since `loop-crate-driver-k22` it is also the **driver**: the one-driver-per
//! -working-tree lease, the prompt composition, the launch configuration grove
//! reads, and [`run`] — the loop itself. What is left outside is one binary per
//! audience, each three functions long.
//!
//! # Opening mirrors the store's, one level up
//!
//! [`read`] and [`write`] answer [`Reading`] and [`Writing`] for the same reason
//! `ordinal_fs_tree::fs::read` and `write` do: a caller cannot scaffold over a
//! live grove or read one that is not there, **because the types do not offer
//! it**. [`verbs::root_init`] takes a [`Vacancy`] and so cannot run over a live
//! grove — that half is absolute, because the vacancy is consumed. The other
//! half is weaker and says so where it lives: a [`Tree`] or a [`TreeWrite`] is
//! proof that a tree was there **when it was opened**, and [`TreeWrite`]'s own
//! header carries what that does and does not buy across two verbs.
//!
//! They take a **worktree**, not a grove root: `<worktree>/.grove` is the only
//! spelling grove has ever opened, and putting the join here means no caller can
//! spell it a second way (`docs/ARCHITECTURE.md#tree-access-lock`).
//!
//! # Three shapes recur across [`verbs`], and each is deliberate
//!
//! * **A verb that reads takes a [`Tree`]; a verb that writes takes a
//!   [`TreeWrite`].** The lock a verb needs is visible in its signature rather
//!   than acquired inside it.
//! * **A search that matched nothing answers [`Sought`]** — the store's word —
//!   rather than an `Option` each verb re-interprets. Reintroducing `Option`
//!   here would move the problem rather than solve it.
//! * **Every verb returns the paths it wrote**, because its caller is a session
//!   that has to name them in a commit message it writes by hand.
//!
//! # One error for the whole crate
//!
//! [`Error`] is opaque, implements `std::error::Error` and `Display`, and is
//! under the same obligation as the runner's: every one names what is wrong
//! **and what fixes it**. Inside, the modules carry `anyhow` — the crate's
//! errors are prose with context stacked on them, which is what `anyhow` is for
//! — and it stops at this boundary, so a consumer takes on no error library of
//! ours.

mod complete;
pub mod driver;
mod driver_lease;
mod loop_driver;
mod task_grow;
mod task_name;
mod task_tree;
mod tree_lifecycle;

pub mod prompt;
pub mod session_config;
pub mod verbs;

/// The version this repository ships, and the only one.
///
/// **One workspace, one release version** (`docs/specs/module-decomposition.md`,
/// decision 1): every member takes `version.workspace = true`, so this constant
/// is the workspace's field however it is reached. The two binaries and the
/// prompt's published version all read it — `crates/grove-llm` would otherwise
/// answer `--version` with a package version of its own, and the prompt would
/// publish one, neither of which names anything an operator can install.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

use std::cell::RefCell;
use std::fmt;
use std::path::{Path, PathBuf};

pub use complete::{interpret, Disposition};
pub use driver_lease::{admit_ambient_session, DriverLease, SessionEpochGuard};
pub use loop_driver::{run, LoopOutcome};
pub use prompt::{compose, Mandate};
pub use session_config::{SessionConfig, TemplateSource};
pub use jj_workspace::{Commit, Workspace};
pub use ordinal_fs_tree::Sought;
pub use task_name::{Handle, HandleError, Kind, Outcome, Parts, Slug, TaskName, TokenError};

/// The task tree, read once under the store's **shared** lock.
///
/// Derefs to its snapshot, so a caller that wants to look at names directly can.
/// Every read verb in [`verbs`] takes one of these.
pub type Tree = task_tree::Tree;

/// The lock over a root that holds **no** tree, and the affordance to create one
/// under it.
///
/// [`verbs::root_init`] consumes one. That it can only be obtained from
/// [`write`] over a genuinely empty root is the whole of grove's refusal to
/// clobber an existing grove — there is no check, because there is no way to
/// call the verb.
pub type Vacancy = task_tree::TreeVacancy;

/// What [`read`] found.
pub enum Reading {
    /// A tree, under the shared lock.
    Tree(Tree),
    /// No tree at this worktree. Not an error: `grove` asks this of a fresh
    /// checkout on every iteration.
    Vacant,
}

/// What [`write`] found — under the **exclusive** lock either way.
pub enum Writing {
    /// A tree, and the affordance to mutate it.
    Tree(TreeWrite),
    /// No tree, and the affordance to create one.
    Vacancy(Vacancy),
}

/// **The right to be the writer** — the surface every mutating verb is on.
///
/// # Why this is a wrapper, and what it is *not*
///
/// A store mutation **consumes** its guard (`crates/ordinal-fs-tree/src/fs/mod.rs`,
/// *A mutation consumes its guard*), so one guard is one operation and a verb
/// signature of `&TreeWrite` is uncallable against that directly. This holds the
/// guard [`write`] opened with, hands it to the first verb that asks, and
/// **reopens** for the next one.
///
/// So it is not *the tree under a lock held for as long as you hold this*. The
/// lock is real from [`write`] until the first verb returns, and after that this
/// value holds nothing until something asks again. Three consequences, and none
/// of them is a detail:
///
/// * **A second verb can find the tree gone.** Between verbs another writer may
///   mutate it, or an operator may remove `.grove/` — in which case the reopen
///   answers the *no tree here* refusal rather than a guard. A `TreeWrite` in
///   hand is not a standing proof that a tree is there; it was one when it was
///   made.
/// * **A second verb waits on a *new* lock**, so it announces contention of its
///   own. The reopen goes through [`task_tree::write`] rather than
///   `reopen_write` for exactly that reason: `reopen_write` skips the diagnostic
///   because *one verb's* later guards are part of a wait already announced, and
///   the gap between two verbs is not that wait.
/// * **Do not hold one while calling anything that opens the tree itself.**
///   [`read`], [`write`], [`verbs::finish_commit`] and both of the driver's own
///   two tree operations take their own lock on a second file description,
///   and two descriptions on one directory do not share an `flock` — so the call
///   blocks forever, against this process. That is the deadlock
///   `collapse-tree-access-k13` deleted a whole layer to remove, and the shape
///   is expressible again as soon as a caller holds a lock across a call. The
///   rule is one sentence: **take the opening you need, spend it, and let it
///   go.**
///
/// The gap between one guard closing and the next opening is exactly the gap
/// `docs/adr/bulk-marks-are-not-atomic.md` records: a subtree prune is *N*
/// rewrites under *N* guards, and grove takes commits rather than implementing
/// transactions (principle 1).
///
/// It is `Send` but **not `Sync`**, and that is the cheap answer rather than a
/// considered one: nothing in this workspace shares one across threads, and
/// making it `Sync` would let two threads race for the same guard — which is the
/// harmful case, where sharing a `&TreeWrite` within one thread is the harmless
/// one. A consumer that needs `Sync` should pass the worktree path and open per
/// use, which is what every caller here does anyway.
pub struct TreeWrite {
    root: PathBuf,
    /// The guard [`write`] opened with, until the first verb takes it.
    ///
    /// `RefCell` rather than `Cell` only because the guard is not `Copy`.
    opened: RefCell<Option<task_tree::Guard>>,
}

impl TreeWrite {
    /// The grove root — `<worktree>/.grove` — in the spelling this was opened
    /// with.
    ///
    /// It is the spelling, not the inode: a reopen re-resolves it, so a worktree
    /// replaced between two verbs is a different directory under the same name.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// One operation's guard: the one this was opened with, or a fresh one.
    ///
    /// **Never call this twice without spending the first**, and never call it
    /// while another guard of this process is live — see the type's own header.
    ///
    /// # Errors
    ///
    /// A tree that has gone since this was opened, or one the store cannot read.
    fn guard(&self) -> Result<task_tree::Guard, Error> {
        // The borrow ends on this line, deliberately and not incidentally: an
        // `if let Some(_) = …borrow_mut().take()` holds the `RefMut` for the
        // whole `if let` under edition 2021, so anything added to that body that
        // touched `self` would panic rather than fail to compile.
        let opened = self.opened.borrow_mut().take();
        match opened {
            Some(guard) => Ok(guard),
            // Announced, because this is a **new** wait: the lock this value was
            // made with has already been released, so a contender that arrived
            // in the gap blocks the caller with nothing said
            // (`docs/ARCHITECTURE.md#tree-access-lock`).
            None => Ok(task_tree::write(&self.root)?),
        }
    }
}

/// Open the grove at `worktree` for reading, or find that there is none.
///
/// # Errors
///
/// A root that is there but unreadable, or a name in it grove refuses — the
/// store halts the whole tree on a name it cannot spell, and [`Error`] carries
/// what is on disk and what it should be.
pub fn read(worktree: &Path) -> Result<Reading, Error> {
    let root = grove_root(worktree);
    match task_tree::read_or_vacant(&root)? {
        task_tree::Vacant::Tree(tree) => Ok(Reading::Tree(tree)),
        task_tree::Vacant::Nothing => Ok(Reading::Vacant),
    }
}

/// Open the grove at `worktree` for writing, or take the lock over the vacancy
/// where one could be created.
///
/// # Errors
///
/// As [`read`]. The lock is taken either way, so a refusal here has already
/// waited for whatever else holds it.
pub fn write(worktree: &Path) -> Result<Writing, Error> {
    let root = grove_root(worktree);
    Ok(match task_tree::write_or_vacancy(&root)? {
        task_tree::Opening::Tree(guard) => Writing::Tree(TreeWrite {
            root,
            opened: RefCell::new(Some(guard)),
        }),
        task_tree::Opening::Vacancy(vacancy) => Writing::Vacancy(vacancy),
    })
}

/// `<worktree>/.grove`, spelled in exactly one place.
fn grove_root(worktree: &Path) -> PathBuf {
    worktree.join(".grove")
}

/// How a session names an existing entry.
///
/// Four forms, and the whole grammar is here: `.` for the grove root, a
/// permanent key (`7` or `[7]`, optionally `[7]-slug`), a [`Handle`]
/// (`<slug>-k<key>`) or a bare slug, and a path — absolute, or relative to the
/// grove root.
///
/// **Which form a reference *is* is decided against the tree, not against the
/// text.** A bare slug and a path are told apart by whether the path exists, and
/// a slug may match several entries — which is why [`verbs::resolve`] answers
/// [`Resolution::Ambiguous`] rather than refusing. What [`Reference::parse`]
/// settles is only that there is something to look for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reference(String);

impl Reference {
    /// Read a reference off a session's command line.
    ///
    /// # Errors
    ///
    /// An empty or blank reference, which names nothing and would otherwise
    /// reach the tree as a slug that cannot match.
    pub fn parse(text: &str) -> Result<Self, Error> {
        if text.trim().is_empty() {
            return Err(Error::msg(
                "an empty reference names nothing. Give `.` for the grove root, a key (`7` or \
                 `[7]`), a handle (`<slug>-k<key>`), a bare slug, or a path under `.grove/`.",
            ));
        }
        Ok(Self(text.to_string()))
    }

    /// The grove root, without going through the text.
    #[must_use]
    pub fn root() -> Self {
        Self(".".to_string())
    }

    /// Whether this reference is the root's own spelling.
    #[must_use]
    pub fn is_root(&self) -> bool {
        self.0 == "."
    }

    /// The text, for a message quoting back what the session asked for.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// The leaf a session was launched to work: its path, its identity, and its
/// kind.
pub type Selection = task_tree::Selection;

/// **One error for the whole crate**, opaque by construction.
///
/// It carries the context stack the modules behind it built, and it is under the
/// obligation `keyed-launch`'s and `jj-workspace`'s errors are under: a message
/// that only reports detection is unfinished, so every one of these names what
/// is wrong **and** what fixes it (principle 2).
///
/// It implements `std::error::Error` with its cause chain intact, so a consumer
/// using `anyhow`, `thiserror` or nothing at all renders it the way it renders
/// any other error — and takes on no dependency of ours to do it.
pub struct Error(anyhow::Error);

impl Error {
    /// An error from a message this crate states itself.
    pub(crate) fn msg(message: impl Into<String>) -> Self {
        Self(anyhow::Error::msg(message.into()))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl fmt::Debug for Error {
    /// The whole chain, the way `anyhow` renders one. `Debug` is what a
    /// `Result`-returning `main` prints, so this is the operator-facing form.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

impl From<anyhow::Error> for Error {
    fn from(error: anyhow::Error) -> Self {
        Self(error)
    }
}
