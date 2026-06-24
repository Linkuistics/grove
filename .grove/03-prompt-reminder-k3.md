# prompt-reminder-k3

**Kind:** work

## Goal

Add one terse, every-session reminder to `content/prompts/continue.md` that
nudges the LLM to externalize surfaced work as a new leaf rather than absorbing
it. The failure mode happens *mid-session*, and `continue.md` is the one prompt
read on every loop iteration — so a short pointer here is the highest-reliability
in-context nudge.

## Context

- **Depends on k2:** use the canonical phrasing of the rule settled in
  `SKILL.md` (the Decompose step) so the prompt and the skill agree word-for-word
  on the trigger language. Do k2 first.
- **Keep it small.** Memory/feedback note "Launcher prompts stay small":
  `content/prompts/*.md` refer to files/skill rather than inlining content. So
  this is **one line** (a reminder + pointer to the skill's Decompose step), not
  a restatement of the whole rule.
- Current `continue.md` is ~2 sentences (Bootstrap → commit-by-handle → complete).
  The new line should slot in naturally without bloating it.

## Done when

- `continue.md` has a single terse line reminding the session to externalize
  surfaced work (new concern → leaf-add/insert; item bigger → leaf-decompose)
  instead of absorbing it, pointing at the skill's Decompose step for detail.
- The prompt stays short (no inlined rule text, no template substitution of large
  content).
- Wording matches the canonical trigger phrasing from k2.

## Notes

`content/` is canonical; building/releasing is out of scope.
