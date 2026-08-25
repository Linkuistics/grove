# finish-k8 — brief


## Goal

Model the complete finish and recovery protocol in Alloy 6, including
hostile/interrupted environmental behaviour: every `FN-` obligation the
catalogue defines answered by an Alloy command, under all three lanes, or by a
declared gap in the family `README.md`.


## Context

`docs/specs/semantic-contract.md` is the sole input, exactly as it was for
`task-tree-k7`: every state, action, outcome, refusal reason, blocked diagnosis
and claim this subtree models is defined there, and nothing here invents a
semantic decision. `TODO.finish_process.md` is what the catalogue was written
*against* — read it for the four questions and the constraints any answer must
hold, not for a specification of the protocol.

The model belongs at `crates/grove-finish/models/`, which the runner already
knows as the `finish` scope. `models/run.sh` exists, is complete, and needs no
extension for this scope — with one exception named under *On the horizon*.

**Two clauses of this brief's own former body are superseded, and by a decision
a human confirmed.** The decomposed leaf asked for *both successful exits*, a
*merge exit*, and *branch/bookmark/worktree ownership*. `model-contract-k5`
put that to the human and recorded the answer in the catalogue's §*Out of
scope*: Grove reads no branch or bookmark, creates no working tree and performs
no integration, so **no merge-and-remove exit is modelled** and `FN-28` states
the single successful exit instead. Do not model a second exit, and do not
reopen it inline — reopening is a brief change plus a rework of
`task-tree-transactions-fail-closed` and the glossary.

**What `task-tree-k7` left this subtree.** The node brief above carries the
runner rule, the cost model, the switch rule and the two bound-vacuity
predictors; all of them apply here unchanged and none is restated. Two things
are specifically this subtree's:

- **Two declared gaps are inherited.** `TT-24.c`'s outcome is
  `Blocked(OwnershipConflict)` with its antecedent *inside a finish or recovery
  transaction*; `TT-24.d`'s subject is the quarantine reaper. Both are declared
  `out-of-bounds` in `crates/grove-task-tree/models/README.md` and named as
  belonging here. This model has the machinery both lack — `FN-25` and `FN-21`
  are exactly their subjects. **But the runner's placement rule sends every
  `TT_`-prefixed command to the task-tree directory**, so a `TT_24c` command in
  a finish model is a placement *failure*, not a cell filled. See *On the
  horizon*.
- **`models/run.sh` is finished and is not this subtree's to rebuild.**
  `names-k33` built it against the whole catalogue, so the `finish` scope needs
  no runner work to be measured — `models/run.sh --scope finish --family alloy
  --no-coverage` reports this scope's empty cells from the first session on.

**The lane is a model parameter, not three models.** Every `FN-` claim is
checked under plain Git, native jj and colocated jj, and a claim that holds on
only some of them is a finding rather than a lane-specific claim. `EN-16` is the
control that makes a collapsed lane visible, and it is the last child's.


## Done when

- `crates/grove-finish/models/finish.als` answers every obligation of `FN-01` –
  `FN-31` with a `check` and its required `witness_` runs, all green under
  `models/run.sh --scope finish --family alloy`, with **coverage asserted** and
  zero empty alloy cells for the finish scope — which is the visible signal that
  the column closed, and the moment `--no-coverage` leaves the README's run line.
- The Alloy-owned assumption mutations that control `FN-` obligations are
  present as their own named commands with the expected result the assumption
  table states: `EN-02` (two-device scope, controls `FN-08`), `EN-09` (a result
  arriving after the classification, controls `FN-15.a`), and the two
  exercise-removals `EN-08` (`crash` removed) and `EN-16` (the lane collapsed to
  one), each run against the named witness sets rather than the whole file.
- **Q4's artifact/transition removal matrix** is recorded in
  `crates/grove-finish/models/README.md` for the alloy family: one row per
  removable artifact or transition — the reserved witness, the evacuation
  manifest, its ready mark, the correlation ticket, the quarantine, the cleanup
  marker, the replace transition, the index image, the recorded anchor, the
  deletion fingerprint — each naming the **first shared-safety obligation** its
  removal breaks under the mutation discipline, or `none`.
- `FN-15.d` and `FN-31.a` are answered by the instrument the catalogue names —
  a witness, **or** a bounded-unreachability `check` over the full scope with
  its bound and result recorded per lane. An unlanded witness satisfies neither
  branch and is a `defer`; recording it as one is a legitimate outcome and is
  what `formal-synthesis-k16` reads for Q2 and Q3.
- `crates/grove-finish/models/README.md` records tool version, bounds per
  command, solver, fairness assumptions, abstractions, deliberate omissions,
  what a green run does not prove, the **witness bound at which each obligation
  first lands**, every declared gap, and the retained counterexamples.
- One mutation per reported obligation, run before the green is believed, with
  **evidence that each mutation actually fires** — `selection-k34` and
  `ownership-k38` each produced a mutation the model's own facts made
  unsatisfiable, which reports exactly as a survivor does.
