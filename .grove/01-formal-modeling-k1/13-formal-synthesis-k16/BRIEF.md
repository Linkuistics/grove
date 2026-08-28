# formal-synthesis-k16 — brief


## Goal

Convert the checked models and replay evidence into binding design decisions and an executable hand-off to documentation.



## Context

This is the formal-phase gate. Its conclusions are local to this experiment, not universal claims about Quint, Alloy, or formal methods. Durable evidence belongs in model READMEs and `docs/formalism-findings.md`, not only in this task file.

## Done when

- Both model families and the common runner are green, have non-zero witnesses/checks, and document every bound, assumption, omitted behaviour, and unresolved tool limitation.
- The shared claim catalogue and component/system model READMEs agree on stable semantics, error taxonomy, VCS refinements, filesystem responsibility, and model-to-crate ownership.
- Experiment 2 contains a bounded synthesis: which formalism caught what, what neither established, cost/counterfactual/verdict, useful combined workflow, and concrete changes to design/tests/docs.
- Every proposed finish simplification from `TODO.finish_process.md` is classified keep, delete/replace, or defer with a claim and replayed evidence. “Model is smaller” is not evidence.
- The ordinal lifecycle experiment has a keep/defer/reject decision. If kept, insert an `impl` leaf immediately before `extract-task-tree-k24` using `grove-llm leaf-insert extract-task-tree-k24 ordinal-root-lifecycle --kind impl`.
- For each model-earned finish simplification, insert one narrowly named `impl` leaf immediately before `collapse-application-k27`, preserving the intended execution order. Do not create a generic “simplify finish” bucket.
- Documentation tasks have no unresolved semantic questions and all durable formal artifacts are linked from their component owners.

## Decomposition

The order encodes one dependency and one freeze.

1. **`experiment-synthesis-k62`** — the whole-repository run, recorded, and
   Experiment 2's bounded synthesis with every pre-registered hypothesis and
   measure decided against its own falsifier. **First, because a measurement
   must be frozen before its subject moves**: every later child edits the
   catalogue and cascades commands into both families, and the experiment
   compares the two columns *as independently built*.
2. **`obligation-placement-k63`** — where an obligation lives when its subject spans
   two component scopes. Six recorded instances of the `TT-24` shape, and the
   answer is what *model-to-crate ownership* means for the crate boundary the
   root brief approves. **Gates the next child**, because several inherited
   dispositions are of the form "restate this as an `FN-` obligation".
3. **`catalogue-disposition-k64`** — every remaining inherited catalogue finding,
   decided and landed, with the model commands that keep both families' coverage
   green. Expect this one to decompose by scope; the split is its own to choose,
   because child 2 changes which scope owns what.
4. **`finish-verdicts-k65`** — `TODO.finish_process.md` Q1 – Q4 answered keep,
   delete/replace or defer against the evidence the catalogue pre-committed to,
   the ordinal root-lifecycle verdict contested rather than inherited, and the
   resulting `impl` leaves inserted into `03-implementation-k3`.
5. **`handoff-audit-k66`** — the documentation phase carries no unresolved semantic
   question, and every durable formal artifact is reachable from its component
   owner.

Children 2 – 5 were cut by child 1's session, which is why their bodies carry the
specific inherited items rather than a generic goal sentence.

**A sixth entry sits between 2 and 3, and it is a review step rather than a new
question.** `obligation-placement-k63` cut
`review-design obligation-placement-k67` and **inserted** it at child 3's slot
rather than appending it, because child 3 is chartered to edit the very artifact
under review and this brief says child 2 *gates* child 3 — a gate checked after
everything it gates is not a gate. The keys are unchanged: `k64`, `k65` and `k66`
are still children 4, 5 and 6. If the review finds nothing it creates nothing and
retires.

## Notes

Retire the formal subtree only after the model commands have been rerun from a clean checkout-equivalent state. Review chains should be added here only for decisions whose uncertainty or blast radius warrants an independent session.

## Decisions (running log)

**This leaf is bigger than its brief, and the measurement that says so is the
coupling between the catalogue and the runner.** `models/run.sh` reads its
obligation manifest **out of** `docs/specs/semantic-contract.md` rather than
transcribing it (`run.sh` header, obligation 3; the catalogue's own *Model paths
and the runner*). So a disposition that adds, removes or re-scopes an obligation
is not a documentation edit: it opens an empty `(family, obligation)` cell that
**both** families must fill with a command before any coverage-asserting run is
green again. Several inherited dispositions are of exactly that kind — a
contention member for the closed refusal set (`RGenContended`), `RRolledBack`
and `RConfigInvalid`, restating `TT-24.c`/`TT-24.d` as `FN-` obligations, and a
`PartialScaffold` state-table member for the shipped ambiguity refusal. The
cascade is model work in two families plus a re-run whose Alloy task-tree cell
alone costs 1 h 57 m wall.

