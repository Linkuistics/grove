# 070-fleet-view — brief

**Kind:** node brief — grilling complete (Q1–Q7 below); decomposed into the impl
leaves mapped under **Decomposition**.

## Goal

Extend the single-repo reader into a multi-repo **fleet view** (concern 1): one
process surfacing groves across many repos, filterable by repo / workstream /
inbox-pending count. Architecturally additive — `RepoView` → `MultiRepoView` —
per the v1 brief's deliberate factoring.

## Context

- v1 data layer: `src/repo_view.rs` — `RepoView::scan(repo_root)`,
  `GroveSummary`, `GroveDetail`. The seam to generalise is the single
  `repo_root`. Aim for `MultiRepoView` as a collection of `RepoView`s rather
  than a rewrite.
- **Repo discovery** is an open design question for this leaf's grilling: how
  does grove learn which repos to span (config file? a registry on the
  `grove-meta` branch? a scan root? explicit `grove tui --repo` flags)?
- **fs-watch `.git/` noise (OBS 2) folds in here.** Watching N repos amplifies
  the `.git/` event noise N-fold. Land the two cheap wins as part of this leaf:
  (a) path-filter `notify` events whose path contains `/.git/`; (b) optionally
  watch `.grove-worktrees/<name>/.grove/` per-grove instead of the worktrees
  root recursively. See root BRIEF "fs-watch .git/ noise" note for detail.

## Done when

- The TUI lists groves spanning multiple repos with repo attribution.
- Filtering by repo / workstream / inbox-pending works.
- `notify` no longer marks dirty on `.git/`-internal churn (path filter landed).

## Decomposition

Grilling (Q1–Q7, running log below) settled the design; the work splits into six
leaves with a clean dependency chain `010 → 020 → {030, 040} → 050 → 060`:

```
010-manifest-discovery   repo discovery: manifest + scan_roots + flags → Vec<repo root>   (ADR-0025)
020-multi-repo-view      MultiRepoView wraps N RepoViews, concurrent scan, grouped; N=1 case
030-fleet-fs-watch       .git/ path filter + one multi-root watcher + targeted per-repo re-scan
040-grouped-nav          subsumed grouped two-level nav: repo sections → groves, sort, collapse, N=1 hide
050-cross-repo-harness   repo-qualify name-keyed workspace/tab maps; verify cross-repo open
060-fleet-filtering      (deferred from MVP) interactive filter — fuzzy + inbox-pending + sort toggle
```

All six are below or at the presentation boundary per the decisions below; the data
layer (010/020) carries no `ratatui`. `060` was deferred from the first cut at Q5 but
stays in-grove to satisfy the root brief's "filterable by …" done-criterion.

## Notes

Sequenced after the harness pane (060). **The harness substrate is now the trellis
fork (ADR-0015/0020), not the `harness-pane` crate** — the original note below was
written pre-fork. The fleet list is a data-layer + nav concern and is unaffected by
the substrate choice; selecting any grove in any repo opens its working set (harness
+ detail + aux tools) the same way it does for the current repo today — the
harness-driving path already "carries the repo explicitly so the cross-repo fleet
reuses this path unchanged" (`src/tui.rs:1129`).

## Decisions (running log)

### Q1 — Repo discovery: **manifest + scan hybrid** (→ ADR-0025)

