# obligation-placement-k67

**Reviews:** obligation-placement-k63

## Goal

Attack the placement rule and the six placements it forced, before
`catalogue-disposition-k64` applies the rule and edits the same catalogue.

## Context

The producer's commit carries all of it: the rule and its trade-off in
[`docs/adr/obligations-follow-context-not-artifact.md`](../../../docs/adr/obligations-follow-context-not-artifact.md),
the placements in `docs/specs/semantic-contract.md`, the new claim `FN-32` in
`crates/grove-finish/models/`, and a `models/run.sh` change. The producer's own
running log names every decision and the evidence behind it.

**Three judgement calls carry the weight, and each is stated so it can be
attacked on its own.**

1. **The rule itself.** *An obligation belongs to the scope that can execute its
   context*, ordered by the approved crate dependency direction, with
   *observation, not machinery* and *the joint is for what no crate delivers
   alone*. Three named alternatives were rejected. The sharp question is whether
   the direction test is really checkable — it depends on every term in an
   obligation's text resolving to a `Vocabulary` entry with an owning scope, and
   nothing enforces that the vocabulary is complete.
2. **Narrowing the catalogue's *one decided outcome*.** The *one artifact, three
   contexts* table fixed `Blocked(OwnershipConflict)` for the in-transaction row.
   The producer removed that, on the evidence that `finish.als` **refuses** at
   the same antecedent and `finish.qnt` **blocks**, both green against
   `FN-10.b`. The table exists precisely so a model cannot choose; the producer
   has made one row choose-able and handed the choice to `catalogue-disposition-k64`.
   Is that the honest reading, or has a decided outcome been un-decided to avoid
   landing a false obligation the other way?
3. **`FN-32` as a new shared-safety claim rather than citations.** The argument
   is the class register: the content is proven in `FN-10.b` and `FN-31.d`, both
   *incumbent mechanics*, so neither is evidence about a candidate protocol. The
   counter-argument is that `TT-24.a` already quantifies over every action and a
   new claim is a manifest change — four cells, two families, and one more
   shared-safety claim for Q1 and Q4 to carry.

## Done when

- The rule is either sound or a specific counter-instance is named: an obligation
  the three clauses place somewhere a reader would call wrong.
- Each of the six placements is checked against the rule as written, not against
  the producer's summary of it. The four cross-scope citations are the cheap ones
  to get wrong: each must name an obligation that is genuinely covered in its own
  scope, and `SY-06.b`'s must carry `TT-20`'s narrowing.
- `FN-32`'s two commands are read for the vacuity hazards this corpus names — an
  antecedent nothing reaches, a property that restates its own model, a witness
  that lands for the wrong reason. Mutation 63 and `mutant_unproven_ownership`
  are the producer's own controls; a reviewer should ask what they do **not**
  kill.
- The runner change is read for what it claims: `control_ob`'s extraction, the
  contested-cell condition, and whether *reported, never fatal* is the right
  strength.
- Findings, or an explicit nothing. A review that finds nothing creates no
  integrate leaf and simply retires.

## Notes

**This step was `leaf-insert`ed rather than appended, and the producer recorded
why**: `catalogue-disposition-k64` is chartered to edit the reviewed artifact,
and the node brief says child 2 gates child 3. Reading the producer's diff
against a catalogue k64 had already rewritten is the reconciliation
`references/decompose.md` warns about.

The producer spent no in-session reviewer, so nothing here is a second opinion on
a first one.
