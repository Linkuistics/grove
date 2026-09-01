# env-selector-coverage-k68

## Goal

Extend the crate's environment test so all four of the seam's repository
selectors are proved removed, rather than three of the four.

## Context

- `crates/jj-workspace/src/jj.rs:28-33` declares
  `REPOSITORY_SELECTORS: [&str; 4] = ["GIT_DIR", "GIT_WORK_TREE",
  "GIT_COMMON_DIR", "GIT_INDEX_FILE"]`, and `raw_output` calls `env_remove` for
  each before spawning the child.
- `resolution_ignores_repository_selection_and_temporary_directory_environment`
  (`crates/jj-workspace/tests/environment.rs:62`) sets `GIT_DIR`,
  `GIT_WORK_TREE`, `GIT_COMMON_DIR` and `TMPDIR` to point at a second, foreign
  workspace, then asserts that resolution from the intended tree still answers
  about the intended tree and that nothing was created in the foreign one.
  `GIT_INDEX_FILE` is the one member of the array the test never sets.
- Found while applying chapter 7's justified-subtraction test to refusal 3
  (`docs/walkthroughs/jj-workspace/07-what-jj-owns.md`, *3 · Ambient repository
  selection*), which distinguishes the mechanism being checked from the list
  being checked. The mechanism is checked — deleting the removal loop fails the
  existing test. This leaf closes the smaller of the two gaps.
- **The larger gap is not this leaf's.** Nothing goes red if jj or Git begins
  reading a fifth name, and no test can close that: it is the copy-versus-
  delegation failure the chapter names, and the argued position there is that the
  direction of error is cheap — removing a variable jj does not read costs
  nothing. Do not turn this into a discovery mechanism.

## Done when

- The environment test sets `GIT_INDEX_FILE` alongside the three selectors it
  already sets, so every member of `REPOSITORY_SELECTORS` is exercised.
- The addition is asserted rather than merely set: removing `GIT_INDEX_FILE` from
  the production array makes the test fail.
- `bash scripts/check.sh` passes.

## Notes

**This touches no frozen root.** `crates/jj-workspace/tests/` is evidence rather
than a book root (root brief, *The corpus, exactly*), so no fragment, ledger or
page moves and the cross-book one-commit rule does not bind here. It is placed
beside the other `jj-workspace` follow-ups for coherence, not because it is
blocked by them.

**Scope is one assertion.** If the work looks larger than that on arrival, the
thing that grew is the ambition rather than the defect — re-read the Context
before widening it.
