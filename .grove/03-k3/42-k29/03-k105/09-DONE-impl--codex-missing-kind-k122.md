# codex-missing-kind-k122

## Goal

Observe the native-only missing-kind case in Codex. Separately, observe the
unreadable confined case.

## Context

The leaf claude-missing-kind-k114 states the cases, the spec sections and the notes. Read it and
its record before this leaf runs.

## Done when

- Each Done when item of claude-missing-kind-k114 holds in Codex, as a separate launch with its own
  record.
- The native roots of the case are the sandbox `.codex` skill roots. The
  record names each root that Codex reads.
- The record compares each Codex result with the Claude Code result of the
  same case.

## Decisions (running log)

- Both cases run unchanged with `--harness codex`. `CodexAdapter.native_root`
  is `.codex/skills`, so the case's native copies land in the sandbox `.codex`
  skill root, and the probes follow the same root. No runner change.
- Build: Grove commit `a20c09778bae`, prepared into this session's scratch
  directory. The Ravel 2.0 prefix is a byte copy of an earlier trial prefix:
  5 files, identical digests, `bin/ravel` SHA-256 `8e10e311c50c…`. The bookmark
  `skill-directory-delivery` is still at `1ca4aecbeae1`.
- Skill roots of Codex 0.166.0 come from `resolve_skill_roots` in
  `codex-rs/ext/skills/src/host_roots.rs` at `rust-v0.166.0`. For this trial
  they are `$CODEX_HOME/skills`, `$HOME/.agents/skills`, `$CODEX_HOME/skills/.system`,
  the admin root under `/etc/codex`, and `.agents/skills` from the project root
  to the working directory. The project marker is `.git`, which the trial
  repository lacks, so the project root is the working directory. Only the
  first and third roots exist.
- Run `missing-kind-20261103T052902Z` passed, one run with no retry. Codex
  searched the sandbox `.codex/skills`, the repository's `.agents` and `.codex`,
  then every `SKILL.md` under the sandbox home. It reported the missing
  `grove-impl`, ran the exact force-stop command, and Grove stopped with exit 1.
  The leaf stayed live, with no teardown, no change and one launch.
- Run `missing-kind-confined-20261103T053135Z` is a gap with the refusal, as
  in discovery-confined in Codex. Codex exited after 1.2 seconds with "failed
  to initialize in-process app-server client", before any stream event. Ravel
  refused as `session_result_missing`, and Grove published nothing. No
  unconfined retry replaced it.
- The probes show both native copies in `.codex/skills` unreadable inside the
  sandbox and readable unconfined. In Codex the native roots lie inside the
  configuration directory, so the sandbox cannot hide the native guidance
  without also hiding the configuration. The record states that second cause.
  `$HOME/.agents/skills` is the only native user root outside `.codex`, and no
  run measured it.
- Closing codex-trials-k105: each `Done when` item holds in the record. The two
  unrun discovery-problem cases carry their reason. The node's gaps go to the
  acceptance brief for acceptance-handoff-k106.
- `bash scripts/check.sh` passed 20 of 21 steps. "Ravel caller assets and
  provenance" refused, because mise selects Bun 1.4.4 and the typecheck pins
  1.4.3, as k114 recorded. This leaf changed no TypeScript asset. `task
  ravel:review` passed with the installed 1.4.3 first on `PATH` and
  `RAVEL=/opt/homebrew/bin/ravel`. The owner's configuration stays unchanged.
