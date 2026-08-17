# rule-ownership-k13

**Integrates:** rule-ownership-k12

## Goal

Repair the rule-ownership design before any corpus rewrite executes against it.
Rework `docs/specs/corpus-rule-ownership.md` and
`docs/adr/corpus-rules-have-one-owner.md` in place; then reconcile the root
brief's work order if the corrected ownership map changes a later leaf's scope.

The review found five actionable defects. Preserve the two confirmed decisions
below while fixing them.

## Context

- `docs/specs/corpus-rule-ownership.md`
- `docs/adr/corpus-rules-have-one-owner.md`
- `.grove/BRIEF.md`
- `src/prompt.rs:136` (`reference_file`)

## Findings

### P1 — `Bound(R)` does not determine one canonical source

The advertised placement function is not a function of its stated input. The
spec defines `Bound(R)` only as a set of kinds and maps “all nineteen” to a
loop-step reference (`docs/specs/corpus-rule-ownership.md:43`), but its own ADR
test says `Bound(R)` is all nineteen and selects the artifact format file instead
(`docs/specs/corpus-rule-ownership.md:353`). That choice requires an additional
event/topic judgement: “about to write an ADR.” The same ambiguity exists between
`records-are-current-state` in `execute.md` and the artifact-specific current-set
rules in `ADR-FORMAT.md` and `SPEC-FORMAT.md`.

The load notation is inconsistent with the fixed runtime too. `always(K)` is
defined as a static path whose file is `SKILL.md` or `reference_file(k)`
(`docs/specs/corpus-rule-ownership.md:139`), while inventory rows assigned to
`execute.md`, `decompose.md`, `retire.md`, and `commit.md` are labelled
`always(19)`. `src/prompt.rs:136` maps kinds only to the ten kind-reference
files; loop-step references are conditionally reached and are not on that static
path.

Replace `Bound(R)` with an input that includes the encounter/load predicate, or
add an explicit deterministic tie-break and distinguish static from conditional
paths. Then recompute every owner and load cell. The ADR's claim that placement
is computed rather than argued cannot stand until this is resolved.

### P1 — the inventory is neither complete nor in its required schema

The root requirement asks every normative concept to carry a rule ID, canonical
source, permitted mirrors, load predicate, and behavioural tests
(`.grove/BRIEF.md:23`). The kind-reference and format-file inventories provide
only a file and a semicolon-separated rule-name list
(`docs/specs/corpus-rule-ownership.md:288` and
`docs/specs/corpus-rule-ownership.md:311`). Roughly half the named rules
therefore have no mirror, load, or test classification.

The nominated source audit also found unlisted rules:

- The global HITL/AFK rule — the mark predicts presence but never limits the
  legitimacy of an escalation — is normative in `content/SKILL.md:189` and
  `content/driving.md:192`, but has no inventory row.
- `content/references/driver.md` has only seven inventory rows, leaving such
  session instructions as configuration revalidation (`:46`) and session-name
  derivation (`:53`) neither owned nor marked for relocation.
- `grilling.md` is reduced to one `grilling-procedure` row, although
  `content/grilling.md:15`, `:63`, `:75`, `:79`, and `:92` separately constrain
  when writes may begin, glossary challenge, code cross-checking, inline glossary
  updates, and test-seam agreement. Some duplicate other proposed owners, but the
  inventory neither names them as mirrors nor relocates them.

Audit every `content/` owner file against the spec's own completeness rule, and
give every surviving rule the full five-column record before downstream leaves
use the map.

### P1 — deleting `driving.md` would delete live policy

The spec claims all of `driving.md`'s normative rules are inventoried and the
remainder has `Bound(R) = ∅` (`docs/specs/corpus-rule-ownership.md:410`). Direct
inspection refutes that claim. Uninventoried imperatives include when and how to
commission prior-art research (`content/driving.md:21`), the research-brief
requirements (`:43`), asking for an LLM recommendation and pushback (`:136` and
`:162`), making an escalation answerable (`:174`), recording decisions inline
(`:197`), maintaining a bidirectional research-to-ADR bridge (`:216`), and
scoping a prune across a flat review chain (`:625`). These alter session conduct;
they are not merely arguments, examples, provenance, or history.

