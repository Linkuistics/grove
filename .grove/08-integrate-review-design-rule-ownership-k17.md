# rule-ownership-k17

**Integrates:** rule-ownership-k16

## Goal

Repair the third adversarial read of the rule-ownership design before any corpus
rewrite executes against it. Rework the spec and ADR set in place, recompute the
affected inventory/load cells, and reconcile the root brief and downstream leaf
contracts.

This review was scoped to `rule-ownership-k15`'s delta. It found five P1 defects
and one P2 defect; the `context` occasion itself, the closed occasion domain, the
cross-file edge topology, the trigger-row coverage, and the ADR split's independent
reversibility were otherwise confirmed.

## Context

- `docs/specs/corpus-rule-ownership.md`
- `docs/adr/corpus-rules-have-one-owner.md`
- `docs/adr/restatement-declares-its-class.md`
- `.grove/07-DONE-review-design-rule-ownership-k16.md`
- `.grove/06-DONE-integrate-review-design-rule-ownership-k15.md`
- `.grove/BRIEF.md` and the downstream implementation leaf contracts

## Findings

### P1 — `finish-is-the-drivers-to-discover` was not recomputed under earliest-step wins

The placement function sends a multi-step rule to its earliest step
(`docs/specs/corpus-rule-ownership.md:85`), but
`finish-is-the-drivers-to-discover` remains `step:Finish` at
`docs/specs/corpus-rule-ownership.md:649` even though its own trigger is *the last
live leaf retires*. That event occurs during Retire; Finish is what the driver may
launch afterwards. Its honest occasion includes `step:Retire`, so rule 6 derives
`references/retire.md`, not `references/finish.md`.

If execution follows the current table, `skill-router-k4` sends all nineteen
sessions to the wrong procedure file while `kind-references-k5` preserves the rule
in a finish-kind reference and `loop-step-references-k11` never takes ownership of
it. Recompute the row, trigger sentence 17, both file worklists, and every affected
edge rather than patching only the owner cell.

### P1 — the asserted reachability graph makes same-file rules cyclic

The new rule says every conditional row creates `F -> O`, requires the sentence in
`F` to name `O`'s path, and requires every chain to terminate without cycles
(`docs/specs/corpus-rule-ownership.md:162-166`). But 43 of the inventory's 88
conditional rows have `F = O`. For example, all five conditional follow-on rules
in `references/driver.md` point from that file to itself
(`docs/specs/corpus-rule-ownership.md:526-530`); the same shape recurs in
`execute.md`, `decompose.md`, `retire.md`, and every format file.

Those rows need no pointer: once the owner file has been reached, its own
conditions and procedures are already present. As written, however,
`loaded-path-budgets-k10` is explicitly chartered to require the self-reference
and reject the cycle (`.grove/17-impl-loaded-path-budgets-k10.md:42-45`), so its
test cannot pass without silently weakening the design. Define edges only for
cross-file transitions (or give reflexive rows an explicit non-edge semantics),
then re-walk the 15 distinct cross-file pairs, incoming-owner set, and cycle set.

### P1 — the 600-word floor is derived from an unmeasured ceiling

The arithmetic table labels the frontmatter/title/intro/headings as unmeasured
with a ceiling of 120 words, estimates the eight `own` rows at about 212, measures
the triggers at 281, and then labels their total about 613
(`docs/specs/corpus-rule-ownership.md:257-262`). The following justification
likewise measures only the triggers, estimates the `own` bodies, and never measures
the first part (`docs/specs/corpus-rule-ownership.md:270-276`). The claimed 613 is
therefore `120 + 212 + 281`: a budget ceiling added as though it were real content.

`skill-router-k4` is now required to enforce the resulting 600 floor and is told
that falling below it proves a dropped row (`.grove/10-impl-skill-router-k4.md:46-58`).
A valid compact implementation can instead fall below 600 and be forced to pad;
conversely, the exact 24-sentence and eight-owner assertions already detect the
omission the floor is supposed to catch. Draft and measure the complete router (or
remove the unevidenced floor) before retaining a lower bound.

### P1 — the spec still restates both ADR decisions it says it only cites

