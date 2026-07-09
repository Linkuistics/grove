# skills-authoring-enrichments-k3

**Kind:** work

## Goal

Land the five Linkuistics-side enrichments (report items L1–L5 in
`docs/research/mattpocock-skills-v1.1-incorporation.md`) in
**`~/Development/skills`** (`Linkuistics/skills`) — cross-repo work per the
one-system decision (root brief Notes; memory `grove-skills-one-system`).

## Context

The entire upstream `writing-great-skills` skill (+ its `GLOSSARY.md`)
postdates the pin the Linkuistics survey was built against. All quotes and
commit hashes are in the report. Check `~/Development/skills`' branch state
before committing; follow that repo's conventions (CHANGELOG.md has an
Unreleased section).

## Work items

1. **L1 — Negation** into
   `plugins/linkuistics/skills/authoring-conventions/SKILL.md`: one line
   generalising the existing superpowers-S3 "match the form to the failure"
   bullet — any bare prohibition names the banned behaviour into the frame;
   state the positive target; reserve `never` for guardrails you can't phrase
   positively, always paired with the positive. Cite `mattpocock`
   writing-great-skills GLOSSARY: Negation.
2. **L2 — context-load / cognitive-load / router** vocabulary into the same
   skill's user-invoked vs model-invoked ("House lever") section.
3. **L3 — sentence-level no-op hunt** (upstream `aa7ed40`): test each
   *sentence* against the no-skill default; when one fails, delete the whole
   sentence rather than trim words.
4. **L4 — survey citation refresh** in
   `docs/research/skill-repo-prior-art.md`: the `mattpocock-S*` citations
   predate `writing-great-skills`; add a dated note pointing at
   `writing-great-skills/{SKILL,GLOSSARY}.md` @ `d574778` as the current
   canonical source (don't rewrite the historical dive).
5. **L5 — design-it-twice subagent workflow** into
   `plugins/linkuistics/skills/codebase-design/SKILL.md`: the concrete
   parallel-subagent procedure (divergent briefs: minimize-interface /
   maximize-flexibility / optimize-common-caller / ports-and-adapters) that
   upstream keeps in `DESIGN-IT-TWICE.md`; Linkuistics states the principle
   only.

## Done when

One focused commit in `~/Development/skills` carrying all five edits +
CHANGELOG entry; each edit cites its upstream source per that repo's citation
discipline.

## Notes
