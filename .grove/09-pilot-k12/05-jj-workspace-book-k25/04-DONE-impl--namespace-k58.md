# namespace-k58

## Goal

Draft `04-namespace.md`, slice `no-consumer-vocabulary`, and take the book to
`book-check --through no-consumer-vocabulary --check all`.

## Context

- Responsibilities: [`docs/specs/jj-workspace-book-structure.md`](../../../docs/specs/jj-workspace-book-structure.md),
  *4 · The namespace it will not name*. Thesis: **the crate has no vocabulary for
  its consumer**, so the consumer supplies the name. This is the chapter that
  carries [`CONTEXT-MAP.md`](../../../CONTEXT-MAP.md)'s argument that
  `jj-workspace` is deliberately not a bounded context.
- Three blocks, 58 lines, all in `crates/jj-workspace/src/lib.rs` and separated
  by the whole of the `impl`: `namespace-reserved-names` (`55-61`),
  `namespace-control-dir` (`119-145`) and `namespace-validation` (`320-343`).
  Three ranges, one concept — say so on the page.
- Required worked-example anchor `worked-reservation`: `control_dir("grove")`
  creating `.jj/grove`; the same call again returning the same directory with its
  contents kept; and a path, an empty name and a jj-owned name each refused.
- It reserves `docs/USAGE.md#usage-driver-lease`, `CONTEXT.md#driver-lease` and
  `CONTEXT.md#loop-control-channel` — all three already declared in the manifest
  and present in their targets.
- The consumer's-half passages are heaviest here, and decision 5 of
  `jj-workspace-structure-k17` requires them to be **marked** — set apart from the
  crate's own vocabulary, so a reader sees at a glance which sentences the crate
  could have written and which it could not. Follow the convention
  `orientation-k55` established; do not invent a second one.

## Done when

- `04-namespace.md` exists; all three blocks resolved, defers replaced, ledger
  rows moved to `resolved`, fragment index rows added, navigation and contents
  updated, concept-index entries curated.
- `book-check --through no-consumer-vocabulary --check all` exits 0.
- The draft stage record's `## Provenance` names this commit.
- `cargo test --locked --workspace`, `cargo clippy` and `cargo fmt --all --check`
  pass. `scripts/check.sh` is red on `book-check` alone.

## Notes

**The one-directional cost argument is the reserved list's whole justification**:
a name jj adds later that `JJ_OWNED_NAMES` has not heard of costs a collision,
and a name listed there that jj drops costs a consumer a different word. Do not
reduce it to "these names are taken".
