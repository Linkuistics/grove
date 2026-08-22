//! The operations' algebra: one planning function per operation, each a pure
//! function of a [`Snapshot`].
//!
//! Nothing here touches a filesystem, and `tests/algebra_has_no_filesystem.rs`
//! is what makes that a seam rather than a convention. What a planning function
//! returns is a [`Decision`] — a plan or a refusal, and no third answer — which
//! the filesystem layer then applies through the one interpreter.
//!
//! The specification is `docs/ordinal-fs-tree/ARCHITECTURE.md`, sections
//! *Operations → Mutating* and *Refusals*; the model is
//! `docs/ordinal-fs-tree/models/operations.qnt`, whose `planAppend` and
//! `planAppendMany` these two are.
//!
//! The other three mutations — `insert`, `promote` and `rewrite` — join this
//! module as their leaves land. They are what the plan shape exists for: five
//! operations, one interpreter, one rollback.

use crate::plan::{Decision, Effect, Level, Plan, Refusal};
use crate::{Container, EntryName, Key, Ordinal, PositionedSpecies, Snapshot};

/// Which entry an operation is aimed at.
///
/// **By key, and by nothing else.** An ordinal is stale the moment anything is
/// inserted before it and a path is stale the moment anything is renamed; the
/// key is the one handle the design promises survives insertion, reordering,
/// relabelling and being moved between levels. The root has no key, because it
/// is a node and not an entry, so it takes a variant of its own.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    /// The tree root: the directory the consumer handed the library.
    Root,
    /// The entry with this key.
    Key(Key),
}

/// An entry an operation is being asked to create: the parts its name will
/// carry, and the bytes it will hold.
///
/// The species is *not* here, because it follows from the parts and from
/// nothing else. A node carries no bytes — a directory has nowhere to put them
/// — and supplying some is [`Refusal::ContentForANode`] rather than a silent
/// discard.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewEntry<P> {
    /// Everything the library does not understand, and the only thing that
    /// decides whether this becomes a file or a directory.
    pub parts: P,
    /// The bytes of a leaf, written verbatim. The library has no content model:
    /// templates, headers and formats are the consumer's.
    pub content: Vec<u8>,
}

impl<P> NewEntry<P> {
    /// A leaf's parts and its bytes.
    #[must_use]
    pub const fn new(parts: P, content: Vec<u8>) -> Self {
        Self { parts, content }
    }

    /// Parts and no bytes — what a node takes, and what an empty leaf takes.
    #[must_use]
    pub const fn empty(parts: P) -> Self {
        Self {
            parts,
            content: Vec::new(),
        }
    }
}

/// **`append`**: add a child at the end of a level — the next free ordinal, a
/// fresh key.
///
/// One `append_many` of one entry, deliberately rather than a second
/// implementation of the same arithmetic: the model's `planAppend` and
/// `planAppendMany` agree on a single-element run by inspection, and two
/// spellings of one rule are two things to keep in step. What `append_many`
/// adds is a *run* — which is why it, and not this, is what makes atomicity
/// observable.
pub(crate) fn append<N: EntryName>(
    snapshot: &Snapshot<N>,
    target: Target,
    entry: NewEntry<N::Parts>,
) -> Decision<N> {
    append_many(snapshot, target, vec![entry])
}

/// **`append_many`**: add several children at consecutive ordinals with
/// consecutive keys, planned from one snapshot and applied as a unit.
///
/// Either the whole run lands or none of it does, and that is the interpreter's
/// doing rather than this function's: what happens here is that *one* snapshot
/// answers every ordinal and every key, so the run is contiguous by
/// construction. Calling `append` in a loop would take a fresh snapshot each
/// time and could interleave with anything else holding the lock in between.
///
/// An empty run is a plan with no effects, which succeeds and changes nothing.
pub(crate) fn append_many<N: EntryName>(
    snapshot: &Snapshot<N>,
    target: Target,
    entries: Vec<NewEntry<N::Parts>>,
) -> Decision<N> {
    let (level, container) = match resolve(snapshot, target) {
        Ok(resolved) => resolved,
        Err(refusal) => return Decision::Refuse(refusal),
    };
    // Both counters come from this one snapshot, and both are read before any
    // effect is built: `maxOrdIn(f, d)` and `freshKey(f)` in the model, taken
    // once and walked forward. The ordinal is the level's, the key is the whole
    // tree's — the names *are* the counter, and there is no file recording the
    // next value because such a file is a second source of truth a hand edit
    // can desynchronise.
    let mut ordinal = last_ordinal(&container);
    let mut key = greatest_key(snapshot);
    let mut effects = Vec::with_capacity(entries.len());
    for entry in entries {
        if N::positioned_species(&entry.parts) == PositionedSpecies::Node
            && !entry.content.is_empty()
        {
            return Decision::Refuse(Refusal::ContentForANode);
        }
        let (Some(next_ordinal), Some(next_key)) = (ordinal.checked_add(1), key.checked_add(1))
        else {
            return Decision::Refuse(if key.checked_add(1).is_none() {
                Refusal::KeysExhausted
            } else {
                Refusal::OrdinalsExhausted
            });
        };
        ordinal = next_ordinal;
        key = next_key;
        effects.push(Effect::Create {
            at: level,
            name: N::compose(Ordinal::new(ordinal), Key::new(key), entry.parts),
            content: entry.content,
        });
    }
    // Guarded like every other plan, though nothing an `append` builds can
    // reach the refusal: every name it composes carries a key no entry in the
    // tree has, so no destination it computes can be taken. The guard is here
    // because it belongs to *plans*, not to operations — `insert` and `promote`
    // are what make it live — and because leaving it off here would make this
    // the one operation whose plan is unchecked.
    Plan::of(effects).guarded(snapshot)
}

/// The level an operation's target names, and the identity a plan refers to it
/// by.
fn resolve<N: EntryName>(
    snapshot: &Snapshot<N>,
    target: Target,
) -> Result<(Level, Container<'_, N>), Refusal> {
    match target {
        Target::Root => Ok((Level::Root, snapshot.root())),
        Target::Key(key) => {
            // `by_key` answers with positioned entries only — a distinguished
            // child has no key — so the refusal below is about a leaf, and a
            // distinguished child cannot be named here at all.
            let Some(entry) = snapshot.by_key(key) else {
                return Err(Refusal::TargetMissing { key });
            };
            let Some(container) = entry.contents() else {
                return Err(Refusal::TargetNotNode {
                    key,
                    species: entry.species(),
                });
            };
            Ok((Level::Entry(entry.index()), container))
        }
    }
}

/// The greatest ordinal in one level, or `0` for a level holding no positioned
/// children — so that the first append into it lands on [`Ordinal::FIRST`].
///
/// The *greatest*, and never the count: density is preserved by every operation
/// and established by none, so a level a hand edit left gapped keeps its gap
/// forever. Counting instead would quietly fill it and collide.
fn last_ordinal<N: EntryName>(container: &Container<'_, N>) -> u32 {
    container
        .positioned()
        .filter_map(|entry| entry.ordinal())
        .map(Ordinal::get)
        .max()
        .unwrap_or(0)
}

/// The greatest key in the **whole tree**, or `0` for a tree holding none.
///
/// Whole-tree and not per-level: a key is unique tree-wide, so `max + 1` has to
/// see every name there is. This is also the reason there is no removal
/// operation — deleting an entry lowers this maximum, and the next allocation
/// re-issues a key that other entries may still reference.
fn greatest_key<N: EntryName>(snapshot: &Snapshot<N>) -> u32 {
    snapshot
        .walk()
        .filter_map(|entry| entry.key())
        .map(Key::get)
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests;
