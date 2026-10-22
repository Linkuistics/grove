# lifecycle-job-k23

**Integrates:** lifecycle-job-k22


## Goal

Triage the findings of `lifecycle-job-k22` against the implementation of
`lifecycle-job-k9`, apply the valid findings within this increment, and verify
the resulting implementation and books.



## Context

- Read the committed review `lifecycle-job-k22`, its source coordinates and
  evidence limits, and the producer's three child logs.
- The root brief and `lifecycle-job-k9` state the contract. The queued leaf
  `interrupted-errors-k10` already owns its declared follow-up; reconcile that
  ownership during triage rather than duplicating the work.
- The project Taskfile supplies the reusable checks and installed-Ravel tasks.

## Done when

- Every finding of `lifecycle-job-k22` has an explicit, evidence-based
  disposition in this leaf's running log.
- Accepted changes and their book coverage are implemented and verified with
  the checks appropriate to the changes. Any work retained for an existing or
  newly cut leaf has a clear owner and remains live.

## Notes

The review records findings, not implementation instructions that must be
accepted. Re-read the contract and source before deciding each disposition.

## Decisions (running log)

- **Source of the findings.** The findings come from the committed review
  `lifecycle-job-k22`. Each disposition below rests on the source at `39e22ad7`
  and on a command run in this session.
- **F1 (P1): an error loses the caught signal. Real, retained.** The source
  shows the three loss paths. `cli.rs` uses `?` on the lifecycle call, so
  `reraise` runs only for `Ok(Interrupted)`. `complete_post_reap_epoch_handoff`
  drops `JobEnd::interrupted` in its `(Ok(_), Err(_))` arm, and it keeps the launch
  error only as text in the both-fail arm. The contract at
  `docs/specs/harness-selection-and-execution.md` requires the signal to survive.
  `interrupted-errors-k10` already owns this work. This leaf added the loss paths
  and the places to change to its Context. It also replaced the wrong
  review that its body named. No second leaf exists, so the work has one owner.
- **F2 (P2): the lease fixtures can reach the developer's terminal. Real, fixed.**
  Seven tests call `launch_foreground`. Six re-execute through
  `fork_sensitive_driver_lease_test_body_runs_here`, whose child kept the
  controlling terminal. The seventh, `paired_witness_failure_preserves_real_launch_and_admission`,
  skipped the helper. The helper now runs the child with `setsid`, the seventh
  test uses the helper, and `launch_foreground` asserts that `/dev/tty` cannot be
  opened. Evidence: the 71 lease tests pass under `script`, which supplies a
  terminal. With `setsid` disabled, eight tests fail on the assertion, so the
  control came back dirty before I credited the clean run.
- **F3 (P2): four prose claims about removed rules. Real, fixed.** Chapter 19
  now says that the version probe and the terminal reset call `ravel_job::scrub`.
  It also says that a teardown outranks a returned continuation and that a returned
  force-stop outranks a teardown. Chapter 20 names `GROVE_LAUNCH_DIR` where it named
  `CHANNEL_VAR`, which no source defines. The row for that name no longer claims
  "nine tests", because I could not measure that count. The coverage sentence in
  chapter 20 no longer credits chapter 19 with the escalation. A search found one
  more claim of the same kind. Chapter 18 said teardown finishes "regardless of
  ... group" and that continuation needs "a gone group". Both belong to the
  removed rule 4, so I fixed them.
- **F4 (P3): counts, exclusivity claims and coordinates. Real, fixed.** The
  dependency count was seven because it counted `keyed-launch` and `serde_json`.
  Five ordinary and one dev dependency remain, so the book now says six. The `lib.rs`
  figures are 420 lines, 49 lines of documentation and 371 after them. The book also
  said 418 and 327, and 327 did not add up even before the change. The import
  exclusivity claim of chapter 16 was wider than the review said. Only `OsStrExt`
  and `OsStringExt` occur nowhere else. The chapter now names the other files.
  The step coordinates are 148, 150–161, 164, 165, 178 and 180. The revalidation
  strings are at 121 and 144, and `transition_to_current` is at 125. The book now
  follows the source on each, and on the three `label()` calls.
  The review proposed 120 for the revalidation call. The context strings that the
  chapter cites are at 121 and 144, so I used those. The manifest comment still names
  `keyed-launch` once, and the chapter now says so.
- **F5 (P3): the result reader byte bound. Noise, no change.** `grove-force-stop` is
  16 bytes (`printf 'grove-force-stop' | wc -c`), and `grove-continue` is 14. The
  bound at `session_result.rs:84` is 16 + 1 = 17, and the book says seventeen. The
  review's figures of fifteen and sixteen are wrong. The book states the right value.
- **Review notes without a finding.** The review reports that the handover, the
  latch, the post-reap order and the lease notices hold. This leaf did not
  re-verify those claims, and it changed none of their code. The review also notes
  that no case injects a failed wait. That gap is a missing test, and no contract
  fails. I leave it without a leaf.
- **Book re-cut.** F2 added 27 lines to `driver_lease.rs`, which now has 2,101
  lines. A script mapped each old line to its new place from the diff.
  It moved 25 fragments of chapter 17 and regenerated their quoted text from the
  source. It also updated the manifest, the source index and the totals (15,101 lines, and 3,782 for
  chapter 17). The script is not in the repository. `book-check` accepts all six books.
- **Prose gate.** I ran the checker `ste-lint.py` on the new paragraphs and on the
  edited task file, and none reports a hard finding. I did not convert the
  untouched paragraphs that the edited chapters already contained.
- **Result.** `bash scripts/check.sh` passes all 20 checks. The `ravel:` tasks were
  not rerun. The only code change is in unit-test code that spawns `/bin/sh`.
