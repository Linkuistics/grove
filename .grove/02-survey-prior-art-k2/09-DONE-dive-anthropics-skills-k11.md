# dive-anthropics-skills-k11

**Kind:** work

## Goal

Deep-dive **anthropics/skills** (https://github.com/anthropics/skills) as a
survey source — **scoped to the authoring layer**: the Agent Skills `spec/`, the
`template/`, and the `skill-creator` skill. This is the authoritative
authoring/packaging reference for our `SKILL.md` format. End with a
**takeaway-for-skills** and a **takeaway-for-grove** (likely "none" for grove).

## Context

- Shortlist rank #8 (`docs/research/skill-repo-prior-art.md` §1a). Verified
  154,814★ (GitHub API, 2026-06-25). The canonical Agent Skills repo.
- Read the node `BRIEF.md` for downstream questions + discipline; `CONTEXT.md`
  for the target split.

## Done when

- A `## anthropics/skills` section is appended to
  `docs/research/skill-repo-prior-art.md` with cited findings, each tagged
  **target** + walk-away note, ending with takeaway-for-skills /
  takeaway-for-grove.

## Notes

- Focus — **skills Q2** ONLY: read `spec/` (frontmatter fields, progressive
  disclosure, `references/` splitting, path-based auto-load), `template/`, and
  `skill-creator`. Compare against how *our* 9 skills are authored — what
  convention should we adopt or where do we already comply?
- **Scope discipline:** SKIP the document/design domain skills (docx, pptx,
  xlsx, pdf, canvas-design, etc.) — they are out of scope for our coding-craft
  marketplace. If a domain skill is genuinely tempting, note it for synthesis,
  don't dive it here.
