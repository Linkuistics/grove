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

Position order encodes dependency and value-first sequencing.

- `02` **herdr-pane-state** — driver-level state reporting. Harness-agnostic:
  the driver is the parent process whatever it spawned, so this needs no hooks
  and no per-harness work. Now a node: measurement showed the unforked mechanism
  is vetoed by herdr, so the route was settled first (fork, two-hunk patch — ADR
  *herdr-optional-ui*), then patch → reporter. **Done.** Its fourth leaf, the
  upstream PR, was abandoned: grove does not contribute upstream. The reporter
  is shipped and tested; what it cannot reach — intra-turn state — is `04`.
- `03` **task-kind-taxonomy** — now a node. Opened as `harness-on-leaf`
  (planning): move harness selection from per-grove env onto the leaf. The
  grilling reframed it — the sequencing problem was a *taxonomy* problem, not a
  harness-location one — so it is now the five-kind → seventeen-kind rework plus
  both routing axes. Its brief carries the design. Independent of `02`.
- `04` **herdr-notes-reverify** — re-verify this brief's herdr Notes and the
  fork-maintenance spec against the fork's current state, once, ahead of the
  three leaves that depend on them. A separate workstream has moved the fork.
- `05` **herdr-turn-hooks** — intra-session turn boundaries, per harness.
  Refines `02`; claude first (cleanest injection), codex and pi after.
- `06` **herdr-grove-plugin** — the plugin. Depends only on the `.grove/`
  directory scheme, so it can follow `02` at any point.
- `07` **jj-first-coverage** — the jj path is primary in code but untested, and
  the docs still lead with git.
- `08` **herdr-pane-misdetection** — planning. grove panes are labelled with the
  wrong agent; upstreaming is closed, so the route is ours to pick (grove-side,
  fork-side, or accept). Last because grove's own reports mask it whenever grove
  holds authority. Independent of everything above.

## Pointers

- ADRs a session here must read: *herdr-optional-ui*, *self-driving-loop*,
  *task-tree-scheme*.
- *model-per-task-kind* — `03` reworks the mechanism that ADR describes.
- Glossary terms in play: herdr integration, Per-kind model selection, HITL/AFK,
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

Findings from the planning grill that the leaves below depend on. All were read
out of herdr's source at planning time and belong to a repo we do not control —
**re-verify before building on any of them**.

- herdr's `claude` and `codex` integrations are **session-identity only, by
  design**: the installed hook script drops every state action. For those
  agents, 100% of `idle`/`working`/`blocked` comes from regex over the terminal
  buffer.
- herdr previously installed `Stop`/`UserPromptSubmit` hooks for both and removed
  them; the uninstall path still cleans them up. The old mapping was
  `Stop → idle`, which is exactly why it did not help — see the next point.
- **`done` is derived, not reported**: it is `idle && !seen`. The real state
  machine is idle/working/blocked, so "finished" and "waiting on you" both
  land on `idle` unless something reports `blocked`.
- A state report whose `agent` label parses to a *different known agent* than the
  one herdr detected is **silently dropped**. Reporting an **unrecognised** label
  (e.g. `grove`) bypasses that gate, and also prevents a screen-detected blocker
  from overriding the report. Both halves **measured true** by `02` — but they
  are not sufficient, and the conclusion drawn from them was wrong:
- **De-facto unforked authority does not exist.** A *third* gate,
  `current_session_owner_conflicts`, drops any report whose `(source, agent)`
  differs from whoever owns the pane's **session identity** — and that owner is
  the harness's own herdr integration, at every SessionStart. The
  session-identity-only integrations dismissed above as inert are exactly what
  locks grove out. This fired *herdr-optional-ui*'s own reopening condition for
  the fork option; `herdr-authority-route-k7` settled it, and the ADR now carries
  the outcome. Re-verified against upstream HEAD, not only 0.7.5 — but note that
  `state.rs` took +1281/-812 in the interim, so **every line number in these
  notes is stale even where the behaviour is not.**
- Full lifecycle authority is a **compiled-in allowlist** — six `(source, agent)`
  pairs on current upstream, after hermes moved out into a new
  `session_identity_only_integration()` category. Nothing reachable from outside
  the binary — a plugin included — can join it. This is why the plugin owns UI
  only, never state. It is *also* not the way in for grove: joining that list is
  verified to make things worse, since the allowlisted path demands a session_ref
  grove does not have. See the ADR.
- grove panes are currently **mis-detected**, but not for the reason this brief
  originally recorded. It said MCP servers inherit the harness's process group,
  so a `codex` MCP server under `claude` makes herdr read the pane as codex.
  herdr already defends against precisely that (upstream #161, fixed in v0.5.11)
  by preferring the process-group *leader*. The defence misses because the leader
  of a grove pane is **`grove` itself**, which herdr cannot identify, so it falls
  back to scoring the whole group. See `CONTEXT.md` and
  `herdr-pane-misdetection-k11`.
