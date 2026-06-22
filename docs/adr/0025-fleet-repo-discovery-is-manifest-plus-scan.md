# 25. Fleet repo discovery is a manifest file plus optional scan roots

- Status: **superseded** (was accepted) — Superseded by
  [ADR-0031](0031-shed-machinery-keep-self-extension-core-and-methodology.md) (grove
  sheds its machinery to a self-extension core) and
  [ADR-0032](0032-loop-substrate-is-a-self-driving-shell-loop-not-archon.md) (the loop
  substrate is a self-driving shell loop). The rmux/ratatui TUI + Fleet tower this ADR
  belongs to is **deleted** in leaf `080-shed-tui`; its runtime lives only in git
  history. The decision is retained here as record. (Prior status: accepted —
  **survived untouched** under the rmux substrate,
  [ADR-0028](0028-rmux-substrate.md) / [ADR-0030](0030-grove-bundles-a-from-source-stock-rmux-daemon.md) §4:
  fleet repo discovery lived **below** the presentation boundary, feeding
  `MultiRepoView`, so the substrate swap did not touch it.)
- Date: 2026-06-04
- Deciders: Antony Blakey (with grove 070 fleet-view planning)
- Builds on: ADR-0013 (presentation boundary — discovery lives below it, feeding
  the `MultiRepoView` data layer), the root brief's "v1's data layer is factored so
  a future `MultiRepoView` is additive, not a rewrite".

## Context

The fleet view (concern 1) is one grove process surfacing groves across **many**
repos. Today the TUI resolves exactly one repo — `--repo <path>` or the cwd's git
root (`src/repo.rs`) — and `RepoView::scan(repo_root)` scans that one. The open
question the 070 brief flags: **how does one grove process learn which repos to
span?** Four candidates were named: explicit `--repo` flags, a config file, a
registry on the `grove-meta` branch, or a scan root.

This is a user-facing contract (whatever we choose, users configure their fleet
through it) and is hard to reverse once people have written config against it — so
it earns a recorded decision.

## Evidence

- **`grove-meta` is per-repo, not global.** It is "a dedicated branch in *each*
  repo" (CONTEXT.md, ADR-0002). A cross-repo registry needs a single source of
  truth spanning repos; `grove-meta` has no such cross-repo instance. Putting the
  fleet list on one repo's `grove-meta` creates a chicken-and-egg — *which* repo's
  branch holds it, and how does grove find that repo before it has the list? It
  also couples the fleet to that one repo's lifecycle.
- **grove's spine prefers legible, walk-away-able artifacts** (constraints 1, 6). A
  plain config file listing repo roots is a standard, team-shareable, hand-editable
  artifact that stays meaningful with grove uninstalled. `--repo` flags persist
  nothing (re-typed every launch); a pure scan root persists nothing user-authored
  and walks the filesystem on every start.
- **A scan root is genuinely useful but not sufficient alone.** Auto-discovering
  every `.grove-worktrees/`-bearing repo under `~/work` is convenient, but it can
  miss repos outside the root and surface ones you didn't mean to include. Explicit
  pinning and scan-discovery are complementary, not exclusive — their union is
  strictly more expressive than either.

## Decision

**Fleet membership is a manifest config file with two keys plus additive flags:**

1. A **manifest file** at an XDG path (e.g. `~/.config/grove/fleet.toml`; exact
   path/format settled in the impl leaf) with:
   - `repos` — explicit repo roots, **always included** (config drift surfaced only
     as a recoverable stderr line, never blocking — see ADR-0003 below in
     Consequences and the 070 brief's Q3).
   - `scan_roots` — directories grove walks to discover repos containing a
     `.grove-worktrees/`. Discovered repos fill in around the explicit ones.
2. **`--repo <path>` flags layer additively** on top of the manifest (repeatable).
3. **The current repo** (cwd's git root, if any) is **always included**, so
   `grove tui` from inside a repo never *loses* that repo by virtue of fleet config
   existing.
4. **Dedup by canonical repo path** when a repo is reached by more than one route
   (`repos` + a `scan_root`, or a `--repo` flag that duplicates a manifest entry).

Explicit sources (`repos`, `--repo`, current repo) win; `scan_roots` fill in the
rest. A repo that fails to resolve is **silently skipped in the UI** (070 Q3); an
*explicitly-listed* missing repo additionally emits one stderr breadcrumb.

*Rejected* — **registry on `grove-meta`**: wrong scope (per-repo branch, no global
instance; chicken-and-egg on which repo hosts it). *Rejected as sole mechanism* —
**`--repo` flags only** (ephemeral) and **scan root only** (no user-authored pin,
walks every launch, can't reach repos outside the root). The chosen design is the
union of manifest + scan, which subsumes both without their individual gaps.

## Consequences

- **Single-repo stays the N=1 case.** `MultiRepoView` is built over the resolved
  repo list; with no manifest and no flags it is just `[current repo]`, so existing
  single-repo behaviour is preserved with zero config (ties to 070 Q4 "subsume").
- **A new user-facing config artifact exists.** It is documented, hand-editable, and
  walk-away-legible (constraint 6). A management verb (`grove fleet add …`) is *not*
  built now — the file is hand-edited; a verb is a lazy future add (constraint 4).
- **Discovery is below the presentation boundary** (ADR-0013): it produces a plain
  `Vec<repo root>` with no `ratatui` dependency, consumed by `MultiRepoView`. A
  future web front-end reuses the same discovery.
- **Scan cost is bounded by `scan_roots`, not the whole filesystem** — grove only
  walks the roots the user named, so the "slow/noisy scan" failure mode of an
  unbounded scan-root design does not apply.
- **Exact path/format are deferred to the impl leaf** (`070/010-manifest-discovery`).
  This ADR binds the *model* (manifest + scan-roots + always-include-current +
  additive flags + dedup), not the serialization details.
