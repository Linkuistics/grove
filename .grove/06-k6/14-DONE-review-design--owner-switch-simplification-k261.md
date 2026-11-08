# owner-switch-simplification-k261

**Reviews:** owner-switch-simplification-k260

## Goal

Read the simplified owner switch adversarially against the owner's real
situation. This grove is the only grove that runs, and the owner is the only
user of Grove and Ravel. Find each removal that rests on more than those
facts, and each hazard that the simplified design leaves uncovered. Produce
findings, not fixes.

## Context

- The subject is the owner design `docs/specs/owner-skill-delivery.md` and
  each decision record as owner-switch-simplification-k260 changed them.
  The producer's task file holds the human's words and the four chosen
  simplifications.
- The section "Owner switch plan draft integration" of
  `docs/owner-skill-delivery-evidence.md` records the findings D1 to D5 of
  the earlier review and their dispositions.

## Done when

- Each finding names the defect, a concrete state of the owner's machine
  that shows it, and the rule that it breaks.
- The findings cover these doubts:
  - Does each removed mechanism defend only against another user or another
    grove? The finish session's own Claude Code process writes
    `~/.claude/settings.json`, and the owner's own tools write files of the
    switch set.
  - Does the owner checklist name each program that the earlier step 1
    named? Can the owner check each item without privileges?
  - Does one backup and one restore cover a stop at each point of the
    switch, a crash of the finish session included?
  - Does one dry run against a disposable copy of HOME certify what the
    real switch runs? See D1.
  - Are D3 and D5 resolved?
  - Does the final session's guidance stay readable through native removal?
- No finding is a fix. A review that finds nothing worth acting on cuts no
  integration leaf.

## Notes

The real HOME is read only for this review.

## Decisions (running log)

- The reviewed producer is commit `eccf8b044d83`. The review is inspection
  only. It reads the committed design, the earlier review and the plan draft
  evidence. It runs no test, build, lint, format or check task. The real
  HOME, launchd and the process table stay read only.
- The evidence document had hard findings only in the completed k258 review
  and in the k260 section under review. A preparatory commit marks the first
  as history and the second as pending, as the earlier review text was
  marked. The review changes no word of either section.
- A scratch measurement withdrew one suspicion. Copies of the manual by
  `cp -p` and `ditto` kept `com.apple.macl` and `com.apple.provenance`, so
  the manual's attributes are no finding.
- The section "Owner switch simplification review" of
  `docs/owner-skill-delivery-evidence.md` holds nine findings, S1 to S9. S1,
  S2 and S3 are P1, so the review cuts the integration step before
  owner-switch-simplification-k262 plans from the design.
