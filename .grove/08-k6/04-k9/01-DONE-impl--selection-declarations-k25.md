# selection-declarations-k25


## Goal
Capture optional selection declarations through Catalog and prevent Grove from
silently ignoring them before workspace selection policy lands.



## Context

## Done when
- Both sources accept one wrapper `select` with zero or more valid profile names,
  retaining list order, repetitions and the declaration's real source span.
  Absence and explicit empty selection remain distinct; malformed/duplicate
  declarations fail structurally. Legacy flat grammar words remain keys.
- Templates convenience ignores declarations and resolves the empty list;
  explicit unknown names fail resolution/conformance against captured origins.
- Grove captures Catalog after existing source admission and refuses either
  declaration, including empty, before mutation or launch with an actionable
  source-bearing message. No declaration preserves existing behavior.
- Public-seam and real Grove regression tests pass, documentation and affected
  source-exact books describe this boundary, and scripts/check.sh passes.

## Notes
Profiles remain explicitly unsupported in this increment. The next child owns
their full grammar, occurrence fold and inactive-profile launch coverage.

## Decisions (running log)

The selected parent decomposes at a usable public seam: declaration capture and
the consumer guard land together, before accepting any profile syntax. This
prevents a future grammar extension from silently discarding declared policy.
The reviewed grammar and API remain unchanged; no new selection policy is added.

Catalog stores selection declarations in the captured named declarations and
returns borrowed Selection records with real UTF-8 byte spans. The convenience
loader continues to resolve an explicit empty list. Grove inspects both captured
declarations after source admission and refuses them before resolution; the
profile-occurrences child extends this guard with profile-backed fixtures.

The single bounded adversarial review found no substantive issues. Public
capture/conformance tests and real Grove launch/root-creation/leaf-add tests
exercise the declaration boundary. All eight principal checks pass, including
workspace tests and final reconstruction of all six books; the 1,774 recorded
check-input digests were unchanged across the run.
