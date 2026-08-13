# family-scope-guard-k18

## Goal

Guard the three **family-shaped `kinds=` scopes** that a kind added later would
have to be hand-added to, so that adding one fails the build until its marker
scope is widened — the same compile-time force `Kind::is_producer` already
applies to the producer roster.

The three, each exactly a family partition of the closed kind set:

| unit | file | scope |
|---|---|---|
| `task-review-kinds` | `content/TASK-FORMAT.md` | the five `review-*` kinds |
| `task-integrate-review-kinds` | `content/TASK-FORMAT.md` | the five `integrate-review-*` kinds |
| `task-research-pair` | `content/TASK-FORMAT.md` | `research-a research-b` |

## Context

Surfaced by `unit-scope-audit-k4`, which recounted every narrowed unit in the
embed while deciding when narrowing is admissible at all. That decision is
recorded in `docs/specs/mandate-delivered-methodology.md`, *A unit narrows when a
kind added later would not need it* — read it first; this leaf builds the one
piece of code it defers.

**The hazard is the explicit list's mirror**, the one
`docs/adr/mandate-delivers-the-methodology.md` names when it rejects family
shorthand: a kind added later is silently *omitted* from guidance it needs. Of
the twenty narrowed units, sixteen name a single kind and are self-correcting — a
new kind arrives with its own units and no single-kind scope ever widens for it.
Four name a **set**, and only `skill-signal` is covered (*Requirement: Every
kind's mandate states exactly one session ending*). The three above are not.

**The omission is silent, and the two existing reds do not reach it.** Adding a
kind goes red at `Kind::is_producer` (`src/leaf.rs`), whose exhaustive `match` is
deliberately not a roster lookup so a new variant fails to build until someone
classifies it; and at the session-ending guard, which names the kind. Neither
points at these three markers, so an author fixes what complained and leaves them
narrow. The composition golden does not catch it either — a new kind produces a
*new* golden section rather than failing an existing one, which is the same
reasoning the ending guard's rationale already records for itself.

**Why this is cheap.** Each scope is *derivable*, not judged: it is a family
partition, so an exhaustive `match` over `Kind` yields the expected label set and
the assertion is that the marker's scope equals it. No second source of truth is
created — the marker stays the single statement of scope, and the test derives
what it should be from the enum rather than restating it. That is precisely the
condition the spec sets for narrowing a set-shaped scope, applied to three scopes
that were narrowed before the condition existed.

## Done when

- Each of the three units' `kinds=` scope is asserted equal to a family set
  derived from an exhaustive `match` over `Kind` — not from a hand-written
  roster, and not from `Kind::ALL` filtered by a name prefix, either of which
  reintroduces the roster this exists to remove.
- A twentieth kind added to the enum **fails to compile or fails the assertion**,
  naming the marker that must be widened. Assert this the way
  `tests/session_kind_guidance.rs` does — it already generates its claims from
  the kind enum so a new kind fails until the guidance names it, and this is the
  same claim about a different surface.
- The classifier carries **both controls**, on the repository's standing rule
  that a sweep which cannot fail is worth nothing: shown failing on a synthetic
  marker whose scope is missing a family member, and passing on the real embed.
- `skill-signal`'s eighteen-label scope is left to the ending guard that already
  covers it; this leaf adds no second check over it.

## Notes

**Do not widen this into the scope audit.** `unit-scope-audit-k4` decided that
nothing narrows further, and this leaf changes **no unit's scope** — it asserts
that three existing scopes stay correct as the kind set moves. If the work
suggests a scope should change, that is the reopen condition in the spec, not
this leaf.

**The seam is the existing one.** `methodology` already exposes parse over
`(path, text)` and the embed's unit set, which is where the markers' scopes come
from; `src/leaf.rs`'s test module is where the exhaustive-match precedent lives.
Prefer both to anything new — the spec's *Test seams* drove the count down to one
deliberately.

**Watch the ordering.** This leaf was `leaf-insert`ed ahead of
`templated-mandate-k12`, which the human placed last on purpose. Anything further
for this concern inserts ahead of that leaf too, rather than appending after it.
