# grove.improve-signaling-to-herdr — brief

## Goal

Make a running grove **legible from outside the session**: herdr should show,
accurately and without screen-scraping, whether a grove pane is working, blocked
on a human, or finished — and what it is working on. The grove's name records
where this started (herdr's `blocked` state not firing when an agent stops to ask
a question); the work grew to cover how grove reports state at all, and how a
herdr plugin can render the task tree.

## Done when

- A herdr pane running `grove do` reports `working` / `blocked` / `idle` from
  grove's own knowledge rather than from regex over the terminal buffer — in
  particular, a grove stalled on a HITL question reads as **blocked**, not
  **done**.
- A herdr plugin renders the live `.grove/` tree, driven entirely by the
  artifacts on disk.
- herdr stays **optional**: with no herdr present, every grove behaviour is
  unchanged (*herdr-optional-ui*).

## Decomposition

Listed in position order, which encodes dependency and value-first sequencing —
but each item is named by its stable `<slug>-k<key>` handle, because positions
shift under `leaf-insert` and handles do not (*task-tree-scheme*).

- **herdr-pane-state-k2** — driver-level state reporting. Harness-agnostic:
  the driver is the parent process whatever it spawned, so this needs no hooks
  and no per-harness work. Now a node: measurement showed the unforked mechanism
  is vetoed by herdr, so the route was settled first (fork, two-hunk patch — ADR
  *herdr-optional-ui*), then patch → reporter. **Done.** Its fourth leaf, the
  upstream PR, was abandoned: grove does not contribute upstream. The reporter
  is written and tested; what it cannot reach — intra-turn state — is
  **herdr-turn-hooks-k4**, and what stops it being *live* is
  **status-surface-live-k23**.
- **task-kind-taxonomy-k3** — now a node. Opened as `harness-on-leaf`
  (planning): move harness selection from per-grove env onto the leaf. The
  grilling reframed it — the sequencing problem was a *taxonomy* problem, not a
  harness-location one — so it is now the five-kind → seventeen-kind rework plus
  both routing axes. Its brief carries the design. Independent of
  **herdr-pane-state-k2**.
  **Done.** Seventeen kinds, `work` → `impl` without breaking live groves, both
  routing axes (family fallback harness-major, per-leaf `**Harness:**`), and a
  hard failure on a kind that resolves no model var. The durable record is
  `docs/specs/task-kind-taxonomy.md` plus *task-kind-taxonomy* and
  *model-per-task-kind*, both reworked in place. Four of its ten leaves were
  consequences discovered by the sweep, not planned — the taxonomy's real cost
  was the doc and config surface, not the enum. Shipped as **v16.0.0** by
  `ship-release-k25`. Work in this repo still runs against
  `./target/debug/grove-llm` whenever it depends on unreleased behaviour — as of
  now, the whole v16.1.0 CHANGELOG entry, which is written but not cut.
- **herdr-notes-reverify-k17** — re-verify this brief's herdr Notes and the
  fork-maintenance spec against the fork's current state, once, ahead of the
  leaves that depend on them. A separate workstream had moved the fork.
  **Done.** Every Notes claim held; the spec needed two corrections, not a
  rewrite. Its real yield was two things nobody was looking for: the surface is
  inert in production (**status-surface-live-k23**) and the tap's caveats
  contradict the ADR (**tap-caveats-reconcile-k24**).
- **status-surface-live-k23** — ship the reporter and get the patched herdr
  server actually running, then pass the fork-maintenance acceptance test on a
  real `grove do` pane. Inserted ahead of the two leaves that refine a surface
  they currently cannot observe. Now a node: a `grove do` pane's ancestry is
  `herdr → shell → grove → harness`, so a session here is a grandchild of both
  processes that must be replaced, and cannot watch its own replacement. Hence
  **ship-release-k25** (cut the release; prove the installed binary reports) then
  **observe-live-surface-k26** (acceptance test under a new driver and a
  restarted server).
  **Done.** The surface is live and measured on a real pane — see Notes. The
  server was replaced by `herdr server live-handoff`, which preserved every pane
  including the one that issued it, so the "restart kills every pane" cost this
  node was sequenced around turned out not to exist. Three rows of ADR
  *herdr-optional-ui*'s table remain unobserved (SIGTERM/SIGHUP, version-skew
  stop, relaunch); none was in scope here, and they now ride along with
  **observe-mid-turn-live-k31**.
