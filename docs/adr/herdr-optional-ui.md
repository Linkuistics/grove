# herdr is an optional UI over grove's artifacts, never a dependency

grove **optimises for** [herdr](https://herdr.dev) — an agent multiplexer whose
sidebar rolls per-pane state up to tabs and workspaces — without ever requiring
it. Two mechanisms, deliberately separated:

- **Semantic state** (`idle` / `working` / `blocked`) is reported *by grove* over
  herdr's socket, addressed by the `HERDR_*` variables herdr places in the pane
  environment. Detection is skipped entirely when they are absent, and a refused
  or slow socket is a no-op, never a failed launch or a stalled loop. Stock herdr
  **drops** these reports; landing them needs a two-hunk patch grove carries in a
  fork (see below).
- **Everything richer** — the task tree, the live leaf, progress — is rendered by
  a **herdr plugin that reads `.grove/` directly**. grove pushes it nothing and
  does not know it exists.

A grove with no herdr present behaves identically, minus the status surface.

## Why it binds

The split falls out of constraint 1: *the directory tree under `.grove/` is the
only state*. Because the tree on disk already **is** the status, a UI over it
needs no cooperation from grove at all — no status protocol, no push, no second
representation to drift. That is what makes "optimised-for" achievable rather
than aspirational: the plugin and the binary version independently, and neither
can break the other.

The one thing artifacts cannot supply is *semantic state at a moment in time* —
whether the agent is mid-turn, or has stopped and is waiting for a human. That is
knowledge grove holds and nothing on disk records, so it is the only thing grove
reports. Keeping the boundary exactly there is what stops the integration
metastasising into a dependency. Some of that knowledge lives in the loop driver
and some only in the harness, which is why reporting has two mechanisms rather
than one (*herdr-turn-boundary-hooks*) — but both report the same three states
over the same socket, and both vanish with no herdr present.

The alternative — herdr as grove's execution substrate, spawning panes per leaf
and sequencing through the socket API — would be more capable and would amend the
spine. Constraint 6 (*walk-away-able*) is stated about artifacts, but a loop that
assumes panes exist is a broken grove without herdr, not a plainer one. That is a
change to grove's contract, not a feature, and it is not this decision.

## Considered options

- **Push status to herdr as structured metadata** — `pane.report_metadata`,
  whose four fields are `tokens`, `display_agent`, `title` and `state_labels`.
  Rejected outright: as the *primary* channel, and as the thin display-only
  garnish this ADR once allowed. The live leaf is exactly the kind of fact
  `.grove/` already holds and the plugin already renders, so pushing it is a
  second representation of the tree on the wire — and **no reporter can hold it
  honestly**. The driver's leaf is a *forecast*, taken before the session exists
  and measured to diverge from the session's own `pick`, cross-vendor
  (*model-per-task-kind*); a grove verb gaining a reporting obligation is ruled
  out by Consequences below; and the turn hooks reach **claude only**
  (*herdr-turn-boundary-hooks*), so on codex and pi the row would be silently
  absent, where absence reads as *no grove* rather than *not reported*.
  The mechanism does not rescue it. Custom `$name` tokens render only where the
  user has listed them, and the default agent rows are
  `[state_icon, workspace, tab]` and `[agent]`. The two fields that *do* render
  unconfigured are worse rather than better: `display_agent` renders by
  **replacing** the `agent` field — work-item identity in an agent-identity
  field, against the principle that grove reports what it *is* and hints what it
  *launched* — and `title` outranks the human's own
  typed pane name in the navigator. None of it survives a restart or a
  `live-handoff`, which snapshot only cwd, label, agent name, session ref and
  argv; semantic state absorbs that by self-healing on the next report, a
  launch-time leaf name does not.
  A reader who wants a per-pane line already has one at zero cost to grove:
  `terminal_title_stripped` in their own sidebar rows — herdr's own documented
  example — which the harness writes and the handoff does carry. Reopen if herdr
  renders custom tokens in a *default* row, or if grove gains a per-turn reporter
  on **every** harness *and* the plugin stops being the richer surface.
- **Join herdr's authority allowlist.** Adding `("herdr:grove", "grove")` to the
  compiled-in `full_lifecycle_hook_authority` list is the obvious fork, and it
  **does not work**: allowlist membership is not a fast lane past the owner gate,
  it is a stricter path *before* it. An allowlisted report must satisfy
  `route_full_lifecycle_hook_report`, which demands the label parse to the
  detected agent (`grove` parses to nothing) and then requires a `session_ref`
  grove does not have — so every report would be dropped. Rejected as strictly
  worse than doing nothing. Reopen only if grove ever gains a resumable session
  identity of its own, which would be a different tool.
- **Report unforked, on an unrecognised label.** Rejected on measurement. An
  unrecognised label does bypass the agent-label gate and the screen-blocker
  override, but a third gate, `current_session_owner_conflicts`, drops any report
  whose `(source, agent)` differs from the pane's session-identity owner — which
  the harness's own herdr integration claims at every SessionStart. There is no
  unforked way through, on 0.7.5 or on current upstream, and a report that lands
  before the owner appears latches the pane at a stale state forever. Reopen if
  herdr ever accepts a state report from a source that owns no session identity.
- **Claim the pane's session identity first, or have the user uninstall the
  harness integration.** Both make grove's reports land, and both work by
  destroying herdr's session-resume for that harness — hostile to another
  integration in the first case, a configuration demand rather than a design in
  the second. Rejected. The uninstall remains usable as a documented escape
  hatch for anyone unwilling to run the patched build.
- **herdr as execution substrate** (panes per leaf, sequencing over the socket).
  Rejected as a spine change, not a feature — see above. Reopened only as a
  deliberate amendment to the constraints, with constraint 6 restated.
- **A grove plugin that reports state as well as rendering it.** Not possible:
  plugin v1 declares argv commands in a manifest, cannot register actions at
  runtime, and cannot add socket methods or join the authority allowlist. A
  plugin has exactly the socket access `grove-llm` already has, so routing state
  through one would add a hop and buy nothing.
- **Restructure the driver's process group** so the harness, not `grove`, leads
  the pane's foreground job (`setpgid` the harness into its own group and
  `tcsetpgrp` it to the foreground). It would work — detection reads the pane
  shell's `e_tpgid`, so the harness would win herdr's leader preference outright
  — and it needs no environment hint. Rejected as buying an outcome
  `HERDR_AGENT` already buys, at the price of rewriting the driver's job-control
  topology: SIGINT delivery (the driver's `SIG_IGN` survival is load-bearing —
  *self-driving-loop*), the SIGTERM/SIGHUP release in the table below,
  `tcsetpgrp` restore on child exit, and `SIGTTOU` handling around the call.
  That is a signal-contract change to buy a screen-manifest selection. Reopen
  only if the hint is withdrawn upstream.
