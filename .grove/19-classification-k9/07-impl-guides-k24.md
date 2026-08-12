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
carved. `kinds-k22` carved them; you write the edges. **You own inventory rows
1–8**, which is more than any other batch:

| row | source (already carved, or in this batch) | target |
|---|---|---|
| 1 | `TASK-FORMAT.md`'s `requirements` bullet | `grilling.md` bodies |
| 4 | `TASK-FORMAT.md`'s *three design kinds* `requirements` bullet (L478–480) | `grilling.md`, `CONTEXT-FORMAT.md` bodies |
| 2 | `TASK-FORMAT.md`'s `design` bullet (L481–484) | `ADR-FORMAT.md` bodies |
| 3 | the same `design` bullet | `SPEC-FORMAT.md` §*current-state* / membership / grain |
| 5 | `grilling.md` §*Offer ADRs sparingly* | `ADR-FORMAT.md` bodies |
| 6 | `grilling.md` §*Update CONTEXT.md inline* | `CONTEXT-FORMAT.md` bodies |
| 7 | `grilling.md` §*Agree the test seams* | `SPEC-FORMAT.md` bodies |
| 8 | `SPEC-FORMAT.md`'s own opening | `SPEC-FORMAT.md` §*current-state* / membership / grain |

Row 1 is the edge the whole batch stands on: without it, every procedural unit you
carve out of `grilling.md` fails reachability. Rows 5–7 are
procedural→procedural chains, which are legal and exactly what *deferral may chain*
is for. Row 8 is how `SPEC-FORMAT.md` roots itself — its opening carries a real
condition (*a spec is written "when the increment is a genuine agreement point", and
most increments write none*).

Run the sweep for all four filenames before you finish, as **evidence** alongside
the inventory:

```
grep -rn 'grilling\.md\|SPEC-FORMAT\.md\|CONTEXT-FORMAT\.md\|ADR-FORMAT\.md' content/
```

**Hits inside `SKILL.md` are not yours and you write nothing for them.** They sit
inside `pending-skill-*` units, and **no edge may have a `pending-*` source** — that
was the lossy ledger `batches-k33` F2 killed. `SKILL.md`'s own ADR and spec
paragraph is inventory rows 20–21, owned by `execute-k29`; its glossary paragraph is
row 29, owned by `finish-cycle-k32`. Report those hits as *not yours* rather than as
missed, and do not park a `defers=` anywhere.

Hits inside `SKILL.md` `## Reference files` are a **standing sweep exclusion** for
every batch: that index names all eight guides and none of its rows is a
trigger→body edge.

### The pre-decided calls in this region

The node brief settles these; apply them rather than re-deciding.

- **Family E bodies — `grilling.md` §*Offer ADRs sparingly* and `ADR-FORMAT.md`'s
  head.** Both **procedural**. The head is a *redirect* to
  `linkuistics:decision-records`, not a condition — which is what your own notes
  below already say, and it is now settled rather than argued.
- **Family C body — `ADR-FORMAT.md` §*Why the set stays minimal*.** **Procedural**,
  rooted by rows 2 and 5. `SKILL.md` L217–227 (the family C/D/E owner, #9) adds a
  further address as row 20; that is a genuine second condition→body address, not a
  reachability crutch, and it is **not yours to write**.
- **Family D second condition — `SPEC-FORMAT.md`'s opening (L6–15).**
  **Triggering.** *"Most increments write no spec at all"* is a condition with
  teeth, and it is what lets this file root itself (row 8).
- **Family D bodies — `SPEC-FORMAT.md` §*The set is current-state*, the membership
  test, and the grain rule (L17–36).** **Procedural.** These are tests you apply
  once you are writing or pruning a spec, not conditions that make you notice you
  should. `SKILL.md` `## Specs` (#12) is a further body of the same rule, and
  `SKILL.md` L217–227 (#9) is the corpus-wide owner; neither is yours.
- **`CONTEXT-FORMAT.md`** carries no family site — it is a pure format guide, all
  procedural, rooted by rows 4 and 6.

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
  two sections, both procedural — now settled above rather than left to you.

## Done when

- All four files are subdivided into real units, and none of them retains a seed
  or `pending-` id.
- **Every procedural unit in all four is reachable**, and the chain that reaches
  the `ADR-FORMAT.md` and `CONTEXT-FORMAT.md` units passes through `grilling.md`
  and terminates. `cargo build` proves all three of those.
- `cargo test` green; `EMBEDDED_UNITS` updated in the same commit, each new id
  named deliberately, and the four seed ids removed.
- **Inventory rows 1–8 are each reported** — written, or declined with a reason.
  Reachability going green is a weaker claim: rows 2 and 5 both reach
  `ADR-FORMAT.md`, so dropping either leaves the build green.
- The sweep is run and its outcome stated, including every `SKILL.md` hit reported
  as *not yours* (rows 20–21 and 29) rather than as missed.

## Notes

- This is the first batch that writes cross-file `defers=`, so it is the first
  chance for `embed-wide-gate-k8`'s reachability, class and termination checks to
  fire in anger. If any of them fires on something you believe is correctly
  classified, that is a finding about the gate or the design — record it, do not
  work around it.
- Doubts to carry forward, by id. The `grilling.md` → `ADR-FORMAT.md` /
  `CONTEXT-FORMAT.md` chains are the least obvious calls in this batch and are
  worth naming even if you are confident.
