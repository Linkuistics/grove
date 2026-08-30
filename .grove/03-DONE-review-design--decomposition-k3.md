# decomposition-k3

**Reviews:** decomposition-k2

## Goal

Read `docs/specs/module-decomposition.md` adversarially, against the codebase it
describes and against `minimalism-k1`'s decisions. The question is whether four
crates, a plugin and a changed filename grammar are **implementable as written**
and **internally consistent** — not whether the shape is agreeable. The human
already agreed the shape; that is the wrong question for this leaf.

## Context

`minimalism-k1` and `decomposition-k2` are both in the tree; read `k2`'s
`## Decisions (running log)` before the spec, because several decisions carry
warrants that did not fit the spec's grain rule.

Three places this design **corrected its own input**, and each is a place to
check the correction rather than the original:

1. **Workspace resolution moved to the VCS seam.** `minimalism-k1`'s `## Context`
   assigned it to the loop; that measurement predates decision 1 (jj only), which
   makes the seam's own refusal `vcs_of`. If the correction is wrong, the loop
   ends up shelling out to `jj` — check whether anything in the loop still needs
   a workspace fact the seam does not expose.
2. **Configuration completeness became document-eager / kind-just-in-time.**
   `minimalism-k1` predicted the quantifier would become *every kind the
   methodology declares*; that restatement needs the manifest its own decision 7
   deleted. Check that the amendment really does preserve what
   `complete-session-configuration` says is load-bearing, and that nothing
   downstream depended on the all-nineteen check.
3. **`entries-are-never-removed` gains a clause.** `minimalism-k1` lists it as
   untouched, but the store gains `delete`. Check whether one clause is enough or
   the record's argument actually reaches root deletion too.

## Done when

- Every interface in the spec is checked against the code it replaces, and each
  finding names the specific call site or contract that cannot be satisfied.
- The `--` grammar is checked for a second ambiguity. The one this design found
  is kind-versus-slug; look for others — the outcome infix against a kind
  beginning `DONE`, a slug that renders `--` through some other path, a node
  directory name that now reads as a leaf.
- The five-site kind table is checked for a **sixth** site. `minimalism-k1` found
  five where its own task file predicted one; this design resolved those five
  without a manifest, and the same undercount is the likeliest defect here.
- The judgement is stated either way. A review that finds nothing creates nothing
  and simply retires.

## Notes

**Three claims worth attacking specifically.**

- *"Grove names a kind only where grove writes the leaf."* Two tokens,
  `requirements` and `finish`. Check the grow verbs, selection, root-init and the
  prompt for a third — and check whether the `finish` sentinel's "ordinary work
  outranks it wherever it sits" rule really is expressible with one literal
  token and no ordering property.
- *"`exists?` is a shape, not a predicate."* The claim is that widening the two
  opening functions removes the classify-then-act race. Check that every current
  caller of the two-phase dance actually maps onto one of the two new variants,
  rather than needing a third answer the design refused to add.
- *"`jj-workspace` is fully domain-free."* The brief said *partly*. Check whether
  anything grove-shaped survives in it — a `.grove/`-aware path, a finish-shaped
  message, a lease detail that is really the loop's.

**One thing that is not a finding.** The second-hop gap in skill delivery
(`grove-<kind>` → the shared spine) is recorded as unmeasured, deliberately, in
the shape `docs/research/wording-micro-test.md` records its own gaps. Restating
it as a finding adds nothing; finding a *cheap instrument* for it would.

**Out of scope.** The four calls the human signed off — the `--` separator, the
configuration amendment, the skill fatness rule, and the crate names — are
settled. Re-argue one only if the review finds it *cannot be implemented*, which
is a different claim from preferring another.
