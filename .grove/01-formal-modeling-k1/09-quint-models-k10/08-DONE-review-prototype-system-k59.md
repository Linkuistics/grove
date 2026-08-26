# system-k59

**Reviews:** system-k13

## Goal

Attack the one thing `cross-model-replay-k15` will not read: the INSTRUMENT this
column's 25 green obligations were reached with. Replay re-derives the findings;
nothing else re-derives the search shape, the narrowings, or the four
obligations that die only inside a bundle.

## Context

The producer is `system-k13` — [`models/system/lifecycle.qnt`](../../../models/system/lifecycle.qnt),
[`models/system/lifecycle-controls.qnt`](../../../models/system/lifecycle-controls.qnt),
the Quint column of [`models/system/README.md`](../../../models/system/README.md),
and entry 046 of [`docs/formalism-findings.md`](../../../docs/formalism-findings.md).

**The independence barrier still applies to you.** Do not open any `.als` file,
any Alloy section of a model-directory `README.md`, or entries 026 – 043 — and
note that 040 – 043 are the Alloy column's four LIFECYCLE entries, so the
temptation is stronger here than it was for either sibling review. Everything you
need is the catalogue, the two `.qnt` files, the runner and the README's Quint
section. The barrier comes down at `cross-model-replay-k15`, not here.

The green run to attack:

```sh
models/run.sh --scope lifecycle --family quint
```

72 commands, 25 of 25 cells, exit 0, 2m 05s wall.

## Done when

Each doubt below is either discharged with evidence or written up as a finding.
A review that finds nothing creates no `integrate-` leaf and simply retires.

### `driverStep` is a search narrowing living in `base`, and that is the biggest hazard here

Both sibling columns put their search narrowings in `scenario_` instances, so
every claim stayed checked unfocused in `base`. This column did not: `base`'s
own `step` is `any { driverStep ×3, envStep }`, and `driverStep` is a chain of
conditionals that picks ONE next move from the state. So every one of the 25
properties is checked only over orderings that chain admits.

- **Enumerate the orderings `driverStep` cannot produce.** For each, decide
  whether Grove's real driver can produce it. A reachable ordering the chain
  omits is a property checked over a world smaller than the claim.
- The chain reads `w`, `d` *and* `hist` (`nextIsTransition`, the `RSCurrentLive`
  branch). Does any branch condition on a HISTORY flag in a way that makes a
  claim about history true by construction?
- `envStep` weights the world at one disjunct in four and caps it at
  `ENV_BUDGET = 3` in `base`. Is there a `SY-` obligation whose falsification
  needs four or more environment actions in one trace? If so it is unfalsifiable
  in `base` and nothing says so.
- The producer's own measurement — 5 of 25 witnesses under a flat menu, 23 of 25
  under `driverStep` — is the evidence that the dial was necessary. Re-run it. Is
  the flat-menu figure reproducible, and does it hold at the runner's real 8000
  samples rather than at the 2000 the producer measured?

### The three narrowings are where a false green and a true one look identical

In each case the producer claims the catalogue is wrong. The alternative is that
the model is wrong and the narrowing hid it.

- **`SY-13`'s sweep** excludes `SCLegacy`, `SCForeign` and `SCMalformed`.
  `mutant_literal_sy13` fires, which shows the exclusion is load-bearing — it
  does **not** show the catalogue is wrong. Read `SY-13`'s two sentences yourself
  and decide independently whether the definition and the enumeration really
  disagree, or whether "admitted" was always meant to include the operator's
  actions and the producer read it too narrowly.
- **`SY-14`'s sweep** is `ADMITTED.filter(touchesTree)`. Check the producer's
  `FN-26` argument: does an operator's restorable exit genuinely require the
  blocked process to release its lease, or is that an artifact of modelling the
  lease as the same object across processes?
- **`SY-04.a`'s cap** is an enabling condition rather than an outcome, so half
  the obligation is not falsifiable through the outcome vocabulary. Is
  `mutant_many_transitions` genuinely checking the claim, or only the counter the
  model keeps for it?

### Four obligations have no isolating control

