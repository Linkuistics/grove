# harness-neutral-prompts-k9

**Kind:** work

## Goal

Review the prompts this repo ships (`content/prompts/*.md`, embedded and
extracted to each harness's global skill dir on `grove do` — see
`project_editing-grove-methodology-content` memory) and the
`~/Development/skills` repo for language that assumes the driving harness is
Claude Code or the model is an Anthropic one, and generalize it — this grove
exists precisely because the trial now drives codex+gpt-5.6-sol and pi+K3
alongside Claude Code (BRIEF.md).

## Context

Raised mid-session by the user while `watcher-test-hardening-k7` was in
flight (2026-07-18); externalized here per grove's decompose rule rather than
absorbed into that leaf. Two surfaces to sweep:

- This repo's `content/prompts/*.md` (`start.md`, `continue.md`, `retire.md`)
  — the launcher prompts read by all three harnesses (`src/launch.rs`
  `load_prompt`). Also worth a pass over any other shipped `content/` prose
  (skill docs, `TASK-FORMAT.md`, etc.) for the same assumption.
- `~/Development/skills` (the `linkuistics`/`grove` skill set symlinked into
  `~/.claude/skills/`, `~/.codex/skills/`, and `~/.pi/agent/skills/` — see
  `project_cross-harness-skills-setup` memory). Edit that repo directly from
  here per `feedback_grove_skills_one_system` memory; commit only in
  `~/Development/skills` itself, never the `~/.claude/plugins/marketplaces/`
  mirror.

Likely offenders: "Claude", "Claude Code", references to Bash-tool-call
semantics phrased as if that's the only tool surface, model names hardcoded
as Anthropic ones in examples, anything assuming the harness's own
first-person voice is "Claude".

## Done when

Both surfaces swept; every found instance either generalized (harness-neutral
wording) or, where a reference is genuinely Claude-Code-specific (e.g. a
skill that only makes sense there), left with a clear scoping note rather
than silently misleading a codex/pi session. Findings and fixes land as a
normal commit in each affected repo.

## Notes

Not urgent/blocking — sequenced ahead of `release-k4` (now k-shifted to
`08-release-k4`) so a fixed prompt set ships in the same release as
`driver-side-kill-k2`, not after.
