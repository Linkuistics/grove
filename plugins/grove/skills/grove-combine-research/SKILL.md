---
name: grove-combine-research
description: The `combine-research` session kind — union two surveys' coverage into one research document, flag every disagreement, and carry the adversarial move neither survey can perform on itself. Use when a grove mandate names this skill, or when running a `combine-research` session in a grove working tree.
harnesses: [claude-code]
---

# combine-research

**Load the `grove` skill now** — on Claude Code, where plugin skills are
namespaced, that is `grove:grove`. It is the shared spine and holds everything
this kind does not own: the constraints, the bootstrap, and the execute,
decompose, retire and commit procedures. What follows is `combine-research`'s,
and is stated nowhere else.

**combine-research** (AFK) — union two surveys' coverage into the unadorned
`docs/research/<slug>.md`, and flag every disagreement between them.

**This kind, not either producer, carries the adversarial move**, and it is the
one check neither survey can perform on itself. Two vendors trained on
overlapping corpora can agree on something false, so a purely confirmatory
combine raises confidence exactly where it should lower it: a correlated error
laundered as corroboration. **Agreement without independent primary sourcing is
a red flag, not a confirmation.** For each agreed claim, ask whether the two
surveys reached it through *different* primary sources; if they cite the same
blog post, or neither cites anything, that agreement is worth less than a
disagreement.
