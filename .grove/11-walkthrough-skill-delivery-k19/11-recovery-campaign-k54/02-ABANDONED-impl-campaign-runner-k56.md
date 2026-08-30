# campaign-runner-k56

## Goal

Build one reusable historical-schedule runner that executes the exact historical
instrument against a pinned skill digest and preserves complete records.

## Context

- Measurement draft: `measurement-design-k55`.
- Reuse lessons from the retained historical harnesses, but do not change their
  pinned digests or turn their drift refusals into current-digest coverage.

## Done when

- The runner accepts frozen prompt, fixture, schedule, control template, enabled
  template, skill manifest, execution identity, and historical rule manifest as
  data rather than embedding one case.
- It verifies that prompt, fixture, command shape, sampling, replacement, and
  record contracts equal the historical bytes and refuses every unrecognized or
  supplemental field.
- It preserves canonical prompt bytes including the terminal LF, assigns five
  historical ABBA/BAAB assignments, copies a fresh sealed home and run directory
  per attempt, and records the exact historical pre/post manifests, raw streams,
  stderr, timing, exit state, and access surface.
- The control/enabled template delta is exactly the target skill subtree and
  the enabled bytes equal the requested manifest before every attempt.
- Before any evaluated attempt, the runner enumerates every model-interface
  surface reachable from the sealed control and enabled templates under the
  frozen command shape and proves that the surfaces already forbidden by
  `rubric.md:66`-`69` and `rubric.md:89`-`93` are absent, including MCP resource
  listing, web search, and external document connectors. The enumeration and
  proof are apparatus conformance with the frozen access boundary, not a new
  prompt, row, sampling rule, threshold, or endpoint, and are emitted for the
  joint freeze manifest.
- It applies the historical rubric's invalid-run predicates and two-replacement
  ceiling exactly. It implements no exposure-gated replacement or resource-
  window alternative.
- A stub executable exercises historical replacement exhaustion, every runner
  branch and record shape, ABBA/BAAB schedule, byte guard, and refusal of
  supplemental fields without launching a live evaluated model.
- The runner and tests follow the repository's Bash conventions and pass
  `shellcheck`.

## Notes

This leaf produces execution records, not access validity or behavioral scores;
those belong to `campaign-auditor-k57` and the adjudication leaves. Recovery
no-skill records are labelled contemporaneous comparators, never pre-skill
baseline records.
