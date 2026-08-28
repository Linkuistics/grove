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

## Decisions (running log)

**This leaf stays one session, and the measurement that says so is that no
verdict needs a model run.** The node's other children each proved bigger than
their brief, so the question was asked before the work rather than after: the
expensive half of this leaf is conditional on a `delete/replace` being live —
`Done when` names an Alloy run of the `relax_EN_03` candidate and
artifact-specific Quint removals for Q4-105 – 107, and both are commissioned
only if a verdict turns on them. None does (below), so what remains is design
conclusions over evidence already on disk, one Quint control line, and one
~4m 25s re-run. The one thing that *would* have decomposed this leaf — inventing
a third candidate protocol for Q1 and modelling it in two families — is
deliberately not done here and is not this leaf's charter; the catalogue fixed
the candidates in advance so the answer is read rather than argued.

**Alloy's non-run of Q1's candidate is not a gap, and a control says so.**
The starting-point evidence records *"Alloy did **not** run the candidate"* as
something to close or declare. It is neither: `EN-03`'s row in the assumption
table assigns its mutation to **Quint**, one family per row, and
`crates/grove-finish/models/finish.als` runs **no** counterfactual-capability
mutation at all — `grep -nE '^(run|check) .*EN_' finish.als` returns nine
commands, every one of them `EN-02`, `EN-08`, `EN-09` or `EN-16`
(premise-break and exercise-removal). The positive control is
`crates/grove-task-tree/models/task-tree.als`, which *does* carry
`expect_unreachable_EN_04_promotion_is_never_observed_half_applied` — Alloy's
own assigned counterfactual — so the instrument finds one where the table puts
one. Q1's row is also the only one of the four whose criterion does **not** say
*in both families*, which is the same fact read from the catalogue's side. So
demanding an Alloy `relax_EN_03` is demanding evidence the pre-registration
never asked for, and the honest record is *declared*, not *closed*.

**Q1's criterion is unmet on the recorded evidence, and the shortfall is in the
control rather than in the reading.** `crates/grove-finish/models/finish-controls.qnt`'s
`relax_EN_03` (line 817) asserts six invariants — `FN-20`, `FN-24.a`, `FN-24.b`,
`FN-27.a`–`.c` — and exactly one witness, `wit_FN_28_the_candidate_reaches_its_successful_exit`.
Q1's criterion asks for more than that in two places. It names `TT-24` in the
retained set and nothing there asserts it; and it requires **each of `FN-24`'s
obligations' witnesses reached at a bound no greater than the incumbent's**,
where what is recorded is that the *properties* hold. The task file asked
whether the evidence is about witnesses or only about properties holding, and
the answer read off the control is: only about properties holding, plus the
candidate's own exit. That last one is not nothing — it is what stops the six
greens being green over a world where nothing happens — but it is `FN-28`'s
witness and not `FN-24`'s.

**Q1 — does the quarantine need to exist? KEEP.** Not because the criterion is
unmet — an unmet criterion is a `defer` — but because **no completion of it
could produce `delete/replace`**, and that is a positive answer rather than an
absence of one. `EN-03` is an assumption the models **grant**: *there is no
atomic recursive directory deletion*. `relax_EN_03` does not test a cheaper
protocol against the world; it *adds the missing capability* and asks whether a
protocol that had it would be admissible. The class table says so in as many
words — a counterfactual capability "**adds** a capability, to ask whether a
cheaper protocol is admissible" — so a green run there establishes
**admissibility**, never **availability**, and only availability licenses
removing anything from `src/`. The Alloy column states the same fact twice as a
fact about the shipped protocol rather than about the model:
*"`EN-03` — no atomic recursive deletion — already forces the shipped removal to
take entry by entry"* (`finish.als:1229`), and *"getting rid of it is the one
thing in the protocol that cannot be one move, so it is the one thing an
interruption can leave half-done"* (`finish.als:2269`).

