# parity-fixture-k7

## Goal

Record what today's resolver produces from the pinned configuration, as the
fixture the sample policy is compared against after the resolver is deleted.

## Context

- Root brief: requirement 11, and the notes *The pinned source of the sample*
  and *Capture before deleting*.
- `direct-dispatch-k4`'s note *The parity capture comes first* says what to
  record.

## Done when

- `.grove/source-config.kdl` matches the digest in the root brief.
- A fixture in the repository, where the dispatch crate's tests can read it,
  holds the program and arguments the resolver produces for every kind the
  pinned file routes, `release-notes` included, under each selection the sample
  will offer: the four arrangements, each alone and with each combination of
  `codex-sol` and `high-effort`. Runtime values are named placeholders.
- The resolver produced the fixture, not a hand transcription, and a line with
  the fixture says how.
- `bash scripts/check.sh` passes.

## Notes

- The resolver keeps a runtime slot symbolic until expansion, so its inspection
  already reports placeholders. A local selection replaces the personal one,
  which is one way to reach every selection.
- No test may read `.grove/`, which the finish cycle deletes. The fixture is the
  record that stays, and the pinned file is not copied beside it.
- Nothing reads the fixture yet. `sample-policy-k10` is its consumer.
