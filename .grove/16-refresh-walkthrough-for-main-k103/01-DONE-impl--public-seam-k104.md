# public-seam-k104

## Goal

Refresh the walkthrough's crate manifest and library façade so the orientation
chapter reconstructs and explains the current `Cargo.toml` and `src/lib.rs`
exactly.

## Context

- Source roots: `crates/ordinal-fs-tree/Cargo.toml` and
  `crates/ordinal-fs-tree/src/lib.rs`.
- Book surfaces: `docs/ordinal-fs-tree/book/01-orientation.md` and the matching
  entries and roots in `docs/ordinal-fs-tree/book/source-index.md`.
- Preserve existing fragment identifiers where they still describe coherent
  units; introduce new unique identifiers only where the current source needs a
  new fragment boundary.

## Done when

- Both source-root inventory entries carry the current authoritative lengths.
- Recursive expansion of `source-crate-manifest` and `source-library`
  reproduces both files byte-for-byte, with every line covered once.
- The orientation prose accurately describes the current features,
  dependencies, exports, modules, and public façade.
- Targeted exact-source checks pass; the full validator reports only the
  mismatches owned by later children.

## Notes

Do not absorb source drift from later slices into this child.

## Decisions (running log)

The executable fixed-corpus contract is refreshed per source-owning child, not
deferred wholesale to final assembly. This child therefore updates the
validator and generated-corpus fixture constants for the manifest and crate
root while leaving every later root at its existing known mismatch.

Existing fragment identifiers and the manifest's source partition remain
stable. The current manifest inherits package fields from the workspace and the
current crate root adds the explicit model boundary for deletion plus the
`sought` and removal report exports; those changes are reflected in prose as
semantic contract changes rather than line-count adjustments.
