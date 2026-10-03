# major-release-k21

## Goal

Prepare the major release this grove ends in (D8), and its cutover on this
machine:

- **The release notes.** `## Unreleased` opens with what an owner must do.
  - An owner policy that reads `repo` or `session_name` refuses until it is
    edited, and the sample shows the `.jj/repo` derivation.
  - `run` supervises instead of exec'ing.
  - A session ends its run with `harness-dispatch exit`, and `grove-llm
    complete` is gone.
  - `grove run` launches through dispatch, with a run record.
  - The skills and the binaries must be upgraded together.
- **Every live grove on the machine.** `docs/RELEASING.md` says what a release
  that changes the session's signal contract owes them. Their v22 drivers'
  sessions end with `complete`, which the new `grove-llm` lacks, so their
  drivers are stopped before publishing and restarted under the new pair.
- **This grove's own finish.** It knows how it ends under the v22 driver once
  `task release:major` has installed the new pair, and this leaf writes that
  into the root brief's Notes, where the finish session reads it.

## Context

- `docs/RELEASING.md`: *Prepare the release*, and *A release that changes how
  a tree is read meets every grove on the machine*, which is the precedent
  section's shape. `CHANGELOG.md`'s rules for `## Unreleased`.
- `CLAUDE.md`'s finish sequence (integration, then `task release:major`, then
  the final signal) and P5 (c)–(f) in `supervised-dispatch-k4`'s running log.
- Facts to verify, not assume:
  - whether `brew upgrade grove` removes the v22 keg, which takes the v22
    `grove-llm` with it;
  - whether a copy of the v22 `grove-llm` preserved outside the keg still
    sends `complete --done` to the v22 driver;
  - that a v22 driver mid-run finds its `harness-dispatch` from its own
    canonical executable path, which a removed keg invalidates.

## Done when

- `## Unreleased` carries the owner-facing framing above, consistent with the
  entries each leaf already logged.
- `docs/RELEASING.md` has the live-grove cutover procedure for a release that
  changes the signal contract.
- The root brief's Notes tell this grove's finish exactly how it ends after the
  release installs the new pair. Each step in them has been checked against the
  v22 binaries.
- A D1/D2 edit of the owner's installed policy is drafted for the owner. It
  must select under both v22, which still passes the parameters, and the new
  pair, which passes none. It is applied only with the owner's confirmation.
- An attended trial of the built pair, with a real harness in a scratch grove,
  has been offered to the owner and either run or declined on record. Use a
  scratch prefix, never the Homebrew one.
- `bash scripts/check.sh` passes.

## Notes

- Do not run the release here. The finish session runs it, with the owner's
  confirmation, by the sequence in `CLAUDE.md`.
- The trial exists because every live grove on the machine is the release's
  first user. The seams use fake harnesses only, by agreement (D9), so no
  automated test has watched a real interactive harness under nested job
  control, or a real harness sandbox writing its exit channel under `.jj/grove/`.
