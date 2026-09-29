# Harness selection is owned by policy

The new harness-selection executable evaluates owner-supplied policy and launches
the resulting harness, model and reasoning effort. Grove supplies its already
selected session kind, optional task file and unchanged session prompt. Ordinary
use of the executable requires no Grove installation or task-tree conventions.
Selection policy is user-owned configuration, expressed as static mappings or
TypeScript computation; Grove's command configuration remains the integration
point. Existing direct-harness commands continue to work.

The executable is a separate package in this repository and initially ships
through Grove's installation and release. Co-location makes development,
integration tests and first deployment easier. Its interface must allow later
extraction into a separate repository without making other callers adopt Grove.
The executable name, implementation language, TypeScript runtime and protocol
representation are subsequent design choices, not settled by this boundary.

The policy owns model preferences, reasoning effort, explicit choice handling
and review-provider rules. For the first release, the supplied review policy
requires a different provider from the artifact's original creator. The package
exposes discoverable artifact-associated execution provenance to TypeScript;
current configuration cannot reconstruct a previous producer's identity. This is
a simple original-creator comparison, with no separate model-inequality check or
multi-author exclusion set. The generic executable contains no review-specific
provider rule.

Policy ownership also determines which code may run. Personal policy is loaded
by default. Repository TypeScript executes only when explicitly selected through
personal configuration or a `--config` argument. Merely cloning or entering a
repository grants no authority to run its policy. An incomplete mapping or
unresolved selection stops with a diagnostic; the executable invents no default
and the supplied first-release policy supplies no automatic fallback.

The first release delivers static routing and a programmable TypeScript
selection interface, inspection before harness launch, and execution/outcome
records for later evaluation. A working local LLM selector and its pilot are
follow-up work. The [routing research](../research/grove-model-effort-routing.md)
motivates the computation boundary and documents possible future experiments;
its model comparisons are priors, not validation of a routing policy.

The cost is an additional executable and policy inspection surface. Grove's
configuration inspection can explain the wrapper's argv; the new executable
must explain its own selection. The existing contracts for
[complete commands](complete-session-configuration.md),
[personal configuration authority](untracked-configuration-delta.md) and
[foreground jobs](the-launched-child-is-a-job.md) still bind the Grove side.
Selection helpers receive no Grove completion authority, and the final harness
retains the driver's execution contract. Trusted TypeScript is executable
configuration, not a sandbox against a hostile local owner.

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
