# incremental-delivery-k4

**Integrates:** incremental-delivery-k3

## Goal

Triage the findings of review `incremental-delivery-k3` and apply the real ones
to the guidance that producer `incremental-delivery-k2` changed.

## Context

Read the findings from the review's own commit and from its body in
`03-DONE-review-impl--incremental-delivery-k3.md`. Verify each against the
source. Do not take the review's conclusions as given.

## Done when

- The integrating session classifies every finding as an unclear contract, a
  real issue, a visible trade-off or noise. It uses
  `references/integrate-review.md`.
- The session repairs each real finding in the guidance and keeps the repair
  small.
- `bash scripts/check.sh` or the narrower `task check:skills` passes after the
  repair. The session records the result here.
- No further review round is cut unless `references/decompose.md` earns it.

## Notes

## Decisions (running log)

- 2026-11-16: Verified the review's three findings against the current source.
  The unnamed scope authority is an unclear contract: name explicit human
  confirmation and retain the live remainder until it arrives. The stalled
  handoff has a real lifecycle gap: use the existing human-input handoff when
  no finite unblock is available, and keep repairs within each kind's remit.
  The finish paragraph, its table and the glossary retain a real contradiction:
  apply decomposition's scope admission test there too. These are small repairs;
  no redesign or additional adversarial review is warranted.
- 2026-11-16: Graph discovery did not return and was cancelled. This integration
  relies on direct Markdown reads, with no structural code or graph-completeness
  claims. A bounded scenario helper checks only the named findings.
- 2026-11-16: Applied the four-document repair. The baseline helper chose the
  intended actions by reconciling other rules, but identified the unnamed scope
  authority, missing blocked-handoff routing, review-remit conflict and finish
  contradiction. This is evidence of ambiguous guidance, not an observed live
  looping failure or a measured behavioral improvement.

## Verification

- `mise exec -- task check:skills` passed after the repair: core conformance,
  98 conformance controls, all five recorded packs alone and in aggregate,
  22 pack-check controls and all 13 plugin/native-delivery tests.
- This is the task's permitted narrow check. No runtime code changed; no
  full-suite or live-grove outcome is claimed.
- A fresh read-only helper applied the repaired guidance to the same three
  named scenarios: only the human can change agreed scope; unavailable external
  input yields the agent turn with the leaf/session open; review sessions report
  repairs as findings; optional cosmetics go to later work while finish is
  proposed. It found no literal contradiction or undefined deciding rule in
  those cases. This bounded check does not establish reliability across agents.

## Acceptance reconciliation

All three review findings are addressed without changing the settled delivery
contract. The root brief's bounded verification is covered by the producer's
recorded six-case walkthrough, the committed independent review and this repair
check. Useful slices, necessary preparation, material defect repair, required
safety checks and original acceptance remain binding. No concrete acceptance
gap or substantial redesign remains to justify another leaf or review round.
The root has no keyed node to name in a cascade; the canonical guidance and
glossary already hold the durable result. Finishing remains the driver's next
session, with its own confirmation.
