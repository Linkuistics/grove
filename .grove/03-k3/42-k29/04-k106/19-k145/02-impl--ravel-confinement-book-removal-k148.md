# ravel-confinement-book-removal-k148

## Goal

Make Ravel's book, `docs/specs/result-job-completion.md` and `CHANGELOG.md`
agree with the code after ravel-confinement-code-removal-k147.

## Context

- The sections "Guarantees of a confined launch", "Book" and "Specifications
  and records that change with the code" of Ravel's
  `docs/skill-discovery-design-evidence.md`.
- The commit of ravel-confinement-code-removal-k147 on the bookmark, and its
  running log. That leaf removed the `requires` directive of the example
  checker. `docs/book/examples.md`, chapter 10 and the directive in chapter
  5 still describe it.

## Done when

- Each book item of the handoff is done. The figures are regenerated with
  `task docs:figures:update`.
- `result-job-completion.md` names no confined directory, and each touched
  section is in controlled English.
- `CHANGELOG.md` holds the removal entry.
- `task docs:book:build`, `task docs:examples`, `task docs:excerpts`,
  `task docs:figures`, `task docs:ste -- --strict` on the changed chapters
  and `task docs:breaks -- --strict` on them pass.
- Outside statements of the withdrawal and frozen evidence, no book page,
  specification or record names confinement as a feature.

## Notes

Do not publish a release, push the bookmark or install `ravel@2`.
