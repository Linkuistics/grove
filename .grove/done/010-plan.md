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

## Decisions (running log)

**Q1 — cwd fate: remove all cwd logic (settled 2026-06-05).** The fleet is
resolved from the manifest (`repos` + `scan_roots`) plus additive `--repo` flags
*only*. No code path detects or depends on the cwd git root: the
`repo::resolve()` gate goes, `Discovery.current_repo` / `Fleet.current_repo` go,
the `current_grove_name` cwd preselect goes. Decisive extra evidence beyond the
user directive: `zellij::session_name(&repo)` derives the zellij session from the
cwd repo basename, so cwd-anchoring spawns a *different* session per launch dir —
a fleet tool wants **one** canonical persistent session, which only a
cwd-independent (constant) session name gives. `--repo` survives as an additive
config input (ADR-0025 §2). Amends ADR-0025 §3 (current-repo-always-included) —
a recorded decision (see ADR below). Costs accepted: no zero-config
"in-a-repo → see its groves" (use `fleet.toml` or `grove tui --repo .`), no
cwd-grove preselect.

**Q2 — session name: singleton `grove-fleet` (settled 2026-06-05).** The fleet
TUI is a singleton zellij session named `grove-fleet`; a second `grove tui`
attaches to the same session rather than spawning a parallel one. `session_name`
stops taking a repo and returns the constant. This is the positive form of the
Q1 consequence (cwd-derived session names fragment the fleet into one session per
launch dir).

**Q4 — header: cli version + fleet counts (settled 2026-06-05).** The header line
shows the running binary's methodology version (`status::CLI_VERSION`, the one
global fact) **plus** a `N repos · M groves` fleet summary. Per-repo version
drift stays per-row in the nav. `primary_view` (own-repo header source) is no
longer needed for the fleet nav — the header derives from the fleet itself.

**Q6 — empty fleet: launch TUI with in-nav empty state (settled 2026-06-05).**
When the resolved fleet is empty the TUI still launches; the nav renders a
helpful empty-state panel pointing at `~/.config/grove/fleet.toml` and `--repo`.
No pre-launch precondition branch is reintroduced — "always launch, always
surface the fleet" is preserved (the gate-shaped pattern this grove removes is
not re-created in a new form).

**Q3 — de-anchor `App` via `Option` (settled 2026-06-05, recommended; open to
pushback).** `App.repo: PathBuf` becomes `App.repo: Option<PathBuf>`; the fleet
nav passes `None`, while `new` (N=1 tests) and `new_detail` (per-grove detail,
which is correctly repo-explicit) pass `Some`. `primary_view`/preselect handle
`None`. Chosen over splitting `App` into separate fleet/detail types — that is a
large refactor of a ~4000-line struct, unjustified now (constraint 4, lazy).
`DashboardSurface.repo: PathBuf` likewise becomes `repo_flags: Vec<PathBuf>` (the
`--repo` flags it was built from) so `set_driver`'s fs-watch re-resolves the same
fleet without a cwd anchor. `App::new`/`new_detail` are **kept** (brief's open
question): `new_detail` backs every per-grove detail surface (repo-explicit, not
a cwd anchor) and `new` backs the N=1 unit tests.
