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
  they currently cannot observe.
- **herdr-turn-hooks-k4** — intra-session turn boundaries, per harness. Refines
  **herdr-pane-state-k2**; claude first (cleanest injection), codex and pi after.
  herdr's own retired claude event→state mapping is in Notes below — start there.
- **herdr-grove-plugin-k5** — the plugin. Depends only on the `.grove/`
  directory scheme, so it can follow **herdr-pane-state-k2** at any point.
- **jj-first-coverage-k6** — the jj path is primary in code but untested, and
  the docs still lead with git.
- **herdr-pane-misdetection-k11** — planning. grove panes are labelled with the
  wrong agent; upstreaming is closed, so the route is ours to pick (grove-side,
  fork-side, or accept). Late because grove's own reports mask it whenever grove
  holds authority. Independent of everything above.
- **tap-caveats-reconcile-k24** — the Homebrew formula's caveats still describe
  upstreaming as pending. Text-only, independent, low priority.

## Pointers

- ADRs a session here must read: *herdr-optional-ui*, *self-driving-loop*,
  *task-tree-scheme*.
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
- **The status surface is not live in production**, for two independent reasons,
  neither a defect. The shipped `grove` 15.0.0 carries **no reporter at all** —
  the binary contains no `HERDR_SOCKET_PATH`, `HERDR_PANE_ID` or
  `pane.report_agent`, because HEAD is unreleased at the same version number. And
  the running herdr *server* predates the patched build, which
  `docs/specs/herdr-fork-maintenance.md` warns leaves the patch inert until herdr
  restarts. **status-surface-live-k23** closes both. Until it does, any
  observation of a live pane is an observation of the *pre-reporter* world.
