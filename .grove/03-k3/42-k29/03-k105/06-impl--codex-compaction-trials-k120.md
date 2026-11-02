# codex-compaction-trials-k120

## Goal

Measure the two compaction cases in Codex as separate trials. The first loses
the path, and the second loses the whole preamble.

## Context

The leaf claude-compaction-trials-k112 states the cases, the spec sections and the notes. Read it and
its record before this leaf runs.

## Done when

- Each Done when item of claude-compaction-trials-k112 holds in Codex, as a separate launch with its own
  record.
- The record names the compaction mechanism of Codex and shows the
  model-visible input after compaction. If no mechanism can run, each trial
  is an unmeasured gap.
- The record compares each Codex result with the Claude Code result of the
  same case.
