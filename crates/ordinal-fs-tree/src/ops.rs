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
//! `docs/ordinal-fs-tree/models/operations.qnt`, whose `plan…` functions these
//! are.
//!
//! `planAppend`, `planAppendMany`, `planInsert`, `planPromote` and
//! `planRewrite` — the whole of the model's operation set. They are what the
//! plan shape exists for: five operations, one interpreter, one rollback.

use crate::plan::{Decision, Effect, Level, Plan, Refusal};
use crate::{
    Container, Entry, EntryName, Key, Ordinal, PositionedSpecies, Snapshot, Sought, Species,
};

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
    let effects = match creations(snapshot, level, &container, entries) {
        Ok(effects) => effects,
        Err(refusal) => return Decision::Refuse(refusal),
    };
    // Guarded like every other plan, though nothing an `append` builds can
    // reach the refusal: every name it composes carries a key no entry in the
    // tree has, so no destination it computes can be taken. The guard is here
    // because it belongs to *plans*, not to operations — `insert` and `promote`
    // are what make it live — and because leaving it off here would make this
    // the one operation whose plan is unchecked.
    Plan::of(effects).guarded(snapshot)
}

/// **`initialize`**: the first names a tree ever holds — a distinguished child
/// carrying the root's own content, and a run of first entries at the root
/// level.
///
/// Planned against an **empty** snapshot, because that is exactly what a vacancy
/// is: a tree with no names, which is also what the arithmetic wants. The run
/// starts at [`Ordinal::FIRST`] with key 1 because [`creations`] reads the
/// level's greatest ordinal and the tree's greatest key off a snapshot holding
/// neither, so no branch anywhere says *this is the first one*.
///
/// # The distinguished child is an ordinary effect
///
/// It has to be, and that it can be is the reason this operation needs no new
/// trait method. A distinguished child carries no ordinal and no key, so
/// [`NewEntry`] cannot describe it and [`EntryName::compose`] cannot build one —
/// but [`EntryName::distinguished`] names it, and the library already places one
/// that way when a promotion moves a leaf's bytes into a new node. So the same
/// [`Effect::Create`] does it here, at [`Level::Root`], and
/// `docs/adr/entry-name-is-the-only-seam.md` is untouched.
///
/// Bytes are `Option<Vec<u8>>` and not `Vec<u8>` because *no distinguished
/// child* and *an empty one* are different trees, and a domain that has one
/// should still be able to make a root without it.
///
/// # There is no root effect here
///
/// The root directory itself is not an entry: it has no name, so no effect can
/// place it and no [`Report`](crate::Report) row can describe it. Creating it is
/// the filesystem layer's, immediately before this plan is applied and under the
/// lock the vacancy already holds — see [`Vacancy::initialize`].
///
/// [`Vacancy::initialize`]: crate::fs::Vacancy::initialize
pub(crate) fn initialize<N: EntryName>(
    snapshot: &Snapshot<N>,
    distinguished: Option<Vec<u8>>,
    entries: Vec<NewEntry<N::Parts>>,
) -> Decision<N> {
    let mut effects = Vec::with_capacity(entries.len() + 1);
    if let Some(content) = distinguished {
        // The same refusal a promotion gives, for the same reason: this domain
        // has no distinguished child and these bytes have nowhere to go. Asked
        // before the entries are looked at, exactly as `promote` asks before it
        // looks at the parts — the answer is about the *domain*, so complaining
        // about the rest of a call that could not have worked is less useful.
        let Some(name) = N::distinguished() else {
            return Decision::Refuse(Refusal::NoDistinguishedChild { promoting: None });
        };
        effects.push(Effect::Create {
            at: Level::Root,
            name,
            content,
        });
    }
    match creations(snapshot, Level::Root, &snapshot.root(), entries) {
        Ok(created) => effects.extend(created),
        Err(refusal) => return Decision::Refuse(refusal),
    }
    // Guarded like every other plan, and like `append`'s nothing it builds can
    // reach the refusal: the level is empty, every composed name carries a
    // distinct fresh key, and a distinguished child is never `same_name` as a
    // positioned one. The guard is here because it belongs to plans.
    Plan::of(effects).guarded(snapshot)
}

