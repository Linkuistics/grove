# 040-grouped-nav

**Kind:** work

## Goal

Render the **grouped two-level** fleet in the native nav (070 Q2 + Q4): repo sections
→ groves, collapsible, fed from `MultiRepoView`. The nav **subsumes** the single-repo
nav — one render path; the lone repo's header **auto-hides when N=1** so today's
single-repo users see no added chrome.

## Context

- The nav is the native leader-focused command surface (ADR-0020/0021). It currently
  receives a flat grove list from the controller (the `grove-state` feed, today
  `name → pending`) and renders one repo's groves.
- 070 Q4 consequence: the controller's grove-feed must carry **repo grouping**;
  single-repo callers construct a one-element `MultiRepoView` and feed that.
- Rendering: repo section header (path/basename + grove count) → indented grove rows.
  **Collapse/expand per repo section** — ephemeral UI state, not persisted (070 Q5,
  constraint 1). **N=1 ⇒ hide the section header**, render groves flat as today.
- **Sort default** (070 Q5): repo sections current-repo-first → explicit `repos`
  (manifest order) → scanned (alphabetical); groves within keep `RepoView` order.
  No sort *toggle* here (that's `060`).
- **No filtering here** — deferred to `060` (070 Q5). This leaf is grouping + sort +
  collapse only.
- Selecting a grove must carry its **repo** (for `050`'s cross-repo open) — the nav
  row already needs the grove's `grove do` command + cwd; extend it with repo identity.

## Done when

- Nav renders `MultiRepoView` as collapsible repo sections → groves, in the defined
  sort order.
- N=1 hides the repo header and matches today's single-repo nav visually.
- Collapse/expand works per section; state resets per session (not persisted).
- A grove row carries enough to open it cross-repo (repo + name + cwd + command).
- fs-watch-driven updates (`030`) re-render the affected section.

## Notes

Depends on `020` (data) and `030` (live updates). Precedes `050` (which consumes the
repo-carrying selection). Confirm whether the controller→nav feed is a pipe/string
(WASM-era) or a native call post-ADR-0020 and extend accordingly — check the current
nav feed path before changing the wire shape.

**Targeted re-scan seam left ready by `030`.** `030` landed the `.git/` filter (live in
both watch paths) and made the native dashboard's watcher span **every** fleet repo's two
roots (`fleet::resolve` + `fleet::fleet_watch_dirs` in `DashboardSurface::set_driver`). But
the `App` still holds a single `RepoView`, so `DashboardSurface::tick` re-scans only this
surface's repo today — a no-op when an *unrelated* fleet repo changes. When this leaf swaps
the `App`'s data source to `MultiRepoView`, route the tick through the **already built +
tested** `MultiRepoView::rescan_for_event_path(event_path)` (it prefix-matches the owning
repo via `fleet::owning_repo` and re-scans only that `RepoView`). That needs the watch to
carry the *dirty event paths* to `tick` — today `spawn_grove_watch` only signals "something
settled" via `request_tick` with no payload — so thread a shared dirty-path buffer (or
extend the tick payload) from the watch thread into `tick`, then call
`rescan_for_event_path` per path so an event under repo A leaves repo B's section untouched.

**Document the fleet manifest in the README here.** Leaf `010` built the
`fleet::resolve` discovery layer and the `fleet.toml` format (`repos` +
`scan_roots` at `~/.config/grove/fleet.toml`; see `src/fleet.rs` module docs),
but deferred the *user-facing* README doc because no running command consumed it
yet (`grove tui` still resolves a single repo until `020`/this leaf wire it).
When the fleet renders here, add the manifest format + `--repo` flag to the
`grove tui` section of `README.md` (010 task note: "document the file format
wherever the TUI/config is documented").
