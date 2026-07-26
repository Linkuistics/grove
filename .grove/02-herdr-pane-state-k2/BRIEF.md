# herdr-pane-state-k2 — brief

## Goal

Make `grove do` report its pane's semantic state to herdr from the **loop
driver** — the parent process — so a grove is legible without any per-harness
hooks. Fixes the headline complaint: a grove stalled overnight on a HITL
question currently reads as `done`.

Decomposed because the mechanism this was briefed on **does not work in the
normal configuration** — measured live, see below. The route has to be settled
before the reporter can be shaped, because the two candidate routes produce
materially different reporters.

## Done when

A `grove do` pane under herdr reads `working` while a session runs and
**`blocked`** while it waits on a human — accepted by herdr in the *normal*
configuration, with the harness's own integration installed — and the loop
leaves no latched pane behind when it stops.

## Measured findings (herdr 0.7.5)

Taken live, from inside a `grove do` pane under herdr, against the running
Homebrew `herdr 0.7.5`. The fork checkout at `~/Development/herdr` is
byte-identical to tag `v0.7.5` across `src/terminal/state.rs`, `src/detect/`
and `src/api/`, so source reading and live behaviour describe the same binary.

**A state report passes three independent gates, not one.** Planning found the
first two and missed the third, which is the decisive one.

1. **Agent-label gate** — `known_agent_label_conflicts_with_detected_agent`
   (`state.rs:1116`). Drops a report whose label parses to a *different known*
   agent than the detected one. `parse_agent_label("grove")` is `None`, so the
   `is_some_and` short-circuits: **bypassed, as hypothesised.** *Confirmed live*
   — reporting `grove`/`grove` to a pane with no session owner set
   `agent: "grove"`, `agent_status: "working"`.
2. **Screen-blocker override** — `visible_blocker_overrides_hook`
   (`state.rs:1339`). Requires `parse_agent_label(label) == detected_agent`;
   `None == Some(codex)` is false, so a screen-detected blocker **cannot**
   override a `grove`-labelled report. *Confirmed by source, not staged live* —
   the pane would have to be showing a real blocker prompt.
3. **Session-owner gate** — `current_session_owner_conflicts` (`state.rs:919`),
   reached from `set_hook_authority_at` (`state.rs:470-479`). **This is what
   drops us.** Any report whose `(source, agent)` differs from whoever owns the
   pane's *session identity* returns `None` unless
   `foreground_agent_confirms_hook_authority_takeover` holds — and that needs
   `parse_agent_label(our_label) == detected_agent`, false for `grove` by
   construction. There is no unforked way through it.

**Why gate 3 always fires in practice.** herdr's claude/codex integrations are
session-identity-only — they report `pane.report_agent_session` at SessionStart
and contribute nothing to state. That report sets `persisted_agent_session`
(`state.rs:1097`), which is exactly what gate 3 reads. So the integration
planning dismissed as inert is precisely what locks grove out. Live proof, this
very pane: `agent_session` = `{source: herdr:claude, agent: claude, id: 01f1…}`,
and a `grove`/`grove` report left `revision` unchanged — silently dropped,
CLI exit 0.

**A landed report latches.** `recompute_effective_state` (`state.rs:1610`)
prefers `hook_authority.state` over `fallback_state` for *any* authority, not
just allowlisted ones. Measured on a scratch pane: report `working` (lands,
`revision` 0→1) → report `blocked` (lands) → a `herdr:claude` session-identity
report arrives (accepted alongside; grove keeps the label) → report `idle`
(**dropped**). The pane stays pinned at `blocked` indefinitely. So a naive
always-report driver is *worse* than today: fresh pane → first report lands →
harness hook claims the session → every later report dropped → status frozen at
whatever grove last got in.

**`release_agent` does not undo it.** Releasing as `grove`/`grove` clears our
authority but deliberately preserves a foreign `persisted_agent_session`
(`preserve_foreign_persisted_session`, `state.rs:1265`). Measured: the session
ref survived two release attempts. Grove cannot hand the pane back to clean
screen detection once another owner exists.

**Mis-detection confirmed, and it is systemic.** Every claude pane in this herdr
instance reports `agent: "codex"` — `wQ:p1`, `wJ:p1`, `wP:p1`, all `cwd`s
running claude, all detected as codex, all `agent_status: "idle"` while
actively mid-turn. The root brief's process-group explanation stands.

**Full lifecycle authority** is the compiled-in seven-pair allowlist at
`src/detect/mod.rs:283` — `herdr:pi`, `herdr:omp`, `herdr:mastracode`,
`herdr:hermes`, `herdr:opencode`, `herdr:kilo`, `herdr:kimi`. Unchanged from
planning.

## What this means

*herdr-optional-ui* pre-committed the fork option to be **reopened "if that
turns out to be false"**. It has turned out to be false. The `01` child settles
the route; that ADR's *Considered options* has been corrected in place to
record the measurement, and the root brief's note likewise.

## Decomposition

- `01` **herdr-authority-route** — planning, HITL. Which route past gate 3, and
  what the reporter therefore looks like. Must come first: the routes produce
  different reporters (see that leaf).
- `02` **report-plumbing** — the driver-side reporter itself: transport, the
  four report sites, the state mapping, tests. Shaped by `01`.

## Context

- `src/loop_driver.rs` — `run_loop` already distinguishes the three terminal
  cases: `Disposition::Relaunch`, `Disposition::Done`, and no-signal
  (crash / Ctrl-C / `/exit`) which becomes `LoopOutcome::Stopped`. Those
  branches are the report sites; `launch_session` is the fourth.
- herdr's write side needs no discovery: `HERDR_ENV=1`, `HERDR_SOCKET_PATH` and
  `HERDR_PANE_ID` sit in the pane environment and are inherited all the way
  down. The method is `pane.report_agent` over the unix socket;
  `pane.report_metadata` carries display-only tokens.
- The `herdr pane …` CLI takes its **positional pane id first** —
  `herdr pane report-agent <PANE_ID> --source … --agent … --state …`. Flags
  before the positional fail with a bare `unknown option: <value>`. Cost ~15
  minutes; worth knowing before hand-driving the socket.

## Still open, inherited from planning

- Report `agent: "grove"`, not the underlying harness — a `grove do` pane is a
  loop relaunching a *sequence* of sessions, and the harness may vary per leaf
  once `03` lands. Unaffected by the findings above; the label is not what
  drops us.
- The `source` string is ours to pick; herdr accepts any (its own tests use
  `custom:hermes`).
- A no-signal exit lumps together a crash, a deliberate Ctrl-C, and `/exit`. Is
  `blocked` right for all three, or should a deliberate exit read as `idle`?
  The driver may be able to tell them apart from the child's exit status or
  terminating signal; if it cannot, say so plainly and pick the safer default
  rather than inventing a distinction the driver can't observe. Belongs to `02`.

**Scope guard**: intra-session turn boundaries are `04`. The value of this node
is precisely that it needs no hooks — resist pulling them in.
