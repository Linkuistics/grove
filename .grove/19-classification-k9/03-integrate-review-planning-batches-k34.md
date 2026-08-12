# batches-k34

**Integrates:** `batches-k33`

## Goal

Repair the classification batching plan before `spine-k21` executes. Apply all
seven findings from `batches-k33` to the node brief and the twelve batch bodies;
do not write any `content/` marker in this session.

The repaired plan must have one unambiguous owner for every cross-file edge,
stable semantic region boundaries, and explicit decisions for the repeated
load-bearing rules. The integration reshapes planning artifacts only.

## Context

Read `batches-k33`'s `## Review findings` in full. The actionable findings are:

1. **F1 — mutable coordinates.** Earlier batches insert marker lines above later
   line ranges in `SKILL.md`, `TASK-FORMAT.md`, and `driving.md`. Make headings
   and predecessor `pending-*` ids authoritative; label line ranges and byte
   counts as pre-classification orientation only.
2. **F2 — lossy pending-edge ledger.** A `defers=` member parked on a coarse
   residual can be silently dropped while another inbound path keeps the target
   reachable. Forbid edges whose source is `pending-*`. If the source is already
   carved, the target/body batch owns the inbound edge; if it is pending, the
   later source batch owns the outbound edge. Remove redistribution as a
   protocol and give every edge one owner.
3. **F3 — filename grep is incomplete.** In particular,
   `prompts/continue.md:2-6` refers to “the skill's Decompose step” without the
   literal filename. Add a semantic cross-file/duplicate-rule inventory and
   reconcile it alongside the filename sweep.
4. **F4 — repeated-rule calls are incomplete and backwards.** Inventory and
   pre-decide the triggering/procedural ownership for every site of reviewer
   ownership, externalize-vs-absorb, ADR reworking, and spec current-state. The
   review names missing sites in `TASK-FORMAT.md` and a second `SKILL.md` site;
   include those, not only the original four-row table. With the calls settled,
   the existing batch order may remain.
5. **F5 — overbroad greenness lemma.** Rename (D), (R), and (T) as the complete
   cross-unit deferral-graph obligations, not all obligations a subdivision
   creates. List the local marker/id/kind rules each batch still owns.
6. **F6 — `## Reference files` false binary.** Decide its actual class and
   inbound trigger, or explicitly carry it as a narrative/design finding for the
   aggregate review. Do not leave the final child choosing only between eight
   unconditional edges and zero.
7. **F7 — incorrect coverage explanation.** The 284-byte residue is a 281-byte
   YAML preamble plus the three separator newlines at baseline `SKILL.md` L246,
   L407, and L609. Assign the separators to adjacent semantic regions and correct
   the region byte counts.

`batches-k33` dismissed the size-spread and non-contiguous-`kinds-k22` doubts;
preserve those shapes unless a finding above independently forces a change.

## Done when

- The node brief's batching contract states the stable boundary and unique edge
  ownership rules, with a complete semantic inventory for implicit edges and
  repeated rules.
- All twelve child bodies agree with that contract; none tells a later session
  to redistribute a `pending-*` list or execute against a mutable line number.
- The four repeated-rule families have explicit, corpus-wide decisions naming
  every known site and each triggering owner/procedural target.
- The greenness lemma is narrowed, the `## Reference files` issue is resolved or
  deliberately handed to aggregate review, and the coverage arithmetic assigns
  all non-preamble bytes.
- No `content/`, Rust, or test file is edited. The next live leaf remains
  `spine-k21` after this integration retires.

## Notes

- This is `integrate-review-planning`: verify each finding against the corpus,
  then repair the plan. If a finding is wrong, record the concrete reason in
  this body rather than silently skipping it.
- Keep the producer's useful batching work. The review did not find a reason to
  merge batches or move `TASK-FORMAT.md` L473–501 out of `kinds-k22`.
