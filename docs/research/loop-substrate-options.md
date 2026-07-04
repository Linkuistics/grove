# Loop-substrate options for grove-on-a-workflow

> **Research spike — `020-loop-substrate-spike`. Decides nothing.** It gathers
> the primary-source evidence that lets `030-substrate-decision` choose the
> substrate that drives grove's loop: how to run **one grove task per fresh
> context** until the task tree is empty, hosting both autonomous **work** tasks
> and interactive multi-turn **grilling** tasks, surviving restart.
>
> **Citation discipline (`driving.md`).** Every capability claim cites a primary
> source by URL and quotes the load-bearing text; where a search came up empty,
> "no primary source found" is recorded as an explicit finding. Archon
> training-data recall is known-stale (it was rewritten ground-up in April 2026),
> so **every Archon claim below is from the live `dev` branch and docs, fetched
> 2026-06-20.** This is read-not-run research: nothing was installed.

## How to read this doc

**The three pass/fail gates** (from the task brief). Each candidate is scored on
all three:

- **(C) fresh-context-per-task** — can it automate "run one task, clear/exit the
  context, start the next task fresh"?
- **(D) interactive grilling INSIDE the loop** — can a *multi-turn* human
  interview happen mid-iteration, then the loop resume? **This is the
  make-or-break.** A candidate that fails D is disqualified for *planning* tasks
  — which are half of what grove does.
- **(B) restart-safety** — after a crash/quit mid-loop, does re-invoking resume
  correctly from tree state (a `grove-llm pick`), not a marker file?

**The candidate roster** (task brief §"Candidates to evaluate"):

1. **Archon** — YAML workflow engine (`loop`/`fresh_context`/`interactive` nodes,
   isolated git worktree per run). The inherited premise (D1–D8) and the grove's
   own name.
2. **iTerm2 triggers** — regex-on-output triggers that fire keystrokes/scripts.
3. **Thin grove PTY-wrap** — a small grove supervisor spawning the agent in a pty.
4. **Headless `claude -p` / Agent SDK shell-loop** — the Ralph-style hot loop.
5. **Harness-native loop features** — what Claude Code itself now offers.

**Bottom line up front.** Gate D cleaves the field, and it cuts against the
inherited premise. The autonomous substrates (1, 4, 5) nail C but cannot host a
real grilling; the interactivity-preserving substrates (2, 3) pass D but pay for
C in fragility or re-grown plumbing. **Archon's two load-bearing promises both
fail on inspection** — `interactive:` is an approve/reject gate, not a
conversation, and the named restart hypothesis is refuted. The recommendation
(§Recommendation) is a thin, stateless, grove-owned loop driver, with a small
PoC flagged for `030`.

---

## Candidate 1 — Archon

### What it is today

