# The loop substrate is a thin, self-driving shell loop grove owns

grove runs its one-task-per-session loop with a stateless shell loop it owns, not an
external workflow engine. The mechanism:

- A trivial **loop driver**: `while grove-llm pick has work: launch claude; on exit,
  relaunch only if the completion signal fired`. `grove-llm pick` is the loop
  condition — empty output means the grove is done.
- Each task runs as a **normal foreground `claude` child** owning the real TTY, so
  interactive grilling, resize, and Ctrl-C are 100% native — **no PTY wrapper, no
  passthrough code**.
- The agent's **last step** (after commit + retire) runs the `grove-llm complete`
  signal verb, which only writes the disposition into a signal file. The **loop
  driver** — the harness's own parent process, and so always able to signal its
  child — watches for that file while the harness runs and performs the
  out-of-band exit itself: grace → SIGTERM → kill-grace → SIGKILL (driver-side
  watcher).
- **Relaunch is opt-in.** The loop relaunches **only** when the completion signal
  fired. Any other exit — human `/exit` or Ctrl-C, or a crash — **stops** the loop,
  resumable later by re-running `grove do`.
- **Restart ≡ continuation by construction.** The loop body holds zero engine state
  and re-derives its position from `grove-llm pick` every iteration. Re-invoking the
  loop *is* resuming it; a crashed mid-task leaf is simply re-picked and redone,
  because commit-before-retire guarantees no half-done state.

## Considered options

The founding premise was to adopt an external YAML workflow engine (Archon) with
`loop` / `fresh_context` / `interactive` nodes. It was rejected, and the rejection is
why grove owns a shell loop rather than an engine plus a database:

- **It fails the make-or-break gate.** Its interactive node is a single
  approve/reject gate, not an open-ended multi-turn interview — and half of grove's
  tasks are planning sessions whose number and content of turns are unknown in
  advance. A grilling can only be *faked* as a pre-authored node chain.
- **Walk-away regression.** It holds live run state (completed nodes, paused state,
  loop counters, conversation) in a SQLite/Postgres DB, so a half-finished run is not
  legible from git alone.
- **Wrong complexity to own.** Adopting it adds an external dependency *and* a DB in
  the exact dimension grove is shedding (see *self-extension-core-and-methodology*).

A second, later option was also tried and rejected — not against the engine, but
within the shell-loop design itself:

- **In-agent self-spawned killer (rejected).** The first cut had `grove-llm
  complete` fork its own detached killer, from inside the agent, to end its own
  harness session — SIGTERM after a grace, SIGKILL fallback. It worked under
  claude's default sandbox, but codex's Seatbelt sandbox denies a same-sandbox
  process from signalling its own session (`(allow signal (target
  same-sandbox))`): the kill silently failed (its `2>/dev/null` hid the
  `EPERM`), so a codex-driven loop never relaunched on its own. The loop driver
  is outside any harness sandbox and is the harness's own parent, so it can
  always signal its child — moving the kill there is strictly more capable and
  adds no new moving part, since the driver already owns the launch and the
  relaunch decision.

The self-driving loop delivers what the engine falsely promised — restart≡
continuation and loop-until-empty — for free, both falling out of one `grove-llm pick`
evaluation with near-zero machinery and full git legibility. Full evidence with
primary-source citations is in `docs/research/loop-substrate-options.md`.

## Consequences

- grove's runtime is a shell loop + a `grove-llm` signal verb + the global skill. No
  external engine, no DB, no PTY wrapper, no portable-pty dependency.
- Restart-safety is structural (the loop holds no state), not a feature to configure.
- The driver ignores SIGINT so it survives the human's Ctrl-C; it watches for the
  completion signal file while its harness child runs and kills that child
  itself once the file appears — relaunch is gated on the same file. No PID is
  ever exported to the agent.
- The driver also selects each session's **launch model by the picked leaf's
  kind** (planning vs work), via native `claude --model` — see
  *model-per-task-kind*.
