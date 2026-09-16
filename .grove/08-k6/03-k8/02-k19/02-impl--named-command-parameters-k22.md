# named-command-parameters-k22

## Goal

Complete named-command-reuse-k19 with declared parameters/defaults and safe
whole-word or embedded parameter substitution.

## Context

Build on named-command-targets-k21. The parent and modular specification own the
full contract; parameter assignment/removal patches still belong to k20.

## Done when

- Parameter declarations/defaults validate structure, names and duplicates. Required
  parameters are demanded only for admitted routes, even when unused in template text.
- Named templates compile parameter fragments before substitution, retaining exact
  word counts, opaque adversarial/empty values and native runtime strings. Reject
  active NUL, unknown/unterminated references and executable substitutions.
- Parameter defaults and multi-origin words appear in inspection; histories and
  reference replacements remain correct and Catalog/convenience loading agree.
- Public generic tests and Grove fake-launch/mutation acceptance demonstrate shared
  defaults, edited defaults, errors before use and all applicable parent criteria.
- Shared/route param/unset patches and profile/selection syntax still fail explicitly.
- All affected documentation/books describe the delivered subset and root checks pass.

## Notes

This is the final child of k19: audit its entire brief, including the existing
reference behavior, before closing. k20 continues to own override maps, removals,
missing-target patches and reset histories for route parameter maps.
