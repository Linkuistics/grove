# static-dispatch-k12 — brief

## Goal

Deliver a standalone `harness-dispatch` command that evaluates an owner's
TypeScript `routes` policy through the installed, matching compiled worker,
within the whole-selection deadline. The command inspects the joint choice. It
runs it by committing the required handoff record and then replacing itself with
the configured harness. This is the walking skeleton every later increment
extends.

## Done when

- A caller with no Grove binary, configuration or task tree supplies a kind and
  a prompt. It can inspect and run one statically routed candidate from a
  temporary personal policy or an explicit `--config`. A fake harness receives
  the exact argv, the caller's cwd and descriptors, and exits natively.
- The Rust package `harness-dispatch` builds independently and depends on no
  Grove, task-tree or jj crate. It inherits the workspace version and lints.
  The worker is compiled with Bun 1.4.2 and all four no-autoload switches. It
  registers every documented package specifier that exists so far as an
  embedded virtual module, and it verifies its protocol and build identity
  before evaluating anything.
- The worker is found only from the real installed front executable, never from
  PATH or the cwd. It starts in a private empty directory with null stdin, a
  fresh environment, captured diagnostic streams and a private framed channel.
  These controls are built here. `evaluation-boundary-k27` later proves them
  with firing configurations.
- Inspection shows the policy source and its authority, the policy version, the
  candidate's provider, model and effort, the explicit choice, the reason,
  executable resolution, expanded argv and timing. It does so in both `--json`
  schema version 1 and human form. Refusals carry the spec's stable codes,
  stages, remedies and exit results.
- The whole-selection deadline holds from worker start: 30 seconds by default,
  1 to 120 through `--timeout-ms`. A worker spinning at import, or awaiting a
  promise that live work keeps pending, is killed after at most one second of
  grace and reaped. The command exits 124 and launches nothing.
- Every `run` commits one handoff record before exec, in a versioned local
  SQLite store with bundled SQLite. The default directory is
  `~/.local/state/harness-dispatch`, and `--state-dir` replaces it. A failed
  commit exits 4 and launches nothing, and a pre-commit refusal creates no run.
  The harness receives `HARNESS_DISPATCH_RUN_ID` and
  `HARNESS_DISPATCH_STATE_DIR`, and the `runId` slot works. An attempt is never
  a success, an exec failure is appended to its attempt, and committed launch
  fields never change. `record show` exports the run.
- Taskfile tasks build, install and check the pair. `scripts/check.sh` runs the
  package checks, including TypeScript type checking of the SDK and examples.
  `cargo test` obtains the worker deterministically or fails loudly.

## Decomposition

1. `routed-inspection-k13`: the crate, worker, protocol and authority, and
   `inspect` of a static route.
2. `harness-exec-k14`: argv slot expansion, prompt input, program resolution,
   and the plain exec behind `run`.
3. `selection-deadline-k44`: the whole-selection deadline, hard kill, reaping
   and exit 124.
4. `handoff-records-k24`: the store, the run ID, the required pre-exec commit,
   exported run identity, the exec-failure detail and `record show`.
5. `deadline-test-load-race-k45`: make the never-identifies deadline test's
   evidence reliable under load, cut by `handoff-records-k24` when it recurred.
6. `choice-and-refusals-k15`: `--choice` under routes, the actionable-refusal
   contract, help examples and the static starter examples.

Each child leaves the command usable, with behavior its successor needs but does
not have to wait for. The node is the release boundary. `run` becomes a
delivered form only with its required record, and evaluation is bounded before
the node closes (review `harness-selection-and-execution-k42`, findings F2 and
F3). Forms owned by later increments must be refused explicitly: `select`,
`loadContext`, `--context`, `record observe`, `host.run` and `--policy-env`.
They must not be accepted and ignored.

## Pointers

- Spec sections: `#package-boundary`, `#command-interface`, `#policy-and-choice`,
  `#policy-authority`, `#execution-contract` (the deadline and the exec itself
  here), `#records-and-outcomes` (the handoff commit and `record show`), the
  whole-selection and lock-wait rows of `#bounded-context`, and `#diagnostics`.
- ADRs: `docs/adr/harness-selection-is-owned-by-policy.md` and
  `docs/adr/policy-evaluation-precedes-process-replacement.md`.
- Evidence: the native and integration probes in
  `docs/design/harness-selection-and-execution/runtime-evidence.md` give the
  build flags and the `Bun.plugin` `build.module` registration that worked. The
  probe itself is not an implementation to adopt.
- Workspace conventions: the root manifest's member, version and lint notes.

## Review

This node is expected to end with a `review-impl` naming this node's handle.
Retiring the node's last leaf closes it. That leaf cuts the review as the node's
sibling, directly after it and ahead of the next increment. Inside the node, a
review would keep the node open, so no session would finish the producer it
reviews (spec `#identity-and-creator`). The reason for the review is that the
protocol, the worker location and identity check, policy authority, the
deadline's hard kill and reaping, and the durability of the pre-exec commit are
the foundation of every later increment and of the trust boundary. They are
cheaper to correct before later increments build on them.