- Material observations are appended to Experiment 2 as entries 031 onward,
  with the six required fields plus the pre-registration's four additions.


## Decomposition

Sixty-one obligations against `task-tree-k7`'s forty-three, plus a lane
parameter that scope did not have. Five sessions, cut along the **machinery**
each claim group needs rather than along the catalogue's section headings —
which is how `task-tree-k7` cut its level, and the two disagree in three places
worth naming. Each child leaves `finish.als` green for the obligations it
claims and the runner able to say exactly which cells are still empty, so no
child is dead until its siblings land.

1. `entry` — the transaction's entry surface: `FN-01`, `FN-05` – `FN-08`
   (8 obligations). Needs the finish leaf, `confirm`, the closed seven-member
   preflight set, task-root identity pinning, the deletion fingerprint, and the
   quarantine target's device. Every outcome is a refusal, so it needs no
   witness species, no repository mutation and no disposition. Owns `EN-02`.
2. `witness` — `FN-09` – `FN-13` (8). Adds the two witness names, publication as
   exactly one atomic same-directory rename, evacuation, the manifest and its
   ready mark, and `crash` between two filesystem steps.
3. `commit` — `FN-03`, `FN-04`, `FN-14` – `FN-18` (12). Adds the repository: the
   three lanes, the recorded anchor, the attempt identity and the correlation
   ticket, the three dispositions classified from evidence, the rollback licence
   and its exactness, and forward recovery. Owns `EN-09`.
4. `handoff` — `FN-19` – `FN-22`, `FN-31` (19). Adds the quarantine, the atomic
   root rename, the four revalidation points and their ten-row table, disposal's
   re-entrancy, the cleanup marker with its `replace` transition, and the reaper.
   **Expect this one to decompose again**; `FN-22` alone is ten obligations and
   `FN-31` is a nested crash-safe protocol.
5. `exits` — `FN-02`, `FN-23` – `FN-30` (14), plus Q4's removal matrix. Adds the
   `Blocked` partition, the full step-boundary crash sweep, hook suppression and
   the single successful exit. Owns `EN-08` and `EN-16`, both of which control
   obligations spread across the whole file and so run last.

Three places this cut departs from the catalogue's own section order, each for
the same reason — a claim sits with the machinery its **witness** needs, not
with the machinery its statement mentions:

- **`FN-02` is in `exits`, not in `entry`.** Its witness is *a decline followed
  by a later successful attempt*, and no successful attempt exists before the
  quarantine rename settles one (`FN-28`). Modelling it earlier would mean
  weakening the catalogue's witness to reach it.
- **`FN-03` and `FN-04` are in `commit`, not in `entry`.** Both are stated about
  intent and both are witnessed by settling forward on a ticket, which is the
  disposition machinery.
- **`FN-31` is in `handoff`, not beside the recovery claims it is printed
  among.** Its subject is the cleanup marker, which is disposal's.

Only the first child is cut now. Each session cuts the next one as its last act,
once the model's actual shape at that point is known — the claim groups are
fixed by the catalogue, but which machinery each needs is not knowable until
the file exists.


## Pointers

- `docs/specs/semantic-contract.md` — §*Claims — finish and recovery* is this
  subtree's whole scope; §*Outcomes* fixes the closed refusal-reason set and the
  two blocked diagnoses; §*The three lanes* is the lane parameter; §*Environment
  assumptions* carries the four mutations this subtree owes and the three
  control classes they fall into; §*What the models must be able to decide*
  fixes which obligations decide each of `TODO.finish_process.md`'s four
  questions, and the **class** register that keeps a shared-safety claim
  separable from incumbent mechanics.
- `crates/grove-task-tree/models/README.md` — the house style for a family
  README in this repository, the two retained false-confidence incidents, the
  mutation matrix's discipline, and the declared-gap line shape the runner
  parses. Read it before writing a command.
- `crates/grove-task-tree/models/task-tree.als` — the house style for a temporal
  Alloy model here: nothing the document merely *claims* is a `fact`, claims are
  named predicates, every action's outcome is a total function of its guard, and
  every command says which assumptions it runs under.
- `TODO.finish_process.md` — the four questions and the *constraints any answer
  must hold*. The interval between removing the task root and recording that
  removal is the whole problem, and it is `FN-24`'s and `SY-05`'s.
- ADRs: `task-tree-transactions-fail-closed`, `supported-workspace-layouts`,
  `grove-does-not-stage-its-own-renames`, `bulk-marks-are-not-atomic`.
- Glossary: *Finish handle*, *Finish-attempt identity*, *Repository anchor*,
  *Deletion fingerprint*, *Entry digest*, *Evacuation manifest*, *Correlation
  ticket*, *Quarantine*, *Complete finish cycle*, *Obligation*.


## What the handoff subtree established, and what `exits` inherits

`handoff-k42` closed with `FN-19` – `FN-22` and `FN-31` complete across three
children — `quarantine-k43`, `revalidation-k44`, `disposal-k45` — taking the
finish scope to **118 commands and fourteen empty alloy cells**, all fourteen
`exits`'. Five things it settled are this node's rather than that subtree's, and
`exits` should not rediscover them.

