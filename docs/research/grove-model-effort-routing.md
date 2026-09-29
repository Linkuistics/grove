# Grove model, effort and dispatch recommendations

Evidence checked 29 September 2026. This is a recommendation and design
exploration, not a benchmark of these models on Grove's own tasks. Personal
configuration has not been changed, and no automatic dispatcher is implemented.

## Adopted boundary

The requirements discussion adopted
[harness selection owned by policy](../adr/harness-selection-is-owned-by-policy.md):
an independent package in this repository, initially shipped with Grove, with
selection rules in personal static or TypeScript configuration. The first release
provides static routing, a programmable selection interface and evaluation
records; the local-selector pilot below remains follow-up work. The interface
examples below are proposals, not implemented APIs.

## Current policy

`grove config show --json` validated the personal configuration with
`codex-design-claude-impl` selected and no project delta. It resolves to
GPT-6 Astra and Claude Opus 5.5; Sonnet is not configured. The comment describing
a `daily`/Fable policy is stale, as is the suggestion to select a `daily` profile.
The separate `codex-led` and `claude-led` profiles contain bindings only: they
need the routes profile selected alongside them to supply lifecycle routes.

The installed task skills and their review, integration, research and editorial
family files were read. Their obligations matter to the recommendation:

- Requirements establishes the problem and sometimes bootstraps design/planning.
- Planning finds independently useful increments and their dependencies.
- Implementation can be bounded by a settled brief; integration still has to
  distinguish a true defect from a bad review finding.
- Reviews inspect and report; they cannot compensate by running the test suite.
- Combine-research must challenge shared unsupported claims, not merely merge text.
- Draft owns technical truth and structure; proof checks all defect classes
  across the document. Art means figures in the permitted medium, not necessarily
  image generation. Copy-edit has a narrower sentence/terminology charter.
- Finish can include significant preservation and, in this project, integration
  and release. Release notes has a separate, bounded source-to-summary skill.

## What others measured

These are original reports, found through general web searches, rather than
only vendor announcements. Their task definitions and harnesses differ.

