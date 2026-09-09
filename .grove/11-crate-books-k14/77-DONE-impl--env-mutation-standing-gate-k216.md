# env-mutation-standing-gate-k216

## Goal

Make "nothing in this workspace mutates its own process environment" a checked
invariant rather than a comment. `testing-support-env-guard-soundness-k203`
removed the last five `std::env::set_var` / `remove_var` call sites and left the
rule stated in prose at `testing/support.rs`'s header and in
`crates/grove/tests/env_hygiene.rs`. Under `edition = "2021"` both functions are
safe fns, so nothing fails when the next author adds one back.

## Context

- k203 measured the end state: a workspace copy with `[workspace.package]
  edition` flipped to `2024` passes `cargo check --workspace --all-targets`
  after one unrelated fix (`E0515` at `crates/ordinal-fs-tree/src/plan.rs:219`,
  discharged by `+ use<'a, N>` on `snapshot.rs`'s `children` — Rust 2024's
  widened RPIT capture rules, nothing to do with the environment). Re-adding one
  `set_var` and one `remove_var` to `testing/support.rs` turns that green run
  into seven `E0133`s. So the compiler *will* eventually hold this — the gate is
  what covers the interval until the migration, and the migration is a leaf this
  campaign has not cut.
- The check itself is small: walk every `.rs` file outside `.git`, `.jj`,
  `.grove`, `.grove-worktrees` and `target`, skip whole-line comments (the rule
  is explained in prose in at least three files and an explanation is not a
  call), and report every remaining line containing `set_var` or `remove_var`.
  It needs two controls or it is decorative: a fixture string carrying both
  forms plus a commented one, asserted to yield exactly the two live lines; and
  an assertion that the walk reached named files, since a truncated enumeration
  agrees with any claim. `crates/grove/tests/reference_navigation.rs` has the
  walker to copy (`collect_files` / `repository_files`, and its
  `UNSWEPT_DIRECTORIES` and `THIS_FILE` idioms).
- **`crates/grove/tests/env_hygiene.rs` is where it belongs and why this is its
  own leaf.** That file's header already says the guards get assertions rather
  than trust, because their failure mode is invisible from inside the suite —
  which is exactly this hazard's shape too. But a fifth `#[test]` there is a
  fifth test in a control two books publish figures from, and k203 declined to
  drag that repair into a deletion. The whole of the extra work is below.

## The pages a fifth test in that file moves

Enumerated at k203 over `docs/`, not assumed. Confirm each against the file as
it then stands; the campaign rule is that **one commit carries the source
change, every affected page, and a green validator run over every book it
touched**.

- `docs/walkthroughs/overview/05-what-the-call-reaches.md` — a `cargo test
  --locked -p grove` console transcript showing `env_hygiene.rs` at `running 4
  tests` / `4 passed` (two lines, ~line 432), and the prose after it counting
  "the sixty-seven integration tests under `tests/`". Both are current-state
  claims about this crate's suite and both move.
- `docs/walkthroughs/grove-loop/20-the-loop.md:154` — "`env_hygiene.rs` with
  four", in the list of `crates/grove/tests/` observers that argues why that
  chapter's control needed a third package. Current state; moves.
