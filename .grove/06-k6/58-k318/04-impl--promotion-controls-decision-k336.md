# promotion-controls-decision-k336

## Goal

Run the six promotion controls against the rehearsal instrument, see each
fail, and decide whether finish can promote.

## Context

- The node brief, `_promotion-upgrade-rehearsal.md`, and the instrument and
  record of k333, k334 and k335.
- `docs/specs/coordinated-release.md`, "Acceptance and limits" and the last
  paragraph of "Compatible tap states".

## Done when

- Each control is seen to fail. The controls are a wrong channel, a
  `versioned-2` cut after promotion and mismatched archives. They are also
  silent schema-3 reuse, a removed retained resource and promotion before
  compatible Grove.
- The record states the decision: proceed to promotion in finish, or stay in
  state C. It gives the evidence for that decision.
- A stay in C cuts the packaging repair as live leaves before
  cross-repo-docs-reconciliation-k319.
- The record is committed, and the parent node closes.
