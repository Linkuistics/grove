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