**And the quarantine is forced, by a derivation that cites no incumbent-mechanics
claim.** `EN-03` makes in-place removal multi-step; `EN-08` grants an
interruption between any two steps; and §*States* has **no member** for a
partially removed task root — the members it would fall into (`Legacy`,
`Malformed`, `Current(Spent)`, `PartialScaffold(_)`) each mean something else,
and after the approved breaking change `Legacy` fails closed. That is `FN-24.a`
— *never into a state that is indistinguishable from a different one* — and
`FN-24` is **shared safety**. The only atomic step the environment grants is a
same-directory rename (`EN-01`), so the task root must leave its own name in one
such rename, and the target of that rename **is** the quarantine. This is also
why `finish-scope-k71` had to add `Reserved(Quarantined)` to §*States* at all:
control row 919 removes the member and `FN-24.a` dies, because *a standing
quarantine reads as an ordinary grove*.

**The two shortfalls are recorded as what they are.** `TT-24`'s is repaired
below by naming the claim the finish scope can actually state. `FN-24`'s
witnesses are named as a commission and **not** run, because running them cannot
change the verdict: under the candidate, `FN-24.a`'s witnesses are one crash
point per step of *the candidate's* step list — `SDisposeInPlace` in place of
`SQuarantineRename`, `SCreateMarker`, `SReplace*`, `SDisposeEntry`,
`SRemoveMarker` — so the commission is "`relax_EN_03` gains a
`wit_FN_24a_crash_at_<step>` per candidate step plus `wit_FN_24b_*` for the
candidate's enumerated step list", in **Quint**, at `relax_EN_03`'s own bound.
Named, per `Done when`, and left unrun with the reason attached.

**Q2 — can the three dispositions become two? KEEP**, and the catalogue's own
rule is what decides it rather than judgement. `FN-15.d` is an **either/or**
obligation, and **both families took the witness branch under the incumbent**:
Alloy's `witness_FN_15d_{git,nativejj,colocatedjj}_indeterminate_reached`, first
landing at **9** states each, and Quint's
`wit_FN_15d_indeterminate_on_{git,native_jj,colocated_jj}`
(`finish-controls.qnt:91`–`93`). `Indeterminate` is **reachable on every lane in
both columns**, so the bounded-unreachability branch Q2's delete/replace
criterion requires was never taken against the shipped protocol at all. Three
independent reasons, any one sufficient: the branch was not taken in either
family; where it *was* taken — `relax_EN_05` — the instrument is randomized
simulation and the module's own header says it is "evidence that `Indeterminate`
is unreachable WITHIN 8000 samples at depth 24 — never a proof", where the
catalogue demands "a Quint exhaustive run to the same depth"; and `relax_EN_05`
grants `EN-05` — *no filesystem transaction can include a version-control
commit* — which is Q1's availability problem in the second question. The third
disposition is not a cost the protocol chose; it is the shape of an external
effect Grove cannot make atomic with its own.

**Q3 — is the marker-replacement sub-transaction reachable? KEEP**, and this is
the one question answered *positively* rather than by the failure of a removal
criterion. Both columns land `FN-31.a`'s witness under the incumbent: Alloy's
`witness_FN_31a_the_stale_marker_is_what_an_interrupted_disposal_leaves` reaches
the source state at **twelve** states by running the protocol up to the crash
rather than positing the disk, and Quint's
`wit_FN_31a_a_state_requiring_a_replacement` (`finish-controls.qnt:177`) reads a
history flag the disposal steps set. The catalogue's rule is explicit that
`FN-31.a`'s witness *merely failing to land* would be a `defer`; it did not fail
to land, in either family, so the unreachability branch is contradicted rather
than unestablished. **The enumeration Q3 asked for is one class, not a list** —
a marker left standing by a disposal that completed the removal it authorised
and was interrupted before retiring it — and it falls out of `FN-21.a`'s
re-enterability rather than out of either model's encoding.