| Evidence | Result useful here | Limit |
|---|---|---|
| [CodeRabbit: Sonnet 5.5](https://www.coderabbit.ai/blog/sonnet-5-5-model-review) | On 13 hard known-bug cases Sonnet caught 6, versus Opus 5.5's 8 or 10 in its two pipeline configurations. Sonnet was substantially faster than Sonnet 5. | Small set. The larger 44-PR run had quality grading pending. “Standard”/“Max” are pipeline configurations, not single effort values. |
| [CodeRabbit: Astra](https://www.coderabbit.ai/blog/gpt-6-astra-code-review-evaluation) | Hard cross-file coverage was 57.1% for Astra versus 47.6% for GPT-5.6 Sol; the overall gap was smaller. | Not a comparison against current GPT-6 Sol or Opus 5.5. Effort is not sufficiently isolated to choose a flag from this result. |
| [First-hand coding comparison](https://www.reddit.com/r/ClaudeAI/comments/1wsqp51/sonnet_55_vs_opus_55_vs_sonnet_5_in_claude_code/) | Ten tasks, three runs each, all at high effort: Sonnet 5.5 and Opus 5.5 both passed 30/30 in Claude Code; reported averages were 53 versus 108 seconds and $0.15 versus $0.36. | Self-reported, small and saturated; no medium/high comparison. Relevant to bounded execution, not architectural superiority. |
| [Artificial Analysis](https://artificialanalysis.ai/articles/claude-sonnet-5-5) | Sonnet at max used about 193k output tokens per Intelligence Index task. Across effort settings, lower token prices did not make it the cheapest option at equivalent aggregate performance. | Broad index, not Grove sessions. Pre-release deployment had a structured-output bug; relevant evaluations were due to be rerun. |
| [ToneBench](https://benchmark.towardsai.com/) and [methodology](https://benchmark.towardsai.com/methodology.html) | Effort affected writing quality and cost; Opus xhigh approached its max result at much lower cost. | Ten YouTube scripts, five drafts each, judged by LLMs against one author's style. This does not establish technical-document accuracy or copy-edit performance. |

Anthropic itself distinguishes bounded Sonnet work from open-ended work needing
Opus's judgment. Sonnet's Claude Code default is medium, and max sometimes
performed worse than xhigh when extra work exceeded the task's scope.
[Sonnet announcement](https://www.anthropic.com/claude-sonnet-5-5).
The installed Claude Code is 2.1.284, meeting the documented Sonnet 5.5 minimum.
[Claude Code model configuration](https://code.claude.com/docs/en/model-config).

No reviewed source directly establishes an optimal model/effort setting for
Grove's requirements interviews, task-tree planning, review integration, proof
or finish. The mapping below is a reasoned starting policy, with those gaps
explicitly retained. It optimizes useful quality and turnaround, not minimum
per-token price. API cost figures are not subscription-allowance predictions.

Effort should rise with the **degree of abstraction and the consequences of an
error**. Requirements, architecture and decomposition can spread a mistake
across many later tasks; their allocation must account for that downstream
rework. This is the user's policy preference and a reasoned allocation principle,
not a measured claim that a particular effort level is optimal. The evidence
above does not justify reducing the existing requirements=max or design=xhigh
settings, nor does Sonnet's behavior at max establish Astra's behavior there.
Both xhigh and max are supported by
[Astra](https://developers.openai.com/api/docs/models/gpt-6-astra).

There is no general high-effort ceiling. Consider uncertainty, coupled constraints,
the reach and reversibility of a decision, and how reliably errors can be caught.
More effort is justified where additional reasoning can address those risks;
it cannot replace missing human requirements, source evidence or verification.
Effort labels are model-specific controls, not comparable quantities across vendors.

## Recommended starting mapping

Use full IDs: Astra = `gpt-6-astra`, Sol = `gpt-6-sol`,
Opus = `claude-opus-5-5`, Sonnet = `claude-sonnet-5-5`.
Sol is an optional fourth model for bounded Codex work; retain Astra medium in
those rows if minimizing the candidate catalog is more valuable.

**Configured rule: a review uses a different provider from the original creator
of the artifact being reviewed.** All five producer/review pairs below satisfy
this rule. The recommended reviewer is conditional on its producer remaining
with the shown provider; a dynamically changed producer may require a different
reviewer. A different model or harness from the same provider is insufficient.

| Kind | Current | Recommendation | Reason or stronger-task exception |
|---|---|---|---|
| requirements | Astra max | Astra max | Retain the existing allocation: ambiguous intent and omissions can invalidate the whole tree. |
| review-requirements | Opus xhigh | Opus xhigh | Retain independent missing-requirement and falsifiability scrutiny before errors propagate. |
| integrate-review-requirements | Astra medium | Astra xhigh | Triage can change the agreed contract; max when it substantially reopens requirements. |
| design | Astra xhigh | Astra xhigh | Retain the existing allocation for abstract, coupled decisions; max for demanding invariants or costly-to-reverse architecture. |
| review-design | Opus high | Opus xhigh | Architectural defects can survive local tests and affect many increments. |
| integrate-review-design | Astra medium | Astra xhigh | Reconcile findings while preserving a coherent design; max when foundational decisions reopen. |
| planning | Opus high | Opus xhigh | Abstract dependency and increment boundaries propagate to many later sessions. |
| review-planning | Astra medium | Astra xhigh | Check completeness, dependency structure and whether each increment can actually land green. |
| integrate-review-planning | Opus medium | Opus xhigh | Repair the dependency structure coherently; high may suffice for a localized correction. |
| prototype | Opus medium | Sonnet medium | Fast reaction-producing artifact; its skill explicitly values cheap iteration. |
| review-prototype | Astra medium | Sol medium | Inspect whether the prototype answers its question; Astra for subtle feasibility claims. |
| integrate-review-prototype | Opus medium | Opus medium | Interpret what was learned, rather than polish throwaway code. |
| impl | Opus medium | Sonnet high | Promising evidence for bounded execution; consider Opus xhigh for abstract, coupled or high-consequence changes. |
| review-impl | Astra medium | Astra high | Preserve strong cross-vendor review; xhigh/max for subtle invariants or costly-to-miss defects. |
| integrate-review-impl | Opus medium | Opus medium | Bounded triage and repairs; xhigh when a finding reopens fundamental assumptions. |
| research-a | Opus high | Opus high | Breadth, sources and uncertainty. |
| research-b | Astra high | Astra high | Preserve the second vendor and identical research brief. |
| combine-research | Opus medium | Opus xhigh | Reconcile conflicting abstractions and challenge apparent agreement that will inform later decisions. |
| draft | Astra high | Astra high | Source-grounded structure and technical truth; try Opus high for prose-led documents. |
| copy-edit | Astra medium | Sonnet medium | Narrower sentence/terminology work and a different vendor from the draft. |
| art | Astra high | Sonnet medium | Contract-bound figures; escalate when the figure requires new technical reasoning. |
| proof | Astra medium | Opus high | Whole-document coherence and a different vendor from the default draft. |
| finish | Astra high | Astra medium | Most mechanics are explicit; keep high locally for Grove's integration/release or unresolved preservation work. |
| release-notes | Astra medium | Sol medium | Bounded synthesis of supplied changes and diff; keep its headless wrapper. |

The first conservative change to trial is just Sonnet high for `impl`, and
Sonnet medium for `prototype`, `copy-edit` and `art`, preserving the stronger
reviewers. Do not replace the shared Claude command's Opus default globally:
that would also move architecture review, planning and research. Use route-level
model exceptions or separate bindings. A shared high-effort profile does not
override explicit route exceptions such as requirements=max.

These are task-kind priors. A small, fully specified design may warrant less
effort; a short implementation involving subtle concurrency may warrant xhigh
or max with a suitable model. A lower allocation should be supported by the
actual task context or comparable outcomes, rather than presumed cheaper from
the task name. The proposed increases above are policy judgments to evaluate,
not effort comparisons established by the published benchmarks.

## Configuration skill

The new [configure-grove skill](../../plugins/grove/skills/configure-grove/SKILL.md)
supports inspecting/recommending policy, applying authorized personal changes,
and creating project deltas. It reads the task contracts, checks current model
support and independent evidence, and validates resolved policy and provenance.
It preserves existing harness permission settings.

Project `.grove.kdl` files are untracked, persistent overrides by kind/binding,
not one-leaf decisions. They cannot introduce commands, profiles or personally
unadmitted kinds. `grove run` ignores them entirely. The skill makes those
limitations explicit instead of promising per-invocation dispatch from a file.

## Separate executable for dispatch

The preferred boundary is the one proposed in the discussion: **Grove selects
the task; a separate executable selects and launches the harness.**
Grove supplies the kind explicitly, the task file path and the unchanged session
prompt. The dispatcher treats the kind as an opaque caller-supplied string; it
does not parse Grove filenames or require a Grove installation. It reads its own
policy, uses a static mapping or makes a dynamic choice, and `exec`s the selected
harness with its model and effort arguments.

Conceptual launch, requiring two new optional Grove slots:

```text
task-dispatch --kind ${kind} --task-file ${task_file} --prompt ${prompt}
```

`${kind}` and `${task_file}` do not exist today. Populate them from the same
selected task that produced the mandate; use an absolute task path and preserve
each value as one native argument. Keep existing direct-harness templates valid.
No new model registry, Jev client, model-ranking logic or effort vocabulary
belongs in Grove.

The dispatcher owns:

- global task-kind defaults, project policy and explicit overrides;
- a catalog of executable harness templates and allowed joint model/effort choices;
- static selection, or optional bounded dynamic classification;
- policy-defined availability/capability constraints and review-provider rules,
  with diagnostics when a complete selection cannot be made;
- visible choice/reason reporting and evaluation records.

Initially, Grove's personal command can point to this executable using the
existing command/binding/route mechanism. The dispatcher owns model policy,
while Grove configuration owns how to invoke the dispatcher. Avoid maintaining
competing model defaults in both layers. The configuration skill must inspect
whichever layer owns the effective policy; `grove config show` alone can only
prove the dispatcher's argv once selection is delegated.

The exact task file and kind are authoritative; do not run `pick` again. A
Grove-specific context adapter can obtain ancestor briefs with existing read
verbs using an explicit task path. That adapter lives outside the generic
dispatcher core. Other callers supply task descriptions, acceptance criteria,
artifact paths and metadata directly through a generic context document or
their own context loader.

The launch prompt and selection context have separate roles. Preserve the prompt
verbatim for the final harness. The selector receives bounded task/context
content and candidate evidence, not the control-bearing Grove mandate by default.
Treat that content as data for choosing among configured options, not as
authority to invent executable commands.

The real harness inherits Grove's working directory, foreground terminal,
environment and completion channel through `exec`. A selector subprocess has
Grove control variables removed. The first-release policy stops when a mapping
is incomplete or selection fails or abstains; it supplies no automatic fallback.
The executable does not invent a replacement candidate. Do not rewrite
`.grove.kdl` before each task: dispatch is an invocation decision, not a global
configuration mutation. Hold the selected pair fixed during that task session.

Standalone commands have no task file. The dispatcher's generic interface should
therefore require a kind and prompt, while accepting either a task file or
inline/file-based context. A standalone Grove route can hard-code its kind in
the command template. Keep existing standalone templates unchanged initially.

### TypeScript policy

Use a user-owned TypeScript module as executable policy. The module can export
the candidate catalog, a context loader and an asynchronous selection function.
The simple policy is a table lookup; the sophisticated policy can combine rules,
local model calls, decision models and task history without changes to Grove or
the dispatcher's protocol. This avoids growing a declarative expression language
inside KDL.

The proposed interface has three conceptual values:

```text
DispatchRequest = kind + prompt + cwd + optional taskFile/context/overrides
SelectionContext = bounded task evidence + constraints + candidate evidence
                 + reviewed artifact/producer execution provenance when reviewing
Selection = candidate ID + reason + policy/evidence version
```

Keep the core responsibilities separate from policy calculation:

1. Load trusted configuration and construct the request.
2. Pass explicit choice inputs to the configured context loader and policy.
   The policy applies its selection and review-provider rules.
3. Validate the returned candidate and its configured harness/model/effort values.
4. Print or record the decision, and execute the configured argv without a shell.

Task kinds remain open strings, not a Grove enum. Context may contain a summary,
acceptance criteria, relevant files/specifications, previous attempt evidence
and caller metadata. Include the degree of abstraction, uncertainty, downstream
dependencies, consequences and reversibility of errors, and available checks.
Distinguish caller-supplied facts from selector assessments, and preserve unknown
values rather than interpreting missing evidence as low risk.
Record which evidence the selector actually saw. If a policy lacks the context
it requires, selection stops with a diagnostic. It does not silently fill that
gap with a default, guess from a filename or treat unknown scope as easy work.

A Grove context loader can read its brief chain and producer/review evidence;
a non-Grove loader can read an issue or job description. Context assembly is
itself measurable work: cap its latency and volume, include its cost in routing
evaluation, and cache by content and policy versions. Do not stuff a whole
repository into every selection call.

TypeScript is trusted executable code, so it is not a sandbox or a security
boundary. Personal configuration or an explicit `--config` argument opts into a
project policy module; merely cloning a repository must not execute its code.
Preserve the same local-policy trust intent as Grove's untracked delta. Dynamic model output
returns a candidate ID, while trusted code owns executable paths and arguments.
The catalog and static mappings remain readable when computed selection is
disabled. A dry-run should show the chosen pair, context sources, reason and
expanded argv without launching a harness.

### Provider separation belongs to configuration

Represent provider identity explicitly in the candidate catalog: changing a
gateway, executable or model within one provider does not create independence.
Identify what is being reviewed and make its original creator's provider
discoverable by TypeScript. Keep execution provenance associated with the artifact;
current configuration cannot reconstruct it after a policy change.

The supplied policy requires
`review.provider != artifact.originalCreator.provider`. Supporting contributions
do not accumulate into an exclusion set, and the first release requires no
separate model comparison. The rule lives in static configuration or TypeScript
computation; it is not hard-coded in the generic executable. Unknown creator
provenance or an empty eligible set is an incomplete selection and stops launch.
The provenance representation and discovery mechanism remain design work; do not
infer recorded creator identity from Grove's current route or a task filename.

The TypeScript runtime, module-loading rules and exact interface are design
choices for the separate dispatcher implementation, not features claimed here.

Existing source supports the boundary: the [driver](../../crates/grove-loop/src/loop_driver.rs)
already selects the task before expanding the command; the [configuration
adapter](../../crates/grove-loop/src/session_config.rs) currently provides four
runtime slots; and [direct-execution rules](../CONFIGURATION.md#command-templates)
already allow an executable wrapper. The current advisory running-session
observer is unsuitable for recovering task identity at wrapper startup because
its marker can be published after the wrapper starts.

## Local selection with oMLX

The inspected machine has an M4 Max, 128 GiB RAM and oMLX 0.6.2. Its local
model directory contains two Qwen3.8-27B MTPLX variants. Those are available
artifacts for an initial comparison, not validated routing models. No model
download, inference benchmark or service reconfiguration was performed.

The installed oMLX request schema and server implement `structured_outputs`
with a bounded choice list and JSON schemas. This makes an existing local model
a practical first classifier: return a candidate ID or `abstain`, then validate
that ID in ordinary code. Runtime grammar availability and latency still need
a smoke test. [Upstream schema](https://github.com/jundot/omlx/blob/main/omlx/api/openai_models.py).

| Option | Fit | Qualification |
|---|---|---|
| Local classifier through oMLX | Smallest experiment with the existing stack. | Structured output is not calibrated confidence. Compare the installed models before downloading another. |
| [local-jev](https://github.com/amithgc/local-jev) | Authors measured Qwen-based typed decisions on an M4 Max with 36 GB; hardware is credible here. | Separate PyTorch/MPS stack. Reported results include prompt selection on benchmark items, so accuracy is optimistic. |
| [AnyJev](https://github.com/nokia-applied-research/AnyJev) | Useful bias correction and calibration experiments. | Its zero-label mode is uncalibrated; calibration needs labels. MLX support remains roadmap work. |
| [jevmlx](https://github.com/bnsd55/jevmlx) | Native MLX decision-model alternative worth comparing. | Different serving/runtime path; limited benchmark does not establish Grove performance. |
| [Hosted Jev](https://docs.typesafe.ai/primitives/choice) | Typed choice with option probabilities. | External service and a new dependency; the distribution is over choices, not verified downstream task success. |

AnyJev's [vLLM adapter](https://github.com/nokia-applied-research/AnyJev/blob/main/anyjev/backends/vllm.py)
requires completions with allowed token IDs and complete label log-probabilities;
its learned-head path needs the base model's hidden state. An OpenAI-compatible
API does not establish either capability. Direct AnyJev-over-oMLX compatibility
is unverified; missing label probabilities must not be silently treated as
evidence against those labels.

## Evaluate the choice, not just the classifier

Use task-kind defaults as priors and the particular task/brief as evidence.
Candidate IDs should name joint harness/model/effort configurations. Deterministic
policy first filters unavailable models, incompatible tools/context, explicit
user choices and budget constraints. Reviews also exclude the artifact's recorded
original creator's provider, irrespective of the preferred model or effort.
Classification then chooses only among eligible alternatives. An unresolved
choice stops rather than invoking an
automatic fallback.

Jev's [confidence](https://docs.typesafe.ai/confidence) is concentration of its
choice distribution. If three candidates each have a 95% chance of success,
their success probabilities can all be 0.95; a choice distribution must sum to
one. Renaming the latter “success probability” would be a mistake. Even separate
yes/no questions need empirical calibration against real outcomes.

[RouteLLM](https://github.com/lm-sys/RouteLLM) provides evidence for learning
routing from preferences. [R2-Router](https://arxiv.org/abs/2602.02823) supports
choosing model and reasoning budget jointly; its budget is output length rather
than provider-native effort, and its query benchmarks are not Grove sessions.
Neither supplies a ready-made policy for the present models and task skills.

Start with a small stratified pilot of frozen task snapshots across requirements,
design, implementation, review, planning and editorial work. Include demanding
abstractions and costly errors, and compare high/xhigh/max where supported rather
than excluding higher effort from the candidate catalog. Run a few repetitions
per configuration,
with the same source revision, brief, tools, verification and budget. This pilot
can reject poor choices; it cannot establish reliable per-kind probabilities.
Keep entire projects/workstreams together when separating training from held-out
evaluation, to avoid learning their answers from neighboring tasks.

Measure complete task outcomes: acceptance, important missed defects, false
review findings, scope drift, repair/escalation, human intervention, elapsed time,
and total usage through acceptance, including attributable downstream repairs
when an upstream decision proves wrong. Evaluate complete workstreams as well as
individual leaves; accepting a requirements or design document is not proof that
its errors have been discovered. Review quality needs known defects or
independent adjudication; a reviewer reporting no problems has not thereby passed.
Requirements/design/planning need contract-specific human judgments, while
editorial evaluation includes source fidelity and charter compliance.

Compare four policies on the same held-out tasks: current defaults, the proposed
static defaults, simple task-feature rules, and the local selector. Run the
selector in shadow mode first, recording what it would choose while the normal
mapping still executes. Shadow logs alone cannot measure unchosen alternatives;
use paired replay in isolated workspaces for that comparison.

Promote dynamic choices only where they preserve the required quality at a
useful reduction in time or total cost. Include selector overhead, model loading
and misrouting rework. Version records by task/brief content, source revision,
skill/harness versions, candidate catalog and selection policy. New models need
new evidence rather than inheriting the old model's confidence calibration.
