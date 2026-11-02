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
