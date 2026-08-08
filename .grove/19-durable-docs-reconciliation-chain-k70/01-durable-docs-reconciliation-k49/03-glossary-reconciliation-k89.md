# glossary-reconciliation-k89

**Kind:** impl

## Goal

Reconcile Grove bounded-context terminology and prove no obsolete current-state
claim survives across documentation surfaces.

## Context

- Depends on `architecture-records-reconciliation-k88`.
- Primary artifacts: `CONTEXT.md`, `CONTEXT-MAP.md`, and citation/navigation
  surfaces touched by the two earlier documentation increments.
- Keep `CONTEXT.md` a glossary, not an implementation guide, and preserve
  bounded-context ownership.

## Done when

- Grove terms describe the implemented configuration, lifecycle, session-kind,
  ownership, and finish behavior consistently; the context map's ownership and
  cross-context relationships remain accurate.
- A repo-wide legacy-claim check enumerates then classifies candidates using
  positive and cross-tree controls, and all current citations navigate.
- Docs/navigation checks, `cargo fmt --check`, and `cargo test --locked` pass.

## Notes

Record structural facts rather than occurrence counts that invalidate
themselves.