Archon was **completely rewritten ~2026-04-07** from a Python task-management/RAG
tool into a TypeScript/Bun "AI workflow engine for coding agents"; the old code
is parked on `archive/v1-task-management-rag`
([issue #957](https://github.com/coleam00/Archon/issues/957)). Tagline: *"The
first open-source harness builder for AI coding... Define your development
processes as YAML workflows"*
([dev README](https://raw.githubusercontent.com/coleam00/Archon/dev/README.md)).

**Maturity signals (the "which complexity to own" inputs):** MIT licensed;
default branch `dev`; latest release **v0.4.1 (2026-05-28)** — pre-1.0; **~312
open issues**, ~22.5k stars; rapid cadence (v0.1.0 Dec 2025 → v0.4.1 May 2026)
*with breaking changes in flight* (e.g. v0.3.7 stopped embedding the Claude Code
SDK in binaries)
([CHANGELOG](https://raw.githubusercontent.com/coleam00/Archon/dev/CHANGELOG.md)).
Agent backends: Claude Code (Agent SDK), Codex, OpenCode, Pi, GitHub Copilot
([ai-assistants.md](https://raw.githubusercontent.com/coleam00/Archon/dev/packages/docs-web/src/content/docs/getting-started/ai-assistants.md)).
**Net: the workflow-engine Archon is ~10 weeks old at evaluation time.**

### Gate verdicts

**Gate C — fresh-context-per-task: PASS.** `fresh_context: true` is documented
exactly: *"Each iteration starts a fresh AI session. No memory of prior
iterations"* — for "When work state lives on disk (files, git)." It clears the
session **only**, not the worktree (the agent "still accesses the same file
system state, which is why the pattern requires reading state from disk")
([loop-nodes.md](https://raw.githubusercontent.com/coleam00/Archon/dev/packages/docs-web/src/content/docs/guides/loop-nodes.md)).
The maintainers' bundled `archon-ralph-dag` is *exactly* grove's pattern — fresh
context per iteration, "implement ONE story... exit," looping `until: COMPLETE`,
`fresh_context: true`
([archon-ralph-dag.yaml](https://github.com/coleam00/Archon/blob/dev/.archon/workflows/defaults/archon-ralph-dag.yaml)).
Crucially for grove's termination condition, the loop supports a **deterministic
bash check**: *"an `until_bash` script... If it exits with code 0, the loop
completes — even if the AI didn't output the completion signal."* **This is the
hook for `grove-llm pick` to drive termination** (modulo polarity: exit-0 means
*stop*, so a one-line wrapper inverts `pick`).

**Gate D — interactive grilling INSIDE the loop: PARTIAL → effectively FAIL for
true multi-turn.** This is decisive and the answer is nuanced. Archon *can* pause
a loop for human input mid-iteration, but each interaction is **a single
approve/reject round-trip, not a free-form interview.** The approval node:
*without `on_reject` a rejection cancels the run; with `on_reject` the executor
"runs the `on_reject.prompt` via AI... then re-pauses at the same gate. This
repeats until the user approves or `on_reject.max_attempts` is reached"* (default
3)
([approval-nodes.md](https://raw.githubusercontent.com/coleam00/Archon/dev/packages/docs-web/src/content/docs/guides/approval-nodes.md)).
A human "turn" is therefore *approve* or *reject-with-reason* — the agent cannot
ask the human a follow-up and receive a conversational answer within a node. The
loop's own between-iteration gate is the same shape: *"Set `interactive: true` to
pause the loop between iterations... The user's feedback is injected into the next
iteration's prompt via `$LOOP_USER_INPUT`"* — **one feedback string per iteration
boundary**, and that variable "is only populated on the first iteration of a
resumed interactive loop; empty string on all other iterations"
([loop-nodes.md](https://raw.githubusercontent.com/coleam00/Archon/dev/packages/docs-web/src/content/docs/guides/loop-nodes.md),
[variables.md](https://raw.githubusercontent.com/coleam00/Archon/dev/packages/docs-web/src/content/docs/reference/variables.md)).
The maintainers' own `archon-interactive-prd` "interview" is built by **chaining
pre-authored `prompt:` + `approval:` node pairs** (ask a batch → capture one
batched answer → next phase), *not* by a conversational node
([archon-interactive-prd.yaml](https://github.com/coleam00/Archon/blob/dev/.archon/workflows/defaults/archon-interactive-prd.yaml)).
**Grilling's defining property is that the number and content of turns is not
known in advance** — an emergent design-tree walk. Archon can only express a
*fixed, pre-authored* sequence of gates. For grove's open-ended planning
interview, **this gate fails.**

**Gate B — restart-safety: PASS, but DB-backed, not stateless.** Re-invoke with
`archon workflow resume <run-id>` — *"Re-executes the workflow, automatically
skipping nodes that completed in the prior run"*; paused runs survive a server
restart because *"The run persists in the database"*
([cli.md](https://raw.githubusercontent.com/coleam00/Archon/dev/packages/docs-web/src/content/docs/reference/cli.md),
[approval-nodes.md](https://raw.githubusercontent.com/coleam00/Archon/dev/packages/docs-web/src/content/docs/guides/approval-nodes.md)).
But resume re-hydrates *a specific run-id from the DB* and skips completed nodes —
it is **not** grove's "re-derive position from `pick`" model (see the hypothesis).

### The named hypothesis — REFUTED (in its strong form)

> *Can `depends_on` + `loop`/`until` + conditionals express **both** restart and
> loop-until-no-live-leaves with **one** declarative `grove-llm pick`, since for a
> stateless self-locating body restart and continuation are the same `pick`?*

**The loop-continuation half is confirmed; the restart half is not how Archon
works.**

- *Continuation via one `pick`: confirmed.* `until_bash` + a `fresh_context`
  Ralph body is exactly a stateless self-locating loop — each iteration reads disk
  state, does one unit, exits; the loop runs until `until_bash` (your `pick`)
  signals empty
  ([loop-nodes.md](https://raw.githubusercontent.com/coleam00/Archon/dev/packages/docs-web/src/content/docs/guides/loop-nodes.md)).
- *But restart is a separate mechanism.* Archon **tracks loop iteration state
  internally**: `max_iterations` is an engine-enforced counter ("If the loop
  reaches this count without a completion signal, the node **fails**"), and
  `resume <run-id>` re-hydrates the *specific prior run* from the DB rather than
  starting a fresh process that re-locates purely from `pick`. So in Archon's
  model **a restart is a DB-state-driven resume of a tracked run, not "just
  another iteration that starts a fresh process."**

**Net:** Archon gives termination "for free" via `until_bash`/`pick`, but
restart-safety is a *second*, DB-keyed mechanism — not the same `pick`. The "one
declarative mechanism for both" claim is **refuted**. To get grove's
restart≡continuation model (D6) you would run the loop *outside* Archon's loop
node — an external driver re-invoking `archon workflow run` each tick — which
**abandons exactly the loop/resume machinery that is Archon's value-add.** At that
point Archon is reduced to a worktree-manager + scheduler, a role grove can fill
far more cheaply.

### A–G mechanics

- **(A) run/worktree lifecycle.** Ephemeral by default — each run gets a worktree
  at `~/.archon/workspaces/<owner>/<repo>/worktrees/<branch>/` with an
  auto-generated branch. **A persistent branch is supported:** `--branch <name>`
  *"Creates/reuses worktree... Reuses existing worktree if healthy"*, and
  `--no-worktree` runs in the current directory. A committed `.grove/` tree on a
  persistent branch survives because worktrees share git objects (*"It's not a
  clone — it shares Git history, objects, and remotes"*). Cleanup is explicit
  (`archon isolation cleanup`, `archon complete <branch>`)
  ([isolation.md](https://raw.githubusercontent.com/coleam00/Archon/dev/packages/docs-web/src/content/docs/book/isolation.md),
  [cli.md](https://raw.githubusercontent.com/coleam00/Archon/dev/packages/docs-web/src/content/docs/reference/cli.md)).
- **(B) restart/resume.** `archon workflow resume <run-id>`; state in the DB table
  `remote_agent_workflow_runs` ("workflow state, step progress")
  ([database.md](https://raw.githubusercontent.com/coleam00/Archon/dev/packages/docs-web/src/content/docs/reference/database.md)).
- **(C) loop/until/fresh_context/interactive-in-loop.** Loop fields: `prompt`,
  `until` (text signal, required), `max_iterations` (required hard cap → node
  *fails* if exceeded), `fresh_context`, `until_bash` (deterministic exit-0),
  `interactive` + `gate_message`. A nested `approval:` node **cannot** sit inside a
  `loop:` body (loops "contain only AI iterations"); the loop's *own*
  `interactive` gate is the only in-loop human hook
  ([loop-nodes.md](https://raw.githubusercontent.com/coleam00/Archon/dev/packages/docs-web/src/content/docs/guides/loop-nodes.md)).
- **(D) interactive pause/resume.** Human responds via natural-language message,
  Web UI button, Slack button, REST, or CLI `archon workflow approve/reject` — all
  auto-resume the paused run; paused runs survive restart
  ([approval-nodes.md](https://raw.githubusercontent.com/coleam00/Archon/dev/packages/docs-web/src/content/docs/guides/approval-nodes.md)).
  The in-loop feedback channel is the single `$LOOP_USER_INPUT` string.
- **(E) node data-flow + arbitrary CLIs.** A `bash:` node's stdout is captured as
  `$nodeId.output` and can drive a later node's `when:` condition (demonstrated:
  `when: "$bash-json-node.output.status == 'ok'"`); `when:` is a *declarative*
  comparison (`== != < <= > >= && ||`), not a shell command
  ([variables.md](https://raw.githubusercontent.com/coleam00/Archon/dev/packages/docs-web/src/content/docs/reference/variables.md),
  [e2e-opencode-all-nodes-smoke.yaml](https://github.com/coleam00/Archon/blob/dev/.archon/workflows/e2e-opencode-all-nodes-smoke.yaml)).
  A `prompt:`/`command:` node runs an SDK agent with the **Bash tool**, so
  **arbitrary CLIs like `grove-llm` are callable** (the maintainers' own loops run
  `archon workflow event emit ...` via Bash) — subject to `allowed_tools`/
  `denied_tools` (not enforced on loop iterations)
  ([archon-ralph-dag.yaml](https://github.com/coleam00/Archon/blob/dev/.archon/workflows/defaults/archon-ralph-dag.yaml)).
- **(F) schema / location / invocation / distribution.** Six node types
  (`command`/`prompt`/`bash`/`loop`/`approval`/`cancel`), plus a `script:` (inline
  TS/Python). Common fields: `id`, `depends_on`, `when`, `trigger_rule`,
  `context`, `idle_timeout`, `retry`, `always_run`, `output_type`; AI nodes add
  `provider`, `model`, `output_format`, `allowed_tools`, `denied_tools`, `hooks`,
  `mcp`, `skills`, `agents`, `effort`, `thinking`, `maxBudgetUsd`, … Workflows live
  in `.archon/workflows/` (and global `~/.archon/workflows/`, bundled `defaults/`);
  invoked via `archon workflow run <name> [msg] --cwd <p> [--branch <b>]
  [--no-worktree] [--resume]`; distributed as committed YAML, the global dir, or a
  **workflow marketplace** (CHANGELOG v0.3.11)
  ([concepts.md](https://raw.githubusercontent.com/coleam00/Archon/dev/packages/docs-web/src/content/docs/getting-started/concepts.md),
  [authoring-workflows.md](https://raw.githubusercontent.com/coleam00/Archon/dev/packages/docs-web/src/content/docs/guides/authoring-workflows.md)).
- **(G) walk-away legibility.** See below.

### Walk-away check — MIXED

Legible on git: the workflow YAML, command files, `.archon/config.yaml`, and a
committed task tree (the Ralph `prd.json`/`progress.txt`, grove's `.grove/`
equivalent). **Hidden in the DB** (SQLite `~/.archon/archon.db` or Postgres, 18
`remote_agent_*` tables): *which nodes completed, the loop's paused/approval
state, the resume pointer, conversation history, and the `max_iterations`
counter* — `workflow_runs` = "workflow state, step progress," `workflow_events` =
"step transitions, artifacts, errors," `messages` = "user and assistant
messages"
([database.md](https://raw.githubusercontent.com/coleam00/Archon/dev/packages/docs-web/src/content/docs/reference/database.md)).
**A half-finished *run* is therefore not walk-away-legible** — uninstall Archon
and the in-flight run state and the ability to `resume` are lost. If grove keeps
*all* durable state in committed `.grove/` files and treats every run as
disposable (the Ralph discipline Archon itself endorses), then only Archon's own
run bookkeeping is lost — but that is precisely the complexity Archon asks you to
host in its DB.

### What I could NOT verify (Archon)

- A node type supporting open-ended, agent-asks/human-answers, N-turn dialogue in
  one context — **none found**; the evidence is that Archon deliberately models
  human interaction as discrete approve/reject gates.
- Whether `archon workflow resume` can resume a run whose worktree was deleted.
- Whether a captured `$LOOP_USER_INPUT` survives a full Archon-process restart
  (only *approval-node* paused runs are documented to survive).
- `foreach`/per-node `env` fields (referenced in the brief) — not found in docs.
- A verbatim sentence that a `prompt:` node's cwd is the worktree root (strongly
  implied and demonstrated by example, not stated outright).

---

## Candidate 2 — iTerm2 triggers

### What it is

iTerm2 "triggers" are per-profile rules that match a regex against terminal
**output** and fire an action. The grove use: a task prints a sentinel line on
completion; a trigger matches it and either *sends text* (`/clear` + next prompt)
or *invokes a Python RPC* that calls `grove-llm pick` and types the result. The
user already runs Claude Code in iTerm2, so grilling happens natively in that
session; only the between-task transition is automated.

### Gate verdicts

**Gate D — interactive multi-turn grilling: PASS (and the candidate's decisive
strength).** This is the *only* candidate where the human and the agent are never
separated. The trigger is an out-of-band observer of the output stream; there is
**no process interposed between keyboard and agent**, so multi-turn grilling is
fully native. The write-back actions inject into the same live session: *"Send
Text: Sends user-defined text back to the terminal as though the user had
typed it"* and *"Inject Data: Inserts bytes into the input stream"*
([triggers](https://iterm2.com/documentation-triggers.html)).

**Gate C — fresh-context-per-task: PARTIAL.** A trigger can *type* `/clear` + the
next prompt, but two documented limits make it fragile: matching is line-scoped
and windowed — *"Only one line at a time is matched"* and *"If a line is very
long, then only the last three wrapped lines are used"* — and instant triggers
have mis-fire edge cases (*"may cause certain regular expressions... to match less
than they otherwise might"*; cf. over-fire reports,
[issue 7832](https://gitlab.com/gnachman/iterm2/-/issues/7832))
([triggers](https://iterm2.com/documentation-triggers.html)). A bare Send-Text
trigger is a dumb regex→action with no access to command output, so *conditional*
logic ("if `pick` is empty, stop") requires the *"Invoke Script Function"* action
wired to a registered Python RPC — which needs a **long-running daemon**
(`run_forever` "is what makes it a long-running daemon")
([registration](https://iterm2.com/python-api/registration.html),
[daemons](https://iterm2.com/python-api/tutorial/daemons.html)). **The "thin"
story collapses into an always-on Python sidecar + iTerm2-specific RPC plumbing.**

**Gate B — restart-safety: PARTIAL/weak.** The trigger holds no task state and
re-fires on the next sentinel; if it invokes `grove-llm pick`, the next task is
re-derived correctly. But loop *liveness* depends on the AutoLaunch daemon being
up **and** a sentinel actually appearing — after a crash mid-task there may be no
sentinel, so **the loop can silently stall with nothing to resume it.**

### Walk-away check — POOR

Trigger config is **not in the repo**: it lives in
`~/Library/Preferences/com.googlecode.iterm2.plist`, or (more legibly) a Dynamic
Profile JSON under `~/Library/Application Support/iTerm2/DynamicProfiles`
([dynamic-profiles](https://iterm2.com/documentation-dynamic-profiles.html)).
Remove the substrate and the committed git state gives *no hint* the loop ever
ran. A Dynamic Profile could be checked into the repo and symlinked, but it still
must be installed into iTerm2's folder, and it only works **in iTerm2 on macOS.**

### Which complexity to own

Brittle terminal/app config tied to **one emulator on one OS**, with loop
reliability resting on string-matching a windowed scrollback. The compensating
virtue is real: **zero interposition → gate D for free.** You trade owning a
supervisor for owning fragile, hidden, emulator-locked config.

### What I could NOT verify (iTerm2)

- A verbatim statement that a registered RPC is silently un-invokable when its
  daemon is down (inferred from the daemon/registration docs).
- A doc guarantee that a single full-line sentinel *always* matches (vs. the
  "last three wrapped lines" truncation).
- Whether `/clear`-via-Send-Text resets Claude Code context identically to a fresh
  process (a Claude Code behavior question). The **coprocess** path is a dead end:
  *"A session can not have more than one coprocess at a time"* and a coprocess's
  stdout "will be treated the same as keyboard input" (collides with the human)
  ([coprocesses](https://iterm2.com/documentation-coprocesses.html)).

---

## Candidate 3 — Thin grove PTY-wrap supervisor

### What it is

A small grove-authored process opens a pty, spawns the agent as the pty child,
copies bytes both ways, watches the child's output for a completion sentinel, and
on match restarts the child with fresh context and the next task from `grove-llm
pick`. The building blocks are real and documented — Rust's
[`portable-pty`](https://docs.rs/portable-pty/latest/portable_pty/) (extracted
from wezterm) provides `openpty`, `spawn_command`, reader/writer, and resize. It
is the Ralph idea (*"In its purest form, Ralph is a Bash loop"*,
[ghuntley.com/ralph](https://ghuntley.com/ralph/)) but pty-wrapping an
*interactive* agent rather than re-spawning a headless one.

### Gate verdicts

**Gate D — interactive multi-turn grilling: PARTIAL (the central risk).** A pty
supervisor *can* host interactive multi-turn input via transparent pass-through —
the canonical primitive is pexpect's `interact()`: *"Keystrokes are sent to the
child process, and the stdout and stderr output of the child process is
printed"*
([pexpect](https://pexpect.readthedocs.io/en/stable/api/pexpect.html)). So gate D
is **not fundamentally broken.** But "passively watch for a sentinel without
corrupting interactivity" *is* the whole engineering risk, and prior art shows it
is fiddly: the supervisor must handle the escape char so it doesn't steal keys
(Ctrl-C is a known sharp edge,
[pexpect#415](https://github.com/pexpect/pexpect/issues/415)); window resize does
**not** propagate for free (*"if you change the window size of the parent the
SIGWINCH signal will not be passed through to the child"*); and injecting a submit
key into a TUI is itself fragile (Enter often must be sent as a separate `0x0d`).
**Interactivity is recoverable, but only by correctly owning raw-mode passthrough,
signal forwarding, and resize — each a way to subtly break grilling UX.**

**Gate C — fresh-context-per-task: PASS (strongest gate).** This is what Ralph is
for — *"ask Ralph to do one thing per loop. Only one thing"*, a clean-context
agent that "reads the current state... does exactly one unit of work... and
exits"
([ghuntley.com/ralph](https://ghuntley.com/ralph/)). The supervisor owns process
lifecycle directly (kill + `spawn_command` a fresh agent), and controls the
sentinel scan end-to-end (no third-party regex-window limit).

**Gate B — restart-safety: PASS (by design, if stateless).** The supervisor can
hold no in-memory task pointer and re-derive every iteration from `grove-llm pick`
— mirroring Ralph's "progress is tracked in files, not in memory." Re-invoking
after a crash just calls `pick` again. The one discipline: never accumulate a
pointer.

### Walk-away check — GOOD (if stateless)

A stateless supervisor that re-derives from `pick` leaves the committed git state
fully legible; the only thing outside the repo is the supervisor's **own source —
grove's legible code, versioned in grove's repo**, not opaque app config.
Materially better than iTerm2's plist hiding.

### Which complexity to own

A small but **real** supervisor that **re-grows exactly the terminal/process
machinery grove just SHED.** grove migrated *off* a forked terminal multiplexer
(trellis/zellij-fork → rmux, ADR-0028) to reduce
this surface; a pty-wrap re-introduces a slice of it — raw-mode passthrough,
SIGWINCH/resize, signal forwarding, escape-char handling, corruption-free stream
scanning, child restart. Less than a full multiplexer, and legible grove code —
but a direct re-acquisition of the burden grove just paid to drop.

### What I could NOT verify (PTY-wrap)

- A `portable-pty` (Rust) example doing transparent interactive pass-through
  *plus* in-stream sentinel detection simultaneously (the pass-through evidence is
  pexpect/Python + tmux-send-keys prior art).
- Whether the agent, killed and re-spawned in a fresh pty, cleanly re-attaches in
  all resize/echo/raw-mode cases (needs empirical testing, not docs).
- False-match rates of scanning a *raw* interactive stream (TUI redraws
  interleaved) for the sentinel.

---

## Candidate 4 — Headless `claude -p` / Agent SDK shell-loop

### What it is

A shell `while` loop that invokes `claude -p "<prompt>"` (or SDK `query()` without
resume) once per task — a new process/session per iteration = fresh context for
free. The loop shells out to `grove-llm pick`, runs the task, repeats.

### Gate verdicts

**Gate C — fresh-context-per-task: PASS.** *"Add the `-p` (or `--print`) flag to
any `claude` command to run it non-interactively"*
([headless](https://code.claude.com/docs/en/headless)); and *"By default,
`query()` creates a new session for each interaction"*
([agent-sdk/sessions](https://code.claude.com/docs/en/agent-sdk/sessions)). Every
task starts clean.

**Gate D — interactive multi-turn grilling: FAIL — DISQUALIFYING.** Headless mode
is non-interactive by design: it runs autonomously to completion and exits. stdin
is *pre-piped data*, not interactive prompts during the run
([headless](https://code.claude.com/docs/en/headless)). Open feature requests
confirm there is no mid-run human interaction
([#15553](https://github.com/anthropics/claude-code/issues/15553),
[#30555](https://github.com/anthropics/claude-code/issues/30555)). **A planning
grilling — "is that assumption right? reconsider?" — cannot happen. This
disqualifies headless as the substrate for planning tasks** (though it remains the
obvious *work-task* engine inside a hybrid).

**Gate B — restart-safety: PASS.** The loop is shell + `grove-llm pick` + git
state; no session DB in the load-bearing path. Re-invoking re-derives from the
tree.

### Walk-away check — EXCELLENT

Just a shell script + git. The most walk-away-able candidate by a wide margin —
nothing hides anywhere.

### Which complexity to own

Almost none — a trivial shell loop. The cost is *capability*, not complexity: you
lose interactivity entirely, so it can only ever drive autonomous work tasks.

### What I could NOT verify (headless)

- Whether `--continue` on a fresh `claude -p` could resume a prior session within
  one invocation (doubtful; docs don't explicitly rule it out).
- Exhaustive autonomy of permission modes
  (`--dangerously-skip-permissions`/`--permission-mode`) across all tool ops.

---

## Candidate 5 — Harness-native loop features

### What it is

What Claude Code itself now ships: `/loop`, `/schedule` (routines), subagents,
background tasks. The question: do any let an *automated* loop pause for
*multi-turn human interaction* and then resume — and/or automate fresh-context
continuation inside one interactive session?

### Gate verdicts

**Gate C — fresh-context-per-task: PARTIAL.** Several features give autonomous
fresh context, but none give fresh-context *continuation within one interactive
session*:

- **`/loop`** runs a prompt repeatedly *"while the session stays open"*; tasks are
  *"session-scoped: they live in the current conversation"* — **context is not
  cleared between iterations; the session accumulates**
  ([scheduled-tasks](https://code.claude.com/docs/en/scheduled-tasks)).
- **`/schedule` routines** are *"a saved Claude Code configuration... run
  automatically"* on cloud infra — fully autonomous, non-interactive
  ([routines](https://code.claude.com/docs/en/routines)).
- **Subagents** (`Agent` tool) each *"run in their own context window"* — fresh
  context, **but autonomous only**; they "run to completion and return a summary"
  ([sub-agents](https://code.claude.com/docs/en/sub-agents)).

**Gate D — interactive multi-turn grilling: FAIL — DISQUALIFYING.** No native
feature lets an automated loop pause for a multi-turn human interview then resume.
`/loop`, `/schedule`, and subagents are all autonomous; `/clear` is a *manual*
command, not automatable from inside a prompt. The most promising native
angle — a skill + `/loop` + `/clear` composing into "clear, run next, loop" —
**does not work as documented**, because `/clear` cannot be triggered
programmatically and `/loop` has no "clear before each iteration" mode.

**Gate B — restart-safety: VARIES.** `/loop` and `/schedule` config lives in
Claude Code's own storage (`~/.claude/`/cloud) and is lossy on walk-away;
subagents in `.claude/agents/` are versioned in git.

### Walk-away check — PARTIAL

Subagent definitions are git-versioned and legible; `/loop` and routine config
live in Claude Code's own state, invisible to the repo.

### Which complexity to own

Nothing new to install — but the capability gap is fatal as a *sole* substrate:
great for autonomous fresh-context work, **cannot host grilling.**

### What I could NOT verify (native)

- Whether `/loop` can take a *skill* invocation as its prompt
  (`/loop /grove-pick-and-run`) and whether that skill can spawn fresh-context
  children. **This is the single most important open native question** — see the
  PoC flag.
- Whether an agent inside an interactive session can *self-terminate the session*
  on task completion (so a bare shell loop could advance without a human ending
  it). **Not settled by reading — flagged for a PoC below.**
- Whether any feature can invoke `/clear`/compaction programmatically from a
  prompt.

---

## Distribution: global skill + backwards-compat (separable, pursued regardless)

This is independent of the substrate choice (D8a) and the evidence is clean.

- **Global install, read live.** A skill at `~/.claude/skills/<name>/SKILL.md` is
  available to *"All your projects"* and is **read live from disk** (*edits "take
  effect within the current session without restarting"*) — **one source of truth,
  no per-project copy, no materialisation drift**
  ([skills](https://code.claude.com/docs/en/skills.md)). Precedence: enterprise >
  personal > project; *"a `code-review` skill in your project's `.claude/skills/`
  replaces the bundled `/code-review`"*; plugin skills are namespaced
  `plugin:skill` so they cannot collide.
- **Plugin + marketplace distribution.** A plugin bundles skills/agents/hooks/MCP
  and is installed at **user scope** (*"install for yourself across all
  projects"*) via a marketplace (`marketplace.json`, `/plugin marketplace add` +
  `/plugin install`), with versioning by explicit `plugin.json` `version` *or* git
  commit SHA, and **auto-update at startup**
  ([plugins-reference](https://code.claude.com/docs/en/plugins-reference.md),
  [discover-plugins](https://code.claude.com/docs/en/discover-plugins.md),
  [plugin-marketplaces](https://code.claude.com/docs/en/plugin-marketplaces.md)).
- **The CLI binary must ship separately.** A plugin's `bin/` is added only to *"the
  Bash tool's `PATH`"* — **not the user's shell PATH** — so a system-wide
  `grove-llm` still needs Homebrew (or similar)
  ([plugins-reference](https://code.claude.com/docs/en/plugins-reference.md)).
- **Backwards-compat is unobstructed.** Nothing in the skill model prevents one
  skill from reading *both* the old `NNN-slug/` directory format and the new
  flat dotted-decimal format — a skill is *"a `SKILL.md` file with
  instructions"* plus scripts; the discrimination logic lives in the skill's prose
  and in `grove-llm`, not in any Claude Code machinery
  ([skills](https://code.claude.com/docs/en/skills.md)).
- **What replaces `grove install` / `VERSION.md`.** Global skill (live-read, one
  source of truth) + Homebrew `grove-llm`. The per-worktree materialisation and the
  cli/repo/worktree three-way drift model **disappear**. The only residual is
  *skill-version vs. binary-version* if released asynchronously — managed by a
  prose/runtime compatibility note in the skill, **not** a Claude Code limitation.

**Could not verify:** a built-in plugin-manifest field to hard-require a minimum
`grove-llm` version (no such constraint field found; enforce via prose/runtime
check).

---

## Recommendation

This doc decides nothing — `030` decides. But the evidence points firmly, and in
a direction that **reverses the grove's founding premise.** Stated plainly, with
no sunk-cost weight given to the grove being *named* after Archon:

**1. Do not adopt Archon as the loop substrate.** It fails on its two
load-bearing promises and on the trade that matters:
- The make-or-break gate **D effectively fails** — `interactive:` is an
  approve/reject gate, not a conversation; an open-ended grilling can only be
  faked as a pre-authored chain of nodes, which is not grilling.
- The **named restart hypothesis is refuted** — restart is a separate DB-keyed
  `resume`, not the same stateless `pick`; to get grove's model you bypass
  Archon's loop node, discarding its value-add.
- **"Which complexity to own" comes out against it:** an external dependency on a
  ~10-week-old, pre-1.0, actively-breaking TypeScript rewrite, *plus* a SQLite/
  Postgres DB holding load-bearing run state (a walk-away regression) — set
  against grove's documented history of *shedding* process machinery
  (ADR-0028) and its "less in grove" directive.
  Adopting Archon adds complexity in the exact dimension grove is trying to remove.

**2. Do not adopt a pure-autonomous substrate (headless `claude -p`, native
`/loop`/`/schedule`) as the *sole* substrate** — both fail gate D for planning
tasks. **But keep headless `claude -p` as the likely *work-task engine*** inside a
hybrid: it is the most walk-away-able fresh-context mechanism that exists.

**3. The substrate should be a thin, stateless, grove-owned loop driver** that
re-derives position from `grove-llm pick` each iteration (which delivers grove's
D6 restart≡continuation *for free* — the very thing Archon was hypothesized to
provide and does not) and launches each task as a **fresh session, interactive for
grilling tasks.** The remaining fork for `030` is *the crank-turner* — how the
loop advances between tasks without a human re-running `grove do`:

   - **(a) Bare shell loop around interactive `claude`** — minimal complexity,
     maximal walk-away, satisfies all three gates *if* an agent can signal its
     session to end on task completion. **This hinges on one unverified fact
     (can an interactive session self-terminate / can `/loop` drive a
     fresh-context skill) — see the PoC flag. Recommend `030` resolve this
     first;** if it holds, it is the answer, and it honors "less in grove" better
     than anything else.
   - **(b) iTerm2 trigger** — gate D for free, but POOR walk-away (hidden plist
     config) and macOS/iTerm2-locked. Disfavored on walk-away grounds.
   - **(c) PTY-wrap supervisor** — legible and passes C/B cleanly, but only
     PARTIAL on D and **re-grows the terminal plumbing grove just shed**. The
     fallback if (a) is impossible and a robust crank-turner is required.

In short: **the spike's evidence says grove should *not* become an Archon
workflow.** It should become a minimal self-driven loop over `grove-llm pick`,
distributed as a global skill + Homebrew `grove-llm`, with the crank-turner chosen
by a small PoC in `030`. Take this finding to `030` honestly — it is what the
spike was for.

## Tradeoff matrix — which complexity to own

| Candidate | C fresh-context | D grilling (make/break) | B restart | Walk-away | Complexity you own |
|---|---|---|---|---|---|
| **1. Archon** | PASS | **FAIL** (approve/reject, not interview) | PASS (DB `resume`) | MIXED (run state in DB) | External 10-wk-old rewrite + DB + node model that fights grove's restart |
| **2. iTerm2 triggers** | PARTIAL (fragile regex) | **PASS** (zero interposition) | PARTIAL (can stall) | POOR (hidden plist, macOS/iTerm-locked) | Brittle terminal/app config on one emulator/OS |
| **3. PTY-wrap** | PASS | PARTIAL (recoverable, fiddly) | PASS | GOOD (grove's own code) | A real supervisor — re-grows shed terminal plumbing |
| **4. Headless `claude -p`** | PASS | **FAIL** (non-interactive) | PASS | EXCELLENT (shell + git) | ~none, but loses interactivity → work-tasks only |
| **5. Native loop features** | PARTIAL (accumulates) | **FAIL** (all autonomous) | VARIES | PARTIAL | ~none, but cannot grill → disqualified as sole substrate |
| **➤ Thin self-driven loop (rec.)** | PASS (fresh process/session) | PASS (fresh *interactive* session) | PASS (stateless `pick`) | EXCELLENT→GOOD (driver = grove code) | Minimal stateless driver; cost ≈ the crank-turner choice (≈0 if PoC-(a) holds) |

## Open items / PoC flag

Per the task brief, a small local proof-of-concept is in-scope *only* where a doc
claim genuinely can't be settled by reading. **One such item gates the
recommendation and should be the first thing `030` does:**

- **PoC — self-driven fresh-context continuation.** Can a `while grove-llm pick;
  do … done` shell loop launch a *fresh interactive* `claude` session per task,
  let the human grill natively, and have the session **end on task completion** so
  the loop advances — *without* a human re-running `grove do`? Resolve via the two
  unverified native facts: (i) can `/loop` drive a fresh-context skill invocation;
  (ii) can an agent self-terminate its interactive session on completion (or be
  told to via a sentinel the loop watches for, à la candidate 2/3 but with no
  emulator lock-in)? If **yes**, recommendation path (a) stands and the substrate
  is essentially free. If **no**, fall back to PTY-wrap (path c) for a robust,
  legible, grove-owned crank-turner; iTerm2 (path b) only if macOS/iTerm lock-in
  and hidden config are acceptable.

## Sources

**Archon** —
[repo](https://github.com/coleam00/Archon) ·
[issue #957 (rewrite)](https://github.com/coleam00/Archon/issues/957) ·
[loop-nodes.md](https://github.com/coleam00/Archon/blob/dev/packages/docs-web/src/content/docs/guides/loop-nodes.md) ·
[approval-nodes.md](https://github.com/coleam00/Archon/blob/dev/packages/docs-web/src/content/docs/guides/approval-nodes.md) ·
[database.md](https://github.com/coleam00/Archon/blob/dev/packages/docs-web/src/content/docs/reference/database.md) ·
[cli.md](https://github.com/coleam00/Archon/blob/dev/packages/docs-web/src/content/docs/reference/cli.md) ·
[variables.md](https://github.com/coleam00/Archon/blob/dev/packages/docs-web/src/content/docs/reference/variables.md) ·
[isolation.md](https://github.com/coleam00/Archon/blob/dev/packages/docs-web/src/content/docs/book/isolation.md) ·
[concepts.md](https://github.com/coleam00/Archon/blob/dev/packages/docs-web/src/content/docs/getting-started/concepts.md) ·
[authoring-workflows.md](https://github.com/coleam00/Archon/blob/dev/packages/docs-web/src/content/docs/guides/authoring-workflows.md) ·
[archon-ralph-dag.yaml](https://github.com/coleam00/Archon/blob/dev/.archon/workflows/defaults/archon-ralph-dag.yaml) ·
[archon-interactive-prd.yaml](https://github.com/coleam00/Archon/blob/dev/.archon/workflows/defaults/archon-interactive-prd.yaml) ·
[archon.diy](https://archon.diy)

**iTerm2** —
[triggers](https://iterm2.com/documentation-triggers.html) ·
[coprocesses](https://iterm2.com/documentation-coprocesses.html) ·
[dynamic-profiles](https://iterm2.com/documentation-dynamic-profiles.html) ·
[python-api registration](https://iterm2.com/python-api/registration.html) ·
[python-api daemons](https://iterm2.com/python-api/tutorial/daemons.html) ·
[issue 7832](https://gitlab.com/gnachman/iterm2/-/issues/7832)

**PTY-wrap** —
[portable-pty](https://docs.rs/portable-pty/latest/portable_pty/) ·
[ghuntley.com/ralph](https://ghuntley.com/ralph/) ·
[pexpect](https://pexpect.readthedocs.io/en/stable/api/pexpect.html) ·
[pexpect#415](https://github.com/pexpect/pexpect/issues/415)

**Claude Code (headless / native / distribution)** —
[headless](https://code.claude.com/docs/en/headless) ·
[agent-sdk/sessions](https://code.claude.com/docs/en/agent-sdk/sessions) ·
[scheduled-tasks](https://code.claude.com/docs/en/scheduled-tasks) ·
[routines](https://code.claude.com/docs/en/routines) ·
[sub-agents](https://code.claude.com/docs/en/sub-agents) ·
[skills](https://code.claude.com/docs/en/skills.md) ·
[plugins-reference](https://code.claude.com/docs/en/plugins-reference.md) ·
[discover-plugins](https://code.claude.com/docs/en/discover-plugins.md) ·
[plugin-marketplaces](https://code.claude.com/docs/en/plugin-marketplaces.md) ·
[claude-code#15553](https://github.com/anthropics/claude-code/issues/15553) ·
[claude-code#30555](https://github.com/anthropics/claude-code/issues/30555)
