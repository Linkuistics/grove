# ADR placement — a grove note

For the ADR **philosophy, format and minimal template**, use the
`linkuistics:decision-records` skill. The when-to-write test is stated here, so
that a session without that plugin still applies the same test; the rest of this
note is grove's placement conventions.

## When a decision earns a record

Write one only when **all three** hold:

- it is **hard to reverse**, and
- it is **surprising without context**, and
- it is the result of a **real trade-off** — a rejected alternative exists.

All three, not any one. A decision that fails any of them is recorded where it
lands — in the code, the spec, or the commit — and needs no ADR. This is an AND
test, and any looser paraphrase of it ("raise ADRs sparingly") is not the test.

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

## Why the set stays minimal

A grove session reads only the ADRs its brief chain cites — three ADRs, not fifty.
That curation works only when `docs/adr/` is a **minimum coherent set describing
the current design**, not an append-only chronology. Keeping the set small and
current-state — edit in place; merge / split / delete as understanding shifts — is
what `linkuistics:decision-records` governs; this note just fixes *where* the files
go.
