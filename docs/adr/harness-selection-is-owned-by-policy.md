# Harness selection is owned by policy

The `harness-dispatch` executable evaluates owner-supplied policy and launches
the resulting harness, model and reasoning effort. Grove supplies its already
selected session kind, optional task file, stable task handle and unchanged
session prompt. Callers supply stable task identities as data, so an
association survives a Grove task's retirement and reordering without any
filename being parsed. The discovery guarantee spans sessions in the same live
grove and workspace. Provenance travels as globally unique run identities, so
handles need no namespace. Run and observation records live outside task bodies
and survive the tree's teardown, and a review task carries only its creator
reference. There is no automatic cross-checkout discovery and no post-teardown
artifact lookup. Ordinary use of the executable requires no Grove installation
or task-tree conventions.
Selection policy is user-owned configuration, expressed as static mappings or
TypeScript computation; Grove's command configuration is the integration
point, and a direct-harness command works beside a dispatched one.

The executable is a separate package in this repository and ships through
Grove's installation and release, with everything needed to evaluate TypeScript
without a separately installed runtime. Co-location makes development,
integration tests and deployment easier. The package depends on no Grove
package, so it can be extracted into a separate repository without making
other callers adopt Grove.
The [area specification](../specs/harness-selection-and-execution.md) owns the
protocol, identity, records and delivery contracts. The
[worker and handoff decision](policy-evaluation-precedes-process-replacement.md)
chooses the Rust front process and bundled TypeScript runtime separately from
this policy-ownership boundary.

The policy owns model preferences, reasoning effort and review-provider rules.
An explicit choice names a configured joint candidate: policy may accept or refuse
it, and the executable rejects a different returned candidate. Grove owners can
express such a choice in personal command configuration; there is no Grove
invocation override for it and no launch metadata in task bodies.

A shipped example review policy requires a different provider from the
artifact's original creator for every review it selects, including explicit
choices and re-invocations. It is activated explicitly through
personal policy and tested as the delivered artifact. Direct-harness routes are
outside its enforcement. Provider is an owner-declared candidate attribute for
model origin, not a gateway or an identity inferred from argv. The package
exposes each recorded run's provider to TypeScript, and a review names its
creator's run, as [a review carries its creator reference](a-review-carries-its-creator-reference.md)
records; current configuration cannot reconstruct the original creator's
identity. An
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
and the supplied policy supplies no automatic fallback. The area
specification owns the admission, ambient-loading and inspection contracts that
carry this out.

The executable delivers static routing and a programmable TypeScript
selection interface, inspection before harness launch, and execution/outcome
records for later evaluation. The specification owns the record semantics:
what an attempt does and does not prove, and how later evidence is added. No
local LLM selector ships. The
[routing research](../research/grove-model-effort-routing.md)
motivates the computation boundary and documents possible experiments; its
model comparisons are priors, not validation of a routing policy.

The cost is an additional executable and policy inspection surface. Grove's
configuration inspection explains the wrapper's argv; the executable explains
its own selection. The contracts for
[complete commands](complete-session-configuration.md),
[personal configuration authority](untracked-configuration-delta.md) and
[foreground jobs](the-launched-child-is-a-job.md) bind the Grove side.
Grove's pre-authoring completeness check covers its configured command; delegated
static or computed policy is validated at launch, not while mutating the tree.
This can stop an unattended run on a previously authored task and is an accepted
cost of delegation. Grove's configuration reference, its usage guide and the
configure-grove skill state that boundary for both inspection surfaces.

Selection helpers are not granted Grove completion authority, and the final
harness retains the driver's wrapper-exec and foreground-job contract; the
specification owns the bounds, cancellation and authority contracts that
preserve this. Trusted TypeScript is executable configuration, not a sandbox
against a hostile local owner. There is no dedicated integration with confined
`grove run` routes: the executable is integrated with Grove's lifecycle and
used independently.

## Considered options

- **Put selection rules in Grove.** That would couple model catalogs, provider
  conventions and selection experiments to Grove releases, and make reuse by
  other callers harder. Reconsider only if independent use is abandoned.
- **Hard-code review diversity in the executable.** The rule is an owner's
  policy and belongs with the rest of selection computation. Provenance is data
  the executable makes available, not a reason to give the core review semantics.
  Reconsider only if the tool acquires an explicit requirement to enforce this
  rule independently of its owner's configuration.
- **Keep the executable in a repository of its own.** Independent ownership
  stays possible, but development, integration testing and deployment benefit
  from one repository and release route. Extract when those benefits no longer
  outweigh independent maintenance and delivery.
