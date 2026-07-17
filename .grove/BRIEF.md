# grove-harness-switch — brief

## Goal
Implement the codex/pi harness switch: grove drives a month-long
codex+gpt-5.6-sol vs pi+Kimi-K3 trial with K3 reviewing everywhere, after
which the losing subscription is cancelled. The design and the task-by-task
implementation plan are approved and committed; this grove executes them.

## Done when
`grove --version` reports v12.0.0 live via brew; one grove per side is stamped
and has run a full task cycle including an observed review-leaf reroute to pi;
the Anthropic subscription is cancelled; the trial clock is running.

## Decomposition
Positions 02-09 are the grove code changes (one plan task each: TDD, one
focused commit). 10-12 are machine config, 13 the skills-repo reviewer
reference, 14 a fresh-context review of the whole branch, 15 release + stamp +
live verification. Order encodes dependencies: registry before stamp/routing,
routing before config, everything before review, review before release.
- 02 harness-registry — pi row, codex --profile, skills_dir, known_names (plan Task 1)
- 03 stamp-explicit — persist explicit --harness in the stamp (plan Task 2)
- 04 harness-pid — GROVE_HARNESS_PID rename + one-release fallback (plan Task 3)
- 05 scoped-model-envs — GROVE_<HARNESS>_<KIND>_MODEL precedence (plan Task 4)
- 06 kind-routing — GROVE_<KIND>_HARNESS + per-harness bin seam (plan Task 5)
- 07 provision-sweep — multi-harness skill provisioning + guards (plan Task 6)
- 08 neutral-text — harness-neutral help + SKILL.md wording (plan Task 7)
- 09 changelog-verify — v12.0.0 changelog; fmt/clippy/test clean (plan Task 8)
- 10 codex-profiles — sol-xhigh / sol-high profiles + smoke test (plan Task 9)
- 11 pi-kimi-wiring — pi-provider-kimi-code + live K3 round-trip (plan Task 10)
- 12 zshenv-cutover — GROVE_* env rewrite from 11's recorded model id (plan Task 11)
- 13 reviewer-spawns-skill — doubt-driven-development spawn reference (plan Task 12)
- 14 branch-review — fresh-context adversarial read of the branch vs spec+plan
- 15 release-stamp-trial — merge, release, live-verify, stamp, cancel (plan Task 13)

## Pointers
- Spec: docs/superpowers/specs/2026-07-18-codex-pi-harness-switch-design.md
- Plan — the how; every code/config leaf executes its named plan task
  VERBATIM (steps, code, and commit messages included):
  docs/superpowers/plans/2026-07-18-codex-pi-harness-switch.md
- Glossary terms in play: Global skill provisioning (07 must reconcile this
  CONTEXT.md entry — it still describes the single-target ~/.claude model),
  Grove name (see CONTEXT.md)
- Test seams these leaves share: GROVE_HARNESS_BIN (and the new per-harness
  GROVE_HARNESS_BIN_<NAME>), GROVE_LLM_BIN, GROVE_SKILL_DIR — fake-binary
  integration tests in tests/loop_driver.rs; every env-touching test holds its
  file's ENV_LOCK mutex

## Notes
- This grove dogfoods: the loop runs on the *installed* brew grove (v11,
  claude harness) while this worktree builds v12. The trial routing only takes
  effect once 10-12 land and 15 ships the binary.
- Execute plan tasks as written; a deviation discovered mid-leaf is folded
  back into the plan file in the same commit, so plan and code never drift.
- 11 has an explicit STOP condition (plan Task 10 step 4): a failed live
  round-trip through the Kimi sub is a design change needing the human — the
  fallback shell is a decision, not a workaround.
- 15 contains user actions (subscription cancellation): propose, never perform.
