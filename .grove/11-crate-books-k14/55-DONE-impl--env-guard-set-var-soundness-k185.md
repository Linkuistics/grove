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

## Decisions (running log)

- **The mutation is removed, not argued for.** The Goal offered either result;
  the comment option was rejected because there is no argument to write. std's
  own rule is absolute rather than conditional — *"In multi-threaded programs on
  other operating systems, the only sound option is to not use `set_var` or
  `remove_var` at all"*, the requirement being that no other thread even *reads*
  the environment and no library advertising which of its functions do
  (<https://doc.rust-lang.org/std/env/fn.set_var.html>). The binary is not
  single-threaded: the harness runs a test on a thread it spawns unless asked
  for `--test-threads=1`, which a test file cannot ask for. A comment claiming
  soundness *here* would have had to assert something about libtest's own thread
  that neither libtest nor std guarantees.
- **The alternative the Context named is unreachable at the seam it names, and
  reachable one level up.** `jj::raw_output` is `pub(crate)`, so an integration
  test cannot hand an environment to the crate's `jj` child; and reaching it
  would test the wrong property anyway, since the claim is about what the seam
  *removes* from an inherited environment rather than what a caller could add.
  What is reachable is making the **test binary** the child: the parent builds
  the fixture, hands the six variables to `Command::env`, and re-runs itself
  with `--exact`. `crates/keyed-launch/tests/reraise.rs` is the same pattern in
  this repository, for the same reason — a property of a *process* can only be
  asserted by starting one. `EnvGuard` is deleted; nothing is saved because
  nothing is mutated.
- **The split needs a control of its own, and it was missing.** A libtest binary
  whose filter matches nothing reports `0 passed` and exits `0`, so a drifted
  test name would leave the parent asserting success over a child that ran no
  test. The first draft's comment claimed the opposite — that a rename would
  fail hard — and that was false. The parent now asserts that `intended/src/f.txt`
  exists after the child returns: the child writes it and nothing else does,
  immediately before the two assertions that carry the claim. Seen to fail:
  renaming the `TEST` constant to `a_name_no_test_has` turns the test red at that
  assertion.
- **Edition 2024 needs nothing from this file.** `Command::env` writes the
  child's environment map and never the parent's, so there is no `set_var`,
  `remove_var` or `unsafe` left to migrate. `set_var`/`remove_var` are `unsafe`
  "starting in the 2024 Edition, while not requiring `unsafe` in previous
  editions"
  (<https://doc.rust-lang.org/edition-guide/rust-2024/newly-unsafe-functions.html>),
  which is why `edition = "2021"` compiled the earlier version at all.
- **Both controls and the whole mutation matrix reproduce** against the
  restructured test, one mutation at a time, restoring between each. Colocated
  fixture and pinned `JJ_CONFIG` unchanged and still in the child's environment.
  Dropping `GIT_INDEX_FILE` from the scrub turns the test **red**, and the
  child's first failing assertion is `intended/.git/index` being absent — the
  ordering the in-file comment claims. Dropping `GIT_DIR`, `GIT_WORK_TREE` or
  `GIT_COMMON_DIR` individually leaves it **green**. Test count unchanged: one
  test in `tests/environment.rs`, thirty-two in the crate.
- **No page moves.** `tests/environment.rs` is outside the corpus
  (`docs/specs/jj-workspace-book-structure.md`: *no chapter owns it and no
  fragment*), so no ledger or fragment is at risk. The three sentences that
  describe it — `02-the-gate.md` (*sets all four*), `03-subprocess-seam.md`
  (*exercises exactly one of the four call sites*, and *sets all four to a
  colocated foreign repository* … *pinning it from the parent*) — describe what
  the test sets and what its controls are, never how the variables get into the
  process, and all remain true of the child's environment. Chapter 7's residue
  row and its `1 passed` / `31 passed` console transcript are unchanged because
  the test count is. Verified by enumerating every mention of `environment.rs`,
  `EnvGuard` and `set_var` across `docs/` and `crates/`.
- **A sibling instance is out of this leaf's scope and is now its own leaf.**
  Widening the enumeration past the task file's path scope found a second
  `EnvGuard` with the same hazard at `testing/support.rs:266,292` — a file under
  no crate, which a `crates/`-scoped grep cannot reach. It is compiled into
  `grove`, `grove-llm` and `grove-loop` test binaries via `#[path]`, so unlike
  this one it mutates the environment of binaries that really do run many tests
  concurrently. Cut as `testing-support-env-guard-soundness-k203`, which also
  carries the sharper finding the enumeration produced: `EnvGuard` is
  constructed **nowhere** — `support.rs` opens with `#![allow(dead_code)]`, so
  nothing warned — and that leaf may be a deletion rather than a redesign. The
  re-exec answer here does not obviously transfer if it is not: that guard is
  general and its `clear_grove_env` removes a computed list.
- **The pinned `JJ_CONFIG` control was seen to fail, which is what proves it
  survives the re-exec.** The restructure moved the pin from a `set_var` in this
  process to a `Command::env` on the child, and a pin that no longer reached jj
  would leave the test green and the control inert — indistinguishable from a
  working one. Rewriting the fixture's `auto-track = "all()"` to `"none()"` turns
  the test **red**, the child failing at `is_tracked`. The pin reaches the
  grandchild because the seam deliberately does not scrub `JJ_*`.
- **Edition 2024 was measured, not asserted.** With `edition.workspace = true`
  replaced by `edition = "2024"` in `crates/jj-workspace/Cargo.toml`,
  `cargo check --locked -p jj-workspace --all-targets` finishes clean — no
  errors and no warnings — on rustc 1.98.0. Manifest restored.
- **The colocated fixture's `--colocate` is inert on jj 0.45.1, and chapter 3's
  account of that control is false.** Cut as
  `colocated-fixture-control-claim-k204`. Removing `--colocate` and re-running
  left the test **green**, which looked like the control failing to be a
  control — but the flag is a no-op: `jj git init --help` says colocation is
  *"the default, and this option has no effect, unless the `git.colocate` config
  is set to `false`"*. The real lever is `--no-colocate`, and under it the test
  is **red** both with the scrub intact and with `GIT_INDEX_FILE` dropped,
  failing at `intended/.git/index`. So `03-subprocess-seam.md`'s *"the test
  would pass whether or not the scrub existed"* is false: a non-colocated
  fixture makes the test vacuous by turning it permanently **red**, not
  permanently green — `env-selector-coverage-k68`'s index assertion presupposes
  colocation. Both findings predate this leaf and neither is caused by its
  change, so both went to k204 rather than inline. This leaf's own claim is
  unaffected: the fixture is still colocated and the pin still reaches jj.
