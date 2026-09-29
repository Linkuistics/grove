# harness-selection-and-execution-k2

**Reviews:** harness-selection-and-execution-k1

## Goal

Independently review the agreed first-release requirements for missing
obligations, internal consistency, falsifiable acceptance conditions and
unintended design constraints.

## Context

Read the committed artifact from `harness-selection-and-execution-k1`, including
its running decisions and verification evidence. The durable boundary is
`docs/adr/harness-selection-is-owned-by-policy.md`; the starting research is
`docs/research/grove-model-effort-routing.md`. The research's sample flags and
types are proposals, not existing APIs.

## Done when

- Findings identify material gaps or contradictions with concrete evidence and
  affected requirements, or explain why no material findings remain.
- The read covers independent use, Grove integration, static and computed
  policy, explicit configuration trust, incomplete-map refusal, simple producer
  provenance, process behavior, records and initial delivery with Grove.
- Each acceptance condition is assessable through the agreed external test
  seams. Settled human choices are distinguished from open design decisions.
- Any required integration is scheduled before `harness-selection-and-execution-k3`
  according to the review procedure. Do not create integration work when there
  are no findings to integrate.

## Notes

The human confirmed this contract and the test seams. The first increment uses
only the original creator's provider for review separation, with the rule owned
by configuration. It stops on incomplete mappings. A working local LLM selector,
its calibration pilot and extraction to a separate repository are follow-up
work. Earlier stronger identity rules in the running log are explicitly
superseded; do not restore them as requirements.

This is an inspection-only read of the committed artifact and recorded evidence.
Produce findings, not fixes, and do not rerun builds, tests, lint or formatting.
