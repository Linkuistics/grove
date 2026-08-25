# handoff-k42 — brief


## Goal

Extend `crates/grove-finish/models/finish.als` to the quarantine and its
disposal: the atomic root rename, the four revalidation points and their ten-row
table, disposal's re-entrancy, the cleanup marker with its `replace` transition,
and the reaper — `FN-19` – `FN-22`, `FN-31`, nineteen obligations.


## Context

`commit-k41` left the file green for `FN-01` and `FN-03` – `FN-18` — the entry
surface, the reserved witness, and the commit with its disposition; `FN-02` is
`exits`' — and took the finish scope's empty alloy cells to **thirty-three**.
Seventy-one commands, 2 m 40 s.

`crates/grove-finish/models/README.md` carries the bounds, the abstractions, the
witness-bound table's forty-three measured rows, the thirty-one-row mutation
matrix, the five retained counterexamples, the three mutation-discipline lessons
and the thirteen *what a green run does not prove* caveats. **Read it before
writing a command.** Four of its sections are specifically this subtree's
inheritance and are named below.

What exists that a child can build on, and what each of it is worth:

- **The transaction now runs to a settled disposition.** Six body steps, then
  `CommitAttempt` (which commits or does not, and reports or does not),
  `Classify` and `Settle`. `Recover` adopts an interrupted attempt's manifest.
  The repository has history (`Repo.tickets`), a reproduced preflight commit and
  an ability to reproduce one. `Blocked` exists as an **outcome atom with no
  diagnosis**.
- **`Blocked`'s diagnosis partition is still unwritten and is `exits`', not this
  subtree's.** `FN-25` is what makes it total, disjoint and exhaustive.
  `commit-k41` deliberately left `BlockedOutcome` bare so that `FN-25.a` would
  not be answered by construction. **`FN-21.c` and `FN-22` will want
  `OwnershipConflict` in the way that `FN-15.c` wanted `Indeterminate`** — if a
  child needs to name a diagnosis, it names it as a model-only `Sys.why` member
  the way this file already names nine, and says in the README why it did not
  extend the outcome. Adding the partition here is the false-confidence shape,
  not a convenience.
- **The `evacuationComplete` / `gateEvacuated` divergence DID NOT go live, and
  it stays armed.** `quarantine-k43` built the rename and `FN-11` stayed green at
  ten states: the rename is two transitions past the commit attempt and
  `doTxnOpen` refuses an absent root, so the protocol's own **ordering** makes
  the divergence unreachable. That is weaker than *the gate enforces it* and
  neither side was edited. The first thing that re-enters a transaction over a
  rootless disk is `FN-22`'s revalidation after the rename, so `revalidation` is
  where to expect it next.
- **`bodySteps` now holds nine members** — the six body steps plus `Recover`,
  `Classify` and `Settle` — and `FN-24.b` will quantify over exactly it. Every
  step a child adds belongs in it, and each should have **at most one persistent
  effect** that is a same-directory rename (`EN-01`) or is itself decomposed.
  `commit-k41` has one step that plainly is not: `doSettle`'s restore branch
  restores the tree, reproduces the exact preflight commit and releases the
  witness together. It is recorded as an abstraction, and it is `exits`' to judge.
- **Half of the disposal stand-in has been replaced; the other half has not.**
  `quarantine-k43` put the real rename in front of the forward settle, so the
  settle now **disposes a quarantine** rather than releasing artifacts in place.
  It is still an abstraction: nothing claims disposal is re-entrant,
  marker-guarded or bounded to Grove's own, and `FN-21` / `FN-31` are
  `disposal`'s. Replacing the first half cost `witness_FN_03`, `witness_FN_16b`
  and `witness_FN_18` one state each — a step inserted into a path costs a state
  to every witness that passes through it — and none of them broke.

What this subtree has to build. **The quarantine is done**; the other three are
not:

- ~~**The quarantine**~~ — built by `quarantine-k43` as `one sig Quar { var
  qRid: lone RootId }`: a second place a `RootId` can be, so that *witness and
  evacuated tree intact* is a frame condition rather than a list of equalities.
  `doQuarRename` fires from `Classified` with `Committed`, produces the new
  `Quarantined` phase, and blocks with a model-only `why` on an occupied target.
- **The four revalidation points and their ten-row table** (`FN-22`). This is the
  machinery `commit-k41` explicitly did **not** build and named as the reason its
  own `Recover` is deliberately narrow: `doRecover` adopts the manifest's attempt,
  anchor and handle under one guard and revalidates nothing. The ten rows are
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
- Material observations are appended to Experiment 2, entry 034 onward — one
  entry per child session at least.
