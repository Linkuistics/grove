# fragment-engine-k29

**Reviews:** fragment-engine-k26

## Goal

Independently rederive whether the committed fragment-engine increment satisfies
its bounded contract and is safe to build the remaining validator work upon.

## Context

- Read the committed `fragment-engine-k26` diff, its task, the parent brief, and
  `docs/specs/ordinal-fs-tree-book.md`; do not rely on the producer session's
  conclusions.
- The producer deliberately left ledger tables, canonical page placement, the
  final diagnostic envelope, and Markdown validation to later leaves.
- Concentrate review effort on malformed byte streams, directive boundaries,
  recursive graph invariants, scoped deferrals, frozen-corpus coverage, and the
  CLI's claims.

## Done when

- Every `fragment-engine-k26` done criterion is checked against code and fresh
  tests, including an independent challenge to at least one malformed-input or
  graph-ordering assumption.
- Findings distinguish defects in this increment from work explicitly owned by
  `ledger-and-pages-k27`, `diagnostic-contract-k28`, or
  `markdown-validation-k9`.
- Any actionable findings cite current file and line evidence and are handed to
  an integrate-review leaf inserted before the next live sibling; if there are
  none, the review records the verification evidence and retires directly.

## Notes

This is a fresh-context review of the sealed producer commit, required because
the in-session adversarial pass preceded substantive corrective edits.

## Decisions (running log)

Reviewed commit `lnpukulmyzsw` against `docs/specs/ordinal-fs-tree-book.md` and
the parent brief by reading `crates/book-validation` in full, rather than by
re-checking the producer's own conclusions. All eight required fragment
diagnostic classes, the scoped/final defer semantics, the literal-byte
discipline, and the CLI's read-only invocation claims hold as written; the
findings below are the exceptions.

The independent challenge was aimed at the malformed-input assumption behind
"safe malformed-input recovery are tested". It falsified two cases the committed
fixtures do not reach: an unclosed or mis-closed literal fence abandons the rest
of the book file (`parser.rs:225`), and an exact `book-page` directive inside a
root or composite is short-circuited before any context test
(`parser.rs:165-168`).

The strongest finding came from re-deriving the frozen-corpus claim rather than
trusting the corpus test. `source_range` clamps every comparison to the declared
range, nothing checks a source file's actual length, and the corpus fixture is
generated from the same bytes it is compared against — so a source file that
gains lines still validates as complete. Confirmed latent, not live: all fifteen
files match the ledger today at 6,618 lines total.

Findings are actionable, so this review cuts the integration step rather than
retiring alone: `fragment-engine-k30`, inserted at the slot held by
`ledger-and-pages-k27` so no leaf edits a cited file before the integrator opens
it.

Adversarial-graph hardening, `related`/`source` evidence, the full total-order
sort, `I001`, ledger-table and page-placement reconciliation, and every Markdown
code were classified as explicitly owned by `ledger-and-pages-k27`,
`diagnostic-contract-k28` and `markdown-validation-k9`, and were left with those
leaves rather than handed to the integrator.
