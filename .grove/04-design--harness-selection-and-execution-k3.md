# harness-selection-and-execution-k3

## Goal

Design the first useful increment of the independent selection-and-execution
command, producing an enduring area specification and a coherent ADR set. Keep
it usable outside Grove and deliver it with Grove initially.

## Context

Synthesize the settled decisions from `harness-selection-and-execution-k1` and
the dispositions of review `harness-selection-and-execution-k2` recorded in
`harness-selection-and-execution-k4`. The current root brief incorporates those
repairs and the human's adoption, validation and runtime-delivery choices; do not
treat the review's proposed resolutions as accepted requirements. The durable boundary is
`docs/adr/harness-selection-is-owned-by-policy.md`; the proposals in
`docs/research/grove-model-effort-routing.md` are starting evidence rather than
an API to preserve.

Ground Grove integration in `docs/CONFIGURATION.md`, the existing ADRs named in
the root brief, `crates/grove-loop/src/session_config.rs`,
`crates/grove-loop/src/loop_driver.rs`, and the tests in
`crates/grove-loop/tests/session_config.rs` and `crates/grove/tests/loop_driver.rs`.
Inspect the existing installation and release workflow when choosing delivery.

## Done when

- The enduring specification accounts for every agreed acceptance case and
  records the human-agreed test seams. The ADR set explains the consequential
  choices without duplicating contracts.
- The design chooses the command/package name, implementation and TypeScript
  runtime, package boundary and delivery on Grove's supported release targets.
- The generic request, joint choice, static mapping and computed TypeScript
  interface are concrete, including bounded context, explicit choice inputs,
  configuration ownership and precedence, inspection, actionable errors and
  refusal when selection is incomplete or invalid.
- A simple artifact-associated execution record lets policy discover the
  original creator's provider. The supplied configuration owns provider
  separation; the generic command has no hard-coded review rule. Execution and
  outcome records distinguish actual evidence, failures and unknown outcomes.
- The Grove adapter passes the authoritative kind, optional task and unchanged
  prompt while preserving existing direct-harness configurations. The design
  accounts for cwd, terminal, cancellation, exit status and completion authority
  before settling how TypeScript is hosted.
- The next review and planning work is scheduled at useful session boundaries.
  Planning will cut independently useful increments against this design; this
  leaf does not pre-build an implementation tree.

## Notes

The human approved the consolidated requirements. Do not repeat the interview
or treat deferred local inference, a selector pilot, calibration or repository
extraction as prerequisites. Keep provenance proportionate to the rare review
case: no separate model comparison or accumulated multi-author exclusions.

Design choices remain open where the requirements leave them open. Resolve
them against current source and executable runtime evidence, preserving the
agreed command and Grove launch test seams.
