# Orientation
<!-- book-page id="orientation" slice="orientation-k11" order="1" -->
[Contents](README.md) | [Next: Name seam](02-name-seam.md)

<a id="purpose-and-boundary"></a>
## Purpose and boundary

`ordinal-fs-tree` stores an ordered tree directly as directories and regular
files. The filesystem carries hierarchy. Each positioned filename carries the
entry's sibling position, stable identity, and consumer-defined information.
There is no index, database, or metadata file, so ordinary filesystem and
version-control tools can inspect the structure without the consumer program.

The library owns ordering, identity, traversal, mutation planning, and
filesystem interpretation. A **consumer** links the library and supplies the
filename vocabulary through `EntryName`. An **operator** drives a program that
consumer built. The `syllabus` binary is one demonstration consumer; lesson,
module, status, and label are its words rather than built-in library concepts.

<a id="working-vocabulary"></a>
## Working vocabulary

The following tree is the starting point for this page's insert. The handed-in
directory `s` is the **root**: a node that is not an entry, so it has no ordinal,
key, or consumer parts. Every named child is an **entry**. A regular-file entry
is a **leaf**; a directory entry is a **node** and may contain children. A root
or node considered as a child-holding container is a **level**.

```text
s/
├── OVERVIEW.md
├── 01-published-orientation-i1.md
└── 02-linear-algebra-i2/
    ├── OVERVIEW.md
    ├── 01-published-foundations-i3.md
    ├── 02-published-vectors-i5.md
    └── 03-draft-matrices-i6.md
```

`OVERVIEW.md` is a **distinguished child**: optional content belonging to its
level rather than an ordered child. It is a regular file but its
`Species::Distinguished` carries no ordinal or key and traversal does not
descend into it. `Found` records the observed filesystem kind without following
it; the consumer's `Verdict` then separates an accepted entry from a foreign
name to skip and a malformed or reserved name that must stop the read.

An **ordinal** is the mutable position among siblings in one level. It is the
leading number in the syllabus names and changes when insertion shifts an entry.
A **key** is tree-wide identity, written after `i`; it remains unchanged by
shifts, moves, and rewrites. `EntryName` is the only consumer seam: it parses an
observed name, exposes ordinal, key, opaque parts, and species, and composes a
name from those values. Species determines leaf, node, or distinguished shape;
the library never interprets the consumer's parts.

<a id="package-contract"></a>
## Package contract

The package identifies the reusable crate, fixes the workspace-compatible Rust
version, and keeps its library dependency boundary explicit. `libc` is the only
unconditional runtime dependency because the filesystem layer needs platform
`flock` constants that `std` does not supply. `clap` is declared optional
for the demonstration binary; its feature activation is a CLI-owned range
deferred in the source index. This fragment establishes the package and
dependency inputs used by the library and by the insert tour.
<!-- fragment «manifest-package-and-library-dependency» owner="orientation-k11" source="crates/ordinal-fs-tree/Cargo.toml" lines="1-42" parent="source-crate-manifest" -->
````toml
[package]
name = "ordinal-fs-tree"
version = "0.1.0"
edition = "2021"
description = "An ordered tree of entries stored as a directory tree, where each entry's position, identity and metadata live in its filename"
license = "Apache-2.0"
repository = "https://github.com/Linkuistics/grove"
# Inherited from the workspace root, whose comment carries the evidence: the
# locked dependency graph cannot be parsed by a cargo below 1.85. This crate has
# no dependencies of its own and would build far lower, but a member that
# promises more than the workspace can deliver promises nothing.
rust-version = "1.85"

# One dependency, and the bar it had to clear is that `std` cannot do the job at
# all. The library is published for reuse, so every dependency it takes it
# imposes on grove and on every later consumer — errors, for instance, are the
# consumer's own type (`EntryName::Err`), so no error-handling crate is pulled in
# for them.
#
# `libc` is here for `flock`, which has no `std` equivalent. The alternative was
# a hand-written `extern "C"` declaration beside hard-coded `LOCK_SH`/`LOCK_EX`
# constants: that compiles everywhere and is silently wrong on any platform whose
# values differ, because a lock taken in the wrong mode still reports success.
# The constants belong to the platform, and this is how a Rust program asks the
# platform for them. It costs grove nothing — the workspace already locks it.
[dependencies]
libc = "0.2"

# `clap` is the CLI's and nobody else's, so it is optional and behind the `cli`
# feature below. An external consumer taking `default-features = false` gets the
# bare library, whose imposed dependency set stays exactly `libc`.
clap = { version = "4", features = ["derive"], optional = true }

# The feature is **on by default**, and that is the load-bearing half. With
# `default = []` the binary would not be built by a plain `cargo test`, so
# `tests/driving_a_tree.rs` would be silently absent from the one command this
# crate's *Done when* actually names — the failure mode
# `docs/formalism-findings.md` entry 003 has already cost this workstream three
# times over: a suite that did not run reports what a suite that found nothing
# reports. Inside this workspace the feature unifies on, which costs grove
# nothing; the escape hatch is `default-features = false`, and increment 2's
# grove dependency line is where it gets exercised.
````
<!-- /fragment -->

