# insert-k24

**Integrates:** insert-k23

## Goal

Integrate `insert-k23`'s one actionable finding: make
`NoOccupantAtOrdinal`'s recovery message honest for a requested ordinal below
the level's first occupied ordinal, without losing the distinct advice for
past-the-end and hand-edited holes.

## Context

Beyond the brief chain and its *Read first* list:

- `insert-k23` in full. Its source coordinates are against producer commit
  `294262c1`; no later implementation leaf intervened.
- `operations.qnt` witnesses `wit_insertPastTheEnd` and
  `wit_insertIntoAGap`. The second proves only `a.at < maxOrdIn`; it does not
  prove that an occupied ordinal exists below `a.at`.
- `Refusal::NoOccupantAtOrdinal` and the unit/on-disk controls cited by the
  review. `Ordinal` deliberately permits zero on a hand-edited tree, so do not
  repair the prose by assuming `Ordinal::FIRST` is an enforced floor.

## Done when

- A refusal for a leading hole — at minimum a level containing ordinal 5 and a
  request for ordinal 1 — never claims that something below the request is
  occupied.
- Past-the-end still directs the caller to `append`; every unoccupied ordinal
  at or below the greatest still explains that no operation fills it and a hand
  edit is required. If the message continues to distinguish an interior gap,
  its carried state and tests actually prove both neighbours exist.
- Unit and public on-disk tests cover the leading-hole message as well as the
  existing interior-gap, empty-level and past-the-end cases.
- Reconcile the refusal docs and entry 012 of `docs/formalism-findings.md` with
  the final distinction; record this integration episode if it changes the
  method claim.
- Run both model suites, the crate and grove test suites, formatting and
  workspace clippy after the fix; retire only when all are green.

## Notes

The review accepted the implementation's greatest-ordinal computation, shift
ordering (including duplicate ordinals), intermediate-state projection,
subtree-preservation evidence and direct create-last controls. Do not widen
this leaf into those settled areas.

Codebase-memory could not index the review workspace because active-daemon
coordination was unavailable and coverage required unavailable approval. Retry
coverage if this session's environment permits it, but treat `insert-k23`'s
complete direct-source citations as the handoff rather than re-deriving the
finding from an empty graph.
