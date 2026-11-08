# owner-staging-discovery-k249

## Goal

Prove discovery, reads and lifetime of the ten keg sources on Claude Code
and Codex in the staging HOME. No native copy of a superseded skill exists
there.

## Context

- The cutover contract in `docs/specs/owner-skill-delivery.md` lists the
  staging trials after its three first steps. The section "Test seams"
  states that the trials use Ravel's public `start-session` and
  `skills resolve` with the actual harnesses.
- owner-staging-setup-k248 supplies the staging HOME, the candidate keg and
  the installed kegs. Use the accepted 2.0 build of
  `scripts/harness-trials/builds.json`.
- `scripts/harness-trials/` and `docs/common-loading-acceptance-record.md`
  hold the earlier trial tooling and cases. Reuse them where they fit.
- The plan's `accepted.trials` names, for each source, the staging trials
  that step 4 of the switch reruns. Step 4 reruns a trial of each of Grove's
  sources whose digest changed. It sets `OWNER_SWITCH_SOURCE` and
  `OWNER_SWITCH_DIGEST`, with HOME set to the staging HOME and `CODEX_HOME`
  set to its `.codex` tree.

## Done when

- On each harness, one trial passes for each case below:
  - A task activates a relevant craft skill without naming it, and the
    agent reads it before the relevant action.
  - The mandatory file is read first.
  - A task names a Grove kind skill, and the agent reads it and a relative
    resource of its source.
  - A delegate reads a skill resource.
  - A compaction occurs, and the agent still reads skills afterwards.
  - A stale native copy coexists with the Ravel copy. The trial records
    which copy the agent reads.
- With one candidate-keg source moved away, a launch refuses as
  `skill_source_missing` before any worker. The source returns, and its
  digest is unchanged.
- A launch without a network receives the same directories.
- With the off value, `ravel skills resolve --json` reports `disabled`.
- A Claude Code trial disables the plugin that holds its own guidance,
  launches a nested probe, and reads its guidance again by absolute path.
- Native discovery on each harness lists no superseded skill, or the trial
  fails.
- Each of Grove's six sources has one scripted trial that step 4 can rerun
  under the conditions above. The evidence names each command.
- The evidence document records each trial, its versions and its result.
  It reconciles each item of "Not measured" that a discovery trial
  measures.

## Notes

This leaf holds many real sessions. If it proves bigger than one session,
decompose it by harness or by case.

A failure is a finding for the owner design or for a pack, not a reason to
weaken a case. Record a scope decision only with the owner's agreement.
