# taxonomy-spec-k12

**Kind:** planning

## Goal

Write the design down and reconcile the ADR set, so `02`–`05` implement from a
written spec rather than from this node's brief. Settle the two questions the
brief lists as still open.

## Context

- The node brief's **Decisions** section is the input. This leaf turns it into
  a spec plus reworked ADRs; it does not re-litigate it.
- ADR *task-kind-taxonomy* — its central claim is "a closed set of **five**",
  with a "Why the set is closed" argument and a "Considered options" entry
  rejecting a free-text label. Seventeen kinds does not weaken that argument
  (the set is still closed, still enumerable, still inheritable by
  `leaf-decompose`) but every count in it is wrong and the discipline table is
  incomplete.
- ADR *model-per-task-kind* — carries the **no-fallback** rule and rejects
  fallback chains outright. The family axis needs that rule carved, not deleted:
  the reason it exists (never pick a model for a kind the user never configured)
  still holds *across* kinds.
- `linkuistics:decision-records` for the rework discipline; `SPEC-FORMAT.md`
  for the spec's shape and the membership/grain rules.

## Done when

- `docs/specs/task-kind-taxonomy.md` exists and states: the seventeen kinds,
  each one's **discipline** and **HITL/AFK** mark, the two patterns (review
  chain, vendor pair) and their differing character, and the two routing
  mechanisms with their precedence.
- Both ADRs are reworked **in place** — no superseding record, no `status:` line
  — and every citation of either is reconciled (grep both slugs across
  `.grove/`, `docs/`, `content/`, `README.md`).
- The two open questions are answered in the spec:
  - **enforced vs documented grammar** (recommendation: documented — mutable
    positions make sibling-order validation unenforceable);
  - **HITL/AFK per kind**, including whether `planning` flips to AFK once
    grilling moves to `requirements`.
- The spec passes the membership test: would a session on an unrelated future
  grove need to read this? If any part is really a work-order for `02`–`05`, it
  belongs in this node's brief instead and dies with `.grove/`.

## Notes

The grain rule matters here: the spec describes *how the area works* and
**cites** the ADRs rather than restating them. Restate one and the two sets will
drift, after which neither binds.

Seventeen disciplines is a lot of prose to write well. If the spec turns into
seventeen thin paragraphs saying the same thing with different nouns, that is
evidence the parameterisation is too fine — surface it rather than padding,
since a kind that cannot justify a distinct discipline is precisely what
*task-kind-taxonomy* says should not exist.
