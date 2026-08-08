# session-kind-methodology-k86

**Kind:** impl

## Goal

Reconcile every canonical methodology surface with the nineteen filename-only
session kinds and the separately versioned viewer contract.

## Context

- Depends on `lifecycle-methodology-k79` and the implemented
  `herdr-session-kind-viewer-k51` contract.
- Primary artifacts: `content/SKILL.md`, `content/TASK-FORMAT.md`,
  `content/driving.md`, examples, and guidance/navigation tests.
- Current task bodies carry no kind or harness routing metadata; distinct
  `research-a` / `research-b` kinds replace vendor harness declarations.

## Done when

- All nineteen kinds, including `research-a`, `research-b`, and driver-reserved
  `finish`, are consistent across the main skill, task format, driving guide,
  examples, and filename grammar references.
- Grow-verb harness flags and body kind/harness guidance are absent.
- Guidance cites filename-only viewer compatibility without coupling
  methodology provisioning to viewer installation or code.
- Focused guidance checks, `cargo fmt --check`, and `cargo test --locked` pass.

## Notes

Taxonomy is one independently reviewable compatibility surface across binary,
methodology, and viewer.
