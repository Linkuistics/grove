# codex-discovery-trial-k116

## Goal

Observe implicit craft discovery and the read order in Codex on the directory
route on 2.0, unconfined and confined.

## Context

The leaf claude-discovery-trial-k108 states the cases, the spec sections and the notes. Read it and
its record before this leaf runs.

## Done when

- Each Done when item of claude-discovery-trial-k108 holds in Codex, as a separate launch with its own
  record.
- The confined run states the Codex sandbox mode. If Codex cannot run under
  Ravel confinement, the record states the refusal as a gap.
- The record compares each Codex result with the Claude Code result of the
  same case.

## Decisions (running log)

- The two Codex runs reuse the cases `discovery` and `discovery-confined`
  unchanged, through `task trial:codex`. Each Codex result therefore compares
  with the Claude Code result of the same case and task.
- The trial tree is prepared fresh at Grove `ec2f1973f1d0`. The 2.0 prefix is
  a copy of an earlier scratch prefix, and its `bin/ravel` digest equals
  `8e10e311…` of `builds.json`. The bookmark `skill-directory-delivery` is
  still at `1ca4aecbeae1`, so the build record does not change.
- `discovery-20261103T012450Z` in Codex passes each observation of k108. It
  also read `machine-environment`, whose broad description matches any
  development command. The record states that as a comparison with Claude
  Code, not as a failure, because the craft read of `coding-style-bash` still
  preceded the action.
- `discovery-confined-20261103T012720Z` in Codex is a gap. Codex exited
  before its session with two `No such file or directory` errors. The
  confined environment held no `CODEX_HOME`, so Codex used `$HOME/.codex` of
  the sandbox home, where the sandbox denies writes. As the brief directs, no
  unconfined retry and no in-sandbox workaround followed.
- No runner change. The two cases ran unchanged, and the record classifies the
  description inspection of call 17 in prose.
- No in-session reviewer. The evidence is the transcript itself, and no code
  changed.
- `mise exec bun@1.4.3 -- bash scripts/check.sh` passed all 21 checks with
  the two new record sections.
