# ravel-book-inside-k331

## Goal

Draft chapter 9 of the Ravel book for 2.0.0, “Jobs, signals and the
terminal”. Check the claim that it changes least, and change what 2.0.0
changed.

## Context

- Read the node brief `_ravel-book-inside.md` and its pointers. The structure
  brief is Ravel `docs/book/contract.md`, sections 1 and 2.
- The paused charter says that the runner is unchanged, so chapter 9 changes
  least. Test that claim. Compare `crates/keyed-launch/` at the revision of
  the last thorough revision of chapter 9 with the current code. Then read
  each statement of chapter 9 that names the worker, the CLI's signal repair
  or the configuration-output reader threads against the Rhai code.
- The paused leaf `rhai-book-inside-k35` asked for two subjects. One is the
  ordered stop with a harness run in progress. The other is the precedence of
  a signal over the selection deadline. The harness runs are in
  `crates/ravel/src/rhai_worker/harness.rs`.
- Chapter 7 and chapter 8, as drafted by `ravel-book-inside-k329` and
  `ravel-book-inside-k330`, set the words and the example. Restate them.

## Done when

- Chapter 9 describes 2.0.0. The running log states the check of the
  “changes least” claim and its result.
- `task docs:examples`, `task docs:ste` and `task docs:breaks` pass for
  chapter 9. Its links to chapters 7 and 8 resolve.
- The chapter follows each Read 1 pattern of contract section 2.
- Figure needs for the art step are in the parent node brief's
  `## Handed forward`.

## Notes

Work in `Ravel.skill-directory-delivery` on `skill-directory-delivery`.
