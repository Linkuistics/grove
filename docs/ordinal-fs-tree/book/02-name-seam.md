# Name seam
<!-- book-page id="name-seam" slice="name-seam-k12" order="2" -->
[Previous: Orientation](01-orientation.md) | [Contents](README.md) | [Next: Reference domain](03-reference-domain.md)

The filename seam converts filesystem observations into values the algebra can
use, and converts algebraic values back into filenames. The consumer owns every
grammar and vocabulary choice. The library owns only ordinal position, stable
identity, classification outcomes, and the laws that keep parsing and
composition mutually consistent.

The complete `src/name.rs` source is the composition below. Its six children
follow the conceptual order of this page and expand without gaps into lines
1–700.

<!-- fragment «name-seam-source» owner="name-seam-k12" source="crates/ordinal-fs-tree/src/name.rs" lines="1-716" parent="source-name" -->
<!-- insert «name-identifiers» -->
<!-- insert «name-classification» -->
<!-- insert «name-representation» -->
<!-- insert «entry-name-trait» -->
<!-- insert «entry-name-derived-readings» -->
<!-- insert «name-component-check» -->
<!-- /fragment -->

<a id="ordinal-and-key"></a>
## Mutable position and stable identity

A positioned filename carries two independent numbers. The ordinal is a
per-level locator and the only sibling sort input. Inserting at ordinal 2 shifts
the previous occupants at ordinals 2 and above, so a durable reference cannot
use an ordinal. The key identifies one entry across shifts, moves, relabelling,
and promotion. It is unique across the whole tree rather than within one level.

For a second consumer whose grammar is `<ordinal>-<state>-<label>-k<key>.note`,
the sigil and extension are consumer choices rather than library rules. In its
rendering `02-draft-plan-k6.note`, the leading `02` can change while `k6`
cannot. A shift to ordinal 3 composes
`03-draft-plan-k6.note`: the entry moved, but every reference to key 6 still
resolves to it. `Ordinal` and `Key` wrap `u32` without imposing a filename
format; their `Display` implementations exist for diagnostics.

Fresh keys are calculated as the greatest key visible anywhere in the snapshot
plus one. The names therefore hold the allocation counter. Removal is absent
because deleting the greatest-keyed entry would lower that counter and permit a
later allocation to reissue an identity that another entry still references. A
consumer represents retirement in its opaque parts instead of deleting the
entry.

This fragment defines the module boundary and the two numeric types. The
consumer formats their values, while the algebra compares them and preserves
the distinction used by every later example on this page.
<!-- fragment «name-identifiers» owner="name-seam-k12" source="crates/ordinal-fs-tree/src/name.rs" lines="1-91" parent="name-seam-source" -->
````rust
//! The seam: what a name is, and the one trait a consumer implements.
//!
//! The library never parses a name, never formats one, and never learns that a
//! name is a string. It knows only that a name can be decomposed into an
//! [`Ordinal`], a [`Key`] and an opaque remainder — and recomposed from them.
//! Everything in this module is that decomposition and nothing else.
//!
//! The specification is `docs/ordinal-fs-tree/ARCHITECTURE.md`, sections *Names
//! belong to the consumer* and *The seam: one trait*; the structural model that
//! checked it is `docs/ordinal-fs-tree/models/structure.als`. Where a comment
//! here names a `check` or a `witness_…`, that is the claim it answers to.

use core::fmt;

/// An entry's **mutable** position among the siblings in one directory, and the
/// sole sort input within that level.
///
/// It is a locator, not an identity: an insert rewrites the ordinals at and
/// after it, so an ordinal is only true until the next insert. Store a [`Key`]
/// in a durable cross-reference, never one of these.
///
/// Ordinals are compared and nothing else — the library never renders one, since
/// how a number appears in a filename is the consumer's grammar. [`fmt::Display`]
/// is here for error messages and diagnostics, not for names.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ordinal(u32);

impl Ordinal {
    /// The ordinal a dense level starts at.
    ///
    /// Density — ordinals being exactly `1..n` — is *preserved* by every
    /// operation and never *established*, so this is not a floor the library
    /// enforces: a hand-edited level may hold anything, `0` included, and the
    /// library will neither notice nor repair it. It is the value an `append`
    /// into an empty node uses, and it is here so that the choice is written
    /// down once rather than spelled again at each such site.
    pub const FIRST: Self = Self(1);

    /// Wrap a number as an ordinal.
    #[must_use]
    pub const fn new(n: u32) -> Self {
        Self(n)
    }

    /// The number this ordinal wraps — what a consumer's grammar formats.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl fmt::Display for Ordinal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// An entry's identity: assigned once, unique across the whole tree, and never
/// rewritten.
///
/// It survives insertion, reordering, relabelling and being moved between
/// levels, which is what makes it the one thing safe to store in a durable
/// cross-reference — and the one handle every operation takes.
///
/// A fresh key is `max(key over the whole tree) + 1`: the names *are* the
/// counter. That is why there is no removal operation, and the reasoning is
/// [`docs/adr/entries-are-never-removed.md`].
///
/// [`docs/adr/entries-are-never-removed.md`]: https://github.com/Linkuistics/grove/blob/main/docs/adr/entries-are-never-removed.md
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Key(u32);

impl Key {
    /// Wrap a number as a key.
    #[must_use]
    pub const fn new(n: u32) -> Self {
        Self(n)
    }

