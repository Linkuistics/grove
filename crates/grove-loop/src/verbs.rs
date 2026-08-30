//! **The twelve verbs a session invokes over its grove.**
//!
//! They live here rather than with the store because ten of the twelve touch the
//! tree and every one is stated in grove's vocabulary — brief chains, kinds,
//! outcomes, handles, finishing — none of which the store has a word for.
//! Co-locating them gives the handle grammar one owner and puts the driver and
//! the verbs on one definition of a kind. The two that reach outward reach the
//! VCS seam ([`finish_commit`]) and the runner ([`complete`]).
//!
//! The three shapes that recur across the surface — the lock in the signature,
//! [`Sought`] instead of an option, and the paths every verb returns — are the
//! crate root's, and stated there.

use std::io::Write;
use std::path::{Path, PathBuf};

use ordinal_fs_tree::Sought;

use crate::{
    complete, task_grow, task_tree, tree_lifecycle, Commit, Error, Handle, Kind, Reference,
    Selection, Slug, Tree, TreeWrite, Vacancy, Workspace,
};

pub use task_tree::{Located, Resolution};

/// Scaffold a fresh grove: the charter brief and the first leaf, as **one**
/// store operation under one lock.
///
/// It takes the [`Vacancy`] rather than a path, so it cannot run over a live
/// grove — the refusal to clobber is the shape and not a check. `kind` defaults
/// to `requirements` at the CLI: one of the two leaves grove itself authors, and
/// the only kind default that survives anywhere.
///
/// # Errors
///
/// A `finish` kind, which is driver-reserved; or a store that could not create
/// the tree.
pub fn root_init(vacancy: Vacancy, slug: &Slug, kind: &Kind) -> Result<Initialized, Error> {
    let mut paths = tree_lifecycle::root_init(vacancy, slug, kind)?;
    // `root_init` reports the charter first and the leaf second, which is the
    // store's own distinguished-child-first ordering; it has already refused a
    // report of any other shape.
    let first_leaf = paths
        .pop()
        .ok_or_else(|| Error::msg("the store initialized a grove and reported no first leaf"))?;
    let brief = paths
        .pop()
        .ok_or_else(|| Error::msg("the store initialized a grove and reported no charter"))?;
    Ok(Initialized { brief, first_leaf })
}

/// What [`root_init`] wrote.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Initialized {
    /// The grove's charter, `.grove/BRIEF.md`.
    pub brief: PathBuf,
    /// The first leaf, which is what `pick` will answer next.
    pub first_leaf: PathBuf,
}

/// The next leaf to work, or the fact that there is none.
///
/// A recursive depth-first **pre-order** walk: the first live leaf, skipping
/// briefs and terminal leaves — retired (`DONE`) and abandoned (`ABANDONED`)
/// alike. [`Sought::Nothing`] is the **finish trigger**, and it is the store's
/// word rather than an option of the loop's own invention.
///
/// # Errors
///
/// A tree carrying a name grove refuses.
pub fn pick(tree: &Tree) -> Result<Sought<Selection>, Error> {
    Ok(sought(task_tree::select_in(tree)?))
}

/// The kind of a named leaf, or of the picked one when none is named.
///
/// [`Sought::Nothing`] only for the unnamed form over a grove with no live
/// leaves; a named path that is not a leaf is an error, because the caller
/// asserted it was one.
///
/// # Errors
///
/// A path that is not a current-format leaf of this tree.
pub fn kind(tree: &Tree, leaf: Option<&Path>) -> Result<Sought<Kind>, Error> {
    Ok(sought(task_tree::kind_in(tree, leaf)?))
}

/// Every `BRIEF.md` from the grove root down to the leaf, in that order.
///
/// A directory level with no `BRIEF.md` is skipped silently: a node is not
/// obliged to carry a charter.
///
/// # Errors
///
/// A path that is not a leaf of this tree.
pub fn brief_chain(tree: &Tree, leaf: &Path) -> Result<Vec<PathBuf>, Error> {
    Ok(task_tree::brief_chain(tree, leaf)?)
}

