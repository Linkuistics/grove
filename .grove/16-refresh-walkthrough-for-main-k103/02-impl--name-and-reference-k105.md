# name-and-reference-k105


## Goal

Refresh the name, sought-object resolution, and reference-domain book slices
against the current sources.


## Context

- Source roots: `crates/ordinal-fs-tree/src/name.rs`, `src/sought.rs`, and
  `src/reference.rs`. `src/sought.rs` is a new production root absent from the
  pre-rebase fixed inventory and must be added to the book system.
- Book surfaces: `02-name-seam.md`, `03-reference-domain.md`, and
  `source-index.md`.

## Done when

- All three source roots tangle byte-for-byte and their inventory entries are
  current.
- Exposition accurately states the current entry-name and reference-domain
  contracts, sought-object resolution, errors, parsing, and type-shape
  guarantees.
- The full validator advances without introducing failures outside the known
  later slices.

## Notes

Retain the post-review clarification that `EntryName` implementations remain
responsible for returning one valid platform path component.
