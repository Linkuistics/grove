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

## Decisions (running log)

**The fixture is `crates/harness-dispatch/tests/fixtures/parity/commands.json`,
with a `README.md` beside it.** That is where the crate's other fixtures live,
and the crate has no walkthrough book for a new file to break.

**`grove config show --json` is the capture.** It reports the resolver's
compiled words, the same ones expansion fills, with slots still symbolic. The
pinned file sat unmodified under a temporary `HOME`, and an ignored `.grove.kdl`
holding only a `select` line reached each selection. The run with no delta and
the delta run of the pinned selection gave identical commands.

**Sixteen entries, each with `arrangement`, `modifiers`, `select` and
`commands`.** A command is `program` and `args`, the names the new contract
uses, and a slot is written `${name}` as the pinned file spells it. `select` is
kept because it is what the resolver was actually given.

**One modifier order is recorded.** `codex-sol` then `high-effort` and the
reverse resolved to identical commands under all four arrangements, so the
sample owes no ordering rule.

**No generator is committed.** It would run a resolver the next leaves delete.
The README carries the recipe instead.

**`high-effort` leaves `release-notes` at `medium`.** It names the `codex` and
`claude` commands only, and the fixture records that. The sample reproduces it.
