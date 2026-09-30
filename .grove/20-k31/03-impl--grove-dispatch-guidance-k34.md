# grove-dispatch-guidance-k34

## Goal

Document, where an owner configures Grove, how to route sessions through
`harness-dispatch` and which configuration owns what. Cover the two inspection
surfaces, the launch-time validation boundary, and the exact remedy when a
delegated mapping is incomplete.

## Context

The requirements put this guidance in usage documentation and in the
configure-grove skill. The skill lives in `plugins/grove/skills/configure-grove/`,
and Grove's usage and configuration references live under `docs/`. Grove's
user-guide coverage tests may pin wording. Update them with the prose, never
around it. The review policy and the creator-line remedy arrive with
`review-policy-k35` and `creator-reference-k38`. Leave room for them without
describing them early.

## Done when

- configure-grove explains activation. The owner points a personal command
  definition at `harness-dispatch run` with the new slots and the prompt, may
  add a literal `--choice`, and creates a personal policy that no install
  overwrites. It explains that Grove's configuration owns the wrapper while
  dispatch policy owns selection. It shows `grove config show` for the wrapper
  and `harness-dispatch inspect` for the selection. It states that Grove's
  pre-authoring check stops at the configured command, so delegated policy can
  refuse at launch after authoring succeeded. It gives the exact remedy for an
  incomplete mapping, and it warns never to grant `GROVE_SIGNAL_FILE` with
  `--policy-env`.
- The Grove usage and configuration references say the same thing for readers
  outside the skill, and link the dispatch usage documentation.
- `harness-dispatch --help` carries a Grove example that matches the
  documentation.
- The spec's notice states what is delivered. The node brief's `Done when`
  holds.
