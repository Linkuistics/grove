# coordinated-ravel-release-k40

**Integrates:** coordinated-ravel-release-k39

## Goal

Triage the review's findings on the coordinated release contract and apply the
real ones in their owning repositories. Do this before the Rhai contract repair
and the rollout planner consume the design.

## Context

Read the findings from the review's own commit in this tree, not from this
body. The reviewed artifacts are Grove's `docs/specs/coordinated-release.md`,
`docs/coordinated-release-evidence.md` and the two release views, and Ravel's
`docs/specs/release-channels.md` at commit
`80e382223129cc659783b78e6e93bd6d3e391739` on bookmark
`coordinated-release-design` in `Ravel.coordinated-release-design`. Work
there under that repository's instructions and VCS discipline. The producer
task `coordinated-ravel-release-k36` records the artifact set and the owner's
acknowledgement. No release, tap change, owner activation or paused-tree edit
belongs to this step.

## Done when

- The session classifies each finding as a contract stated unclearly, a real
  issue, a visible trade-off, or noise, and records the reason.
- The session applies each accepted finding in its owning document and reworks
  each ADR set as `ADR-FORMAT.md` directs, with every citation reconciled.
- The two specifications, the evidence document, the release views and the root
  and node handoffs agree with each other after the edits.
- Grove's document checks pass, Ravel's isolated workspace is sealed on an empty
  successor, and the task records the exact Ravel commit for k34 and rollout.
- A finding that demands a redesign rather than a repair becomes a new producer
  review chain beside this leaf, not an edit here.

## Notes

This step may spend one narrow in-session reviewer. Preserve every running
session's original ending command. ravel-rhai-skill-delivery-k34 and the
rollout planner consume this step's result.

## Decisions (running log)

- The session read the nine findings from review commit `78778753` and the
  artifacts at Grove `78778753` and Ravel `80e38222`. Ravel main is still
  `24a54523`, locally and on origin. No release, tap, owner or paused-tree
  change belongs to this step.
- Each finding reproduced against source. Finding 1: `authority.rs` on Ravel
  main refuses a lone `policy.ts` with `config_format_retired` and exit 3. The
  tap's Grove formula tells users to copy `policy.ts`. The owner has only
  `policy.ts`. Real issue: row D claims a compatible pair for upgrades.
- Finding 1 keeps the owner's earlier choice in the paused workstream: no
  migration guide. The repair names the break and requires the published remedy.
  Remedy: the 2.0.0 changelog, book chapter 4 and compatible Grove's
  `config.rhai` sample. Ravel's `configuration-script-uses-rhai` ADR already
  records the clean cutover, so the specification cites it.
- Finding 2 is a real issue. Both decisions pass all three ADR tests. Ravel
  gains `a-caller-transition-publishes-a-versioned-formula-first`. Grove gains
  `finish-promotes-after-the-grove-release`. The two specification paragraphs
  shrink to citations.
- Finding 3 is a real issue. The session chose the frozen transition formula.
  After promotion, unversioned `ravel` is the only publication channel for
  later cuts. The task refuses `versioned-2` once unversioned `ravel` has major
  2.
- Under finding 3, promotion's tap commit also marks `ravel@2` deprecated in
  favour of unversioned `ravel`. The formula stays installable at the promoted
  version. The session rejected tracking every 2.x cut: it doubles each publication to
  keep a formula that duplicates the default.
- Finding 4 is a real issue. `cmd_record_teardown` requires session-epoch
  admission through `GROVE_LAUNCH_DIR`. The compatible Grove release must keep
  that contract. Otherwise rollout must deliver the helper preservation in
  `docs/RELEASING.md`.
- Finding 5 is a real issue. Both smoke scripts hard-code 1.1.0. The
  dual-Ravel stage already owns release inputs against independent builds.
  Rollout pins the published 2.0.0 archives beside 1.1.0 before Grove's
  release.
- Finding 6 is a real issue. The Homebrew manual documents periodic full
  cleanup and `HOMEBREW_NO_CLEANUP_FORMULAE`. Both specifications now require
  `ravel` in that list wherever every Homebrew invocation reads it. The window
  is from the owner's unversioned upgrade to the old driver's reap.
- Finding 7 is a contract stated unclearly. The delivery evidence gives the
  reason: a prepared prompt changes what `request.prompt` means to a script.
  The specification adds that test and the cost. It leaves the schema-3
  decision to the directory design.
- Finding 8 is a real issue. The paused `release-ravel-2-0-0-k37` would cut
  3.0.0 after this grove, because `release-prepare.sh` bumps when the baseline
  is complete and main changed. Rollout gains the brief-update text for that
  workstream.
- Finding 9 is a contract stated unclearly. The stay-in-C decision is
  rollout's, before `finish-commit`. Inside finish, only the recovery table's
  resume rows apply. No finding demands a redesign, so no new producer chain is
  cut.
- The review's visible costs stay as accepted trade-offs. The specifications
  already state them, and no owner decision changes them.
- Converted first, as preparatory changes: the Grove specification, the
  evidence document, the views README, the common-loading and rollout briefs,
  and Ravel's channel specification. The root brief, RELEASING.md, the
  delivery specification and the k12, k34 and k37 bodies stay unchanged. Each
  stays true after the edits.
- The conversions landed first. Grove change `qxmnnwsm` converts its five
  documents. Ravel commit `c94741960df5a62f569efae1d1cec3c05f0d54ee` converts
  the channel specification.
- Ravel's integrated commit for k34 and rollout is
  `2a77ffbda3b2510f0997c740c0642b18afaeece7`. It is on local bookmark
  `coordinated-release-design` in `Ravel.coordinated-release-design`, above
  main `24a54523`. The workspace sits on empty successor `ltsuvvpr`. Nobody
  pushed or landed it.
- Homebrew's installed documentation states that a deprecated formula warns
  and still installs. That supports the frozen `ravel@2` rule, so the session
  spent no in-session reviewer. Implementation fixtures and rehearsals will
  test the rule.
- The root brief keeps its sentence "subject to its focused review before
  dependent work". The review has now happened, so the sentence stays true.
  The human-agreed charter therefore stays unchanged.
- Verification: the checker reports no hard finding in any edited document or
  commit message. Every local link and anchor resolves. `reference_navigation`
  passes 13 of 13. No added line has trailing whitespace.
- A whitespace-joined sweep found no stale review status and no stale Ravel
  commit pointer. The line-based grep missed wrapped phrases, so the session
  replaced it with that sweep, and a control and a mutation proved it.
- No review leaf names this integration, and this launch has no
  `GROVE_PROVIDER_EVIDENCE`, so no Creator line changes. The common-loading
  node keeps live leaves, so no close cascade runs.
