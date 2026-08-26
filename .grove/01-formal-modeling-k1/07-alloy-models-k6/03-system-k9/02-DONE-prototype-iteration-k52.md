# iteration-k52


## Goal

Add the loop's own step to `models/system/lifecycle.als`: `SY-04`, `SY-08` and
`SY-10` — five obligations. The iteration boundary, configuration validation
ahead of every transition, selection taken once, the launch window, and
generation staleness with its visible timeout.


## Context

- `docs/specs/semantic-contract.md` §*Claims — system lifecycle* `SY-04`,
  `SY-08` and `SY-10`; §*Actions* for `launch` and `reap`, the two Lifecycle
  actions `admission-k51` deliberately did not model; §*Outcomes* for
  `EpochStale`, the refusal reason `SY-10.a` names.
- `models/system/README.md` is the first thing to read. It carries the
  composition boundary, the two false-confidence incidents this file has already
  produced, the mutation matrix and the witness first-landing bounds. **Read the
  two incidents before writing a fact**: both were a construction fact that had
  absorbed the claim it sat beside.
- `CONTEXT.md` *Session epoch* — the launch-generation binding, what an ambient
  command must match, and the explicit `_Avoid_` that it is workflow consistency
  among cooperating processes and not authentication.
- The measuring invocation is
  `models/run.sh --scope lifecycle --family alloy --no-coverage`.


## Done when

- `SY-04.a`, `SY-04.b`, `SY-08`, `SY-10.a` and `SY-10.b` each have a `check` and
  their required `witness_` runs, all green, and the runner reports exactly
  fourteen empty alloy cells for the lifecycle scope.
- `SY-04.a`'s witness is **each transition taken alone**, which is a witness per
  lifecycle transition rather than one witness — and the transition set is the
  one this file has when the slice lands, `launch` and `reap` included.
- `SY-08`'s witness is a leaf inserted **during the launch window**, so the model
  needs the window to be a state a trace passes through rather than an atomic
  step; a model in which selection and launch are one transition answers the
  claim by construction.
- `SY-10.b`'s timeout is a **visible stop**, not a silent park. It is the second
  place in this scope where a wait has to be observable, and `Proc.waits` /
  `Deferred` — `admission-k51`'s declared abstraction — is where it lands or is
  deliberately not reused. Say which in the README.
- One mutation per reported obligation, each with evidence that it actually
  fires. Two of `admission-k51`'s six were unkillable; expect the same question
  and answer it the same way, in the matrix rather than by a fourth attempt.
- `models/system/README.md` extended: bounds and witness first-landing per new
  obligation, any new abstraction, and any new declared gap in the shape the
  runner parses.
- Experiment 2 entry 041 is appended with the six required fields plus the
  pre-registration's four additions, including the M5 authoring figure — this
  scope is the H8 comparison's system-level arm and `admission-k51` contributed
  its first point (0.42 h/obligation).
- The next leaf (`roots`, `SY-05` – `SY-07`) is cut as a sibling, its body
  carrying the machinery question this session's file actually leaves open.


## Notes

**`seen` must be reset, and this is the leaf that owns it.** `Proc.seen` records
the guards a process has acquired and `admission-k51` never resets it, on the
stated grounds that within its bounds a process runs one admission cycle and
that resetting it would be modelling the loop's iteration. That is exactly this
leaf's subject. **Reset it at the iteration boundary and re-measure every
inherited witness**: `seen` is read by `ordered`, so it is a field a guard
consults, which by the finish scope's rule owes the full witness sweep rather
than a monotonicity argument.

**Two order clauses are currently unexercised, and resetting `seen` is what may
exercise them.** The grant site's and the take-tree site's `mayTake` clauses both
survive mutation today — a grant cannot violate an order the wait already
satisfied, and the take-tree clause needs a re-acquisition nothing admits. Once
`seen` resets, a re-acquisition exists. Re-run M5 and M5b from the matrix and
record whether they now fire; if they do, the clauses stop being belt on
fastened braces and the README's paragraph changes.

**`SY-04.b` is the reason `doValidateConfig` exists at all.** It is opaque in
`admission-k51` — one action with no content, present only so `SY-02`'s *before
configuration validation* had something to be before. Its content is this
leaf's: full validation precedes every transition, so an invalid configuration
leaves the working tree **byte-identical**. The tree is *present or absent* in
this scope, so *byte-identical* has to be read at that grain and the reading
recorded, or the claim will be answered by a model that cannot see a byte.

