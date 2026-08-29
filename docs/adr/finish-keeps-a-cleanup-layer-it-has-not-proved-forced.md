# Finish keeps a cleanup layer it has not proved forced

The finish protocol's three nested crash-safe transactions — the finish
transaction, the commit seam, and quarantine cleanup with its marker and
replacement — **all stay**, and the implementation phase changes none of them.
Two of the four questions `TODO.finish_process.md` raised are **decided**; the
other two are **deferred**, and the record says which is which because the
difference is the whole of what a reader needs.

This record replaces `TODO.finish_process.md`, which asked the four questions and
said to delete itself into an ADR once they were answered.

| question | verdict | what decides it |
|---|---|---|
| **Q1 — does the quarantine need to exist?** | **defer** | no run has decided it. The only candidate ever executed needs a capability `EN-03` says does not exist, and the criterion that judges it is blind to the difference it judges; the available candidate has never been run in either family |
| **Q2 — can the three dispositions become two?** | **keep** | `FN-15.d` answered by the **witness** branch in both families: `Indeterminate` is reachable on every lane under the incumbent |
| **Q3 — is the marker-replacement sub-transaction reachable?** | **moot** | it was reachable, and `FN-31.a`'s witness landed in both families — but its only caller was the colocated Git-index auxiliary, which went with the Git lane at `drop-git-lane-k7` ([*jj is the only lane*](jj-is-the-only-lane.md)). The sub-transaction is deleted, so the question has no subject rather than a new answer |
| **Q4 — what does finish still owe the user?** | **keep** for six rows, **defer** for the cleanup layer's three | six of ten removal-matrix rows name a shared-safety obligation. The three that read `none` in both families are the pre-registered evidence for `delete/replace`, and each of the three has a different reason it cannot be read as a licence |

**Keeping the code while deferring the question is the safe side of the
asymmetry, not an answer to it.** A wrong `keep` retains the whole layer and its
`unsafe` blocks and nothing downstream reopens it; a wrong `delete` converts a
fail-closed refusal into a silent wrong state
([`task-tree-transactions-fail-closed`](task-tree-transactions-fail-closed.md)).
So the layer stays while the question is open — and *stays* is not *forced*.

## Q1 and Q4's three rows are deferred permanently, and the layer therefore stays

**The campaign that was going to decide them was stopped on 2026-08-28, by the
human paying for it, and the reason was cost rather than evidence.** The formal
phase had consumed several days of LLM work; the judgement was that its
remaining leaves would not return anything generally applicable, and that the
campaign's transferable output is what it taught about driving an LLM loop rather
than the crate split it was commissioned to justify. `sweep-ownership-k81`,
`alloy-candidate-k82` and `q1-q4-verdict-k83` — the three leaves that would have
supplied the missing evidence and read the verdict off it — were abandoned
unrun, together with the documentation and implementation phases.

**So `defer` is now this record's answer and not a placeholder in it.** Nothing
downstream is waiting on Q1 or on the three cleanup rows; no leaf exists that
will decide them; and the asymmetry above is what makes that a tolerable place to
stop. The layer stays, and the honest reason it stays is that **removing it was
never shown to be safe**, not that keeping it was shown to be necessary.

**What was established before the stop, and is worth more than the verdict would
have been.** The available candidate — stepwise in-place disposal granted no
capability `EN-03` withholds — exists, runs, and is measured in the Quint column
(`crates/grove-finish/models/finish.qnt`, `IN_PLACE_DISPOSAL`). Against it, three
of Q1's four retained shared-safety claims turned out to be **blind to the
difference they were retained to judge**
([`a-shared-safety-claim-names-the-role-not-the-artifact`](a-shared-safety-claim-names-the-role-not-the-artifact.md)),
and the candidate's own disposal was found to delete entries it could not prove
were still the manifest's. A criterion met over a blind claim is the failure mode
this record's *rule* section exists to stop; the campaign met it three times and
caught it three times. **That is the durable result, and it is a result about
instruments rather than about the quarantine.**

**What would reopen this.** Evidence, not appetite: a run of the available
candidate against a retained set in which every member has a
candidate-reachable site and an available kill — which needs the `FN-32` site
`sweep-ownership-k81` was chartered to decide, and the Alloy mirror
`alloy-candidate-k82` was chartered to build. Both are recorded in
`crates/grove-finish/models/README.md` with what they were owed. Anyone
reopening Q1 starts there and not from `TODO.finish_process.md`'s framing.

