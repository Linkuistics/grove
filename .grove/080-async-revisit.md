# 080-async-revisit

**Kind:** planning (a real re-evaluation now — D2 picked in-process pty, so the
async pressure returns; start from the 050 evidence, don't refactor for its own sake)

## Goal

Re-evaluate concern 4 (the sync→async refactor) now that the harness backend is
**in-process pty** (D2 / ADR-0014). Decide whether the event loop needs async at
all, needs a minimal slice, or stays sync — backed by concrete pressure from
060's real implementation, not speculation.

## Context

- **D2 picked in-process pty, so the original async case is live again** — grove
  holds N ptys and pumps their output in the loop. But the 050 spike already
  produced **strong evidence that sync suffices**: reader-thread → `mpsc` →
  `try_recv` drain per pane between `event::poll` ticks, and under claude's startup
  burst the **max backlog was 4 chunks/tick** — the sync loop absorbed it
  trivially. The starting hypothesis is therefore **"keep sync"**, to be confirmed
  or refuted by 060's heavier real-world use, not reopened from scratch.
- **What could still force async** (test these against 060, not in the abstract):
  - sustained heavy output (fast-scrolling build logs, not a 5 KB startup burst)
    across **several** simultaneous harness panes;
  - input latency under load (the `frame_ms` HUD metric is the probe);
  - whether the per-pane reader-thread + channel model scales to fleet-scale pane
    counts, or whether thread-per-pty becomes the cost rather than the loop.
- The fallback if sync genuinely breaks is a **minimal** async slice (e.g. tokio
  only for pty I/O), not a wholesale rewrite — and only if 060 surfaces concrete
  pressure. v1's `notify` fs-watch is already comfortably sync.
- Inputs: 050's "Findings" running log (in `done/`) and whatever 060 surfaces
  about event-loop pressure in practice.

## Done when

- A recorded decision: keep sync / add minimal async (scoped) / full async
  refactor — each justified by the concrete pressure (if any) measured in 060. If
  "keep sync," retire with a short rationale + ADR/note citing the 050 backlog
  evidence, formally closing the deferred concern.

## Notes

Deliberately last. The 050 evidence makes "keep sync" the leading answer — resist
refactoring for its own sake (grove constraint 4). Let 060's real, multi-pane,
heavy-output use either confirm sync or produce the specific number that justifies
async.