    /// The number this key wraps — what a consumer's grammar formats.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
````
<!-- /fragment -->

<a id="worked-names"></a>
## Worked names: observation, verdict, and consequence

The filesystem reader supplies two inputs to `EntryName::parse`: one UTF-8
filename component and an unfollowed `Found`. `Found::Other` includes symbolic
links, sockets, and device nodes, so following a link cannot make a contradictory
entry appear valid. The consumer returns one `Verdict`; the sum type makes the
classification total and disjoint.

Consider a hypothetical document consumer whose grammar is
`<ordinal>-<state>-<label>-k<key>.note`, with `.document-lock` reserved by that
same consumer:

| Observed name and kind | Consumer verdict | Read consequence |
|---|---|---|
| `02-draft-plan-k6.note`, `Found::File` | `Entry(name)` with ordinal 2, key 6, document parts, and leaf species | Admit the entry; rendering the parsed name must reproduce the same filename. |
| `README.md`, `Found::File` | `Foreign` | Skip this name. If it were a directory, skip its entire subtree. |
| `02-draft-plan.note`, `Found::File` | `Malformed(error)` because the consumer recognises its grammar but the key is missing | Halt the whole snapshot and return the consumer's recovery advice. |
| `.document-lock`, `Found::File` | `Reserved(error)` | Halt because consumer-owned transaction state cannot safely be interpreted as an entry. |
| `02-draft-plan-k6.note`, `Found::Dir` | `Malformed(error)` because the leaf spelling contradicts the observed directory | Halt rather than hide a subtree behind a skipped name. |

Only the consumer decides which spellings are foreign, malformed, or reserved.
The library assigns the consequences. A foreign directory is recursively
absent from the snapshot; a malformed or reserved name anywhere makes the
entire snapshot fail when it appears in a directory the walk reaches. Descendants
of a foreign directory are never observed. An accepted spelling is canonical
only when formatting the parsed value yields that exact spelling. Accepting both
`2-draft-plan-k6.note`
and `02-draft-plan-k6.note` as the same value would make two filesystem entries
one algebraic name and could introduce duplicate ordinals and keys.

Species is the algebra's filesystem-shape vocabulary. Leaves and distinguished
children require regular files; nodes require directories. A distinguished
child is not positioned and traversal does not descend into it. The consumer
must reject a spelling whose species contradicts `Found` because only the
consumer can construct the domain-specific error that explains recovery.

This fragment turns an unfollowed filesystem observation into the shape and
classification values used by `parse`. The consumer supplies the verdict and
error; the reader uses the variant to admit, skip, or halt without guessing.
<!-- fragment «name-classification» owner="name-seam-k12" source="crates/ordinal-fs-tree/src/name.rs" lines="92-246" parent="name-seam-source" -->
````rust

/// Which of the three kinds of thing a name names.
///
/// The species **follows from the parts**: a consumer whose leaves and nodes
/// carry different metadata expresses that as variants of
/// [`EntryName::Parts`], and the library never needs to be told which it is
/// looking at.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Species {
    /// A regular file with no children.
    Leaf,
    /// A directory holding zero or more children.
    Node,
    /// A node's own content rather than one of its children: a regular file
    /// carrying neither ordinal nor key, which a walk does not descend into.
    Distinguished,
}

impl Species {
    /// What a name of this species must be on disk.
    ///
    /// A distinguished child requires a regular *file* for the same reason a
    /// leaf does, and it is the reason worth stating: a walk does not descend
    /// into a distinguished child, so one that were a directory would hide an
    /// entire subtree from every traversal while the tree reported itself
    /// healthy. `structure.als` exhibits exactly that tree —
    /// `witness_distinguished_directory_hides_a_subtree`.
    ///
    /// This is the library's half of the fifth obligation; the consumer's half
    /// is [`EntryName::parse`] refusing what `found` contradicts. Alloy calls
    /// the pair `agreesWith` / `SpeciesAgreementIsParsed`.
    #[must_use]
    pub const fn requires(self) -> Found {
        match self {
            Self::Leaf | Self::Distinguished => Found::File,
            Self::Node => Found::Dir,
        }
    }

    /// Whether what the listing reported agrees with what this name declares.
    #[must_use]
    pub const fn agrees_with(self, found: Found) -> bool {
        matches!(
            (self.requires(), found),
            (Found::File, Found::File) | (Found::Dir, Found::Dir)
        )
    }
}

impl fmt::Display for Species {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Leaf => "leaf",
            Self::Node => "node",
            Self::Distinguished => "distinguished child",
        })
    }
}

/// Which of the two kinds of thing a **positioned** name names.
///
/// [`Species`] minus the distinguished child, which is the species no
/// positioned name can have. It exists so that
/// [`EntryName::positioned_species`] can be a function of
/// [`Parts`](EntryName::Parts) *and of nothing else*: the ordinal and the key
/// are not in scope there, so a species that changes when an entry is shifted
/// cannot be written. `structure.als` assumes exactly that in
/// `SpeciesFromParts`, and every derived operation rests on it — a shift is a
/// `compose` with a new ordinal, and a shift that changed a leaf into a node
/// would be a shift that renamed a file into a directory.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PositionedSpecies {
    /// A regular file with no children.
    Leaf,
    /// A directory holding zero or more children.
    Node,
}

