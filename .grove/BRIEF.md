# fleet-only-tui — brief

## Goal

Make `grove tui` (the trellis dashboard) driven **entirely by the fleet config**
— `fleet.toml` (`repos` + `scan_roots`) plus additive `--repo` flags — and
**eliminate all cwd git-repo checking and single-repo / "local" anchoring**.

User directive (2026-06-05): *"We should eliminate all cwd checking or
local/single git repo code, and just use the config."*

Today `grove tui` hard-requires the cwd to be a git repo and fails outside one
(`Error: not in a git repo (cwd: …)`) even when `fleet.toml` lists repos — so the
fleet view is unreachable unless you happen to be standing in a git repo. This is
a direct follow-up to the just-finished `grove-always-starts-in-local-mode` grove
(which made trellis unconditional so `fleet.toml` is *surfaced*); this grove
removes the remaining gate that still blocks fleet-only use.

## Done when

- `grove tui` run from **any** directory (incl. a non-git dir) surfaces the fleet
  resolved from `fleet.toml` (+ `--repo`), with no cwd git-repo requirement.
- No code path calls `repo::resolve(None)` / `git_common_dir(cwd)` to *gate* the
  TUI, and there is no single-repo "current repo" anchor that the dashboard
  depends on.
- A sensible, helpful state when the resolved fleet is **empty** (no manifest, no
  scan hits, no `--repo`) — decided during planning (likely a clear message, not
  a git-repo error).
- `cargo test` passes; tests that construct the dashboard around a mandatory
  `repo: PathBuf` are updated to the fleet-only model.

## Decomposition

To be grown by `010-plan` (grilling). Likely shape: a design/grilling increment
that resolves the open questions below, then work leaf(s) to (a) make repo
resolution non-gating / optional, (b) rework `dashboard_surface`/`App` off the
single-repo anchor, (c) update tests + docs/CHANGELOG/glossary.

## Pointers (from the diagnosing session — verify before relying on)

- Hard gate: `src/trellis_host.rs` `run_client` (~:159) `repo::resolve(args.repo.as_deref())?`
  and `run_server` (~:117-120) `GROVE_REPO` else `repo::resolve(None)?`.
- Error source: `src/repo.rs:37,51` `bail!("not in a git repo (cwd: …)")`.
- Single-repo anchor the dashboard is built around:
  `src/tui.rs` `dashboard_surface(repo: PathBuf)` (~:4133) → `fleet::resolve(&[repo])`
  → `App::new_fleet(repo, fleet, preselect)`; `App.repo: PathBuf` used in
  `primary_view`/header version source (~:602,1667,1714,1727), grove lookup
  (~:612), preselect (`current_grove_name`), and fs-watch in `set_driver`
  (~:4465 `fleet::resolve(&[self.repo])`). Also `detail_surface(repo, grove)`.
- The fleet layer **already** models the absence of a cwd repo:
  `src/fleet.rs` `Fleet.current_repo: Option<PathBuf>` ("the cwd's git root, *if
  any*", ADR-0025 §3); `resolve_with_warnings` handles `None`. The free
  `fleet::resolve(repo_flags: &[PathBuf])` reads the manifest + scans regardless.
- Fleet discovery decision: `docs/adr/0025-fleet-repo-discovery-is-manifest-plus-scan.md`.

## Notes

Diagnosed via systematic-debugging and reproduced: with
`~/.config/grove/fleet.toml` = `scan_roots = ["/Users/antony/Development"]`,
`grove tui` from `/tmp/<non-git>` errors `not in a git repo` — the gate fires
before the fleet (which the scan_root would fully populate) is ever built.

Open design questions for `010-plan` to grill:
- Does `--repo` survive as an additive fleet input (ADR-0025 keeps it), and does
  the cwd git root still get auto-added as a convenience, or is *all* cwd logic
  removed (manifest + `--repo` only)? User leans "just use the config."
- Fate of `Fleet.current_repo` and the `current_grove_name` preselect when there
  is no anchor repo.
- The header "version source" (`primary_view`) currently = the surface's own
  repo's `RepoView`. What does the header show with no primary repo? (pick a
  fleet member? drop the version line? per-repo in the nav?)
- fs-watch roots without a `self.repo` anchor (watch all fleet roots).
- Empty-fleet UX.
- Whether `App::new`/`new_detail` single-repo constructors are still needed.
