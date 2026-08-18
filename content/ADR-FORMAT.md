# ADR placement — a grove note

This note carries grove's own ADR rules: when a decision earns a record, where
records live, and what keeps the set honest. The
`linkuistics:decision-records` skill deepens the philosophy and the template;
what binds without it is stated here.

## When a decision earns a record

Write one only when **all three** hold:

- it is **hard to reverse**, and
- it is **surprising without context**, and
- it is the result of a **real trade-off** — a rejected alternative exists.

All three, not any one. A decision that fails any of them is recorded where it
lands — in the code, the spec, or the commit — and needs no ADR. This is an AND
test, and any looser paraphrase of it ("raise ADRs sparingly") is not the test.

## The minimal record

A slug title, the decision in a sentence or two, the trade-off it settles, and
the alternative that was rejected and why. No status line, no date, no number,
no template sections that carry nothing.

## Where ADRs live

ADRs live in `docs/adr/`, one file per record, **slug-named**: `docs/adr/<slug>.md`.
The slug *is* the ADR's identity — cite it by slug/title, never by a number.
Create the `docs/adr/` directory lazily, only when the first ADR is needed.

In a multi-context repo (one carrying a root `CONTEXT-MAP.md`), whether the set
splits is **conditional on the repo's shape**, not automatic:

- **Split** when the contexts are *peers*, each rooted in its own subtree:
  system-wide decisions under the root `docs/adr/`, context-specific ones under
  that context's own `docs/adr/`. The map points to where each context lives.
- **Stay flat** — one root `docs/adr/` for the whole repo — when a split cannot
  produce that partition. The usual case is a context that occupies the **repo
  root**: its ADRs have no home but `docs/adr/`, so splitting exiles the nested
  context's records while leaving the root set just as mixed. A set small enough
  that one directory is the cheaper index is the other case. Then
  `CONTEXT-MAP.md` records which context **owns** each record, and the slug is
  unique **repo-wide** rather than per-directory.

Choose on the evidence, not the letter. The point of a split is a partition a
reader can trust; a split that leaves the root directory still mixed has bought a
second search path for nothing.

## The set is a minimum coherent set

A grove session reads only the ADRs its brief chain cites — three ADRs, not
fifty. That curation works only when `docs/adr/` is a **minimum coherent set
describing the current design**, not an append-only chronology.

So a session that *changes* a decision an ADR already records **reworks the set
in place** and **never appends a superseding record**:

- **Edit in place.** Rewrite the affected record to state what *now* binds and
  why. Merge two whose decisions converged; split one that turned out to cover
  two independent calls; delete one whose decision no longer holds. No
  `superseded by`, no status line — the artifacts hold the present and the VCS
  holds the past (constraint 1).
- **Keep the set minimal.** After the edit it should be the fewest records that
  coherently explain the current design. A rework leaving both the old record
  *and* a new one live has failed the test.
- **Reconcile every citation.** A merged or deleted record leaves dangling
  references behind: find its slug across the briefs, the other records and
  `docs/`, and fix or drop each pointer. A dangling citation is a defect, not
  acceptable collateral.

## Retiring research into the set

An adopted research finding gets a **bridge pointing both ways**: the survey
under `docs/research/` gains a section naming the ADRs its findings landed in,
and each of those ADRs cites the survey by primary source in its rationale. A
finding left only in `docs/research/` binds nothing, and a reader of either
artifact can then trace the chain without re-doing the research. A finding that
overturns a recorded decision rewrites that record under the rule above rather
than spawning a superseding one.
