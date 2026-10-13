# grove-ravel-docs-k12 — brief

## Goal

Finish Grove's current-state ownership and installation prose after the runtime
and artifact cutovers, with the executable-owned documentation preserved in Ravel.

## Context

- Requires grove-ravel-delivery-k6's green archive/formula/check handoff; k5/k6
  already update every source-pinned page and gate-facing assertion they change.
- k3 moved the executable's spec, generic ADR content and Bun evidence into
  Ravel. Reconcile the remaining Grove documents against that committed copy.

## Done when

- README, usage, release and architecture guides, glossary, caller ADR/spec
  clauses, active walkthroughs/diagrams and plugin guidance describe independently
  installed Ravel and Grove's public CLI/parameter contract. Link to the durable
  Ravel spec/decisions/evidence; remove stale executable implementation claims
  from Grove only after verifying the destination exists. Grove keeps its
  caller contract, local runner decision and task/Creator/teardown conventions.
- Enumerate/classify old-name and ownership references over all affected prose,
  summary tables, diagrams and root/config surfaces. Retain named history and
  legacy-live-driver exceptions. Use observed positive/cross-tree controls,
  chase links and reconcile decisions in place; no duplicate extraction spec.
- Book/reference/conformance validation and `task check` pass. Record the
  independently tested source and release-ready handoff for k13. Ravel injects no instructions and owns no result meanings; Grove supplies
  end-session instructions and k8 extends its caller-owned force-stop handling.

## Notes

This is the bounded prose half split from k6, after its green artifact boundary.
No installation, publication or live-channel changes here.

## Decisions (running log)

- 2026-10-14: Decompose at the discovered documentation seam: the old executable
  specification and runtime evidence retain roughly 2,700 lines of pre-extraction
  claims, alongside nine stale visual views and independent architecture/glossary
  claims. Complete only the caller spec/views child in this session. Follow with
  ownership reconciliation and whole-surface acceptance before k13.
- Keep the existing spec/view paths as caller-owned entry points, with links to
  verified Ravel v0.1.0 documents. Preserve useful inbound anchors, explaining
  retired store vocabulary explicitly rather than keeping obsolete contracts.

## Working handoffs

1. grove-ravel-caller-views-k56: current caller spec, visual views and evidence
   locator, with destination/source/link checks and a green repository gate.
2. grove-ravel-ownership-k57: architecture, glossary/context map, remaining
   current guides, ADR/spec and walkthrough/plugin prose; chase all citations.
3. grove-ravel-prose-acceptance-k58: enumerate/classify the complete affected
   surface, observed controls, book/reference/conformance and full task check,
   then close k12 and promote the release-ready handoff for k13.

## Caller spec/views handoff — k56

The spec, nine views, manifest summaries, presentation README and runtime
evidence locator now describe the independent public CLI/caller boundary.
docs/grove-ravel-docs-evidence.md holds the bounded verified-destination,
anchor/reference, observed-control, parser, review and frozen-gate evidence.
All 18 task checks, 1,378 workspace cases, nine doctests and six source books
pass; all 46,943 measured subjects agree. The source artifact and this
decomposition land together in k56's sealed change.

k57/k58 remain live. This is no whole-repo prose acceptance or release-ready
k12 closure. Original driver/config/product installations remain preserved;
no runtime, installation, publication, new driver or live-channel change.
