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
- **Cross-family (multi-provider) selection — rejected; the kind is the key, the
  *family* is not.** A `review` leaf gains most from a reviewer that does not share
  the author's family-level style prior (self-preference bias is causally
  established and family-level — Panickssery et al., NeurIPS 2024), so the obvious
  extension is to let a kind select a whole *provider*, not just a tier. Two routes
  were costed in full (`docs/research/cross-family-review-providers.md`); **both are
  rejected, and the mechanism above stays single-provider.**
  - *Redirect `claude`'s endpoint* (`ANTHROPIC_BASE_URL` + `ANTHROPIC_AUTH_TOKEN` per
    kind, `--model <vendor-id>` passed through verbatim). This is not a
    model-selection change — it is a **credential and provider-configuration surface
    grove does not have**: a live secret to hold and never persist, an *undocumented*
    collision between `settings.json`'s `env` block and a `Command::env` child
    environment (community evidence says the `env` block silently wins —
    `claude-agent-sdk-typescript#217`), an open interactive-auth bug against the one
    variable it must use (`anthropics/claude-code#7855`, `has repro` — and grove's
    sessions are interactive and TTY-owning), and a subagent-alias trap. That is
    infrastructure, and it earns a design of its own before any of it is bolted onto
    the launch path.
  - *codex as the review harness.* ~Ten changes (research doc, Q5), one of which —
    `--sandbox danger-full-access` — exists *only* to defeat codex's Seatbelt policy
    (`(allow signal (target same-sandbox))`), which would otherwise make grove's
    completion `kill -TERM <parent>` return `EPERM`. grove would adopt a second
    harness and immediately disable the one property that distinguishes it, plus a
    second skill location (`~/.agents/skills/`), a second instruction-file convention
    (`AGENTS.md`), and a terminal-reset obligation (codex installs no `SIGTERM`
    handler). Wanting codex *itself* as the reviewer is a different question, not
    this one. (An unrelated defect this surfaced, grove's stale codex
    `name_args: &["--name"]`, is tracked as `Linkuistics/grove#1`.)

  What is *not* the reason: cost (~$18/mo — trivial) or fidelity (the loop, skills,
  subagents and the `complete` handshake all survive a redirected endpoint). The
  reason is that the payoff is **unquantified — no study isolates same-family
  fresh-context vs different-family review on the same artifacts** (research doc,
  Q6) — while the cost is a provider/credential layer grove would have to own. A
  cheap bet is still a bad trade when it is paid for in infrastructure. The
  best-evidenced review lever is an **external deterministic verifier** (build,
  tests, static analysis as an oracle): orthogonal to provider choice, and cheaper.
  What would reopen this: a coherent provider/credential design for grove, or
  evidence that actually measures the cross-family increment in defect-detection
  recall.
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