impl PositionedSpecies {
    /// The same thing, widened to the three-way [`Species`].
    #[must_use]
    pub const fn species(self) -> Species {
        match self {
            Self::Leaf => Species::Leaf,
            Self::Node => Species::Node,
        }
    }
}

impl fmt::Display for PositionedSpecies {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.species(), f)
    }
}

/// What a directory listing reports is under a name, **unfollowed**.
///
/// Unfollowed is load-bearing: a symbolic link wearing an entry's name is
/// [`Found::Other`], so a consumer's `parse` sees it as the contradiction it is
/// rather than as whatever it points at.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Found {
    /// A regular file.
    File,
    /// A directory.
    Dir,
    /// Anything else — a symbolic link, a socket, a device node.
    Other,
}

impl fmt::Display for Found {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::File => "a regular file",
            Self::Dir => "a directory",
            Self::Other => "neither a regular file nor a directory",
        })
    }
}

/// The outcome of classifying one directory entry.
///
/// Three outcomes, not two, plus a narrow fourth — and the distinction that
/// matters is between [`Foreign`](Verdict::Foreign) and
/// [`Malformed`](Verdict::Malformed). A `README.md` sitting in the tree is
/// foreign: skipping it is correct and costs nothing. A name that is *almost*
/// one of yours — a typo, a hand-edit, a mangled attribute — is not foreign,
/// and skipping it is data loss. When the skipped name is a *directory*, an
/// entire subtree vanishes from every traversal while the tree still reports
/// itself as healthy.
///
/// So: a name the consumer recognises as its own must either parse completely
/// or halt the operation. It may never be silently ignored.
///
/// Totality and disjointness of the trichotomy are guaranteed by this being a
/// sum type and by nothing else — Alloy checked it
/// (`TrichotomyIsTotalAndDisjoint`) and found it free, which is a way of saying
/// the model was testing the compiler.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Verdict<N, E> {
    /// A well-formed entry: walk it, order it, count it.
    Entry(N),
    /// Not this consumer's name. Ignore it completely — and recursively, since
    /// skipping a foreign *directory* skips everything beneath it. That is
    /// sound precisely because the consumer said the name was not its own.
    Foreign,
    /// This consumer's name, and broken. Halt, carrying the consumer's own
    /// error so the refusal can say what to *do* about it.
    Malformed(E),
    /// This consumer's name, and deliberately not an entry — a transaction
    /// witness, a lock marker, a sentinel left by an interrupted operation.
    /// Halts work the same way a malformed name does, and for the same reason:
    /// the library cannot know what it means, so proceeding past it is a guess.
    Reserved(E),
}
````
<!-- /fragment -->

<a id="triple-and-view"></a>
## The algebraic representation of a name

After parsing, the algebra does not retain the filename string. A positioned
name exposes a `Triple` containing its ordinal, key, and borrowed `Parts`.
`Parts` combines the label and every consumer-defined attribute into one opaque
associated type. The library may clone and compare parts but cannot construct,
inspect, or render them independently.

`NameView` represents the complete structural choice. `Positioned(Triple)`
contains all three positioned fields together. `Distinguished` contains none of
them. This shape prevents a leaf with no ordinal or a distinguished child with
a key from being represented. Manual `Clone` and `Copy` implementations avoid
requiring `Parts: Clone` merely because `Triple` and `NameView` borrow parts.

For the accepted example, the view is conceptually
`Positioned { ordinal: 2, key: 6, parts: document(draft, "plan") }`. The
distinguished spelling, when this consumer defines one, produces
`NameView::Distinguished` and has no triple.

This fragment defines the values passed from the consumer boundary into the
algebra. Parsing produces a name whose view supplies the triple; later mutation
code reuses that triple when it composes a shifted or rewritten name.
<!-- fragment «name-representation» owner="name-seam-k12" source="crates/ordinal-fs-tree/src/name.rs" lines="247-345" parent="name-seam-source" -->
````rust

/// The `(ordinal, key, parts)` a positioned name is isomorphic to.
///
/// *Isomorphic* is load-bearing, and it means both directions at once:
/// [`EntryName::compose`] recovers the name from the triple, and
/// [`EntryName::triple`] recovers the triple from the name. Everything the
/// algebra does, it does on these — the library holds no strings.
///
/// The distinguished child has no triple, which is what makes
/// [`EntryName::triple`] return an [`Option`]; see that method.
pub struct Triple<'a, P> {
    /// Where the entry sits among its siblings.
    pub ordinal: Ordinal,
    /// Which entry this is.
    pub key: Key,
    /// Everything the library does not understand.
    pub parts: &'a P,
}

