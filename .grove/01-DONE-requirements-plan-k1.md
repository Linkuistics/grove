# plan-k1

## Goal

Establish, in the human's own words, what per-project configuration override
should mean — the bootstrap leaf of this grove, whose only input was
[Linkuistics/grove#10](https://github.com/Linkuistics/grove/issues/10) and the
grilling that followed it.

## Context

The issue asked for a project-local `config.kdl` that "either completely replaces
the global config or selectively overrides", motivated by wanting to swap which
harness runs which phase in a particular project, to balance account usage.

Six of this leaf's open questions had interdependent answers, which met the
threshold for the full one-question-at-a-time interview.

## Done when

- The six decisions are settled with the human and confirmed as a set. **Done** —
  they are recorded in `.grove/BRIEF.md`, which is where every descendant reads
  them; this file deliberately does not duplicate them.
- The test seams are agreed while the human is present. **Done** — resolution is
  covered at `SessionConfig::load` and nowhere else, a boundary taken knowingly
  with its cost stated.
- The tree is grown far enough for a fresh session to continue. **Done** —
  `config-resolution-k2` (design) then `local-config-kdl-k3` (impl).

## Notes

No `review-requirements` leaf was cut, deliberately. These requirements were
settled by direct interview with the human, who confirmed the whole set before
anything was recorded — the human *was* the adversarial read. A session asked to
second-guess decisions their owner had just made is the ceremonial session the
lazy review chain exists to remove.

The workstream was judged small enough for this bootstrap leaf to resolve
requirements and cut the leaves itself rather than hand off to a `planning`
session: the design space closed during the interview, and what remains is one
decision record and one implementation.

Two things surfaced here that are *not* this grove's work and were reported to
the human rather than leafed: the three-way disagreement between the provisioned
skill, the installed binary's embed, and this checkout's `content/` (recorded in
the root brief's Notes, because a session editing `content/` must know), and the
pre-existing spawn-failure diagnostic that names the configuration path
unconditionally — that one *is* in scope, folded into `local-config-kdl-k3`,
because this change is what turns it from cosmetic into wrong.
