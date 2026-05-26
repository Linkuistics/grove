# 010-revise-skill-retire-paragraph

**Kind:** work

## Goal
Revise `content/SKILL.md` so the retirement doctrine reliably produces a parent-chain check at the end of every leaf — with user confirmation before each node-level retirement, recursing upward until an ancestor still has live leaves (or the grove root is reached).

## Context
- The current `**Retire.**` paragraph in `content/SKILL.md` reads as a fact ("When a node's last live leaf completes, …"), not as a step the agent owes the session. Make it imperative and procedural — something an agent *does* before ending the session.
- The inner-loop mermaid graph already has `retire{"node's last live leaf?"}` as a decision diamond after `commit`. Decide whether the diamond needs sharpening (e.g., to show the loop back to the same check after a node retire) or whether prose alone carries the recursion clearly. Prefer the smaller change.
- Distinguish leaf-level retirement (the `mv` of the just-finished leaf into its parent's `done/` — mechanical, no asking) from node-level retirement (the promote-upward + `mv` of the whole node into `.grove/done/` — judgement, ask the user). Blurring the two will make agents either over-ask (annoying) or under-ask (current failure mode).
- Keep `content/prompts/continue.md` as a one-liner pointing at the skill — methodology stays in SKILL.md.

## Done when
- `content/SKILL.md`'s `**Retire.**` paragraph:
  - reads as something the agent *does*, not just something that is true;
  - covers the parent-chain walk after every leaf retirement;
  - calls out the user-confirmation step ("ask the user before retiring") for *node-level* retirement specifically;
  - notes the cascade (a node retire may empty its parent; keep walking until a node still has live leaves or you reach the root);
  - keeps the leaf/node distinction sharp;
  - stays within the existing one-page-of-rules constraint (Principle 7).
- The mermaid graph is consistent with the revised prose — sharpened if needed, otherwise left alone.
- `docs/workflows/multi-step.md` is skimmed and updated only if it now describes the loop incorrectly. Aim for a small wording tweak, not a rewrite.
- `content/prompts/continue.md` and `content/prompts/retire.md` are untouched.
- One focused commit. Then check whether retiring this leaf empties the grove (this is the only live leaf) — and apply the new doctrine to the grove itself.
