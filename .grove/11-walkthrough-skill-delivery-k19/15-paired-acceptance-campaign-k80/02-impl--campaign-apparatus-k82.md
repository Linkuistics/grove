# campaign-apparatus-k82

## Goal

Build the minimum auditable execution, access-audit, and blind-scoring seams
needed to run the frozen paired instrument without a comprehensive reusable
harness.

## Context

- Instrument draft: `acceptance-instrument-k81`.
- Historical failure modes: F1-F3 and F9-F13 in
  `evaluation-recovery-k73`.

## Done when

- A bounded operator procedure plus only the small deterministic helpers it
  earns accepts manifest data rather than embedding a surface. If a documented
  human action is simpler, it records exact inputs, procedure, outputs, and an
  operator declaration instead of adding reusable automation.
- Each attempt uses a fresh context and preserved run directory. Control homes
  contain no target skill; enabled homes preload the manifest-pinned bytes
  before the prompt. Preload manifests and prompt digests prove delivery and
  byte-identical user prompts for every pair.
- A preload absence or digest mismatch marks the apparatus invalid and launches
  no acceptance-bearing continuation. A valid preload followed by refusal,
  omission, timeout, or failure to follow instructions is classified solely by
  the frozen post-exposure outcome rule.
- Pair assignment is precommitted and counterbalanced. The two arms of each
  pair execute back-to-back with start/end timestamps under one pair id.
- Replacement eligibility is a deterministic function of preserved exposure
  phase. Proven pre-exposure failures consume the manifest's global resource
  budget; resumption starts with the earliest incomplete assignment, occurs
  only between whole pairs, and cannot depend on arm or observed outcome. The
  first post-exposure outcome is retained.
- The auditor replays the complete attempt history before scoring. An illegal
  replacement marks the affected surface protocol-failed under the frozen
  consequence and cannot be repaired by selecting another attempt.
- Full raw events feed access and validity auditing. Treatment-neutral bundles
  redact explicit arm metadata without discarding behavioral evidence, and
  every scored bundle is sent to two independent blind scorers. A separately
  owned blind resolver applies only the frozen disagreement rule.
- Each scorer records a forced arm guess after scoring and before labels are
  revealed. Guess accuracy is emitted under the supplemental namespace and is
  never a filter or verdict operand.
- Synthesized fixtures and stub executables cover preload mismatch, every
  exposure and replacement branch, resource exhaustion and resumption,
  pair-atomic scheduling, prohibited access, missing/truncated finals, bundle
  redaction, two-scorer invocation, and blind resolution without launching a
  live evaluated model.
- Any scripts follow the repository's applicable language conventions and pass
  their focused checks.

## Notes

The apparatus proves observable request and record boundaries, not operating-
system unreadability. Scoring helpers preserve evidence; they never rewrite raw
events or infer treatment adherence from delivery alone.
