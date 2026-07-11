# Cross-family review providers

What it would cost to run a `review` leaf on a model from a **different family**
than the one that produced the work. Commissioned by `review-provider-research-k10`;
its one consumer is `review-provider-design-k11`, which must pick a route or record
a rejection.

`review` earns its place as a task kind on fresh-context adversarial reading alone
(ADR *task-kind-taxonomy*). Cross-family is an **amplifier**, not the kind's
justification. So this document decides how much to pay for the amplifier, and
"neither route is worth it" was a permitted answer from the outset.

Observed **2026-07-10**. `claude` v2.1.206, `codex-cli` 0.142.5, `grove`/`grove-llm`
9.1.0 (Homebrew).

## How to read the evidence classes

Every claim below carries one. They are not decoration — several agent-supplied
claims were **wrong or overstated**, and re-checking them against the primary
source changed the conclusions (see Q1's subagent finding and Q5's `--name`
finding).

| Class | Meaning |
|---|---|
| **[P]** | Primary source, fetched and read during this session. Quoted. |
| **[S]** | Primary source located by a research subagent; URL recorded; **not** independently re-fetched. |
| **[O]** | Observed on this machine — the command and its output are given. |
| **[C]** | Community-reported (issue tracker, third-party). Explicitly not authoritative. |
| **[I]** | Inference. Labelled wherever it appears. |

Prices are dated; they move.

---

## Outcome — both routes rejected (2026-07-11)

**grove stays single-provider.** `review-provider-design-k11` grilled this document
and initially took Route A gated on a spike; on reflection the user **abandoned the
multi-provider effort entirely** before that spike ran, on the grounds that it
*needs more infrastructure thinking*. The rejection — and the reasons, which are
this document's — is recorded in ADR *model-per-task-kind* → *Considered options*.
The ADR describes no cross-family mechanism; there is nothing to un-ship, because
the walk-away check below held and no code was ever written.

The reason is **not** cost (~$18/mo) and **not** fidelity (Q1: the loop, skills,
subagents and the `complete` handshake all survive a redirected endpoint). It is
that Route A is not a model-selection change at all — it is a **credential and
provider-configuration surface grove does not have** (a live secret; the
undocumented `settings.json`-`env` precedence of Q4; the open interactive-auth bug
of Q3; the subagent-alias trap of Q1d) — bought for a payoff that is **measured
nowhere** (Q6). That trade is bad while the infrastructure is unowned, and it earns
a design of its own before any of it is bolted onto the launch path.

**What this document is now for.** It is the standing evidence base for that
decision, and the thing to re-read if the question reopens — which it would on
either of the two triggers in *What would change grove's mind*: a coherent
provider/credential design for grove, or evidence that actually measures the
cross-family increment in defect-detection recall. Prices and version-pinned
observations are dated; the *Missing sources* list still stops a future reader
re-running the same fruitless searches. Note that the one lever this research rates
highest — an **external deterministic verifier** for the `review` kind (Q6) — is
orthogonal to provider choice, cheaper than any of it, and remains unbuilt.

`review` itself is unaffected: it earns its place as a task kind on fresh-context
adversarial reading alone (ADR *task-kind-taxonomy*).

---

## Q1 — Route A fidelity

**Route A**: same `claude` harness, different endpoint. `ANTHROPIC_BASE_URL` +
`ANTHROPIC_AUTH_TOKEN` set on the child process only; `--model <vendor-id>` passed
through.

### Verdict table

| Capability | Survives? | Class | Evidence |
|---|---|---|---|
| `--model` passthrough, unvalidated | **Yes** | [P] | model-config, quoted below |
| Read/Edit/Write/Bash | Yes, model-dependent | [I]+[S] | client-side tools; fidelity = vendor model's tool-use quality |
| Skill loading (the grove skill itself) | **Yes** | [S] | filesystem discovery, zero server dependency |
| Subagents (mechanism, fresh context) | **Yes** | [P] | sub-agents doc, quoted below |
| Subagents pinned to an alias | **No**, unless remapped | [P] | `Agent` tool `model` param resolves client-side |
| `grove-llm complete` handshake | **Yes** — unconditionally | [O] | read `src/complete.rs`; see below |
| Remote Control | **No** — disabled v2.1.196+ | [S] | remote-control#requirements |
| Server-side WebSearch | No | [S] | executes at the inference provider |
| WebFetch | Yes (client-side fetch) | [C] | not Anthropic-documented |
| MCP tool search | Off by default | [S] | `ENABLE_TOOL_SEARCH=true` to re-enable |
| Fine-grained tool streaming | Off by default | [S] | behind a custom base URL |
| Adaptive thinking | May hard-400 | [S] | CC sends the field to unrecognised model names |
| 1M-context entitlement | Moot | [I] | you get the vendor model's native window |
| Prompt caching | Vendor semantics | [S] | `cache_control` forwarded; TTL undocumented off-Anthropic |

