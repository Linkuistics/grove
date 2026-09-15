# witness-release-orders-k38


## Goal
Prove release-order precedence and rejection of forced numeric/key reuse through
the production runtime observer.



## Context
First child of witness-interleavings-k36. Consume witnessed-binding-k35's
WitnessIo seam and native fork-isolated fixture. Replacement/epoch concurrency
is assigned to witness-replacement-observers-k39; platform evidence stays k37.

## Done when
- Directory-first, private-first, both-released and post-directory release
  controls pass with real native probes at the private filesystem/lock seam.
- A replacement tree reuses the key and simulated numeric identity. Removing
  directory verification exposes false SameTree attachment; the real-launch
  Running positive remains green. Restore production and rerun, recording exact
  subjects, digests, commands and per-case outcomes.
- Document the model boundary, preserve no-escaped-guard/read-only assertions,
  reconcile affected books, and pass focused tests and the principal gate.

## Decisions (running log)

Use deterministic native lock holders to model independent descriptor teardown.
Prepare real lease/epoch/Started bytes first; replace only their lock holders
inside the test, never the public activity provider. Force numeric equality by
rewriting only the fixture epoch's tree device/inode to a distinct replacement
root, explicitly modelling reuse rather than claiming the host reused an inode.
The separate real-launch positive guards against blanket activity rejection.

## Notes

### Mutation and restoration evidence — 2026-09-16

Base revision: `200a6de1` (witnessed-binding-k35), plus this task's diff.
Host: Darwin 25.6.0 arm64, local APFS; Homebrew rustc 1.98.1 and cargo 1.98.1.
This is deterministic release-model evidence, not the macOS/Linux process-death
evidence owned by k37.

The frozen subjects were all regular files recursively under `crates/` and
`.cargo/`, plus `Cargo.toml` and `Cargo.lock`, sorted by path (159 files).
Each manifest line is `<SHA256>  <repository-relative path>\n`.
Before/after comparisons checked every subject, not just aggregate counts.

| Subject | SHA256 |
|---|---|
| Restored observer source (`crates/grove-loop/src/driver_lease/observation.rs`) | `2062f6a138949e7648da295d8e43c7e96539911624b0797a1d94dd23bbce86f0` |
| Mutated observer source | `483c02348498f49e6fe679329466847a7616623f7c7957d3c1bee4748d660022` |
| Baseline/restored subject manifest | `1d8e84c5463cbc4d79c769ceead084189e6d2ffaeaa08bb97367aedef260c0e1` |
| Mutated subject manifest | `f0842a884329a9f285ee8f7ce2015d2e8bae437d5f5e48aa25144400ddff9a0b` |

Only mutation: in `tree_relation`, replace `if io` before
`.probe(tree.directory())` with `if false && io`, bypassing the directory probe
and taking SameTree after numeric equality. No test/provider replacement.
Neither mutation run changed any frozen subject. Restore from the saved source
and compare every subject against baseline before rerunning.

Commands use the common prefix `cargo test --locked -p grove-loop --lib` and
suffix `-- --nocapture`:

| Filter / source | Observed result |
|---|---|
| `witness_release_directory_first_reused_identity_and_key_cannot_attach` / mutant | Expected exit 101; assertion displayed old `work-k1`, `TreeRelation::SameTree`, Running on replacement `replacement-k1`. |
| `witness_real_launch_started_and_reaped_reach_public_observation` / mutant | 1 passed; Started was SameTree Running and Reaped was Idle. |
| `witness_` / restored | 17 passed, 0 failed, 0 ignored. |

Restored per-case results include directory-first reused identity/key → Busy
then Idle after private release; private-first → Idle while directory remains
contended; both-released → Idle; release after verified directory → Idle with
the exact directory-verified / both-released / private-released event trace.
The real-launch positive passed again. Every other selected witness test passed
individually, including marker/probe error precedence, identity race bounds,
read-only captures and guard release. No assertion uses elapsed sleeps.

The walkthrough owns the added test bytes in `runtime-test-release-orders`;
source ranges and chapter/whole-book totals are reconciled. Its existing
authoring contract remains in force. Complete protocol review remains assigned
to k27, with no competing in-session review.

Final verification: `bash scripts/check.sh` exited 0; all eight principal
checks passed (format, shellcheck, clippy, plugin install, conformance,
conformance suite, workspace tests and six final book validations). The loop
book reconstructs 16 files / 13,561 lines with no deferred bytes. All frozen
source subjects still matched the restored manifest after the gate. Principal
output SHA256: `854fb40fa9090aae9db032341ebc22e8c01f708810f31daaa6b3921c0eb3ba98`.

k38's close conditions hold. k36 remains live through k39; neither its
epoch-before-preparation mutation nor its concurrency obligations are claimed
by this release-model result. No parent-chain closure or ADR change is due.
