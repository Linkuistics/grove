# handoff-k42


## Goal

Extend `crates/grove-finish/models/finish.als` to the quarantine and its
disposal: the atomic root rename, the four revalidation points and their ten-row
table, disposal's re-entrancy, the cleanup marker with its `replace` transition,
and the reaper — `FN-19` – `FN-22`, `FN-31`, nineteen obligations.

**The `finish-k8` brief expects this leaf to decompose**, and nothing
`commit-k41` learned argues against it: `FN-22` alone is ten obligations and
`FN-31` is a nested crash-safe protocol. Decompose along the *machinery* each
group needs, as this level has cut every child so far.


## Context

`commit-k41` left the file green for `FN-01` and `FN-03` – `FN-18` — the entry
surface, the reserved witness, and the commit with its disposition; `FN-02` is
`exits`' — and took the finish scope's empty alloy cells to **thirty-three**. Seventy-one
commands, 2 m 40 s.

`crates/grove-finish/models/README.md` carries the bounds, the abstractions, the
witness-bound table's forty-one measured rows, the twenty-nine-row mutation
matrix, the four retained counterexamples, the two mutation-discipline lessons
and the eleven *what a green run does not prove* caveats. **Read it before
writing a command.** Four of its sections are specifically this leaf's
inheritance and are named below.

What exists that you can build on, and what each of it is worth:

- **The transaction now runs to a settled disposition.** Six body steps, then
  `CommitAttempt` (which commits or does not, and reports or does not),
  `Classify` and `Settle`. `Recover` adopts an interrupted attempt's manifest.
  The repository has history (`Repo.tickets`), a reproduced preflight commit and
  an ability to reproduce one. `Blocked` exists as an **outcome atom with no
  diagnosis**.
- **`Blocked`'s diagnosis partition is still unwritten and is `exits`', not
  yours.** `FN-25` is what makes it total, disjoint and exhaustive.
  `commit-k41` deliberately left `BlockedOutcome` bare so that `FN-25.a` would
  not be answered by construction. **`FN-21.c` and `FN-22` will want
  `OwnershipConflict` in the way that `FN-15.c` wanted `Indeterminate`** — if you
  need to name a diagnosis, name it as a model-only `Sys.why` member the way this
  file already names nine, and say in the README why you did not extend the
  outcome. Adding the partition here is the false-confidence shape, not a
  convenience.
- **`evacuationComplete` requires the task root still present; `gateEvacuated`
  does not — and this leaf is where that divergence goes live.** The two have
  been written apart since `witness-k40` precisely because the day a step removes
  the root, the difference becomes a counterexample. **The quarantine rename is
  that step.** It was armed for you; expect `FN-11`'s check to have something to
  say about it, and do not close the divergence by editing either side to match
  the other without a recorded reason.
- **`bodySteps` now holds nine members** — the six body steps plus `Recover`,
  `Classify` and `Settle` — and `FN-24.b` will quantify over exactly it. Every
  step you add belongs in it, and each should have **at most one persistent
  effect** that is a same-directory rename (`EN-01`) or is itself decomposed.
  `commit-k41` has one step that plainly is not: `doSettle`'s restore branch
  restores the tree, reproduces the exact preflight commit and releases the
  witness together. It is recorded as an abstraction, and it is `exits`' to judge.
- **The forward settle's release of the witness and the manifest is a stand-in
  for disposal, and it is yours to replace.** `FN-18` needed the artifacts to be
  gone so that `FN-03`'s retry had no local trace to read; it needed nothing about
  *how*. Your quarantine rename, cleanup marker and reaper are the real thing,
  and the first question to ask is whether replacing the stand-in breaks
  `witness_FN_03` — if it does, that is a finding about `FN-03`'s witness rather
  than about your step.

What this leaf has to build, and none of it exists:

- **The quarantine**: a target directory, its device (already in the signature as
  `World.qDev`, and `FN-08` is the only thing that has ever read it), and the
  atomic rename of the task root into it.
- **The four revalidation points and their ten-row table** (`FN-22`). This is the
  machinery `commit-k41` explicitly did **not** build and named as the reason its
  own `Recover` is deliberately narrow: `doRecover` adopts the manifest's attempt,
  anchor and handle under one guard and revalidates nothing. Your ten rows are
  what say when that is legitimate.
- **The cleanup marker and its `replace` transition** (`FN-31`). The catalogue
  keeps `replace-cleanup-marker` a separate action because
  `TODO.finish_process.md` Q3 asks whether *replacement* — as against creating or
  removing a marker — is reachable at all. **A model that folds it away answers
  Q3 by construction.**
- **The reaper** (`FN-21`), which is also `TT-24.d`'s subject.


## Done when

- Every obligation of `FN-19` – `FN-22` and `FN-31` is answered by a `check` and
  its required `witness_` runs, all green under
  `models/run.sh --scope finish --family alloy --no-coverage`, with the finish
  scope's empty-cell count down to fourteen.
