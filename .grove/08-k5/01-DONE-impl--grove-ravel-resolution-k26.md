# grove-ravel-resolution-k26


## Goal
Add and verify Grove's PATH-only Ravel resolver and supported CLI version check
beside the bundled-dispatch locator, as the expand stage of k5.



## Context
`crates/grove/src/dispatch.rs` currently supplies both callers. The new resolver
will live beneath that module until k27 switches its entry point. Installed
`/opt/homebrew/bin/ravel --version` reports `ravel 0.1.0`.

## Done when
- Only executable regular files in absolute PATH directories are candidates;
  empty/relative entries, cwd, Grove siblings and repository fallbacks cannot win.
  Select the first candidate, canonicalize it once and check that same path.
- Accept stable `ravel 0.1.PATCH` versions; refuse older, unrecognized and
  malformed version responses or unsuccessful probes with an upgrade remedy.
  Missing-tool errors name `brew install linkuistics/taps/ravel`.
- Tests cover precedence, symlinks, missing/non-executable candidates, cwd and
  old/malformed decoys. A reusable task tests the real installed front's version
  and worker/result round trip without host Bun/Node in its child PATH.
- The existing caller ADR records trust in executable absolute PATH entries and
  the compatibility floor. Changed source-pinned blocks/manifests/indexes agree.
  Focused nextest checks, formatting/lint and the relevant Grove handoff suite pass.

## Notes
The old locator remains active in this expand stage. k27 removes the temporary
dead-code allowance when both consumers switch. No migration-wide completion is
claimed by retiring this child.

## Decisions (running log)

Use the existing caller contract as the design and stage resolution independently
before switching both consumers. Ravel 0.1.0 is installed and supplies the minimum
two-verb contract. Stable 0.1 patch releases share this supported contract; a new
minor/major or prerelease requires explicit compatibility review.

Skip empty and relative PATH components, rather than deriving candidates from
the working directory. Trust the first executable candidate even when its version
refuses: falling through would conceal the owner's selected installation.

Behavioral disproof uses executable tests rather than an in-session reviewer:
the resolver cases and real installed-pair case were observed failing against
the unimplemented resolver, then passing after implementation. A fresh read of
the same small resolver would duplicate this instrument. The later k29 closure
contests migration completeness; no fresh-context reviewer was spent here.

Freeze the opaque continuation fixture as `grove-continue`. k27 will make both
actual result readers and caller-supplied ending instructions consume that
literal; this stage proves only the installed transport round trip.

The host Bun is 1.4.3; retained dispatch builds require 1.4.2. Reuse Ravel's
verified local `target/tools/bun` by prefixing its directory on the check PATH,
without changing the host or either installation. `task test:grove` retains the
old worker build and runs the Grove suite through nextest.

## Verified handoff

- `task ravel:resolution RAVEL=/opt/homebrew/bin/ravel`: six resolver tests and
  the real copied 0.1.0 front/worker round trip passed. The subprocess fixture
  runs through its owning test; the separate installed test runs explicitly.
- With Ravel's pinned Bun 1.4.2 directory prefixed to PATH, `task test:grove`
  passed all 145 tests. The final per-test result map matches the initial run,
  rather than only agreeing on totals. Three ignored roles/cases are accounted
  for by the fixture parents and separate installed task.
- `cargo fmt --all --check` and `cargo clippy --locked -p grove --all-targets`
  passed. Final overview book-check reconstructs all eight roots and 1,415 lines
  with no deferred source. The new resolver is included, not excluded, by the
  book's fixed corpus rule.
- Complete logs and per-item/digest records are in
  `target/grove-ravel-resolution/`: `handoff-nextest.log`,
  `handoff-resolution.log`, `fmt.log`, `clippy.log`, `book.log`,
  `subjects-before.json`, `subjects-after.json` and `handoff-summary.json`.
  All 1,990 individually frozen repository/installation/configuration subjects
  matched after the run. The snapshots include tracked inputs, original Grove
  22.1.0 and Ravel 0.1.0 kegs, the pinned Bun, owner Ravel policy and tap formula.
- Ravel's checkout remains clean at the empty successor of 3916a4a1; the tap
  is clean. No installation, policy, release or original driver channel changed.

This child meets its expansion contract. k27/k28/k29 remain live beneath k5;
actual launches still use the old bundled locator until the consumer migration.
The repository-wide `task check` and spelling/control inventory belong to k29,
after that migration, and have not been claimed here.
