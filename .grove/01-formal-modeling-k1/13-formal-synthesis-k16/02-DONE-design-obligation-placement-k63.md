# obligation-placement-k63


## Goal

Decide where an obligation lives when its subject spans two component scopes,
and land the decision — in `docs/specs/semantic-contract.md`, in the runner's
placement rule, or in a declared exception — so that
`catalogue-disposition-k64` can dispose the remaining findings without deciding
this one case by case.

## Context

**This is the parent's `Done when` clause on *model-to-crate ownership*.** It is
a design question about the crate boundary the root brief approves, not a
bookkeeping question about file paths: the runner sends every `TT_`-prefixed
command to `crates/grove-task-tree/models/` and every `FN_`-prefixed command to
`crates/grove-finish/models/`, so an obligation's **identifier prefix is its
crate assignment**. Deciding the prefix decides which crate's model owns the
claim, and therefore which crate must deliver it.

**Six recorded instances, and they are three different failure modes.**

*The cell that cannot be filled from either side.*

1. **`TT-24.c`** — `Blocked(OwnershipConflict)` inside a finish or recovery
   transaction.
2. **`TT-24.d`** — the quarantine reaper's decline.

   `crates/grove-task-tree/models/README.md` declares both `out-of-bounds`
   because the context each names is a finish context. The finish column has
   both machineries — `FN-25` states `Blocked(OwnershipConflict)` inside a
   transaction and `FN-21.c` states the reaper's decline — so **the re-statement
   would be a citation change rather than new modelling**. A `TT_24c` command in
   the finish directory is a placement failure, not a filled cell.

   The Quint column answered both anyway, and entry 048 found that
   `inv_TT_24c` is a transcription of `gateOutcome`'s own branch that **no
   control kills**. Re-verified at the source by `experiment-synthesis-k62`, by
   enumerating every control in `task-tree-controls.qnt` rather than grepping
   for one: the file declares fourteen, of which `TT-24.**d**` has one
   (`inv_fail_EN_13_TT_24d_the_reaper_stops_declining`) and `TT-24.**c**` has
   none. The asymmetry between the two sub-obligations is real, so a fix that
   treats them as one case will be wrong about one of them. So the coverage matrix currently **scores a transcription
   above an honest declaration**, and the instrument rewards the cheaper move.
   That is a fact about the runner's incentive, and it belongs to this leaf
   rather than to a scope.

*The row decided by a mutation and cited to an obligation no local command can
answer.*

3. **Q4 row 6** in `crates/grove-finish/models/README.md` — the **cleanup
   marker**. Mutation row x1 strips `reapable` back to *there is a quarantine*,
   and what the mutation demonstrates is `TT-24` — *Grove never mutates what it
   cannot prove is its own*, which the register lists as **shared safety**. No
   command in that directory can be the row's evidence.

*Clauses imported into the lifecycle scope and checked in neither.*

4. **`SY-06.b`'s ordering clause** — *completed before any format
   classification runs*. `models/system/` reads `partial` and `legacy` as marks
   already made and has no classification step; the order is `TT-18`'s.
5. **`SY-05.b`'s other half** — the catalogue says `SY-05` and `FN-11`/`FN-19`
   SHALL be checked together, and an `FN_`-prefixed command in `models/system/`
   is a placement error. The lifecycle file states the observation; the finish
   file states the steps.
6. **`SY-14`'s operator exit** — *until an operator acts*. `FN-26` names the two
   restorable exits and they are the finish model's; §*Actions* puts operator
   actions outside the admitted set. `models/system/README.md` argues this one
   is **not a gap**, and that argument is part of what this leaf must accept or
   reject.

`models/system/README.md` §*The composition boundary, closed* is the table to
read first — it is written so that a session here need not open `lifecycle.als`.

## Done when

