# Mutation algebra
<!-- book-page id="mutation-algebra" slice="mutation-algebra-k15" order="5" -->
[Previous: Read path](04-read-path.md) | [Contents](README.md) | [Next: Filesystem interpreter](06-filesystem-interpreter.md)

A mutation begins from the immutable `Snapshot` described on the previous page.
The algebra receives that snapshot and the operation's arguments, and returns a
total `Decision`: either a guarded `Plan` or a `Refusal`. It cannot read or alter
the filesystem. A plan is an ordered value made from `Create` and `MoveTo`
effects; the next page explains the single interpreter that applies those
effects and produces a `Report`.

<!-- fragment «mutation-operations-source» owner="mutation-algebra-k15" source="crates/ordinal-fs-tree/src/ops.rs" lines="1-543" parent="source-operations" -->
<!-- insert «ops-surface-and-inputs» -->
<!-- insert «ops-append» -->
<!-- insert «ops-insert» -->
<!-- insert «ops-promote» -->
<!-- insert «ops-rewrite» -->
<!-- insert «ops-resolution-and-allocation» -->
<!-- /fragment -->

<!-- fragment «mutation-plan-source» owner="mutation-algebra-k15" source="crates/ordinal-fs-tree/src/plan.rs" lines="1-568" parent="source-plan" -->
<!-- insert «plan-effects» -->
<!-- insert «plan-guarded» -->
<!-- insert «plan-decision-and-refusals» -->
<!-- insert «plan-refusal-messages» -->
<!-- /fragment -->

<!-- fragment «mutation-report-source» owner="mutation-algebra-k15" source="crates/ordinal-fs-tree/src/report.rs" lines="1-152" parent="source-report" -->
<!-- insert «report-structure-and-order» -->
<!-- insert «report-debug» -->
<!-- /fragment -->

<a id="mutation-inputs"></a>
## Operation inputs

`Target` names a level: either the root or the node carrying a stable key.
`NewEntry` carries consumer-owned parts and optional leaf bytes. Its species is
not an independent input; `EntryName::positioned_species` derives leaf or node
from the parts. These types and the module boundary establish the page's input:
one snapshot plus values, with no path or filesystem handle available to the
algebra.

The operations module owns the pure boundary and the two request types. This
fragment takes parsed names and opaque parts as inputs, establishes key-only
targeting and parts-derived species, and supplies the values used by the worked
insert without introducing any filesystem capability.

<!-- fragment «ops-surface-and-inputs» owner="mutation-algebra-k15" source="crates/ordinal-fs-tree/src/ops.rs" lines="1-68" parent="mutation-operations-source" -->
````rust
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
use crate::{Container, Entry, EntryName, Key, Ordinal, PositionedSpecies, Snapshot, Species};

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
````
<!-- /fragment -->

<a id="effects-and-levels"></a>
## Effects and levels

A plan names a destination level as the root, a snapshot node, or a node created
by an earlier effect in the same plan. The last form is required by promotion:
the destination for the promoted content does not exist when planning begins.
`Effect` contains only forward `Create` and `MoveTo` actions. Removal belongs to
the interpreter's private undo type, so no forward operation can construct a
removal plan.

The plan module owns this vocabulary. The fragment turns snapshot identities
and composed names into two primitive forward effects, establishes their written
order as data, and gives the worked insert a representation whose destinations
can be checked before any effect runs.

<!-- fragment «plan-effects» owner="mutation-algebra-k15" source="crates/ordinal-fs-tree/src/plan.rs" lines="1-118" parent="mutation-plan-source" -->
````rust
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
````
<!-- /fragment -->

<a id="worked-insert-decision"></a>
## One complete insert decision

The opening command inserts the draft lesson `limits` at ordinal 2 in the module
with key 2. The relevant level in the captured snapshot is:

```text
02-linear-algebra-i2/
├── OVERVIEW.md
├── 01-published-foundations-i3.md
├── 02-published-vectors-i5.md
└── 03-draft-matrices-i6.md
```

