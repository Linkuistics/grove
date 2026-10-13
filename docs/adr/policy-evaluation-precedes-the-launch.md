# Policy evaluation precedes the launch

Independent Ravel uses a native front and a matching compiled TypeScript worker.
The front retains launch environment/terminal, evaluates bounded owner selection
without session authority, reaps the worker and then supervises the selected
process. Grove relies on that public behavior; it never calls the private worker.
[Ravel's executable specification](https://github.com/Linkuistics/Ravel/blob/v0.1.0/docs/specs/session-execution.md)
owns the detailed runtime/authority controls.

The separation keeps executable policy out of the process holding the enclosing
session channel. Grove scrubs its enclosing launch and provider-evidence
authority before invoking Ravel. Ravel forbids grants of its own and legacy
dispatch controls to the policy worker and supplies fresh ending context only
to the managed process. Grove's optional provenance wrapper
supplies selected-provider evidence as caller data; there is no Ravel run ID or
store ([creator reference](a-review-carries-its-creator-reference.md)).

The compiled worker keeps host Bun/Node out of the installed contract. Its
startup/import controls, per-target checks and runtime notices belong to Ravel,
not Grove. The transferred
[worker decision](https://github.com/Linkuistics/Ravel/blob/v0.1.0/docs/adr/policy-evaluation-precedes-the-launch.md)
and [runtime evidence](https://github.com/Linkuistics/Ravel/blob/v0.1.0/docs/runtime-evidence.md)
record the pinned observations and limits. Grove verifies the public installed
pair with fake agents; it does not duplicate those private runtime claims.

Considered alternatives:

- **One Bun executable owns everything.** Rejected because executable
  configuration would start in the same process that holds the run's exit
  channel for the harness's whole life, and runtime startup hooks would run
  before application-level environment scrubbing. A native boundary is needed
  before evaluation. Reopen if a runtime offers a verified equivalent startup,
  supervision and signal-state contract.
- **Embed a small JavaScript engine into Rust.** This makes distribution smaller
  but makes the package own TypeScript transformation, module loading and the
  host APIs needed for filesystem and network policy. Reopen if the compiled
  worker cannot meet the supported-target floor or its delivery cost outweighs
  those obligations. Do not silently fall back to a system runtime.

The [policy ownership decision](harness-selection-is-owned-by-policy.md) remains
separate: changing the TypeScript host need not move selection rules into Grove.
