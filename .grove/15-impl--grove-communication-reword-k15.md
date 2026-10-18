# grove-communication-reword-k15

## Goal

Reword grove's "Human-facing communication" section so that it sits under the
skill's precedence order and does not pull against it. The section asks for a
guided, conversational, explanatory style; the skill rules govern the wording
inside that style.

## Context

- The section is in `plugins/grove/skills/grove/SKILL.md`. The spec
  `docs/specs/technical-communication.md`, section *Precedence in grove's own
  text*, states what it must say, and two smaller edits: the entry for
  `linkuistics:technical-communication` in `references/grove.md`, and one
  sentence in `references/commit.md` that allows a conversion commit before the
  task's commit.
- Keep what house style owns: what to explain, leading with the outcome,
  explaining before asking for a decision, and where evidence goes. Change only
  what asks for wording that the skill's rules restrict, such as long or
  idiomatic sentences.
- The skill text reaches users through the Claude marketplace, Grove's embedded
  snapshot for Codex and `plugins/install.sh`. Find any other copy of the
  section, such as an embedded or generated one, and update it.
- `plugins/grove/conformance.sh` checks the grove skills.

## Done when

- The section, the hub entry and the commit sentence meet the spec's
  requirements, and every copy of each is updated.
- The edited text passes the document checker, apart from justified departures.
- `bash scripts/check.sh` is green, including conformance.