/// **`insert`**: add a child at an occupied ordinal, shifting the occupant and
/// every later sibling up by one.
///
/// One rename per shifted sibling and one create, in that order — and the
/// renames run **highest-ordinal-first**, which is the whole of `planInsert`
/// plus `shiftIds` in the model.
///
/// # Why highest-first, since it is not what it looks like
///
/// Not to avoid a collision. A name embeds a tree-unique key, so two siblings
/// never want the same filename and *no* order collides on a well-formed tree;
/// lowest-first is refused only where a hand edit already duplicated a key
/// **and** its parts at adjacent ordinals, which `operations.qnt`'s `corrupted`
/// instance is built from. `docs/formalism-findings.md` entry 003 is where the
/// document's first stated reason was found to be wrong, and the model is what
/// found it.
///
/// The reason that applies to every tree is the **intermediate state**.
/// Highest-first vacates each destination before it is needed, so ordinals stay
/// distinct at every step of the apply and an operation interrupted half way
/// leaves a level that is merely *gapped* — which this design admits
/// everywhere. Run the other way, the same shift passes through a state
/// carrying a **duplicate ordinal**, which it does not. Since a process killed
/// mid-apply is unrecoverable, the order is what decides which of those two a
/// crash leaves: `inv_ordinalsDistinctThroughout`, against
/// `wit_shiftTransientlyDuplicatesAnOrdinal` in the `lowest_first` instance.
///
/// That is a property of the plan, which is a value — so it is read off the
/// plan rather than inferred from a loop's direction, and that is exactly what
/// `ARCHITECTURE.md` says the two rejected shapes could not offer.
pub(crate) fn insert<N: EntryName>(
    snapshot: &Snapshot<N>,
    target: Target,
    at: Ordinal,
    entry: NewEntry<N::Parts>,
) -> Decision<N> {
    let (level, container) = match resolve(snapshot, target) {
        Ok(resolved) => resolved,
        Err(refusal) => return Decision::Refuse(refusal),
    };
    // Every positioned sibling the insert displaces, **highest ordinal first**:
    // `positioned()` is in walk order, which within a level is ascending, so
    // this is the model's `reverseI(asc)` and shares its tie-break on a level a
    // hand edit left carrying two entries at one ordinal.
    let mut shifted: Vec<Entry<'_, N>> = container
        .positioned()
        .filter(|sibling| sibling.ordinal().is_some_and(|ordinal| ordinal >= at))
        .collect();
    shifted.reverse();
    // `idsAtOrdinal(f, d, at).size() == 0` in the model. Both halves of the
    // refusal are this one test: past the last sibling, and a gap in a
    // hand-edited level. See [`Refusal::NoOccupantAtOrdinal`] for why they are
    // one refusal and two messages.
    if !shifted.iter().any(|sibling| sibling.ordinal() == Some(at)) {
        // The span of ordinals this level occupies, least and greatest, in one
        // pass over the same immutable level the shift was collected from. Both
        // ends are needed: the greatest separates *past the last sibling* from
        // *at or below it*, and the least is what decides whether a hole at or
        // below the greatest has an occupant underneath it to name. A fold
        // rather than `min()` and `max()` so the two ends cannot be read from
        // different traversals, and so they are present or absent together.
        let occupied = container
            .positioned()
            .filter_map(|sibling| sibling.ordinal())
            .fold(None, |span: Option<(Ordinal, Ordinal)>, ordinal| {
                Some(match span {
                    None => (ordinal, ordinal),
                    Some((least, greatest)) => (least.min(ordinal), greatest.max(ordinal)),
                })
            });
        return Decision::Refuse(Refusal::NoOccupantAtOrdinal {
            ordinal: at,
            occupied,
        });
    }
    if N::positioned_species(&entry.parts) == PositionedSpecies::Node && !entry.content.is_empty() {
        return Decision::Refuse(Refusal::ContentForANode);
    }
    let mut effects = Vec::with_capacity(shifted.len() + 1);
    for sibling in shifted {
        let Some(triple) = sibling.triple() else {
            unreachable!("`positioned` yields no distinguished child, and every other name has one")
        };
        let Some(next) = triple.ordinal.get().checked_add(1) else {
            return Decision::Refuse(Refusal::OrdinalsExhausted);
        };
        // A shift is not an operation: it is `compose(new_ordinal, key, parts)`,
        // derived, and therefore incapable of disturbing a key, a label or an
        // attribute. And it is one rename of one entry — a shifted *node* is
        // one directory rename, with nothing inside it touched.
        effects.push(Effect::MoveTo {
            entry: sibling.index(),
            to: level,
            name: N::compose(Ordinal::new(next), triple.key, triple.parts.clone()),
        });
    }
    let Some(key) = greatest_key(snapshot).checked_add(1) else {
        return Decision::Refuse(Refusal::KeysExhausted);
    };
    effects.push(Effect::Create {
        at: level,
        name: N::compose(at, Key::new(key), entry.parts),
        content: entry.content,
    });
    Plan::of(effects).guarded(snapshot)
}

