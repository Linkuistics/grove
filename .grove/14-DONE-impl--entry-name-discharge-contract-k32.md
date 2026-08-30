# entry-name-discharge-contract-k32

## Goal

Reconcile `EntryName`'s claimed type-system discharge with the fact that trait
methods can depend on hidden mutable state across calls.

## Context

- `crates/ordinal-fs-tree/src/name.rs` describes `NameView` and
  `positioned_species` as making two obligations unrepresentable.
- `crates/ordinal-fs-tree/src/conformance.rs` reports those obligations as
  discharged rather than sampling them.
- `docs/ordinal-fs-tree/ARCHITECTURE.md` and
  `docs/ordinal-fs-tree/book/02-name-seam.md` state the same contract at
  different levels.

## Done when

- The contract explicitly settles whether deterministic behavior from explicit
  inputs is an assumed semantic law or is enforced by another mechanism.
- `name.rs`, the architecture, conformance reporting, and relevant tests agree
  on what Rust's type shape proves and what it does not.
- An adversarial implementation using interior or global mutable state is used
  as evidence where it can distinguish the alternatives.
- If accepted source bytes change after the book node closes, every affected
  book fragment is updated and whole-book source validation is rerun.

## Notes

The production source corpus is frozen while `ordinal-fs-tree-book-k10` is
active. This leaf records the source concern instead of changing the crate
inside a book-authoring slice.

## Decisions (running log)

- Treat stability across calls as an assumed semantic law of `EntryName`, not
  as a property enforced by the library or the conformance sampler. Rust makes
  each `NameView` structurally complete and keeps `self`, ordinal, and key out
  of `positioned_species`'s explicit inputs, but interior or global mutable
  state can still change either answer for identical explicit inputs.
- Replace conformance's full-discharge terminology with type-shape constraints.
  Each reported constraint will state both what Rust enforces and the
  deterministic behavior that remains assumed.
- Preserve the existing public `Discharged` and
  `DISCHARGED_BY_THE_TYPE_SYSTEM` surface as deprecated compatibility data. Its
  legacy `how` text must carry the same per-call limitation; new code uses the
  explicit constraint fields.
- Describe the conformance module as publishing the constraints beside its
  sampled report, not as including them in `Report` itself.