## The rule this record exists to stop anyone re-deriving

**A counterfactual-capability control measures admissibility, never
availability**, and Q1's pre-registered `delete/replace` criterion is written
entirely in terms of one passing: *disposal-in-place under `relax_EN_03` holding
every retained shared-safety claim, with each of `FN-24`'s obligations' witnesses
reached at a bound no greater than the incumbent's.*

**That criterion was completed rather than abandoned, and completing it is what
shows it decides nothing.** `relax_EN_03` was first reduced to differing from
`base` in exactly one `const` — it had narrowed the *world* to `ENV_BUDGET = 0`
with `ENV_PHASES` and `ENV_KINDS` empty, which is not a narrowing of the
candidate and left its retained set with no antecedent to be about. It now
carries `FN-24.a`'s ten per-step crash witnesses over the candidate's own step
list, `FN-24.b`'s two branch enumerations in `scenario_march_under_the_candidate`,
a reached `FN-32` antecedent and a kill control that fires. Every one of them
lands. **The criterion is met as written, and it is defective twice over:**

- **It is admissibility-typed.** Met, it returns `delete/replace` for a protocol
  that requires the atomic recursive deletion `EN-03` says does not exist.
- **It is satisfiable while one of its retained claims has no content over the
  difference it is judging.** `FN-32`'s transaction-side sites are
  `SCreatePreparing`, `SQuarantineRename` and `SCreateMarker`; the last two are
  unreachable under the candidate by construction, so both the reached antecedent
  and the kill land at the witness slot — a step the candidate inherits from the
  incumbent unchanged. The claim cannot be given content there either, because
  the candidate removes every artifact its other sites are about. A retained set
  cannot classify a difference one of its members is blind to.

Neither of those is an argument about the criterion. Both are what running it
produced, which is the difference between this record and the one it replaces.

**A criterion that decides nothing yields no verdict in either direction.** The
previous reading took the criterion's failure as `keep`; missing evidence has no
sign, and the catalogue's own rule is that *a question whose deciding witness is
never reached is `defer`, not delete*. It is not `keep` either.

**That is not "arguments never decide", and the distinction matters** — five of
the ten rows in Quint's own Q4 matrix are `argument` rows read as evidence, and
this record would be incoherent if it counted an argument pointing at `delete`
and discounted one pointing at `keep`. The reason Q1's argument does not carry it
is specific, and §*Q1* below states it.

**And an impossibility argued from the shape of one `const` is a statement about
the model.** The previous record said no control *can* remove the quarantine
while `EN-03` holds, "because in this protocol the artifacts and the missing
capability are the same parameter". They were the same parameter in
`finish.qnt`: one `const` `ATOMIC_DISPOSAL`, whose true branch replaces
`SQuarantineRename` and every step after it with a single `SDisposeInPlace`,
itself one atomic `settle`. **What was
missing was a transition, not expressivity** — `Place = AtRoot | InWitness |
Disposed` was already per-entry and `SDisposeEntry` already removed entry at a
time — so the true sentence was *no command runs the candidate*, and that the
model had no dial for it was a fact about the model.

**`honest-classification-k80` built the dial and the sentence is no longer true
of the Quint column.** `IN_PLACE_DISPOSAL` grants nothing — every `EN-`
assumption is `base`'s, `ATOMIC_DISPOSAL` included — and selects the strategy:
the published witness emptied one entry at a time at the task root's own name,
the witness released, the root released. Alloy still runs no such candidate and
`alloy-candidate-k82` owes it.

## Q1 — what is decided, and what the deferral is waiting for

**Decided, and it survives the deferral.** Given that a quarantine exists,
everything below it follows. Disposal is multi-step, so it must be re-enterable
(`FN-21.a`); the quarantine sits at a name `EN-13` says a foreign entry may
occupy, so a sweep needs a document proving what is Grove's (`FN-21.b`,
`FN-21.c`); a document recording progress must be advanced without a reader ever
seeing it absent or doubled (`FN-31.b`), which is the replace transition. The
marker protocol is not a layer that grew to protect an intermediate state the
first two could avoid producing.

