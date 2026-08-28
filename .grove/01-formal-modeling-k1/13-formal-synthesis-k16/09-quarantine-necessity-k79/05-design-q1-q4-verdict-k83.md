# q1-q4-verdict-k83

## Goal

Classify Q1 and Q4's three cleanup rows from what children 1 – 3 ran, rework
[`docs/adr/finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md`](../../../../docs/adr/finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md)
**in place** to what then binds, and leave the catalogue, both model READMEs,
this node's brief and `03-implementation-k3`'s saying one thing.

## Context

**The criterion, availability-typed, from
[`semantic-contract.md`](../../../../docs/specs/semantic-contract.md) *What the
models must be able to decide*:** *a candidate strategy that requires no
capability the environment table records as absent, checked against the
shared-safety set at the incumbent's bounds — retaining every claim classifies Q1
`delete/replace`, and being shown to break one classifies it `keep`. No such
candidate run is `defer`.*

**The retained set is `FN-20`, `FN-24`, `FN-27`, `FN-32`**, at the bounds the
incumbent reached them at. Read the run against **all four**, and read a green
only where the claim had content over the difference being judged — that is the
whole lesson of `relax_EN_03`, whose criterion was met in full and decided
nothing because `FN-32` was trivial over everything the candidate changed.

**What Q2 and Q3 are, and that this leaf does not reopen them.** Both are `keep`
on witnesses reached under the **incumbent** — `FN-15.d`'s witness branch in both
families, `FN-31.a`'s witness in both. Q3 is answered *within* the incumbent, and
only a `delete/replace` on Q1 makes its subject moot; it does not make it wrong.

**Q4's three rows, and the three different reasons they are blocked.** Quint's
Q4-105 – 107 are **one bundled result from `relax_EN_03`**, the
counterfactual-capability module, so by the record's own rule they supply **zero**
qualifying cells rather than three. Alloy's Q4-6 is a real available-world `none`
bounded by the reaper hole `sweep-ownership-k81` settles. Alloy's Q4-7 is a
vacuity artifact of its own mutation. Whatever this leaf writes must keep the
three distinguishable.

**The asymmetry rule, which decides the shape of a `keep`.** A wrong `keep`
retains 10,366 lines and 31 `unsafe` blocks and nothing downstream reopens it; a
wrong `delete` converts a fail-closed refusal into a silent wrong state. So a
`keep` returned here must name **what would reopen it**, and it must not be the
`keep` `finish-verdicts-k65` wrote — one read off a criterion's failure. Missing
evidence has no sign.

## Done when

- Q1 is classified against the availability-typed criterion, citing the commands
  that decide it and the bounds they ran at. `defer` is available only if the run
  itself was blocked by something this leaf names.
- Q4's three cleanup rows are classified from `sweep-ownership-k81`'s result and
  the candidate runs, each row on its own reason, and both READMEs' matrices say
  the same thing as the catalogue's §*Q4* paragraph.
- The ADR is reworked **in place** — `ADR-FORMAT.md`'s rule: edit, merge, split or
  delete, never append a superseding record. If the verdict changes its title the
  slug changes with it and every citation is repointed; `k78` found **31 across 17
  files** when it did this once.
- If the answer is `delete/replace`, the narrowly named `impl` leaf is inserted
  immediately before `collapse-application-k27` — one leaf per model-earned
  transformation, never a generic *simplify finish* bucket — and
  `.grove/03-implementation-k3/BRIEF.md`'s *Promoted from
  `TODO.finish_process.md`* paragraph is corrected. If it is `keep`, that
  paragraph's no-op stays and says why **on the evidence this subtree produced**.
- The finish-family commands are green in both families after every edit, with
  counts and wall times recorded. The whole-repository run is `handoff-audit-k66`'s
  and is not this leaf's.
- A `review-design` beside this leaf is cut if it lands a verdict — the producer
  that decided these questions the first time spent no reviewer and was wrong in
  two of four, and a session that reverses or confirms that on new evidence of its
  own making is the same shape.

## Notes

**Four things `handoff-audit-k66` is still owed and this leaf must not silently
absorb**: the whole-repository run; the root-init phase-ordering repair (a
product change the root-creation verdict did not close); the four product-facing
diagnostic questions; and `docs/formalism-findings.md`'s
`](../adr/bulk-marks-are-not-atomic.md)` link defect, one `../` too many.

**`docs/formalism-findings.md` is append-only.** Its historical mentions of the
deleted `TODO.finish_process.md` stay; what gets corrected is the forward-pointing
annotation at the entry that routed the questions here, if this leaf's verdict
moves it.
