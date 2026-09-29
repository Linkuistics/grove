# harness-selection-and-execution-k1

## Goal

Resolve the remaining requirements for the separate harness-selection and
execution tool, then establish the design/planning work needed to deliver it.

## Context

The root brief preserves the prior discussion and its constraints. The research
at `docs/research/grove-model-effort-routing.md` surveys current model/effort
results, the Grove integration seam, and Jev/oMLX options. The new
`configure-grove` skill supplies the configuration-maintenance workflow.

## Done when

- The independent executable's first deliverable, ownership and integration
  boundaries are agreed with the human.
- Static/computed selection, task context, provider provenance and error/fallback
  behavior have concrete acceptance criteria.
- Test seams are agreed and durable requirements are recorded in the appropriate
  project documentation.
- The next design/planning work is cut at useful session boundaries, with review
  scheduled where needed and no duplicated interrogation of settled requirements.

## Notes

This is the initial requirements leaf, prepared for a fresh Grove session.
The preceding session released the configuration skill and applied personal
model/effort defaults; it did not implement the router or resolve the open
requirements. Do not treat the research's proposed flags or TypeScript types as
an existing API.