**The argument for `keep`, and the two things it quietly fixes.** `EN-03` grants
that there is no atomic recursive directory deletion, so removing a task root's
contents takes more than one step; `EN-08` grants an interruption between any two
of them; and the catalogue's §*States* had no member for a partially removed
task root. The only atomic step the
environment grants is a same-directory rename (`EN-01`), so a task root that must
leave its own name in one such rename leaves it into the quarantine. This is the
reasoning the previous record carried, and it is **still the best available
reasoning**. It does not entail `keep`, for two reasons that are checkable rather
than methodological:

- **It argues from a table this experiment authored, and which gained a member
  mid-experiment.** §*States* acquired `Reserved(Quarantined)` during the formal
  phase. *Our table has no row for it* is therefore a statement about the table,
  which is the same move this record forbids two sections up. A candidate that
  proposes `Reserved(Disposing)` — the root at its own name, a document at a
  reserved name — is not refuted by it. **`honest-classification-k80` settled
  that in the catalogue and it settles the leg rather than the question**:
  membership is the `Reserved` class sentence — *an artifact at a name Grove
  reserves says Grove has work outstanding at that name* — so such a candidate
  has the member on exactly `Reserved(Quarantined)`'s terms, and the table
  refuses nothing. What survives is a question about the protocol, not the table:
  a candidate keeping a reserved-name progress document has not removed the
  cleanup layer's document, it has moved it.
- **Its predicted failure was called order-dependent, and the run withdrew
  that.** The reasoning was that `classifiesHonestly` guards both of its failable
  conjuncts on `groveReservationStands` — *a witness or a quarantine is present* —
  so a stepwise disposal retiring the published witness **last** keeps that true
  throughout, and the predicted violation would hold only for a candidate that
  retires the witness first. Two things are now measured. **No available
  candidate can retire the witness first**: after evacuation every entry is
  *inside* the published witness, and `EN-03` denies the recursive removal that
  would take both together, so witness-last is the candidate's only order. And
  the leak is **after** the witness, not before it — between releasing the empty
  witness and releasing the root there is a present task root with no witness, no
  quarantine and a proven-committed finish's disposal outstanding, which classifies
  `Current(*)` in 3410 of 8000 traces with no interruption anywhere
  (`scenario_in_place_march`). What the old guard did was accept that disk, which
  is a defect in the claim rather than a property of the candidate —
  [`a-shared-safety-claim-names-the-role-not-the-artifact`](a-shared-safety-claim-names-the-role-not-the-artifact.md).

**Not decided, and the commission is now half executed.** The candidate that
reasoning aims at — in-place disposal that is *non-atomic*, keeping neither the
quarantine nor the marker — is the only no-quarantine strategy the environment
table actually permits. The commission was stated in two halves because the first
half alone would produce a false green. **`honest-classification-k80` ran both
halves in the Quint column**, and the second half turned out to need something
other than what this record predicted.

1. **The strategy — done, and it cost what was estimated.** `IN_PLACE_DISPOSAL`
   in `finish.qnt` is a `const` independent of `ATOMIC_DISPOSAL` selecting
   stepwise in-place disposal with no quarantine and no marker, reusing the
   per-entry `Disposed` place and
   `SDisposeEntry`'s resumption point: three step arms and the bookkeeping in
   `persistentEffect`, `ALL_STEPS`, `DECLARED_STEPS`, `phaseOf` and a branch list.
   The candidate reaches its own successful exit in 3266 of 8000 traces.
2. **The apparatus — the diagnosis was right and the prescription was wrong.**
   The prediction holds exactly: with no quarantine and no in-tree witness
   `groveReservationStands` is false and `classifiesHonestly`'s two failable
   conjuncts are vacuous, which
   `wit_FN_24a_the_artifact_guarded_encoding_accepts_it` reaches. But what the
   control needed was **not** a §*States* member and **not** a manifest cascade.
   It needed `FN-24.a`'s failable half stated over the **role** — *work
   outstanding over the task root* — instead of over the incumbent's two
   artifacts, which is a defect in a shared-safety claim rather than a gap in the
   state table
   ([`a-shared-safety-claim-names-the-role-not-the-artifact`](a-shared-safety-claim-names-the-role-not-the-artifact.md)).
   **The member is admissible and is not what refutes or rescues anything**: the
   `Reserved` class sentence admits `Reserved(Disposing)` for any protocol that
   leaves a document at a reserved name, and the runnable candidate leaves none,
   so there is no condition such a row could be written over. `models/run.sh
   --list` printed **130** obligations before and after; no cell opened and no
   cascade was paid.

