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
