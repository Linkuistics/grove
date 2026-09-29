# Harness selection is owned by policy

The `harness-dispatch` executable evaluates owner-supplied policy and launches
the resulting harness, model and reasoning effort. Grove supplies its already
selected session kind, optional task file, stable task handle and unchanged
session prompt. Callers supply stable artifact identities as data; association
must survive a Grove task's retirement and reordering without parsing filenames.
The first-release discovery guarantee spans sessions in the same live grove and
workspace; design namespaces identities and keeps records outside task bodies
and the tree's teardown. Automatic cross-checkout discovery and post-teardown
artifact lookup are not required. Ordinary use of the executable requires no
Grove installation or task-tree conventions.
Selection policy is user-owned configuration, expressed as static mappings or
TypeScript computation; Grove's command configuration remains the integration
point. Existing direct-harness commands continue to work.

The executable is a separate package in this repository and initially ships
through Grove's installation and release, including everything needed to evaluate
TypeScript without a separately installed runtime. Co-location makes development,
integration tests and first deployment easier. Its interface must allow later
extraction into a separate repository without making other callers adopt Grove.
The [area specification](../specs/harness-selection-and-execution.md) owns the
protocol, identity, records and delivery contracts. The
[worker and handoff decision](policy-evaluation-precedes-process-replacement.md)
chooses the Rust front process and bundled TypeScript runtime separately from
this policy-ownership boundary.

The policy owns model preferences, reasoning effort and review-provider rules.
An explicit choice names a configured joint candidate: policy may accept or refuse
it, and the executable rejects a different returned candidate. Grove owners can
express such a choice in personal command configuration; there is no new Grove
invocation override or launch metadata in task bodies.

For the first release, a shipped example review policy requires a different
provider from the artifact's original creator for every review it selects,
including explicit choices and re-invocations. It is activated explicitly through
personal policy and tested as the delivered artifact. Direct-harness routes remain
outside its enforcement. Provider is an owner-declared candidate attribute for
model origin, not a gateway or an identity inferred from argv. The package
exposes discoverable artifact-associated creator provenance to TypeScript;
current configuration cannot reconstruct the original creator's identity. An
explicit owner declaration remedies missing execution records and is visibly
labelled as declared; absence of both forms stops review. This is a simple
original-creator comparison, with no separate model-inequality check or
multi-author exclusion set. The generic executable contains no review-specific
provider rule. The supplied Grove policy/context adapter interprets review
relationships; independent callers can supply equivalent generic associations.

Policy ownership also determines which code may run. Personal policy is loaded
by default. Repository TypeScript executes only when explicitly selected through
personal configuration or a `--config` argument. Merely cloning or entering a
repository grants no authority to run its policy, and the host discovers no
ambient repository configuration on its behalf. An incomplete mapping or
unresolved selection stops with a diagnostic; the executable invents no default
and the supplied first-release policy supplies no automatic fallback. The area
specification owns the admission, ambient-loading and inspection contracts that
carry this out.

The first release delivers static routing and a programmable TypeScript
selection interface, inspection before harness launch, and execution/outcome
records for later evaluation. The specification owns the record semantics:
what an attempt does and does not prove, and how later evidence is added. A
working local LLM selector and its pilot are follow-up work. The
[routing research](../research/grove-model-effort-routing.md)
motivates the computation boundary and documents possible future experiments;
its model comparisons are priors, not validation of a routing policy.

The cost is an additional executable and policy inspection surface. Grove's
configuration inspection can explain the wrapper's argv; the new executable
must explain its own selection. The existing contracts for
[complete commands](complete-session-configuration.md),
[personal configuration authority](untracked-configuration-delta.md) and
[foreground jobs](the-launched-child-is-a-job.md) still bind the Grove side.
Grove's pre-authoring completeness check covers its configured command; delegated
static or computed policy is validated at launch, not while mutating the tree.
This can stop an unattended run on a previously authored task and is an accepted
cost of delegation. Both inspection surfaces must make the boundary clear.

Selection helpers are not granted Grove completion authority, and the final
harness retains the driver's wrapper-exec and foreground-job contract; the
specification owns the bounds, cancellation and authority contracts that
preserve this. Trusted TypeScript is executable configuration, not a sandbox
against a hostile local owner. Dedicated integration with confined `grove run`
routes is follow-up work; lifecycle integration and independent use are the
first release.

## Considered options

- **Put selection rules in Grove.** That would couple model catalogs, provider
  conventions and selection experiments to Grove releases, and make reuse by
  other callers harder. Reconsider only if independent use is abandoned.
- **Hard-code review diversity in the executable.** The rule is an owner's
  policy and belongs with the rest of selection computation. Provenance is data
  the executable makes available, not a reason to give the core review semantics.
  Reconsider only if the tool acquires an explicit requirement to enforce this
  rule independently of its owner's configuration.
- **Create a separate repository immediately.** Independent ownership remains
  the eventual direction, but initial development, integration testing and
  deployment benefit from one repository and release route. Extract when those
  benefits no longer outweigh independent maintenance and delivery.