/// **`promote`**: turn a leaf into a node, with the node's parts supplied by the
/// caller, moving the leaf's bytes verbatim into the new node's distinguished
/// child and keeping the leaf's own ordinal and its own key.
///
/// Two effects, or three with a first child: create the node, move the leaf into
/// it as the distinguished child, and — optionally — create one child inside it.
/// That is `planPromote` exactly, and its `withChild` run is the plan the
/// [`Level::Created`] variant exists for: the level the second effect acts in
/// does not exist when the plan is built.
///
/// # Named by key, and by nothing else
///
/// There is no [`Target`] here. Every other mutation takes one because the tree
/// root is a level a child can go into; a promotion's target is an *entry* that
/// has to be a leaf, and the root is neither an entry nor a leaf. The model
/// agrees — `TagPromote` carries a bare key where `TagInsert` carries a
/// `Target`.
///
/// # It breaks an invariant on the way through, and there is no ordering that
/// avoids it
///
/// The node has to exist before the leaf's content can move into it, and the
/// node carries the leaf's **own** ordinal and key — that is what identity
/// preservation means. So between effect one and effect two both are on disk,
/// sharing an ordinal and a key: `wit_promoteTransientlyDuplicatesAKey` and
/// `wit_promoteTransientlyDuplicatesAnOrdinal`, which are *reached* rather than
/// excluded, and which `inv_ordinalsDistinctThroughout` exempts by name. The
/// library has no name for a temporary, and a node with any other ordinal or key
/// would not be the same entry. The invariants therefore hold of **quiescent**
/// trees, and the lock is what makes that safe.
///
/// # The parts come from the caller because the library cannot make them
///
/// Species follows from parts, so naming the promoted node needs parts that
/// imply `Node`. `Parts` is opaque with bounds `Clone + Eq`: the library can
/// copy one it already holds and compare two of them, and that is all — and
/// every `Parts` value it can reach belongs to a name already in the tree, none
/// of which describes *this* entry as a node. A trait method mapping a leaf's
/// parts to a node's would widen the seam to serve one operation and force every
/// domain to declare a canonical mapping that is often lossy;
/// `docs/adr/entry-name-is-the-only-seam.md` is the record that argues it.
pub(crate) fn promote<N: EntryName>(
    snapshot: &Snapshot<N>,
    key: Key,
    parts: N::Parts,
    first_child: Option<NewEntry<N::Parts>>,
) -> Decision<N> {
    // The refusals in the model's own order, which is `planPromote`'s: missing,
    // then not a leaf, then no distinguished child, then parts that are not a
    // node. The order is observable — a promotion of a *node* in a domain with
    // no distinguished child has two true refusals and reports the first — so it
    // is transcribed rather than reinvented.
    let Sought::Match(leaf) = snapshot.by_key(key) else {
        return Decision::Refuse(Refusal::TargetMissing { key });
    };
    if leaf.species() != Species::Leaf {
        return Decision::Refuse(Refusal::PromoteNotLeaf {
            key,
            species: leaf.species(),
        });
    }
    // Asked before the parts are looked at, because the answer is about the
    // *domain* and not about this call: a domain with no distinguished child can
    // never promote anything, and saying so is more useful than complaining
    // about the parts of a call that could not have worked.
    let Some(distinguished) = N::distinguished() else {
        return Decision::Refuse(Refusal::NoDistinguishedChild {
            promoting: Some(key),
        });
    };
    if N::positioned_species(&parts) != PositionedSpecies::Node {
        return Decision::Refuse(Refusal::PromotePartsNotNode { key });
    }
    let Some(triple) = leaf.triple() else {
        unreachable!("`by_key` answers with positioned entries, which all have a triple")
    };
    let level = level_of(&leaf.container());
    let mut effects = vec![
        // The node is a *new* directory carrying the promoted leaf's own ordinal
        // and key. Nothing is allocated here: `freshKey` is not consulted,
        // because the entity is unchanged and only its shape moved — which is
        // also why *no key is ever reissued* is a claim about allocation rather
        // than about creation, and why the model says so at length.
        Effect::Create {
            at: level,
            name: N::compose(triple.ordinal, triple.key, parts),
            content: Vec::new(),
        },
        // The leaf's own file, renamed into the node it now sits in. Its bytes
        // move because the file moves: the library has no content model, and a
        // rename is the only thing that can carry bytes it never read.
        Effect::MoveTo {
            entry: leaf.index(),
            to: Level::Created(0),
            name: distinguished,
        },
    ];
    if let Some(child) = first_child {
        // A node is a directory and has nowhere to hold bytes. The refusal
        // belongs to *every operation that creates an entry* rather than to a
        // list of them — `docs/formalism-findings.md` entry 012 is where the
        // list went stale — and this is an operation that creates an entry.
        if N::positioned_species(&child.parts) == PositionedSpecies::Node
            && !child.content.is_empty()
        {
            return Decision::Refuse(Refusal::ContentForANode);
        }
        // `freshKey(f)` over the snapshot the promotion was planned from. The
        // node consumed no key, so the tree's greatest is still the leaf's own
        // maximum and this is the model's `compose(1, freshKey(f), …)`.
        let Some(child_key) = greatest_key(snapshot).checked_add(1) else {
            return Decision::Refuse(Refusal::KeysExhausted);
        };
        effects.push(Effect::Create {
            at: Level::Created(0),
            name: N::compose(Ordinal::FIRST, Key::new(child_key), child.parts),
            content: child.content,
        });
    }
    // Guarded like every other plan. The first effect is the delicate one: its
    // destination is checked while the leaf is *still there*, unvacated, at the
    // leaf's own ordinal and key. What separates the two names is the species —
    // a leaf's parts and a node's — and occupancy compares the species, so this
    // holds for every conforming domain and not only for one whose `Parts`
    // equality happens to distinguish them. It did not hold before
    // `promote-k25`: the comparison was the `NameView` alone, and a lawful
    // domain whose equality is coarser than its rendering lost every promotion
    // to `DestinationOccupied`. See `EntryNameExt::same_name`.
    //
    // The two later destinations are in a directory this plan has just created,
    // and `occupied` answers `false` for a `Level::Created` unconditionally — a
    // directory nothing has ever been written into can only be occupied by this
    // plan's own effects, and a distinguished child and a positioned name are
    // never `same_name`. So effects two and three cannot reach the refusal on
    // **any** tree, damaged or not; the whole of a promotion's exposure to it is
    // effect one.
    //
    // That exposure is real: a tree a failed rollback already damaged carries a
    // node and a leaf sharing an ordinal and a key, and promoting that leaf
    // composes the name the leftover node holds. `wit_damagedTreeStrandsALaterOperation`
    // is that case, and it is worth being precise that the witness does not say
    // so — it is `outcome == RefusedDestinationOccupied and not(isInsert(…))`,
    // which does not distinguish which effect refused. Which effect it is comes
    // from reading `occupied`, not from the model.
    Plan::of(effects).guarded(snapshot)
}

