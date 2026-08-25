# disposal-k45


## Goal

Answer `FN-21` and `FN-31` in `crates/grove-finish/models/finish.als` — disposal's
re-entrancy, the cleanup marker's create / replace / remove transitions, the
replacement's atomicity with respect to readers, and the reaper. Seven
obligations, and the last of `handoff-k42`'s three children.

Cut `exits` as the next sibling **of this node**, under `finish-k8`, as this
leaf's last act.


## Context

`revalidation-k44` left the file green for `FN-22` at **101 commands, 5 m 46 s,
twenty-one empty alloy cells**, and the ten-row table is now the thing the two
handoffs run under. Read `crates/grove-finish/models/README.md` before writing a
command; four of its sections are specifically this leaf's inheritance and are
named below.

**What now exists that a disposal slice builds on.**

- **The forward settle is the disposal, still in one step.** It fires at
  `Quarantined` on a fresh `observed = Committed`, and clears `Quar.qRid`,
  `Slot` and `Man` together. `FN-22.i`'s check states exactly that and says so:
  the catalogue's stable state after *complete: dispose* is **task root
  `Absent`, quarantine holding the root** — the state from which `FN-21`'s
  disposal proceeds — and this file passes through it and out of it in one
  transition. **`FN-21.a`'s *re-enterable from any interruption* is the claim
  that turns that one step into several**, and it is the same shape as the split
  `FN-22` forced on the restoration. Expect to pay for it the way that split was
  paid for: a new phase between, and one state on every witness that passes
  through the forward settle.
- **`witness_FN_19` still leaves a state nothing in the file can leave**, and
  `README.md` records it. An interruption immediately after the rename leaves a
  complete quarantine over an **absent** task root, and `doTxnOpen` requires
  `some Root.rid`, so no transaction can be opened on that disk. The catalogue's
  answer is the reaper, which is a sweep rather than a transaction over the task
  root — so `FN-21.b` and `FN-21.c` are the first commands in this file that
  will run **outside** the phase machine entirely. By the cost law's newest form
  (below) that is the expensive shape, and it is worth deciding before writing.
- **`doRootNameTaken` exists**, narrowly guarded on *the task root absent and
  the quarantine holding one*. It is the world occupying the task-root name, and
  it is already the antecedent `FN-22.h` needed; a reaper claim about a foreign
  entry at a reserved name may want a sibling of it rather than a widening.
- **The quarantine has no NAME.** `Quar.qRid` is the whole signature and this
  file has no filename grammar, so *the quarantine target is occupied* is one
  condition where the shipped protocol has a per-handle, per-attempt family of
  them. `README.md` flags this as **this leaf's to widen if `FN-21.b`'s cleanup
  manifest needs it** — and `FN-31.d`'s *a marker Grove cannot prove is its own*
  probably does, because ownership without a name is `Slot.owner`'s trick and a
  marker has no attempt to be owned by.
- **`exits` inherits three model-only `why` values, not one.**
  `W14QuarantineOccupied`, `W15CommittedAfterRestore` and `W16ReturnIncomplete`
  are all `Blocked` branches the catalogue diagnoses `RecoveryPending` or
  `OwnershipConflict`, and none of them extends the outcome. `FN-25`'s partition
  is still deliberately unwritten. **`FN-21.c` wants `OwnershipConflict` by
  name**; name it as a `Sys.why` member the way the file already names sixteen,
  and say in the `README.md` why the outcome was not extended. Adding the
  partition here is the false-confidence shape, not a convenience.

**Three things `revalidation-k44` measured that change how this leaf budgets.**

- **The cost law has been pessimistic twice running, and the variable being
  over-counted is DWELL.** `quarantine-k43` corrected `commit-k41` by a factor
  of six; `revalidation-k44` found the corrected form still four times too
  pessimistic — five net (phase, guard) points predicted +53% on the widest
  command and measured **+14%**, because all five sit at phases a trace passes
  through once. The operative form: **budget by the number of STATES OF A TRACE
  at which a transition is enabled**, not by (phase, guard) pairs. **The reaper
  is the first thing in this scope the dwell form predicts will be expensive**,
  because a sweep is enabled at states no phase machine constrains — so it is
  also the third chance to measure the law, and the first chance to see it wrong
  in the *dear* direction. Take the ordering the corpus recommends (a static
  scope switch, then a narrowed antecedent, then a smaller bound); do not take
  the multiplier.