- **herdr-turn-hooks-k4** — intra-session turn boundaries, per harness. Refines
  **herdr-pane-state-k2**; claude first (cleanest injection), codex and pi after.
  **Done.** claude launches under herdr now carry an inline `--settings` hook
  block wiring `UserPromptSubmit` ⇒ `working` and `Stop` ⇒ `blocked`-unless-
  signalled; measured live, on the real `claude`, on all three signal cases. The
  durable record is ADR *herdr-turn-boundary-hooks*. codex and pi are deferred on
  **facts, not effort** — codex has no turn-end hook event and persists hook
  trust per content hash; pi's own herdr extension reports `idle` at turn end,
  the same conflation. What it did *not* cover was mid-turn blockers — closed by
  **herdr-mid-turn-blockers-k30**, which widened the same ADR rather than adding
  one.
- **herdr-mid-turn-blockers-k30** — the gap inside a turn: a permission prompt
  stalls an unattended loop exactly as badly as a question, and grove's own
  authority suppressed the screen detection that used to catch it by accident.
  Deferred out of **herdr-turn-hooks-k4** because `blocked` there needs a paired
  restore that only a per-tool-call event gives — a different design, not a
  bigger version of the same one.
  **Done.** Two more rows in the injected block, as a pair: `Notification`
  (matched to three dialog types) ⇒ `blocked`, `PostToolUse` ⇒ `working`. Both
  design questions were settled by reading the shipped claude binary rather than
  its self-contradicting docs — see Notes. Redundancy suppression was
  *rejected*, contra the leaf's opening framing: grove's hook is a fresh process,
  so remembering costs what it saves, and a report per tool call is what
  re-asserts authority after a herdr restart. What it could not do is watch a
  real permission prompt end to end — that is **observe-mid-turn-live-k31**.
- **observe-mid-turn-live-k31** — the one measurement only production has: a real
  pane, parked on a real permission prompt, reading `blocked`. Carries the three
  rows of *herdr-optional-ui*'s table nobody has observed (SIGTERM/SIGHUP,
  version-skew stop, relaunch), which were homeless once **herdr-turn-hooks-k4**
  finished. Needs v16.1.0 shipped first; the version-skew guard makes that a
  single leaf rather than two (the leaf explains how).
  **Headline observed; one short session left.** v16.1.0 is shipped and
  installed, and under that driver a real permission dialog held past six seconds
  read **`blocked`** and returned to `working` on grant, mid-turn — see Notes.
  The version-skew row is observed too. What remains is only the *recording* of
  the last two rows (relaunch, SIGTERM/SIGHUP): both are armed by a detached
  observer that acts after the arming session is dead, so the leaf stays live for
  one session whose whole job is to read
  `target/k31-relaunch-interrupt.log` and retire. Costs: two release-path
  frictions, both folded into **release-doctor-toolchain-gap-k27**, which is now
  a two-friction leaf rather than the doctor alone.
- **herdr-grove-plugin-k5** — the plugin. Depends only on the `.grove/`
  directory scheme, so it can follow **herdr-pane-state-k2** at any point.
- **jj-first-coverage-k6** — the jj path is primary in code but untested, and
  the docs still lead with git.
