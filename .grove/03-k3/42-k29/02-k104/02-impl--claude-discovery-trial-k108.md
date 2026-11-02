# claude-discovery-trial-k108

## Goal

Observe implicit craft discovery and the read order in Claude Code on the
directory route on 2.0, unconfined and confined.

## Context

- The "Agent discovery and native installations", "The preamble" and "Actual
  harness trials" sections of Ravel's `docs/specs/skill-discovery.md`.
- The Directory bootstrap, Original mandate and Confined standalone rows of
  Grove's delivery specification.

## Done when

- The case runs a Grove leaf of a named kind with Grove's bare-name mandate.
  Its task names no skill. The activity matches exactly one craft skill, such
  as a Bash script for `coding-style-bash`.
- The transcript shows the listing or the description inspection, then the
  read of the craft skill's snapshot path, before the governed action.
- The mandatory files are read in configured order before the named-kind read
  and before the caller's "first" action.
- One mandatory file has a relative supporting read, and the transcript shows
  that read against the skill's own directory.
- Both sources come from one plain Git checkout of this repository at a
  recorded commit, with no checkout tool.
- `ravel skills resolve` with the same settings runs under a profile that
  denies the network, with no `git` on `PATH`. It gives the descriptor
  identity of the launch.
- A confined `grove run` of the same task reads the required resources outside
  the working directory. Its writes and its protected configuration reads
  fail.
- The record states each result and each limit.

## Notes

The harness needs the network for its model, so the no-network check covers
Ravel's resolution only. The fixture already proves a no-network launch.

If Claude Code cannot run under Ravel confinement, record the refusal as a
gap. Do not retry unconfined to get a pass.

A run that passes only after a prompted retry is a failure of this case.
