# skill-router-k4

## Goal

Rewrite `content/SKILL.md` from 3,152 words into a compact session
protocol/router of roughly **700–900 words**, retaining every lifecycle invariant
it is the canonical source for and routing the rest.

## Context

`SKILL.md` is on **every** session's loaded path, so it is the single largest
lever on what a session pays to read. It is currently a page of conditions that
has re-grown into a partial retelling of the methodology.

These must survive the rewrite, as rules a session holds and not as pointers:

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
- Every rule listed above is stated in `SKILL.md` as the canonical source the
  inventory assigns it, with no mirror the inventory did not permit.
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
