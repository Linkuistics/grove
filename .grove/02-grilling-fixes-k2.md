# grilling-fixes-k2

**Kind:** work

## Goal

Apply the three sure-thing upstream fixes to `content/grilling.md`
(report items G1–G3 in
`docs/research/mattpocock-skills-v1.1-incorporation.md`).

## Context

grove's `content/grilling.md` is bundled from `mattpocock/skills@b8be62f`;
upstream has since fixed a bug in the exact line grove carries, added a
confirmation gate, and restructured the source files. `content/` is canonical —
the binary embeds it and provisions the global skill on `grove do`; editing
`content/` is the whole job (release/version-bump is a separate concern).

## Work items

1. **G1 — self-grilling fix (defect).** Replace line 11
   ("If a question can be answered by exploring the codebase, explore the
   codebase instead.") with upstream's fix (`e5932a7`):
   > If a *fact* can be found by exploring the codebase, look it up rather
   > than asking me. The *decisions*, though, are mine — put each one to me
   > and wait for my answer.
2. **G2 — confirmation gate**, grove-worded (upstream `0e9a072` says "Do not
   enact the plan until I confirm we have reached a shared understanding";
   grove's planning leaf legitimately grows the tree and writes ADRs, so the
   gate should be about not *committing decisions / growing the tree* before
   confirmed shared understanding — pick wording that fits the
   `<what-to-do>` block).
3. **G3 — provenance refresh + own-the-fusion note.** Update the header: fresh
   pin `d574778`; content now derives from **two** upstream files
   (`skills/productivity/grilling/SKILL.md` +
   `skills/engineering/domain-modeling/SKILL.md`); add the one-line
   "intentionally fused — upstream split them; grove has no skill-to-skill
   invocation, so the split would be cosmetic" note
   (per `~/Development/skills/docs/research/grove-recommendations.md §8`).
4. **Δ4 (cosmetic)** — append to the one-at-a-time rule:
   "Asking multiple questions at once is bewildering."

## Done when

`content/grilling.md` carries all four edits; nothing else in the file
regresses (the domain-awareness half and the `linkuistics:decision-records`
ADR deferral stay exactly as they are — deliberate grove divergences).

## Notes
