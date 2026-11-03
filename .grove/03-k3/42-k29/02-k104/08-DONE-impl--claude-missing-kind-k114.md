# claude-missing-kind-k114

## Goal

Observe the native-only missing-kind case in Claude Code. Separately, observe
the unreadable confined case.

## Context

The Source and skill failures and Confined standalone rows of Grove's delivery
specification. The native-only rows of its Supported routes section.

## Done when

- Missing kind: the case runs on the native route on 2.0, with delivery
  disabled. The sandbox native roots hold the spine and lack the leaf's kind
  skill.
- The agent reports the missing kind skill. It runs the exact force-stop
  command of its launch prompt.
- After the run, the leaf is still live. No teardown is recorded, no output
  is published and the driver does not launch again.
- Unreadable confined: a confined `grove run` with injection disabled has
  native trees outside every grant. The agent reports the unreadable guidance
  and force-stops, and Grove publishes no output.
- The two cases have separate records. Neither credits the native stage's
  fixture.

## Notes

If Claude Code cannot run under Ravel confinement, record the unreadable
confined case as a gap with the refusal.

The force-stop command reaches only the trial's own Ravel channel.

claude-discovery-trial-k108 measured that Claude Code 2.1.316 exits before its
session exists under Ravel's macOS confinement. It cannot open its temporary
directory `/tmp/claude-<uid>`. The `discovery-confined` section of
`docs/common-loading-acceptance-record.md` holds the run. The runner's
`standalone` mode and its probes are reusable for the unreadable confined case.

## Decisions (running log)

- The missing-kind case installs only the native spine, `grove`, in the sandbox
  `.claude/skills`. That is the smallest native root that holds the spine and
  lacks the kind skill. The leaf asks for a trivial file, so a written file
  would show a silent substitute.
- The confined case installs complete native copies of `grove` and
  `grove-impl`. The guidance is then unreadable, not missing, and only the
  sandbox can hide it.
- A probe case now also probes the `SKILL.md` of each native copy. Without
  that probe, the record of a native confined run could not show that the
  sandbox hides the native trees. A fake-harness test with a broken-runner
  control proves it.
- Run `missing-kind-20261103T001740Z` passed: one report, then the exact
  force-stop command, with the leaf live and one launch.
- Run `missing-kind-confined-20261103T002036Z` is a gap with the refusal, as
  the leaf's notes direct. Claude Code exited on `/tmp/claude-501` before its
  session existed. No unconfined retry replaced it.
- The executable reads `CLAUDE_CODE_TMPDIR` and otherwise uses `/tmp`, and it
  ignores `TMPDIR`. The record states that as a lead for
  `acceptance-handoff-k106`. This leaf did not test the variable, because the
  authority control rejects every `CLAUDE_CODE_*` name and the leaf's notes
  ask for a gap.
- `bash scripts/check.sh` passed 20 of 21 steps. "Ravel caller assets and
  provenance" refused, because mise now selects Homebrew's Bun 1.4.4 and the
  typecheck pins 1.4.3. That is drift in the owner's machine configuration, and
  this leaf changed no TypeScript asset. The same step passed with the
  installed 1.4.3 first on `PATH` and `RAVEL=/opt/homebrew/bin/ravel`. The
  owner's configuration stays unchanged.
