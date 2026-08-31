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

## Decisions (running log)

The executable fixed-corpus contract advances with each source-owning child.
This child therefore adds `src/sought.rs` to the validator, generated corpus,
normative inventory, and source ledger, producing an intermediate sixteen-root
corpus before the filesystem-lifecycle child adds `src/fs/remove.rs`.

`src/sought.rs` is one full-file literal fragment owned by `name-seam-k12`.
Its search-result vocabulary is small and coherent as one unit, and explaining
it beside the name vocabulary keeps the new public answer type within this
child's named book surface without pre-empting the later snapshot refresh.

The `name.rs` and `reference.rs` partitions retain their existing fragment IDs.
Only their ranges move: the name delta expands the identifier fragment, and the
reference delta expands the vocabulary fragment. Prose now distinguishes the
unsupported removal of one entry from supported whole-tree deletion, and the
reference-domain source records that both its grammar and grove's current task
grammar enforce canonical positions.

Adding a source root advances the normative fixed-corpus contract atomically:
the root and block tables, aggregate counts, validator fixtures, and early-use
ledger all describe the same sixteen-root, 7,118-line corpus. `Sought` therefore
also gains a local orientation statement at its first use, distinguishing a
completed search with no match from both mutation refusal and error while
leaving accessor absence represented by `Option`.
