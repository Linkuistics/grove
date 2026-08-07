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
