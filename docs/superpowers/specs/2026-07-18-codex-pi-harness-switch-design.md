# Codex/Pi harness switch — design

**Date:** 2026-07-18
**Status:** live — shipped in grove v12.0.0; trial running 2026-07-18 to
~2026-08-18

## Context and goals

The user is retiring the Anthropic subscription. Two axes drive the replacement:

1. **Cost/quota** — subscriptions beat metered API at daily-driver volume.
2. **Model mobility** — the ability to trial different models for driving and
   review, across harnesses (codex, pi, and future candidates), without
   re-plumbing anything.

Subscription fences shape the architecture: the OpenAI sub is licensed only
inside codex; the Kimi Code sub is open to third-party harnesses
(Anthropic/OpenAI protocol at `https://api.kimi.com/coding/`). Kimi K3
(released 2026-07-16) benchmarks at or near GPT-5.6-sol on agentic coding, so
it is a driving-model candidate, not merely a reviewer.

## Decisions

- **Month-long A/B trial** of two driving stacks, stamped per grove:
  codex + gpt-5.6-sol (OpenAI sub) vs pi + K3 (Kimi Code sub). At the end,
  flip the losing groves' stamps and cancel the losing subscription.
- **K3 is the reviewer everywhere** — both review seams (grove review-kind
  leaves and doubt-driven-development fresh-context checks), on both sides of
  the trial, billed to the Kimi sub.
- **Pi is the K3 shell**, not the claude binary and not Kimi CLI. Rationale:
  minimal/inspectable context (quota-friendly on a 5-hour-window sub), the
  `pi-provider-kimi-code` package already bridges the Kimi sub, pi is already
  primed in the user's stack (skills symlinked, AGENTS.md), and pi natively
  speaks OpenRouter — so it doubles as the model-experimentation layer.
  Kimi CLI is the first-party fallback if the pi provider disappoints.
  The claude binary is fully retired.
- **Codex task kinds tier by reasoning effort** on gpt-5.6-sol:
  planning/research → xhigh, prototype/work → high, via codex profiles.
- **Cancel Anthropic immediately** — nothing in this design touches
  Anthropic's API. Keep the OpenAI sub through the trial.
- **Interactivity requirement**: review leaves run as foreground harness
  sessions (fully interactive); doubt-check spawns run headless but pi
  sessions persist and can be resumed interactively.
- **Cross-model second opinions** flow through sanctioned headless spawns in
  both directions: pi session → `codex exec --profile sol-*` (OpenAI sub);
  codex session → headless pi → K3 (Kimi sub). Subscription tokens are never
  reused outside their licensed harness.
- Deferred (explicitly out of scope): OpenRouter/GLM wiring beyond what pi
  provides natively; the GLM Coding Plan (buy only if the meter later proves
  it); Hermes evaluation; any grove config-file refactor (approach C).

## Architecture

```
                    ┌─────────────────────────────┐
                    │  grove loop driver           │
                    │  (per-grove harness stamp)   │
                    └──────┬───────────────┬──────┘
         stamped: codex    │               │    stamped: pi
                           ▼               ▼
               ┌───────────────────┐   ┌────────────────────────┐
               │ codex             │   │ pi                      │
               │ --profile sol-*   │   │ pi-provider-kimi-code   │
               │ (OpenAI sub)      │   │ → K3 (Kimi sub)         │
               └─────────┬─────────┘   └───────────┬────────────┘
                         │                         │ review leaves run
   review-kind leaves:   │                         │ here natively
   GROVE_REVIEW_HARNESS=pi ────────────────────────┤
                                                   ▼
                                      ┌────────────────────────┐
                                      │ K3 reviewer             │
                                      │ (interactive review     │
                                      │ leaves; headless-but-   │
                                      │ resumable doubt spawns) │
                                      └────────────────────────┘
```

## Grove binary changes

### harness.rs — pi entry, codex profiles

Add to `HARNESSES`:

```rust
Harness {
    name: "pi",
    project_dir: ".pi",          // opt-in detection marker; stamp/explicit is the normal route
    exec_bin: "pi",
    name_args: &[],              // verify at implementation
    model_args: &["--model"],    // verify at implementation
},
```

Change codex `model_args` to `&["--profile"]` (profiles bind model +
reasoning effort; breaking for bare-model-name users, CHANGELOG-documented).
Derive the `unknown harness` error's known-list from `HARNESSES` instead of a
hardcoded string.

### harness_stamp.rs — persist explicit choices

`maybe_stamp` gains `explicit: bool`: an explicit `--harness` always writes
the stamp, not only in multi-harness repos. This is the migration mechanism:
one explicit flag per grove, stamped permanently.

### loop_driver.rs — two routing seams + PID rename

Resolved per picked leaf, per iteration:

- **Per-kind harness override**: `GROVE_<KIND>_HARNESS` (in practice only
  `GROVE_REVIEW_HARNESS=pi`). When set for the picked leaf's kind, that
  harness's exec_bin/args are used for the launch instead of the stamped one.
  Unknown name → hard error naming the variable and known harnesses.
- **Per-harness model env**: model resolution for kind K on harness H checks
  `GROVE_<H>_<K>_MODEL` before `GROVE_<K>_MODEL` — but the base var only
  applies when H is the *stamped* harness. A per-kind override that reroutes
  to a different harness skips the base var entirely: it was written with the
  stamped harness in mind (a codex profile name is garbage to pi and vice
  versa), so it must not follow the leaf across a reroute. With no scoped var
  set, a rerouted leaf launches with no `--model` flag at all. Resolution
  runs against the post-override harness, so the seams compose.
- **`GROVE_CLAUDE_PID` → `GROVE_HARNESS_PID`**: required for namespace
  coherence with the new `GROVE_<H>_*` scheme. The wrapper exports both for
  one release; readers (`complete.rs`, `llm_cli.rs`) try new-then-old.

### provision.rs — multi-harness extraction