- A stated rule decides, for any obligation, which scope owns it — and the rule
  is checkable rather than a preference. The obvious candidates: *the scope that
  can execute the claim's context*, *the scope that owns the artifact the claim
  names*, and *the claim keeps the prefix of the crate that must deliver it*.
  Whichever is chosen, the six instances above are re-decided under it and the
  three that disagree with today's placement are named.
- Each of the six instances is resolved as **re-state**, **declared gap**, or
  **cross-scope citation**, with the consequence for the runner's coverage
  matrix stated per instance.
- Where an instance re-states an obligation, both families' commands are renamed
  and `models/run.sh --scope <scope> --family <family>` is green with coverage
  asserted for every scope touched. **The runner reads its manifest out of the
  catalogue**, so a re-statement is not landed until both columns answer the new
  identifier.
- The incentive defect is addressed or explicitly accepted: a transcription with
  no control currently fills a cell that an honest `GAP` leaves empty. If it is
  accepted, the reason is recorded where the next reader of the coverage matrix
  will meet it.
- `catalogue-disposition-k64` inherits a placement rule it can apply, not a
  precedent it has to infer.

## Notes

**A review chain is worth considering here and this leaf should decide rather
than default.** The blast radius is the crate boundary — `grove-task-tree`,
`grove-finish` and the `models/system/` joint — and `documentation-k2` and
`implementation-k3` both build on it. Against that, the decision is small and
its evidence is already assembled. `references/execute.md` allows one
in-session reviewer; a scheduled `review-design` leaf is the alternative.

Do not decide the catalogue findings this rule then applies to — those are
`catalogue-disposition-k64`'s. Decide the rule, land the placements it forces,
and stop.

## Decisions (running log)

**The rule is *placement follows the context, not the artifact*, and it is
checkable against the approved crate dependency direction rather than against
taste.** The root brief fixes `grove-task-tree` → `grove-finish` → `grove`
(the application joint), and the three model scopes sit on that order:
`TT-` below `FN-` below `SY-`. Three clauses, applied in order:

1. **Direction.** An obligation may name states, actions, outcomes and artifacts
   from scopes at or below its own, and may **never** name one from a scope
   above it. An obligation naming something from above is misplaced and belongs
   to the **highest scope its text names**.
2. **Observation, not machinery.** A scope above may name a lower scope's
   *observation* — a fact readable at the boundary — and never its *step*. A
   clause naming a lower scope's step (an ordering between two of them, a pair
   of them to be "checked together", an action the upper scope does not admit)
   is not the upper scope's to check: it stays in place only as a declared
   **cross-scope citation** to the obligation that owns the step.
3. **The joint is for what no crate delivers alone.** `SY-` is for a claim no
   single crate can deliver, not for any claim that mentions two. A claim
   entirely inside one crate's execution belongs to that crate however many
   other crates' artifacts it names.

**Why the artifact rule loses, and the argument is a dependency inversion rather
than a preference.** Today `TT-24` is filed where it is because the *artifact* —
a foreign entry at a name Grove reserves — is the task tree's. But two of its
three contexts are `grove-finish`'s, and `grove-finish` depends on
`grove-task-tree`, not the reverse. Filing those obligations under `TT-` asks the
**lower** crate to deliver an **upper** crate's behaviour, which the approved
dependency direction forbids. That is checkable: read the obligation's text, look
each term up in the catalogue's §*Vocabulary*, and take the highest owning scope.
The rule is falsifiable by exhibiting a term whose owner disagrees with the
prefix — which is what all six recorded instances are.

**Translation is the test, and it is what makes the rule more than a filing
convention.** An obligation stated in the wrong scope's vocabulary cannot be
carried verbatim into its owner: `TT-24.c`'s antecedent (*a foreign entry at a
reserved name*) is `FN-21.c`'s reaper-standpoint predicate, and
`crates/grove-finish/models/finish.als` records that using it inside a live
transaction "fires between `QuarRename` and `MarkerCreate` on every ordinary
forward path". So `TT-24.c` transplanted verbatim is **false**, and its
in-transaction form has to be stated over `dgUnclassifiableAtReservedName`
instead. An obligation that changes truth value under translation was never
checkable where it sat.