**Q4 — what does finish still owe the user? KEEP every row, and the answer to
the question as asked is the classification rather than a removal.** Six rows
name a shared-safety obligation in at least one column and are therefore
**protecting the user**: the reserved witness, the evacuation manifest, its
ready mark, the correlation ticket, the recorded anchor, the deletion
fingerprint. One is `abstracted` in both (the index image). **Three read `none`
in both families** — the quarantine (Q4-5 / Q4-105), the cleanup marker
(Q4-6 / Q4-106, the Alloy row's `TT-24.a` citation having been withdrawn by
`obligation-placement-k68`) and the replace transition (Q4-7 / Q4-107) — so
those three protect **Grove's own intermediate artifacts**, which is
`TODO.finish_process.md` Q4's question answered precisely.

**A `none` is not a licence to remove, and the reason is a defect in what the
matrix can measure rather than in either column's work.** Q4's criterion reads
*can be removed without breaking any shared-safety claim*, but neither `none`
establishes that:

- **Alloy's three are `argument` or a mutation of a neighbour**, and what an
  `argument` row establishes is that **no shared-safety claim NAMES the
  artifact**. `FN-24.a` names no artifact at all — it is stated over *exactly one
  stable state* — and it is precisely the claim the quarantine exists to make
  satisfiable under `EN-03`. So an artifact can be forced by an environment
  assumption plus a claim that names none of it, and the matrix has no cell for
  that.
- **Quint's three are one dial, and the dial is the capability.** Not merely
  "one bundled result with three names", which is what its README warns about:
  `ATOMIC_DISPOSAL` is a single `const` whose true branch replaces
  `SQuarantineRename` and every step after it with one `SDisposeInPlace`
  (`finish.qnt:1707`). Enumerated rather than sampled — of the thirty-four
  instance modules in `finish-controls.qnt`, **`relax_EN_03` is the only one that
  sets it true**, and every other sets it false. So no control removes the
  quarantine while `EN-03` still holds, and none can: in this model the three
  artifacts and the missing capability are the same parameter, **because the
  artifacts exist to compensate for the capability's absence**.

**So the commission Quint's README offers is declined, with the reason.**
"Artifact-specific removals for Q4-105 – 107" would not be mutations of the
incumbent; each would be a **fourth candidate protocol** — in-place *non-atomic*
disposal that keeps the marker, say — which is strictly worse than both existing
candidates (it has the quarantine's resumption problem *and* an observable
partial task root) and which nothing in the catalogue pre-registered. Separating
them would change no verdict here, because Q1 and Q3 keep all three on grounds
no separation touches. Recorded as declined rather than deferred: a `defer`
would imply the evidence would decide something.

**The general finding, which is worth more than the four verdicts and is what an
implementer must not re-derive: a counterfactual-capability control measures
admissibility, and Q1 and Q2's delete/replace criteria were written as if it
measured availability.** Both criteria are satisfiable only under a granted
capability the assumption table itself records as absent — `EN-03` and `EN-05` —
so as pre-registered, neither question could ever have returned
`delete/replace` against the shipped world. That is a defect in the criteria and
not in the evidence, it was invisible until both columns were green and read
together, and it is the reason the four answers are `keep` rather than `defer`:
there is no missing measurement, only a mis-typed one.

**Q1's retained set names `FN-32` where it named `TT-24`, and that is a swap
rather than an addition.** The catalogue left this to whoever answered Q1, and
the answer follows from the placement rule rather than from the verdict:
`TT-24`'s transaction context **is** `FN-32` — `obligation-placement-k63`
retired the letters `c` and `d` into `FN-32` and `FN-21.c` — and a scope above
`TT-` may not cite `TT-24.a` as evidence about an action `TT-` does not admit,
which is why Q4-6's citation had to be withdrawn. A retained set naming a claim
the *mutating* family cannot discharge is a criterion that cannot be met by
construction, and Q1's was one. Nothing is dropped: the claim is asserted where
its context lives, and `relax_EN_03` now carries
`inv_FN_32_ownership_still_proven_under_the_candidate`. **Landed rather than
declared** — the assumption table's own rule is that a family's failure to meet a
row is established by running the attempt, never by costing it in prose, and the
same applies to a retained claim.