- **compose-task-chains-k29** — make the review chain (`X` → `review-X` →
  `integrate-review-X`) and the research vendor pair the *habitual* shape a
  session cuts leaves in — in `SKILL.md`'s Decompose step, in `TASK-FORMAT.md`,
  and in the **bootstrap** prompt, which cuts the decomposition that shapes every
  later session — and give a cut chain a naming structure that makes it legible
  from `find .grove` alone. Encouragement only: grove validates no ordering
  between leaves and this must not start (*task-kind-taxonomy*). Raised by the
  human during **herdr-turn-hooks-k4**; independent of the herdr work. Sequenced
  ahead of the remaining planning leaf so that session cuts under the new
  guidance.
- **herdr-pane-misdetection-k11** — planning. grove panes are labelled with the
  wrong agent; upstreaming is closed, so the route is ours to pick (grove-side,
  fork-side, or accept). Late because grove's own reports mask it whenever grove
  holds authority. Independent of everything above.
- **tap-caveats-reconcile-k24** — the Homebrew formula's caveats still describe
  upstreaming as pending. Text-only, independent, low priority.
- **release-doctor-toolchain-gap-k27** — two frictions the release path makes the
  operator remember. `release-doctor.sh` passes while the release build dies,
  because the doctor asks *rustup* what targets are installed and the build asks
  whatever `cargo` is on `PATH` (found by **ship-release-k25**, and it bit again
  in **observe-mid-turn-live-k31**); and `cargo release` refuses jj's
  always-detached HEAD, so a cut needs `--allow-branch HEAD` that `release.toml`
  should carry. Independent of the herdr work, sequenced last.
- **session-leaf-binding-k28** — design. The driver resolves the leaf *before* the
  session exists (that peek is what binds harness and model), then hands the
  session no leaf identity, so the session re-picks independently. They agree only
  because nothing mutates `.grove/` in between — an unenforced coincidence whose
  failure mode is a session silently running the wrong model for its kind. Decide
  whether to bind, and reconcile the skill's Pick step with where the pick really
  happens. Raised by the human during **observe-live-surface-k26**; independent of
  the herdr work.

## Pointers

- ADRs a session here must read: *herdr-optional-ui*, *self-driving-loop*,
  *task-tree-scheme*.
- *herdr-turn-boundary-hooks* — the second reporting mechanism (hooks grove
  injects into a claude launch), and why codex and pi are blocked on facts.
- *model-per-task-kind* — **task-kind-taxonomy-k3** reworked the mechanism that
  ADR describes.
- Glossary terms in play: herdr integration, Kind routing, HITL/AFK,
  Task kind (see `CONTEXT.md`).
- herdr's source is checked out at `~/Development/herdr` — `AntonyBlakey/herdr`,
  a fork with `ogulcancelik/herdr` as `upstream`. The reducer deciding whether a
  state report lands is `src/terminal/state.rs`; screen manifests are
  `website/agent-detection/<agent>.toml`; integration assets (the installed hook
  scripts) are `src/integration/assets/<agent>/`.
- The fork is **already in production**: `/opt/homebrew/bin/herdr` resolves to
  `linkuistics-herdr`, built from the fork and shipped from `linkuistics/taps`,
  the same tap grove ships from. How the carry is maintained — branch layout,
  rebase cycle, version suffix scheme, the required build environment, and the
  A/B check that verifies a rebase — is
  `docs/specs/herdr-fork-maintenance.md`. Read it before touching the fork.
- **We do not contribute upstream.** Offering the authority patch as a `fix:` PR
  was drafted and then decided against; the fork is a permanent carry (ADR
  *herdr-optional-ui*). Do not re-propose it, and do not file issues upstream
  either. Read the fork as ours to maintain, not as a staging area for
  contributions.

## Notes

Findings the leaves above depend on, **re-verified in full on 2026-07-27** by
`herdr-notes-reverify-k17` — against `upstream/master` at `dc2506ea`, with
`authority-fix` at `b1484e37` and `ui-layout` at `d17e0f42`. Every claim held;
two were widened. They are stated as **behavioural contracts, not line
references**, because `state.rs` moves under them. This is still a repo we do not
control: re-verify anything load-bearing before building on it.

