# claude-continuation-trial-k111

## Goal

Observe continuation in Claude Code, and the retained reads after a source
change and a settings change.

## Context

Item 4 of "The preamble" and the Lifetime row of Ravel's
`docs/specs/skill-discovery.md`. The Resource lifetime row of Grove's delivery
specification.

## Done when

- A first turn on the directory route on 2.0 reads the mandatory and relevant
  guidance and stops before the work completes.
- Between the turns, a new commit changes a read file in the source checkout.
  The settings change too.
- The continued session keeps the original descriptor. It reads the mandatory
  and relevant guidance again before work continues, and gets the original
  bytes.
- A fresh launch after the change receives a new descriptor with the new
  bytes.
- The record states the continuation mechanism and whether each re-read came
  unprompted.

## Notes

Establish the continuation mechanism before the case. `claude --resume` is
the candidate. A continued session outside Ravel's managed process loses the
variable, so record the environment of each turn.