**The cost law reached a form that stopped needing correction, and it has a
number for a shape this scope had never measured.** `commit-k41` said *budget by
transitions × the bound they are reachable at*; `quarantine-k43` found that six
times too pessimistic and refined it to *(phase, guard) points × the bound*;
`revalidation-k44` found that four times too pessimistic and refined it again to
**the number of STATES OF A TRACE at which a transition is enabled** — a phase a
trace passes through once is one state, however many guards select it.
`disposal-k45` measured the first transition enabled at a phase a trace can
**rest** in, isolated it in three variants of one file, and found the law right:
one sweep at a dwell phase cost **+11%** on the widest command where four
pass-through transitions plus two phases plus a scope dimension cost **+7%**, and
un-narrowing the sweep's guard cost a further **+4%**. **Take the ordering — a
static scope switch, then a narrowed antecedent, then a smaller bound — and do
not take the multiplier**; it has been wrong three times and right once.

**The bound register now carries seven shapes and a third vacuity predictor.**
The seventh is that a step inserted at the **end** of a path costs nothing to a
witness whose final assertion re-anchors onto it, which is *passes through*
sharpened rather than a new shape. The third vacuity predictor is more
consequential and is new in kind: **the file's own witness-bound rule is a floor
that can sit below the real floor**, whenever an obligation's witnesses posit a
disk while its antecedent is a deep transition — `FN-31.c`'s witnesses land at 3
and 4 and its antecedent first occurs at 10, so the rule applied literally would
have made the check green and empty. Read a check's antecedent for the deepest
transition it names and take the larger of the two numbers.

**`Blocked` still carries no diagnosis, and four slices' abstinence is what makes
`FN-25` a finding.** `exits` inherits **four** model-only `why` values its
partition must absorb — `W14QuarantineOccupied`, `W15CommittedAfterRestore`,
`W16ReturnIncomplete` and `W17OwnershipConflict`. The fourth is the only one the
catalogue itself names, and it appears on a **`NoOp`** as well as on a block (the
reaper's decline), which is exactly the case the catalogue warns must not be read
onto a refusal. The `why` set's shape is not the outcome set's.

**Q4's removal matrix has five decided rows and two that cannot be decided from
this subtree.** The five are transcribed rather than re-derived, and the two —
the quarantine's and the cleanup marker's — carry notes saying why: both artifacts
are *incumbent mechanics*, so every mutation that breaks them is evidence about
the incumbent protocol and about nothing else, and the shared-safety claims that
could name them are `FN-24`'s and `FN-27`'s, which are `exits`'.

**`TODO.finish_process.md` Q3 is answered on the Alloy side, and no ADR was
written.** The marker-replacement sub-transaction is reachable — a witness at ten
states, over a source state a second witness reaches at twelve — and the
enumeration Q3 asked for is one class: *a marker left standing by a disposal that
completed its removal and was interrupted before retiring it*. It is recorded in
`crates/grove-finish/models/README.md` and in Experiment 2 entry 036 and nowhere
else, deliberately: the answer is **evidence** for `formal-synthesis-k16` rather
than a decision, it is not hard to reverse, and Q1's counterfactual — disposal in
place under `relax_EN_03`, which is Quint's — would remove the quarantine, the
marker and the replacement together and moot it.

## On the horizon

- **`TT-24.c` and `TT-24.d` cannot be filled from either directory** as the
  placement rule stands: the finish model has their machinery and the task-tree
  model has their prefix. Whether they should be re-stated as `FN-` obligations,
  or the placement rule should admit a cross-scope command, is
  `formal-synthesis-k16`'s to settle. This subtree's job is to make the answer
  cheap — `FN-21.c` and `FN-25` already state the same content under `FN-`
  prefixes, so a re-statement would be a citation change rather than new
  modelling. Say so in the family README rather than leaving the reader to
  rediscover it.
- **The removal matrix is prose the runner does not yet check.** The catalogue
  calls it "a runner obligation like any other: a removable artifact with no row
  fails the run", and `models/run.sh` has no such check — it reads `README.md`
  for declared gaps and for nothing else. Whether `run.sh` grows a matrix reader
  is `exits`' question when it writes the matrix, and a leaf if the answer is
  yes. `exits-k46`'s body carries it as a `Done when`, so it is charged rather
  than merely foreseen.


## Notes

Do not read the Quint side of Experiment 2, and do not open
`crates/grove-finish/models/*.qnt` if one appears. The independence protocol
holds until both families are green.

The catalogue's **class** register is load-bearing here in a way it was not for
the task-tree scope: a candidate protocol is checked against the shared-safety
claims **only**, at the bounds the incumbent reached them at. A child that lets
an incumbent-mechanics claim leak into a shared-safety statement has answered
one of `TODO.finish_process.md`'s questions by construction, which is the shape
of a false-confidence incident rather than a finding.

If the model needs to represent a branch, a bookmark, a worktree or a merge,
that is evidence the *Out of scope* boundary above has been crossed, not that
the model needs a wider signature.
