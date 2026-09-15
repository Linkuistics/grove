# item-status-k5

**Integrates:** item-status-k3

## Goal

Triage the `item-status-k3` design review's findings against
`docs/specs/item-status.md` and the observation extension in
`docs/adr/one-live-driver-per-working-tree.md`, and apply the real ones so the
design the `item-status-k4` planning step decomposes is the corrected one.

## Context

Read the findings from the review's own commit, not from this body: they are
in the `## Findings` section of `item-status-k3`'s leaf, each anchored to
`path:line` in the producer's commit `d13209e1d39a`, with the source evidence
and the classification the reviewer proposed. Grade each one yourself as a
contract stated unclearly, a real issue, a visible trade-off, or noise; the
review's "What held" list says which attack axes were checked and passed, so
they need not be re-derived. The root brief carries the human-agreed
requirements and test seams, and the review cited them by line.

The spec and ADR are prose; nothing in this leaf runs the viewer or driver. The
spine's `ADR-FORMAT.md` governs any rework of the ADR set (edit in place, keep
the set minimal, reconcile every citation, including `CONTEXT.md`,
`CONTEXT-MAP.md`, `docs/ARCHITECTURE.md` and the two specs that point at the
design). A finding that demands the observation model be rethought rather than
repaired becomes a new producer review chain beside this leaf, not a fix here.

## Done when

Every finding in the review has a recorded disposition in this leaf's running
log, the accepted ones are applied to the spec and ADR set with citations
reconciled, the spec and ADR still agree with each other and with the root
brief, and the waiting `item-status-k4` planning step can read a design that
needs no further correction before slicing.

## Notes

The integration may spend one narrow in-session reviewer if a fix is
substantive and non-mechanical; it need not.

## Decisions (running log)

- F1 — real issue, repaired with the review's short-guard option; accepted
  residual liveness trade-off. Tree/file capture finishes and drops its guard
  before the bounded epoch read; no advisory guard escapes the observer. The
  ADR explicitly accepts suspension inside that smaller window reaching the
  existing 30-second bound and an ordinary overlap diagnostic. Removing the
  epoch guard entirely would require a publication redesign, outside this leaf.
  Source confirms the timed acquisition and loop-fatal handoff path.
- F2 — contract stated unclearly. The shared selector deliberately refuses
  duplicate keys for driver and viewer, with or without exclusion. Current
  `selected` only checks duplicate live finishes while viewer row collection
  checks keys. The spec now names the behavior change, diagnostic, validation
  scope and usage-documentation obligation for implementation.
- F3 — contract stated unclearly. Transfer the launch root pin and witness
  together into a lease-owned launch-observation value before publication;
  selection remains copyable value data. Every orderly release closes witness,
  then pin, then (on lease drop) driver ownership. This removes dependence on
  the per-iteration selection outliving `drive` on an unconfirmed-reap error.
- F4 — real structural issue. Move mandate/publication details and the observer
  procedure to the spec, leaving rationale, ownership decision and trade-offs
  in the ADR with a direct citation. Keep the existing ADR slug: observation
  evidence extends this ownership protocol and a second record would require
  readers to chase its predecessor-handoff context. No new ADR is warranted.
- F5 — contract stated unclearly. Runtime worktree validation compares directory
  device/inode, allowing path aliases while exact-workspace discovery excludes
  ancestors. Admission's canonical-path validation remains unchanged.
- F6 — contract stated unclearly. Witness exclusive acquisition is nonblocking;
  contention is observation-only allocation failure and cannot block launch.
- F7 — real trace gap. Help must name the active view and Tab's full-width
  destination; added an application acceptance scenario.
- F8 — contract stated unclearly. Selection uses its gutter alone, NEXT uses
  bold normal-foreground activity text, and RUNNING keeps bold yellow item and
  activity text. These concrete treatments preserve terminal lifecycle colors
  and support semantic style assertions without relying on color.

- Narrow review — one reviewer spent on F1/F3. Its epoch-order read found no
  new deadlock beyond the accepted suspension window. Its actionable lifetime
  finding is a real issue: orderly Drop does not establish kernel descriptor
  cleanup order on death, so root-pin release may precede witness release.
  The claimed death-safe no-rebinding property is therefore unsupported.
  This is a redesign boundary, not a second inline repair: `item-status-k6`
  is inserted before planning to establish the mechanism and commission its
  review. The spec and ADR flag the unresolved guarantee rather than claiming
  readiness. No second reviewer was spent. This follows the integration
  family's substantial-redesign exception to this leaf's original Done when.
