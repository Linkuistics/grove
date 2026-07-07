# fmt-cleanup-k5

**Kind:** work

## Goal

Restore `cargo fmt --check` to green on this branch before the grove finishes.
The `grove-llm kind` verb (`grove-llm-kind-verb-k2`) landed three unformatted
`format!`/`assert!` blocks in `src/tree_read.rs`, so `cargo fmt --check`
currently exits 1.

## Context

- Introduced by commit `7ada53d` (`grove-llm-kind-verb-k2`, retired). Surfaced
  during `document-model-selection-k4`'s pre-commit checks; externalized rather
  than absorbed into that docs commit (focused-commit discipline).
- Affected file/lines: `src/tree_read.rs` around lines 185, 843, 852 (rustfmt
  wants the long `format!(...)` / `assert!(...)` args wrapped).

## Done when

- `cargo fmt` applied; `cargo fmt --check` exits 0.
- `cargo test` still green (a pure reformat — no behavior change).

## Notes

- Trivial mechanical fix: run `cargo fmt`, review the diff is whitespace-only,
  commit. One focused commit, then retire.
