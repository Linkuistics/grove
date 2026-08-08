# herdr-session-kind-viewer-k51

**Kind:** impl

## Goal

Teach the separately versioned Herdr renderer the current nineteen-kind
filename grammar as an independently demonstrable consumer of the tree format.

## Context

- Depends on `session-kind-tree-integrate-k25`; no later runtime or methodology
  contraction is needed to render the final filename grammar.
- Primary artifacts: `herdr-plugin/grove_tree.py`, `herdr-plugin/README.md`, and
  filename-only renderer fixtures.
- The viewer remains UI-only and must not invoke Grove, inspect task bodies, or
  become a workflow-state authority.

## Done when

- The renderer separates one of the nineteen non-prefix filename-kind labels
  after either terminal infix and keeps the stable slug/key separate from
  routing metadata.
- Fixtures cover every kind, live/DONE/ABANDONED leaves, chain nodes, finish,
  malformed task-shaped names, and foreign files without opening task bodies.
- Viewer documentation states the filename-only compatibility contract and
  independent installation/versioning boundary.
- Focused renderer tests and the full `cargo fmt --check` / `cargo test
  --locked` suite pass.

## Notes
