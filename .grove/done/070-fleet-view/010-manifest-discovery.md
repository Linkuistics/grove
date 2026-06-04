# 010-manifest-discovery

**Kind:** work

## Goal

Build the fleet **repo-discovery** layer (ADR-0025): resolve the set of repo roots a
fleet grove process spans, from a manifest file + scan roots + flags + the current
repo. Produces a plain `Vec<repo root>` (canonical, deduped) — **no `RepoView`, no
`ratatui`** (below the presentation boundary, ADR-0013). Consumed by `020`.

## Context

- Today: `repo::resolve(arg: Option<&Path>)` (`src/repo.rs:7`) → one repo (the `--repo`
  flag or cwd's git root). This leaf adds the *multi*-repo resolver beside it; the
  single-repo path stays as the N=1 case.
- ADR-0025 binds the **model**, this leaf settles the **serialization**:
  - Manifest path: XDG (`~/.config/grove/fleet.toml` or `$XDG_CONFIG_HOME/grove/`).
  - Format: TOML with `repos = [..]` (explicit, always included) and
    `scan_roots = [..]` (walked to find dirs containing `.grove-worktrees/`).
  - `--repo <path>` flags layer additively (repeatable) — extend `TuiArgs`
    (`src/cli.rs:49`), today a single `repo: Option<PathBuf>`.
  - Current repo (cwd git root, if any) **always included**.
  - **Dedup by canonical path** across all routes.
- Scan match target: a directory is a fleet repo if it contains `.grove-worktrees/`
  (the signal `RepoView::scan` already keys on). Bound scan cost to named roots only.
- Failure handling (070 Q3 / ADR-0025): a repo that fails to resolve is **silently
  skipped**; an *explicitly-listed* `repos` entry that fails emits one stderr
  breadcrumb (never blocks, never shown in UI).

## Done when

- A resolver (e.g. `fleet::resolve(args) -> Vec<PathBuf>`) returns the canonical,
  deduped repo list from manifest + scan_roots + `--repo` flags + current repo.
- No manifest + no flags ⇒ `[current repo]` (N=1 preserved, zero config).
- Missing/`scan_root`-empty repos skipped silently; explicit-missing logs one stderr line.
- Unit tests cover: explicit-only, scan-only, both-with-dedup, current-repo-always,
  explicit-missing-breadcrumb, empty everything.
- No `ratatui` import in the discovery module.

## Notes

No `grove fleet add` management verb in this grove — the manifest is hand-edited
(ADR-0025 Consequences). Document the file format wherever the TUI/config is
documented. First leaf; everything downstream consumes its `Vec<repo root>`.
