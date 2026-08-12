# provisioned-skill-refresh-integrate-k18

**Integrates:** provisioned-skill-refresh-review-k14

## Goal

Apply the actionable findings from `provisioned-skill-refresh-review-k14` so
the build-boundary documentation says skew is unsafe in both directions and
`tests/provision.rs` actually guards every instructed CLI command path it claims
to guard.

## Context

The producer commit is `provisioned-skill-refresh-k9`. Its central mechanical
claims held: `include_dir!` fixes one embed at compile time;
`provision_installed` has one production caller in `bare_grove`; the loop never
re-enters it; and migration/finish transitions do not provision. Do not reopen
`shared-skill-dir-clobber-k13` here — its separate review and implementation
leaves own the cross-build shared-directory defect.

Apply these findings from the review:

1. **P2 — The changelog still says old-skill/new-binary skew is safe.**
   `CHANGELOG.md:141-144` says an older skill "names only verbs that exist."
   That is the producer's rejected first draft and contradicts both
   `docs/ARCHITECTURE.md:651-659` and `tests/provision.rs:83-89`; v17's
   `leaf-add-chain` is the direct counterexample. State that skew is unsafe in
   both directions.
2. **P2 — The scanner can miss a valid instruction while its controls pass.**
   `scan_instructed_verbs` requires whitespace immediately after `grove-llm`,
   so ``Run `grove-llm` `leaf-add-chain` …`` is invisible. The corpus currently
   yields 11 distinct verbs, but the floor is only eight plus `leaf-add`, so up
   to three other verbs can disappear from the scan without failing the
   controls. Make complete coverage enforceable and pin adjacent code spans.
   Preserve the useful existing behavior: two spaces and contiguous table-cell
   invocations count; a hyphenated verb across a wrap fails closed; negative
   prose may conservatively remain a false positive.
3. **P2 — `exposed_verbs` flattens nested command paths.** Its recursive walk
   records every bare subcommand name. If the CLI later exposes
   `grove-llm admin repair`, an invalid `grove-llm repair` instruction passes.
   Compare actual invocable paths — top-level verbs for the current scanner, or
   full command paths if instruction scanning grows to support them.
4. **P3 — The glossary crosses its definition-only boundary.** Trim
   `CONTEXT.md`'s **Embedded methodology** entry to the term's definition and
   re-litigation guards. Stamp behavior, skew cases, and the test mechanism
   belong in `docs/ARCHITECTURE.md` and the separate **Methodology identity** /
   **Build pairing** definitions.
5. **P3 — The root brief records a correction instead of the corrected state.**
   `.grove/BRIEF.md:45-50` already gives the true reason; remove the redundant
   `Corrected by …` history block at lines 52-57. The producer commit preserves
   that history.

## Done when

- The changelog no longer blesses either direction of skill/CLI skew.
- Scanner tests demonstrate the adjacent-code-span case cannot disappear, and
  their completeness control cannot pass after silently losing any currently
  instructed verb.
- Exposed CLI commands are compared at the same path grain as scanned
  instructions.
- `CONTEXT.md` and `.grove/BRIEF.md` carry only current, correctly grained
  context.
- The focused tests pass, then `cargo test`, `cargo clippy --all-targets
  --all-features -- -D warnings`, and `cargo fmt --check` are clean.

## Notes

This leaf owns every edit and all post-fix verification. The review leaf made no
production or test-code changes and ran no verification commands.
