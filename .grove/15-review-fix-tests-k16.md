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
