# agent-hint-observe-k34

**Kind:** impl

## Goal

Observe, on a real herdr pane, that a grove-launched harness is detected as
*itself* rather than as `codex` — then write the durable record the whole node
exists to produce.

## Context

`agent-hint-k33` sets `HERDR_AGENT` in grove's launch environment. Nothing so far
has watched herdr act on it. Two facts shape how this must be measured:

- **A landed grove report hides detection.** `herdr pane get` on a live `grove do`
  pane shows `agent: "grove"`, because grove's hook report takes precedence.
  Detection is only visible in the gaps — **before grove's first report**, and
  **after it releases** at `complete --done`. Plan the observation around a gap,
  or read whatever field exposes the detected agent underneath rather than the
  effective one.
- **A session cannot observe its own driver.** The change is in the driver that
  spawned this session, so the pane you are in is running the *old* one. Unlike
  `observe-live-surface-k26` and `observe-mid-turn-live-k31`, this does **not**
  need a release to get around that: the observed pane can run
  `./target/debug/grove do` in a throwaway working tree. Confirm the built binary
  is the one under test before reading anything into the result.

Decomposing the measurement, cheapest first — the two halves fail for different
reasons and are worth separating if the end-to-end result surprises:

1. **herdr's half**, with no grove involved: a pane running
   `HERDR_AGENT=claude <something long-lived>` should report `agent: claude`. If
   this fails, the fault is the installed herdr, not grove.
2. **grove's half**: a `grove do` pane, read in a gap.

## Done when

- The detected agent for a grove-launched pane is the launched harness, observed
  live and recorded with the herdr build it was measured against.
- ADR *herdr-optional-ui* is **reworked in place**: the paragraph ending "What to
  do about that is undecided and out of this ADR's scope" is replaced by the
  decision and its principle (*grove reports what it is; it hints what it
  launched*), and the rejected process-group route joins **Considered options**
  with its reopen condition. Never append a superseding record
  (`linkuistics:decision-records`).
- `CONTEXT.md`'s **Pane mis-detection** entry is rewritten from a live defect
  ("Undecided — `herdr-pane-misdetection-k11`") to the resolved mechanism, keeping
  its `_Avoid_` line about the MCP-process-group misdiagnosis, which stays true and
  is still the trap.
- The root brief's Notes entry on mis-detection is reconciled with the outcome.

## Notes

**Report only what you saw.** If the hint does not take on the installed herdr,
that is the finding — the brief's route rests on a code read plus a documented
contract, not on a measurement, and this leaf is where that gap closes either way.
The fallback routes and their costs are in the brief; a negative result reopens
them rather than needing a fresh investigation.

**One question worth answering while a pane is in hand**, because it is free here
and expensive later: after grove releases at `complete --done`, does the pane snap
back to the detected harness, or stay at `agent: null` / `agent_status: unknown`
as `observe-live-surface-k26` recorded? By then the harness has exited, so the
hint may have exited with it and the group may hold only `grove`. Whatever the
answer, it belongs in the spec's observation nuances rather than being asserted
from theory.
