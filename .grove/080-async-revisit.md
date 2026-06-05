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

## Decision — concern 4 is dissolved; grove stays sync (no ADR)

**Outcome: keep sync. Concern 4 closed as architecturally dissolved, not merely
"absorbed."** Recorded as this note (committed → git history), not an ADR: a
non-change confirming the status quo doesn't clear the ADR bar (not surprising,
not hard to reverse, no live trade-off). Decided with the user (grilling, this
session).

**Why the concern no longer applies.** Concern 4 was scoped against an
architecture where *grove's own event loop* juggled the three async drivers in
the root brief: a multiplexer control socket, N-repo `notify` streams, and
subprocess output juggling. Three pivots later (ADR-0014 → 0015 → 0020) grove is
a hard fork of zellij (`crates/trellis`), and none of those drivers is grove's
loop anymore:

- **Subprocess/pty output juggling → trellis's**, not grove's. zellij's threaded
  server owns terminal emulation for every harness pane. The 050 workload (grove
  pumping a `claude` pty in its sync loop) no longer exists as grove code.
- **Multiplexer control socket → dissolved.** ADR-0020 made grove's logic
  in-process; control is direct `HostDriver` calls. The proxy seam (ADR-0016),
  WASM nav (0018), and reply-only back-channel (0019) all evaporated.
- **N-repo `notify` streams → already off the render path.** A dedicated
  `std::thread` per surface coalesces fs-watch bursts under the 200 ms `DEBOUNCE`
  and wakes the screen thread via `driver.request_tick()`
  (`src/tui.rs` `spawn_grove_watch`, ~4913). The surface mutates only in `tick()`.
  This is the fleet-scale design (070 Q6: one watcher over every fleet repo's two
  grove-state roots) and it is non-blocking w.r.t. grove's render path by
  construction.

grove has **no event loop of its own to refactor**: its `handle_key` / `render` /
`tick` are callbacks trellis invokes. There is no I/O left for a "minimal async
slice" to own; a full async refactor is plainly unjustified (constraint 4).

**Evidence (test against real use, not the abstract — per this brief):**

- **050 spike:** sync pump max backlog **4 chunks/tick** under claude's startup
  burst (ADR-0014). Sync absorbed it trivially.
- **060/010:** kept the embed sync on that evidence, explicitly deferring the
  call here.
- **060 + 070 real use** (heavy harness panes; multi-repo fleet fs-watch): every
  running log reviewed — **no `frame_ms` / input-latency / output-backlog / jank
  finding surfaced.** The HUD `frame_ms` probe the brief named never flagged.

**The fallback, if ever needed,** remains a minimal async slice (e.g. tokio for a
*future* out-of-process / network surface — the ADR-0020 GraphQL/network boundary,
explicitly deferred), not a rewrite of grove's render path. Nothing in v2 reaches
for it. v1's `notify` fs-watch stays comfortably sync (now thread + tick).

**Closes** the last deferred concern from the root brief; this is the grove's
final leaf.
