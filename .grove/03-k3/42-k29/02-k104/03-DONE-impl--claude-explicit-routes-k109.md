# claude-explicit-routes-k109

## Goal

Observe each explicit request row of Ravel's agent discovery rules in Claude
Code, on the directory route on 2.0.

## Context

The request table in the "Agent discovery and native installations" section of
Ravel's `docs/specs/skill-discovery.md`. The Directory bootstrap and Explicit
loading mechanism rows of Grove's delivery specification.

## Done when

Each case below is a separate launch with its own record. Each record compares
the observed route with the row of the spec.

- A bare name request, with native copies absent, loads the skill from the
  supplied directories.
- A plugin-qualified name request, such as `linkuistics:decision-records`,
  with native copies absent, loads from the supplied directories. It does not
  select a native loader.
- A native loader request with a native match uses the loader first.
- A native loader request with no native match says so. It then loads the
  same guidance from the supplied directories, with disclosure.
- A native loader error other than "no match" gives a report and no bypass.
- An express native-only restriction with no native copy gives a report
  before dependent work and no fallback.
- A concrete path loads that file. A failing concrete path gives a report
  before dependent work and no other location.

## Notes

The native loader of Claude Code is its Skill tool. Install a native copy only
inside the sandbox home, and record it.

Establish how to cause a loader error other than "no match" before the case.
If no such error can be caused, record that row as an unmeasured gap.

## Decisions (running log)

- Eight cases, one launch each: `explicit-bare`, `explicit-plugin`,
  `explicit-native-match`, `explicit-native-absent`, `explicit-native-error`,
  `explicit-native-only`, `explicit-path` and `explicit-path-missing`. Each is
  a Grove `impl` leaf on directory 2.0, with the mandatory files of `discovery`.
- Every case requests `decision-records` and asks for one decision record. That
  skill has no `paths` metadata. A native copy of `coding-style-bash` has
  `paths`, so Claude Code could withhold it until a `.sh` file is touched, and
  a "native match" case would then measure "no match".
- The non-"no match" loader error is Claude Code's deny rule. The sandbox
  `.claude/settings.json` denies `Skill(decision-records)`, and a native copy
  exists. Claude Code 2.1.316 then refuses the Skill call as "Skill execution
  blocked by permission rules", error code 11, in its input validation. The
  binary's strings show that this check does not depend on the permission mode.
  The `disable-model-invocation` refusal, code 4, was rejected: its message
  itself tells the agent not to replicate the skill by other means, so a
  "no bypass" would credit Claude Code's text, not Ravel's preamble.
- The runner gains three case fields: `native` copies a skill directory of the
  checkout into the sandbox `.claude/skills`, `claudeSettings` writes the
  sandbox `.claude/settings.json`, and `plant` copies a checkout file under
  the trial's `guidance/`. A tree file's `{{TRIAL}}` becomes the trial
  directory. Native copies must equal the declared ones, or the run fails.
- The extraction adds the agent's text blocks as `text` events, and each Skill
  call keeps its result text. A disclosure and a report are text, so order
  against actions needs them.
- The concrete path is `{{TRIAL}}/guidance/decision-records/SKILL.md`, outside
  the repository and outside every snapshot. Its missing form keeps the skill's
  directory name, so a snapshot copy is a tempting substitute.
- The 2.0 prefix is a byte copy of the k108 prefix, whose `bin/ravel` digest
  `8e10e311…` the preparation checked again. The Ravel workspace stayed
  untouched. The preparation built Grove at `b63ca828d3ef`, with the same
  executable digests as k107 and k108.
- Each case ran once, with no retry. Six rows pass. `explicit-native-error`
  and `explicit-native-only` are gaps: no bypass and no fallback, but the agent
  ran the force-stop command with no report text. Obligation 4 of Ravel's
  preamble says only "Another native error is a report", which may cause the
  first gap. Obligation 6 does ask for a report, so the second gap is an
  adherence failure. `acceptance-handoff-k106` resolves both.
- `explicit-path-missing` reported before dependent work and used no other
  location, then ran no ending command. The record states that ending as
  outside the row, for k106 to decide.
- Several calls failed because zsh reads `=====` as an `=command` expansion.
  The configured mandatory order held in each case. The runner stays
  unchanged for this: it records what the agent ran.
- No in-session reviewer. The results are observations of transcripts, and
  each new runner path has a broken-runner control.
