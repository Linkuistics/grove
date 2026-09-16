# captured-configuration-k7


## Goal

Make existing flat configuration available as an owned Catalog and resolved
Templates snapshot with structured diagnostics. A generic consumer can load
once and expand the captured command even after its input files change.



## Context

Implement the reviewed Catalog/Selection/Diagnostic seam for the
already-supported flat language. `flat-provenance-k15` adds inspection over
these captured sources next. Named commands and profiles follow in
`reusable-commands-k8` and `profile-composition-k9`. This step owns no Grove
selection policy and need not accept new wrapper forms yet.

Current conformance takes a path and Vocabulary; migrate it and its callers to
the reviewed Catalog plus explicit Selection signature in this change. Retain
`Templates::load` for existing consumers as a delegation with empty selection,
not a second reader. Discover callers afresh rather than trusting a path list.

## Done when

- Public Catalog loading captures each participating source once. For flat
  files its selection accessors return absence; an explicitly supplied unknown
  profile reports `unknown_profile`, with no fabricated source span.
- Legacy primary/overlay behavior, local-only non-admission and eager template
  validation survive. The unchanged legacy scanner and exact argv fixtures still
  pass. Vocabulary validation reserves `param.` through both entry points.
- Retain captured source bytes and parsed primary/overlay declarations needed
  by the next leaf's histories; do not reduce Catalog to only the winning text.
  Resolved keys are deterministic and `source(key)` identifies the template
  source. Inspection-only public records and `inspect()` remain unexposed until
  `flat-provenance-k15` implements them; no stub or invented history is shipped.
- Load, require and expand failures expose stable diagnostic records with real
  paths/spans where available and actionable remedies. Aggregate independent
  structural errors across readable documents without inventing cascades.
- Conformance resolves the supplied Catalog and selection without rereading
  paths, rejects an empty admitted-key set, and exercises expansion. Existing
  callers, examples and doc tests compile with its new signature.
- A non-Grove integration test uses an opaque key and slot vocabulary unrelated
  to session kinds or prompt. It proves `Templates::load` equivalence to empty
  Catalog resolution, snapshot independence after edit/removal of the sources
  and dropping Catalog, using exact expanded argv for a known context.
  Keep Argv construction private and native runtime strings lossless.

## Verification and documentation

Exercise Catalog/Templates and conformance through public APIs with temporary
files, retaining the existing legacy and launch suites. Update keyed-launch's
public docs and its source-exact book in the same commit, including the
conformance chapter, template/argv explanations, manifests, indices and new
source roots. Update any consumer books whose source changes. Preserve explicit
pending notices for inspection, the wrapper/profile language and Grove cutover;
do not claim the complete feature yet. Run the root brief's common checks.

## Notes

Do not merely publish record types for a later reader: this leaf delivers
working legacy capture, diagnostics and expansion. Later children
extend the same representation and source capture rather than building a
parallel modular loader.
