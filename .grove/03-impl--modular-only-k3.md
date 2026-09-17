# modular-only-k3

## Goal

Remove non-modular configuration support and reconcile tests, examples and
documentation with the root brief's modular-only contract.

## Context

- `modular-fixtures-k2` migrates retained-behavior consumers first.
- The reader starts at Catalog/Templates loading. Discovery found flat
  acceptance in `crates/keyed-launch/src/templates.rs` and mixed/literal-target
  handling in `templates/named.rs`; trace consumers before deleting code.
- Public inspection types are in `crates/keyed-launch/src/inspection.rs`.
  Grove's text/JSON renderers and examples installer consume those contracts.
- Packaged examples include `grove.legacy-override.example.kdl`, with installer
  inventories and success assertions that must be reconciled when it is removed.

## Done when

- Every root-brief acceptance condition holds. Public load/expansion tests pass
  with modular fixtures; meaningful old-form cases assert loader rejection and
  legacy-only success expectations are removed.
- Flat entries in either source and mixed documents fail with source-attributed,
  actionable diagnostics, including former flat keys named `config` or other
  grammar words. An ordinary shape error is sufficient; do not keep a second
  parser just to recognize the retired syntax.
- Legacy-only parser, resolver, inspection and rendering machinery is removed,
  with public modular inspection contracts and consumers kept coherent.
- Packaged samples and existing installer tests agree. Removing a packaged
  example does not delete or rewrite previously installed files or active policy.
- Existing specs, ADRs, glossary, guides and affected source-exact walkthroughs
  are current. `bash scripts/check.sh` passes on the final result.
- An audit enumerates configuration forms and classifies remaining old-form
  occurrences as rejection evidence or historical material. A clean search for
  selected legacy symbol names alone does not establish removal.

## Notes

The human has settled removal and the load/expansion test seam. Use the existing
durable documents named in the root brief, without another approval cycle or
new CLI acceptance suite. Keep both public loading entry points and current
load points. Preserve empty-document and base-only modular behavior.

The keyed-launch walkthrough covers all runner production Rust. The overview
book covers Grove's configuration reporters and example installer. Their source
fragments and explanatory text are consumers of this change, too. Apply the
ordinary Grove review threshold once the implementation artifact exists.
