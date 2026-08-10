# session-kind-methodology-k86

**Kind:** impl

## Goal

Reconcile every canonical methodology surface with the nineteen filename-only
session kinds and their task-tree grammar contract.

## Context

- Depends on `lifecycle-methodology-k79`.
- Primary artifacts: `content/SKILL.md`, `content/TASK-FORMAT.md`,
  `content/driving.md`, examples, and guidance/navigation tests.
- Current task bodies carry no kind or harness routing metadata; distinct
  `research-a` / `research-b` kinds replace vendor harness declarations.

## Done when

- All nineteen kinds, including `research-a`, `research-b`, and driver-reserved
  `finish`, are consistent across the main skill, task format, driving guide,
  examples, and filename grammar references.
- Grow-verb harness flags and body kind/harness guidance are absent.
- Guidance states the filename-only task grammar without coupling methodology
  provisioning to an external renderer.
- Focused guidance checks, `cargo fmt --check`, and `cargo test --locked` pass.

## Notes

Taxonomy is one independently reviewable compatibility surface across binary
and methodology.
