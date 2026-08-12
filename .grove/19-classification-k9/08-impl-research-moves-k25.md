# research-moves-k25

## Goal

Classify **`content/driving.md` from its body start to the line before `## When to
retire research into ADRs versus leave it`** (baseline L1–263, 13,580 bytes): the
file's framing, `## In this guide`, `## When not to start a grove`, `## When to
commission prior-art research`, `## How to write a research leaf brief`, `##
Running the vendor pair`, `## When to invoke a design discussion (grilling)` and its
four move subsections (WDYT, pushback, don't merge questions, record decisions
inline).

This is batch 5 of 12, and the first of four over `driving.md`. Its framing unit is
**the root three later batches depend on** — see below.

## Context

Read the node brief's *The batching contract* first.

### Region and residual

- **Anchors are authoritative; L1–263 is a baseline coordinate.** Carve from the
  file's body start to the line **before** `## When to retire research into ADRs
  versus leave it`.
- The seed unit `driving` is **consumed**. Mint exactly one residual,
  **`pending-driving-evidence`**, covering that heading to end of file, as
  `class=triggering kinds=*` **with no `defers=`** — a residual is a coverage
  placeholder, never an edge ledger.

### `driving.md` roots much of itself, and that is why it comes before `SKILL.md`

The node brief's corpus table calls this file "mostly procedural", and that is
true of its *bodies* — but its `## When to …` headings are **genuine
conditions**, and they are what makes this file batchable without `SKILL.md`.
Three in this region:

- **`## When to commission prior-art research`** — plainly triggering. A session
  never told this condition exists never commissions research, and never learns
  there was a question. Its bodies are `## How to write a research leaf brief`
  and `## Running the vendor pair`.
- **`## When to invoke a design discussion (grilling)`** — triggering; the four
  `###` moves beneath it are its procedural body.
- **`## When not to start a grove`** — read this one carefully before assuming it
  is triggering. The condition it states is faced by *a human deciding whether to
  start a grove*, not by a session already inside one. If you conclude it reaches
  no session's mandate honestly, it is procedural and must be reached from
  somewhere — the file's framing unit is the natural root. **Say which you chose
  and why**; it is the first genuinely debatable call in this file.

### The framing unit, and what later batches will hang off it

`driving.md`'s opening (L1–16) and `## In this guide` (L18–35) are the file's
catch-all entry: *"the moves a human collaborator makes that turn the loop into
productive design work"*, followed by an index naming **every** section of the file.
That is what makes it an honest root rather than a convenience: the opening names
externalizing, doubting and source-grounding outright, and the index names the rest
by title.

**Carve a framing unit that can serve as this file's root, and name its id in your
leaf body.** Three later obligations depend on it:

- **row 12** — `decompose-moves-k28` roots `## Externalizing surfaced work` here,
  because that section is a **body** whose semantic owner (`SKILL.md`
  `**Decompose.**`) is not carved until #9;
- **row 13** — `decompose-moves-k28` roots `## Anti-patterns` and `## The shortest
  version` here;
- **row 23** — `execute-k29` deferring `SKILL.md` L224–227 (*"See `driving.md` for
  the field-guide habits…"*) into the grilling-moves bodies.

Note the licence comment at L37–40 (mattpocock/skills attribution for the no-fog
early exit). It belongs with `## When not to start a grove` and must not be split
away from the prose it attributes.

### Edge inventory rows owned: 34 and 35

| row | edge | note |
|---|---|---|
| 34 | `## When to invoke a design discussion (grilling)` (L182, L189) → `grilling.md` bodies | `guides-k24` carved the target, so this is writable and looks genuine: the condition is *invoke a design discussion*, the body is the interview procedure |
| 35 | `## Running the vendor pair` (L141, L156) → `TASK-FORMAT.md` §*The vendor pair* / §*What the shapes are not* bodies | Read them twice. Both are parenthetical `(`TASK-FORMAT.md`)` **citations supporting a claim** — *"There is no node directory"*, *"a fact the filename already carries"* — not trigger→body edges. **Declining with that reason is the expected outcome**, and a decline recorded is a different act from an edge silently not written |

`SKILL.md` references in this region point at conditions, not bodies, and they sit
inside `pending-skill-*` anyway. **No edge may have a `pending-*` source**, and a
`defers=` naming a triggering unit is a build error. Report those hits as *not
yours*; park nothing.

## Done when

- The region between the two anchors is subdivided into real units;
  `pending-driving-evidence` covers the rest of the file and nothing else, and
  carries no `defers=`.
- `cargo build` and `cargo test` are green.
- `EMBEDDED_UNITS` updated in the same commit: `driving` removed, the new
  `driving-*` ids added, `pending-driving-evidence` added.
- **Rows 34 and 35 are reported** — written, or declined with the reason.
- The framing unit's id is named in this leaf's body, and the `## When not to
  start a grove` call is stated with its reasoning.

## Notes

- `## Running the vendor pair` (3,182 bytes) contains a fenced `leaf-add-pair`
  example. Do not split mid-fence; the parser forbids it and the build will say
  so, but the authoring rule behind it is the one no build checks.
- Doubts to carry forward, by id.
