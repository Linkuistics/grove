# witnessed-activity-k45

**Reviews:** running-lifetimes-k44


## Goal

Adversarially review the complete shipped witnessed-activity protocol and viewer
contract, including resource lifetimes, interleavings, native evidence and docs.



## Context

Re-derive producer commits by permanent handle. The final producer is
running-lifetimes-k44; also consume running-rows-k43, launch-events-k24,
launch-witnesses-k25 (through witnessed-epoch-k33), and witnessed-observation-k26
(through witness-platforms-k37). The parent brief contains acceptance ownership
and native evidence. k37 records platform fingerprints and per-case results;
k38/k40/k41/k42 record release/order mutations, concurrency and foreign holders.

Contract: docs/specs/item-status.md and the one-live-driver-per-working-tree ADR.
Implementation: grove-loop observation/driver_lease, keyed-launch runner events,
jj-workspace discovery and grove-tui observation/Viewer/tests/witnessed.rs.
Usage, architecture, context map, G6 and source-derived books must agree.

## Done when

- Attack Started/Reaped ordering for failed/immediate spawn, signals, escalation,
  confirmed/unconfirmed reap errors, unwind/drop and surviving exec children.
  Check lease-owned paired descriptors, private-before-directory orderly release,
  close-on-exec and admission preservation on observational failure.
- Attack epoch-before-preparation/publication, selected-root pin/recheck,
  independent random names/exclusive creation, binding grammar, exact markers,
  cleanup and no guard across spawn or caller work.
- Attack directory-before-private probe precedence, all release orders and
  numeric/key-reuse mutations, first-arriving replacement observers, shared
  concurrency and old-epoch-delayed handoff. Verify positive controls survive
  mutations; distinguish modeled schedules from kernel-teardown claims.
- Assess macOS/Linux native evidence case by case, including actual reap,
  both independent probes, leftover records and surviving children. Preserve
  native-backend/cooperating-process assumptions and suspension limitations.
- Attack Viewer runtime comparison independently of rejected trees/root errors;
  same-tree key binding/exclusion after whole-tree validation; lifecycle/current
  species, absence, unreadable trees and previous-tree reuse. Check minimum-size
  labels/qualifiers/keys, style separation, hidden/File activity, saved state,
  read-only aliases/multiple viewers and responsive retry.
- Check docs and walkthrough reconstruction against current source. Findings
  name evidence and concrete impact; create adjacent actionable integration only
  if findings earn it. No implementation fixes or in-session reviewers here.

## Notes

witnessed-view-k27 closes with k44. witnessed-activity-k12 remains live for this
review and any actionable integration; implementation tests alone do not close it.
