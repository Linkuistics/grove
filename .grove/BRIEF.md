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
  was the doc and config surface, not the enum. **Still unreleased:**
  `Cargo.toml` is 15.0.0 and the shipped `grove-llm` refuses `--kind impl`, so
  work in this repo needs `./target/debug/grove-llm`.
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
  stop, relaunch); none was in scope here, and each is cheap to check under
  **herdr-turn-hooks-k4**.
- **herdr-turn-hooks-k4** — intra-session turn boundaries, per harness. Refines
  **herdr-pane-state-k2**; claude first (cleanest injection), codex and pi after.
  **Done.** claude launches under herdr now carry an inline `--settings` hook
  block wiring `UserPromptSubmit` ⇒ `working` and `Stop` ⇒ `blocked`-unless-
  signalled; measured live, on the real `claude`, on all three signal cases. The
  durable record is ADR *herdr-turn-boundary-hooks*. codex and pi are deferred on
  **facts, not effort** — codex has no turn-end hook event and persists hook
  trust per content hash; pi's own herdr extension reports `idle` at turn end,
  the same conflation. What it does *not* cover is mid-turn blockers, now
  **herdr-mid-turn-blockers-k30**.
- **herdr-mid-turn-blockers-k30** — the gap inside a turn: a permission prompt
  stalls an unattended loop exactly as badly as a question, and grove's own
  authority suppressed the screen detection that used to catch it by accident.
  Deferred out of **herdr-turn-hooks-k4** because `blocked` there needs a paired
  restore that only a per-tool-call event gives — a different design, not a
  bigger version of the same one.
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
- **release-doctor-toolchain-gap-k27** — `release-doctor.sh` passes while the
  release build dies, because the doctor asks *rustup* what targets are installed
  and the build asks whatever `cargo` is on `PATH`. Found by **ship-release-k25**;
  independent of the herdr work, sequenced last.
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
- **A herdr restart need not kill every pane** — and the route is a **plain CLI
  subcommand**, `herdr server live-handoff --import-exe <path>`, listed in
  `herdr server`'s own help. Earlier notes here claimed no CLI path existed and
  that a raw `server.live_handoff` socket call was required; that was wrong.
  Demonstrated: every pane survived the swap, including the pane that issued it.
  Costs are bounded — TUI clients disconnect and must reattach, and a handoff
  carries at most 64 panes. It stays the human's call because it interrupts their
  UI. **Do not reach for `herdr update --handoff`**: that fetches *upstream*
  herdr and would clobber the fork.
