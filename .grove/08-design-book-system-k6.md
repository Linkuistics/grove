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
- The ownership ledger declares every deferred hole in a source file split
  across slices, names the later slice that fills it, and records each slice's
  owned-source line count. The fragment grammar represents a deferred reference
  distinctly from an unresolved reference.
- Conceptual order follows reader dependencies rather than file order. The
  opening slice maps one complete operation at low resolution through CLI,
  public guard, snapshot, decision, plan, interpreter, report, and exit before
  later slices expand those layers; the CLI slice resolves the same operation
  in full.
- The prose contract is operational: self-containedness, direct declarative
  style, local-context repetition, optional cross-references, worked examples,
  and audience assumptions each have reviewable criteria.
- Interfaces for fragment validation and Markdown/link validation are specified
  narrowly enough that `book-validation-k7` can implement them without
  redesigning the book.
- The design assigns every planned book slice a non-overlapping conceptual and
  fragment-ownership scope, while allowing a source file to be explained out of
  file order.
- For every type first used before the slice owning its source, the ledger names
  the owning later slice and the minimum definition or behavior that the
  earlier chapter must restate locally.

## Notes

This is a design artifact, not the first chapter. Prefer a small explicit format
whose exactness can be tested over a flexible notation that requires human
interpretation.
