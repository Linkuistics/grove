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
turns out to be false"**. It turned out to be false, and `herdr-authority-route-k7`
spent that reopening: **grove carries a two-hunk patch in a herdr fork.** The ADR
now records the settled route, the patch, and its consequences; this brief no
longer carries that argument.

Two corrections that brief made, worth keeping in front of anyone working here:

- **Line numbers below are stale.** Upstream took +1281/-812 on `state.rs` since
  these measurements. The *behaviours* re-verified true against upstream HEAD;
  the line references did not. Find things by name.
- **The latching hazard is dissolved**, not managed. With the owner gate no
  longer vetoing, grove's later reports land, so grove corrects itself while it
  is alive. What remains is a different problem — grove's authority never
  expires — which is handled by releasing on exit, not by a latch guard.

## Shipped state (established by `herdr-authority-patch-k9`)

The patch is landed, shipped, and **measured working end-to-end**. What `03`
can now assume, and what it must not:

- **Fork layout — two branches, deliberately.** `authority-fix` is off
  `upstream/master` and carries *only* the fix (one commit, +74/−2, one file):
  that is `herdr-upstream-pr-k10`'s PR branch, kept pure. `ui-layout` is the
  **ship** branch and reached the same content by **merge**, not rebase, so it
  fast-forwards from `origin/ui-layout` and never needs a force-push. Both are
  pushed. Recurring-rebase discipline is therefore: rebase `authority-fix` onto
  new `upstream/master`, then merge it into `ui-layout`.
- **Shipped as `0.7.5-linkuistics.1`.** Suffix scheme is upstream's version plus
  `-linkuistics.<seq>`, `<seq>` incrementing per ship and resetting when upstream
  bumps — a sequence, not a sha, because shas do not compare. **Gotcha, measured
  against Homebrew's own `Version`:** the one-time move from `-uilayout.<sha>`
  sorts as a *downgrade* (`linkuistics` < `uilayout` alphabetically), so that
  transition needs `brew reinstall`; later `<seq>` bumps upgrade normally.
- **`herdr --version` cannot tell you which build is installed** — it prints
  bare `0.7.5` for both, since the suffix is Homebrew's, not Cargo's. Use the
  Cellar path (`readlink -f $(which herdr)`) or `brew list --versions`.

**Correction to this brief's own `revision` claim.** The measured findings below
say a landed report advances `revision` 0→1, and `k9`'s "Done when" repeated it.
**That is wrong on current upstream**: `revision` is incremented only by
`report_metadata` token changes (`src/app/api/panes.rs`, in the `token_changed`
closure). A landed *state* report does not touch it. The observables that do
change are **`agent`** (→ the reported label) and **`agent_status`** (→ the
reported state). `03` must not wait on `revision` to confirm its own reports.

**Measured live**, A/B on one scratch pane whose session identity is owned by
`herdr:claude`/`claude`, same CLI command, differing only in which server was
listening — old unpatched server vs new patched one:

| report | old server | new server |
|---|---|---|
| `grove`/`working` | dropped, exit 0, no change | `agent=grove`, `agent_status=working` |
| `grove`/`blocked` | — | lands (**latching hazard dissolved**) |
| `grove`/`idle` | — | lands; surfaces as `done` (herdr's `idle && !seen`) |
| `release-agent` | — | authority cleared, back to screen detection |

`agent_session` stayed `herdr:claude/claude:test-sess-0001` through **every**
step including release — hunk 2 verified: grove never disturbs session-resume.

**The running server is still the old binary.** herdr's server process predates
the install, and the patch lives in the server's `state.rs`, so on the live
session grove's reports are *still* dropped until herdr is restarted. Restarting
kills every pane, so it is the human's call, not an agent's. Until then, `03`
can be written and unit-tested but not confirmed against the daily herdr.

**Build environment** (cost real time; both are traps):
- `ZIG=/opt/homebrew/opt/zig@0.15/bin/zig` is **required** — the default
  `/opt/homebrew/bin/zig` is 0.16 and the vendored `libghostty-vt` refuses it.
- `PATH="$HOME/.cargo/bin:$PATH"` is required so rustup honours the repo's
  `rust-toolchain.toml` pin (1.96.1). Homebrew's cargo is 1.97 and invents two
  clippy failures in files the patch never touches. herdr's `pre-commit` hook
  runs `just lint`, so both must be set for `git commit` to succeed.
- Two herdr tests are **flaky upstream**, independent of the patch:
  `workspace::…::generated_workspace_ids_are_short_base32_handles` and
  `api::server::pane_graphics_stream::…::inactive_owner_cancels_idle_stream_…`.
  Both pass in isolation and fail in a full run. Verified identical on unpatched
  `upstream/master` (baseline 2850 pass / 1 fail → patched 2852 / 1: exactly the
  two new tests added, zero regressions). Run with `--test-threads=1`; a default
  parallel run flakes ~12 tests on *either* tree.

## Reporter state (established by `report-plumbing-k8`)

The driver-side reporter is built, wired, and tested (`src/herdr.rs` plus four
report sites in `src/loop_driver.rs`). Three corrections it forced, all of which
outrank what earlier leaves here recorded:

- **`src/loop_driver.rs` already had a signal handler**, contrary to `k7`'s
  running log and this leaf's own brief: `run` sets SIGINT to `SIG_IGN` (so the
  driver survives Ctrl-C and reaches the relaunch-vs-stop decision). SIGINT
  therefore needed *no* work — a Ctrl-C arrives as an ordinary no-signal stop.
  SIGTERM/SIGHUP was the real new code.
