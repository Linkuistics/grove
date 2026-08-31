# plan-k8

**Integrates:** plan-k2

## Goal

Triage the adversarial findings from `plan-k2` and revise the requirements
artifact so the real findings are resolved before planning begins.

## Context

Read the committed `plan-k2` review and the producer it reviews, `plan-k1`.
The review is intentionally findings-only; this session owns classification,
the fixes it accepts, and any tree reordering or new work those fixes require.

## Done when

Every `plan-k2` finding is classified as valid, trade-off, or noise; every valid
finding is reflected in the root brief, producer decision set, and task tree as
appropriate; the resulting requirements are internally consistent, falsifiable,
and leave `walkthroughs-k3` a safe artifact to plan from.

## Notes

Do not treat the review as the charter: reject or narrow findings that do not
survive re-reading the evidence. Preserve the human's decisions where the issue
is only their summary or acceptance wording. Reconcile ordering changes against
the actual pre-order `pick` walk rather than against task positions by name.
