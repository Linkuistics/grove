# herdr-pane-state-k2

**Kind:** work

## Goal

Make `grove do` report its pane's semantic state to herdr from the **loop
driver** — the parent process — so a grove is legible without any per-harness
hooks. Smallest slice that fixes the headline complaint: a grove stalled
overnight on a HITL question currently reads as `done`.

## Context

- `src/loop_driver.rs` — `run_loop` already distinguishes the three terminal
  cases: `Disposition::Relaunch`, `Disposition::Done`, and no-signal
  (crash / Ctrl-C / `/exit`) which becomes `LoopOutcome::Stopped`. Those
  branches are the report sites; `launch_session` is the fourth.
- herdr's write side needs no discovery: `HERDR_ENV=1`, `HERDR_SOCKET_PATH` and
  `HERDR_PANE_ID` sit in the pane environment and are inherited all the way down
  (verified live during planning — a Bash tool call inside `grove do` inside
  `claude` sees all three). The method is `pane.report_agent` over the unix
  socket; `pane.report_metadata` carries display-only tokens.
- Root brief `## Notes` — especially the **agent-label gate** and the
  **de-facto-authority hypothesis**, which this leaf is the first chance to
  measure.

## Done when

- Session launch reports `working`; a `complete --done` finish reports `idle`;
  a no-signal stop reports a state distinguishing "needs a human" from a clean
  exit (see the open question below).
- Reporting is a **no-op** when the three env vars are absent or the socket
  refuses — never a failed launch, never a stalled loop (*herdr-optional-ui*).
  A slow or wedged socket must not block the driver; bound the attempt.
- **Measured, not assumed**: confirm what herdr does with an unrecognised
  `agent` label. Does `agent: "grove"` land while the pane is mis-detected as
  codex, and does it stop screen detection from overriding us? Record the answer
  in the commit message, and correct the root brief's note if the hypothesis is
  wrong.
- Tests cover the `LoopOutcome`/`Disposition` → reported-state mapping without
  needing a live herdr.

## Notes

**Settled in planning** (`01-plan-k1`, retired):

- Report `agent: "grove"`, not the underlying harness. A `grove do` pane is
  genuinely a loop relaunching a *sequence* of sessions, and may vary harness
  per leaf once `03` lands — "grove" is the honest label. It also sidesteps the
  conflict gate that would otherwise drop the report outright.
- The `source` string is ours to pick; herdr accepts any (its own tests use
  `custom:hermes`). It will **not** grant full lifecycle authority — that is a
  compiled-in allowlist — so do not design as though screen detection is off.

**Open question this leaf must resolve** (deferred from planning): a no-signal
exit lumps together a crash, a deliberate Ctrl-C, and `/exit`. Is `blocked`
right for all three, or should a deliberate exit read as `idle`? The driver may
be able to tell them apart from the child's exit status or terminating signal;
if it cannot, say so plainly and pick the safer default rather than inventing a
distinction the driver can't actually observe.

**Scope guard**: intra-session turn boundaries are `04`. The value of this leaf
is precisely that it needs no hooks — resist pulling them in.
