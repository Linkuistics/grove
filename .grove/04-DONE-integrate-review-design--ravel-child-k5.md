# ravel-child-k5

**Integrates:** ravel-child-k3

## Goal

Triage the findings of `ravel-child-k3` against the design of
`ravel-child-k2`. Apply the supported findings and record a reason for each
finding that does not warrant a change.

## Context

- The review `ravel-child-k3`, including its producer commit and source
  coordinates. Read its findings from that review, rather than treating them
  as an implementation mandate.
- The root brief and the design log in `ravel-child-k2`.
- The caller specs, the reworked ADRs and their views that the review names.
- The following planning leaf, `ravel-child-plan-k4`, consumes the integrated
  design and root brief.

## Done when

- Every finding has a supported disposition: applied, an accepted trade-off,
  or rejected with evidence.
- Accepted changes are present in the design records and any affected planning
  instructions. All relevant citations remain coherent.
- Changed artifacts have the verification appropriate to those changes, with
  the evidence and limits recorded.

## Notes

The owner settled the requirements in `plan-k1`. Keep the integration within
those requirements. This leaf owns fixes and their verification. The review
changed no production or test code.

## Decisions (running log)

- **F1 is a real issue, and the design records take the fix (integrate,
  2026-10-22).** The source shows both paths. The lifecycle propagates a
  launch error at `crates/grove-loop/src/loop_driver.rs:229` before it reads
  the latch at `:246`. The CLI propagates it at `crates/grove/src/cli.rs:100`,
  so `reraise` at `:105` never runs. A failed wait keeps a forwarded signal only
  in a local of `crates/keyed-launch/src/run.rs:1254`. The requirements say that a
  TERM or HUP during a session, between sessions or after the reap makes
  `grove` die of that signal. So accepting the loss would contradict them.
- **The error of the shared function carries the interruption (integrate,
  2026-10-22).** A failed spawn or wait takes the latch into its error, as the
  reap takes it into the end value. The function keeps its notices and sends no
  KILL.
- **Every error of the loop keeps a caught signal (integrate, 2026-10-22).**
  The same loss reaches each `?` of the loop after the handlers exist, for
  example a failed pick after a TERM between sessions. One rule closes the whole
  class: Grove reports the error, then dies of the signal from the launch error
  or the latch. A failed invalidation follows the same rule. `grove run` keeps
  its exit 1 for each failure, because no requirement asks it to die of a
  signal.
- **The handler boundary goes into the spec (integrate, 2026-10-22).** The
  function installs the handlers at its first call. Before it, each signal keeps
  its inherited disposition. After it, a caught signal belongs to the driver,
  even when the next spawn fails.
- **F2 is a real issue, and the ADRs lose the procedures (integrate,
  2026-10-22).** Each ADR keeps its decision, reasons, costs and rejected
  alternatives, and cites the spec section for each procedure. The two ADRs
  rejected the same alternative, a kill of what survives in the group of
  `ravel`. The supervision ADR now owns that rejection and the rejection of a
  bounded wait, because both follow from its decision that Ravel alone cleans
  up. The spec cites the supervision ADR for the reason of the 1.1.0 floor and
  for the trust in PATH, where it restated them.
- **The sweep found three omissions in the work orders (integrate,
  2026-10-22).** A sweep of the tree for the reversed decision found records
  that the brief did not list. `docs/USAGE.md` and the driver reference of the
  methodology state the survivor group. The release target tests pin the Ravel
  1.0.0 archives, which the 1.1.0 floor refuses. Ravel v1.1.0 publishes the
  same three archives. The root brief now lists all three, and the planning
  leaf ties the release pins to the floor. The other matches of the sweep use
  "survivor" in an unrelated sense, or belong to code that the removal of the
  crate reaches.
- **The verification fits a change to records only (integrate,
  2026-10-22).** The checker `ste-lint.py` reports no hard finding in any
  edited record. Each relative link and anchor in the spec, the two ADRs and
  the README of the views resolves. `diagrams.json` parses. Each edge of
  `loop-decision.mmd` names a defined node. No Mermaid renderer ran, because
  none is installed and the viewer needs network access. A diff matched each
  claim that left an ADR to the spec or to the other ADR. No test reads the
  edited records, so `scripts/check.sh` did not run. No code changed.
- **No in-session reviewer (integrate, 2026-10-22).** The F1 contract is a
  small addition at one boundary, and the post-reap unit seam covers its
  lifecycle half. The planning and implementation leaves read it again.