The size, measured rather than impressionistic: **97 handoff sites across 11
files name `formal-synthesis-k16`** and hand it a disposition. Controlled in
both directions — a live sibling handle (`cross-model-replay-k15`) finds 14
sites in the same command, an invented handle (`formal-synthesis-k99`)
finds 0 — so the figure is neither a broken grep reading clean nor a
loose pattern matching everything. Against that, the leaf's own `Done when`
carries seven items, of which one (the whole-repository green run) is a ~3 h
background measurement and one (the catalogue disposition) is a pass over a
1,537-line spec that cascades into 30k lines of models.

Decomposed with `grove-llm leaf-decompose`. The children and their order are in
the brief's *Decomposition*.

**The first child is the experiment synthesis, and the ordering argument is that
a measurement must be frozen before its subject moves.** Experiment 2 compares
**the two columns as they were independently built**. Every disposition child
after this one edits the catalogue and cascades commands into both families,
which moves M5's checked-claim counts, M7's run costs and the coverage figures.
Measuring after that would report figures for a third set of models that no
independence protocol ever governed. So the synthesis runs first, on the
artifacts entry 048 read, and the dispositions run against a frozen record.
The whole-repository run was already in flight when this was settled, which is
the same argument from the other end: it is the measurement of the tree as the
formal phase built it.

## `catalogue-disposition-k64` is closed, and this is what its subtree promoted upward

Child 3 retired with every inherited catalogue finding disposed. Its four
children were `closed-sets-k69` (itself a node of two — `routing-and-prose-k73`
and `closed-set-additions-k74`), `task-tree-scope-k70`, `finish-scope-k71`
(reviewed by `finish-scope-k75`, integrated by `finish-scope-k76`) and
`lifecycle-scope-k72`, which carried the whole-node closing sweep. **Children 4
and 5 — `finish-verdicts-k65` and `handoff-audit-k66` — open on a settled
catalogue**, which is what the ordering was for, so what they inherit is stated
here rather than left in a retired subtree.

**The catalogue as frozen.** Refusal reasons **17 → 21**:
`DeletionNotCommitted`, `ConfigurationInvalid`, `GenerationContended`,
`ScaffoldIncomplete(class)`. Outcomes **6 → 6** — `Stopped` was refused as a
seventh outcome and granted as a reason instead. Blocked diagnoses **2 → 2**, but
`FN-25.a` no longer claims they are disjoint: the definitions overlap reachably
and the claim is now that the **carried** diagnosis is the one precedence selects.
Obligations **128 → 130** (`FN-29` gained `.a`/`.b`; `TT-17` split), which
`models/run.sh --list` prints. §*States* gained `Reserved(Quarantined)`, moved
the whole reserved class ahead of `Absent`, and split the witnessless root into
`PartialScaffold(Exact)` and `PartialScaffold(Ambiguous)`. §*Actions* gained the
`validate-config` row it had always been short.

**Five records carry the decisions**, and a session here reads the ones its
question touches rather than all five:
[`a-refusal-leaves-nothing-standing`](../../../docs/adr/a-refusal-leaves-nothing-standing.md)
(what separates `Refused` from `Blocked`, and what a reason names),
[`a-witnessless-root-refuses-what-it-cannot-account-for`](../../../docs/adr/a-witnessless-root-refuses-what-it-cannot-account-for.md),
[`success-is-proved-by-the-ticket-not-the-tree`](../../../docs/adr/success-is-proved-by-the-ticket-not-the-tree.md),
[`a-closed-partition-is-over-outcomes-not-states`](../../../docs/adr/a-closed-partition-is-over-outcomes-not-states.md)
and
[`a-lifecycle-claim-says-what-it-is-over`](../../../docs/adr/a-lifecycle-claim-says-what-it-is-over.md).

**What is still routed, and to whom.** `finish-verdicts-k65` owns items 26, 27
and 30 — `TODO.finish_process.md` Q1 – Q4, the ordinal root-lifecycle verdict
(**already decided** at `root-lifecycle-stays-with-its-receipt`, so no leaf is
inserted before `extract-task-tree-k24`; the narrowed root-*creation* successor
question is what remains), and `FN-13`'s class-register disagreement.
`handoff-audit-k66` owns items 29 and 36 — the crate-facing seams with the
derived Rust tests, and four product-facing diagnostic questions — plus the
whole-repository run from this brief's own `Notes`, and the root-init
phase-ordering repair `task-tree-scope-k70` handed it. Items 31 – 35 name the
**model owners** and no leaf here. One of `k66`'s is sharpened rather than moot:
with nothing on the disk carrying a disposition, *should the reaper re-read the
disposition before disposing* can only be argued as *the marker is an
instruction, not evidence*.

