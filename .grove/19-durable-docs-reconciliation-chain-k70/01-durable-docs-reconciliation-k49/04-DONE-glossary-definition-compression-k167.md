# glossary-definition-compression-k167

**Kind:** impl

## Goal

Return `CONTEXT.md` to a glossary: definitions plus their `_Avoid_` distinctions,
with implementation detail held by the architecture and spec sets that now own
those seams.

## Context

- Raised by `glossary-reconciliation-k89`'s check of this node's `Done when`
  bullet "`CONTEXT.md` remains a glossary rather than an implementation guide".
  The check fails: `content/CONTEXT-FORMAT.md` says "Keep definitions tight. One
  or two sentences max", and 36 of 40 entries exceed that. The longest —
  **Finish transaction** (28 sentences), **Complete finish cycle** (27),
  **Session epoch** (15), **Promotion transaction** (13) — carry manifest
  layouts, hashing rules, device/inode revalidation, and rollback topology.
- The duplication is new, not longstanding: `architecture-records-reconciliation-k88`
  had just written those same seams into `docs/ARCHITECTURE.md` at their
  implemented interfaces. Two durable sets now state the same mechanism, which is
  the failure the minimum-coherent-set discipline exists to prevent — restate one
  and the two will disagree, after which neither binds.
- Compression is the risk, not the mechanics. The `_Avoid_` lines encode
  distinctions this grove paid for (`[[Driver lease]]` vs `[[Tree access lock]]`,
  harvested vs pruned, brief chain vs review chain); losing one silently is worse
  than the duplication. Keep them.

## Done when

- Every entry states what the term *is*, with its `_Avoid_` lines intact; detail
  that describes *how a seam works* survives in `docs/ARCHITECTURE.md` or the
  spec set, and is deleted from the glossary rather than copied.
- No term is dropped, and every `[[wikilink]]` still names a live entry.
- `CONTEXT-MAP.md` ownership and the bounded-context split are unchanged.
- `tests/legacy_claim_sweep.rs` and `tests/reference_navigation.rs` stay green
  without relaxing a check or adding a classification that hides a real claim;
  `cargo fmt --check` and `cargo test --locked` pass.

## Notes

The enclosing `durable-docs-reconciliation-chain-k70` already supplies the
adversarial pass over this work, so this leaf needs no review chain of its own.