// `Clone` and `Copy` by hand rather than by derive. A derive would generate
// `impl<P: Clone> Clone for Triple<'_, P>`, and this type holds a *reference* to
// `P` — so the bound is spurious, and a spurious bound on a public type is the
// kind that propagates into consumers' signatures for no reason. The same
// avoidance is why the seam is one trait on the name rather than a domain type
// threaded through the tree; see `docs/adr/entry-name-is-the-only-seam.md`.
impl<P> Clone for Triple<'_, P> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<P> Copy for Triple<'_, P> {}

impl<P: fmt::Debug> fmt::Debug for Triple<'_, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Triple")
            .field("ordinal", &self.ordinal)
            .field("key", &self.key)
            .field("parts", self.parts)
            .finish()
    }
}

impl<P: PartialEq> PartialEq for Triple<'_, P> {
    fn eq(&self, other: &Self) -> bool {
        self.ordinal == other.ordinal && self.key == other.key && self.parts == other.parts
    }
}

impl<P: Eq> Eq for Triple<'_, P> {}

/// What a name *is*: a positioned entry with a triple, or the distinguished
/// child with none.
///
/// This is the whole of the choice, in one value, and that is what makes the
/// obligation *a name is positioned or distinguished, never neither* — and its
/// other half, *never both* — unrepresentable rather than checkable. Under
/// separate `triple()` and `species()` accessors a name could return `None` and
/// [`Species::Leaf`] together (`witness_leaf_name_without_an_ordinal`), or a
/// triple and [`Species::Distinguished`] together; neither can be written now.
///
/// [`EntryNameExt::triple`] and [`EntryNameExt::species`] are read off this, and
/// are derived rather than implemented for the same reason a sibling shift is.
pub enum NameView<'a, P> {
    /// An ordinary entry, isomorphic to its triple.
    Positioned(Triple<'a, P>),
    /// A node's own content: no ordinal, no key, no parts.
    Distinguished,
}

impl<P> Clone for NameView<'_, P> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<P> Copy for NameView<'_, P> {}

impl<P: fmt::Debug> fmt::Debug for NameView<'_, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Positioned(t) => f.debug_tuple("Positioned").field(t).finish(),
            Self::Distinguished => f.write_str("Distinguished"),
        }
    }
}

impl<P: PartialEq> PartialEq for NameView<'_, P> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Positioned(a), Self::Positioned(b)) => a == b,
            (Self::Distinguished, Self::Distinguished) => true,
            _ => false,
        }
    }
}

impl<P: Eq> Eq for NameView<'_, P> {}

````
<!-- /fragment -->

<a id="entry-name-contract"></a>
## The one consumer seam

`EntryName` is the only point where the library is parameterised by a consumer.
The name type owns parsing, composition, distinguished naming, structural view,
species selection, error advice, and its single `Display` rendering. There is no
domain object, callback registry, locking hook, or second formatting surface.

The trait establishes seven obligations:

1. `compose(o, k, p)` exposes exactly `o`, `k`, and `p` through its view.
2. Parsing and rendering are canonical in both directions: formatting an
   accepted filename reproduces its exact bytes, while a composed or
   distinguished name reparses with the same view and species.
3. A name is positioned or distinguished, never neither or both.
4. A positioned species depends only on parts, never on ordinal or key.
5. `distinguished()` names the only spelling that parses as distinguished.
6. `parse` returns `Malformed` when the declared species contradicts `Found`.
7. `Display` renders one nonempty path component other than `.` or `..`.

