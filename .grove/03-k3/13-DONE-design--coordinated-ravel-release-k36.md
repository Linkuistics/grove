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

## Decisions (running log)

- Read-only inspection found local/remote Ravel main at
  `24a545236185b0910ba6117fe8f9e8b2e2c8a748`, with Rhai schema 3 and a still-1.1.0
  manifest. No local/remote 2.0 tag or GitHub release was found. The tap and
  installed pair remain Grove 22.8.0 / Ravel 1.1.0.
- The historical directory design `b93bb927` is a separate line, not main.
  The proposed combined release is 2.0.0 with script/request schema 4. k34 owns
  exact fields and compatibility; other schema families do not advance by habit.
- Created isolated jj workspace `Ravel.coordinated-release-design` from exact
  main for its release-channel contract. No active workspace, other task tree,
  installed binary, tap or owner configuration was edited.
- Asked the human to acknowledge transferred release/upgrade ownership and
  recommended combined 2.0.0 through `ravel@2` first. The answer is pending.
  No hold is inferred from this tree; this leaf remains live.
- Drafts: `docs/specs/coordinated-release.md`,
  `docs/coordinated-release-evidence.md`, and Ravel's
  `docs/specs/release-channels.md`. Public choices and handoff are marked
  proposed. The mandatory scheduled design review will challenge the artifact;
  no competing in-session reviewer is used.
- Cut `coordinated-ravel-release-k39` immediately after this producer and before
  dependent work. The producer remains live, so no finishing Creator receipt is
  written yet. Ravel's draft is in working change `tnmsvtxq`; its final exact
  commit remains to be recorded when the owner handoff is settled.
- The visual source is served through `task design:harness-selection PORT=52710`.
  HTTP/manifest checks are available, but rendering was not verified: no browser
  connection was available and Safari control timed out. Keep that limitation
  explicit; HTTP success is not visual verification.
- Validation passed for local links/anchors and whitespace in the three new
  documents, viewer manifest references and graph entry markers. Read-only
  `git diff --check HEAD` passed in both workspaces. `brief-chain` resolves the
  inserted review through the expected root and common-loading briefs. No
  product/release test was claimed for this documentation-only draft.
- Remaining before retirement: receive and record the ownership/release-choice
  answer; reconcile any resulting design changes; make the agreed finish
  extension explicit in the root charter; record the exact Ravel artifact
  commit; finish the review handoff, retire and seal the Grove task commit.
  While required input is pending, keep this session open and send no ending
  signal, per the shared execute procedure.

- The human confirmed: "this is the only workstream that is currently live,
  and will stay that way until we are finished because of the difficulty of
  our cutover." Together with k32's explicit transfer of Ravel release work,
  this establishes the acknowledged single-owner handoff. This grove owns
  release, upgrade and cutover execution; no other workstream resumes during
  it. No notification, signal or edit to another task tree is needed or implied.
- Settle the recommended combined Ravel 2.0.0 route, with script/request schema
  4 reserved for k34's exact directory contract. Publish keg-only `ravel@2`
  first, compatible Grove next, then promote the same Ravel assets. The design
  remains subject to its scheduled review; this is not publication approval
  or a claim that release tooling exists.
- The single-live-workstream constraint means this grove must carry every
  unfinished Rhai documentation or acceptance prerequisite needed for release.
  Rollout planning owns those concrete tasks in a separate Ravel workspace;
  it cannot wait for the paused Rhai workstream to deliver them. Its unrelated
  editorial goals and task markers remain untouched.
- Reconciled the root charter, common-loading and rollout handoffs with the
  acknowledged ownership and finish-owned promotion. The owning specifications
  now state the design for review rather than awaiting an ownership answer.
- Safari became available on continuation. Both final diagrams rendered and
  fresh deep links reached their intended views. The tap-state view now runs
  horizontally; the ownership view uses shorter labels. Desktop light
  appearance was inspected; mobile and dark appearance were not measured.
- Ravel's release-channel contract is sealed at
  `80e382223129cc659783b78e6e93bd6d3e391739`, local bookmark
  `coordinated-release-design`, based on
  `24a545236185b0910ba6117fe8f9e8b2e2c8a748`. Its isolated workspace is clean
  on an empty successor. It was not landed on main or pushed; k34 and rollout
  consume this exact artifact and reconcile it with the feature line.
- Final document/link/manifest validation passed. A whitespace check found a
  trailing blank line in this task log; it was removed and the check passed.
  The public release/tooling and actual owner-cutover checks remain assigned
  to implementation and rollout. The focused review is k39, next in this node.
- This launch has no `GROVE_PROVIDER_EVIDENCE`. Under the retirement procedure,
  the review carries no invented Creator line. The common-loading node remains
  live through k39 and its other children; there is no parent close cascade.
