# skill-router-k4

## Goal

Rewrite `content/SKILL.md` from 3,152 words into a compact session
protocol/router of **at most 900 words**, retaining every lifecycle invariant
it is the canonical source for and routing the rest.

## Context

`SKILL.md` is on **every** session's loaded path, so it is the single largest
lever on what a session pays to read. It is currently a page of conditions that
has re-grown into a partial retelling of the methodology.

These must survive the rewrite as rules a session holds — but **"holds" now has
two shapes, and the inventory says which each one gets**. Eight rows are `own`
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

- `content/SKILL.md`'s body is **at most 900 words**, and the number is a
  consequence of the routing discipline rather than a target hit by compression.
  **There is no floor, and you must not write to one.** The drafted content —
  26 triggers at 302 words, seven `own` bodies at 192 — projects a file near
  500–620, but that is a projection: the intro, headings and the eighth `own` body
  are yours to write and nobody has measured them. **Record what you actually
  measured**, in the leaf's running log and in the test's own documentation.
- Every rule listed above appears in the class the inventory assigns it — `own`
  for the eight, one `trigger` sentence for the rest — and no rule the inventory
  marks `none` appears at all.
- The budget is **asserted**, not achieved and forgotten: total words ≤900,
  **exactly 26** `trigger` sentences, each ≤25 words, and the eight `own` rows
  present. Those three exact assertions are what detect a dropped row — by name,
  which is why a word floor was removed rather than lowered again. The spec's *What
  `SKILL.md` can hold, arithmetically* table is the contract, and its *The trigger
  sentences* table is the canonical wording — reword within the grammar if you must,
  but do not add or drop a sentence: the set is the reachability graph's edge list
  out of this file, and a missing sentence silently unreaches a rule.
- **Sentences 25 and 26 are load-bearing and new.** They are the only conditions
  that send a session to `references/decompose.md` to decide whether its artifact
  earns a review chain or its question earns a vendor pair. Without them a producer
  holding neither condition never opens the file and never asks — none of sentences
  7–11 fires for *my artifact is load-bearing and finished*.
- The spine is `own` here, as **one** row rather than seven constraints — six corpus files cite
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
- Cut the `review-impl` step as this leaf's last act — **done**, as
  `skill-router-k20`. A 75% reduction of the file
  every session reads is exactly the load-bearing artifact the review chain exists
  for, and the specific doubt to write into that leaf's body is *which retained
  rule got quietly weakened into a pointer*.

## Running log

**Measured, not projected.** `content/SKILL.md`'s body is **796 words** (from
3,152), 113 lines, 26 trigger sentences, longest 16 words, mean 11.6. The spec
projected 500–620; the difference is the three parts nobody had drafted when that
projection was made — the intro, the section headings, the routing table's cells,
and the eighth `own` body. The number is recorded in the test's own doc comment
and here, and it is a measurement: the ceiling of 900 is what binds, and there is
no floor to write to.

**The budget assertion** is `tests/methodology.rs::the_skill_holds_its_condition_register_budget`
— ≤900 words, exactly 26 triggers, each ≤25 words, and the eight `own` rows named
one at a time so a failure says which row left. The 500-line ceiling beside it is
untouched and now passes trivially; `loaded-path-budgets-k10` replaces it.

**Four rules were stated in `SKILL.md` and nowhere else in `content/`**, so
removing them would have deleted them outright rather than rehoming them. The
spec's *later leaf performs the move* convention assumes the earlier leaf may
keep its copy, and this leaf's contract forbids that, so this leaf performed the
whole move: `one-focused-commit`'s scope and the Retire-first reason into
`references/commit.md`; `node-close-is-implicit` plus *the close asks the human
nothing*, and `pruning-is-hitl`'s *an agent never prunes on its own*, into
`references/retire.md`. Every other rule's owner already stated it. The inventory
was right about the owners; what it could not say is which owners were not yet
carrying their rule, and that gap is now recorded in the spec.

**A procedure register must not restate the condition.** The pruning move first
landed in `retire.md` in the trigger's own words (*finds its leaf's path decided
against*), which made `SKILL.md`'s sentence 13 redundant to a delivery check —
`tests/lifecycle_invariants.rs`'s condition-severing control caught it, which is
the k3 net doing exactly what it was built for. Reworded to state the obligation
without the situation.

**Several triggers name an owner that does not yet carry the rule** — the repo-claim,
decision-log, escalation, fog, prior-art, triage and shape-selection rules are
still in `driving.md` until `loop-step-references-k11` and `corpus-split-k6` move
them. Not homelessness (they are stated in `content/`), and not this leaf's to
fix: the canonical trigger set is the design's, and repointing it at `driving.md`
would name a file those leaves delete. Recorded so the window is deliberate
rather than discovered.

**Five test files were repointed**, not relaxed: `commit_guidance` (commit scope
→ `commit.md`; Retire-before-Commit now asserted over the two trigger sentences'
order), `composition_guidance` (the re-derives/consumes asymmetry →
`decompose.md`; *no launch metadata* → `TASK-FORMAT.md`), `lifecycle_invariants`
(the condition-severing control's excised text → trigger 13), `prompt` (the VCS
condition's opener), `retire_guidance` (node close and pruning → `retire.md`,
with the two `SKILL.md` triggers asserted alongside). Full suite: 948 passing,
clippy clean.
