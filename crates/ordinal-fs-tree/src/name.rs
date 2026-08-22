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

/// The one trait. All genericity lives here: there are no callbacks, no hooks,
/// no registration and no configuration objects, and there is no `Domain` type.
///
/// A name **is** a type — one wrapping a string, owning its own parsing,
/// validation and formatting. [`fmt::Display`] is that formatting, and it is
/// the only rendering the library knows about.
///
/// # What an implementation must guarantee
///
/// Six obligations, and the library can check none of them at run time. They
/// are stated because the structural model found that four were missing, and
/// that a design missing any one of them admits a tree the library will quietly
/// corrupt. Each is written on the method it constrains; [`crate::conformance`]
/// checks the four that Rust does not already make unrepresentable, and names
/// the two it does — [`view`](EntryName::view) and
/// [`positioned_species`](EntryName::positioned_species) carry those two.
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

    /// What this name is: a positioned entry with its triple, or the
    /// distinguished child.
    ///
    /// One method rather than the `ordinal()` / `key()` / `parts()` /
    /// `species()` accessors an earlier draft had, because the choice is one
    /// choice. Read a triple off it with [`EntryNameExt::triple`] and a species
    /// with [`EntryNameExt::species`]; both are derived from this and from
    /// [`positioned_species`](EntryName::positioned_species), so neither is an
    /// implementation's to get wrong.
    ///
    /// # Obligation: a name is positioned or distinguished, never neither
    ///
    /// **Rust discharges this one, and both halves of it.** The architecture
    /// document states it of three separate `Option` accessors — `ordinal()`,
    /// `key()` and `parts()`, which "are `Some` together or `None` together" —
    /// and a name of species [`Species::Leaf`] with no ordinal is then
    /// admitted: an entry that cannot be ordered, shifted or promoted, and that
    /// no triple names (`witness_leaf_name_without_an_ordinal`). A `None`
    /// triple beside a `Distinguished` species was the same defect inverted.
    /// [`NameView`] carries the triple *and* the distinguished-or-not choice in
    /// one value, so neither state can be written and there is nothing here for
    /// [`crate::conformance`] to check.
    fn view(&self) -> NameView<'_, Self::Parts>;

    /// The species of a positioned name carrying these parts.
    ///
    /// # Obligation: the species follows from the parts
    ///
    /// **Rust discharges this one too**, by the signature: this is an
    /// associated function of the *name type* over a `&Parts`, so there is no
    /// `self`, no ordinal and no key to consult. A domain whose leaves and
    /// nodes differ expresses that as variants of [`Parts`](EntryName::Parts).
    ///
    /// `structure.als` assumes it as `SpeciesFromParts`, and the derivation
    /// that rests on it is the sibling shift: shifting is
    /// `compose(new_ordinal, key, parts)`, so a species that could vary with
    /// the ordinal would make a shift able to turn a leaf into a node — a
    /// rename of a file into a directory, with the subtree that implies.
    ///
    /// A discharge claim is a proof nobody wrote down, so here is the control.
    /// The domain below wants its species to depend on where the entry sits,
    /// and does not compile — there is no `self` to ask:
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
/// makes *a name is positioned or distinguished, never neither* and *the
/// species follows from the parts* unrepresentable rather than checkable — an
/// overridable `species()` would put both back within reach.
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
}

impl<N: EntryName> EntryNameExt for N {}
