# session-kind-tree-integrate-k25

**Kind:** integrate-review-impl
**Integrates:** session-kind-tree-review-k24

## Goal

Apply the verified findings from `session-kind-tree-review-k24` while preserving the reviewed artifact's contract.

## Context

- Verify every `session-kind-tree-review-k24` finding against the binding spec.
- Keep legacy interpretation confined to migration inputs; do not restore
  current body metadata as a compatibility shortcut.

## Done when

- Every finding has a recorded disposition and every verified issue is fixed
  with a public-seam regression test.
- The format witness, filename grammar, finish reservation, and composition
  relationships remain one coherent contract.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

If a fix requires migration transaction design, externalize it under
`session-kind-tree-chain-k22` rather than absorbing `session-kind-migration-k27`.

## Dispositions

- **F1 — real issue, fixed.** Witness absence now selects the legacy grammar
  regardless of a kind-shaped slug; `design-notes` is covered through the
  public migration seam.
- **F2 — real issue, fixed.** The legacy-v2 adapter recognizes both terminal
  infixes, with an `ABANDONED` public migration regression.
- **F3 — real issue, fixed.** Exact LF-terminated witness validation remains
  strict, but one canonical constant now drives comparison, writing, and a
  diagnostic that exposes the differing bytes.
- **F4 — contract stated unclearly, fixed.** The binding spec, glossary, and
  `herdr-session-kind-viewer-k51` now state the real non-prefix label invariant;
  the label set has a direct invariant test and the misleading public-seam test
  name is corrected.
- **F5 — real issue and test gap, fixed.** The two uncalled current-tree body
  readers are removed, their neighboring source and fixture commentary now
  states filename-only routing, and the standalone legacy `research` mapping is
  explicitly characterized.
- **F6 — valid sequencing risk, already externalized.** Receipt implementation
  removal belongs to `review-receipt-removal-k84`; shipped guidance
  reconciliation belongs to `review-methodology-k87`. This leaf changes neither
  surface.

## Verification evidence

- A fresh-context adversarial review found one remaining F5 documentation gap;
  the stale source and fixture commentary was corrected, and the reviewer found
  no other actionable issue across F1–F4 or the F6 ownership boundary.
- `cargo fmt --check` passes.
- `cargo test --locked` passes in full.
- `cargo doc --locked --no-deps` passes; the deleted readers' documentation
  warning is gone. Remaining warnings are pre-existing work outside this leaf.