**What the commission still owes**, and it is the half this record's own
`FN-32` paragraph predicted would be hardest:

- **An `FN-32` site the candidate can reach.** Unrepaired. The candidate's site
  can only be the resumed disposal's ownership proof, which is the same artifact
  Q4-6's reaper hole is about — `sweep-ownership-k81`'s, as one question.
- **The Alloy column.** `FN_24a`'s conjuncts (c) and (d) carry the same artifact
  guard (`no Slot.occ and no Quar.qRid`), reached independently, and Alloy runs
  no in-place candidate at all — `alloy-candidate-k82`'s.

**And one reading the commission would have produced has already been closed off.**
Under the repaired claim the candidate is red, and under `relax_EN_03` — the
*counterfactual* candidate, one atomic `settle` with no intermediate disk —
`inv_FN_24a_one_stable_state_under_the_candidate` still **holds**. The admissible
protocol passes and the available one does not, which is the difference this
record's headline rule is about, arriving as a measurement. **It is not yet a
verdict**: `FN-32` is still blind to this candidate, so no run has checked it
against a *complete* retained set, and a red claim under an incomplete set is
evidence about the instrument.

Both families owe an answer. Q1's is the only row of the four whose criterion
does not say *in both families*, which stopped mattering when it went `defer`: an
availability result Grove would act on is owed by both columns.

## Q2 — the third disposition is the shape of an external effect

`FN-15.d` is an either/or obligation — `Indeterminate` reachable by witness, *or*
positively unreachable within a stated bound — and **both families took the
witness branch under the incumbent**: `witness_FN_15d_{git,nativejj,colocatedjj}_indeterminate_reached`
first landing at nine states, and `wit_FN_15d_indeterminate_on_{git,native_jj,colocated_jj}`
in `finish-controls.qnt`. `Indeterminate` is reachable on every lane, in both
columns. The unreachability
branch was taken only under `relax_EN_05` — commit and evacuation as one step —
and even there by randomized simulation, where the catalogue demands an
exhaustive run.

`Recovery pending` exists because neither a commit nor its absence can always be
proven from outside the commit, and no lane escapes it. It is not surface the
protocol chose. **Q2 is `keep` on its own evidence**, and it does not need the
admissibility reading: a reached witness under the incumbent is a fact about the
shipped world.

## Q3 — the replacement is reached, not posited

Both families land `FN-31.a`'s witness. Alloy reaches the source state at twelve
states by running the protocol from the disk an interruption mid-evacuation
leaves, through the rename, the marker and the removal, and crashing before the
marker is retired. Quint reads a flag the disposal steps set, and a state
requiring replacement is reached more often than a disposal runs to completion —
the replacement is *forced before* disposal can finish rather than occurring
occasionally after it.

The enumeration Q3 asked for is **one class, not a list**: a cleanup marker left
standing by a disposal that completed the removal it authorised and was
interrupted before retiring it.

**Q3 is answered *within* the incumbent, and Q1's deferral is what could make it
moot.** It does not make it wrong: the replace transition is reachable in the
protocol Grove ships, which is what Q3 asked. If Q1 later returns
`delete/replace`, Q3's subject goes with the layer.

## Q4 — six rows protect the user, and three cannot yet be read either way

