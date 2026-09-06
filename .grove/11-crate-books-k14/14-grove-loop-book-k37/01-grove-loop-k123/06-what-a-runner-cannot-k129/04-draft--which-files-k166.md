# which-files-k166

## Goal

Draft chapter 18 of the `grove-loop` book, *Which files take part* —
`crates/grove-loop/src/session_config.rs` lines 1–358, one whole root — to a
valid slice through `whose-file-and-whether`.

## Context

- **The rule: everything a template *is* belongs to `keyed-launch`; what is left
  here is whose file, and whether the second one is admissible.** The personal
  file's path, the two roots the delta is searched at, `DeltaRoots`,
  `TemplateSource`, the four slots grove's templates are written against, and the
  refusal of a **tracked** delta — a question about grove's worktree, answered
  through grove's version-control seam, and the boundary between an untrusted
  repository and arbitrary code execution.
- At 44% comment prose by the structure brief's count the charter is **do not
  restate**: the comments argue and the fragment graph quotes them verbatim.
  Prose connects arguments across items, names the test, and stops. If you take
  your own count, say which rule you counted under — chapter 15 recorded a
  one-point disagreement with the brief's percentages and no leaf was cut for it.
- **This chapter owns the stale sentence and must adjudicate it, not correct it.**
  `session_config.rs` line 89, in `TemplateSource`'s doc comment, says the loop
  re-reads the configuration once per iteration. `loop_driver.rs` calls
  `templates.load(&delta_roots)` **twice** — line 241 before
  `transition_to_current` and line 260 after the leaf is selected. The type's own
  doc comment names both reads in the same sentence that says *once*, so the
  count is stale rather than the design. State it beside the fragment that
  reproduces the bytes; `template-source-read-count-k86` holds the fix and lands
  after this book. Chapter 20 owns the two calls and shows the refutation.
- Pinned by `the_four_slots_are_the_vocabulary_and_prompt_is_the_required_one`,
  `a_snapshotted_jj_delta_is_refused_in_both_jj_shapes`,
  `a_trackedness_probe_that_cannot_be_completed_fails_closed` and
  `a_grove_configuration_conforms_to_the_runners_own_kit` — all in
  `crates/grove-loop/tests/`, which is evidence and not corpus, so **read each
  test's fixture and its helpers** before writing what it pins. A cited test can
  pin a narrower claim than its citation.
- The chapter keeps `complete-session-configuration`, named in prose and never
  linked. It declares no anchor of its own.
- No `docs/ARCHITECTURE.md` residue marker is assigned to this chapter.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  whose-file-and-whether --check all` is valid at 9,673 resolved lines, 860
  deferred, `final=false`.
- The `whose-file` ownership row reads `resolved`, the chapter-1 cast row owned
  by `whose-file-and-whether` reads `explained`; contents and navigation updated;
  `scripts/check.sh` red on `book-check` alone, and this file says so.

## Notes

**The corpus is frozen.** The stale sentence is adjudicated on the page and not
edited.

## Decisions (running log)
