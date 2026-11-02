# grove-ravel-runtime-fixtures-k94

## Goal

Make Grove's installed-Ravel test seam select its Ravel runtime explicitly.
Then prove the native route on 2.0 for the lifecycle and standalone callers
with fixture configurations.

## Context

- The node brief's "Builds and homes" section gives the two builds and the
  HOME of each route.
- `testing/support.rs` writes every fixture policy as `policy.ts` with schema 2.
  `testing/ravel.rs` copies the 1.1 front and its `libexec/ravel/ravel-policy`
  worker. The 2.0 build installs one executable and no worker.
- Inline schema-2 policies also occur in the lifecycle, activation, standalone,
  loop-driver, native-delivery, session-command, session-result and job tests.
- Ravel's migration table in `docs/specs/configuration-evaluation.md` maps each
  TypeScript policy construct to Rhai.

## Done when

- The test support takes the runtime from an explicit variable, beside
  `GROVE_TEST_RAVEL`. It never infers the runtime from the version output.
  A missing runtime with a 2.0 build fails with a precise message.
- For the 2.0 runtime, each fixture writes `config.rhai` with `SCHEMA_VERSION`
  4. It passes `request.prompt` to the fake agent. For 1.1 it keeps the
  schema-2 `policy.ts`. The copied prefix keeps the layout of its build.
- Six tasks pass against each build: `ravel:resolution`, `ravel:commands`,
  `ravel:job`, `ravel:activation`, `ravel:fixtures` and `test:authority`.
- Each run uses its own temporary HOME. Delivery is disabled for the 2.0 runs.
- A 2.0 lifecycle case observes that `request.prompt`, `request.originalPrompt`
  and the composed mandate are equal byte for byte.
- Three refusals reach the user unchanged and leave the leaf live. They are a
  schema-3 script, a lone `policy.ts` under 2.0 and a settings key of the other
  major.
- `native_skill_delivery` passes against each build. Startup and failure change
  no old or foreign native entry.
- The Taskfile carries the runtime variable through each task that needs it.
  `bash scripts/check.sh` still passes with the default 1.1 PATH.

## Notes

Fix any Grove caller defect that the 2.0 runs reveal. Record it in the
running log. A defect in the Ravel build goes to the Ravel owner through the
k3 brief, not into Grove.

Controls: a fixture that writes `policy.ts` under the 2.0 runtime must fail. A
2.0 run with the 1.1 runtime selected must fail. Watch each fail once.

Record the 2.0 build identity from a routing notice and the `bin/ravel`
SHA-256 of each prefix. Earn a `review-impl` only if the work changes
caller behaviour rather than test wiring.

## Decisions (running log)

**Builds.** On 2026-11-03 the 1.1 keg `bin/ravel` had SHA-256
`c52f990b2059c8bbe8efba10a741d3a0311000ac1712d71e882638f54c3ac1ed`. This session
installed the 2.0 build from `Ravel.skill-directory-delivery` at output commit
`4f831feb4d0f`, with an empty working copy, by `task install PREFIX=<scratch>`.
Its `bin/ravel` had SHA-256
`151635d25169e6a76c7eed0a1de64af661b3d77554c9bbef88ab85a59afb823d`. Both match
the node brief.

**Runtime variable.** `GROVE_TEST_RAVEL_RUNTIME` takes `1.1` or `2.0`. The
support reads the layout only to refuse a mismatch: a 1.x prefix holds
`libexec/ravel/ravel-policy`, and a 2.x prefix has no worker. An unset or empty
variable keeps its established meaning, 1.1, which only a build with the 1.x
worker satisfies. So a missing runtime with a 2.0 build fails with the remedy,
and `bash scripts/check.sh` keeps its default 1.1 PATH with no change.

**Taskfile.** Each task that passes `RAVEL` also passes `RAVEL_RUNTIME` as
`GROVE_TEST_RAVEL_RUNTIME`. Task gives an exported variable precedence, and an
unset `RAVEL_RUNTIME` renders empty, which the support reads as unset.

