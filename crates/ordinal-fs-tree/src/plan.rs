//! What the algebra decides, and the machinery every mutation is built out of.
//!
//! A mutation is four steps — snapshot, algebra, plan, apply — and only the last
//! touches the filesystem. This module is the third: the [`Plan`] a mutation
//! turns into, the primitive [`Effect`]s it is made of, the [`Decision`] the
//! algebra returns for every input, and the [`Refusal`] that is the other half
//! of one.
//!
//! The specification is `docs/ordinal-fs-tree/ARCHITECTURE.md`, sections *How an
//! operation runs*, *The plan is checked against itself, in order* and
//! *Refusals*; the model is `docs/ordinal-fs-tree/models/operations.qnt`, whose
//! `Effect`, `Decision` and `Outcome` types this one mirrors.
//!
//! # The plan is a value, and that is the whole point
//!
//! `ARCHITECTURE.md` names the two shapes it rejected — pure functions over name
//! lists, and read-transform-diff — and both are rejected for the same reason:
//! they leave the *order* of the renames as an accident of a loop rather than a
//! property of a value anything can read. Because a plan is a value, one
//! interpreter applies every operation and one rollback unwinds every operation,
//! so five operations cannot drift into five slightly different unwinds.
//!
//! # Two variants forward, two back
//!
//! `operations.qnt` gives `Effect` three variants and then says of one of them:
//! *`Remove` never appears in a forward plan — it is only ever generated as the
//! undo of a `Create`.* Here that comment is a type: [`Effect`] has the two
//! variants a plan can hold, and the undo of an effect is the interpreter's own
//! `Undo`, which is the only thing that can remove anything. The model's
//! `inv_rollbackRemovesOnlyItsOwn` is then structural rather than checked — an
//! undo naming a path the run did not create cannot be constructed.

use crate::{EntryName, EntryNameExt, Key, Ordinal, PositionedSpecies, Snapshot, Species};

/// Which level an effect acts in: the tree root, a node already in the tree, or
/// a node this same plan creates.
///
/// The third variant is not speculative — `operations.qnt`'s `planPromote`
/// builds exactly it, moving the promoted leaf into the node the previous effect
/// created — and it is why a level is named by an identity rather than by a
/// path: half the levels a plan mentions do not exist yet when the plan is
/// built.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Level {
    /// The tree root: a node that is not an entry.
    Root,
    /// A node in the snapshot, by its position in the snapshot's own arena.
    Entry(usize),
    /// The node created by an earlier effect of this plan, by that effect's
    /// position in it.
    ///
    /// `promote` is what builds it: the node has to exist before the leaf's
    /// content can move into it, and the plan is a value built before anything
    /// has run — so the level the second effect acts in is named by the effect
    /// that will create it.
    Created(usize),
}

/// One primitive filesystem action.
///
/// There is no *remove* here, deliberately; see this module's header.
#[derive(Debug)]
pub(crate) enum Effect<N> {
    /// Bring a new entry into being. A node is a directory and a leaf is a
    /// regular file holding `content` — which of the two is not carried here,
    /// because the species follows from the parts and therefore from `name`.
    Create {
        /// The level it appears in.
        at: Level,
        /// Its name.
        name: N,
        /// The bytes of a leaf, written verbatim. Empty for a node.
        content: Vec<u8>,
    },
    /// Rename an entry already in the tree, possibly into another level. A
    /// sibling shift is this and nothing else: `compose(new_ordinal, key,
    /// parts)`, so it cannot disturb a key, a label or an attribute.
    MoveTo {
        /// The entry being moved, by its position in the snapshot's arena.
        entry: usize,
        /// The level it lands in.
        to: Level,
        /// The name it takes.
        name: N,
    },
}

impl<N: EntryName> Effect<N> {
    /// The level this effect's destination sits in, and the name it takes
    /// there.
    fn destination(&self) -> (Level, &N) {
        match self {
            Self::Create { at, name, .. } => (*at, name),
            Self::MoveTo { to, name, .. } => (*to, name),
        }
    }