- **The ceiling moved to twelve, and it moved because a debt was paid.** It had
  stood at ten since `witness-k40`. `FN-22.h` lands at twelve and five more
  commands at eleven. A disposal slice that decomposes the forward settle should
  expect the same arithmetic the restoration split produced: **a step inserted
  into a path costs a state to every witness that passes through it**, and
  `witness_FN_03`, `witness_FN_16b`, `witness_FN_18`, `witness_FN_22i` and every
  `FN-22` after-rename command pass through the forward settle.
- **A sixth entry in the bound register**, and it is the one a disposal slice is
  most likely to trip next: **a step that stops being ENABLED at a phase costs a
  state to every witness that closed its lasso on it.** Twelve inherited
  witnesses moved +1 when `doClassify` lost `Classified`. Sweep all
  seventy-odd witnesses; do not sweep only the ones you touched.

**`interruptedMidEvacuation` IS REACHABLE — the debt is paid and this leaf
inherits the result rather than the caveat.** It first lands at eleven states,
which is why no slice before the ceiling moved could have checked it. The
methodological half is worth more than the result: the limit's *stated remedy*
(`FN-22`'s table) was wrong about its own instrument — the honest check was a
**witness** that runs the body up to the disk. **Read every inherited limit for
whether its stated remedy is actually the remedy**, because this scope has now
carried one for three slices that a nine-transition `run` discharged in a
minute.


## Done when

- Every obligation of `FN-21` (`.a` – `.c`) and `FN-31` (`.a` – `.d`) is answered
  by a `check` and its required `witness_` runs, all green under
  `models/run.sh --scope finish --family alloy --no-coverage`, with the finish
  scope's empty-cell count down from twenty-one to fourteen.
- **`FN-31.a` is answered by the instrument the catalogue names** — a witness
  for a reachable source state from which disposal must *replace* rather than
  create or remove a marker, **or** a bounded-unreachability `check` of
  `FN-15.d`'s form over the full scope with its bound and result recorded **per
  lane** — or recorded as a `defer` with the reason. `TODO.finish_process.md` Q3
  asks whether replacement is reachable at all, so **a model that folds the
  transition away answers Q3 by construction** and is the one shape this leaf
  must not take. `commit-k41` answered `FN-15.d`'s identically-shaped obligation
  with witnesses on all three lanes and recorded no `defer`; that is the bar, not
  a precedent that it is always reachable.
- Every check runs at a bound at least as large as the widest first-landing
  bound among its obligation's witnesses, **measured by sweep over every witness
  in the file**, not only the new ones.
- One mutation per obligation, each with evidence that it fires and a note of
  what it left green. `FN-21.a` and `FN-31.c` are *the same machinery asked at
  two grains* — re-enterability of disposal, and an interruption inside the
  replacement resumed — so expect the neighbour-kill trap between them
  specifically, and check what each mutation leaves standing.
- The family `README.md` gains the new bounds, abstractions, witness-bound rows,
  mutation-matrix rows, any retained counterexample, and any Q4 removal-matrix
  row the mutations decide. **`FN-21` and `FN-31` are both *incumbent
  mechanics*, so a mutation breaking one is not a Q4 answer** — the matrix
  stands at five decided rows and `FN-24`/`FN-27` are what could add to it.
- Material observations are appended to Experiment 2 as entry 036.
- `exits` is cut as a sibling **of `handoff-k42`**, under `finish-k8`, carrying
  what is actually left open: `FN-02`, `FN-23` – `FN-30`, `EN-08`, `EN-16`, and
  Q4's removal matrix with the five rows already decided transcribed rather than
  re-derived.


## Notes

**On review.** No sibling in this subtree has cut a `review-prototype` step.
`finish-k8` and `handoff-k42` both named `FN-22`'s table as the plausible
exception, because a table is the shape a mutation cannot falsify row by row;
`revalidation-k44` decided against it, and the reason is reusable here: the table
was written as a **total function** held apart from every transition, with a
check binding every step taken at a point to it, so a deleted row makes the
function partial and the check red. **State the completeness as data and the
mutation discipline reaches it.** A disposal slice cuts one only if it produces a
claim that discipline cannot reach; `FN-31.a`'s bounded-unreachability branch is
the candidate, because an absent instance and an unreachable one report
identically and no mutation distinguishes them.

Do not read the Quint side of Experiment 2, and do not open
`crates/grove-finish/models/*.qnt` if one appears. The independence protocol
holds until both families are green.
