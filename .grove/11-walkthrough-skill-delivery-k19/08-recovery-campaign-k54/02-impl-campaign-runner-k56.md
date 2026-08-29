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
  attempt, executes each pair's two arms back-to-back, and records exact pre/post
  manifests, raw streams, stderr, per-arm start/end timestamps under the pair
  id, exit state, and exposure phase.
- The control/enabled template delta is exactly the target skill subtree and
  the enabled bytes equal the requested manifest before every attempt.
- A deterministic replacement gate maps the recorded exposure phase to
  replaceable or retained. Proven pre-exposure failures consume the frozen
  sequence of automatic, globally bounded execution windows. A window begins a
  pair only with its full frozen reserve, and any exposure makes that pair
  atomic: its two arms complete back-to-back or the pair becomes terminally
  unavailable. Resumption between pairs chooses the earliest incomplete pair in
  frozen schedule order; the first post-exposure outcome ends its arm's
  assignment and is never replaced.
- A stub executable exercises every exposure phase, replacement decision,
  automatic resource-window stop/resume, mid-pair reserve exhaustion, runner
  branch, record shape, adjacent pair schedule, timestamp, and byte guard
  without launching a live evaluated model.
- The runner and tests follow the repository's Bash conventions and pass
  `shellcheck`.

## Notes

This leaf produces execution records, not access validity or behavioral scores;
those belong to `campaign-auditor-k57` and the adjudication leaves.
