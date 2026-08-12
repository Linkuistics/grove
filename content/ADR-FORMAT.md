<!-- unit: adr-placement-note class=procedural defers="adr-where-adrs-live adr-why-the-set-stays-minimal" -->
# ADR placement — a grove note

For the ADR **philosophy, format, minimal template, and the when-to-write test**,
use the `linkuistics:decision-records` skill. This note keeps only grove's
placement conventions.

<!-- unit: adr-where-adrs-live class=procedural -->
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

<!-- unit: adr-why-the-set-stays-minimal class=procedural -->
## Why the set stays minimal

A grove session reads only the ADRs its brief chain cites — three ADRs, not fifty.
That curation works only when `docs/adr/` is a **minimum coherent set describing
the current design**, not an append-only chronology. Keeping the set small and
current-state — edit in place; merge / split / delete as understanding shifts — is
what `linkuistics:decision-records` governs; this note just fixes *where* the files
go.
