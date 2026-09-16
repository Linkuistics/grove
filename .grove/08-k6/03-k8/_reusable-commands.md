# reusable-commands-k8 — brief


## Goal

Let generic consumers define a command once, reuse it through bindings and
explicit routes, and vary shared or per-key parameters without changing argv
boundaries. Deliver the complete base-and-overlay path through Catalog and
Templates, including useful inspection and errors.



## Context

Build on `flat-provenance-k15`. Implement command definitions and the
wrapper's base `values`, `bind` and `route` patches under the reviewed grammar;
profile declarations, selection declarations and include traversal belong to
`profile-composition-k9`. Keep not-yet-supported forms explicit errors, never
silently accepted configuration. Legacy flat names remain shape-disambiguated.
Grove still calls `Templates::load`, so wrapper base patches become usable
through SessionConfig as soon as this leaf lands. Own that delivered behavior.

## Done when

- Personal definitions, binding targets, route targets and command/route value
  maps obey structural uniqueness, name/shape rules and personal/local
  restrictions. Flat and wrapper base routes share the duplicate namespace;
  a flat kind named `config` can coexist with the wrapper.
- Defaults, shared values and route overrides have the specified specificity.
  A later shared value preserves a route exception. `unset` exposes inheritance,
  including empty strings and absent/old-schema removals. Binding switches
  preserve route overrides; whole literal replacement clears them and records
  resets, and a switch from literal to binding starts a fresh route map.
- Personal targets authorize keys before local patches. Missing personal
  targets fail the entire load with `missing_target`, even when another valid
  kind is requested; local targets cannot repair them. Valid local-only keys
  stay non-admitted without poisoning other commands. Local values may complete
  a personally targeted command's required parameters.
- Effective references, values targets and parameter names validate after the
  fold. Unused command templates stay dormant; effective bindings validate their
  referenced templates even without routes. Required values are demanded only
  when instantiating admitted routes, including unused required declarations.
- Named templates tokenize before parameter substitution. Embedded and repeated
  parameters, whole-word runtime slots, `$$`, escaped parameter-looking text,
  unknown/unterminated forms, vocabulary collisions and literal executable
  restrictions match the spec. Values are opaque and never recursively expanded
  or re-split; whitespace, quotes, slot text, punctuation and empty values retain
  exact word boundaries. Reject active template/parameter/runtime NUL before
  spawn and preserve native runtime path strings. Legacy dollar scanning stays
  unchanged.
- Inspection histories include overridden assignments, removals and route
  resets; word origins include every contributing template/parameter source.
  Active diagnostics aggregate deterministically with related spans and names.
  `Templates::load` is equivalent to empty-selection Catalog resolution for
  wrapper base patches and vocabulary errors as well as flat files.
- Grove's existing SessionConfig and composed launch/mutation seams use wrapper
  base routes and local parameter overrides correctly. A fake executable sees
  the exact resolved argv; invalid base policy refuses before mutation/launch.
  Profile and selection declarations still fail explicitly at this increment.

## Verification and documentation

Use the public Catalog/Templates seam with a generic key/vocabulary; test shared
reuse, per-route exceptions, literal transitions, missing targets and exact argv
as complete user cases. Retain conformance and legacy regression coverage.
Update the keyed-launch book and crate documentation alongside implementation,
including all newly introduced modules and changed manifest bytes. Narrow the
spec's pending status to profiles and the remaining Grove adapter/CLI work.
Update `docs/CONFIGURATION.md`, relevant usage/architecture prose and the glossary
to document the wrapper base grammar Grove now accepts, its local overrides and
its explicit refusal of profiles/selections. Update affected consumer books if
their sources change. Do not postpone delivered Grove documentation to `k10` or
advertise selection policy yet. Run the root brief's common checks.

## Notes

Use the spec's grammar and record types rather than exposing an arbitrary patch
input API. This is a working generic reuse feature before profile composition;
tests that only inspect parser nodes do not establish its acceptance boundary.

## Decomposition

The template compiler, reference fold and parameter histories exceed one focused
session. Each child must land usable public behavior and current documentation:

1. `argv-nul-validation-k18`: reject NUL in flat templates and native runtime
   values before constructing Argv, retaining exact non-NUL native values.
2. `named-command-reuse-k19`: named definitions with parameter defaults, bindings
   and explicit routes work through Catalog, inspection and Grove. Parameter
   assignment/removal patches remain explicit errors until the next child.
3. `parameter-overlays-k20`: shared and route value maps, removals, target
   transitions, authorization and provenance complete this node's base contract.

## Decisions (running log)

The existing loader already reserves the parameter vocabulary prefix, but NUL
can reach Argv from both flat template words and offered runtime values. Deliver
this independently observable validation boundary first. The remaining children
own every other original acceptance criterion; profiles remain with `k9`.

## Named-command reuse handoff

`named-command-reuse-k19` now delivers command schemas/defaults, explicit target
reuse and local target replacement, safe named fragments, complete admitted
routes and multi-origin inspection through Catalog/Templates and Grove. Its
public tests and Grove fake-launch/mutation seams pass the principal checks.
The final child audits the complete k19 contract; no k19 work remains.

For `parameter-overlays-k20`, defaults already have ParameterDefault histories
and declaration origins. Named compilation retains internal literal/parameter
fragments until route instantiation; inspection and expansion then share public
literal/slot words. Required values (including unused declarations) are demanded
only for admitted routes. Add assignment/removal maps and reset histories at
that resolution boundary, preserving dormant definitions, active binding checks,
native runtime values, and source-independent snapshots. Parameter patches and
profiles/selections remain explicit errors at this handoff.