**One route, two sources.** `testing/ravel.rs` has a `Route` that writes the
same fixed-command selection for either runtime, and a `quote` whose literals
both TypeScript and Rhai read. Cases with their own selection logic carry one
source for each runtime.

**Observation without file writes.** A Rhai script writes no file and spawns
no process. Where a 1.1 policy wrote what it saw, the 2.0 script passes the
same data to the selected command. That command writes the data and then
starts the fake agent. The 2.0 worker has an empty environment and null stdin by Ravel's
contract, and a script can measure neither. Those worker measurements stay
1.1-only, and the 2.0 cases measure what a script can read: granted names
through `ravel::env`.

**Holding selection.** A Rhai `sleep` holds selection. The 2.0 worker is the
child `ravel-rhai-config-worker` of the front, so the interrupt case finds it
in the process table. Probed: SIGINT during a held 2.0 selection ends with
`refused (session_cancelled, stage exec)` and reaps the worker.

**Review assets.** The six creator-lifecycle cases need the review adapter and
the provenance wrapper, which are TypeScript only until
grove-rhai-review-assets-k96. Under 2.0 each case reports that and runs
nothing. k96 removes the guard `review_assets_unported`.

**Defect found by 2.0: host PATH.** `hold_terminal_reset` in
`crates/grove/tests/loop_driver.rs` built its PATH from the test process's
PATH, so two terminal cases started the host's Homebrew `ravel` instead of
the build under test. Under 1.1 the two were the same keg, which hid it. On
2.0 the host 1.1 build refused, because the HOME held only `config.rhai`.

The helper now starts from `support::ravel_path()`. This was a test defect, and
Grove's caller code needed no change.

**Host guard.** Each 2.0 run puts a stub `ravel` first on PATH. The stub exits
86 with `host-ravel-reached`. A case that reaches a PATH `ravel` other than the
copied build then fails, so it cannot pass for the wrong reason. Under
`mise exec`, `command -v ravel` found the stub, which is the positive control.

**Foreign configuration case.** The new loop-driver case covers the three
refusals of the other major on each build. 2.0: `unsupported_version` for
schema 3, `config_format_retired` for a lone `policy.ts`, and
`settings_invalid` for `policyEnv`. 1.1: `unsupported_version` for schema 3,
`policy_missing` for a lone `config.rhai`, and `settings_invalid` for
`configEnv`. The case compares the driver's stderr with the first line and the
remedy that the same build states directly under the same HOME.

**Book.** `crates/grove/src/dispatch/ravel.rs` is in the corpus of the overview
book, so its fragment, source root, ledger rows and roll-up totals moved from
330 to 351 lines. The prose about the 1.x worker controls stays for
grove-ravel-two-admission-k97, which owns the books.

**Controls, each seen to fail once.** A 2.0 build with runtime `1.1`, with the
runtime unset, and a 1.1 build with runtime `2.0` each stopped with the remedy
of `Runtime::for_build`. A fixture that wrote `policy.ts` under 2.0 failed with
`config_format_retired`. The matched builds passed the same case.

**Results.** Against the 1.1 keg and against the 2.0 prefix behind the host
guard, all six tasks passed: `ravel:resolution`, `ravel:commands`, `ravel:job`,
`ravel:activation`, `ravel:fixtures` and `test:authority`. `plugin_fallback`
and `native_skill_delivery` passed against each build, 11 cases each. With the
default 1.1 PATH, `bash scripts/check.sh` passed all 20 principal checks, and
the diff digest was the same before and after the run.

**Bun drift.** The first `check.sh` run failed the caller-asset check before
any test. `scripts/ravel-policy-typecheck.sh` pins Bun 1.4.3, and the owner's
global mise configuration selects Homebrew's latest Bun, now 1.4.4. The passing
run used `mise exec bun@1.4.3 -- bash scripts/check.sh`. This change did not
cause the drift and does not fix it.

**Review.** The work changes test wiring, test support and documentation. No
caller code changed, so the work earns no `review-impl` leaf, and this session
spent no in-session reviewer.
