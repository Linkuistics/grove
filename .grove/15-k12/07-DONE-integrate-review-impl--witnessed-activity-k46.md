# witnessed-activity-k46

**Integrates:** witnessed-activity-k45


## Goal

Triage the findings of the complete witnessed-activity protocol review and
apply the real ones, so witnessed-activity-k12 can close on a green principal
gate.



## Context

Read the findings from witnessed-activity-k45's own commit and task file; this
body carries only the handle. The review found no correctness or security
defect in the protocol; what it recorded is low-severity and concerns test
isolation in `crates/grove-loop/src/driver_lease/observation.rs`, one
reader-side grammar check in the same file against the writer in
`driver_lease/witnesses.rs`, and one clause of `docs/specs/item-status.md`.
The parent brief and `docs/specs/item-status.md` remain the contract; k37's
frozen native evidence applies to the shipped protocol source unchanged.

## Done when

- Each finding is verified against the source and classified as a contract
  stated unclearly, a real issue, a visible trade-off or noise, with the
  classification and reason recorded in this task's running log.
- Every finding classified real is applied within the review's stated repair
  shape; anything larger than a repair is cut as a new producer review chain
  beside this leaf rather than done here.
- Affected fixtures, source-derived walkthrough fragments, indexes and
  manifests are reconciled in the same change where a repaired file is a book
  root; `bash scripts/check.sh` passes after all edits.
- The k12 brief's Done when is re-checked against the delivered subtree and
  the node is closed or the missing work is named as a leaf.

## Notes

Do not add an in-session reviewer beside this scheduled integration. No
production protocol change is expected from these findings; if a real one is
needed, stop and say so rather than redesigning under this leaf.

## Decisions (running log)

- Read k45's findings from commit ce900578 and checked the current source.
  Graph generation 2026-09-15T11:44:22Z is stale for these files (the observation
  and witness modules are not tracked); direct source reads supply the evidence.
- F1 — real issue. Each of the eleven named tests can observe the epoch after
  releasing an exclusive descriptor. A sibling fork can retain that descriptor
  until exec and turn the expected result into Busy. Reuse the existing exact-test
  subprocess wrapper before fixture creation in each named test. Preserve all
  lock assertions and production code; the existing tests are the verification
  seam, so no implementation-mirroring test is added.
- F2 — accepted visible trade-off. The writer and cleanup use witness- plus 32
  lowercase hex digits; the reader deliberately enforces the spec's plain
  basename rule, not that narrower grammar. A corrupted record can name a
  control file and, with its matching identity, reach a shared probe there.
  This can transiently interfere with replacement acquisition. The ADR explicitly
  excludes repository-control corruption and impersonated witnesses; cooperating
  writers cannot emit such a name. Retain the shipped protocol and its frozen
  native evidence rather than expand its corruption boundary in this integration.
  Reopen if control-record corruption becomes supported input. No claim is made
  that the current reader reserves driver.lease against such records.
- F3 — contract stated unclearly. observe_private validates, reads and probes
  the same open file; no second open occurs. Clarify step 5 to require that
  validated descriptor, independently opened from the driver's descriptor.
- Reconcile the existing grove-loop walkthrough under its established authoring
  contract: update literal test fragments and derived source ranges/counts, and
  explain the isolation without changing its audience or scope. No extra review
  is commissioned, as this leaf explicitly forbids one.

## Verification and closure

The source comparison against ce900578 establishes that the complete Rust diff
is exactly eleven insertions of the existing three-line isolation wrapper.
Production bytes before cfg(test) are identical. F1 is derived from those
fixture/epoch lifetimes and k42's recorded failing matrix execution, rather
than claimed as a newly reproduced intermittent failure. All existing assertion
bodies remain intact. No new Linux execution is claimed: k37's exact frozen
platform record remains evidence for the unchanged production protocol.

`cargo test --locked -p grove-loop --lib driver_lease::observation::tests --
--test-threads=8` passed all 33 cases. The first focused book validation found
three stale roll-up figures in chapter 21; those were reconciled before the
principal run. `bash scripts/check.sh` then exited 0: all eight checks passed,
including workspace tests and six final book validations. The grove-loop book
reconstructs 16 roots / 14,477 lines with no deferred bytes.

Gate subjects were all 1,800 paths from `jj file list`, including source,
fixtures, Cargo configuration/lock, docs/books, scripts/plugins and task notes.
Every SHA256 matched before and after the run. JSON path/digest manifest SHA256:
`dc1b408d448c0ef2a8f3ea10d07b2e0452a536473c8861681d85e3b2b5d7bea9`.
Principal output SHA256:
`3954bb25022fa5a54b04a8a203b4dc38ff45fe79533b9968e6ba1f9827b5aefc`.
Only closure records and retirement follow this measurement.

Rechecked k12's Done when against its delivered subtree:

| Obligation | Delivered evidence |
|---|---|
| Successful/failed/immediate launch, Started/Reaped ordering, helper errors, pair ownership, pin preparation, epoch ordering, random publication and cleanup | k24 and k28/k30/k32/k33; k45's source verification; green workspace regressions |
| Bounded read-only observer, typed relation, exact marker/probe precedence, malformed records, substitutions, I/O errors and lock release | k34/k35 and their retained tests; this leaf isolates the named epoch-sensitive controls |
| Both release orders, forced identity/key reuse, mutation sensitivity, waiting replacements, concurrent and foreign shared holders | k38/k40/k41/k42 records and restored tests; k45 verified their production ordering |
| Native macOS/Linux killed-holder, both locks, exec-survivor and first-arriving replacement controls | k37's 29-case per-platform table and independent descriptor-leak mutations; production unchanged here |
| Real public Viewer binding, lifecycle/current species, moves/absence/replacement, exclusion/finish, style and minimum-size summaries | k43/k44 and k27's acceptance reconciliation; green application regressions |
| Read-only aliases/missing controls, pause/handoff bounds, stale-session admission, saved state and navigation | k35/k40/k43/k44 and earlier idle/view increments; green workspace regressions |
| Current usage/architecture/context/G6 and exact walkthrough reconstruction | k27's shipped documentation reconciliation and this leaf's final book validations |
| Complete adversarial review resolved | k45 plus this leaf: F1 repaired, F2 accepted within the existing corruption boundary, F3 clarified |

No missing k12 work remains. Promote its shipped behavior and evidence/limitation
pointers to the root brief and close witnessed-activity-k12. The root's required
full-width interaction, lifecycle, witnessed activity, documentation and ownership
contract are covered by k9/k10/k16/k11/k12 and the green gate. This was the sole
remaining live leaf; retire it normally and leave whole-grove finishing to the
driver. The existing ADR boundary and design stand; no ADR change is needed.
