# orientation-k11

## Goal

Create the book scaffold and its first independently readable slice: the
library's purpose, core vocabulary, fragment notation, package contract, and
public surface.

## Context

- Inputs: `book-system-k6`, `book-validation-k7`, and this subtree's brief.
- Primary source emphasis: `Cargo.toml` and `src/lib.rs`; use other fragments
  only where the design ledger assigns them to this conceptual opening.

## Done when

- The multi-page book has its index/navigation scaffold and states the reader,
  scope, exclusions, and how to read/expand `«fragment-id»` references.
- Purpose, root/entry/leaf/node, ordinal/key, distinguished child, consumer, and
  operator are established through one concrete tree before deeper mechanisms
  depend on them.
- The manifest and crate-root surface are explained in conceptual order,
  including feature/dependency boundaries and the library/CLI separation.
- Every claimed source fragment is registered in the ownership ledger, tangles
  exactly in scoped validation, and appears once.
- Markdown/link checks and relevant existing crate verification remain green.

## Notes

This slice must stand on its own for a reader who has not opened the repository
docs. Later pages may deepen a concept but must not be required to decode this
opening.
