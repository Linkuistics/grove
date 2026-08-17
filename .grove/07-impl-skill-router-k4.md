# skill-router-k4

## Goal

Rewrite `content/SKILL.md` from 3,152 words into a compact session
protocol/router of roughly **700–900 words**, retaining every lifecycle invariant
it is the canonical source for and routing the rest.

## Context

`SKILL.md` is on **every** session's loaded path, so it is the single largest
lever on what a session pays to read. It is currently a page of conditions that
has re-grown into a partial retelling of the methodology.

These must survive the rewrite as rules a session holds — but **"holds" now has
two shapes, and the inventory says which each one gets**. Seven rows are `own`
class: `SKILL.md` *is* their canonical source, because their whole content is
their trigger and no procedure remains to defer. The rest are one `trigger`
sentence of ≤25 words — the situation, a single-clause obligation, and the owner
file's path — which is more than a bare pointer and less than the procedure. Do
not read the list below as licensing a full restatement of all ten:

- the driver's mandate is **authoritative**;
- **no second pick** — `grove-llm pick` is a diagnostic, and the mandate wins;
- the driver's **VCS statement is definitive**; do not re-derive it, and a harness
  banner that disagrees does not win;
- **stale-launch handling** — a handle resolving to nothing or to a terminal leaf
  is a stale launch, not work to redo;
- **bootstrap order** — glossary, cited ADRs, brief chain root→leaf, task file;
- the **execution / decomposition boundary** — what belongs in this session and
  what belongs in the tree;
- **human-only pruning**;
- **retire before commit**;
- the **commit boundary** — one task, one focused commit, named by handle;
- **finish ownership and the terminal-signal distinction** — the driver decides a
  grove is finished, and `SIGNAL.md` and `SIGNAL-FINISH.md` are not
  interchangeable.

Everything else routes. A condition earns its place in `SKILL.md` only when a
session that does not already hold it would fail to *ask* — the asymmetry the
corpus is already cut along: withholding a procedure costs a lookup the session
knows to make, withholding a condition yields an unasked question.

## Done when

- `content/SKILL.md`'s body is roughly 700–900 words, and the number is a
  consequence of the routing discipline rather than a target hit by compression.
- Every rule listed above appears in the class the inventory assigns it — `own`
  for the seven, one `trigger` sentence for the rest — and no rule the inventory
  marks `none` appears at all.
- The budget is **asserted**, not achieved and forgotten: total words in range, at
  most **19** `trigger` sentences, each ≤25 words, and the seven `own` rows
  present. The spec's *What `SKILL.md` can hold, arithmetically* table is the
  contract, including which trigger rows share a sentence.
- The spine is `own` here, as **one** row rather than seven — six corpus files cite
  the constraints by number and none of them is on a static path. Carry
  constraint 4's *just-in-time, not few* clause with it.
- Every routed rule names the reference file carrying it, and every named path
  resolves — the routing-table check in `tests/methodology.rs` still passes.
- The YAML frontmatter survives provisioning unchanged.
- `behavior-evals-k3` is still green.
- The file teaches nothing it is not the canonical source for; a reader looking
  for a procedure is sent, not summarised at.

## Notes

- Execute `rule-ownership-k2`'s inventory. If a rule's assignment there turns out
  to be wrong once you are writing the prose, that is a finding worth recording —
  fix the inventory too, so the two do not drift on day one.
- `content/SIGNAL.md` and `content/SIGNAL-FINISH.md` are **byte-frozen** for this
  grove. You may change how `SKILL.md` refers to them; you may not change them.
- The 500-line ceiling in `tests/methodology.rs` still exists at this point and
  will pass trivially. It is replaced in `loaded-path-budgets-k10`, not here —
  do not delete it early and leave the corpus unmeasured in between.
- Cut the `review-impl` step as this leaf's last act. A 75% reduction of the file
  every session reads is exactly the load-bearing artifact the review chain exists
  for, and the specific doubt to write into that leaf's body is *which retained
  rule got quietly weakened into a pointer*.
