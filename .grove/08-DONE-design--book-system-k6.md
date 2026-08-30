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

## Decisions (running log)

The opening complete operation is `insert`. It crosses the demonstration CLI,
exclusive write guard and snapshot, total algebraic decision, guarded plan,
shared effect interpreter, rollback boundary, report, and exit mapping while
making the ordinal/key distinction and highest-first sibling shift observable.
Promotion's transient duplicate ordinal and key remain for the detailed
mutation and interpreter chapters rather than complicating the first
low-resolution path.

The book uses a separate `README.md`, eight numbered concept pages, and two
lookup indexes. The numbered pages are the sole canonical reading path;
concept and source indexes provide optional lookup rather than another layer of
explanation.

The fragment graph is self-describing in raw Markdown. Source roots and the
ownership ledger live in `source-index.md`; literal fragment definitions live
beside their explanations. A separate manifest was rejected because it would
duplicate graph relationships and make the Markdown insufficient on its own.

Fragments form one single-parent tree per source file. Definitions are either
raw four-backtick literal fences or ordered compositions of whole-line
`insert` references, never both. There are no transforms, aliases, reuse,
continuations, implicit concatenation, or cross-source references.

The source remains authoritative. Validation expands fragments in memory and
compares the result byte for byte; it never writes or regenerates production
source.

Orientation describes the complete insert path with exact identifiers and
concrete values but does not copy later-owned source. This preserves its
settled ownership of only the non-CLI manifest ranges and `src/lib.rs` while
the CLI chapter can resolve the same operation with source at full resolution.

Only two production files cross ownership boundaries. `Cargo.toml` defers the
CLI feature and binary declaration from `orientation-k11` to
`syllabus-cli-k17`; `src/fs/mod.rs` interleaves read ranges owned by
`read-path-k14` with write/interpreter ranges owned by
`filesystem-interpreter-k16`. All other source roots belong wholly to one
slice.

Scoped validation checks a canonical prefix through one authoring slice,
requires every completed ownership block to be exact, and accepts only
explicit deferrals to later slices. Final validation requires all fifteen
roots, all 6,618 lines, and zero deferrals.

Fragment and Markdown checks consume one shared byte-level lexer. Directive
attribute order, spacing, fence forms, invalid contexts, encoding, line endings,
and recovery are fixed; neither validator independently guesses whether a line
is notation or literal content.

Page identities, navigation forms, ownership rows, fragment-index rows, and
early-use rows have exact raw-Markdown schemas. Every non-root definition must
appear in the numbered page assigned to its owner slice. The validator CLI
requires an explicit repository root and accepts only the seven source-owning
scoped prefixes; assembly is final-only.

The in-session adversarial review found the lexer, table schemas, diagnostic
records, page mapping, repository loading, scope enumeration, and prose example
placement underspecified. Those findings were verified and repaired in the
design. Because the repairs are themselves load-bearing format work, a separate
`review-design` leaf will re-derive the design rather than spending a second
fresh reviewer in this producer session.
