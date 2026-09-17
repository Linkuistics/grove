# modular-loader-k10

## Goal

Complete modular-only configuration loading, remove compatibility machinery, and
reconcile the durable contracts and source-exact books with the result.

## Context

The parent brief carries all original modular-only-k3 acceptance conditions.
`modular-examples-k9` removed the packaged flat sample, installer inventory entry,
legacy example launch case and corresponding overview fragment. It did not
change loading or inspection. The general reference changed only its sample
count; the release smoke recipe still needs migration.

Graph discovery found Catalog::load and Templates::load in
`crates/keyed-launch/src/templates.rs`, mixed namespace capture in named::parse,
and literal-target folding in named::resolve in `templates/named.rs`.
`inspection.rs` retains LiteralTemplate/Reset and optional command/binding fields.
Recheck their consumers before removal. The graph's docs subtree is excluded;
read the documentation directly.

## Done when

- The parent and root briefs' remaining acceptance conditions hold, including
  source-attributed rejection of flat and mixed documents and grammar-word keys.
- Public load/expansion assertions preserve modular behavior and empty documents;
  obsolete flat success cases and machinery are removed.
- Inspection types and Grove text/JSON consumers agree, without legacy-only paths.
- Specs, ADRs, glossary, guides, release smoke recipe and affected source-exact
  walkthroughs are current. Enumerate configuration forms across the repository
  and classify old-form survivors as rejection or historical evidence.
- `bash scripts/check.sh` passes; decide review under the usual Grove threshold.

## Notes

Keep the settled public load/expansion test seam; no new CLI acceptance suite.
The root's fixture migration handoff identifies remaining legacy-specific tests.
Do not rewrite active personal configuration or previously installed samples.
