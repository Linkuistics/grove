# codex-explicit-routes-k117

## Goal

Observe each explicit request row of Ravel's agent discovery rules in Codex,
on the directory route on 2.0.

## Context

The leaf claude-explicit-routes-k109 states the cases, the spec sections and the notes. Read it and
its record before this leaf runs.

## Done when

- Each Done when item of claude-explicit-routes-k109 holds in Codex, as a separate launch with its own
  record.
- The record names the concrete native loading mechanism of Codex, with
  evidence. Where Codex has no native loader that differs from a file read,
  the loader rows do not apply, and the record says why.
- The record compares each Codex result with the Claude Code result of the
  same case.

## Decisions (running log)

- Codex 0.166.0 has no native skill loader that differs from a file read. Its
  model input lists the skills of its native roots by name, description and
  `SKILL.md` path. Its skill instructions say that "filesystem skills should
  be read from the filesystem". Its nested tools in `codex debug
  prompt-input` hold no skill tool. A disposable probe session in a scratch
  sandbox home printed `ALL_TOOLS`. The tool output in its session file names
  125 enabled tools, and no name contains "skill". The executable holds
  `skills.list` and `skills.read`, but its instructions name them only for
  orchestrator skills, and the probe did not enable them. Codex's
  [skills page](https://learn.chatgpt.com/docs/build-skills) says that Codex
  "reads the full SKILL.md" when it selects a skill. Its explicit invocation is
  a `$name` mention or `/skills` in the user's prompt. A task file that the
  agent reads cannot make that mention.
- The four loader rows therefore do not apply in Codex:
  `explicit-native-match`, `explicit-native-absent`, `explicit-native-error`
  and `explicit-native-only`. Each request names Claude Code's Skill tool. The
  record states each row with this evidence and with the Claude Code result.
  No launch runs for them.
- Four launches run in Codex, with the cases unchanged: `explicit-bare`,
  `explicit-plugin`, `explicit-path` and `explicit-path-missing`.
- `explicit-native-only` also runs once in Codex, unchanged. Its express
  restriction names a mechanism that Codex lacks, so the restriction row
  still applies: the agent must report the limit and not fall back. The
  record credits that run to the restriction row only, never to a loader row.
- This session prepared the trial tree fresh at Grove `7d9196db96e9`, with the same
  executable digests as k116. The 2.0 prefix is a copy of the k116 prefix,
  and its `bin/ravel` digest equals `8e10e311…`. The bookmark
  `skill-directory-delivery` is still at `1ca4aecbeae1`.
- All five runs pass their rows. `explicit-native-only` reported the missing
  Skill tool and then force-stopped. `explicit-path-missing` reported the
  failed read and then force-stopped, where Claude Code ran no ending command.
  The record leaves that ending difference to `acceptance-handoff-k106`.
- Two runs printed the `description` line of every skill before they read the
  task. The record names this as a description inspection, not a load,
  because neither run printed a line of a skill body.
- No runner change and no in-session reviewer. The results are transcript
  observations, and no code changed.