### 1a. `--model` is passed through verbatim — **confirmed** [P]

> "The check runs only on the Anthropic API. On Amazon Bedrock, Google Cloud's
> Agent Platform, Microsoft Foundry, Claude Platform on AWS, and behind an LLM
> gateway or a custom `ANTHROPIC_BASE_URL`, your provider or gateway defines the
> model names, **so Claude Code passes any string through without checking it.**"
> — <https://code.claude.com/docs/en/model-config>

The brief's already-verified claim holds. The same page also settles precedence:
`/model` > `--model` > `ANTHROPIC_MODEL` > the `model` settings field [P].

### 1b. Skills survive; the grove skill is safe [S]

Skills are discovered from the filesystem (`~/.claude/skills/<name>/SKILL.md`),
watched for edits, and injected client-side. Claude Code's skills follow the
[agentskills.io](https://agentskills.io) open standard, which by construction is
not tied to Anthropic's API
(<https://code.claude.com/docs/en/skills>). Explicit `/skill-name` invocation is
100% client-side. **Automatic** invocation reduces to whether the vendor model
emits a well-formed `Skill` tool-use block — i.e. it collapses into 1c. [I]

### 1c. Tool use is the one genuinely model-dependent capability

Read/Edit/Write/Bash are client-side: `claude` declares them and executes them
locally when the model returns a well-formed Anthropic `tool_use` block. [I] So
fidelity is a property of the vendor model, not of the transport.

Notably, malformed `tool_use` blocks are **not** a third-party-only failure — they
have been reported against Anthropic's own models
(<https://github.com/anthropics/claude-code/issues/64176>, `#63604`) [C]. And a
deliberate search for `tool_use` breakage in Anthropic's `claude` client (as
opposed to a community router proxy) running against GLM or Kimi found
**nothing**. That absence is a mild positive signal and nothing more — see
*Missing sources*.

### 1d. Subagents — the agent's report was **overstated**; here is the corrected finding

A research subagent reported that Route A requires remapping all four
`ANTHROPIC_DEFAULT_*_MODEL` variables or "every alias-pinned subagent and built-in
… will hard-fail." Re-reading the primary doc shows this is **narrower** than
claimed:

- `model:` frontmatter accepts `sonnet, opus, haiku, fable`, a full model ID, or
  `inherit` — and **"Defaults to `inherit`"** [P, sub-agents doc, line 277].
- "Each subagent starts with a **fresh, isolated context window**." [P, ibid.]
- Every general-purpose built-in **inherits**. As of v2.1.198 even `Explore`
  inherits rather than pinning Haiku. Only `statusline-setup` (Sonnet) and
  `claude-code-guide` (Haiku) pin a model [P] — and neither is reachable from a
  review session.
- grove's own repo has **no `.claude/agents/`** and pins no model [O].

So the hazard is real but precise: it fires only when a caller **explicitly passes
`model`** to the `Agent` tool (whose parameter is an alias enum), because the alias
resolves *client-side* against Anthropic's table before the request leaves. The
remedy is either "don't pass `model`" or set the four vars, which Claude Code
documents as controlling "what … the `sonnet`, `opus`, `haiku`, and `fable`
aliases resolve to" [P, model-config line 215].

**This matters for grove specifically**, because `linkuistics:doubt-driven-development`
instructs a session to "spawn a fresh-context reviewer — a harness subagent
(`code-reviewer`, or a plain `Explore`)". Under Route A, that subagent must inherit,
not pin.

### 1e. The `complete` handshake is endpoint- **and** harness-agnostic — confirmed by reading the source [O]

The brief asked to confirm this, and the answer is unambiguous. `src/complete.rs`:

```rust
let script = format!(
    "sleep {grace}; kill -TERM {pid} 2>/dev/null; sleep {kill_grace}; kill -KILL {pid} 2>/dev/null"
);
```

`pid` comes from `$GROVE_CLAUDE_PID`, which `src/loop_driver.rs` sets via
`sh -c 'export GROVE_CLAUDE_PID=$$; exec "$@"'` — and `exec` preserves the PID, so
the variable holds *whatever binary was exec'd*. Nothing in the path touches the
endpoint, the credential, or the model. The handshake's only requirements are:

1. the agent can run a shell command,
2. that command inherits the parent's environment,
3. the harness process dies on `SIGTERM`/`SIGKILL`.

All three are properties of the *process*, not of the *API*. **Route A cannot break
the handshake.** (Route B can — see Q5.6.)

Corollary worth recording: `GROVE_CLAUDE_PID` is a **misleading name**, not a
claude coupling. It names the harness PID.

### 1f. Which degradations break the loop, and which only break the review?

**None break the loop.** Remote Control, WebSearch, MCP tool search, adaptive
thinking, prompt-cache economics and the 1M entitlement are all *session-quality*
concerns. The loop's hard dependencies are exactly the three in 1e. [I, grounded in
`loop_driver.rs` + `complete.rs`]

The one that would genuinely hurt a *review*: **no server-side WebSearch**. An
adversarial reviewer that wants to check a framework claim against upstream docs
loses `WebSearch` and retains only `WebFetch` (client-side) [C]. For a diff-reading
adversarial pass this is a small loss; for a review that must verify library
behaviour it is not.

---

## Q2 — The market

Providers exposing an Anthropic-Messages-compatible endpoint. All [S] unless noted.

| Provider | `ANTHROPIC_BASE_URL` | Auth var | Model IDs | Context | Price (USD/1M) | CC documented? |
|---|---|---|---|---|---|---|
| **Z.ai / Zhipu GLM** | `https://api.z.ai/api/anthropic` | `ANTHROPIC_AUTH_TOKEN` | `GLM-5.2`, `GLM-5-Turbo`, `GLM-4.7` | 1M | Coding-plan Lite **$18/mo** | Yes |
| **Moonshot Kimi** | `https://api.moonshot.ai/anthropic` | `ANTHROPIC_AUTH_TOKEN` | `kimi-k2.7-code`, `kimi-k2.6` | 256K | ~$0.95 in / $4.00 out | Yes |
| **DeepSeek** | `https://api.deepseek.com/anthropic` | `ANTHROPIC_API_KEY` (also accepts token) | `deepseek-v4-pro`, `-flash` | 1M | $0.435 / $0.87 (pro, promo) | Yes |
| **MiniMax** | `https://api.minimax.io/anthropic` | `ANTHROPIC_AUTH_TOKEN` | `MiniMax-M3` | 1M | $0.60 / $2.40 | Yes |
| **Qwen / DashScope** | `https://coding.dashscope.aliyuncs.com/apps/anthropic` | `ANTHROPIC_API_KEY` (`sk-sp-…`) | `qwen3.7-plus`, `qwen3-coder-plus`, … | n/s | Coding plan ~$10–50/mo | Yes |
| **OpenRouter** | `https://openrouter.ai/api` | `ANTHROPIC_AUTH_TOKEN` | any `author/model` slug | per model | pass-through | Yes |
| **Fireworks** | `https://api.fireworks.ai/inference` | via `apiKeyHelper` | mapped GLM/DeepSeek | per model | per model | Yes |

**Not substantiated:** Together AI *claims* Anthropic-Messages support on its
GLM-5.2 page but publishes **no base URL**; its own quickstart documents only an
OpenAI-compatible endpoint.
**Proxy-only, no first-party endpoint:** Cerebras, Groq, Mistral. Groq has an
*open feature request* asking for one — strong evidence it does not exist.
**Self-host:** vLLM and llama.cpp both implement the Anthropic Messages API
natively, so a local model can serve `ANTHROPIC_BASE_URL` directly.

### Price context

Opus 4.8 is $5 in / $25 out per 1M. Every provider above is between **5× and 35×
cheaper on output**, and Z.ai's coding plan is a flat $18/month. A `review` leaf is
a read-heavy, write-light session. **Cost is not a reason to reject cross-family
review; it is a reason in favour.**

### Two caveats the vendors state themselves

- Z.ai: the coding plan **"is strictly limited to use within officially supported
  tools and products"** — Claude Code is one of the listed tools, so grove's use is
  inside the licence, but scripted/non-interactive use may not be.
  (<https://docs.z.ai/devpack/quick-start>)
- Qwen: **"Do not use the plan's API key for automated scripts, application
  backends, or other non-interactive scenarios."** grove's loop launches an
  *interactive* TTY-owning session, so this is arguably satisfied — but it is close
  enough to the line that k11 should decide deliberately rather than by default.

### Is pointing Claude Code at a third-party endpoint permitted?

Anthropic's Claude Code legal page restricts **credential misuse**, not base-URL
redirection: OAuth "is intended exclusively for purchasers of Claude Free, Pro, Max,
Team, and Enterprise subscription plans," and Anthropic "does not permit third-party
developers to … route requests through Free, Pro, or Max plan credentials."
(<https://code.claude.com/docs/en/legal-and-compliance>) [S] That is the *opposite*
direction from Route A, where the third party issues its own key.

**No primary source found** that permits *or* forbids redirecting `ANTHROPIC_BASE_URL`
to a non-Anthropic model. A GitHub issue asking Anthropic for exactly this
clarification (`anthropics/claude-code#5577`) was **closed with no official answer**.
Claude Code natively supports Bedrock/Vertex/Foundry, so third-party *routing* is
contemplated; routing to a *non-Claude model* is simply unaddressed. Record this as
an unresolved licensing question, not as tacit permission.

---

## Q3 — Off-subscription blast radius

**It is per-process, and `ANTHROPIC_AUTH_TOKEN` is the right variable to use.**

Documented authentication precedence, highest first [S,
<https://code.claude.com/docs/en/authentication>]:

1. Cloud provider creds (`CLAUDE_CODE_USE_BEDROCK` / `_VERTEX` / `_FOUNDRY`)
2. **`ANTHROPIC_AUTH_TOKEN`** — sent as `Authorization: Bearer`. "Use this when
   routing through an LLM gateway or proxy."
3. `ANTHROPIC_API_KEY` — sent as `X-Api-Key`
4. `apiKeyHelper`
5. `CLAUDE_CODE_OAUTH_TOKEN`
6. Subscription OAuth from `/login`

Rank 2 sits above rank 6, so the gateway token wins **for that process**. Findings:

- **No durable write for `AUTH_TOKEN`.** Claude Code "reads environment variables at
  startup." Credentials are managed only through `/login` and `/logout` [S]. By
  contrast, `ANTHROPIC_API_KEY` in *interactive* mode prompts once and **remembers
  the choice**, persisting a fingerprint under `customApiKeyResponses` in
  `~/.claude.json` [S]. **This is a design-relevant asymmetry: a provider profile
  should set `ANTHROPIC_AUTH_TOKEN`, never `ANTHROPIC_API_KEY`**, or grove leaves
  residue in the user's config.
- **The OAuth login is not invalidated.** The login-expiry warning "appears only
  when a claude.ai or Claude Console login is the active credential, and not when a
  cloud provider, `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, or `apiKeyHelper`
  supplies the credential" [S] — i.e. the stored login persists underneath,
  merely not selected.
- **Clean return to Max: documented only for `ANTHROPIC_API_KEY`** ("Run
  `unset ANTHROPIC_API_KEY` to fall back to your subscription"). For
  `ANTHROPIC_AUTH_TOKEN` the same behaviour follows from the shared per-process
  precedence model, but **no source states it explicitly** — [I]. See *Missing
  sources*.
- **Known bug** [C]: setting `ANTHROPIC_AUTH_TOKEN` has been reported to break
  *local interactive* sessions with auth errors
  (<https://github.com/anthropics/claude-code/issues/7855>, open, `has repro`).
  grove's review session **is** interactive and TTY-owning. k11 must test this before
  committing to Route A.

---

## Q4 — The `settings.json` hazard

**This is the single largest documentation gap in the whole investigation, and it
lands squarely on grove's mechanism.**

grove sets child env via Rust `Command::env` (`loop_driver.rs`: `cmd.env("GROVE_SIGNAL_FILE", …)`).
That is the "variable already present in the process environment" case.

- **Anthropic does not document** whether the `env` block in `settings.json`
  overrides, or is overridden by, a same-named variable already in the process
  environment. The settings page describes `env` only as "Environment variables
  applied to every session and to subprocesses Claude Code spawns from it" and is
  silent on collision. **No primary source found**, either way. [P, by absence]
- **Community evidence says the `env` block wins** in v2.x — i.e. it would silently
  defeat grove's per-session override [C]:
  - `anthropics/claude-code#8500` — "Environment Variables No Longer Override
    settings.json in v2.0.1"; closed *not planned*, no explanation.
  - `lmstudio-ai/lms#561` — CC v2.1.144: shell env ignored in favour of
    `~/.claude/settings.json`; workaround was to delete the `env` block.
  - `anthropics/claude-agent-sdk-typescript#217` — **the exact `Command::env`
    analogue**: programmatic `options.env` ignored when the key exists in the user
    `env` block; "Even setting `settingSources: []` does not change this behavior."
    Open, bug-labelled.
- The **documented scope precedence** puts command-line arguments (rank 2) above
  user settings (rank 5) [S], so `--settings '{"env":{…}}'` *should* win. But
  `#8500` reports that `--settings`, `--setting-sources`, and the undocumented
  `CLAUDE_CODE_DONT_INHERIT_ENV` all failed to override; the only workaround found
  was pointing `--settings` at an *alternate file*. [C]

### Can grove detect it cheaply?

Yes — by reading the JSON, because **no CLI reports the effective resolved config**:

- `claude config` **does not exist** in v2.1.206; it errors and redirects to the
  in-session `/config` panel. [O]
- `claude auth status --json` exists and is non-interactive, but reports only
  `authMethod` / `apiProvider` / `subscriptionType` — **not** the resolved `env` or
  base URL. [O]
- Subcommands in v2.1.206: `agents, auth, auto-mode, doctor, gateway, install, mcp,
  plugin, project, setup-token, ultrareview, update`. No effective-config dump. [O]

So detection is a four-file JSON read for an `.env` key containing `ANTHROPIC_*`,
across managed → local → project → user scope.

**On this machine the hazard is latent, not live** — no `env` block exists in any
scope [O]:

```
/Users/antony/.claude/settings.json: env block = None
.claude/settings.json: (absent)
.claude/settings.local.json: (absent)
```

**Question for k11 (not answered here):** on detecting a conflicting `env` block,
should grove *warn and proceed*, or *refuse to launch the profiled session*? A warn
that the user ignores means the review silently ran on the wrong model — the exact
failure the whole feature exists to prevent.

---

## Q5 — Route B: codex as the review harness

**Route B is roughly ten changes, one of which requires disabling codex's security
model.** All [O] observations are from `codex-cli 0.142.5` on this machine; source
citations are `openai/codex@main` [S].

### Change list

| # | grove must change | Why | Risk | Class |
|---|---|---|---|---|
| 1 | `exec_bin`/`project_dir` → `codex` / `.codex` | trivial | Low | [O] |
| 2 | **Delete `name_args`** (`&["--name"]` → `&[]`) | **no such flag exists**; passing it aborts arg-parse | Low | **[O]** |
| 3 | `model_args = ["--model"]` | codex has `-m, --model <MODEL>` | Low | [O] |
| 4 | Add `--ask-for-approval never` | else the completion command blocks on a human prompt | High | [S] |
| 5 | Add `--sandbox danger-full-access` | **else the PID kill is denied** | **High** | [S] |
| 6 | Inject `GROVE_CLAUDE_PID` via `-c shell_environment_policy.set=…` | survives a user's `inherit = "core"` hardening | Medium | [S] |
| 7 | Write the grove skill to `~/.agents/skills/grove/` | codex reads a different path | Medium | [S] |
| 8 | Write methodology into `AGENTS.md` | no per-launch prompt-file flag | Medium | [S] |
| 9 | Reset terminal after kill (or `--no-alt-screen`) | codex installs **no SIGTERM handler**; leaves raw mode + alt screen | Medium | [S] |
| 10 | `continue` → `codex resume --last "<prompt>"` | different subcommand shape | Low | [O] |

### 5.1 grove's source contains a **false verification claim** [O]

`src/harness.rs` declares, for codex:

```rust
// Verified during implementation against `codex --help`; if codex
// doesn't support a session-name flag, leave empty and skip pre-naming.
name_args: &["--name"],
```

The comment asserts verification. The complete top-level long-flag list of
codex 0.142.5, dumped from `codex --help`, is:

```
--add-dir --ask-for-approval --cd --config
--dangerously-bypass-approvals-and-sandbox --dangerously-bypass-hook-trust
--disable --enable --help --image --local-provider --model
--no-alt-screen --oss --profile --remote --remote-auth-token-env --sandbox
--search --strict-config --version
```

There is no `--name` and no `-n`; `codex --help | grep -c -- '--name'` returns `0`.
Session names exist but are assigned *after* start — `codex resume [SESSION_ID]`
documents `SESSION_ID` as a "session id (UUID) **or session name**" [O], and the
name is set in-session via `/rename`. Naming at launch is an **open feature
request** (`openai/codex#22526`, `#4163`) [S].

So `name_args: &["--name"]` would make codex abort during argument parsing, before
any session starts. The `Harness` entry is wrong, and the comment is worse than
wrong — it claims a verification that contradicts what `codex --help` prints.
Filed upstream (see *Actions taken*).

### 5.2 The sandbox denies the completion kill — the landmine [S]

codex's Seatbelt base policy (`codex-rs/sandboxing/src/seatbelt_base_policy.sbpl`):

```
(deny default)
(allow signal (target same-sandbox))
```

A model-run command may signal **only processes inside its own sandbox**. The codex
parent — the PID grove wants to kill — is outside it. `kill -TERM <parent>` and
`kill -KILL <parent>` both return `EPERM`. Detached children inherit the parent's
policy, so forking the killer does not escape.

Escape hatch: `--sandbox danger-full-access` (or
`--dangerously-bypass-approvals-and-sandbox`) maps to `SandboxType::None`, which
runs the raw argv with no seatbelt wrapper. So Route B's completion handshake works
**only** with codex's sandbox and approval gate both switched off.

That is the crux of Route B's cost. grove would be adopting a second harness and
immediately disabling the property that distinguishes it.

### 5.3 What works without change

- **Positional prompt**: `codex [OPTIONS] [PROMPT]` starts an interactive TUI that
  owns the TTY [O] — the same shape as `claude`. `codex exec` is headless and
  therefore wrong for a session a human may need to interrupt.
- **Env inheritance**: default `shell_environment_policy.inherit = All`; the only
  automatic excludes are `*KEY*`, `*SECRET*`, `*TOKEN*` [S]. `GROVE_CLAUDE_PID` and
  `GROVE_SIGNAL_FILE` match none, so they pass through.
- **Death on signal**: no `SIGTERM` handler exists in `codex-rs/tui/` [S], so the
  default disposition terminates; `SIGKILL` is untrappable regardless.
- **Skills**: codex has a native Agent Skills system with the same `SKILL.md` +
  `name`/`description` frontmatter and progressive disclosure — but at
  `~/.agents/skills/` and `.agents/skills/`, not `~/.claude/skills/` [S]. The grove
  skill's *content* is largely portable; its install location and invocation surface
  are not.
- **Non-OpenAI providers**: codex supports `model_provider` + `[model_providers.<id>]`
  with `base_url` / `env_key` / `wire_api` [S]. So Route B is *also* a cross-family
  route — it just costs more.

---

## Q6 — The null route

**Argue it, don't assume it.** The honest answer is that the decisive experiment has
never been run.

### What Claude Code already gives you, same-family

- Subagents run in "its own context window"; "**Each subagent starts with a fresh,
  isolated context window.** It doesn't see your conversation history, the skills
  you've already invoked, or the files Claude has already read." [P, sub-agents doc]
- The `model:` field accepts `sonnet, opus, haiku, fable`, a full model ID, or
  `inherit` [P]. **Every option is a Claude model.** There is no documented way to
  point a subagent at another family. Native review is same-family *by construction*.
- Anthropic's own stated rationale is **fresh context, not a different family**:
  "A fresh context improves code review since Claude won't be biased toward code it
  just wrote." [S, best-practices]
- `linkuistics:doubt-driven-development` already prescribes exactly this pass, and
  grove's `driving.md` already documents it.

### The case *for* the null route (same-family fresh context is enough)

1. **The only study that isolates fresh context finds fresh context is the active
   ingredient.** *Cross-Context Review* (Tae-Eun Song, arXiv:2603.12123, 2026-03-12)
   [P, abstract verified]: CCR reached **F1 28.6%**, beating same-session self-review
   **24.6% (p=0.008, d=0.52)**. Reviewing *twice* in the same session was **worse**
   (SR2 21.7%) and did not beat one same-session review (p=0.11) — so **separation,
   not repetition, drove the gain.**
2. **Family diversity does not guarantee decorrelated errors.** A nine-judge,
   seven-family panel yielded ~"two effective votes" and negligible-to-negative lift
   over its best single member (arXiv:2605.29800) [S]. If a cross-family *panel* can
   collapse, a single cross-family reviewer's marginal value over a fresh same-family
   one is not guaranteed either.
3. **"Family" may be the wrong axis.** Post-training recipe produced larger
   behavioural divergence than the largest controlled cross-family gap
   (arXiv:2606.20632) [S, abstract only].
4. **The best-evidenced lever is an external verifier, not a rival model.** What
   rescues self-correction in the literature is an oracle or tool — a code
   interpreter, a test suite, a symbolic checker — not a second LLM (Huang et al.,
   arXiv:2310.01798; Kamoi et al., TACL 2024, arXiv:2406.01297) [S]. A `review` leaf
   that runs the build and the tests plausibly recovers more than a model swap would,
   at zero provider cost.

### The case *against* the null route (fresh context is not enough)

1. **Self-preference is not a memory artifact, so fresh context cannot remove it.**
   Panickssery, Bowman & Feng, *LLM Evaluators Recognize and Favor Their Own
   Generations* (NeurIPS 2024, arXiv:2404.13076) [P, abstract verified]: "an LLM
   evaluator scores its own outputs higher than others' while human annotators
   consider them of equal quality," and "we discover **a linear correlation between
   self-recognition capability and the strength of self-preference bias; using
   controlled experiments, we show that the causal explanation resists
   straightforward confounders.**" The judge was never told which text was its own
   and evaluated in a separate context — **which is precisely the fresh-context
   same-family condition.**
2. **The mechanism is family-level style familiarity.** Wataoka et al.
   (arXiv:2410.21819) [S] find judges over-reward low-perplexity text "regardless of
   whether the outputs were self-generated." A different Claude shares the family's
   perplexity landscape; only a different family changes the prior.
3. **Same-family self-refinement amplifies bias.** Xu et al. (ACL 2024,
   arXiv:2402.11436) [S]: GPT-4 self-bias rose 8.06 → 14.6 across ten refinement
   iterations while the model believed it was improving; external feedback reduced it.
4. **Diverse juries measurably cut intra-model bias.** PoLL (arXiv:2404.18796) [S]:
   Cohen's κ against humans 0.763 vs GPT-4's 0.627 on NQ, score-spread std 2.2 vs 6.1
   — and every model still scored *itself* highest.
5. **Practitioner claims of "40–60% more issues" from cross-model review are
   marketing** with no disclosed methodology [C]. Assign them no weight.

### The crux — **no source found**

**No study isolates (a) same-model fresh-context review against (b) different-family
review, on the same artifacts, holding everything else constant.**

- The one paper that isolates *fresh context* (arXiv:2603.12123) tests **a single
  model family**. It is a single-author preprint, n=30, synthetic injected errors,
  absolute F1 ≈29%. Its within-family result is suggestive, not settled.
- The self-preference papers show the bias *survives* fresh context but measure
  **preference/win-rate, not defect-detection recall in a review workflow**.
- PoLL conflates *diversity* with *judge count*; "Nine Judges" shows count without
  decorrelation is worthless. Neither isolates one-vs-one.

Searches that returned adjacent-but-not-isolating results are recorded under
*Missing sources*. The absence is the finding: **the amplifier's size is unmeasured.**

---

## Walk-away check

*With per-kind provider profiles removed entirely, what still works?*

**Everything the `review` kind exists for.** Verified against `src/loop_driver.rs`,
not assumed [O]:

```rust
match kind {
    ...
    Kind::Review => review,   // GROVE_REVIEW_MODEL
}
```

and, when that var is unset, `select_model` returns `None` and **no `--model` flag is
passed at all**, so the session inherits the user's own default. The kind keeps its
value: a fresh-context adversarial read, on a model the user chose, with subagents
and `doubt-driven-development` intact.

Nothing operational depends on provider profiles. No verb, no ADR, no prompt, no
brief references them; they are a *prospective* consequence of ADR
*model-per-task-kind*, never implemented.

**What is genuinely lost:**

1. The family-level, perplexity-driven self-preference blind spot stays (Q6, case
   against, items 1–2). This is a real, peer-reviewed, causally-established effect.
2. The measured intra-model-bias reduction of a diverse jury (PoLL) is forgone.
3. **The magnitude of both losses, for the single-reviewer code-review case, is
   unmeasured.** That is the whole finding.

**What is *not* lost:** the biggest evidenced lever — fresh context plus an external
deterministic verifier — is fully available on the null route.

### One further constraint on the baseline [O]

Per-kind model selection is **not released**. `GROVE_REVIEW_MODEL` appears twice in
this branch's `loop_driver.rs` and **zero times on `main`**; the Homebrew binary
(`grove-llm 9.1.0` → `main` @ `9421517`) predates commit `a1df397`
(`task-kinds-impl-k9`). Consequently the released `grove-llm kind` still knows only
`planning`/`work` and **errors** on this very leaf:

```
$ grove-llm kind
Error: task file …/10-review-provider-research-k10.md has an invalid `**Kind:**` line
```

The loop tolerates this — `resolve_kind` degrades to `None` and launches on the
default model — so nothing jams. But k11 must know that **Route A would ship on top
of an unreleased base**, and that the degrade-on-read behaviour ADR
*task-kind-taxonomy* mandates exists only on this branch.

---

## Actions taken

This leaf is AFK research and does not grow the tree. Two defects it surfaced were
therefore routed outward rather than absorbed:

- **`Linkuistics/grove#1`** — `src/harness.rs` declares codex `name_args: &["--name"]`
  under a comment claiming it was verified; the flag does not exist, and codex would
  abort during argument parsing. Also notes `model_args` can be `&["--model"]`.
  <https://github.com/Linkuistics/grove/issues/1>
- **ADR `model-per-task-kind`** — its "Two caveats bound it" paragraph is now known to
  understate the Route A environment surface. **Deliberately not edited here**: fixing
  it is a design call, and this leaf's brief scopes decisions to `review-provider-design-k11`.
  See *Synthesis → ADR reconciliation owed*.

## Missing sources — searched, not found

Recorded per `driving.md`: the absence is itself a finding, and stops a future reader
re-running the same fruitless search.

1. **Whether `settings.json`'s `env` block overrides the process environment.**
   Anthropic documents neither direction. Searched: settings, env-vars, model-config.
   Only community bug reports exist. *This is the highest-value gap.*
2. **Whether unsetting `ANTHROPIC_AUTH_TOKEN` cleanly returns to the Max
   subscription.** Documented for `ANTHROPIC_API_KEY` only. The `AUTH_TOKEN` case is
   inference from the shared precedence model.
3. **Any Anthropic statement permitting or forbidding base-URL redirection to a
   non-Claude model.** `anthropics/claude-code#5577` asked; closed unanswered.
4. **`tool_use` malformation reports for Anthropic's own `claude` client against GLM
   or Kimi.** None found. (The one Kimi interleaving bug lives in a community router
   proxy, a different code path.) A mild positive signal; not confirmation.
5. **`/cost` behaviour, and prompt-cache TTL semantics, against a third-party
   endpoint.** Billing *attribution* is documented; the readout is not.
6. **A complete list of valid codex `--model` slugs.** Only `gpt-5.5` and `o3`
   observed. Account-dependent.
7. **codex's exact exit status on SIGTERM.** Inferred from the absence of a handler;
   not confirmed (no codex session was run).
8. **The Q6 crux**: any study isolating same-model-fresh-context vs different-family
   review. Searched for cross-model vs self-review code-review empirics, heterogeneous
   multi-agent debate ablations, and judge-count-vs-judge-diversity ablations. All
   returned adjacent work only.
9. **Z.ai Pro/Max coding-plan prices** (only Lite $18 is primary-confirmed);
   **Together AI's `ANTHROPIC_BASE_URL`** (claimed, never published).

---

## Synthesis for `review-provider-design-k11`

### Which route

**Route A, or nothing.** Route B is dominated on every axis.

Route B costs ten changes (Q5), of which one — `--sandbox danger-full-access` — exists
solely to defeat the sandbox that would otherwise block grove's completion handshake.
codex *can* reach other providers (`model_provider` + `base_url`), so Route B buys
nothing Route A does not, while adding a second harness, a second skill location, a
second instruction-file convention, and a terminal-reset obligation. Route B's only
independent merit would be wanting codex *itself* as the reviewer — a different
question, not this one.

### At what cost

Route A is genuinely small, but **larger than ADR *model-per-task-kind* currently
claims**. That ADR's final consequence says the extension is bounded by "two
caveats." The evidence says otherwise:

1. `select_model` must return a *profile* (model + base URL + auth token), not an
   `Option<String>`. `launch_session` gains two `cmd.env` calls. That part really is
   ~10 lines.
2. **Use `ANTHROPIC_AUTH_TOKEN`, never `ANTHROPIC_API_KEY`** — the latter persists an
   approval fingerprint into `~/.claude.json` (Q3).
3. The review session must not pass `model` to the `Agent` tool, **or** grove must
   also export the four `ANTHROPIC_DEFAULT_*_MODEL` vars (Q1d). Since
   `doubt-driven-development` tells review sessions to spawn subagents, this is
   load-bearing, not theoretical.
4. The `settings.json` `env` block may silently defeat `Command::env` (Q4), and
   Anthropic documents nothing. Detection is a four-file JSON read; the *policy* on
   detection is k11's call.
5. `ANTHROPIC_AUTH_TOKEN` has an open bug against *interactive* sessions
   (`claude-code#7855`) — and grove's sessions are interactive. **Test before
   committing.**
6. Accept the loss of server-side `WebSearch`, Remote Control, and prompt-cache
   economics. None break the loop.

**The `complete` handshake needs no work at all.** It kills by PID; it is agnostic to
endpoint and harness alike (Q1e). The brief listed it as a Route B obstacle; it is
not one — the obstacle there is codex's sandbox, not grove's code.

### What would change grove's mind

- **Against Route A:** if `claude-code#7855` reproduces on grove's interactive
  launch, Route A is dead as specified. If a `review` session's value turns out to
  depend on `WebSearch`, likewise.
- **Against the null route:** a study isolating the crux (Q6) and finding a material
  cross-family increment in *defect-detection recall*. None exists today.
- **Toward the null route:** wiring the `review` kind to an external deterministic
  verifier (build, tests, static analysis). This is the best-evidenced lever in the
  entire literature, it is cheaper than any provider machinery, and grove does not do
  it yet. **If k11 can only afford one thing, the evidence says it should be this,
  not the provider profile.**

### Honest summary of the evidence

Fresh context is *proven* to help, within one family, in one small preprint.
Self-preference is *proven* to survive fresh context, and to be family-level. The
increment cross-family adds, on top of fresh context, for this task, is **measured
nowhere**. Cross-family review is therefore a **cheap bet on a plausible, unquantified
mechanism** — roughly $18/month and ~30 lines of driver code — not a demonstrated win.
That is a defensible bet. It is not a demonstrated requirement, and k11 should not
record it as one.

### Questions recorded for k11, not answered here

1. Detect-and-warn, or detect-and-refuse, on a conflicting `settings.json` `env` block?
2. Does the provider profile also own the four `ANTHROPIC_DEFAULT_*_MODEL` vars, or
   does grove instead document "review subagents must inherit"?
3. Is a flat-rate coding plan (Z.ai, $18/mo) inside its "officially supported tools"
   licence when driven by grove's loop? Qwen's terms explicitly exclude
   non-interactive use.
4. Should `review` gain an external-verifier obligation *before* it gains a provider
   profile?

### ADR reconciliation — discharged

This section recorded a debt: ADR *model-per-task-kind* called cross-provider
selection "a clean additive extension" bounded by "two caveats," which understates
the env surface (items 2–5 above). The debt is **paid, and by rejection rather than
by correction** — see *Outcome* at the head of this document. The ADR no longer
describes any cross-family mechanism to understate; the five-item cost surface above
is now the *evidence for* the rejection recorded in its *Considered options*.
