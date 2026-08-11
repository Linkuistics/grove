# chain-contiguity-review-k7

**Reviews:** chain-contiguity-k6

## Goal

Try to **disprove** the placement rule `chain-contiguity-k6` wrote into the
methodology, and the reconciliation it performed across nine surfaces. This is a
meta-grove: `content/` at this commit is what gets provisioned to the *next*
session, so a rule that is wrong here is a rule every future grove obeys.

## Context

`chain-contiguity-k6` narrowed the blanket position *"a chain is not contiguous
by construction, and that cost is accepted"*. The narrowing rests on **one
asymmetry**, stated in the same words on every surface:

- A **`review-*`** step **re-derives**: it reads the producer's commit — held
  immutably by history — and computes its own `path:line` citations against the
  tree as it then stands. So a gap before it is free, and `leaf-add` is right for
  it wherever it lands.
- An **`integrate-review-*`** step **consumes**: its input is citations the
  review already froze into prose, resolved against a working tree that has since
  moved. An intervening edit to a cited file shifts them, and the drift is
  *silent*. So it is cut adjacent to its review by default, with `leaf-insert` at
  the review's first live sibling; `leaf-add` is correct only when the review has
  no live sibling after it.

The exception was made a check rather than a judgement: depart only when the
intervening work **provably touches no file the findings cite**.

The evidence for the `integrate` half is concrete and already in the tree —
`clippy-baseline-k4` had to place a `CHANGELOG.md` entry below line 67 by hand
because finding 6 of `flat-lazy-review-review-k3` cited `CHANGELOG.md:67`.

## Done when

Each item below is either confirmed or refuted with an argument.

### 1. The `producer → review` half — the claim I am least sure of

"A gap before a review is free" is asserted, not demonstrated, and it is the
half with no worked example behind it. Attack it:

- Construct the case where it fails. If unrelated work rewrites the producer's
  artifact before the review is picked, the review reads a merged tree. Is
  "it can always read the producer's commit" a real remedy, or does it quietly
  ask a review session to diff against history it has no instruction to find?
  Nothing in `content/` tells a `review-*` session to locate its producer's
  commit.
- The task file for `k6` framed this hop as *"cannot be split, because a producer
  cuts its review as its own last act and nothing can intervene."* `k6` rejected
  that framing as false — `leaf-add` appends at the parent's end, so a
  pre-existing live leaf sits between them — and substituted the re-derives /
  consumes argument. Was that substitution right, or did it discard a true
  constraint the original framing was reaching for?

### 2. The trigger condition, which `k6` corrected mid-session

The rule first read "whenever any live leaf holds the **next slot**", then was
corrected to "whenever the review has any **live sibling after it**" — because
`leaf-add` appends at the parent's *end*, not at the next slot, and because
terminal leaves between the two steps are invisible to `pick`.

- Re-derive that condition from `pick`'s pre-order walk and `leaf_add`'s
  positioning, and say whether the corrected version is exactly right.
- Specifically: is the *directory-local* framing sound? The claim is that a live
  leaf in a **sibling node directory** after the review's own directory cannot
  intervene, because pre-order finishes a directory before leaving it. Check it
  against `tree_read`'s walk rather than against the prose.
- `an_integration_cut_with_insert_lands_beside_the_review_it_integrates`
  (`src/tree_grow.rs`) pins one shape. Does it pin the *interesting* one, or only
  the easy one? The `DONE`-leaf-in-between case is argued in prose and tested
  nowhere.

### 3. Is the exception's "test" actually performable?

"Depart only when the intervening work provably touches no file the findings
cite." A leaf's file set is knowable only after it runs.

- Does that make the exception vacuous — i.e. is the rule really "always
  `leaf-insert`", dressed as a conditional? If so, say whether the conditional
  earns its complexity or should collapse.
- `k6` added "if that check cannot be performed … you do not have the exception",
  which is the fail-safe direction. Confirm it reads that way to a session that
  wants the exception.

### 4. Surface sweep — did anything go unreconciled, or get over-reconciled?

Nine surfaces were edited: `content/SKILL.md`, `content/TASK-FORMAT.md`,
`content/driving.md`, `CONTEXT.md` (**Review chain**, **Pick**, **Position**),
`CHANGELOG.md` (in the live `## Unreleased`), `docs/USAGE.md`,
`docs/ARCHITECTURE.md`, `docs/specs/doubt-grove-review-mechanics.md`,
`src/llm_cli.rs` (both verbs' `--help`), plus `src/tree_grow.rs` test comments
and `.grove/BRIEF.md`.

- Sweep for a surviving statement of the blanket position that was missed.
- The reverse failure matters as much: has adjacency leaked into a claim about
  what the *tree* or the *verbs* guarantee? Every surface is supposed to say the
  rule binds the session cutting the leaf and nothing else.
- `plugins/linkuistics/skills/doubt-driven-development/SKILL.md` was
  **deliberately not** touched — it instructs only the `producer → review`
  escalation, whose verb is unchanged. Confirm that boundary, or name what it
  should carry.

### 5. The guidance test

`guidance_cuts_the_integrate_step_adjacent_to_the_review_it_integrates`
(`tests/composition_guidance.rs`) pins, per surface: `leaf-insert`, `silent`,
`touches no file the findings cite`, and the `re-derive` / `consume` pair; plus
`cross-leaf grammar` where the framing must survive, and an `assert_absent` for
the superseded sentence on the instructing surfaces only.

- Can the test pass while the guidance is wrong? In particular, would it catch a
  surface that kept every pinned token but inverted the two hops?
- `assert_contains(surface, text, "silent")` on `CHANGELOG.md` matches 21 times
  for unrelated reasons. Is that pin vacuous there, and does it matter?
- `CHANGELOG.md` was excluded from the `assert_absent` loop on the grounds that
  released sections are a frozen record. Judge that call.

### 6. The ADR decision

`k6` applied the three-part when-to-write test and wrote **no** ADR: the rule is
prose with no mechanism, so it fails *hard to reverse*; the leaf itself defers
enforcement to a separate decision with its own ADR; and the surrounding
flat-and-lazy decision is not an ADR either (it lives in `docs/ARCHITECTURE.md`
and `CONTEXT.md`), so a sub-rule ADR would sit oddly against it.
`docs/adr/grove-owns-escalated-review.md` was left untouched because
*"Escalating is one `leaf-add`"* describes the `producer → review` hop, which is
unchanged.

Test both halves — the omission, and the claim that the ADR carries no dangling
citation.

## Notes

**Where `k6` was least confident**, in order: item 1 (the `producer → review`
half rests on an argument, not an example), item 3 (whether the exception is
performable at all), and item 2's directory-local framing.

**Out of scope**, settled by `flat-lazy-review-k2` and re-affirmed by `k6`'s own
brief: reinstating the chain node, and any mechanism that *enforces* contiguity.
Re-open either only with an argument neither brief considered.

This is inspection only — findings, not fixes. Do not run build, test, lint or
format commands, and do not edit the surfaces; cut an
`integrate-review-impl` leaf as your last act if there is anything worth acting
on, and nothing if there is not.
