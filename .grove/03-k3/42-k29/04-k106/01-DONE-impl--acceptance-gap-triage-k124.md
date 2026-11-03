# acceptance-gap-triage-k124

## Goal

Classify each gap and each unmeasured result of the trial record. Turn each one
into precise repair work before research, or into a scope question to the
human.

## Context

- `docs/common-loading-acceptance-record.md`: every case section whose result
  is a gap, unmeasured or not run. The briefs of k104 and k105 list them.
- The acceptance tables of `docs/specs/codex-skill-provisioning.md` and of
  Ravel's `docs/specs/skill-discovery.md`.

## Done when

- The record has a triage section. It gives each named gap its harness, its
  case, its cause as the record states it, and its decision.
- Each decision is a repair leaf, cut before
  `stage-evidence-reconciliation-k125`, or a scope question to the human.
- No gap is credited as a pass.

## Notes

A repair leaf states the exact change to the runner or the recipe, the case to
run again and the observation that closes the gap.

## Decisions (running log)

**Decomposition.** acceptance-handoff-k106 was too big for one session. It
became a node with four children: this triage, the stage reconciliation, the
integrated checks and the completion audit. This session does the triage only.

**Confinement in both harnesses: repair.** Each harness exits before its
session because it cannot write outside the grants. Claude Code needs
`/tmp/claude-<uid>`, and Codex needs its configuration directory. A trial
wrapper can point both at writable locations, so the gap has a runner repair.
`confined-harness-trials-k128` owns it. A blocker in Ravel's profile becomes a
scope question in that leaf.

**Missing report in Claude Code: repair.** In `explicit-native-error` and
`explicit-native-only`, the agent force-stopped with no text. Obligation 4 of
the preamble classifies another loader error as "a report" but gives no
order. A wording repair and one more run of each case is precise work.
`loader-report-wording-k129` owns it.

**Whole-preamble loss in Claude Code: repair.** Claude Code restored snapshot
files after compaction, so both attempts failed the gate. A first turn that
writes five or more work files after the guidance reads may push the snapshot
files out of the restoration. `claude-preamble-loss-trial-k130` owns it.

**`problem-ambiguity` and `problem-stale-native`: covered.** A pack reference
made both unable to measure their rows. `problem-ambiguity-unreferenced` and
`problem-stale-native-cli` measured the same rows with no pack reference, and
both passed in both harnesses. No repair is needed. The two original cases
keep no credit of their own.

**Continuation re-read: clarify, by the human's choice.** A resume that keeps
the whole earlier conversation is not a re-entry, so it needs no re-read. The
guidance text stays in the context. `loader-report-wording-k129` changes
obligation 10 and the specification, and records that the row holds under the
clarified rule.

**Rows with no mechanism: explicit gaps, by the human's choice.** The three
Codex native-loader rows, Codex `compaction-path`, the scrubbing adapter in
both harnesses and the encrypted Codex delegate message stay explicit
unmeasured gaps. The completion audit lists them.

**Codex `compaction-preamble`: a known limit, by the human's choice.**
`token_budget` loses the whole conversation and is not the default. The gap
stays recorded. The audit and the rollout documentation name it.
