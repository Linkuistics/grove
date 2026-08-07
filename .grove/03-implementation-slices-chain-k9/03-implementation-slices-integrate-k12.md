# implementation-slices-integrate-k12

**Kind:** integrate-review-planning
**Integrates:** implementation-slices-review-k11

## Goal

Apply the verified findings from `implementation-slices-review-k11` while preserving the reviewed artifact's contract.

## Context

- Verify each planning finding and reshape the tree directly where warranted.

## Done when

- The implementation tree is complete, ordered, vertically sliced, and faithful
  to the reviewed design.
- Every accepted finding is reflected in the tree and every rejected finding is
  visibly classified in this task.

## Notes

## Findings disposition

- `implementation-slices-review-k11 P1` — **real issue**. Current-format
  writers move into `session-kind-tree-k23`; migration no longer owns ordinary
  root creation or grow-verb filename output.
- `P2` — **real issue**. New reviewed `tree-access-lock-k52` lands before
  `session-kind-migration-k27`, and migration depends on its integration.
- `P3` — **real issue**. Finish-lock and finish-delete/root-recreate checks move
  to `finish-lifecycle-k43`; `session-epoch-k35` uses direct tree removal and
  root initialization for its pre-finish handle-reuse boundary.
- `P4` — **real issue**. Universal locking and random signal-path lifecycle are
  split into reviewed `tree-access-lock-k52` and `session-signal-path-k57`;
  epoch admission and its meta-grove authority scrub remain together.
- `P5` — **real issue**. `herdr-session-kind-viewer-k51` is an independent leaf
  immediately after the filename grammar it renders; the later methodology
  leaf no longer owns viewer code.
- `P6` — **real issue**. `legacy-review-removal-k47`,
  `methodology-and-viewer-k48`, and `durable-docs-reconciliation-k49` are review
  chains. The relationship-module split remains inside the reviewed k47
  artifact rather than becoming a separate mechanical prerequisite.
- `P7` — **contract stated unclearly in the tree**. The existing
  `receipt-guidance-test-cleanup-k17` leaf keeps its stable handle and moves to
  the root before `session-config-k19`; the planning review chain again contains
  only its three composition steps.
- `P8` — **contract stated unclearly**. k17 names all three failing assertion
  rows and explicitly limits itself to relaxing/removing the receipt-era test
  row; durable spec reconciliation remains with k49.
- `P9` — **contract stated unclearly**. k43 now specifies the generated finish
  task body and its promote → `finish-commit` → `complete --done` order.
- `P10` — **contract stated unclearly**. k39 now requires the version-checked
  absolute `grove-llm` path to be reused in Herdr hook JSON.
- `P11` — **real operational trade-off**. k46 records the meta-grove constraint:
  the installed driver continues this grove, and branch binaries are not
  installed into that live control path before finish.

No finding was rejected as noise or as a context misread.
