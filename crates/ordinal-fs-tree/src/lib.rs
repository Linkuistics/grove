//! An ordered tree of entries stored as a directory tree, where each entry's
//! position, identity and metadata live in its **filename**.
//!
//! The filesystem carries the hierarchy; the names carry everything else. There
//! is no index, no database, and no metadata file — a directory listing *is* the
//! data structure. That is the whole proposition: a tree you can read with `ls`,
//! edit with `mv`, diff in version control, and reason about without running the
//! program that owns it.
//!
//! The library owns the algebra — ordering, identity, traversal, and the
//! mutations that preserve both. It owns none of the vocabulary. What a name
//! looks like, what metadata it carries, and what any of it *means* are supplied
//! by the consumer through [`EntryName`], which is the only seam there is.
//!
//! # Where the design lives
//!
//! Not here. The specification of record is
//! `docs/ordinal-fs-tree/ARCHITECTURE.md`, and its claims are **checked rather
//! than reviewed** by two models beside it — `models/structure.als` for whether
//! the shape is coherent, `models/operations.qnt` for whether the operations
//! preserve it. Each has a runner reporting pass/fail per claim. Where a doc
//! comment in this crate names a `check`, a `witness_…`, an `inv_…` or a
//! `wit_…`, that is the claim it answers to, and a test carrying such a name in
//! a comment is discharging it.
//!
//! **The models lead.** Where a model and this code disagree, change the model
//! first, re-run its runner, and only then the code — and record the
//! disagreement in `docs/formalism-findings.md`, because the catalogue of them
//! is a deliverable in its own right.
//!
//! # Where the filesystem lives
//!
//! In [`fs`], and nowhere else. Every other module in this crate is the
//! algebra: pure, testable without a directory, and modellable without an
//! abstraction of one. That boundary is what makes a later split of this crate
//! into separately-modellable units mechanical rather than archaeological, and
//! `tests/algebra_has_no_filesystem.rs` is what holds it — inside one crate, a
//! seam the compiler does not enforce is a seam nothing measures.
//!
//! # Getting started as a consumer
//!
//! Implement [`EntryName`] for your own name type, then check it against the
//! obligations the library assumes and cannot enforce:
//!
//! ```
//! # use ordinal_fs_tree::{conformance, reference::SyllabusName, Found, Ordinal, Key};
//! # use ordinal_fs_tree::reference::{Parts, Status, Label};
//! let report = conformance::check::<SyllabusName>(
//!     &[
//!         ("OVERVIEW.md", Found::File),
//!         ("01-published-orientation-i1.md", Found::File),
//!         ("02-linear-algebra-i2", Found::Dir),
//!         ("README.md", Found::File),
//!     ],
//!     &[
//!         (Ordinal::new(1), Key::new(1),
//!          Parts::lesson(Status::Published, Label::new("orientation").unwrap())),
//!         (Ordinal::new(2), Key::new(2),
//!          Parts::module(Label::new("linear-algebra").unwrap())),
//!     ],
//! );
//! report.assert_conforming();
//! ```
//!
//! [`reference`] is that implementation, and it is the course syllabus the
//! architecture document uses for every one of its examples.

pub mod conformance;
mod error;
#[cfg(test)]
mod fixtures;
// The one line in this crate, outside `src/fs/` itself, that names the
// filesystem module — and `tests/algebra_has_no_filesystem.rs` exempts exactly
// this shape and nothing else. A *re-export* of anything under it stays a
// violation, deliberately: an algebra module could then reach the filesystem
// through a crate-root alias that a textual scan cannot see. So the guards live
// at `ordinal_fs_tree::fs::{read, write}` and are not lifted to the crate root.
pub mod fs;
mod name;
mod ops;
mod plan;
pub mod reference;
mod report;
mod snapshot;
mod sought;

pub use error::Error;
pub use name::{
    EntryName, EntryNameExt, Found, Key, NameView, Ordinal, PositionedSpecies, Species, Triple,
    Verdict,
};
pub use ops::{NewEntry, Target};
pub use plan::Refusal;
pub use report::{Created, Renamed, Report};
pub use snapshot::{Container, Entry, Snapshot, Walk};
pub use sought::Sought;