The pure call is equivalent to:

```text
ops::insert(
    snapshot,
    Target::Key(Key::new(2)),
    Ordinal::new(2),
    NewEntry::empty(Parts::lesson(Status::Draft, Label::new("limits")?)),
)
```

Target resolution finds the module's `Container`. The operation collects
positioned siblings at or after ordinal 2 in ascending snapshot order, then
reverses the collection. The shifted sequence is therefore key 6 at ordinal 3,
then key 5 at ordinal 2. The presence of key 5 at exactly ordinal 2 satisfies
the insert-specific occupant condition.

Each shift recomposes the existing sibling from `ordinal + 1`, its unchanged
key, and its cloned parts. The whole-tree greatest key is 6, so the created
lesson receives key 7. The resulting plan is:

```text
1. MoveTo key 6 in module key 2:
   03-draft-matrices-i6.md -> 04-draft-matrices-i6.md
2. MoveTo key 5 in module key 2:
   02-published-vectors-i5.md -> 03-published-vectors-i5.md
3. Create in module key 2:
   02-draft-limits-i7.md
```

Highest-first shifting preserves distinct ordinals after every landed move. It
is not needed to prevent an ordinary filename collision because each sibling's
key remains in its name. Its general purpose is the intermediate state: a
process stopped between moves leaves a gap, whereas lowest-first shifting would
temporarily duplicate an ordinal. The plan expresses this order directly.

The insert planner owns target resolution, the exact occupant test, derived
sibling shifts, fresh-key allocation, and effect construction. This fragment
turns the worked snapshot and request into the three effects above while
preserving every shifted key and part and refusing before a plan exists when an
input condition fails.

<!-- fragment «ops-insert» owner="mutation-algebra-k15" source="crates/ordinal-fs-tree/src/ops.rs" lines="145-251" parent="mutation-operations-source" -->
````rust

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
````
<!-- /fragment -->

<a id="guarded-plans"></a>
### Guarding the ordered plan

`Plan::guarded` folds effects through a simulated state in plan order. For each
effect it checks both the snapshot entries not already vacated and the
destinations claimed by earlier effects. A move excludes its own source entry,
which permits a rewrite onto its current name. A `Level::Created` begins empty,
but earlier effects in the same plan may already have claimed names inside it.

For this pristine insert, every destination is already distinct from every
other snapshot name because each positioned name includes its key and parts as
well as its ordinal. A snapshot-only check would therefore accept this
particular plan too. The sequential fold becomes decisive on an admitted
hand-edited tree: if adjacent siblings duplicate a key and parts, the
highest-first move vacates the complete name a later move needs before that
later destination is checked. It also detects two effects in one plan that
claim the same complete name. The worked plan returns
`Decision::Proceed(plan)`; an occupied destination instead returns
`Decision::Refuse` without exposing the plan to the interpreter.

The plan guard owns the pure simulation of earlier arrivals and departures. This
fragment folds the worked insert in interpreter order, establishes that every
destination is free in the state where it will be used, and converts a conflict
into a refusal before the filesystem layer receives an effect.

<!-- fragment «plan-guarded» owner="mutation-algebra-k15" source="crates/ordinal-fs-tree/src/plan.rs" lines="119-229" parent="mutation-plan-source" -->
````rust

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
````
<!-- /fragment -->

<a id="reports"></a>
### Reports preserve landed order

The algebra returns a plan, not a report. The interpreter constructs the public
`Report` while effects land, recording caller-spelled paths that cannot exist in
the pure layer. The connection remains exact: `created()` and `renamed()` retain
their own effect-relative orders, while `paths()` follows the mixed plan order.

For the worked insert, the successful interpreter constructs this report
meaning:

```text
renamed():
  name = 04-draft-matrices-i6.md
  from = s/02-linear-algebra-i2/03-draft-matrices-i6.md
  to   = s/02-linear-algebra-i2/04-draft-matrices-i6.md
  name = 03-published-vectors-i5.md
  from = s/02-linear-algebra-i2/02-published-vectors-i5.md
  to   = s/02-linear-algebra-i2/03-published-vectors-i5.md
created():
  name = 02-draft-limits-i7.md
  path = s/02-linear-algebra-i2/02-draft-limits-i7.md
paths():
  s/02-linear-algebra-i2/04-draft-matrices-i6.md
  s/02-linear-algebra-i2/03-published-vectors-i5.md
  s/02-linear-algebra-i2/02-draft-limits-i7.md
```

The names expose key 6 followed by key 5 in `renamed()` and key 7 in
`created()`. The caller-spelled paths expose the same two moves followed by the
create in `paths()`. The separate `Landing` sequence is necessary because two
species-specific vectors cannot reconstruct an interleaving such as promotion's
create, move, create.

The report module owns the consumer-visible record written during application.
This fragment turns landed creates and moves into names and caller-spelled paths,
preserves both per-species order and complete effect order, and makes the worked
insert's highest-first shift observable after the internal plan is gone.

<!-- fragment «report-structure-and-order» owner="mutation-algebra-k15" source="crates/ordinal-fs-tree/src/report.rs" lines="1-119" parent="mutation-report-source" -->
````rust
//! What a mutating operation tells the consumer it did.
//!
//! A plan is internal — `ARCHITECTURE.md` says so in as many words: *a consumer
//! calls `tree.insert(...)` and receives a report of what happened, never a plan
//! to apply*. This is that report, and it is written as the interpreter goes, so
//! it describes what the filesystem did rather than what the algebra intended.
//!
//! Paths are here and names are here, and both are needed. A name is the
//! library's own currency — a consumer reads the fresh key off it, which is the
//! one thing an `append` produces that the caller could not have known — while a
//! path is what the consumer opens, and it is built from the caller's own
//! spelling of the root, because nothing in this crate canonicalises anything.

use core::fmt;
use std::path::{Path, PathBuf};

use crate::EntryName;

/// An entry this operation brought into being.
pub struct Created<N> {
    /// Its name, carrying the ordinal and the key the library allocated.
    pub name: N,
    /// Where it now is, in the caller's own spelling of the root.
    pub path: PathBuf,
}

/// An entry this operation renamed — a sibling shift, or a promoted leaf moving
/// into its new node.
pub struct Renamed<N> {
    /// The name it now carries.
    pub name: N,
    /// Where it was.
    pub from: PathBuf,
    /// Where it is now.
    pub to: PathBuf,
}

/// What a mutating operation did.
///
/// Empty when the operation had nothing to do — an `append_many` of no entries
/// is a plan of no effects, which succeeds and changes nothing.
pub struct Report<N> {
    created: Vec<Created<N>>,
    renamed: Vec<Renamed<N>>,
    /// What landed, in the order it landed, as an index into one of the two
    /// vectors above.
    ///
    /// The two vectors alone cannot answer *in what order* — they are two
    /// species-sorted buckets, and a mixed plan interleaves them. An `insert` is
    /// shifts then a create, so a creation-first reading reports the new entry
    /// before every shift that made room for it; a promotion with a first child
    /// is create, move, create, which no pair of buckets can reconstruct at all.
    /// Keeping the order here rather than merging the two vectors is what lets
    /// [`Report::created`] and [`Report::renamed`] stay in *their* own order,
    /// which is where the highest-first shift rule is observable.
    landed: Vec<Landing>,
}

/// One thing this operation did, by species and by its place in that species'
/// own list.
#[derive(Clone, Copy, Debug)]
enum Landing {
    Created(usize),
    Renamed(usize),
}

impl<N: EntryName> Report<N> {
    /// A report of nothing yet.
    pub(crate) fn empty() -> Self {
        Self {
            created: Vec::new(),
            renamed: Vec::new(),
            landed: Vec::new(),
        }
    }

    pub(crate) fn record_created(&mut self, name: N, path: PathBuf) {
        self.landed.push(Landing::Created(self.created.len()));
        self.created.push(Created { name, path });
    }