**Landed and re-run.** `models/run.sh --scope finish --family quint`: **exit 0**,
**240 commands** (one more than `finish-scope-k76`'s 239), 0 failures, `cells: 63
complete, 0 declared gaps, 0 empty, of 63`, Q4 matrix 10 of 10 rows, 5m 07s wall.
`inv_FN_32_ownership_still_proven_under_the_candidate` **holds**. Provenance
per the node brief: the catalogue, the finish README and both `.qnt` files were
digested before the run and re-digested after — byte-identical either side, GAP
lines 4 both times — and the run line was written afterwards. `models/run.sh
--list` prints **130 obligations**, unchanged, so every catalogue edit here is
manifest-neutral: they are table cells and prose, and the manifest regex matches
only claim headings and obligation bullets. **The whole-repository run is
`handoff-audit-k66`'s** and is not attempted here.

**`FN-13` is shared safety: the register is right and the finish README's
commit-slice note was wrong.** Its role-form is *a transaction never lands its
own in-flight artifacts in the user's history, and blocks rather than proceed if
it would*, which any admissible protocol must have; that it names a concrete
artifact decides nothing, because the register's own rule is that such a claim
names the incumbent realisation of a role. It also sits with the two claims its
own correction was reconciled against — `FN-14` and `FN-29.b`, both shared safety
— and `finish-scope-k71` restated its outcome as `Blocked(RecoveryPending)`,
which is a statement about the outcome discriminator rather than about a step
list. **The consequence is that Q4-1 IS a Q4 answer**, where the note said
neither of rows 14 and 17 yet was; row 14's still is not (`FN-11` is incumbent
mechanics), row 17's is. What the row still does not become is evidence about a
*candidate*, and for the reason already in its cell: row 17 mutates the **gate**,
and a total removal of the reserved witness satisfies `FN-13` vacuously. Q4-1
reads *the reserved witness protects the user, under any protocol that still has
one*, which Quint's Q4-101 reaches independently by a different obligation.

**The ordinal root-lifecycle rejection is UPHELD, and the contest found one
sentence to correct rather than nothing.** The instrument is gone — the prototype
was throwaway — so the contest was whether the argument carries itself, tested
argument by argument. Two of the three are independent of the prototype and
either alone suffices: `FN-20`'s role is mutation-killed in both families and
`FN-03`'s ticket is checked in both (first argument), and `FN-22`'s two
`Committed -> …` departure rows are in the catalogue with witnesses in both
columns (third argument). The second — a caller consulted mid-transaction can
only be refused — was measured in the prototype and nowhere else, and dies with
it; it is not load-bearing. **The sentence that did not survive** is *the
library's whole world is `<root>` and `<root>/..`, so it has nowhere to put a
receipt that is not the leftover*: `<root>/..` **is** in the library's world and
a receipt there would outlive the root. The rejection is stronger without it —
the problem is not where a receipt can sit but what it can attest, and no
filesystem artifact can attest an *external* effect. Corrected in place.

**Root creation is rejected too, on the measurement the ADR asked for.** The
deferred question was whether creation alone earns a place in the library, and
what it would move is: `create_root_unlocked`, **ten** lines; `write_root_brief`
plus `root_brief_body`, **eleven**, entirely domain content that would be passed
back in as bytes; and `tree_format::write_current_last`, **thirty-six**, whose
generic residue is *create a same-directory temporary idempotently and rename it
into place* — about thirty lines of `std::fs` with no algorithm behind them.
Against that the library's interface gains three concepts it does not have:
creating a root at all, a distinguished child arriving at creation rather than
through `promote`, and a consumer-supplied identity token with a publish-last
contract. **The correctness half cannot move either way**: `TT-20`'s load-bearing
clauses are stated over grove's format taxonomy, which the library has no words
for. And library ownership would not close the two-phase window — a lock does not
survive a crash, `EN-06` grants only cooperating serialization, and the catalogue
already routes the real fix (make root initialisation's first write
root-init-exclusive) to `handoff-audit-k66`; the guard release is load-bearing in
the other direction anyway, since it is what lets a second cooperating process
complete a partial root. So the answer is depth, not correctness: **once the
coordinator is removed there is nothing left to hide.** Recorded in
`root-lifecycle-stays-with-its-receipt`, whose opening now carries two exceptions
on separate grounds and whose reopener for creation is a *second consumer*.

**NO `impl` LEAF IS INSERTED, at either target, and that is the whole of this
leaf's effect on the tree.** Four `keep`s earn none before
`collapse-application-k27` — the `Done when` is explicit that `defer` or `keep`
earns no leaf, and there is no generic bucket to fall back on. The upheld ordinal
verdict earns none before `extract-task-tree-k24`. The implementation brief's
order sentence still names both steps; a note there records that each is now a
no-op rather than an omission, because a later session meeting *apply separately
inserted model-proven finish simplifications* with nothing inserted must be able
to tell the two apart.

**`TODO.finish_process.md` is deleted, and what replaces it is
[`docs/adr/finish-layers-are-forced-not-chosen.md`](../../../docs/adr/finish-layers-are-forced-not-chosen.md).**
`handoff-audit-k66` says in as many words that the file still being present when
it runs is a finding about this leaf, so disposal here is deletion and not a
recommendation. The ADR carries everything durable the file held: the four
questions with their verdicts, the module-by-module cost table (10,366 lines,
34% of `src/`, 31 `unsafe` blocks), the four constraints any answer must hold,
the rejected alternatives, and the reopener. Two things were promoted elsewhere
rather than into it — the `tests/lifecycle_cutover.rs` naming note, which is
unrelated to finish, went to `implementation-k3`'s brief with the judgement
attached; and the root brief's pointer now names the ADR.

**The citation sweep was enumerated and controlled, not pattern-swept.** Every
site naming the file was listed and classified, rather than a list of paths being
fixed: **one** was a markdown link that would dangle (`docs/ARCHITECTURE.md`) and
it is repointed; **fourteen** were bare mentions in the two model files and the
finish README, all naming the *questions*, and all now name the ADR; three
documents cited it for a fact it carried (`docs/preservation-baseline.md` for the
cost table, `root-lifecycle-stays-with-its-receipt` for *the interval is the
whole problem*, `docs/ARCHITECTURE.md` for the `TODO.<subject>.md` convention)
and each now cites the ADR. **`docs/formalism-findings.md` is deliberately left
alone**: it is an append-only log whose entries record what each session read at
the time, and rewriting nine of them would falsify the record — so it gains one
annotation, in the log's own `> **[...]**` idiom, at the entry that routed the
questions here. Controls: the same dangling-link pattern finds **14** live markdown links to
`success-is-proved-by-the-ticket-not-the-tree.md`, so it is not a broken
instrument reading clean; searched against `.grove/`, `src/`, `tests/`,
`build.rs`, `Cargo.toml`, `release.toml` and `scripts/` as well, all clean.

**A `review-design` leaf is cut, and inserted rather than appended.** Two facts
made the call rather than a preference for second opinions. The first is the
shape of the central argument: **this session declared a pre-registered criterion
mis-typed, and its four verdicts follow from that declaration** — a conclusion
that licenses itself is what an adversarial read exists for. The second is that
the allowance for an in-session reviewer could not be spent at all: the harness
this session ran under forbids subagents, so absent a leaf the decision would get
**no** fresh-context challenge anywhere. Against that, the blast radius is the
whole implementation phase's finish work, and a wrong `keep` is the permanent
direction — nothing downstream reopens a question answered *keep*.
`grove-llm leaf-insert handoff-audit-k66 finish-verdicts --kind review-design`
put it at position 07 and shifted `k66` to 08, because `k66` is chartered to
certify that the documentation phase carries no unresolved semantic question and
an audit that certifies a conclusion a pending review might move has certified
nothing. Its body carries the specific doubts, the sharpest being that the
*forced-quarantine* derivation rests on a combination — quarantine removed with
`ATOMIC_DISPOSAL = false` — that **no command in either family runs**, and that
this session's reason for that (the dial is one `const`) is a fact about the
model's parameterisation until someone checks it is a fact about the protocol.
