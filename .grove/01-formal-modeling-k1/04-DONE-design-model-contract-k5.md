# model-contract-k5


## Goal

Define one tool-neutral semantic contract and claim catalogue that the independently built Quint and Alloy models can both test.



## Context

The contract is an abstraction boundary, not a pseudo-implementation. It must distinguish task-tree semantics, finish/recovery semantics, the external VCS/filesystem environment, and the system lifecycle while retaining enough detail to represent every load-bearing concern in `TODO.finish_process.md`.

## Done when

- Shared vocabulary precisely defines identities, task states, root states, finish-attempt identity, confirmation intent, artifacts, observations, stable/transient states, and terminal outcomes.
- Actions and total refusal outcomes are named independently of tool syntax. `RecoveryPending` means a correlated Grove-owned attempt is incomplete; `OwnershipConflict` means state is unrelated, ambiguous, or cannot be proved safe to mutate.
- Git, native jj, and colocated jj have explicit environmental assumptions and lane-specific obligations while sharing one abstract outcome contract.
- The finish claims cover confirmation, persisted intent, external correlation ticket, witness, evacuation before `.grove` deletion, quarantine, branch/bookmark preservation, merge/removal, crash/restart, idempotent recovery, no unrelated mutation, and both successful exits.
- The task-tree claims cover name/ordinal/format invariants, selection, mutation, terminality, root identity, fail-closed ownership, and the boundary delegated to `ordinal-fs-tree`.
- The system claims connect completed sessions, exhaustion, explicit finish entry, interruption, recovery, and root absence.
- Exact model paths, runner entry point, claim identifiers, evidence format, and experiment logging convention are fixed. Neither model family needs to invent a semantic decision.

## Notes

Do not encode Rust module names or current helper functions as state. Do not read or alter product implementation while defining the contract; use the approved requirements, baseline, current public behaviour, and `TODO.finish_process.md`.

## Decisions (running log)

**Finish keeps today's contract; the brief's merge-and-remove exit is recorded
as a non-goal, not modelled.** Put to the human, who chose it over modelling both
exits and over carrying merge-and-remove as a flagged optional scope. The root
brief's Notes ask finish to "make both successful exits explicit: preserve the
branch/bookmark, or merge and then remove only the proved-owned
branch/worktree", which no shipped behaviour supplies — `docs/ARCHITECTURE.md`
says the user owns topology and Grove reads no branch or bookmark, and
`CONTEXT.md`'s `Complete finish cycle` carries an explicit `_Avoid_` against
describing finish as merging anything git-topological, naming that as the pre-v11
cycle. Three further pieces of evidence went into the question:
`TODO.finish_process.md`'s fourth grilling question pushes toward *less*
machinery rather than more; the root brief's pointer to GitHub issue #13 as
"source context for the finish-process work" is wrong on inspection — #13 is
*Extract tree-on-disk facilities as a library*, the `ordinal-fs-tree`
extraction, and says nothing about finish; and the preservation ledger's "never
resets, merges, deletes, or rewrites work it cannot prove belongs to the current
finish attempt" reads as a list of dangerous verbs rather than a promise that
Grove performs any of them. The consequence is recorded in the spec's
`## Out of scope` rather than only here, so `formal-synthesis-k16` meets it in a
durable artifact; reopening it is a brief change plus a rework of
`task-tree-transactions-fail-closed` and the glossary entry, never an inline
widening of a model.

**One seam: the repository model runner, asserting claim coverage in both
directions.** Put to the human, who chose it over adding a Rust conformance test
beside it and over deferring the crate-facing seams wholly to
`formal-synthesis-k16`. `SPEC-FORMAT.md`'s three seam rules — prefer an existing
seam, propose a new one at the highest point, drive the count toward one — all
point the same way here, and the repository already runs models through
`docs/ordinal-fs-tree/models/run-alloy.sh` and `run-quint.sh`, whose pass/fail
conventions the new runner adopts rather than reinvents. Coverage is asserted
both ways because the pre-registration names *the absent suite* as a live
hazard with "a model file no runner reaches is a runner defect, asserted as
such": one direction catches a claim nobody modelled, the other catches a model
command answering to no claim. Findings that must reach the product land in the
existing black-box test binaries; this phase invents no Rust seam, which is also
what keeps `Do not alter product behaviour in this phase` true.

**One catalogue at `docs/specs/semantic-contract.md`, not three scoped ones.**
The three scopes share identities, states and outcomes — a finish claim is
stated about the same tree the task-tree claims are about, and the lifecycle
claims are about the joint — so splitting by scope would put the shared
vocabulary in one of the three or, worse, in all three. The second is exactly
the drift the catalogue exists to prevent, and it is the drift a comparison
between two independently built model families would then be measuring instead
of the formalisms. The scopes are sections; the claim identifiers carry the
scope, so a model file can still declare which subset it covers. Recorded in
`CONTEXT-MAP.md` as a **grove**-owned record: it consumes the `ordinal-fs-tree`
boundary in grove's vocabulary and states no claim in the library's, which is
the map's own discriminator.

