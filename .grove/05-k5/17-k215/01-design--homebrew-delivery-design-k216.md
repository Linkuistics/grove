# homebrew-delivery-design-k216

## Goal

Amend the specifications and the decision records, so that they describe
Homebrew installation and Ravel delivery as `skill-marketplace-decision-k201`
settled. Answer the questions of the parent brief.

## Context

- The parent brief `homebrew-skill-delivery-k215` states the outcomes and the
  open questions.
- The running log of `skill-marketplace-decision-k201` holds the decisions in
  the human's words. Its section "Route effects" holds the evidence behind
  them.
- `docs/specs/owner-skill-delivery.md` names mise checkouts or a new
  marketplace in these sections:
  - "Pack checkouts", "Native residue" and "Resource lifetime".
  - "The final session and new launches" and "The switch inside finish".
  - "Rollback", "Purge" and "Cutover contract".
- `docs/specs/external-pack-contract.md`: the sections "Distribution" and "The
  checker without a copy".
- `docs/specs/codex-skill-provisioning.md`: the supported delivery routes.
- `docs/specs/pack-membership.md`: the lines about `skill-marketplace-k185`.
  The view `docs/design/external-pack-contract/membership-boundaries.mmd`
  shows the same step.
- `docs/adr/skill-delivery-is-the-users-choice.md`: Grove's formula now
  couples Grove's release to the packs, and the native skill routes retire.
- The tap `~/Development/Delivery/homebrew-taps` holds the formulae of Grove,
  Ravel and TestAnyware.
- The tasks `release:formula:test` and `release:smoke` test Grove's formula
  and the installed layout of its archive.

## Done when

- Each document above agrees with the decisions of k201. `ADR-FORMAT.md`
  governs the rework of the decision records.
- Each question of the parent brief has an answer in a specification or a
  decision record.
- Each of the six seams of the root brief has its place in a specification.
- A change to a recovery rule of the owner design changes
  `docs/models/owner-switch.qnt` first, and `task model:owner-switch` stays
  green.

## Notes

The amended owner design carries the cutover inside finish, so it is
load-bearing. At the end of the session, judge whether it earns a review
chain. Cut that chain with `leaf-insert` before `homebrew-delivery-plan-k217`.

`owner-pack-checkouts-k93` now names Homebrew installation at the level that
k201 settled. Its details follow this design.