    pub(crate) fn record_renamed(&mut self, name: N, from: PathBuf, to: PathBuf) {
        self.landed.push(Landing::Renamed(self.renamed.len()));
        self.renamed.push(Renamed { name, from, to });
    }

    /// The entries this operation created, in the order it created them.
    ///
    /// For an `append_many` that is the order the run was asked for, at
    /// consecutive ordinals and consecutive keys.
    #[must_use]
    pub fn created(&self) -> &[Created<N>] {
        &self.created
    }

    /// The entries this operation renamed, in the order it renamed them.
    ///
    /// A sibling shift renames highest-ordinal-first, so this is that order and
    /// not the level's: reading it is how a caller sees the property the
    /// architecture document argues for, which is about the *intermediate*
    /// states an interrupted operation leaves.
    #[must_use]
    pub fn renamed(&self) -> &[Renamed<N>] {
        &self.renamed
    }

    /// Every path this operation left behind, created and renamed alike, in the
    /// order the effects landed.
    ///
    /// **Exactly the plan's own order**, and not all the creations followed by
    /// all the renames: the two differ for every mixed plan, which is every
    /// `insert` and every promotion carrying a first child.
    pub fn paths(&self) -> impl Iterator<Item = &Path> {
        self.landed.iter().map(|landing| match landing {
            Landing::Created(at) => self.created[*at].path.as_path(),
            Landing::Renamed(at) => self.renamed[*at].to.as_path(),
        })
    }
}
````
<!-- /fragment -->

<a id="operation-variants"></a>
## Append and append-many

`append` delegates to `append_many` with one entry. `append_many` resolves one
target level, reads that level's greatest ordinal and the whole tree's greatest
key once, then advances both counters for the requested run. An empty run builds
an empty guarded plan, which proceeds and later produces an empty report.

The ordinal comes from the target level; the key comes from the entire snapshot.
Both use the greatest value rather than a count. A hand-edited gap therefore
remains a gap, and no allocation reuses a key merely because it lies in another
level. The loop validates every entry before returning, so bytes paired with
node parts refuse the whole decision rather than producing a partial plan.

One decision produces one guarded plan for the run. Its atomicity is bounded:
when a forward effect reports failure and rollback completes, the whole run is
restored; process death and rollback failure can leave an intermediate state.
The next page explains those interpreter outcomes.

The append planner owns consecutive allocation for one planned run. This
fragment turns one resolved level and a vector of new entries into consecutive
create effects, preserves pre-existing gaps, and supplies no effects at all for
the empty-run case used to explain a successful no-op.

<!-- fragment «ops-append» owner="mutation-algebra-k15" source="crates/ordinal-fs-tree/src/ops.rs" lines="69-144" parent="mutation-operations-source" -->
````rust

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
````
<!-- /fragment -->

<a id="promotion"></a>
## Promotion

Promotion names an entry by key rather than a level by `Target`. It requires a
leaf, a domain-defined distinguished name, and supplied parts that imply a node.
The caller supplies those parts because the algebra treats `Parts` as opaque and
cannot derive a domain-specific leaf-to-node mapping.

The plan first creates a node at the leaf's existing ordinal and key, then moves
the leaf file into that node under the distinguished name. An optional first
child adds a third create in `Level::Created(0)` with ordinal 1 and one fresh
key. The node itself consumes no key because it is the same entry under a new
shape.

The first effect must precede the move because the move's destination directory
does not yet exist. Between those effects the old leaf and new node coexist with
the same ordinal and key. No alternative order removes that intermediate state:
identity preservation requires those values, and the destination must exist
before content can enter it. The exclusive lock prevents cooperating readers
from observing the state; the next page covers interruption and rollback.

The promotion planner owns the fixed refusal precedence and the only plan that
creates a destination level for a later effect. This fragment turns one leaf
identity into create-then-move, optionally adds a first child, preserves the
promoted key and ordinal, and exposes the unavoidable transient duplication in
the plan's written order.

