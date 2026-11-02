# owner-pack-checkouts-k79

## Goal

Settle the owner and the procedure that install the skill-pack repositories
as local checkouts and keep them current, outside Ravel. Revise the
owner-delivery design and the external-pack contract to match.

## Context

- The root brief records the decision of ravel-git-acquire-k77: Ravel reads
  local directories only.
- `docs/specs/owner-skill-delivery.md`, `docs/owner-skill-delivery-evidence.md`
  and `docs/owner-skill-inventory.tsv` describe the current owner design.
- The brief of external-research-pack-k4 still requires pinned Git sources and
  offline pins in Ravel.
- The machine-environment contract gives machine setup to the mise bootstrap.
  It is one candidate owner. The owner's projects use jj for checkouts.

## Done when

- The design names the owner of installation and update and the commands that
  each one runs.
- The design states how a revision is chosen and recorded, and what a
  launch does when a checkout is missing.
- The design states the behaviour without a network, after the first install.
- The owner-delivery specification and its evidence agree with the design.
  The text that replaces the pinned-Git clauses of external-research-pack-k4
  is ready for the replanning leaf.

## Notes

The choice of owner can be the human's to make. If it is, ask with a
recommendation and evidence, as the spine directs. Earn a focused review if
the design changes the owner-delivery specification materially.
