# quarantine-necessity-k79

## Goal

Decide `TODO.finish_process.md` Q1 and Q4's three cleanup rows against a control
for the only no-quarantine strategy the environment table actually permits.
`finish-verdicts-k78` left both at `defer` and commissioned this; it is the last
thing standing between the formal phase and a verdict on 10,366 lines and 31
`unsafe` blocks.

## Context

**What is already settled, and must not be re-derived.**
[`docs/adr/finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md`](../../../docs/adr/finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md)
carries the four questions, their dispositions and the four binding constraints.
Q2 and Q3 are `keep` on witnesses reached under the incumbent, and this leaf does
not reopen them. Q1's pre-registered criterion is **met in full and cannot decide
anything**: it is stated over `relax_EN_03`, a counterfactual-capability control,
and running it to completion returns `delete/replace` for a protocol that needs
the atomic recursive deletion `EN-03` says does not exist. The replacement
criterion is availability-typed and is in
[`semantic-contract.md`](../../../docs/specs/semantic-contract.md), *What the
models must be able to decide*.

**The one measurement that is missing.** The available candidate is **in-place
disposal that is non-atomic**: the task root's contents removed entry by entry at
its own name, with no quarantine rename and no cleanup marker. **No command in
either family runs it** — which is not the same as *cannot express it*, and the
difference is this leaf's first economy. Quint ties the quarantine's existence to
`ATOMIC_DISPOSAL` in one `const` — its true branch replaces `SQuarantineRename`
and every step after it with a single `SDisposeInPlace`, itself one atomic
`settle` ([`finish.qnt`](../../../crates/grove-finish/models/finish.qnt), the
`SRevalBeforeQuarantine` arm and `SDisposeInPlace`) — but `Place = AtRoot |
InWitness | Disposed` is already per-entry and `SDisposeEntry` already removes
entry at a time with a resumption point. **What is missing is a transition, not
expressivity**: one `const`, one or two step arms, and the bookkeeping in
`persistentEffect`, `ALL_STEPS`, `DECLARED_STEPS` and `phaseOf`. Alloy runs no
counterfactual-capability mutation at all. `k65` read the coupling as a fact
about the protocol; it is a fact about the encoding.

**THE STRATEGY DIAL ALONE RETURNS A FALSE GREEN. THIS IS THE LEAF'S WHOLE RISK.**
Built and run as it stands, the retained shared-safety set is green over the new
candidate **for structural reasons**, and the run would read as `delete/replace`:

- `classifiesHonestly` guards both of its failable conjuncts on
  `groveReservationStands(w)` = `w.witness != WNone or w.quar.present`
  ([`finish.qnt`](../../../crates/grove-finish/models/finish.qnt):812, 864-868).
  With no quarantine and the witness gone, both conjuncts are vacuous — so
  `FN-24.a` cannot fail. And with the witness retired **last**, the disk reads
  `Reserved(Published)` throughout and `FN-24.a` does not fail even in the
  interesting order. **The predicted violation is order-dependent and the model
  says so; do not assume it.**
- `FN-32`'s three transaction-side sites are `SCreatePreparing`,
  `SQuarantineRename` and `SCreateMarker`. A candidate with neither quarantine
  nor marker reaches only the first, which it inherits from the incumbent
  unchanged — so `FN-32` is trivial over everything the candidate changes, the
  same defect `finish-verdicts-k78` found in `relax_EN_03`'s retained set and
  could not repair there either.

**So the apparatus is the expensive half and it must be built first.** Two
additions, and both are the kind whose cost this brief warns about:

- **A §*States* member for the partially disposed root** — the candidate's
  analogue of `Reserved(Quarantined)`, e.g. `Reserved(Disposing)`: the root at
  its own name with a document at a reserved name. **This is where the argument
  for `keep` actually lives**: `k65` argued the candidate inadmissible because
  §*States* has no member for a partially removed root, and §*States* is a table
  this experiment authored and already extended once. Deciding whether such a
  member is admissible **is** deciding Q1, and it is a catalogue change with the
  manifest cascade both families pay.
- **An `FN-32` site the candidate can reach.** Alloy solved the mirror problem —
  `MarkerReplace` is *the only `groveActs - Reap` member whose marker mutation is
  gated on ownership*, "so this is where the claim has content"
  ([`finish.als`](../../../crates/grove-finish/models/finish.als):5930). The
  candidate has no marker, so the site has to be the resumed sweep's, which is
  the same question Q4's reaper hole asks. **The two commissions are one
  question**; do not solve them separately.

**And `FN-24.b` is not the rejector, however much it looks like one.** Its
invariant is `ALL_STEPS.forall(s => ... or DECLARED_STEPS.contains(s))`, so
whether the candidate's removal step passes is a modeller's choice about a static
table. That is *a claim stated over a model's own classifier*, which this node
has already been burned by twice.

**Q4's three rows are blocked in three different ways, and the ADR names each.**
Quint's three are one bundled result from `relax_EN_03`, so by the record's own
rule they are counterfactual and supply **no** qualifying cell. Alloy's Q4-6 is a
real available-world `none` bounded by the reaper's ownership-proof hole — which
must be settled one way or the other before the row can be read: either an
obligation is stated over the sweep's ownership proof (a manifest change, both
families, cascading) or the catalogue records the silence and the `none` cells
are annotated as such. Alloy's Q4-7 needs neither: row 45's green is a vacuity
artifact of its own mutation, and what would replace it is a control that
narrows the replace transition away **while keeping an `FN-32` site with
content**. Do not carry the ADR's old universal sentence about "the reaper's
actions" forward; `FN-27` is quantified over a set containing `Reap` and stayed
green, and the true statement is the narrow one.

## Done when

- Both families can express a no-quarantine strategy independently of the
  capability dial, and the non-atomic in-place candidate is run against the
  retained shared-safety set at the incumbent's bounds, with reachable
  antecedents and a kill control for every retained claim asserted.
- Q1 is classified from that run: `delete/replace` if the candidate retains the
  set, `keep` if a runnable control shows it breaking one, `defer` only if the
  run itself is blocked by something this leaf names.
- Q4's reaper-coverage question is settled — an obligation stated, or the silence
  recorded — and the three cleanup rows are classified from the result.
- The ADR is reworked **in place** to what now binds — the grove skill's
  `ADR-FORMAT.md` rule: edit, merge, split or delete, and never append a
  superseding record — and the catalogue, both model READMEs, this brief and the
  implementation brief say one thing. If the verdict changes the ADR's title, the
  slug changes with it and every citation is repointed; `k78` found 31 across 17
  files.
- If the answer is `delete/replace`, the narrowly named `impl` leaf is inserted
  immediately before `collapse-application-k27` and
  `.grove/03-implementation-k3/BRIEF.md`'s *Promoted from `TODO.finish_process.md`*
  paragraph is corrected; if it is `keep`, that paragraph's no-op stays and says
  why on evidence.
- The finish-family commands are green in both families after the changes. The
  whole-repository run is `handoff-audit-k66`'s and is not this leaf's.

## Notes

**Budget before you cut.** This brief's *Three measurements to budget from*
applies in full: an obligation addition opens an empty `(family, obligation)`
cell both families must fill; `models/run.sh --list`'s count is silent about the
expensive kind of change; and the Alloy finish cell is the long pole. If the
`FN-24.a` predicate change or the reaper obligation turns out to be a manifest
change, this leaf is bigger than one session and should decompose rather than
run long.

**Cut a `review-design` beside this leaf if it lands a verdict.** The producer
that decided these questions the first time spent no reviewer and was wrong in
two of four; a session that reverses or confirms that on new evidence of its own
making is the same shape.
