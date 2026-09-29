# harness-selection-and-execution-k6

## Goal

Cut independently useful implementation increments for harness-dispatch from the
reviewed design, preserving the first release's standalone command, supplied
policy, Grove integration and installed delivery acceptance.

## Context

Read `docs/specs/harness-selection-and-execution.md`, its cited ADRs and the
runtime/source evidence under `docs/design/harness-selection-and-execution/`.
Review `harness-selection-and-execution-k5` and any intervening integration own
design findings; use the resulting current design, not the producer's original
proposals when they differ. The root brief records accepted requirements and
the two process seams. No implementation tree has been pre-built.

## Done when

- The first executable increments can each be demonstrated or tested without
  waiting for a horizontal sibling to make them useful.
- Implementation coverage includes the actual shipped example policy, creator
  adoption/observation workflow, bounded worker trust and cancellation, optional
  Grove slots/scope lifecycle, direct-harness compatibility and release layout.
- Package tasks join the repository Taskfile. Per-target installed static and
  TypeScript smoke tests without another runtime, Linux floor verification and
  documentation/configure-grove acceptance are explicit work with owners.
- Review work is scheduled where justified; no local selector pilot, calibration
  campaign, repository extraction or confined grove-run integration is added.

## Notes

`task check` wraps the existing repository check script. The design viewer runs
through `task design:harness-selection`. The native Bun experiment validates only
the bounded facts in its evidence document; it is not an implementation artifact
or permission to skip process and release checks.
