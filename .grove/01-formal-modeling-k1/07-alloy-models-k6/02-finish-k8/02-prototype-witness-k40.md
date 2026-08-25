# witness-k40


## Goal

Extend `crates/grove-finish/models/finish.als` to the reserved witness: the
build/publish split, the discardable unpublished witness, evacuation before
deletion, the manifest and its ready mark, and the witness's exclusion from every
candidate commit — `FN-09` – `FN-13`, eight obligations.


## Context

`entry-k39` left the file green for `FN-01` and `FN-05` – `FN-08`, and
`crates/grove-finish/models/README.md` carries the bounds, the abstractions, the
mutation matrix and the four *what a green run does not prove* caveats. **Read
it before writing a command.**

What exists that you can build on:

- **The transaction's entry, in two steps.** `TxnOpen` pins the task root's
  identity and `Preflight` runs the seven gates; a failed gate yields a named
  refusal with a byte-identical tree and repository, and the phase returns to
  `Fresh`. `Preflight`'s success branch sets `Txn.phase = Entered`, and that is
  where this leaf's first step attaches.
- **`Slot`, in the signature and never occupied.** The reserved name a witness
  would be built at is already there, precisely so `FN-05.b` had something the
  preflight could have mutated. Occupying it is this leaf's, and `Slot.occ`
  wants to become the witness's *class* — preparing or published — rather than
  the single `Reserved` marker it is now.
- **A coarse tree.** An entry is an opaque object with a type and a role; there
  is no filename grammar and `FN-09` – `FN-13` should not need one. `FN-12`'s
  digest is the catalogue's opaque equality, so `Entry` gains a digest field and
  nothing that constructs one.
- **The lane, unused.** It is a signature parameter already and no obligation in
  this slice needs to distinguish the three either.

What this leaf has to build, and none of it exists:

- **`crash` between any two filesystem steps.** `entry-k39` has no crash action
  at all — every one of its obligations is a single-step refusal. `FN-09.a`,
  `FN-09.b`, `FN-10` and `FN-12.a` are all *interruption at a named point*, so
  the transaction's steps must become a list with crash boundaries between them.
  That is also the machinery `FN-24.b` will later quantify over, so the step
  list is worth writing as one thing rather than accreting it.
- **Publication as exactly one atomic same-directory rename** (`FN-09.a`), which
  is `EN-01`'s grant and the only atomicity this model may assume.
- **Evacuation** (`FN-11`): every ordinary root entry inside the published
  witness, beneath a verified manifest, before any commit is attempted. The
  witness is *the interval between publication and commit*, which needs a notion
  of a commit having been attempted without this slice modelling the commit —
  `commit-k4x`'s machinery must not be smuggled in to reach it.
- **The manifest, marked ready last** (`FN-12.a`), and the refusal of an
  undigestible entry type before any mutation (`FN-12.b`) — which `FN-05.a`'s
  seventh member already reaches from the preflight, so the two must agree
  rather than duplicate.


## Done when

- Every obligation of `FN-09` – `FN-13` is answered by a `check` and its required
  `witness_` runs, all green under
  `models/run.sh --scope finish --family alloy --no-coverage`, with the finish
  scope's empty-cell count down to forty-five.
- **Every check runs at a bound at least as large as the widest first-landing
  bound among its obligation's witnesses**, measured rather than assumed. This is
  `entry-k39`'s rule and the reason it exists is in the README: four of its
  fourteen witnesses first land one state wider than the file's conventional
  minimum, and `FN-05.a` was green at that minimum over a member it never
  reached. Adding `crash` will move these bounds again.
- One mutation per obligation, each with **evidence that it fires** — one
  existing witness re-run under it, still landing. Three of `entry-k39`'s nine
  reported as survivors and none of the three was a fact about a check: two were
  unsatisfiable because the mutation was added *underneath* a frame condition,
  and one was the identity.
- The family `README.md` gains the new bounds, the new abstractions, the
  witness-bound table's new rows, and any retained counterexample.
- Material observations are appended to Experiment 2 as entry 032.
- The next leaf (`commit`, `FN-03`, `FN-04`, `FN-14` – `FN-18`) is cut as a
  sibling, its body carrying what this file actually leaves open.


## Notes

**`FN-11`'s witness is an interval, and an interval claim needs interval-many
states.** That is the first of `task-tree-k7`'s two bound-vacuity predictors, and
this is the first obligation in the finish scope it applies to: *the task root
present, unwalkable and holding every entry, between publication and commit* is
not a state, it is a stretch of trace with a publication before it and an
attempted commit after it. Budget for it while writing the bound.

**Adding `crash` is the first reachable-transition addition this file makes**,
and the cost model says to expect it to be the expensive kind: four reachable
transitions cost the sibling file +41%, where five behind a *static* switch cost
+10%. `entry-k39`'s whole file runs in 38s, so there is room — but measure the
file's **widest** command as well as its tightest, because the tight one prices
transitions and only a wide one prices state.

**Do not let `FN-13` collapse into the commit slice.** *Every candidate committed
tree excludes the witness* needs a candidate tree and an exclusion, not a
disposition. If reaching it honestly requires the commit machinery, that is
either a declared gap with a reason or evidence the cut between this leaf and
`commit` is in the wrong place — say which, rather than quietly pulling the
commit forward.

Do not read the Quint side of Experiment 2. The independence protocol holds until
both families are green.
