# groves-do-not-create-worktrees — brief

## Goal

grove exits the git-topology business. The user (or their worktree tooling,
e.g. worktrunk) creates the working tree and runs `grove do` — argument-less —
from inside it; grove never creates, integrates, or deletes worktrees or
branches, ships no topology utilities, and the complete finish cycle ends at
deleting `.grove/` and signalling done. Grove name = working-tree basename;
`.grove-worktrees/` loses all canonical status.

## Done when

The CLI is stripped to `do` / `migrate` / `retire` with in-worktree addressing
(cli-rework-k2), the embedded methodology content describes the new scheme
(methodology-content-k3), and v11.0.0 is released and verified live
(release-k4).

## Decomposition

Planning (plan-k1, this grove's grilling) → cli-rework-k2 →
methodology-content-k3 → release-k4, each a vertical slice sequenced so prose
always describes shipped behaviour.

## Pointers

- ADR *user-owned-worktrees* — the decision and its rejected alternatives
  (grove-owned topology; `create`/`remove` utilities; merge-in-session).
- ADRs *do-is-sole-lifecycle-verb*, *in-session-finish-cycle* — reworked in
  place during planning; the implementation leaves must match them.
- `.grove/01-plan-k1.md` — the grilling's running decision log.

## Notes

Glossary (`CONTEXT.md`) was reconciled inline during planning: *Complete
finish cycle* rewritten, *Grove name* added, *root-init* rephrased.