`Harness` gains `global_skills_dir` (claude: `~/.claude/skills`, codex:
`~/.codex/skills`, pi: `~/.pi/agent/skills` — structurally different, hence a
field, not derived from `project_dir`). `provision_global_skill` extracts
`content/` into `<skills_dir>/grove/` for every harness whose root exists.
Where the existing `grove` entry is a symlink (today's codex and pi setups),
replace it with a real extracted dir. `load_prompt` reads from the launching
harness's copy.

### complete.rs, llm_cli.rs, cli.rs

PID env rename with fallback; doc text made harness-neutral; harness lists
include pi; model-env docs describe the per-harness scheme.

### content/ — neutral methodology wording

SKILL.md's loop description rewritten harness-neutrally ("one fresh
foreground harness session"; `GROVE_HARNESS_PID`), describing the current
scheme on its own terms. Prompts (`start.md`, `continue.md`, `retire.md`)
are already harness-agnostic — no change.

### Unchanged by design

Loop shape (fresh session per task, completion signal, restart ≡
continuation), stamp file format, `grove-llm` surface.

## User config (outside this repo)

`~/.zshenv` — replace the five `GROVE_*_MODEL` lines:

```sh
export GROVE_CODEX_PLANNING_MODEL=sol-xhigh
export GROVE_CODEX_RESEARCH_MODEL=sol-xhigh
export GROVE_CODEX_PROTOTYPE_MODEL=sol-high
export GROVE_CODEX_WORK_MODEL=sol-high
export GROVE_PI_PLANNING_MODEL=k3      # exact ids per pi-provider-kimi-code — verify
export GROVE_PI_RESEARCH_MODEL=k3
export GROVE_PI_PROTOTYPE_MODEL=k3
export GROVE_PI_WORK_MODEL=k3
export GROVE_PI_REVIEW_MODEL=k3
export GROVE_REVIEW_HARNESS=pi
```

Bare `GROVE_*_MODEL` vars are removed (unset → grove skips the model flag →
harness default, existing semantics).

`~/.codex/sol-xhigh.config.toml` and `~/.codex/sol-high.config.toml` — one
file per profile (this codex build's `--profile` layers
`$CODEX_HOME/<name>.config.toml`, not a `[profiles.<name>]` table in
`config.toml`), each binding `model = "gpt-5.6-sol"` +
`model_reasoning_effort`. `config.toml` untouched.

Pi — install `pi-provider-kimi-code`, set its API key env (and
`KIMI_CODE_PROTOCOL` if needed); one live round-trip before any grove is
stamped. OpenRouter experimentation is `OPENROUTER_API_KEY` + a model id,
native to pi.

## Skills repo (~/Development/skills)

`doubt-driven-development` gains `references/harness-spawns.md`: per-harness
recipes for materialising the fresh-context reviewer — from codex, headless
pi → K3; from pi, `codex exec --profile sol-xhigh` for a sol cross-check.
Don't-bias rules unchanged. SKILL.md gets a two-line platform-adaptation
pointer. Commit in `~/Development/skills` only (the marketplace mirror is
disposable).

## Migration runbook

1. Implement + test grove changes → release via `scripts/release-*.sh` →
   verify the live brew binary's behaviour, not just the build.
2. Pi: provider package + key + live K3 round-trip.
3. Codex: profile blocks + `codex exec --profile sol-high` smoke test.
4. `~/.zshenv` rewrite (new shells pick it up).
5. Skills repo commit.
6. Per grove: one explicit `grove do --harness codex|pi` (stamp fix makes it
   permanent).
7. One full task cycle per side, including a review leaf in a codex grove to
   observe the pi reroute.
8. Cancel Anthropic (`~/.claude/` stays on disk; grove's claude harness entry
   remains for other users).
9. Month trial → flip losing stamps → cancel losing sub.

## Trial log

- **2026-07-18** — grove v12.0.0 released; codex-side and pi-side groves
  stamped. Trial clock starts here. Compare quality, quota pressure, and
  wall-clock across sides through the end date below.
- **Trial end: ~2026-08-18.** At that point, flip the losing side's stamps
  to the winner and cancel the losing subscription.

## Error handling

- Unknown `GROVE_<KIND>_HARNESS` value → hard launch error naming the
  variable and known harnesses.
- Kimi 5h-quota exhaustion → pi exits non-zero → loop stops, resumable later
  (existing restart ≡ continuation semantics; documented, no new machinery).
- Unknown codex profile → codex's error passes through; loop stops.
- Provisioning replaces `grove` skill entries only when they are symlinks or
  grove-provisioned dirs; anything unexpected → bail with a message.
- Absent harness roots (e.g. no `~/.pi/`) → provisioning silently skips.
- `GROVE_HARNESS_PID` readers fall back to `GROVE_CLAUDE_PID` for one
  release.

## Testing

- **Unit**: explicit-stamp persistence in a single-harness repo; env
  precedence (`GROVE_PI_REVIEW_MODEL` beats `GROVE_REVIEW_MODEL`);
  harness-override + model resolution composing against the post-override
  harness; multi-root provisioning in a tempdir `$HOME` including symlink
  replacement and foreign-dir refusal; known-harness error lists pi.
- **Integration**: PATH-injected fake `pi`/`codex` scripts logging argv —
  a review leaf in a codex-stamped grove launches fake-pi with `--model k3`;
  a work leaf launches fake-codex with `--profile sol-high`.
- **Live**: runbook steps 2, 3, and 7 are the end-to-end verification —
  drive the real flow, don't trust a green build.

## Verify at implementation

- pi launch flags: model selection (`--model`?), session naming (if any),
  headless/print mode invocation.
- Exact K3 model id exposed by `pi-provider-kimi-code`, and the package's
  API-key env var name and install command.
- `codex --profile` accepted in both interactive and `exec` modes.
- One live K3 round-trip through the Kimi sub endpoint from pi.
