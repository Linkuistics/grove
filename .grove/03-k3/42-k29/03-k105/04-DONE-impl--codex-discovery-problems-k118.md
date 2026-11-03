# codex-discovery-problems-k118

## Goal

Observe the discovery problems that Ravel assigns to the agent, in Codex, on
the directory route on 2.0.

## Context

The leaf claude-discovery-problems-k110 states the cases, the spec sections and the notes. Read it and
its record before this leaf runs.

## Done when

- Each Done when item of claude-discovery-problems-k110 holds in Codex, as a separate launch with its own
  record.
- The stale native copy sits in the sandbox `.codex` skill root. The record
  states the native startup text that lists it.
- The record compares each Codex result with the Claude Code result of the
  same case.

## Decisions (running log)

- Six cases run in Codex, each once and unchanged, with `--harness codex`:
  `problem-source-missing`, `problem-ambiguity-unreferenced`,
  `problem-missing`, `problem-stale-native-cli`, `problem-user-invoked` and
  `problem-user-invoked-named`. They cover each Done when item of
  claude-discovery-problems-k110. The two cases with a pack reference,
  `problem-ambiguity` and `problem-stale-native`, do not run, as the k110 log
  directs. Grove's `ADR-FORMAT.md` names `linkuistics:decision-records`, so
  neither case can measure its row.
- The stale native copy lies in the sandbox `.codex/skills/cli-tool-design`.
  The runner's `native_root` already selects `.codex/skills` for Codex.
- In Codex, `disable-model-invocation: true` is Claude Code metadata. Only the
  preamble rule of Ravel's discovery specification can keep
  `authoring-conventions` unread.
- The trial tree is prepared fresh at Grove `8bb243d733b4`. The 2.0 prefix is
  a copy of the k117 prefix, and its `bin/ravel` digest equals `8e10e311…`.
  The bookmark `skill-directory-delivery` is still at `1ca4aecbeae1`.
- All six cases pass their rows, and no row is a gap. The record is
  "Discovery problems in Codex" in `docs/common-loading-acceptance-record.md`.
- The stale native copy appears in the Codex skill listing under the root
  `r0`, `$TRIAL/home/.codex/skills`, as `r0/cli-tool-design/SKILL.md`. The
  prelaunch rendering of `codex debug prompt-input` lists it too. No call
  names that path.
- `problem-user-invoked` passes with no stated reason. The agent chose the
  bundled `skill-creator` before it printed the metadata line, so the record
  cannot attribute the unread result to the metadata rule.
- `problem-user-invoked-named` read the skill, then force-stopped before the
  write. `authoring-conventions` requires `writing-skills`, which no source or
  native root holds. The row asks only for the read, so it passes. The record
  gives the difference to acceptance-handoff-k106 as an observation.
- Codex ends a report differently from Claude Code: force-stop in two cases,
  and no ending command in the ambiguity case. Each ending left the leaf live.
- The `python3` heredoc reads of Codex have the class `command`, because the
  classifier takes paths from the argument vector. The record names those
  reads by hand. No runner change: the result tables need no classifier fix.
- The record had three hard findings of the checker in quotations of earlier
  sections. The record convention lets a text row shorten, so each row now
  ends before the flagged clause.
- No in-session reviewer. The results are observations of transcripts, and no
  code changed.
