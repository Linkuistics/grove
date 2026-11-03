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

## Decisions (running log)

- **Chapter 2's section is "Trust limits".** Each link to
  `#trust-and-confinement` now names `#trust-limits`. The section states that
  the harness sandbox is the only boundary, and that 2.0.0 refuses both flags.
- **Chapter 5's confinement section becomes "Know what the managed process
  receives".** It holds the filesystem rule, the refusal, the environment,
  the channel placement by `--exit-dir` with `exit_dir_unusable`, and the
  terminal lending of an ordinary launch. Each link to
  `#confine-the-managed-process` names the new anchor.
- **The source overlap moves to "Give the agent the owner's skill
  directories".** A table of the four protected paths follows the
  `skill_access_unavailable` refusal. The table also keeps the prose run
  under the 450-word limit of `task docs:breaks`.
- **Chapter 9's "Separate noninteractive execution from confinement" becomes
  "Separate noninteractive execution from an ordinary launch".** No link
  named either chapter 9 anchor. The runner test table moves to the end of
  that section, without `confinement.rs` and the confined cancellation.
- **Prose runs that already broke `task docs:breaks` get tables.** Chapter
  6 had two runs over 450 words before this leaf. Three tables now break
  them: the cache directory conditions, the signal statuses of
  `skill_resolution_cancelled`, and the places of a prepared launch.
  Chapters 7 and 10 had STE problems before this leaf, and they are fixed.
- **`result-job-completion.md` converts five sections.** "One completion
  decision", "Acquisition and entry limits", "Cleanup after partial
  acquisition", the prose of "Outcome and diagnostic order" and "Boundary
  criterion". The "prepared private workspace" of "Acquisition and entry
  limits" was the optional confined workspace, as
  `result-job-source-evidence.md` names it. Pending markers still cover the
  untouched prose.
- **Frozen records keep their confinement text.** The evidence under
  `docs/evidence/`, `component-evidence.md`, `runtime-evidence.md`,
  `documentation-lessons.md`, the dated inspection in `figures.md` and the
  measured trace table of `native-entry-transition-design.md` record past
  revisions. The released changelog entries also stay.
