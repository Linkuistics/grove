# review-fix-tests-k16

**Kind:** work

## Goal
Make the branch's test suite green *and* honest. First of the four
branch-review-k14 fix leaves, and a prerequisite for the other three — until
the suite is trustworthy, no other fix is verifiable.

## Context
Findings B1 and T1–T8 in `.grove/14-DONE-branch-review-k14.md`. Read them
there; the short form:

- `cargo test` fails 5/9 in `tests/loop_driver.rs` on this machine because the
  tests scrub only the five base `GROVE_<KIND>_MODEL` vars and not the new
  dimension — ambient `GROVE_REVIEW_HARNESS=pi` + `GROVE_PI_REVIEW_MODEL`
  (written to `~/.zshenv` by leaf 12) reroute `loop_selects_model_by_kind`.
  Confirm with `env -u GROVE_REVIEW_HARNESS cargo test --test loop_driver`.
- The scrub surface is now 25 names (5 kinds × [base, 3 harnesses] + 5
  `GROVE_<KIND>_HARNESS`) — a `clear_grove_env()` test helper, not more
  `remove_var` lines.
- `ENV_LOCK.lock().unwrap()` at ten sites in `tests/loop_driver.rs` plus
  `tests/provision.rs:126` and `tests/complete.rs:149` turns the first real
  failure into a `PoisonError` cascade that hides it. Use
  `.unwrap_or_else(|e| e.into_inner())`.
- Two safety tests are mutation-proven vacuous: `known_names_lists_every_
  registry_row` (tests/harness.rs:37) and the `CRITICAL` symlink assertion
  (tests/provision.rs:97-102).
- Missing negative cases are listed as T6.

## Done when
- `cargo test` passes with the developer's real environment loaded, and also
  under `env -i`-style isolation — the suite no longer depends on ambient
  `GROVE_*`.
- Re-running the T1/T2 mutations (hardcode `known_names()`; swap `remove_file`
  for `remove_dir_all` in `provision_target`) turns tests **red**.
- T6's negative cases exist, including the `GROVE_CLAUDE_PID` co-export.
- Env is save/restored via a drop guard, so a failing assert cannot leak.
- One focused commit.

## Notes
- T2 raises a real question to settle here: `Cargo.toml` declares
  `rust-version = "1.74"`, but the symlink-never-followed property is
  guaranteed by `remove_dir_all`'s 1.77-era hardening. Either raise the MSRV
  or record that `provision_target`'s ordering is load-bearing — don't leave
  the declared floor below the version the safety argument needs.
- Fixing the *test* is the whole job here; B5 (the `GROVE_HARNESS_BIN` leak
  that made the misroute silent) belongs to `review-fix-routing-k17`.

## Result
Green under `cargo test` with the real dogfooded `~/.zshenv` loaded (its
`GROVE_REVIEW_HARNESS=pi` + `GROVE_PI_*` vars, plus this very session's own
`GROVE_PLANNING_MODEL` etc.) and again under `env -i` (PATH/HOME/TMPDIR/
toolchain vars only, every `GROVE_*` gone) — 282+ cases across both runs, zero
failures. `cargo fmt --check` and `cargo clippy --all-targets --all-features
-- -D warnings` both clean. One focused commit.

Added `tests/support/mod.rs` (`EnvGuard` — save/restore via `Drop`, tolerant
of a poisoned lock; `clear_grove_env()` scrubbing the full 25-name routing/
model surface; `lock_env()` wrapping `.lock().unwrap_or_else(|e|
e.into_inner())`) and wired it into every env-mutating test in
`loop_driver.rs`/`complete.rs`/`provision.rs`. Verified the poison-tolerance
live: injected a deliberate panic into one `loop_driver` test — its 11
siblings still reported real pass/fail, not `PoisonError` (the exact B1
cascade). B1's *root* failure (`loop_selects_model_by_kind`) is fixed by
`clear_grove_env()`, proven against the real ambient `GROVE_REVIEW_HARNESS=pi`
in this very shell, not a simulation.

T3, T6, T7, T8 fixed as specified: continue-path `GROVE_REVIEW_HARNESS`
coverage added alongside the existing start-path test; five T6 negative cases
added (unknown name in a stamp file and via explicit `--harness` in
`tests/harness_stamp.rs`; empty-string `GROVE_<KIND>_HARNESS` and empty-string
model var; the `GROVE_CLAUDE_PID` co-export, folded into the existing PID test
rather than a new one) — 856→ tests grew from 9 to 12 in `loop_driver.rs` plus
2 in `harness_stamp.rs`; `tests/complete.rs:148` and `tests/provision.rs:134`
converted to `EnvGuard` so a real inherited `GROVE_CLAUDE_PID`/`GROVE_SKILL_DIR`
survives the test; the dead `"precious"` disjunct in
`provision_refuses_a_foreign_directory` removed.

**T1 and T2 deviate from the letter of "Done when" — folded back here rather
than silently patched.** Both proposed mutations (hardcode `known_names()`;
swap `remove_file` for `remove_dir_all` in `provision_target`'s symlink
branch) turned out to be **unfalsifiable by any runtime assertion**, verified
by actually applying each to the working tree and running the suite before
reverting:
- **T2**: `remove_dir_all` called directly on a path that is itself a symlink
  never recurses through it — confirmed three ways: (1) a standalone probe
  program against this machine's toolchain, (2) reading the shipped std
  source (`rustc --print sysroot`'s `library/std/src/sys/fs/unix.rs`
  `remove_dir_all_modern`, which `lstat`s the top-level path and unlinks
  rather than recursing — and even the generic non-Unix-optimized fallback in
  `sys/fs/common.rs` has the identical special case), and (3) the public
  rustdoc's "TOCTOU race conditions" section, stable since 1.0.0: *every*
  platform protects against this **except Miri, QNX, Redox OS, and VxWorks**
  — none of which grove targets. So the swap is byte-for-byte behaviourally
  identical to the original on every platform grove ships for, at any Rust
  version — confirmed empirically (applied the swap, ran `cargo test --test
  provision`: all 6 green, including the "CRITICAL" assertion). **No MSRV
  bump — the declared 1.74 floor was never the issue.** The comment in
  `provision_target` and the test now state the real load-bearing reason for
  the explicit branch: it forces replacement past `sync_to_stamp`'s
  stamp-matches-so-skip shortcut, not a symlink-following risk that doesn't
  exist. That *is* a real, verified mutation: deleting the whole
  `is_symlink` branch (not just the one line) does turn
  `provision_replaces_a_symlinked_grove_entry_with_a_real_dir` red, via the
  pre-existing `wrote`/`meta.is_dir()` assertions — confirmed the same way.
- **T1**: `HARNESSES` is a fixed `const` with no test-time seam to vary its
  contents, so any hardcoded `known_names()` body that happens to match
  today's registry is indistinguishable from a real derivation by any
  black-box assertion — proven by applying the exact hardcode and rerunning:
  green either way, regardless of how the test's expectation is computed.
  Rewrote the test to derive its expectation from `HARNESSES` directly
  instead of a copied literal, which protects the real failure mode (a 4th
  harness added to the registry without updating a hand-maintained
  `known_names()`) even though it cannot retroactively catch today's already-
  consistent hardcode.

Both are the same underlying shape: a proposed mutation that is provably
unobservable given the current design (no mutable test-time input for T1; a
documented, stable, universally-guaranteed std contract for T2), not a test
that was insufficiently clever. Recorded here rather than re-litigated by the
next session that reads this leaf.
