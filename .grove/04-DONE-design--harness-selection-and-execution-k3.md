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

## Decisions (running log)

Package and runtime: name the independent command/package `harness-dispatch`.
Use a Rust front process for admission, bounded evaluation, records and final
Unix exec, with a separately compiled Bun policy worker shipped in the same
installation. The worker receives neither the launch prompt nor caller-reserved
completion environment. This avoids rebuilding a JavaScript host API around a
small embedded engine and keeps the foreground process identity through handoff.
Bun 1.4.2 on this macOS arm64 host compiled and ran a TypeScript policy with a
relative TypeScript import using only `/usr/bin:/bin` on PATH. Disabling all four
compile-time autoload switches kept hostile cwd `.env` and `bunfig.toml` inert;
the otherwise identical default build loaded both (the positive control).
Linux delivery still requires the agreed per-target execution checks.

Artifact and review boundary: the area specification will own the protocol;
ADRs will hold the policy ownership and runtime/handoff trade-offs. An independent
`review-design` leaf will examine the completed design before planning, so no
competing in-session reviewer is used. The existing command and Grove launch
process seams remain the acceptance boundary. Formal modelling would not verify
the principal uncertainties here (runtime discovery, exec, environment and
delivered bytes); executable probes and process-boundary tests answer them.

Protocol: use the same TypeScript entry for static exact routes and an optional
computed selector, with a named policy export, typed joint candidates and
candidate-ID-only results. Rust validates explicit-choice agreement, context
limits and the selected executable. Inspection is a proposal with measured
context and resolved authority, not a side-effect-free promise or cached launch.
The default bound is 30 seconds and 256 KiB context; absolute ceilings and refusal
rules are in the enduring spec.

Identity: add optional lifecycle kind/task-file/task-ID/task-scope slots. The
stable handle is namespaced by a lazily allocated dispatch UUID in the existing
workspace control area, retained across sessions and rotated on authoritative
root recreation. It conveys no epoch authority. Manual root replacement has an
explicit reset, rather than treating unpinned device/inode numbers as a durable
proof. Direct-harness routes do not allocate dispatch state.

Provenance: register one creator explicitly, from an externally confirmed
producing run or a labelled owner declaration. First registration persists across
re-invocations; correction requires its revision and a reason. This deliberately
costs an owner/observer registration step for reviews rather than pretending that
a pre-exec attempt identifies the invocation which actually produced an artifact.
Generic creator lookup and versioned later-observation commands remain outside
the supplied policy's provider comparison. SQLite transactions make required
record persistence and creator-revision checks one pre-handoff boundary.

Runtime admission: all four Bun autoload features are disabled, worker environment
is explicitly constructed, and the worker runs in a private cwd. The guarded
probe still executed a BUN_OPTIONS preload, establishing why pre-start Rust
scrubbing is necessary. The package supplies the compiled worker and uses no
runtime from PATH. Arbitrary JavaScript/filesystem/network policy is in scope;
detached policy services and a general child-process SDK are not needed in v1.

## Verification

`task check` exited 0: all eleven principal checks passed, including the complete
Rust suite and final validation of all six walkthrough books. The jj snapshot of
the tracked inputs stayed at `09189c2871a225cf7164d06830ba3fe7c2e611b1` before and
after the run. Those inputs include the source, fixtures, manifests, check
scripts, Taskfile and documentation. Output is retained at
`/tmp/grove-harness-selection-k3-check.log`. This note and the retirement rename
follow the run and change no checked implementation or design contract.

The new preview Taskfile entry was run on port 8770 and served the expected
manifest; its default port command was also inspected. Manifest/source/update
links validated, and all 27 local links in the main design/ADR/evidence documents
resolved. A native jj-diff whitespace check passed. Safari rendered all three
views after a sequence-label syntax repair, and a fresh creator deep link reached
the intended view. The desktop light presentation was inspected; mobile/dark
appearance was not measured. The bounded Bun runtime results and limitations are
in the enduring evidence document. No new production command or Linux delivery
is claimed as implemented or verified by this leaf.

The acceptance table was checked against the root brief's cases. The design
review `harness-selection-and-execution-k5` is queued before planning
`harness-selection-and-execution-k6`; actionable review findings must insert
integration before planning. Both grow operations validated their configured
kinds. These live siblings mean there is no ancestor close to cascade.
