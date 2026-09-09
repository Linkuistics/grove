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
