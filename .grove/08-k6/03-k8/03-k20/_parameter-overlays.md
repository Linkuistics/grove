# parameter-overlays-k20 — brief

## Goal

Complete reusable base commands with shared and per-route parameter patches,
local overrides, removals, target transitions and their provenance.

## Context

Build on named-command-reuse-k19 and satisfy every remaining criterion in the
parent reusable-commands-k8 brief before closing that node. Profiles and selection
remain profile-composition-k9's work.

## Done when

- Values and route param/unset maps follow scope specificity, uniqueness and
  final-state validation, including empty values and old-schema removals.
- Binding switches preserve exceptions; whole literal replacement and switching
  from literal to binding reset the route map with inspectable histories.
- Missing personal targets fail the whole load; local targets cannot repair
  them. Valid local-only keys remain non-admitted, and local values can complete
  personally targeted commands. Effective references validate after the fold.
- Public-seam tests exercise every remaining parent criterion, including shared
  edits preserving route exceptions, multiple word origins, active diagnostics,
  required unused declarations, opaque values and NUL parameter rejection.
- Grove launch/mutation tests prove local parameter overrides reach exact argv
  and invalid base policy refuses before use. Loader convenience agrees with
  empty-selection Catalog resolution.
- Docs, glossary and all affected books describe the completed wrapper/base
  contract, narrowing pending status to profiles and remaining adapter/CLI work.
  Root checks pass and the parent close audits its full original Done when.

## Decomposition

The existing resolver instantiates only declaration defaults. Shared value
folding is a usable increment before route maps introduce target transitions,
resets and primary parameter-only authorization errors. Both need public tests,
Grove acceptance and source-exact documentation, so each is a focused session:

1. `shared-command-values-k23`: shared primary/local assignments and removals,
   final-state validation, exact expansion and provenance.
2. `route-parameter-overrides-k24`: route exceptions and transitions, authority,
   remaining parent acceptance and closure of both parent nodes.
