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
  `docs/ravel-target-inputs.md` and the scripts that the brief names.
  Their consumers also pin 1.0.0:
  - `scripts/test_ravel_target_inputs.py`: the `ravel-v1.0.0` archive root,
    line 18.
  - `scripts/ravel-target-inputs.test.sh`: archive names and prefixes, lines
    22–29 and 68–71.
  - `scripts/release-smoke.sh`: three archive names, lines 211, 344 and 394.
  - `crates/grove-loop/tests/session_result_installed.rs`: the version
    assertion at line 157.
- The overview book quotes the whole resolver file as fragment
  `ravel-resolution` in `docs/walkthroughs/overview/06-standalone-invocations.md`
  (source lines 1-313, indexed in `source-index.md` and `concept-index.md`).
  `book-check` rebuilds it, so a resolver change needs the chapter edit in
  the same leaf.
- Installed Ravel on this machine is 1.1.0. Check it with `ravel --version`.

## Done when

- `ravel 1.0.0` and `ravel 1.0.9` are refused with the upgrade remedy.
  `ravel 1.1.0` and `ravel 1.12.3` pass. The malformed-version cases still
  refuse.
- Every fixture that prints `ravel 1.0.0` as a valid version prints 1.1.0.
  This includes the assertion in `session_result_installed.rs` and the
  `grove-installed-smoke.sh` version check.
- Each consumer of the archive layout expects the `ravel-v1.1.0` root and
  names: `test_ravel_target_inputs.py`, `ravel-target-inputs.test.sh`,
  `release-smoke.sh` and the five scripts of the brief. A search of
  `scripts/` and `Taskfile.yml` for `ravel-v1.0.0` and `ravel 1.0.0` finds only
  dated evidence. Show that the search finds a seeded match.
- Chapter 06 of the `overview` book, its `source-index.md` entry and its
  `concept-index.md` entry match the changed resolver file, including the new
  line count. The conversion of chapter 06 lands first as its own change.
- Each pin of a Ravel archive in the release inputs names a Ravel 1.1.0 archive.
  Each SHA-256 digest is measured from the published release for all three
  targets. The measurement is recorded in `docs/ravel-target-inputs.md`.
  The dated measurement of the earlier run stays unchanged.
- `ravel:targets:prepare` and the Ravel inputs in `docs/RELEASING.md` describe
  Ravel 1.1.0.
- `bash scripts/check.sh` passes. These tasks pass with `RAVEL` set to the
  installed front: `ravel:resolution`, `ravel:fixtures` and `ravel:result`.
  `ravel:targets:controls` passes with `ARCHIVES` set to the directory of the
  1.1.0 archives (`Taskfile.yml` requires that variable, and `RAVEL` alone does
  not run it).
- A changed record passes the technical-communication checker. Convert each
  edited record as its own change before the edit.

## Notes

- The digests need the published release assets of Ravel 1.1.0. If the
  network is not available, stop and say so. Never invent a digest.
- Comments that cite the `v1.0.0` documents of Ravel in `session_command.rs`
  and `session_result.rs` are source of the `grove-loop` book. The
  later leaves that edit those files decide on them. Leave them here.
- Chapter 06 of the `overview` book is edited again by `ravel-job-k7` and
  `grove-run-job-k8`. This leaf converts it, and the later leaves reuse that
  conversion.
- This leaf may split at the seam between code and release inputs. The code half
  is small. The release half is mostly mechanical.
