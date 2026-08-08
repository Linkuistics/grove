# adoption-migration-session-kind-transition-k98

**Kind:** impl

## Goal

Make `grove do` carry an adopted v1-flat or old `NNN-slug` tree through every
required compatibility transition before the first strict session-kind read.

## Context

`driver-lease-integrate-k33` strengthened its hostile-Git regression with a
v1-flat tree and surfaced a separate pre-existing pipeline gap. Adoption reports
success after converting `1-[1]-test.md` to the older v2 shape
`01-test-k1.md`, but no `FORMAT` witness exists yet; the immediately following
`grove-llm kind` therefore refuses the tree before the session-kind lifecycle
transition can run. Keep repository-selection hardening in
`driver-lease-integrate-k33`; this leaf owns only compatibility-transition
composition and its end-to-end launch behavior.

## Done when

- One `grove do` invocation on each supported legacy grammar reaches the current
  session-kind format before strict kind routing.
- The transition remains transactional/fail-closed and preserves the intended
  migration commit boundaries.
- End-to-end regressions cover the first routed launch after adoption, not only
  direct migration entry points.
- `cargo fmt --all --check` and `cargo test --locked` pass.

## Notes

Reproducer observed while integrating `driver-lease-review-k32`: after
`migrate_on_adoption` prints a successful one-file migration, `.grove/` contains
`01-test-k1.md` and `BRIEF.md` but no `FORMAT`; `grove-llm kind` then reports
that the legacy tree must be migrated.