- `FN-31.a` is answered by the instrument the catalogue names — a witness, **or**
  a bounded-unreachability `check` over the full scope with its bound and result
  recorded **per lane** — or recorded as a `defer` with the reason. `commit-k41`
  answered `FN-15.d`'s identically-shaped obligation with witnesses on all three
  lanes and recorded no `defer`; that is the bar, not a precedent that it is
  always reachable.
- **Every check runs at a bound at least as large as the widest first-landing
  bound among its obligation's witnesses**, measured by sweep rather than
  assumed. `commit-k41` swept all forty-one witnesses from 2 to 14 states in
  about two minutes at concurrency five; the script is trivial to rewrite and the
  measurement is not optional — see *Notes*.
- One mutation per obligation, each with **evidence that it fires** — one
  existing witness re-run under it, still landing.
- The family `README.md` gains the new bounds, the new abstractions, the
  witness-bound table's new rows, the mutation matrix's new rows, any retained
  counterexample, and any further Q4 removal-matrix rows the mutations decide.
  Five rows are already decided and are recorded there; **three of them are
  `commit-k41`'s and are the first whose first-broken obligation is a
  shared-safety claim**, which is what makes them answers to Q4 rather than notes
  toward one.
- If a claim holds on only some lanes, that is a **finding** and it goes in the
  README rather than into a lane-specific claim.
- Material observations are appended to Experiment 2 as entry 034.
- The next leaf (`exits`, `FN-02`, `FN-23` – `FN-30`, plus Q4's removal matrix,
  owning `EN-08` and `EN-16`) is cut as a sibling **of whatever level this leaf's
  decomposition leaves it at**, its body carrying what is actually left open.


## Notes

**The cost law `commit-k41` measured, and why it matters more to you than to
anyone before you.** Four reachable transitions cost the inherited *entry-surface*
commands +30% and the file's widest inherited command (`FN-13`, ten states)
**+128%** — medians of three, one host, one sitting. The marginal cost of a
transition is **superlinear in the trace length it is reachable at**, so:

> **Budget by counting transitions × the bound they are reachable at, not by
> counting transitions.**

You are the longest-trace slice in this scope. A quarantine rename, a marker, a
`replace` and a reaper are four transitions reachable only at the far end of a
ten-state trace, which is the exact worst case that law describes. Prefer, in
order: a static scope switch, a narrowed antecedent, a smaller bound. **Pin a
switch that deletes state; leave free a switch that admits an action.**

**Where a witness may start, and the debt `commit-k41` took on.** Fifteen of that
slice's eighteen witnesses start from `interruptedMidEvacuation` — a *posited
disk* at `Txn.phase = Fresh`, written to be exactly what the six body steps plus
a `crash` produce. It is **not checked** to be reachable, and the README records
that as a limit. **The check would be `FN-22`'s revalidation table, which is
yours.** If your ten rows say that disk is not something a recovery may act on,
fifteen inherited witnesses are testifying about a state no execution reaches,
and no check will tell you — a check quantifies over all traces and does not care
which one a witness picked. That is the single most valuable thing this leaf can
discover, and it is cheap to look for first rather than last.

**Four measurement and mutation rules, all inherited, none restated in the
README's own voice more than once.**

1. **One sentinel is not enough**, and the tightest command is nearly blind to
   new state *and* under-reports new transitions — measure the widest as well.
2. **Whole-suite totals do not compare across sessions.**
3. **A single command's cost is bimodal within one sitting** (2.0 s, 10.1 s,
   2.0 s on the same bytes). Median of three, or report nothing.
4. **A step that stops being a no-op costs a state to every witness that ended on
   it.** `commit-k41` made `doCommitAttempt` advance the phase and
   `witness_FN_11` went 9 → 10 — one `FAIL … no instance` in the first full run.
   Re-measure any witness whose last transition you made mutating.

And two about mutations, both learned the hard way in `commit-k41`:

- **The frame you must remove is every frame**, and a redundantly-stated one
  hides the other copy. A mutation that removes one of two copies is a semantic
  no-op and reports exactly as a survivor does.
- **Writing the claim apart from the transition protects the claim from a
  mutation aimed at the transition.** That is the house style working as
  designed, and it is why the *aim* of a mutation must be checked as carefully as
  its satisfiability.

**On review.** `commit-k41` cut no `review-prototype` step, consistent with
`entry-k39` and `witness-k40`: the artifact is adversarially verified by one
mutation per obligation with named fire-evidence, and by a runner that fails on
zero work and on an unnamed command, which is a stronger and cheaper check than a
read. Cut one if your slice produces a claim the mutation discipline cannot
reach — `FN-22`'s ten-row table is the plausible candidate, because a table is
exactly the shape a mutation cannot falsify row by row.

Do not read the Quint side of Experiment 2, and do not open
`crates/grove-finish/models/*.qnt` if one appears. The independence protocol
holds until both families are green.
