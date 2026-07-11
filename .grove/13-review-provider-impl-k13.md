# review-provider-impl-k13

**Kind:** work

## Goal

Implement the per-kind **provider profile** in the loop driver: a `review` (or any)
leaf whose kind has `GROVE_<KIND>_BASE_URL` + `GROVE_<KIND>_AUTH_TOKEN` set (with a
model) launches its `claude` session off that endpoint, on the child process only.
Per ADR `model-per-task-kind` → *Cross-family provider profiles*.

## Precondition (the gate)

**`review-provider-spike-k12` must have returned "Route A clear."** If the spike
found `claude-code#7855` reproduces on grove's interactive launch, this leaf does
**not** implement — it instead reworks ADR `model-per-task-kind`'s *Cross-family
provider profiles* section in place to record Route A as tried-and-blocked (never a
superseding ADR; `linkuistics:decision-records`), updates the research doc's
`## Findings adopted`, commits that, and stops. Check the spike's findings note
first.

## Context

Read first: ADR `model-per-task-kind` (the whole mechanism, esp. the partial-profile
rule and the two hazards), the k12 spike findings, and `src/loop_driver.rs`
(`select_model` ~L207, `launch_session` ~L154, `resolve_kind`, `env_model`) plus
`src/harness.rs` (`Harness`, `model_args`).

**Base is unreleased.** Per-kind model selection (`GROVE_REVIEW_MODEL`, the
five-kind `Kind`) lives on this branch only; the Homebrew `grove-llm` 9.1.0 predates
it. So this builds on branch code, and a release cuts both together — not a
regression to the released binary.

## The change surface (from reading the code)

- **`select_model` returns a profile, not `Option<String>`.** Today it yields
  `Option<String>` (the `--model` value). Widen to something like
  `Option<LaunchModel>` where a profile carries `model` + optional `base_url` +
  `auth_token`. The existing early return (none of the five vars set ⇒ `None`, a
  zero-subprocess byte-identical launch) **must be preserved** — it is load-bearing
  for the common path.
- **Per-kind resolution** reads the kind's three vars: `GROVE_<KIND>_MODEL`,
  `GROVE_<KIND>_BASE_URL`, `GROVE_<KIND>_AUTH_TOKEN` (reuse `env_model`'s
  empty-is-unset treatment). Apply the **base-URL-is-activation-key** +
  **complete-triad-or-nothing** rule: no base-URL ⇒ model-only (unchanged); base-URL
  present with a complete triad ⇒ cross-family profile; base-URL present but triad
  incomplete ⇒ **no override at all** + a stderr diagnostic (do not pass the foreign
  model to the Max endpoint).
- **`launch_session` exports the endpoint on the child only** when a profile is
  active: two `cmd.env` calls — `ANTHROPIC_BASE_URL`, `ANTHROPIC_AUTH_TOKEN` (never
  `ANTHROPIC_API_KEY`) — alongside the existing `--model` arg. Keep the
  `model_args`-empty guard (codex opts out).
- **settings.json `env`-collision detection**, gated to profile-active only: a
  four-file JSON read (managed → local → project → user) for an `env` key naming any
  `ANTHROPIC_*`; warn (never refuse — constraint 5) that the settings block may
  override grove's exported endpoint. Whether this is load-bearing or belt-and-braces
  is settled by k12 experiment 2 — implement per that result.
- **Docs:** the three-var profile, the partial-profile degrade, the
  `ANTHROPIC_AUTH_TOKEN`-not-`API_KEY` reason, and the "cross-family review subagents
  must inherit, don't pin a `model` alias" note go into `--help` (`src/cli.rs`) and
  the README, alongside the existing five model vars. `CONTEXT.md`'s *Per-kind model
  selection* entry gains the provider-profile extension.

## Done when

- The driver activates a cross-family profile for a kind with a complete triad, and
  degrades (model-only, or nothing) with a diagnostic on an incomplete one; the
  no-var common path is byte-for-byte unchanged.
- Tests cover: complete triad → profile; incomplete triad → no override + diagnostic;
  model-only → same-family `--model`; no vars → `None`.
- The settings-`env` collision warning fires only when a profile is active.
- `--help`, README, and `CONTEXT.md` updated. The `## Findings adopted` pointer and
  the ADR reflect "implemented", not "planned".
- Verified against the real driver path (not only unit tests) — a review-kind launch
  with a profile set reaches the foreign endpoint (reuse the k12 rig).

## Notes

AFK work leaf. Cite any Claude Code auth/precedence behaviour to the source at the
decision site (`driving.md`). This is a hard-to-reverse, correctness-sensitive change
(a wrong endpoint silently runs the review on the wrong model) — doubt-pass the
partial-profile logic and the `cmd.env` scoping before it stands.
