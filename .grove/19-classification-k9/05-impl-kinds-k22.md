# kinds-k22

## Goal

Classify **`content/TASK-FORMAT.md` lines 1–192 and lines 473–501** (13,189 bytes
together): the file's framing and filename grammar, `## The nineteen kinds` with
its per-kind discipline bullets, and `## The three design kinds — extra guidance`.

This is batch 2 of 12, and it is **where the `kinds=`-scoped units live if they
live anywhere**. It is the highest-value classification judgement in the corpus.

## Context

Read the node brief's *The batching contract* first.

### Region and residual

- Carve **L1–L192** and **L473–L501**. The two regions are one closure: L473–501
  is extra guidance on three of the nineteen kinds L74–101 introduces.
- The seed unit `task-format` is **consumed**. Mint exactly one residual,
  **`pending-task-shapes`**, covering the middle **L193–L472**, as
  `class=triggering kinds=*` with no `defers=`. `shapes-k23` consumes it.

### Cross-file deferral: none yet, and that is deliberate

L77 points at `grilling.md`, L99–101 at `driving.md`, and L478–486 at
`grilling.md`, `ADR-FORMAT.md`, `SPEC-FORMAT.md` and `BRIEF-FORMAT.md`. **Write no
`defers=` for any of them.** Those files are still single trivial triggering
units, so a `defers=` would fail twice over — unknown target, and wrong class if a
same-named unit existed.

That is the decoupling lemma working as intended: nothing has moved yet, so the
"triggering unit left with no `defers=` when its body moved" trap is not live
here. The batches that carve those bodies own the edges, and their inbound sweep
(`grep -rn 'grilling\.md' content/` and friends) will find your markers:

- `guides-k24` will add `defers=` to your `requirements` unit, your `design`
  unit, and the corresponding bullets in `## The three design kinds`.
- `decompose-moves-k28` will add `defers=` to your `planning` unit for
  `BRIEF-FORMAT.md`.
- One of `research-moves-k25` … `decompose-moves-k28` will add `defers=` to your
  `impl` unit for `driving.md`.

**Name those units so a later sweep can find them by grep**, and list their ids in
your leaf body.

### The judgement this batch exists for

`kinds=*` is the overwhelming default; an explicit list is for guidance genuinely
about **one kind's discipline**. This region is the densest concentration of such
guidance in the corpus, and there is **no family shorthand and no negation** — a
scope that wants to say "every producer" is spelled out in full or is `*`.

Weigh each of these separately rather than as a block:

- The **per-kind bullets** (L74–115: five producers, the research trio) — each is
  a candidate `kinds=<that kind>` unit. But a kind's discipline is also what a
  session of *another* kind needs when deciding what to cut next, which argues
  some of them are `kinds=*`. Decide per bullet, and say why.
- The **HITL/AFK paragraph** (L64–72) — it is about all nineteen and ends with
  "the mark **predicts, it does not permit**", a condition every kind needs. Very
  likely `kinds=*`.
- The **kind table and the "closed set" paragraph** (L37–62) — the fact that the
  set is closed and that `finish` is driver-reserved is a condition; the table
  itself may be its procedural body.
- The **filename-grammar framing** (L6–33) — `pruning is HITL, never an agent's
  own call` is a condition with real teeth; the round-tripping rules read
  procedural.

### Traps specific to this region

- **Classifying by size.** A long conditional is still a condition; the `planning`
  bullet (L84–94) is the longest and is plainly triggering for `planning`.
- **`kinds=` is required on triggering and forbidden on procedural.** A scoped
  procedural unit is a build error, and it is the easiest mistake to make here
  because the scope feels informative.

## Done when

- `content/TASK-FORMAT.md` L1–192 and L473–501 are subdivided into real units;
  `pending-task-shapes` covers L193–472 and nothing else.
- `cargo build` and `cargo test` are green.
- `EMBEDDED_UNITS` updated in the same commit: `task-format` removed, the new
  `task-*` ids added, `pending-task-shapes` added — each named deliberately.
- The ids that later batches must sweep back to (`requirements`, `design`,
  `planning`, `impl`, and the three design-kind bullets) are listed in this leaf's
  body by id.

## Notes

- Follow the id-naming convention `spine-k21` recorded; do not invent a second.
- Doubts to carry forward, by id — the scope decisions here are the ones most
  likely to be wrong in a way no build sees, so be generous about naming them.
