# baseline-source-fragments-k39


## Goal

Run and score the five frozen external-source inventory and fragment-design
repetitions.

## Context

- Frozen contract: `docs/evaluations/writing-code-walkthroughs/baseline/rubric.md`.
- External source: `targets/ocaml/check_floor.ml` at the pinned APIAnyware
  revision and digest recorded by the rubric.
- Preserve outcomes under `docs/evaluations/writing-code-walkthroughs/baseline/source-fragments/`.

## Done when

- Five fresh contexts receive only the frozen external file snapshot and prompt.
- Each outcome is scored for exact inventory, concept order, fragment ownership,
  reconstruction, and mechanical validation.
- Access manifests and raw answers make the external target and contamination
  boundary auditable.

## Notes

Fail rather than silently refreshing the fixture if its source digest differs
from the frozen rubric.

## Decisions (running log)

The fixture and frozen rubric matched their predeclared digests, but every one
of the five planned repetitions exhausted its initial run and two replacement
attempts without a final assistant message. All 15 attempts failed DNS/network
transport and timed out after 1,200 seconds. The frozen invalid-run rule makes
the third invalid attempt terminal, so the leaf reports a five-sample shortfall
with no scores or classifications instead of selecting substitutes or running a
fourth attempt.
