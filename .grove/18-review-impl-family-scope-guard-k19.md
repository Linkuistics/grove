# family-scope-guard-k19

**Reviews:** family-scope-guard-k18

## Goal

Adversarially read the family-scope guard `family-scope-guard-k18` landed: the
new section at the foot of `tests/session_kind_guidance.rs`, and the current-state
prose it wrote into `docs/specs/mandate-delivered-methodology.md`, `CHANGELOG.md`
and `CONTEXT.md`.

The producer's commit names the work item by that handle; read its diff against
the current source. **Inspection only** — do not run the build, the tests, or a
formatter, and do not edit the guard. Findings only; the fixes belong to whatever
`integrate-review-impl` leaf this session decides is needed.

## Context

The guard closes the hazard `unit-scope-audit-k4` recorded: three `kinds=` scopes
in `content/TASK-FORMAT.md` — `task-review-kinds`, `task-integrate-review-kinds`,
`task-research-pair` — are each exactly a family partition of the closed kind set,
spelled out label by label because the grammar has no family shorthand, and a kind
added to one of those families had to be hand-added to the marker with nothing
complaining.

What landed:

- An exhaustive `match` over `Kind` (`fn family`) classifying every kind into one
  of six families, with `Family::members()` deriving each family's membership from
  it, so a twentieth kind fails to compile until it is classified.
- `FAMILY_SCOPES`, three `(unit id, family)` pairs, and
  `every_family_shaped_scope_names_its_whole_family` asserting each marker's scope
  reaches exactly its family — reach read through the composer's own
  `Scope::admits`.
- `no_scope_is_shaped_like_a_family_without_being_guarded`, the inverse sweep.
- Two control tests, and the `family()` doc comment recording what the match does
  **not** force.

The producer verified the whole chain by hand, outside the commit: a twentieth
kind (`Kind::ReviewSpike`) added to the enum failed to compile at `fn family`, and
once classified into `Family::Review` failed the assertion with
`content/TASK-FORMAT.md: task-review-kinds … missing [review-spike]`. That
experiment was reverted; nothing in the tree records it, so **re-derive it rather
than trusting this paragraph**.

## Done when

Each doubt below is answered — confirmed, or written up as a finding with the
`path:line` it lands on. They are ordered by how much of the session they deserve;
the first is the one this leaf exists for.

1. **The inverse sweep was not asked for.** `family-scope-guard-k18`'s `Done when`
   has four bullets and none of them is
   `no_scope_is_shaped_like_a_family_without_being_guarded`; the producer added it
   on the argument that a registry nothing checks is the same silent omission one
   level up. Attack that. Is it in scope for a leaf whose Notes say *do not widen
   this into the scope audit*? Can it fire on a scope a future author would
   legitimately write — one that happens to equal a family's membership without
   being *about* that family? Is `multi_member_families`'s exclusion of
   single-member families a principled line or a patch over the false positives
   `kinds=finish` and `kinds=combine-research` would otherwise produce?
2. **Two exhaustive matches over `Kind` now live in test code**, and nothing
   cross-checks them: `fn family`'s `Family::Producer` arm restates the five kinds
   `src/leaf.rs`'s `is_producer` lists. The leaf's Goal was to *remove* a
   hand-maintained roster. Is this a second one wearing a classifier's clothes, and
   if so is the fix to cross-check them, to derive one from the other, or to accept
   it and say why?
3. **A misclassified kind passes.** The `family()` doc comment admits it: a
   `review-…` kind filed under `Family::Producer` leaves `task-review-kinds` narrow
   and every assertion green. The producer judged this the same residue
   `is_producer` already carries and declined to close it, because the only closure
   available is a cross-check against label prefixes — the derivation the leaf's
   `Done when` rules out. Is that reasoning sound, or is a cross-check admissible
   precisely *because* it is not the derivation?
4. **Reach, not spelling.** `admitted_kinds` asks the scope which kinds it admits
   rather than comparing its label list, so a marker that lists a member twice, or
   lists the family out of `Kind::ALL` order, passes. The producer's argument is
   that the claim is about which kinds a scope carries. Is a duplicate label
   something a human should be made to fix?
5. **The prose is current-state and untested.** Four documents gained claims about
   this guard. Check each against the code rather than against this brief:
   - `docs/specs/mandate-delivered-methodology.md` — the rewritten paragraphs under
     *A unit narrows when a kind added later would not need it*, and the new
     *Test seams* bullet. Is the spec still a **minimum coherent set**, or did two
     paragraphs land where one would do? The producer deliberately added **no**
     `## Requirements` entry, on the reading that this is a maintenance guard
     rather than a contract about what a session receives — is that the right
     grain?
   - `CHANGELOG.md`'s `## Unreleased` / `### Added` entry — right section, and
     accurate?
   - `CONTEXT.md`'s new `_Avoid_` line under *Triggering unit / procedural unit*.
     Is it a definition, or has implementation detail leaked into the glossary?
6. **The controls.** Both classifiers are shown failing and passing on synthetic
   input, with the real embed as the passing half. Is any assertion in them true
   for a reason other than the one it claims — in particular
   `the_family_shape_sweep_separates_a_family_from_every_other_scope`'s four quiet
   shapes, each of which must be quiet for its *own* reason and not incidentally.

## Notes

**Ordering.** This leaf sits ahead of `templated-mandate-k12`, which the human
placed last on purpose. If findings warrant an `integrate-review-impl` leaf, cut
it with `leaf-insert` at the first sibling entry after this one whose subtree
still holds live work — which is `templated-mandate-k12` — rather than appending
after it.

**A review that finds nothing creates nothing.** The producer's own judgement is
that doubts 1 and 2 are the ones with a real chance of surviving, and that 4 and 6
are most likely confirmations. If that holds, retire without cutting an integrate
leaf.