- **Three `626` figures that probably do *not* move, and the leaf owes a
  verdict rather than an edit.** `20-the-loop.md:168-174` ("626 test runs, 616
  passing and 10 failing", "626 runs are 625 distinct names"),
  `10-growing.md:1375` ("42 test binaries and 626 tests in all"), and
  `19-the-core.md:147` ("Chapter 10, whose copy was scoped differently — 626
  tests over three crates"). Each reads as a record of a run that was taken,
  and chapter 10's says so outright — "the workspace as it stood when the
  mutation ran, one test smaller than it is now". A record of a measurement is
  not falsified by later work; a sentence asserting a present total is. Read
  each in its own paragraph and decide which it is. Getting this wrong in the
  *safe* direction — editing a frozen record to a number no run produced — is
  the worse error, because it invents a measurement.
- No fragment or ledger row is at risk: `testing/` is under no crate, and
  `crates/*/tests/` is evidence rather than corpus. The pages above are prose
  and a transcript.

## Done when

- A test in `crates/grove/tests/env_hygiene.rs` fails on any `set_var` or
  `remove_var` in a first-party `.rs` file, with both controls above, and its
  message names the remedy — `Command::env` / `Command::env_remove` for a child,
  and re-running the test binary as that child where the property can only be
  *observed* under an environment (`crates/jj-workspace/tests/environment.rs` is
  the worked pattern).
- Its failure message is checked by making it fail once, not by reading it.
- `testing/support.rs`'s header and `env_hygiene.rs`'s doc comment stop saying
  nothing scans for a re-introduction, and stop pointing at this leaf.
- Every page in the list above is either corrected or has a stated reason for
  being left, and `bash scripts/check.sh` passes — which includes `book-check`
  over every book.

## Notes

**The gate is not the reason the mutation is gone.** k203 deleted `EnvGuard`
because nothing constructed it and because `set_var` has no sound use in a
process whose other threads read the environment
(<https://doc.rust-lang.org/std/env/fn.set_var.html>) — libtest runs every test
on a thread it spawns. This leaf adds no argument to that; it only stops the
argument from having to be rediscovered.

**Do not reinstate an environment lock alongside it.** k203 also deleted
`lock_env` and both `ENV_LOCK` statics. One of them had a single lock site in
its whole binary and therefore serialised nothing; the other serialised two
tests against a mutation that no longer existed. A mutex is not a remedy for
`set_var` in any case — the unsoundness is a data race against *reads* that no
library advertises, not against other writers.

## Decisions (running log)

- **One `#[test]`, not two.** Both controls live inside
  `no_first_party_source_mutates_its_own_process_environment`, with the fixture
  asserted **first** so a matcher that has stopped matching fails at the control
  rather than passing the scan in silence. A second `#[test]` would have made
  the file's count 4→6 and falsified the pages differently from the arithmetic
  this leaf was priced on; it also matches the file's own stated idiom for
  `both_guards_are_present_and_neither_subsumes_the_other` — a pair asserted from
  one test so a reviewer sees it stated together.
- **The matcher is a pure `fn environment_mutation_lines(&str) -> Vec<usize>`.**
  That is what makes the fixture a control at all: a walk-and-assert written as
  one body can only be falsified by planting a violation in the tree. Whole-line
  comments (`//`, `///`, `//!`) are skipped; a trailing comment on a line of code
  is deliberately not, so the scan errs towards reporting.
- **Failure message checked by making it fail, red then green.** A live
  `set_var` and `remove_var` were appended to `testing/support.rs`; the test
  failed naming `testing/support.rs:507` and `:508` and printed the remedy
  (`Command::env` / `Command::env_remove`, and re-running the test binary as the
  child per `crates/jj-workspace/tests/environment.rs`). Reverted; green again.
  Both halves observed, not inferred.
- **`edition = "2021"` verified at `Cargo.toml:89`**, not recalled — it is the
  fact that decides whether this gate is needed at all.
- **The three `626` figures do not move.** Each is a record of a run that was
  taken, not an assertion of a present total: `20-the-loop.md:168-174` reports
  what that chapter's own control run printed, `10-growing.md:1375` says outright
  that it is "the workspace as it stood when the mutation ran", and
  `19-the-core.md:147` cites chapter 10's copy. Editing any of them to a number
  no run produced would invent a measurement.
- **But `10-growing.md`'s relative clause did move, and the task file did not
  spot it.** "…one test smaller than it is now" is a comparison against the
  present, sitting inside a frozen record. Measured rather than derived: `cargo
  test --locked --no-fail-fast -p grove-loop -p grove-llm -p grove` now reports
  **629 tests over 42 binaries, 246 of them `grove-loop`'s inline module** — so
  the record's 626/245 is **three** tests smaller, not one. Two of those three
  predate this leaf. Corrected to "three tests smaller than it is now"; the
  626 and the 245 are untouched.
- **A fourth record the task file did not enumerate:** `20-the-loop.md:1798`'s
  control of "**277 tests with no failures at all**" is `-p grove-loop --lib`
  plus `-p grove` over `env_hygiene`, `lifecycle_cutover` and `loop_driver` — so
  it counts this file. Same class as the 626s: a record of the run each row below
  it was diffed against, with no relative clause. Not moved.
- **`20-the-loop.md:154` moved: "`env_hygiene.rs` with four" → "with five".**
  That list is a current-state argument about what
  `crates/grove/tests/` holds, not a report of a run.
- **The overview's `cargo test --locked -p grove` transcript was stale beyond
  this leaf, so it was regenerated whole rather than patched.**
  `corpus_exception_inventory.rs` read 5 and runs 3;
  `reference_navigation.rs` read 12 and runs 13; `env_hygiene.rs` read 4 and
  now runs 5. Patching only the third line would have left a transcript no run
  ever produced — the same error class as editing a frozen record — so the whole
  block is one real run, timings included.
- **"the sixty-seven integration tests" did *not* move, contrary to the task
  file's expectation.** The three drifts cancel exactly (−2, +1, +1): the crate
  totals 69 tests, two of them the unit tests inside the corpus, leaving 67. The
  count is right for a reason the page does not state, which is why it was
  re-derived rather than incremented.
- **The elision note moved with the transcript**: the run elides *two* lines a
  fixture's `jj` printed (`Working copy (@) now at:` and `Parent commit (@-)`),
  not one.
- **`docs/preservation-baseline.md:482,1298` and `docs/candidate-lessons.md:42`
  are left.** The baseline is explicitly the contract "**before** the modularity
  refactor, measured rather than described" — a frozen record — and its claim
  that `env_hygiene.rs` "does **not** assert the spawned-child scrub set" is
  still true of the file after this leaf.
- **`05-what-the-call-reaches.md`'s `bash scripts/check.sh` block is left at "3
  book(s) checked".** It is an elided (`...`) record of the drafting session's
  run, and the prose beside it already explains that the set grows by discovery
  rather than from a list, so a reader today getting six is what that paragraph
  predicts. Same class as the 626s.
- **No fragment, ledger row or ADR moved.** `testing/` is under no crate and
  `crates/*/tests/` is evidence rather than corpus, as the task file anticipated;
  `bash scripts/check.sh` reports all 8 principal checks passing with 6 books
  green.
