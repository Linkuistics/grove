# behavior-evals-k19

**Integrates:** behavior-evals-k18

## Goal

Repair the lifecycle safety net before any corpus rewrite executes against it.
Make rule delivery genuinely trigger-specific, strengthen the matcher controls so
each claim is proved rather than one claim per rule, close the uncovered review
budget clause, and reject any cross-scope signal leak. Preserve legitimate
rewording across the eight planned corpus rewrites.


## Context

- `tests/lifecycle_invariants.rs`
- `docs/adr/behavioural-coverage-asserts-delivery.md`
- `docs/specs/corpus-rule-ownership.md`
- `docs/specs/doubt-grove-review-mechanics.md`
- `.grove/10-DONE-review-impl-behavior-evals-k18.md`

The ADR's delivery-versus-conduct boundary is sound; these findings are defects
in the implementation of that boundary, not a request to replace the standing
deterministic suite with a model-in-the-loop gate.

## Findings

### P1 — file-wide reachability lets the wrong trigger deliver a rule

`named_files` treats any occurrence of an owner's path anywhere in a loaded file
as an edge (`tests/lifecycle_invariants.rs:120`), and `loaded_path` follows that
edge for every invariant without retaining which condition named it
(`tests/lifecycle_invariants.rs:102`). The design's edge is row-specific: a
sentence in the source names the owner for that row's trigger
(`docs/specs/corpus-rule-ownership.md:124`).

The planned router makes the failure concrete. Trigger sentences 12 and 13 both
name `references/retire.md`, for retirement and pruning respectively
(`docs/specs/corpus-rule-ownership.md:272`). If sentence 13 is deleted while
sentence 12 remains, the walk still loads `retire.md`; the pruning prose there
satisfies both pruning claims, and the suite stays green even though a session
whose path looks decided against has no condition telling it to open that file.
That is the deleted-in-effect state the test claims to reject. Make delivery
rule/trigger-specific rather than proving only that some edge reaches the owner
file.

### P1 — the near-miss control does not control every claim

The negative control accepts an invariant whenever `unmet` contains *any* claim
(`tests/lifecycle_invariants.rs:637`). Multi-claim invariants can therefore have
one well-controlled claim while another is a topic match or even accepts the
opposite rule:

- `bigger-than-brief-decomposes` claim 0 requires only “proves bigger” and
  “node” (`tests/lifecycle_invariants.rs:374`). “A leaf that proves bigger stays
  a leaf, not a node” passes it. The existing near miss already satisfies claim
  0; claim 1 alone makes the invariant fail (`tests/lifecycle_invariants.rs:386`).
- `review-budget` claim 2 requires only “bootstrap” and “mandate”
  (`tests/lifecycle_invariants.rs:491`). The generic Bootstrap paragraph already
  contains both words (`content/SKILL.md:103`), so deleting the allowance's
  mandate/adoption predicate from its owner does not make this claim fail.
- `one-focused-commit` makes “artifact”, “grow verb”, and “grow-verb” alternatives
  in one group (`tests/lifecycle_invariants.rs:449`). It therefore accepts “one
  focused commit carries the artifact and rename; grow-verb writes land later”,
  although the rule requires all three classes of change together.

Give every claim its own plausible negative control and tighten the groups that
currently ignore polarity or turn required conjunctions into alternatives.

### P1 — the starred review-budget rule is missing its over-budget outcome

The B-starred inventory row requires both the leaf-wide one-reviewer cap **and**
that a second need becomes a `review-*` leaf
(`docs/specs/corpus-rule-ownership.md:562`). The test covers the cap and leaf-wide
scope, then covers the mandate/Bootstrap predicate from the separate, unstarred
row at line 563 (`tests/lifecycle_invariants.rs:482`). It never asserts the
second-review externalisation clause. Pinning the `review-budget` rule ID in the
coverage-table test (`tests/lifecycle_invariants.rs:606`) cannot recover the
missing semantics, so a rewrite that keeps the cap but deletes what to do when it
is spent passes. Add that clause to the B-starred rule's claims.

### P1 — the signal-scope control permits a dangerous partial leak

`each_endings_rule_is_absent_from_the_other_kinds_path` asserts only that
`unmet` is non-empty (`tests/lifecycle_invariants.rs:866`). That proves the other
kind's path does not state the *whole* foreign invariant; it does not prove the
foreign instructions are absent. A `finish` path can acquire “run bare
`grove-llm complete` as the last action” and still pass because the second
non-finish claim is absent. Conversely, an ordinary kind can acquire the teardown
`complete --done` instruction and pass while the other two finish outcomes are
absent. Either partial leak is already the dangerous wrong action. Require every
claim of the foreign ending rule to be absent from the other kind set.

### P2 — two matchers reject licensed canonical rewording

The stale-launch claim requires the literal word `abandoned`
(`tests/lifecycle_invariants.rs:336`), while both the inventory and canonical
trigger permit the abstraction “terminal leaf”
(`docs/specs/corpus-rule-ownership.md:551`, `:262`). A correct rewrite that says
“terminal leaf” without spelling both terminal states goes red. Likewise,
`one-focused-commit` requires the exact phrase “one focused commit”
(`tests/lifecycle_invariants.rs:449`), while the canonical row states the
contract as the artifact, grow-verb writes, DONE rename, and cascade output
landing together (`docs/specs/corpus-rule-ownership.md:641`). “One task's commit
carries … together” preserves the rule and fails the matcher. Add semantic
alternatives without weakening the required conjunctions described above.

## Confirmed verdicts

- The twelve pinned row IDs are exactly the current B-starred rows in the eight
  required areas; the defect is claim fidelity, not a missing row ID or area.
- Excising `SKILL.md`'s kind-routing table as a selector is correct. The defect is
  treating every remaining file mention as every rule-specific edge.
- `docs/adr/behavioural-coverage-asserts-delivery.md` earns its place: the
  standing-instrument choice is hard to reverse, surprising under the word
  “behavioural”, and a real repeatability/localisation-versus-conduct trade-off.
  Its rejection of model-in-the-loop evaluation as the standing gate is sound,
  and it preserves that instrument for observed obedience failures out of band.
- Leaving `tests/retire_guidance.rs` to `skill-router-k4` was correct. That leaf
  owns the file move that invalidates the existing site pin and can replace it at
  the same boundary; changing it in the green-before-and-after producer would
  weaken current coverage early.

## Done when

- Each invariant claim has an independent near-miss control, and the concrete
  inverse/incomplete sentences above fail.
- Delivery of a conditional rule depends on its own trigger, not another path
  mention to the same owner file.
- `review-budget` covers the second-review externalisation outcome.
- Every foreign ending claim is absent from the other kind set.
- The canonical “terminal leaf” and “one task's commit … together” rewordings
  pass without admitting the incomplete forms above.
- The suite's deterministic delivery boundary and the ADR remain unchanged in
  meaning; no model-in-the-loop standing gate is added.

## Notes

- `content/SIGNAL.md`, `content/SIGNAL-FINISH.md`, and `src/prompt.rs` remain out
  of scope and byte-unchanged.
- Re-run the producer's recorded test, clippy, and format verification only after
  the fixes land; the review itself ran none of them.
