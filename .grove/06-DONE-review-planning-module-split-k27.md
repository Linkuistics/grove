# module-split-k27

**Reviews:** module-split-k4

## Goal

Read the decomposition adversarially, **before eighteen sessions build on it**.
The artifact is the ordered run of leaves under `.grove/` plus the root brief's
`## Decomposition` and `## Standing notes` sections; the contract it must satisfy
is `module-split-k4`'s own `## Done when` and the four conditions below.

## Context

**Reviewed artifact:** the tree cut by `module-split-k4`. Find that session's
commit from the handle and read its diff against the current tree.

The inputs the plan was cut from, in the order the planning session read them:
`docs/specs/module-decomposition.md` (the whole input, not to be redesigned),
`decomposition-k2`'s `## Decisions (running log)`, `decomposition-k5`'s findings
and running log, and `minimalism-k1`'s `## Deletion list` and measurements.

## Done when

Four questions are answered against the artifact, with evidence, and each finding
names the contract it violates:

1. **Coverage.** Walk the spec — all eleven decisions, the four requirements with
   their scenarios, the four test seams, and the `## Out of scope` list — against
   the brief's mapping table. Is anything covered by **no** leaf? Is anything in
   the mapping table claimed by a leaf whose own `## Done when` does not actually
   deliver it? The mapping is the artifact under review, not a shortcut past the
   spec.
2. **Ordering.** Does any leaf depend on work a **later** leaf lands? The
   forced-orderings table is the plan's own claim about this and is where a wrong
   answer is most expensive. Check the unforced orderings too: three starting
   points are claimed independent (the runner, the VCS seam after the git lane
   goes, the store's operations) and that independence is an assertion, not a
   measurement.
3. **Green.** The plan asserts that **every** leaf lands with the suite passing.
   That is a strong claim for a 15,200-line deletion and a four-crate extraction.
   Find the leaf where it is false. The likeliest candidates are the crate splits
   that put an interface in one leaf and its only consumer in the next, and the
   plugin's expand/contract pair.
4. **Meta-grove hazards.** Two leaves change the tooling the *next* session uses:
   `grammar-separator-k15` and `open-kind-k20`. Are their install sequences
   actually sufficient on this machine, given Homebrew's Cellar symlinks and the
   `PATH` precedence trap? Is the claim right that the build-pairing guard stops
   the loop before k18 and does not after? Is there a **third** leaf that changes
   the verb surface and does not say so?

## Notes

**Why this chain was cut.** `references/decompose.md`'s test names *a
decomposition others will build on for months* as earning a review chain, and
this is one: eighteen sessions, two hard-to-reverse orderings, and a leaf whose
failure mode is a wedged loop rather than a red test. The producer also had to
carry three corrections its own inputs had already made to each other
(`tree_format`'s survival, the pair verb's fate, the completeness quantifier), so
the risk of a fourth going unnoticed is not hypothetical.

**Sequence, not prose, is what is under review.** A finding that the plan should
have been written differently is not useful; a finding that leaf X cannot run
where it sits is.

**If nothing is worth acting on, create nothing and retire.** That is the normal
outcome and the reason the chain is lazy. An `integrate-review-planning` step is
cut only if this session has findings — and it is cut with `leaf-insert` at the
first sibling entry after this one whose subtree still holds live work, which
here is `delete-migration-k6`, because every intervening leaf would move the
coordinates a finding cites.
