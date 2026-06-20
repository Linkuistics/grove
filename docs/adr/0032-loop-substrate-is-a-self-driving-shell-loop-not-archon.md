# 32. The loop substrate is a self-driving shell loop, not an Archon workflow

- Status: **accepted** (decided in `refactor-to-archon` 030-substrate-decision;
  the mechanism is built and proven by leaf 040-substrate-wiring, whose PoC gates
  the final kill-realisation choice)
- Date: 2026-06-20
- Deciders: Antony Blakey (with grove `refactor-to-archon` 020-loop-substrate-spike
  + 030-substrate-decision)
- Pairs with: **ADR-0031** (the shed-machinery end-state this substrate completes)
- Evidence: `docs/research/loop-substrate-options.md` (the 020 spike)

## Context

The refactor's founding premise was literally "refactor grove to be an **Archon**
workflow" (archon.diy — a YAML workflow engine of `loop`/`fresh_context`/
`interactive` nodes). The 020 spike evaluated Archon against four alternatives
(iTerm2 triggers, a PTY-wrap supervisor, a headless `claude -p` loop, and
harness-native loop features) on three pass/fail gates: **(C)**
fresh-context-per-task, **(D)** interactive multi-turn grilling *inside* the loop
(the make-or-break — half of grove's tasks are planning/grilling), and **(B)**
restart-safety. Full evidence with primary-source citations is in
`docs/research/loop-substrate-options.md`.

## Decision

**Reject Archon. The loop substrate is a thin, stateless, self-driving shell
loop that grove owns.** Mechanism:

- A trivial **loop driver**: `while grove-llm pick has work: launch claude; on
  exit, decide relaunch-or-stop`. `grove-llm pick` is the loop condition (empty →
  the grove is done).
- Each task runs as a **normal foreground `claude` child** owning the real TTY —
  so interactive grilling, resize, and Ctrl-C are 100% native, with **no PTY
  wrapper and no passthrough code**.
- The agent, as its **last step** (after commit + retire), runs a `grove-llm`
  **completion signal** verb. That triggers an **out-of-band kill** of the claude
  session — the external "exit" the agent cannot cleanly perform on itself.
  (Leaned realisation: a *self-spawned delayed killer* the verb forks; the file-
  watch-daemon variant is the alternative. The 040 leaf's PoC settles which.)
- **Relaunch is opt-in:** the loop relaunches **only** when the completion signal
  fired. *Any* other exit — human `/exit`/Ctrl-C, or a crash — **stops** the loop,
  resumable later by re-running `grove do <name>`. This keeps interrupts stopped.
- **Restart ≡ continuation by construction:** the loop body holds zero engine
  state and re-derives position from `grove-llm pick` every iteration (the spike's
  D6). Re-invoking the loop *is* resuming it; a crashed mid-task leaf is simply
  re-picked and redone (commit-before-retire guarantees no half-done state).

## Rationale (from the 020 spike)

- **Archon fails the make-or-break gate D.** Its `interactive:` is a single
  **approve/reject** gate (optionally a bounded reject→rework→re-pause cycle), not
  an open-ended multi-turn interview; a grilling whose number/content of turns is
  unknown in advance can only be *faked* as a pre-authored node chain. Disqualifying
  for planning tasks.
- **The named hypothesis is refuted.** Archon's restart is a separate **DB-keyed
  `resume <run-id>`**, not the same stateless `grove-llm pick` as loop-continuation;
  to get grove's restart≡continuation model you would bypass Archon's loop node,
  discarding its value-add.
- **Walk-away regression.** Archon holds live run state (completed nodes, paused
  state, loop counters, conversation) in a SQLite/Postgres DB — a half-finished run
  is not legible from git alone.
- **"Which complexity to own" comes out against it.** Archon is a ~10-week-old,
  pre-1.0 TypeScript rewrite; adopting it adds an external dependency *and* a DB in
  the exact dimension grove is shedding (ADR-0031), against grove's documented
  history of shedding process machinery (ADR-0028) and the "less in grove" directive.
- **The self-driving loop delivers what Archon falsely promised — for free.**
  Restart≡continuation and loop-until-empty both fall out of one `grove-llm pick`
  evaluation, with ~zero machinery and full git legibility.

## Consequences

- grove's runtime becomes: a shell loop + a `grove-llm` signal verb + the global
  skill. **No external engine, no DB, no PTY wrapper, no portable-pty dependency.**
- Restart-safety is structural (constraint 1), not a feature to configure.
- Leaf **040-substrate-wiring** builds and proves it: the loop driver, the signal
  verb, the kill realisation (a/b), the **interrupt/stop semantics** (relaunch
  opt-in), and the PoC (foreground claude + the kill + clean relaunch, grilling
  intact). The substrate's correctness rides on that leaf; if the PoC surfaces a
  blocker, it escalates here.
- The grove's *name* (`refactor-to-archon`) now misdescribes the outcome — kept as
  a historical label; the spike reversing its own premise is the spike working.
