# confined-run-k23

**Reviews:** confined-run-k12
**Creator:** run 477c2f49-6350-4a58-a10b-6cc44f9a9cab

## Goal

Adversarially inspect the committed confinement increment before lifecycle
launch-cutover builds on it. Read the node's contract and the committed source;
find violations or unstated assumptions rather than accepting its producer's
conclusions. This is inspection only.

## Scope and contract

- `docs/specs/standalone-invocations.md`, the area's *Confinement* and *A
  standalone invocation*, and `.grove/10-k12/_confined-run.md` define the result.
- Dispatch's canonical grant refusal protects policy, settings and records,
  including prospective paths, aliases, implicit system reads and grants beneath
  writable roots. Its own executable and selected executable are the exempt
  runtime resources. Check `crates/harness-dispatch/src/confinement.rs` and the
  native backends in `crates/keyed-launch/src/confinement.rs`.
- Selection sees the owner's grants and bounds outside confinement. The
  confined harness gets a minimal environment and fresh run identity, and no
  enclosing launch, exit or display authority. Inspect the dispatch run path
  and `crates/grove/src/standalone.rs` together.
- Grove publishes only after dispatch exit 0, its protected ending file reports
  observed `exit_signal`, its own cancellation is absent, and both jobs and the
  harness group have ended. Test acknowledged failures, surviving writers,
  cancellation at selection/run/publication and nested runs. Check that the
  report cannot be supplied from inside the sandbox and that output reads stay
  attached to the held staging directory.
- The canonical sibling dispatch path in the prompt must remain usable under
  confinement, including a symlinked sibling and Linux's filesystem view.
  `NoninteractiveLaunch` supplies no second completion channel; review the
  optional-channel runner core for changes to ordinary interactive launches.
- The changed `overview`, `keyed-launch` and `grove-llm` books must explain the
  implemented ownership, cancellation and publication contracts. Read their
  worked operations and source-fragment introductions as an editorial axis,
  independently of the validator's byte-equality evidence.

## Evidence to inspect

- Real-command confinement suites in `crates/harness-dispatch/tests/` and
  `crates/grove/tests/standalone.rs`; the runner's confinement, noninteractive,
  process/PTY and wait-order suites.
- `scripts/release-prepare.test.sh` includes a real confined deterministic
  release-notes harness that acknowledges through the prompt's exit command.
- The producer tested on macOS Seatbelt. Linux bubblewrap was not exercised
  there, so do not infer its behavior from the host's passing result.
- Codebase graph access was unavailable because of an incompatible active
  generation; source reads supplied the producer's structural evidence.

## Done when

Findings are anchored to the committed artifact, with the violated contract,
a reproducer or specific reasoning, and severity. If nothing is found, say
which scope was inspected and what platform or evidence limits remain. Follow
the review skill's retirement and integration procedure.
