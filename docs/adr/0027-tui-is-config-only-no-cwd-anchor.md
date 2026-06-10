# 27. The TUI is config-only — the cwd git-repo anchor is removed

- Status: accepted — **survives; realisation amended** by
  [ADR-0028](0028-rmux-substrate.md) / [ADR-0030](0030-grove-bundles-a-from-source-stock-rmux-daemon.md) §4
  (rmux substrate, 2026-06-10). The config-only, no-cwd-anchor model is **untouched**;
  the singleton fleet session it set (`grove-fleet`, §4 below) is now realised as a
  persistent **rmux** session (`CreateOrReuse`) rather than a zellij one. Not superseded.
- Date: 2026-06-05
- Deciders: Antony Blakey (grove `fleet-only-tui` 010-plan grilling)
- Amends: ADR-0025 §3 (current repo always included) — removed; ADR-0026
  (trellis is the only TUI) — this is the follow-up that makes that always-on
  trellis reachable from anywhere.

## Context

ADR-0025 made fleet membership a manifest (`repos` + `scan_roots`) plus additive
`--repo` flags, and §3 *always included the cwd's git root* as a convenience so
`grove tui` from inside a repo never lost that repo. ADR-0026 made trellis the
unconditional TUI so `fleet.toml` is surfaced.

But a gate survived both: `trellis_host::run_client` (and the re-exec'd
`run_server`) call `repo::resolve(None)` **before** the fleet is built, which
`bail!`s `not in a git repo (cwd: …)` outside a git repo. With
`scan_roots = ["…/Development"]` in `fleet.toml`, `grove tui` from a non-git
`/tmp` dir errored even though the scan root would have fully populated the
fleet — the gate fired before the fleet existed. The fleet view was unreachable
unless you happened to be standing in a git repo. Separately, the home dashboard
was built around a mandatory single `repo: PathBuf` anchor (`dashboard_surface`
→ `App.repo`) — the single-repo-era leftover.

User directive (2026-06-05): *"We should eliminate all cwd checking or
local/single git repo code, and just use the config."*

This is a user-facing contract change (where the fleet comes from), hard to
reverse once people rely on it, and a genuine trade-off (a zero-config
convenience vs. one coherent config-driven model) — so it earns a recorded
decision.

## Decision

**The TUI is resolved purely from config; no cwd git root is detected or used.**

1. **Remove the gate.** Neither the client nor the server path calls
   `repo::resolve()` to gate the TUI. `run_client`/`run_server` resolve the
   **fleet** directly (`fleet::resolve(&repo_flags)`) and launch regardless of
   the cwd.
2. **Remove the cwd fleet input (amends ADR-0025 §3).** `Discovery.current_repo`
   and `Fleet.current_repo` are removed; nothing auto-includes the cwd's git
   root. The fleet is `manifest.repos` + `scan_roots` discoveries + `--repo`
   flags, deduped. `--repo` survives unchanged as the additive, explicit input
   (ADR-0025 §2) — including `grove tui --repo .` to pin the current directory
   deliberately.
3. **De-anchor the dashboard.** `App.repo` becomes `Option<PathBuf>` (the fleet
   nav passes `None`; the N=1 `App::new` and the per-grove `App::new_detail`
   pass `Some` — the per-grove detail surface is correctly repo-explicit, not a
   cwd anchor). `DashboardSurface` stores the `--repo` flags it was built from
   (not a single `repo`) so `set_driver`'s fleet fs-watch re-resolves the same
   repo set without a cwd anchor.
4. **The fleet TUI is a singleton session.** With no cwd anchor,
   `zellij::session_name` is a constant (`grove-fleet`) rather than derived from
   the cwd repo basename. A second `grove tui` attaches to the same session
   instead of fragmenting the fleet into one session per launch directory. The
   client→server handoff drops `GROVE_REPO` (a single anchor) in favour of
   passing the `--repo` flags; the cwd is inherited by the re-exec so scan-root
   resolution is identical on both sides.
5. **An empty fleet launches the TUI.** When the resolved fleet is empty (no
   manifest, no scan hits, no `--repo`) the TUI still launches and the nav
   renders an in-nav empty-state pointing at `~/.config/grove/fleet.toml` and
   `--repo`. No pre-launch precondition branch is reintroduced — the
   gate-shaped pattern this ADR removes is not re-created in a new form.
6. **The header derives from the fleet.** With no anchor repo the header shows
   the running binary's methodology version (`status::CLI_VERSION`) plus a
   `N repos · M groves` fleet summary; per-repo version drift stays per-row in
   the nav. `primary_view` (the own-repo header source) is no longer needed for
   the fleet nav.

## Consequences

- **`grove tui` runs from anywhere.** From a non-git `/tmp`, from `$HOME`, from
  inside a repo — all surface the same config-resolved fleet. The "not in a git
  repo" error is gone for the TUI path.
- **Zero-config "in a repo → see its groves" is gone.** Standing in a repo with
  no manifest now shows the empty-state, not that repo's groves. The deliberate
  replacements are a one-line `fleet.toml` (`repos = ["."]` or a `scan_roots`
  entry) or `grove tui --repo .`. Accepted as the cost of one coherent model.
- **The cwd-grove preselect is gone.** Launching from inside a grove worktree no
  longer auto-highlights that grove; the nav starts at row 0. Minor.
- **One persistent fleet session.** `grove tui` is now effectively a singleton
  app — reattaching the `grove-fleet` session — which matches a multi-repo
  tool's mental model better than per-cwd sessions did.
- **`repo::resolve` / `git_toplevel` survive for non-TUI callers.** The worktree
  lifecycle (`grove do`, retire) still resolves the repo from cwd; only the TUI
  stops gating on it. This ADR scopes to the TUI path.
- **`App::new` / `App::new_detail` are kept.** The brief asked whether the
  single-repo constructors were still needed; they are — `new_detail` backs
  every per-grove detail surface and `new` backs the N=1 unit tests. What is
  removed is the *mandatory cwd anchor*, not single-repo construction.