/// What a session's reference names.
///
/// **Ambiguity is an answer, not an error**: the caller is a session that can
/// re-ask with a narrower reference, and [`Resolution::Ambiguous`] carries each
/// match's handle so it can.
///
/// # Errors
///
/// A malformed key reference, or a tree carrying a name grove refuses.
pub fn resolve(tree: &Tree, reference: &Reference) -> Result<Sought<Resolution>, Error> {
    Ok(task_tree::resolve_in(tree, reference)?)
}

/// Append one or more leaves under `parent`, all carrying `slug`, as **one**
/// unit: consecutive ordinals, consecutive keys, all of it or none of it.
///
/// A one-kind list is the ordinary add; the research pair is a three-kind one,
/// and the three tokens are the methodology's, not grove's.
///
/// # Errors
///
/// An empty kind list, a `finish` kind, a parent that is not a node, or a store
/// refusal.
pub fn leaf_add(
    tree: &TreeWrite,
    parent: &Reference,
    slug: &Slug,
    kinds: &[Kind],
) -> Result<Vec<PathBuf>, Error> {
    Ok(task_grow::leaf_add(
        tree.guard()?,
        parent.as_str(),
        slug,
        kinds,
    )?)
}

/// Take `target`'s slot, shifting it and every later sibling up by one.
///
/// # Errors
///
/// A `finish` kind, a target that is the root or a brief, or a store refusal.
pub fn leaf_insert(
    tree: &TreeWrite,
    target: &Reference,
    slug: &Slug,
    kind: &Kind,
) -> Result<Inserted, Error> {
    Ok(task_grow::leaf_insert(
        tree.guard()?,
        target.as_str(),
        slug,
        kind,
    )?)
}

pub use task_grow::{Inserted, Renumber};

/// The stale position-prefixed references a [`leaf_insert`] left behind, written
/// to `out`.
///
/// **Not a thirteenth verb**: it is the second half of `leaf-insert`'s contract,
/// which the store cannot supply because nothing in it knows what a reference
/// is. It takes a guard of its own, because the tree it scans is the one the
/// shift *left*.
///
/// # Errors
///
/// A tree that could not be reopened — and **only** that. A write to `out` that
/// fails is dropped rather than returned: the insert has already landed by the
/// time this runs, and a lint that cannot print must not turn a mutation the
/// caller has been told about into a failure.
pub fn surface_cross_refs(
    tree: &TreeWrite,
    renumbered: &[Renumber],
    out: &mut impl Write,
) -> Result<(), Error> {
    if renumbered.is_empty() {
        return Ok(());
    }
    Ok(task_grow::surface_cross_refs(
        tree.guard()?,
        renumbered,
        out,
    )?)
}

/// Turn a leaf into a node, its bytes becoming the node's charter, with one
/// first child.
///
/// `kind` **overrides** the inherited kind rather than defaulting it: with no
/// override the first child is driven as the decomposed leaf was.
///
/// # Errors
///
/// A path that is the root, a brief or an already-terminal leaf; a `finish`
/// kind; or a store refusal.
pub fn leaf_decompose(
    tree: &TreeWrite,
    leaf: &Path,
    first_child: &Slug,
    kind: Option<&Kind>,
) -> Result<Decomposed, Error> {
    let (brief, first_child) =
        tree_lifecycle::leaf_decompose(tree.guard()?, leaf, first_child, kind.cloned())?;
    Ok(Decomposed { brief, first_child })
}

/// What [`leaf_decompose`] wrote.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Decomposed {
    /// The former leaf's bytes, now the node's charter.
    pub brief: PathBuf,
    /// The node's first child, so a node is never childless.
    pub first_child: PathBuf,
}

/// Mark one leaf `DONE` in place. Filename only — the file's bytes do not move.
///
/// # Errors
///
/// A path that is the root, a brief, a node or an already-terminal leaf.
pub fn leaf_retire(tree: &TreeWrite, leaf: &Path) -> Result<PathBuf, Error> {
    Ok(tree_lifecycle::leaf_retire(tree.guard()?, leaf)?)
}

