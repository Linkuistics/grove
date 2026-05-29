# 020-extend-status-with-worktree-versions

**Kind:** work

## Goal

Extend `grove status` to read each grove worktree's `VERSION.md` per
harness, compute drift against the repo's installed `VERSION.md`, and
render output per the agreed shape (option α in the running log of
[[010-shape-the-feature]]).

## Context

- `src/status.rs` — current implementation; already reads repo
  `VERSION.md` per harness via `version_md::read_version` and shows
  groves with `(live, done)` leaf counts.
- `src/version_md.rs` — `VERSION.md` reader/writer; same accessor works
  for repo and worktree paths.
- `src/repo.rs` — `repo::grove_worktrees_dir` gives the per-grove
  worktree root.
- `src/harness.rs` — `HARNESSES` and `install_path` are how to locate
  the per-harness `<worktree>/<install-path>/VERSION.md`.
- Decisions from [[010-shape-the-feature]] (running log) that drive this
  leaf:
  - Layer names `cli` / `repo` / `worktree`.
  - Drift rule: raw string-equality, both shown on mismatch, no semver.
  - Output shape α (append per-row).
  - Unknown (missing/malformed `VERSION.md`) is `(unknown)`, not drift.
  - Orphan worktrees (harness uninstalled in repo) show `repo=(none)`,
    not warned unless cli/worktree itself drifts.
- The CLI version shown in the header line is `env!("CARGO_PKG_VERSION")`.

## Done when

- `grove status` output header line reads `grove cli X.Y.Z, installs in
  <repo>:` (CLI version added).
- Each grove row shows `… leaves:L/R worktree=W.X.Y` and, on mismatch,
  trailing `⚠ repo=A.B.C` (or `⚠ cli=…` if drift is against the CLI).
- The existing `harness=…` row column from `print_grove_summary` is
  removed (multi-harness handled with a `[harness]` row prefix when
  more than one is installed).
- Unknown / orphan cases render as agreed above.
- Unit tests cover: aligned, drift between worktree and repo,
  drift between worktree and cli, unknown worktree `VERSION.md`,
  orphan worktree (harness no longer installed in repo).

## Notes

- This leaf is foundational for [[050-tui-version-display]] — the TUI
  needs the same worktree-VERSION reader. Consider factoring the
  per-worktree-per-harness version lookup into a small reusable helper
  (e.g. on `crate::status` or a new module), so 050 can call it.
- Don't introduce a semver dependency; string equality only.
- No ADR for this leaf — the decisions are documented in `CONTEXT.md`
  (the three-layer glossary entry) and are observable from the code.