- herdr's `claude` and `codex` integrations are **session-identity only, by
  design**. Each installed hook script gates on
  `case "$action" in session) ;; *) exit 0 ;;`, and the only RPC it can send is
  `pane.report_agent_session`; neither pair is in the authority allowlist. For
  those agents, 100% of `idle`/`working`/`blocked` comes from regex over the
  terminal buffer.
- herdr **used to** install lifecycle hooks for both and now installs exactly one,
  `SessionStart → session`. Both the install *and* the uninstall path strip the
  retired set, which is wider than this brief previously recorded — for claude:
  `SessionStart→idle`, `UserPromptSubmit→working`, `PreToolUse→working`,
  `PostToolUse→working`, `PostToolUseFailure→working`, `SubagentStop→working`,
  `PermissionRequest→blocked`, `Stop→idle`, `SessionEnd→release`. That list is
  herdr's own retired event→state mapping for claude, and it is where
  **herdr-turn-hooks-k4** should start rather than re-deriving one. Note it maps
  `Stop → idle`, which is exactly why it never helped — see the next point.
- **`done` is derived, not reported**: `Idle && !seen`, at three independent
  sites (the agent view's status name, the API's pane status, the navigator's
  `Done` filter). The real state machine is idle/working/blocked, so "finished"
  and "waiting on you" both land on `idle` unless something reports `blocked`.
- A state report whose `agent` label parses to a *different known agent* than the
  one herdr detected is **silently dropped**; an **unrecognised** label (`grove`)
  bypasses that gate, because the check only fires when the label parses to
  something — and `grove` is absent from herdr's agent-name table. It also
  prevents a screen-detected blocker from overriding the report. Both halves
  measured true by **herdr-pane-state-k2** — but they are not sufficient:
- **De-facto unforked authority does not exist.** A *third* gate drops any report
  whose `(source, agent)` differs from whoever owns the pane's **session
  identity** — and that owner is the harness's own herdr integration, claimed at
  every SessionStart. The session-identity-only integrations dismissed above as
  inert are exactly what lock grove out. This fired *herdr-optional-ui*'s own
  reopening condition for the fork option; `herdr-authority-route-k7` settled it,
  and the ADR carries the outcome.
- Full lifecycle authority is a **compiled-in allowlist** — still exactly six
  `(source, agent)` pairs (`pi`, `omp`, `mastracode`, `opencode`, `kilo`,
  `kimi`), with `hermes` alone in the separate session-identity-only category.
  Nothing reachable from outside the binary — a plugin included — can join it,
  which is why the plugin owns UI only, never state. It is *also* not the way in
  for grove, and the reason is sharper than "the allowlisted path demands a
  session_ref": a **non**-allowlisted report is waved straight through the
  routing step, whereas an allowlisted one whose label does not parse to the
  detected agent falls through to a branch requiring both a `session_ref` and a
  `seq`, and is dropped without them. Allowlist membership is a stricter path,
  not a fast lane.
- grove panes are **mis-detected** — now observed live, not inferred: a `grove
  do` pane reads `agent: codex` while its `agent_session` is owned by
  `herdr:claude`. The brief originally blamed MCP servers inheriting the
  harness's process group; herdr already defends against precisely that (upstream
  #161, v0.5.11) by preferring the process-group *leader*. The defence misses
  because the leader of a grove pane is **`grove` itself**, unidentifiable to
  herdr, so it falls back to scoring the whole group — where a `codex mcp-server`
  helper outranks the real harness. See `CONTEXT.md` and
  `herdr-pane-misdetection-k11`.
