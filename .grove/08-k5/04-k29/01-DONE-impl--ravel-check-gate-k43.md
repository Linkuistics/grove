# ravel-check-gate-k43


## Goal
Make the repository check exercise Rust runtime suites with nextest and include
the installed-Ravel caller policy/provenance assets, while retaining doctests
and bundled archive compatibility until k6.



## Context
First child of grove-ravel-contract-k29. The resolver's old locator/allowances
and release-notes' dispatch:build dependency/build assertions are already gone
in k39. The original v22.1.0 driver/ending channel stays live.

## Done when
- scripts/check.sh runs workspace runtime suites with nextest and keeps Cargo
  doctests separately; check reporting UI remains visible.
- The gate runs task ravel:review against explicit GROVE_TEST_RAVEL and retains
  pre-k6 worker/probe/archive checks; its summary reflects the actual checks.
- The changed reusable gate passes with complete saved logs and individually
  matching before/after subject digests. Source-pinned corpora are unaffected
  or reconciled if their source changes.

## Implementation plan
1. Keep the existing environment entry point (GROVE_TEST_RAVEL), the same
   Taskfile gate and retained archive/build recipes.
2. Replace workspace cargo test with nextest run --locked --workspace
   --no-fail-fast, then cargo test --locked --workspace --doc. Add the existing
   task ravel:review with the explicit installed front. Derive summary count
   from the checks actually invoked.
3. Verify shell lint, real release-preparation/build assumptions and one real
   task check with the pinned Bun directory. Freeze every tracked input plus
   retained ignored policy dependencies and external installed/owner assets.
4. Retire/seal this child only. k44/k45 retain whole-surface and k5 closure.

## Decisions (running log)
- The existing explicit GROVE_TEST_RAVEL environment remains the repository
  gate's input; no new resolver, installation or implicit selection is added.
- scripts/check.sh and Taskfile.yml are outside all six source-book corpora;
  the subsequent TUI integration-fixture repair also changes no source-pinned
  book input or production Rust source.
- The first frozen full gate exposed nine TUI witness fixtures passing the old
  dispatch front directly into the migrated loop (`start-session` refused).
  Repair this gate prerequisite using the existing explicit installed-Ravel
  fixture helper and public end-session result; preserve the witness assertions.
  These integration tests are outside the six source-book corpora. Scrub every
  shared original/new authority control on their self-spawned driver processes.
  The initial 3,092 individual subject hashes all matched after the failed run.
- The migrated TUI driver-kill fixture passed but nextest reproducibly reported
  a leaked inherited stderr pipe: Drop waited only for the already-killed driver.
  Carry the actual supervisor PID over the existing readiness socket and wait
  for its exit after requesting session shutdown. All ten focused witnesses
  now pass without a leak; no assertion or nextest leak detection was weakened.
- The actual gate/witness tests and nextest's independent leak detector provide
  executable evidence for this recipe/fixture repair. A same-source in-session
  reviewer would duplicate that check; k44 still owns exhaustive surface audit.

## Verification and handoff

`PATH=/Users/antony/Development/Ravel/target/tools:$PATH
GROVE_TEST_RAVEL=/opt/homebrew/bin/ravel task check` passes all **14** principal
checks. The workspace nextest run passes all 1,723 cases, with 31 ignored cases
(self-spawned roles/explicit installed fixtures); the integrated `ravel:review`
task passes 88 cases, including nine ignored public-policy asset fixtures.
All nine Cargo doctests pass. All six source books validate final/all with zero
deferred lines; no book corpus input changed. Formatting, shellcheck, all-target
clippy, plugin install, 21 conformance mutation controls, release-preparation
(including real confined Ravel), worker/probe/type and retained archive checks
pass. No model, installation, version cut, real publication or new driver ran.

Complete evidence is in `target/k43/`: `check-red.log`, `check-final.log`,
`prepare.log`, `tui-green.log`, `tui-leak-red.log`, `tui-clean-green.log`,
`before.json`/`after.json`, `before-final.json`/`after-final.json`,
`digest-comparison.json`, `final-digest-comparison.json`, `per-test-evidence.json`,
`repaired-cases.json` and `measure.py`. The same 1,723 workspace case identities
are present in both full runs; each of the nine failed TUI cases changes to PASS.
The intentional driver-kill case's reproducible LEAK changes to PASS in the
focused suite and final workspace run, without hiding its stderr or changing
the witness assertions. Both frozen measurements have identical individual
hashes for all 3,092 subjects: tracked inputs including dotfiles/task notes,
book sources/manifests/indexes, retained worker/declarations and ignored compiler
dependencies, measured tools, complete original 22.1.0 Grove keg, independent
Ravel keg, owner Ravel policy and Ravel tap formula. Only handoff/retirement
notes are changed after the final measurement.

Measured source pins:
- scripts/check.sh: `7c937edfbdf212ce6e65f726d9ac03f1601840cffa95e90da10386dfd2edc065`
- crates/grove-tui/tests/witnessed.rs: `062ea91decd6155b7ebb8ee415b251309dcaf8156b26a7ca19ed5987a56f0102`

Ravel remains installed 0.1.0. Adjacent Ravel/tap workspaces are clean empty
successors of `3916a4a1`/`bd5aa37b`. The original live 22.1.0 command bytes and
ending channel remain intact. k43's criteria hold; k29/k5 remain live for k44's
complete surface reconciliation and k45's complete acceptance (including
`ravel:fixtures`/`ravel:commands` for the other explicit installed fixtures).

## Notes
