# driving-conditions-k15

## Goal

Finish the universal rewrite: `content/driving.md`'s **8 universal triggering
units** and `content/SPEC-FORMAT.md`'s **1**, plus the placement of the remaining
format documents in `content/references/`. After this child, every universal
triggering unit in the corpus is a condition line.

## What this covers

**`driving.md`** (44 kB, 8 universal triggering + 18 procedural — the corpus's
most procedure-heavy file, and therefore the cheapest of the three rewrite
children). Its universal conditions: when to commission prior-art research, when
to retire research into ADRs, verifying framework decisions against the source,
verifying a repo-wide claim, recording fog without pre-slicing it, prune/reorder/
file-an-issue triage, no session summary, and never closing by inviting questions
in general.

**`SPEC-FORMAT.md`**'s one universal triggering unit (when a spec is written).

**The remaining format documents** — `ADR-FORMAT.md`, `BRIEF-FORMAT.md`,
`CONTEXT-FORMAT.md`, `SPEC-FORMAT.md`, `grilling.md` — carry **no** universal
triggering units at all (0, 0, 0, 1, 0). They are already procedure. So they move
into `content/references/` largely as-is; the work is placement and cross-linking,
not rewriting.

## The file-set budget closes here

This child is where the "under about eight files beside the ten per-kind ones"
bound is actually testable, because it is the last child to add one. Count the set
`loop-conditions-k13`, `task-format-conditions-k14` and this child have produced
and hold it against that bound. Any file over ~300 lines gets a table of contents
— the format documents and `driving.md`'s remainder are the ones most likely to
cross it.

## Attribution travels with the prose

`driving.md` carries adapted-from notices — `addyosmani/agent-skills` (MIT) and
`mattpocock/skills` (MIT), with license texts under `content/LICENSES/`. Those
notices are attached to specific sections and **must move with the sections they
attribute**, not stay behind in a file that no longer holds the text. Check
`content/LICENSES/` still resolves from wherever the attributed prose lands.

## Done when

- `driving.md`'s 8 and `SPEC-FORMAT.md`'s 1 universal units are condition lines
  with deferred remainders.
- The format documents sit at their final `content/references/` paths.
- Every universal `class=triggering` unit in the corpus is now a condition line —
  the composed-mandates golden is the check, and at this point every kind's mandate
  should be near the ~8 kB the spec's arithmetic predicts.
- Attribution notices resolve to the prose they attribute.
- The build gate passes and `cargo test` is green.

## Notes

`tests/reference_navigation.rs` walks the user-facing documentation surface and
requires each entry to actually contain a relative link — so it cannot pass by
finding nothing to do. Moving the format documents will move links; reconcile it
rather than relaxing it.

`tests/composition_guidance.rs` reads `content/driving.md` directly and is the
other file to watch.
