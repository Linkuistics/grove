# trial-harness-adapter-k123

## Goal

Give the trial runner one harness seam. Each harness difference goes behind
one adapter for each harness. The case, source and record layers then do not
test the harness name.

## Context

The human raised this concern during codex-continuation-trial-k119. They
expect the code to show the layers: configuration, skill provision and harness
execution, each with a small surface and few interactions.

Grove's production crates already keep the layers apart. No source file under
`crates/*/src` names `RAVEL_SKILL_DIRECTORIES`, `descriptorPath` or `skills
resolve`. Only the tests `directory_route.rs` and `ravel_asset_cases.rs` name
them.

`scripts/harness_trial.py` does not keep the layers apart. It is one module
of about 3,100 lines. At Grove `e13cc55b7ab5` plus k119, 17 places test the
harness name. They lie in the route, the refusals, the link check, the stream
and session-file reads, the resume and the run itself. k119 added several of
them.

k120 adds compaction for Codex, and k121 adds delegation. Each would add more
of these tests unless the adapter exists first. This leaf therefore runs
before them.

## Done when

- One adapter for each harness owns each harness difference. These are the
  differences:
  - the refusals of the inherited environment and the sign-in store
  - the pre-launch probes and the harness's own links
  - the arguments of a launch and of a resume
  - the session-file lookup, the stream fields and the startup
- Outside the adapters, no code tests the harness name. Give a sweep with a
  positive control and a control that is seen to fail.
- The case, the source clone, the settings and the record use only the
  adapter's interface.
- `task trial:controls` passes unchanged, with each broken-runner control
  pointed at the adapter function that it breaks.
- `task trial:reextract` of one earlier run of each harness gives the same
  events as before.
- No real trial runs. The refactor changes no measurement.

## Notes

Keep the Claude Code and Codex behaviour byte-identical in the records. A
change in a record is a defect of the refactor, not an improvement.

## Decisions (running log)

- One adapter instance for each harness, `CLAUDE` and `CODEX`, of the classes
  `ClaudeAdapter` and `CodexAdapter`. The registry `ADAPTERS` maps a name to
  its adapter. Only the registry and the adapters name a harness. The case,
  the source clone, the settings and the record call the adapter's methods.
- The human added during this session: configuration and skill provision can
  be coupled. The configuration can change the skill provision for each
  invocation, and the provision mechanism stays a separate component. The
  runner therefore keeps skill provision as its own layer: the Ravel
  settings, the native copies and the planted files. The case and its route
  decide them for each invocation, and a continuation changes the settings
  between two invocations. The adapter gives the provision layer only the
  harness's facts, such as its native skill root and its settings file.
- A broken-runner control of a harness difference patches the adapter
  instance, such as `CODEX.resume_args`. `broken` resolves a dotted name. No
  check changes. A test that read a harness fact by a module name, such as
  `sign_in_store("codex")`, reads it from the adapter.
- The classifier asks every adapter for its roots, so a Claude Code run that
  reads `~/.codex` keeps the class `native`. The classifier's residue stays
  the Codex residue only, and `skill_files` also counts the claude.ai synced
  skills as residue. That difference is in the earlier records, so the
  refactor keeps it.
- The live-authority patterns `CONTROL_PATTERN` and `HARNESS_PATTERN` stay
  module constants. They guard against any live harness, whichever harness
  the trial runs.
- The Claude Code delegation link moves into `ClaudeAdapter`. The compaction
  readers stay format readers of Claude Code's session file. They find the
  session file through the adapter, and k120 puts the Codex form behind it.
- Evidence of identical records: the fake-harness records of 18 runs, and
  the `reextract` output of 14 earlier real runs of both harnesses. Each
  comes from the baseline and from the refactor. The comparison normalises
  commit hashes, times and elapsed seconds. Two baseline passes agree on
  every file.
- Results: both comparisons agree on every file, 54 fake-harness files and
  28 `reextract` files. A mutated copy of the refactor changes one Codex
  record fact and one Claude Code row. It differs in 25 and 8 files, so the
  comparison can fail. The `reextract` copies link to the original `home`,
  `repo` and `records`, so the paths match. No original record changed.
- The sweep enumerates every harness-name constant and every condition on a
  harness or adapter identifier, by the syntax tree. On the baseline it
  finds 70 candidates outside an adapter. On the refactor, five candidates
  remain outside, and none tests a harness name. One is the live-authority
  pattern. Four read an adapter's answer in an `or` or a comparison. Three
  injected tests, by name, by adapter identity and by case field, each show
  as outside.
- `task trial:controls` passes all 50 tests. The feeder's `harness`
  argument is now `program_argv`, because the name `command` collided with
  the subcommand.
