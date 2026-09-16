# named-command-reuse-k19

## Goal

Define a parameterized command once and reuse it through explicit bindings and
routes, with declared defaults and safe named-template compilation.

## Context

Follow the parent and modular spec. `argv-nul-validation-k18` supplies the flat
and runtime NUL boundary. This increment owns a complete usable subset, not a
parser API waiting for another child.

## Done when

- Personal wrapper command definitions, parameter declarations/defaults, binds
  and targeted routes resolve through Catalog/Templates and Grove SessionConfig.
- Local binding/route target changes and legacy whole-template replacements work;
  shared/per-route param/unset patches are explicitly refused until k20.
- Structural uniqueness, name/shape disambiguation, dormant definitions, active
  bindings, required parameters, named dollar scanning, safe substitution and
  native values satisfy the parent's applicable criteria. No arbitrary patch API.
- Inspection explains reference chains, defaults, target replacements and word
  origins. Catalog and convenience loading agree; flat compatibility remains.
- Grove fake-launch/mutation acceptance proves wrapper base behavior, local
  target overrides and refusal before use. Profiles/selections stay refused.
- Current docs and all affected books describe exactly this delivered subset;
  root checks pass. Do not advertise parameter patches until k20 lands.

## Notes

The next child adds assignment/removal patches and completes the parent's fold
semantics. Include production modules in the existing recursive book corpus.
