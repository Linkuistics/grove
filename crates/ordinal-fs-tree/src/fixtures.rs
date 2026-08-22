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

use crate::snapshot::{Builder, Snapshot};
use crate::{EntryName, Key, Ordinal};

use crate::reference::{Label, Parts, Status, SyllabusName};

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
