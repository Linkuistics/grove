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

## Decisions (running log)

- **`EnvGuard` was dead, so this leaf is a deletion and there is no soundness
  argument to write.** The Context's suspicion is confirmed by enumeration, not
  by the compiler: across every file in the checkout the identifier occurs
  seven times, and none is a construction — the `struct`, its `impl`, its `Drop`,
  **two** doc-comment references inside `support.rs` (its lines 31 and 220; the
  Context above says three, counted one high), and two prose mentions in
  `crates/grove/tests/env_hygiene.rs`. Nothing warned because `support.rs` opens
  `#![allow(dead_code)]`, and that allow is *required* for an honest reason
  stated at its line 17 — each of the three consumers `#[path]`-compiles the
  whole module into its own test binary, so an item used by one looks dead to
  the other two. The allow that makes selective use legal is exactly what makes
  a genuinely dead item invisible, which is why five unsound call sites survived
  with zero callers.
- **`lock_env` and both `ENV_LOCK` statics went with it, because neither was
  guarding anything.** The Done-when anticipated keeping them and having the
  three call sites state what they lock against; the honest answer is *nothing*,
  and a comment saying so is worse than the deletion. Two independent grounds:
  (1) with `EnvGuard` gone the workspace contains no process-env mutation at all
  — enumerated below — so there is no window for a concurrent reader to observe;
  (2) `crates/grove/tests/loop_driver.rs` had **one** `lock_env` call in its
  whole binary, and a mutex with a single lock site serialises nothing, so its
  half was inert regardless of (1). The other two sites, both in
  `crates/grove-loop/tests/driver_lease.rs`, serialised only each other. Neither
  test mutates process env: every child is configured through `Command::env` /
  `env_remove`, and both fixtures are hermetic — own `TempDir` worktree, own
  `HOME`, and `grove_driver` scrubs `grove_env_names()` from the child. The
  deleted comment's stated reason — *"they read the ambient environment to build
  a scrubbed child env"* — is void once nothing writes it.
- **The delock was measured, not assumed safe.** Removing a serialisation can
  only be shown safe by running the tests it stopped overlapping, so
  `cargo test --locked -p grove-loop --test driver_lease` was run three times:
  **23 passed** every time, in 39.5s, 6.0s and 6.0s (the first run's figure is
  a cold one and is reported rather than explained — no attempt was made to
  attribute it to the delock). `-p grove -p grove-llm -p grove-loop` is green
  twice over with zero failures, and the counts two books publish are unchanged:
  `env_hygiene.rs` 4, `loop_driver.rs` 11, `lifecycle_cutover.rs` 17.
- **Edition 2024 has nothing to do to `testing/support.rs`, measured against a
  control.** A workspace copy with `[workspace.package] edition` flipped to
  `2024` fails `cargo check --workspace --all-targets` on exactly one error, and
  it is unrelated: `E0515` at `crates/ordinal-fs-tree/src/plan.rs:219`, Rust
  2024's widened `impl Trait` lifetime capture, discharged by the compiler's own
  suggestion — `+ use<'a, N>` on `snapshot.rs`'s `children`. With that one line
  applied the whole workspace, all targets, compiles clean. The control on that
  green run: re-adding one `set_var` and one `remove_var` to `testing/support.rs`
  turns it into **seven** `E0133`s. So the migration is free *because* of the
  deletion, not because the edition is lenient — and none of those `unsafe`
  blocks could have been discharged, since the safety condition is that no other
  thread reads the environment and libtest gives no test that guarantee. The
  finding about `ordinal-fs-tree` is recorded here rather than fixed: the corpus
  is frozen, `src/snapshot.rs` and `src/plan.rs` are book roots, and an
  edition-2024 migration is not this leaf.
- **The whole-checkout enumeration, re-run against the final tree.** Every
  occurrence of `set_var` or `remove_var` outside `target/`, `.jj/` and `.git/`:
  three doc-comment lines in `crates/grove/tests/env_hygiene.rs`, three in
  `crates/jj-workspace/tests/environment.rs`, seven in `testing/support.rs`, and
  the rest in `.grove/` task files. **Zero call sites** — filtering the `.rs`
  hits to non-comment lines returns nothing. `crates/jj-workspace`'s prose is
  k185's; the other two are this leaf's own.
- **`grove_env_names` survives with four callers, and its doc comment now names
  one consumer instead of two.** `crates/grove/tests/loop_driver.rs:144`,
  `crates/grove-loop/tests/driver_lease.rs:130` and
  `crates/grove-llm/tests/removed_surface.rs:554,694` all read it for
  `Command::env_remove`. `env_hygiene.rs`'s two prose mentions of `EnvGuard` —
  the doc comment at its line 52 and the assertion message at 62, as the
  Context gave them — are rewritten to name only the subprocess half, and the
  test they belong to still asserts exactly what
  `docs/walkthroughs/grove-loop/20-the-loop.md:1870` says it does.
- **The standing gate is real work and is now its own leaf.** "Nothing may
  mutate this process's environment" is held by prose in two files and by
  nothing executable, which is weaker than the two guards `env_hygiene.rs`
  already asserts and is a poor fit for a file whose header says the guards get
  assertions rather than trust. The scan is small — walk every first-party
  `.rs`, skip whole-line comments, report `set_var`/`remove_var` — but a fifth
  `#[test]` in that file is a fifth test in a control two books publish figures
  from, so it drags a page repair a deletion should not carry. Cut as
  `env-mutation-standing-gate-k216`, whose body enumerates the four pages and
  flags that three of the four `626` figures read as **records of runs that were
  taken** rather than present totals — chapter 10's says so outright — so the
  leaf owes a verdict on each rather than a reflexive edit.
- **No page moves for this leaf.** Confirmed by enumeration across `docs/`,
  `plugins/`, `scripts/` and the root Markdown, not assumed: `EnvGuard` and
  `lock_env` appear in no book, and the only mentions of `testing/support.rs`
  are `14-finishing.md:1535` (about `workspace_binary`) and `20-the-loop.md:1872`
  (about the scrub list) — both untouched. `testing/` is under no crate and
  `crates/*/tests/` is evidence rather than corpus, so no fragment or ledger row
  exists to shift. The `driver_lease.rs` and `loop_driver.rs` line counts in
  `docs/specs/grove-loop-book-structure.md:54` are the **inline `mod tests`** of
  `crates/grove-loop/src/`, different files from the two integration targets
  edited here.
- **No ADR.** The record test is an AND — hard to reverse, surprising without
  context, a real trade-off with a rejected alternative — and the first fails:
  re-adding a `set_var` costs one line, which is precisely why k216 exists. The
  decision lands where it was made, in `testing/support.rs`'s header, with both
  citations beside it. k185 made the same call for the same reason.
