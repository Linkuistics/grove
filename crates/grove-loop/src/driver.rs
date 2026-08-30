//! **The two tree operations the loop driver performs that no session verb
//! exposes**, and the reason they are not in [`crate::verbs`].
//!
//! A verb is something a session invokes deterministically mid-task. These are
//! neither: [`transition_to_current`] runs before any session exists, and
//! [`materialize_finish`] creates the one leaf `leaf-add` is forbidden to create
//! (`finish` is driver-reserved). Putting them beside the twelve would say the
//! surface has fourteen verbs, which it does not.
//!
//! `loop-crate-driver-k22` moved `DriverLease`, `compose` and `run` into this
//! crate, so both of these are now [`crate::run`]'s internals and nothing
//! outside the crate calls either in production. **The module stayed public
//! anyway**, and the reason is stated rather than glossed: they are the only way
//! to put a tree into the two states the verb suite has to test against — the
//! pre-loop transition, and the driver-reserved `finish` leaf `verbs::leaf_add`
//! refuses to write. Making them crate-private would have bought nothing the
//! compiler can check and cost a second copy of `tests/verbs.rs`'s jj fixture
//! harness inside the crate.
//!
//! **Both open the tree themselves**, so neither may be called while a
//! [`crate::TreeWrite`] or [`crate::Tree`] is held: two file descriptions on one
//! directory do not share an `flock`, and the call would block against the
//! caller's own opening, forever. That is why they take a worktree path rather
//! than an opening — the driver has no opening to give them, because it runs
//! these *before* it has one.

use std::path::Path;

use crate::{task_tree, tree_lifecycle, Error, Selection};

pub use tree_lifecycle::CurrentTransition;

/// Bring a worktree to a state the loop can drive: a grove, or a fresh one.
///
/// # Errors
///
/// A tree that holds only its charter, or one whose entries are in no grammar
/// grove reads — both of which grove names and refuses rather than repairs
/// (principle 2).
pub fn transition_to_current(worktree: &Path) -> Result<CurrentTransition, Error> {
    Ok(tree_lifecycle::transition_to_current(worktree)?)
}

/// The resumable `finish` leaf for an otherwise empty tree — or the live leaf
/// that turned up under the same lock, in which case nothing is written.
///
/// One observation for both halves: the re-selection that may return early and
/// the append that happens when it does not read the **same** snapshot, so
/// nothing can appear between them.
///
/// # Errors
///
/// A store refusal, or a sentinel created without a key.
pub fn materialize_finish(worktree: &Path) -> Result<Selection, Error> {
    let guard = task_tree::write(&worktree.join(".grove"))?;
    Ok(tree_lifecycle::materialize_finish(guard)?)
}
