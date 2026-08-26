# sessions-k54


## Goal

Close the `SY-` column: add the session's own ending, the crash, and the two
sweeps to `models/system/lifecycle.als` — `SY-09`, `SY-12`, `SY-13` and `SY-14`,
eight obligations. The session step with exactly three endings, crash at every
lifecycle point, the stable-state sweep and the `Blocked` persistence sweep.
This is the last child of `system-k9`; when it lands, `--no-coverage` leaves
`models/system/README.md`'s run line and the scope asserts coverage.


## Context

- `docs/specs/semantic-contract.md` §*Claims — system lifecycle* `SY-09`,
  `SY-12` – `SY-14`; §*States* for the **stable / transient** distinction
  `SY-13` is stated over; §*Outcomes* for `Blocked` and the two closed
  diagnoses `SY-14` refuses under; §*Actions* for the five Environment actions
  and for what *admitted* excludes.
- `models/system/README.md` — read *Four incidents worth carrying forward* and
  *The mutation matrix* before writing a fact. The fourth incident is new and
  is the one this leaf is most exposed to, because both sweeps are invariants
  over a free initial state: **an invariant the transitions PRESERVE but cannot
  ESTABLISH reports a counterexample about state 0 rather than about the
  design**, and written as an `always` fact instead it asserts the claim and
  every mutation survives. Classify each new invariant before checking it.
- `CONTEXT.md` *Terminal disposition*, *Complete finish cycle*, *Session epoch*,
  *Obligation*.
- ADRs: `one-build-owns-a-session` and `one-live-driver-per-working-tree`
  (`SY-09`'s three endings and the no-signal stop), `bulk-marks-are-not-atomic`.
- The measuring invocation is
  `models/run.sh --scope lifecycle --family alloy --no-coverage`, and dropping
  `--no-coverage` is this leaf's last act rather than its first.


## Done when

- `SY-09.a` – `SY-09.c`, `SY-12`, `SY-13.a`, `SY-13.b`, `SY-14.a` and `SY-14.b`
  each have a `check` and their required `witness_` runs, all green, and the
  runner reports **zero** empty alloy cells for the lifecycle scope with
  **coverage asserted** — `models/run.sh --scope lifecycle --family alloy`, no
  `--no-coverage`, and the flag removed from `models/system/README.md`'s run
  line in the same edit.
- **`SY-13.a`, `SY-13.b` and `SY-14.a` are existential-reachability or sweep
  instruments and NOT liveness properties**, and `README.md` says so where a
  reader would otherwise read fairness into them. `SY-13.a` records the
  **longest** admitted sequence within the bound **and its length**; `SY-13.b`
  and `SY-14.a` are each an **exhaustive sweep with its bound stated**. A child
  that reaches for `eventually` where the catalogue says *there exists a
  bounded sequence* has manufactured liveness by omitting the hostile choices.
- **`EN-08` is this leaf's and no sibling discharges it.** Exercise-removal:
  with `crash` removed, `SY-12`'s crash-point witnesses become unreachable and
  every property stays green. The finish scope ran the `FN-` half over its own
  witnesses; the `SY-12` half is unrun. The control is
  `expect_unreachable_EN_08_…` — a `run` that must find NO instance, which the
  runner inverts — and the assumption table's expected result is *the run fails
  on zero work rather than reporting green*, so the control must demonstrate
  both halves: the witnesses die, the properties do not.
- One mutation per reported obligation, each **swept for the bound at which it
  first fires** and compared against the check's bound — `roots-k53` established
  that discipline (seven mutations, all firing at 3 against checks at 5) and
  entry 041's `M8` incident is what makes it necessary. A survivor is
  investigated to one of the family README's three named causes, with a
  **differential probe** where an unsatisfiable mutation and a live-but-invisible
  one must be told apart.
- `models/system/README.md` extended: bounds and witness first-landing per new
  obligation, any new abstraction, any new declared gap in the shape the runner
  parses, and — because this closes the scope — a **final composition-boundary
  table** that a reader of `formal-synthesis-k16` can use without reading the
  model.
- Experiment 2 entry 043 is appended with the six required fields plus the
  pre-registration's four additions, including the M5 authoring figure. This
  scope is the H8 comparison's system-level arm and it now has three points
  (0.42, 0.60 and 0.46 h/obligation, mean 0.49); the fourth closes the arm, and
  the entry should say what the four together do and do not support.


## Notes

**`SY-13` inherits a concrete state from `roots-k53` that it must classify, and
it was found rather than designed.** `SY-04.b` gates every Lifecycle transition
but `acquire-lease` on a valid configuration, so **`release-lease` is
unreachable under an invalid configuration**: a driver whose configuration goes
invalid mid-loop cannot release its lease by returning. It is not a sink —
`SY-01.b` makes process death an ordinary release, and that is the exit the
shipped driver takes — but it is the first state `SY-13` has to decide about,
and whether *process death* counts as an **admitted action** is the decision.
If it does not, the state is a sink and `SY-13.a` fails on a state the design
actually reaches; if it does, `SY-13` is partly a claim about the kernel.
Neither answer is obviously right and the catalogue does not settle it. Expect
this to be the leaf's sharpest question and record it either way.

**The stable/transient distinction is `SY-13`'s whole antecedent and this file
does not yet have it.** §*States* defines a transient state as one existing only
inside one operation, while its exclusive guard is held, or between two
filesystem steps of one transaction — and `roots-k53` created exactly such a
state: `PartialScaffold`, the interval between `doInitRoot` and
`doCompleteScaffold`. It is nonetheless **stable** by the catalogue's own
definition (an ordinary invocation observes and acts on it), which is a useful
calibration for the classification this leaf must write. `Reserved(*)` is the
transient class and is `crates/grove-finish/models/`'s; expect to import
stability as an opaque predicate over the observation rather than to enumerate.

**`Blocked` does not exist in this file yet, and adding it is the composition
question of this leaf.** `SY-14` is stated over a blocked tree, `FN-25`'s two
diagnoses are the finish model's, and the node brief's rule says import the
smallest observation that decides the claim — *the tree is blocked* — with
`crates/grove-finish/models/` named as the owner of what put it there and which
diagnosis it carries. A `Blocked` with internals here is this file becoming a
third copy of the finish contract. Note that `Result` already has `Blocked`'s
neighbours (`Stopped`, `Deferred`) as declared abstractions and `Blocked` itself
is the catalogue's own outcome, so it enters as a `Result` member and not as an
abstraction.

**Two `TT-` obligations remain declared gaps unfillable from either sibling
directory; `finish-k8` found a third instance in Q4's removal matrix, and
`roots-k53` found the fourth — `SY-06.b`'s *completed before any format
classification runs*, whose ordering is `TT-18`'s.** Wherever a `SY-` claim's
content is really a `TT-` or `FN-` one, record it in the family README in the
same shape rather than solving it. `formal-synthesis-k16` inherits the whole
set, and this leaf is the last chance to state one cheaply.

**One cost fact worth planning around.** `roots-k53` measured that a free
`var`-referenced signature, not the transition count, is where this file's run
cost lives: eight new transitions and six new `World` fields moved nothing, and
one new signature (`Grove`, scope 3) took three commands from ~1.5 s to 3–9 s
and the suite from 37 s to ~110 s. Both sweeps this leaf owes are exhaustive
over states, so **budget the bound before writing the command**, and prefer a
new opaque observation on an existing signature to a new signature.

Do not read the Quint side of Experiment 2, and do not open
`models/system/*.qnt` if one appears. The independence protocol holds until both
families are green.