/// **`rewrite`**: replace an entry's parts, keeping its ordinal, its key and its
/// species.
///
/// One effect — a rename onto the same level — and that is the whole operation:
/// `planRewrite` in the model, which builds `MoveTo(i, parentOf(f, i),
/// compose(ordOf(n), keyOf(n), p))` and guards it. This is how an attribute
/// changes, and `docs/adr/entries-are-never-removed.md` is why the operation
/// matters more than its size: with no way to remove an entry, a domain retires
/// one by rewriting an attribute.
///
/// # The library neither knows nor cares what changed
///
/// [`Parts`](EntryName::Parts) is opaque, so *what moved* is not a question this
/// function can ask and not one it needs to: it verifies that the ordinal, the
/// key and the species survived, and renames. Anything that inspected the parts
/// beyond their species would be the seam leaking, and both models are written
/// on the premise that it cannot — neither carries a string at all.
///
/// # Named by key, like `promote`, and for the same reason
///
/// There is no [`Target`]. A rewrite's target is an *entry*, and the tree root
/// is not one — it has no name to rewrite, no ordinal and no key. The model
/// splits them the same way: `TagRewrite` carries a bare key where `TagInsert`
/// carries a target.
///
/// # The species check is `promote`'s, with the opposite verdict
///
/// `promote` refuses parts that are not a node; this refuses parts that are not
/// what the entry already is. Both read [`EntryName::positioned_species`] and
/// nothing else, and they differ in one place worth naming: `promote`'s expected
/// species is the constant [`Species::Node`], so its refusal carries no species,
/// while this one's is whatever the target happens to be, so
/// [`Refusal::RewriteSpeciesChange`] carries it.
///
/// # The no-op has to survive, and it survives twice
///
/// A rewrite to the parts an entry already carries is a rename onto its own
/// path, and it must **succeed** — `wit_rewriteToSameParts`. Nothing here does
/// that: it falls out of occupancy excluding the object being moved, which is
/// [`Effect::mover`] and is why that method exists. The interpreter then carries
/// the same exclusion across the boundary by short-circuiting a same-path
/// rename, because a plan the algebra proved applicable must not be refused by
/// the layer applying it. One property, two mechanisms, and neither alone is
/// enough.
pub(crate) fn rewrite<N: EntryName>(
    snapshot: &Snapshot<N>,
    key: Key,
    parts: N::Parts,
) -> Decision<N> {
    // `resolve(f, ByKey(k))` and its refusal, in the model's own order: missing
    // first, then the species. `by_key` answers with positioned entries only, so
    // the distinguished child cannot be named here at all — the same fact that
    // makes `promote`'s *not a leaf* refusal reachable in one direction only.
    let Sought::Match(entry) = snapshot.by_key(key) else {
        return Decision::Refuse(Refusal::TargetMissing { key });
    };
    let Some(triple) = entry.triple() else {
        unreachable!("`by_key` answers with positioned entries, which all have a triple")
    };
    // Both sides read through `positioned_species`, and neither reads the
    // entry's `species()`. That would widen to [`Species`], whose third variant
    // no positioned entry can have — a refusal that can report a case no
    // argument produces, which is the defect `docs/formalism-findings.md` entry
    // 014 found in the document's own statement of `promote`'s refusal.
    let species = N::positioned_species(triple.parts);
    if N::positioned_species(&parts) != species {
        return Decision::Refuse(Refusal::RewriteSpeciesChange { key, species });
    }
    let level = level_of(&entry.container());
    // The ordinal and the key are the entry's **own**, taken off its current
    // name: this is `compose(ordOf(n), keyOf(n), p)`, so a rewrite cannot move
    // an entry, reorder a level, or reissue a key however wrong the parts are.
    Plan::of(vec![Effect::MoveTo {
        entry: entry.index(),
        to: level,
        name: N::compose(triple.ordinal, triple.key, parts),
    }])
    .guarded(snapshot)
}

