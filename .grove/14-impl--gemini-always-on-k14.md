# gemini-always-on-k14

## Goal

Make the skill always on in Gemini as far as Gemini allows, as the design maps
it. Gemini has no global instruction file in the human's setup, so the design
may leave it with the skill and the model running the checker itself.

## Context

- Read the design `docs/specs/technical-communication.md` and `harness-hooks-a.md`
  for Gemini's hook and instruction mechanisms, and reuse the shared scripts and
  switch from `claude-code-always-on-k11`.
- `plugins/install.sh` links the skill into `~/.gemini/skills`.

## Done when

- In a real Gemini session, show by running it which layers work and which fall
  back to the model running the checker, and that the switch turns them off. Put
  the evidence in the running log.
- `bash scripts/check.sh` is green.
- If Gemini offers nothing beyond the skill, the design's mapping and this leaf's
  running log say so plainly, and no mechanism is added for its own sake.