- **"Release on every catchable exit" is wrong** and is not what shipped. A
  no-signal stop reports `blocked` and **holds** authority; releasing there
  returns the pane to (mis-detected) screen detection, which reads a parked
  grove as `idle` → herdr's derived `done` — the headline bug, restored. The full
  table is now in ADR *herdr-optional-ui*.
- **The mid-turn HITL stall is not closed by this node**, and genuinely needs
  `04-herdr-turn-hooks-k4`. The driver's whole vocabulary is session-start /
  session-end; it sees no turn boundaries. A session waiting on a question reads
  **`working`** — strictly better than `done` (no false green on the tab
  rollup), but not `blocked`. `04` inherits exactly this gap, and nothing else:
  transport, source/agent identity, the state vocabulary, and release are all
  settled and shared.

**Still unverified against the daily herdr, and not verifiable by an agent.**
Re-measured this session on the live pane: a `grove`-labelled report returns
`{"result":{"type":"ok"}}` and changes nothing (`agent` stays `codex`,
`agent_status` stays `idle` while mid-turn). The **running server** (pid from
Fri 24 Jul) predates the `0.7.5-linkuistics.1` install, and the patch lives in
the server's `state.rs`. Restarting herdr kills every pane, so it is the human's
call. Everything below the socket is proven against a fake herdr; the one thing
outstanding is that a real patched server accepts these exact bytes.

## Decomposition

Position order encodes dependency.

- `01` **herdr-authority-route** — planning, HITL. *Done.* Settled the route:
  fork, general fix in its minimal form, precedence not full authority, release
  on catchable exits, upstream PR on a separate non-blocking track. Two of its
  premises did not survive contact with the code — see *Reporter state* above.
- `02` **herdr-authority-patch** — land the two hunks on the fork, test them,
  ship via `linkuistics/taps`. *Done.* Shipped as `0.7.5-linkuistics.1`. See
  *Shipped state* below — including the one thing still standing between `03`
  and an end-to-end verification: the **running** herdr server predates the
  patch and must be restarted.
- `03` **report-plumbing** — the driver-side reporter: transport, the four
  report sites, the state mapping, release-on-exit (including signal handling
  the driver lacks), tests. *Done.* See *Reporter state* above.
- `04` **herdr-upstream-pr** — the same patch as an upstream `fix:` PR, plus the
  mis-detection bug filed as an issue. Deliberately last and deliberately
  non-blocking; a merge would end the fork carry, but nothing waits on it.

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

## Still open

Nothing, other than the herdr restart named in *Reporter state* above — which is
the human's call, not an agent's.

Settled, and no longer open: grove reports `agent: "grove"` (the label is not
what drops us — see the ADR); the `source` string is ours to pick, and herdr
accepts any, but it must stay **stable**, since release matches on
`(source, agent)` — shipped as `grove`/`grove`. Both of this section's former
questions were answered by `report-plumbing-k8`: a no-signal exit reports
`blocked` for a crash, a Ctrl-C and an `/exit` alike (the driver can separate a
crash from the clean exits but not the two clean exits from each other, and
`blocked` is right for all three regardless, so it reads no exit status at all);
and the mid-turn stall is **not** reachable from the driver — it needs `04`.

**Scope guard**: intra-session turn boundaries are `04`. The value of this node
is precisely that it needs no hooks — resist pulling them in.
