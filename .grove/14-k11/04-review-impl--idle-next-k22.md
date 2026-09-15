# idle-next-k22

## Goal
Adversarially review the shipped typed tree/runtime observer against
idle-next-k11 and typed-observation-k18, using producers captured-tree-k20 and
bounded-runtime-k21 as the authoritative implementation artifacts.

## Context
Read docs/specs/item-status.md, the ancestor contracts and both producer commits.
The observation operation is consumed by real browsing. k19 separately owns
visible activity and independent two-capture runtime acceptance; inspect that
increment too if it has landed before this review runs. Witnessed RUNNING belongs
to witnessed-activity-k12 and is not claimed by legacy-active compatibility.

## Done when
- Verify independent tree/activity results, exact namespace isolation and aliases,
  mandatory parser reuse, Idle/Busy/Unavailable classification and no lease probe.
- Attack descriptor/path races, including workspace-alias changes around discovery,
  record replacement before/after lock and read, wrong types, unreadable controls,
  the 64 KiB bound and eight-attempt bound. Check nonblocking/read-only/CLOEXEC
  acquisition and filesystem preservation, including administration bytes.
- Verify tree capture precedes the runtime guard and no advisory guard escapes;
  assess the after-capture and in-epoch pause controls, concurrent shared readers,
  30-second deterministic handoff bound, recovery and stale admission tests.
- Check usage/seam prose and source-derived books against the shipped behavior.
  Principal checks are evidence, not substitutes for an adversarial read.
- Record findings with source anchors and severity. Create adjacent
  integrate-review-impl with bare stem idle-next only for actionable findings;
  its brief references this review rather than transcribing findings.

## Notes
No competing in-session reviewer ran. The graph generation predates these
producers; refresh coverage or use current source for changed/new modules.
