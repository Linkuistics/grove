# ravel-book-inside-k307 — brief

## Goal

Draft chapters 7, 8 and 9 of the Ravel book for 2.0.0. They are the parts of
Ravel, the walk through the code, and the jobs, signals and terminal.

## Context

- The paused charter, read only: `Ravel.gh-issue-2/.grove/20-k34/_rhai-book-inside.md`
  and its first draft leaf `01-draft--rhai-book-inside-k35.md`. They give the
  chapter goals, the figures to check and the owner's Read 1 patterns.
- Ravel `docs/book/contract.md`, sections 1 and 2, and
  `docs/documentation-lessons.md`, including F9 to F18.
- Source of truth: the code on `skill-directory-delivery`,
  `docs/specs/configuration-evaluation.md`, `docs/specs/skill-discovery.md`
  and `CONTEXT.md`. Check each claim against the source and the running
  binary.

## Done when

- Chapters 7 to 9 describe 2.0.0: the Rhai worker, the host bridge, the skill
  directories and the source layout. They name no Bun worker, no SDK and no
  `libexec` layout, except in a statement of what was removed.
- Chapter 8 follows a session through the code that exists now. Each excerpt
  is exact, and `task docs:excerpts` passes.
- The draft checks the claim that chapter 9 changes least.
- The chapters follow the Read 1 patterns of section 2 of the contract.
- `task docs:examples` and `task docs:ste` pass for these chapters. Figure
  needs for the art step are listed.

## Notes

Work in `Ravel.skill-directory-delivery` on `skill-directory-delivery`. The
paused workstream's own drafts did not run. Do not edit or run its leaves.

## Decomposition

The draft of 2026-11-11 found the work too large for one session and split it
by chapter. Each child drafts one chapter, in this order:

1. `ravel-book-inside-k329`: chapter 7, the parts of Ravel. It sets the
   words and the concrete example that chapters 8 and 9 restate.
2. `ravel-book-inside-k330`: chapter 8, the walk through the code. Its
   evaluation section names functions that the Rhai path no longer calls.
3. `ravel-book-inside-k331`: chapter 9, jobs, signals and the terminal. It
   checks the claim that chapter 9 changes least. It comes last because it
   links to headings of chapter 8.

The copy-edit leaf `ravel-book-inside-k309` already stands under the parent
node. Thus, no child cuts a copy-edit leaf.
