# The loop driver selects each session's launch model by the leaf's task kind

The self-driving loop launches each task's `claude` session on a model chosen by
the picked leaf's **kind** (*task-kind-taxonomy*), so a grove can grill on a strong
reasoning model, survey literature on a long-context one, and grind mechanical work
on a cheaper/faster one. Selection uses Claude Code's **native `--model` flag** — no
router, no proxy.

## Mechanism

- **The driver peeks.** Before launching, the loop driver resolves the next live
  leaf's kind via `grove-llm kind [<leaf>]` (which reads the task file's
  `**Kind:**` line), then launches `claude --model <model>` where `<model>` is the
  env var for that kind. This keeps the loop stateless — the kind is re-derived
  from the filesystem every iteration (self-driving-loop, constraint 1), matching
  the driver's existing role of setting launch args (it already sets the session
  name). An unrecognised kind degrades to `work` (*task-kind-taxonomy*), so a
  hand-edited task file cannot jam the loop.
- **One env var per kind.** `GROVE_PLANNING_MODEL`, `GROVE_RESEARCH_MODEL`,
  `GROVE_PROTOTYPE_MODEL`, `GROVE_WORK_MODEL`, `GROVE_REVIEW_MODEL`, read at each
  launch.
- **Unset ⇒ no `--model` flag, and no fallback.** The session inherits the user's
  own default (`ANTHROPIC_MODEL` / settings). This is the load-bearing rule:
  because the selection priority is `--model` > `ANTHROPIC_MODEL` > settings
  (Claude Code model-config docs), passing `--model` unconditionally would silently
  clobber a user's own default. grove passes it **only when the user has opted in**
  for that kind.
- **Per-harness flag template.** The `Harness` struct carries a `model_args`
  template (`claude: ["--model"]`), parallel to `name_args`, so *how* to pass a
  model is per-harness while the *value* comes from the env var. claude is wired;
  codex is best-effort/lazy (one grove runs one harness).
- **Start-path is planning by construction.** On a brand-new grove the driver
  launches `start` while `.grove/` does not yet exist (the agent runs `root-init`
  *inside* that session), so there is no leaf to peek — but `root-init`'s first leaf
  is always **planning** (fresh-grove-start-contract), so the `start` path uses
  `GROVE_PLANNING_MODEL` unconditionally.
- **Kind is the only key.** There is no per-leaf override. `pick`'s default output
  is unchanged (the launched agent still parses it as a bare path); `kind` is a
  separate verb.

## The launch model is a default, not a lock

The launch `--model` is only the session's *starting* model. Claude Code's
in-session `/model` is **higher priority** than `--model`, so the agent or the human
can switch mid-session at will — this is exactly what covers "a `work` session that
turns into substantial planning": no grove machinery, native `/model`.

Whether that switch survives into the next task depends on grove's own no-fallback
rule, which is the non-obvious part:

- For a kind whose env var **is set**, the driver passes `--model` on the next
  launch, and the flag outranks the saved default. The override does **not** persist.
- For a kind whose env var is **unset**, grove passes no flag — and interactive
  `/model` saves the choice as the user's default for new sessions. The override
  **does** persist, into every subsequent unconfigured session.

