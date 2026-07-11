# review-provider-design-k11

**Kind:** planning

## Goal

Grill `review-provider-research-k10`'s findings to a decision: does grove gain
**per-kind provider profiles** (Route A), **per-kind harness selection** (Route B),
or **neither** — and grow the implementing work leaf if the answer is one of the
first two.

## Context

Read first: `docs/research/cross-family-review-providers.md` (the whole point of
k10), then ADRs `model-per-task-kind` — whose final consequence already sketches
Route A's shape and names its two caveats — and `task-kind-taxonomy`.

The user's motivating want (2026-07-10): run a `review` leaf on a model from a
different family than the one that wrote the code, naming **GLM** and **codex** as
candidates. The ADR-level constraint that shapes any answer: grove rejected a
model **router/proxy** on Max-subscription grounds, and `--model` alone selects a
tier within one family, not a family.

## Questions to grill

1. Route A, Route B, or neither? (A "neither" outcome is a real result: it retires
   the ambition into `model-per-task-kind`'s consequences as a considered-and-
   rejected option, so nobody re-proposes it.)
2. If Route A: is the env-var namespace right — `GROVE_<KIND>_BASE_URL` /
   `GROVE_<KIND>_AUTH_TOKEN` alongside `GROVE_<KIND>_MODEL`? A **secret in an env
   var** is a new class of config for grove; does it belong there, in
   `apiKeyHelper`, or nowhere?
3. Partial profiles: base-URL set but model unset, or token unset. Error, or the
   no-fallback rule extended (any incomplete profile ⇒ no override at all)?
4. The `settings.json`-`env`-outranks-the-driver hazard: warn, detect, or document?
5. If Route B: does `Harness` move from per-grove to per-kind, and what happens to
   `complete`'s `GROVE_CLAUDE_PID` and the claude-shaped `content/prompts/`?
6. Does the decision change **`review`'s discipline** in `TASK-FORMAT.md` — e.g. a
   cross-family reviewer cannot read `CONTEXT.md` conventions as fluently, so the
   review leaf's brief must carry more?

## Done when

Decisions recorded. `model-per-task-kind` reworked in place to state the outcome —
whether that is the provider-profile mechanism or its considered rejection (never a
superseding ADR; `linkuistics:decision-records`). A `## Findings adopted` pointer
added to `docs/research/cross-family-review-providers.md`. Implementing work leaf
grown if there is anything to implement.

## Notes

HITL. This leaf exists because the provider question was **externalized** rather
than absorbed into `task-kinds-model-selection-k6` — the taxonomy did not depend on
it, and a provider profile is strictly additive to the `GROVE_<KIND>_…` namespace.
If the grilling shows the dependency runs the other way, that is a finding worth
recording, not a reason to reopen the taxonomy.

## Decisions (running log)

Grilled 2026-07-10; the human delegated the route call ("you decide"), so the
recommendations below stand as decisions. Evidence lives in
`docs/research/cross-family-review-providers.md` (cited by section).

1. **Route A, gated on a spike.** Per-kind provider profile in the driver — same
   `claude` harness, redirected endpoint. **Route B rejected**: it adds a second
   harness whose one distinguishing property (codex's Seatbelt sandbox) must be
   disabled with `--sandbox danger-full-access` purely to let grove's completion
   `kill -TERM <parent>` through (Q5.2), and codex reaches other providers itself
   (`model_providers.<id>` + `base_url`), so B buys nothing A does not. **Neither
   rejected**: self-preference bias is causally established and *family-level*
   (Panickssery et al. NeurIPS 2024; Wataoka et al.), so fresh context cannot
   remove it; and cross-family review is the cheapest leaf to move off-subscription
   ($18/mo flat, read-heavy/write-light), a *measured* win independent of the
   unquantified epistemic bet. The loop is untouched — `complete` kills by PID,
   endpoint-agnostic (Q1e, `src/complete.rs` [O]).

2. **Mechanism: per-kind provider profile.** `GROVE_<KIND>_MODEL` gains two
   optional siblings, `GROVE_<KIND>_BASE_URL` and `GROVE_<KIND>_AUTH_TOKEN`. When
   the base-URL is present it activates a cross-family profile; the driver exports
   `ANTHROPIC_BASE_URL` + `ANTHROPIC_AUTH_TOKEN` to the child process only,
   alongside `--model`. **Secret home is the env var, read at launch, never
   persisted** — not `apiKeyHelper` (a durable helper script fights constraints
   1/6), and **`ANTHROPIC_AUTH_TOKEN`, never `ANTHROPIC_API_KEY`** (the latter
   persists an approval fingerprint into `~/.claude.json`, Q3).

3. **Partial-profile rule (extends the no-fallback rule).** base-URL is the
   *activation key*. A bare `GROVE_<KIND>_MODEL` with no base-URL keeps the
   existing same-family tier-selection behaviour (k9), unchanged. With base-URL
   present, the **triad** (model + base-URL + auth-token) must be complete;
   incomplete ⇒ **no cross-family override at all** — grove does not pass the
   foreign model ID to the Max endpoint (a hard-400 or silent wrong-model), it
   degrades to the user's own default. Degrade-with-a-stderr-diagnostic, never
   gate the launch (constraint 5), matching the taxonomy's read-degrades house
   style (Q3).

4. **settings.json `env`-block hazard: detect-and-warn, not refuse — and only when
   a profile is active.** A four-file JSON read (managed → local → project → user)
   for a colliding `ANTHROPIC_*` key; warn because community evidence says the
   settings `env` block *wins* over `Command::env`, silently defeating the override
   (Q4, the investigation's highest-value documentation gap). Refuse-to-launch
   would violate constraint 5; nagging on the common no-profile path is noise.

5. **Empirical gate before any driver code (the spike).** Two facts nobody has can
   each produce the exact failure the feature prevents — a review you believe ran
   cross-family but did not: `anthropics/claude-code#7855` (open, `has repro`) —
   interactive `ANTHROPIC_AUTH_TOKEN` breaks with auth errors, and grove's sessions
   are interactive/TTY-owning; and the settings-`env`-vs-`Command::env` precedence
   is undocumented. A `prototype` leaf resolves both against reality before the
   `work` leaf writes the driver. If #7855 reproduces on grove's launch, Route A is
   **blocked** and the impl leaf reworks the ADR to record the rejection instead.

6. **`review` discipline (Q6): a cross-family review leaf's brief carries more.**
   The reviewer loses server-side `WebSearch` (keeps client-side `WebFetch`) and
   reads `CONTEXT.md` conventions less fluently. Recorded as an ADR consequence;
   **`TASK-FORMAT.md` deliberately not changed** (lazy — cross-family review is
   opt-in and unproven; a schema change is premature).

7. **External deterministic verifier (Q6 synthesis) is orthogonal, not gated on
   this, and not grown here.** The best-evidenced lever in the literature — an
   oracle (build, tests, static analysis), not a rival model — is a separate
   improvement to the `review` kind. Recorded for a future grove / GitHub issue,
   not a leaf in this workstream (whose goal is the pocock-skills incorporation).

**Tree grown:** `review-provider-spike-k12` (prototype, the gate) and
`review-provider-impl-k13` (work, the driver profile — gated on k12).
ADR `model-per-task-kind` reworked in place; `## Findings adopted` pointer added to
the research doc.
