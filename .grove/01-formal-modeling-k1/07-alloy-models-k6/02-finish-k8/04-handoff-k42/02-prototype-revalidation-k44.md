# revalidation-k44


## Goal

Answer `FN-22` in `crates/grove-finish/models/finish.als` — the four
revalidation points, the ten-row table, and the quarantine **return** as its own
transition. Ten obligations, and the largest single claim group in the finish
scope.


## Context

`quarantine-k43` left the file green for `FN-19` and `FN-20`, at **75 commands,
3 m 05 s, thirty-one empty alloy cells**. Both handoffs `FN-22` is stated over
now exist: `commit-k41` built the **restoration** (`doSettle`'s rollback branch)
and `quarantine-k43` built the **quarantine rename** (`doQuarRename`, from
`Classified` with `Committed`, producing the new `Quarantined` phase). What does
not exist is any recheck around either of them, and that absence is deliberate in
three specific places this leaf should read before writing a command:

- **`doClassify` was NOT opened to `Quarantined`.** It is re-runnable at
  `Attempted`, `Classified` and `Settled` and at no phase between the rename and
  the settle. That gap *is* `FN-22`'s *after the quarantine rename* row, and
  `quarantine-k43` left it empty rather than fill it, because writing it there
  would have answered two of this leaf's ten rows by construction.
- **`doRecover` still revalidates nothing.** It adopts the manifest's attempt,
  anchor and handle under one guard, and `commit-k41` recorded that as
  deliberately narrow with this leaf named as the reason.
- **There is no return.** `FN-22.f`, `.g` and `.h` need the quarantine to come
  back atomically, and the rename has no inverse in the file.

**The single most valuable thing this leaf can discover, and it is cheap to look
for first.** Fifteen of `commit-k41`'s eighteen witnesses — and every witness in
the file that starts from `interruptedMidEvacuation` — begin from a **posited
disk** written to be exactly what the six body steps plus a `crash` produce. It
is not checked to be reachable and the family `README.md` records that as a
limit, naming this leaf's ten rows as the check. If those rows say that disk is
not something a recovery may act on, a large fraction of the file's witnesses are
testifying about a state no execution reaches — and **no check will tell you**,
because a check quantifies over all traces and does not care which one a witness
picked.

**Two things `quarantine-k43` measured that change how this leaf should budget.**

- **The cost law now has a second variable, and it is the one this leaf is
  exposed to.** One transition reachable only at the far end of a ten-state trace
  cost the file's widest command **+5%**, not the +30% `commit-k41`'s law
  predicted — because it is enabled at exactly one phase and one disposition.
  Budget by **(phase, guard) points × the bound they are reachable at**. Four
  revalidation points at two handoffs is the opposite shape from a single deep
  rename, and the ten-row table is *enabling conditions* rather than transitions.
  The measurement is in `README.md` under *Cost*; read it before choosing between
  a switch, a narrower antecedent and a smaller bound.
- **A step inserted into a path costs a state to every witness that passes
  through it.** The rename moved three inherited witnesses 9 → 10 and their three
  checks with them. A revalidation point spliced before or after either handoff
  will do the same to everything downstream of it; the sweep is not optional and
  forty-three witnesses swept 2..14 at concurrency five took about two and a half
  minutes.

**A mutation must leave its neighbours standing.** `quarantine-k43` added a third
way for a mutation to fail its aim, after `entry-k39`'s *cannot fire* and
`commit-k41`'s *wrong half*: a mutation that kills its target **and a
neighbouring obligation** has not isolated what the target uniquely says, and it
reports as a kill either way. `FN-22`'s rows sit close together and close to
`FN-15` and `FN-16`; expect to check what each mutation leaves green as carefully
as what it takes red.


## Done when

- Every obligation of `FN-22` (`.a` – `.j`) is answered by a `check` and its
  required `witness_` runs, all green under
  `models/run.sh --scope finish --family alloy --no-coverage`, with the finish
  scope's empty-cell count down from thirty-one to twenty-one.
- **`FN-22.a`'s *all four revalidation points are performed, and none is
  skipped*** is stated so that a table with a missing row is a counterexample,
  not a silence. This is the obligation the mutation discipline reaches least
  well, and it is the reason a `review-prototype` step may be earned here — see
  *Notes*.
- The two `Committed` departures the shipped material never distinguished —
  `Committed -> NotCommitted` (a rollback that succeeds, ending `Refused`) and
  `Committed -> Indeterminate` (a block) — are separately reachable and
  separately checked. Collapsing them lets a block be reported as a refusal,
  which is the distinction `FN-29` requires an operator to be able to make.
- Whether `interruptedMidEvacuation` is reachable is **answered**, either by a
  witness that runs the body up to it or by a recorded finding that it is not —
  and if it is not, every witness resting on it is re-examined rather than left
  standing.
- Each check runs at a bound at least as large as the widest first-landing bound
  among its own obligation's witnesses, measured by sweep, with every inherited
  witness re-measured.
- One mutation per obligation, each with evidence that it fires and a note of
  what it left green.
- The family `README.md` gains the new bounds, abstractions, witness-bound rows,
  mutation-matrix rows, any retained counterexample, and any Q4 removal-matrix
  row the mutations decide.
- Material observations are appended to Experiment 2 as entry 035.
- `disposal` (`FN-21`, `FN-31`) is cut as the next sibling under `handoff-k42`.


## Notes

`FN-25`'s `Blocked` partition is still `exits`' and is still deliberately
unwritten. `FN-22`'s table produces four `Blocked` rows and will want
`RecoveryPending` by name; name it as a model-only `Sys.why` member the way the
file already names ten — `quarantine-k43` added `W14QuarantineOccupied` on
exactly this basis — and say in the `README.md` why the outcome was not extended.

**On review.** No sibling in this subtree has cut a `review-prototype` step: the
artifact is adversarially verified by one mutation per obligation with named
fire-evidence, and by a runner that fails on zero work and on an unnamed command.
`FN-22` is the one slice `finish-k8` and `handoff-k42` both named as the
plausible exception, because **a table is exactly the shape a mutation cannot
falsify row by row** — a per-row mutation kills a row, and nothing kills *the
table is complete*. Decide it against `FN-22.a` once written; deciding against
review is a normal outcome.

Do not read the Quint side of Experiment 2, and do not open
`crates/grove-finish/models/*.qnt` if one appears. The independence protocol
holds until both families are green.
