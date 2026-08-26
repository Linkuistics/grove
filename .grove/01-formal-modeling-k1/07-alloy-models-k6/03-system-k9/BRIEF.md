# system-k9 — brief


## Goal

Model the temporal lifecycle from ordinary Grove work through completion,
finish, interruption, recovery, and root absence: every `SY-` obligation the
catalogue defines answered by an Alloy 6 command, or by a declared gap in the
family `README.md`.


## Context

This is a cross-component model in `models/system/`. It composes the task-tree
and finish contracts **at their observations** rather than copying their
internal state — which is this subtree's single hardest discipline and the one
thing no sibling scope had to hold. `docs/specs/semantic-contract.md` is the
sole input, as it was for `task-tree-k7` and `finish-k8`; nothing here invents a
semantic decision.

`models/run.sh` already knows this scope as `lifecycle` and maps it to
`models/system` (`prefix_scope`, `scope_dir`). It is complete and needs no
extension. The measuring invocation while the column is under construction is
`models/run.sh --scope lifecycle --family alloy --no-coverage`; the moment the
`SY-` column closes, `--no-coverage` leaves `models/system/README.md`'s run
line, which is the visible signal that the column closed.

**Composition at observations, stated as a rule.** A `SY-` claim reads a
task-tree or finish fact through the smallest observation that decides it — a
root *state* from §*States*, an *outcome* from §*Outcomes*, a *disposition*, a
blocked *diagnosis* — never through the machinery that produces it. Concretely:
this model has no `Filename`, no position, no key, no digest, no manifest, no
witness slot, no quarantine internals and no lane mechanism. Where a claim
needs one of those to be *true*, it is imported as an opaque predicate over the
observation and the sibling model is cited as its owner. A signature here that
duplicates a `TT-` or `FN-` structure is the failure mode this brief exists to
prevent: it is machinery no `SY-` obligation reads, it makes this file a third
copy of two contracts, and it makes any agreement between the three families
evidence about copying rather than about the design.

**Two omissions are inherited and are not this subtree's to reopen.** No
merge-and-remove exit is modelled (`model-contract-k5` put it to the human and
recorded the answer in §*Out of scope*), and no branch, bookmark or worktree
appears anywhere. `FN-28` states the single successful exit and `SY-13`'s two
terminal dispositions are stated over it.

**What the two sibling scopes left this subtree.** The node brief above carries
the runner rule, the cost model at three points and its correction for `var`
fields, the switch rule, the three bound-vacuity predictors, the static-atom law
(~10 ms of translation per atom per command), the mutation matrix's six failure
modes, and the two measurement rules (measure the widest command as well as the
tightest; whole-suite totals do not compare across sessions). All of them apply
here unchanged and none is restated. Four things are specifically this
subtree's:

- **`SY-05` arrives already constrained by a falsified formulation.** `finish-k8`
  proved that *the task root is absent* is not a fact the protocol can hold —
  after the quarantine rename the name is free, the world can occupy it, and it
  can give what it put there the quarantined root's own identity. Three
  formulations died on that one trace. **The correlation ticket is the only
  durable evidence a finish succeeded.** Model absence as something Grove
  *establishes and preserves*, never as something that *holds*, or this file will
  re-derive the same counterexample at its own cost. Recorded in
  `docs/formalism-findings.md` entry 039.
- **`SY-05.b` is a joint claim by construction.** The catalogue says `SY-05` and
  `FN-11`/`FN-19` "SHALL be checked together". `FN-11` and `FN-19` are the finish
  model's, and a `FN_`-prefixed command here is a placement failure. What this
  file owes is the *observation* — no trace exposes an absent task root before
  the deletion is proven — stated over its own transitions, with the finish
  model named as the owner of the underlying steps. Expect this to be the
  subtree's sharpest composition question.
- **The `TT-24` placement disagreement has a fourth consequence waiting here.**
  Two `TT-` obligations are declared gaps unfillable from either sibling
  directory, and `finish-k8` found a third instance in Q4's removal matrix.
  Wherever a `SY-` claim's content is really a `TT-` or `FN-` one, record it in
  the family README in the same shape rather than solving it —
  `formal-synthesis-k16` inherits the whole set and a fourth instance is cheap
  for it only if it is *stated*.
- **Three Alloy mutations are this scope's**, and no sibling discharges them:
  `EN-07` (premise-break, a shared-lock scope, controls `SY-11.b`), `EN-14`
  (premise-break, the working-tree root itself removed, controls `SY-01` — its
  `SY-05` half is named in the same row), and `EN-08` (exercise-removal, `crash`
  removed, whose controls include `SY-12`). `EN-08`'s sibling half was run in the
  finish scope over `FN-` witnesses; the `SY-12` half is unrun and is this
  subtree's.


## Done when

- `models/system/lifecycle.als` answers every obligation of `SY-01` – `SY-14`
  with a `check` and its required `witness_` runs, all green under
  `models/run.sh --scope lifecycle --family alloy`, with **coverage asserted**
  and zero empty alloy cells for the lifecycle scope — the moment
  `--no-coverage` leaves the README's run line.
- The three Alloy-owned assumption mutations are present as their own named
  commands with the expected result the assumption table states: `EN-07`
  (`SY-11.b` fails under a shared-lock scope), `EN-14` (`SY-01` fails when the
  working-tree root does not outlive the task root), and `EN-08` (`SY-12`'s
  crash-point witnesses become unreachable with `crash` removed, and every
  property stays green).
- `SY-13.a` records the **longest** admitted sequence within the bound and its
  length, and `SY-13.b` and `SY-14.a` are each an **exhaustive sweep** with its
  bound stated. All three are existential-reachability or sweep instruments
  rather than liveness properties, and the README says so where a reader would
  otherwise read fairness into them.
