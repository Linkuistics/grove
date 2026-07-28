# What happens inside a session reaches herdr through hooks grove injects per launch, on claude alone

The loop driver is the harness's **parent process**, so its whole observable
vocabulary is *session started* / *session ended*. It cannot see a **turn**
end — and a session that stalls mid-session on a question ends no session, so
the pane reads `working` until a human notices. That is the half of the
"stalled overnight, looks fine" complaint driver-level reporting structurally
cannot reach.

grove closes it by asking the harness. On every **claude** launch the loop
driver appends `--settings` carrying an inline JSON hook block that wires four
of claude's events to `grove-llm report-turn`:

| event | matcher | reports |
|---|---|---|
| `UserPromptSubmit` | — | `working` |
| `Stop` | — | **`blocked` unless `$GROVE_SIGNAL_FILE` says the task completed on purpose** |
| `Notification` | `permission_prompt\|elicitation_dialog\|elicitation_url_dialog` | `blocked` |
| `PostToolUse` | — | `working` |

The first two are the **turn boundaries**. The second two are the **mid-turn
pair**, and they are a pair rather than one more boundary: a permission prompt
stalls an unattended loop exactly as badly as a question does, but granting the
permission fires no event of its own, so a mid-turn `blocked` needs a paired
restore or it pins the pane until the turn finally ends. `PostToolUse` is the
only event claude fires in between.

A `done` disposition in the signal file silences every row a machine can
fire — the driver is about to report `idle` and release, and anything in that
window is a report in the wrong direction. `UserPromptSubmit` is exempt, because
a prompt submitted after `complete --done` is a human actually typing.

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
the cost is one ~3ms process spawn per event, socket or no socket — which is
what makes it affordable to put a reporter on the per-tool-call path at all.

**The notification matcher is a property of claude's code, not a guess.** The
three types grove matches are exactly the sites claude raises from its
*idle-notify* path, which fires only once the human has ignored the dialog for
six seconds. That is already grove's own definition of unattended, so a human
answering promptly never causes a flap, and reaching the hook at all *means*
nobody is there. It is also why `idle_prompt` is excluded — a different site,
and one that only fires with no request in flight, i.e. after `Stop` has already
reported `blocked` — as are the informational types (`auth_success`,
`agent_completed`). Matchers on `Notification` filter on the payload's
`notification_type` and a matcher drawn from `[A-Za-z0-9_|]` is compared as an
**exact-string alternation**, so `permission_prompt` cannot also fire on
`worker_permission_prompt`. A matcher naming a type this claude has never heard
of is inert; an unknown *event* name is dropped with a warning and the rest of
the block still applies.

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
  Rejected at the head: `Stop → idle` is the conflation being fixed. The
  per-tool-call half is now *half* adopted — `PostToolUse` earns its place as the
  mid-turn restore, but `PreToolUse` and `SubagentStop` still buy nothing, since
  the only transition needing to be undone is the one `PostToolUse` already
  covers. Nothing would reopen the rest.
- **`PermissionRequest` rather than `Notification` as the block signal.** Both
  fire on a permission prompt, but `PermissionRequest` is a **decisional** hook —
  exit code 2 there *denies the permission*. A status reporter has no business on
  a security-relevant decision path, however reliably it exits zero. It is also
  worse on the merits: it fires the instant the prompt appears, where
  `Notification` waits out claude's six-second idle check and so only fires when
  a human really is absent. Reopen only if `Notification` stops carrying
  permission prompts.
- **Suppress the redundant `working` reports `PostToolUse` emits**, the way
  herdr's own pi extension keeps a `lastState`. Rejected: pi's extension is
  in-process, where remembering is a free variable; grove's hook is a fresh
  process per tool call, so remembering costs a file read and write — about what
  the socket line it saves costs. It would also *lose* something real, because a
  report on every tool call is what re-asserts grove's authority after a herdr
  restart mid-session; a cache would hold the stale value until the next session.
  Reopen if a herdr-side cost of per-tool-call reports is ever measured.
- **`PostToolBatch` rather than `PostToolUse` as the restore.** It fires once per
  model round trip rather than once per tool, and — because it waits for *every*
  call in the batch to resolve — it would also close the parallel-batch race in
  *Consequences*. Rejected for now purely on age: it is a much newer event name
  than the rest of the block, and an older claude drops an unknown event with a
  warning printed on every launch. Reopen once it can be assumed present.

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
  print a `hook error` notice into the transcript at every turn — and, since one
  row is per-tool-call, at every tool call too — and `UserPromptSubmit` stdout is
  **injected into the conversation as context**, so a stray byte there would be
  read by the model as an instruction. The verb returns no `Result` for the same
  reason the rest of the reporting does not: an absent, refusing, or stock herdr
  is a no-op by design.
- **A pane can now read `blocked` while a session is still alive.** That is the
  point, and it self-heals: the next tool call or `UserPromptSubmit` reports
  `working` over it, as does the next task's launch.
- **A parallel batch can undo a mid-turn block early.** If one call in a batch is
  waiting on a permission prompt while a sibling is still running, the sibling's
  `PostToolUse` reports `working` over the `blocked`, and nothing corrects it
  until the turn ends. It needs a batch whose other member outlives the prompt's
  six-second timer, and it is still strictly better than the pre-mid-turn
  behaviour of reading `working` throughout; `PostToolBatch` closes it properly
  when it can be assumed present.
- **A mid-turn stall that raises no notification is still uncovered.** claude
  raises one for permission prompts and MCP elicitations, but not for a tool that
  renders its own dialog — `AskUserQuestion`, for one — so a session parked there
  still reads `working` until its turn ends. Nothing in this design can reach it:
  there is no event to hook.
- **The mid-turn row only fires where the harness actually asks**, and a
  permissive permission mode is the common case that silences it. Measured
  2026-07-28 under `defaultMode: "auto"` with `skipDangerousModePermissionPrompt`
  set: an `rm -rf`, an explicit sandbox override, and a call to an
  un-allowlisted MCP server all ran with no dialog at all — so no
  `Notification`, and the pane read `working` throughout. The row is worth
  having anyway (an unattended overnight loop is exactly the case a human sets
  a *prompting* mode for), but it is not a general answer to "grove is stuck":
  a session in a fully permissive mode has no mid-turn stall to report.
- **The six-second timer is gated on human *inattention*, not elapsed dialog
  time.** Dialogs held open for several seconds with the human present and
  interacting did not fire the notification; the one that did had sat ~10s
  untouched. That is the semantics this design wants — it detects *unattended*,
  not merely *slow* — but it means the row cannot be provoked on demand by
  simply leaving a dialog up while watching it.
- **The claim that the status surface stops at session boundaries is now false
  for claude, and still true for codex and pi.** Anything asserting otherwise —
  glossary, ADR, spec — has to say which harness it means.
- The hook calls `grove-llm` by the path the driver resolved, so a driver and
  its injected hooks cannot drift; the version-skew guard already covers the
  agent-side binary.
