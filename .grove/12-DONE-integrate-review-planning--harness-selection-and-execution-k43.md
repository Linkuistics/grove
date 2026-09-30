# harness-selection-and-execution-k43

**Integrates:** harness-selection-and-execution-k42

## Goal

Triage the findings from `harness-selection-and-execution-k42` against the
settled first-release design, and reconcile the implementation plan before
`grove-task-slots-k11` runs.

## Context

Read that review's task and focused commit, the producer
`harness-selection-and-execution-k6`, the root brief and the reviewed subtree.
The specification and its ADRs remain the design authority. This integration
owns planning repairs and their verification, not implementation of the feature.

## Done when

- Every finding has an explicit evidence-based disposition in this leaf's
  running log.
- The actionable dispositions are reflected coherently in the relevant task
  bodies, node briefs and root implementation plan. Their dependencies and
  completion boundaries agree with the settled design.
- The acceptance coverage and independently demonstrable increment boundaries
  are reconciled with any tree changes. Further substantial planning review is
  scheduled if warranted by the integration procedure.

## Notes

The review's handle is the handoff; its findings are not copied into this
charter. Resolve stable handles after any renumbering. This leaf was inserted
immediately after the review, before the first implementation increment.

## Decisions (running log)

Read the findings from review commit `d7a505c058ea`. Each was checked against
the reviewed subtree, `docs/specs/harness-selection-and-execution.md`,
`docs/adr/a-review-carries-its-creator-reference.md` and the spine's
`references/decompose.md`. The spec and ADRs are unchanged. No in-session
reviewer was spent.

**F1: real issue, fix the tree.** A review cut inside its producer node keeps
that node open. The spec says a node that stays open writes nothing
(`#identity-and-creator`). So the node's finisher would be the session that
retires the review, or its integration, after the review selection that needed
the reference. The flat review chain in `references/decompose.md` makes a
review its producer's sibling, not its child. The repair keeps review ahead of
the next increment. The leaf whose retirement closes the node now cuts the
review as that node's root sibling, with `grove-llm leaf-insert` at the first
root entry after the node. Five node briefs and five last acts change. The
documentation review from `dispatch-documentation-k41` already follows a leaf
producer at the root.

**F2: real issue, fix the tree.** Only the Rust-side deadline ends a worker that
is spinning synchronously, or one awaiting a promise that live asynchronous work
keeps pending. Static `routes` policy already executes TypeScript at import.
Increments 2–5 were planned to leave such a policy running forever. The
repair adds `selection-deadline-k44` to `static-dispatch-k12`, after
`harness-exec-k14` so its tests can show that `run` launches nothing. It owns
`--timeout-ms`, the hard kill after at most one second of grace, reaping and
exit 124. `computed-selection-k21` and `bounded-context-k22` add those two hang
shapes for `select` and `loadContext`. A promise stranded when the event loop
drains keeps its own refusal. `selection-cancellation-k28` keeps signal
cancellation and the post-result checks.

**F3: real issue, fix the tree.** The spec makes the pre-exec record a
precondition of every run (`#records-and-outcomes`). The plan still had
`static-dispatch-k12` close with a released `run` that has no record. The minimum handoff is
`handoff-records-k24`. It needs nothing from computed policy, because run lookup is `run-lookup-k26`'s, and nothing
from the evaluation boundary, which appends to its commit later. The review
confirmed both orderings. So k24 moves by hand, key preserved, into
`static-dispatch-k12` before `choice-and-refusals-k15`. No increment boundary
then exposes `run` without its record. Delivery now builds and smokes bundled
SQLite from its first cross-build. That strengthens the early probe of the
supported-target floor. `dispatch-records-k23` keeps observations and run lookup.
Releasability is claimed at increment boundaries, not at leaf boundaries
inside a node. The root brief now says so.

**F4: visible trade-off, no cut.** `routed-inspection-k13` is the largest leaf,
and its worker-acquisition decision is delegated to it on purpose. No
inspection shows that it cannot fit one session, so a speculative split would
break the laziness rule. The F2 deadline is a separate leaf and does not grow
k13. k13's notes now name the behavioral seam to use if it decomposes. The
`ambient-authority-k30` risk is lower and unchanged.

**F5: visible trade-off, already accepted.** The planning rule literally asks
for a separate grove per stage. `harness-selection-and-execution-k6` recorded
why these stages are nodes: one charter, one first release, and a single
release at finish. That log lives in a retired leaf outside every brief chain.
The root brief's implementation plan now states the deviation and its reason.

**Coverage after the repair.** No acceptance obligation was dropped or
duplicated. Owners changed as follows:

- Interruption and timeout: the deadline leaf, k21 and k22 own the timeout
  cases, and k28–k30 and k33 own the signal cases.
- Required record: k24 inside static dispatch, then k25, k26 and k29.
- Release and install: k17 and k18 build, notice and smoke bundled SQLite. k24
  keeps only the host build.
- Authority-and-lifecycle seam row: timeout moves to the deadline leaf, k21 and
  k22. Interruption stays with k28.
- Records seam row: k24 (commit failure, attempt versus exec failure,
  pre-commit refusal), k25, k26, k29, k37 and k40.

The other rows keep the owners in the review's coverage table.

**No further planning review.** These are repairs inside the reviewed
decomposition: two obligations move earlier and five review cuts move one
level out. That is not a rethink. The `review-impl` of `static-dispatch-k12`
inspects the enlarged foundation node, and its doubts now name the deadline and
the handoff commit.
