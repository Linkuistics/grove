# The loop driver routes each session by the leaf's task kind — which harness runs it, and which model it loads

The self-driving loop launches each task's session on a harness and a model
chosen by the picked leaf's **kind** (*task-kind-taxonomy*), so a grove can grill
on a strong reasoning model, survey literature on a long-context one, and send
reviews to a different vendor entirely. Selection uses each harness's **native
launch flags** — no router, no proxy.

The slug names the model axis because that half came first; the record covers
both, because the two axes share a resolution rule (the family axis below) and
cannot be described coherently apart.

## Mechanism

- **The driver peeks — once, for both facts.** Before launching, the loop driver
  runs `grove-llm kind --with-harness`, which reads the next live leaf's
  `**Kind:**` line and, when it has one, its `**Harness:**` line. One subprocess,
  because the peek runs on every iteration and the second fact is one line away
  in the same file. This keeps the loop stateless — both are re-derived from the
  filesystem every iteration (*self-driving-loop*, constraint 1), matching the
  driver's existing role of setting launch args (it already sets the session
  name).
- **Which harness: a policy, plus a per-leaf fact.** `GROVE_<KIND>_HARNESS` names
  the harness that runs leaves of that kind, whatever the grove is stamped to.
  Unset means the **stamped** harness — which is not a default but an explicit
  binding recorded on disk. A leaf may additionally name its own harness, which
  outranks the policy. Both exist because they answer different questions: the
  env var is one rule for every grove and no tree knows about it ("reviews go to
  codex, because that is what I pay for"), while a leaf-level declaration is a
  fact about *one* leaf — this one goes elsewhere *because its sibling does not*.
  That second case is not expressible as a kind→harness function at all: the
  vendor pair runs the same kind on two vendors
  (`docs/specs/task-kind-taxonomy.md`). The full order is **leaf, kind, family,
  stamp**, and the leaf declaration is read strictly — an unrecognised name
  refuses to launch rather than degrading, since honouring it wrongly is the
  misroute it exists to prevent.
- **Which model: four keys, harness-major.** `GROVE_<HARNESS>_<KIND>_MODEL`, then
  `GROVE_<HARNESS>_<FAMILY>_MODEL`, then `GROVE_<KIND>_MODEL`, then
  `GROVE_<FAMILY>_MODEL`. The harness axis outranks the kind axis because the two
  are different kinds of constraint: the harness axis is a **correctness** axis (a
  codex profile name is garbage to pi — crossing it yields a value that is invalid
  for the binary, not merely suboptimal), while the kind axis is a **preference**
  axis (a family's model is less specific but still the user's, and still valid).
- **Routing keys on a family, not only the full kind.** Two families exist,
  `review-*` and `integrate-review-*`; the other seven kinds stand alone. Without
  a family axis, a seventeen-kind set would need the same one-line policy written
  five times and hand-kept in sync, and the parameterised taxonomy would not pay
  for itself (*task-kind-taxonomy*). This is not a new concept — grove already
  runs *specific beats general* on the harness axis; the family axis extends the
  same rule along the kind axis.
- **A reroute consults no unscoped var.** When the launch harness differs from the
  stamped one, neither the unscoped model keys nor a global binary override apply.
  A base var was written with *some* harness in mind — typically the stamped
  one — so it must not follow a kind to a different vendor. The lattice truncates
  to the two harness-scoped keys.
- **Per-harness flag template.** Each harness carries its own model-flag template
  (claude and pi: `--model`; codex: `--profile`, because a codex profile binds
  model *and* reasoning effort, which a bare model flag cannot express), parallel
  to the session-name template. *How* to pass a model is per-harness; the *value*
  comes from the env.
- **Start-path is planning by construction.** On a brand-new grove the driver
  launches `start` while `.grove/` does not yet exist (the agent runs `root-init`
  *inside* that session), so there is no leaf to peek — but `root-init`'s first
  leaf is always **planning** (*fresh-grove-start-contract*), so the `start` path
  routes as planning unconditionally.

## A kind with no model is a configuration error, not a default

**If a picked leaf's kind resolves no model var, grove fails loudly rather than
launching.** This is the load-bearing rule, and it is the *inverse* of the one
this record previously carried ("unset ⇒ no `--model` flag; the session inherits
the user's own default").

The old rule was defended on the grounds that passing `--model` unconditionally
would clobber a default the user already had. That is true, and it is also beside
the point: falling through to the harness's own default is still grove deciding
which model runs a `review-impl` leaf — it just decides less visibly. The
dangerous state is **partial configuration**, where some kinds are routed, others
silently are not, and nothing on screen distinguishes the two. Requiring the var
makes that state unrepresentable.

Three exemptions, each an *absence of the question* rather than a default:

- **No live leaf.** The finish-cycle iteration has no task to require a var for.
- **A harness whose model-flag template is empty.** It has opted out of model
  selection; requiring a flag it cannot pass would make it unlaunchable.
- **Harness absence.** No `GROVE_<KIND>_HARNESS` means the stamped harness — an
  explicit binding, recorded on disk, chosen once.

A **degraded kind peek** — the kind could not be determined at all — now bails in
every case, where it previously bailed only when a harness override was
configured. Once model selection is required, an unknown kind cannot be routed by
guessing in either axis.

## The launch model is a default, not a lock

The launch model flag is only the session's *starting* model. An in-session
`/model` switch is higher priority, so the agent or the human can change it at
will — that, and not grove machinery, is what covers "an `impl` session that turns
into substantial design work."

The asymmetry this record used to warn about — that `/model` persists into later
sessions of *unconfigured* kinds but is overridden for configured ones — largely
dissolves under the requirement above: with every kind configured, every launch
passes a flag, and no in-session switch survives into the next task. It survives
only for a harness that is exempt because it takes no model flag at all.

## Considered options

- **Unset ⇒ inherit the user's own default (the previous rule; now rejected).**
  It made grove a no-op until you opted in, never clobbered an existing default,
  and let you configure one kind and leave the rest alone. That last property is
  the defect: setting three of nine vars leaves six kinds silently routed to
  whatever your default happens to be, which reads identically to having
  configured them. Explicit beats convenient here, because the failure is silent
  and the cost of the alternative is one error message.
- **Requiring vars only once the user opts in (rejected).** With no model var set
  anywhere, treat kind-based selection as *off* and launch bare; the moment any
  var is set, require them all. This preserves out-of-the-box usability and the
  zero-subprocess launch, and it still makes partial configuration impossible.
  Rejected in favour of the simpler and more explicit rule: two modes are harder
  to reason about than one, and "grove launched without a model because you
  configured nothing" is exactly the invisible behaviour the requirement exists to
  eliminate.
- **A fallback chain across kinds (rejected — but the family axis is not one).**
  Letting an unset `GROVE_RESEARCH_MODEL` fall back to `GROVE_IMPL_MODEL` (or to a
  declared base kind) is grove choosing a model for a kind the user never
  configured, and it can silently *downgrade*: a user who sets only
  `GROVE_IMPL_MODEL=sonnet` would find research leaves demoted from their own Opus
  default to Sonnet. Under the requirement above the same reasoning now produces
  an error instead of a launch, which is strictly louder. The **family** axis is
  carved out and is different in character: `review-*` is a set the user
  explicitly configured *as a set*, so resolving `review-impl` through
  `GROVE_REVIEW_MODEL` delivers a value chosen for that leaf, not one borrowed
  from an unrelated discipline. The rejection targets falling back *across*
  disciplines; it never targeted resolving within one.
- **A model router (rejected).** claude-code-router and similar proxy Claude Code's
  API requests to route across *providers* (OpenRouter, DeepSeek, Ollama, Gemini).
  They need an API key and, on a Max subscription, either break the subscription
  billing or drain pay-per-token credits. For Opus↔Sonnet on a Max sub, native
  `--model` does the identical routing on the subscription for free — a router is
  the wrong tool and adds a proxy grove would have to own.
- **A per-leaf *model* axis (rejected for now).** Every model currently in use is
  distinguishable by harness alone (claude ⇒ claude, gpt ⇒ codex, kimi ⇒ pi), so a
  `**Model:**` line would be machinery for a case that is not live (constraint 4).
  It is purely additive when it comes — a second optional line, not a design to
  unpick. What would reopen it: one model family genuinely running on two
  harnesses.
- **Cross-family (multi-provider) selection as a *methodological* lever —
  rejected; the kind is the key, the *family* is not.** A review leaf gains most
  from a reviewer that does not share the author's family-level style prior
  (self-preference bias is causally established and family-level — Panickssery et
  al., NeurIPS 2024), so the obvious extension is to let a kind select a whole
  *provider* on that reasoning. Two routes were costed in full
  (`docs/research/cross-family-review-providers.md`); **both were rejected.**
  - *Redirect `claude`'s endpoint* (`ANTHROPIC_BASE_URL` + `ANTHROPIC_AUTH_TOKEN`
    per kind). This is not a model-selection change — it is a **credential and
    provider-configuration surface grove does not have**: a live secret to hold
    and never persist, an *undocumented* collision between `settings.json`'s `env`
    block and a child environment, an open interactive-auth bug against the one
    variable it must use (`anthropics/claude-code#7855`, and grove's sessions are
    interactive and TTY-owning), and a subagent-alias trap.
  - *Adopting a second harness purely to review on it.* Costed at ~ten changes,
    one of which existed only to defeat codex's Seatbelt policy, which would
    otherwise make grove's completion signal fail.

  What is *not* the reason: cost (trivial) or fidelity. The reason is that the
  payoff is **unquantified — no study isolates same-family fresh-context vs
  different-family review on the same artifacts** — while the cost is a
  provider/credential layer grove would have to own. The best-evidenced review
  lever is an **external deterministic verifier** (build, tests, static analysis
  as an oracle): orthogonal to provider choice, and cheaper. What would reopen
  this: a coherent provider/credential design for grove, or evidence that
  measures the cross-family increment in defect-detection recall.

  **This is untouched by the harness axis above.** `GROVE_REVIEW_HARNESS=codex` is
  a *commercial* decision — the subscription being paid for — not reviewer-bias
  mitigation, and it routes to a harness already in grove's table rather than
  building a provider layer.
- **Agent self-switch via `/model` (rejected as the mechanism).** Having the
  launched agent read its own kind and switch at session start needs no driver
  change, but the bootstrap context already ran on the wrong model, it depends on
  the agent remembering every session, and it is unverifiable in an unattended
  relaunch. Native `/model` still *layers on top* for mid-session overrides — it
  just isn't the launch-default mechanism.
- **CLI flags / config file for configuration (rejected).** `grove do` flags do not
  survive a loop restart (they aren't re-derived from state — fights
  restart ≡ continuation). A config file adds non-task state under `.grove/`
  (constraint 1). Env vars are walk-away-able (the loop's shell equivalent
  expresses them trivially), stateless, and re-derived every `grove do`.

## Consequences

- **The zero-subprocess launch is gone.** The driver previously skipped the kind
  peek entirely when no routing var was set, making an unconfigured launch
  byte-for-byte the pre-feature one. A requirement that must hold on *every*
  iteration cannot be checked by a short-circuit whose purpose is to avoid
  looking, so `grove-llm kind` now runs unconditionally on the `continue` path.
- **Not backward compatible, deliberately.** A grove that ran with no
  configuration now errors until its kinds resolve. Full coverage is about **nine**
  vars — seven standalone kinds plus two family vars — against a ceiling of 95 (17
  kinds × 5, plus 2 families × 5). The ceiling is not the burden: the stamped
  harness absorbs every kind that is not rerouted, so a realistic *policy* is two
  lines plus a per-leaf declaration on the vendor pair. **The model surface does
  not shrink the same way** — the stamp absorbs the harness axis only, so falling
  through to it still requires a model var per kind, and someone driving groves
  stamped to several harnesses needs nine *per harness* in the harness-scoped
  spelling. Measured on the first real migration: ~27 vars, not nine
  (`docs/specs/task-kind-taxonomy.md`, *Model selection is required*).
- **`GROVE_WORK_MODEL` needs no deprecation path.** `work` was renamed `impl`
  (*task-kind-taxonomy*), and because `GROVE_IMPL_MODEL` is *required*, a stale
  config fails loudly on the first launch. The error is the migration notice.
- The env vars are a public config surface, documented in `--help` and README.
- No durable state is added; restart ≡ continuation is preserved.
