# document-model-selection-k4

**Kind:** work

## Goal

Document the per-kind model-selection feature for users: the two env vars, the
"unset ⇒ inherit your default" behavior, and the caveat that an in-session
`/model` switch does **not** persist across relaunch. Verify the whole feature
end-to-end.

## Context

- ADR: `docs/adr/model-per-task-kind.md` — the source of truth; docs summarize
  it, don't restate the rationale.
- Glossary: `CONTEXT.md` already carries the **Per-kind model selection** term
  (added in planning) — keep user docs consistent with it.
- Where user-facing grove docs live: `README.md` (repo), `content/` (the
  embedded methodology / grove skill), and `grove do --help` / CLI help text.
  Find the existing spot where env/config knobs are documented and add there;
  follow `linkuistics:cli-tool-design` (examples, actionable text).
- Prior leaves `grove-llm-kind-verb-k2` and `driver-model-selection-k3` must be
  landed first (this documents shipped behavior).

## Done when

- `GROVE_PLANNING_MODEL` / `GROVE_WORK_MODEL` are documented where users look
  (README and/or `grove do --help`), with a copy-pasteable example
  (`GROVE_PLANNING_MODEL=opus GROVE_WORK_MODEL=sonnet grove do <name>`).
- The **"`/model` doesn't persist across relaunch"** caveat is stated plainly in
  the user-facing docs — the launch model is a per-kind default re-applied each
  task; an in-session `/model` override is intentional but session-scoped.
- The "unset ⇒ inherit your existing default, no `--model` passed" behavior is
  stated (so users aren't surprised grove is a no-op until they opt in).
- If the grove methodology `content/` mentions the loop/model, it's consistent
  (no contradiction with the ADR). Only add a note if it earns its place — the
  optional mid-session-switch guidance line stays lazy (constraint 4).
- Verify end-to-end with the real binary (`/verify` or a recording
  `GROVE_HARNESS_BIN`): confirm `--model` is passed per kind and absent when
  unset — i.e. the docs match observed behavior, not just intent.

## Notes

- No new ADR — this is user docs + verification. The durable rationale already
  lives in `docs/adr/model-per-task-kind.md`.
- Keep it terse; the ADR carries the depth. This leaf makes the feature
  discoverable and confirms it works.
