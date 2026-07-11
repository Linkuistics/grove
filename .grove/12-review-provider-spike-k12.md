# review-provider-spike-k12

**Kind:** prototype

## Goal

Resolve — empirically, on this machine — the two unknowns that could invalidate
cross-family Route A before any driver code is written (`review-provider-design-k11`
decision 5). Deliberately throwaway probing: the deliverable is the *reaction*
(does Route A survive?), not surviving code.

## Context

Read first: ADR `model-per-task-kind` → *Cross-family provider profiles* (the
route this gates), and `docs/research/cross-family-review-providers.md` Q3 (the
`#7855` bug) and Q4 (the `settings.json`-`env` precedence gap).

HITL — needs a human to supply a real cross-family token (Z.ai coding-plan or any
Anthropic-Messages-compatible endpoint) and to observe the interactive session.
Do **not** commit any token; use a throwaway shell export, and scrub it from the
findings note.

## The two experiments

1. **`claude-code#7855` — does `ANTHROPIC_AUTH_TOKEN` break an *interactive*
   session?** This is the route-killer. Launch an interactive, TTY-owning `claude`
   with `ANTHROPIC_BASE_URL` + `ANTHROPIC_AUTH_TOKEN` set on that process only
   (mirroring how the driver's `Command::env` would set them), pointed at a real
   third-party endpoint with a valid `--model`. Confirm it reaches the model and a
   trivial turn completes — *not* an auth error. grove's review session is exactly
   this shape, so if it reproduces, Route A is **blocked as specified**.
2. **The `settings.json` `env`-block precedence.** Plant an `env` block with a
   *different* `ANTHROPIC_BASE_URL` (a sentinel that would visibly fail or route
   elsewhere) in one settings scope, then launch with the process-env base-URL set
   the driver's way. Observe which one wins. This settles the undocumented
   `Command::env`-vs-settings-`env` question (research Q4) that decides whether
   grove's detect-and-warn is a nicety or load-bearing.

Keep both minimal — a couple of shell one-liners and one interactive launch each.
This is a spike, not a harness.

## Done when

- A short findings note (append to `docs/research/cross-family-review-providers.md`
  under a `## Spike results (k12)` section, or a sibling note) records, for each
  experiment: the exact command, the observed behaviour, and the verdict — with the
  token redacted.
- The verdict is unambiguous: **Route A clear**, or **Route A blocked** (with which
  experiment blocked it).
- If blocked: say so plainly; `review-provider-impl-k13` will then record the
  rejection instead of implementing.

## Notes

Prototype = throwaway. Reset any planted `settings.json` `env` block and `unset`
the token afterwards; leave the machine as found. The value is the observation, not
the artifact. If neither endpoint/token is available to the human at session time,
that is itself a finding — record "unverified, blocked on credentials" and let k13
decide whether to proceed on inference or hold.