**Claim identifiers are the cross-reference key, and coverage is asserted both
ways.** `TT-nn`, `FN-nn`, `SY-nn` for claims and `EN-nn` for assumptions,
hyphenated in prose and underscored in model identifiers because neither
language admits a hyphen. Sixty-nine claims and sixteen assumptions, contiguous
and unique. The command-naming convention is the two existing
`docs/ordinal-fs-tree/models/` runners' — `check`/`witness_` for Alloy,
`inv_`/`wit_` for Quint — rather than a third invented here. A family that
cannot express a claim **declares** it in its `README.md` with the reason, and
the runner counts a declared gap as covered and reports it, so *not modelled*
and *forgotten* never look alike. Coverage runs in both directions because a
one-way check passes a model that has grown a command answering to no claim,
which is how a model quietly becomes its own specification.

**Every claim carries a witness obligation, and every assumption a relaxation
owner.** The pre-registration names *the vacuous invariant* and *agreement
mistaken for proof* as live hazards, and the catalogue is the single common
ancestor of both families, so both controls have to be built into the contract
rather than left to each model's judgement. Each claim names what must hold and
what must be reachable; each of the sixteen assumptions names the family that
relaxes it and the instance that does so. Three assumptions — interruption,
hand-editing, and the three lanes — are marked *exercised rather than relaxed*,
because their negation is not a smaller world but the model this experiment
exists to avoid; for those the control is that the dependent claims must fail
when the action is removed.

**The four `TODO.finish_process.md` questions are mapped to deciding claims.**
`formal-synthesis-k16` must classify every proposed finish simplification keep /
delete-replace / defer with evidence, and its brief rules out "the model is
smaller" as evidence. So the catalogue fixes which claims decide each question
and what a *delete/replace* verdict would have to look like, and states that an
unreached deciding witness is **defer** rather than delete — reading an absence
of evidence as evidence of absence is the vacuous-invariant habit under another
name. That makes the phase gate mechanical instead of argumentative.

**Five finish terms went into `CONTEXT.md`; two candidates did not.**
Finish-attempt identity, evacuation manifest, correlation ticket, finish
disposition, the recovery-pending/ownership-conflict pair, and quarantine are
domain concepts every later finish conversation needs, and the catalogue is
unreadable without them. *Claim identifier* and *claim catalogue* were rejected
on `CONTEXT-FORMAT.md`'s "it is a glossary and nothing else" — they name this
workstream's process vocabulary, not grove's domain, which is the same test
`experiment-baseline-k4` applied to *preservation baseline*. Repository anchor
and deletion fingerprint are folded into the evacuation-manifest entry rather
than given entries of their own; they are components of that record and have no
independent life.

**No ADR was written, and none was reworked.** `ADR-FORMAT.md`'s test is an AND
of three conditions. The merge-and-remove exclusion is surprising without
context and settles a real trade-off, but it is not hard to reverse — reversing
it is a decision to model a scope, not an unwinding of shipped code — and it is
not a *new* decision either: `task-tree-transactions-fail-closed` and
`docs/ARCHITECTURE.md` already record that Grove performs no integration. What
is new is declining to reopen it here, which is scope and lands in the spec's
`## Out of scope`. The `RecoveryPending`/`OwnershipConflict` partition is not yet
settled — the models decide whether it is total, disjoint and reachable, and
`formal-synthesis-k16` decides adoption — so recording it now would be an ADR
for a decision nobody has taken. Nothing in this session changed a decision an
existing record holds, so the set needed no rework and no citation was left
dangling.

**The runner is built by the first model family and extended by the second.** A
runner is not a model, so building it jointly costs the independence protocol
nothing — what neither family may read before both are green is the other's
*models*. Stated in the spec because otherwise both leaves inherit an instrument
neither is told to build, and a runner assembled afterwards from two scripts
that already disagree is the *absent suite* hazard arriving by a different road.

**A `review-design` step was cut, and inserted ahead of the model leaves rather
than appended.** The pre-registration names the catalogue as the single common
ancestor of both families and *agreement mistaken for proof* as the hazard that
follows, which is precisely a doubt an in-session reviewer cannot discharge: the
enumerated-assumption control is itself part of what needs challenging.
`decompose.md` earns a chain for "a landed spec", and appending at the node's
end would put the review after `alloy-models-k6` and `quint-models-k10` have
been written from the artifact under review — the same timing argument
`experiment-baseline-k4` made for inserting its own review ahead of this leaf.
The review's body names the three doubts this session could not resolve about
its own work.
