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