The demonstration source sits outside `src/`, making it a package-sharing
consumer rather than a module inside the algebra. The exact feature declaration
and binary target remain deferred to the CLI page; this fragment records why the
boundary is structural and why the binary is named for the syllabus it drives.
<!-- fragment «manifest-library-cli-boundary» owner="orientation-k11" source="crates/ordinal-fs-tree/Cargo.toml" lines="46-61" parent="source-crate-manifest" -->
````toml

# The source is **outside `src/`**, and that was measured rather than assumed.
# `tests/algebra_has_no_filesystem.rs` lexes every `.rs` under `src/` outside
# `src/fs/` and refuses the identifier `fs`; a CLI must call
# `ordinal_fs_tree::fs::read`, which is that identifier. A probe planted at
# `src/bin/` failed `the_algebra_cannot_reach_the_filesystem` naming its own
# line, and the same file moved out of `src/` passed. The alternative is a
# second exemption in the guard, and the guard's own header says why that is the
# wrong direction: a new module is inside the algebra by default, which is the
# direction that fails safe. It also says structurally what `CLI.md` says in
# prose — the CLI is not a module of this library, it is a consumer that shares
# a package with it.
#
# The binary is named for the tree it drives and not for the library: a binary
# called `ordinal-fs-tree` would advertise a generic tool, which is the one thing
# `CLI.md` spends its first section rejecting.
````
<!-- /fragment -->

Development dependencies support filesystem isolation, binary contract tests,
and the lexer-based algebra boundary check without becoming consumer
dependencies. Workspace lints apply, while release metadata excludes this crate
from Grove's release lane until separate publication is deliberately settled.
This fragment completes the orientation-owned manifest ranges without including
the CLI feature or binary declaration.
<!-- fragment «manifest-development-and-release» owner="orientation-k11" source="crates/ordinal-fs-tree/Cargo.toml" lines="66-116" parent="source-crate-manifest" -->
````toml

# `tempfile` is a *dev*-dependency, so no consumer inherits it. The filesystem
# tests need a directory that is unique per run and removed afterwards, and
# hand-rolling that is a worse version of a crate this workspace already uses.
[dev-dependencies]
tempfile = "3.10"

# `assert_cmd` drives the `syllabus` binary in `tests/driving_a_tree.rs`. Those
# are contract tests over the binary rather than unit tests behind it, because
# the point of the CLI leaf is exercising the library from *outside* — through a
# real `Display`, a real error text and a real domain implementation. It is a
# dev-dependency of a member the workspace already locks it for.
assert_cmd = "2.0"

# `proc-macro2` is the Rust lexer, and `tests/algebra_has_no_filesystem.rs` is
# where a hand-rolled one was silently wrong. That test used to strip comments
# textually; `reading-k19` found that a `"/*"` inside a string literal left it at
# positive comment depth, so every filesystem use after it disappeared while the
# guard reported clean. The alternative to this dependency is a partial Rust
# lexer maintained here — raw strings and their hash counts, byte and character
# literals, and the `'a` lifetime that looks exactly like an unterminated
# character literal — which is more surface than the guard itself and fails in
# the same silent direction. Asking the real lexer instead makes the question
# trivial: a use of the filesystem is an `Ident`, and a comment or a string
# literal never is one.
#
# It is a *dev*-dependency, so no consumer inherits it, and the workspace already
# locks this version for its own build.
proc-macro2 = { version = "1.0", features = ["span-locations"] }

[lints]
workspace = true

# `cargo release` drives the workspace, and `release.toml` configures a cut of
# *grove*. Excluding this member keeps that cut byte-identical to what it was
# before the workspace existed: no version bump here, no tag, no changelog
# section.
#
# **Whether this crate wants its own `CHANGELOG`, version and release lane is
# still open**, and it is open because it depends on an answer nothing has yet
# had to give: is `ordinal-fs-tree` ever published on its own, or is it only
# ever consumed in-tree by grove? Published separately, it owes consumers a
# version they can pin and a changelog they can read; consumed in-tree only, a
# second release lane is ceremony over a crate that ships inside grove's tag
# anyway. Answering it **by accident**, at grove's next release, is the outcome
# this line prevents — `publish = false` is repo-wide in `release.toml`, so
# without this exclusion the member would simply be swept along by a `cargo
# release` cut of grove and the question would be settled by a version bump
# nobody decided on.
[package.metadata.release]
release = false
````
<!-- /fragment -->

<a id="public-surface"></a>
## Public surface

The crate root states the storage proposition and keeps filesystem access in
`fs`. Its public surface exposes the conformance kit and reference domain,
`fs::read` and `fs::write`, the `EntryName` seam and name vocabulary,
mutation inputs and refusals, reports, and immutable snapshot views. The private
`ops`, `plan`, and internal support modules keep algebraic implementation
details behind those values. This fragment is the complete crate root and is the
source-level map used by every later page.
<!-- fragment «library-crate-surface» owner="orientation-k11" source="crates/ordinal-fs-tree/src/lib.rs" lines="1-94" parent="source-library" -->
````rust
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

