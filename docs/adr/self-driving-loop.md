# The loop substrate is a thin, self-driving shell loop grove owns

grove runs its one-task-per-session loop with a stateless shell loop it owns, not an
external workflow engine. The mechanism:

- A trivial **loop driver**: `loop { launch a harness session; on exit, relaunch
  only if the completion signal fired }`. The **completion signal is the driver's
  loop condition** — the driver never runs `grove-llm pick` itself. `pick` is the
  *agent's* loop condition, evaluated in-session: empty output means the grove is
  done, and that session proposes the finish cycle and signals `complete --done`.
- Each task runs as a **normal foreground harness child** owning the real TTY, so
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
- **Version-skew guard.** Once per session launch, the driver checks that
  `grove-llm --version` — resolved as the *agent* resolves it, through PATH —
  still reports the driver's own compiled-in version. A long-running driver keeps
  executing the text segment it started with even after `brew upgrade` replaces
  (or deletes) the binary on disk, while the agent's `grove-llm` is PATH-resolved
  afresh at every invocation — so an upgrade mid-loop silently splits the signal
  protocol's two halves (observed as a pre-watcher driver paired with a
  watcher-era `grove-llm`: every session hung at its completion signal, nothing
  ever relaunched, no diagnostic). On a confirmed disagreement the driver **stops
  before the next session**, naming both versions and the restart instruction —
  stopping is free (restart ≡ continuation) where continuing risks exactly that
  silent hang. An *unreadable* version (missing binary, failed or unparseable
  `--version`) only warns and continues: the guard guides, it does not gate
  (constraint 5). Per session, not per driver start, because a mid-loop upgrade
  is precisely the case a start-time check misses.
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
- codex launches additionally carry a VCS store write grant (`--add-dir`, the
  gitdir or the jj repo store) so the sandboxed session can commit and retire
  at all — see *codex-gitdir-grant*.
- A `brew upgrade` mid-loop stops the loop at the next session boundary with a
  restart instruction, instead of hanging at the next completion signal;
  re-running `grove do` resumes on the new binary.
