# The loop driver selects each session's launch model by the leaf's task kind

The self-driving loop launches each task's `claude` session on a model chosen by
the picked leaf's **kind** (`planning` vs `work`), so a grove can run planning
on a stronger reasoning model and work on a cheaper/faster one (or any pairing).
Selection uses Claude Code's **native `--model` flag** — no router, no proxy.

## Mechanism

- **The driver peeks.** Before launching, the loop driver resolves the next
  live leaf's kind via `grove-llm kind [<leaf>]` (which reads the task file's
  `**Kind:**` line through `leaf::Kind::parse`), then launches
  `claude --model <model>` where `<model>` is the env var for that kind. This
  keeps the loop stateless — the kind is re-derived from the filesystem every
  iteration (self-driving-loop, constraint 1), matching the driver's existing
  role of setting launch args (it already sets the session name).
- **Configured by env vars.** `GROVE_PLANNING_MODEL` and `GROVE_WORK_MODEL`,
  read at each launch. **Unset ⇒ no `--model` flag** — the session inherits the
  user's own default (`ANTHROPIC_MODEL` / settings). This is the load-bearing
  rule: because the selection priority is `--model` > `ANTHROPIC_MODEL` >
  settings (Claude Code model-config docs), passing `--model` unconditionally
  would silently clobber a user's own default, so grove passes it **only when
  the user has opted in** for that kind.
- **Per-harness flag template.** The `Harness` struct carries a `model_args`
  template (`claude: ["--model"]`), parallel to `name_args`, so *how* to pass a
  model is per-harness while the *value* comes from the env var. claude is wired;
  codex is best-effort/lazy (one grove runs one harness).
- **Start-path is planning by construction.** On a brand-new grove the driver
  launches `start` while `.grove/` does not yet exist (the agent runs
  `root-init` *inside* that session), so there is no leaf to peek — but
  `root-init`'s first leaf is always **planning** (fresh-grove-start-contract),
  so the `start` path uses `GROVE_PLANNING_MODEL` unconditionally.
- **Two buckets only.** The model is keyed purely on `Kind`; there is no
  per-leaf override. `pick`'s default output is unchanged (the launched agent
  still parses it as a bare path); `kind` is a separate verb.

## The launch model is a default, not a lock

The launch `--model` is only the session's *starting* model. Claude Code's
in-session `/model` is **higher priority** than `--model`, so the agent or the
human can switch mid-session at will — this is exactly what covers "a `work`
session that turns into substantial planning": no grove machinery, native
`/model`. The one caveat worth documenting: an in-session `/model` switch does
**not persist across relaunch** — the next task is a fresh `claude` the driver
re-launches on the kind default. That is correct (each task is keyed on its own
kind), but it can surprise, so it is called out in the user-facing docs.

## Considered options

- **A model router (rejected).** claude-code-router and similar proxy Claude
  Code's API requests to route across *providers* (OpenRouter, DeepSeek, Ollama,
  Gemini). They need an API key and, on a Max subscription, either break the
  subscription billing or drain pay-per-token credits
  (github.com/NousResearch/hermes-agent/issues/40014). For Opus↔Sonnet on a Max
  sub, native `--model` does the identical routing on the subscription for free —
  a router is the wrong tool and adds a proxy grove would have to own.
- **Agent self-switch via `/model` (rejected as the mechanism).** Having the
  launched agent read its own kind and `/model`-switch at session start needs no
  driver change, but the bootstrap context already ran on the wrong model, it
  depends on the agent remembering every session, and it is unverifiable in an
  unattended relaunch. Native `/model` still *layers on top* for mid-session
  overrides — it just isn't the launch-default mechanism.
- **CLI flags / config file for configuration (rejected).** `grove do` flags do
  not survive a loop restart (they aren't re-derived from state — fights
  restart≡continuation). A config file adds non-task state under `.grove/`
  (constraint 1). Env vars are walk-away-able (the loop's shell equivalent
  expresses them trivially), stateless, and re-derived every `grove do`.

## Consequences

- The driver gains one `grove-llm kind` call per iteration on the `continue`
  path (none on `start`). No durable state; restart ≡ continuation is preserved.
- `GROVE_PLANNING_MODEL` / `GROVE_WORK_MODEL` are a public config surface —
  documented in `--help`/README alongside the "`/model` doesn't persist across
  relaunch" caveat.
- Backward compatible: with neither env var set, grove launches exactly as
  before (no `--model`), so existing groves are unaffected.
- Per-leaf model override remains a clean v2 extension if it earns its place —
  its only unique value is forcing a strong model on a `work` leaf that runs
  unattended (no human to `/model`).
