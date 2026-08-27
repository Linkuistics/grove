# finish-verdicts-k65


## Goal

Answer `TODO.finish_process.md`'s four questions **keep**, **delete/replace** or
**defer** against the evidence the catalogue pre-committed to; contest rather
than inherit the ordinal root-lifecycle verdict; and cut the `impl` leaves each
answer earns.

## Context

**The catalogue fixed the deciding evidence in advance, so these are read rather
than argued.** `docs/specs/semantic-contract.md` §*What the models must be able
to decide* names, per question, the **shared-safety** claims a candidate must
retain, the **incumbent mechanics** it may replace, and the exact observation
that would classify it *delete/replace*. **"The model is smaller" is not
evidence**, and a question whose deciding witness is never reached is **defer**,
not delete.

The state of the evidence at this leaf's cutting, assembled by
`experiment-synthesis-k62` from both model READMEs. **It is a starting point to
verify, not a finding to adopt** — every figure below is quoted from a README
and none was re-derived.

**Q1 — does the quarantine need to exist?** Retain `FN-20`, `FN-24`, `FN-27`,
`TT-24`; mechanics at stake `FN-19`, `FN-21`, `FN-31`.
- *Quint* ran the candidate. `relax_EN_03` retains `FN-20`, `FN-24.a`,
  `FN-24.b`, `FN-27.a`–`.c` — **all hold**, and the candidate's own successful
  exit is reached in 46% of traces.
- *Alloy* did **not** run the candidate. Its Q4 row 5 reads `none` by argument,
  and its own README says so explicitly: *"Nothing here runs that candidate."*
- **Two gaps to close or to declare.** `TT-24` is in Q1's retained set and is
  **not** in Quint's `relax_EN_03` retained list — and it is the placement
  problem `obligation-placement-k63` settles. And Q1's criterion names
  `FN-24`'s obligations' **witnesses** reached at a bound no greater than the
  incumbent's; check that the recorded evidence is about witnesses and not only
  about properties holding.

**Q2 — can the three dispositions become two?** Retain `FN-15`, `FN-25`.
- The criterion is `FN-15.d`'s **bounded-unreachability check** passing for
  `Indeterminate` on a lane, at a bound strictly greater than `FN-15.b`/`FN-15.c`
  first-witness bounds, **in both families**.
- *Alloy* answered `FN-15.d` **by a witness per lane**: `Indeterminate` is
  **reachable** under the incumbent at those bounds. The unreachability branch
  was not taken.
- *Quint* reports `Indeterminate` unreached on every lane under `relax_EN_05`,
  and its README qualifies it exactly: randomized simulation, so a zero count is
  evidence of unreachability *within* 8000 samples at depth 24, **never a
  proof**.
- The catalogue's own rule then decides it, and it should be recorded as decided
  by the rule rather than by judgement.

**Q3 — is the marker-replacement sub-transaction reachable?** Retain `FN-24`.
- *Alloy*: **yes, by witness, at ten states**, and the enumeration Q3 asked for
  is **one class rather than a list** — a marker left standing by a disposal
  that completed the removal it authorised and was interrupted before retiring
  it. The source state is reached, not posited (twelve states).
- *Quint*: `FN-31.a`'s witness **landed**, and not marginally — a state
  requiring replacement is reached in **40.9%** of traces against 28.0% that run
  a disposal to completion, so the replacement is *forced* before disposal can
  finish.
- Both columns therefore contradict the unreachability branch rather than merely
  failing to establish it. **Q3 is answered within the incumbent; Q1 is what
  could make the question moot**, and the finish README says which finding
  decides which is this leaf's call.

**Q4 — what does finish still owe the user?** The removal matrix, per family.
- **The quarantine**: `none` in both (alloy Q4-5, quint Q4-105). Q4's
  delete/replace criterion met.
- **The replace transition**: `none` in both (alloy Q4-7, quint Q4-107).
- **The cleanup marker**: quint Q4-106 `none`; **alloy Q4-6 names `TT-24`** —
  so *not* `none` in both. The Alloy row is decided by mutation row x1 and cited
  to an obligation no command in that directory can answer, which is instance 3
  of the placement shape.
- **Quint's rows 105 – 107 are one bundled result with three names.** No control
  there removes the quarantine while retaining the marker, or the marker while
  retaining the replace transition. Its README says so and names the remedy:
  *commission artifact-specific removals if Q4's decision needs them separated.*
  Deciding whether it does is this leaf's, and commissioning them is real model
  work in both families.

**The ordinal root-lifecycle verdict is `reject`, and this leaf's job is to
contest it, not to transcribe it.** Entry 047 and
[`root-lifecycle-stays-with-its-receipt`](../../../docs/adr/root-lifecycle-stays-with-its-receipt.md)
record two retained counterexamples with separate causes: the library cannot own
the terminal step of destruction, because between the settle rename and disposal
the container's root is `Absent` and the library has nowhere to put a receipt;
and four revalidation points are necessary but not sufficient, closed only by a
caller obligation the library cannot check. The prototype was **throwaway by
construction and is gone**, so the instrument cannot be attacked — entry 047
says in as many words that what a fresh context should contest is the
**verdict**, and that this leaf is chartered to do it. The narrowed successor
question — *creation alone, which needs no coordinator* — is carried forward by
that record.

## Done when

- Q1, Q2, Q3 and Q4 each carry **keep**, **delete/replace** or **defer**, with
  the claim and the replayed evidence, and each verdict names the catalogue rule
  that produced it. Where a verdict is `defer`, the *specific* missing evidence
  is named — which family, which instrument, which bound.
- Where a verdict needs evidence neither column produced (an Alloy run of the
  `relax_EN_03` candidate, artifact-specific Quint removals for Q4-105 – 107),
  the leaf either commissions it and lands it green, or records `defer` with the
  commission named. It does not decide on evidence it wished existed.
- The ordinal root-lifecycle verdict is contested on its own terms and recorded
  as upheld or overturned. **If upheld, no leaf is inserted before
  `extract-task-tree-k24`** and the narrowed successor question stays where the
  ADR put it.
- For each **model-earned** finish simplification, one narrowly named `impl`
  leaf is inserted immediately before `collapse-application-k27`, preserving the
  intended execution order:
  `grove-llm leaf-insert collapse-application-k27 <stem> --kind impl`.
  **No generic "simplify finish" bucket.** A verdict of `defer` or `keep` earns
  no leaf.
- `TODO.finish_process.md`'s fate is decided. The file says to delete it when
  the work lands **or** when the answer is "keep it as it is", in which case the
  reasoning belongs in an ADR. Deleting it is the parent's `Done when` for the
  whole grove; this leaf decides *what replaces it* and where.

## Notes

Insert order matters and the verb does not check it: `leaf-insert` puts the new
leaf at the target's slot and shifts the target up, so inserting two leaves at
the same target puts the **second** one first. Insert in reverse of the intended
execution order, or insert each at the previous insertion.

`TODO.finish_process.md`'s four constraints bind every answer — the interval
between removing `.grove/` and recording that removal, never rewriting history
to clear a block, three symmetric VCS shapes, and the HITL boundary not being
machinery. A `delete/replace` that converts a refusal into a silent wrong state
is not a simplification; that file carries the worked example.
