# launch-cutover-k25

**Integrates:** launch-cutover-k19

## Goal

Triage `launch-cutover-k19`'s six findings against the source and apply those that
hold, so that `current-state-docs-k20` and `major-release-k21` describe an agreed
cutover. This session owns corrections to what `launch-cutover-k15` produced
(`launch-directory-k16`, `dispatch-ending-k17`, `signal-contract-k18`), not the
documents `current-state-docs-k20` owns.

## Context

- Read the findings from `launch-cutover-k19`'s committed review, not from this
  body. Classify each on evidence: a contract stated unclearly, a real issue, a
  visible trade-off, or noise.
- F1 is the one to start with: a shipped skill in `plugins/linkuistics` and the
  spec behind it still say `grove-llm complete`. The reader in
  `crates/grove-llm/tests/instructed_verbs.rs` is sound but scoped to
  `plugins/grove/skills`; settle whether to widen it to every shipped plugin.
  Sweep by enumerating, then classifying, as `references/execute.md` requires, and
  watch each control fail before crediting a clean read.
- F2 and F4 are test changes. F3 and F5 are a contract question each (the table's
  missing row; the legacy epoch reading). F6 is two sentences, in the skill and the
  prompt.
- A leaf that changes a crate's source updates that crate's walkthrough book in
  the same commit, and logs the change under `## Unreleased` in `CHANGELOG.md`.

## Done when

- Each finding is classified; each real one is fixed with a test, or accepted
  visibly with its reason in this leaf's log.
- No shipped skill instructs a `grove-llm` verb the CLI lacks, across every plugin
  this repository ships.
- `bash scripts/check.sh` passes.

## Notes

- This is a meta-grove: end the session with the installed v22 `grove-llm`,
  never `./target/debug/grove-llm`, as the root brief says.
- `current-state-docs-k20` still owns `docs/USAGE.md`, `docs/ARCHITECTURE.md`,
  the READMEs and `CLAUDE.md`; the review's "For the leaves after this one" lists
  them. Do not take them here.
- `major-release-k21` owns the live-grove cutover. The review notes that a running
  v22 driver whose session meets a v23 `grove-llm` cannot send `complete`; that is
  its question, not this leaf's.

## Decisions (running log)

- I1 — F1 is a real issue: widening the existing instructed-verb reader to every
  `plugins/*/skills` tree fails on the wrapped `complete` instruction in the
  Linkuistics doubt skill. Keep the scanner and broaden its corpus; the skill
  and its spec should defer to the prompt's ending rather than own another one.
- I2 — F2 is a real test gap; F3 is a contract stated unclearly. The driver
  already orders interruption, teardown, a surviving dispatch group, then the
  exit-signal ending. Add precedence coverage and the omitted row, identifying
  dispatch's group separately from the harness's group.
- I3 — F4 is a real test defect: `ending` is the wrong filename and even the
  correct FIFO cannot block the nonblocking reader. Use `ending.json` and a
  separate abandoned directory containing a valid exit-signal document, and
  check that cleanup starts exactly one new launch.
- I4 — F5 is a contract stated unclearly. The cutover retires the old protocol;
  no cross-version viewer/admission compatibility was agreed. Remove active
  epoch fallback to `signal-path-hex` and migrate the viewer fixtures to
  `launch-dir-hex`, with a regression proving a legacy active record refuses.
- I5 — F6 is a contract stated unclearly: without a signal, an interactive
  harness can remain at its prompt. State that the loop waits until that
  harness ends, then stops with the leaf live. A teardown already recorded
  continues to finish after reap unless the driver itself was interrupted.
- Evidence limitation — the graph CLI refuses startup because another
  pre-coordination/unverified generation is active. Source reads and searches
  supply this session's evidence; no graph completeness claim is made.

- I6 — F2's survivor combination is tested at the runner-result seam, rather
  than manufacturing a process that survives SIGKILL. `interpret_launch` is
  the existing ordered branch body extracted unchanged; the actual driver
  calls it after reap/invalidation. The real PTY case pins interruption over
  teardown. Reversing interrupt/teardown failed both tests; moving survivor
  above teardown failed the runner-result test. Both mutations were restored.
- I7 — F5's regression first failed because the legacy record still admitted
  a launch-directory session, then passed after the active fallback was removed.
  The inactive parser continues rejecting either authority-field spelling;
  that is refusal, not legacy admission. The viewer fixtures now name launch
  directories and still exercise their epoch-rotation retry.

- I8 — F4's count control was seen to fail: deliberately signalling from the
  new first harness caused two launches and failed the one-launch assertion.
  Restored the own-exit harness. The reader's existing supported-report test
  independently recognizes the exact exit-signal document used in the abandoned
  fixture. These controls pin stale outcomes not being acted on; they do not
  claim to detect a discarded read with no observable consequence. The cleanup
  source itself contains no content read.
- I9 — F1's widened corpus still yields the pinned ten instructed verbs, all
  exposed by clap; the offending wrapped mention failed before correction.
  The scanner's existing removed-verb, wrapped-line, reopened-span and
  complete-set controls cover the reader rather than relying on a pattern list.

- I10 — The first full check failed on the old wording pin in the existing
  prompt contract test and on Overview's closing navigation (new evidence
  prose was appended after it). Correct the pin to the waiting-then-stopped
  contract and move the prose before navigation. Also reconcile the epoch
  book's current test enumeration and remove its stale prompt-size figure.
  All 1,926 tracked repository inputs captured during that run were unchanged
  at completion; these are verification failures, not a moving-subject result.

- I11 — The second full run exposed one more F5 fixture: TUI browser's
  `runtime_records` still wrote an active `signal-path-hex` record. Its test
  failed before reaching the missing-observation-witness case it owns. Migrate
  that fixture to `launch-dir-hex` too. Enumerating the literal across crates,
  testing helpers and walkthroughs found this fixture, the intentional inactive
  refusal, the legacy-admission regression and their exact book copies; no
  other fixture writer appeared. All pre-run tracked input digests remained
  unchanged during the second run. All six books passed final validation.

## Verification

- Final `task check` (the Taskfile entry point for `bash scripts/check.sh`)
  passed all 12 principal checks: formatting, shellcheck, clippy, plugin
  installation and conformance, release workflow checks, the complete locked
  workspace test suite, and final validation of all six books.
- All 1,926 tracked repository inputs matched their pre-run SHA-256 digests
  after the final check. The evidence log is `/tmp/grove-k25-check-3.log`.
- Regression controls: the expanded skill reader first failed on the retired
  wrapped command; the legacy epoch first admitted and then refused; reversing
  interruption/teardown failed both the deterministic and PTY tests; moving the
  survivor guard above teardown failed the deterministic test; a deliberate
  live exit signal failed the abandoned fixture's one-launch assertion.
- F1–F6 are repaired; none requires a redesign or another producer chain. The
  root brief carries the settled cutover facts to k20 and k21. This root-level
  leaf has no enclosing node to close; documentation and release leaves remain
  live. Verification ran on the macOS host; Linux execution was not exercised.
