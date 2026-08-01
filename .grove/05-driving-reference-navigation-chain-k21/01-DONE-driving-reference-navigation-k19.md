# driving-reference-navigation-k19

**Kind:** impl

## Goal

Make the long Grove field guide navigable under the skill authoring
progressive-disclosure convention.

## Context

`content/driving.md` is a roughly 650-line reference linked from
`content/SKILL.md` and has no table of contents. The house authoring convention
recommends navigation once a reference exceeds roughly 300 lines. This surfaced
during `composition-guidance-k17` and is unrelated to its review contract.

## Done when

- `content/driving.md` has a compact table of contents that reflects its useful
  conceptual sections without becoming a second outline of the methodology.
- Anchor links render correctly and a focused test or lint protects them from
  obvious drift.

## Notes

Keep the reference one level deep and avoid changing section content merely to
make the navigation prettier.
