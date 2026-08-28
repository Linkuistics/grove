# grove.code-walkthrough-for-ordinal-fs-tree — brief

## Goal

Produce a multi-page Markdown book that gives a Rust-proficient developer a
complete, conceptually ordered understanding of the `ordinal-fs-tree` library
and its demonstration CLI. Capture the reusable method that produces the book
as a deployable Linkuistics skill.

## Done when

- The book includes the crate manifest and every production source fragment in
  the library and demonstration CLI. Test files, test-only fixtures, and the
  Alloy and Quint model source are evidence, not reproduced source.
- The exposition establishes the purpose, vocabulary, architecture, public API,
  control and data flow, invariants, filesystem boundary, concurrency, rollback,
  errors and refusals, reference domain, CLI, and design trade-offs.
- The book is self-contained. ADRs, tests, formal models, repository docs, and
  external sources may establish a claim for the author, but the book states
  every fact a reader needs to understand that claim.
- Every reproduced code fragment has a unique `«fragment-id»`; fragment
  references state where another fragment is inserted; recursively expanding
  the root fragments reproduces every in-scope source file exactly, independent
  of the book's explanatory order.
- The prose is declarative, direct, and non-emotive. It assumes strong Rust and
  operating-system knowledge while introducing the crate's concepts in small,
  concrete steps. It uses no rhetorical questions, narrative suspense,
  metaphors, idioms, or persuasive framing.
- Internal cross-references support navigation and optional depth rather than
  carrying required context. When two pieces of information must be mentally
  integrated to understand the current code, the shorter load-bearing context
  is repeated locally. Repetition is removed when each occurrence is already
  independently intelligible and adds no current explanatory value.
- `plugins/linkuistics` ships a generic `writing-code-walkthroughs` skill. The
  skill elicits the target, source scope, audience, depth, output form, style,
  and verification requirements one question at a time before authoring.
- Mechanical checks cover fragment tangling, source coverage, Markdown
  structure, and local links. Independent reviews cover technical accuracy and
  editorial quality. The crate's existing verification remains green.

## Decomposition

The requirements agreement is intentionally followed by a fresh `planning`
session. That session will turn the two coupled deliverables, their research and
their review obligations into small ordered leaves without making this
human-in-the-loop session carry the implementation context.

## Pointers

- Existing domain language and design: `docs/ordinal-fs-tree/CONTEXT.md` and
  `docs/ordinal-fs-tree/ARCHITECTURE.md`
- Demonstration CLI contract: `docs/ordinal-fs-tree/CLI.md`
- Source scope: the production library and CLI in `crates/ordinal-fs-tree`, plus
  its manifest
- Evidence: the crate's tests and the Alloy and Quint models under
  `docs/ordinal-fs-tree/models`
- Skill destination: `plugins/linkuistics/skills/writing-code-walkthroughs`
- Agreed test seams: exact-source tangling; complete source-to-fragment
  coverage; Markdown/link validation; technical review; editorial review;
  existing crate verification; baseline and skill-enabled behavioral scenarios

## Notes

External research is about writing and communication: human-oriented concept
ordering, worked examples, cognitive load, progressive disclosure, and precise
technical prose. Repository source, tests, models, and authoritative dependency
documentation remain the evidence for technical claims.

The cross-reference rule follows the distinction in cognitive-load research:
split sources that must be mentally integrated increase extraneous load, while
unnecessary duplicate sources can do the same. The design must judge the logical
relationship at each use, not apply a blanket ban on repetition or links.