`SY-07.a`, `SY-09.a`, `SY-09.b` and `SY-14.b` die only inside a bundle. For
each: construct an isolating mutation, or establish that none exists and say
why. A bundle control that also kills a neighbour cannot distinguish which claim
was carrying the weight.

### Two witnesses have almost no margin

`SY-06.b` lands in 16 traces of 8000 and `SY-07.a` in 18. Both are deterministic
under the fixed seed. Establish whether each is a genuine protocol path or an
artifact of the scenario's own construction — the same question `finish-k57`
asked of the twelve `scenario_` witnesses, and the one the *vacuous invariant*
hazard is about. Check the other twenty-three the same way if the two turn out
badly.

### The composition abstraction may have abstracted away the claim

The tree is a ten-field summary and the finish is a five-phase cursor. `SY-05.b`
is the whole reason the scope exists, and it is checked against a model where
`FN-11` and `FN-19` are *represented by a single boolean*.

- Does `inv_SY_05b` check anything the composition did not already guarantee by
  where the producer chose to set `deletionProven`?
- `SY-11.b`'s cycle needs the launch generation held by the session's process
  rather than the driver's. The producer changed `launchOp` to do that mid-build
  in order to make `mutant_guard_disorder` fire. Is that faithful to Grove, or
  was the model bent until the control worked?

### The `verify_small` result is quoted with its depth

`quint verify` returns `NoError` at `--max-steps=4`, and the README says depth 4
reaches no scaffold. Confirm the shortest-path figures (eight driver moves to a
completed scaffold, nineteen to a proven finish) and that no obligation in the
column is cited anywhere as model-checked.

## Notes

Do not fix anything. Findings only — an `integrate-review-prototype` leaf is cut
by you, and only if you have findings worth acting on.

Two conclusions the producer reached that you should try hardest to break,
because they are the ones the rest rests on: that `driverStep` removes no
behaviour from the model, and that the five catalogue findings in entry 046 are
defects in the catalogue rather than in the reading of it.

## Review result

The instrument does not support its two load-bearing claims. `driverStep`
removes reachable lifecycle behaviour, one control said to cover three session
endings covers only one, the blocked-action sweep mistakes any successful
non-tree action for clearing a block, and the launch-generation abstraction does
not describe Grove's lock ownership. These are findings for integration; no
model, runner, catalogue, README or experiment-log source was changed here.

### Findings

#### F1 — `driverStep` removes real finish-session endings and reverses the epoch handoff

After a finish leaf is launched, the `RSCurrentFinishOnly` branch forces
`finishStepOp` until the task root is absent; `sessionEnds` is unreachable while
the finish leaf or its reserved witness remains
(`models/system/lifecycle.qnt:1574`, `models/system/lifecycle.qnt:1593`). It
therefore cannot represent either successful reopening path the methodology
requires: a finish session that declines before teardown, or one that adds
ordinary work and signals relaunch. It also cannot represent an early no-signal
or failed finish session. All are possible in the real foreground process; the
Rust loop accepts any of the three signal dispositions after any launched
session (`src/loop_driver.rs:162`, `src/loop_driver.rs:190`).

The model also interprets the ending in `reapOp` while leaving `genOpen` true
(`models/system/lifecycle.qnt:1258`). Grove does the opposite: after the child is
reaped it acquires the exclusive epoch handoff, invalidates the epoch, and only
then reads the signal (`src/loop_driver.rs:170`, `src/loop_driver.rs:353`). This
ordering is load-bearing enough to have its own error context in production.

The meaningful ordering audit is:

| ordering absent from `driverStep` | can Grove produce it? | disposition |
|---|---|---|
| tree transition before configuration validation | no; the pre-transition config load is first | faithful exclusion |
| launch before selection | no | faithful exclusion |
| a second lifecycle transition in one driver iteration | no; root transition/recovery and finish allocation are mutually exclusive at the current tree shapes | faithful exclusion |
| release the driver lease between iterations | no; the lease spans the loop | faithful exclusion |
| finish session ends before starting teardown | yes: decline, failure, no signal, or relaunch | missing behaviour |
| finish session adds ordinary work and relaunches | yes; it is the methodology's reopening exit | missing behaviour |
| epoch invalidation before signal interpretation | yes; this is the implemented handoff | model orders it backwards |
| invalid configuration stops the loop | yes; `SessionConfig::load` returns an error | model repeatedly records a refusal instead |

