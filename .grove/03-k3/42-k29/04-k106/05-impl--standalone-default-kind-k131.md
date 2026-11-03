# standalone-default-kind-k131

## Goal

Give `grove run` a default kind `standalone`, and remove the promise that a
standalone run reports missing guidance. The human decided both during
confined-harness-trials-k128.

## Context

- The record sections "missing-kind-confined in Codex, repaired" and "Where a
  durable repair belongs" of `docs/common-loading-acceptance-record.md`.
- `crates/grove/src/standalone.rs` and `crates/grove/tests/standalone.rs`.
- The sentences "It reports the missing guidance and runs the force-stop
  command" in `docs/USAGE.md` and `docs/specs/standalone-invocations.md`.
- The Confined standalone row and the paragraph on
  `crates/grove/tests/standalone.rs` in `docs/specs/codex-skill-provisioning.md`.

## Done when

- `grove run` accepts no kind. Outside an active grove, it then routes the
  kind `standalone`. An explicit kind keeps its present behaviour. Help text,
  `docs/USAGE.md` and the standalone specification name the default and its
  condition.
- Inside an active grove, a bare `grove run` does not use `standalone`. The
  leaf asks the human two questions before it changes code. How does
  `grove run` detect an active grove? What does a bare `grove run` do inside
  one? The answers go into the running log and the specification.
- The kind `standalone` requires no skill. It exists so that the owner's
  policy can select a harness, a model and an effort for a standalone
  invocation. The prompt of a standalone run gains no load instruction and no
  missing-guidance rule, for any kind.
- No document promises that a standalone agent reports missing guidance. The
  Confined standalone row keeps its sandbox part: native trees stay unreadable
  and Grove adds no grant. The agent part becomes an explicit gap by the
  human's decision, with `missing-kind-confined-20261103T062817Z` as its
  evidence.
- The standalone fixture states what it proves: Grove's interpretation of a
  force-stop. It no longer stands as evidence of agent behaviour.
- The record's triage gives `missing-kind-confined` in both harnesses its
  decision. Grove's full `bash scripts/check.sh` passes.

## Notes

The human's words: "standalone should not *require* a skill. it is primarily
so that the harness/model/effort can be selected for standalone invocations.
The fact that the skill might or might not exist shouldn't matter."
And later: "The use of `standalone` as a default kind should only be used if
there is no active grove."

The owner's Ravel policy needs a route for `standalone` before a bare
`grove run` launches. Say so in the documentation. Do not change the owner's
settings.
