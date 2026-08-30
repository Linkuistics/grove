# ledger-and-pages-k27


## Goal

Reconcile every source-index ledger table and enforce canonical source-root and
owner-page placement on top of the fragment engine.

## Context

- Builds on `fragment-engine-k26` and the Source and ownership ledger section
  of `docs/specs/ordinal-fs-tree-book.md`.
- The producer's first adversarial pass found that directive-only fixtures could
  pass without any Source roots, Ownership blocks, Fragment index, or Early
  uses table and that fragments could live in noncanonical files.

## Done when

- Missing, malformed, reordered, duplicated, extra, or contradictory ledger
  rows produce `F009` against both the table and directive authority.
- All fifteen roots appear only in `source-index.md` in fixed order.
- Every definition appears in the canonical numbered page assigned to its owner
  or produces `F010` with actual and required locations.
- Scoped and final committed fixtures carry all four mandatory tables and pass.
- Tests cover each ledger and placement failure independently and Clippy passes.

## Notes

This child does not broaden into Markdown navigation; `markdown-validation-k9`
owns page structure and local links.

## Decisions (running log)

Ledger tables are parsed as strict raw Markdown rows while ordinary fenced
content remains opaque. Source and ownership rows are checked against both the
fixed book contract and the parsed directive graph; fragment-index rows are
derived from that graph.

The early-use ledger requires the six settled rows and accepts additional rows
only when their symbol syntax, canonical page and anchor, later owner, scope
status, uniqueness, and order are valid.

`F010` compares each definition's physical file with its owner's canonical
numbered page. Page identity, navigation, and link structure remain with
`markdown-validation-k9`.
