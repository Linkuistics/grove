# A review carries its creator reference

A review selected through Grove's supplied policy learns the original creator's
provider from a record in its task body. It never substitutes today's mapping
for the producer kind. Grove owns that record and its policy wrapper; Ravel has
no persistent provenance, run lookup or caller-specific protocol.

The optional Grove `withProvenance` wrapper captures the actual public Selected
provider, owner policy version, request kind/task identity and a fresh attempt
UUID. Its fixed Bash launcher passes the canonical JSON record to the managed
process as `GROVE_PROVIDER_EVIDENCE` and execs the original argv. The session
that finishes a producer copies its own record inline as
`**Creator:** recorded <JSON>` directly beneath each live review's
`**Reviews:**` line. Retirement remains a filename-only verb; the session writes
this record by hand before committing. The task commit pins it for review retries.
A node's creator is the child session which closes it, independent of that
child's kind. A finishing retry replaces earlier evidence, while an unfinished
attempt writes nothing. Missing evidence clears an earlier Creator; invalid
evidence stops for repair. An artifact without evidence needs the owner's
`**Creator:** declared <provider>`. The new review consumer refuses missing,
invalid and legacy run references with explicit remedies.

[The production and consumption contract](../grove-provider-evidence.md) owns
the exact schema, canonical validation, generic measured context shape, budgets,
confinement grants and refusal rules. Ravel's public SDK supplies ordinary
policy selection and measured reads; no Ravel feature or store is required.
The current increment supplies evidence; k42 supplies creator-aware review routing.

This reopens the earlier rejection of caller-owned launch receipts: extraction
removed the dispatch store, so the caller now preserves the provenance it needs.
An inline receipt avoids a durable lookup namespace and dangling file references.
Handles remain local to a grove; the UUID identifies the selected attempt rather
than making task handles comparable across groves. The reference dies with the
tree and its committed history remains readable without the tools.

The costs are explicit. The owner labels model origin; neither a gateway nor a
harness name proves it. The session copying the receipt attests its association
with the artifact. Same-user code can forge either; measured sources establish
which bytes a policy read, not authenticated backend identity. The wrapper adds
Bash and argv overhead, and confined callers must grant the original executable.
It preserves actual arguments, labels, settings and grants rather than inferring
an origin or changing routing choices.

Sessions explicitly still launched by the original installed-v22 driver retain
`**Creator:** run <run-id>` from their own `HARNESS_DISPATCH_RUN_ID`, together
with their original ending contract. Those references belong exclusively to the
original legacy policy/store. New Grove/Ravel consumers never access that store
and require an owner declaration or newly produced receipt instead.

## Considered alternatives

- **Look up a workspace path/task handle.** Successive groves reuse handles and
  paths, so an earlier same-handle execution can impersonate a current creator.
- **Retain or add a Ravel store/run lookup.** Violates the extracted two-verb
  process/router boundary and introduces persistence the caller does not need.
- **Recompute the producer's provider from today's routes.** A finishing attempt
  may use an earlier arrangement, gateway or child kind. The selected receipt
  preserves the actual origin instead.
- **Write an unlabelled provider string.** Hides the distinction between an
  owner declaration and selected evidence. The explicit forms retain it.
- **Keep receipts in a separate file store.** Adds mutable paths, lifetimes and
  lookup machinery. A committed inline record and attempt UUID suffice under
  the existing same-user attestation contract. Reopen if authenticated launch
  association or a durable external history becomes a requirement.
- **Require receipt taskId to equal the reviewed handle.** Refuses legitimate
  node-close attribution. The closing child's identity remains visible instead.
