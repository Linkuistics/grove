# claude-delegate-trial-k113

## Goal

Observe one real Claude Code delegate. Separate its reads, its adherence and
its inherited ending authority.

## Context

Item 5 of "The preamble" and the "Read access and delegation" section of
Ravel's `docs/specs/skill-discovery.md`. The Delegate authority row of its
acceptance table.

## Done when

- A parent on the directory route on 2.0 delegates part of the task to a real
  delegate.
- The record states what the parent gave the delegate: the descriptor path,
  the obligations and no ending instructions.
- The delegate reads the mandatory guidance first. It then reads the retained
  resources that its part needs.
- The record states the delegate's inherited ending context, and whether it
  obeyed the instruction not to end.
- A separate disposable session measures whether the delegate can send an
  early result. A first-writer check follows.
- The record names the delegate mechanism and the adapter. It names a
  scrubbing adapter as an unmeasured gap when none exists.

## Notes

The delegate mechanism of Claude Code is its Agent tool. An in-process delegate
shares the process environment.

The early-result attempt is a measurement of authority, not of adherence.
Prompt it only in a disposable trial session, never in this live session.