/// Mark abandoned work `ABANDONED` in place: one leaf, or every *live* leaf
/// beneath one node.
///
/// Filename only, and **not atomic across a subtree** — a subtree prune is *N*
/// rewrites under *N* guards, which is what
/// `docs/adr/bulk-marks-are-not-atomic.md` records and why the report below
/// names every path it marked.
///
/// # Errors
///
/// A path that is the root or a brief, or a store refusal partway through — in
/// which case the message names what had already been marked.
pub fn leaf_prune(tree: &TreeWrite, path: &Path) -> Result<Pruned, Error> {
    let result = tree_lifecycle::leaf_prune(tree.guard()?, path)?;
    Ok(Pruned {
        marked: result.marked,
        left_done: result.left_done,
    })
}

/// What [`leaf_prune`] marked, and what it deliberately left alone.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pruned {
    /// Every leaf newly marked `ABANDONED`, in the order it was marked.
    pub marked: Vec<PathBuf>,
    /// Retired leaves in the subtree, left as they were: abandoning work that
    /// was finished would misreport it.
    pub left_done: Vec<PathBuf>,
}

/// Commit the teardown the finish session performed. **Reaches the VCS seam.**
///
/// The tree is revalidated under the exclusive lock, deleted through the store,
/// and the deletion committed — in that order, with the lock held right up to
/// the unlink. Grove takes a commit; it does not implement a transaction
/// (principle 1), so what puts a failed teardown back is `jj undo`, and every
/// refusal here says so.
///
/// **It opens the tree itself**, unlike every other verb here, because the
/// teardown's whole subject is a tree that stops existing — there is nothing to
/// hand back a [`TreeWrite`] over. So do not call it while holding one: two file
/// descriptions on one directory do not share an `flock`, and this would block
/// against the caller's own opening, forever. See [`TreeWrite`]'s header.
///
/// # Errors
///
/// A grove root that is absent, is not a directory, or is a symlink; live work
/// remaining; a handle that is not the live finish leaf's; an untracked tree,
/// which no `jj undo` could restore; or a commit that failed.
pub fn finish_commit(workspace: &Workspace, finish: &Handle) -> Result<Commit, Error> {
    Ok(tree_lifecycle::finish_commit(workspace, finish)?)
}

/// Write the relaunch flag to the signal file and return. **Reaches the
/// runner's channel.**
///
/// Ending the session is the loop driver's job — it is watching for this very
/// channel — so there is nothing else to do here. Outside a loop it is a no-op
/// that says so.
///
/// # Errors
///
/// A signal file that could not be written.
pub fn complete(signal_file: Option<&Path>, done: bool) -> Result<Signalled, Error> {
    let disposition = if done {
        complete::Disposition::Done
    } else {
        complete::Disposition::Relaunch
    };
    match signal_channel(signal_file) {
        Some(path) => {
            complete::signal(&path, disposition)?;
            Ok(Signalled::Wrote(path))
        }
        None => Ok(Signalled::NoLoop),
    }
}

/// Whether [`complete`] would signal, and where.
///
/// **Public because the order matters.** The caller admits this session against
/// the channel it is about to signal, and it has to ask *before* the write — so
/// the answer cannot be something [`complete`] returns.
#[must_use]
pub fn signal_channel(signal_file: Option<&Path>) -> Option<PathBuf> {
    signal_file.map(Path::to_path_buf).or_else(|| {
        std::env::var_os("GROVE_SIGNAL_FILE")
            .filter(|value| !value.is_empty())
            .map(PathBuf::from)
    })
}

/// What [`complete`] did.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Signalled {
    /// The flag was written to this channel; the loop driver will act on it.
    Wrote(PathBuf),
    /// There is no channel — this session is not running under the loop driver,
    /// and whoever started it ends it.
    NoLoop,
}

/// `Option` in, [`Sought`] out — the one place the crate crosses that boundary.
///
/// The modules behind these verbs answer `Option` because they are grove's own
/// internals and Rust's `Option` combinators are what they are written in. What
/// the *surface* answers is the store's word, so no consumer has to invent one.
fn sought<T>(found: Option<T>) -> Sought<T> {
    match found {
        Some(value) => Sought::Match(value),
        None => Sought::Nothing,
    }
}
