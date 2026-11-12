# ravel-book-inside-k311

## Goal

Read the whole Ravel book, Parts I to IV, as one reader before the 2.0.0 cut.

## Context

- Ravel `docs/book/contract.md`, all sections.
- The paused charter's whole-book criteria, read only:
  `Ravel.gh-issue-2/.grove/20-k34/_rhai-book-inside.md`.

## Done when

- The proof finds no stale TypeScript, Bun, SDK or `libexec` statement, no
  broken link and no unresolved term in any of the eleven chapters.
- It checks each Read 1 pattern of section 2 in all eleven chapters.
- The version placeholder `@RAVEL_BOOK_VERSION@` stays for the release to
  replace.
- `task docs:book:build`, `task docs:examples`, `task docs:excerpts`,
  `task docs:figures` and `task docs:ste` pass.
- The result states that this proof does not replace the owner's Read 1 or
  Read 2, which stay with the paused workstream.

## Notes

Work in `Ravel.skill-directory-delivery` on `skill-directory-delivery`.

## Decisions (running log)

- The proof uses the completed art commit `7d9526e9` in the Ravel delivery workspace. Both working copies started empty. The paused Rhai workstream stays read only, and its owner Read 1 and Read 2 remain open.
- The baseline gates pass: book build, 27 complete examples, 22 excerpts, 21 figures and STE with no problems. These gates do not establish correctness of an example under Homebrew defaults.
- The proof spends its one independent reviewer on cross-chapter consistency and every Read 1 pattern in all eleven chapters. The parent owns the handed-forward cases and source/installation/link checks.
- The versioned-first release requires the reader route to install `ravel@2` with `--skip-link` and put its `bin` first on the example shell's PATH. This selects the Rhai executable without changing Homebrew's shared command.
- Chapter 3 needs an explicit settings file with an empty skill-source array. An empty HOME alone does not disable the installation's default directory. The example will declare this input so the printed prompt remains reproducible.
- A scratch installed prefix with a valid default source exposed eight failing examples in chapters 4 and 5. The other 19 passed. The first fixture attempt had an invalid source layout and is not evidence about the examples. All session examples will now declare empty skill sources in their private settings, with setup and file inventories reconciled. The retired-format example runs before source resolution and needs no change.
- The independent review found actionable summary omissions for skill delivery, an absolute worker-deadline claim, local term definitions and check logs inside the book. The fixes preserve the teaching structure. Detailed run records stay in the existing evidence document, as the contract requires. Obsolete contract facts about the worker and the first Pages release are factual updates, not changes to the owner's structure or checkpoint agreement.
- All independent findings were valid and actionable. Prompt summaries, skill-aware environment and exit-status summaries, descriptor retention, the diagnostic inventory, terminal recovery qualifiers and local definitions now agree across the chapters. The example-input correction is covered by the installed-prefix executable seam. No second review context was used.
- Final verification passes: book build with no warning, 27 examples in both the checkout and installed-prefix runs, 22 excerpts, 21 figures, zero STE problems and no long prose runs. The two example runs agree item by item. The 288 frozen source inputs stayed unchanged across both runs.
- Link verification accepts 1,173 local chapter references, 16 linked repository-file targets and 17 external destinations. Planted missing-page, anchor and figure controls fail as intended. The version placeholders remain. Public rendering of the unreleased revision remains a publication check.
- The isolated Linux Firefox inspection displays the revised chapter 3 installation procedure and first-session settings table without clipping. The guest is stopped. This targeted view adds no reader-comprehension evidence and does not replace the completed art inspection.
- All five proof handoffs are closed and removed from the parent brief. The factual contract updates preserve the owner's structure and reader checkpoints. No correction leaf is needed. The parent remains live through `ravel-main-landing-k312` and `ravel-two-publication-k313`, so no parent closes in this retirement.
- Durable proof evidence is `Ravel/docs/evidence/ravel-book-k311/README.md`. Lesson L75 now records the eight installed-prefix counterexamples and the explicit-settings correction. This proof does not replace the owner's Read 1 or Read 2 in the paused workstream.
- Ravel commit `424af53d3f0c` seals the proof and its evidence. The local `skill-directory-delivery` bookmark names it, and the next Ravel working copy is empty. Publication and main integration remain with the following leaves.
