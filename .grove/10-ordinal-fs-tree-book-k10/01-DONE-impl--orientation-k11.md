# orientation-k11

## Goal

Create the book scaffold and its first independently readable slice: the
library's purpose, core vocabulary, fragment notation, package contract, and
public surface.

## Context

- Inputs: `book-system-k6`, `book-validation-k7`, and this subtree's brief.
- Primary source emphasis: `Cargo.toml` excluding the `cli` feature and
  `[[bin]]` declaration, which `syllabus-cli-k17` owns per the ledger, and
  `src/lib.rs`. This slice owns no fragments beyond that assigned manifest and
  crate-root scope.

## Done when

- The multi-page book has its index/navigation scaffold and states the reader,
  scope, exclusions, and how to read/expand `«fragment-id»` references.
- Purpose, root/entry/leaf/node, ordinal/key, distinguished child, consumer, and
  operator are established through one concrete tree before deeper mechanisms
  depend on them.
- One complete operation is followed end to end at low resolution through CLI
  invocation, public guard, snapshot, decision, plan, interpreter, report, and
  exit, naming the layers later slices expand. `syllabus-cli-k17` resolves this
  same operation in full.
- The manifest and crate-root surface are explained in conceptual order,
  including feature/dependency boundaries and the library/CLI separation,
  without claiming the CLI-specific manifest fragments.
- Every claimed source fragment is registered in the ownership ledger, tangles
  exactly in scoped validation, and appears once.
- This is the first leaf to run both validators against real book content;
  Markdown/link checks and relevant existing crate verification remain green.
  Any validator defect it surfaces is externalised as a leaf rather than fixed
  inline.

## Notes

This slice must stand on its own for a reader who has not opened the repository
docs. Later pages may deepen a concept but must not be required to decode this
opening.
