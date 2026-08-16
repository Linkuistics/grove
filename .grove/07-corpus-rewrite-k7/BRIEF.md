# corpus-rewrite-k7 — brief

## Goal

Rewrite `content/` from a marker-partitioned mandate corpus into a
progressive-disclosure skill: a short `SKILL.md` of conditions over a flat
`content/references/` directory carrying the procedures, in the ten-file per-kind
layout [`skill-delivered-methodology`](../../docs/specs/skill-delivered-methodology.md)
derives.

Nothing here touches Rust. The delivery swap is `guaranteed-core-k9`'s and the
machinery deletion is `mandate-machinery-k10`'s; both come after this node and
both assume this node's output.

## The mechanical fact every child depends on

**The unit markers stay in place through this whole node, and they are what makes
each child verifiable.** Two consequences, and neither is obvious:

1. **`build.rs` walks every markdown file under `content/` recursively**
   (`build.rs`, `collect_markdown`) and demands total marker partition. A
   marker-free `content/references/design.md` does not compile. So the rewrite
   cannot outrun the machinery deletion — it must be marker-preserving, and the
   markers come off in `mandate-machinery-k10` together with everything that
   reads them.

2. **The mandate is the union of triggering units** (`src/methodology.rs`,
   `compose`) and the target `SKILL.md` is *also* the union of triggering
   conditions. So `SKILL.md` shortening and `${prompt}` shortening are not two
   events to sequence — they are one edit. Rewriting an ~880-byte
   `class=triggering` unit into a one-line condition plus a `class=procedural`
   remainder shrinks both at once.

That dissolves the spec's one open ordering question (*"whether that forces one
increment or two with a flag is the planning call"*): **neither, and no flag.**
The design fact — `${prompt}` must not shrink before `SKILL.md` is short — is
satisfied by construction here, because there is no edit that does one without
the other.

## The two verification instruments

- **`tests/goldens/composed-mandates.tsv`** records the ordered unit ids each
  kind's mandate carries. A child that only *moves* units leaves it byte-identical
  — `<!-- file: order=N -->` controls composition order independently of file
  layout, so relocation is provably free. A child that *rewrites* units changes it,
  and the diff is exactly the shrink that child claims.
- **The build gate itself.** Marker grammar, `defers=` reachability, id and
  `order=` uniqueness, no deferral cycles. A child that breaks its own bookkeeping
  fails the contributor's build rather than a stranger's session.

## Shape of each rewritten unit

One universal `class=triggering` unit yields:

- **a condition sentence** in `SKILL.md`, still `kinds=* class=triggering`, now
  carrying `defers=<remainder-id>`; and
- **a remainder** — `class=procedural`, in the `content/references/` file the
  condition routes to.

Worked example the spec gives, on `skill-decompose`: ~1.1 kB about externalizing
versus absorbing becomes *work surfaced that does not serve this leaf's stated
goal, or the leaf proved bigger than its brief* → `references/decompose.md`, and
the two triggers, two verbs, the inline-continuation bar and the laziness
argument all move to that file.

A **narrowed** triggering unit is different: it stays triggering, in its per-kind
reference file, and keeps composing into that kind's mandate until the cutover.
Do not demote a narrowed condition to procedural — that would shrink the narrowed
kinds' mandates with no compensating pointer, which is a strict loss.

## The bound to hold each child against

The spec's arithmetic, so the target is a bound rather than a hope: **~51
condition lines, a loop narrative of about eighty, and a ten-row routing table —
near 200 lines, roughly 8 KiB against today's 50 KiB**, well inside the ~500-line
house ceiling. The universal triggering units split by source file as: `SKILL.md`
27, `TASK-FORMAT.md` 14, `driving.md` 8, `SPEC-FORMAT.md` 1, `MANDATE.md` 1 (that
last one is the mandate framing, which dies at the cutover, not here).

Grouping the universal *procedures* is left to the children, bounded rather than
enumerated: the loop steps and the existing format documents are the natural
seams, the set stays under about eight files beside the ten per-kind ones, and any
file over ~300 lines gets a table of contents.

## Pointers

- [`skill-delivered-methodology`](../../docs/specs/skill-delivered-methodology.md)
  — *The skill's layout is recovered, not invented* has the ten-file table;
  *`SKILL.md` states conditions; `references/` states procedures* has the split.
- `grove-llm methodology <id>` still works throughout this node and is the fastest
  way to read a deferred body while rewriting the condition that defers to it.
  It dies in `mandate-machinery-k10`.
- The guidance suite reads `content/` prose directly and will move with it:
  `tests/commit_guidance.rs`, `tests/retire_guidance.rs`,
  `tests/composition_guidance.rs`, `tests/session_kind_guidance.rs`,
  `tests/reference_navigation.rs`. Each asserts a *claim*, not a location — a
  child that relocates prose fixes the path, and a child that rewrites prose owes
  the claim a new home or an argument for dropping it.

## On the horizon

The `no procedure in SKILL.md` requirement is discharged by **review, not by
test** — the spec is explicit that the classification has no classifier once the
markers are deleted, and that no budget test may be cited as evidence for it.
`skill-opening-k16` is where that review is cut.