The signatures constrain the visible structural forms of obligations 3 and 4.
Each `view` result is one complete positioned-or-distinguished choice, and
`positioned_species` receives only `&Parts`, leaving the triple's ordinal and
key out of scope. Rust does not prove that consumer methods are deterministic
or free of hidden mutable state; stability across calls remains part of the
semantic trait contract. The remaining five are also semantic obligations Rust
cannot check; the reference domain's reusable conformance kit exercises them on
consumer-supplied samples. The filesystem layer separately enforces the seventh
before a rendering can become a path. The [reference-domain
chapter](03-reference-domain.md#conformance-obligations) defines both mechanisms
where their implementation is introduced.

Composition is total and infallible. In the orientation insert, the algebra
reads the triple for `02-published-vectors-i5.md` and composes ordinal 3 with the
same key and parts. The generic operation never splices a string and cannot
alter a label or attribute accidentally. New entries similarly receive the
requested ordinal, a fresh key, and caller-supplied parts.

Species follows from parts so a shift cannot turn a regular file into a
directory. `Parts: Eq` does not require equality to distinguish every rendered
spelling or species. The derived `same_name` reading below refines triple
equality with species for occupancy decisions. It does not reconstruct exact
rendering identity when equal parts of the same species render differently;
the parsing and rendering round trips constrain that separate boundary.

This fragment is present at the seam itself. A consumer implements these inputs
and transformations; snapshot reads and mutation planning depend on the stated
round trips, species rule, and one-component rendering contract.
<!-- fragment «entry-name-trait» owner="name-seam-k12" source="crates/ordinal-fs-tree/src/name.rs" lines="346-615" parent="name-seam-source" -->
````rust
/// The one trait. All genericity lives here: there are no callbacks, no hooks,
/// no registration and no configuration objects, and there is no `Domain` type.
///
/// A name **is** a type — one wrapping a string, owning its own parsing,
/// validation and formatting. [`fmt::Display`] is that formatting, and it is
/// the only rendering the library knows about.
///
/// # What an implementation must guarantee
///
/// Seven obligations. Six of them the library assumes and cannot check at run
/// time; the seventh it **enforces**, and the asymmetry is stated below rather
/// than left to be noticed. They are stated because the structural model found
/// that four were missing, and that a design missing any one of them admits a
/// tree the library will quietly corrupt. Each is written on the method it
/// constrains — except the seventh, which constrains [`fmt::Display`] and is
/// therefore written here. [`crate::conformance`] samples five semantic
/// obligations and separately publishes the visible constraints Rust places on
/// the other two. [`view`](EntryName::view) and
/// [`positioned_species`](EntryName::positioned_species) carry those
/// constraints; deterministic answers across calls remain semantic laws.
///
/// # Obligation: a name renders as one path component
///
/// [`fmt::Display`] yields exactly one filename: not the empty string, not `.`
/// or `..`, and never anything holding a path separator. The library joins that
/// rendering to a level's directory to reach the entry, so a name rendering as
/// `../outside` or as an absolute path would make a create, a rename, a
/// rollback removal and every reported path address **outside the tree whose
/// containing directory is the only thing locked** — which is the library's
/// central proposition, *one directory tree is the data structure*, broken by a
/// value the algebra never sees. Occupancy compares
/// [`view`](EntryName::view)s, so the composed name looks perfectly canonical
/// while the path it renders does not.
///
/// **This is the one obligation the library does not merely assume.** Neither
/// model can pose it — both hold no strings by design, exactly as they hold no
/// bytes — so there is no witness to point at and no checked claim behind it.
/// What there is instead is a boundary: every name a snapshot admits and every
/// name a plan will place is rendered and checked before it becomes a path, and
/// a violation is [`Error::NameIsNotOneComponent`] rather than an escape.
/// [`crate::conformance`] checks it too, so a cooperative domain meets it in a
/// test rather than in an operation.
///
/// [`Error::NameIsNotOneComponent`]: crate::Error::NameIsNotOneComponent
pub trait EntryName: Sized + Clone + fmt::Display {
    /// Everything the library does not understand: the label, and whatever
    /// attributes the domain carries. Entirely opaque.
    ///
    /// The bounds are the whole of what the library may do with one — copy a
    /// value it already holds, and compare two of them. It has no constructor
    /// for a `Parts`, which is why `promote` takes the promoted node's parts
    /// from its caller rather than deriving them
    /// (`witness_promote_cannot_name_its_output`).
    ///
    /// # What `Eq` here does *not* promise
    ///
    /// Any lawful equivalence relation, including one **coarser** than the
    /// domain's own rendering: two parts that compare equal may still render as
    /// two filenames, and may still name different species. Both models make the
    /// opposite assumption for free — `structure.als` compares `Parts` atoms and
    /// `operations.qnt` compares ints, so in each of them equal parts are the
    /// *same* parts — and neither can pose a coarser one. So the library does
    /// not derive name identity from parts equality alone:
    /// [`EntryNameExt::same_name`] is where that is decided, and it is what
    /// occupancy compares.
    type Parts: Clone + Eq;

    /// The domain's own error, so a refusal can carry recovery advice.
    ///
    /// Detection alone produces errors that are useless to whoever hits them.
    /// A domain contributes both the detection and the advice, or the library's
    /// refusals say only that something is wrong.
    type Err: std::error::Error + Send + Sync + 'static;

    /// Classify one directory entry: its name, and what the listing reports is
    /// under that name, unfollowed.
    ///
    /// # Obligation: the grammar is canonical
    ///
    /// Distinct filenames never parse to the same name — `format(parse(f)) == f`,
    /// and not merely `parse(format(n)) == n`. State one direction only and a
    /// grammar may accept two spellings of one name, at which point two files on
    /// disk *are* one entry, sharing a key and an ordinal, and the tree carries a
    /// duplicate key that no invariant rules out
    /// (`witness_two_filenames_name_one_entry`).
    ///
    /// The cheapest way to hold it: parse leniently, render the result, and
    /// refuse if the rendering differs from the input.
    ///
    /// # Obligation: `parse` refuses what `found` contradicts
    ///
    /// A name declaring [`Species::Leaf`] over a directory, or [`Species::Node`]
    /// over a regular file, is [`Verdict::Malformed`] and never
    /// [`Verdict::Entry`]. The second argument is not decoration: the library can
    /// see the contradiction and has no domain error value with which to report
    /// it, so the judgement belongs where the recovery advice already lives
    /// (`witness_species_mismatch_is_unclassifiable`).
    fn parse(name: &str, found: Found) -> Verdict<Self, Self::Err>;

    /// Build a positioned name. The species follows from `parts`.
    ///
    /// Infallible and total, which is what lets the sibling shift be *derived*
    /// rather than implemented: shifting is `compose(new_ordinal, key, parts)`
    /// and nothing else.
    ///
    /// # Obligation: compose places what it is given
    ///
    /// `compose(o, k, p)` yields a name whose [`triple`](EntryName::triple) is
    /// `Some` with `ordinal == o`, `key == k` and `parts == p`. Without this the
    /// isomorphism says nothing, and the sibling shift is free to move one
    /// entry's key onto another's position while every stated invariant still
    /// holds (`witness_shift_corrupts_identity`).
    fn compose(ordinal: Ordinal, key: Key, parts: Self::Parts) -> Self;

    /// The name a node's distinguished child takes, if this domain has one.
    ///
    /// A distinguished child carries neither an ordinal nor a key, so it can
    /// never be produced by [`compose`](EntryName::compose) — this is the only
    /// way the library can name one. `None` means the domain has no
    /// distinguished child, and promotion is refused rather than guessed at.
    ///
    /// # Obligation: `distinguished()` names the only entry of its species
    ///
    /// [`parse`](EntryName::parse) yields [`Species::Distinguished`] for this
    /// name and for nothing else, and this name's own
    /// [`triple`](EntryName::triple) is `None`. That is what makes *at most one
    /// distinguished child per node* true — the filesystem supplies the rest,
    /// since a directory cannot hold two entries of one name — so it is a
    /// theorem rather than an invariant anything has to enforce
    /// (`DistinguishedIsUniquePerNode`, against
    /// `witness_two_distinguished_children`).
    fn distinguished() -> Option<Self> {
        None
    }

    /// What this name is: a positioned entry with its triple, or the
    /// distinguished child.
    ///
    /// One method rather than the `ordinal()` / `key()` / `parts()` /
    /// `species()` accessors an earlier draft had, because the choice is one
    /// choice. Read a triple off it with [`EntryNameExt::triple`] and a species
    /// with [`EntryNameExt::species`]; both are derived from this and from
    /// [`positioned_species`](EntryName::positioned_species), so an
    /// implementation cannot override the derived readings independently. The
    /// readings still depend on the implementation honoring their deterministic
    /// call laws.
    ///
    /// # Obligation: a name is positioned or distinguished, never neither
    ///
    /// Rust constrains each returned value, and both halves of its visible
    /// shape. The architecture document states the obligation of three separate
    /// `Option` accessors — `ordinal()`, `key()` and `parts()`, which "are `Some`
    /// together or `None` together" — beside an independent species. That
    /// admits both a leaf with no ordinal and a name carrying a triple while
    /// claiming species [`Species::Distinguished`]
    /// (`witness_leaf_name_without_an_ordinal`). [`NameView`] carries the triple
    /// and the positioned-or-distinguished choice in one returned value, so
    /// neither malformed value can be returned.
    ///
    /// The type does **not** make the choice stable across calls. An
    /// implementation can consult interior or global mutable state and return
    /// `Positioned` once and `Distinguished` next on repeated calls with the
    /// same receiver and no caller-visible mutation. The semantic law is
    /// therefore explicit: hidden state does not affect `view`'s answer. Rust
    /// does not enforce that law, and a finite sample in [`crate::conformance`]
    /// cannot prove it.
    fn view(&self) -> NameView<'_, Self::Parts>;

    /// The species of a positioned name carrying these parts.
    ///
    /// # Obligation: the species follows from the parts
    ///
    /// Rust constrains the method's explicit inputs: this is an associated
    /// function of the *name type* over a `&Parts`, so it receives no `self`,
    /// ordinal or key. A domain whose leaves and nodes differ expresses that as
    /// variants of [`Parts`](EntryName::Parts).
    ///
    /// `structure.als` assumes it as `SpeciesFromParts`, and the derivation
    /// that rests on it is the sibling shift: shifting is
    /// `compose(new_ordinal, key, parts)`, so a species that could vary with
    /// the ordinal would make a shift able to turn a leaf into a node — a
    /// rename of a file into a directory, with the subtree that implies.
    ///
    /// # What the signature covers, and what it does not
    ///
    /// It prevents the body from reading a name, ordinal or key through an
    /// explicit parameter. It does not prevent global mutable state, so the
    /// semantic law remains an assumption: `positioned_species` is deterministic
    /// from the parts value across calls. It also does not make the answer a
    /// function of the parts' *equivalence class*: the
    /// bound on [`Parts`](EntryName::Parts) is `Eq` and not "equality as fine as
    /// this function", so `a == b` with differing species is lawful and breaks
    /// no obligation. That is a real domain — `promote-k25` wrote one — and the
    /// library meets it by comparing the species itself wherever two names are
    /// asked to be one; see [`EntryNameExt::same_name`]. Requiring the
    /// congruence of a domain instead was the alternative, and it was rejected:
    /// no sample of parts can exercise it, so [`crate::conformance`] could only
    /// ever report it untested.
    ///
    /// The compile-time control below establishes only the explicit-input
    /// constraint. This domain wants its species to depend directly on where
    /// the entry sits, and does not compile because there is no `self` to ask:
    ///
    /// ```compile_fail
    /// # use core::fmt;
    /// # use ordinal_fs_tree::{EntryName, Found, Key, NameView, Ordinal, PositionedSpecies, Verdict};
    /// # use ordinal_fs_tree::reference::{Parts, SyllabusError, SyllabusName};
    /// #[derive(Clone)]
    /// struct OrdinalDependent(SyllabusName);
    /// # impl fmt::Display for OrdinalDependent {
    /// #     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Display::fmt(&self.0, f) }
    /// # }
    /// impl EntryName for OrdinalDependent {
    ///     type Parts = Parts;
    ///     type Err = SyllabusError;
    /// #   fn parse(n: &str, f: Found) -> Verdict<Self, Self::Err> {
    /// #       match SyllabusName::parse(n, f) {
    /// #           Verdict::Entry(n) => Verdict::Entry(Self(n)),
    /// #           Verdict::Foreign => Verdict::Foreign,
    /// #           Verdict::Malformed(e) => Verdict::Malformed(e),
    /// #           Verdict::Reserved(e) => Verdict::Reserved(e),
    /// #       }
    /// #   }
    /// #   fn compose(o: Ordinal, k: Key, p: Self::Parts) -> Self { Self(SyllabusName::compose(o, k, p)) }
    /// #   fn view(&self) -> NameView<'_, Self::Parts> { self.0.view() }
    ///     fn positioned_species(parts: &Self::Parts) -> PositionedSpecies {
    ///         // A node at an even ordinal, a leaf at an odd one. It cannot be
    ///         // said: no `self`, and no ordinal on `parts`.
    ///         if self.0.ordinal().get() % 2 == 0 {
    ///             PositionedSpecies::Node
    ///         } else {
    ///             PositionedSpecies::Leaf
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// And the other half of the control, since a `compile_fail` that fails for
    /// the wrong reason proves nothing: the same domain, differing only in the
    /// body, compiles.
    ///
    /// ```
    /// # use core::fmt;
    /// # use ordinal_fs_tree::{EntryName, Found, Key, NameView, Ordinal, PositionedSpecies, Verdict};
    /// # use ordinal_fs_tree::reference::{Parts, SyllabusError, SyllabusName};
    /// #[derive(Clone)]
    /// struct PartsDependent(SyllabusName);
    /// # impl fmt::Display for PartsDependent {
    /// #     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Display::fmt(&self.0, f) }
    /// # }
    /// impl EntryName for PartsDependent {
    ///     type Parts = Parts;
    ///     type Err = SyllabusError;
    /// #   fn parse(n: &str, f: Found) -> Verdict<Self, Self::Err> {
    /// #       match SyllabusName::parse(n, f) {
    /// #           Verdict::Entry(n) => Verdict::Entry(Self(n)),
    /// #           Verdict::Foreign => Verdict::Foreign,
    /// #           Verdict::Malformed(e) => Verdict::Malformed(e),
    /// #           Verdict::Reserved(e) => Verdict::Reserved(e),
    /// #       }
    /// #   }
    /// #   fn compose(o: Ordinal, k: Key, p: Self::Parts) -> Self { Self(SyllabusName::compose(o, k, p)) }
    /// #   fn view(&self) -> NameView<'_, Self::Parts> { self.0.view() }
    ///     fn positioned_species(parts: &Self::Parts) -> PositionedSpecies {
    ///         parts.species()
    ///     }
    /// }
    /// ```
    fn positioned_species(parts: &Self::Parts) -> PositionedSpecies;
}
````
<!-- /fragment -->

<a id="derived-readings"></a>
## Readings consumers cannot override

`EntryNameExt` is a sealed blanket implementation, not another seam. Every
`EntryName` receives `triple`, `species`, and `same_name` from `view` and
`positioned_species`; a consumer cannot supply inconsistent alternatives.

`triple` removes the distinguished case. `species` widens a positioned leaf or
node and assigns `Species::Distinguished` to the other view. `same_name`
compares both view and positioned species. This is the library's occupancy
equivalence, not byte-for-byte filename identity. The extra species comparison
matters when a lawful consumer equality treats two parts values as equal even
though one denotes a leaf and the other a node. Plan occupancy uses
`same_name`, so a promotion reusing an entry's ordinal and key does not mistake
its replacement node for the leaf being replaced. Equal same-species parts may
still render differently; `same_name` deliberately follows the consumer's
`Eq`, while canonical parsing and rendering are checked independently.

This fragment derives the readings used throughout snapshots, operations,
plans, and consumers. Sealing keeps the algebra's identity and species rules
uniform even though the underlying parts and their equality belong to the
consumer.
<!-- fragment «entry-name-derived-readings» owner="name-seam-k12" source="crates/ordinal-fs-tree/src/name.rs" lines="616-690" parent="name-seam-source" -->
````rust

mod sealed {
    /// Sealed so that [`super::EntryNameExt`] cannot be implemented by hand:
    /// the whole point of it is that its two methods are *derived*.
    pub trait Sealed {}
    impl<N: super::EntryName> Sealed for N {}
}

/// What every [`EntryName`] can do without implementing anything further.
///
/// Blanket-implemented and sealed, which is the load-bearing part: these are
/// not provided methods an implementation may override, they are readings of
/// [`EntryName::view`] and [`EntryName::positioned_species`]. That is what
/// constrains the visible shape of *a name is positioned or distinguished,
/// never neither* and *the species follows from the parts*. Stable answers
/// across calls remain semantic laws of [`EntryName`]; Rust permits an
/// implementation to consult hidden mutable state. An overridable `species()`
/// would weaken even the visible constraint by adding `self` as an explicit
/// input.
pub trait EntryNameExt: EntryName + sealed::Sealed {
    /// `Some` for a positioned name, `None` for the distinguished one.
    fn triple(&self) -> Option<Triple<'_, Self::Parts>> {
        match self.view() {
            NameView::Positioned(triple) => Some(triple),
            NameView::Distinguished => None,
        }
    }

    /// Which of the three kinds of thing this name names.
    fn species(&self) -> Species {
        match self.view() {
            NameView::Positioned(triple) => Self::positioned_species(triple.parts).species(),
            NameView::Distinguished => Species::Distinguished,
        }
    }

    /// Whether these two names are **one name** — the comparison every
    /// occupancy decision makes.
    ///
    /// The whole [`NameView`] *and the species*, and the second half is the
    /// part that is not obvious. [`Parts`](EntryName::Parts) is bounded by
    /// `Clone + Eq` and by nothing else, so a domain's equality may be any
    /// lawful equivalence — including one coarser than its own rendering.
    /// [`positioned_species`](EntryName::positioned_species) is a function of
    /// the parts *value* and not of its equivalence class, so two parts that
    /// compare equal may name different species. Nothing in the seam forbids
    /// that, and a domain doing it breaks no obligation.
    ///
    /// A promotion is where it becomes visible, because it is the one
    /// operation whose new name deliberately reuses the old one's ordinal and
    /// key: the parts are then all that is left to tell leaf from node, and
    /// under a coarse equality they say nothing. Comparing views alone finds
    /// the promoted node's destination occupied by the very leaf it replaces
    /// and refuses a valid promotion — `promote-k25`'s finding, and the reason
    /// this is a named reading rather than `view() == view()` spelled at each
    /// site.
    ///
    /// Two names this answers `false` for are two filenames. A domain whose
    /// leaf and node spellings coincided would fail the canonicity check in
    /// [`crate::conformance`], which reparses every composed name and compares
    /// the species that comes back.
    fn same_name(&self, other: &Self) -> bool {
        match (self.view(), other.view()) {
            (NameView::Positioned(a), NameView::Positioned(b)) => {
                a == b && Self::positioned_species(a.parts) == Self::positioned_species(b.parts)
            }
            (NameView::Distinguished, NameView::Distinguished) => true,
            (NameView::Positioned(_), NameView::Distinguished)
            | (NameView::Distinguished, NameView::Positioned(_)) => false,
        }
    }
}

