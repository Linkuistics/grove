# skill-loading-replan-k80

## Goal

Reconcile the briefs and the live leaves of the tree with the decision that
Ravel reads local sources only. Grow the implementation leaves of
ravel-local-sources-k78 and owner-pack-checkouts-k79.

## Context

- The root brief records the decision of ravel-git-acquire-k77. Its paragraph
  under "Ownership boundaries", "Ravel", supersedes each pinned-Git requirement
  in a brief below it, until this leaf reconciles that brief.
- These briefs still state pinned Git sources, offline pins or the temporary
  Git refusal:
  - `common-skill-loading-k3`
  - `ravel-local-delivery-k23`
  - `grove-dual-ravel-k27`
  - `common-loading-acceptance-k29`
  - `external-research-pack-k4`
- ravel-pinned-delivery-k25 is closed. The human rejected its path, and its
  remaining leaves are abandoned.

## Done when

- No brief or live leaf requires Ravel to fetch, install or update a source.
- Each node brief states its Done when for local sources and for the
  checkout mechanism of owner-pack-checkouts-k79.
- Implementation leaves exist for the Ravel removal and the checkout
  mechanism, in an order that keeps each repository green.

## Notes

Use the committed outputs of k78 and k79. Do not repeat their design work.
owner-pack-checkouts-k82 integrated the review of k79. Take the root-brief
text from the handoff in k82, not from k79. That handoff also lists the
obligations that the integration added.

owner-pack-checkouts-k84 integrated the second review. Its handoff adds
obligations to the list in k82 and replaces k82's item about the
continuation. The focused review owner-pack-checkouts-k85 reads that repair
before this leaf. Read any integration it cuts before this leaf too.

owner-pack-checkouts-k86 integrated the third review. Its in-session reviewer
then found defects that need a new recovery model for the switch. The design
leaf owner-switch-recovery-k87 and its review now come before this leaf.
Take the switch's resumption, rollback and rehearsal contract from that
chain, not from the handoffs of k84 or k86.
