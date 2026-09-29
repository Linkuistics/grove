# Selecting models for Grove work

Evaluate a **harness, model and effort combination**, not a model name in
isolation. Record the date, supported model ID, effort spelling, tool access,
context needs and pricing or subscription constraints relevant to the user.

## Build the evidence

Consult current official model and harness documentation for availability and
supported settings. Also search the general web for independent evaluations
and first-hand task results. Open the original reports: repeated summaries of
one vendor chart are one source, not corroboration.

For each useful comparison record its task type, harness, effort, sample size,
grading method, quality, latency and cost basis. Separate API list-price cost
from subscription usage. A pipeline's "Standard" or "Max" label may combine
several effort settings; do not turn it into a single CLI flag. Aggregate
leaderboards and saturated toy tasks supply weak evidence for a difficult
repository-specific decision.

Compare cost and time **per accepted task**, including retries, reviews and
human repairs, and attributable downstream rework across the task tree.
Acceptance of an upstream artifact does not establish that its errors have
been discovered. Per-token price alone does not establish which model is cheaper.
Higher effort is not guaranteed to improve results. Report missing comparative
evidence, especially for new releases and non-coding work.

## Read the task contract

Use the installed skill's obligations to distinguish:

- discovery, ambiguous requirements and architectural trade-offs;
- bounded execution with settled acceptance criteria;
- adversarial review that must discover an unstated defect;
- integration that must reject false findings as well as fix real ones;
- source-grounded research and reconciliation of conflicting evidence;
- prose editing, technical drafting and whole-document consistency checks;
- deterministic release mechanics and judgment about what must survive them.

These are features of the work, not fixed model tiers. For example, a short
concurrency fix may require more judgment than a large mechanical migration;
`proof` may need stronger reasoning than sentence-level `copy-edit`.

**Higher abstraction and/or a higher cost of error justify more effort.**
Consider coupled constraints, uncertainty, downstream dependencies, reversibility
and the strength of available checks. Requirements, design and planning often
warrant higher effort because a mistake can shape many later tasks. Do not cap
recommendations at high: include supported higher settings where the reasoning
demands or consequences justify them. Additional effort does not replace human
clarification, evidence or deterministic verification.

Treat effort as a model-specific control. Evidence that max is wasteful for one
model or task does not justify lowering another model's setting for abstract work.
Preserve an existing demanding-task allocation unless task-specific reasoning
or comparable evaluation supports changing it; state when a new recommendation
is an inference rather than a measured improvement.

Every review requires a different model provider from its producer. This requirement
does not make agreement independent evidence when both sides use the same source.
Identify review relationships from the task contracts: `review-X` examines the
artifact produced by `X`, including preceding integrations that changed it.
Inspect custom review contracts too. Editorial stages that edit the artifact
are not automatically review sessions merely because they read it; preserve
any separately specified diversity requirements for them.

## Static policy and per-invocation selection

Use task-kind defaults as a starting point. Tailor a local override using the
specific task, its briefs, uncertainty, verification strength, consequences of a
miss, degree of abstraction, downstream dependencies and observed failures.
For dynamic selection, supply these features with their supporting context;
unknown risk or missing context is not evidence for a lower allocation.
Preserve explicit user model/effort choices.
If an explicit review choice conflicts with provider separation, report the
conflict and do not apply it. Resolve an eligible choice within the authorized
scope; an explicit override does not waive provider separation.

For static policy, check each producer/review pairing under the complete
effective configuration. This proves a prospective pairing only. Grove itself
records no producer provider, and changing defaults does not change the identity
of a past producer. Check pending reviews affected by provider changes using
the human's account or trustworthy harness execution records; unknown provenance
must be resolved before activating a conflicting or unverifiable pairing.

For an external invocation-time router, require the reviewed artifact's producer
identity and actual model provider as context. The router must exclude that
provider before ranking candidates, and validate the result again before launch.
If multiple providers produced the reviewed artifact, exclude all recorded
producing providers. Identify the underlying model provider, not its gateway or
the executable hosting it. Missing provenance requires resolution, not an
assumption based on the current producer default.

Automatic pre-launch selection is a separate mechanism, not a capability of a
static `.grove.kdl`. If one exists, inspect its contract and configure only its
supported interface. Otherwise propose it separately; do not invent a Grove
hook, runtime slot or task-file override.

A learned selector needs outcomes from comparable tasks and an approved menu
of model/effort pairs. Treat its option probability as a classification signal
until held-out outcomes demonstrate calibration for successful task completion.
Compare it against fixed task-kind defaults and a simple rules baseline before
letting it change launches. Fall back to declared policy on insufficient context,
an unavailable model or a failed selection; make that fallback visible.
Fallbacks must satisfy the same review-provider constraint. If none does, stop
with an actionable diagnostic; availability or cost does not relax the rule.