The task's history-conditioning suspicion is stale against the committed
artifact: `driverStep`, `nextIsTransition`, and the `RSCurrentLive` branch read
only `w` and `d`; none branches on `hist`
(`models/system/lifecycle.qnt:1565`, `models/system/lifecycle.qnt:1574`). History
flags remain property instrumentation, not search guards. `hist.envUsed` does
guard environment actions, which is the declared budget rather than a hidden
claim-specific history branch.

#### F2 — the session does not hold the launch generation, so the `SY-11.b` counterexample is bent to the control

`launchOp` transfers `genHolder` to the launched session process specifically so
the guard-disorder mutant can close a cycle
(`models/system/lifecycle.qnt:1154`). Grove has no such continuously held
session lock. Each ambient `grove-llm` operation independently acquires a shared
epoch lock and holds it only through that operation and its later tree guard
(`src/driver_lease.rs:845`, `src/driver_lease.rs:893`). The driver takes the
exclusive epoch lock during activation or post-reap invalidation
(`src/driver_lease.rs:268`, `src/loop_driver.rs:339`).

There can be a real wait edge between an in-flight ambient operation and epoch
invalidation, but its holder is the operation, not the foreground session, and
the invalidation begins after reap. The current two-process cycle therefore does
not establish that Grove's actual guard graph has the path the mutant needs.
Rebuild the abstraction from the shared-operation/exclusive-handoff ownership
above, then decide whether `SY-11.b` still has a reachable negative control.

#### F3 — `SY-09.a` and `SY-09.b` have no bundle control; the README's claim is false

The README says `mutant_no_signal_is_done` kills relaunch, done and no-signal as
one bundle (`models/system/README.md:1205`). The dial only changes the `k == 3`
arm of `reapOp`; the `k == 1` and `k == 2` arms are constants
(`models/system/lifecycle.qnt:1260`). Its controls file consequently declares
only `inv_fail_MUT_SY_09c_no_signal_is_inferred_as_done`
(`models/system/lifecycle-controls.qnt:683`).

Targeted 8,000-sample runs on `mutant_no_signal_is_done` confirmed the source
reading: `inv_SY_09a_relaunch_continues_the_loop` and
`inv_SY_09b_done_ends_the_loop` reported no violation; only `inv_SY_09c...`
violated. Two isolating controls are straightforward and should replace the
unsupported bundle claim:

- `SY-09.a`: make only ending 1 map to `Stopped`, leaving endings 2 and 3
  unchanged.
- `SY-09.b`: make only ending 2 map to `Relaunch`, leaving endings 1 and 3
  unchanged.

#### F4 — the literal `SY-14.a` sweep confuses “succeeded” with “cleared the block”

The sweep defines `cleared` as “some action returned `OApplied`”
(`models/system/lifecycle.qnt:1365`). A literal sweep therefore condemns
`release-lease` or `validate-config` merely for succeeding even though neither
changes `w.fin.block`. That is not `SY-14.a`, whose property is that no admitted
action *clears* the block. The transition observer already has the correct
state-based test (`models/system/lifecycle.qnt:868`).

The producer's `FN-26` justification does not rescue this: an operator can
restore topology while the foreground session and its parent driver remain
alive, then retry the ambient operation under the same lease. More
fundamentally, releasing a lease still does not clear a block. Restricting
`SY-14.b`'s “every action on a blocked tree” to tree-touching actions is a
plausible reading; using that restriction to make `SY-14.a` pass hides a faulty
measurement.

An isolating `SY-14.b` mutation also exists: keep the block sticky and make
blocked tree actions return a non-naming refusal such as `RNotLive`. Then
`SY-14.a` remains green while `SY-14.b` alone fails.

#### F5 — the flat-menu measurement is neither durable nor reproducible