- **The status surface is live in production, and verified end to end**
  (2026-07-27, `observe-live-surface-k26`). Both silences that made it inert are
  closed: `ship-release-k25` shipped **v16.0.0** so the installed `grove` carries
  the reporter, and the server was replaced by live handoff (PID 3825 → 77248),
  so the running herdr carries the patch. What was measured on a real pane whose
  `agent_session` was owned by `herdr:claude`: the fork acceptance test passes on
  all four points (report lands, a *second differing* report lands, release
  returns the pane, `agent_session` byte-identical throughout), and on a real
  `grove do` pane the driver reports `working` at launch, **`blocked` held** on a
  no-signal stop, and **releases** on `complete --done`. `working` was also seen
  overwriting a stale `blocked` — the ADR's self-healing claim, observed.
  Two observation nuances, both now in the spec: release reads `agent: null` /
  `agent_status: unknown` rather than snapping back to the detected agent, and
  the `idle` before release is not externally observable at 30 ms polling.
- **Hooks merge; they do not clobber** (measured 2026-07-27,
  `herdr-turn-hooks-k4`). claude's `--settings` takes inline JSON as an
  *additional* settings source and hooks are **unioned** across sources — proved
  twice, once against a project `settings.json` and once live, where herdr's own
  installed `SessionStart` hook claimed session identity in the same run as
  grove's injected turn reports. Hook subprocesses also inherit the driver's
  environment (`GROVE_SIGNAL_FILE`, `HERDR_*`), which is what makes the
  discriminator readable from inside the session. A `grove-llm` invocation costs
  **~3ms**, socket or no socket — so per-turn reporting is cheap, and the only
  real objection to going per-*tool-call* is socket chatter, not latency.
- **The shipped claude binary is readable, and it is the better source than the
  hooks docs** (measured 2026-07-27, `herdr-mid-turn-blockers-k30`). It is a
  Mach-O with the bundled JS inside, so `strings` over
  `~/.local/share/claude/versions/<v>` gives the actual implementation; the
  docs page contradicts itself about `Notification` matchers and the
  implementation does not. What that settled, all against 2.1.220:
  matchers **do** filter `Notification`, keyed on the payload's
  `notification_type`; a matcher drawn from `[A-Za-z0-9_|]` is compared as an
  **exact-string alternation**, not a substring regex; an unknown hook **event**
  name is dropped with a warning while the rest of the block still applies (an
  unknown *matcher* value is simply inert); `permission_prompt`,
  `elicitation_dialog` and `elicitation_url_dialog` come from one shared
  *idle-notify* helper that fires **once, after six seconds with no human
  interaction**; and `PermissionDenied` fires only for the auto-mode classifier,
  never on an interactive denial. `PostToolBatch` exists and fires once per model
  round trip, but appears in no changelog entry, so it is too new to assume.
- **A `Notification` cannot be provoked from `claude -p`** — it is raised by TUI
  dialog components, so print mode never reaches one. Everything else in the
  injected block *is* reachable: feed grove's own generated `--settings` to
  `claude -p` against a `UnixListener` and diff a tool-using prompt against a
  tool-free one. Driving interactive claude under `expect` was tried four times
  and did not get the model as far as a permission prompt.
- **codex has no turn-end hook event**, verified against codex-cli 0.145.0: the
  set is `pre_tool_use`, `permission_request`, `post_tool_use`, `pre_compact`,
  `post_compact`, `session_start`, `session_end`, `user_prompt_submit`,
  `subagent_start`, `subagent_stop`. Independently, hook **trust is persisted**
  per source-location and content hash in `~/.codex/config.toml`'s
  `[hooks.state]`, so a `-c`-injected hook has no trust record.
- **pi's herdr extension reports `idle` at turn end** — `agent_settled` with no
  outstanding `herdr:blocked` yields `idle`
  (`src/integration/assets/pi/herdr-agent-state.ts`). So pi is a full lifecycle
  reporter that still has the headline bug. It also dedups (`lastState`), which
  is the pattern to copy if grove ever needs redundancy suppression. And
  `pi -e <path>` **is** a per-launch injection route — contra
  **herdr-turn-hooks-k4**'s original note that none was found.
