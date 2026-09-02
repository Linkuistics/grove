# three-steps-k79

## Goal

Draft chapter 3 of the overview: slice `one-call`, `03-three-steps.md`, owning
`entry-point-three-steps` (`crates/grove/src/main.rs` 1–13) and
`surface-resolve-lease-run` (`crates/grove/src/cli.rs` 20–53).

## Context

- Draft stage, child 3 of 5 of `overview-k76`. Responsibilities are the
  structure brief's *3 · Three steps* section: the three things the loop cannot
  do for itself; the workspace resolved **once** here and handed to both the
  lease and the loop, and what that seam replaced (`loop-crate-driver-k22`,
  `docs/adr/one-live-driver-per-working-tree.md` — evidence, cited by path);
  the shape of one foreground iteration; and the signal path, which is the
  chapter's centre of gravity.
- The required example anchor is `worked-run`: the carried invocation at full
  resolution — `current_dir`, `Workspace::resolve`, `DriverLease::acquire`,
  `TemplateSource::from_env`, `grove_loop::run` — and both endings. `Finished`
  and `Stopped` reach `Ok(())`; `Interrupted(SIGTERM)` reaches `reraise`, and
  whoever started `grove` reads a wait status of `128 + 15`. The two error
  endings `run`'s doc comment names are the same trace stopping earlier.
  `docs/USAGE.md`'s *Stopping the loop* transcript shows `143`; the loop
  fixtures in `crates/grove/tests/loop_driver.rs` and `lifecycle_cutover.rs`
  are the tests that prove the reraise. Signal semantics are stated once, as
  the chapter's premise, with `128 + N` named rather than derived.
- This chapter **owns** all five early-use rows: flip each to `explained` and
  supply the full explanation at the site that reads it. `Workspace` is
  `jj-workspace`'s type re-exported at `crates/grove-loop/src/lib.rs`, and
  `reraise` is `keyed-launch`'s; name that without explaining either crate.
- Citations placed by the brief: `docs/USAGE.md#usage-driver-lease` beside
  `DriverLease::acquire`, `docs/USAGE.md#usage-session-lifecycle` beside
  `grove_loop::run`; glossary `driver-lease`, `stated-vcs` beside
  `Workspace::resolve`, and `loop-control-channel` beside the completion signal.
- `main.rs` is owned here rather than in chapter 1 because its module
  documentation is the three-steps argument in miniature (brief, *The mapping
  onto the corpus*).
- Leave the page shaped to receive *Runtime flow* at `architecture-move-k31`;
  move nothing.

## Done when

- Both blocks' fragments are defined on the page, both defers are replaced by
  inserts, both ownership rows read `resolved`, the fragment index has the rows,
  and all five early-use rows read `explained`.
- Contents, navigation and the concept index are updated.
- `book-check --through one-call --check all` is valid: 120 resolved lines, 84
  deferred. The repository Markdown sweep passes. `scripts/check.sh` stays red
  on `book-check` alone, by design.
