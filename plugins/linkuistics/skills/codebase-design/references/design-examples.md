# Design examples and tradeoffs

Read this when a choice remains ambiguous after applying `codebase-design`.
These are Linkuistics examples, not examples reproduced from the authors. The
conceptual sources are mapped in [the provenance record](../../../PROVENANCE.md#codebase-design).
Signatures below are language-neutral sketches.

## Tradeoffs to resolve explicitly

| Tension | Decision rule |
|---|---|
| Deep modules and independent concerns | Group code that shares knowledge or invariants. Separate independently varying concerns. A deep public module can compose several internal modules. |
| Information hiding and transparent data | Expose stable domain facts when that simplifies composition; hide storage layouts and incidental representations. A map still has a schema contract, and a class does not inherently entangle concerns. |
| Internal work and caller convenience | Put common mechanics with their owner. Keep genuine caller policy explicit, including freshness, durability, and resource limits. Defaults must have documented semantics. |
| Useful generality and speculative extension | Prefer operations that simplify actual uses. Add extension protocols for concrete variation or a concrete test need; account for their configuration and compatibility costs. |
| Immutability and resource limits | Measure allocation, copying, and retention. When mutation is justified, specify its owner, lifetime, and synchronisation rather than relying on an immutable wrapper around shared state. |
| Temporal decoupling and operational cost | A queue can separate producer and consumer timing. It also introduces capacity, ordering, duplicate-delivery, cancellation, retry, and observation decisions. Use one when the required buffering or independent lifetimes justify them. |
| Design investment and delivery | Compare expected benefit with migration cost, remaining lifetime, and the task's scope. Retaining a sound existing design is a valid outcome. |

## A report that reads live state

**Situation.** `report.render()` reads storage and the clock, updates shared
totals, then formats text. Its call is short, but repeated calls can observe
different inputs and formatting cannot be tested without storage setup.

**Candidate.** Acquire a snapshot, then use
`calculate(snapshot, rules, asOf) -> ReportData` and
`render(data, format) -> Text`. An orchestrator owns acquisition and delivery.
Keep a convenient public operation where it saves callers work; it can compose
these pieces internally.

- **Ousterhout:** count the obligations of real callers. Requiring each caller
  to coordinate snapshots, rules, and delivery may make the interface worse.
- **Hickey:** check whether calculation now depends only on explicit values.
  An immutable parameter that contains a live database handle retains the
  dependency. An immutable snapshot also needs a defined consistency guarantee.
- **Feathers:** if acquisition cannot be supplied as values, identify the
  storage or clock seam and the enabling point selecting its test substitute.

**Counterexample and verification.** A complete snapshot may exceed memory
limits. Consider bounded input batches or a stream with an explicit transaction
lifetime, then check whether those semantics satisfy the report. Verify output
compatibility, acquisition failures, repeatability for fixed inputs, and the
absence of hidden reads during calculation. Immutability alone proves none of
the acquisition guarantees.

## An error condition callers need not handle

**Situation.** Every caller checks whether a job is active before cancelling it.
The job can finish between the check and cancellation.

**Candidate.** If the domain wants eventual inactivity, offer
`ensureInactive(jobId)` and let the scheduler own coordination. Define whether
success means the request was accepted or the job is already inactive, how
in-flight work is treated, and how completion is observed. Repeated requests can
have one documented result without forcing callers to repeat a racy precheck.

This applies Ousterhout's preference for removing avoidable error cases. Hickey's
analysis still asks who coordinates state transitions and which temporal
assumptions remain. The scheduler must implement the guarantee; a simpler name
does not establish it.

**Counterexample and verification.** Billing may need to distinguish completion
from cancellation. Preserve that distinction, along with authorisation and
scheduler failures. Test repeated requests, completion races, and work whose
side effects cannot be undone. Defining an acceptable inactive state is different
from catching every failure and returning success.

## A long, cohesive parser

**Situation.** A parser shares a grammar and mutable cursor among several
helpers. A locale formatter sits beside it. One proposal merges everything
because all the work is in-process; another splits every function at fifteen
lines.

**Candidate.** Keep grammar and cursor invariants under one owner and expose a
useful parse contract. Separate the formatter when its policy varies independently.
Extract parser helpers where they hide knowledge or establish useful contracts;
their length is evidence to inspect, not a decision rule.

Ousterhout's depth test concerns caller knowledge. Hickey's simplicity test
concerns entanglement. Neither says that fewer files, more methods, or local
execution alone establishes a better decomposition. If tokenisation has actual
independent consumers or invariants, it may earn its own internal module.

**Counterexample and verification.** A streaming consumer may require an
incremental interface and explicit cursor lifetime. Confirm the actual consumers
before replacing that contract with a whole-input parser. Preserve malformed-input,
randomised, algorithmic, and regression coverage. Move cases observable through
the new interface there; retain meaningful internal tests. Remove helper-call-order
assertions only after accounting for the behaviours they protected.

## A tool approaching retirement

**Situation.** A stable exporter will retire in six weeks. A generic framework
costs two days to introduce and is expected to save ten minutes on each of two
remaining format changes. The current task is a one-hour defect fix.

**Decision.** Make the focused fix and verify the affected contract. The stated
savings do not justify the migration. Revisit if retirement is delayed, new
consumers appear, or concrete defects show that the current structure obstructs
safe fixes. Do not invent a larger future to justify abstraction. This is a
Linkuistics application of design-cost accounting, not an author-prescribed
numeric threshold.

## Diagnostic patterns

Treat these as prompts for inspection, not automatic findings.

| Pattern | Inspect | Useful correction when supported |
|---|---|---|
| Pass-through layer | Does it preserve compatibility, translate a protocol, enforce policy, or provide a needed seam? | Remove a redundant hop; retain and document a real contract. |
| Information leakage | Do callers share representation knowledge or reconstruct the same algorithm? | Give the decision an owner and expose the required operation. |
| Decomposition by execution phase | Do read, validate, and process modules all know one format? | Group shared format knowledge; keep independent policies separate. |
| Mutable context object | Which operations depend on hidden identity, time, or previous mutations? | Pass explicit values where possible and give transitions an owner. |
| Universal manager | Does one module combine unrelated policies and lifecycles? | Separate independent concerns, retaining convenient composition where useful. |
| Configuration replacing design work | Must every caller choose internals that could have a sound default? | Internalise common mechanics and expose genuine policy choices. |
| Cosmetic simplicity | Is the evidence only line count, method count, a familiar API, or an immutable outer wrapper? | Trace caller obligations and actual dependencies before claiming improvement. |