- **The turn hooks are in the installed binary from v16.1.0** (shipped
  2026-07-28, `observe-mid-turn-live-k31`). Until then everything
  **herdr-turn-hooks-k4** and **herdr-mid-turn-blockers-k30** built was invisible
  in production for the same reason the reporter was before v16.0.0: it was in
  the tree, not in the binary that launches sessions. `strings` on the installed
  `grove` now shows `report-turn`, `UserPromptSubmit`, `PostToolUse`,
  `Notification`, `permission_prompt` and `elicitation_dialog`. **The one-line
  check that a session is running under a hook-carrying driver is its own argv**:
  `ps -o command= -p $PPID` shows `--settings` or it does not.
- **The mid-turn pair is observed end to end in production** (2026-07-28,
  `observe-mid-turn-live-k31`), on claude 2.1.220 under the v16.1.0 driver. A
  real permission dialog left untouched ~10s reported **`blocked`**, and granting
  it returned the pane to `working` *mid-turn* — a further tool call followed in
  the same turn. Neither half is an artefact of hand-invoking `report-turn`;
  `PostToolUse ⇒ working` was separately isolated by putting the pane at
  `blocked` inside a tool call and watching the hook restore it with no report
  command running. The sidebar rendering is confirmed against herdr's own source:
  `state_dot` maps `Blocked → red`, and **red is unique to blocked** across all
  five rows (`src/ui/status.rs`), so the red dot an operator sees *is* `blocked`.
- **The version-skew stop row is observed**: `blocked` 14s after the signal, held
  5½ min, `agent=grove` never released — `plan_for(Stop::VersionSkew)` exactly.
- **Two bounds on how far the mid-turn row reaches**, both now in ADR
  *herdr-turn-boundary-hooks*. A **permissive permission mode raises no dialog at
  all** — under `defaultMode: "auto"` with `skipDangerousModePermissionPrompt`,
  an `rm -rf`, an explicit sandbox override and an un-allowlisted MCP call all
  ran unprompted, so nothing to report; provoking the row needs a *prompting*
  mode. And the **six-second timer is gated on human inattention, not elapsed
  dialog time**: dialogs held several seconds with the human present did not fire
  it. Both are the design working (it detects *unattended*), but they mean the
  row is not a general answer to "grove is stuck".
- **`herdr pane get`'s `revision` is not a report discriminator** — it tracks
  pane lifecycle, not agent state, and stayed put across three state changes. So
  "reported the same state" and "reported nothing" remain indistinguishable from
  the socket; the only way to tell a silent row from a re-reporting one is to
  arrange a *different* pre-state, or to watch for the `agent=null` that only
  release produces.
- **Cutting a release needs two workarounds this repo does not record.** `export
  PATH="$HOME/.cargo/bin:$PATH"` (Homebrew's cargo otherwise wins and the Linux
  targets fail on a missing `std`), and `cargo release … --allow-branch HEAD`
  (jj colocation keeps git's HEAD detached, and cargo-release's default
  `allow-branch` is `["*", "!HEAD"]`). Both belong to
  **release-doctor-toolchain-gap-k27**. The rest of the path is unremarkable and
  jj-clean: git makes the detached release commit and tag, jj imports it, and
  `jj bookmark set main -r <release-change>` puts the bookmark on it.
- **A herdr restart need not kill every pane** — and the route is a **plain CLI
  subcommand**, `herdr server live-handoff --import-exe <path>`, listed in
  `herdr server`'s own help. Earlier notes here claimed no CLI path existed and
  that a raw `server.live_handoff` socket call was required; that was wrong.
  Demonstrated: every pane survived the swap, including the pane that issued it.
  Costs are bounded — TUI clients disconnect and must reattach, and a handoff
  carries at most 64 panes. It stays the human's call because it interrupts their
  UI. **Do not reach for `herdr update --handoff`**: that fetches *upstream*
  herdr and would clobber the fork.