/// A run of creations at the end of one level: consecutive ordinals over the
/// level's greatest, consecutive keys over the whole tree's.
///
/// The arithmetic of [`append_many`], extracted because [`initialize`] places
/// its first entries by exactly the same rule — into a root that holds nothing,
/// so the run starts at [`Ordinal::FIRST`] and at key 1 without either function
/// having to say so. Two spellings of one rule are two things to keep in step,
/// and the model has one: `planAppendMany`.
///
/// Both counters come from the one snapshot, and both are read before any effect
/// is built: `maxOrdIn(f, d)` and `freshKey(f)` in the model, taken once and
/// walked forward. The ordinal is the level's, the key is the whole tree's — the
/// names *are* the counter, and there is no file recording the next value
/// because such a file is a second source of truth a hand edit can
/// desynchronise.
fn creations<N: EntryName>(
    snapshot: &Snapshot<N>,
    level: Level,
    container: &Container<'_, N>,
    entries: Vec<NewEntry<N::Parts>>,
) -> Result<Vec<Effect<N>>, Refusal> {
    let mut ordinal = last_ordinal(container);
    let mut key = greatest_key(snapshot);
    let mut effects = Vec::with_capacity(entries.len());
    for entry in entries {
        if N::positioned_species(&entry.parts) == PositionedSpecies::Node
            && !entry.content.is_empty()
        {
            return Err(Refusal::ContentForANode);
        }
        let (Some(next_ordinal), Some(next_key)) = (ordinal.checked_add(1), key.checked_add(1))
        else {
            return Err(if key.checked_add(1).is_none() {
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
    Ok(effects)
}

/// The level a container is, as a plan names it.
fn level_of<N: EntryName>(container: &Container<'_, N>) -> Level {
    match container.entry() {
        None => Level::Root,
        Some(node) => Level::Entry(node.index()),
    }
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
            let Sought::Match(entry) = snapshot.by_key(key) else {
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
/// see every name there is. This is also the reason no operation removes an
/// entry — doing so lowers this maximum, and the next allocation re-issues a key
/// that other entries may still reference. Deleting the **root** does not reach
/// this function at all: there is no next allocation once the tree is gone.
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
