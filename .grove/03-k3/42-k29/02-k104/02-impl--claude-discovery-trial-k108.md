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

## Decisions (running log)

- Two cases share one task: `discovery` runs it as a Grove `impl` leaf, and
  `discovery-confined` runs it through `grove run`. The task asks for a Bash
  script `stamp.sh` with no arguments, so `coding-style-bash` is the one
  activity match. A script with arguments could also match `cli-tool-design`.
- The sources are `plugins/grove` and `plugins/linkuistics` of one checkout.
  The mandatory order is `using-jujutsu`, then the `grove` spine. That order
  differs from the order the kind skill would suggest, so a matching read
  order shows the configured order. Both files have relative references.
- The runner gains a `standalone` mode. A confined launch can write only in its
  invocation directory, so the wrapper prints its record and the stream to
  Grove's run log in the sandbox home. The events then come from the stream.
- The wrapper probes reads and writes inside the launch, before the harness
  starts. The runner repeats the same probes unconfined. Ravel seals snapshots
  read-only by mode, so a failed snapshot write is no sandbox evidence alone.
- The harness program is outside every grant, so the standalone case grants it
  with `--runtime-read`, as an owner grants a credential.
- The offline resolution runs after the lifecycle launch, with the same HOME,
  under the fixture's network-denying profile and with only the front on PATH.
  A network control and a git control must each be seen to differ.
