# testing-support-env-guard-soundness-k203

## Goal

Decide, and record, what `testing/support.rs`'s `EnvGuard` does about
`std::env::set_var` being unsound in a multi-threaded process — the same question
`env-guard-set-var-soundness-k185` settled for `crates/jj-workspace`, against the
harder instance. After this leaf the workspace has no `set_var` or `remove_var`
call left, or it has one whose survival is argued in the file that makes it.

## Context

- `EnvGuard::set`, `EnvGuard::remove` and its `Drop` call `std::env::set_var` /
  `remove_var` (`testing/support.rs:266`, `:274`, `:292`). Since k185 these are
  the **only** such calls in the repository — enumerated over the whole checkout,
  not over `crates/`, which is how they were missed: `testing/` is under no crate
  and a `crates/`-scoped grep cannot reach it.
- The hazard is strictly larger here than it was there. `testing/support.rs` is
  compiled by `#[path]` into the test binaries of `grove`, `grove-llm` and
  `grove-loop` (`crates/*/tests/support/mod.rs`), and those binaries run many
  tests concurrently — so a mutation is racing sibling tests' reads rather than
  only the harness thread's. std's rule admits no exception: *"In multi-threaded
  programs on other operating systems, the only sound option is to not use
  `set_var` or `remove_var` at all"*
  (<https://doc.rust-lang.org/std/env/fn.set_var.html>).
- `Cargo.toml:71` is `edition = "2021"`, where both are still safe fns, and
  `Cargo.toml:84-86` carries an edition-2024 tripwire for pinned dependencies —
  so the migration is live rather than hypothetical, and it turns each of these
  five call sites into a compile error
  (<https://doc.rust-lang.org/edition-guide/rust-2024/newly-unsafe-functions.html>).
- **`EnvGuard` may already be dead, and if so this leaf is a deletion.**
  Enumerating the identifier across every `.rs` file in `crates/` and `testing/`
  finds the type declared and documented, and constructed **nowhere**: the only
  other occurrences are two prose mentions in `crates/grove/tests/env_hygiene.rs`
  (lines 52 and 62) and three doc-comment references inside `support.rs` itself.
  Nothing warns, because `support.rs` opens with `#![allow(dead_code)]` — which
  it needs for an honest reason, stated at line 17: each consumer compiles the
  whole module into its own binary, so not every item is used by every one. That
  allow is what makes a genuinely dead item indistinguishable from a
  selectively-used one. Establish this before designing anything: a type nothing
  constructs needs no soundness argument.
- `lock_env` (`testing/support.rs:35`) is **not** dead — three call sites, at
  `crates/grove/tests/loop_driver.rs:864` and
  `crates/grove-loop/tests/driver_lease.rs:1033,1125`. Its doc comment justifies
  its poison tolerance by reference to `EnvGuard`'s `Drop`, so if `EnvGuard`
  goes, that comment is describing something that no longer exists and those
  three sites need a reason of their own — they hold an env lock while mutating
  no process env, which is a claim worth stating rather than inheriting.
- `crates/grove/tests/env_hygiene.rs:52-62` asserts a pairing between
  `grove_env_names()` and every `Command::env_remove` call site, and names
  `EnvGuard` as one half of it. `grove_env_names()` has the second consumer that
  comment describes — subprocess scrubbing — so the function outlives the guard;
  check what that test actually asserts before editing its prose.

## Done when

- `testing/support.rs` contains no `set_var` or `remove_var`, or contains one
  with an argument in the file for why it is sound *there* — and the whole-repo
  enumeration is re-run to say which.
- If `EnvGuard` is deleted: `lock_env`'s doc comment no longer justifies itself
  by a type that is gone, its three call sites say what they are locking against,
  and `env_hygiene.rs`'s prose names only halves that exist. If it is kept: the
  file says what an edition-2024 migration has to do to it.
- Whatever is chosen survives an edition-2024 `cargo check` in the tripwire's
  spirit, or the file states explicitly what the migration will have to do.
- `bash scripts/check.sh` passes, and the three suites that compile this module —
  `-p grove`, `-p grove-llm`, `-p grove-loop` — are green.

## Notes

**`crates/jj-workspace/tests/environment.rs` is the worked precedent, not the
template.** k185 removed its mutation by making the *test binary* the child:
build the fixture, hand the variables to `Command::env`, re-run yourself with
`--exact`, and carry a sentinel proving the child ran, because a libtest binary
whose filter matches nothing reports `0 passed` and exits `0`. That works there
because one test needs one environment. It does not obviously transfer here: this
guard is general, its `clear_grove_env` removes a **computed** list, and a
re-exec per caller would be a large change to binaries this leaf does not own.
Read k185's running log for the shape of the argument, not for the fix.

**No page is at risk.** `testing/` is under no crate, so it is in no book's
corpus and owns no fragment or ledger row. Confirm by enumerating mentions of
`testing/support.rs` and `EnvGuard` across `docs/` before committing, rather than
by assuming it.
