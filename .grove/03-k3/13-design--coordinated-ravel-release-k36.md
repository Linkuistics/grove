# coordinated-ravel-release-k36

## Goal

Set one coordinated Ravel/Grove release plan for this grove, including the Rhai
work already on Ravel main and the directory feature that this grove delivers.

## Context

Read the human's scope expansion on common-skill-loading-k32, root requirements
and discovery-rollout-k6. Ravel main was `24a54523`; its other task tree still
contains release-ravel-2-0-0-k37 and homebrew-upgrade-k38. Those are references
to reconcile with the owner, not permission to edit or signal the other tree.
Grove 22.8 currently admits 1.x only and its formula depends on unversioned
Ravel. The existing rollout brief proposes a versioned Ravel formula until
compatible Grove is released. Read actual Ravel and tap release procedures.

## Done when

- Establish the exact landed Rhai inputs and whether any release step has
  already run. Record one release owner in this grove and an acknowledged
  handoff with the other workstream, so its release leaves cannot race this
  grove's execution. Do not assume that editing this tree imposed a hold there.
- Settle whether Rhai and directory delivery ship together in the first
  coordinated release or in distinct releases. Pass exact version/schema
  constraints to ravel-rhai-skill-delivery-k34. Do not silently reserve the
  same public identity for two incompatible contracts.
- Design every intermediate tap state for fresh installs and upgrades. Start
  with the existing versioned-Ravel route, leaving unversioned Ravel compatible
  with released Grove until Grove support is published. If another route is
  needed, explain the trade-off and name the required earlier Grove work and
  release. A version predicate alone is not compatibility.
- Order Ravel integration, checks, release/tag/archive/book/formula publication,
  installed verification, Grove integration/release and eventual unversioned
  promotion. Name owning tasks and repositories. Preserve the original 1.1.0
  keg and inherited ending commands for all active drivers, including this one.
- Record exact restart/activation and partial-release recovery boundaries.
  Rollout must resume any completed cut without another version bump. Keep
  Grove's driver-owned finish and its final confirmation contract intact.
- Commit durable release contracts in their owning repositories as authorized,
  record exact commits here, and repair this tree's stage and rollout handoffs.
  Cut a focused design review before any dependent publication or implementation.

## Notes

The human authorized Ravel release work within this grove. Do not ask for the
same generic authorization again. This design performs no release, tap mutation,
owner activation or other-session signaling. Public choices outside the settled
scope can still need a specific human decision. Existing active work is preserved.
