# skill-integrate-k4

**Kind:** integrate-review-impl

## Goal

Apply the findings from `skill-review-k3` to
`plugins/linkuistics/skills/using-codebase-memory/SKILL.md`, leaving the file in
a state where every claim it makes is one the reviewer's own commands reproduce.

## Context

- `.grove/03-DONE-skill-review-k3.md` § Findings — the input.
- The skill file itself.
- `plugins/linkuistics/skills/authoring-conventions/SKILL.md` if any finding is
  about frontmatter or description shape.

## Done when

- Every **refuting** finding is resolved: the prose is corrected, or the claim
  is deleted, or — where the fact is real but not testable here — it is marked
  `UNVERIFIED` per the house source-citation convention.
- Each finding is dispositioned. A finding deliberately **not** acted on gets a
  sentence saying why, appended to the review leaf's `## Findings` section.
- Every command in the corrected file is run **once more**, after the edits.
  Corrections are the most likely place for a new false claim to enter.
- Committed with `jj describe` / `jj new`.

## Notes

Findings are not orders — read them with the same scepticism the reviewer was
asked to bring. A finding may itself be wrong, and disagreeing with one *in
writing* is a legitimate disposition. What is not legitimate is silently
dropping one.

If a finding is out of scope for this file — a defect in the spec, the plan, the
glossary, or `codebase-memory-mcp` itself — do not absorb it. Externalize it with
`grove-llm leaf-add`, or raise it with the human if it is an abandonment
question.
