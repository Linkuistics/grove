//! The trees this crate's unit tests are written against, in one place.
//!
//! The course syllabus of `ARCHITECTURE.md`'s examples, built through
//! [`Builder`] rather than read from a directory — which is what the algebra
//! boundary buys, and why every pure test in this crate can be a unit test.
//!
//! Shared rather than restated per module for the reason the reference domain
//! itself is shared: the document's examples and the fixtures the code is
//! checked against must not drift apart. Test-only, so none of it reaches a
//! consumer.

use core::fmt;

use crate::snapshot::{Builder, Snapshot};
use crate::{EntryName, Found, Key, NameView, Ordinal, PositionedSpecies, Verdict};

use crate::reference::{Label, Parts, Status, SyllabusError, SyllabusName};

/// A lesson: a leaf, carrying its publication status.
pub(crate) fn lesson(ordinal: u32, key: u32, status: Status, label: &str) -> SyllabusName {
    SyllabusName::compose(
        Ordinal::new(ordinal),
        Key::new(key),
        Parts::lesson(status, Label::new(label).expect("a well-formed label")),
    )
}

/// A module: a node holding lessons and further modules.
pub(crate) fn module(ordinal: u32, key: u32, label: &str) -> SyllabusName {
    SyllabusName::compose(
        Ordinal::new(ordinal),
        Key::new(key),
        Parts::module(Label::new(label).expect("a well-formed label")),
    )
}

/// This domain's distinguished child: `OVERVIEW.md`.
pub(crate) fn overview() -> SyllabusName {
    SyllabusName::distinguished().expect("this domain has a distinguished child")
}

/// A tree holding nothing at all.
pub(crate) fn empty_tree() -> Snapshot<SyllabusName> {
    Builder::new().finish()
}

/// `ARCHITECTURE.md`'s own tree, built in the order the document draws it.
///
/// ```text
/// OVERVIEW.md
/// 01-published-orientation-i1.md
/// 02-linear-algebra-i2/
///   OVERVIEW.md
///   01-published-vectors-i5.md
///   02-draft-matrices-i6.md
/// 03-draft-assessment-i9.md
/// ```
pub(crate) fn documents_tree() -> Snapshot<SyllabusName> {
    let mut builder = Builder::new();
    let root = builder.root();
    builder.add(root, overview());
    builder.add(root, lesson(1, 1, Status::Published, "orientation"));
    let algebra = builder
        .add(root, module(2, 2, "linear-algebra"))
        .expect("a module is a node");
    builder.add(algebra, overview());
    builder.add(algebra, lesson(1, 5, Status::Published, "vectors"));
    builder.add(algebra, lesson(2, 6, Status::Draft, "matrices"));
    builder.add(root, lesson(3, 9, Status::Draft, "assessment"));
    builder.finish()
}

/// Every name in a snapshot, in walk order.
pub(crate) fn rendered(snapshot: &Snapshot<SyllabusName>) -> Vec<String> {
    snapshot.walk().map(|e| e.name().to_string()).collect()
}

/// A domain that reads a tree honestly and **composes** names that leave it.
///
/// The adversary the seventh obligation exists for, in the shape that can reach
/// the interpreter's rename path: `parse` renders back exactly what it was given,
/// so a real syllabus tree snapshots normally and there are entries to move —
/// while every name `compose` produces renders with a leading `../`, which
/// `Path::join` resolves *out of the tree whose containing directory is locked*.
///
/// It breaks canonicity as well, unavoidably: a spelling holding a separator is
/// not one a directory listing can offer, so no domain can both read such names
/// off a disk and render them. `tests/conformance_kit.rs`'s `Escaping` is the
/// half that keeps canonicity by claiming only its own spellings, and it can
/// therefore only ever be handed an empty tree. Between the two, both boundaries
/// where a name becomes a path have an adversary in front of them.
#[derive(Clone)]
pub(crate) struct Sneaky {
    inner: SyllabusName,
    escapes: bool,
}

impl fmt::Display for Sneaky {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.escapes {
            write!(f, "../{}", self.inner)
        } else {
            fmt::Display::fmt(&self.inner, f)
        }
    }
}

impl EntryName for Sneaky {
    type Parts = Parts;
    type Err = SyllabusError;

    fn parse(name: &str, found: Found) -> Verdict<Self, Self::Err> {
        match SyllabusName::parse(name, found) {
            Verdict::Entry(inner) => Verdict::Entry(Self {
                inner,
                escapes: false,
            }),
            Verdict::Foreign => Verdict::Foreign,
            Verdict::Malformed(e) => Verdict::Malformed(e),
            Verdict::Reserved(e) => Verdict::Reserved(e),
        }
    }

    fn compose(ordinal: Ordinal, key: Key, parts: Self::Parts) -> Self {
        Self {
            inner: SyllabusName::compose(ordinal, key, parts),
            escapes: true,
        }
    }

    fn distinguished() -> Option<Self> {
        SyllabusName::distinguished().map(|inner| Self {
            inner,
            escapes: false,
        })
    }

    fn view(&self) -> NameView<'_, Self::Parts> {
        self.inner.view()
    }

    fn positioned_species(parts: &Self::Parts) -> PositionedSpecies {
        SyllabusName::positioned_species(parts)
    }
}

/// A domain with **no distinguished child**, which is what `operations.qnt`'s
/// `no_distinguished` instance is.
///
/// `HAS_DISTINGUISHED = false` in the model; here it is
/// [`EntryName::distinguished`] answering `None`, and the consequence is the
/// same: promotion is refused outright, because the leaf's content would have
/// nowhere to go.
///
/// It disclaims `OVERVIEW.md` rather than merely declining to name it. A domain
/// that answered `None` here and still parsed some name as `Distinguished` would
/// have a distinguished child the library cannot name — the obligation
/// *`distinguished()` names the only entry of its species* read backwards — so
/// the honest domain has none at all, and `Foreign` is how a consumer says *not
/// mine*.
#[derive(Clone)]
pub(crate) struct Contentless(SyllabusName);

impl fmt::Display for Contentless {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl EntryName for Contentless {
    type Parts = Parts;
    type Err = SyllabusError;

    fn parse(name: &str, found: Found) -> Verdict<Self, Self::Err> {
        match SyllabusName::parse(name, found) {
            Verdict::Entry(inner) => match inner.view() {
                NameView::Distinguished => Verdict::Foreign,
                NameView::Positioned(_) => Verdict::Entry(Self(inner)),
            },
            Verdict::Foreign => Verdict::Foreign,
            Verdict::Malformed(e) => Verdict::Malformed(e),
            Verdict::Reserved(e) => Verdict::Reserved(e),
        }
    }

    fn compose(ordinal: Ordinal, key: Key, parts: Self::Parts) -> Self {
        Self(SyllabusName::compose(ordinal, key, parts))
    }

    fn view(&self) -> NameView<'_, Self::Parts> {
        self.0.view()
    }

    fn positioned_species(parts: &Self::Parts) -> PositionedSpecies {
        SyllabusName::positioned_species(parts)
    }
}

/// One lesson at the root, in a domain that has no distinguished child.
pub(crate) fn contentless_tree() -> Snapshot<Contentless> {
    let mut builder = Builder::new();
    let root = builder.root();
    builder.add(
        root,
        Contentless::compose(
            Ordinal::FIRST,
            Key::new(1),
            Parts::lesson(Status::Draft, Label::new("orientation").expect("a label")),
        ),
    );
    builder.finish()
}