- The next leaf after this subtree (`exits`, `FN-02`, `FN-23` – `FN-30`, plus
  Q4's removal matrix, owning `EN-08` and `EN-16`) is cut by this node's **last**
  child as a sibling **of this node**, under `finish-k8`, its body carrying what
  is actually left open.


## Decomposition

Nineteen obligations against a file whose widest command is already ten states,
and the cost law says the marginal cost of a transition is superlinear in the
trace length it is reachable at — so this level is cut along the **machinery**
each claim group needs, as `finish-k8` cut its own, and the cut is also a cut
along *transitions added per session*.

1. `quarantine` — `FN-19`, `FN-20` (2 obligations). The quarantine target and its
   device, the atomic rename of the whole task root into it, and the replacement
   of the forward settle's release stand-in with that rename. Two obligations is
   small in cells and is not small in work: this is the child that takes the
   `evacuationComplete` / `gateEvacuated` divergence live, re-tests
   `witness_FN_03` against a real disposal path, and pays the first — and by the
   cost law the dearest — long-trace transition of the three.
2. `revalidation` — `FN-22` (10). The four revalidation points, the ten-row
   table, and the quarantine **return** as its own transition, including the two
   `Committed` departures the shipped material never distinguished. Needs
   `quarantine`'s rename to exist and the restoration that `commit-k41` already
   built, so it is the only child whose machinery is *both* handoffs.
3. `disposal` — `FN-21`, `FN-31` (7). Disposal's re-entrancy, the cleanup
   marker's create / replace / remove transitions, replacement's atomicity with
   respect to readers, and the reaper. `FN-21.a`'s *re-enterable from any
   interruption* and `FN-31.c`'s *interruption inside the replacement, resumed*
   are the same machinery asked at two grains, which is why the two claim groups
   are one child rather than two. Cuts `exits` under `finish-k8` as its last act.

Why `FN-22` is not split, and why `FN-19`/`FN-20` are not folded into it. The
table is one claim about a discipline — *every observed disposition at every
point has a stated corrective action and a stated stable state* — and `FN-22.a`
quantifies over all four points at once. Splitting it row-wise would let a child
report green over a partial table, which is exactly the shape a per-row mutation
cannot falsify. Folding `FN-19` into it would put the first long-trace transition
and the ten-row table in one session, which the cost law says is the one pairing
to avoid.

Only the first child was cut when this node was created. Each session cuts the
next one as its last act, once the model's actual shape at that point is known.


## Notes

**The cost law `commit-k41` measured, and why it matters more here than anywhere
before.** Four reachable transitions cost the inherited *entry-surface* commands
+30% and the file's widest inherited command (`FN-13`, ten states) **+128%** —
medians of three, one host, one sitting. The marginal cost of a transition is
**superlinear in the trace length it is reachable at**, so:

> **Budget by counting transitions × the bound they are reachable at, not by
> counting transitions.**

**`quarantine-k43` measured that arithmetic at a factor of six too pessimistic,
and corrected the law rather than the advice.** One transition reachable only at
the far end of a ten-state trace cost the widest command **+5%**, against about
+30% predicted — because it is enabled at exactly **one** (phase, guard) point,
where `commit-k41`'s four included transitions enabled at three phases each. The
operative form from here:

> **Budget by the number of (phase, guard) points a transition is enabled at,
> multiplied by the bound they are reachable at.**

That is *prefer a narrowed antecedent* arriving with a number on it. It matters
most to `revalidation`, whose four revalidation points at two handoffs are the
opposite shape from one deep rename. Prefer, in order: a static scope switch, a
narrowed antecedent, a smaller bound. **Pin a switch that deletes state; leave
free a switch that admits an action.**

**Where a witness may start, and the debt `commit-k41` took on.** Fifteen of that
slice's eighteen witnesses start from `interruptedMidEvacuation` — a *posited
disk* at `Txn.phase = Fresh`, written to be exactly what the six body steps plus
a `crash` produce. It is **not checked** to be reachable, and the README records
that as a limit. **The check would be `FN-22`'s revalidation table**, which is
`revalidation`'s. If those ten rows say that disk is not something a recovery may
act on, fifteen inherited witnesses are testifying about a state no execution
reaches, and no check will tell you — a check quantifies over all traces and does
not care which one a witness picked. That is the single most valuable thing this
subtree can discover, and it is cheap to look for first rather than last.

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
   Re-measure any witness whose last transition a child made mutating.

And two about mutations, both learned the hard way in `commit-k41`:

- **The frame you must remove is every frame**, and a redundantly-stated one
  hides the other copy. A mutation that removes one of two copies is a semantic
  no-op and reports exactly as a survivor does.
- **Writing the claim apart from the transition protects the claim from a
  mutation aimed at the transition.** That is the house style working as
  designed, and it is why the *aim* of a mutation must be checked as carefully as
  its satisfiability.

And a third, from `quarantine-k43`:

- **A mutation that kills its target AND a neighbouring obligation has not
  isolated what the target uniquely says**, and it reports as a kill either way.
  Check what a mutation leaves **green** as carefully as what it takes red.

**On review.** `entry-k39`, `witness-k40`, `commit-k41` and `quarantine-k43` each cut no
`review-prototype` step: the artifact is adversarially verified by one mutation
per obligation with named fire-evidence, and by a runner that fails on zero work
and on an unnamed command, which is a stronger and cheaper check than a read. A
child cuts one if its slice produces a claim the mutation discipline cannot
reach — `FN-22`'s ten-row table is the plausible candidate, because a table is
exactly the shape a mutation cannot falsify row by row.

Do not read the Quint side of Experiment 2, and do not open
`crates/grove-finish/models/*.qnt` if one appears. The independence protocol
holds until both families are green.