For each section, deliberately keep and rehome its operative rule or decide that
the behaviour is no longer Grove policy and reconcile every condition that still
points at it. Do not move it to `docs/` under the current `Bound = ∅` assertion.

### P1 — the mirror rule conflicts with its own inventory and the brief

The design says a `SKILL.md` mirror may only be a one-sentence condition naming a
procedure file and may not carry a test, threshold, list, or procedure
(`docs/specs/corpus-rule-ownership.md:69`). The inventory nevertheless permits
the seven-item spine list (`:169`), the bootstrap-order list (`:199`), the
one-reviewer threshold (`:211`), the integration-placement test (`:232`), and
the triage mapping (`:253`) in `SKILL.md`. The root brief independently requires
`SKILL.md` itself to carry bootstrap order and several lifecycle invariants
(`.grove/BRIEF.md:14`). A pointer-only condition does not carry the order; a
mirror that does carry it violates the ADR.

Specify the exact legal `SKILL.md` sentence for every permitted mirror, including
whether the spine is an explicit exception, and reconcile that result with the
700–900-word target. There are 39 `SKILL.md` mirror rows whose current rule
descriptions already total about 670 words before router prose; the size target
is plausible only if the condition/procedure reduction is made concrete rather
than assumed.

### P2 — the B/S classification is incomplete and misclassifies conduct

The nine required behavioural areas are named in the prose test-seam list, but
the inventory does not actually classify all of them: `grilling-threshold` is in
the two-column kind table and has no B/S cell. Conversely,
`no-adjacency-exception` (`docs/specs/corpus-rule-ownership.md:233`),
`no-fourth-status` (`:254`), and `nothing-after-finish` (`:274`) direct session
conduct but are S-only. A uniqueness sweep can show that one sentence owns a
rule; it cannot show that a session obeys it.

Give every rule an explicit test class, add B wherever conduct is the property,
and reserve S for the single-source/budget property the definition states. Keep
`behavior-evals-k3` green-before/green-after by separating a rule's stable
conduct check from the post-rewrite uniqueness check when both apply.

## Eight-claim disposition

1. **Placement function:** refuted by the all-nineteen ADR counterexample and
   static/conditional-path mismatch.
2. **`Bound(R)` is judgement-free:** refuted; the set “must obey” is semantic,
   and the missing HITL/AFK rule plus the record-writer case require judgements
   the runtime mapping cannot supply.
3. **Inventory completeness:** refuted by both nominated files and the missing
   inventory columns.
4. **`driving.md` has no normative remainder:** refuted by the imperative
   sections cited above.
5. **Two registers fit `SKILL.md`:** refuted until the forbidden-list/threshold
   mirrors and the brief's “still carries” requirement are reconciled.
6. **B/S split:** refuted as an inventory contract. The prose names the nine
   required areas, but one has no class and multiple conduct rules are S-only.
7. **Deferral policy:** confirmed. A controlled recount found 14 distinct
   `(file, skill)` pairs across 9 files, and the table makes a per-row decision
   under one useful generating question; repeated outcomes do not make it a
   blanket answer.
8. **ADR threshold and separability:** confirmed. The ownership decision is hard
   to reverse, surprising, and a real trade-off; it governs intra-corpus filing,
   while `skill-delivers-the-methodology` governs cross-channel delivery. Either
   can change without forcing the other, so the records are a minimum coherent
   pair once the placement decision itself is corrected.

## Done when

- The five findings above are resolved in the spec and ADR in place.
- Every inventory rule has all required metadata, and each nominated owner file
  has been audited sentence by sentence.
- `driving.md` is deleted only after every surviving imperative has an embedded,
  reachable owner.
- The root brief and downstream leaf scopes are reconciled with the corrected
  map before `behavior-evals-k3` runs.

## Notes

- Inspection only produced these findings; `rule-ownership-k12` changed no
  reviewed artifact and ran no test, build, lint, or format command.
- Controlled inspection confirmed the producer's supporting counts: 14 distinct
  plugin deferrals across 9 files; the integration-placement rule in the six
  named source files; and three working-increment statements across
  `planning.md` and `execute.md`. Those counts do not repair the completeness
  defects above.