    /// The name this effect places, whatever it does with it.
    ///
    /// The interpreter renders exactly this to reach a path, which is why the
    /// seventh obligation is checked against it and against nothing else in a
    /// plan.
    pub(crate) const fn name(&self) -> &N {
        match self {
            Self::Create { name, .. } | Self::MoveTo { name, .. } => name,
        }
    }

    /// The entry this effect moves, which occupancy must exclude: a rewrite
    /// whose new parts equal the old is a rename onto itself, and without the
    /// exclusion the library would refuse its own no-op.
    fn mover(&self) -> Option<usize> {
        match self {
            Self::Create { .. } => None,
            Self::MoveTo { entry, .. } => Some(*entry),
        }
    }
}

/// An ordered list of primitive effects, checked against itself before anything
/// runs.
///
/// Internal. A consumer calls an operation and receives a [`Report`] of what
/// happened, never a plan to apply — `ARCHITECTURE.md` is explicit that this is
/// structure and not interface.
///
/// [`Report`]: crate::Report
#[derive(Debug)]
pub(crate) struct Plan<N> {
    effects: Vec<Effect<N>>,
}

impl<N: EntryName> Plan<N> {
    /// A plan of these effects, in this order.
    pub(crate) fn of(effects: Vec<Effect<N>>) -> Self {
        Self { effects }
    }

    /// The effects, in the order the interpreter must apply them.
    pub(crate) fn effects(&self) -> &[Effect<N>] {
        &self.effects
    }

    /// The decision this plan is, once it has been checked against the snapshot
    /// it was built from.
    ///
    /// **The check is sequential, and that is a design decision rather than an
    /// implementation detail.** The plan is folded through the snapshot, so each
    /// destination is met in the state the interpreter will meet it in.
    /// Checking every destination against the *snapshot* instead — the obvious
    /// reading of "a pure function of the snapshot" — refuses correct inserts,
    /// and makes the highest-first shift order buy nothing at all, because under
    /// it both orders are refused in exactly the same cases.
    /// `docs/formalism-findings.md` entry 003 records that the document had not
    /// made this decision until the model forced it; the model makes it in
    /// `planIsApplicable`, and this is that function.
    pub(crate) fn guarded(self, snapshot: &Snapshot<N>) -> Decision<N> {
        match self.refusal(snapshot) {
            Some(refusal) => Decision::Refuse(refusal),
            None => Decision::Proceed(self),
        }
    }

    /// `Some` when some effect would meet an occupied destination, folding the
    /// plan through the snapshot in order.
    fn refusal(&self, snapshot: &Snapshot<N>) -> Option<Refusal> {
        let mut arrived: Vec<(Level, &N)> = Vec::new();
        let mut vacated: Vec<usize> = Vec::new();
        for effect in &self.effects {
            let (level, name) = effect.destination();
            let mover = effect.mover();
            if occupied(snapshot, &arrived, &vacated, level, name, mover) {
                return Some(Refusal::DestinationOccupied {
                    ordinal: name.triple().map(|t| t.ordinal),
                    key: name.triple().map(|t| t.key),
                });
            }
            if let Some(entry) = mover {
                vacated.push(entry);
            }
            arrived.push((level, name));
        }
        None
    }
}

/// Whether `name` is already taken in `level`, given the effects that have
/// already been folded in.
///
/// Names are compared by [`same_name`](EntryNameExt::same_name) and never by
/// rendering, which is what *the library holds no strings* means in practice.
/// Two names that answer `true` there are one filename, because the grammar is
/// canonical — that obligation is exactly what makes this comparison sound, and
/// `structure.als`'s `witness_two_filenames_name_one_entry` is the picture of a
/// domain that broke it.
///
/// The comparison is `same_name` and not `view() == view()` because a domain's
/// `Parts` equality may be coarser than its rendering: see that method, and
/// `promote-k25`, which found a lawful domain whose leaf and node parts compare
/// equal and whose every valid promotion was therefore refused as
/// [`Refusal::DestinationOccupied`]. Both comparisons below take the same rule,
/// so an arrived effect and a snapshot entry cannot disagree about what one
/// name is.
fn occupied<N: EntryName>(
    snapshot: &Snapshot<N>,
    arrived: &[(Level, &N)],
    vacated: &[usize],
    level: Level,
    name: &N,
    mover: Option<usize>,
) -> bool {
    let already = match level {
        // A level this plan created is a directory nothing has ever been
        // written into, so only this plan's own effects can have occupied it.
        Level::Created(_) => false,
        Level::Root | Level::Entry(_) => snapshot
            .level(level)
            .into_iter()
            .flat_map(|container| container.children())
            .any(|child| {
                let index = child.index();
                Some(index) != mover && !vacated.contains(&index) && child.name().same_name(name)
            }),
    };
    already
        || arrived
            .iter()
            .any(|(at, taken)| *at == level && taken.same_name(name))
}

