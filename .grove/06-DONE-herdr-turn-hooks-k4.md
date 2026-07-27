# herdr-turn-hooks-k4

**Kind:** work

## Goal

Report state at **intra-session turn boundaries**, refining the session-level
reporting from **herdr-pane-state-k2**. The driver knows when a session starts
and ends; only the
harness knows when a *turn* ends — which is the moment the agent has stopped and
a human is needed.

## Context

- **herdr-pane-state-k2** must land first — this leaf refines its mapping, it
  does not replace it.
- Injection differs sharply per harness:
  - **claude**: `--settings <file-or-json>` accepts an **inline JSON string**, so
    hooks can be injected per launch with nothing persisted and no contention
    with herdr's own installer. Do this one first.
  - **codex**: `-c key=value` config overrides, but hooks require persisted
    trust — `--dangerously-bypass-hook-trust` is the documented escape and is
    labelled DANGEROUS. Note `--profile` is already taken by model selection.
  - **pi**: extension-install model (`pi install`), no per-launch injection
    found. herdr's own pi integration is already a **full lifecycle authority**,
    so check whether pi needs anything from us at all before building.
- herdr's removed claude/codex hook wiring (`src/integration/targets.rs`
  uninstall path) shows the event names available: `Stop`, `UserPromptSubmit`,
  `SessionStart`, `SubagentStop`.

## Done when

- A claude-hosted grove session reports `blocked` when a turn ends and a human
  is needed, and does **not** flap to blocked at the end of every task.
- Absent herdr, injected hooks are inert — no output, no failure, no latency.
- codex and pi are either done or explicitly deferred with the reason recorded.

## Notes

**The discriminator, settled in planning.** In a grove session a turn ends
exactly two ways:

| turn ended | `GROVE_SIGNAL_FILE` exists? | truth |
|---|---|---|
| the model ran `grove-llm complete` | yes | driver about to relaunch → `working` |
| anything else | no | model stopped mid-task, human needed → `blocked` |

So `Stop → blocked unless the signal file is there`. This needs **no new model
contract**: `grove-llm complete` is already mandatory as the last action of every
task, so the disposition is already being deposited. Do not invent a "declare
your intent" verb — it would be a second thing to forget.

This is also why herdr could not fix it upstream and grove can: herdr sees a turn
end and genuinely cannot tell "done" from "asking". grove knows, because grove is
the thing that relaunches.

**Open question deferred from planning**: whether to also wire `Notification`,
which fires on permission requests. **Decided: turn boundaries only, this
increment.** `blocked` on a permission prompt needs a paired restore once the
permission is granted, and the only event that fires there is per-tool-call —
a different design (and a redundancy-suppression question) rather than a bigger
version of this one. Externalised as **herdr-mid-turn-blockers-k30**, which ADR
*herdr-turn-boundary-hooks* names as the reopen condition; the two measurements
that leaf needs are recorded in its Context so it need not re-derive them.

**Watch for**: reporting must stay cheap. **Measured: ~3ms per invocation**
(30 invocations in 80ms, both against a live socket and with the pane
environment scrubbed) — process startup dominates and the socket is invisible
against it. The latency worry was unfounded; what remains is chatter, which only
bites if reporting ever goes per-tool-call.

## Outcome

Shipped for **claude**; codex and pi deferred on facts, not effort. Durable
record: ADR *herdr-turn-boundary-hooks*, plus the glossary's reworked
*Session-boundary visibility / Turn-boundary hooks* entry and a correction to
*herdr-optional-ui*, whose "the status surface stops at session boundaries"
consequence was made false by this.

**Verified live, not just in tests.** grove's own generated `--settings` string
was captured off a real driver run and fed verbatim to the real `claude -p`
against a unix-socket listener, on all three signal states:

| `$GROVE_SIGNAL_FILE` | reports observed |
|---|---|
| absent (stopped mid-task) | `working` (prompt), then **`blocked`** |
| `relaunch` (task done) | `working`, `working` — no flap |
| `done` (grove finished) | `working` only; `Stop` stays silent |

herdr's own `pane.report_agent_session` appeared in all three runs alongside
grove's reports, which is the merge property holding in production rather than
in a fixture.

**What is not covered**, and stated so nothing later assumes otherwise: mid-turn
blockers on every harness (**herdr-mid-turn-blockers-k30**), and turn boundaries
at all on codex and pi. Three rows of *herdr-optional-ui*'s release table
(SIGTERM/SIGHUP, version-skew stop, relaunch) remain unobserved on a live pane —
inherited unchanged from **status-surface-live-k23**, untouched here.