- **A third fork hunk on herdr's `identify_agent_in_job` fallback.** Rejected on
  the same ground that holds the patch to two hunks: the carry is permanent, so
  every hunk is a rebase obligation forever, and a fix needing no fork beats one
  that does. It would also fix only *our* build, leaving stock-herdr users with
  the mis-label.
- **Accept the mis-label as known noise.** Legitimate only while the fix was
  expensive. It is one environment variable.
- **Offer the patch upstream as a `fix:` PR.** Rejected; the fork is a permanent
  carry, not a temporary one. The patch is inside the automated intake budget
  (1 file, +74/−2, against a limit of 20 files / 1000 lines) and a `fix:` title
  is honest, so the gate was never the obstacle. The cost is what follows a
  submission: herdr is an explicitly opinionated solo-maintainer project whose
  `CONTRIBUTING.md` reserves the right to close anything off-direction, the
  patch sits in the hook-authority seam the maintainer is actively rewriting
  (`state.rs` took +1281/−812 while this grove ran), and an open PR is a standing
  obligation to track a fast-moving `master` and answer two review bots. Against
  that, the only prize is ending a carry that is already shipping and measured
  working. Reopen if upstream separates session identity from lifecycle state on
  its own — which retires the patch outright — or if the recurring rebase ever
  costs more than the submission would.

## The patch, and what it costs