/// What the algebra returns for every input: a plan to apply, or a refusal.
///
/// Two variants and no third, which is the whole of *every operation is total*.
/// `operations.qnt` makes the same claim structurally — `decide` returns a
/// `Decision` for every state and every argument, so an unmodelled case would
/// have to be a missing branch the typechecker rejects.
#[derive(Debug)]
pub(crate) enum Decision<N> {
    /// This plan, checked against itself in order.
    Proceed(Plan<N>),
    /// Nothing will be changed, and this is why.
    Refuse(Refusal),
}

/// A stated outcome in which an operation changes nothing.
///
/// Not an error thrown from inside the algebra: the algebra returns it, and the
/// filesystem layer is where it becomes an [`Error::Refused`]. Every one of
/// these is a row of `ARCHITECTURE.md`'s *Refusals* section, and — except where
/// a variant says otherwise — a modelled `Outcome` in `operations.qnt` with a
/// witness proving it is reachable.
///
/// [`Error::Refused`]: crate::Error::Refused
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// No entry carries this key. `operations.qnt`'s `RefusedTargetMissing`.
    TargetMissing {
        /// The key that named nothing.
        key: Key,
    },
    /// The target is not a level: a leaf is a regular file and holds nothing, and
    /// a distinguished child is a node's own content rather than a level of the
    /// tree. `operations.qnt`'s `RefusedTargetNotNode`.
    TargetNotNode {
        /// The key of the entry that was named.
        key: Key,
        /// What it turned out to be.
        species: Species,
    },
    /// Some effect's destination is already occupied. `operations.qnt`'s
    /// `RefusedDestinationOccupied`.
    ///
    /// The document narrows what can reach this: a foreign name can never
    /// occupy a destination, because the grammar is canonical, and a symbolic
    /// link wearing an entry's name halts at the snapshot rather than occupying
    /// anything. What remains is a tree carrying a duplicated key and a tree
    /// damaged by a failed rollback.
    DestinationOccupied {
        /// The ordinal of the name that could not be placed, if it had one.
        ordinal: Option<Ordinal>,
        /// Its key, if it had one.
        key: Option<Key>,
    },
    /// Bytes were supplied for an entry whose parts imply a node, and a
    /// directory has nowhere to put them.
    ///
    /// **This refusal discharges no model claim, and cannot.** Content is
    /// outside both models by design — `operations.qnt`'s handoff block records
    /// bytes as unmodelled — so this is the library's own, in the same position
    /// as [`Error::NonUtf8Name`]: a case the library can see and no model can
    /// pose. Discarding the bytes silently is the alternative, and it is not
    /// one.
    ///
    /// [`Error::NonUtf8Name`]: crate::Error::NonUtf8Name
    ContentForANode,
    /// An `insert` named an ordinal no sibling occupies. `operations.qnt`'s
    /// `RefusedNoOccupantAtOrdinal`.
    ///
    /// **One refusal, two situations, and the document gives a rationale for
    /// only one of them.** Past the last sibling, inserting is `append`'s job
    /// and is refused rather than quietly redirected — the two differ in their
    /// effect on every later sibling, so guessing which was meant would be
    /// guessing at intent. Into a **gap** in a hand-edited level that rationale
    /// plainly does not apply, and the honest answer is the harder one: density
    /// is preserved by every operation and established by none, so *no*
    /// operation fills a gap and a gapped ordinal can be occupied only by hand.
    /// `operations.qnt` witnesses the two separately — `wit_insertPastTheEnd`
    /// and `wit_insertIntoAGap` — which is how the second came to be noticed at
    /// all; `docs/formalism-findings.md` entry 003 records it.
    ///
    /// The **span** of ordinals the level occupies is carried so the message can
    /// tell those apart and give the advice that fits, rather than offering the
    /// reader a fork. The greatest alone cannot: it separates *past the last
    /// sibling* from *at or below it*, but every message about a hole at or
    /// below the greatest that names a lower neighbour is claiming something
    /// `greatest` does not prove. A level holding only ordinal 5, asked for
    /// [`Ordinal::FIRST`], has no occupant below the request at all — and
    /// because density is preserved and never established, [`Ordinal::FIRST`]
    /// is not a floor that would make such a level impossible. So the least is
    /// carried too, and the interior-gap message is emitted only where both
    /// neighbours are proven to exist.
    ///
    /// One field rather than two, because a level either holds positioned
    /// children or it does not: two independent `Option`s could disagree, and
    /// this refusal exists because state carried for a message can be wrong.
    NoOccupantAtOrdinal {
        /// The ordinal that named no sibling.
        ordinal: Ordinal,
        /// The least and greatest ordinals the level holds, in that order, or
        /// `None` for a level holding no positioned children at all. They are
        /// equal on a level holding exactly one occupied ordinal.
        occupied: Option<(Ordinal, Ordinal)>,
    },
    /// `promote` was aimed at something that is not a leaf.
    /// `operations.qnt`'s `RefusedPromoteNotLeaf`.
    ///
    /// **The document names two cases here and only one of them is reachable.**
    /// *A node is already a node, and a distinguished child has no ordinal to
    /// carry across; both are refused* — but an operation names its target by
    /// key, a distinguished child carries no key, and so neither this library
    /// nor the model can be handed one: `by_key` yields positioned entries, and
    /// `idsWithKey` filters on `isPositioned`. What `species` carries is
    /// therefore what was actually found, and on every path that reaches this
    /// today it is [`Species::Node`].
    PromoteNotLeaf {
        /// The key of the entry that was named.
        key: Key,
        /// What it turned out to be.
        species: Species,
    },
    /// Bytes were supplied for a distinguished child in a domain whose
    /// [`EntryName::distinguished`] is `None`. `operations.qnt`'s
    /// `RefusedNoDistinguishedChild`, and the whole content of its
    /// `no_distinguished` instance.
    ///
    /// Refused outright rather than guessed at: the alternatives are discarding
    /// the bytes silently and inventing a name the domain never declared, and
    /// neither is one.
    ///
    /// # Two operations, one refusal
    ///
    /// [`promote`] moves a leaf's content into the new node's distinguished
    /// child, and [`Vacancy::initialize`] writes a fresh root's. Both are the
    /// same condition — *this domain has no distinguished child, and these
    /// bytes have nowhere to go* — so they answer with the same refusal rather
    /// than with two that would have to be kept in step. What distinguishes
    /// them is `promoting`, which is the key of the leaf on the one path that
    /// has a key at all: a root initialization names no entry, because the tree
    /// root is not one.
    ///
    /// [`promote`]: crate::fs::WriteGuard::promote
    /// [`Vacancy::initialize`]: crate::fs::Vacancy::initialize
    NoDistinguishedChild {
        /// The key of the leaf that would have been promoted, or `None` when a
        /// root initialization asked for one.
        promoting: Option<Key>,
    },
    /// The parts `promote` was given do not imply species `Node`.
    /// `operations.qnt`'s `RefusedPromotePartsNotNode` — the same check
    /// `rewrite` makes, with the opposite verdict.
    ///
    /// The parts come from the caller because the library cannot make them:
    /// `Parts` is opaque with bounds `Clone + Eq`, so every `Parts` value the
    /// library can reach belongs to some entry already in the tree and none of
    /// those describes *this* entry as a node. What it can do is check what it
    /// was handed.
    PromotePartsNotNode {
        /// The key of the leaf that would have been promoted.
        key: Key,
    },
    /// The parts `rewrite` was given imply a different species from the one the
    /// entry already has. `operations.qnt`'s `RefusedRewriteSpeciesChange` —
    /// the same check [`Refusal::PromotePartsNotNode`] makes, with the opposite
    /// verdict: `promote` requires the parts to name a *different* species and
    /// this requires them to name the *same* one.
    ///
    /// A rewrite replaces an entry's parts and keeps its ordinal, its key and
    /// its species; changing the species would rename a regular file into a
    /// directory, which is not a rename at all. Changing shape is
    /// [`WriteGuard::promote`]'s job, and it goes one way only, because a node's
    /// children have nowhere to go.
    ///
    /// **One species carried, not two**, for the reason
    /// [`Refusal::NoOccupantAtOrdinal`] carries one field: a
    /// [`PositionedSpecies`] has exactly two variants, so *the entry is a leaf*
    /// and *the supplied parts make a node* are one fact written twice, and two
    /// fields that restate each other are two fields that can disagree. It is
    /// carried at all — where [`Refusal::PromotePartsNotNode`] carries only a
    /// key — because `promote`'s expected species is the constant
    /// [`Species::Node`] and this one's is whatever the entry happens to be.
    ///
    /// [`WriteGuard::promote`]: crate::fs::WriteGuard::promote
    RewriteSpeciesChange {
        /// The key of the entry that was named.
        key: Key,
        /// The species that entry has, which the supplied parts contradict.
        species: PositionedSpecies,
    },
    /// The tree's greatest key is the greatest a key can be, so `max + 1` has
    /// nowhere to go.
    ///
    /// **No model claim, and for a reason worth stating**: an integer in either
    /// model is unbounded, so neither can pose exhaustion at all. A [`Key`] is a
    /// `u32`, and a hand-edited name carrying the maximum makes every allocation
    /// after it impossible. Refused rather than wrapped, because a wrapped
    /// allocation re-issues a key that is still referenced — the one thing the
    /// whole no-removal rule exists to prevent.
    KeysExhausted,
    /// The level's greatest ordinal is the greatest an ordinal can be, so there
    /// is no position after it. No model claim, for the same reason as
    /// [`Refusal::KeysExhausted`].
    OrdinalsExhausted,
}

