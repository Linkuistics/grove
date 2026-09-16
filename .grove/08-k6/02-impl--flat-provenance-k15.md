# flat-provenance-k15

## Goal

Make captured flat configuration inspectable through the reviewed public
provenance view. A generic consumer can explain a resolved command's sources
and overridden values and compare inspected words with actual expansion.

## Context

Depends on `captured-configuration-k7`; `reusable-commands-k8` builds on this
complete flat inspection seam. Use the Catalog's captured bytes and declarations
and the compiled representation used by Templates expansion. Do not reopen paths,
create another reader or change the legacy language.

## Done when

- Implement `Templates::inspect()` and the reviewed inspection/output records
  for flat policy. Literal targets, overwritten primary/overlay assignments,
  non-admitted local keys, sources and histories represent real captured input.
  No fabricated binding/parameter/profile activity appears for flat commands.
- Response-local IDs resolve, byte spans address captured source contents,
  ordering is deterministic, paths stay native and each compiled word carries
  its real origins. The literal/slot representation is shared with expansion;
  public records never construct an unchecked Argv.
- A non-Grove public API test loads temporary primary/overlay files, resolves,
  edits/removes the sources and drops Catalog. Inspection still explains the
  original values and exact argv. Filling inspected words with a known runtime
  context agrees with expansion, including native runtime strings.
- Legacy argv, eager validation, local-only non-admission, structured diagnostics
  and Catalog/Selection conformance from `k7` remain green. The convenience
  loader and explicit empty resolution expose equivalent inspection results.

## Verification and documentation

Use public Catalog/Templates tests, including an overwritten flat route and a
local-only key, rather than parser internals. Keep equality tests independent
enough to detect lost history or stale file reads. Update keyed-launch public
docs, module-interface implementation status and its source-exact book in this
commit, including affected manifests, fragments, indices and any corpus exception
inventory rows. Update other books if covered sources change. Inspection is now
available to library consumers; the human CLI and modular grammar remain pending.
Run the root brief's common checks before committing.

## Notes

This is a usable flat explanation feature before reusable commands exist. Later
children extend its histories and word origins to parameters and occurrences;
they do not repair a placeholder inspection result.
