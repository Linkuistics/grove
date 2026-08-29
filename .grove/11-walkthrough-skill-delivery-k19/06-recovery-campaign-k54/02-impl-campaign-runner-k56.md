# campaign-runner-k56

## Goal

Build one reusable paired-campaign runner that can execute every frozen case
against an explicit skill digest and preserve a complete assignment record.

## Context

- Measurement draft: `measurement-design-k55`.
- Reuse lessons from the retained historical harnesses, but do not change their
  pinned digests or turn their drift refusals into current-digest coverage.

## Done when

- The runner accepts frozen prompt, fixture, schedule, control template,
  enabled template, skill manifest, and execution identity as data rather than
  embedding one historical case.
- It preserves canonical prompt bytes including the terminal LF, assigns five
  counterbalanced pairs, copies a fresh sealed home and run directory per
  attempt, and records exact pre/post manifests, raw streams, stderr, timing,
  exit state, and exposure phase.
- The control/enabled template delta is exactly the target skill subtree and
  the enabled bytes equal the requested manifest before every attempt.
- Attempt and replacement accounting distinguishes provable pre-exposure
  infrastructure failure from every retained post-exposure outcome and can
  continue later assignments after a shortfall under the measurement design.
- A stub executable exercises every runner branch, record shape, schedule, and
  byte guard without launching a live evaluated model.
- The runner and tests follow the repository's Bash conventions and pass
  `shellcheck`.

## Notes

This leaf produces execution records, not access validity or behavioral scores;
those belong to `campaign-auditor-k57` and the adjudication leaves.