**Three measurements to budget from rather than re-derive.**

- **Manifest-changing has two meanings and only one moves a number.**
  `models/run.sh --list` matches claim headings and obligation bullets, so a
  closed-set addition, a state-table member and a **definition** all leave the
  count unchanged while still costing both families a matching outcome. The cheap
  check is silent about the expensive kind.
- **Budget an Apalache cell alone.** Concurrent cells barely contend until one of
  them is `QUINT_VERIFY=1`, which is heap- and CPU-hungry in a way the simulator
  is not; `finish-scope-k71` measured ~10 minutes of pure contention.
- **A one-bit field on an Alloy signature is not a one-bit change to a bounded
  temporal search.** The lifecycle Alloy cell went from 4m 27s to **12m 20s** at
  a slightly *lower* command count, because `Proc` gained one `var lone Flag`.

**Two method results worth carrying into `k65` and `k66`'s own instruments.**

- **A sweep whose report lives inside its own subject measures itself.**
  `formal-synthesis-k99`, the invented handle three sessions used as a negative
  control, went from 0 sites to 1 the moment a durable record said it found 0;
  and writing *31 sites* makes it 32. Invent a fresh handle each time, and read
  counts as *of the tree including the paragraph reporting them*.
- **A claim stated over a model's own classifier is a restatement of the gate it
  is about.** Two separate instances landed here — `TT-24.c`'s uncontrolled
  transcription, and a `SY-06.b` history flag that made an existing mutation
  control go green. State the claim over the **disk**, or over whatever the
  contract's own test is over, and keep the neighbour asserted green.

**The provenance rule, because this node broke it three times.** A run whose
subject moved under it is not a measurement of the file it reports on, however
harmless the edit looks — and `models/run.sh` reads the catalogue as its manifest
and each scope README for its `GAP` lines, so all of them are subjects. Freeze,
record digests, run, record again. A run line written *after* its run is a record
of a run rather than a moved subject, and the check that tells the two apart is
the GAP-line count either side.

## `finish-verdicts-k65` is closed, and this is what `handoff-audit-k66` inherits

`finish-verdicts-k65` answered `TODO.finish_process.md` Q1 – Q4 **keep**, upheld
the ordinal root-lifecycle rejection and extended it to root *creation*, and
**inserted no `impl` leaf at either target**. **Two of those four verdicts did
not survive review — read the `finish-verdicts-k78` section at the end of this
brief before acting on anything below.**

**It did cut a review, and inserted it at `handoff-audit-k66`'s slot rather than
appending it** — the same move `obligation-placement-k63` made, for the same
reason. `review-design finish-verdicts-k77` now holds position 07 and `k66` has
shifted to 08; the keys are unchanged. The argument for insertion is that `k66`
is chartered to certify *the documentation phase carries no unresolved semantic
question*, and an audit that certifies a conclusion a pending review might move
has certified nothing. The doubt written into `k77`'s body is specific and it is
not a general request for a second opinion: `k65` declared a **pre-registered
criterion mis-typed** and its four verdicts follow from that declaration, which
is exactly the self-serving shape a fresh context exists to attack — and `k65`
could spend no in-session reviewer, because the harness it ran under forbade
subagents. If `k77` finds nothing it creates nothing and retires.

**The file is gone.** `TODO.finish_process.md` is deleted and
[`docs/adr/finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md`](../../../docs/adr/finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md)
replaces it — the four questions with their verdicts, the module-by-module cost
table, the four binding constraints, the rejected alternatives and the reopener.
Every citation outside `.grove/` was enumerated and repointed; `k66` should find
**zero** references to the removed file in the durable set, and one deliberate
survivor: `docs/formalism-findings.md`, which is an append-only log and keeps its
historical mentions plus one forward-pointing annotation at the entry that routed
the questions here.

**One pre-existing link defect remains and it is still `k66`'s** —
`docs/formalism-findings.md`'s `](../adr/bulk-marks-are-not-atomic.md)`, one
`../` too many. A 227-link resolve over the files this child touched found that
one and nothing else.

**The manifest did not move and neither did the other two scopes.** Every
catalogue edit here is a table cell or prose; `models/run.sh --list` prints
**130 obligations**, unchanged. `--scope finish --family quint` is **exit 0**,
240 commands, 63 of 63 cells, Q4 matrix 10 of 10, re-run after the one model
change this child made (`relax_EN_03` gained `FN-32`). **The whole-repository run
is still owed and is `k66`'s**, and nothing here has been measured end to end.