herdr conflates two separable concerns: who owns a pane's *session identity*
(for resume) and who reports its *lifecycle state*. `set_hook_authority_at` lets
the identity owner veto any differently-sourced state report, and clears the
identity record on every accepted one. grove carries a two-hunk fix encoding one
principle — **a hook report that makes no session-identity claim neither
conflicts with, nor clears, the identity owner** — gating both behaviours on
`session_ref.is_some()`. grove never sends a `session_ref`, so it reports state
without touching identity, and the harness's own session-resume survives intact.

The fork is shipped from `linkuistics/taps` alongside grove itself, versioned as
upstream's version plus a local suffix, and tracked closely against upstream —
the discipline that keeps a recurring rebase cheap, and the reason the patch is
held to two hunks rather than a more thorough refactor of the same seam.

**The carry is permanent.** Offering the patch upstream was considered and
rejected (see above), so nothing is expected to end it except upstream reaching
the same separation independently. That raises the price of every future hunk:
each one is a rebase obligation forever, so the two-hunk discipline above is now
a standing constraint rather than a courtesy to a reviewer.

grove takes **precedence** over screen detection, not authority *instead of* it:
staying outside herdr's allowlist keeps `fallback_state` live underneath. Two
known gaps followed. The first is accepted here; the second is now fixed, and the
fix needed no fork hunk.

grove's authority has **no expiry** in herdr, so a driver killed uncatchably
(SIGKILL, panic, OOM) pins the pane at its last reported state until
`herdr pane release-agent`. A staleness TTL would fix it and is rejected: it
changes behaviour for every third-party reporter, and it is a second principle
in a patch deliberately held to one.

The `fallback_state` underneath **was not reliable for grove panes**: herdr
identifies the agent from the pane's foreground process group, whose leader is
`grove` — a name herdr cannot identify — so detection fell back to scoring the
whole group and settled on a `codex mcp-server` helper rather than the harness
grove launched. grove closes that by setting herdr's documented
`HERDR_AGENT=<harness>` hint on every harness child it spawns, at both launch
sites. The probe consults every **non-leader** member of the foreground job for a
hint *before* reaching the group scoring that misfired, so one environment
variable is the whole fix — on stock herdr as much as on the fork.

The principle, and the reason the two surfaces do not collide: **grove reports
what it *is* (`grove`); it hints what it *launched* (`claude` / `codex` / `pi`).**
They are different fields serving different jobs — the report carries semantic
state grove alone knows, the hint selects which screen manifest herdr parses the
TUI with — and keeping them distinct is what lets both be honest at once. The
hint cannot disturb the reports: a screen-detected blocker overrides a hook report
only when `parse_agent_label(authority.agent_label) == detected_agent`, and
grove's label parses to `None`, so moving `detected_agent` from `Some(Codex)` to
`Some(Claude)` moves nothing in that gate.

The fix is **measured, not merely reasoned** (2026-07-28, against the installed
`0.7.5-linkuistics.1`). On two real grove panes with byte-identical process
shapes — leader `grove`, a live `claude`, a `codex mcp-server` helper — the
hint-less build read `agent: codex` (and `agent_status: done` while claude was
alive), and the hint-carrying build read `agent: claude`. On a real `grove do`
pane with grove's authority released to expose detection underneath, the pane
re-acquired as `claude` and herdr then read a stalled grilling session as
**`blocked`** off claude's own screen manifest — the `fallback_state` this
paragraph used to disclaim, working. The hint is inherited by the harness's whole
process subtree, so even the `codex` helper carries `HERDR_AGENT=claude` and
cannot win the hint path either.

## When grove releases — and the two cases where it must not

Release is **not** "on every catchable exit". That was this ADR's own earlier
formulation and it is wrong: it would undo the fix a moment after making it.
grove releases exactly when it stops having an opinion about the pane, and holds
its report when the opinion is *"a human is needed"*:

| the loop | reports | releases |
|---|---|---|
| launching a session | `working` | — |
| relaunching (per-task signal) | nothing — the next launch re-reports `working` | — |
| `complete --done` (grove finished) | `idle`, then release | **yes** |
| stopped with no signal (`/exit`, Ctrl-C, crash) | `blocked` | no |
| stopped by the version-skew guard, or an error | `blocked` | no |
| SIGTERM / SIGHUP | nothing | **yes** |

