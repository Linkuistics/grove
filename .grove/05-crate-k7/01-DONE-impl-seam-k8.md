# seam-k8

## Goal

Stand the crate up and build the seam everything else is written against: the
name types, the `EntryName` trait, one reference domain implementing it, a
conformance kit that lets any domain check its own implementation against the
five obligations, and the test that keeps the algebra out of `std::fs`.

Nothing here touches a filesystem. That is the point of doing it first — the
whole of this leaf is checkable without a directory.

## Context

Beyond the brief chain:

- `ARCHITECTURE.md` — *The model*, *Names belong to the consumer*, *The seam:
  one trait*, and above all *What an implementation must guarantee*, which is
  the five obligations this leaf makes checkable.
- `structure.als`, and `run-alloy.sh`. The trait's shape is what that model
  checked; its `witness_…` commands each reproduce a defect the current shape
  closes, and they are the cheapest possible explanation of why a law is there.
- `docs/formalism-findings.md` entry 002 — its counterfactual on `Option`
  completeness is a direct instruction to this leaf: **before modelling a
  structural property, ask whether the target language already forbids it.**
  A name modelled as `enum Name { Positioned { ordinal, key, parts },
  Distinguished }` makes the bad state unrepresentable and one obligation
  disappears. Take that offer where Rust makes it, and record which obligations
  became free — that is the second data point for a routing-table row that
  currently rests on one.
- `src/tree_format.rs` and `src/tree_id.rs` for prior art on grove's own name
  grammar. Prior art only: grove's vocabulary must not enter the crate.

## Done when

- `crates/ordinal-fs-tree/` exists and builds; the root `Cargo.toml` carries a
  `[workspace]` table with `grove` still the root package; `cargo test` and
  `cargo clippy --all-targets` are green for **both** crates. Confirm grove's
  own build and tests survived becoming a workspace member — that is the one
  way this leaf can break something outside itself.
- `Ordinal`, `Key`, `Species`, `Found`, `Verdict` and the `EntryName` trait
  exist as `ARCHITECTURE.md` states them, with each of the five obligations
  written at the point a reader meets it.
- The reference domain — the course-syllabus domain of the document's examples,
  modules and lessons carrying a draft/published attribute — implements the
  trait, and the document's own example names round-trip through it.
- A conformance kit: a consumer hands it sample names and sample triples and
  learns which obligations its implementation violates. It covers every
  obligation Rust's type system did not already make unrepresentable, and it
  says which those were.
- A test asserts the algebra cannot reach `std::fs`. It fails when someone adds
  the import — check that by adding one, watching it fail, and removing it.
- Both model runners have been run once on this machine, and their output read.
- An entry in `docs/formalism-findings.md`.

## Notes

**The one control that matters here.** A test that checks the algebra is
`std::fs`-free is exactly the shape of instrument that reads clean when it is
broken. Whatever the mechanism — a source scan, a module-graph assertion, a
build-time check — prove it fails on a deliberate violation before trusting a
pass. Entry 003's closing lesson is that three separate instruments in this
workstream reported *found nothing* and *succeeded* with the same bytes.

**The conformance kit is a deliverable, not a nicety.** The five obligations are
the consumer's and the library cannot check them; a design missing any one of
them admits a tree the library will quietly corrupt. grove's own domain
implementation, in increment 2, is the second consumer, and it should not be the
thing that discovers an obligation by corrupting a live task tree.

**This is the most load-bearing artifact in the subtree** — every later leaf is
written against this trait. If an adversarial read is warranted, cut
`leaf-add crate-k7 seam --kind review-impl` as this session's last act, writing
the specific doubt into its body.

## Decisions (running log)

**The trait folds three `Option` accessors into one.** `ordinal()` / `key()` /
`parts()` became `fn triple(&self) -> Option<Triple<'_, Self::Parts>>`, and the
obligation *a name is positioned or distinguished, never neither* stopped being
checkable because it stopped being representable. This is entry 002's own
counterfactual — *before modelling a structural property, ask whether the target
language already forbids it* — applied at the seam rather than only inside a
domain, which is the stronger place: it holds for every consumer, not just for
one that happens to use an enum. It is a deviation from `ARCHITECTURE.md` as
written, so the document, `structure.als` and the conformance kit were reconciled
to it in the same commit. It is also hard to reverse — the trait is the public
surface, and `docs/adr/entry-name-is-the-only-seam.md` says so — which is what
earned the review leaf.

**The kit reports untested obligations, not just violated ones.** A kit handed no
samples otherwise reports *conforming* while checking nothing, which is the exact
shape of entry 003's three dead instruments. `Report::is_conforming` is false
while any obligation was never exercised, and its own test suite includes the
empty-samples case as a control. The same reasoning put a positive control
(`the_detector_fires_on_a_deliberate_violation`) beside the no-filesystem test,
and the deliberate violation was added to `src/name.rs`, watched failing, and
removed.

**The no-filesystem rule is stated by directory, not by module list.**
Everything under `src/` outside `src/fs/` is the algebra. A later leaf that adds
a pure module inherits the guarantee without having to ask for it, where an
allowlist would have to be remembered. `src/fs/` does not exist yet; the next
leaf creates it.

**The reference domain is public, in `src/reference.rs`.** `crate-k7`'s brief
requires one domain shared by every test *and* by the CLI, and a
`#[cfg(test)]`-only module cannot be shared with a binary target. The alternative
— a cargo feature — compiles the crate twice for no benefit while the CLI lives
in the same workspace. Documented as an example rather than a base class.

**`Ordinal` and `Key` are `u32` newtypes.** Matching grove's prior art, which
increment 2 has to fit through this seam, and the ADR's own "the keys are
integers". `Ordinal::FIRST` is the one named value, and its doc says what it is
not: a floor, since density is preserved and never established.

**The workspace lint tables are the root's; the new crate is excluded from
`cargo release`.** Hoisting `[lints]` to `[workspace.lints]` keeps grove's clippy
baseline byte-identical and makes a new member inherit it by writing one line,
rather than by remembering to restate it. `[package.metadata.release] release =
false` keeps a grove release cut exactly what it was — whether this crate wants
its own version and release lane is `crate-k7`'s horizon question, and answering
it by accident at the next release is what that line prevents.

**The review leaf was inserted, not appended.** `leaf-add` would have placed it
after all eight dependent leaves. A `review-*` step normally does not care where
it lands, because it re-derives its citations from the producer's commit; this
one does, because the artifact under review is the thing the other eight are
written against, and a finding delivered last is a rework rather than a
correction.
