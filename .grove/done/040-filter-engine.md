# 040-filter-engine

**Kind:** work

## Goal

The pure filter/sort engine behind the [[filter mode]]: a criteria struct
(fuzzy needle, inbox-pending toggle, lifecycle cycle, sort order) and a
projection from the fleet to a **flat ranked list** when any dimension is
engaged.

## Context

- Dimensions and sort cycle fixed by 010-plan Q3/Q4: needle over
  `<repo>/<grove>`; inbox-pending toggle; lifecycle all → live → seed; sort
  name → recency (020's field) → inbox-pending count.
- Pure module (no `ratatui` needed beyond what nav already uses; ideally none
  — it produces row data for nav to render). Headless tests.
- Fuzzy ranking wants a crate (e.g. `nucleo-matcher` or `fuzzy-matcher`);
  pick by maintenance/weight, record the choice in the leaf on completion.

## Done when

- `Criteria::engaged()` distinguishes idle (grouped shape) from engaged
  (flat ranked shape); empty-needle-with-toggle-engaged still ranks (stable
  under the active sort).
- Ranking is deterministic: fuzzy score desc, then the active sort order as
  tiebreak, then name.
- Headless tests: needle ranking, each toggle, each sort order, engaged/idle
  boundary, seeds under lifecycle filtering.

## Notes

The mode's *interaction* (keys, focus, footer) is 050 — this leaf is the
engine only, consumable by tests before any key exists.

**Completion notes (2026-06-10):**

- Fuzzy crate: **`nucleo-matcher` 0.3** — Helix's matcher, actively
  maintained, tiny dep footprint; `fuzzy-matcher` (skim's) rejected as
  dormant since 2023. A single fuzzy `Atom` (not `Pattern::parse`), so fzf
  operator syntax (`^`, `!`, `'`) is matched literally, no surprises.
- Module: `src/tui/filter.rs` (inside the directory wall; imports no
  ratatui). `Criteria` + `engaged()`, `SortOrder::cycle` /
  `LifecycleFilter::cycle`, `project(fleet, criteria) → Vec<Ranked>`, with
  the private `rank()` split out so tests rank hand-built rows without
  tempdir/git fixtures.
- Haystack = the row label: `<repo>/<grove>` at N>1, bare name at N=1 (same
  convention as the nav's flat picker shape) — at a single-repo fleet the
  repo name doesn't vacuously match every row.
- `engaged()` is `self != Criteria::default()` — the boundary is
  "any dimension off its default", including a non-Name sort alone.