The no-signal row is the whole point. Releasing there hands the pane straight
back to screen detection, which reads a parked grove as `idle` — herdr's derived
`done` — which *is* the "stalled overnight, shows as finished" complaint. And
`blocked` is not a stale leftover: the grove has live leaves and genuinely needs
a human, and it self-heals, because the next session reports `working` over the
top of it once the owner gate no longer vetoes later reports.

**SIGINT needs no handler**, contrary to what the route that chose this recorded.
The driver already sets SIGINT to `SIG_IGN` so it survives the human's Ctrl-C and
reaches the relaunch-vs-stop decision; a Ctrl-C that stops the loop does so by
killing the *session*, which the driver then sees as a no-signal exit and reports
as `blocked` down the ordinary path. SIGTERM is the case that genuinely needed
new code — with SIGHUP alongside it, since that is what a closing pane delivers
to its foreground process group.

Whether a crash, a deliberate `/exit`, and a Ctrl-C should read differently was
considered and rejected. The driver *can* separate a crash from the two clean
exits (the child's exit status), but not `/exit` from Ctrl-C — both are the
harness exiting cleanly at the human's request. More decisively, the right report
is `blocked` for all three: `pick` still has live leaves in every case, so
anything else would claim the grove had progressed when it had not. grove
therefore does not read the exit status at all, rather than inventing a
distinction that changes nothing.

## Consequences

- grove reports as agent **`grove`**, not as the harness it launched. This is
  honest — a `grove do` pane is a loop relaunching a sequence of sessions, and
  the harness may vary per leaf — and it sidesteps herdr's gate that silently
  drops a report whose label names a different *known* agent than the one it
  detected.
- grove **hints** `HERDR_AGENT=<harness>` on every harness child, at both launch
  sites, unconditionally — not gated on being inside a herdr pane, unlike the
  turn hooks. The asymmetry is deliberate: injecting hooks changes the launch
  argv and arms a per-tool-call subprocess, while an environment variable changes
  no argv and spawns nothing, and nothing but herdr ever reads it. Because the
  hint rides the harness's environment, it dies with the harness — which is why a
  released pane re-acquires the harness only while one is still running.
- **The status surface, not the loop, is what depends on the patched build.**
  Under stock herdr the reports are dropped and the pane falls back to screen
  detection — the same outcome as no herdr at all, which is exactly what this
  ADR already promises. The optional-UI claim therefore survives the fork: it
  now reads *a grove with no herdr, or with stock herdr, behaves identically,
  minus the status surface.*
- **The fork is a standing maintenance obligation, not a temporary one.** Every
  upstream release means a rebase of `authority-fix` and a merge into the ship
  branch, indefinitely. That is the price of the status surface, and it is the
  reason no further hunk joins the patch without clearing the same bar the first
  two did.
- grove must **release** its authority when it stops having an opinion, because
  herdr will never expire it — see the table above for which exits release and
  which deliberately hold a `blocked` report instead. That obligation is real
  code in the loop driver, including a SIGTERM/SIGHUP handler it did not
  previously have.
- **The driver's own reporting stops at session boundaries**, because the driver
  is the harness's parent and sees a session start, a session end, and nothing
  in between. What happens **inside** a session — a turn ending, and a dialog
  stalling one mid-turn — reaches herdr by a second mechanism: hooks grove
  injects into the launch, so the harness reports them itself. That exists for
  **claude** only; see *herdr-turn-boundary-hooks* for how, and for why codex and
  pi are blocked on facts rather than on effort. On codex and pi a session that
  stalls mid-turn still reads `working`, not `blocked`: better than the pre-patch
  `done`, but not the whole fix.
- The plugin's only contract is the `.grove/` directory scheme
  (*task-tree-scheme*), which is already published and stable. Changing that
  scheme is now also a plugin-compatibility question.
- No grove verb gains a reporting obligation; `leaf-add`, `leaf-retire` and the
  rest stay pure working-tree changes.
- Nothing here is load-bearing for the loop. If herdr, the socket, or the plugin
  is absent or broken, `grove do` is unaffected.
