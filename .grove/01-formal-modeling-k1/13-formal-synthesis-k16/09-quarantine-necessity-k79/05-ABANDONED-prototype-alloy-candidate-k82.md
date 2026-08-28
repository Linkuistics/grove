# alloy-candidate-k82

## Goal

Give the Alloy column what children 1 and 2 gave the Quint one: an
**available-world** in-place-disposal candidate, `FN-24.a` restated so it can
judge one, and whatever cell the catalogue changes opened. Alloy currently runs
**no** counterfactual-capability mutation at all, so this is a new instrument
rather than a transcription.

## Context

**What the Alloy column has today, and what it does not.** Every `EN_`-named
command in [`finish.als`](../../../../crates/grove-finish/models/finish.als) is
`EN-02`, `EN-08`, `EN-09` or `EN-16`; the assumption table assigns `EN-03`'s
mutation to Quint, one family per row. Q4-5 (the quarantine) reads `none` on an
**argument**, and its `FN-24` line says why in its own words: *"Removing the
quarantine removes `SReservedQuarantined` and the disks that need it in the same
stroke, so the partition stays total and disjoint over what is left."* **That is
a mutation that removes the artifact and the problem together** — it never models
a root emptied in place, so it is not a measurement of this candidate. Read it as
such rather than as an Alloy result about the available world.

**Where the same apparatus defect sits in this file.** `FN_24a`'s conjuncts (c)
and (d) are `classified = SAbsent implies (no Slot.occ and no Quar.qRid)` and
`classified in currentStates implies (no Slot.occ and no Quar.qRid)` (4770-4771)
— the incumbent's two artifacts, in the consequent rather than the antecedent but
to the same effect. `classifiedRaw` (1085) and `earlierThan` (1120) are where a
new §*States* member lands, and both are written as data on purpose: *every arm
is the catalogue's row verbatim*, and the strict precedence exists so that
deleting a pair leaves two survivors rather than silently changing which one
wins. Matrix row 49 is the control that restores the former order and turns
`FN_24a` red — the positive control for any edit here.

**Cost, measured rather than estimated.** 180 commands, **14 m 33 s** at the last
recorded measurement (186 commands after `finish-scope-k71`, unchanged command
set); 27 m 44 s wall when run concurrently with a `QUINT_VERIFY=1` cell, which is
contention rather than cost. Budget the two serially. A `var` field is **not**
free even when no guard reads it — budget a few per cent of the whole file, and
do not apply the static per-atom law to it.

**Java.** `models/run.sh` probes for Java 17+ and the host default is below
Alloy 6's floor; `~/.local/share/jdk/amazon-corretto-21.jdk/Contents/Home/bin/java`
is what the probe finds. A tool that failed to launch reports what a tool that
found nothing reports — obligation 1 of the runner.

## Done when

- `finish.als` expresses stepwise in-place disposal in the **available** world —
  no relaxation of `EN-03` — as its own strategy, with the temporal shape Alloy 6
  is here for rather than a static snapshot.
- `FN_24a`'s conjuncts (c) and (d) carry the same mechanism-neutral form child 1
  landed in the catalogue, and the Alloy column has a control that **kills** the
  restated conjunct under the candidate. A restatement with no kill is not landed.
- Whatever §*States* member and whatever obligation children 1 and 2 landed have
  their Alloy cells filled, or a `GAP` line declaring what this family cannot
  express and why — never an undeclared empty cell.
- Q4-5's row is either re-run against the candidate this leaf builds or annotated
  with what its existing `none` does and does not measure. Its own `FN-24`
  sentence is the evidence that it removes the artifact and the problem together.
- `models/run.sh --scope finish --family alloy` is exit 0, with command count,
  cell count and wall time recorded beside the numbers above, and the Quint cell
  re-run if any shared subject moved.
- Nothing here classifies Q1 or Q4's rows.

## Notes

**The independence barrier is long down** (`cross-model-replay-k15`), so reading
the Quint column is allowed and expected here — but say so where a finding was
carried across rather than reached, because the experiment's unique/overlap count
is already asymmetric in the Alloy direction and every new asymmetry is one more
thing `experiment-synthesis-k62`'s frozen record cannot account for.

**Do not import the Quint column's machinery into a property.** A cell one family
answers while the other declares a gap is where a transcription hides; the runner
reports it, and `TT-24.c` is the instance that already happened.
