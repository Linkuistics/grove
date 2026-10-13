# ravel-installed-fixtures-k36


## Goal
Consolidate the existing resolver/result installed-pair test setup into a
shared fixture usable by the pending atomic caller migration.



## Context
Read the inherited k35/k33 contracts. Ravel 0.1.0 is installed at
`/opt/homebrew/bin/ravel`; use its canonical front and documented
`libexec/ravel/ravel-policy` layout. Existing duplicate setup lives in
`crates/grove/src/dispatch/ravel.rs` and
`crates/grove-loop/tests/session_result_installed.rs`.

This is an expansion only. Both production callers, their current policies and
the live original 22.1.0 driver remain on dispatch. No ancestor closes here.

## Done when

- A shared test-only fixture copies the explicit source front/worker into a
  temporary separate prefix, supplies an isolated PATH and removes all nine
  enclosing authority names through the existing shared scrub helper.
- Both existing real resolver/result instruments consume it, retaining their
  real-worker/fake-agent result/stream assertions. Prove copied installation
  success and an observed missing-worker negative control without Bun/Node or
  a Grove sibling installation.
- A `task ravel:fixtures RAVEL=/absolute/path/to/ravel` entry point runs both
  instruments with nextest. Existing focused caller/loop checks stay green;
  any changed source-pinned block/index/manifest is reconciled.
- Save complete logs and individual matching input digests; record installed
  Ravel version and exact source pair digests. Retire only this first child.

## Notes

## Implementation plan

1. Move pair-copy setup to `testing/ravel.rs`, included through shared support;
   expose `InstalledRavel::copy_from(source, prefix)`, `executable()`, `bin()`
   and `command()` / `isolate(&mut Command)` for caller-owned placement and process
   isolation. `command()` removes `grove_env_names()` and supplies only this
   fixture's bin directory as PATH.
2. Adapt both existing installed tests and observe failure before the helper
   exists. Add a real copied-pair test whose missing-worker control returns a
   nonzero status and no result; restore the worker and verify exact stdout.
3. Implement the small file-copy/command helper and run the focused instruments.
   Retain each caller's own policy and result interpretation.
4. Add and run `ravel:fixtures`; reconcile overview's exact resolver block and
   line manifest. Run caller/loop nextest and format/clippy once all
   inputs are frozen, then retire and seal the child through jj.

## Decisions (running log)

- Ravel 0.1.0 rejects grants for `RAVEL_*` and legacy dispatch controls before
  policy evaluation (observed CLI exit 2; source contract is Ravel's
  `docs/specs/session-execution.md`, Policy authority and runtime discovery).
  Measure all nine names at process entry through a shell-builtins probe that
  execs the actual copied front after the clean read. Observe each sentinel
  produce exit 97 separately; re-exec the test under inherited sentinels so
  Cargo's force-clear cannot satisfy the clean case. Preserve the original k34
  caller tests for k37 to adapt at their actual boundary.
- `isolate` applies the same scrub and bin-only PATH to a caller or probe;
  `command` constructs the real-front command through that helper. The result
  runner instrument uses `grove_env_names()` rather than its former partial
  hand-maintained list. Each installed invocation receives a temporary HOME
  and explicit policy, so owner settings cannot steer fixture behavior.
  Production launch/result/ending semantics stay on dispatch during this child.
- Decline an in-session reviewer: executable entry probes observed each dirty
  sentinel, the missing-worker control failed, and removing the RAVEL_EXIT_FILE
  scrub made the entry test fail. These directly contest the fixture's behavioral
  claims; a fresh read of the same helper would be a weaker instrument.

## Verification and handoff

`testing/ravel.rs` now provides the explicit-source pair copy, canonical
executable/bin accessors, `command()` and `isolate(&mut Command)`. Both existing
installed instruments use it. Its subprocess-entry control verifies all nine
inherited authority names with separate dirty reads before execing the actual
Ravel front. Its worker-removal control returns exit 5 with empty stdout, then
the restored pair returns exactly `grove-continue`. The two runner APIs retain
all eight result/status cases and stream-isolation assertions.

The supplied installation reports `ravel 0.1.0`. Canonical source and SHA-256:

- `/opt/homebrew/Cellar/ravel/0.1.0/bin/ravel`:
  `1e7f61777da30f69fdf6df8ef132fd50cb5a7420cc71bef65cf46e6565e6b87b`
- `/opt/homebrew/Cellar/ravel/0.1.0/libexec/ravel/ravel-policy`:
  `c6d63fc215c15cd86d8752cc972b5111a71cff9b3b1ae622ef4556c25ac88c18`

Final commands and complete logs under `target/ravel-installed-fixtures-k36/`:

- `task ravel:fixtures RAVEL=/opt/homebrew/bin/ravel`: all 14 checks pass
  (`fixtures-final.log`: six resolver, one installed resolver, four reader and
  three installed result/fixture tests).
- `PATH=/Users/antony/Development/Ravel/target/tools:$PATH task test:grove test:grove-loop`:
  all 147 caller and 385 loop nextest cases pass, with three explicit installed/
  role skips in each default suite; loop doctests contain no executable cases
  (`runtime-final.log`). Bundled prerequisites used Bun 1.4.2.
- `cargo fmt --all --check` and `cargo clippy --workspace --all-targets` pass
  (`format.log`, `clippy.log`).
- Overview `book-check --final --check all` passes: eight roots, all 1,403
  declared source lines reconstructed, none deferred (`book-final.log`). Its
  resolver block, manifest/index and chapter rollups were reconciled together.
- `red.log` / `isolate-red.log` show the expected absent-helper/method failures;
  `scrub-mutation.log` shows the observed behavioral failure after removing
  RAVEL_EXIT_FILE from the scrub; final fixture checks pass after restoration.
- `measure.py`, `before.json`, `after.json`, `digest-comparison.json`: all 2,012
  individually frozen subjects match. Subjects are all tracked inputs, the
  installed Ravel pair, selected tools, built dispatch worker assets, measurement
  instrument and every regular file in the original Grove 22.1.0 keg. The final
  handoff prose and retirement are written after that comparison.

All first-child criteria hold. k37 remains live and owns the atomic caller
switch; k35/k33/k31/k27 stay open. No production caller, prompt, policy, ending
command or installed executable was switched, and no release/install ran.
This session retains its original `harness-dispatch exit` last action.
