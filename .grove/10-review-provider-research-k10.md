# review-provider-research-k10

**Kind:** work

<!-- re-kind to `research` once task-kinds-impl-k9 lands; the enum does not yet
     accept the label, so `leaf-add --kind research` would have errored. -->

## Goal

Establish, with citations, what it would actually cost to run a `review` leaf on a
model from a **different family** than the one that produced the work — so that
`review-provider-design-k11` can pick a route instead of guessing. Output:
`docs/research/cross-family-review-providers.md`.

## Context

`review` earns its place as a kind on fresh-context adversarial reading alone
(ADR `task-kind-taxonomy`); cross-family is an *amplifier*, not its justification.
So this research decides how much to pay for the amplifier, and a "neither route is
worth it" finding is a perfectly good answer.

Two candidate routes, established as **alternatives, not complements**:

- **Route A — a different endpoint, same harness.** `claude` reads
  `ANTHROPIC_BASE_URL` / `ANTHROPIC_AUTH_TOKEN` fresh from the process environment
  at launch, and passes `--model` through **verbatim** to a non-Anthropic host. The
  driver already sets child env (`cmd.env`, `src/loop_driver.rs`), so this is ~10
  lines: `GROVE_<KIND>_BASE_URL` / `_AUTH_TOKEN` alongside `GROVE_<KIND>_MODEL`
  (ADR `model-per-task-kind`, final consequence).
- **Route B — a different harness.** `Harness` already carries `exec_bin` +
  `model_args`, but is selected **once per grove** (`SelectMode::Single`); codex has
  `model_args: &[]`, `content/prompts/*.md` are claude-shaped, `complete.rs` reads
  `GROVE_CLAUDE_PID`, and codex has no grove skill.

Already verified (code.claude.com/docs, via `claude-code-guide`, 2026-07-10) — do
not re-derive, but **do** spot-check anything load-bearing:

- `--model <arbitrary>` is passed through verbatim when `ANTHROPIC_BASE_URL` points
  at a non-Anthropic host; no validation against a Claude alias table.
- Env is read fresh at process launch — **but** `~/.claude/settings.json`'s `env`
  block outranks shell exports, so a user's settings can silently beat the driver.
- The gateway credential outranks the Max subscription for that session. Remote
  Control is disabled (v2.1.196+); prompt-cache TTL differs; 1M-context / Fable
  entitlements do not transfer.

## The downstream questions this research must answer

Named for its one consumer, `review-provider-design-k11`, which must choose a route
and the smallest `loop_driver` change that implements it:

1. **Route A fidelity.** Pointed at a GLM-class Anthropic-compatible endpoint, does
   `claude` retain (a) tool use / file edits, (b) **skill loading** — the grove skill
   itself — (c) subagents, and (d) the `grove-llm complete` handshake (it kills by
   `GROVE_CLAUDE_PID`, so it should be harness- and endpoint-agnostic — confirm)?
   Which of these degrade, and does a degraded one break the loop or just the review?
2. **The market.** Which providers actually expose an Anthropic-Messages-compatible
   endpoint today (Z.ai GLM, Moonshot Kimi, others)? Auth shape, model ids, context
   window, price. The user named **GLM** specifically — start there, but report the
   set.
3. **Off-subscription blast radius.** Is it truly per-process? Can a `review`
   session run off-subscription while the *next* `work` session returns to Max with
   no residue (no written settings, no cached auth)?
4. **The settings.json hazard.** If a user has `ANTHROPIC_BASE_URL` in their
   settings `env` block, the driver's per-kind profile loses silently. Can grove
   detect this cheaply and warn? Is there a launch flag that wins?
5. **Route B end-to-end cost.** What does codex-as-review-harness actually require —
   session-name flag, model flag, a prompt equivalent, a grove-skill equivalent, and
   whether `complete`'s PID kill survives? Give a change list, not an impression.
6. **The null route.** Claude Code subagents (`Agent` tool, `model` param) and
   `linkuistics:doubt-driven-development` give fresh-context adversarial review
   **within** the family. Is same-family fresh-context review enough of the benefit
   that cross-family is not worth either route's cost? Argue it, don't assume it.

## Done when

`docs/research/cross-family-review-providers.md` exists and answers 1–6, with:

- **A citation per claim**, primary sources preferred (vendor docs, Claude Code
  docs, source). No claim from memory.
- **A walk-away check**: with per-kind provider profiles removed entirely, what
  still works? (Expected: `review` leaves launch on `GROVE_REVIEW_MODEL` or the
  user's default — the kind keeps its value.) State what is genuinely lost.
- **Missing sources recorded.** Where a search finds no primary source, say so
  explicitly rather than inferring — the absence is itself a finding (`driving.md`).
- A closing **Synthesis for `review-provider-design-k11`** section answering, in one
  place: which route, at what cost, and what would change grove's mind.

## Notes

AFK: no grilling, no tree growth. If the research surfaces a *decision*, record the
question for k11 rather than answering it.

Findings that change a decision get cited in the relevant ADR's rationale
(`model-per-task-kind`'s provider-profile consequence); the rest stay here with a
pointer. Add a "Findings adopted" pointer back once k11 lands (`driving.md`).
