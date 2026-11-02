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
- The runner gains three things: `native[].commit` copies a skill directory
  from an older commit of the checkout, `moveAway` with `expectRefusal`
  records a refusal case, and each snapshot read names its source.
