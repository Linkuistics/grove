# grove-ravel-static-assets-k40

## Goal
Add independently usable Grove static policy examples and sample under
`assets/ravel-policy/`, using the installed Ravel 0.1.0 SDK and ordinary file
imports, while keeping pre-k6 package checks intact.

## Context
Transfer `crates/harness-dispatch/worker/examples/grove-static.ts` and
`worker/sample/policy.ts`. Reuse the generic `ravel/examples/static` helper,
the installed SDK declarations and `testing/ravel.rs` real-pair fixture. This
increment enforces no creator/provider comparison; k41 and k42 own that work.

## Done when
- Both assets typecheck through a reusable task against the installed SDK,
  with strict settings and fixtures that exercise their public imports.
- Fake-agent sessions through the copied independent installed front/worker
  verify the static routes, sample defaults and arrangements/modifiers,
  unknown/invalid choices, exact prompt arguments and secondary-jj grants.
  The child PATH has no host Bun/Node; no model launches occur.
- Readable instructions show ordinary file imports and `.ravel-choice`.
  The owner's actual policy/settings/choice bytes remain unchanged. Retain old
  package fixtures and check affected source-book coverage.
- Focused and full asset checks pass with saved logs and individually frozen
  inputs. Retire only k40; k28 stays open for provenance/review work.

## Implementation plan
1. Write installed-worker boundary tests before adding assets; observe the
   missing-assets failure. Expectations come from literal old sample routes.
2. Transfer the two files, changing imports and current executable instructions;
   preserve executable routing logic. Add strict installed-SDK typecheck fixtures
   and `ravel:policy` reusable tasks.
3. Run the asset suite, formatting/lint/typechecks, source-book validation and
   old-package checks appropriate to the additive change. Record exact evidence.
4. Retire and seal k40, leaving the provenance/review children live, then send
   the session's original `harness-dispatch exit` signal as the last action.

## Decisions (running log)

- Transfer executable route logic unchanged; replace SDK/example imports, choice
  filename and current usage text. Static provider labels remain placeholders;
  the sample retains its four arrangements, two modifiers, defaults, concrete
  commands and secondary-workspace grants. The owner's policy is not replaced.
- Resolve type declarations from the explicitly supplied installed Ravel front's
  canonical prefix. Reuse the pinned development compiler and strict settings;
  runtime checks copy only the installed executable pair and isolate PATH.
- The grant fixture must expect the canonical main-repository path: Ravel
  canonicalizes request.cwd, including macOS's /var → /private/var alias.
- Host Bun is 1.4.3 and the strict typecheck correctly refuses it. Measurements
  use the already verified Bun 1.4.2 at Ravel's target/tools on the development
  PATH; the copied runtime pair needs no host Bun/Node.
- The executable bodies equal their originals after just the intended public
  import and version-label substitutions. The new typecheck currently reuses
  the old package's pinned compiler/tsconfig; k6 must relocate that development
  dependency before removing the bundled package. No changed production source
  belongs to an existing source-book corpus; validate those books at handoff.

## Verified handoff

Grove now owns the readable static example/sample in `assets/ravel-policy/`.
Both import Ravel's public generic SDK; owners import/copy the Grove files
normally. The static example's routes and the sample's choices remain intact.
No owner policy/settings/choice, Ravel source or original installed driver was
changed. k41 owns provenance production; k42 owns Creator-aware adapter/review
consumption. k28 therefore remains open and no ancestor closes here.

Final checks, with Bun 1.4.2 selected on the development PATH:

- `task ravel:policy RAVEL=/opt/homebrew/bin/ravel`: installed-SDK strict
  typecheck and four nextest cases passed, comprising 58 selections and seven
  refusals against copied Ravel 0.1.0. All session kinds, both providers, exact
  prompt arguments, defaults, four arrangements, both modifiers, invalid
  choices, release-notes and secondary-jj grants are exercised. The runtime
  child PATH contains only the copied pair and fake agents.
- `task ravel:policy:compatibility`: old worker/SDK typechecks and all 47
  grove/review/sample/examples/choice-file nextest cases passed. The legacy
  embedded adapter/store fixtures remain available until the later migration
  and delivery increments.
- `cargo fmt --all --check`, focused shellcheck/shfmt and
  `cargo clippy --locked -p grove --all-targets` passed. Every one of the six
  source-book roots passed `book-check --final --check all`; no production
  corpus source changed in this additive increment.
- The route mutation (`impl` high → low) failed on the fake-agent's actual
  argument vector. The type mutation exposed TS2322 for a string substituted
  for a Route. Both were restored before final measurement. The initial
  missing-assets red tests and the host-Bun pin refusal are also saved.
- All 3,017 individually digested subjects matched before/after final checks:
  repository sources/configs/books excluding VCS/build/tree notes, pinned
  compiler dependencies, installed Ravel 0.1.0 and original Grove 22.1.0 kegs,
  both owner configuration directories and the verified Bun binary. The exact
  subject inventory/digests and complete logs are in `target/k40-evidence/`:
  `policy-final.log`, `compatibility-final.log`, `books-final.log`,
  `fmt-final.log`, `shellcheck-final.log`, `shfmt-final.log`,
  `clippy-final.log`, `red.log`, `route-mutation.log`, `type-mutation.log`,
  `before.json`, `after.json` and `digest-comparison.json`.

For k6: ship the readable assets through Grove's own layout, and move the
pinned compiler/config currently reused from the old package before removing
that package. No SDK or equal-version layout dependency on a Ravel checkout is
needed; the installed Ravel declarations and copied pair are the handoff seam.

