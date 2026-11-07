# owner-switch-program-k243

**Integrates:** owner-switch-program-k242

## Goal

Triage the implementation review of the owner switch program. Apply the
valid findings and verify the resulting program before later cutover work.

## Context

- Read owner-switch-program-k242 and the evidence document it names.
- The reviewed producer is owner-switch-program-k91, delivered by k239,
  k240 and k241 at commit `7d69e659`.
- `docs/specs/owner-skill-delivery.md` owns the contract. The model and
  rehearsal have the separate scopes that its test seams state.

## Done when

- Each finding has an explicit disposition supported by the contract and
  source evidence.
- Accepted repairs and their appropriate verification are complete. A
  changed recovery rule changes the model first.
- The handoff to owner-switch-entries-k92 and the real-plan rehearsal in
  owner-ravel-skill-inventory-k38 reflects the settled result.

## Notes

This leaf owns the repairs and their verification. The review was
inspection-only and supplied no new executable verification evidence.

## Decisions (running log)

**Base.** On 2026-11-07 this checkout was at `2e7ee49fd62`, the commit of
owner-switch-program-k242. The subject is program `7d69e659`.

**Triage.** A reading of `scripts/owner_switch.py` at the base reproduced
each of the nine findings. Each one is a real issue, and none needs a
redesign. Each repair stays inside the program, its rehearsal and the
contract's existing rules.

- F1, real: each gate opens `OWNER_SWITCH_PLAN` again. The repair binds the
  gate to the admitted digest through `OWNER_SWITCH_PLAN_DIGEST`. A gate
  that the program launches stops when the bytes differ.
- F2, real: the process gate keeps each descendant of a kept MCP server.
  The repair keeps a chain of processes that match the plan's MCP patterns,
  such as a launcher and the server that it launches. Each other descendant
  of a server counts as another process. The spec states the chain.
- F3, real: only the installation prelude dropped Claude Code's five
  location variables. The repair drops them from the environment of every
  command of a plan. It drops `CODEX_HOME` and each `RAVEL_*` name too.
- F4, real: the loader accepted any path. Each plan path must now be
  relative, normalised and free of `..`. Before any step, each resolved
  parent must stay inside HOME. The real HOME has no linked parent on these
  paths, and the manual's folder resolves inside HOME too.
- F5, real: missing gates, probes and commands defaulted to nothing. The
  loader now requires every field of the plan format. Each step that runs
  commands needs a non-empty list. The list of each of steps 4 to 7 must
  name the program's own gate command.
- F6, real: the trials ran in the switch's HOME. The plan's accepted values
  now name the retained staging HOME and the candidate keg. Each trial runs
  with HOME set to the staging HOME and `CODEX_HOME` set to its `.codex`
  tree.
- F7, real: nothing read the reader evidence again. Inside the probes, the
  run now reads each version directory of step 7's evidence in the backup
  again. A missing directory, an orphan marker or a lost registration
  counts as a failed probe. The model already lets any probe fail, so the
  rule adds a cause and no transition. The model stays unchanged.
- F8, real: the receipt lacked the activated configuration and the purge
  inventory. The receipt now holds them as the last paragraph of "The
  switch inside finish" requires. The backup keeps the evidence of steps 4
  to 7, so a later continuation reads the formula versions.
- F9, real: a record without the rerun of `settings-writes.sh` passed. A
  passing record now needs that rerun's pass and its Claude Code version.
  The task always runs it. The hooks installation runs a Claude Code
  command only with the version that a passing record names.

**Spec.** "The switch inside finish", "Concurrent writers", "The rehearsed
plan" and "Test seams" now state each rule that the repairs apply. The
evidence section "Owner switch review integration" holds the dispositions
and the guarding controls.

**Mutants.** Nine mutants of the repaired program each lacked one repair.
Each one failed its guarding control. The positive control
`mcp-server-kept` passed against the F2 mutant. The F1 mutant completed the
switch with a changed source, which is the case that F1 states.

**A rename.** The first `bash scripts/check.sh` failed the sweep of
`GROVE_*` names. It found this leaf's constant `GROVE_CODEX_SNAPSHOTS`. The
constant is now `CODEX_SNAPSHOT_STORE`. The rename changed no mechanism, so
the mutants did not run again. The rehearsal ran again on the renamed
program.

**Verification.** On the final program `b2eec19b87fe` and rehearsal script
`91f0be921bb5`, `task owner:switch:rehearse` passed 184 of 184 controls with
Python 3.15.1. Its rerun of `settings-writes.sh` passed with Claude Code
2.1.319, and the record passed. Before the rename, the system Python 3.9.6
passed 37 of 37 core, gate and review controls. `task model:owner-switch`
reported "all claims hold". The digests of both scripts, the Taskfile,
`settings-writes.sh` and both model files were the same before and after
these runs.

The second run of `bash scripts/check.sh` failed only the standalone test
`a_timed_out_selection_launches_nothing_and_publishes_nothing` with "select
never ran". owner-switch-gates-k241 recorded the same failure, and this leaf
changed no Rust source. The third run of `bash scripts/check.sh` passed all
18 of its principal steps, with 1,344 tests passed.
