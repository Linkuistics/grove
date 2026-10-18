# grove-communication-reword-k15

## Goal

Reword grove's "Human-facing communication" section so that it sits under the
skill's precedence order and does not pull against it. The section asks for a
guided, conversational, explanatory style; the skill rules govern the wording
inside that style.

## Context

- The section is in `plugins/grove/skills/grove/SKILL.md`. `skill-design-k5`
  decided what it must say. The precedence order is: the human's request, then
  meaning, then the skill's rules for words and sentences, then house style
  (this section, the explanatory output style, templates and commit
  conventions).
- Keep what house style owns: what to explain, leading with the outcome,
  explaining before asking for a decision, and where evidence goes. Change only
  what asks for wording that the skill's rules restrict, such as long or
  idiomatic sentences.
- The skill text reaches users through the Claude marketplace, Grove's embedded
  snapshot for Codex and `plugins/install.sh`. Find any other copy of the
  section, such as an embedded or generated one, and update it.
- `plugins/grove/conformance.sh` checks the grove skills.

## Done when

- The section is reconciled with the design, and every copy of it is updated.
- The edited text passes the document checker, apart from justified departures.
- `bash scripts/check.sh` is green, including conformance.

## Notes

- If `skill-design-k5` decided to leave the section unchanged, the human decides
  whether to prune this leaf. Do not prune it yourself.
