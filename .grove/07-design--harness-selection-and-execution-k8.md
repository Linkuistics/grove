# harness-selection-and-execution-k8

## Goal

Redesign the harness-dispatch artifact-identity, original-creator and
supplied-review-policy area. An unattended Grove review must find its original
creator's provider without a per-review human step, and the design must hold
no Grove state that contradicts a standing decision.

## Context

This leaf exists because review `harness-selection-and-execution-k5` found the
original-creator mechanism unreachable in unattended loops (its F1). It also
found the per-grove dispatch scope in conflict with
`docs/adr/one-live-driver-per-working-tree.md` (its F2). Integration
`harness-selection-and-execution-k7` classified every k5 finding, and its running
log records the dispositions this design must satisfy. For F3 (fail-closed
review recognition), F4 (provider membership check) and F5 (no tree enumeration
the SDK cannot perform), satisfy each or record why the new design makes it
moot. The same applies to F11's association wording and the glossary's
dispatch-scope entry.

The human questioned why creators must be registered and why a handle namespace
exists. Their suggestions, none yet settled, are listed in the root brief's
design handoff. The earlier proposal k7 put to them — `grove-llm leaf-retire`
writing a `**Produced-by:**` line into the producing leaf — was not adopted, but
its analysis in k7's log still applies.

Rules any option may collide with:

- `plugins/grove/skills/grove/TASK-FORMAT.md` — the body carries no launch
  metadata, including no record of how a past session ran.
- `plugins/grove/skills/grove/references/retire.md` — retirement touches one
  filename.
- `plugins/grove/skills/grove/references/decompose.md` — Grove records nothing
  about how the producer ran.
- `plugins/grove/conformance/rules.tsv` — the corresponding conformance rules.
- The one-live-driver ADR — its rejected "generation id under `.grove/`" option
  and its statement that `.grove/` is the only durable workflow state.

The spec's affected sections are "Artifact identity and original creator",
"Supplied review policy", the `task_scope` slot and `dispatch-scope` commands in
"Grove integration", and creator revisions in "Records". The `creator.mmd` view
belongs to the same area.

## Done when

- The spec states one mechanism by which the supplied review policy obtains the
  original creator's provider for a Grove review. When the producer ran through
  dispatch, it needs no per-review human step. A declared remedy remains for
  direct-harness and pre-adoption artifacts, and missing both refuses.
  Non-Grove callers can still supply a generic reviewed-artifact/creator
  association, and the generic core still has no review semantics.
- It deterministically answers which producing invocation is the original
  creator when a leaf ran more than once. A retry of the review cannot silently
  change that answer.
- Every durable piece of state the design keeps has one home, reconciled with
  the ADR set in place. Machinery the new mechanism makes unnecessary is deleted
  from the spec, glossary, ADRs and views, not left beside it: the scope, its
  slot and commands, the inode tripwire and creator registration, where applicable.
- The human has explicitly confirmed any amendment to the brief's acceptance
  sentences or to the methodology rules above before it is adopted. If a choice
  still turns on the human's preference, stop and ask with a recommendation.
- The spec header's redesign note is removed, and the test-seam rows cover the
  new mechanism.
- As its last act, this leaf cuts `review-design` with `leaf-insert` at the
  planning leaf `harness-selection-and-execution-k6`, so the review runs before
  planning; the human required that review. Its body names this leaf's handle
  and scopes the read to the whole current design, including k7's repairs.

## Notes

The human asked not to overdevelop this rare case. Prefer the option that
removes machinery, and compare each suggestion concretely:

- Who writes the provenance, and from what? For example, dispatch could export
  the selected candidate to the final harness's environment.
- Which invocation does it name?
- How does a review find it: the task file supplied to dispatch, `grove-llm`, or
  a caller-supplied value?
- How is recorded provenance distinguished from declared?
- What happens with a review cut eagerly by a session that did not produce the
  artifact?

Grove passes an opaque command, so a Grove-runner option must say where Grove
would learn the provider. New runtime claims need their own probes and positive
controls, as k7's integration probe did.
