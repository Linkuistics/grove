# decision-records-skill-k4

**Kind:** work

## Goal

Push the **reopening trigger** upstream into the `linkuistics:decision-records`
skill, where it belongs — it is a fact about good ADRs generally, not a grove-local
habit.

## Context

- The skill: `~/.claude/plugins/marketplaces/linkuistics/plugins/linkuistics/skills/decision-records/SKILL.md`
  (edit the **marketplace source**, not the `plugins/cache/` copy).
- grove and the linkuistics skills are **one system**: implement this directly, do
  not park it as a recommendation.
- The skill today asks for the rejected alternative *"when the rejection is
  non-obvious and someone will otherwise re-propose it"* — i.e. it already knows
  **why** a rejection is recorded. What it never asks is what would make the
  rejection *stop holding*.
- The exemplar is in this repo: `docs/adr/model-per-task-kind.md`, whose
  cross-family rejection ends — *"What would reopen this: a coherent
  provider/credential design for grove, or evidence that actually measures the
  cross-family increment in defect-detection recall."*
- ADR *pruning* in this repo is the second exemplar (its every rejection carries
  one).

## Done when

- The skill's **Considered options** guidance asks a rejection to state **what would
  reopen it** — the condition under which the path becomes live again.
- The argument is made, not just the rule: a rejection without a reopening trigger
  is a **tombstone**; with one it is a **gate with a key**. A future reader can then
  test the trigger against present conditions instead of re-litigating the decision
  from scratch — which is the entire reason the entry exists.
- It stays **proportionate**: not every rejection has a meaningful trigger (some are
  closed forever — a naming call, a dead technology), and the skill must say so.
  "Nothing would reopen this" is a legitimate and useful answer; a *mandatory*
  trigger would just breed ceremony, which is the failure mode that skill is most
  careful about ("The value is in recording *that* a decision was made and *why* —
  not in filling out sections").
- Committed in the linkuistics repo with its own focused commit.

## Notes

Resist scope creep into the rest of the skill. One idea, well argued, in the section
that already exists.
