# 020-config-only-tui

**Kind:** work

## Goal

Make `grove tui` resolved **purely from config** (manifest + `scan_roots` +
`--repo` flags), with no cwd git-repo gate and no single-repo anchor. Implements
ADR-0027. One cohesive change that compiles + `cargo test` passes as a unit
(the resolution-layer and dashboard-layer edits must land together — removing
the gate while `dashboard_surface` still demands a `repo: PathBuf` would not
compile).

## Context

Decisions are in ADR-0027 and the `010-plan` running log. Verified pointers
(line numbers approximate — re-grep before editing):

**Gate removal + fleet-direct resolution + client/server handoff** —
`src/trellis_host.rs`:
- `run_client` (~:158) `let repo = crate::repo::resolve(args.repo.as_deref())?;`
  — the gate. Replace with `let roots = crate::fleet::resolve(&args.repo …);`
  (gather `--repo` flags from `RepoArgs`) and build the surface from `roots`.
- `run_server` (~:117-120) `GROVE_REPO` else `repo::resolve(None)?` → resolve the
  fleet on the server side too. The re-exec inherits cwd + env; pass the `--repo`
  flags across (replace the single `GROVE_REPO` with the flag list, e.g.
  `GROVE_REPO_FLAGS` newline-joined, or reuse the existing arg plumbing).
- `dashboard_surface(repo)` call site (~:129) → pass the resolved roots / flags.
- `session_name(&repo)` call (~:166) → constant `grove-fleet` (see zellij.rs).
- `std::env::set_var("GROVE_REPO", &repo)` (~:164) → flag-list handoff.

**Fleet layer cwd removal** — `src/fleet.rs`:
- Remove `Discovery.current_repo` (~:70) and its resolution step in
  `resolve_with_warnings` (~:102-108). Update `resolve(repo_flags)` (~:165) to
  drop `let current_repo = crate::repo::resolve(None).ok();` (~:174).
- Update the unit tests that set `current_repo:` (several in the `tests` mod) —
  drop the field; the `current_repo_*` tests either delete or re-express via
  `--repo` flags.

**Session name** — `src/zellij.rs`:
- `session_name(repo: &Path) -> String` (~:26) → no arg, returns `"grove-fleet"`.
  Update the `session_name_sanitises_and_namespaces` test (~:49) accordingly
  (or replace with a trivial constant assertion).

**Dashboard de-anchor** — `src/tui.rs`:
- `dashboard_surface(repo: PathBuf)` (~:4133) → take the resolved roots (or
  `--repo` flags) instead of a single anchor; `App::new_fleet` with `repo: None`.
- `App.repo: PathBuf` (~:62) → `Option<PathBuf>`. Fix `primary_view` (~:602),
  `new_fleet` preselect match `*repo == app.repo.as_path()` (~:464),
  `detail_grove` (~:612, used by detail surface — keep working via `Some`).
- `App::new` (~:427) / `App::new_detail` (~:478) — **keep**; they pass `Some(repo)`.
  Only the fleet nav passes `None`.
- Header (`render_header` ~:1663): show `cli` version **+ `N repos · M groves`**
  fleet summary (Q4). Derive counts from `app.fleet`.
- Empty-fleet in-nav empty state (Q6): when `app.fleet.repos()` is empty,
  `render_grove_list` (~:1701) shows a helpful panel naming
  `~/.config/grove/fleet.toml` and `--repo`.
- `DashboardSurface.repo: PathBuf` (~:4173) → `repo_flags: Vec<PathBuf>` (the
  `--repo` flags it was built from). `set_driver` (~:4465)
  `fleet::resolve(std::slice::from_ref(&self.repo))` → `fleet::resolve(&self.repo_flags)`.
  `current_grove_name` preselect (~:4145) is removed (cwd gone).

## Done when

- `grove tui` from a non-git dir (e.g. `/tmp/x`) with a `fleet.toml` `scan_roots`
  surfaces the fleet — no "not in a git repo" error. (Reproduce per root BRIEF.)
- No TUI code path calls `repo::resolve(None)` / detects the cwd git root to gate
  or anchor the dashboard. `repo::resolve`/`git_toplevel` survive for non-TUI
  callers (`grove do`/retire) untouched.
- Empty fleet launches the TUI with the in-nav empty-state (no pre-launch exit).
- `cargo build` and `cargo test` pass; tests constructing `App`/`dashboard_surface`
  around a mandatory `repo` are updated to the `Option`/flags model.

## Notes

If this proves too big for one session, the natural sub-split is
resolution-layer (fleet.rs + trellis_host.rs + zellij.rs) then dashboard-layer
(tui.rs) — but they must end on a compiling, test-green commit together, so
prefer one task unless it genuinely overruns.
