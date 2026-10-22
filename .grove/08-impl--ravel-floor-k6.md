# ravel-floor-k6

## Goal

Make Grove require Ravel 1.1.0. The resolver then refuses 1.0.x and accepts
stable Ravel 1.x from 1.1.0. The release inputs pin the Ravel 1.1.0 archives.
This leaf comes first because the later leaves rely on the corrected failure
paths of Ravel 1.1.0. It also touches none of the code that the later leaves
change.

## Context

- The root brief: *Requirements* ("Grove requires Ravel 1.1.0"), *Tests*
  (the resolver cases) and *The 1.1.0 floor in the release inputs*.
- The resolver and its inline tests are in `crates/grove/src/dispatch/ravel.rs`.
  The refusal keeps its remedy, `brew upgrade ravel`.
- The release inputs are in `scripts/ravel_target_inputs.py`,
  `docs/ravel-target-inputs.md` and the five scripts that the brief names.
- Installed Ravel on this machine is 1.1.0. Check it with `ravel --version`.

## Done when

- `ravel 1.0.0` and `ravel 1.0.9` are refused with the upgrade remedy.
  `ravel 1.1.0` and `ravel 1.12.3` pass. The malformed-version cases still
  refuse.
- Every fixture that prints `ravel 1.0.0` as a valid version prints 1.1.0.
- Each pin of a Ravel archive in the release inputs names a Ravel 1.1.0 archive.
  Each SHA-256 digest is measured from the published release for all three
  targets. The measurement is recorded in `docs/ravel-target-inputs.md`.
  The dated measurement of the earlier run stays unchanged.
- `ravel:targets:prepare` and the Ravel inputs in `docs/RELEASING.md` describe
  Ravel 1.1.0.
- `bash scripts/check.sh` passes. These tasks pass with `RAVEL` set to the
  installed front: `ravel:resolution`, `ravel:fixtures`, and
  `ravel:targets:controls`.
- A changed record passes the technical-communication checker. Convert each
  edited record as its own change before the edit.

## Notes

- The digests need the published release assets of Ravel 1.1.0. If the
  network is not available, stop and say so. Never invent a digest.
- Comments that cite the `v1.0.0` documents of Ravel in `session_command.rs`
  and `session_result.rs` are source of the `grove-loop` book. The
  later leaves that edit those files decide on them. Leave them here.
- This leaf may split at the seam between code and release inputs. The code half
  is small. The release half is mostly mechanical.
