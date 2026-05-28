# grove-status-should-show-active-grove-grove-versions — brief

## Goal

Two concerns, kept together because they live in the same CLI-surface
neighbourhood:

**A. Version-drift visibility.** Make grove-methodology version drift
evident at every surface a human checks: `grove status` and the TUI.
Three versions are in play and any pair can drift:

1. **`cli`** — the `grove` binary itself (`env!("CARGO_PKG_VERSION")`).
2. **`repo`** — `<repo>/.<harness>/skills/grove/VERSION.md`, one per
   installed harness.
3. **`worktree`** — `<repo>/.grove-worktrees/<name>/.<harness>/skills/grove/VERSION.md`,
   one per worktree per harness (**not currently shown anywhere**).

**B. CLI-surface cleanup.** The grilling for (A) surfaced redundant verbs:
`grove list` and `grove version` are subsets of the new `grove status`;
`grove install` and `grove update` differ only in a pre-flight safety
check (`src/install.rs:37-51`) and share `InstallArgs` and the same
backing function. Remove the redundant verbs and merge `install`/`update`
into a single verb whose shape is grilled in this leaf.

Note: the working-title phrase "active grove" turned out to be misleading
— this work has nothing to do with detecting which grove is "current".
The title is preserved as the branch/worktree name only.

## Done when

- `grove status` shows the three versions and flags drift between them.
- `grove list` and `grove version` are removed.
- `grove install` and `grove update` are merged into a single verb whose
  shape is the outcome of the grilling here.
- The TUI surfaces the cli/repo versions in a header and per-row worktree
  versions with drift markers, matching `grove status`'s rule.
- A worktree with no `VERSION.md` (legacy) is handled explicitly, not
  silently — rendered `(unknown)`, not flagged as drift.
- `CHANGELOG.md` documents the removed/merged verbs as a breaking change.

## Decomposition

Not yet decomposed beyond `010-shape-the-feature.md`. The grilling there
settles the surface-specific output shape questions before child leaves
are written.

## Pointers

- `src/status.rs` — current `grove status`; already reads repo `VERSION.md`
  per harness and shows worktrees with leaf counts. Adds drift between repo
  versions only, not between repo and worktree.
- `src/list.rs` — current `grove list`; prints one worktree name per line.
  Plain enough to be a scripting target; output-shape changes need care.
- `src/version.rs` — current `grove version`; prints CLI + per-harness repo
  versions.
- `src/version_md.rs` — `VERSION.md` reader/writer; the format is a
  markdown-table row, read via line-prefix matching.
- `src/harness_stamp.rs` — orthogonal: per-grove *harness name* stamp, not
  a version stamp. Only written in multi-harness repos.
- `src/tui.rs` — currently has zero "version" references; will need a new
  surface.
- Glossary `CONTEXT.md` — no entry yet for "version drift" or the three-layer
  model; add inline as terms settle.

## Notes

The three-layer model itself may want an ADR once written down, because
later work (e.g. `grove update` semantics, multi-harness repos) will want to
reference it.
