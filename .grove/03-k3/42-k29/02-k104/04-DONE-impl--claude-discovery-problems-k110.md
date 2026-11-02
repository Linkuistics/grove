# claude-discovery-problems-k110

## Goal

Observe the discovery problems that Ravel assigns to the agent, in Claude
Code, on the directory route on 2.0.

## Context

The "Agent discovery and native installations" section of Ravel's
`docs/specs/skill-discovery.md`. The Source and skill failures row of Grove's
delivery specification.

## Done when

Each case below is a separate launch with its own record.

- Ambiguity: two different matching skill files in two sources, with no
  explicit reference, give a report before dependent work. Source order
  chooses no winner.
- Missing guidance: a name that no source and no native root holds gives a
  report before dependent work.
- Stale native copy: an older native copy of a craft skill sits in the sandbox
  native root beside the directory copy. The record states which copy the
  agent read and the native startup text. The agent does not load both only
  because both exist.
- User-invoked skill: activity that matches the description of a skill with
  `disable-model-invocation: true` leaves it unread. The same skill is read
  when the caller names it.
- Missing source: a source checkout moved away refuses as
  `skill_source_missing` before any harness launch. This check spends no
  model call.

## Notes

`authoring-conventions` sets `disable-model-invocation: true` in this
checkout. Check that before you use it.

## Decisions (running log)

- Six cases, one launch each: `problem-ambiguity`, `problem-missing`,
  `problem-stale-native`, `problem-user-invoked`, `problem-user-invoked-named`
  and `problem-source-missing`. The user-invoked bullet asks for two
  observations, so it has two cases. Each lifecycle case is a Grove `impl` leaf
  on directory 2.0, with the mandatory files of `discovery`.
- Ambiguity: a third source, `scripts/harness-trials/sources/rival-records`,
  is a plugin directory of the same checkout. Its `decision-records` has the
  same name and a different convention: numbered files under
  `docs/decisions/`. The task asks for a decision record by activity alone and
  names no directory, so the written file shows which copy the agent followed.
- Missing guidance: the task names `changelog-conventions`. No source, no
  native root and no synced claude.ai skill holds that name.
- Stale native copy: `decision-records` at `3699e04a1ed`, before the current
  revision, in the sandbox `.claude/skills`. Its description equals the
  current one. The task asks for the record by activity alone, under
  `docs/adr/`.
- User-invoked: `authoring-conventions`, which sets
  `disable-model-invocation: true` at the checkout commit. The activity is a
  new `SKILL.md`. Its description itself says "user-invoked, run it by hand",
  so an unread result may come from that text as well as from obligation 7.
  The record states this limit.
- Missing source: the runner makes a plain Git clone of the prepared checkout
  at the same commit in the trial directory. The settings name that clone for
  `linkuistics`. The runner moves it away before the driver, and restores it
  after. A direct `ravel skills resolve --json` runs before and after the
  restoration, and the restored one must resolve. No harness launch, no
  session file and an unchanged tree are required.
- The runner gains three things. `native[].commit` copies a skill directory
  from an older commit of the checkout. `moveAway` with `expectRefusal`
  records a refusal case. Each snapshot read names its source.
- `problem-ambiguity` did not measure its row. Grove's `ADR-FORMAT.md` names
  `linkuistics:decision-records`, and the agent read that file before the
  record. It matched the namespace to the source id `linkuistics`, checked
  that source's `plugin.json` name, disclosed the conflict and the resolution,
  and wrote under `docs/adr/`. Ravel's rule lets an explicit reference from the
  pack choose. The record keeps this launch as an observation of a
  pack-referenced choice, not as a result of the row.
- A new case `problem-ambiguity-unreferenced` measures the row. Its task is
  the `stamp.sh` task of `discovery`. A third source,
  `scripts/harness-trials/sources/rival-style`, holds a different
  `coding-style-bash`. No Grove skill and no mandatory file names any
  `coding-style` skill. A grep of `plugins/grove/skills` for each craft skill
  name found `decision-records`, its positive control. It found no
  `coding-style` name. The rival asks for `#!/bin/bash`, the long `set -o`
  options and a `main` function, so the script shows which copy governed it.
  This is a new case, not a retry: the first launch stays in the record.
- Correction: `problem-stale-native` did read one copy. Call 11 prints
  Grove's `ADR-FORMAT.md` first, so the classifier named the call
  `supporting`. The same command then prints the directory copy of
  `decision-records/SKILL.md`. No call reads the native copy or uses the Skill
  tool. A first reading of the table took the class for the whole call and
  missed this. The pack's `ADR-FORMAT.md` names
  `linkuistics:decision-records`, so this choice also had a pack reference.
- A new case `problem-stale-native-cli` measures the choice without a pack
  reference. It was cut before the correction above, and it stays as a second
  observation. Its native copy is
  `cli-tool-design` at `9a35d526f33`, which has no `paths` metadata, so
  Claude Code lists it at startup. No Grove skill names `cli-tool-design`. The
  task asks for a CLI design document only, so no other craft skill matches.
  An older `coding-style-bash` was rejected: every revision carries `paths`,
  so Claude Code could withhold the native copy until a `.sh` file is touched.
- The two user-invoked cases pass. In the activity case, call 16 prints lines 1
  to 8 of `authoring-conventions`, which are its front matter. The agent
  cites the metadata as its reason not to load the skill.
- Results: seven passes and one case not measured. `problem-ambiguity` is the
  case not measured, because the pack's reference chose. The two reports kept
  the leaf live: Ravel refused as `session_result_missing`, and Grove stopped
  the loop.
- For `codex-discovery-problems-k118`: a case about `decision-records` meets
  the reference in Grove's `ADR-FORMAT.md`. Use the cases
  `problem-ambiguity-unreferenced` and `problem-stale-native-cli` for the
  rows without a reference.
- No in-session reviewer. The results are observations of transcripts. Each
  new runner path has a fake-harness check that fails against a broken runner.
