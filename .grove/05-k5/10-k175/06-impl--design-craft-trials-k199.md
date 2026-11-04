# design-craft-trials-k199

## Goal

Prove the boundary that this move adds, under Ravel directory delivery. A
task activates a craft skill from the `craft/` root without naming it, and
the agent reads that skill before the relevant action. Prove it in real Claude
Code and Codex sessions, with native copies absent.

## Context

- `docs/specs/pack-membership.md`, the row "The first `craft/` root" in "The
  acceptance of each move".
- `docs/common-loading-acceptance-record.md`: "The runner", the case
  `discovery` as the model of implicit discovery, and "Requirements directory
  trials" as the model of a case with an `external` source.
- The removal commit and the published pack commits from design-removal-k198.
- The case declarations of trial-supplier-refusal-k196 for a skill that a
  task relies on without naming it.

## Done when

A new case runs in both harnesses with two sources. `grove` comes from the
prepared Grove checkout at the removal commit. `design` comes from a plain Git
checkout of the published pack commit through the case's `external` object,
with the roots `["skills", "craft"]`. Each run is from a directory outside the
Grove working copy, the pack working copy and the checkout. Each run uses the
actual session and delegate read permissions.

- The launched kind is a core kind. Its task needs a design craft skill, and
  neither the mandate nor the task names that skill. The case declares the
  skill as one that the task relies on, so the refusal of k196 requires it from
  the `design` source.
- The agent reads the craft skill's `SKILL.md` from the `craft` directory of
  the `design` source before the action that the skill governs.
- The case passes the refusal check before its launches.
- The case section in the acceptance record names the Ravel build, the Grove
  commit, the pack commit, each `resourceDigest` and the observations. A gap
  is a named gap with a disposition, never a pass.
- `docs/repository-moves-record.md` links the case section.
- `bash scripts/check.sh` passes after the case and record are added.

## Notes

`decision-records` is a natural subject: a task that settles a durable
decision and must record it. Choose a craft skill and a task with an
unambiguous activation. No core skill may state the whole rule. The spine's
`ADR-FORMAT.md` states what binds without `decision-records`. A read of the
format file alone is therefore not the activation that this trial measures.

The specification asks for this one activation. It asks for no delegate case,
working-copy case, source refusal or offline launch. The research move proved
those mechanisms.

A real run spends subscription usage. Do not remove or change the owner's
native installations. The sandbox home links only each harness's sign-in
store.
