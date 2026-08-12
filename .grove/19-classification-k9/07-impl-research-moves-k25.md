# research-moves-k25

## Goal

Classify **`content/driving.md` lines 1–263** (13,580 bytes): the file's framing,
`## In this guide`, `## When not to start a grove`, `## When to commission
prior-art research`, `## How to write a research leaf brief`, `## Running the
vendor pair`, `## When to invoke a design discussion (grilling)` and its four move
subsections (WDYT, pushback, don't merge questions, record decisions inline).

This is batch 5 of 12, and the first of four over `driving.md`.

## Context

Read the node brief's *The batching contract* first.

### Region and residual

- Carve `content/driving.md` **L1–L263**.
- The seed unit `driving` is **consumed**. Mint exactly one residual,
  **`pending-driving-evidence`**, covering **L264–L754**.

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
productive design work"*. `SKILL.md` L224 (*"See `driving.md` for the field-guide
habits…"*) is the hub-side trigger that will eventually defer here, and
`decompose-moves-k28` will hang `driving.md`'s genuinely orphan sections
(`## Anti-patterns`, `## The shortest version`) off whatever you carve here.

So: **carve a framing unit that can serve as this file's root**, and name its id
in your leaf body. Three later batches depend on it.

Note the licence comment at L37–40 (mattpocock/skills attribution for the no-fog
early exit). It belongs with `## When not to start a grove` and must not be split
away from the prose it attributes.

### Cross-file deferral

- `TASK-FORMAT.md` is cited at L141 and L156 — both point at *the shape*, which
  `shapes-k23` has already carved. Sweep those: where the reference is genuinely
  trigger→body, write the `defers=`.
- `grilling.md` is cited at L189. `guides-k24` has carved it, so this edge is
  available and should be written.
- `SKILL.md` references in this region point at conditions, not bodies. **A prose
  cross-reference is not automatically a `defers=`**, and a `defers=` naming a
  triggering unit is a build error.

## Done when

- `content/driving.md` L1–263 is subdivided into real units;
  `pending-driving-evidence` covers L264–754 and nothing else.
- `cargo build` and `cargo test` are green.
- `EMBEDDED_UNITS` updated in the same commit: `driving` removed, the new
  `driving-*` ids added, `pending-driving-evidence` added.
- The framing unit's id is named in this leaf's body, and the `## When not to
  start a grove` call is stated with its reasoning.

## Notes

- `## Running the vendor pair` (3,182 bytes) contains a fenced `leaf-add-pair`
  example. Do not split mid-fence; the parser forbids it and the build will say
  so, but the authoring rule behind it is the one no build checks.
- Doubts to carry forward, by id.
