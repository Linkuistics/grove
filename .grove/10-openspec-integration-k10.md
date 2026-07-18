# openspec-integration-k10

**Kind:** planning

## Goal

Work out whether and how to incorporate openspec.dev
(<https://openspec.dev/>) into grove's own skill/methodology, and grow the
tree with whatever follow-up work that implies.

## Context

- Raised by the user mid-session on `harness-spawn-preflight-k8`, unrelated to
  that leaf's goal — externalized here rather than absorbed.
- This repo *is* grove's own source (`content/` is the canonical skill grove
  embeds and provisions — see `content/SKILL.md`, `content/SPEC-FORMAT.md`).
  grove already has its own spec/ADR conventions (`docs/adr/`,
  `docs/specs/<slug>.md`, `linkuistics:decision-records`); a first grilling
  question is how openspec.dev's model overlaps or conflicts with those before
  reaching for it, rather than bolting on a second convention.

## Done when

Not yet known — this is the first planning pass. At minimum: what
openspec.dev actually offers is understood, a decision on adoption (fully, in
part, or not at all) is reached and recorded (ADR only if it clears the
when-to-write bar), and the tree carries concrete child leaves for whatever
that decision implies.

## Notes

Scope check during grilling: this grove's `BRIEF.md` charters trial-hardening
loop-mechanics friction specifically. If openspec-integration turns out to be
a substantial, unrelated workstream, consider whether it should move to its
own grove rather than live out its life as a leaf here — ask the user.
