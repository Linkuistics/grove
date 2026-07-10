# review-provider-design-k11

**Kind:** planning

## Goal

Grill `review-provider-research-k10`'s findings to a decision: does grove gain
**per-kind provider profiles** (Route A), **per-kind harness selection** (Route B),
or **neither** — and grow the implementing work leaf if the answer is one of the
first two.

## Context

Read first: `docs/research/cross-family-review-providers.md` (the whole point of
k10), then ADRs `model-per-task-kind` — whose final consequence already sketches
Route A's shape and names its two caveats — and `task-kind-taxonomy`.

The user's motivating want (2026-07-10): run a `review` leaf on a model from a
different family than the one that wrote the code, naming **GLM** and **codex** as
candidates. The ADR-level constraint that shapes any answer: grove rejected a
model **router/proxy** on Max-subscription grounds, and `--model` alone selects a
tier within one family, not a family.

## Questions to grill

1. Route A, Route B, or neither? (A "neither" outcome is a real result: it retires
   the ambition into `model-per-task-kind`'s consequences as a considered-and-
   rejected option, so nobody re-proposes it.)
2. If Route A: is the env-var namespace right — `GROVE_<KIND>_BASE_URL` /
   `GROVE_<KIND>_AUTH_TOKEN` alongside `GROVE_<KIND>_MODEL`? A **secret in an env
   var** is a new class of config for grove; does it belong there, in
   `apiKeyHelper`, or nowhere?
3. Partial profiles: base-URL set but model unset, or token unset. Error, or the
   no-fallback rule extended (any incomplete profile ⇒ no override at all)?
4. The `settings.json`-`env`-outranks-the-driver hazard: warn, detect, or document?
5. If Route B: does `Harness` move from per-grove to per-kind, and what happens to
   `complete`'s `GROVE_CLAUDE_PID` and the claude-shaped `content/prompts/`?
6. Does the decision change **`review`'s discipline** in `TASK-FORMAT.md` — e.g. a
   cross-family reviewer cannot read `CONTEXT.md` conventions as fluently, so the
   review leaf's brief must carry more?

## Done when

Decisions recorded. `model-per-task-kind` reworked in place to state the outcome —
whether that is the provider-profile mechanism or its considered rejection (never a
superseding ADR; `linkuistics:decision-records`). A `## Findings adopted` pointer
added to `docs/research/cross-family-review-providers.md`. Implementing work leaf
grown if there is anything to implement.

## Notes

HITL. This leaf exists because the provider question was **externalized** rather
than absorbed into `task-kinds-model-selection-k6` — the taxonomy did not depend on
it, and a provider profile is strictly additive to the `GROVE_<KIND>_…` namespace.
If the grilling shows the dependency runs the other way, that is a finding worth
recording, not a reason to reopen the taxonomy.
