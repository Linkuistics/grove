# batches-k13

## Goal

Derive the **dependency-ordered classification batches** for this node and cut
them as child leaves. The deliverable is more tree — not a single marker written
into `content/`.

Each batch must be a vertical slice: it leaves `cargo build` and `cargo test`
green on its own commit, it advances the real classification by a nameable amount,
and it does not wait on a sibling to be worth having.

## Context

The node brief carries the classification rule, the corpus and the traps; read it
first and do not restate it. What this leaf owns is the **order**.

### The constraint that makes this a planning problem

`embed-wide-gate-k8` enforces, across the whole embed, that every `defers=`
resolves, that it resolves to a **procedural** unit, and that every procedural
unit is reachable from some triggering one. The starting state is one trivial
`class=triggering kinds=*` unit per file.

So the moment a batch classifies file A and gives one of its triggering units a
`defers=` pointing at a procedure that *belongs* in file B, the build fails unless
file B has been classified far enough to declare that procedural unit. Equally, a
batch that carves procedural units out of file B without any triggering unit
reaching them fails reachability. **Both directions of the deferral graph
constrain the batching, and the constraint crosses files.**

`content/SKILL.md` is the hub: it defers into `driving.md`, `grilling.md`,
`TASK-FORMAT.md`, and the four format guides. A one-file-per-child split is
therefore the *least* likely shape to be green.

### What to actually produce

Map the deferral closure first, then cut batches from it. Some shapes worth
weighing rather than a prescription:

- **By closure** — a triggering unit and the procedural units it defers to land
  together, wherever those live. Green by construction; batches may be oddly
  shaped and may touch several files each.
- **By file, with a bridging order** — classify leaf-of-the-graph files first
  (the format guides, `grilling.md`, `driving.md`), so that by the time
  `SKILL.md`'s conditions are carved, their targets already exist. Cheaper to
  reason about per session; requires proving the order really is topological.
- **Hybrid** — the small format guides in one batch, `driving.md` and
  `TASK-FORMAT.md` next, `SKILL.md` split across several closure-shaped batches
  because it alone is 51,524 bytes.

Whatever you choose, say **why** it is green at every boundary, because that
claim is the entire output of this leaf.

Weigh session size honestly alongside greenness. A batch that is green but asks
one context to judge 40 kB of prose has traded one runaway leaf for another.

### Two things every batch's body must carry

- **The pinned complete id set is updated by each batch**, deliberately, naming
  each new id. That is the design's confirmation point, so a batch that adds units
  and leaves the constant to a later batch has skipped it.
- **The doubts, carried forward.** Each batch records the units it was least sure
  about, by id, so the final batch can assemble them into the aggregate
  `review-impl` handoff rather than reconstructing them from diffs.

## Done when

- The deferral closure over the nine embedded files is mapped well enough to
  justify the batch boundaries, and that mapping is written down where the batch
  leaves can read it — this leaf's body or the node brief, not a scratch file.
- Child leaves exist for every batch, in dependency order, each with a body
  stating its files, its share of the closure, why it is green on its own commit,
  and the two obligations above.
- The **final** batch's body states that its last act is to cut the aggregate
  `review-impl` leaf, naming the pre-classification baseline commit and every
  batch child's `<slug>-k<key>` handle.
- No `content/` marker is written by this session. If mapping the closure requires
  trying a marking to see whether it is green, that is a signal the batching is
  underdetermined — say so and cut a smaller first batch rather than absorbing the
  work.

## Notes

- This leaf exists because `increments-review-k11` B5 found the single
  `classification-k9` leaf too large, and `increments-integrate-k12` decomposed it
  rather than guessing the batches from outside the corpus. Guessing them here,
  with the files open, is the point — do not shortcut back to filenames.
- If the closure turns out to be shallow enough that two or three batches obviously
  suffice, say so and cut them. A planning leaf that concludes "this is smaller
  than it looked" is a correct outcome, not a wasted session; the runaway risk was
  real either way.
- Conversely, if a single file's closure is itself too big for one session, that
  file's batch is a `leaf-decompose` candidate for the session that reaches it —
  note it in that batch's body rather than pre-emptively nesting here.