- `models/system/README.md` records tool version, bounds per command, solver,
  fairness assumptions, abstractions, deliberate omissions, what a green run
  does not prove, the **witness bound at which each obligation first lands**,
  every declared gap, the retained counterexamples, and the composition
  boundary — which `TT-`/`FN-` facts are imported as opaque observations and
  which sibling model owns each.
- One mutation per reported obligation, run before the green is believed, with
  **evidence that each mutation actually fires** — three sessions across the two
  sibling scopes produced a mutation the model's own facts made unsatisfiable,
  which reports exactly as a survivor does.
- Material observations are appended to Experiment 2 as entries 040 onward, with
  the six required fields plus the pre-registration's four additions.


## Decomposition

Twenty-five obligations against `task-tree-k7`'s forty-three and `finish-k8`'s
sixty-one — the smallest of the three scopes, because it composes rather than
restates. Cut along the **machinery** each claim group needs, which is how both
sibling scopes cut their levels. Each child leaves `lifecycle.als` green for the
obligations it claims and the runner able to say exactly which cells are still
empty, so no child is dead until its siblings land.

1. `admission` — the guard stack: `SY-01`, `SY-02`, `SY-03`, `SY-11`
   (6 obligations). Needs the lease, the layout preflight and its verdict, the
   launch generation, the tree guard, their order, a second process, and process
   death. Needs no task-root state beyond presence, no session, no finish, no
   configuration. Owns `EN-07` and `EN-14`.
2. `iteration` — the loop's own step: `SY-04`, `SY-08`, `SY-10` (5). Adds the
   iteration boundary, configuration validation ahead of every transition,
   selection taken once, the launch window, and generation staleness with its
   visible timeout.
3. `roots` — the task root's lifecycle: `SY-05`, `SY-06`, `SY-07` (6). Adds the
   root-state classification this scope reads, scaffolding and its interrupted
   subset, exhaustion, the driver-owned finish leaf, and absence as an
   established-and-preserved fact.
4. `sessions` — the ending, the crash and the sweeps: `SY-09`, `SY-12`, `SY-13`,
   `SY-14` (8). Adds the session step with exactly three endings, crash at every
   lifecycle point, the stable-state sweep and the `Blocked` persistence sweep.
   Owns `EN-08`, whose control is spread across the whole file and so runs last.

One departure from the catalogue's own order, for the reason both siblings gave
— a claim sits with the machinery its **witness** needs, not with the machinery
its statement mentions:

- **`SY-04` is in `iteration`, not in `admission`.** Its subject is the seven
  lifecycle actions `admission` introduces, but its witness is *each transition
  taken alone* across an iteration boundary, and `SY-04.b`'s configuration
  validation exists nowhere in `admission`'s machinery. Splitting it would mean
  checking the "at most one" half against a set the file had not finished
  populating.

Only the first child is cut now. Each session cuts the next as its last act,
once the file's actual shape at that point is known — the claim groups are fixed
by the catalogue, but which machinery each needs is not knowable until the file
exists.


## Pointers

- `docs/specs/semantic-contract.md` — §*Claims — system lifecycle* is this
  subtree's whole scope; §*States* fixes the root classification and the
  stable/transient distinction `SY-13` is stated over; §*Actions* fixes the
  seven-member Lifecycle group and the five Environment actions; §*Outcomes*
  fixes the closed refusal-reason set and the two blocked diagnoses `SY-14`
  refuses under; §*Environment assumptions* carries the three mutations this
  subtree owes; §*What the models must be able to decide* names `SY-05` as one
  of Q4's retained shared-safety claims.
- `crates/grove-finish/models/README.md` — the mutation matrix's six failure
  modes, the bound register's seven shapes and three vacuity predictors, and the
  declared-gap line shape the runner parses. Read it before writing a command.
- `crates/grove-task-tree/models/README.md` — the house style for a family
  README here, and the two retained false-confidence incidents.
- `crates/grove-finish/models/finish.als` — the house style for a temporal Alloy
  model at this size: a free initial state narrowed only where a phase would
  otherwise be a running transaction nobody started, claims as named predicates,
  every action's outcome a total function of its guard, every command pinning
  the assumptions it runs under.
- ADRs: `one-live-driver-per-working-tree` (`SY-01`),
  `supported-workspace-layouts` (`SY-02`), `bulk-marks-are-not-atomic`
  (`SY-11.b`'s cycle), `complete-session-configuration` (`SY-04.b`),
  `one-build-owns-a-session`.
- Glossary: *Driver lease*, *Workspace layout preflight*, *Session epoch*,
  *Tree access lock*, *Terminal disposition*, *Partial scaffold*, *Correlation
  ticket*, *Complete finish cycle*, *Obligation*. The first four are exactly
  `admission`'s three guards plus the shorter one they must not be confused
  with, and each carries an `_Avoid_` line that is a claim in this scope.


## Notes

Do not read the Quint side of Experiment 2, and do not open
`models/system/*.qnt` if one appears. The independence protocol holds until both
families are green.

`SY-13` is **existential reachability and deliberately not liveness**. Stating
it as *the loop will reach one* would need a fairness premise these models have
no grounds to grant — nothing schedules the operator, and `EN-15` says Grove
cannot verify a confirmation. A child that reaches for `eventually` where the
catalogue says *there exists a bounded sequence* has manufactured liveness by
omitting the hostile choices, which the node brief above names as the thing to
avoid.

If the model needs a filename, a position, a key, a digest, a manifest or a
quarantine internal, that is evidence the composition boundary above has been
crossed, not that the model needs a wider signature.
