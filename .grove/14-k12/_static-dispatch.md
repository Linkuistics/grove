# static-dispatch-k12 — brief

## Goal

Deliver a standalone `harness-dispatch` command that evaluates an owner's
TypeScript `routes` policy through the installed, matching compiled worker. The
command inspects the joint choice and runs it by replacing itself with the
configured harness. This is the walking skeleton every later increment extends.

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
- Taskfile tasks build, install and check the pair. `scripts/check.sh` runs the
  package checks, including TypeScript type checking of the SDK and examples.
  `cargo test` obtains the worker deterministically or fails loudly.

## Decomposition

1. `routed-inspection-k13`: the crate, worker, protocol and authority, and
   `inspect` of a static route.
2. `harness-exec-k14`: argv slot expansion, prompt input, program resolution,
   and `run` by exec.
3. `choice-and-refusals-k15`: `--choice` under routes, the actionable-refusal
   contract, help examples and the static starter examples.

Each child leaves the command usable, with behavior its successor needs but does
not have to wait for. Forms owned by later increments must be refused
explicitly: `select`, `loadContext`, `--context`, records and `--policy-env`.
They must not be accepted and ignored.

## Pointers

- Spec sections: `#package-boundary`, `#command-interface`, `#policy-and-choice`,
  `#policy-authority`, `#execution-contract` (only the exec itself here) and
  `#diagnostics`.
- ADRs: `docs/adr/harness-selection-is-owned-by-policy.md` and
  `docs/adr/policy-evaluation-precedes-process-replacement.md`.
- Evidence: the native and integration probes in
  `docs/design/harness-selection-and-execution/runtime-evidence.md` give the
  build flags and the `Bun.plugin` `build.module` registration that worked. The
  probe itself is not an implementation to adopt.
- Workspace conventions: the root manifest's member, version and lint notes.

## Review

This node is expected to end with a `review-impl` naming this node's handle.
Its last leaf cuts that review inside this node. The reason is that the protocol,
the worker location and identity check, and policy authority are the foundation
of every later increment and of the trust boundary. They are cheaper to correct
before four increments build on them.
