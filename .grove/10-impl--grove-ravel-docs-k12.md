# grove-ravel-docs-k12

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
