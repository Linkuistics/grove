# claude-visible-report-k135

## Goal

Make Claude Code show its report of a native loader error as visible text
before the force-stop command. Do the same for a blocked restriction. Then run
`explicit-native-error` and `explicit-native-only` in Claude Code again.

## Context

- The record section "Loader report repair" of
  `docs/common-loading-acceptance-record.md`, from
  `loader-report-wording-k129`. In both Claude Code runs, the agent wrote an
  accurate report in a thinking block directly before the force-stop call. No
  text block held it. Claude Code then added "[Your previous response had no
  visible output. …]" after the session had ended.
- `missing-kind` in the same record. Claude Code reported in text before a
  force-stop when the mandate named the report and no tool error came first.
- Ravel's `crates/ravel/src/skills/preamble.rs`, obligations 4 and 6, on the
  unpushed bookmark `skill-directory-delivery` in the
  `Ravel.skill-directory-delivery` workspace. Build commit `0fb12d5f308b`.
- Grove's mandate wording for the ending command: "run this exact command as
  your last action — then do nothing else".
- The runner `scripts/harness_trial.py` omits thinking blocks from its event
  table. The record quotes them from `records/0/stream.jsonl`.

## Done when

- The leaf decides where the wording belongs: Ravel's obligations 4 and 6, or
  Grove's ending wording. The task file records the decision and its reason.
- The wording says that the report is a visible reply to the human, not only
  reasoning. It also says that the report comes in its own step, before the
  ending command call.
- If Ravel changes, the bookmark moves, the Ravel tests that pin the preamble
  pass, and the specification agrees. The new build is in
  `scripts/harness-trials/builds.json` and in the record, by commit, build
  identity and `bin/ravel` SHA-256, before any result.
- If Grove changes, its tests pass and the trial preparation uses the
  changed commit.
- `explicit-native-error` and `explicit-native-only` run once each in Claude
  Code. A case passes when a text block states the error or the limit before
  dependent work and before the force-stop command. A thinking block is not a
  report.
- A case that fails again stays a gap, and the leaf asks the human a scope
  question about it.

## Notes

The human chose this repair on 2026-11-03, in `loader-report-wording-k129`,
over accepting the two rows as explicit gaps. The human did not accept a
report in a thinking block as a pass.

Codex passed `explicit-native-only` under the k129 build. If the wording
changes Ravel's preamble, the leaf may run that Codex case again as a control.

A Ravel change moves the bookmark again. `stage-evidence-reconciliation-k125`
reconciles the stage evidence with the newest build.

## Decisions (running log)

- **Owner of the wording.** Ravel's obligations 4 and 6 take the change, and
  Grove's ending wording keeps its text. The report is Ravel's obligation, and
  Ravel serves callers other than Grove, so it names "any ending command".
  Under Grove's unchanged ending wording, `missing-kind` reported in visible
  text. The defect is the channel of a Ravel report, not the ending.
- **Wording.** Obligations 4 and 6 put the report "in a visible reply to the
  human". They add "Reasoning alone is not a report, because the human may not
  see it", which gives the agent the reason. "Give that reply as its own step,
  before dependent work and before any ending command call" sets the order.
  Obligations 5, 8, 9 and 10 keep their text. The specification states the
  same rule, and its history stays in Ravel's design evidence.
- **Ravel output.** Base `main` `24a545236185`, unchanged after `jj git
  fetch`. Build commit `e9cc79993b46`, identity `fd8dfdc2…`, `bin/ravel`
  `d076dc01…`. `task check` passed 732 of 732. The identity script gives
  k129's `45eea220…` for `0fb12d5f`, as its control. Output commit
  `0d60cb531240` is the tip of the unpushed bookmark.
- **Build record.** `builds.json` takes the new build and points its `source`
  at the record section "Visible report repair".
