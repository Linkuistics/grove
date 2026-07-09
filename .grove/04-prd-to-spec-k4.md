# prd-to-spec-k4

**Kind:** planning

## Goal

Grill and decide: reframe grove's **PRD** as a **spec** — the user's rationale
(agreeing with upstream `to-spec`) is that what grove's planning increments
produce is *not* really a PRD. Land the decision (docs + methodology edits, or
follow-on work leaves), reworking the artifact story coherently.

## Context

- grove today: `docs/prd/` = "human-facing agreement checkpoints; committed,
  never retired" (SKILL.md artifacts table + PRD section); **and separately**
  `docs/specs/*-design.md` = "workstream-level technical design". A rename
  must reconcile these two rows — one artifact kind, two, merged?
- Upstream `to-spec/SKILL.md` (see report §G5): synthesises the conversation
  into a spec; explicitly "Do NOT interview the user" (grilling already
  happened); **sketches the test seams as a planning output** — "Use the
  highest seam possible… the ideal number is one. Check with the user that
  these seams match their expectations."
- Seam-sketching is folded into THIS leaf (plan-k1 decision): if grove's
  planning flow produces a spec, naming the agreed test seam belongs in it.

## Questions to grill

1. Is it a rename (`docs/prd/` → merged into `docs/specs/`?) or a reframe
   (grove keeps two artifact kinds with sharper names)? What happens to
   existing PRDs in consuming repos (migration? leave in place?).
2. What does grove's spec *contain* — the agreement checkpoint content plus
   named test seams? Adopt to-spec's "don't re-interview; synthesise" rule?
3. Which files change: `content/SKILL.md` (PRD section + artifacts table +
   loop prose), `grilling.md` (MAY-write-a-PRD line), glossary entry,
   launcher prompts? An ADR, or is this below the when-to-write bar?

## Done when

Decisions recorded (running log → durable docs), tree grown with the
implementing work leaf(s) or the edits done inline if they fit the session.

## Notes

Clean-cutover prose discipline applies: describe the new scheme on its own
terms; don't carry "formerly PRD" contrast through the docs (git holds
history).