Both behaviours are correct (each task is keyed on its own kind, and an
unconfigured kind is by definition the user's own default), but the asymmetry
surprises, so it is called out in the user-facing docs.

## Considered options

- **A fallback chain (rejected).** Letting an unset `GROVE_RESEARCH_MODEL` fall back
  to `GROVE_WORK_MODEL` (or to a declared base kind) looks like a convenience. It is
  grove choosing a model for a kind the user never configured, which is the precise
  thing the load-bearing rule above forbids — and it can silently *downgrade*: a
  user who sets only `GROVE_WORK_MODEL=sonnet` would find research leaves demoted
  from their own Opus default to Sonnet. With no chain, an unconfigured kind gets
  exactly what it would get if grove did not exist. The migration objection ("a
  research leaf is `**Kind:** work` today, so re-kinding changes its model") does
  not survive contact: re-kinding is a deliberate human edit, not a silent
  migration.
- **A model router (rejected).** claude-code-router and similar proxy Claude Code's
  API requests to route across *providers* (OpenRouter, DeepSeek, Ollama, Gemini).
  They need an API key and, on a Max subscription, either break the subscription
  billing or drain pay-per-token credits
  (github.com/NousResearch/hermes-agent/issues/40014). For Opus↔Sonnet on a Max sub,
  native `--model` does the identical routing on the subscription for free — a
  router is the wrong tool and adds a proxy grove would have to own.
- **codex as the review harness (Route B, rejected).** For cross-family review
  specifically, a second option is to launch the `review` leaf under `codex` rather
  than redirect `claude`'s endpoint. Rejected: it costs ~ten changes (research doc,
  Q5), one of which — `--sandbox danger-full-access` — exists *only* to defeat
  codex's Seatbelt policy (`(allow signal (target same-sandbox))`), which would
  otherwise make grove's completion `kill -TERM <parent>` return `EPERM`. grove
  would adopt a second harness and immediately disable the one property that
  distinguishes it, plus a second skill location (`~/.agents/skills/`), a second
  instruction-file convention (`AGENTS.md`), and a terminal-reset obligation (codex
  installs no `SIGTERM` handler). And codex reaches other providers itself
  (`model_providers.<id>` + `base_url`), so Route B buys nothing the provider
  profile above does not. Its only independent merit — wanting codex *itself* as the
  reviewer — is a different question, not this one. (An unrelated defect this
  surfaced, grove's stale codex `name_args: &["--name"]`, is tracked as
  `Linkuistics/grove#1`.)
- **Agent self-switch via `/model` (rejected as the mechanism).** Having the launched
  agent read its own kind and `/model`-switch at session start needs no driver
  change, but the bootstrap context already ran on the wrong model, it depends on the
  agent remembering every session, and it is unverifiable in an unattended relaunch.
  Native `/model` still *layers on top* for mid-session overrides — it just isn't the
  launch-default mechanism.
- **CLI flags / config file for configuration (rejected).** `grove do` flags do not
  survive a loop restart (they aren't re-derived from state — fights
  restart≡continuation). A config file adds non-task state under `.grove/`
  (constraint 1). Env vars are walk-away-able (the loop's shell equivalent expresses
  them trivially), stateless, and re-derived every `grove do`.

## Consequences

- The driver gains one `grove-llm kind` call per iteration on the `continue` path
  (none on `start`), and only when at least one of the five env vars is set —
  otherwise the launch is byte-for-byte the pre-feature one. No durable state;
  restart ≡ continuation is preserved.
- The five env vars are a public config surface — documented in `--help`/README
  alongside the `/model`-persistence asymmetry above.
- Backward compatible: with no env var set, grove launches exactly as before (no
  `--model`), so existing groves are unaffected.

## Cross-family provider profiles

The same mechanism extends from *tier within one family* to *a different family*.
A `review` leaf gains most from a reviewer that does **not** share the author's
family-level style prior: self-preference bias is causally established and
family-level (Panickssery et al., NeurIPS 2024; Wataoka et al.), so a fresh
same-family context cannot remove it, whereas a different family changes the prior.
The evidence base is `docs/research/cross-family-review-providers.md`.

**A per-kind provider profile.** `GROVE_<KIND>_MODEL` gains two optional siblings,
`GROVE_<KIND>_BASE_URL` and `GROVE_<KIND>_AUTH_TOKEN`. The **base-URL is the
activation key**:

- **No base-URL** — the existing same-family behaviour above, unchanged: a bare
  `GROVE_<KIND>_MODEL` selects a tier on the user's own subscription.
- **base-URL present** — the driver exports `ANTHROPIC_BASE_URL` +
  `ANTHROPIC_AUTH_TOKEN` **to the child process only** (`Command::env`), alongside
  `--model <vendor-id>`. Claude Code passes `--model` through verbatim to a
  non-Anthropic endpoint (model-config docs), reads the base-URL and token fresh
  from the environment at launch, and the gateway token (auth precedence rank 2)
  outranks the Max OAuth (rank 6) for that one process — an off-subscription review
  session, with the loop, subagents, and the grove skill all intact (the skill
  loads from the filesystem, endpoint-independent).

**Partial profiles extend the no-fallback rule.** With base-URL present the
**triad** (model + base-URL + auth-token) must be complete. An incomplete triad
yields **no cross-family override at all** — grove must not send a foreign model ID
to the Max endpoint (a hard-400, or a silent wrong-model run), so it degrades to
the user's own default. As everywhere in the loop, this **degrades with a
diagnostic, never gates** the launch (constraint 5; the taxonomy's read-degrades
rule). A profile is thus all-or-nothing per kind, on top of the model-only path.

**The secret lives in the env var, read at launch, never persisted.** Not
`apiKeyHelper` (a durable helper script is state, fighting constraints 1 and 6);
and specifically **`ANTHROPIC_AUTH_TOKEN`, never `ANTHROPIC_API_KEY`** — the latter
persists an approval fingerprint into `~/.claude.json` in interactive mode, leaving
residue in the user's config. Unsetting the token returns cleanly to the Max
subscription (the OAuth login persists underneath, merely unselected).

**Two hazards bound the mechanism, and both are real, not theoretical.**

1. **The `settings.json` `env` block may silently win.** grove sets the child
   environment via `Command::env`; whether `~/.claude/settings.json`'s `env` block
   overrides a variable already in the process environment is **undocumented by
   Anthropic**, and community evidence (including the exact `Command::env`
   analogue, `claude-agent-sdk-typescript#217`) reports the `env` block wins in
   v2.x — which would silently redirect the review onto the wrong endpoint, the
   precise failure the feature exists to prevent. grove **detects and warns** (a
   four-file JSON read across managed → local → project → user scope for a
   colliding `ANTHROPIC_*` key) **only when a profile is active**; it does not
   refuse (constraint 5), and does not nag on the common no-profile path.
2. **The subagent-alias trap.** A subagent launched with an explicit `model` alias
   (`opus`/`sonnet`/…) resolves that alias *client-side* against Anthropic's table
   before the request leaves, so it hard-fails against a foreign endpoint. This
   matters because `linkuistics:doubt-driven-development` tells a review session to
   spawn a fresh-context reviewer subagent: under a cross-family profile that
   subagent must **inherit** (the documented default), not pin. grove documents
   "cross-family review subagents must inherit" rather than exporting the four
   `ANTHROPIC_DEFAULT_*_MODEL` remap vars, keeping the surface small.

**Empirical gate.** One open bug can invalidate the whole route:
`anthropics/claude-code#7855` (open, `has repro`) reports that
`ANTHROPIC_AUTH_TOKEN` breaks *interactive* sessions with auth errors — and grove's
sessions are interactive and TTY-owning. Combined with hazard 1's undocumented
precedence, the route is verified by a `prototype` spike *before* the driver code
is written; if #7855 reproduces on grove's launch, cross-family Route A is blocked
and this section is reworked to record that rejection.

**A cross-family review leaf's brief carries more.** The reviewer loses server-side
`WebSearch` (client-side `WebFetch` survives) and reads `CONTEXT.md` conventions
less fluently than a same-family model, so a review leaf run off a foreign endpoint
should carry more of its own context. This is a brief-authoring habit, not a
`TASK-FORMAT.md` schema change — cross-family review is opt-in and unproven, so the
schema stays as is.

**What this route does *not* buy, recorded so it is not re-litigated.** The
increment cross-family adds *on top of* fresh context, for the single-reviewer
code-review case, is **measured nowhere** — no study isolates same-family
fresh-context vs different-family review on the same artifacts (research doc, Q6).
Cross-family review is therefore a cheap bet (~$18/mo, ~30 lines of driver) on a
plausible, causally-grounded, but unquantified mechanism — defensible, not a
demonstrated requirement. The best-evidenced review lever is an **external
deterministic verifier** (build, tests, static analysis as an oracle), which is
orthogonal to this route and cheaper; it is left to a separate future decision.
