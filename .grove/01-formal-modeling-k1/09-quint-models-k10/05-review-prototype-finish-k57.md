# finish-k57

**Reviews:** finish-k12

## Goal

Read the Quint finish/recovery model adversarially and decide whether its green
run is evidence or construction.

## Context

The producer is the adjacent leaf `finish-k12`; the reviewed artifact is the
commit that retires it —
`crates/grove-finish/models/finish.qnt`,
`crates/grove-finish/models/finish-controls.qnt`, the Quint section of
`crates/grove-finish/models/README.md`, and entry 045 of
`docs/formalism-findings.md`.

**The independence barrier still applies.** Do not open any `.als` file, the
Alloy sections of a model-directory `README.md`, or entries 026 – 043.
`cross-model-replay-k15` is where the barrier comes down; this is not it.

**Why this chain exists, and it is not that the artifact is large.**
`cross-model-replay-k15` already reads this model adversarially and re-derives
every finding, so a review that only re-read the claims would duplicate it. Two
things replay will not read, and both are places where a false green and a true
one produce identical output:

1. **The search dial and the twelve `scenario_` instances.** Every witness in
   this column lives in a focused instance, because an unfocused search reaches
   the end of a twenty-step transaction with probability `(1/k)^20`. That is a
   real problem with a defensible remedy — and it is also exactly the shape of
   an instrument that can be narrowed until it passes. The question a reviewer
   owns is whether each witness's ghost is set by something the **protocol**
   does or by something the **scenario's own construction** guarantees. A ghost
   that only a hand edit can set, witnessed in a scenario whose only admitted
   environment action is that hand edit, is a tautology wearing a percentage.
2. **The eight model mutations.** "The obligation was true by construction and
   this dial proves it" and "I mutated the model until the claim died" produce
   the same runner output. `mutant_short_preflight` kills four obligations at
   once; whether that means four obligations rest on one coding habit, or that
   one dial was drawn wide enough to catch four unrelated things, is a judgement
   the producing session is the wrong context to make.

## Done when

- Each of the 129 witnesses is classified: **protocol-established** (the ghost is
  set by a transition the claim is about), **construction-established** (the
  ghost is set by the scenario's own setup and the witness would land whatever
  the protocol did), or **unclear**. Every non-protocol classification is a
  finding, and the count is reported whatever it is.
- The eight model mutations are each judged against the obligation they kill:
  does the mutation remove the mechanism the obligation names, or something
  wider? `mutant_short_preflight` and `mutant_unproven_ownership` are the two
  drawn widest and should be read first.
- The six material findings in entry 045 are each checked against the catalogue
  text they cite, and any that overstates what the model establishes is reported.
  Finding 1 (`EN-01` narrower than the protocol's own steps) is the load-bearing
  one and rests on a reading of "same-directory"; finding 6 rests on a
  counterexample and should be the easiest to confirm or refute.
- The declared abstractions are checked for one thing only: does any obligation
  read a field the abstraction makes vacuous? The index image is already
  declared as such; the question is whether anything else is.
- The Q4 removal matrix's ten rows are checked in the direction no runner can:
  is the obligation each row names actually the **first** shared-safety
  obligation its artifact's removal breaks? Three rows read `none` on the
  strength of one control (`relax_EN_03`) and they are what Q1 will be decided
  on.

## Notes

Inspection only: no runs, no edits, no re-derivation of the model. Findings are
the output, and the paired `integrate-review-prototype` step owns every fix and
all post-fix verification.

Two conclusions are worth stating explicitly if they stand, because a review
that reports only defects leaves the next session unable to tell what was
checked: that the `FN-25.b` instrument (two independent encodings of one
partition, checked for agreement) is sound, and that the step-list encoding
answers `FN-24.b` rather than restating it.
