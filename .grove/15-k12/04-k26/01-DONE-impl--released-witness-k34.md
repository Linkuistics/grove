# released-witness-k34


## Goal
Restore Idle through production observation after a validated private witness
is released, even while its active epoch and Started bytes remain.



## Context
`driver_lease/observation.rs` already binds mandatory records under a bounded
shared epoch guard. `witnesses.rs::publish` owns the version-1 extension grammar.
Keep admission unchanged and reuse its field/hex parsing helpers.

## Done when
- Validate all version-1 fields, canonical key/handle/kind and namespace-local
  witness basename before using its published descriptor identity.
- A read-only shared private probe unlocks immediately on success and returns
  Idle independently of marker bytes; contention remains Unavailable until k35.
- Public observer tests cover released/held evidence, malformed and missing
  extension fields, mismatched identities, invalid names and substituted file
  types. Existing guard, bounded-read and admission regressions remain green.
- Update shipped behavior and affected source-derived books; focused tests and
  `bash scripts/check.sh` pass. This leaf makes no native process-death claim.

## Notes
Implementation plan: first add a production-seam test using lease preparation
and retained active bytes, observe its expected Idle failure, then implement
extension validation and bounded shared probing. Exercise malformed evidence
against a released valid positive control so blanket Unavailable cannot pass.
Update the runtime-observation walkthrough fragments and their manifest/index,
then run the principal gate, retire and seal this child. The remaining k26
children retain Running, interleavings and native platform evidence.

## Decisions (running log)

Only the final private probe can establish Idle for an active witnessed epoch.
This slice never establishes Running, so it needs no directory binding probe;
k35 must insert that probe before the private probe when adding Running.

The released-evidence behavioral claim is checked by a failing-then-passing
public observer test, complemented by validation faults interleaved with valid
Idle controls. These executable controls are the doubt instrument for this
slice; k27 already owns commissioning complete protocol review. No additional
in-session reviewer was materialised.

The first principal run exposed the existing Viewer diagnostic contract for a
legacy active epoch beside a missing tree. Keep the active-epoch explanation
as parser error context rather than exposing only a missing field name; the
existing public Viewer regression now passes without changing its assertion.

## Verification

The initial principal run passed seven checks and failed only cargo test at
`missing_tree_preserves_runtime_observation_and_recovers`; all six books passed.
Its 1,790 repository-file subject digests were unchanged before/after execution.
The missing-version diagnostic correction addresses that failure. Final results
follow from the complete rerun:

- `cargo test --locked -p grove-loop --lib driver_lease::observation`: all
  12 observer tests pass after the correction.
- `cargo test --locked -p grove-tui --test browser
  missing_tree_preserves_runtime_observation_and_recovers`: passes; the same
  case also passes in the final workspace run.
- `bash scripts/check.sh`: all eight principal checks pass. All six books
  validate; grove-loop reconstructs 12,775 lines with zero deferred lines.
- Each new case passed in both principal runs:
  `witness_released_active_epoch_is_idle_despite_leftover_bytes`,
  `witness_extension_validation_cannot_be_replaced_by_blanket_unavailability`,
  `witness_missing_replaced_and_nonregular_evidence_is_unavailable`.

Final tested revision before this evidence/retirement bookkeeping:
`3fb879af4055a6b321a4be404d0c7b7c7725ba11`, jj change `vrxswkxyqrzn`.
Observer source SHA256:
`0c4de0ac9b4493a8981c44ec20c9320918ae8ba7c186e3c2818be7f938d5b9a8`.
All 1,790 regular-file subjects from `jj file list` were hashed before and after
the final run, including manifests, sources, fixtures, scripts, skills, books
and task notes; every digest matched. The sorted path-to-digest manifest SHA256
was `af55e38871aab960f46f896c6d6361ba09e9a82703802c92388fc70d0f007ee3`.
The initial red test returned Unavailable where released evidence required Idle;
the final positive and malformed-evidence cases prevent blanket refusal from
passing. These are host observer/lease controls, not the native kill/reap,
forced-reuse or Linux evidence owed by k36/k37.
