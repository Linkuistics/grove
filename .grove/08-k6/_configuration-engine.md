# configuration-engine-k6 — brief


## Goal

Deliver the generic modular configuration engine through Catalog, Templates,
structured inspection and diagnostics, preserving validated Argv construction
and legacy Grove behavior throughout, with explicit interim language boundaries.



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
- Grove uses wrapper base commands after `reusable-commands-k8`. Once profiles
  parse, `profile-composition-k9` guards the adapter against silently dropping
  either source's selection declaration. No declaration means empty selection;
  a present declaration refuses until `workspace-configuration-k10` implements
  the policy. Existing launch tests and all affected books/checks remain green.

## Decomposition

1. `captured-configuration-k7`: Catalog/Selection, source snapshots, structured
   diagnostics and conformance work for flat policy, without stub inspection.
2. `flat-provenance-k15`: flat inspection and provenance explain the captured
   commands and agree with expansion after source changes and Catalog disposal.
3. Reusable commands: base command definitions, bindings and per-key parameters
   produce complete inspectable argv, including explicit local overrides.
4. Profile composition: the full generic language adds selected occurrences,
   include order, declarations and active-only validation; Grove explicitly
   refuses declarations until its selection policy lands.

Each child extends a usable public configuration API with behavior observable
without its successor. Keep unsupported new forms explicitly rejected until
their owning child lands; do not accept and silently ignore unfinished syntax.
Do not expose stubbed resolution or inspection methods. In-progress notices in
the specs and user reference must describe both the delivered generic subset and
Grove's actual accepted/refused forms, and narrow as each child lands.

## Pointers

- Public seam: Catalog load/resolve, Templates require/expand/inspect, diagnostics
  and conformance, using temporary primary/overlay files.
- Current implementation touchpoints: keyed-launch templates, argv, vocabulary,
  errors, exports and conformance. File placement is the implementer's choice;
  keep the public interface small and follow the reviewed output contract.
- Book ownership: `docs/walkthroughs/keyed-launch/`, plus any consumer source
  changed while migrating a public signature. Discover affected corpora from
  manifests, including new modules, and validate final reconstruction.
  Any `[[corpus.add]]` or `[[corpus.exclude]]` change also updates the matching
  exception inventory in `docs/specs/walkthrough-books.md` and passes the
  repository inventory test. New inline test files do not earn an exclusion
  merely by appearing in a manifest; production modules remain in the corpus.

## Notes

This is one generic-library increment within the complete feature tree. Do not
move Grove selection policy into the runner to make a test easier. The later
workspace leaf replaces the temporary declaration guard with selection policy
and owns the complete live-loop acceptance.
