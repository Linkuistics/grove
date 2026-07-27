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
which fires on permission requests. `Stop` only fires at turn boundaries, so a
permission prompt blocks mid-turn and would read as `working`. Today herdr's
screen manifests catch some of those by accident; once grove reports an
unrecognised agent label, screen detection can no longer override us, so that
accidental coverage is lost. Decide whether the first increment is turn
boundaries only, or turn boundaries plus `Notification`.

**Watch for**: reporting must stay cheap. A hook runs on every turn end; a
socket timeout there is a per-turn latency tax.
