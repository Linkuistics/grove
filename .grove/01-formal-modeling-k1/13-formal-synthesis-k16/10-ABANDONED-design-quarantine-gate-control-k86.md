# quarantine-gate-control-k86

## Goal

Give `FN-22.e` an available kill in the Quint finish column, or record in the
catalogue and the model README that it cannot have one and why. Today the claim
is green because no reachable state in any module can make it false — which is
the shape this node has been burned by four times, and the standard
`honest-classification-k84` set for `FN-24.a` and `FN-28`.

## Context

**Measured, not argued.** `inv_FN_22e_notcommitted_before_the_rename_diverts` is
`not(hist.renamedWithoutCommitted)`, and `renamedWithoutCommitted` is written at
exactly one site — `SQuarantineRename` — which `SRevalBeforeQuarantine` reaches
only on `d == DCommitted`. So the flag is **unreachable in `base`: 0 of 8000
traces**, at `--max-steps=24`, with every environment kind admitted and a budget
larger than any trace can spend. The claim is a fact about the control flow, not
about the protocol. Found by `integrate-review-design honest-classification-k85`
while building `FN-28`'s isolating kills, and externalised rather than absorbed
because it is not one of the five findings that leaf was chartered to integrate.

**The dial that supplies the kill is known, and it was built and reverted once.**
A library `const` — call it `REMOVAL_GATED` — read at `SRevalBeforeQuarantine`:

```
if (d == DCommitted or not(REMOVAL_GATED))
  go(f1, if (ATOMIC_DISPOSAL) SDisposeInPlace
         else if (IN_PLACE_DISPOSAL) SDisposeRootEntry
         else SQuarantineRename)
```

With it off the transaction proceeds to the removal whatever the disposition
says, and `renamedWithoutCommitted` is then reached — measured at **16 of 8000**
traces with `ENV_KINDS = Set(3)`, `ENV_BUDGET = 2`. `k85` built exactly this,
confirmed the kill for `FN-22.e`, and reverted it: the `const` must be added to
**every** module instantiation (2 in `finish.qnt`, 37 in `finish-controls.qnt`),
which is a wide mechanical change to the whole column for a claim outside that
leaf's charter.

**One thing `k85` could NOT get out of the same dial, and it matters to the
`Done when`.** Making `FN-28`'s proven-result operand die there additionally
needs `OApplied` after an ungated removal — the late result must arrive between
`SQuarantineRename` and `SRevalAfterQuarantine`, two adjacent steps — and no
`ENV_PHASES`/`ENV_BUDGET` combination `k85` tried reached it inside 30000
samples. `FN-28` got its kill elsewhere (`mutant_status_classifier`), so this
leaf owes only `FN-22.e`; do not assume the same trace serves both.

**The `Done when` must decide, not describe.** If the dial lands, `FN-22.e` is no
longer green-by-construction and the mutation matrix gains a row. If the search
cannot reach the killing state at the runner's own bound, that is a real limit
and belongs in the README beside the correlation ticket's attempt-binding, which
is already declared there as a conjunct no control can kill — not left as an
unqualified green.

## Done when

- `FN-22.e` either has a control that makes it fail, named in the mutation
  matrix with its trace count, or the model README and the catalogue record that
  it is satisfied by the control flow and say what would control it.
- Whichever way it goes, no other command's verdict moves: the finish/quint cell
  is green with its command count, cell count and wall time recorded, compared
  against the run this leaf starts from, and `models/run.sh --list`'s obligation
  count is stated before and after.
- If the `const` lands, every instantiation carries it and the diff is that
  `const` and its one control — a wide mechanical change is not licence to
  restate anything else.

## Notes

**This is an instrument leaf, not a protocol decision.** Nothing here changes
what Grove does or what the catalogue asks for; it changes whether one claim's
green is evidence. Do not let it grow into a re-reading of `FN-22`.