A fleet manifest config file lists explicit `repos` (always included) *and* optional
`scan_roots` (walked to discover repos containing a `.grove-worktrees/`). Explicit
wins; scan fills in. The current repo (cwd's git root) is always included even if not
listed. `--repo` flags layer on additively.

- *Rejected* — **registry on `grove-meta`**: wrong scope; `grove-meta` is per-repo, so
  there is no single repo whose branch could host the cross-repo registry (chicken-egg).
- *Rejected as sole mechanism* — **`--repo` flags only**: ephemeral, re-typed every launch.
- *Folded in* — **scan root** and **manifest** are not exclusive; the hybrid is their union,
  which keeps zero-config-via-scan available without losing pinned explicit repos.

Open sub-details deferred to decomposition: exact config path/format (lean XDG +
TOML), what the scanner matches on (`.grove-worktrees/` presence vs `.git`), and
dedup when a repo is found by both `repos` and a `scan_root`.

### Q2 — Fleet list model: **grouped by repo (two-level, collapsible)**

Repos are collapsible sections; groves nest under their repo. Repo attribution is
*structural* (the section header), not a per-row badge. This **mirrors the data
structure** — `MultiRepoView` is a collection of `RepoView`s, so the two-level UI
renders the data's own shape directly rather than flattening + stamping repo onto
each row. The native nav becomes a two-level tree (repo header → groves).

- *Rejected* — **flat list + repo attribute**: would require flattening N RepoViews
  and carrying repo on every row; grouping is the more natural fit for the collection
  shape. (Repo as a *filter* dimension still applies on top of the grouping — Q4.)
- *Rejected* — **repo picker → per-repo**: barely "fleet"; loses all-at-a-glance.

### Q3 — Unreadable / missing repos: **silently skip** (in the UI)

A configured repo that is missing, not a git repo, or has no `.grove-worktrees/` is
simply absent from the fleet list — no error section, no startup failure. `scan()`
runs per repo concurrently and one failure never blocks the others.

- *Rejected* — **flag dim section**: rejected for UI noise; the clean list wins.
- *Rejected* — **hard-fail startup**: punishes the normal repo-comes-and-goes lifecycle.
- *Refinement (impl)*: distinguish the two failure causes. A `scan_root` that finds
  nothing is *expected* and stays silent. An **explicitly-listed `repos` entry** that
  fails to resolve is possible config drift — keep it out of the UI per the decision,
  but emit a single recoverable stderr line so a vanished pinned repo leaves a
  breadcrumb. Never blocks; never shown in the list.

### Q4 — Fleet nav vs single-repo nav: **subsume (one render path)**

`MultiRepoView` becomes the *single* data source the nav renders; the single-repo
case is just `MultiRepoView` over a one-element repo list (a fleet of one). The nav
always renders the grouped two-level model; **when N=1 the lone repo's section header
auto-hides** so today's single-repo users see no added chrome. One code path — no
permanent fork between single- and multi-repo rendering.

- *Rejected* — **parallel fleet mode**: forks the nav into two surfaces that drift;
  every future nav change would have to touch both. Subsuming is the additive-not-
  rewrite spirit the root brief mandates ("`MultiRepoView` is additive, not a rewrite").
- *Consequence*: the controller's grove-feed to the nav must carry repo grouping (today
  it pipes `name → pending`); single-repo callers construct a one-element `MultiRepoView`.

### Q5 — Filtering: **defer; ship grouping + sort only for v1**

Filtering (fuzzy match / inbox-pending toggle) is **cut from this leaf's MVP**. v1 ships
the two-level grouping plus a sort order; collapsible repo sections cover the "focus on
one repo" need without a filter. Real filtering becomes its **own follow-up leaf**
(`060-fleet-filtering`, last in this node), keeping the root brief's "filterable by …"
promise without bloating the first cut.

- *Rejected for now* — **fuzzy box + toggles** / **per-dimension controls**: both are
  more surface than the first multi-repo cut needs; deferred, not abandoned.
- **Sort default** (shipped here): repo sections ordered current-repo-first, then explicit
  `repos` in manifest order, then scanned repos alphabetically; groves *within* a repo keep
  the existing `RepoView` order (lifecycle then numeric prefix). A sort *toggle*
  (e.g. inbox-desc) is part of the deferred filtering leaf, not v1.
- Collapse/expand of repo sections is ephemeral UI state (not persisted) — consistent
  with constraint 1.

### Q6 — fs-watch at fleet scale: **targeted per-repo re-scan, one watcher**

A single `notify` watcher watches every repo's `.grove-worktrees/` + `.grove-meta/inboxes/`
roots. On a (debounced, `.git/`-filtered) event, **prefix-match the event path against the
known repo roots** to identify the owning repo and re-scan **only that** `RepoView`. The
other repos' views are untouched.

- **`.git/` path filter lands regardless** (both briefs' OBS-2 cheap win): drop any event
  whose path contains a `/.git/` component before it can mark dirty. Removes the pack/ref/
  index churn that the 200ms debounce currently only masks — and which fleet-scale watching
  would amplify N-fold.
- *Rejected* — **whole-fleet re-scan**: cost scales with fleet size on every keystroke-driven
  `.grove/` write; the prefix-match that avoids it is nearly free.
- *Rejected* — **per-repo watchers**: N watcher threads/handles for no gain over one
  multi-root watcher.
- The per-grove watch refinement (watch `.grove-worktrees/<name>/.grove/` instead of the
  worktrees root recursively) stays *optional* — the `.git/` filter already removes the noise
  it targeted; revisit only if event volume proves it necessary.

### Q7 — Cross-repo harness/working-set open (design note, not a gated decision)

Opening a grove that lives in another repo reuses the **existing** harness-driving path,
which already "carries the repo explicitly so the cross-repo fleet reuses this path
unchanged" (`src/tui.rs:1129`, `278`). cwd for `grove do <name>` is taken from the grove's
*owning* repo, not the process cwd — already carried, not re-derived.

**One genuine wrinkle to handle in impl:** harness tabs are keyed by bare grove *name*
today (`open_harnesses: BTreeSet<String>`, `src/tui.rs:3505` — "the native, name-keyed
analogue"; same for `mounted_grove: Option<String>`). Across repos two groves can share a
name (`grove/fix-bug` and `acme-api/fix-bug`), so the workspace/tab key must become
**repo-qualified** — a `(repo, name)` pair or a derived `<repo>:<name>` string — to avoid
collision and mis-focus. Lands in leaf `050-cross-repo-harness`.
