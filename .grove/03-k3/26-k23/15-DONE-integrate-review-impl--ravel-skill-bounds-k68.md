# ravel-skill-bounds-k68

**Integrates:** ravel-skill-bounds-k64

## Goal

Triage the findings of `ravel-skill-bounds-k64` against the current Ravel
feature line. Apply the accepted findings and verify the resulting change.

## Context

- Read the review's committed artifact and its source coordinates at
  `52d856978b24`. The review records reconciliation with feature commit
  `0cc7f8c4f336`.
- Work in `/Users/antony/Development/AgenticOrchestration/Ravel.skill-directory-delivery`
  on bookmark `skill-directory-delivery`, following the parent brief's
  cross-repository commit and check contract. Do not push the bookmark.

## Done when

- Each finding has a reasoned disposition, and every accepted finding has
  been integrated with appropriate verification.
- Record the resulting Ravel commit and check evidence here and in the
  Grove task commit. Preserve the findings-only review as the source of the
  handoff.

## Notes

This leaf consumes the review, rather than treating its findings as an
unconditional implementation charter. The integration owns fixes and all
post-fix checks.

## Decisions (running log)

- Base: Ravel workspace `Ravel.skill-directory-delivery`, bookmark
  `skill-directory-delivery` at `0cc7f8c4f336`, with an empty working copy.
  The review reconciled its findings with this tip, so no later drift exists.
  The findings were read from the review commit `9dadfc1502f`. Each cited
  code path is unchanged at this tip.
- Finding 1, the capture-stop fixture: a real evidence defect.
  `stopped_in_capture` sends SIGSTOP and does not wait until the child is
  stopped. It can see a staging that already holds its manifest, or one that
  publication renames next. A failed assertion drops a plain `Child`, which
  leaves a stopped resolver. `wait_with_output` has no bound. Fix: a guard
  owns the child. It waits for the stop through `waitpid` with `WUNTRACED`,
  and only then examines the staging. A stop outside capture kills the child,
  resets the cache and tries again, within a bound of attempts. The guard
  kills and reaps the child on each exit path, and its wait has a bound.
- Finding 2, the book's cancellation paragraph: a real issue in the book. The
  specification permits complete retained objects after a refusal. A signal
  consumed after the descriptor publication still gives
  `skill_resolution_cancelled`. Fix: the book says that Ravel removes its
  private staging and never leaves a partial descriptor. It says that a
  signal after the publication can leave the complete descriptor, as any
  refusal can. No rollback is added.
- No finding is noise or a trade-off, and none needs a redesign.
- Mutation controls of the fixture, each run and then reverted. A first
  attempt that always misses: both cancellation fixtures pass through the
  reset and the retry. Every attempt misses: the fixture fails after five
  attempts, and no resolver process survives. The survivor check is a
  `pgrep` of `target/debug/ravel skills resolve`. Against the original helper,
  with its manifest assertion forced to fail, it found one stopped resolver,
  which was then killed. So the check can fail, and the guard closes the
  leak.
- Documentation: `docs/book/src/06-reference.md` alone changed. The
  specification already states retention after a refusal, and the
  `CHANGELOG.md` entry makes no claim about the descriptor. The book's STE
  check reports 0 problems in the chapter, and it reports a planted problem.
- Verification on macOS, after the last edit, with the two changed files
  digested before and after the check: `task check` exit 0, with format,
  lint, 724 of 724 run tests passed, 48 skipped, and the fake-harness smoke
  test passed. All nine `skills_bounds` tests pass. `task docs:book:build`,
  `task docs:excerpts:test` (24 of 24 mutants killed),
  `task docs:breaks:test`, `task docs:ste:test` and `task book:integrity`
  exit 0. `cargo clippy` for `aarch64-unknown-linux-gnu` on all targets with
  `-D warnings` is clean. No Linux run occurred. The `waitpid` and
  `WUNTRACED` use is POSIX and is the same on Linux.
- Ravel output commit `f1aff8e5562f2d015cc853d4951d7fb2d851a800`, parent
  `0cc7f8c4f336`. The local bookmark `skill-directory-delivery` moved there
  through jj. It is not pushed. The findings-only review
  `ravel-skill-bounds-k64` stays the source of this handoff.
- No in-session reviewer was spent. An executable control covers each path
  of the fixture fix.
