# harness-selection-and-execution-k6

## Goal

Cut independently useful implementation increments for harness-dispatch from the
reviewed design, preserving the first release's standalone command, supplied
policy, Grove integration and installed delivery acceptance.

## Context

Read `docs/specs/harness-selection-and-execution.md`, its cited ADRs and the
runtime/source evidence under `docs/design/harness-selection-and-execution/`.
Review `harness-selection-and-execution-k5` and any intervening integration own
design findings; use the resulting current design, not the producer's original
proposals when they differ. Integration `harness-selection-and-execution-k7`
sent the artifact-identity/original-creator area to design
`harness-selection-and-execution-k8` and its review; plan that area only from
their result. The root brief records accepted requirements and
the two process seams. No implementation tree has been pre-built.

## Done when

- The first executable increments can each be demonstrated or tested without
  waiting for a horizontal sibling to make them useful.
- Implementation coverage includes the actual shipped example policy, the
  original-creator mechanism k8 settles with the methodology amendments,
  conformance rows and pins that
  `docs/adr/a-review-carries-its-creator-reference.md` says ship with it, later
  outcome observation, bounded
  worker trust and cancellation, optional Grove slots, direct-harness
  compatibility and release layout.
- Package tasks join the repository Taskfile. Per-target installed static and
  TypeScript smoke tests without another runtime, Linux floor verification and
  documentation/configure-grove acceptance are explicit work with owners.
- Review work is scheduled where justified; no local selector pilot, calibration
  campaign, repository extraction or confined grove-run integration is added.

## Notes

`task check` wraps the existing repository check script. The design viewer runs
through `task design:harness-selection`. The native Bun experiment validates only
the bounded facts in its evidence document; it is not an implementation artifact
or permission to skip process and release checks.

## Decisions (running log)

**Increments are nodes of this grove, not separately created groves.** The
planning skill asks for a separate grove per working stage. Here each increment
is a dependency-ordered root-level node, or a single leaf when it fits one
session. The root brief charters this grove to deliver the whole first release,
and the repository's finish sequence releases once, at this grove's end. No
session can create a grove: nothing in `grove` or `grove-llm` creates one, and
one live driver owns each working tree. Precedent agrees: `modular-configuration-k4`
and `editorial-guidance-k3` planned the same way. Every increment boundary still
leaves `task check` green and Grove releasable. A human who wants an earlier
release can therefore finish at any boundary without unpicking work.

**Increment order.** Each stage leaves the product working and adds behavior
its successors build on:

1. Grove task slots.
2. Static standalone dispatch.
3. Installed delivery.
4. Computed policy and bounded context.
5. Execution records and observations.
6. The unattended-safe evaluation boundary.
7. Grove sessions launched through dispatch.
8. The supplied review policy.
9. The creator reference.
10. Documentation acceptance.

The slots come first because they depend on nothing and help any wrapper, not
only dispatch. Computed policy precedes records because policy run lookup
travels over the SDK host protocol that computed policy introduces. Records
precede the boundary work so that the linearization point is built once, with
the handoff commit already in place. Grove dispatch needs the slots, run IDs
and signal transparency. The creator lifecycle cases need both Grove dispatch
and the review policy.

**Delivery comes third, deliberately early.** The worker ADR reopens if the
compiled worker cannot meet the supported-target floor, and that floor is the
largest untested risk: glibc 2.17 and the emulated CPU models have never run a
Bun-compiled worker here. Finding out after five more increments would waste
their SDK and protocol work. Once delivery lands, the per-target installed
smoke task also becomes the regression instrument. Later leaves that change
the worker, the layout or the native dependencies must rerun it. The computed
increment adds the TypeScript `select` smoke case, and records adds bundled
SQLite's C build to a formerly pure-Rust cross-compile.

**Cut every increment's leaves now.** The fog-or-ticket test cuts leaves now: the spec states every
contract precisely, so every leaf's question can be phrased today, and the
root brief requires explicit owners for each obligation. The alternative was a
planning-continuation leaf per node. That would cost six more sessions only to
re-read the same spec. A leaf that proves too big decomposes. A late concern
goes in with `leaf-insert`.

**Review is scheduled at two levels.** The decomposition is load-bearing: a new
subsystem, its trust boundary and its release route will be built on it for
weeks. This session therefore ends by inserting a `review-planning` leaf
directly after itself, ahead of every increment. For implementation, the node
briefs say where a `review-impl` is expected and why. The expectations fall on five
nodes: `static-dispatch-k12`, `dispatch-records-k23`, `evaluation-boundary-k27`,
`review-policy-k35` and `creator-reference-k38`. In each, the node's last leaf
cuts the review inside the node, naming the node's handle, so the node cannot
close before review. `dispatch-delivery-k16` and `grove-dispatch-k31` carry no
expectation, because their per-target and PTY executions are the instruments. Delivery and the documentation leaf carry their own
instruments: per-target execution and a mandatory documentation-acceptance
review.

**Tests never skip for a missing worker.** Command-seam and Grove-boundary tests
need the compiled worker. The first skeleton leaf decides how `cargo test` and
`task check` obtain it deterministically. Its absence must fail loudly, since a
skipped acceptance test reads exactly like a passing one.

**Coverage walk.** Each cell in the spec's `#test-seams` table, each root
acceptance case, and each item in this leaf's `Done when` was walked to an
owning leaf. The walk found two gaps, now closed. A missing-import refusal went
to `routed-inspection-k13`. The rule that inspect is not side-effect-free went
to `choice-and-refusals-k15`'s documentation. The walk added no local
selector, calibration, extraction or confined `grove run` work. A reviewer can
repeat the walk from the leaf bodies, because each names the spec rows it owns.
