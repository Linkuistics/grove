# book-system-k6

## Goal

Design the multi-page book, literate fragment graph, source-ownership ledger,
and deterministic validation contracts before tooling or prose depends on them.

## Context

- Requirements: `plan-k1` and the root brief.
- Research input: `walkthrough-method-k5`.
- Technical evidence: `docs/ordinal-fs-tree/{CONTEXT,ARCHITECTURE,CLI}.md`, the
  production crate source, its tests, and the Alloy and Quint models.
- The exact in-scope file inventory is carried by
  `ordinal-fs-tree-book-k10`'s brief.

## Done when

- A committed design artifact fixes the book directory, page order, navigation,
  chapter responsibilities, and the source-to-fragment ownership ledger.
- The fragment grammar defines unique `«fragment-id»` declarations, insertion
  references, source roots, whitespace/newline preservation, recursive
  expansion, and deterministic diagnostics for duplicates, unresolved
  references, cycles, unreachable fragments, missing source, and duplicated
  source.
- The design states how an authoring leaf can prove scoped progress while the
  final assembly proves exhaustive coverage of all fifteen in-scope files.
- Conceptual order follows reader dependencies rather than file order and maps a
  complete operation through CLI, public guard, algebra, plan, interpreter,
  report, and error/refusal surfaces before expanding each layer.
- The prose contract is operational: self-containedness, direct declarative
  style, local-context repetition, optional cross-references, worked examples,
  and audience assumptions each have reviewable criteria.
- Interfaces for fragment validation and Markdown/link validation are specified
  narrowly enough that `book-validation-k7` can implement them without
  redesigning the book.
- The design assigns every planned book slice a non-overlapping conceptual and
  fragment-ownership scope, while allowing a source file to be explained out of
  file order.

## Notes

This is a design artifact, not the first chapter. Prefer a small explicit format
whose exactness can be tested over a flexible notation that requires human
interpretation.