The removal matrix in `crates/grove-finish/models/README.md` names a
shared-safety obligation for the reserved witness, the evacuation
manifest, its ready mark, the correlation ticket, the recorded anchor and the
deletion fingerprint; the index image is `abstracted`; and the quarantine, the
cleanup marker and the replace transition read `none` in both families. So the
answer to *how much of the machinery protects the repository as against Grove's
own intermediate artifacts* is: **six rows protect the user; three are the
cleanup layer, and what a `none` establishes about them is the open question.**
(The six is a reading across both columns rather than a per-family fact: Alloy's
ready-mark row names `FN-10.a`, which is incumbent mechanics, and says of itself
that it is "not yet a Q4 answer". Quint's names `FN-24.a` and is one.)

**The `none`s are real, and none of the three is a licence — for three different
reasons.** The pre-registered rule is that a row reading `none` in both families
is `delete/replace` evidence, and it is not overridden here by a paragraph. It is
read against what each cell actually measures:

- **Quint's three are one bundled result from `relax_EN_03`**, which is the
  counterfactual-capability module. By this record's own headline rule they
  measure admissibility and say nothing about the shipped world, so the Quint
  column supplies **zero** qualifying per-row cells rather than three. Applying
  that rule to Q1 and not to the Q4 rows the same module produced was the previous
  reading's inconsistency, and it is corrected rather than argued around.
- **Alloy's Q4-6 is an available-world mutation and its `none` stands**, bounded
  by a hole the catalogue owns: *no shared-safety obligation in this repository
  constrains the quarantine reaper's ownership proof.* `FN-32` is stated over
  `groveActs - Reap`, with `Reap` excluded on purpose, and the sweep's own
  fail-closed ownership is `FN-21.b`/`FN-21.c`, both incumbent mechanics. **The
  narrow statement is the true one**: `FN-27` — one of Q4's own retained claims —
  *is* quantified over a set containing `Reap` and stayed green under that
  mutation, so the claim set did look at the sweep. What it never asks is whether
  the sweep can prove what it touches. The Quint face is that `OWNERSHIP_PROVEN`
  is a free `const` rather than something the marker's presence derives.
- **Alloy's Q4-7 is neither of those, and needs no coverage argument.** The
  replace transition is a transaction step `FN-32` does examine — it is the *only*
  member of `groveActs - Reap` whose marker mutation is gated on ownership, which
  is where the claim's marker half has any content at all. So row 45, which
  narrows that transition away, removes the claim's content along with it: its
  green is **a vacuity artifact of its own mutation**, not a finding that nothing
  protects the transition.

**So the three rows establish neither verdict, and the disposition is `defer`.**
The commission is Q1's instrument in the **available** world — where Quint has no
cell at all — plus one obligation question that must be settled before it is run:
**either state a shared-safety obligation over the sweep's ownership proof, or
record in the catalogue that the matrix is structurally silent about it and
annotate the `none` cells as such** — and say which, because a matrix whose
`none` cells cannot be read is worse than one row short.

## What keeping it costs, which is the whole of the other side

Re-measured after `drop-git-lane-k7`, which removed the Git-index auxiliary
family and two thirds of the commit seam — **not** by answering Q1 or Q4, but by
deleting the lane those parts existed for.

| module | lines | role |
|---|---|---|
| `src/finish_transaction.rs` | 3,645 | preflight, witness, evacuation, rollback, quarantine handoff, recovery |
| `src/repo/finish_commit.rs` | 608 | the teardown commit seam and its three dispositions |
| `src/finish_cleanup.rs` | 944 | post-commit quarantine disposal |
| `src/finish_cleanup/unix.rs` | 459 | raw `openat` / `renameat2` / `unlinkat` wrappers and their `unsafe` blocks |
| `src/finish_cleanup/reaper.rs` | 34 | lease-owned reaping of orphaned quarantine |
| **total** | **5,690** | plus 6,901 lines of test |

For one operation, run once per grove, at the end. The 2026-08-17 simplification
pass measured this and left it alone as a redesign rather than a
contract-preserving simplification; the formal phase is what was asked to decide
it, and on two of the four questions it has not.

**"The model is smaller" is not evidence**, and no verdict above uses a line
count as one — the cost table is the price of the open question, never an
argument for closing it either way.

## The constraints every answer had to hold

These bound any future proposal too, the commissioned one included, and they are
recorded here because the scoping note that carried them is gone.

- **The interval is the whole problem.** Between removing `.grove/` and recording
  that removal, a later invocation would read a fresh grove. Nothing may
  reintroduce a window where that is observable — which is why the correlation
  ticket rather than the tree is the evidence
  ([`success-is-proved-by-the-ticket-not-the-tree`](success-is-proved-by-the-ticket-not-the-tree.md),
  `FN-03`, `FN-28`).
- **Never rewrite history to clear a blocked state.** An unresolvable outcome
  stays blocked and operator-recoverable, naming the artifact, the recorded and
  observed topology, and the two restorable exits (`FN-26`).
- **Three VCS shapes stay symmetric** — Git, native jj, colocated jj
  ([the VCS seam](../ARCHITECTURE.md#symmetric-vcs-rule), `EN-16`), and a model
  that collapses the lane passes every property check, which is why the collapse
  is exercised rather than assumed.
- **The HITL boundary is not machinery.** `finish-commit` cannot attest that a
  human spoke through an opaque command; it is the deterministic last-moment
  guard, not a substitute for the confirmation contract (`FN-01`, and `EN-15`'s
  counterfactual, under which no obligation strengthens).

## The alternatives, and why each is rejected

| | what it removes | why rejected |
|---|---|---|
| **disposal in place** (Q1's counterfactual candidate, `relax_EN_03`) | the quarantine, the marker, the replace transition — ~3,000 lines and all 31 `unsafe` blocks | needs atomic recursive deletion, which `EN-03` says does not exist. Admissible, unavailable — and *admissible* is now measured, since its criterion is met in full |
| **two dispositions** (Q2's candidate, `relax_EN_05`) | `Recovery pending` and the recovery surface it generates | needs the version-control commit inside the filesystem transaction, which `EN-05` says is impossible |
| **fold the replacement into a branching write** | `marker_replacement.rs`, 960 lines | answers Q3 by construction rather than by reachability, which is a false-confidence incident rather than a finding. Both families reach the source state |
| **declare Q1 and the cleanup rows `keep` on the strength of the counterfactual** | nothing | this is what the record previously did. A criterion shown to be mis-typed yields no verdict in either direction, and a `none` row is overridden by re-measurement rather than by a paragraph |
| **remove one of the three `none` rows on its own, on the matrix's evidence** | one of the quarantine, the marker, the replacement | the rows are real; the three reasons above are why none is yet a licence. Removing on them today would be acting on a counterfactual cell, a claim set that never asks whether the sweep can prove what it touches, and a green produced by the mutation that erased the claim's content |
| **close the questions as `defer` and never commission the control** | nothing | leaves the whole layer standing on an argument, which is the asymmetry `finish-verdicts-k77` named. The commission is cheap next to what it decides |

## What would reopen — or close — this

**Closing:** the commissioned control above, run in both families. That is the
one thing that turns Q1 and the three Q4 rows into a verdict rather than a
disposition, and it is queued in the formal phase rather than left to the reader.
It is now a subtree rather than a leaf — `quarantine-necessity-k79` —
and `honest-classification-k80` has closed the first of its four children. What
remains before a verdict is `FN-32`'s candidate-reachable site with Q4-6's reaper
hole (`sweep-ownership-k81`, one question), the Alloy column
(`alloy-candidate-k82`), and the verdict itself with this record's rework
(`q1-q4-verdict-k83`).

**Reopening, if it closes as `keep`:** a filesystem primitive that makes recursive
removal atomic with respect to namespace visibility, or a version-control lane
whose commit can be made atomic with a filesystem transaction. Either would move
an assumption out of the environment table, and the counterfactual already run
for it becomes evidence about an available protocol rather than an admissible
one.

**The evidence for everything above is now
[`docs/formalism-findings.md`](../formalism-findings.md) entries 026 – 048, and
that is the whole of it.** This section used to name the three model
`README.md`s beside them. All three have been retired: the catalogue was
`docs/specs/semantic-contract.md`, whose §*States* this record argues from, and
it and the `TT-` and `SY-` model directories went with the campaign's apparatus
(`delete-formal-models-k29`), the `FN-` directory `crates/grove-finish/models/`
at `delete-finish-models-k30`.

**Two things this record leaves open therefore stop being open work and become
history.** The reopen condition above asks for a run of the available candidate
against a retained set in which every member has a candidate-reachable site and
an available kill; it named `sweep-ownership-k81` and `alloy-candidate-k82` as
its two prerequisites, and it directed anyone reopening Q1 to
`crates/grove-finish/models/README.md`. There is no model to run, no column to
mirror and no `README.md` to start from, so **Q1 cannot be reopened on the terms
this record states** — reopening it now means building an instrument first, and
the reason to want one is `evidence-outlives-the-instrument`'s stated reopen
condition rather than this record's. What survives unchanged is the *rule* the
section below it exists to stop anyone re-deriving, which is a rule about
criteria and controls and needs no model to state.

The decision survived the instrument that found it, which is the outcome that
campaign was run to test, and `docs/formalism-findings.md` keeps the record of
how it was found.
