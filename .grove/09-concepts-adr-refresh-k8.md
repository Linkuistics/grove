# concepts-adr-refresh-k8

**Kind:** work

## Goal

Refresh `docs/concepts.md`'s **ADR** section to describe grove's current decision-record
scheme. Surfaced during `prd-to-spec-k4` and externalized rather than absorbed: the
staleness is a v9.0.0 leftover, unrelated to the PRD→spec cutover that leaf landed.

## Context

`docs/concepts.md` is grove's explainer for the vocabulary it borrows (DDD, Ubiquitous
Language, Bounded Context, Context Map, ADR, Spec). Its **ADR** section is stale in three
ways, all predating this workstream:

1. *"ADRs live in `docs/adr/` as `NNNN-slug.md`, one decision per file, numbered
   sequentially."* — grove's ADRs are **slug-named** (`docs/adr/<slug>.md`); sequential
   numbering was deliberately rejected (`linkuistics:decision-records`).
2. It cites `../content/ADR-FORMAT.md` as *"grove's preferred shape"* — since v9.0.0 that
   file is only a **placement** note; the format and template live in the
   `linkuistics:decision-records` skill.
3. It restates the three-part when-to-write test inline. That test is now owned by
   `linkuistics:decision-records`; restating it risks the two drifting.

The section also omits the rule that most distinguishes grove's ADRs: they are a
**minimum coherent set describing the current design**, edited/merged/deleted in place —
never a superseding chronology. `content/driving.md` ("Reworking ADRs and briefs") and
`CONTEXT.md` carry it; `concepts.md` does not.

## Done when

- `concepts.md`'s ADR section states slug-naming, the minimum-coherent-set rule, and
  edit-in-place rework; it **cites** `linkuistics:decision-records` for the format,
  template, and when-to-write test rather than restating them.
- Its `## Spec` section (landed by `prd-to-spec-k4`) and its ADR section agree on the
  grain rule: an ADR records one decision and its trade-off; a spec describes how an area
  works, and cites ADRs without restating them.
- No other section of `concepts.md` contradicts the current methodology (check
  Context Map / Ubiquitous Language against `CONTEXT.md` and `content/CONTEXT-FORMAT.md`).

## Notes

Clean-cutover prose discipline: describe the current scheme on its own terms. Do not
narrate the move away from `NNNN-` numbering — git and the CHANGELOG hold that.

Adjacent, already correct: `content/ADR-FORMAT.md` itself, and `CONTEXT.md`'s ADR-related
entries. This leaf edits `docs/concepts.md` only, unless the sweep in `Done when` turns up
a genuine contradiction elsewhere.
