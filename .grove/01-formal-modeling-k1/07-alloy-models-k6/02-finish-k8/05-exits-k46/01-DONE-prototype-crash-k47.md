# crash-k47


## Goal

Answer `FN-24.a` and `FN-24.b` in `crates/grove-finish/models/finish.als` — *every
interruption lands in exactly one stable state*, and *every step of the
transaction has at most one persistent effect*. Own **`EN-08`**, the `crash`
exercise-removal. Leave the file green under
`models/run.sh --scope finish --family alloy --no-coverage` with twelve empty
alloy cells rather than fourteen.


## Context

Read the node brief above first; it says why this leaf exists and what the two
siblings after it are for. `crates/grove-finish/models/README.md` is the other
required read — the bound register, the cost law, the mutation matrix's
discipline, and *what a green run does not prove*.

**Two machineries this leaf adds, and neither exists in the file today.**

- **A stable-state classification of the disk.** The catalogue's task-root state
  table (§*States*) is eleven rows; this file's disk can be in four of them —
  `Absent`, `Reserved(Preparing)`, `Reserved(Published)`, and a present current
  root. `FN-24.a`'s *exactly one* is a claim about that classification, so write
  it **as data**, the way `observed` and `tableAction` are written: a total
  function stated apart from every transition, whose arms carry their own full
  guards rather than being disjoint by an ordering the model imposes. A
  classification made disjoint by construction answers the claim by construction.
- **A persistent-effect enumeration.** `FN-24.b`'s grain is *the effect*, not
  *the field*: a same-directory rename touches two names and is one effect
  (`EN-01`), and removing a directory removes what is inside it. Getting the
  grain wrong makes `doRevalidate`'s witness release read as four effects and
  `doWEvacuate` as two, which would turn a correct protocol into a red check.

**What the file already tells you about the two steps that will not be `lone`.**
`disposal-k45` and `commit-k41` each left one, and the node brief and
`README.md` name both: `doDispose` clears the quarantine, the reserved slot and
the manifest together *because in this model they are one directory* (`EN-03`
says the shipped removal is entry-by-entry), and `doSettle`'s restore branch
restores the tree **and** reproduces the preflight commit on a
working-copy-as-commit lane. Both are **declared**, with what it would take to
decompose them — that is what `FN-24.b`'s own words ask for, not a check
weakened until it passes.

**`Reap` is in neither `bodySteps` nor `txnActs`, and the file says so twice.**
A sweep is not a step of the transaction, takes no confirmation, and never had a
disposition to revalidate. `FN-24.b` quantifies over `bodySteps`; read that
exclusion before asking the question of the reaper. Whether the sweep *should*
owe the one-persistent-effect discipline is `FN-24.b`'s to decide and record, not
something the set already answered.

**Budget `FN-24.a` as a dwell claim.** `crash` is already enabled at every step
boundary, so the check adds no transition — but its antecedent quantifies over
sixteen `bodySteps` and the deepest of them first occurs late, so the bound is
the cost. Take the ordering `disposal-k45` measured — a static scope switch,
then a narrowed antecedent, then a smaller bound — and **do not take the
multiplier**; it has been wrong three times and right once.

**Read every check's antecedent for the deepest transition it names**, and take
the larger of that and the widest first-landing bound among the obligation's
witnesses. `FN-31.c` is the retained case where the witness rule alone would have
made a check green and empty.


## Done when

- `FN-24.a` and `FN-24.b` are each answered by a `check` and its required
  `witness_` runs, all green under `models/run.sh --scope finish --family alloy
  --no-coverage`, and the runner reports **twelve** empty alloy cells for the
  finish scope.
- **`FN-24.a`'s witness is the step-boundary sweep it says it is** — *one crash
  point per step* — rather than one crash shown once. If sixteen commands is not
  what lands, say in `README.md` exactly which boundaries are witnessed, which
  are not, and what a green `FN_24a` therefore does not prove.
- **`EN-08` is present as its own named command** with the expected result the
  assumption table states: with `crash` removed, the named witnesses (`FN-09`,
  `FN-10`, `FN-24`, `FN-31.c`) are unreachable and the run fails on zero work
  rather than reporting green. Run it against the named witness set, not the
  whole file.
- The two multi-effect steps are **declared in `README.md`**, each with what it
  would take to decompose it, and neither is hidden by a weakened check.
- One mutation per obligation, with evidence that it fires and a note of what it
  left **green** — `FN-24.b`'s subject overlaps almost every frame condition in
  the file, so the neighbour sweep matters here more than it has anywhere.
- Every check runs at a bound at least as large as both its obligation's widest
  first-landing witness bound and the bound at which the deepest transition its
  own antecedent names first occurs; the witness-bound table is re-swept over
  every witness in the file, not only the ones this leaf adds.
- `README.md` gains the new bounds, the abstraction the two declared steps are,
  the witness-bound rows, the mutation rows, and any *what a green run does not
  prove* entry this slice earns.
- Material observations are appended to Experiment 2 as entries 037 onward, with
  the six required fields plus the pre-registration's four additions.
- The next child of `exits-k46` is cut as this session's last act, its body
  carrying what this session learned that the node brief could not state in
  advance.


## Notes

No `review-prototype` step is cut by default: the artifact is adversarially
verified by one mutation per obligation with named fire-evidence and a neighbour
sweep, and by a runner that fails on zero work and on an unnamed command. The
node brief records where the subtree's first plausible exception is, and it is
not this leaf's — it is Q4's prose matrix, which the third child writes.

Do not read the Quint side of Experiment 2, and do not open
`crates/grove-finish/models/*.qnt` if one appears. The independence protocol
holds until both families are green.
