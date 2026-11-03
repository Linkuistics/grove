# claude-force-stop-report-k136

## Goal

Make Grove's force-stop instruction require a visible reply that states the
reason, as its own step before the force-stop call. Then run
`explicit-native-error` in Claude Code again.

## Context

- The record section "Visible report repair" of
  `docs/common-loading-acceptance-record.md`, from
  `claude-visible-report-k135`. Its subsection "What the second repair
  shows" states the diagnosis. In three Claude Code runs, the agent went from
  a loader error to the force-stop call, and a thinking block held its report.
  When the agent ended its turn, its final reply held the report.
- Grove's mandate in `crates/grove-loop/src/prompt.rs`. Two sentences name the
  force-stop command: the missing-guidance rule of the load instruction, and
  the signalling contract. The signalling contract asks for the "last action —
  then do nothing else". The module header states the too-late test for
  mandate text.
- `crates/grove-loop/tests/prompt.rs`, which pins the mandate text, and
  `docs/walkthroughs/grove-loop/18-the-core.md`, which quotes it.
- Ravel's obligations 4 and 6 on the unpushed bookmark
  `skill-directory-delivery`, at output commit `0d60cb531240`. They already
  require a visible reply as its own step before any ending command call.
  This leaf does not change Ravel.

## Done when

- The mandate tells the agent to state the reason for a force-stop in a
  visible reply to the human. That reply is its own step, before the
  force-stop call. The
  wording covers both force-stop sentences, or the leaf records why one
  sentence suffices.
- Grove's prompt tests pin the new sentence. They failed against the earlier
  text. The walkthrough quotes the new text. `bash scripts/check.sh`
  passes, or the leaf records each failure that this change did not cause.
- The trial preparation uses the changed Grove commit. The record names the
  new `grove` and `grove-llm` SHA-256 values before any result.
- `explicit-native-error` runs once in Claude Code. The case passes when a
  text block states the loader error before dependent work and before the
  force-stop command. A thinking block is not a report.
- If the case fails again, it stays a gap. The record and the completion
  audit name it as an explicit gap in Claude Code, and the leaf retires
  without a further repair.

## Notes

The human chose this repair on 2026-11-03, in `claude-visible-report-k135`,
over accepting the gap or a structural force-stop reason. The human accepted
the recommendation that a failure of this repair becomes an explicit gap with
no further repair leaf.

`explicit-native-only` passed for the row in k135. A run of it again is
optional, and it is a control only.

The change applies to the mandate of every Grove session. Check that the new
sentence does not ask for a report before a normal `grove-continue` ending.

## Decisions (running log)

- **Both force-stop sentences carry the rule.** The signalling contract
  states it in full: state the reason in a visible reply, as its own step,
  before the call. It also says why. Reasoning alone is not that reply,
  because the human may not see it, and no step follows the force-stop call.
  The missing-guidance rule of the load instruction also names the visible
  reply, before the call. A load can fail before the session reads the
  contract. `explicit-native-error` force-stops for a failed
  `decision-records` load, so only the contract sentence governs that case.
- **The rule passes the too-late test.** It governs the mechanism of the last
  action, and only the contract states that mechanism. A session whose skill
  is missing can read no other source. The `prompt.rs` doc comments state
  this.
- **A normal ending asks for no reply.** The new test
  `a_force_stop_states_its_reason_in_a_visible_reply_before_the_call` pins
  the force-stop sentence whole. It also holds that no other sentence of the
  contract contains "reply". The continue sentence keeps its text.
- **The tests failed against the earlier text.** Before the source change,
  `the_load_instruction_states_the_missing_guidance_force_stop` and the new
  test failed. The other 20 tests of `crates/grove-loop/tests/prompt.rs`
  passed. After the change, all 22 passed.
- **The fake session finds the command by the end of a line.** The fake
  session of `crates/grove/tests/native_skill_delivery.rs` found the
  force-stop command after a line that was exactly `action instead:`. The new
  wrap gives `as your last action instead:`, and the full check failed
  `missing_or_unreadable_kind_guidance_force_stops_with_the_leaf_live`. The
  pattern now matches a line that ends with `last action instead:`, so a
  rewrap does not break it. No contract line ends with `instead:`.
- **The walkthrough follows the source.** Chapter 18 quotes the changed
  blocks, and the book's counts move with the file: 279 lines, 176 comment
  lines, nineteen citations and 15,283 book lines. The new source comment
  cites the acceptance record and names no task key, so the task-key census
  keeps its text. `book-check --final` passes. The chapter was never
  converted to controlled English. This leaf cleared the lint findings in its
  own sentences only, and a conversion stays a separate change.
- **The full check passes.** `bash scripts/check.sh` passed all 21
  principal checks, with 1334 of 1334 nextest tests, and with mise's
  installed Bun 1.4.3 first on `PATH`. Without it, "Ravel caller assets and
  provenance" fails, because the owner's global mise configuration selects
  Bun 1.4.4 and the policy typecheck pins 1.4.3. `loader-report-wording-k129`
  recorded the same failure. This leaf changed no part of that step's subject.
- **The case failed, and it is an explicit gap.** The run
  `explicit-native-error-20261103T084218Z` used Grove commit `1f7b8013d4f6`.
  The record named its `grove` and `grove-llm` SHA-256 values in commit
  `bdb1d81c`, before the run. The prompt file held both new sentences. The
  agent again wrote an accurate report, with a reason, in a thinking block
  directly before the force-stop call. No text block named the loader error.
  As the human decided in k135, the case is an explicit gap in Claude Code,
  with no further repair. The record says so, and the note of
  `acceptance-completion-audit-k127` tells the audit to name it.
- **The ending diagnosis is weaker now.** In the same run, the agent also
  wrote a progress note in a thinking block, with a tool call after it. The
  record states that the channel is not only a property of the ending. The
  force-stop wording stays, because it serves every Grove session.
