# herdr-grove-plugin-k5

**Kind:** work

## Goal

A herdr plugin that renders the live `.grove/` tree — which leaf is running, its
kind, what is done, what is left. All UI logic lives in the plugin and is driven
**entirely by the artifacts on disk**; grove pushes it nothing.

## Context

- herdr plugins are a directory with a `herdr-plugin.toml` manifest plus argv
  commands. `[[panes]]` with `placement = "overlay"` is the surface for a
  status view; `[[events]]` hooks and `[[actions]]` are also manifest-declared.
  The whole herdr CLI is the plugin API (`HERDR_BIN_PATH` points at the running
  binary); the socket API is there for raw JSON.
- **Plugin v1 cannot register actions at runtime and cannot add socket methods.**
  It also cannot become a state authority — that is a compiled-in allowlist. UI
  only; semantic state stays with **herdr-pane-state-k2** and
  **herdr-turn-hooks-k4**.
- ADR *task-tree-scheme* — the directory scheme the plugin parses. This is the
  plugin's only real dependency, and it is a published contract.
- `grove-llm pick` / `brief-chain` / `kind` / `resolve` already expose the tree
  programmatically, if shelling out beats parsing.

## Done when

- A pane shows the current grove's tree with the live leaf marked, and updates as
  the loop advances.
- The plugin reads only `.grove/` (directly, or via `grove-llm`) — no new
  reporting obligation on grove, no new state file (constraint 1).
- Deleting the plugin changes nothing about grove; deleting grove leaves the
  plugin with nothing to render but breaks nothing (*herdr-optional-ui*).

## Notes

**Why artifacts rather than pushed status**: the tree on disk already *is* the
state, so the plugin needs zero cooperation from grove and the two can version
independently. This is the split that keeps constraint 6 intact — it is the whole
reason the herdr relationship can be "optimised-for" rather than "required".

Open, for this session to decide: what the pane actually renders (whole tree vs
current path vs progress summary), how it finds the grove root from the pane's
cwd, and whether it watches the filesystem or polls. Also worth deciding whether
`pane.report_metadata` tokens (e.g. `$grove_leaf`) belong here for the sidebar
row, or in **herdr-pane-state-k2** — they are display-only either way.

Depends on **herdr-pane-state-k2** only for knowing what grove itself already
reports, so the two
do not duplicate. Otherwise independent.
