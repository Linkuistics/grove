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
