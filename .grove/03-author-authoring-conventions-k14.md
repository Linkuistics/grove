# author-authoring-conventions-k14

**Kind:** work

## Goal

Create a **house authoring-conventions** reference for this marketplace, and apply the
one decision the survey forces: the **description-shape** rule, auditing+fixing the 9
existing skills' descriptions to match it. (Synthesis skills disposition **AUTHOR #3**.)

## Context

- This is a **thin house delta**, NOT a fork: we already depend on superpowers'
  `writing-skills` (its SessionStart hook loads it every session — see survey
  §obra/superpowers S1). The note records *our* conventions and points at the upstream
  craft skills; it does not duplicate them.
- The ⚑ **decision** (survey Synthesis, "One decision the survey forces"): our corpus is
  split — the marketplace skills use anthropics' *"what + when, pushy"* shape while
  **grove's own skill** uses superpowers' *"when-only"* shape. Resolve to one house rule:
  `description` = one-sentence **capability** + explicit **"Use when"** triggers, pushy
  enough to beat undertriggering, **never** a step-by-step workflow/process summary.
  (hermes-S2's ≤60-char cap is too tight for our routing sentences — keep the shape, not
  the byte cap; we sit ≤470 under a 1024 spec limit.)
- Conventions to encode (all from the Synthesis "Authoring conventions" cluster): C1
  description-discipline; Match-the-Form-to-the-Failure + the prohibition-backfires caveat
  (superpowers-S3 / addyosmani-S4 / anthropics-S4); the micro-test-against-a-no-skill-
  control (superpowers-S4 / anthropics-S6); progressive-disclosure thresholds (<500-line
  body, >300-line ref → ToC, one-level-deep, **no `@path` links**); the user-invoked vs
  model-invoked lever (mattpocock-S3); and the source-authority + UNVERIFIED contract
  lifted from addyosmani-S2.

## Done when

- A conventions artifact exists (a reference doc, or a `disable-model-invocation`
  user-invoked authoring skill — decide which; it is hand-only authoring guidance, so
  zero-context-load user-invoked is the natural fit per mattpocock-S3).
- The 9 existing skill `description:` fields are audited against the house rule and any
  that summarize a *workflow* are reshaped to capability + "Use when".

## Notes

- Read the Synthesis section of `docs/research/skill-repo-prior-art.md` first (the
  CONVENTION items cite their primary sources).
- Sequenced first deliberately: it codifies *how* to write the skills authored in the
  sibling leaves (k15–k17).
- The CI lints (size-budget, `skills-ref validate`, name-collision, manifest-invariant)
  are **out of scope here** — they are "[LINT] adopt as the corpus grows", not part of
  this note. Mention them as future work only.
