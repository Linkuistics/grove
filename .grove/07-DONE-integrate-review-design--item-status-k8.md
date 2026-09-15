# item-status-k8

**Integrates:** item-status-k7

## Goal

Triage the `item-status-k7` design review's findings against
`docs/specs/item-status.md` and the process-death sections of
`docs/adr/one-live-driver-per-working-tree.md`, and apply the real ones so the
design the `item-status-k4` planning step decomposes is the corrected one.

## Context

Read the findings from the review's own commit, not from this body: they are
in the `## Findings` section of `item-status-k7`'s leaf, each anchored to
`path:line` in the producer's commit `ab1b760685337392feb30aef4e5b3cd29171f9bc`,
with the evidence and the classification the reviewer proposed. Grade each one
yourself as a contract stated unclearly, a real issue, a visible trade-off, or
noise; the review's "What held" list says which attack axes were checked and
passed, and records the real kernel line numbers it read, so they need not be
re-derived. The root brief carries the human-agreed requirements and test
seams.

The spec and ADR are prose; nothing in this leaf runs the viewer or driver.
The spine's `ADR-FORMAT.md` governs any rework of the ADR (edit in place, keep
the set minimal, reconcile every citation). A finding that demands the
observation model be rethought rather than repaired becomes a new producer
review chain beside this leaf, not a fix here; the review proposed none such.

## Done when

Every finding in the review has a recorded disposition in this leaf's running
log, the accepted ones are applied to the spec and ADR with citations
reconciled, the spec and ADR still agree with each other and with the root
brief, and the waiting `item-status-k4` planning step can read a design that
needs no further correction before slicing.

## Notes

The integration may spend one narrow in-session reviewer if a fix is
substantive and non-mechanical; it need not. If it opens the external kernel
links, record that the ranges were read rather than only the local anchors.

## Decisions (running log)

- Read the findings from review commit `7405fd1f`'s retired `item-status-k7`
  leaf and graded them against the spec, ADR and root brief.
- **F1 — real issue, accepted.** Opened the tagged external files and read the
  numbered ranges, not merely the local anchors: Linux v6.12 `fs/file.c`
  398–428 and `fs/file_table.c` 405–445; XNU xnu-11215.1.10
  `bsd/kern/kern_descrip.c` 1018–1061 and `bsd/vfs/vfs_vnops.c` 1814–1850,
  795–850. They contain the claimed descriptor traversal and lock-before-reference
  release. Corrected all five links and named `fdt_invalidate` accurately.
- **F2 — contract stated unclearly, accepted.** Spec preparation follows
  exclusive invalidation; observation step 3 rejects an inactive epoch before
  step 4 probes. Those windows exclude viewer contention during preparation.
  Replaced the ADR's contrary claim with that ordering argument and retained
  foreign-holder failure as the actual conservative outcome.
- **F3 — contract stated unclearly, accepted with a visible trade-off.** Step 5
  derives Idle from a successful shared probe, so silently ineffective locks
  cannot guarantee Unavailable. Narrowed the spec and scenario to reported
  errors and stated the backend precondition, including the possible Idle/NEXT
  consequence outside it. Mirrored this in the ADR. Declined a preparation
  self-check: it could detect failed exclusion but would not validate the
  lifetime and teardown semantics this design also requires. This clarifies
  the existing native-backend boundary without changing the protocol.
- **F4 — contract stated unclearly, accepted.** The real-process replacement
  control need not force inode reuse and therefore cannot establish pinning's
  non-reuse property. Stated its identity-comparison/binding scope and linked
  the pin assumption to the ADR's same-object final-close evidence, explicitly
  noting the lack of a separate non-reuse test. The forced-reuse barrier and
  mutation controls remain required. No filesystem-specific allocator claim
  is needed for this disposition.
- **Validation and handoff.** Read the final diff against the root brief and
  observation steps; validated all eight local links/fragments in the two
  design documents and checked their trailing whitespace. External evidence
  was checked as recorded under F1. Graph generation `2026-09-15T11:44:22Z`
  excludes `docs/` and reports changed/untracked prose metadata, so this
  integration relies on direct document reads, not graph completeness. No
  runtime tests or in-session reviewer were needed for these prose repairs.
  The waiting `item-status-k4` already points to the current spec and correction
  integration; no citation rename or parent close is needed while it is live.