**`SY-04.a` and `SY-11.a` are both quantified over the transition set, and this
slice adds to it.** `SY_11a_every_acquisition_site_applies_the_guard_order` is
written over *every guard newly seen*, deliberately, so that a sixth acquisition
site does not silently escape it. Check that it still holds after `launch` and
`reap` land, and treat a new acquisition site as a reason to re-run M6a.


## Decisions (running log)

**The iteration boundary is an action of this file's own (`IterA`), and it
clears three things together.** §*Actions* has no boundary in it, correctly — a
boundary is not something the loop *does* — but `SY-04.a` counts transitions
*per iteration* and a count needs an edge to be taken between. `doIter` clears
`spent` (the turn's one Lifecycle transition), `sel` (which is precisely why a
leaf added during the launch window is the next iteration's work) and resets
`seen`. Declared as an abstraction in `models/system/README.md` beside
`Proc.waits`.

**`seen` resets to `p.holds`, not to nothing.** `HeldImpliesTaken` is a
construction fact, so emptying `seen` while the driver still holds its lease
across the boundary would make the boundary unsatisfiable for exactly the
process whose loop it is. The reading is that `seen` records the guards taken
*in this iteration*, and an iteration begins holding whatever the last one did
not release.

**`acquire-lease` is exempt from configuration validation, and the exemption is
`SY-02`'s word rather than a convenience.** *An unsupported workspace is refused
at lease acquisition, before configuration validation* — so the one gate that
runs before validation cannot be gated on it. `SY_04b`'s second conjunct states
the exemption explicitly (`LifecycleAct - AcquireLeaseA`) rather than leaving it
implicit in the transition, because a reader who did not know would read the
gap as an oversight.

**The epoch record models only its rotation write.** The ADR has three write
points; this file has one — `open-epoch` writes the record active with a fresh
identity. The two *inactive* writes are collapsed away on the glossary's own
grounds that rotation is the stronger mechanism. `no World.gen` stays reachable
as a free initial state, so an inactive record is unwritten rather than
unrepresentable.

**`SY-02`'s declared session seam is closed by `SY-10.a`, and NOT by widening
`SY_02`.** Widening would have been false: a driver may release its lease while
a generation it opened is still live, so *some driver holds a lease* is not an
invariant. The session half is `SY_10a`'s third conjunct (an ambient tree
operation happens only while holding an epoch guard) composed with its second (a
session acquires one only at a matching generation).

**The grant site is a SECOND admission site for the generation match**, which
the catalogue does not say and this file found by checking it. A stale session
never waits, so a *waiting* session was fresh when it asked — but the record can
rotate while it is blocked, and a grant that only resumes the wait admits a
session whose generation is no longer live. The ADR settles it: *shared-guard
acquisition is the admission boundary*, and a grant is an acquisition.

**`SY-10.b` is stated without a clock, and `Proc.waits`/`Deferred` is reused
rather than duplicated.** §*Deliberate omissions* models clocks, timeouts and
retry counts as non-determinism, so `doTimeout` is non-deterministically enabled
and nothing here says the timeout *will* fire. *Never a silent park* is stated
as: a generation wait ends only in the waiter's own step, and that step reports
something.

**Two visible stops the catalogue's closed outcome set cannot name — `Stopped`
(`SY-10.b`) and `RefConfigInvalid` (`SY-04.b`) — are declared as this file's
abstractions and named for `formal-synthesis-k16`,** not added to the catalogue.
A new member of a closed set imposes a matching outcome on the Quint column,
which is the same reason entry 039's `SY-05` constraint and entry 040's missing
`EN` row were recorded rather than acted on. Written up as entry 041.

**`SY_10a` runs at seven states, and the number is a correction.** It was
written at five, was green, and its conjunct-2 mutation survived at five and at
six. An `M8` false-confidence incident, recorded as the family README's third.

**M5 and M5b both still survive, and for opposite reasons — both established by
a differential probe rather than asserted.** M5b (take-tree) is now a *live*
mutation `SY_11a` cannot see, because the check quantifies over `p.seen' -
p.seen` and a re-acquisition adds nothing to `seen`. M5 (grant) is still inert,
and the mechanism is now nameable: `doIter` is guarded on `no p.waits`, so a
blocked process cannot cross an iteration boundary. Neither is fixed here —
`SY-11.a` is covered and no `SY-` obligation states anything about
re-acquisition; `formal-synthesis-k16` inherits it.
