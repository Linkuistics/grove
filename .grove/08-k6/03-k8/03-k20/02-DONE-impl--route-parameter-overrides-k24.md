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

## Decisions (running log)

- Extend the existing named fold with optional route targets and separate route
  parameter maps. Personal target authority is captured before local patches;
  binding switches retain maps, while literal transitions reset them. Validate
  surviving route assignments against the final command only for admitted keys.
- Keep one captured origin per declaration even when a target transition records
  several reset assignments. Retain reset/unset histories on the command view,
  including parameters absent from the final schema.
- Follow the reviewed spec and existing public seams: add failing generic
  acceptance tests, implement the fold, add Grove launch/mutation acceptance,
  update the source-exact books and current reference, then run principal checks
  and audit both parent briefs before retirement.

- The generic route acceptance cases failed on the former unsupported syntax
  before implementation, then all 23 named-command cases passed. The Grove fake
  executable observed exact opaque and empty local route values, shared edits
  preserving exceptions, and route unset exposing shared inheritance. Missing
  personal targets refused launch and initial scaffolding even when local policy
  supplied a complete target for that key.
- One bounded fresh-context reviewer inspected the fold against the spec and
  found no code defect. Its actionable prose finding (the spec introduction
  still called route patches pending) was corrected with the other current
  reference updates. No second reviewer is needed for that mechanical fix.
- Existing KDL accessors and shell-word splitting are unchanged; this introduces
  no new version-dependent framework decision. Only the keyed-launch book owns
  the changed production files; no corpus add/exclude exception changed.

## Parent acceptance audit

`parameter-overlays-k20` and `reusable-commands-k8` are covered as follows, subject
to the final principal-check run:

| Original obligation | Observable evidence |
|---|---|
| Grammar, names, uniqueness, primary/local restrictions and flat/wrapper namespace | Named command structural/schema/shared-value tests plus route duplicate and malformed-patch cases; flat compatibility suites |
| Defaults/shared/route specificity, empty values and absent/old-schema removals | Shared-value capture/unset cases and route specificity/switch cases in `keyed-launch/tests/named_commands.rs` |
| Binding switches preserve exceptions; literals clear maps and retain resets | Route switch test checks both changed binding and changed route targets, removed old-schema values, literal replacement history and literal-to-binding values |
| Primary target authority and final-state validation | Missing-target test reports real primary spans, local literal cannot repair, local-only target/parameter routes remain non-admitted, local values complete required parameters |
| Active references, dormant definitions and required unused parameters | Existing active-binding/default/shared-value tests plus route final-schema and unused-parameter completion cases |
| Safe named/legacy scanning, vocabulary reservation, exact words and native/NUL safety | Named scanner/default/native-value tests and k18 template/runtime tests, with new route opaque/empty/NUL assertions |
| Histories, multi-origin words, deterministic diagnostics, loader equivalence and source independence | Route capture compares Catalog/convenience after both sources are removed; switch tests assert Reset/Unset histories; diagnostics preserve source order and names |
| Grove wrapper/local behavior and refusal before launch/tree writes | `grove/tests/lifecycle_cutover.rs` records actual argv and unchanged/refused trees; `grove-llm/tests/leaf.rs` checks local route completion and missing personal authority before leaf creation |
| Current documentation and source-exact books | Reference, usage, architecture, module/spec boundary, glossary and keyed-launch source fragments/indices updated; profiles and selections remain explicitly unsupported |

Profiles/selection composition remain `profile-composition-k9` work. No root
acceptance or pending adapter/CLI work is claimed complete by closing these nodes.

## Verification and closure

`bash scripts/check.sh` passed all eight principal checks: formatting,
shellcheck, Clippy, plugin installation, skill conformance and its regression
suite, locked workspace tests, and final validation of all six books. The
keyed-launch book reconstructs 11 files and 3,755 lines with no deferred ranges.
SHA-256 digests for all 1,801 tracked inputs (production, tests, manifests,
scripts, configuration, documentation and task artifacts) matched before and
after the final run. An earlier run was deliberately stopped to add the direct
leaf-mutation acceptance case; only the completed final run is completion evidence.

The audit above is satisfied. `parameter-overlays-k20` and
`reusable-commands-k8` close with this leaf; their reusable-command handoff is
promoted to `configuration-engine-k6`. The approved ADR choices remain unchanged:
route specificity, direct argv execution and personal target authority are now
implemented for the complete base/local subset. Profile composition remains live.