The claimed flat variant is absent from the producer commit and its jj evolution
history. A disposable instance reconstructed the stated 27-disjunct menu from
the 27 committed action definitions, imported the same `base` constants, and
restored the four focused witness predicates. With the non-default
`--step=flatStep`, depth 24, seed `0x5e0a51d3c0ffee01`:

- 2,000 samples reached 13 of 25 witnesses, not 5;
- 8,000 samples reached 14 of 25 witnesses, not 5.

The README is internally inconsistent too: it says 23 of 25 witnesses land in
`base` with two scenarios (`models/system/README.md:1147`), then states that
`base` owns 21 witnesses and four witnesses live in scenarios
(`models/system/README.md:1154`, `models/system/README.md:1299`). A comparable
measurement must preserve the exact alternate `step` and command, or be removed;
an unreconstructable figure cannot support the claim that the narrowing removed
nothing.

#### F6 — `SY-05.b` checks the summary's setter, not composition with the finish model

`finishStepOp` chooses both when `deletionProven` becomes true and when the
summarised tree becomes absent (`models/system/lifecycle.qnt:1293`).
`inv_SY_05b` then asks whether that same setter ever recorded the opposite
ordering (`models/system/lifecycle.qnt:1720`). The early-deletion dial proves the
history flag can notice a deliberately reordered setter, but it cannot detect a
real `FN-11` or `FN-19` path the five-phase cursor omitted. Thus the green is an
internal consistency check over the composition abstraction, not evidence that
the two component observations compose in Grove. The README and entry 046 should
either downgrade the claim explicitly or add a seam that imports independently
established component outcomes rather than manufacturing both operands together.

### Doubts discharged

- **`SY-13` narrowing:** the producer is right. The contract explicitly excludes
  environment/operator actions from the admitted set, while `Legacy`, `Foreign`
  and `Malformed` are stable and only hand edits leave them. The definition of
  terminal disposition therefore conflicts with its exact two-member
  enumeration. This is a catalogue defect, not a broader intended meaning of
  “admitted”.
- **Environment budget:** no existing negative control needs four environment
  actions. The only two mutants configured with budget 4—restart-repeat and
  guard-disorder—still violated their target invariants at budget 3 under the
  runner's 8,000 samples, depth and seed. The current mutation set supplies no
  evidence of a `SY-` falsification hidden specifically by `ENV_BUDGET = 3`.
- **`SY-04.a`:** `mutant_many_transitions` genuinely exercises the iteration
  counter and makes the invariant fail. It does not exercise an outcome or a
  refusal, but the claim says “at most one” rather than requiring a deferral
  outcome. The qualification is honest; the catalogue-finding claim about a
  missing refusal is not established by this obligation.
- **The four isolating controls:** all exist. Besides the two `SY-09` mutations
  and the `SY-14.b` refusal mutation above, isolate `SY-07.a` by making the
  driver append two finish leaves while leaving session creation refused. That
  fails exact-one without touching `SY-07.b`.
- **Thin witnesses:** both are real paths in `base`, not focused-scenario
  constructions. An MBT trace for `SY-06.b` completed the normal partial
  scaffold, then hand-edited a legacy tree and observed its refusal. An MBT trace
  for `SY-07.a` reached a spent tree, appended the sentinel, crashed, and reused
  it after restart. The latter is hostile rather than the ordinary retirement
  path, but it genuinely exercises append/reuse under `EN-08` and `EN-11`.
- **Depth claim:** deterministic runs with non-default `--step=driverStep(0)`
  reached completed scaffold at depth 8 but not 7, and proven absence at depth
  19 but not 18. The README and entry 046 explicitly say the depth-4 `NoError`
  covers no tree/finish/block obligation; no `SY-` obligation is presented as
  model-checked.

### Review execution note

The review did not run `models/run.sh`, tests, builds, lint or formatting. It ran
only the targeted Quint simulations named above against the committed model and
disposable review-only modules; those temporary modules were removed before
this finding was recorded. The codebase-memory indexer could not create a fresh
worker under the sandbox. Its only Grove index was an older sibling checkout and
reported every model/document path missing or excluded, so all cited review
paths were read directly; current Rust source was checked after graph-assisted
candidate discovery.
