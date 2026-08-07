# implementation-slices-k10

**Kind:** planning

## Goal

Turn the integrated configuration-driven session design into ordered, vertical
implementation leaves that each land green and demonstrate useful behavior.


## Context

- Plan from the reviewed `docs/specs/config-driven-sessions.md` and reconciled
  ADR set, not from the pre-design assumptions in this initial brief.
- Use review chains for load-bearing implementation artifacts; keep ordinary
  mechanical slices as lone leaves when review would add no value.

## Done when

- The root tree contains the full implementation sequence, including migration,
  deletion of obsolete routing/receipt surfaces, docs, and verification.
- The deletion sequence explicitly covers the harness-stamp module/tests and
  ignore rules; command/subcommand and `--harness` surfaces; task-body
  `Kind`/`Harness` fields; `kind --with-harness --json` routing evidence;
  research-pair harness flags; `GROVE_SESSION_TARGET` and its scrub/tests; review
  receipts, comparisons, notices, and associated fixtures. This includes the
  receipt-era cross-surface assertion
  `canonical_guidance_explains_decomposed_receipts_and_pruning_scope` in
  `tests/composition_guidance.rs`, which is expected to fail after the
  requirements glossary changes and before its implementation slice lands.
- Reconciliation slices cover `content/` (including prompts and the nineteen-kind
  taxonomy), the doubt skill, the herdr plugin's filename parser, README and all
  user/architecture docs, plus splitting the surviving promotion/lock material
  from deleted receipt material in the current review-mechanics spec.
- Each slice crosses the agreed process or tree seam, has concrete acceptance
  criteria, and can pass independently without waiting for a later sibling.
- Ordering and dependencies are explicit in the tree; no implementation is
  performed in this planning session.

## Notes

The implementation sequence is expand, cut over, then contract:

- `receipt-guidance-test-cleanup-k17` first repairs the already-surfaced
  canonical-guidance regression without restoring receipt terminology.
- Reviewed expansion slices establish `session-config-k19` and
  `session-kind-tree-k23`; `herdr-session-kind-viewer-k51` then independently
  proves that filename grammar in the separately versioned renderer.
- `tree-access-lock-k52` moves every current tree operation onto the universal
  working-tree lock before reviewed `session-kind-migration-k27` consumes it.
  Reviewed `driver-lease-k31`, `session-signal-path-k57`, and
  `session-epoch-k35` then build process ownership in independently executable
  layers.
- Reviewed cutover slices make the configured driver active in
  `lifecycle-cutover-k39` and complete teardown in `finish-lifecycle-k43`.
- `legacy-launch-removal-k46` performs the mechanical launch contraction.
  Reviewed `legacy-review-removal-k47`, `methodology-and-viewer-k48`, and
  `durable-docs-reconciliation-k49` contract and reconcile the load-bearing
  relationship, methodology, and durable-record artifacts;
  `acceptance-verification-k50` proves the integrated matrix.

The twelve load-bearing seams use review chains. The four lone leaves are the
focused receipt-test repair, the independently versioned viewer, the mechanical
launch-surface deletion, and final acceptance. Every dependency is named by a
stable handle in the child task that consumes it.
