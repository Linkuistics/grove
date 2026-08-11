# chain-contiguity-integrate-k8

**Integrates:** chain-contiguity-review-k7

## Goal

Apply the actionable findings from the adversarial review of
`chain-contiguity-k6`: state the exact pre-order placement condition, remove the
remaining structural contradiction, and make the guidance tests capable of
detecting an inverted or incomplete rule.

## Context

### 1. The placement trigger is wrong for a later sibling node

`content/SKILL.md:353-361` says to insert before the first live *leaf* following
the review among its siblings. The walk does not operate only on sibling leaves.
`src/tree_read.rs:113-129` reads each level in position order and immediately
recurses into a node, so this tree is a counterexample:

```text
01-review-impl-sync-review-k10.md
02-follow-up-k11/
  BRIEF.md
  01-impl-follow-up-k12.md
03-DONE-impl-old-k13.md
```

If the integration is appended, `pick` descends into `follow-up-k11` before it.
The correct insertion target is the **node directory** `follow-up-k11`, not its
descendant live leaf; targeting the descendant would insert at the wrong level.
The exact condition is: find the first later sibling entry in the review's own
parent whose subtree contains eligible ordinary live work, and insert before
that sibling entry. Later terminal leaves and fully terminal nodes do not count.

The directory-local boundary itself is sound. If the review is inside a node,
pre-order finishes that node (including a newly appended integration) before it
visits a later sibling of the node. A live leaf in that outer sibling node cannot
intervene. Preserve that distinction when reconciling the surfaces.

`src/tree_grow.rs:1121-1163` pins only the easy direct-live-leaf shape and never
calls `pick`; it neither exposes this node counterexample nor pins the harmless
DONE-entry case.

### 2. The producer-to-review reasoning is directionally right but overclaims

The historical remedy is real: the review body names the producer handle, task
commits must name that handle, and the review discipline already requires the
producer's committed diff. A reviewer can therefore locate the producer commit,
and no citations have been frozen before that reviewer runs. The original claim
that this hop "cannot be split" was false; a pre-existing live sibling can run
first.

However, `content/SKILL.md:340-345` says a gap is *free* and intervening work
"invalidates nothing." If that work rewrites the reviewed artifact,
requirements, or evidence, the producer's historical diff and the current tree
diverge and the reviewer must reconcile them. That is not the integration hop's
silent stale-citation failure, so it does not by itself justify adjacency, but it
is not zero-cost either. Narrow the claim to the property established: the
review re-derives a fresh handoff and therefore has no pre-written citation to
stale. Say explicitly that it locates the producer commit by stable handle and
checks that diff against the current source.

Under that narrower conclusion, the untouched
`plugins/linkuistics/skills/doubt-driven-development/SKILL.md:82-88` boundary is
correct: producer-to-review escalation remains `leaf-add`.

### 3. The exception is fail-safe but nearly vacuous

The sentence at `content/SKILL.md:364-369` does read in the safe direction: if
the check cannot be performed, there is no exception. But when a review is
cutting its integration, a later live leaf normally has not run, and Grove does
not make its eventual file set an enforceable part of the task contract. A goal
or pointer list is not proof that the session will touch no cited file (the
`clippy-baseline-k4` CHANGELOG edit is the worked counterexample).

Prefer collapsing the exception to "insert before the first blocking sibling"
unless a concrete, checkable source of proof can be named. If the exception is
retained, define sufficient evidence rather than telling a session to prove an
open-ended future file set. Keep the current fail-safe fallback either way.

### 4. The surface sweep missed a current structural contradiction

No surviving current instruction says merely "use `leaf-insert` when order
matters," and the edited surfaces consistently say the placement rule is
methodology rather than a tree guarantee. Released CHANGELOG prose is correctly
left historical.

But `CONTEXT.md:436-445` still says a review chain's steps "run in sequence
because they are siblings at adjacent positions," and `CONTEXT.md:455-457` says
they are adjacent because each was appended. Both are false for the explicitly
accepted producer-to-review gap: appending after a pre-existing sibling makes
the chain non-adjacent and lets that sibling run between its steps. Reconcile
these current-state claims rather than relying on the following convention
qualification to contradict them.

### 5. The guidance test does not pin the semantics it claims to pin

`tests/composition_guidance.rs:163-224` performs independent whole-document
substring checks. A surface can invert the rule (say the review consumes and the
integration re-derives) while retaining `re-derive`, `consume`, `leaf-insert`,
and the exception phrase, and the test still passes. The `silent` assertion for
the whole CHANGELOG is likewise satisfied by unrelated entries. Pin each verb
and property to its hop in one sufficiently distinctive assertion or in a
bounded section.

Excluding all of `CHANGELOG.md` from the superseded-sentence assertion is too
broad. Preserve frozen releases, but bound the assertion to the live
`## Unreleased` section so the old unaided-judgement instruction cannot return
there unnoticed.

Add behavioral shapes for:

- a later direct live leaf (the existing easy case);
- a later sibling node with a live descendant (insert before the node);
- terminal leaves or terminal nodes between review and integration (append is
  still selected next); and
- a review inside a node with live work in a later outer sibling node (append
  inside the review's node remains next by pre-order).

The tests must ask `pick` what runs next; a filename adjacency assertion alone
does not establish the scheduling claim.

### 6. No new ADR remains the right decision

The placement guidance is prose with no enforcing mechanism, so it fails the
decision-record skill's *hard to reverse* condition even though it is surprising
and trades adjacency against intervening work. Do not add a sub-rule ADR.
`docs/adr/grove-owns-escalated-review.md` has no dangling citation and remains
coherent while producer-to-review escalation stays `leaf-add`. Reconcile that
ADR and the doubt skill only if integration instead changes that hop's verb.

## Done when

- Every instructing surface and both CLI help texts describe the exact
  directory-local blocking-sibling condition, including live node subtrees and
  terminal-entry exemptions.
- The producer-to-review explanation claims fresh re-derivation rather than a
  literally cost-free gap, and tells the reviewer how the stable handle leads
  to the producer commit.
- The future-file-set exception is either removed or given concrete sufficient
  evidence a session can actually check.
- The contradictory current-state adjacency claims in `CONTEXT.md` are removed.
- Guidance tests bind `review` to re-derivation and `integrate` to consumption,
  scope CHANGELOG checks to `## Unreleased`, and exercise selection for the four
  tree shapes above.
- The minimum coherent ADR set remains unchanged unless the integration changes
  producer-to-review placement after all.
- Run the full test suite, formatting check, and clippy after applying the fixes.

## Notes

The producer recorded: 548 library tests plus all 39 test binaries passed, with
zero failures; `cargo fmt --check` and `cargo clippy --all-targets` were clean.
This inspection-only review did not rerun them.
