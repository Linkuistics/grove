# receipt-guidance-test-cleanup-k17

**Kind:** impl

## Goal

Reconcile the composition-guidance regression tests with the already-confirmed
removal of producer launch receipts, generations, target comparison, and
diversity warnings.

## Context

- `cargo test` during `driver-exclusivity-integrate-k16` stopped in
  `tests/composition_guidance.rs::canonical_guidance_explains_decomposed_receipts_and_pruning_scope`
  because three receipt-era rows are stale: `CONTEXT.md` × `factual source
  session`, `CONTEXT.md` × `producer generation`, and
  `docs/specs/doubt-grove-review-mechanics.md` × `legacy node receipts are
  uncheckable`.
- `jj file show -r @- CONTEXT.md` proved both phrases were already absent before
  that integration leaf, so this is implementation lag from the broader
  config-driven-session design rather than its regression.
- Preserve `docs/specs/config-driven-sessions.md` and the root brief's confirmed
  removal of receipts, `GROVE_SESSION_TARGET`, and diversity inference; do not
  restore dead terminology merely to satisfy the old assertion.

## Done when

- Receipt-specific guidance assertions are removed or rewritten around the
  surviving `Reviews` / `Integrates` composition contract.
- The obsolete review-mechanics-spec row is removed or relaxed in the test; this
  leaf does not broadly reconcile that durable spec, which remains owned by
  `durable-docs-reconciliation-k49`.
- All related canonical-guidance fixtures agree with the migrated nineteen-kind
  filename and config-driven routing design.
- The full test suite passes without reintroducing any removed receipt surface.

## Notes
