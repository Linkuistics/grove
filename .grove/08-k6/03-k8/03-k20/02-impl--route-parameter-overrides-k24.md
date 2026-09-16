# route-parameter-overrides-k24


## Goal

Complete parameter-overlays-k20 with route parameter maps, transitions, authority
and provenance, auditing all original reusable-commands-k8 acceptance criteria.



## Context

Build on shared-command-values-k23. The shared `values` scope already assigns,
removes, validates final values and explains them. Profiles remain k9's work.

## Done when

- Route `param`/`unset` maps and parameter-only routes obey grammar, uniqueness,
  final-schema validation and default/shared/route specificity. Shared edits
  preserve route exceptions; absent and old-schema removals are legal.
- Binding switches preserve exceptions; whole literal replacement and switching
  from literal to binding reset maps with inspectable histories.
- Missing personal targets fail the whole load with real personal origins;
  local targets cannot repair them. Local values may complete personal targets;
  valid local-only keys remain non-admitted without poisoning other commands.
- Public and Grove launch/mutation tests cover route overrides, resets, missing
  targets, exact argv, deterministic diagnostics and convenience equivalence.
- Audit every Done when in parameter-overlays-k20 and reusable-commands-k8;
  close both only when fully satisfied, promoting needed context upward.
- Docs/glossary/all affected source-exact books describe completed wrapper/base
  behavior, narrowing pending status to profiles and adapter/CLI work; root
  checks pass.

## Notes
