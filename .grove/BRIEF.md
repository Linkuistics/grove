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
  and no per-harness work. Smallest slice that fixes the headline complaint, and
  it proves the socket path end-to-end.
- `03` **harness-on-leaf** — planning. Move harness selection from per-grove env
  (`GROVE_<KIND>_HARNESS`) onto the leaf, so a node can sequence work across
  vendors (impl → review → integrate). Independent of `02`.
- `04` **herdr-turn-hooks** — intra-session turn boundaries, per harness.
  Refines `02`; claude first (cleanest injection), codex and pi after.
- `05` **herdr-grove-plugin** — the plugin. Depends only on the `.grove/`
  directory scheme, so it can follow `02` at any point.
- `06` **jj-first-coverage** — the jj path is primary in code but untested, and
  the docs still lead with git.

## Pointers

- ADRs a session here must read: *herdr-optional-ui*, *self-driving-loop*,
  *task-tree-scheme*.
- *model-per-task-kind* — `03` reworks the mechanism that ADR describes.
- Glossary terms in play: herdr integration, Per-kind model selection, HITL/AFK,
  Task kind (see `CONTEXT.md`).
- herdr's source is checked out at `~/Development/herdr` — a fork of
  `ogulcancelik/herdr` with an `upstream` remote. The reducer deciding whether a
  state report lands is `src/terminal/state.rs`; screen manifests are
  `website/agent-detection/<agent>.toml`; integration assets (the installed hook
  scripts) are `src/integration/assets/<agent>/`.

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
  from overriding the report — which appears to give de-facto exclusive authority
  with no fork changes. **Hypothesis from reading the reducer, not measured**;
  `02` should measure it.
- Full lifecycle authority is a **compiled-in allowlist** of seven
  `(source, agent)` pairs. Nothing reachable from outside the binary — a plugin
  included — can join it. This is why the plugin owns UI only, never state.
- grove panes are currently **mis-detected**: MCP servers inherit the harness's
  foreground process group, so a `codex` MCP server running under `claude` makes
  herdr identify the pane as codex and evaluate the wrong manifest.