impl<N: EntryName> EntryNameExt for N {}

````
<!-- /fragment -->

<a id="surrounding-flow"></a>
## Placement in read and mutation flow

During a read, `fs::read::snapshot` obtains each directory entry's unfollowed
kind and calls `N::parse`. `Entry` values enter the snapshot, `Foreign` values
are omitted, and `Malformed` or `Reserved` values become errors immediately.
Snapshot construction uses the derived species to decide whether to descend,
and ordering later reads ordinals from positioned views.

During a mutation, pure operations inspect triples and call `N::compose` to
build effect names. Insert shifts siblings highest-ordinal-first by composing a
new ordinal with each old key and cloned parts, then composes the new entry with
a fresh tree-wide key. `Plan::guarded` uses `same_name` when checking whether an
effect destination is occupied. The filesystem interpreter receives names only
after those algebraic decisions are complete.

On runtime read and mutation paths, string inspection occurs only at the
filesystem boundary. A parsed snapshot name is checked before it is admitted,
and every effect name is checked before the first effect runs. An empty
rendering, `.`, `..`, a slash, or a NUL yields
`Error::NameIsNotOneComponent`; no path outside the locked tree is joined and no
partial plan needs rollback for this boundary error.

This fragment implements that shared boundary predicate. The reader and
interpreter supply rendered names, and the result either certifies one Unix
filename component or gives the stable reason carried by the error.
<!-- fragment «name-component-check» owner="name-seam-k12" source="crates/ordinal-fs-tree/src/name.rs" lines="691-716" parent="name-seam-source" -->
````rust
/// Why a rendering is not one filename, or `None` when it is one.
///
/// The library's half of the obligation *a name renders as one path component*
/// — the seventh, and the only one it enforces rather than assumes. A rendering
/// that passes here is one [`std::path::Path::join`] can only place *inside* the
/// directory it is joined to.
///
/// The rule is Unix's, as the whole crate is: `/` is the one separator and a NUL
/// byte cannot appear in a filename at all. A port to a platform with a second
/// separator extends this function and nothing else, which is the point of it
/// being one function.
pub(crate) fn not_one_component(rendered: &str) -> Option<&'static str> {
    if rendered.is_empty() {
        return Some("is empty");
    }
    if rendered == "." || rendered == ".." {
        return Some("names a directory rather than something in one");
    }
    if rendered.contains('/') {
        return Some("holds a path separator, so it names more than one component");
    }
    if rendered.contains('\0') {
        return Some("holds a NUL byte, which no filename may");
    }
    None
}
````
<!-- /fragment -->

[Previous: Orientation](01-orientation.md) | [Contents](README.md) | [Next: Reference domain](03-reference-domain.md)
