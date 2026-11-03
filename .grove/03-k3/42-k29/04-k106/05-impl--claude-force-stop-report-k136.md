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
