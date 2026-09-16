# modular-configuration-k13

**Reviews:** modular-configuration-k4


## Goal

Adversarially review the modular configuration implementation tree against the
approved root requirements and integrated design. Produce findings about the
plan, without implementing the feature or rewriting the producer's work.



## Context

Find the producer commit by `modular-configuration-k4`; inspect its diff against
the current tree. The planned subject is `configuration-engine-k6` (children
`captured-configuration-k7`, `reusable-commands-k8`, `profile-composition-k9`),
then `workspace-configuration-k10`, `configuration-inspection-k11` and
`configuration-examples-k12`. The root's acceptance ownership map is part of the
artifact, not evidence that the map is correct.

Use `docs/specs/modular-configuration.md`, its referenced runner interface and
configuration ADRs. The design review was integrated by
`modular-configuration-k5`; re-derive planning concerns from the current contract
rather than reopening the original requirements interview.

## Done when

- Check whether each child fits one focused session and delivers usable,
  independently verifiable behavior with green callers and source-exact books.
  In particular, challenge the captured-flat → reusable-base → profiles sequence:
  can each public API/grammar boundary work honestly, without stubs, silent
  acceptance of unfinished syntax, a second loader or an expensive throwaway
  implementation? Identify any smaller useful slices the plan missed.
- Independently trace every spec acceptance row and additional module obligation
  to a concrete owner/test seam. Look especially for lost validation scopes,
  authority before overlay, profile occurrence semantics, diagnostic aggregation,
  native strings, multi-origin histories and inactive versus selected failures.
- Challenge the public-signature migration, generic non-Grove consumer, captured
  source lifetime, empty-selection loader equivalence and conformance path.
  Check that adapter/reload tests cover both mutation and launch boundaries.
- Inspect CLI dispatch and observable error obligations, including parser-level
  JSON usage failures, held leases, stale epochs and read-only working-tree
  effects. Confirm that equality with launch is conditional on the same captured
  inputs/runtime context, not an invented cross-load guarantee.
- Check that documentation and recursively included book sources are owned by
  the changing leaf; a later reconciliation must not excuse an intermediate red
  book. Check actual personal example delivery, repository-byte packaging,
  collision/race/partial-failure behavior and active policy preservation.
- Findings name the concrete contradiction or missing work and its effect;
  distinguish real gaps from preference and avoid treating the ownership table
  as proof. A clean review may retire without an integration leaf.

## Notes

If actionable findings exist, cut `integrate-review-planning` where it runs
next, before the first live implementation sibling entry. Its body should cite
this review handle, not transcribe findings as mandatory work. No review of a
review and no in-session reviewer; this leaf is the fresh adversarial context.
