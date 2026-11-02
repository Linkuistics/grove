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
