# quarantine-k43


## Goal

Give `crates/grove-finish/models/finish.als` the quarantine and the atomic root
rename into it, and answer `FN-19` and `FN-20` — two obligations, and the three
long-trace transitions this subtree's other children are budgeted against.


## Context

Everything this leaf inherits is in the `handoff-k42` brief above and in
`crates/grove-finish/models/README.md`. Three things are specifically this
leaf's, and all three are consequences of being the child that first removes the
task root:

- **The `evacuationComplete` / `gateEvacuated` divergence goes live here.**
  `evacuationComplete` requires `some Root.rid`; `gateEvacuated` does not. They
  have been written apart since `witness-k40` against the day a step removes the
  root. The rename is that step. Expect `FN-11`'s check to have something to say,
  and do not close the divergence by editing either side to match the other
  without a recorded reason.
- **The forward settle's release of the witness and the manifest is a stand-in
  for disposal, and this leaf replaces it with the rename.** The first question
  to ask is whether that breaks `witness_FN_03` — the retry that settles forward
  on the ticket alone, with no local trace to read. If it does, that is a finding
  about `FN-03`'s witness rather than about the new step.
- **`FN-20` is a *shared-safety* claim stated over the role**, not over the
  quarantine: *no artifact a transaction leaves behind is a receipt for it*. Its
  check must be stated so that a candidate protocol leaving something else behind
  would still be answering it — the quarantine is the incumbent realisation, and
  Q1 is decided against the role. `FN-19` next to it is *incumbent mechanics*.
  Getting that class boundary wrong is a false-confidence incident, not a finding.


## Done when

- `FN-19` and `FN-20` are each answered by a `check` and the witnesses the
  catalogue names — for `FN-19`, an interruption immediately after the rename
  leaving a complete quarantine and an absent task root; for `FN-20`, a leftover
  artifact present while the tree is classified fresh — all green under
  `models/run.sh --scope finish --family alloy --no-coverage`, with the finish
  scope's empty-cell count down from thirty-three to thirty-one.
- Each check runs at a bound at least as large as the widest first-landing bound
  among its own obligation's witnesses, **measured by sweep**, and every
  inherited witness whose last transition this leaf made mutating is re-measured.
- One mutation per obligation, each with evidence that it fires — an existing
  witness re-run under it, still landing.
- The family `README.md` gains: the new transition's cost measured against both
  sentinels (median of three), the new bounds, the new abstractions, the
  witness-bound table's new and moved rows, the mutation matrix's new rows, any
  retained counterexample, and the removal-matrix row for **the quarantine** if
  its mutation decides one.
- Material observations are appended to Experiment 2 as entry 034.
- `revalidation` (`FN-22`) is cut as the next sibling under `handoff-k42`, its
  body carrying what the model's actual shape at that point leaves open.


## Notes

Budget by transitions × the bound they are reachable at. This leaf adds the
dearest single transition in the subtree — a rename reachable only at the far end
of a ten-state trace — so if the file's widest command gets materially dearer,
that number is the datum `revalidation` and `disposal` plan against, and it
belongs in the README whichever way it comes out.
