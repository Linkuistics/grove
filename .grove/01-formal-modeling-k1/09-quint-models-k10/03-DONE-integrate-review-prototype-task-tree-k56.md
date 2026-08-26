# task-tree-k56

**Integrates:** task-tree-k55

## Goal

Integrate the adversarial review of `task-tree-k11`: repair the Quint runner's
false-green paths, strengthen the task-tree prototype where its evidence is
weaker than its claims, and reconcile the durable catalogue/finding record.

## Context

The reviewed producer commit is `7299dc9b`. The complete review is the adjacent
`review-prototype` leaf `task-tree-k55`; its seven findings are the source of
truth for this integration.

The independence barrier still applies. Do not open any `.als` file, the Alloy
sections of a model-directory README, or entries 026–043. This leaf owns fixes
and all post-fix verification that the review session was forbidden to run.

Review findings, verbatim in substance:

1. `models/run.sh` accepts a syntactically valid invented obligation such as
   `TT-99`, because `ob_of` is never checked against `manifest`; reverse
   obligation coverage is therefore not asserted.
2. `quint_run_verify` turns unrecognised non-zero Apalache exits into green
   per-property verdicts; a dead backend must abort with exit 2.
3. the bulk-repair ghosts retain no interrupted plan, accept any later bulk mark
   as the repair, and cannot mark a refused retry divergent; they do not
   establish re-running the identical invocation.
4. `TT-15.a` is narrowed by `walkStageReached` to avoid its conflict with
   `TT-24.b`, but the README and entry 044 disclose only the `TT-17` and `TT-20`
   narrowings.
5. `stepOpBlocked` returns `Blocked(OwnershipConflict)` even for an ordinary
   create collision before any effect lands, so entry 044 overstates that this
   case proves a missing outcome rather than a model defect.
6. the brief's `verify_` own-command rule and the runner/README's inherited-
   property rule disagree; `verify_small` itself owns zero commands.
7. the retained `TT-20` trace is described as both four transitions and as
   `beginOp` + three `stepOp`s + `foreignWrite` + `crashNow` (six).

## Done when

- Reverse coverage validates every parsed obligation against the catalogue in
  both dialects. A syntactically valid invented obligation and deletion of the
  last real property or witness are each demonstrated to make the runner red.
- A broken `quint` on `PATH` and a broken Apalache/JVM heap each abort with exit
  2 rather than becoming a verdict; the checks distinguish tool death from a
  genuine counterexample.
- `TT-23.b` records the interrupted plan and establishes that the same request
  completes to the same result; a refused retry falsifies the property, and the
  target-state-idempotence inference remains explicit.
- The `TT-15.a` qualification is either added to the catalogue and entry 044 as
  a finding or shown not to be a narrowing with wording that makes the staging
  premise explicit.
- Pre-effect ordinary collisions and post-effect partial mutations receive the
  outcomes their catalogue contexts require; entry 044's closed-outcome finding
  is revised to claim only what the corrected model establishes.
- One documented `verify_` convention agrees across the runner, README, model,
  and this chain's record, and no module loses commands under the parser.
- The `TT-20` trace, transition count, and derived test use one consistent
  replayable sequence.
- The task-tree Quint run and focused runner mutation controls are green after
  the fixes, with the exact commands and evidence recorded durably.

## Notes

This is integration, not a second review. Preserve the three hostile-read
conclusions that stood: `TT-17` is a catalogue contradiction; the
`PartialScaffold`/foreign-write counterexample is real; and `EN-11` does not gate
`TT-24.b`.
