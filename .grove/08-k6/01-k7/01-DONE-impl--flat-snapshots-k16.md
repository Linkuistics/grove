# flat-snapshots-k16


## Goal
Deliver captured flat configuration through Catalog/Selection and migrate
conformance to resolve the same captured inputs without reopening their paths.



## Context
The parent owns the complete captured-configuration contract. This child uses
the reviewed signatures from the modular and module-decomposition specs;
structured diagnostic records and aggregation belong to the next child.

## Done when
- Catalog owns original source bytes, parsed declarations and slot vocabulary;
  Templates survives source edits/removal and dropping Catalog.
- Flat selections are absent; nonempty explicit selection refuses unknown
  profiles. Unsupported wrapper forms still fail existing shape validation.
- Templates::load delegates through empty Catalog resolution. Both entry points
  reserve `param.`; legacy scanner, authority and eager validation stay intact.
- Conformance accepts Catalog plus Selection, resolves without rereading,
  rejects no admitted keys, and exercises expansion. All callers migrate.
- Public tests use an opaque non-Grove key/vocabulary and assert exact native
  argv, deterministic keys, winner source and convenience equivalence.
- Public docs and every affected source-exact book reflect this increment;
  pending diagnostics, inspection and modular syntax are explicit. Root checks pass.

## Notes
Implementation plan: first add public snapshot and conformance tests and observe
the missing API failure. Then split source capture from resolution inside the
existing template module, preserving the scanner, and migrate callers. Update
the library contract and book fragments/prose/indices, run focused tests, then
the full principal checks. Retire only this child; the parent stays live.

## Decisions (running log)

Keep capture and the existing validator together in the template module. Retain
both original documents and their compiled declarations, including overwritten
and overlay-only entries, behind shared owned storage. Resolution clones only
the winning commands; snapshots keep the captured inputs alive for the next
provenance increment. No new dependency or framework API is needed.

The bounded source review found no concrete defect in ownership, authority,
selection refusal, vocabulary reservation or conformance. The reviewer read
current sources because graph coordinates were stale after edits. Public tests
exercise these invariants; diagnostic aggregation and inspection remain the
explicit successor contracts. No additional review cycle is needed.

Verification: the new public API tests first failed on the absent Catalog and
Selection exports, then passed. `bash scripts/check.sh` passed all eight
principal checks, including workspace tests and final reconstruction of all six
books. The keyed-launch book reconstructs 2,275 lines with no deferred ranges.
SHA-256 verification before and after the full run matched every enumerated
repository file (including manifests, source, tests, fixtures, scripts and docs;
excluding ignored build output and VCS metadata). Structured diagnostics remain
live in `structured-diagnostics-k17`; the parent contract is not yet complete.
