# 010-plan

**Kind:** planning

## Goal

Grill the design for making `grove tui` fleet-config-driven with **no** cwd
git-repo gating or single-repo anchoring, then grow the work leaves. Settle the
open questions in the root brief before any code is written.

## Context

`grove tui` currently fails outside a git repo even with a populated
`fleet.toml`. Root cause (confirmed + reproduced in the diagnosing session):
`run_client` → `repo::resolve(None)?` bails before the fleet is built. The fleet
layer already supports `current_repo: None`, but the dashboard is constructed
around a mandatory single `repo: PathBuf` anchor (`dashboard_surface` → `App`),
which is the single-repo-era leftover to remove.

See the root `BRIEF.md` for the full pointer list and the reproduction.

This is a follow-up to `grove-always-starts-in-local-mode` (made trellis the
unconditional, fleet-surfacing TUI). That grove's CHANGELOG/ADR-0026 and the
fleet ADR-0025 are the relevant prior art.

## Done when

- The open design questions in the root brief are resolved with the user
  (anchor/header model, fate of `--repo` and `current_repo`, empty-fleet UX),
  and recorded inline in `CONTEXT.md` / a small ADR where they're genuine
  decisions.
- The tree is grown: `dashboard_surface`/`App` rework, repo-resolution change,
  and test/doc updates are decomposed into ordered work leaves (lazily — only as
  far as understanding supports).

## Notes

Start with a grilling session (one question at a time, recommended answers).
First question to put to the user: should *all* cwd logic go (manifest + `--repo`
only), or should the cwd git root still be auto-added to the fleet as a
convenience when present? The user's stated lean is "just use the config."
