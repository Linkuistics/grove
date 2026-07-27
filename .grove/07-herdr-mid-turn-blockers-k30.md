# herdr-mid-turn-blockers-k30

**Kind:** impl

## Goal

Report `blocked` when a claude session stalls **inside** a turn — on a
permission prompt, or on any other dialog waiting for a human — and take the
report back down once the human answers. **herdr-turn-hooks-k4** closed the
turn *boundary*; this is the gap left inside one.

## Context

Deferred out of **herdr-turn-hooks-k4** with the reason recorded in ADR
*herdr-turn-boundary-hooks* (*Considered options*, the `Notification` entry),
which names this leaf as its reopen condition.

Why it is real work rather than a fourth line in the injected settings: the
report needs a **paired restore**. `Notification` (or `PermissionRequest`)
gives `blocked` cheaply, but once the permission is granted the agent works on,
and the only claude event that fires there is **per tool call**
(`PostToolUse`). So the increment is not "one more hook" — it is a decision
about putting a reporter on the per-tool-call path, and if it goes there, about
suppressing the redundant `working` reports it would emit.

It also matters because grove's own authority **took away** the accidental
coverage that used to exist: herdr's screen manifests caught some permission
prompts by pattern, and a landed grove report now takes precedence over screen
detection (*herdr-optional-ui*). So this is closing a gap grove opened.

Two things measured during **herdr-turn-hooks-k4** that this leaf starts from
rather than re-deriving:

- A `grove-llm report-turn` invocation costs **~3ms**, socket or no socket
  (30 invocations in 80ms, measured both under a live fake herdr and with the
  pane environment scrubbed). Per-tool-call reporting is therefore *cheap*; the
  objection to it is chattiness on herdr's socket, not latency.
- herdr's own pi extension already solves that chattiness the obvious way — it
  keeps a `lastState` and skips a report that would not change anything
  (`src/integration/assets/pi/herdr-agent-state.ts`, `publishState`). grove has
  no equivalent because nothing has needed one yet.

## Done when

- An unattended `grove do` pane sitting on a permission prompt reads `blocked`,
  not `working`.
- Granting the permission returns the pane to `working` without waiting for the
  turn to end.
- The claim in ADR *herdr-turn-boundary-hooks* about what is and is not covered
  is reconciled with whatever ships.

## Notes

**Two candidate events, and they are not equivalent.** `PermissionRequest` is a
**decisional** hook — exit code 2 there *denies the permission*. Putting a
status reporter on a security-relevant decision path is a smell, even though
grove's verb always exits zero. `Notification` is **observational** and its
documented matcher values (`permission_prompt`, `idle_prompt`,
`agent_needs_input`, …) name exactly the human-needed cases. Prefer
`Notification` unless something forces otherwise.

**Settle the matcher question by measurement, not by reading.** The Claude Code
hooks page contradicts itself: its matcher table says `Notification` filters on
notification type, while a later paragraph says Stop / Notification /
UserPromptSubmit ignore matchers entirely. A live interactive check settles it.
Note the design is *robust either way* — if matchers are ignored the hook also
fires on `agent_completed`/`auth_success`, whose worst case is a transient
`blocked` that the very next `Stop` corrects.

**`idle_prompt` is probably already covered.** It fires after ~60s idle waiting
for input — but if the agent stopped and is waiting, `Stop` has already fired
and already reported `blocked`. Check before wiring it; it may add nothing.

**Do not regress the boundary case.** `herdr-turn-hooks-k4`'s live evidence is
three cases (no signal → `blocked`; `relaunch` → `working`; `done` → silent),
reproducible by feeding grove's own generated `--settings` string to
`claude -p` against a unix-socket listener. Re-run it after any change to the
injected block.