impl core::fmt::Display for Refusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TargetMissing { key } => write!(
                f,
                "no entry in this tree has key {key}. Operations name their \
                 target by key because an ordinal is stale as soon as anything \
                 is inserted before it and a path is stale as soon as anything \
                 is renamed; check the key, or walk the tree to find the entry \
                 you meant."
            ),
            Self::TargetNotNode { key, species } => write!(
                f,
                "the entry with key {key} is a {species}, which holds nothing. \
                 Children go in a node — promote it first, or name a node."
            ),
            Self::DestinationOccupied { ordinal, key } => {
                f.write_str("the name this operation would have placed is already taken")?;
                if let (Some(ordinal), Some(key)) = (ordinal, key) {
                    write!(f, " (ordinal {ordinal}, key {key})")?;
                }
                f.write_str(
                    ". A tree the library built cannot do this, so something else \
                     has: a hand edit that duplicated a key, an earlier operation \
                     whose rollback failed, or a writer that did not take the lock. \
                     Look at the level and repair it by hand.",
                )
            }
            Self::ContentForANode => f.write_str(
                "bytes were supplied for an entry whose parts make it a node, and a \
                 directory has nowhere to hold them. Supply no content, or supply \
                 parts that make it a leaf.",
            ),
            Self::NoOccupantAtOrdinal { ordinal, occupied } => {
                write!(
                    f,
                    "nothing sits at ordinal {ordinal} in that level, and \
                           an insert shifts an occupant up rather than filling a \
                           hole. "
                )?;
                match occupied {
                    // Below everything this level occupies. Nothing sits under
                    // the request, so the gap message's lower neighbour does
                    // not exist and saying otherwise would be false — but the
                    // conclusion is the gap case's, not `append`'s, since
                    // `append` would take `greatest + 1` and not this ordinal.
                    Some((least, _)) if ordinal < least => write!(
                        f,
                        "That ordinal is below ordinal {least}, the lowest this \
                         level occupies, so nothing in this level sits below it. \
                         No operation fills an unoccupied ordinal at or below the \
                         level's greatest — ordinal density is preserved by every \
                         operation and established by none — so it can be \
                         occupied only by hand, with `mv`.",
                    ),
                    // Strictly between the least and the greatest, since it is
                    // occupied by nothing and therefore neither of them: both
                    // neighbours are proven to exist.
                    Some((_, greatest)) if ordinal <= greatest => f.write_str(
                        "That ordinal is a gap in this level: something below it \
                         and something above it are occupied. No operation fills \
                         a gap — ordinal density is preserved by every operation \
                         and established by none — so a gapped ordinal can be \
                         occupied only by hand, with `mv`.",
                    ),
                    // Past the last sibling — or into a level holding no
                    // positioned children at all, where every ordinal is past
                    // the last sibling.
                    _ => f.write_str(
                        "That ordinal is past the last sibling, which is \
                         `append`'s job: it takes the next free ordinal and \
                         leaves every other entry alone, where an insert would \
                         shift them. The two differ in what they do to the rest \
                         of the level, so call the one you meant.",
                    ),
                }
            }
            Self::PromoteNotLeaf { key, species } => write!(
                f,
                "the entry with key {key} is a {species}, and promotion turns a \
                 leaf into a node. A node is already one; name a leaf, or add \
                 children to this node directly."
            ),
            // One condition, two operations, and the advice differs by which
            // asked: a promotion has a leaf to name and a fallback that keeps
            // its content, and an initialization has neither.
            Self::NoDistinguishedChild { promoting } => {
                f.write_str(
                    "this domain has no distinguished child, so the content supplied \
                     for one has nowhere to go. ",
                )?;
                match promoting {
                    Some(key) => write!(
                        f,
                        "Promotion moves the bytes of the leaf with key {key} verbatim \
                         into the new node's distinguished child; give the domain one \
                         by implementing `EntryName::distinguished`, or create the node \
                         and move the content yourself."
                    ),
                    None => f.write_str(
                        "A root initialization writes those bytes into the new root's \
                         distinguished child; give the domain one by implementing \
                         `EntryName::distinguished`, or initialize the tree without a \
                         distinguished child.",
                    ),
                }
            }
            Self::PromotePartsNotNode { key } => write!(
                f,
                "the parts supplied for promoting the leaf with key {key} make a \
                 leaf, not a node, and a promotion has to name a directory. The \
                 species follows from the parts and from nothing else, so supply \
                 parts your domain composes a node from."
            ),
            Self::RewriteSpeciesChange { key, species } => {
                write!(
                    f,
                    "the entry with key {key} is a {species}, and the parts \
                     supplied for rewriting it make a {other}. A rewrite \
                     replaces an entry's parts and keeps its ordinal, its key \
                     and its species — only the opaque remainder of the name \
                     moves. ",
                    other = match species {
                        PositionedSpecies::Leaf => PositionedSpecies::Node,
                        PositionedSpecies::Node => PositionedSpecies::Leaf,
                    }
                )?;
                // The advice differs by direction because the operations do.
                // One shape change exists and it goes one way: a leaf's content
                // has somewhere to land, and a node's children have nowhere.
                // Offering `promote` to whoever asked for the impossible
                // direction would be advice that fails when taken.
                match species {
                    PositionedSpecies::Leaf => f.write_str(
                        "Supply parts that make a leaf, or call `promote`, which \
                         is the operation that turns a leaf into a node and \
                         moves the leaf's content into the new node rather than \
                         discarding it.",
                    ),
                    PositionedSpecies::Node => f.write_str(
                        "Supply parts that make a node. Nothing turns a node \
                         back into a leaf: its children would have nowhere to \
                         go, and no operation removes an entry.",
                    ),
                }
            }
            Self::KeysExhausted => f.write_str(
                "this tree's greatest key is the greatest a key can be, so there is \
                 no fresh one to allocate: a key is `max + 1` over every name in the \
                 tree. Nothing the library built can reach this — look for a \
                 hand-written name carrying an enormous key and lower it.",
            ),
            Self::OrdinalsExhausted => f.write_str(
                "this level's greatest ordinal is the greatest an ordinal can be, so \
                 there is no position after it. Look for a hand-written name carrying \
                 an enormous ordinal and lower it.",
            ),
        }
    }
}

#[cfg(test)]
mod tests;
