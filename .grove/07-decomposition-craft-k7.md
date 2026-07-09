# decomposition-craft-k7

**Kind:** work

## Goal

Fold the decomposition-craft enrichments (report items G4 + remaining G5 in
`docs/research/mattpocock-skills-v1.1-incorporation.md`) into grove's
methodology prose: `content/driving.md`, `content/BRIEF-FORMAT.md`,
`content/CONTEXT-FORMAT.md`, and (lightly) `content/SKILL.md`'s Decompose
step.

## Context

Sequenced after k5 (issues-substrate) so the prose lands on the decided
substrate, and after k4/k6 so any renamed terms (spec; enriched kinds) are
already settled. All upstream quotes are in the report — write in grove's
voice (adapt, don't transplant; per the existing addyosmani precedent in
driving.md, note adapted-from provenance where a section derives from
upstream).

## Work items

1. **Vertical-slice leaf shape** (`to-tickets`): a good child leaf cuts a
   narrow-but-complete path and is independently demoable/verifiable — a
   second axis for the "fits this session" test. → driving.md Externalizing +
   SKILL.md Decompose (one clause).
2. **Wide-refactor expand→contract exception** (`to-tickets`): the
   blast-radius change no vertical slice can land green; expand → migrate in
   batches → contract, each batch a leaf. → driving.md.
3. **"Not yet specified" horizon note + fog-or-ticket test** (`wayfinder`): a
   short "on the horizon" note in a `BRIEF.md` records the dim view between
   sessions without pre-slicing it; the test is "can you state the question
   precisely now — not answer it now". → BRIEF-FORMAT.md + driving.md
   Decompose habits.
4. **Durable-brief discipline** (`triage/AGENT-BRIEF.md`): briefs state
   behavioural contracts and named types, not file paths / line numbers —
   extends the key-durability discipline to brief *content*. → BRIEF-FORMAT.md.
5. **No-fog "don't grove this" gate** (`wayfinder`): a one-paragraph "when NOT
   to start a grove" — if the whole journey fits one session, you don't need a
   map. → driving.md (near the top) or SKILL.md fresh-grove-start.
6. **Glossary rule** (`teach/GLOSSARY-FORMAT.md`): "Use the glossary's own
   terms inside definitions." → CONTEXT-FORMAT.md rules (a deliberate
   divergence from upstream's trimmed version — note it in the provenance
   header).
7. **Negation lint pass**: sweep the bundled `content/*.md` for bare
   prohibitions not paired with a positive target; rephrase the few that fail.
   (Validation pass, expected near-no-op.)

## Done when

All seven items landed in grove's voice, one focused commit; provenance notes
added where sections derive from upstream files.

## Notes
