# clippy-baseline-k4

## Goal

Clear the eight pre-existing `cargo clippy --all-targets` warnings, then make the
clean state hold — a baseline nothing re-establishes is a baseline that decays
back to eight within a release.

## Context

Surfaced while verifying `flat-lazy-review-k2`, whose `Done when` asked for a
clean clippy run. Every warning is in a file that leaf does not touch (confirmed
against `jj st`), so fixing them there would have widened a deliberately scoped
deletion commit into the finish-transaction and lease subsystems. Externalized
rather than absorbed, per the Decompose rule.

The eight, as of `flat-lazy-review-k2`:

```
src/tree_migration_transaction.rs:1106  very complex type — factor into a `type`
src/tree_migration_transaction.rs:1175  very complex type — factor into a `type`
src/driver_lease.rs:585                 file opened with `create`, `truncate` behavior not defined
src/finish_cleanup/auxiliary.rs:198     large size difference between variants
src/finish_cleanup.rs:62                casting to the same type is unnecessary (`u64` -> `u64`)
src/repo/finish_commit.rs:27            large size difference between variants
src/repo/finish_commit.rs:32            large size difference between variants
src/repo/finish_commit.rs:41            `Result.or_else(|x| Err(y))` — use `map_err`
```

Two are not purely mechanical and want a judgement, not a `--fix`:

- **`driver_lease.rs:585`** is the interesting one. `create` without an explicit
  `truncate` is exactly the ambiguity that bites a *control file*, and this one
  backs the driver lease. Read what the file is for before changing it: the fix
  is to state the intent explicitly, which may be `truncate(false)`.
- **`large size difference between variants`** (three sites) is a `Box` decision
  on error/result enums, not a rename. Boxing a hot variant to satisfy a lint is
  a real trade; if the enum is constructed rarely, `#[allow]` with a stated
  reason is the honest answer.

## Done when

- `cargo clippy --all-targets` reports zero warnings, and the real output is
  pasted rather than asserted from expectation.
- `cargo test` still passes — `driver_lease.rs` and the finish-transaction files
  are the two subsystems where a "harmless" change is not.
- Any warning answered with `#[allow]` rather than a fix carries a comment
  saying **why** the lint is wrong here, not that it was noisy.
- Clean stays clean: decide how, and do it. The obvious candidate is a
  `[lints.clippy]` table in `Cargo.toml` (or a CI step) that makes a warning a
  build failure — without one this leaf's work is undone by the next warning
  nobody notices.

## Notes

Deliberately *not* in scope: broadening to `clippy::pedantic` or `nursery`. This
leaf is about the baseline the default lint set already reports, and a wider set
is a separate decision with its own trade.
