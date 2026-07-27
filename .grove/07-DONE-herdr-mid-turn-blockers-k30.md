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

## Outcome

Shipped as **two more rows in the injected block**, a pair: `Notification`
(matched to `permission_prompt|elicitation_dialog|elicitation_url_dialog`) ⇒
`blocked`, `PostToolUse` ⇒ `working`. The durable record is ADR
*herdr-turn-boundary-hooks*, reworked in place — retitled, given the four-row
table, and with its deferral entry replaced by the three real trade-offs. The
glossary term is now *Turn hooks*, not *Turn-boundary hooks*.

**Both open questions were settled by reading the shipped claude binary**
(`strings` over 2.1.220 exposes the bundled JS), which is a measurement of the
artifact rather than of the self-contradicting docs:

- **Matchers do filter `Notification`**, keyed on `notification_type`
  (`case "Notification": a = n.notification_type`), and a matcher drawn from
  `[A-Za-z0-9_|]` takes an **exact-string alternation** path — so
  `permission_prompt` cannot also fire on `worker_permission_prompt`. claude's
  own changelog dates this to 2.0.37.
- **The three matched types are exactly claude's *idle-notify* sites** — one
  helper, mounted by the permission dialog and the two elicitation dialogs,
  which resets a `lastInteractionTime` on mount and fires **once, after six
  seconds of no human interaction**. So the selection rule is a property of
  claude's code, not a guess, and reaching the hook already means *unattended*.
- **An unknown hook *event* name is dropped with a warning and the rest of the
  block still applies** — which is why `PostToolBatch` was tempting but is not
  worth an every-launch warning on an older claude.
- `PermissionDenied` fires **only** for the auto-mode classifier, not on an
  interactive denial, so it is not a general restore.

**Redundancy suppression was considered and rejected**, contra this leaf's
opening framing. herdr's pi extension dedups because it is *in-process*, where
remembering is a free variable; grove's hook is a fresh process per tool call, so
a `lastState` costs a file read and write — about what the socket line it saves
costs — and it would lose the free self-healing a per-tool-call report gives
after a herdr restart. Recorded as a *Considered options* entry.

**Measured live, real claude, real socket** (`claude -p` against a `UnixListener`,
fed grove's own generated `--settings` captured from a real driver launch): the
payload parses with no validation warning; a tool-using prompt reports
`working`(prompt) → `working`(**PostToolUse**) → `blocked`(Stop, no signal), and
the differential — the same prompt with no tool call — reports only
`working` → `blocked`, so the extra report is unambiguously the new hook. The
k4 boundary evidence is therefore re-run and not regressed.

**Not observed live: the `Notification` half end to end.** That notification is
raised by a TUI dialog component, so `claude -p` cannot reach it, and four
attempts at driving interactive claude under `expect` never got the model as far
as a permission prompt. Externalised as **observe-mid-turn-live-k31** rather than
claimed — the same call `status-surface-live-k23` made about an unobserved
surface.