pub use error::Error;
pub use name::{
    EntryName, EntryNameExt, Found, Key, NameView, Ordinal, PositionedSpecies, Species, Triple,
    Verdict,
};
pub use ops::{NewEntry, Target};
pub use plan::Refusal;
pub use report::{Created, Renamed, Report};
pub use snapshot::{Container, Entry, Snapshot, Walk};
````
<!-- /fragment -->

<a id="insert-tour"></a>
## Insert tour

The operator inserts a draft lesson named `limits` at ordinal 2 under the
module whose stable key is 2:

```console
syllabus --root s lesson-insert 2 2 limits
```

The parsed `Cli` contains a `Verb::LessonInsert`. `run` dispatches it to the
CLI `insert` helper with `Target::Key(Key::new(2))`,
`Ordinal::new(2)`, and `Parts::lesson(Status::Draft, label)`, where the label's
validated string is `limits`.
`Label`, `Status`, `Parts`, and `SyllabusName` belong to this consumer's
reference domain; they are not library defaults. `Streams` keeps result data
on stdout and advisories on stderr, while `Failure` pairs operator-facing text
with an exit category.

The helper calls `fs::write::<SyllabusName>(&cli.root)`. It acquires an
exclusive advisory lock and reads the whole tree into a `Snapshot`; the
returned `WriteGuard` owns that lock, the caller-spelled root, and the
snapshot. A `Snapshot` is the immutable parsed tree captured under a guard,
and its `Entry` values are borrowed views. The related `ReadGuard` couples a
shared lock, caller-spelled root, and snapshot. A write mutation consumes its
guard so a second mutation cannot reuse the pre-mutation snapshot.

The CLI helper constructs `NewEntry::empty(parts)` and passes it to
`WriteGuard::insert`. The guard method passes its captured snapshot, the target,
the ordinal, and the new entry to `ops::insert`. `Target`
names either the root or a stable key. `NewEntry` carries opaque parts and
optional bytes. The pure operation returns a total `Decision`: either a
`Refusal`, which changes nothing, or a `Plan` of ordered `Effect` values.
For this tree the target resolves to the module level, ordinal 2 is occupied,
and the greatest key is 6.

The operation builds this guarded plan:

```text
1. MoveTo key 6:
   s/02-linear-algebra-i2/03-draft-matrices-i6.md
   → s/02-linear-algebra-i2/04-draft-matrices-i6.md
2. MoveTo key 5:
   s/02-linear-algebra-i2/02-published-vectors-i5.md
   → s/02-linear-algebra-i2/03-published-vectors-i5.md
3. Create key 7:
   s/02-linear-algebra-i2/02-draft-limits-i7.md
```

The displaced siblings move highest ordinal first. Each move recomposes only the
ordinal and preserves the key and parts. This ordering vacates every destination
before it is needed and leaves distinct ordinals after every landed effect.
`Plan::guarded` folds the effects through the snapshot in the same order the
interpreter will meet them, rejecting a conflicting destination before any
filesystem action begins.

`WriteGuard::run` turns a refusal into `Error::Refused`; otherwise it calls
`fs::apply::apply`. The interpreter validates that every planned name renders
as one path component, creates an `apply::Run`, and calls `Run::step` for
each effect. `Run` records landed paths, moves, undo actions, and the
`Report`. `apply::Faults` is an internal test seam and is disabled on this
public path. If a step fails, `Run::unwind` performs registered undo actions in
reverse. A successful unwind returns a clean failure stating that the tree is as
found; an unwind failure returns a partial-rollback error and stops. Boundary
errors, refusals, clean rollback, and partial rollback remain distinct.

All three effects succeed here. The report records the two renames in
highest-first order, the create, and all landed paths in plan order. The CLI
prints only the created subject to stdout:

```text
7	s/02-linear-algebra-i2/02-draft-limits-i7.md
```

Unless `--quiet` is set, stderr carries the advisory landing trace:

```text
lesson-insert: 3 effects, in the order they landed:
  renamed  s/02-linear-algebra-i2/03-draft-matrices-i6.md -> s/02-linear-algebra-i2/04-draft-matrices-i6.md
  renamed  s/02-linear-algebra-i2/02-published-vectors-i5.md -> s/02-linear-algebra-i2/03-published-vectors-i5.md
  created  s/02-linear-algebra-i2/02-draft-limits-i7.md
```

The process exits with status 0. The resulting level is:

```text
02-linear-algebra-i2/
├── OVERVIEW.md
├── 01-published-foundations-i3.md
├── 02-draft-limits-i7.md
├── 03-published-vectors-i5.md
└── 04-draft-matrices-i6.md
```

Later pages expand the name seam, reference domain, snapshot read, mutation
algebra, filesystem interpreter, and CLI source. The CLI page returns to this
same command and starting tree at full source resolution.

[Contents](README.md) | [Next: Name seam](02-name-seam.md)
