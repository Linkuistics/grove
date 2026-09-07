# env-guard-set-var-soundness-k185

## Goal

Decide, and record, what `crates/jj-workspace/tests/environment.rs`'s `EnvGuard`
does about `std::env::set_var` being unsound in a multi-threaded process — before
an edition-2024 migration forces the question as a compile error.

## Context

- `EnvGuard::set` and its `Drop` call `std::env::set_var` / `remove_var`
  (`crates/jj-workspace/tests/environment.rs:43-58`). The workspace is on
  `edition = "2021"` (`Cargo.toml:71`), where both are safe fns, so this compiles
  today.
- The file's header comment (lines 3-8) argues only that a **separate test binary**
  keeps the mutation from leaking into unrelated tests. That is a different
  hazard, and the comment does not claim to address this one. The process still
  has the harness's main thread alongside the test thread, which is the condition
  `set_var` is unsound under and the reason Rust 2024 made both `unsafe`.
- The restore itself is not the problem: `Drop` runs on unwind, and `env` is
  declared after `tmp`, so the guard restores before the `TempDir` is removed. The
  hazard is the mutation, not the cleanup.
- `Cargo.toml:84-86` already carries an edition-2024 tripwire for pinned
  dependencies, so the migration is a live concern in this workspace rather than a
  hypothetical one.
- Surfaced by the adversarial read commissioned inside `env-selector-coverage-k68`
  (finding 9), which classified it as pre-existing and out of that leaf's scope.

## Done when

- The file states what it relies on, or stops relying on it. Either is a result:
  a comment arguing the mutation is sound *here* and why, or a change that removes
  the mutation — passing the environment to the child rather than setting it in
  the parent is the obvious alternative, but the crate builds every `Command`
  behind its own seam, so establish whether a test can reach that at all before
  assuming it can.
- Whatever is chosen survives an edition-2024 `cargo check` in the tripwire's
  spirit, or the file says explicitly what the migration will have to do.
- The test's two controls — the colocated fixture and the pinned `JJ_CONFIG` —
  still hold, and the mutation matrix in `k68`'s log still reproduces:
  dropping `GIT_INDEX_FILE` from `REPOSITORY_SELECTORS` fails this test, dropping
  any other member does not.
- `bash scripts/check.sh` passes.

## Notes

**Check whether any page has to move.** `tests/` is evidence rather than a book
root, so no fragment or ledger is at risk — but chapter 3 of the `jj-workspace`
book describes this test's fixture and both of its controls, and chapter 7's
residue table carries a row about the selector array. If the fix changes what
those sentences describe, the page corrections land in the same commit.