The spec says the two ADRs are cited rather than restated
(`docs/specs/corpus-rule-ownership.md:43-51`), then reproduces the complete
`Bound`/`Occasion` domain and ordered placement function at
`docs/specs/corpus-rule-ownership.md:53-89`, duplicating
`docs/adr/corpus-rules-have-one-owner.md:3-20`. It likewise reproduces the three
restatement classes and sharing rule at
`docs/specs/corpus-rule-ownership.md:198-239`, duplicating
`docs/adr/restatement-declares-its-class.md:3-25`.

The split itself is coherent and independently reversible, and the markdown
citations now point to the right record. But these duplicate normative statements
recreate the drift surface the design exists to remove and contradict the spec's
own `spec-grain-rule` (`docs/specs/corpus-rule-ownership.md:869`). Choose one
canonical statement for each decision and leave the other artifact with a citation
plus only the derivation or inventory detail it owns.

### P1 — the sentence-level audit still omits two selection rules

`driving.md` tells a session **when** a question earns a vendor pair
(`content/driving.md:92-94`) and **when** an artifact earns a review chain
(`content/driving.md:399-402`). These are independently violable selection rules,
not the construction rules already inventoried: `chain-is-lazy` says when the next
step is created, and `pair-is-eager` says how the three steps land
(`docs/specs/corpus-rule-ownership.md:586-587`). The relocation table likewise
lists only the construction rules (`docs/specs/corpus-rule-ownership.md:1016` and
`:1024-1027`). The current `decompose.md` already repeats both criteria
(`content/references/decompose.md:93-96` and `:115-118`), but no inventory row
names their canonical source, mirror, load predicate, or test.

Following the inventory as the implementation worklist can therefore delete the
source criteria with `driving.md`, or preserve unowned duplicates by accident;
either outcome fails the root requirement that every normative concept have one
owner. Re-run the promised imperative-sentence audit over all three nominated
files, add these rows at the design's stated grain, and reconcile the relocation
table. While doing so, include the already-inventoried
`escalated-review-routes-through-config` sentence at
`content/driving.md:501-508`, which the relocation table also omits.

### P2 — trigger sentence 1 violates the grammar, and the pair count contradicts itself

The restatement ADR forbids an enumeration in a trigger
(`docs/adr/restatement-declares-its-class.md:14-17`), but canonical sentence 1
enumerates “launched, picked or configured”
(`docs/specs/corpus-rule-ownership.md:308`) while the spec claims no trigger has
an enumeration (`docs/specs/corpus-rule-ownership.md:333-336`). The two driver
rows may still share one sentence because `launch` is one occasion and they have
one owner; reword the condition as that single situation rather than enumerating
its members.

Separately, the arithmetic explanation says **four** sentences cover two rows,
then lists five pairs and correctly computes `29 - 5 = 24`
(`docs/specs/corpus-rule-ownership.md:279-294`). Fix the prose. The mechanical
recomputation otherwise confirmed 29 unique trigger rows mapped exactly once to 24
sentences; every listed word count is correct and totals 281.

## Confirmed recomputations

- All 134 inventory rows use the closed occasion domain. The only recorded
  multi-step row, `escalation-names-the-tradeoff`, correctly resolves to Execute;
  the missed Retire occasion above is the exception.
- The three `context` rows match the bounded definition: durable artifact
  ownership, the plugin/toolchain prerequisite, and the binary build boundary.
  Rule 5 derives `references/grove.md` for all three. The ADR honestly concedes
  that choosing an occasion remains reviewer-checked judgement.
- Ignoring the reflexive-row defect, all 15 distinct cross-file edge pairs have a
  static terminus and no cycle; the downstream contracts assign the required
  pointer sentences to the file-owning leaves.
- The two ADRs contain independently reversible trade-offs, and a repository-wide
  markdown citation check found no stale citation to the pre-split half.

## Done when

- Every P1 is repaired at the model level and all affected inventory, trigger,
  edge, brief, and implementation-contract cells are recomputed.
- The P2 trigger/count contradictions are corrected while preserving the verified
  29-row coverage and measured 281-word trigger set.
- The ADR/spec set is again a minimum coherent set with one normative source per
  decision and no dangling citations.
- `content/SIGNAL.md`, `content/SIGNAL-FINISH.md`, and `src/prompt.rs` remain
  unchanged.

## Notes

- This review ran no test, build, lint, or format command and edited no reviewed
  artifact. Its only writes are this integration leaf and the review leaf's
  retirement.