<!-- fragment «ops-promote» owner="mutation-algebra-k15" source="crates/ordinal-fs-tree/src/ops.rs" lines="252-396" parent="mutation-operations-source" -->
````rust

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
    let Some(leaf) = snapshot.by_key(key) else {
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
        return Decision::Refuse(Refusal::PromoteNoDistinguished { key });
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
````
<!-- /fragment -->

<a id="rewrite"></a>
## Rewrite and the successful no-op

Rewrite also names an entry by key. It takes the target's current ordinal and
key, substitutes the supplied parts, and checks that old and new parts imply the
same positioned species. The single `MoveTo` remains in the entry's current
level, so rewriting can change consumer attributes without moving the entry or
changing its identity.

Supplying the parts already present is valid and produces a guarded move onto
the same name. Occupancy excludes the mover, so the algebra proceeds rather than
refusing its own no-op. The interpreter separately recognises equal source and
destination paths; both layers are required for the public operation to succeed
without changing the filesystem.

The rewrite planner owns species preservation and name recomposition for opaque
attribute changes. This fragment turns a keyed entry and replacement parts into
one same-level move, preserves ordinal and key by construction, and keeps the
same-parts request in the successful half of the total decision.

<!-- fragment «ops-rewrite» owner="mutation-algebra-k15" source="crates/ordinal-fs-tree/src/ops.rs" lines="397-476" parent="mutation-operations-source" -->
````rust

/// **`rewrite`**: replace an entry's parts, keeping its ordinal, its key and its
/// species.
///
/// One effect — a rename onto the same level — and that is the whole operation:
/// `planRewrite` in the model, which builds `MoveTo(i, parentOf(f, i),
/// compose(ordOf(n), keyOf(n), p))` and guards it. This is how an attribute
/// changes, and `docs/adr/entries-are-never-removed.md` is why the operation
/// matters more than its size: with no removal, a domain retires an entry by
/// rewriting an attribute.
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
    let Some(entry) = snapshot.by_key(key) else {
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
````
<!-- /fragment -->

The shared helpers convert public targets into snapshot levels and derive both
allocation maxima from names already present. This fragment takes containers or
the whole snapshot as input, preserves the distinction between per-level
ordinals and tree-wide keys, and keeps every operation on the same resolution
and allocation rules used by the worked insert.

<!-- fragment «ops-resolution-and-allocation» owner="mutation-algebra-k15" source="crates/ordinal-fs-tree/src/ops.rs" lines="477-543" parent="mutation-operations-source" -->
````rust

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
````
<!-- /fragment -->

<a id="refusals"></a>
## Refusals are decisions

`Decision` has exactly two variants. `Proceed` carries a checked plan;
`Refuse` carries a stated reason and no effects. Every refusal below is created
while inspecting the snapshot, request, or plan value, before the filesystem
interpreter is called. It therefore changes nothing.

- `TargetMissing` comes from target resolution in append, insert, promote, or
  rewrite when no positioned entry carries the requested key.
- `TargetNotNode` comes from append or insert resolution when the target key
  names a leaf rather than a child-holding level.
- `NoOccupantAtOrdinal` comes from insert before effect construction. It covers
  an empty level, a hole at or below the greatest ordinal, and a request past the
  last sibling; the carried least/greatest span lets the message distinguish
  those states without inventing a lower neighbour.
- `ContentForANode` comes from every operation path that creates a new entry
  when node-implying parts are paired with bytes.
- `PromoteNotLeaf`, `PromoteNoDistinguished`, and
  `PromotePartsNotNode` are checked by promotion in that order. The order is
  observable when several conditions are false at once.
- `RewriteSpeciesChange` comes from rewrite when replacement parts imply the
  opposite positioned species.
- `KeysExhausted` comes from any path allocating `greatest key + 1` when the
  snapshot already contains `u32::MAX`.
- `OrdinalsExhausted` comes from append or insert when incrementing an ordinal
  would overflow.
- `DestinationOccupied` comes only from the ordered plan guard. It records the
  ordinal and key of the destination when those components exist.

The decision and refusal types own the total result and its complete algebraic
taxonomy. This fragment turns every failed precondition or guarded destination
into an explicit value, establishes that refusal is the no-effects branch, and
keeps exhaustion cases visible even though the unbounded formal model cannot
pose them.

<!-- fragment «plan-decision-and-refusals» owner="mutation-algebra-k15" source="crates/ordinal-fs-tree/src/plan.rs" lines="230-419" parent="mutation-plan-source" -->
````rust

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
    /// `promote` was called in a domain whose [`EntryName::distinguished`] is
    /// `None`. `operations.qnt`'s `RefusedPromoteNoDistinguished`, and the whole
    /// content of its `no_distinguished` instance.
    ///
    /// Refused outright rather than guessed at: promotion moves the leaf's
    /// content into the new node's distinguished child, and a domain with no
    /// distinguished child gives that content nowhere to go. The alternatives
    /// are discarding it silently and inventing a name the domain never
    /// declared, and neither is one.
    PromoteNoDistinguished {
        /// The key of the leaf that would have been promoted.
        key: Key,
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
````
<!-- /fragment -->

Refusal display is generic library vocabulary rather than consumer-specific
rendering. The formatter owns recovery advice for each carried state, including
the three `NoOccupantAtOrdinal` cases and the asymmetric rewrite advice: a leaf
may be promoted into a node, but no operation can turn a populated node into a
leaf without deciding what happens to its children.

The refusal formatter owns the public explanation of every pure refusal. This
fragment turns carried keys, species, ordinals, and spans into precise recovery
text, preserves the distinctions made at each decision site, and gives the
worked insert's refusal alternatives meaning without requiring filesystem work.

<!-- fragment «plan-refusal-messages» owner="mutation-algebra-k15" source="crates/ordinal-fs-tree/src/plan.rs" lines="420-568" parent="mutation-plan-source" -->
````rust

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
            Self::PromoteNoDistinguished { key } => write!(
                f,
                "this domain has no distinguished child, so promoting the leaf \
                 with key {key} would leave its content nowhere to go. Promotion \
                 moves a leaf's bytes verbatim into the new node's distinguished \
                 child; give the domain one by implementing \
                 `EntryName::distinguished`, or create the node and move the \
                 content yourself."
            ),
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
                         go, and entries are never removed.",
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
````
<!-- /fragment -->

The report types implement `Debug` without requiring the consumer's name type to
implement `Debug`; names already have the `Display` rendering the library is
allowed to use. This fragment turns reports into diagnostic structures without
widening the `EntryName` contract and completes the exact report source owned by
this page.

<!-- fragment «report-debug» owner="mutation-algebra-k15" source="crates/ordinal-fs-tree/src/report.rs" lines="120-152" parent="mutation-report-source" -->
````rust

// `Debug` by hand rather than by derive, for the reason `Triple` and `Entry`
// give: a derive would bound `N: Debug`, and a spurious bound on a public type
// propagates into consumers' signatures. A name is `Display`, which is the one
// rendering the library knows about.
impl<N: EntryName> fmt::Debug for Created<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Created")
            .field("name", &self.name.to_string())
            .field("path", &self.path)
            .finish()
    }
}

impl<N: EntryName> fmt::Debug for Renamed<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Renamed")
            .field("name", &self.name.to_string())
            .field("from", &self.from)
            .field("to", &self.to)
            .finish()
    }
}

impl<N: EntryName> fmt::Debug for Report<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Report")
            .field("created", &self.created)
            .field("renamed", &self.renamed)
            .field("landed", &self.landed)
            .finish()
    }
}
````
<!-- /fragment -->

The algebraic seam is now complete. Append, append-many, insert, promote, and
rewrite all consume one immutable snapshot and return either a guarded ordered
plan or a refusal. The plan records what must happen; the filesystem interpreter
on the next page determines whether those effects land and records what actually
happened in a report.

[Previous: Read path](04-read-path.md) | [Contents](README.md) | [Next: Filesystem interpreter](06-filesystem-interpreter.md)