**Two things `k66` should not re-derive.**

- **The root-init window stays `k66`'s and the root-creation verdict does not
  close it.** Library-owned creation was rejected on depth, and a lock does not
  survive a crash in any case; the repair the catalogue names — make root
  initialisation's first write a root-init-exclusive one — is unchanged and is
  still a product change.
- **A `defer` was available and was not taken, deliberately.** Q1's criterion is
  genuinely unmet in one place (`FN-24`'s witnesses are not re-run under the
  candidate) and the commission is named in the finish README. It was left unrun
  because completing it could not change the answer: the candidate is admissible
  and unavailable. If `k66`'s audit reads that as an unresolved semantic
  question, it is not one — the ADR carries why.


## `finish-verdicts-k77` and `k78` are closed, and two of the four verdicts moved

`review-design finish-verdicts-k77` attacked the reading that produced `k65`'s
four `keep`s and returned two findings; `integrate-review-design
finish-verdicts-k78` verified both, applied them, and **reclassified Q1 and Q4's
three cleanup rows from `keep` to `defer`.** Q2 and Q3 stand unchanged, on the
review's own finding that each rests on a witness reached under the incumbent.
The record is
[`docs/adr/finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md`](../../../docs/adr/finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md);
the previous slug is gone and every citation outside the retired task bodies was
repointed.

**What decided it, and it is a measurement rather than a re-reading.** `k65`
declared Q1's pre-registered `delete/replace` criterion mis-typed — stated over a
counterfactual-capability control, which measures admissibility — and then read
its failure as `keep`. `k78` completed the criterion instead of abandoning it.
`relax_EN_03` had narrowed the *world* to `ENV_BUDGET = 0` with empty
`ENV_PHASES`/`ENV_KINDS`, so its retained `FN-32` had no foreign artifact to meet
and `inv_FN_24a`'s crash half had no crash; it now differs from `base` in exactly
one `const`, carries `FN-24.a`'s ten per-step crash witnesses over the
candidate's step list, a reached `FN-32` antecedent (317 traces), a kill control
that fires (`mutant_unproven_ownership_under_the_candidate` — the first `FN-32`
kill in the suite under `ATOMIC_DISPOSAL = true`), and `FN-24.b`'s two branch
enumerations in `scenario_march_under_the_candidate`. **All of them land, so the
criterion is met as written — and running it to completion is what shows it
decides nothing, twice over.** Met, it returns `delete/replace` for a protocol
requiring the atomic recursive deletion `EN-03` says does not exist; and it is
satisfiable while `FN-32`, one of its four retained claims, is trivial over
everything the candidate changes, because the candidate removes every artifact
that claim's other sites are about. A criterion with either defect yields no
verdict in either direction, and missing evidence has no sign: `defer`, not
`keep`. **The second defect is `k78`'s in-session reviewer's**, and the leaf's
running log carries every finding it returned with its classification.

**Three things `handoff-audit-k66` must not re-derive, and one it must stop
relying on.**

- **`k65`'s note that "a `defer` was available and was not taken, deliberately"
  is withdrawn.** It rested on *completing the commission cannot change the
  answer*, and completing the commission is what changed it.
- **Two `defer`s with a named commission are settled dispositions, not
  unresolved semantic questions.** The formal brief's own `Done when` admits
  `defer` as one of three classifications, and the commission is written into the
  ADR and into the catalogue's Q4 paragraph rather than left to a reader.
- **The manifest still has not moved.** Every catalogue edit here is prose;
  `models/run.sh --list` prints **130 obligations**, unchanged. `--scope finish
  --family quint` is **exit 0**, **254 commands** (240 before), 63 of 63 cells,
  Q4 matrix 10 of 10. The Alloy column was not re-run and did not change. **The
  whole-repository run is still owed and is still `k66`'s.**
- **The pre-existing `](../adr/bulk-marks-are-not-atomic.md)` link defect in
  `docs/formalism-findings.md` is still `k66`'s** — `k78` touched that file only
  to repoint the ADR slug and to correct the forward annotation that said all
  four questions answered `keep`.

**And one leaf was cut and inserted ahead of `k66`.** The commission is not a
note: `design quarantine-necessity-k79` holds position 09 and `k66` has shifted
to 10, keys unchanged. The insertion argument is the one this brief has now made
three times — `k66` certifies that the documentation phase carries no unresolved
semantic question, and an audit taken before the control that decides Q1 has run
would certify the disposition rather than the answer. If `k79` returns
`delete/replace`, the `impl` leaf this brief's `Done when` contemplates before
`collapse-application-k27` is `k79`'s to insert; if it returns `keep`, the ADR is
`k79`'s to rework back.