# chain-construction-k38

**Kind:** design

## Goal

Decide how to make the review chain (`X` → `review-X` → `integrate-review-X`)
and the research vendor pair (`research` → `research` → `combine-research`)
**reliably constructed as a unit**, rather than merely encouraged in prose — up
to and including a `grove-llm` verb that cuts the whole chain in one call.

The scope is **construction only**. Sequencing and scheduling stay unenforced:
`pick` remains a walk, not a scheduler (*task-tree-scheme*), and nothing here
may make grove refuse to proceed on an ordering.

## Context

Raised by the human during **release-doctor-toolchain-gap-k27**, as the
follow-on to two leaves that between them mapped the whole question and
deliberately stopped short of this:

- **compose-task-chains-k29** established that both patterns *were* documented
  in five reference surfaces and were still never used — this grove's own tree
  had 32 keys, 26 leaves, **zero** chains and zero pairs. It fixed the three
  surfaces a session reads *while cutting leaves* (bootstrap prompt, `SKILL.md`
  Decompose, `TASK-FORMAT.md`) and settled the naming as a **shared stem plus a
  terminal step suffix** (`<stem>-review` / `<stem>-integrate`; `<stem>-a` /
  `<stem>-b` / `<stem>-combine`). It stopped at **encouragement**, on purpose.
- **chain-group-unit-k36** asked whether a chain should be a first-class
  *scheduling* unit and answered **no** — two of the three costs motivating it do
  not exist, and the construct would **gate**. But it named the one real gap and
  the habit that closes it: **cut a chain's steps together; use `leaf-insert`,
  not `leaf-add`, for a step decided on after its producer ran.** That habit is
  the seam this leaf works in — k36 closed the *scheduling* question and left the
  *construction* one open.

So the open question is narrow and well-posed: encouragement demonstrably did
not produce chains before k29; is post-k29 prose enough, or does construction
need a mechanism? Note this leaf's own chain was cut by hand as three
`leaf-add` calls with hand-written slugs — one call per step, three chances to
stop after the first, and the naming convention re-derived from the brief rather
than supplied. That is the friction under examination.

## Done when

- A decision, recorded where a session will meet it, on **whether a mechanism
  exists at all** — a well-argued "prose is enough, here is why k29's fix
  changed the odds" is a legitimate and cheaper answer than a verb, and closes
  the question for good.
- If a mechanism: its **shape** is designed, not just named — the verb surface
  (e.g. `leaf-add-chain <parent> <stem> --kind <producer-kind>`, a `--chain`
  flag on `leaf-add`, or something else), what it emits, how the vendor pair's
  two-producers-plus-combine shape is expressed in the same surface, and what
  happens when only part of a chain is wanted.
- The design is tested against **constraints 3 and 5** explicitly — *suggested
  shape, not enforced schema*, and *grove guides, it does not gate*. A verb that
  makes the chain the **easy** path is compatible with both; a verb that makes a
  bare `leaf-add` harder, or that validates a tree's chain-completeness, is not.
  Say which side of that line the design sits on and why.
- The **kind-set consequence** is settled: producer kinds have a `review-` and
  an `integrate-review-` sibling, but `research` does not — its chain is a pair
  plus `combine-research`. Whatever the surface is, it covers both shapes or
  says why it covers only one.
- Whether the durable record is `docs/specs/task-kind-taxonomy.md` (which
  already carries the naming convention), an ADR, or both.

## Notes

Deliberately a `design` leaf, not `impl`: the human is clear on the goal and
explicitly open on the mechanism ("maybe even ensure through the grove-llm
tool"). Implementation, if any, decomposes from the outcome.

Cut as a chain — `chain-construction-k38` → `chain-construction-review-k39` →
`chain-construction-integrate-k40`, all three at once — because the artifact is
load-bearing (it would change `grove-llm`'s verb surface and the guidance three
surfaces carry), which is k29's own escalation call. Self-demonstrating, and
the cutting friction it exposed is evidence for this leaf.
