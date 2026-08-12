# guides-k24

## Goal

Classify **four whole files** (15,667 bytes together):

| file | bytes |
|---|---|
| `content/grilling.md` | 4,735 |
| `content/SPEC-FORMAT.md` | 5,364 |
| `content/CONTEXT-FORMAT.md` | 3,502 |
| `content/ADR-FORMAT.md` | 2,066 |

This is batch 4 of 12. It is one closure, not four files that happen to be small:
**`grilling.md` is the entry point, and the other three hang off its move
sections.**

## Context

Read the node brief's *The batching contract* first.

### Region and residual

All four files are carved **whole**. The seed ids `grilling`, `spec-format`,
`context-format` and `adr-format` are consumed and **no residual is minted**.

### The rooting, which is the whole reason these four are one batch

Three of these files carry **no condition of their own** — they are procedures
entered from elsewhere — so this batch must reach back to roots earlier batches
carved. `kinds-k22` carved them; you write the edges.

1. **`grilling.md` ← `TASK-FORMAT.md`'s `requirements` unit** (and its twin in
   `## The three design kinds`, L478–480: *"**requirements** runs the grilling
   procedure (`grilling.md`)"*). Add `defers=` to those units. This is the edge
   the whole batch stands on: without it, every procedural unit you carve out of
   `grilling.md` fails reachability.
2. **`ADR-FORMAT.md` ← `grilling.md` §*Offer ADRs sparingly*** — a
   procedural→procedural chain, which is legal and is exactly what *deferral may
   chain* is for. `TASK-FORMAT.md`'s `design` bullet (L481–484) is the second
   inbound edge; add it too. `SKILL.md`'s ADR paragraph is a third, and
   `execute-k29` will sweep it in later — you do not owe it.
3. **`CONTEXT-FORMAT.md` ← `grilling.md` §*Update CONTEXT.md inline***, plus
   `TASK-FORMAT.md`'s `requirements` bullet (*"updates `CONTEXT.md` **inline** as
   terms are resolved"*).
4. **`SPEC-FORMAT.md`** is the one file here that **can root itself**: its opening
   carries a real condition — *a spec is written "when the increment is a genuine
   agreement point", and most increments write none*. Root it internally, then add
   the inbound edges from `grilling.md` §*Agree the test seams* and
   `TASK-FORMAT.md`'s `design` bullet.

Run the inbound sweep for all four filenames before you finish:

```
grep -rn 'grilling\.md\|SPEC-FORMAT\.md\|CONTEXT-FORMAT\.md\|ADR-FORMAT\.md' content/
```

Hits inside `SKILL.md` will still be sitting in `pending-skill-*` units. Per the
brief: add the `defers=` to the `pending-` unit itself, and the batch that carves
that region redistributes it. **Or** leave those to `execute-k29` /
`finish-cycle-k32`, whose regions they are — but then say so in your body, because
silence there is indistinguishable from having missed them.

### Judgement notes per file

- **`grilling.md`** is vendored and fused from two upstream skills. Its
  `<what-to-do>` block is a procedure a `requirements` session executes; its
  `## During the session` moves are each a small condition-plus-body. Note that
  the file opens with two HTML comments carrying licence provenance — those
  belong to whichever unit follows them, and **must not be split away from it**.
- **`SPEC-FORMAT.md`** carries an inline licence comment at L66–69 mid-file
  (OpenSpec attribution). Keep it with its `## Requirements` prose.
- **`CONTEXT-FORMAT.md`** is a pure format guide with a long leading provenance
  comment. Almost entirely procedural; the condition ("you are resolving a term
  and the glossary needs an entry") lives in `SKILL.md` and `grilling.md`.
- **`ADR-FORMAT.md`** is the smallest and the clearest: no condition of its own,
  two sections, both procedural. Its head paragraph is a *redirect* to
  `linkuistics:decision-records`, not a condition — resist reading it as one just
  because it is first.

## Done when

- All four files are subdivided into real units, and none of them retains a seed
  or `pending-` id.
- **Every procedural unit in all four is reachable**, and the chain that reaches
  the `ADR-FORMAT.md` and `CONTEXT-FORMAT.md` units passes through `grilling.md`
  and terminates. `cargo build` proves all three of those.
- `cargo test` green; `EMBEDDED_UNITS` updated in the same commit, each new id
  named deliberately, and the four seed ids removed.
- The inbound sweep is run and its outcome stated in this leaf's body — including
  any hit deliberately left to a later batch.

## Notes

- This is the first batch that writes cross-file `defers=`, so it is the first
  chance for `embed-wide-gate-k8`'s reachability, class and termination checks to
  fire in anger. If any of them fires on something you believe is correctly
  classified, that is a finding about the gate or the design — record it, do not
  work around it.
- Doubts to carry forward, by id. The `grilling.md` → `ADR-FORMAT.md` /
  `CONTEXT-FORMAT.md` chains are the least obvious calls in this batch and are
  worth naming even if you are confident.
