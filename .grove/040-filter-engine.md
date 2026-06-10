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
