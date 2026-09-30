# review-policy-k35 — brief

## Goal

Ship the supplied, inactive review policy as a delivered artifact. Every review
it selects uses a candidate whose provider origin differs from the original
creator's. The creator is resolved from a run reference's recorded catalog
snapshot, or from an owner's labelled declaration. A missing, unknown or
non-member creator refuses and names its remedy. The rule works for non-Grove
callers through the generic reviewed-artifact form. It also works for Grove
tasks through an adapter that reads only the supplied task file.

## Done when

- A reusable selector over a generic reviewed artifact is exported under an
  embedded `harness-dispatch/examples/…` specifier. It applies the provider rule
  to exact configured review-kind entries, and other kinds use the owner's
  static table. The rule runs on every invocation, retry and explicit choice,
  and never selects a replacement on failure.
- A run reference resolves through `host.run`. A missing run, or one carrying a
  launch-failure or not-executed detail, refuses with the declaration remedy. An
  existing run of any task identity, or one of unknown execution, is admitted
  on the reference's attestation. Inspection shows the run's task identity
  beside the reviewed ID.
- The creator's provider must be an exact, case-sensitive member of the current
  catalog's provider-origin set. A relabelled origin or a misspelt declaration
  refuses with a correction remedy. The selected candidate's origin must
  differ, and a gateway label change cannot establish separation.
- The `harness-dispatch/grove` adapter is an explicit import, versioned and
  built with the worker. It reads only the supplied task file, on every
  invocation, through the measured SDK read. For a configured review entry it
  requires exactly one standalone `**Reviews:**` line and one standalone
  `**Creator:**` line. `**Reviews:**` under a kind that is not a configured
  review entry refuses. It never derives kind or identity from the filename,
  resolves handles, enumerates the tree, reads briefs or selects another leaf.
- Inspection and the review's run record carry the evidence class, the creator
  reference, the resolved provider, and the digest of the task file the
  reference came from. The spec's shipped-examples seam row passes against the
  actual shipped files.
- The archive assertions include the adapter and the review example, and the
  per-target smoke still passes. The usage documentation gives explicit
  personal activation instructions and every refusal's remedy.

## Decomposition

1. `review-selector-k36`: the generic selector and its example policy, provider
   membership and separation, and run-reference and declaration handling.
2. `grove-review-adapter-k37`: the adapter, its fixtures, the Grove example that
   composes adapter and selector, the task-file seam cases and activation.

## Pointers

- Spec sections: `#review-policy`, `#identity-and-creator` and the shipped-examples
  row of `#test-seams`.
- ADRs: `docs/adr/a-review-carries-its-creator-reference.md` and
  `docs/adr/harness-selection-is-owned-by-policy.md`. The core stays free of
  review semantics: no review-kind list and no `Reviews` or `Creator` grammar
  in Rust.

## Review

This node is expected to end with a `review-impl` naming this node's handle.
Its last leaf cuts that review inside this node. The provider rule is the
first release's flagship requirement. A permissive comparison, or a refusal
that quietly admits, reads exactly like a working policy in every test that
does not target it.
