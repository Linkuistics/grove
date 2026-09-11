# supplied-names-k14

## Goal

Deliver caller-supplied distinguished names and canonical rendered identity as
an independently usable API, while preserving Grove's existing filenames.

## Context

The parent owns the complete reviewed level-validation contract. This child
owns the supplied-name boundary: the trait factory's removal, initialization
and promotion inputs, caller adaptations, name-sample conformance and books.

## Done when

- Initialization takes `Option<(N, Vec<u8>)>` and promotion takes an explicit
  distinguished `N`; positioned supplied names refuse without filesystem effects.
- `EntryName::distinguished` is removed, with no compatibility factory.
  `validate_distinguished` accepts root-or-node names and a complete name set;
  invoking it from readers and projected plans belongs to the following child.
- Distinguished identity compares canonical rendering; positioned identity
  keeps view and species. Explicit conformance samples accept different valid
  distinguished values and reject broken name laws.
- A real disk test initializes with `INDEX.md`, promotes into `OVERVIEW.md`,
  checks preserved bytes and reads the resulting names through the library.
- All reference, test and Grove callers compile. Grove supplies `TaskName::Brief`
  explicitly; its current grammar and fixtures remain intact.
- Changed source roots land with book fragments, ledgers, indexes and prose;
  `bash scripts/check.sh` passes on unchanged tracked inputs.

## Notes

Reader and projected-level enforcement is deliberately the next working
increment. No level-validation guarantee is claimed for this boundary; the
complete architecture remains the parent contract. The source graph could not
start because an incompatible active generation held coordination, so source
inspection was used without disturbing it.

## Evidence

- The canonical-identity regression failed on the original implementation's
  `true` answer for different distinguished renderings, then passed with
  rendered comparison. The two-name kit and disk initialization/promotion
  demonstrations pass, as do positioned-destination refusal and existing guard,
  rollback and coarse-parts-equality tests.
- `bash scripts/check.sh` passed all eight principal checks, including the
  locked workspace suite and all six books. SHA-256 digests of every tracked
  file (sources, manifests, fixtures, scripts and documentation included) were
  identical before and after that run. Tree bookkeeping followed the run.
- Boundary and fixture decisions were recorded as they settled in the parent's
  running log. The complete parent contract remains live in `valid-levels-k16`,
  after the API review `supplied-names-k15`.
