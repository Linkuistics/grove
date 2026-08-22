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

/// The one trait. All genericity lives here: there are no callbacks, no hooks,
/// no registration and no configuration objects, and there is no `Domain` type.
///
/// A name **is** a type — one wrapping a string, owning its own parsing,
/// validation and formatting. [`fmt::Display`] is that formatting, and it is
/// the only rendering the library knows about.
///
/// # What an implementation must guarantee
///
/// Five obligations, and the library can check none of them. They are stated
/// because the structural model found that four were missing, and that a design
/// missing any one of them admits a tree the library will quietly corrupt. Each
/// is written on the method it constrains; [`crate::conformance`] checks the
/// four that Rust does not already make unrepresentable, and names the one it
/// does.
pub trait EntryName: Sized + Clone + fmt::Display {
    /// Everything the library does not understand: the label, and whatever
    /// attributes the domain carries. Entirely opaque.
    ///
    /// The bounds are the whole of what the library may do with one — copy a
    /// value it already holds, and compare two of them. It has no constructor
    /// for a `Parts`, which is why `promote` takes the promoted node's parts
    /// from its caller rather than deriving them
    /// (`witness_promote_cannot_name_its_output`).
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

    /// `Some` for a positioned name, `None` for the distinguished one.
    ///
    /// # Obligation: a name is positioned or distinguished, never neither
    ///
    /// **Rust discharges this one.** The architecture document states it of
    /// three separate `Option` accessors — `ordinal()`, `key()` and `parts()`,
    /// which "are `Some` together or `None` together" — and a name of species
    /// [`Species::Leaf`] with no ordinal is then admitted: an entry that cannot
    /// be ordered, shifted or promoted, and that no triple names
    /// (`witness_leaf_name_without_an_ordinal`). One [`Option`] over all three
    /// makes that state unrepresentable, so there is nothing left for an
    /// implementation to get wrong and nothing for
    /// [`crate::conformance`] to check.
    ///
    /// What the type does *not* forbid is a name that is positioned *and*
    /// claims [`Species::Distinguished`]. That half stays checkable, and it is
    /// checked under the obligation on
    /// [`distinguished`](EntryName::distinguished).
    fn triple(&self) -> Option<Triple<'_, Self::Parts>>;

    /// Which of the three kinds of thing this name names.
    ///
    /// It follows from the parts, so an implementation reads its own parts
    /// rather than storing a species beside them.
    fn species(&self) -> Species;
}
