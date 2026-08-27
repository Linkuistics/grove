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

## Findings

### F1 — High — `TT-24.a` is a counter-instance to the placement rule, and Q4-6 consumes the ambiguity

The ADR says to assign every term in an obligation an owning scope, take the
highest one, and use the joint only when no crate delivers the claim alone
(`docs/adr/obligations-follow-context-not-artifact.md:14`,
`docs/adr/obligations-follow-context-not-artifact.md:23`,
`docs/adr/obligations-follow-context-not-artifact.md:26`). `TT-24.a` then says
*no action* mutates an unprovable entry
(`docs/specs/semantic-contract.md:887`,
`docs/specs/semantic-contract.md:893`), but `action` is not a term with one
owner: the catalogue's own vocabulary partitions it across Tree mutation,
Finish, Lifecycle and Environment groups
(`docs/specs/semantic-contract.md:475`,
`docs/specs/semantic-contract.md:480`).

The landed design uses both incompatible readings. It keeps `TT-24.a` under
`TT-` and says it reaches the other contexts wherever a model admits them
(`docs/specs/semantic-contract.md:899`), while explicitly acknowledging that
the task-tree model does not admit the finish transaction steps
(`crates/grove-finish/models/README.md:2915`). But Q4-6 cites the task-tree
cell as the shared-safety evidence for a finish-scope cleanup-marker mutation,
precisely because no finish command answers it
(`crates/grove-finish/models/README.md:2849`,
`crates/grove-finish/models/README.md:2860`). If *action* means every catalogue
action, the direction/joint clauses do not place the obligation in `TT-` and its
own-scope commands do not cover the quantifier. If the `TT-` prefix implicitly
narrows *action* to task-tree actions, Q4-6 cannot cite that coverage as evidence
about a reaper action. Either reading invalidates one landed placement. The
integration must make scope part of an obligation's checkable text and then
re-decide `TT-24.a` and Q4-6; a scope-local green over a different action set is
not cross-scope evidence about the mutation.

### F2 — High — `FN-32`'s cleanup-marker half has no isolating falsifier, and its witness lands for the slot half's reason

`FN-32` covers two reserved names — the witness slot and cleanup marker
(`docs/specs/semantic-contract.md:1322`,
`docs/specs/semantic-contract.md:1330`) — and the Alloy property has one
conjunct for each (`crates/grove-finish/models/finish.als:5613`). The witness
puts both artifacts beside a `Discard` step
(`crates/grove-finish/models/finish.als:5620`), but `doDiscard` frames the marker
with unconditional `markSame`; it is not a marker-mutating step and cannot probe
the marker ownership gate (`crates/grove-finish/models/finish.als:1844`). The
recorded mutation 63 removes only `slotSame`
(`crates/grove-finish/models/README.md:1860`), so it kills the first conjunct
while saying nothing about the second.

The Quint control does not close that hole: its own README calls
`mutant_unproven_ownership` a bundle control
(`crates/grove-finish/models/README.md:3230`), and the result records `FN-10.b`
and `FN-32` dying through their shared aggregate `mutatedUnproven` flag
(`crates/grove-finish/models/README.md:3408`). Thus both families can stay green
if the cleanup-marker branch is tautological or its ownership gate is wrong,
provided the witness-slot branch still supplies the recorded failure. The new
shared-safety claim is not yet evidence about both artifacts it names. It needs
an isolating cleanup-marker mutation and a witness whose relevant step can
actually mutate that marker, or the claim must be narrowed to the independently
controlled subject.

### F3 — Medium — the contested-cell report labels an incomplete cell “answered,” and the durable runner controls do not exercise the new path

The coverage calculation defines a complete cell as both a property and a
witness; property-only is `NO-WITNESS`
(`models/run.sh:1007`, `models/run.sh:1009`, `models/run.sh:1010`). The new
contested-cell condition instead adds a family to `answered_by` when it has only
`covered_prop`, without requiring `covered_wit`
(`models/run.sh:1031`, `models/run.sh:1032`). A vacuous property whose antecedent
nothing reaches will therefore be reported as “answered” in the very report
introduced to expose false-confidence evidence. A coverage-asserting run will
also fail later, but the evidence statement remains false; a non-coverage run
can print it without that counterweight.

The producer's injected-gap check exercised a fully covered cell, so it could not
expose this branch. No durable control was added: `models/run-controls.sh` still
says every runner obligation must be shown to fail and still enumerates only its
seven earlier controls (`models/run-controls.sh:4`,
`models/run-controls.sh:213`). “Reported, never fatal” can remain a policy only
after the report distinguishes a complete answer from a property-only cell and
the extractor/condition have committed positive and negative controls.
