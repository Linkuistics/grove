# signal-unit-placement-k3

## Goal

Make the `skill-signal` unit compose **last** in every kind's mandate, so the
instruction to run `grove-llm complete` is the final thing a session reads at
launch.

## Context

`skill-signal` lives in `content/SKILL.md` (line 558 at the time of writing),
which carries `<!-- file: order=2 -->`. Seven whole files compose after it:

```
1 MANDATE.md   2 SKILL.md   3 TASK-FORMAT.md   4 BRIEF-FORMAT.md
5 CONTEXT-FORMAT.md   6 ADR-FORMAT.md   7 SPEC-FORMAT.md
8 grilling.md   9 driving.md
```

So the unit that says "your **last action**" is followed by the majority of the
mandate. This slice runs second deliberately: it changes what *every* session
receives, so it lands against the already-improved baseline of
`retire-next-steps-k2`.

## Done when

- `skill-signal` composes last in the mandate of every kind its scope admits —
  all eighteen non-`finish` kinds.
- The completeness invariant still holds: every triggering unit in the mandate of
  every kind its scope admits, every procedural unit in none, every procedural
  unit reachable by following declared deferrals (`tests/methodology.rs`).
- `content/` file positions stay contiguous from 1, and the test pinning that
  convention passes.
- CHANGELOG entry under `## Unreleased`.

## Notes

**The obvious mechanism, and why it is cheap.** A new `content/` file carrying
`<!-- file: order=10 -->` and holding the moved unit. Appending at the end is the
one insertion that renumbers nothing — `CONTEXT.md`, *File directive*: positions
are contiguous from 1 as a readability convention pinned by a test, a legal gap
is not insertion slack, and inserting *between* files renumbers every later one.

**The open question, and it is genuinely yours to settle.** Moving the unit out
leaves `SKILL.md`'s loop narrative reading Retire → Commit → *(gap)* → Finish.
Decide what, if anything, stands in that gap. Three constraints bear on it:

- Unit ids are unique across the whole embed, so the unit cannot simply appear
  in both places.
- A mandate slice is source bytes, never a paraphrase (`CONTEXT.md`, *Mandate
  slice*), so a hand-written summary left behind in `SKILL.md` would be a second
  source of truth for a rule that already has one.
- Constraint 7: if the loop does not fit on a page, cut until it does.

If settling that proves bigger than one focused session, `leaf-decompose` rather
than absorbing it.

**Verification is a rebuild away.** `content/` is fixed at build time by
`include_dir!`, so inspect the composed result through `grove-llm methodology`
and the test suite — not by launching a session from this loop (root brief,
*caveat on verification*).
