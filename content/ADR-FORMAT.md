# ADR placement — a grove note

For the ADR **philosophy, format, minimal template, and the when-to-write test**,
use the `linkuistics:decision-records` skill. This note keeps only grove's
placement conventions.

## Where ADRs live

ADRs live in `docs/adr/`, one file per record, **slug-named**: `docs/adr/<slug>.md`.
The slug *is* the ADR's identity — cite it by slug/title, never by a number.
Create the `docs/adr/` directory lazily, only when the first ADR is needed.

In a multi-context repo (one carrying a root `CONTEXT-MAP.md`), place each ADR
with the context it belongs to: system-wide decisions under the root `docs/adr/`,
context-specific decisions under that context's own `docs/adr/`. The map points to
where each context lives.

## Why the set stays minimal

A grove session reads only the ADRs its brief chain cites — three ADRs, not fifty.
That curation works only when `docs/adr/` is a **minimum coherent set describing
the current design**, not an append-only chronology. Keeping the set small and
current-state — edit in place; merge / split / delete as understanding shifts — is
what `linkuistics:decision-records` governs; this note just fixes *where* the files
go.