**Three resolution classes, and the third is not a resting place.**

- **re-state** — the obligation changes prefix. Its letter is retired and never
  reused (the catalogue's own renumbering rule), and the receiving scope answers
  the new identifier in **both** families before the move is landed.
- **cross-scope citation** — the obligation keeps its prefix, and its text names
  the obligation in another scope carrying the clause it cannot check. **A
  citation carries the cited obligation's declared narrowings**, because a
  citation that hides a narrowing manufactures confidence the cited command does
  not have.
- **declared gap** — the family cannot express it, in the existing `GAP` shape.
  A gap declared on **both** families is the catalogue's own signal that the
  obligation is misplaced — the runner already counts those separately and calls
  them "a finding about the catalogue rather than a covered obligation" — so it
  is the trigger for clause 1, not somewhere an obligation may rest.

**Rejected: keep the artifact rule and let both families declare gaps.** It is
self-refuting as a resting place, by the catalogue's own text quoted above. It
also leaves required behaviour checked nowhere while the coverage matrix reports
two families' worth of honest work.

**Rejected: relax the runner's placement rule so a `TT_` command may live in the
finish directory.** The prefix is the crate assignment; relaxing it means the
catalogue stops saying which crate delivers a claim, which is precisely the
parent's `Done when` on model-to-crate ownership. It would also let the two
families place the same claim in different directories, which is the divergence
the per-family coverage rule exists to prevent.

**Rejected: a fourth prefix for cross-scope obligations.** It needs a fourth
model directory with no crate behind it, against the root brief's rule that
models are owned beside the semantic component they constrain. The joint already
has a home — `SY-`/`models/system/` — and clause 3 is what keeps that home from
absorbing every claim that merely mentions two scopes.

**The six instances, re-decided under the rule. Two move; four are declared in
place; none is a declared gap.** The `Done when` predicted three disagreements
with today's placement and the rule yields **two** — the difference is that
instance 3 and instances 4 – 6 keep their scope and gain a *class*, which is a
change of status rather than of placement.

| # | instance | class | consequence for the coverage matrix |
|---|---|---|---|
| 1 | `TT-24.c` | **re-state** → `FN-32` | task-tree loses an obligation; Alloy's GAP line goes with it and Quint's `inv_TT_24c`/`wit_TT_24c` are deleted. Finish gains `FN-32`, answered by both families. Manifest 129 → 128 |
| 2 | `TT-24.d` | **re-state** → `FN-21.c`, which already exists | task-tree loses an obligation; Alloy's GAP line goes, Quint's two commands and the `TT_24d` control go. Finish gains **nothing**: `FN-21.c` already states it, cites it, and is answered by both families |
| 3 | Q4-6 (alloy) | **cross-scope citation** → `TT-24.a` | none. `grove-finish` sits above `grove-task-tree`, so a mutation here may break a lower scope's property; `TT-24.a` is covered in its own scope by both families. The row gains the sub-identity and the class |
| 4 | `SY-06.b`'s ordering clause | **cross-scope citation** → `TT-18`, `TT-20` | none; manifest-neutral. The citation **carries `TT-20`'s declared narrowing** |
| 5 | `SY-05.b`'s other half | **cross-scope citation** → `FN-11`, `FN-19` | none; manifest-neutral. *Checked together* means each half is checked where its subject lives |
| 6 | `SY-14`'s operator exit | **cross-scope citation** → `FN-26` | none; manifest-neutral. `models/system/README.md`'s *not a gap* argument is **accepted**; the residue is declared a limit of the catalogue, since no model claims the operator's own act |

**Instance 2 cost nothing, and instance 1 cost a claim — the brief predicted a
citation change for both and was right about one.** The brief said "the
re-statement would be a citation change rather than new modelling", because
`FN-25` states the in-transaction `Blocked(OwnershipConflict)` and `FN-21.c`
states the reaper's decline. `FN-21.c` is a complete re-statement, down to
citing `TT-24.d` in its own text, and both families already answer it. `FN-25`
is not: it is stated over `Blocked` outcomes **and nothing else**, so it says
which diagnosis a block carries and never that this situation blocks.

**`TT-24.c` was false as literally worded, and the finish column has been
proving the opposite.** `finish.als`'s
`check FN_10b_content_the_discard_cannot_classify_fails_closed` requires
`Sys.res' in Refused` at exactly `TT-24.c`'s antecedent — an unclassifiable
artifact at a reserved name, met inside a transaction — and is green.
`finish.qnt` **blocks** at the same step (`SRemoveWitness`'s discard branch calls
`blockNow(…, "foreign")`) and is also green. Both answer `FN-10.b`, whose text
says only *fails closed*. So two independent columns resolved one sentence in
opposite directions, and `TT-24.c`'s Quint transcription could not see it
because it transcribed its own model's gate. **The catalogue's second row is
therefore narrowed to what both columns agree on**, with the outcome question
handed to `catalogue-disposition-k64` beside the other opposite-resolution items.

**`FN-32` is a new claim rather than a citation, and the argument is the class
register.** The shared-safety half of `TT-24.c` — *a transaction never mutates an
artifact at a reserved name it cannot prove is its own* — is proven twice in
`finish.als`, at `FN_10b`'s `treeSame` and `FN_31d`'s
`(Sys.act' in disposalSteps and markerForeign) implies markSame`. **Both claims
are incumbent mechanics**, so neither is evidence about a candidate protocol,
while fail-closed ownership is a property any admissible protocol must have. Q1
retains `TT-24` and `TT-24.a` survives, but `grove-task-tree`'s model has no
transaction steps in its action set. `FN-32` is that home, classed shared safety,
and `finish-verdicts-k65` inherits one more retained claim for Q1 and one more
candidate first-broken obligation for Q4.

**`FN-32` is controlled in both families, and the control is what says it is not
a transcription.** Alloy: mutation row 63 drops `slotSame` from `doDiscard`'s
else-branch — the discard removes exactly what it just refused to classify —
which **kills `FN_32` and leaves `FN_21c` green**; row x1's mutation kills
`FN-21.b`/`FN-21.c` and leaves `FN_32` green. Quint: `mutant_unproven_ownership`
(`OWNERSHIP_PROVEN = false`) violates
`inv_fail_MUT_FN_32_a_transaction_mutates_what_it_cannot_prove`, verified by
running it. The two obligations read **different flags** —
`hist.mutatedUnproven` is set only at a transaction step, `hist.foreignMutated`
also by the sweep — so neither mutation can kill the other. That separability is
the whole reason `FN-32` is not `FN-21.c` said twice.

**The imported machinery stays in `task-tree.qnt`, and only what nothing reads
went with the obligations.** `TFinishTxn`, `TReap` and `BlockedO` are read by
surviving commands — `wit_TT_24a` accepts either non-mutating outcome, `TT-24.b`
excludes both non-ordinary tags by name, and `relax_EN_13` needs the sweep for
`TT-04` — so the rule does not force their removal: it governs where an
*obligation* is filed, not what a model may contain. `hist.reapWitness` had
exactly one reader, `inv_TT_24d`, and went with it.

**The incentive defect is addressed in the runner, narrowly, and the narrowness
is the decision.** `models/run.sh` now reports a **contested cell** — one family
answering an obligation another declared a gap on — together with whether the
answering family carries a control naming it. It is **reported and never fatal**:
a family may honestly answer what another cannot express, and a control is not
always available, so making it red would train a reader to explain the colour
away, which is the failure the runner's *named subset* rule already exists to
prevent. What it buys is that the next reader of the coverage matrix meets the
fact, which `alloy:gap quint:ok` hides — that line reads as the declining family
being behind, and in the one recorded instance the declining family was right.

**Rejected: require a control for every obligation.** It is the sharper rule and
it is not affordable — 128 obligations across two families — and it is *not the
test that decides placement*, only the evidence a contested cell is read with:
it catches `TT-24.c`, whose transcription no control killed, and misses
`TT-24.d`, whose imported reaper carried a control that fired perfectly well
while still being another crate's action.

**Rejected: print `declined` instead of `gap`.** Cosmetic. It renames the symptom
and leaves the matrix unable to say anything about the answering side, which is
where the transcription is.

**The report is positively controlled rather than assumed to work.** A temporary
`- **GAP** alloy \`SY-08\`` was injected into `models/system/README.md` — an
obligation the Alloy column answers — and `models/run.sh --scope lifecycle` was
run against it, so the new code path fires on a cell that is genuinely contested.
A check that has never been seen to fire is not a check. The injection is
reverted in this session and the clean run is recorded beside it.

**After this leaf there are zero contested cells**, because the only two were
`TT-24.c` and `TT-24.d`. A report that prints nothing is the point of a net.

**A `review-design` leaf is cut, and it is `leaf-insert`ed ahead of
`catalogue-disposition-k64` rather than appended.** The `Notes` asked this leaf
to decide rather than default, weighing a load-bearing artifact against a small
decision with assembled evidence. **The work outgrew that weighing**: the leaf
landed an ADR fixing the crate boundary's placement rule, edits to
`docs/specs/semantic-contract.md`, a **new shared-safety claim** (`FN-32`) in the
finish contract, commands in three model files, and a change to the runner. Two
of those are judgement calls a fresh context should attack — narrowing the
catalogue's *one decided outcome* for the in-transaction row, and introducing a
claim rather than dissolving `TT-24.c` into citations.

`references/decompose.md` says a `review-*` step re-derives, so `leaf-add` is
right wherever it lands. **It does not land harmlessly here**: `leaf-add` appends
after `handoff-audit-k66`, and `catalogue-disposition-k64` is chartered to edit
the very artifact under review, so the reviewer would be reconciling a historical
diff against a catalogue that has moved — the reconciliation that reference calls
visible but real. The node brief also says child 2 **gates** child 3, and a gate
checked after everything it gates is not a gate. So the review is inserted at
`catalogue-disposition-k64`'s slot and runs next.

**The in-session reviewer allowance was not spent.** This session's environment
forbids spawning subagents, and the tree-level chain is the stronger instrument
for a decision with this blast radius in any case: it gets a whole fresh session
against the committed artifact rather than a stripped extract inside this one.

**One process failure, recorded because it cost two hours of measurement rather
than because it changed a conclusion.** Three scope runs were launched and then
invalidated: `models/run.sh` was edited in place while all three were executing
it, and `task-tree.als`/`.qnt` while the task-tree run read them. A bash script
rewritten under a running shell, and a model file rewritten under a running
`exec` loop, are both undefined rather than merely untidy. The runs were killed
and re-launched only after **every** file the runner reads had reached its final
state. The rule that would have prevented it: finish all edits, then measure —
a suite is an instrument, and an instrument you adjust mid-reading has not read
anything.

**The contested-cell report fired on its positive control, and the control found
a defect in the report itself.** A temporary `- **GAP** alloy `SY-08`` was
injected into `models/system/README.md` and `models/run.sh --scope lifecycle`
run against it. The new section printed:

```text
-- contested cells: one family answered what another declared out of reach.
  SY-08  alloy declared a gap; quint answered, and carries a control
-- 1 contested, of which 0 have no control on the answering side
```

**And the same run printed `-- cells: 49 complete,  declared gaps, 0 empty`** —
the gap count blank, in all three scopes. The new block declared local variables
named `declared` and `answered`, which are the coverage counters computed above
it and printed below it. A cosmetic defect, but in a summary line, in an
instrument whose whole job is to report honestly. Renamed to `gapped_by` /
`answered_by` with the reason written beside them, and all three scopes re-run,
because a run recorded against a runner that has since changed is the staleness
this session already paid two hours to avoid.

**The `carries a control` half is verified rather than assumed.**
`models/system/lifecycle-controls.qnt` really does declare
`inv_fail_MUT_SY_08_a_leaf_added_during_the_window_preempts`, so the line's claim
was true and not a false positive from a loose match. The extractor was then
unit-tested over all four control shapes and two non-control shapes: `EN_nn_<OB>`
and `MUT_<OB>` extract, with and without a sub-identity letter
(`inv_fail_MUT_TT_21a_…` → `TT-21.a`); `wit_unreach_EN_08_…` and
`expect_unreachable_EN_11_…` correctly extract **nothing**, because those are
stated over a removed dimension rather than over one obligation; and ordinary
`inv_`/`witness_` commands extract nothing, because they are not controls.

**The runs, re-run by this session against final files rather than quoted.**
Every file the runner reads had reached its final state before any of these was
launched, which is the rule the earlier invalidated set bought.

| run | commands | cells | Q4 matrix | exit |
|---|---|---|---|---|
| `models/run.sh --scope task-tree` | 210 | **82 of 82, 0 declared gaps, 0 empty** | — | 0 |
| `models/run.sh --scope finish` | 413 | **124 of 124, 0 declared gaps, 0 empty** | alloy 10 of 10 (2 `none`, 1 abstracted); quint 10 of 10 (3 `none`, 1 abstracted) | 0 |
| `models/run.sh --scope lifecycle` (control, gap injected) | 166 | 49 complete, 1 declared gap, 0 empty, of 50 | — | 0 |
| `models/run.sh --scope lifecycle` (clean, gap reverted) | 166 | **50 of 50, 0 declared gaps, 0 empty** | — | 0 |

Both families, per scope, with coverage asserted — no `--family` narrowing and no
`--no-coverage` anywhere.

**Exit 0 established by enumeration rather than by the absence of the word
FAIL.** All 23 sites in `models/run.sh` that set `fail=1` were enumerated and
their distinctive diagnostics searched for together — `placement error:`,
`runner error:`, `MISSING SCOPE`, `NO ROW`, *rows that do not resolve*,
*commands naming no obligation*, *declared gaps in BOTH families*,
*not reported by the run*, *failed to run*, *failed to complete*,
*refusing to guess*, and a `FAIL ` line. **Zero hits in all three logs**, and the
pattern was itself controlled against a synthetic three-line input, which it
matched three times — so a pattern that failed to compile could not have produced
the clean read.

**The repository now carries zero declared gaps, and that is a consequence of
this leaf rather than a coincidence.** `experiment-synthesis-k62`'s
whole-repository run reported *256 complete, 2 declared gaps, 0 empty, of 258*,
and those two were `TT-24.c` and `TT-24.d`. With both retired the catalogue
carries **128** obligations and every `(family, obligation)` cell is filled by a
command. The two `- **GAP**` lines still in the corpus are the fenced shape
examples in the task-tree and finish READMEs, which the runner skips by design —
verified by extracting every such line with the runner's own fence rule and
finding **no live declaration in any scope**.

**One correction landed after the runs, and it is provably not a re-measurement.**
`FN-32`'s claim text named *the witness slot, the quarantine, the cleanup marker*
as the reserved names it ranges over, and the quarantine does not belong in that
list: a quarantine no cleanup marker yet authorises is a state the ordinary
forward path passes through between the root rename and the marker's creation, so
the name carries no ownership bit either way. Both families' commands already
covered only the two names that do — the finish README declared it, the catalogue
did not — so the fix is to the claim's words. **`models/run.sh --list` was
captured before and after and diffed byte-for-byte: identical, 128 obligations.**
No command changed, no obligation changed, so the four recorded runs stand.
