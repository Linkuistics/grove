# quint-statement-shape-k61


## Goal

Restate the three Quint lifecycle properties that `cross-model-replay-k15` found
are green in a shape that cannot fail, so the Quint column carries the evidence
its Alloy sibling does.


## Context

Entry 048 finding 5. Three Alloy findings do not replay into Quint, and in none
of the three is Quint's property false — each is stated in a shape that cannot
discover what Alloy's found:

- **`SY-04.b`** — Quint accumulates `refusalMutated` from **one operation's own
  before/after pair** (`isRefusal(o) and wPost.tree.bytes != wPre.tree.bytes`),
  so a `hand-edit` during an invalid configuration is never attributable to a
  refusal. Alloy states it over the trace, and entry 042 is the counterexample:
  the claim as worded constrains the operator's hands, which no filesystem-hosted
  tool can promise.
- **`SY-10.b`** — discharged by a history flag, `not(hist.silentPark)`, so the
  model never has to write down what the timeout stop *returns*. Entry 041 found
  that the closed outcome set has no member for it.
- **`SY-01.b`** — the catalogue half replays and holds. The instrument half does
  not: Quint's `inv_fail_MUT_SY_01b` fires where both Alloy mutations were
  unsatisfiable. Nothing to restate here; it is listed so the leaf's sweep is
  over all three rather than over the two that need work.

The general rule entry 048 states, and the reason this is one leaf rather than
three: **a property stated over one action's own before/after pair cannot
discover that its claim was quantified too widely; a property stated over the
trace can.** The work is to find every `SY-` property of the first shape, not
only the two already named.

## Done when

- Every `SY-` property is classified as *stated over the trace* or *stated over
  one operation's pair*, and the classification is in the model README.
- `SY-04.b` and `SY-10.b` are restated over the trace, or a declared reason says
  why the restatement is wrong for Quint.
- Each restatement has a control that fires — a mutation, or an environment
  action that falsifies it — so the new statement is not a second unfalsifiable
  green.
- `models/run.sh --scope lifecycle --family quint` is green with coverage
  asserted, and the run line is recorded.
- Entry 048's finding 5 gains the outcome, in place.

## Notes

Do not edit `docs/specs/semantic-contract.md`. Both catalogue findings behind
this leaf are `formal-synthesis-k16`'s to disposition; this leaf changes the
model's evidence, not the claim.

The lifecycle Quint suite is cheap — 3m 40s wall at the recorded figures — so a
full re-run per iteration is affordable here in a way it is not in the other two
scopes.

`TT-24.c` is **not** this leaf's. Entry 048 records that Quint's `inv_TT_24c` is
a transcription of `gateOutcome`'s own branch with no control, against an Alloy
column that declared the same cell an honest gap. The fix may be a control, a
declared gap, or restating `TT-24.c`/`TT-24.d` as `FN-` obligations — and the
third is a catalogue change, so the choice is `formal-synthesis-k16`'s.
