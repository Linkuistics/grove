# ordinal-fs-tree-book-k10 — brief

## Goal

Produce the complete, self-contained `ordinal-fs-tree` code-walkthrough book,
with every in-scope source file reconstructable exactly from conceptually
ordered fragments.

## Context

The intended reader is proficient in Rust, common crates, and operating-system
APIs. Introduce the crate's concepts in small concrete steps without teaching
general Rust or filesystem basics.

## Done when

- The book establishes purpose, vocabulary, architecture, public API, control
  and data flow, invariants, filesystem boundary, concurrency, rollback, errors
  and refusals, reference domain, CLI behavior, and design trade-offs.
- It is self-contained: repository docs, ADRs, tests, formal models, and external
  research support the author but are never required reading for the reader.
- Every fragment ID is unique and every insertion point is explicit; recursively
  expanding source roots reproduces all fifteen files below byte for byte with
  no missing, duplicated, unresolved, cyclic, or unreachable fragment.
- The prose is declarative, direct, and non-emotive, with no rhetorical
  questions, narrative suspense, metaphors, idioms, or persuasive framing.
- Required local context is repeated when integration is necessary; internal
  cross-references provide navigation or optional depth and never carry required
  understanding alone.
- Exhaustive fragment/source coverage, Markdown structure, local links, and the
  existing crate verification pass at assembly and after any integrated review
  findings.
- Fresh-context technical review against source, tests, models, and repository
  contracts completes before fresh-context editorial review against the prose
  and self-containedness contracts.

## Decomposition

- `orientation-k11` establishes the purpose, reader contract, notation,
  package, and public surface, then follows one complete operation end to end at
  low resolution.
- `name-seam-k12` explains filename algebra, identity/order, parsing, and the
  consumer seam.
- `reference-domain-k13` makes the abstraction concrete through the syllabus
  domain and its conformance obligations.
- `read-path-k14` follows filesystem discovery into snapshots, traversal, and
  public read behavior.
- `mutation-algebra-k15` explains total decisions, operations, plans, effects,
  reports, and refusals without the filesystem.
- `filesystem-interpreter-k16` explains locking, application order,
  intermediate states, rollback, concurrency, and error taxonomy.
- `syllabus-cli-k17` explains the demonstration consumer, verbs, streams, exit
  codes, and resolves the opening slice's operation across all layers.
- `book-assembly-k18` supplies cross-cutting invariants and trade-offs, closes
  every ledger row, runs whole-book verification, and commissions review.

Every slice leaves a readable book increment and passes the scoped validators
for the fragment ownership it claims. `book-system-k6` places the
complete-operation tour in `orientation-k11` before the layer sequence and may
refine which source fragments each conceptual slice owns; it may not weaken the
subsequent layer order or the whole-corpus criterion.

## Pointers

- Vocabulary and design: `docs/ordinal-fs-tree/CONTEXT.md` and
  `docs/ordinal-fs-tree/ARCHITECTURE.md`.
- CLI contract: `docs/ordinal-fs-tree/CLI.md`.
- Evidence: crate tests, `docs/ordinal-fs-tree/models/{structure.als,operations.qnt}`,
  their runners, relevant ADRs, and `docs/formalism-findings.md`.
- Book method: `walkthrough-method-k5`; format and ownership:
  `book-system-k6`; validators: `book-validation-k7`.

## Source corpus

The fragment graph reconstructs these files in full, including any `#[cfg(test)]`
items embedded in them:

- `crates/ordinal-fs-tree/Cargo.toml`
- `crates/ordinal-fs-tree/bin/syllabus.rs`
- `crates/ordinal-fs-tree/src/lib.rs`
- `crates/ordinal-fs-tree/src/conformance.rs`
- `crates/ordinal-fs-tree/src/error.rs`
- `crates/ordinal-fs-tree/src/name.rs`
- `crates/ordinal-fs-tree/src/ops.rs`
- `crates/ordinal-fs-tree/src/plan.rs`
- `crates/ordinal-fs-tree/src/reference.rs`
- `crates/ordinal-fs-tree/src/report.rs`
- `crates/ordinal-fs-tree/src/snapshot.rs`
- `crates/ordinal-fs-tree/src/fs/mod.rs`
- `crates/ordinal-fs-tree/src/fs/read.rs`
- `crates/ordinal-fs-tree/src/fs/apply.rs`
- `crates/ordinal-fs-tree/src/fs/lock.rs`

Standalone test modules and fixtures (`src/fixtures.rs`, `src/**/tests.rs`, and
`tests/**`) and the Alloy and Quint model source are evidence only and are not
reproduced.

## Notes

The fifteen in-scope source files are frozen for this node's duration. A defect
found in crate source is externalised as a leaf rather than fixed inside a book
slice. If an accepted source change lands, every affected slice is updated and
exhaustive fragment validation is rerun before this node closes.

`reference-domain-k13`, `mutation-algebra-k15`,
`filesystem-interpreter-k16`, and `syllabus-cli-k17` are expected
`leaf-decompose` candidates. If one exceeds a focused session, decomposition at
a stated conceptual seam is the intended response rather than extending the
session.

`book-assembly-k18` creates the technical `review-impl` leaf as its final act.
The technical reviewer inspects accuracy and completeness without editing or
running checks. If it has findings, it creates an adjacent
`integrate-review-impl`; that integrator verifies and fixes the findings, reruns
all book and crate checks, then creates the editorial reviewer as its final act.
If technical review has no findings, the reviewer creates the editorial review
directly. Editorial review inspects clarity, ordering, cognitive load,
self-containedness, repetition/link choices, and the prose contract. It creates
an integration leaf only for real findings; editorial integration reruns the
whole-book and crate checks. Every review or integration body names the exact
producer/review handle through `**Reviews:**` or `**Integrates:**`.

Both reviews use kind `review-impl`, both integrations use kind
`integrate-review-impl`, and all four use the bare stem `book-assembly`; each
step is therefore referenced by its full `<slug>-k<key>` handle, never by the
stem alone. The editorial leaf's `**Reviews:**` names the session that last
wrote the book: `book-assembly-k18` when technical review had no findings, or
the technical integration handle when it did. Its body explicitly scopes the
read to the whole book rather than only that handle's diff.
