# doubt-moves-k27

## Goal

Classify **`content/driving.md` lines 415–586** (11,128 bytes): `## Doubting
inside a picked Grove leaf` (2,188 bytes) and `## The review chain — when doubt
earns its own leaves` (8,940 bytes).

This is batch 7 of 12. `## The review chain` is the **single largest section in
`driving.md`**, which is why these two sections get a batch of their own.

## Context

Read the node brief's *The batching contract* first.

### Region and residual

- Carve `content/driving.md` **L415–L586**, consuming the front of
  `pending-driving-doubt`.
- Mint exactly one residual, **`pending-driving-decompose`**, covering
  **L587–L754**.
- Redistribute any `defers=` inherited from `pending-driving-doubt`.

### The judgement this batch exists for

Both sections state a **predicate** and then a long body, and the predicate is the
part a session cannot afford to miss.

- **`## Doubting inside a picked Grove leaf`** opens on the composition rule's
  precondition — *the driver launched this session with a selected-leaf mandate
  and the session adopted it by running Bootstrap; merely finding `.grove/` does
  not count*. That precondition is triggering and is scoped to nothing narrower
  than `*`: every kind can be picked. The budget it then states ("at most one
  reviewer across the whole picked leaf", and what the second need signals) is
  also a condition, not a procedure — a session that does not know the budget
  exists spends it silently.
- **`## The review chain — when doubt earns its own leaves`** is 8,940 bytes and
  will not be one unit. Its opening states *when* to reach for the chain, which is
  triggering; the bulk is the mechanics of cutting each step, writing its body,
  and placing an integration — procedural. Expect several units, and expect the
  triggering share to be a small fraction of the section's bytes.

**This region overlaps `SKILL.md` twice.** `SKILL.md`'s *Review ownership inside a
picked leaf* paragraph (carved by `execute-k29`) and its *Cut the next step*
paragraph (carved by `shape-cutting-k30`) state the same two rules more tersely.
Decide here which side holds the condition and which holds the body, **record the
call in this leaf's body**, and expect `execute-k29` and `shape-cutting-k30` to
defer into your units rather than restate them. Getting this wrong in either
direction is the most consequential misclassification available in this batch:
duplicate the condition on both sides and every mandate carries it twice; put it
on neither and the mandate carries it nowhere.

### Cross-file deferral

- `SKILL.md` and `TASK-FORMAT.md` are both cited in this region. `TASK-FORMAT.md`
  is fully carved (`kinds-k22`, `shapes-k23`), so those edges are available where
  the reference is genuinely trigger→body.
- `SKILL.md` references still land in `pending-skill-*` units. Per the brief:
  either add the `defers=` to the `pending-` unit and let its batch redistribute,
  or leave it to `execute-k29` / `shape-cutting-k30` and **say so**.

## Done when

- `content/driving.md` L415–586 is subdivided into real units;
  `pending-driving-decompose` covers L587–754 and nothing else.
- Any `defers=` inherited from `pending-driving-doubt` is redistributed and
  accounted for.
- `cargo build` and `cargo test` are green.
- `EMBEDDED_UNITS` updated in the same commit, each new id named deliberately.
- The `SKILL.md` overlap call is recorded in this leaf's body, by id, for
  `execute-k29` and `shape-cutting-k30`.

## Notes

- This region contains fenced `grove-llm leaf-add` / `leaf-insert` examples. Do
  not split mid-fence.
- Doubts to carry forward, by id. The condition/body split between this file and
  `SKILL.md` is the doubt most worth handing to the aggregate reviewer.
