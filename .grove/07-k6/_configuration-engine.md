# configuration-engine-k6 — brief


## Goal

Deliver the generic modular configuration engine through Catalog, Templates,
structured inspection and diagnostics, preserving validated Argv construction
and the existing flat Grove consumer throughout.



## Context

The root brief supplies the behavior and common verification obligations.
Read `docs/specs/modular-configuration.md` and the runner section of
`docs/specs/module-decomposition.md`; the latter defines the output records.
The runner accepts explicit source paths and arbitrary keys/slot vocabularies.
Grove's home lookup, local source admission, selection policy and VCS stay in
the consumer.

## Done when

- Catalog captures sources once, exposes optional selection declarations and
  resolves an explicit selection without further file I/O. Templates owns its
  snapshot independently of the Catalog and files.
- The complete grammar, occurrence-based profile fold, primary key authority,
  parameter specificity, safe word compilation, diagnostics and provenance
  satisfy the reviewed spec through the public configuration seam.
- `Templates::load` is the same Catalog path with empty explicit selection;
  conformance accepts Catalog plus Selection, reports semantic failures and
  rejects vacuous checks. A non-Grove consumer proves there is no hidden Grove
  vocabulary, key registry, source discovery or default-selection policy.
- The existing Grove adapter and launch tests remain green before its later
  migration. All affected source-exact books and principal checks pass.

## Decomposition

1. Captured configuration: the Catalog/Selection and output seams work for flat
   policy, with source snapshots, legacy inspection and diagnostic records.
2. Reusable commands: base command definitions, bindings and per-key parameters
   produce complete inspectable argv, including explicit local overrides.
3. Profile composition: the full language adds selected occurrences, include
   order, selection declarations and active-only semantic validation.

Each child extends a usable public configuration API with behavior observable
without its successor. Keep unsupported new forms explicitly rejected until
their owning child lands; do not accept and silently ignore unfinished syntax.
Do not expose stubbed resolution or inspection methods. In-progress notices in
the specs and user reference must distinguish the delivered generic subset from
Grove's still-flat adapter, and narrow as each child lands.

## Pointers

- Public seam: Catalog load/resolve, Templates require/expand/inspect, diagnostics
  and conformance, using temporary primary/overlay files.
- Current implementation touchpoints: keyed-launch templates, argv, vocabulary,
  errors, exports and conformance. File placement is the implementer's choice;
  keep the public interface small and follow the reviewed output contract.
- Book ownership: `docs/walkthroughs/keyed-launch/`, plus any consumer source
  changed while migrating a public signature. Discover affected corpora from
  manifests, including new modules, and validate final reconstruction.

## Notes

This is one generic-library increment within the complete feature tree. Do not
move Grove selection policy into the runner to make a test easier. The later
workspace leaf owns the adapter cutover and live-loop acceptance.
