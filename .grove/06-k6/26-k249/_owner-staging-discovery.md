# owner-staging-discovery-k249 — brief

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
- The plan's `accepted.trials` names the staging trials of each source.
  The switch program no longer reruns them. When a released source differs
  from its accepted digest, the switch stops in native delivery. The
  operator then reruns that source's trials, revises acceptance and repeats
  the dry run before a fresh attempt. "The switch inside finish" of the
  owner design states that rule.
- These trials prove the sources in staging. The one switch dry run of
  owner-switch-rehearsal-k255 proves the plan in a disposable copy of HOME.
  Keep the two apart.
- owner-staging-setup-k248 retained the staging HOME, the candidate keg
  and the 2.0 build under `/Users/antony/.local/state/owner-staging`.
  `task owner:staging:launch` runs one staging session. The section
  "Staging setup and isolation trial" of
  `docs/owner-skill-delivery-evidence.md` records them. A staging session
  runs in a working directory outside the real HOME. Otherwise Claude Code
  reads the stale `grove` copy in the real `~/.claude/skills`.

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
- Each source has scripted trials that the operator can rerun after a
  released source changes. Each trial runs with HOME set to the staging HOME
  and `CODEX_HOME` set to its `.codex` tree. The evidence names each command.
- The evidence document records each trial, its versions and its result.
  It reconciles each item of "Not measured" that a discovery trial
  measures.

## Notes

This leaf holds many real sessions. If it proves bigger than one session,
decompose it by harness or by case.

A failure is a finding for the owner design or for a pack, not a reason to
weaken a case. Record a scope decision only with the owner's agreement.

## Decomposition

The first session found that this work needs a trial runner and about
fifteen real sessions. It split the work by case into four leaves:

1. `staging-trial-runner-k275` builds the scripted staging trial runner.
   It runs the source trials, which launch no model. It also runs the
   discovery case on both harnesses: the mandatory read first and the
   implicit craft read.
2. `staging-kind-delegate-trials-k276` adds the Grove kind skill case with
   a relative resource, and the delegate case, on both harnesses.
3. `staging-compaction-stale-trials-k277` adds the compaction case and the
   stale-copy coexistence case on both harnesses.
4. `staging-guidance-probe-k278` runs the Claude Code trial that disables
   the plugin of its own guidance. It then reconciles "Not measured" and
   closes the evidence of this node.

Each model case also reads the native skill listing of its session. A
superseded name fails the case. The stale-copy case is the one exception.
It plants its stale copy in the working directory, never in the staging
HOME. It expects that one name.

Each leaf adds its cases to the runner of the first leaf. Each leaf records
its own trials in the evidence section of this node.
