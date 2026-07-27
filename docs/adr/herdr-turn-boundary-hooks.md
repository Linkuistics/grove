# Turn boundaries reach herdr through hooks grove injects per launch, on claude alone

The loop driver is the harness's **parent process**, so its whole observable
vocabulary is *session started* / *session ended*. It cannot see a **turn**
end — and a session that stalls mid-session on a question ends no session, so
the pane reads `working` until a human notices. That is the half of the
"stalled overnight, looks fine" complaint driver-level reporting structurally
cannot reach.

grove closes it by asking the harness. On every **claude** launch the loop
driver appends `--settings` carrying an inline JSON hook block that wires
claude's two turn-boundary events to `grove-llm report-turn`:
`UserPromptSubmit` → `working`, and `Stop` → **`blocked` unless
`$GROVE_SIGNAL_FILE` says the task completed on purpose**.

## Why it binds

**The discriminator needs no new model contract.** A grove turn ends exactly
two ways: the model ran `grove-llm complete` — in which case the disposition is
already sitting in the signal file and the driver is winding the session
down — or it did not, in which case the model stopped mid-task and a human is
needed. `complete` is *already* mandatory as the last action of every task, so
the fact the hook reads is already being deposited by a step nobody can skip
without the loop stalling anyway. A "declare your intent before stopping" verb
would have been a second thing to forget, and forgetting it would look exactly
like the stall it was meant to report.

This is also why herdr cannot fix this upstream and grove can. herdr sees a turn
end and genuinely cannot tell "finished" from "asking" — which is why its own
retired claude mapping bound `Stop → idle`, and why `idle` is what herdr derives
`done` from. grove knows, because grove is the thing that relaunches.

**Injection is per launch and persists nothing.** claude's `--settings` takes a
JSON *string* as an additional settings source, and hooks are **unioned** across
sources — measured, not assumed: a project `Stop` hook and a `--settings` `Stop`
hook both fire, and in a live pane herdr's own installed `SessionStart` hook
claims session identity alongside grove's reports without either disturbing the
other. So grove contends with nothing, writes to no file the user owns, and
leaves nothing behind when the loop stops.

**Nothing is injected outside a herdr pane.** The gate is the pane environment
grove already reads, applied at the *launch* site rather than only inside the
verb, so with no herdr present the argv is byte-identical to a grove that never
had turn hooks: no hook to fire, nothing to spawn, no new surface. Under herdr
the cost is one ~3ms process spawn per boundary, socket or no socket.

## Considered options

- **Persist the hooks in the user's `settings.json`** (as herdr's own installer
  does). Rejected: it is a mutation of the user's configuration that outlives
  the grove, in the same file herdr's installer writes, needing an uninstall
  path and an answer for what happens when both write. Per-launch injection has
  none of those questions. Reopen only if claude ever drops inline `--settings`.
- **A new "I am about to stop and ask" verb for the model to call.** Rejected:
  it adds a second mandatory last action, and the failure mode of forgetting it
  is indistinguishable from the bug. The signal file already carries the fact.
  Nothing would reopen this — the redundancy is the objection.
- **Adopt herdr's retired claude event→state mapping wholesale**
  (`PreToolUse`/`PostToolUse`/`SubagentStop` → `working`, `Stop` → `idle`, …).
  Rejected on both ends: `Stop → idle` is the conflation being fixed, and the
  per-tool-call `working` reports buy nothing once `UserPromptSubmit` covers the
  only transition that needs undoing. Reopen if mid-turn blocking states are
  ever reported (see below), which is what would need a per-tool restore.
- **Also report mid-turn permission prompts** (`Notification`, or the
  `PermissionRequest` event). Deferred, not rejected: it is real coverage — an
  unattended loop that hits a permission prompt stalls exactly as badly as one
  that hits a question, and grove's own authority now suppresses the screen
  detection that used to catch it by accident. What stops it landing here is
  that `blocked` needs a paired restore-to-`working` once the permission is
  granted, and the only event that fires there is per-tool-call; that is a
  different design (and a redundancy-suppression question) rather than a bigger
  version of this one. Reopen with `herdr-mid-turn-blockers-k30`.

## Why claude alone

Not an ordering of effort — the other two harnesses are blocked on facts, and
both were checked rather than assumed.

**codex has no turn-end hook event at all.** Its set is `pre_tool_use`,
`permission_request`, `post_tool_use`, `pre_compact`, `post_compact`,
`session_start`, `session_end`, `user_prompt_submit`, `subagent_start`,
`subagent_stop` (codex-cli 0.145.0). `session_end` is the boundary the driver
already sees. Independently, codex hook **trust is persisted** per
source-location and content hash in `~/.codex/config.toml`'s `[hooks.state]`, so
a `-c`-injected hook carries no trust record; the only escape,
`--dangerously-bypass-hook-trust`, disables trust for *every* hook in the
invocation, which is a security downgrade grove must not impose on a user's
config to light up a status pixel. Either blocker alone is sufficient. Reopen if
codex gains a turn-end event *and* per-invocation hook trust.

**pi already has a full-lifecycle reporter, and it has the same bug.** herdr's
own pi extension is in the compiled-in authority allowlist and reports
`working`/`blocked`/`idle` from pi's events — but it reports **`idle` on
`agent_settled`**, i.e. at every turn end, which is precisely the conflation
this ADR exists to break. So pi is not covered, and pi is also not
straightforwardly fixable here: `pi -e <path>` is a genuine per-launch injection
route, but a grove extension would race herdr's own on the same event with no
defined ordering. Reopen as its own leaf if pi-hosted groves become common.

## Consequences

- **`grove-llm report-turn` is machinery, not a verb for the model.** It is on
  the `grove-llm` surface because it reads the signal file `complete` writes —
  same protocol, same side of the fence — but nothing should ever call it but
  the injected hook. Its help says so.
- **It must exit zero and print nothing, always.** A non-zero exit makes claude
  print a `hook error` notice into the transcript at every turn, and
  `UserPromptSubmit` stdout is **injected into the conversation as context** —
  a stray byte there would be read by the model as an instruction. The verb
  returns no `Result` for the same reason the rest of the reporting does not:
  an absent, refusing, or stock herdr is a no-op by design.
- **A pane can now read `blocked` while a session is still alive.** That is the
  point, and it self-heals: the next `UserPromptSubmit` reports `working` over
  it, as does the next task's launch.
- **The claim that the status surface stops at session boundaries is now false
  for claude, and still true for codex and pi.** Anything asserting otherwise —
  glossary, ADR, spec — has to say which harness it means.
- The hook calls `grove-llm` by the path the driver resolved, so a driver and
  its injected hooks cannot drift; the version-skew guard already covers the
  agent-side binary.
